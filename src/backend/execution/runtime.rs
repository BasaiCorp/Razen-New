// src/backend/execution/runtime.rs
//! Clean runtime execution engine based on the proven old implementation

use std::collections::HashMap;
use std::io::{self, Write, BufRead};
use std::{thread, time::Duration};
use super::ir::IR;
use super::value::Value;

/// Runtime execution engine with stack machine - OPTIMIZED with typed values
pub struct Runtime {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    functions: HashMap<String, usize>, // Changed to usize for direct addressing
    call_stack: Vec<(usize, HashMap<String, Value>)>,
    _exception_handlers: Vec<(String, usize)>,
    function_params: HashMap<String, Vec<String>>, // Store function parameter names
    clean_output: bool,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            stack: Vec::with_capacity(1024), // Pre-allocate for performance
            variables: HashMap::with_capacity(256),
            functions: HashMap::new(),
            call_stack: Vec::new(),
            _exception_handlers: Vec::new(),
            function_params: HashMap::new(),
            clean_output: false,
        }
    }

    pub fn set_clean_output(&mut self, clean: bool) {
        self.clean_output = clean;
    }
    
    pub fn is_clean_output(&self) -> bool {
        self.clean_output
    }

    /// Register function parameter names for proper parameter binding
    pub fn register_function_params(&mut self, name: String, params: Vec<String>) {
        self.function_params.insert(name, params);
    }

    /// Execute IR instructions
    pub fn execute(&mut self, ir: &[IR]) -> Result<(), String> {
        if !self.clean_output {
            println!("Starting Razen execution engine...");
            println!("Generated {} IR instructions", ir.len());
        }

        // Pre-pass: register function addresses and extract parameter info from compiler
        let mut function_count = 0;
        let mut first_function_addr = None;
        for (i, instruction) in ir.iter().enumerate() {
            if let IR::DefineFunction(name, actual_addr) = instruction {
                // Use the actual function address, not the DefineFunction instruction address
                self.functions.insert(name.clone(), *actual_addr);
                if first_function_addr.is_none() {
                    first_function_addr = Some(*actual_addr);
                }
                function_count += 1;
                if !self.clean_output {
                    println!("Registered function '{}' at address {} (instruction at {})", name, actual_addr, i);
                }
            }
        }
        
        if !self.clean_output && function_count > 0 {
            println!("Registered {} functions", function_count);
        }
        
        // Pre-execute module initialization code (constants and variables)
        // ONLY execute instructions BEFORE the first DefineFunction
        let mut init_count = 0;
        let mut init_end_pc = 0;
        
        for (i, instruction) in ir.iter().enumerate() {
            // Stop at the first function definition
            if matches!(instruction, IR::DefineFunction(_, _)) {
                init_end_pc = i;
                break;
            }
            
            // Execute initialization instructions
            match instruction {
                IR::PushInteger(i) => self.stack.push(Value::Integer(*i)),
                IR::PushNumber(n) => self.stack.push(Value::Number(*n)),
                IR::PushString(s) => self.stack.push(Value::String(s.clone())),
                IR::PushBoolean(b) => self.stack.push(Value::Boolean(*b)),
                IR::PushNull => self.stack.push(Value::Null),
                IR::StoreVar(name) => {
                    if let Some(value) = self.stack.pop() {
                        self.variables.insert(name.clone(), value);
                        init_count += 1;
                        if !self.clean_output {
                            println!("Initialized: {} = {}", name, self.variables.get(name).map(|v| v.to_string()).unwrap_or("?".to_string()));
                        }
                    }
                },
                _ => {
                    // Skip other instructions during initialization
                }
            }
        }
        
        if !self.clean_output && init_count > 0 {
            println!("Initialized {} module variables/constants", init_count);
            println!("Initialization ended at PC {}", init_end_pc);
            println!("DEBUG: Variables after init: {:?}", self.variables.keys().collect::<Vec<_>>());
        }
        
        // Clear the stack after initialization
        self.stack.clear();
        
        if !self.clean_output {
            println!("DEBUG: Variables after stack clear: {:?}", self.variables.keys().collect::<Vec<_>>());
        }

        // Start normal execution from the beginning (position 0)
        // The Jump instructions will skip over function bodies
        let mut pc = 0;
        while pc < ir.len() {
            let instruction = &ir[pc];
            
            match instruction {
                IR::PushInteger(i) => {
                    self.stack.push(Value::Integer(*i));
                },
                IR::PushNumber(n) => {
                    self.stack.push(Value::Number(*n));
                },
                IR::PushString(s) => {
                    self.stack.push(Value::String(s.clone()));
                },
                IR::PushBoolean(b) => {
                    self.stack.push(Value::Boolean(*b));
                },
                IR::PushNull => {
                    self.stack.push(Value::Null);
                },
                IR::Pop => {
                    self.stack.pop();
                },
                IR::Dup => {
                    if let Some(value) = self.stack.last().cloned() {
                        self.stack.push(value);
                    }
                },
                IR::Swap => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(b);
                        self.stack.push(a);
                    }
                },
                IR::StoreVar(name) => {
                    if let Some(value) = self.stack.pop() {
                        if !self.clean_output {
                            println!("DEBUG: StoreVar '{}' = '{}', call_stack depth: {}", name, value, self.call_stack.len());
                        }
                        // Check if this is a qualified module variable (contains '.')
                        if name.contains('.') {
                            // Module-level variable - always store globally
                            self.variables.insert(name.clone(), value);
                        } else if let Some((_, func_vars)) = self.call_stack.last_mut() {
                            // Local variable - store in function scope
                            if !self.clean_output {
                                println!("DEBUG: Storing '{}' in function scope", name);
                            }
                            func_vars.insert(name.clone(), value);
                        } else {
                            // Global variable
                            if !self.clean_output {
                                println!("DEBUG: Storing '{}' in global scope", name);
                            }
                            self.variables.insert(name.clone(), value);
                        }
                    }
                },
                IR::LoadVar(name) => {
                    // Check if this is a qualified module variable (contains '.')
                    let value = if name.contains('.') {
                        // Module-level variable - load from global scope
                        if !self.clean_output {
                            println!("DEBUG: Loading module var '{}' from global scope", name);
                            println!("DEBUG: Available global vars: {:?}", self.variables.keys().collect::<Vec<_>>());
                        }
                        self.variables.get(name)
                    } else if let Some((_, func_vars)) = self.call_stack.last() {
                        // Try function scope first, then global
                        func_vars.get(name).or_else(|| self.variables.get(name))
                    } else {
                        // Load from global scope
                        self.variables.get(name)
                    };

                    if let Some(val) = value {
                        if !self.clean_output {
                            println!("DEBUG: LoadVar '{}' = '{}', call_stack depth: {}", name, val, self.call_stack.len());
                        }
                        self.stack.push(val.clone());
                    } else {
                        if !self.clean_output {
                            println!("DEBUG: Variable '{}' not found, pushing 'null', call_stack depth: {}", name, self.call_stack.len());
                        }
                        self.stack.push(Value::Null);
                    }
                },
                IR::SetGlobal(name) => {
                    if let Some(value) = self.stack.pop() {
                        self.variables.insert(name.clone(), value);
                    }
                },
                IR::Add => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a.add(&b));
                    }
                },
                IR::Subtract => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a.subtract(&b));
                    }
                },
                IR::Multiply => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a.multiply(&b));
                    }
                },
                IR::Divide => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a.divide(&b)?);
                    }
                },
                IR::Modulo => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_num), Some(b_num)) = (a.to_number(), b.to_number()) {
                            if b_num != 0.0 {
                                self.stack.push(Value::Number(a_num % b_num));
                            } else {
                                return Err("Modulo by zero".to_string());
                            }
                        }
                    }
                },
                IR::Power => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_num), Some(b_num)) = (a.to_number(), b.to_number()) {
                            self.stack.push(Value::Number(a_num.powf(b_num)));
                        }
                    }
                },
                IR::Negate => {
                    if let Some(a) = self.stack.pop() {
                        match a {
                            Value::Number(n) => self.stack.push(Value::Number(-n)),
                            Value::Integer(i) => self.stack.push(Value::Integer(-i)),
                            _ => self.stack.push(Value::Null),
                        }
                    }
                },
                IR::Equal => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(a.equal(&b)));
                    }
                },
                IR::NotEqual => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(!a.equal(&b)));
                    }
                },
                IR::GreaterThan => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(a.greater_than(&b)));
                    }
                },
                IR::GreaterEqual => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(a.greater_equal(&b)));
                    }
                },
                IR::LessThan => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(a.less_than(&b)));
                    }
                },
                IR::LessEqual => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(a.less_equal(&b)));
                    }
                },
                IR::And => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(a.is_truthy() && b.is_truthy()));
                    }
                },
                IR::Or => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(Value::Boolean(a.is_truthy() || b.is_truthy()));
                    }
                },
                IR::Not => {
                    if let Some(a) = self.stack.pop() {
                        self.stack.push(Value::Boolean(!a.is_truthy()));
                    }
                },
                IR::Jump(target) => {
                    if !self.clean_output {
                        println!("DEBUG: Jump from {} to {}", pc, target);
                    }
                    pc = *target;
                    continue;
                },
                IR::JumpIfFalse(target) => {
                    if let Some(value) = self.stack.pop() {
                        if !value.is_truthy() {
                            pc = *target;
                            continue;
                        }
                    }
                },
                IR::JumpIfTrue(target) => {
                    if let Some(value) = self.stack.pop() {
                        if value.is_truthy() {
                            pc = *target;
                            continue;
                        }
                    }
                },
                IR::Call(name, arg_count) => {
                    // Handle builtin functions
                    if self.is_builtin(name) {
                        self.execute_builtin(name, *arg_count)?;
                    } else if name.contains('.') {
                        // Check if this is a stdlib function call (module.function)
                        let parts: Vec<&str> = name.splitn(2, '.').collect();
                        if parts.len() == 2 {
                            let module_name = parts[0];
                            let function_name = parts[1];
                            
                            if crate::stdlib::is_stdlib_module(module_name) {
                                // Collect arguments from stack
                                let mut args = Vec::new();
                                for _ in 0..*arg_count {
                                    if let Some(arg) = self.stack.pop() {
                                        args.push(arg);
                                    }
                                }
                                args.reverse(); // Arguments are pushed in reverse order
                                
                                if !self.clean_output {
                                    println!("[DEBUG] Calling stdlib function: {}.{}", module_name, function_name);
                                }
                                
                                // Call stdlib function
                                match crate::stdlib::call_stdlib_function(module_name, function_name, args) {
                                    Ok(result) => {
                                        self.stack.push(result);
                                    }
                                    Err(e) => {
                                        return Err(format!("Stdlib function error: {}", e));
                                    }
                                }
                            } else {
                                // Not a stdlib module, try user-defined function
                                self.call_user_function(name, *arg_count, &mut pc)?;
                            }
                        } else {
                            self.call_user_function(name, *arg_count, &mut pc)?;
                        }
                    } else {
                        // User-defined function call
                        self.call_user_function(name, *arg_count, &mut pc)?;
                    }
                },
                IR::MethodCall(method_name, arg_count) => {
                    // Check if this is a builtin method first
                    if self.is_builtin(method_name) {
                        // Handle builtin methods - the object is already on the stack as the first argument
                        self.execute_builtin(method_name, *arg_count)?;
                    } else {
                        // Method calls are handled similarly to function calls
                        // but the first argument is the 'self' object
                        let mut args = Vec::new();
                        for _ in 0..*arg_count {
                            if let Some(arg) = self.stack.pop() {
                                args.push(arg);
                            }
                        }
                        args.reverse(); // Arguments are pushed in reverse order
                        
                        if args.is_empty() {
                            return Err("Method call requires at least self argument".to_string());
                        }
                        
                        let self_obj = &args[0];
                        let method_args = &args[1..];
                        
                        // Extract type from the object
                        let object_type = if let Value::Struct { type_name, .. } = self_obj {
                            // Proper struct value - get type directly
                            Some(type_name.clone())
                        } else if let Value::String(obj_str) = self_obj {
                            // Fallback: parse string representation (backwards compatibility)
                            if obj_str.starts_with('{') && obj_str.contains("__type__:") {
                                let content = &obj_str[1..obj_str.len()-1];
                                let pairs: Vec<&str> = content.split(", ").collect();
                                let mut found_type = None;
                                for pair in pairs {
                                    if let Some(colon_pos) = pair.find(": ") {
                                        let key = &pair[..colon_pos];
                                        let value = &pair[colon_pos + 2..];
                                        if key == "__type__" {
                                            found_type = Some(value.to_string());
                                            break;
                                        }
                                    }
                                }
                                found_type
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        
                        // Build the full method name using dot notation
                        let full_method_name = if let Some(type_name) = object_type {
                            format!("{}.{}", type_name, method_name)
                        } else {
                            // Fallback: try to find any method with this name
                            let mut found_name = format!("Object.{}", method_name);
                            for func_name in self.functions.keys() {
                                if func_name.ends_with(&format!(".{}", method_name)) {
                                    found_name = func_name.clone();
                                    break;
                                }
                            }
                            found_name
                        };
                        
                        if !self.clean_output {
                            println!("Looking for method '{}' (full name: '{}')", method_name, full_method_name);
                        }
                        
                        if let Some(&func_addr) = self.functions.get(&full_method_name) {
                            // Create new method scope with self and parameters
                            let mut func_variables = HashMap::new();
                            func_variables.insert("self".to_string(), self_obj.clone());
                            
                            // Bind method parameters
                            if let Some(param_names) = self.function_params.get(&full_method_name) {
                                // Skip first parameter (self) if it exists in param_names
                                let method_param_names = if param_names.first() == Some(&"self".to_string()) {
                                    &param_names[1..]
                                } else {
                                    param_names
                                };
                                
                                for (i, param_name) in method_param_names.iter().enumerate() {
                                    if i < method_args.len() {
                                        func_variables.insert(param_name.clone(), method_args[i].clone());
                                    } else {
                                        func_variables.insert(param_name.clone(), Value::Null);
                                    }
                                }
                            }
                            
                            // Save current state and jump to method
                            // Use same approach as regular function calls - keep variables in call stack
                            self.call_stack.push((pc + 1, func_variables));
                            pc = func_addr;
                            continue;
                        } else {
                            return Err(format!("Method '{}' not found", full_method_name));
                        }
                    }
                },
                IR::Return => {
                    let return_value = self.stack.pop().unwrap_or(Value::Null);
                    if let Some((return_addr, _func_variables)) = self.call_stack.pop() {
                        // Don't restore variables - global variables stay in self.variables
                        self.stack.push(return_value);
                        pc = return_addr;
                        continue;
                    } else {
                        self.stack.push(return_value);
                    }
                },
                IR::Print => {
                    if let Some(value) = self.stack.pop() {
                        print!("{}", value);
                        io::stdout().flush().unwrap();
                    }
                },
                IR::ReadInput => {
                    let stdin = io::stdin();
                    let mut line = String::new();
                    stdin.lock().read_line(&mut line).expect("Failed to read line");
                    if line.ends_with('\n') {
                        line.pop();
                        if line.ends_with('\r') {
                            line.pop();
                        }
                    }
                    self.stack.push(Value::String(line));
                },
                IR::Exit => {
                    return Ok(());
                },
                IR::Sleep => {
                    if let Some(duration_val) = self.stack.pop() {
                        if let Some(duration) = duration_val.to_number() {
                            thread::sleep(Duration::from_secs_f64(duration));
                        }
                    }
                },
                IR::FloorDiv => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_num), Some(b_num)) = (a.to_number(), b.to_number()) {
                            if b_num != 0.0 {
                                self.stack.push(Value::Number((a_num / b_num).floor()));
                            } else {
                                return Err("Division by zero".to_string());
                            }
                        }
                    }
                },
                IR::CreateArray(size) => {
                    // Pop 'size' elements from stack and create an array
                    let mut elements = Vec::new();
                    for _ in 0..*size {
                        if let Some(element) = self.stack.pop() {
                            elements.push(element);
                        }
                    }
                    elements.reverse(); // Restore original order
                    
                    // Create proper Array value
                    self.stack.push(Value::Array(elements));
                },
                IR::CreateMap(size) => {
                    if !self.clean_output {
                        println!("DEBUG: CreateMap with size {}", size);
                        println!("DEBUG: Stack size before CreateMap: {}", self.stack.len());
                    }
                    
                    // Pop 'size * 2' elements from stack (key-value pairs)
                    let mut pairs = Vec::new();
                    for _ in 0..*size {
                        if let (Some(value), Some(key)) = (self.stack.pop(), self.stack.pop()) {
                            if !self.clean_output {
                                println!("DEBUG: Pair - key: {}, value: {}", key, value);
                            }
                            pairs.push((key, value));
                        }
                    }
                    pairs.reverse(); // Restore original order
                    
                    // Check if first key is "__type__" to determine if this is a struct
                    let mut type_name = None;
                    let mut fields = HashMap::new();
                    
                    for (key, value) in pairs {
                        let key_str = key.to_string();
                        if key_str == "__type__" {
                            type_name = Some(value.to_string());
                        } else {
                            fields.insert(key_str, value);
                        }
                    }
                    
                    // If we have a type name, create a proper Struct value
                    if let Some(type_name) = type_name {
                        if !self.clean_output {
                            println!("DEBUG: Creating struct '{}' with fields: {:?}", type_name, fields.keys().collect::<Vec<_>>());
                        }
                        self.stack.push(Value::Struct { type_name, fields });
                    } else {
                        // Otherwise, create a proper Map value
                        if !self.clean_output {
                            println!("DEBUG: Creating map with {} fields", fields.len());
                        }
                        self.stack.push(Value::Map(fields));
                    }
                },
                IR::GetKey => {
                    // Pop key and object from stack, push the value for that key
                    if let (Some(key), Some(object)) = (self.stack.pop(), self.stack.pop()) {
                        match &object {
                            // Handle Array indexing
                            Value::Array(arr) => {
                                if let Some(index) = key.to_integer() {
                                    if index >= 0 && (index as usize) < arr.len() {
                                        self.stack.push(arr[index as usize].clone());
                                    } else {
                                        if !self.clean_output {
                                            println!("DEBUG: Array index {} out of bounds (len: {})", index, arr.len());
                                        }
                                        self.stack.push(Value::Null);
                                    }
                                } else {
                                    if !self.clean_output {
                                        println!("DEBUG: Invalid array index: {}", key);
                                    }
                                    self.stack.push(Value::Null);
                                }
                            },
                            // Handle Map indexing
                            Value::Map(map) => {
                                let key_str = key.to_string();
                                if let Some(value) = map.get(&key_str) {
                                    self.stack.push(value.clone());
                                } else {
                                    if !self.clean_output {
                                        println!("DEBUG: Key '{}' not found in map", key_str);
                                    }
                                    self.stack.push(Value::Null);
                                }
                            },
                            // Handle Struct field access
                            Value::Struct { type_name, fields } => {
                                let key_str = key.to_string();
                                if !self.clean_output {
                                    println!("DEBUG: GetKey on struct '{}', looking for field '{}', available fields: {:?}", type_name, key_str, fields.keys().collect::<Vec<_>>());
                                }
                                if let Some(value) = fields.get(&key_str) {
                                    if !self.clean_output {
                                        println!("DEBUG: Found field '{}' = '{}'", key_str, value);
                                    }
                                    self.stack.push(value.clone());
                                } else {
                                    if !self.clean_output {
                                        println!("DEBUG: Field '{}' not found in struct", key_str);
                                    }
                                    self.stack.push(Value::Null);
                                }
                            },
                            // Fallback for other types
                            _ => {
                                if !self.clean_output {
                                    println!("DEBUG: GetKey not supported for value type: {:?}", object);
                                }
                                self.stack.push(Value::Null);
                            }
                        }
                    }
                },
                IR::BitwiseAnd => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_int), Some(b_int)) = (a.to_integer(), b.to_integer()) {
                            self.stack.push(Value::Integer(a_int & b_int));
                        } else {
                            self.stack.push(Value::Integer(0)); // Error case
                        }
                    }
                },
                IR::BitwiseOr => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_int), Some(b_int)) = (a.to_integer(), b.to_integer()) {
                            self.stack.push(Value::Integer(a_int | b_int));
                        } else {
                            self.stack.push(Value::Integer(0)); // Error case
                        }
                    }
                },
                IR::BitwiseXor => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_int), Some(b_int)) = (a.to_integer(), b.to_integer()) {
                            self.stack.push(Value::Integer(a_int ^ b_int));
                        } else {
                            self.stack.push(Value::Integer(0)); // Error case
                        }
                    }
                },
                IR::BitwiseNot => {
                    if let Some(a) = self.stack.pop() {
                        if let Some(a_int) = a.to_integer() {
                            self.stack.push(Value::Integer(!a_int));
                        } else {
                            self.stack.push(Value::Integer(0)); // Error case
                        }
                    }
                },
                IR::LeftShift => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_int), Some(b_int)) = (a.to_integer(), b.to_integer()) {
                            self.stack.push(Value::Integer(a_int << b_int));
                        } else {
                            self.stack.push(Value::Integer(0)); // Error case
                        }
                    }
                },
                IR::RightShift => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_int), Some(b_int)) = (a.to_integer(), b.to_integer()) {
                            self.stack.push(Value::Integer(a_int >> b_int));
                        } else {
                            self.stack.push(Value::Integer(0)); // Error case
                        }
                    }
                },
                IR::DefineFunction(_, _) | IR::Label(_) => {
                    // Ignored at runtime
                },
                _ => {
                    if !self.clean_output {
                        println!("Unimplemented instruction: {:?}", instruction);
                    }
                }
            }
            pc += 1;
        }

        if !self.clean_output {
            println!("Execution completed successfully");
            println!("Final stack size: {}", self.stack.len());
            println!("Variables in scope: {}", self.variables.len());
        }
        Ok(())
    }

    /// Apply color formatting to text based on color name or hex code
    fn apply_color_formatting(&self, color_spec: &str) -> String {
        let spec = color_spec.trim();
        
        // Handle hex codes (e.g., "#FF0000", "#ff0000")
        if spec.starts_with('#') && spec.len() == 7 {
            if let Ok(rgb) = self.hex_to_rgb(&spec[1..]) {
                return format!("\x1b[38;2;{};{};{}m", rgb.0, rgb.1, rgb.2);
            }
        }
        
        // Handle 16 standard color names
        match spec.to_lowercase().as_str() {
            // Standard 16 colors (4-bit)
            "black" => "\x1b[30m".to_string(),
            "red" => "\x1b[31m".to_string(),
            "green" => "\x1b[32m".to_string(),
            "yellow" => "\x1b[33m".to_string(),
            "blue" => "\x1b[34m".to_string(),
            "magenta" | "purple" => "\x1b[35m".to_string(),
            "cyan" => "\x1b[36m".to_string(),
            "white" => "\x1b[37m".to_string(),
            // Bright colors
            "gray" | "grey" => "\x1b[90m".to_string(),
            "bright_red" | "lightred" => "\x1b[91m".to_string(),
            "bright_green" | "lightgreen" => "\x1b[92m".to_string(),
            "bright_yellow" | "lightyellow" => "\x1b[93m".to_string(),
            "bright_blue" | "lightblue" => "\x1b[94m".to_string(),
            "bright_magenta" | "lightmagenta" | "pink" => "\x1b[95m".to_string(),
            "bright_cyan" | "lightcyan" => "\x1b[96m".to_string(),
            "bright_white" | "lightwhite" => "\x1b[97m".to_string(),
            // Special colors
            "orange" => "\x1b[38;5;208m".to_string(), // 8-bit orange
            "brown" => "\x1b[38;5;94m".to_string(),  // 8-bit brown
            // Reset
            "reset" | "none" | "default" => "\x1b[0m".to_string(),
            _ => {
                // Invalid color, return reset
                "\x1b[0m".to_string()
            }
        }
    }
    
    /// Convert hex string to RGB tuple
    fn hex_to_rgb(&self, hex: &str) -> Result<(u8, u8, u8), ()> {
        if hex.len() != 6 {
            return Err(());
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ())?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ())?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ())?;
        
        Ok((r, g, b))
    }

    fn is_builtin(&self, name: &str) -> bool {
        matches!(name, "print" | "println" | "printc" | "printlnc" | "input" | "read" | "write" | "len" | "append" | "remove" | "toint" | "tofloat" | "tostr" | "tobool" | "typeof" | "create_range" | "array_get" | "concat_string" | "load_var_by_name" | "Ok" | "Err" | "Some" | "None" | "is_ok" | "is_err" | "is_some" | "is_none" | "unwrap" | "unwrap_or")
    }

    fn execute_builtin(&mut self, name: &str, arg_count: usize) -> Result<(), String> {
        match name {
            "print" => {
                if let Some(value) = self.stack.pop() {
                    print!("{}", value);
                    io::stdout().flush().unwrap();
                }
                // Push null as return value
                self.stack.push(Value::Null);
            },
            "println" => {
                if let Some(value) = self.stack.pop() {
                    println!("{}", value);
                } else {
                    println!();
                }
                // Push null as return value
                self.stack.push(Value::Null);
            },
            "printc" => {
                // Colored print: printc(text, color)
                if arg_count >= 2 {
                    let color_spec = self.stack.pop().unwrap_or(Value::String("reset".to_string())).to_string();
                    let text = self.stack.pop().unwrap_or(Value::String("".to_string())).to_string();
                    
                    let color_code = self.apply_color_formatting(&color_spec);
                    let reset_code = "\x1b[0m";
                    
                    print!("{}{}{}", color_code, text, reset_code);
                    io::stdout().flush().unwrap();
                } else {
                    return Err("printc() requires 2 arguments: text and color".to_string());
                }
                // Push null as return value
                self.stack.push(Value::Null);
            },
            "printlnc" => {
                // Colored println: printlnc(text, color)
                if arg_count >= 2 {
                    let color_spec = self.stack.pop().unwrap_or(Value::String("reset".to_string())).to_string();
                    let text = self.stack.pop().unwrap_or(Value::String("".to_string())).to_string();
                    
                    let color_code = self.apply_color_formatting(&color_spec);
                    let reset_code = "\x1b[0m";
                    
                    println!("{}{}{}", color_code, text, reset_code);
                } else {
                    return Err("printlnc() requires 2 arguments: text and color".to_string());
                }
                // Push null as return value
                self.stack.push(Value::Null);
            },
            "input" => {
                if arg_count > 0 {
                    if let Some(prompt) = self.stack.pop() {
                        print!("{}", prompt);
                        io::stdout().flush().unwrap();
                    }
                }
                
                let stdin = io::stdin();
                let mut line = String::new();
                stdin.lock().read_line(&mut line).expect("Failed to read line");
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }
                self.stack.push(Value::String(line));
            },
            "len" => {
                if let Some(value) = self.stack.pop() {
                    let len = match &value {
                        Value::String(s) => s.len(),
                        _ => value.to_string().len(),
                    };
                    self.stack.push(Value::Integer(len as i64));
                } else {
                    self.stack.push(Value::Integer(0));
                }
            },
            // Dot notation type conversion methods
            "toint" => {
                if let Some(value) = self.stack.pop() {
                    if let Some(int_val) = value.to_integer() {
                        self.stack.push(Value::Integer(int_val));
                    } else {
                        return Err(format!("Cannot convert '{}' to int", value));
                    }
                } else {
                    return Err("toint() requires one argument".to_string());
                }
            },
            "tofloat" => {
                if let Some(value) = self.stack.pop() {
                    if let Some(float_val) = value.to_number() {
                        self.stack.push(Value::Number(float_val));
                    } else {
                        return Err(format!("Cannot convert '{}' to float", value));
                    }
                } else {
                    return Err("tofloat() requires one argument".to_string());
                }
            },
            "tostr" => {
                if let Some(value) = self.stack.pop() {
                    // Everything can be converted to string
                    self.stack.push(Value::String(value.to_string()));
                } else {
                    return Err("tostr() requires one argument".to_string());
                }
            },
            "tobool" => {
                if let Some(value) = self.stack.pop() {
                    self.stack.push(Value::Boolean(value.is_truthy()));
                } else {
                    return Err("tobool() requires one argument".to_string());
                }
            },
            "typeof" => {
                if let Some(value) = self.stack.pop() {
                    let type_name = match value {
                        Value::Integer(_) => "int",
                        Value::Number(_) => "float",
                        Value::String(_) => "str",
                        Value::Boolean(_) => "bool",
                        Value::Array(_) => "array",
                        Value::Map(_) => "map",
                        Value::Struct { ref type_name, .. } => type_name.as_str(),
                        Value::Result { .. } => "Result",
                        Value::Option { .. } => "Option",
                        Value::Null => "null",
                    };
                    self.stack.push(Value::String(type_name.to_string()));
                } else {
                    return Err("typeof() requires one argument".to_string());
                }
            },
            "create_range" => {
                // Create a range object from start, end, and inclusive flag
                if arg_count >= 3 {
                    let inclusive = self.stack.pop().unwrap_or(Value::Boolean(false));
                    let end = self.stack.pop().unwrap_or(Value::Integer(0));
                    let start = self.stack.pop().unwrap_or(Value::Integer(0));
                    
                    // Store range as a formatted string "start..end" or "start..=end"
                    let range_str = if inclusive.is_truthy() {
                        format!("{}..={}", start, end)
                    } else {
                        format!("{}..{}", start, end)
                    };
                    self.stack.push(Value::String(range_str));
                } else {
                    return Err("create_range() requires 3 arguments (start, end, inclusive)".to_string());
                }
            },
            "array_get" => {
                // Get array element by index (simplified implementation)
                if arg_count >= 2 {
                    let array_name = self.stack.pop().unwrap_or(Value::String("array_0".to_string())).to_string();
                    let index = self.stack.pop().unwrap_or(Value::Integer(0));
                    
                    // Try to parse index as number
                    if let Some(idx) = index.to_integer() {
                        let var_name = format!("{}{}", array_name, idx);
                        if let Some(value) = self.variables.get(&var_name) {
                            self.stack.push(value.clone());
                        } else {
                            // Debug: print available variables
                            if !self.clean_output {
                                println!("Looking for variable '{}', available: {:?}", var_name, self.variables.keys().collect::<Vec<_>>());
                            }
                            self.stack.push(Value::Null);
                        }
                    } else {
                        self.stack.push(Value::Null);
                    }
                } else {
                    return Err("array_get() requires 2 arguments (array, index)".to_string());
                }
            },
            "concat_string" => {
                // Concatenate two strings/values
                if arg_count >= 2 {
                    let second = self.stack.pop().unwrap_or(Value::String("".to_string()));
                    let first = self.stack.pop().unwrap_or(Value::String("".to_string()));
                    let result = format!("{}{}", second, first); // Note: reversed order due to stack
                    self.stack.push(Value::String(result));
                } else {
                    return Err("concat_string() requires 2 arguments".to_string());
                }
            },
            "load_var_by_name" => {
                // Load a variable by its name (from stack)
                if arg_count >= 1 {
                    let var_name = self.stack.pop().unwrap_or(Value::String("".to_string())).to_string();
                    if let Some(value) = self.variables.get(&var_name) {
                        self.stack.push(value.clone());
                    } else {
                        if !self.clean_output {
                            println!("Variable '{}' not found, available: {:?}", var_name, self.variables.keys().collect::<Vec<_>>());
                        }
                        self.stack.push(Value::Null);
                    }
                } else {
                    return Err("load_var_by_name() requires 1 argument".to_string());
                }
            },
            // Result type constructors
            "Ok" => {
                if arg_count >= 1 {
                    let value = self.stack.pop().unwrap_or(Value::Null);
                    self.stack.push(Value::ok(value));
                } else {
                    return Err("Ok() requires 1 argument".to_string());
                }
            },
            "Err" => {
                if arg_count >= 1 {
                    let error = self.stack.pop().unwrap_or(Value::String("Error".to_string()));
                    self.stack.push(Value::err(error));
                } else {
                    return Err("Err() requires 1 argument".to_string());
                }
            },
            // Option type constructors
            "Some" => {
                if arg_count >= 1 {
                    let value = self.stack.pop().unwrap_or(Value::Null);
                    self.stack.push(Value::some(value));
                } else {
                    return Err("Some() requires 1 argument".to_string());
                }
            },
            "None" => {
                self.stack.push(Value::none());
            },
            // Result/Option methods
            "is_ok" => {
                if let Some(value) = self.stack.pop() {
                    self.stack.push(Value::Boolean(value.is_ok()));
                } else {
                    return Err("is_ok() requires 1 argument".to_string());
                }
            },
            "is_err" => {
                if let Some(value) = self.stack.pop() {
                    self.stack.push(Value::Boolean(value.is_err()));
                } else {
                    return Err("is_err() requires 1 argument".to_string());
                }
            },
            "is_some" => {
                if let Some(value) = self.stack.pop() {
                    self.stack.push(Value::Boolean(value.is_some()));
                } else {
                    return Err("is_some() requires 1 argument".to_string());
                }
            },
            "is_none" => {
                if let Some(value) = self.stack.pop() {
                    self.stack.push(Value::Boolean(value.is_none()));
                } else {
                    return Err("is_none() requires 1 argument".to_string());
                }
            },
            "unwrap" => {
                if let Some(value) = self.stack.pop() {
                    match value.unwrap() {
                        Ok(inner) => self.stack.push(inner),
                        Err(e) => return Err(e),
                    }
                } else {
                    return Err("unwrap() requires 1 argument".to_string());
                }
            },
            "unwrap_or" => {
                if arg_count >= 2 {
                    let default = self.stack.pop().unwrap_or(Value::Null);
                    let value = self.stack.pop().unwrap_or(Value::Null);
                    self.stack.push(value.unwrap_or(default));
                } else {
                    return Err("unwrap_or() requires 2 arguments".to_string());
                }
            },
            _ => {
                // Pop arguments for unimplemented builtins
                for _ in 0..arg_count {
                    self.stack.pop();
                }
                self.stack.push(Value::Null);
            }
        }
        Ok(())
    }

    /// Helper method to call user-defined functions
    fn call_user_function(&mut self, name: &str, arg_count: usize, pc: &mut usize) -> Result<(), String> {
        // User-defined function call - collect arguments from stack
        let mut args = Vec::new();
        for _ in 0..arg_count {
            if let Some(arg) = self.stack.pop() {
                args.push(arg);
            }
        }
        args.reverse(); // Arguments are pushed in reverse order

        if !self.clean_output {
            println!("[DEBUG] Looking for function '{}' in functions: {:?}", name, self.functions.keys().collect::<Vec<_>>());
        }
        
        if let Some(&func_addr) = self.functions.get(name) {
            if !self.clean_output {
                println!("[DEBUG] Found function '{}' at address {}", name, func_addr);
            }
            // Create new function scope with parameters
            let mut func_variables = HashMap::new();
            
            // If we have parameter names stored, bind arguments to parameters
            if let Some(param_names) = self.function_params.get(name) {
                if !self.clean_output {
                    println!("[DEBUG] Binding {} parameters for function '{}'", param_names.len(), name);
                }
                for (i, param_name) in param_names.iter().enumerate() {
                    if i < args.len() {
                        if !self.clean_output {
                            println!("[DEBUG] Binding param '{}' = '{}'", param_name, args[i]);
                        }
                        func_variables.insert(param_name.clone(), args[i].clone());
                    } else {
                        func_variables.insert(param_name.clone(), Value::Null);
                    }
                }
            } else {
                if !self.clean_output {
                    println!("[DEBUG] No param names registered for function '{}', using generic names", name);
                }
                // Fallback: create generic parameter names
                for (i, arg) in args.iter().enumerate() {
                    func_variables.insert(format!("param{}", i), arg.clone());
                }
            }
            
            // Save current state and jump to function
            // Don't replace self.variables - keep global variables intact!
            self.call_stack.push((*pc + 1, func_variables));
            *pc = func_addr;
        } else {
            // Function not found - push null and continue
            if !self.clean_output {
                println!("[INFO] Function '{}' not found", name);
            }
            self.stack.push(Value::Null);
        }
        
        Ok(())
    }
    
    // Helper methods for RAIE adaptive engine
    
    /// Get variable value for specialized operations
    pub fn get_variable_value(&self, name: &str) -> Result<Value, String> {
        // Check function scope first
        if let Some((_, func_vars)) = self.call_stack.last() {
            if let Some(value) = func_vars.get(name) {
                return Ok(value.clone());
            }
        }
        
        // Check global scope
        if let Some(value) = self.variables.get(name) {
            Ok(value.clone())
        } else {
            Err(format!("Variable '{}' not found", name))
        }
    }
    
    /// Set variable value for specialized operations
    pub fn set_variable_value(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Check if we're in a function call
        if let Some((_, func_vars)) = self.call_stack.last_mut() {
            // If variable exists in function scope, update it
            if func_vars.contains_key(name) {
                func_vars.insert(name.to_string(), value);
                return Ok(());
            }
        }
        
        // Otherwise, set in global scope
        self.variables.insert(name.to_string(), value);
        Ok(())
    }
    
    /// Get current variable scope size (for performance monitoring)
    pub fn get_variable_count(&self) -> usize {
        let mut count = self.variables.len();
        if let Some((_, func_vars)) = self.call_stack.last() {
            count += func_vars.len();
        }
        count
    }
    
    /// Clear all variables (for testing/benchmarking)
    pub fn clear_variables(&mut self) {
        self.variables.clear();
        if let Some((_, func_vars)) = self.call_stack.last_mut() {
            func_vars.clear();
        }
    }
}
