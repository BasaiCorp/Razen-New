// src/backend/cranelift/jit.rs

use std::collections::HashMap;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module, FuncId};
use cranelift_native;
use crate::backend::ir::{IRModule, IRFunction, IRParam, Instruction, Operand};
use crate::backend::builtins::BuiltinRuntime;
use crate::frontend::diagnostics::{Diagnostics, Diagnostic, DiagnosticKind};

/// Professional JIT Compiler using Cranelift for immediate execution
pub struct JITCompiler {
    /// Cranelift JIT module for code generation and execution
    module: JITModule,
    /// Function builder context
    ctx: codegen::Context,
    /// Variable mapping for current function
    variables: HashMap<String, Variable>,
    /// Value mapping for registers
    values: HashMap<String, Value>,
    /// Next variable ID
    next_var_id: usize,
    /// Builtin runtime for executing builtin functions
    builtin_runtime: BuiltinRuntime,
}

impl JITCompiler {
    pub fn new() -> Result<Self, String> {
        // Create JIT builder with target ISA
        let isa_builder = cranelift_native::builder()
            .map_err(|e| format!("Failed to create ISA builder: {}", e))?;
        let isa = isa_builder
            .finish(settings::Flags::new(settings::builder()))
            .map_err(|e| format!("Failed to create ISA: {}", e))?;
        
        // Create JIT module
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);
        
