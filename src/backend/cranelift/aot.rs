// src/backend/cranelift/aot.rs

use std::collections::HashMap;
use std::fs;
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectModule, ObjectBuilder};
use cranelift_native;
use crate::backend::ir::{IRModule, IRFunction, Instruction, Operand};
use crate::backend::CompiledProgram;
use crate::frontend::diagnostics::{Diagnostics, Diagnostic, DiagnosticKind};

/// Professional AOT (Ahead-of-Time) Compiler using Cranelift for object file generation
pub struct AOTCompiler {
    /// Cranelift module for code generation
    module: ObjectModule,
    /// Function builder context
    ctx: codegen::Context,
    /// Variable mapping for current function
    variables: HashMap<String, Variable>,
    /// Value mapping for registers
    values: HashMap<String, Value>,
    /// Next variable ID
    next_var_id: usize,
}

impl AOTCompiler {
    pub fn new() -> Result<Self, String> {
        // Create target ISA
        let isa_builder = cranelift_native::builder()
            .map_err(|e| format!("Failed to create ISA builder: {}", e))?;
        let isa = isa_builder
            .finish(settings::Flags::new(settings::builder()))
            .map_err(|e| format!("Failed to create ISA: {}", e))?;
        
        // Create object module
        let module = ObjectModule::new(
            ObjectBuilder::new(
                isa,
                "razen_program".to_string(),
                cranelift_module::default_libcall_names(),
            ).map_err(|e| format!("Failed to create object builder: {}", e))?
        );
        
        Ok(AOTCompiler {
            module,
            ctx: codegen::Context::new(),
            variables: HashMap::new(),
            values: HashMap::new(),
            next_var_id: 0,
        })
    }
    
    /// Compile IR to native code using Cranelift AOT
    pub fn compile(&mut self, ir_module: IRModule) -> Result<CompiledProgram, Diagnostics> {
        // Generate code for each function
        for function in &ir_module.functions {
            if let Err(e) = self.generate_function(function) {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { 
                        message: format!("AOT compilation failed for function '{}': {}", function.name, e) 
                    }
                ));
                return Err(diagnostics);
            }
        }
        
        // Finalize the module
        let module = std::mem::replace(&mut self.module, ObjectModule::new(
            ObjectBuilder::new(
                cranelift_native::builder().unwrap().finish(settings::Flags::new(settings::builder())).unwrap(),
                "temp".to_string(),
                cranelift_module::default_libcall_names(),
            ).unwrap()
        ));
        let object_product = module.finish();
        let object_bytes = object_product.emit()
            .map_err(|_e| {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(Diagnostic::new(
                    DiagnosticKind::Custom { 
                        message: "Failed to emit object code".to_string() 
                    }
                ));
                diagnostics
            })?;
        
        Ok(CompiledProgram {
            bytecode: object_bytes,
            entry_point: 0,
            symbols: HashMap::new(), // TODO: Extract symbols from module
        })
    }
    
    /// Compile to object file
    pub fn compile_to_object(&mut self, ir_module: IRModule, output_path: &str) -> Result<(), String> {
        // Compile the IR module
        let compiled_program = self.compile(ir_module)
            .map_err(|diag| format!("Compilation failed: {}", diag.diagnostics.len()))?;
        
        // Write object file
        fs::write(output_path, &compiled_program.bytecode)
            .map_err(|e| format!("Failed to write object file '{}': {}", output_path, e))?;
        
        Ok(())
    }
    
    /// Generate code for a single function
    fn generate_function(&mut self, ir_function: &IRFunction) -> Result<(), String> {
        // Skip functions with no basic blocks
        if ir_function.basic_blocks.is_empty() {
            return Ok(());
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
                Self::generate_instruction_static(instruction, &mut builder, &mut self.values)?;
            }
            
            // Handle terminator
            let mut has_terminator = false;
            if let Some(ref terminator) = basic_block.terminator {
                Self::generate_instruction_static(terminator, &mut builder, &mut self.values)?;
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
        
        Ok(())
    }
    
    /// Generate code for a single instruction (static version to avoid borrowing issues)
    fn generate_instruction_static(
        instruction: &Instruction, 
        builder: &mut FunctionBuilder, 
        values: &mut HashMap<String, Value>
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
                // For AOT, create stack slots
                let slot = builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8));
                let addr = builder.ins().stack_addr(types::I64, slot, 0);
                values.insert(dest.clone(), addr);
            }
            
            Instruction::Store { dest, src } => {
                let addr = Self::operand_to_value_static(dest, builder, values)?;
                let value = Self::operand_to_value_static(src, builder, values)?;
                builder.ins().store(MemFlags::new(), value, addr, 0);
            }
            
            Instruction::Call { dest, func: _, args: _ } => {
                // For AOT compilation, function calls need to be resolved at link time
                if let Some(dest) = dest {
                    let zero = builder.ins().iconst(types::I64, 0);
                    values.insert(dest.clone(), zero);
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
}

impl Default for AOTCompiler {
    fn default() -> Self {
        Self::new().expect("Failed to create AOTCompiler")
    }
}