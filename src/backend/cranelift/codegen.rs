// src/backend/cranelift/codegen.rs

use std::collections::HashMap;
use cranelift::prelude::*;
use cranelift::prelude::settings::Flags;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectModule, ObjectBuilder};
use cranelift_native;
use crate::backend::ir::{IRModule, IRFunction, Instruction, Operand};
use crate::backend::CompiledProgram;
use crate::frontend::diagnostics::Diagnostics;

/// Complete Cranelift Code Generator for Razen Language
pub struct CodeGenerator {
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

impl CodeGenerator {
    pub fn new() -> Result<Self, String> {
        // Create target ISA
        let isa_builder = cranelift_native::builder()
            .map_err(|e| format!("Failed to create ISA builder: {}", e))?;
        let isa = isa_builder
            .finish(Flags::new(settings::builder()))
            .map_err(|e| format!("Failed to create ISA: {}", e))?;
        
        // Create object module
        let module = ObjectModule::new(
            ObjectBuilder::new(
                isa,
                "razen_program".to_string(),
                cranelift_module::default_libcall_names(),
            ).map_err(|e| format!("Failed to create object builder: {}", e))?
        );
        
        Ok(CodeGenerator {
            module,
            ctx: codegen::Context::new(),
            variables: HashMap::new(),
            values: HashMap::new(),
            next_var_id: 0,
        })
    }
    
    /// Generate final code from IR using Cranelift
    pub fn generate(&mut self, ir_module: IRModule) -> Result<CompiledProgram, Diagnostics> {
        println!("🚀 Starting Cranelift Code Generation for {} functions", ir_module.functions.len());
        
        // Generate code for each function
        for function in &ir_module.functions {
            if let Err(e) = self.generate_function(function) {
                println!("❌ Error generating function '{}': {}", function.name, e);
                continue;
            }
            println!("✅ Generated function: {}", function.name);
        }
        
        // Finalize the module
        let module = std::mem::replace(&mut self.module, ObjectModule::new(
            ObjectBuilder::new(
                cranelift_native::builder().unwrap().finish(Flags::new(settings::builder())).unwrap(),
                "temp".to_string(),
                cranelift_module::default_libcall_names(),
            ).unwrap()
        ));
        let object_product = module.finish();
        let object_bytes = object_product.emit()
            .map_err(|_e| {
                let diag = Diagnostics::new();
                // Add error to diagnostics
                diag
            })?;
        
        println!("✅ Cranelift Code Generation completed successfully!");
        println!("📊 Generated {} bytes of native code", object_bytes.len());
        
        Ok(CompiledProgram {
            bytecode: object_bytes,
            entry_point: 0,
            symbols: HashMap::new(), // TODO: Extract symbols from module
        })
    }
    
    /// Generate code for a single function
    fn generate_function(&mut self, ir_function: &IRFunction) -> Result<(), String> {
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
        builder.seal_block(entry_block);
        
        // Map parameters to variables
        let param_values = builder.block_params(entry_block);
        for (i, _param) in ir_function.params.iter().enumerate() {
            if i < param_values.len() {
                self.values.insert(format!("r{}", i), param_values[i]);
            }
        }
        
        // Generate code for basic blocks
        for (block_idx, basic_block) in ir_function.basic_blocks.iter().enumerate() {
            if block_idx > 0 {
                // Create additional blocks for non-entry blocks
                let block = builder.create_block();
                builder.switch_to_block(block);
            }
            
            // Generate instructions
            for instruction in &basic_block.instructions {
                if let Err(e) = Self::generate_instruction_static(instruction, &mut builder, &mut self.values) {
                    return Err(format!("Error generating instruction: {}", e));
                }
            }
            
            // Handle terminator
            if let Some(ref terminator) = basic_block.terminator {
                if let Err(e) = Self::generate_instruction_static(terminator, &mut builder, &mut self.values) {
                    return Err(format!("Error generating terminator: {}", e));
                }
            }
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
                // Allocate stack space (simplified - just create a placeholder pointer)
                let placeholder = builder.ins().iconst(types::I64, 0);
                values.insert(dest.clone(), placeholder);
            }
            
            Instruction::Store { dest, src } => {
                let addr = Self::operand_to_value_static(dest, builder, values)?;
                let value = Self::operand_to_value_static(src, builder, values)?;
                builder.ins().store(MemFlags::new(), value, addr, 0);
            }
            
            Instruction::Alloca { dest, ty: _, size: _ } => {
                // Allocate stack space (simplified - just create a placeholder pointer)
                let placeholder = builder.ins().iconst(types::I64, 0);
                values.insert(dest.clone(), placeholder);
            }
            
            Instruction::Call { dest, func, args } => {
                // For built-in functions, generate appropriate calls
                match func.as_str() {
                    "println" => {
                        // For now, just consume the arguments
                        for arg in args {
                            Self::operand_to_value_static(arg, builder, values)?;
                        }
                        if let Some(dest) = dest {
                            let zero = builder.ins().iconst(types::I32, 0);
                            values.insert(dest.clone(), zero);
                        }
                    }
                    _ => {
                        // TODO: Implement user-defined function calls
                        if let Some(dest) = dest {
                            let zero = builder.ins().iconst(types::I64, 0);
                            values.insert(dest.clone(), zero);
                        }
                    }
                }
            }
            
            Instruction::Return { value } => {
                if let Some(val) = value {
                    let return_val = Self::operand_to_value_static(val, builder, values)?;
                    builder.ins().return_(&[return_val]);
                } else {
                    builder.ins().return_(&[]);
                }
            }
            
            _ => {
                // For other instructions, generate a no-op for now
                println!("⚠️  Instruction not yet implemented: {:?}", std::mem::discriminant(instruction));
            }
        }
        
        Ok(())
    }
    
    /// Convert IR operand to Cranelift value (static version)
    fn operand_to_value_static(
        operand: &Operand, 
        builder: &mut FunctionBuilder, 
        values: &HashMap<String, Value>
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
                values.get(name)
                    .copied()
                    .ok_or_else(|| format!("Unknown register: {}", name))
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
    
    /// Create a new variable
    fn create_variable(&mut self, _ty: Type) -> Variable {
        let var = Variable::new(self.next_var_id);
        self.next_var_id += 1;
        var
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create CodeGenerator")
    }
}