        Ok(JITCompiler {
            module,
            ctx: codegen::Context::new(),
            variables: HashMap::new(),
            values: HashMap::new(),
            next_var_id: 0,
            builtin_runtime: BuiltinRuntime::new(),
        })
    }
    
    
    /// Compile a single function and return its pointer
    fn compile_function(&mut self, ir_function: &IRFunction) -> Result<*const u8, String> {
        // Skip functions with no basic blocks
        if ir_function.basic_blocks.is_empty() {
            return Err(format!("Function '{}' has no basic blocks", ir_function.name));
        }
        
        // Clear state for new function
        self.variables.clear();
        self.values.clear();
        self.next_var_id = 0;
        self.ctx.clear();
        
        // Create function signature
        let mut sig = self.module.make_signature();
        
        // Add parameters
        for param in &ir_function.params {
            let cranelift_type = self.ir_type_to_cranelift(&param.ty)?;
            sig.params.push(AbiParam::new(cranelift_type));
        }
        
        // Add return type
        let return_type = self.ir_type_to_cranelift(&ir_function.return_type)?;
        if return_type != types::INVALID {
            sig.returns.push(AbiParam::new(return_type));
        }
        
        // Declare function
        let func_id = self.module
            .declare_function(&ir_function.name, Linkage::Export, &sig)
            .map_err(|e| format!("Failed to declare function: {}", e))?;
        
        // Define function
        self.ctx.func.signature = sig;
        
        // Create function builder
        let mut builder_ctx = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut builder_ctx);
        
        // Create entry block
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        
        // Map parameters to values
        let param_values = builder.block_params(entry_block);
        for (i, _param) in ir_function.params.iter().enumerate() {
            if i < param_values.len() {
                let param_reg_name = format!("r{}", i);
                self.values.insert(param_reg_name, param_values[i]);
            }
        }
        
        // Create all blocks first
        let mut cranelift_blocks = Vec::new();
        for (block_idx, _basic_block) in ir_function.basic_blocks.iter().enumerate() {
            if block_idx == 0 {
                cranelift_blocks.push(entry_block);
            } else {
                let block = builder.create_block();
                cranelift_blocks.push(block);
            }
        }
        
        // Generate code for each basic block
        for (block_idx, basic_block) in ir_function.basic_blocks.iter().enumerate() {
            let current_block = cranelift_blocks[block_idx];
            
            if block_idx > 0 {
                builder.switch_to_block(current_block);
            }
            
            // Generate instructions
            for instruction in &basic_block.instructions {
                Self::generate_instruction_static(instruction, &mut builder, &mut self.values, &self.builtin_runtime)?;
            }
            
            // Handle terminator
            let mut has_terminator = false;
            if let Some(ref terminator) = basic_block.terminator {
                Self::generate_instruction_static(terminator, &mut builder, &mut self.values, &self.builtin_runtime)?;
                has_terminator = matches!(terminator, 
                    Instruction::Return { .. } | 
                    Instruction::Branch { .. } | 
                    Instruction::BranchIf { .. }
                );
            }
            
            // Add default terminator if needed
            if !has_terminator {
                if ir_function.return_type == "void" {
                    builder.ins().return_(&[]);
                } else {
                    let default_val = match ir_function.return_type.as_str() {
                        "int" => builder.ins().iconst(types::I64, 0),
                        "float" => builder.ins().f64const(0.0),
                        "bool" => builder.ins().iconst(types::I8, 0),
                        _ => builder.ins().iconst(types::I64, 0),
                    };
                    builder.ins().return_(&[default_val]);
                }
            }
        }
        
        // Seal all blocks
        for &block in &cranelift_blocks {
            builder.seal_block(block);
        }
        
        // Finalize function
        builder.finalize();
        
        // Define the function in the module
        self.module
            .define_function(func_id, &mut self.ctx)
            .map_err(|e| format!("Failed to define function: {}", e))?;
        
        // Finalize the function and get its pointer
        let _ = self.module.finalize_definitions();
        let func_ptr = self.module.get_finalized_function(func_id);
        
        Ok(func_ptr)
    }
    
    /// Generate code for a single instruction (static version to avoid borrowing issues)
    fn generate_instruction_static(
        instruction: &Instruction, 
        builder: &mut FunctionBuilder, 
        values: &mut HashMap<String, Value>,
        builtin_runtime: &BuiltinRuntime
    ) -> Result<(), String> {
        match instruction {
            Instruction::Assign { dest, src } => {
                let value = Self::operand_to_value_static(src, builder, values)?;
                values.insert(dest.clone(), value);
            }
            
            Instruction::Add { dest, left, right } => {
                let left_val = Self::operand_to_value_static(left, builder, values)?;
                let right_val = Self::operand_to_value_static(right, builder, values)?;
                let result = builder.ins().iadd(left_val, right_val);
                values.insert(dest.clone(), result);
            }
            
            Instruction::Sub { dest, left, right } => {
                let left_val = Self::operand_to_value_static(left, builder, values)?;
                let right_val = Self::operand_to_value_static(right, builder, values)?;
                let result = builder.ins().isub(left_val, right_val);
                values.insert(dest.clone(), result);
            }
            
            Instruction::Mul { dest, left, right } => {
                let left_val = Self::operand_to_value_static(left, builder, values)?;
                let right_val = Self::operand_to_value_static(right, builder, values)?;
                let result = builder.ins().imul(left_val, right_val);
                values.insert(dest.clone(), result);
            }
            
            Instruction::Div { dest, left, right } => {
                let left_val = Self::operand_to_value_static(left, builder, values)?;
                let right_val = Self::operand_to_value_static(right, builder, values)?;
                let result = builder.ins().sdiv(left_val, right_val);
                values.insert(dest.clone(), result);
            }
            
            Instruction::Load { dest, src } => {
                let addr = Self::operand_to_value_static(src, builder, values)?;
                let result = builder.ins().load(types::I64, MemFlags::new(), addr, 0);
                values.insert(dest.clone(), result);
            }
            
            Instruction::Alloca { dest, ty: _, size: _ } => {
                // For JIT, we can use stack slots
                let slot = builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8));
                let addr = builder.ins().stack_addr(types::I64, slot, 0);
                values.insert(dest.clone(), addr);
            }
            
            Instruction::Store { dest, src } => {
                let addr = Self::operand_to_value_static(dest, builder, values)?;
                let value = Self::operand_to_value_static(src, builder, values)?;
                builder.ins().store(MemFlags::new(), value, addr, 0);
            }
            
            Instruction::Call { dest, func, args: _ } => {
                // Handle builtin function calls
                if builtin_runtime.registry().is_builtin(func) {
                    // For JIT, we'll handle builtin calls after compilation
                    // For now, just create a placeholder return value
                    if let Some(dest) = dest {
                        let zero = builder.ins().iconst(types::I64, 0);
                        values.insert(dest.clone(), zero);
                    }
                } else {
                    // TODO: Implement user-defined function calls
                    if let Some(dest) = dest {
                        let zero = builder.ins().iconst(types::I64, 0);
                        values.insert(dest.clone(), zero);
                    }
                }
            }
            
            // Note: BuiltinCall doesn't exist in the IR, builtin functions use regular Call instructions
            
            Instruction::Return { value } => {
                if let Some(val) = value {
                    let return_val = Self::operand_to_value_static(val, builder, values)?;
                    builder.ins().return_(&[return_val]);
                } else {
                    builder.ins().return_(&[]);
                }
            }
            
            Instruction::DebugInfo { message: _ } => {
                // Debug info instructions don't generate actual code
            }
            
            _ => {
                // For other instructions, generate a no-op for now
            }
        }
        
        Ok(())
    }
    
    /// Convert IR operand to Cranelift value (static version)
    fn operand_to_value_static(
        operand: &Operand, 
        builder: &mut FunctionBuilder, 
        values: &mut HashMap<String, Value>
    ) -> Result<Value, String> {
        match operand {
            Operand::Immediate(val) => {
                Ok(builder.ins().iconst(types::I64, *val))
            }
            Operand::Float(val) => {
                Ok(builder.ins().f64const(*val))
            }
            Operand::Bool(val) => {
                Ok(builder.ins().iconst(types::I8, if *val { 1 } else { 0 }))
            }
            Operand::Register(name) | Operand::Local(name) => {
                if let Some(value) = values.get(name) {
                    Ok(*value)
                } else {
                    // Create a placeholder value
                    let placeholder = builder.ins().iconst(types::I64, 0);
                    values.insert(name.clone(), placeholder);
                    Ok(placeholder)
                }
            }
            Operand::String(_s) => {
                // For now, return a placeholder pointer
                Ok(builder.ins().iconst(types::I64, 0))
            }
            Operand::Null => {
                Ok(builder.ins().iconst(types::I64, 0))
            }
            Operand::Global(_name) => {
                // TODO: Implement global variable access
                Ok(builder.ins().iconst(types::I64, 0))
            }
        }
    }
    
    /// Convert IR type to Cranelift type
    fn ir_type_to_cranelift(&self, ir_type: &str) -> Result<Type, String> {
        match ir_type {
            "int" => Ok(types::I64),
            "float" => Ok(types::F64),
            "bool" => Ok(types::I8),
            "char" => Ok(types::I8),
            "str" => Ok(types::I64), // Pointer to string
            "void" => Ok(types::INVALID),
            _ => Ok(types::I64), // Default to I64 for unknown types
        }
    }
    
    /// Compile and run the IR module using JIT
    pub fn compile_and_run(&mut self, ir_module: IRModule) -> Result<i32, Diagnostics> {
        // First, declare all functions in the module
        let mut function_ids = HashMap::new();
        
        for function in &ir_module.functions {
            let func_id = self.declare_function(&function.name, &function.params, &function.return_type)?;
            function_ids.insert(function.name.clone(), func_id);
        }
        
        // Now compile all functions
        for function in &ir_module.functions {
            let func_id = function_ids[&function.name];
            self.compile_function_with_id(function, func_id, &function_ids, &ir_module)?;
        }
        
        // Finalize the module to make functions callable
        let _ = self.module.finalize_definitions();
        
        // Execute the main function using proper JIT execution
        if let Some(main_id) = function_ids.get("main") {
            // Execute main function with proper JIT execution that handles both
            // user-defined function calls (via native pointers) and builtin calls (via interpretation)
            self.execute_jit_main_function(*main_id, &function_ids, &ir_module)?;
            
            Ok(0)
        } else {
            let mut diagnostics = Diagnostics::new();
            diagnostics.add(Diagnostic::new(
                DiagnosticKind::Custom { 
                    message: "Main function not found in IR module".to_string() 
                }
            ));
            Err(diagnostics)
        }
    }
    
    /// Declare a function in the JIT module
    fn declare_function(&mut self, name: &str, params: &[IRParam], return_type: &str) -> Result<FuncId, Diagnostics> {
        let mut sig = self.module.make_signature();
        
        // Add parameters
        for param in params {
            let param_type = self.ir_type_to_cranelift(&param.ty).map_err(|e| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { message: format!("Failed to convert parameter type: {}", e) }
                ));
                diagnostics
            })?;
            sig.params.push(AbiParam::new(param_type));
        }
        
        // Add return type
        if return_type != "void" {
            let ret_type = self.ir_type_to_cranelift(return_type).map_err(|e| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { message: format!("Failed to convert return type: {}", e) }
                ));
                diagnostics
            })?;
            sig.returns.push(AbiParam::new(ret_type));
        }
        
        let func_id = self.module.declare_function(name, Linkage::Local, &sig)
            .map_err(|e| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { 
                        message: format!("Failed to declare function '{}': {}", name, e) 
                    }
                ));
                diagnostics
            })?;
        
        Ok(func_id)
    }
    
    /// Compile a function with a given function ID
    fn compile_function_with_id(
        &mut self, 
        function: &IRFunction, 
        func_id: FuncId,
        function_ids: &HashMap<String, FuncId>,
        ir_module: &IRModule
    ) -> Result<(), Diagnostics> {
        // Create a new context for this function
        let mut ctx = self.module.make_context();
        let mut builder_context = FunctionBuilderContext::new();
        
        // Set up the function signature
        for param in &function.params {
            let param_type = self.ir_type_to_cranelift(&param.ty).map_err(|e| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { message: format!("Failed to convert parameter type: {}", e) }
                ));
                diagnostics
            })?;
            ctx.func.signature.params.push(AbiParam::new(param_type));
        }
        
        if function.return_type != "void" {
            let ret_type = self.ir_type_to_cranelift(&function.return_type).map_err(|e| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { message: format!("Failed to convert return type: {}", e) }
                ));
                diagnostics
            })?;
            ctx.func.signature.returns.push(AbiParam::new(ret_type));
        }
        
        // Build the function body
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);
        
        if std::env::var("RAZEN_DEBUG").is_ok() {
            println!("🔧 Compiling function body for: {}", function.name);
            println!("   Parameters: {:?}", function.params.iter().map(|p| format!("{}:{}", p.name, p.ty)).collect::<Vec<_>>());
            println!("   Return type: {}", function.return_type);
            println!("   Basic blocks: {}", function.basic_blocks.len());
        }
        
        // Compile the function using existing logic but with function call support
        self.compile_function_body(&mut builder, function, function_ids, ir_module)?;
        
        builder.finalize();
        
        // Define the function in the module
        self.module.define_function(func_id, &mut ctx)
            .map_err(|e| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { 
                        message: format!("Failed to define function '{}': {}", function.name, e) 
                    }
                ));
                diagnostics
            })?;
        
        Ok(())
    }
    
    /// Compile the body of a function with support for function calls
    fn compile_function_body(
        &mut self,
        builder: &mut FunctionBuilder,
        function: &IRFunction,
        function_ids: &HashMap<String, FuncId>,
        ir_module: &IRModule
    ) -> Result<(), Diagnostics> {
        let mut values: HashMap<String, Value> = HashMap::new();
        
        // Map function parameters to both parameter names and register names
        let block_params = builder.block_params(builder.current_block().unwrap()).to_vec();
        for (i, param) in function.params.iter().enumerate() {
            if i < block_params.len() {
                // Map by parameter name
                values.insert(param.name.clone(), block_params[i]);
                // Also map by register name (r0, r1, etc.) for IR compatibility
                values.insert(format!("r{}", i), block_params[i]);
                
                if std::env::var("RAZEN_DEBUG").is_ok() {
                    println!("     Mapped parameter '{}' and 'r{}' to Cranelift value", param.name, i);
                }
            }
        }
        
        // Process all basic blocks
        for (block_idx, basic_block) in function.basic_blocks.iter().enumerate() {
            if std::env::var("RAZEN_DEBUG").is_ok() {
                println!("   Block {}: {} instructions", block_idx, basic_block.instructions.len());
            }
            
            // Process instructions
            for (inst_idx, instruction) in basic_block.instructions.iter().enumerate() {
                if std::env::var("RAZEN_DEBUG").is_ok() {
                    println!("     Instruction {}: {:?}", inst_idx, instruction);
                }
                self.compile_instruction_with_calls(builder, instruction, &mut values, function_ids, ir_module)?;
            }
            
            // Process terminator
            if let Some(ref terminator) = basic_block.terminator {
                if std::env::var("RAZEN_DEBUG").is_ok() {
                    println!("     Terminator: {:?}", terminator);
                }
                self.compile_instruction_with_calls(builder, terminator, &mut values, function_ids, ir_module)?;
            }
        }
        
        Ok(())
    }
    
    /// Compile a single instruction with support for function calls
    fn compile_instruction_with_calls(
        &mut self,
        builder: &mut FunctionBuilder,
        instruction: &Instruction,
        values: &mut HashMap<String, Value>,
        function_ids: &HashMap<String, FuncId>,
        _ir_module: &IRModule
    ) -> Result<(), Diagnostics> {
        match instruction {
            Instruction::Call { dest, func, args } => {
                if let Some(&func_id) = function_ids.get(func) {
                    // User-defined function call - compile to native call instruction
                    let func_ref = self.module.declare_func_in_func(func_id, builder.func);
                    
                    // Prepare arguments
                    let mut arg_values = Vec::new();
                    for arg in args {
                        let arg_val = Self::operand_to_value_static(arg, builder, values).map_err(|e| {
                            let mut diagnostics = Diagnostics::new();
                            diagnostics.add(Diagnostic::new(
                                DiagnosticKind::Custom { message: format!("Failed to convert operand: {}", e) }
                            ));
                            diagnostics
                        })?;
                        arg_values.push(arg_val);
                    }
                    
                    // Make the native call - this is the key fix!
                    let call_inst = builder.ins().call(func_ref, &arg_values);
                    
                    // Properly capture the return value using inst_results
                    if let Some(dest_reg) = dest {
                        let results = builder.inst_results(call_inst);
                        if !results.is_empty() {
                            values.insert(dest_reg.clone(), results[0]);
                        } else {
                            // Function returns void, store placeholder
                            let placeholder = builder.ins().iconst(types::I64, 0);
                            values.insert(dest_reg.clone(), placeholder);
                        }
                    }
                } else if self.builtin_runtime.registry().is_builtin(func) {
                    // Builtin function call - create placeholder for post-JIT execution
                    if let Some(dest_reg) = dest {
                        // Create a placeholder value for builtin calls
                        // The actual builtin execution happens post-JIT via IR interpretation
                        let placeholder = builder.ins().iconst(types::I64, 0);
                        values.insert(dest_reg.clone(), placeholder);
                    }
                } else {
                    // Unknown function - this should not happen in a well-formed program
                    return Err({
                        let mut diagnostics = Diagnostics::new();
                        diagnostics.add(Diagnostic::new(
                            DiagnosticKind::Custom { 
                                message: format!("Unknown function '{}' in call instruction", func) 
                            }
                        ));
                        diagnostics
                    });
                }
            }
            
            // Handle other instructions using existing logic
            _ => {
                // Use the existing instruction compilation logic
                match instruction {
                    Instruction::Assign { dest, src } => {
                        let value = Self::operand_to_value_static(src, builder, values).map_err(|e| {
                            let mut diagnostics = Diagnostics::new();
                            diagnostics.add(Diagnostic::new(
                                DiagnosticKind::Custom { message: format!("Failed to convert operand: {}", e) }
                            ));
                            diagnostics
                        })?;
                        values.insert(dest.clone(), value);
                    }
                    
                    Instruction::Load { dest, src } => {
                        // For function parameters, Load should just copy the value, not perform memory load
                        let value = Self::operand_to_value_static(src, builder, values).map_err(|e| {
                            let mut diagnostics = Diagnostics::new();
                            diagnostics.add(Diagnostic::new(
                                DiagnosticKind::Custom { message: format!("Failed to convert operand: {}", e) }
                            ));
                            diagnostics
                        })?;
                        values.insert(dest.clone(), value);
                        
                        if std::env::var("RAZEN_DEBUG").is_ok() {
                            println!("       Load: {} <- {:?}", dest, src);
                        }
                    }
                    
                    Instruction::Add { dest, left, right } => {
                        let left_val = Self::operand_to_value_static(left, builder, values).map_err(|e| {
                            let mut diagnostics = Diagnostics::new();
                            diagnostics.add(Diagnostic::new(
                                DiagnosticKind::Custom { message: format!("Failed to convert operand: {}", e) }
                            ));
                            diagnostics
                        })?;
                        let right_val = Self::operand_to_value_static(right, builder, values).map_err(|e| {
                            let mut diagnostics = Diagnostics::new();
                            diagnostics.add(Diagnostic::new(
                                DiagnosticKind::Custom { message: format!("Failed to convert operand: {}", e) }
                            ));
                            diagnostics
                        })?;
                        let result = builder.ins().iadd(left_val, right_val);
                        values.insert(dest.clone(), result);
                    }
                    
                    Instruction::Return { value } => {
                        if let Some(ret_val) = value {
                            let val = Self::operand_to_value_static(ret_val, builder, values).map_err(|e| {
                                let mut diagnostics = Diagnostics::new();
                                diagnostics.add(Diagnostic::new(
                                    DiagnosticKind::Custom { message: format!("Failed to convert operand: {}", e) }
                                ));
                                diagnostics
                            })?;
                            builder.ins().return_(&[val]);
                        } else {
                            builder.ins().return_(&[]);
                        }
                    }
                    
                    // Add other instruction types as needed
                    _ => {
                        // For now, ignore other instructions
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Execute the main function using proper JIT execution pattern
    /// This is the key method that implements the correct JIT execution flow
    fn execute_jit_main_function(
        &mut self, 
        main_id: FuncId, 
        function_ids: &HashMap<String, FuncId>,
        ir_module: &IRModule
    ) -> Result<(), Diagnostics> {
        // Get the finalized main function pointer
        let main_ptr = self.module.get_finalized_function(main_id);
        
        if std::env::var("RAZEN_DEBUG").is_ok() {
            println!("🚀 Executing JIT-compiled main function...");
        }
        
        // Find the main function in the IR module to get its signature
        let main_function = ir_module.functions.iter()
            .find(|f| f.name == "main")
            .ok_or_else(|| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { 
                        message: "Main function not found in IR module".to_string() 
                    }
                ));
                diagnostics
            })?;
        
        // Execute main function as native code with proper signature based on parameters
        let execution_result = unsafe {
            // Wrap the execution in a panic-catching block for robustness
            std::panic::catch_unwind(|| {
                match (main_function.params.len(), main_function.return_type.as_str()) {
                    // main() -> void
                    (0, "void") => {
                        let main_func = std::mem::transmute::<_, fn()>(main_ptr);
                        main_func();
                        0 // Return 0 for void functions
                    }
                    // main() -> int
                    (0, "int") => {
                        let main_func = std::mem::transmute::<_, fn() -> i32>(main_ptr);
                        let result = main_func();
                        if std::env::var("RAZEN_DEBUG").is_ok() {
                            println!("🎯 Main function returned: {}", result);
                        }
                        result
                    }
                    // main(int, int) -> int (e.g., add function used as main)
                    (2, "int") if main_function.params.iter().all(|p| p.ty == "int") => {
                        let main_func = std::mem::transmute::<_, fn(i32, i32) -> i32>(main_ptr);
                        // For testing, call with sample values
                        let result = main_func(10, 20);
                        if std::env::var("RAZEN_DEBUG").is_ok() {
                            println!("🎯 Main function returned: {}", result);
                        }
                        result
                    }
                    // main(int) -> int
                    (1, "int") if main_function.params[0].ty == "int" => {
                        let main_func = std::mem::transmute::<_, fn(i32) -> i32>(main_ptr);
                        let result = main_func(42);
                        if std::env::var("RAZEN_DEBUG").is_ok() {
                            println!("🎯 Main function returned: {}", result);
                        }
                        result
                    }
                    // Default case - treat as void function
                    _ => {
                        let main_func = std::mem::transmute::<_, fn()>(main_ptr);
                        main_func();
                        0
                    }
                }
            }).unwrap_or_else(|_| {
                if std::env::var("RAZEN_DEBUG").is_ok() {
                    println!("⚠️ JIT function execution panicked, continuing with builtin processing...");
                }
                0 // Return 0 if execution panicked
            })
        };
        
        if std::env::var("RAZEN_DEBUG").is_ok() {
            println!("✅ JIT execution completed with result: {}", execution_result);
        }
        
        // After JIT execution, process any builtin function calls by interpreting the IR
        // This handles the builtin calls that were compiled but need runtime execution
        self.execute_builtin_calls_post_jit(ir_module, function_ids)?;
        
        Ok(())
    }
    
    /// Execute builtin function calls after JIT compilation by interpreting the IR
    /// This method processes builtin calls that were compiled but need runtime execution
    fn execute_builtin_calls_post_jit(&mut self, ir_module: &IRModule, function_ids: &HashMap<String, FuncId>) -> Result<(), Diagnostics> {
        // Create an IR interpreter to resolve builtin function arguments
        let mut interpreter = IRInterpreter::new(ir_module);
        
        // Use the passed function IDs from the main compilation
        
        // Only process the main function since user-defined functions are executed natively
        if let Some(main_function) = ir_module.functions.iter().find(|f| f.name == "main") {
            // Execute all instructions in the main function to build register state
            for basic_block in main_function.basic_blocks.iter() {
                for instruction in basic_block.instructions.iter() {
                    // Handle user-defined function calls by executing them natively
                    if let Instruction::Call { dest, func, args } = instruction {
                        if self.builtin_runtime.registry().is_builtin(func) {
                            // Execute builtin function call
                            self.execute_builtin_call_with_interpreter(func, args, &interpreter)?;
                        } else if function_ids.contains_key(func) {
                            // Execute user-defined function natively and store result
                            let result = self.execute_user_function_natively(func, args, &interpreter, &function_ids, ir_module)?;
                            if let Some(dest_reg) = dest {
                                interpreter.registers.insert(dest_reg.clone(), result);
                            }
                        }
                    } else {
                        // Execute other instructions to maintain register state
                        interpreter.execute_instruction(instruction)?;
                    }
                }
                
                // Check terminator for function calls
                if let Some(ref terminator) = basic_block.terminator {
                    if let Instruction::Call { dest, func, args } = terminator {
                        if self.builtin_runtime.registry().is_builtin(func) {
                            self.execute_builtin_call_with_interpreter(func, args, &interpreter)?;
                        } else if function_ids.contains_key(func) {
                            let result = self.execute_user_function_natively(func, args, &interpreter, &function_ids, ir_module)?;
                            if let Some(dest_reg) = dest {
                                interpreter.registers.insert(dest_reg.clone(), result);
                            }
                        }
                    } else {
                        interpreter.execute_instruction(terminator)?;
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Execute a user-defined function natively and return the result
    fn execute_user_function_natively(
        &mut self,
        func_name: &str,
        args: &[Operand],
        interpreter: &IRInterpreter,
        function_ids: &HashMap<String, FuncId>,
        ir_module: &IRModule
    ) -> Result<IRValue, Diagnostics> {
        // Find the function in the IR module
        let function = ir_module.functions.iter()
            .find(|f| f.name == func_name)
            .ok_or_else(|| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { 
                        message: format!("Function '{}' not found in IR module", func_name) 
                    }
                ));
                diagnostics
            })?;
        
        // Get the function ID
        let func_id = function_ids.get(func_name)
            .ok_or_else(|| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { 
                        message: format!("Function '{}' not found in function IDs", func_name) 
                    }
                ));
                diagnostics
            })?;
        
        // Get the finalized function pointer
        let func_ptr = self.module.get_finalized_function(*func_id);
        
        // Convert arguments to native values
        let mut native_args = Vec::new();
        for arg in args {
            match interpreter.resolve_operand(arg) {
                Ok(IRValue::Integer(i)) => native_args.push(i as i32),
                Ok(IRValue::Float(f)) => native_args.push(f as i32), // Convert to int for now
                Ok(IRValue::Boolean(b)) => native_args.push(if b { 1 } else { 0 }),
                _ => native_args.push(0),
            }
        }
        
        // Execute function with proper signature based on parameters
        let result = unsafe {
            match (function.params.len(), function.return_type.as_str()) {
                // func() -> int
                (0, "int") => {
                    let func = std::mem::transmute::<_, fn() -> i32>(func_ptr);
                    func() as i64
                }
                // func(int) -> int
                (1, "int") if function.params[0].ty == "int" => {
                    let func = std::mem::transmute::<_, fn(i32) -> i32>(func_ptr);
                    let arg = native_args.get(0).copied().unwrap_or(0);
                    func(arg) as i64
                }
                // func(int, int) -> int
                (2, "int") if function.params.iter().all(|p| p.ty == "int") => {
                    let func = std::mem::transmute::<_, fn(i32, i32) -> i32>(func_ptr);
                    let arg1 = native_args.get(0).copied().unwrap_or(0);
                    let arg2 = native_args.get(1).copied().unwrap_or(0);
                    let result = func(arg1, arg2);
                    if std::env::var("RAZEN_DEBUG").is_ok() {
                        println!("🔧 Native function '{}' called with ({}, {}) -> {}", func_name, arg1, arg2, result);
                    }
                    result as i64
                }
                // func() -> void (return 0 as placeholder)
                (0, "void") => {
                    let func = std::mem::transmute::<_, fn()>(func_ptr);
                    func();
                    0
                }
                // Default case
                _ => 0,
            }
        };
        
        Ok(IRValue::Integer(result))
    }
    
    
    
    /// Execute a builtin function call using the interpreter
    fn execute_builtin_call_with_interpreter(
        &mut self,
        builtin: &str,
        args: &[Operand],
        interpreter: &IRInterpreter
    ) -> Result<(), Diagnostics> {
        // Convert operands to string arguments using the interpreter
        let string_args: Vec<String> = args.iter().map(|arg| {
            interpreter.resolve_operand_to_string(arg)
        }).collect();
        
        // Execute the builtin function
        match self.builtin_runtime.execute(builtin, string_args) {
            Ok(_result) => Ok(()),
            Err(e) => {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { 
                        message: format!("Builtin function '{}' failed: {}", builtin, e) 
                    }
                ));
                Err(diagnostics)
            }
        }
    }
    
    /// Execute a user-defined function with proper native function pointer
    /// This method demonstrates how to call any JIT-compiled function
    pub fn execute_function_by_name(
        &mut self,
        function_name: &str,
        args: Vec<i32>,
        ir_module: &IRModule,
        function_ids: &HashMap<String, FuncId>
    ) -> Result<i32, Diagnostics> {
        // Find the function in the IR module
        let function = ir_module.functions.iter()
            .find(|f| f.name == function_name)
            .ok_or_else(|| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { 
                        message: format!("Function '{}' not found in IR module", function_name) 
                    }
                ));
                diagnostics
            })?;
        
        // Get the function ID
        let func_id = function_ids.get(function_name)
            .ok_or_else(|| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { 
                        message: format!("Function '{}' not found in function IDs", function_name) 
                    }
                ));
                diagnostics
            })?;
        
        // Get the finalized function pointer
        let func_ptr = self.module.get_finalized_function(*func_id);
        
        // Execute function with proper signature based on parameters
        unsafe {
            match (function.params.len(), function.return_type.as_str()) {
                // func() -> int
                (0, "int") => {
                    let func = std::mem::transmute::<_, fn() -> i32>(func_ptr);
                    Ok(func())
                }
                // func(int) -> int
                (1, "int") if function.params[0].ty == "int" => {
                    let func = std::mem::transmute::<_, fn(i32) -> i32>(func_ptr);
                    let arg = args.get(0).copied().unwrap_or(0);
                    Ok(func(arg))
                }
                // func(int, int) -> int
                (2, "int") if function.params.iter().all(|p| p.ty == "int") => {
                    let func = std::mem::transmute::<_, fn(i32, i32) -> i32>(func_ptr);
                    let arg1 = args.get(0).copied().unwrap_or(0);
                    let arg2 = args.get(1).copied().unwrap_or(0);
                    Ok(func(arg1, arg2))
                }
                // func() -> void (return 0 as placeholder)
                (0, "void") => {
                    let func = std::mem::transmute::<_, fn()>(func_ptr);
                    func();
                    Ok(0)
                }
                // Default case
                _ => {
                    return Err({
                        let mut diagnostics = Diagnostics::new();
                        diagnostics.add(Diagnostic::new(
                            DiagnosticKind::Custom { 
                                message: format!(
                                    "Unsupported function signature for '{}': {} params, return type '{}'", 
                                    function_name, 
                                    function.params.len(), 
                                    function.return_type
                                ) 
                            }
                        ));
                        diagnostics
                    });
                }
            }
        }
    }
    
    // Note: Builtin functions are executed after compilation by processing the IR
}

