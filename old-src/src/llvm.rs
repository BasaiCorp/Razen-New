use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module as LlvmModule;
use inkwell::passes::PassManager;
use inkwell::values::{FunctionValue, PointerValue, BasicMetadataValueEnum, BasicValueEnum, BasicValue};
use inkwell::types::{BasicTypeEnum, BasicMetadataTypeEnum, BasicType};
use inkwell::AddressSpace;
use std::collections::HashMap;

// Assuming your IR enum and Value enum are accessible via crate:: path
use crate::compiler::IR as RazenIR;
use crate::value::Value as RazenValue;

pub struct LlvmCompiler<'ctx> {
    pub context: &'ctx Context,
    pub module: LlvmModule<'ctx>,
    pub builder: Builder<'ctx>,
    fpm: Option<PassManager<FunctionValue<'ctx>>>,

    // Manages named values (variables, function parameters) in the current scope
    // Maps variable names to their LLVM PointerValue and the expected type
    variables: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>,
    // Keeps track of defined functions for calls
    functions: HashMap<String, FunctionValue<'ctx>>,
    
    // Stack for managing values during compilation
    value_stack: Vec<BasicValueEnum<'ctx>>,
}

impl<'ctx> LlvmCompiler<'ctx> {
    // Helper to get BasicTypeEnum from BasicValueEnum for inkwell 0.2.0
    fn get_basic_value_type(&self, value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, String> {
        match value {
            BasicValueEnum::ArrayValue(v) => Ok(v.get_type().as_basic_type_enum()),
            BasicValueEnum::IntValue(v) => Ok(v.get_type().as_basic_type_enum()),
            BasicValueEnum::FloatValue(v) => Ok(v.get_type().as_basic_type_enum()),
            BasicValueEnum::PointerValue(v) => Ok(v.get_type().as_basic_type_enum()),
            BasicValueEnum::StructValue(v) => Ok(v.get_type().as_basic_type_enum()),
            BasicValueEnum::VectorValue(v) => Ok(v.get_type().as_basic_type_enum()),
        }
    }
    
    // Helper to convert BasicValueEnum to BasicMetadataValueEnum
    fn basic_value_to_metadata(&self, value: BasicValueEnum<'ctx>) -> BasicMetadataValueEnum<'ctx> {
        match value {
            BasicValueEnum::IntValue(v) => v.into(),
            BasicValueEnum::FloatValue(v) => v.into(),
            BasicValueEnum::PointerValue(v) => v.into(),
            BasicValueEnum::ArrayValue(v) => v.into(),
            BasicValueEnum::StructValue(v) => v.into(),
            BasicValueEnum::VectorValue(v) => v.into(),
        }
    }

    pub fn new(context: &'ctx Context, module_name: &str, enable_optimizations: bool) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        let fpm = if enable_optimizations {
            let fpm_instance = PassManager::create(&module);
            // Common optimization passes
            fpm_instance.add_instruction_combining_pass();
            fpm_instance.add_reassociate_pass();
            fpm_instance.add_gvn_pass(); // Global Value Numbering
            fpm_instance.add_cfg_simplification_pass(); // Control Flow Graph Simplification
            fpm_instance.add_promote_memory_to_register_pass(); // Mem2Reg
            // Add more passes like dead code elimination, loop optimizations etc. as needed
            fpm_instance.initialize();
            Some(fpm_instance)
        } else {
            None
        };

        LlvmCompiler {
            context,
            module,
            builder,
            fpm,
            variables: HashMap::new(),
            functions: HashMap::new(),
            value_stack: Vec::new(),
        }
    }

    // --- Type Conversion --- 
    fn to_llvm_type(&self, razen_type: &RazenValue) -> BasicTypeEnum<'ctx> {
        // This is a simplified mapping. Real type mapping can be more complex,
        // especially with user-defined types, structs, etc.
        match razen_type {
            RazenValue::Int(_) => self.context.i64_type().into(),
            RazenValue::Float(_) => self.context.f64_type().into(),
            RazenValue::Bool(_) => self.context.bool_type().into(),
            RazenValue::String(_) => self.context.i8_type().ptr_type(AddressSpace::default()).into(), // char*
            RazenValue::Array(_) => {
                // For arrays, we'll use a pointer to the element type
                // This is a simplified approach - you might want to handle arrays differently
                self.context.i8_type().ptr_type(AddressSpace::default()).into()
            },
            RazenValue::Null => self.context.i8_type().ptr_type(AddressSpace::default()).into(), // void* or a specific null marker type
            _ => panic!("Unsupported RazenValue type for LLVM conversion: {:?}", razen_type),
        }
    }

