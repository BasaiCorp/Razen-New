// src/backend/execution/runtime.rs
//! Clean runtime execution engine based on the proven old implementation

use std::collections::HashMap;
use std::io::{self, Write, BufRead};
use std::{thread, time::Duration};
use super::ir::IR;

/// Runtime execution engine with stack machine
pub struct Runtime {
    stack: Vec<String>,
    variables: HashMap<String, String>,
    functions: HashMap<String, String>, // Separate function registry
    call_stack: Vec<(usize, HashMap<String, String>)>,
    _exception_handlers: Vec<(String, usize)>,
    function_params: HashMap<String, Vec<String>>, // Store function parameter names
    clean_output: bool,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            stack: Vec::new(),
            variables: HashMap::new(),
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
        for (i, instruction) in ir.iter().enumerate() {
            if let IR::DefineFunction(name, actual_addr) = instruction {
                // Use the actual function address, not the DefineFunction instruction address
                self.functions.insert(name.clone(), actual_addr.to_string());
                function_count += 1;
                if !self.clean_output {
                    println!("Registered function '{}' at address {} (instruction at {})", name, actual_addr, i);
                }
            }
        }
        
        if !self.clean_output && function_count > 0 {
            println!("Registered {} functions", function_count);
        }

        let mut pc = 0;
        while pc < ir.len() {
            let instruction = &ir[pc];
            
            match instruction {
                IR::PushNumber(n) => {
                    self.stack.push(n.to_string());
                },
                IR::PushString(s) => {
                    self.stack.push(s.clone());
                },
                IR::PushBoolean(b) => {
                    self.stack.push(b.to_string());
                },
                IR::PushNull => {
                    self.stack.push("null".to_string());
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
                        if let Some((_, func_vars)) = self.call_stack.last_mut() {
                            func_vars.insert(name.clone(), value);
                        } else {
                            self.variables.insert(name.clone(), value);
                        }
                    }
                },
                IR::LoadVar(name) => {
                    let value = if let Some((_, func_vars)) = self.call_stack.last() {
                        func_vars.get(name)
                    } else { None };

                    if let Some(val) = value {
                        self.stack.push(val.clone());
                    } else if let Some(val) = self.variables.get(name) {
                        self.stack.push(val.clone());
                    } else {
                        self.stack.push("undefined".to_string());
                    }
                },
                IR::SetGlobal(name) => {
                    if let Some(value) = self.stack.pop() {
                        self.variables.insert(name.clone(), value);
                    }
                },
                IR::Add => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            self.stack.push((a_num + b_num).to_string());
                        } else {
                            // String concatenation
                            self.stack.push(format!("{}{}", a, b));
                        }
                    }
                },
                IR::Subtract => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            self.stack.push((a_num - b_num).to_string());
                        }
                    }
                },
                IR::Multiply => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            self.stack.push((a_num * b_num).to_string());
                        }
                    }
                },
                IR::Divide => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            if b_num != 0.0 {
                                self.stack.push((a_num / b_num).to_string());
                            } else {
                                return Err("Division by zero".to_string());
                            }
                        }
                    }
                },
                IR::Modulo => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            if b_num != 0.0 {
                                self.stack.push((a_num % b_num).to_string());
                            } else {
                                return Err("Modulo by zero".to_string());
                            }
                        }
                    }
                },
                IR::Power => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            self.stack.push(a_num.powf(b_num).to_string());
                        }
                    }
                },
                IR::Negate => {
                    if let Some(a) = self.stack.pop() {
                        if let Ok(a_num) = a.parse::<f64>() {
                            self.stack.push((-a_num).to_string());
                        }
                    }
                },
                IR::Equal => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            self.stack.push((a_num == b_num).to_string());
                        } else {
                            self.stack.push((a == b).to_string());
                        }
                    }
                },
                IR::NotEqual => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            self.stack.push((a_num != b_num).to_string());
                        } else {
                            self.stack.push((a != b).to_string());
                        }
                    }
                },
                IR::GreaterThan => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            self.stack.push((a_num > b_num).to_string());
                        } else {
                            self.stack.push((a > b).to_string());
                        }
                    }
                },
                IR::GreaterEqual => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            self.stack.push((a_num >= b_num).to_string());
                        } else {
                            self.stack.push((a >= b).to_string());
                        }
                    }
                },
                IR::LessThan => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            self.stack.push((a_num < b_num).to_string());
                        } else {
                            self.stack.push((a < b).to_string());
                        }
                    }
                },
                IR::LessEqual => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            self.stack.push((a_num <= b_num).to_string());
                        } else {
                            self.stack.push((a <= b).to_string());
                        }
                    }
                },
                IR::And => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push((is_truthy(&a) && is_truthy(&b)).to_string());
                    }
                },
                IR::Or => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push((is_truthy(&a) || is_truthy(&b)).to_string());
                    }
                },
                IR::Not => {
                    if let Some(a) = self.stack.pop() {
                        self.stack.push((!is_truthy(&a)).to_string());
                    }
                },
                IR::Jump(target) => {
                    pc = *target;
                    continue;
                },
                IR::JumpIfFalse(target) => {
                    if let Some(value) = self.stack.pop() {
                        if !is_truthy(&value) {
                            pc = *target;
                            continue;
                        }
                    }
                },
                IR::JumpIfTrue(target) => {
                    if let Some(value) = self.stack.pop() {
                        if is_truthy(&value) {
                            pc = *target;
                            continue;
                        }
                    }
                },
                IR::Call(name, arg_count) => {
                    // Handle builtin functions
                    if self.is_builtin(name) {
                        self.execute_builtin(name, *arg_count)?;
                    } else {
                        // User-defined function call - collect arguments from stack
                        let mut args = Vec::new();
                        for _ in 0..*arg_count {
                            if let Some(arg) = self.stack.pop() {
                                args.push(arg);
                            }
                        }
                        args.reverse(); // Arguments are pushed in reverse order

                        if !self.clean_output {
                            println!("Looking for function '{}' in functions: {:?}", name, self.functions.keys().collect::<Vec<_>>());
                        }
                        
                        if let Some(func_addr_str) = self.functions.get(name) {
                            if let Ok(func_addr) = func_addr_str.parse::<usize>() {
                                if !self.clean_output {
                                    println!("Found function '{}' at address {}", name, func_addr);
                                }
                                // Create new function scope with parameters
                                let mut func_variables = HashMap::new();
                                
                                // If we have parameter names stored, bind arguments to parameters
                                if let Some(param_names) = self.function_params.get(name) {
                                    for (i, param_name) in param_names.iter().enumerate() {
                                        if i < args.len() {
                                            func_variables.insert(param_name.clone(), args[i].clone());
                                        } else {
                                            func_variables.insert(param_name.clone(), "null".to_string());
                                        }
                                    }
                                } else {
                                    // Fallback: create generic parameter names
                                    for (i, arg) in args.iter().enumerate() {
                                        func_variables.insert(format!("param{}", i), arg.clone());
                                    }
                                }
                                
                                // Save current state and jump to function
                                self.call_stack.push((pc + 1, self.variables.clone()));
                                self.variables = func_variables;
                                pc = func_addr;
                                continue;
                            }
                        } else {
                            // Function not found - push undefined and continue
                            if !self.clean_output {
                                println!("Warning: Function '{}' not found", name);
                            }
                            self.stack.push("undefined".to_string());
                        }
                    }
                },
                IR::MethodCall(method_name, arg_count) => {
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
                    
                    // For now, we'll look for methods in the format "TypeName::method_name"
                    // We need to infer the type from the variable name or object structure
                    
                    // Try to find the method by checking all available methods
                    let mut full_method_name = format!("Object::{}", method_name);
                    
                    // Look for any method with this name in our functions
                    for func_name in self.functions.keys() {
                        if func_name.ends_with(&format!("::{}", method_name)) {
                            full_method_name = func_name.clone();
                            break;
                        }
                    }
                    
                    if !self.clean_output {
                        println!("Looking for method '{}' (full name: '{}')", method_name, full_method_name);
                    }
                    
                    if let Some(func_addr_str) = self.functions.get(&full_method_name) {
                        if let Ok(func_addr) = func_addr_str.parse::<usize>() {
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
                                        func_variables.insert(param_name.clone(), "null".to_string());
                                    }
                                }
                            }
                            
                            // Save current state and jump to method
                            self.call_stack.push((pc + 1, self.variables.clone()));
                            self.variables = func_variables;
                            pc = func_addr;
                            continue;
                        } else {
                            return Err(format!("Invalid method address for '{}'", full_method_name));
                        }
                    } else {
                        return Err(format!("Method '{}' not found", full_method_name));
                    }
                },
                IR::Return => {
                    let return_value = self.stack.pop().unwrap_or_else(|| "null".to_string());
                    if let Some((return_addr, caller_variables)) = self.call_stack.pop() {
                        self.variables = caller_variables;
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
                    self.stack.push(line);
                },
                IR::Exit => {
                    return Ok(());
                },
                IR::Sleep => {
                    if let Some(duration_str) = self.stack.pop() {
                        if let Ok(duration) = duration_str.parse::<f64>() {
                            thread::sleep(Duration::from_secs_f64(duration));
                        }
                    }
                },
                IR::FloorDiv => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            if b_num != 0.0 {
                                self.stack.push((a_num / b_num).floor().to_string());
                            } else {
                                return Err("Division by zero".to_string());
                            }
                        }
                    }
                },
                IR::CreateArray(size) => {
                    // Pop 'size' elements from stack and create an array representation
                    let mut elements = Vec::new();
                    for _ in 0..*size {
                        if let Some(element) = self.stack.pop() {
                            elements.push(element);
                        }
                    }
                    elements.reverse(); // Restore original order
                    
                    // For now, represent array as a string (in a full implementation, we'd use proper data structures)
                    let array_repr = format!("[{}]", elements.join(", "));
                    self.stack.push(array_repr);
                },
                IR::CreateMap(size) => {
                    // Pop 'size * 2' elements from stack (key-value pairs) and create a map representation
                    let mut pairs = Vec::new();
                    for _ in 0..*size {
                        if let (Some(value), Some(key)) = (self.stack.pop(), self.stack.pop()) {
                            pairs.push(format!("{}: {}", key, value));
                        }
                    }
                    pairs.reverse(); // Restore original order
                    
                    // For now, represent map as a string (in a full implementation, we'd use proper data structures)
                    let map_repr = format!("{{{}}}", pairs.join(", "));
                    self.stack.push(map_repr);
                },
                IR::GetKey => {
                    // Pop key and object from stack, push the value for that key
                    if let (Some(key), Some(object)) = (self.stack.pop(), self.stack.pop()) {
                        let mut found = false;
                        
                        // For now, parse the object as a map-like string representation
                        if object.starts_with('{') && object.ends_with('}') {
                            let content = &object[1..object.len()-1]; // Remove braces
                            if !content.is_empty() {
                                let pairs: Vec<&str> = content.split(", ").collect();
                                
                                for pair in pairs {
                                    if let Some(colon_pos) = pair.find(": ") {
                                        let pair_key = &pair[..colon_pos];
                                        let pair_value = &pair[colon_pos + 2..];
                                        
                                        if pair_key == key {
                                            self.stack.push(pair_value.to_string());
                                            found = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        
                        if !found {
                            // Key not found, push null
                            self.stack.push("null".to_string());
                        }
                    }
                },
                IR::BitwiseAnd => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<i64>(), b.parse::<i64>()) {
                            self.stack.push((a_num & b_num).to_string());
                        } else {
                            self.stack.push("0".to_string()); // Error case
                        }
                    }
                },
                IR::BitwiseOr => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<i64>(), b.parse::<i64>()) {
                            self.stack.push((a_num | b_num).to_string());
                        } else {
                            self.stack.push("0".to_string()); // Error case
                        }
                    }
                },
                IR::BitwiseXor => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<i64>(), b.parse::<i64>()) {
                            self.stack.push((a_num ^ b_num).to_string());
                        } else {
                            self.stack.push("0".to_string()); // Error case
                        }
                    }
                },
                IR::BitwiseNot => {
                    if let Some(a) = self.stack.pop() {
                        if let Ok(a_num) = a.parse::<i64>() {
                            self.stack.push((!a_num).to_string());
                        } else {
                            self.stack.push("0".to_string()); // Error case
                        }
                    }
                },
                IR::LeftShift => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<i64>(), b.parse::<i64>()) {
                            self.stack.push((a_num << b_num).to_string());
                        } else {
                            self.stack.push("0".to_string()); // Error case
                        }
                    }
                },
                IR::RightShift => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<i64>(), b.parse::<i64>()) {
                            self.stack.push((a_num >> b_num).to_string());
                        } else {
                            self.stack.push("0".to_string()); // Error case
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

    fn is_builtin(&self, name: &str) -> bool {
        matches!(name, "print" | "println" | "input" | "read" | "write" | "len" | "append" | "remove" | "toint" | "tofloat" | "tostr" | "tobool" | "create_range" | "array_get" | "concat_string" | "load_var_by_name")
    }

    fn execute_builtin(&mut self, name: &str, arg_count: usize) -> Result<(), String> {
        match name {
            "print" => {
                if let Some(value) = self.stack.pop() {
                    print!("{}", value);
                    io::stdout().flush().unwrap();
                }
                // Push null as return value
                self.stack.push("null".to_string());
            },
            "println" => {
                if let Some(value) = self.stack.pop() {
                    println!("{}", value);
                } else {
                    println!();
                }
                // Push null as return value
                self.stack.push("null".to_string());
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
                self.stack.push(line);
            },
            "len" => {
                if let Some(value) = self.stack.pop() {
                    self.stack.push(value.len().to_string());
                } else {
                    self.stack.push("0".to_string());
                }
            },
            // Dot notation type conversion methods
            "toint" => {
                if let Some(value) = self.stack.pop() {
                    match value.trim().parse::<i64>() {
                        Ok(int_val) => self.stack.push(int_val.to_string()),
                        Err(_) => {
                            // Try to convert from float first
                            match value.trim().parse::<f64>() {
                                Ok(float_val) => self.stack.push((float_val as i64).to_string()),
                                Err(_) => {
                                    // Try boolean conversion
                                    if value == "true" || value == "True" {
                                        self.stack.push("1".to_string());
                                    } else if value == "false" || value == "False" {
                                        self.stack.push("0".to_string());
                                    } else {
                                        return Err(format!("Cannot convert '{}' to int", value));
                                    }
                                }
                            }
                        }
                    }
                } else {
                    return Err("toint() requires one argument".to_string());
                }
            },
            "tofloat" => {
                if let Some(value) = self.stack.pop() {
                    match value.trim().parse::<f64>() {
                        Ok(float_val) => self.stack.push(float_val.to_string()),
                        Err(_) => {
                            // Try boolean conversion
                            if value == "true" || value == "True" {
                                self.stack.push("1.0".to_string());
                            } else if value == "false" || value == "False" {
                                self.stack.push("0.0".to_string());
                            } else {
                                return Err(format!("Cannot convert '{}' to float", value));
                            }
                        }
                    }
                } else {
                    return Err("tofloat() requires one argument".to_string());
                }
            },
            "tostr" => {
                if let Some(value) = self.stack.pop() {
                    // Everything can be converted to string
                    self.stack.push(value);
                } else {
                    return Err("tostr() requires one argument".to_string());
                }
            },
            "tobool" => {
                if let Some(value) = self.stack.pop() {
                    let bool_val = match value.as_str() {
                        "true" | "True" | "1" => "true",
                        "false" | "False" | "0" | "" | "null" => "false",
                        _ => {
                            // Non-empty strings are truthy, try to parse as number
                            match value.trim().parse::<f64>() {
                                Ok(num) => if num != 0.0 { "true" } else { "false" },
                                Err(_) => if !value.trim().is_empty() { "true" } else { "false" }
                            }
                        }
                    };
                    self.stack.push(bool_val.to_string());
                } else {
                    return Err("tobool() requires one argument".to_string());
                }
            },
            "create_range" => {
                // Create a range object from start, end, and inclusive flag
                if arg_count >= 3 {
                    let inclusive = self.stack.pop().unwrap_or("false".to_string());
                    let end = self.stack.pop().unwrap_or("0".to_string());
                    let start = self.stack.pop().unwrap_or("0".to_string());
                    
                    // Store range as a formatted string "start..end" or "start..=end"
                    let range_str = if inclusive == "true" {
                        format!("{}..={}", start, end)
                    } else {
                        format!("{}..{}", start, end)
                    };
                    self.stack.push(range_str);
                } else {
                    return Err("create_range() requires 3 arguments (start, end, inclusive)".to_string());
                }
            },
            "array_get" => {
                // Get array element by index (simplified implementation)
                if arg_count >= 2 {
                    let array_name = self.stack.pop().unwrap_or("array_0".to_string());
                    let index = self.stack.pop().unwrap_or("0".to_string());
                    
                    // Try to parse index as number
                    if let Ok(idx) = index.parse::<usize>() {
                        let var_name = format!("{}{}", array_name, idx);
                        if let Some(value) = self.variables.get(&var_name) {
                            self.stack.push(value.clone());
                        } else {
                            // Debug: print available variables
                            if !self.clean_output {
                                println!("Looking for variable '{}', available: {:?}", var_name, self.variables.keys().collect::<Vec<_>>());
                            }
                            self.stack.push("null".to_string());
                        }
                    } else {
                        self.stack.push("null".to_string());
                    }
                } else {
                    return Err("array_get() requires 2 arguments (array, index)".to_string());
                }
            },
            "concat_string" => {
                // Concatenate two strings/values
                if arg_count >= 2 {
                    let second = self.stack.pop().unwrap_or("".to_string());
                    let first = self.stack.pop().unwrap_or("".to_string());
                    let result = format!("{}{}", second, first); // Note: reversed order due to stack
                    self.stack.push(result);
                } else {
                    return Err("concat_string() requires 2 arguments".to_string());
                }
            },
            "load_var_by_name" => {
                // Load a variable by its name (from stack)
                if arg_count >= 1 {
                    let var_name = self.stack.pop().unwrap_or("".to_string());
                    if let Some(value) = self.variables.get(&var_name) {
                        self.stack.push(value.clone());
                    } else {
                        if !self.clean_output {
                            println!("Variable '{}' not found, available: {:?}", var_name, self.variables.keys().collect::<Vec<_>>());
                        }
                        self.stack.push("null".to_string());
                    }
                } else {
                    return Err("load_var_by_name() requires 1 argument".to_string());
                }
            },
            _ => {
                // Pop arguments for unimplemented builtins
                for _ in 0..arg_count {
                    self.stack.pop();
                }
                self.stack.push("null".to_string());
            }
        }
        Ok(())
    }
}

/// Helper function for boolean logic
fn is_truthy(s: &str) -> bool {
    !matches!(s, "false" | "0" | "" | "null" | "undefined" | "False")
}