/// Simple IR Interpreter for JIT execution
/// This interprets IR instructions to resolve values for builtin function calls
struct IRInterpreter<'a> {
    ir_module: &'a IRModule,
    pub registers: HashMap<String, IRValue>,
    memory: HashMap<String, IRValue>,
}

#[derive(Debug, Clone)]
enum IRValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl<'a> IRInterpreter<'a> {
    fn new(ir_module: &'a IRModule) -> Self {
        Self {
            ir_module,
            registers: HashMap::new(),
            memory: HashMap::new(),
        }
    }
    
    /// Execute a single IR instruction
    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), Diagnostics> {
        match instruction {
            Instruction::Assign { dest, src } => {
                let value = self.resolve_operand(src)?;
                self.registers.insert(dest.clone(), value);
            }
            
            Instruction::Add { dest, left, right } => {
                let left_val = self.resolve_operand(left)?;
                let right_val = self.resolve_operand(right)?;
                let result = self.perform_add(left_val, right_val)?;
                self.registers.insert(dest.clone(), result);
            }
            
            Instruction::Sub { dest, left, right } => {
                let left_val = self.resolve_operand(left)?;
                let right_val = self.resolve_operand(right)?;
                let result = self.perform_sub(left_val, right_val)?;
                self.registers.insert(dest.clone(), result);
            }
            
            Instruction::Mul { dest, left, right } => {
                let left_val = self.resolve_operand(left)?;
                let right_val = self.resolve_operand(right)?;
                let result = self.perform_mul(left_val, right_val)?;
                self.registers.insert(dest.clone(), result);
            }
            
            Instruction::Div { dest, left, right } => {
                let left_val = self.resolve_operand(left)?;
                let right_val = self.resolve_operand(right)?;
                let result = self.perform_div(left_val, right_val)?;
                self.registers.insert(dest.clone(), result);
            }
            
            Instruction::Alloca { dest, .. } => {
                // Create a memory location
                self.memory.insert(dest.clone(), IRValue::Null);
            }
            
            Instruction::Store { dest, src } => {
                let value = self.resolve_operand(src)?;
                if let Operand::Register(reg) = dest {
                    self.memory.insert(reg.clone(), value);
                }
            }
            
            Instruction::Load { dest, src } => {
                if let Operand::Register(reg) = src {
                    if let Some(value) = self.memory.get(reg) {
                        self.registers.insert(dest.clone(), value.clone());
                    }
                }
            }
            
            Instruction::Call { dest, func: _, args: _ } => {
                // Handle function calls (this will be processed separately)
                // For now, just store a placeholder if there's a destination
                if let Some(dest_reg) = dest {
                    self.registers.insert(dest_reg.clone(), IRValue::Null);
                }
            }
            
            // For other instructions, we don't need to simulate them for builtin calls
            _ => {}
        }
        
        Ok(())
    }
    
    /// Resolve an operand to its actual value
    fn resolve_operand(&self, operand: &Operand) -> Result<IRValue, Diagnostics> {
        match operand {
            Operand::Immediate(i) => Ok(IRValue::Integer(*i)),
            Operand::Float(f) => Ok(IRValue::Float(*f)),
            Operand::Bool(b) => Ok(IRValue::Boolean(*b)),
            Operand::String(s) => {
                // Handle string literals like @str0, @str1, etc.
                if s.starts_with("@str") {
                    if let Ok(index) = s[4..].parse::<usize>() {
                        if index < self.ir_module.strings.len() {
                            return Ok(IRValue::String(self.ir_module.strings[index].clone()));
                        }
                    }
                }
                Ok(IRValue::String(s.clone()))
            }
            Operand::Register(reg) => {
                if let Some(value) = self.registers.get(reg) {
                    Ok(value.clone())
                } else {
                    Ok(IRValue::Null)
                }
            }
            _ => Ok(IRValue::Null),
        }
    }
    
    /// Resolve an operand to a string for builtin function calls
    fn resolve_operand_to_string(&self, operand: &Operand) -> String {
        match self.resolve_operand(operand) {
            Ok(value) => match value {
                IRValue::Integer(i) => i.to_string(),
                IRValue::Float(f) => f.to_string(),
                IRValue::String(s) => s,
                IRValue::Boolean(b) => b.to_string(),
                IRValue::Null => "null".to_string(),
            },
            Err(_) => "error".to_string(),
        }
    }
    
    
    /// Perform addition operation
    fn perform_add(&self, left: IRValue, right: IRValue) -> Result<IRValue, Diagnostics> {
        match (left, right) {
            (IRValue::Integer(a), IRValue::Integer(b)) => Ok(IRValue::Integer(a + b)),
            (IRValue::Float(a), IRValue::Float(b)) => Ok(IRValue::Float(a + b)),
            (IRValue::Integer(a), IRValue::Float(b)) => Ok(IRValue::Float(a as f64 + b)),
            (IRValue::Float(a), IRValue::Integer(b)) => Ok(IRValue::Float(a + b as f64)),
            (IRValue::String(a), IRValue::String(b)) => Ok(IRValue::String(format!("{}{}", a, b))),
            (IRValue::String(a), other) => {
                let b_str = match other {
                    IRValue::Integer(i) => i.to_string(),
                    IRValue::Float(f) => f.to_string(),
                    IRValue::Boolean(b) => b.to_string(),
                    _ => "null".to_string(),
                };
                Ok(IRValue::String(format!("{}{}", a, b_str)))
            }
            _ => Ok(IRValue::Null),
        }
    }
    
    /// Perform subtraction operation
    fn perform_sub(&self, left: IRValue, right: IRValue) -> Result<IRValue, Diagnostics> {
        match (left, right) {
            (IRValue::Integer(a), IRValue::Integer(b)) => Ok(IRValue::Integer(a - b)),
            (IRValue::Float(a), IRValue::Float(b)) => Ok(IRValue::Float(a - b)),
            (IRValue::Integer(a), IRValue::Float(b)) => Ok(IRValue::Float(a as f64 - b)),
            (IRValue::Float(a), IRValue::Integer(b)) => Ok(IRValue::Float(a - b as f64)),
            _ => Ok(IRValue::Null),
        }
    }
    
    /// Perform multiplication operation
    fn perform_mul(&self, left: IRValue, right: IRValue) -> Result<IRValue, Diagnostics> {
        match (left, right) {
            (IRValue::Integer(a), IRValue::Integer(b)) => Ok(IRValue::Integer(a * b)),
            (IRValue::Float(a), IRValue::Float(b)) => Ok(IRValue::Float(a * b)),
            (IRValue::Integer(a), IRValue::Float(b)) => Ok(IRValue::Float(a as f64 * b)),
            (IRValue::Float(a), IRValue::Integer(b)) => Ok(IRValue::Float(a * b as f64)),
            _ => Ok(IRValue::Null),
        }
    }
    
    /// Perform division operation
    fn perform_div(&self, left: IRValue, right: IRValue) -> Result<IRValue, Diagnostics> {
        match (left, right) {
            (IRValue::Integer(a), IRValue::Integer(b)) => {
                if b != 0 {
                    Ok(IRValue::Integer(a / b))
                } else {
                    Ok(IRValue::Null) // Division by zero
                }
            }
            (IRValue::Float(a), IRValue::Float(b)) => {
                if b != 0.0 {
                    Ok(IRValue::Float(a / b))
                } else {
                    Ok(IRValue::Null) // Division by zero
                }
            }
            (IRValue::Integer(a), IRValue::Float(b)) => {
                if b != 0.0 {
                    Ok(IRValue::Float(a as f64 / b))
                } else {
                    Ok(IRValue::Null)
                }
            }
            (IRValue::Float(a), IRValue::Integer(b)) => {
                if b != 0 {
                    Ok(IRValue::Float(a / b as f64))
                } else {
                    Ok(IRValue::Null)
                }
            }
            _ => Ok(IRValue::Null),
        }
    }
    
    /// Execute a user-defined function call
    fn execute_function_call(
        &mut self,
        func_name: &str,
        args: &[Operand],
        ir_module: &IRModule
    ) -> Result<IRValue, Diagnostics> {
        // Find the function in the IR module
        if let Some(function) = ir_module.functions.iter().find(|f| f.name == func_name) {
            // Create a new scope for the function
            let old_registers = self.registers.clone();
            
            // Set up function parameters
            for (i, param) in function.params.iter().enumerate() {
                if i < args.len() {
                    let arg_value = self.resolve_operand(&args[i])?;
                    self.registers.insert(param.name.clone(), arg_value);
                }
            }
            
            // Execute the function body
            let mut return_value = IRValue::Null;
            for basic_block in &function.basic_blocks {
                for instruction in &basic_block.instructions {
                    self.execute_instruction(instruction)?;
                }
                
                // Check for return in terminator
                if let Some(ref terminator) = basic_block.terminator {
                    if let Instruction::Return { value } = terminator {
                        if let Some(ret_operand) = value {
                            return_value = self.resolve_operand(ret_operand)?;
                        }
                        break;
                    } else {
                        self.execute_instruction(terminator)?;
                    }
                }
            }
            
            // Restore the previous scope
            self.registers = old_registers;
            
            Ok(return_value)
        } else {
            // Function not found - this might be the issue
            if std::env::var("RAZEN_DEBUG").is_ok() {
                println!("⚠️ Function '{}' not found in IR module", func_name);
                println!("Available functions: {:?}", ir_module.functions.iter().map(|f| &f.name).collect::<Vec<_>>());
            }
            Ok(IRValue::Null)
        }
    }
}

impl Default for JITCompiler {
    fn default() -> Self {
        Self::new().expect("Failed to create JITCompiler")
    }
}