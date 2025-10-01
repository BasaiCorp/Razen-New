// src/backend/execution/runtime_optimized.rs
//! Highly optimized runtime execution engine with typed values

use std::collections::HashMap;
use std::io::{self, Write, BufRead};
use std::{thread, time::Duration};
use super::ir::IR;
use super::value::Value;

/// Optimized runtime execution engine with typed stack
pub struct OptimizedRuntime {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    functions: HashMap<String, usize>,
    call_stack: Vec<(usize, HashMap<String, Value>)>,
    function_params: HashMap<String, Vec<String>>,
    clean_output: bool,
}

impl OptimizedRuntime {
    pub fn new() -> Self {
        OptimizedRuntime {
            stack: Vec::with_capacity(1024), // Pre-allocate for performance
            variables: HashMap::with_capacity(256),
            functions: HashMap::new(),
            call_stack: Vec::new(),
            function_params: HashMap::new(),
            clean_output: false,
        }
    }

    pub fn set_clean_output(&mut self, clean: bool) {
        self.clean_output = clean;
    }

    pub fn register_function_params(&mut self, name: String, params: Vec<String>) {
        self.function_params.insert(name, params);
    }

    /// Execute IR instructions with optimized typed operations
    pub fn execute(&mut self, ir: &[IR]) -> Result<(), String> {
        if !self.clean_output {
            println!("Starting Razen optimized execution engine...");
            println!("Generated {} IR instructions", ir.len());
        }

        // Pre-pass: register functions
        for (i, instruction) in ir.iter().enumerate() {
            if let IR::DefineFunction(name, actual_addr) = instruction {
                self.functions.insert(name.clone(), *actual_addr);
                if !self.clean_output {
                    println!("Registered function '{}' at address {} (instruction at {})", name, actual_addr, i);
                }
            }
        }

        // Pre-execute module initialization
        let mut init_end_pc = 0;
        for (i, instruction) in ir.iter().enumerate() {
            if matches!(instruction, IR::DefineFunction(_, _)) {
                init_end_pc = i;
                break;
            }
            
            match instruction {
                IR::PushNumber(n) => self.stack.push(Value::Number(*n)),
                IR::PushString(s) => self.stack.push(Value::String(s.clone())),
                IR::PushBoolean(b) => self.stack.push(Value::Boolean(*b)),
                IR::PushNull => self.stack.push(Value::Null),
                IR::StoreVar(name) => {
                    if let Some(value) = self.stack.pop() {
                        self.variables.insert(name.clone(), value);
                    }
                },
                _ => {}
            }
        }
        
        self.stack.clear();

        // Main execution loop
        let mut pc = 0;
        while pc < ir.len() {
            let instruction = &ir[pc];
            
            match instruction {
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
                        if name.contains('.') {
                            self.variables.insert(name.clone(), value);
                        } else if let Some((_, func_vars)) = self.call_stack.last_mut() {
                            func_vars.insert(name.clone(), value);
                        } else {
                            self.variables.insert(name.clone(), value);
                        }
                    }
                },
                IR::LoadVar(name) => {
                    let value = if name.contains('.') {
                        self.variables.get(name)
                    } else if let Some((_, func_vars)) = self.call_stack.last() {
                        func_vars.get(name).or_else(|| self.variables.get(name))
                    } else {
                        self.variables.get(name)
                    };

                    if let Some(val) = value {
                        self.stack.push(val.clone());
                    } else {
                        self.stack.push(Value::Null);
                    }
                },
                IR::SetGlobal(name) => {
                    if let Some(value) = self.stack.pop() {
                        self.variables.insert(name.clone(), value);
                    }
                },
                // Optimized arithmetic operations
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
                // Optimized comparison operations
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
                    if self.is_builtin(name) {
                        self.execute_builtin(name, *arg_count)?;
                    } else {
                        let mut args = Vec::new();
                        for _ in 0..*arg_count {
                            if let Some(arg) = self.stack.pop() {
                                args.push(arg);
                            }
                        }
                        args.reverse();
                        
                        if let Some(&func_addr) = self.functions.get(name) {
                            let mut func_variables = HashMap::new();
                            
                            if let Some(param_names) = self.function_params.get(name) {
                                for (i, param_name) in param_names.iter().enumerate() {
                                    if i < args.len() {
                                        func_variables.insert(param_name.clone(), args[i].clone());
                                    } else {
                                        func_variables.insert(param_name.clone(), Value::Null);
                                    }
                                }
                            }
                            
                            self.call_stack.push((pc + 1, func_variables));
                            pc = func_addr;
                            continue;
                        } else {
                            self.stack.push(Value::Null);
                        }
                    }
                },
                IR::Return => {
                    let return_value = self.stack.pop().unwrap_or(Value::Null);
                    if let Some((return_addr, _)) = self.call_stack.pop() {
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
                IR::BitwiseAnd => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_int), Some(b_int)) = (a.to_integer(), b.to_integer()) {
                            self.stack.push(Value::Integer(a_int & b_int));
                        } else {
                            self.stack.push(Value::Integer(0));
                        }
                    }
                },
                IR::BitwiseOr => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_int), Some(b_int)) = (a.to_integer(), b.to_integer()) {
                            self.stack.push(Value::Integer(a_int | b_int));
                        } else {
                            self.stack.push(Value::Integer(0));
                        }
                    }
                },
                IR::BitwiseXor => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_int), Some(b_int)) = (a.to_integer(), b.to_integer()) {
                            self.stack.push(Value::Integer(a_int ^ b_int));
                        } else {
                            self.stack.push(Value::Integer(0));
                        }
                    }
                },
                IR::BitwiseNot => {
                    if let Some(a) = self.stack.pop() {
                        if let Some(a_int) = a.to_integer() {
                            self.stack.push(Value::Integer(!a_int));
                        } else {
                            self.stack.push(Value::Integer(0));
                        }
                    }
                },
                IR::LeftShift => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_int), Some(b_int)) = (a.to_integer(), b.to_integer()) {
                            self.stack.push(Value::Integer(a_int << b_int));
                        } else {
                            self.stack.push(Value::Integer(0));
                        }
                    }
                },
                IR::RightShift => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if let (Some(a_int), Some(b_int)) = (a.to_integer(), b.to_integer()) {
                            self.stack.push(Value::Integer(a_int >> b_int));
                        } else {
                            self.stack.push(Value::Integer(0));
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

        Ok(())
    }

    fn is_builtin(&self, name: &str) -> bool {
        matches!(name, "print" | "println" | "printc" | "printlnc" | "input" | "len")
    }

    fn execute_builtin(&mut self, name: &str, arg_count: usize) -> Result<(), String> {
        match name {
            "print" => {
                if let Some(value) = self.stack.pop() {
                    print!("{}", value);
                    io::stdout().flush().unwrap();
                }
                self.stack.push(Value::Null);
            },
            "println" => {
                if let Some(value) = self.stack.pop() {
                    println!("{}", value);
                } else {
                    println!();
                }
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
                    let len = match value {
                        Value::String(s) => s.len(),
                        _ => 0,
                    };
                    self.stack.push(Value::Integer(len as i64));
                } else {
                    self.stack.push(Value::Integer(0));
                }
            },
            _ => {
                for _ in 0..arg_count {
                    self.stack.pop();
                }
                self.stack.push(Value::Null);
            }
        }
        Ok(())
    }
}
