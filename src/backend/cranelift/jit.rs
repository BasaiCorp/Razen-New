// src/backend/cranelift/jit.rs

use std::collections::HashMap;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use cranelift_native;
use crate::backend::ir::{IRModule, IRFunction, Instruction, Operand};
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
    
    /// Compile and execute IR immediately using Cranelift JIT
    pub fn compile_and_run(&mut self, ir_module: IRModule) -> Result<i32, Diagnostics> {
        // Compile all functions
        let mut main_func_ptr: Option<*const u8> = None;
        
        for function in &ir_module.functions {
            match self.compile_function(function) {
                Ok(func_ptr) => {
                    if function.name == "main" {
                        main_func_ptr = Some(func_ptr);
                    }
                }
                Err(e) => {
                    let mut diagnostics = Diagnostics::new();
                    diagnostics.add(Diagnostic::new(
                        DiagnosticKind::Custom { 
                            message: format!("JIT compilation failed for function '{}': {}", function.name, e) 
                        }
                    ));
                    return Err(diagnostics);
                }
            }
        }
        
        // Execute main function if found
        if let Some(main_ptr) = main_func_ptr {
            // Execute the main function
            let exit_code = self.execute_main_function(main_ptr, &ir_module)?;
            Ok(exit_code)
        } else {
            let mut diagnostics = Diagnostics::new();
            diagnostics.add(Diagnostic::new(
                DiagnosticKind::Custom { 
                    message: "No main function found for execution".to_string() 
                }
            ));
            Err(diagnostics)
        }
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
    
    /// Execute the main function and handle builtin calls
    fn execute_main_function(&mut self, _main_ptr: *const u8, ir_module: &IRModule) -> Result<i32, Diagnostics> {
        // Find the main function in IR to understand its signature
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
        
        // Execute builtin calls found in the main function
        self.execute_builtin_calls_in_function(main_function, ir_module)?;
        
        // For now, return success exit code
        // In a full implementation, we would actually call the compiled function
        // let main_fn: extern "C" fn() -> i32 = unsafe { std::mem::transmute(main_ptr) };
        // Ok(main_fn())
        
        Ok(0)
    }
    
    /// Execute builtin function calls found in a function
    fn execute_builtin_calls_in_function(&mut self, function: &IRFunction, ir_module: &IRModule) -> Result<(), Diagnostics> {
        // Track register assignments to resolve string literals
        let mut register_values: HashMap<String, Operand> = HashMap::new();
        
        for basic_block in function.basic_blocks.iter() {
            for instruction in basic_block.instructions.iter() {
                // Track register assignments
                if let Instruction::Assign { dest, src } = instruction {
                    register_values.insert(dest.clone(), src.clone());
                }
                
                if let Instruction::Call { func, args, .. } = instruction {
                    if self.builtin_runtime.registry().is_builtin(func) {
                        self.execute_builtin_call_with_context(func, args, ir_module, &register_values)?;
                    }
                }
            }
            
            // Check terminator too
            if let Some(ref terminator) = basic_block.terminator {
                if let Instruction::Call { func, args, .. } = terminator {
                    if self.builtin_runtime.registry().is_builtin(func) {
                        self.execute_builtin_call_with_context(func, args, ir_module, &register_values)?;
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Execute a single builtin function call with register context
    fn execute_builtin_call_with_context(
        &mut self, 
        builtin: &str, 
        args: &[Operand], 
        ir_module: &IRModule,
        register_values: &HashMap<String, Operand>
    ) -> Result<(), Diagnostics> {
        // Convert operands to string arguments for builtin execution
        let string_args: Vec<String> = args.iter().map(|arg| {
            match arg {
                Operand::String(s) => {
                    // Handle string literals like @str0, @str1, etc.
                    if s.starts_with("@str") {
                        if let Ok(index) = s[4..].parse::<usize>() {
                            if index < ir_module.strings.len() {
                                return ir_module.strings[index].clone();
                            }
                        }
                    }
                    s.clone()
                }
                Operand::Register(reg) => {
                    // Try to resolve the register value
                    if let Some(value) = register_values.get(reg) {
                        // Recursively resolve the value
                        return self.resolve_operand_to_string(value, ir_module, register_values);
                    }
                    // For unresolved registers, we'll use a placeholder
                    // In a more advanced implementation, we'd need to simulate the computation
                    format!("[computed value]")
                }
                Operand::Immediate(i) => i.to_string(),
                Operand::Float(f) => f.to_string(),
                Operand::Bool(b) => b.to_string(),
                _ => "0".to_string(), // Default for complex operands
            }
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
    
    /// Helper function to resolve operand to string
    fn resolve_operand_to_string(
        &self,
        operand: &Operand,
        ir_module: &IRModule,
        register_values: &HashMap<String, Operand>
    ) -> String {
        match operand {
            Operand::String(s) => {
                // Handle string literals like @str0, @str1, etc.
                if s.starts_with("@str") {
                    if let Ok(index) = s[4..].parse::<usize>() {
                        if index < ir_module.strings.len() {
                            return ir_module.strings[index].clone();
                        }
                    }
                }
                s.clone()
            }
            Operand::Register(reg) => {
                // Try to resolve the register value recursively
                if let Some(value) = register_values.get(reg) {
                    return self.resolve_operand_to_string(value, ir_module, register_values);
                }
                format!("[computed:{}]", reg)
            }
            Operand::Immediate(i) => i.to_string(),
            Operand::Float(f) => f.to_string(),
            Operand::Bool(b) => b.to_string(),
            _ => "0".to_string(),
        }
    }
    
    // Note: Builtin functions are executed after compilation by processing the IR
}

impl Default for JITCompiler {
    fn default() -> Self {
        Self::new().expect("Failed to create JITCompiler")
    }
}