    // --- Main Compilation Logic --- 
    pub fn compile_function(&mut self, name: &str, params: Vec<(&str, RazenValue)>, return_type: RazenValue, body_ir: &[RazenIR]) -> Result<FunctionValue<'ctx>, String> {
        let llvm_return_type = self.to_llvm_type(&return_type);
        let llvm_param_types: Vec<BasicMetadataTypeEnum<'ctx>> = params
            .iter()
            .map(|(_, razen_val)| self.to_llvm_type(razen_val).into())
            .collect();

        let fn_type = llvm_return_type.fn_type(&llvm_param_types, false);
        let function = self.module.add_function(name, fn_type, None);
        self.functions.insert(name.to_string(), function);

        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        // Create allocas for parameters and store initial values
        for (i, (param_name, param_type)) in params.iter().enumerate() {
            if let Some(param_value) = function.get_nth_param(i as u32) {
                let param_llvm_type = self.to_llvm_type(param_type);
                let alloca = self.create_entry_block_alloca(&format!("param_{}", param_name), param_llvm_type, function)?;
                self.builder.build_store(alloca, param_value);
                self.variables.insert(param_name.to_string(), (alloca, param_llvm_type));
            }
        }

        // Compile IR instructions for the function body
        for instruction in body_ir {
            self.compile_ir_instruction(instruction, function)?;
        }

        // Verify function
        if function.verify(true) {
            if let Some(fpm) = &self.fpm {
                fpm.run_on(&function);
            }
            Ok(function)
        } else {
            unsafe { function.delete(); }
            Err(format!("LLVM function '{}' verification failed.", name))
        }
    }

    fn compile_ir_instruction(&mut self, instruction: &RazenIR, current_function: FunctionValue<'ctx>) -> Result<(), String> {
        match instruction {
            RazenIR::PushNumber(val) => {
                let i64_val = self.context.i64_type().const_int(*val as u64, true);
                self.value_stack.push(i64_val.as_basic_value_enum());
                println!("[LLVM] Pushed number to stack: {}", val);
            }
            RazenIR::PushString(s) => {
                let str_val = self.builder.build_global_string_ptr(s, ".str");
                let str_ptr = str_val.as_pointer_value();
                self.value_stack.push(str_ptr.as_basic_value_enum());
                println!("[LLVM] Pushed string to stack: {}", s);
            }
            RazenIR::PushBoolean(val) => {
                let bool_val = self.context.bool_type().const_int(if *val { 1 } else { 0 }, false);
                self.value_stack.push(bool_val.as_basic_value_enum());
                println!("[LLVM] Pushed bool to stack: {}", val);
            }
            RazenIR::PushNull => {
                let null_ptr = self.context.i8_type().ptr_type(AddressSpace::default()).const_null();
                self.value_stack.push(null_ptr.as_basic_value_enum());
                println!("[LLVM] Pushed null to stack");
            }
            
            RazenIR::LoadVar(name) => {
                let (ptr_val, element_type) = self.variables.get(name)
                    .ok_or_else(|| format!("Undefined variable: {}", name))?;
                
                let loaded_val: BasicValueEnum = match element_type {
                    BasicTypeEnum::IntType(int_type) => {
                        self.builder.build_load(*int_type, *ptr_val, name).into()
                    }
                    BasicTypeEnum::FloatType(float_type) => {
                        self.builder.build_load(*float_type, *ptr_val, name).into()
                    }
                    BasicTypeEnum::PointerType(ptr_type) => {
                        self.builder.build_load(*ptr_type, *ptr_val, name).into()
                    }
                    BasicTypeEnum::ArrayType(array_type) => {
                        self.builder.build_load(*array_type, *ptr_val, name).into()
                    }
                    BasicTypeEnum::StructType(struct_type) => {
                        self.builder.build_load(*struct_type, *ptr_val, name).into()
                    }
                    BasicTypeEnum::VectorType(vector_type) => {
                        self.builder.build_load(*vector_type, *ptr_val, name).into()
                    }
                };

                self.value_stack.push(loaded_val);
                println!("[LLVM] Loaded var '{}' to stack", name);
            }
            
            RazenIR::StoreVar(name) => {
                let val_to_store = self.value_stack.pop()
                    .ok_or_else(|| "Stack underflow during StoreVar".to_string())?;

                if let Some((ptr_val, expected_type)) = self.variables.get(name) {
                    // Check if the type of val_to_store matches the expected type
                    let val_type = self.get_basic_value_type(val_to_store)?;
                    if val_type == *expected_type {
                        self.builder.build_store(*ptr_val, val_to_store);
                    } else {
                        return Err(format!("StoreVar type mismatch for '{}': expected {:?}, got {:?}", 
                                         name, expected_type, val_type));
                    }
                } else {
                    // If variable doesn't exist, create an alloca for it
                    let llvm_type = self.get_basic_value_type(val_to_store)?;
                    let alloca = self.create_entry_block_alloca(name, llvm_type, current_function)?;
                    self.builder.build_store(alloca, val_to_store);
                    self.variables.insert(name.clone(), (alloca, llvm_type));
                }
                println!("[LLVM] Stored to var '{}'", name);
            }
            
            RazenIR::Add => {
                let rhs = self.value_stack.pop().ok_or_else(|| "Stack underflow during Add (rhs)".to_string())?;
                let lhs = self.value_stack.pop().ok_or_else(|| "Stack underflow during Add (lhs)".to_string())?;
                
                let result = match (lhs, rhs) {
                    (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                        self.builder.build_int_add(l, r, "addtmp").as_basic_value_enum()
                    }
                    (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                        self.builder.build_float_add(l, r, "addtmp").as_basic_value_enum()
                    }
                    _ => return Err("Type mismatch in Add operation".to_string()),
                };
                
                self.value_stack.push(result);
                println!("[LLVM] Performed Add operation");
            }
            
            RazenIR::Subtract => {
                let rhs = self.value_stack.pop().ok_or_else(|| "Stack underflow during Subtract (rhs)".to_string())?;
                let lhs = self.value_stack.pop().ok_or_else(|| "Stack underflow during Subtract (lhs)".to_string())?;
                
                let result = match (lhs, rhs) {
                    (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                        self.builder.build_int_sub(l, r, "subtmp").as_basic_value_enum()
                    }
                    (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                        self.builder.build_float_sub(l, r, "subtmp").as_basic_value_enum()
                    }
                    _ => return Err("Type mismatch in Subtract operation".to_string()),
                };
                
                self.value_stack.push(result);
                println!("[LLVM] Performed Subtract operation");
            }
            
            RazenIR::Multiply => {
                let rhs = self.value_stack.pop().ok_or_else(|| "Stack underflow during Multiply (rhs)".to_string())?;
                let lhs = self.value_stack.pop().ok_or_else(|| "Stack underflow during Multiply (lhs)".to_string())?;
                
                let result = match (lhs, rhs) {
                    (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                        self.builder.build_int_mul(l, r, "multmp").as_basic_value_enum()
                    }
                    (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                        self.builder.build_float_mul(l, r, "multmp").as_basic_value_enum()
                    }
                    _ => return Err("Type mismatch in Multiply operation".to_string()),
                };
                
                self.value_stack.push(result);
                println!("[LLVM] Performed Multiply operation");
            }
            
            RazenIR::Divide => {
                let rhs = self.value_stack.pop().ok_or_else(|| "Stack underflow during Divide (rhs)".to_string())?;
                let lhs = self.value_stack.pop().ok_or_else(|| "Stack underflow during Divide (lhs)".to_string())?;
                
                let result = match (lhs, rhs) {
                    (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                        self.builder.build_int_signed_div(l, r, "divtmp").as_basic_value_enum()
                    }
                    (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                        self.builder.build_float_div(l, r, "divtmp").as_basic_value_enum()
                    }
                    _ => return Err("Type mismatch in Divide operation".to_string()),
                };
                
                self.value_stack.push(result);
                println!("[LLVM] Performed Divide operation");
            }
            
            RazenIR::Call(fn_name, arg_count) => {
                if let Some(function_to_call) = self.functions.get(fn_name) {
                    // Pop arguments from stack
                    let mut call_args: Vec<BasicMetadataValueEnum<'ctx>> = Vec::with_capacity(*arg_count);
                    for _ in 0..*arg_count {
                        let arg = self.value_stack.pop()
                            .ok_or_else(|| format!("Stack underflow during Call to '{}'", fn_name))?;
                        call_args.push(self.basic_value_to_metadata(arg));
                    }
                    
                    // Arguments are in reverse order, so reverse them
                    call_args.reverse();
                    
                    let call_site_val = self.builder.build_call(*function_to_call, call_args.as_slice(), "calltmp");
                    
                    if let Some(ret_val_basic) = call_site_val.try_as_basic_value().left() {
                        self.value_stack.push(ret_val_basic);
                        println!("[LLVM] Call to '{}' returned value", fn_name);
                    } else {
                        println!("[LLVM] Call to '{}' is void", fn_name);
                    }
                } else {
                    return Err(format!("Call to undefined function: {}", fn_name));
                }
            }
            
            RazenIR::Return => {
                if let Some(ret_val) = self.value_stack.pop() {
                    self.builder.build_return(Some(&ret_val));
                } else {
                    self.builder.build_return(None);
                }
                println!("[LLVM] Built return instruction");
            }
            
            RazenIR::Pop => {
                self.value_stack.pop().ok_or_else(|| "Stack underflow during Pop".to_string())?;
                println!("[LLVM] Popped value from stack");
            }
            
            RazenIR::Dup => {
                let val = self.value_stack.last().ok_or_else(|| "Stack underflow during Dup".to_string())?;
                self.value_stack.push(*val);
                println!("[LLVM] Duplicated top stack value");
            }
            
            RazenIR::Swap => {
                if self.value_stack.len() < 2 {
                    return Err("Stack underflow during Swap".to_string());
                }
                let len = self.value_stack.len();
                self.value_stack.swap(len - 1, len - 2);
                println!("[LLVM] Swapped top two stack values");
            }
            
            RazenIR::Print => {
                // For now, just pop and discard the value
                self.value_stack.pop().ok_or_else(|| "Stack underflow during Print".to_string())?;
                println!("[LLVM] Print instruction (value discarded)");
            }
            
            RazenIR::Exit => {
                // Create a call to exit(0)
                let exit_fn = self.module.add_function("exit", 
                    self.context.i32_type().fn_type(&[self.context.i32_type().into()], false), 
                    None
                );
                let exit_code = self.context.i32_type().const_int(0, false);
                self.builder.build_call(exit_fn, &[exit_code.into()], "exit");
                self.builder.build_unreachable();
                println!("[LLVM] Added exit call");
            }
            
            // Handle other IR instructions that exist in your codebase
            RazenIR::SetupTryCatch |
            RazenIR::ClearTryCatch |
            RazenIR::ThrowException |
            RazenIR::SetGlobal(_) |
            RazenIR::Modulo |
            RazenIR::Power |
            RazenIR::FloorDiv |
            RazenIR::Negate |
            RazenIR::Equal |
            RazenIR::NotEqual |
            RazenIR::GreaterThan |
            RazenIR::GreaterEqual |
            RazenIR::LessThan |
            RazenIR::LessEqual |
            RazenIR::And |
            RazenIR::Or |
            RazenIR::Not |
            RazenIR::Jump(_) |
            RazenIR::JumpIfFalse(_) |
            RazenIR::JumpIfTrue(_) |
            RazenIR::ReadInput |
            RazenIR::CreateArray(_) |
            RazenIR::GetIndex |
            RazenIR::SetIndex |
            RazenIR::CreateMap(_) |
            RazenIR::GetKey |
            RazenIR::SetKey |
            RazenIR::DefineFunction(_, _) |
            RazenIR::Label(_) |
            RazenIR::Sleep |
            RazenIR::LibraryCall(_, _, _) => {
                return Err(format!("Unsupported Razen IR instruction for LLVM: {:?}", instruction));
            }
        }
        Ok(())
    }

    // Helper to create an alloca in the entry block of a function
    fn create_entry_block_alloca(&self, name: &str, ty: BasicTypeEnum<'ctx>, function: FunctionValue<'ctx>) -> Result<PointerValue<'ctx>, String> {
        let temp_builder = self.context.create_builder();
        let entry = function.get_first_basic_block().unwrap();
        match entry.get_first_instruction() {
            Some(first_instr) => temp_builder.position_before(&first_instr),
            None => temp_builder.position_at_end(entry),
        }
        
        match ty {
            BasicTypeEnum::IntType(int_type) => Ok(temp_builder.build_alloca(int_type, name)),
            BasicTypeEnum::FloatType(float_type) => Ok(temp_builder.build_alloca(float_type, name)),
            BasicTypeEnum::PointerType(ptr_type) => Ok(temp_builder.build_alloca(ptr_type, name)),
            BasicTypeEnum::ArrayType(array_type) => Ok(temp_builder.build_alloca(array_type, name)),
            BasicTypeEnum::StructType(struct_type) => Ok(temp_builder.build_alloca(struct_type, name)),
            BasicTypeEnum::VectorType(vector_type) => Ok(temp_builder.build_alloca(vector_type, name)),
        }
    }

    pub fn dump_module(&self) {
        self.module.print_to_stderr();
    }
}