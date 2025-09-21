// src/backend/builtins.rs

use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

use crate::backend::semantic::Type;

/// Registry for all builtin functions in the Razen language
#[derive(Debug, Clone)]
pub struct BuiltinRegistry {
    functions: HashMap<String, BuiltinFunction>,
}

/// Represents a builtin function with its signature and implementation
#[derive(Debug, Clone)]
pub struct BuiltinFunction {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub implementation: BuiltinImplementation,
}

/// Different types of builtin function implementations
#[derive(Debug, Clone)]
pub enum BuiltinImplementation {
    /// Console I/O functions
    Print,
    Println,
    Input,
    
    /// File I/O functions (placeholders for now)
    Read,
    Write,
    Open,
    Close,
}

/// Runtime for executing builtin functions
pub struct BuiltinRuntime {
    registry: BuiltinRegistry,
}

/// Global registry instance
static BUILTIN_REGISTRY: Lazy<Arc<Mutex<BuiltinRegistry>>> = Lazy::new(|| {
    Arc::new(Mutex::new(BuiltinRegistry::new()))
});

impl BuiltinRegistry {
    /// Create a new builtin registry with all I/O functions
    pub fn new() -> Self {
        let mut registry = BuiltinRegistry {
            functions: HashMap::new(),
        };
        
        registry.register_io_functions();
        registry
    }
    
    /// Register all I/O builtin functions
    fn register_io_functions(&mut self) {
        // Console output without newline
        self.register_function(BuiltinFunction {
            name: "print".to_string(),
            params: vec![("value".to_string(), Type::Any)],
            return_type: Type::Void,
            implementation: BuiltinImplementation::Print,
        });
        
        // Console output with newline
        self.register_function(BuiltinFunction {
            name: "println".to_string(),
            params: vec![("value".to_string(), Type::Any)],
            return_type: Type::Void,
            implementation: BuiltinImplementation::Println,
        });
        
        // Console input with optional prompt
        self.register_function(BuiltinFunction {
            name: "input".to_string(),
            params: vec![("prompt".to_string(), Type::String)],
            return_type: Type::String,
            implementation: BuiltinImplementation::Input,
        });
        
        // File I/O functions (placeholders)
        self.register_function(BuiltinFunction {
            name: "read".to_string(),
            params: vec![("filename".to_string(), Type::String)],
            return_type: Type::String,
            implementation: BuiltinImplementation::Read,
        });
        
        self.register_function(BuiltinFunction {
            name: "write".to_string(),
            params: vec![
                ("filename".to_string(), Type::String),
                ("content".to_string(), Type::String)
            ],
            return_type: Type::Bool,
            implementation: BuiltinImplementation::Write,
        });
        
        self.register_function(BuiltinFunction {
            name: "open".to_string(),
            params: vec![
                ("filename".to_string(), Type::String),
                ("mode".to_string(), Type::String)
            ],
            return_type: Type::Any, // File handle type
            implementation: BuiltinImplementation::Open,
        });
        
        self.register_function(BuiltinFunction {
            name: "close".to_string(),
            params: vec![("handle".to_string(), Type::Any)],
            return_type: Type::Bool,
            implementation: BuiltinImplementation::Close,
        });
    }
    
    /// Register a builtin function
    fn register_function(&mut self, function: BuiltinFunction) {
        self.functions.insert(function.name.clone(), function);
    }
    
    /// Get a builtin function by name
    pub fn get_function(&self, name: &str) -> Option<&BuiltinFunction> {
        self.functions.get(name)
    }
    
    /// Get all builtin function names
    pub fn get_function_names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
    
    /// Check if a function is a builtin
    pub fn is_builtin(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
    
    /// Validate a builtin function call
    pub fn validate_call(&self, name: &str, arg_types: &[Type]) -> Result<Type, String> {
        if let Some(function) = self.get_function(name) {
            // Special case for input() which can have 0 or 1 parameters
            if name == "input" {
                if arg_types.is_empty() || arg_types.len() == 1 {
                    return Ok(function.return_type.clone());
                } else {
                    return Err(format!("Function '{}' expects 0 or 1 arguments, got {}", name, arg_types.len()));
                }
            }
            
            // Standard parameter validation
            if function.params.len() != arg_types.len() {
                return Err(format!(
                    "Function '{}' expects {} arguments, got {}",
                    name, function.params.len(), arg_types.len()
                ));
            }
            
            // Type compatibility check (relaxed for Type::Any)
            for (i, ((_, expected_type), actual_type)) in function.params.iter().zip(arg_types.iter()).enumerate() {
                if *expected_type != Type::Any && *actual_type != Type::Any && expected_type != actual_type {
                    return Err(format!(
                        "Argument {} of function '{}' expects type '{}', got '{}'",
                        i + 1, name, expected_type, actual_type
                    ));
                }
            }
            
            Ok(function.return_type.clone())
        } else {
            Err(format!("Unknown builtin function: '{}'", name))
        }
    }
}

impl BuiltinRuntime {
    /// Create a new builtin runtime
    pub fn new() -> Self {
        BuiltinRuntime {
            registry: BuiltinRegistry::new(),
        }
    }
    
    /// Execute a builtin function call
    pub fn execute(&self, name: &str, args: Vec<String>) -> Result<String, String> {
        if let Some(function) = self.registry.get_function(name) {
            match &function.implementation {
                BuiltinImplementation::Print => {
                    if args.len() != 1 {
                        return Err("print() expects exactly 1 argument".to_string());
                    }
                    print!("{}", args[0]);
                    io::stdout().flush().unwrap_or(());
                    Ok("".to_string()) // void return
                }
                
                BuiltinImplementation::Println => {
                    if args.len() != 1 {
                        return Err("println() expects exactly 1 argument".to_string());
                    }
                    println!("{}", args[0]);
                    Ok("".to_string()) // void return
                }
                
                BuiltinImplementation::Input => {
                    match args.len() {
                        0 => {
                            // input() with no prompt
                            let mut input = String::new();
                            io::stdin().read_line(&mut input)
                                .map_err(|e| format!("Failed to read input: {}", e))?;
                            Ok(input.trim().to_string())
                        }
                        1 => {
                            // input(prompt)
                            print!("{}", args[0]);
                            io::stdout().flush().unwrap_or(());
                            let mut input = String::new();
                            io::stdin().read_line(&mut input)
                                .map_err(|e| format!("Failed to read input: {}", e))?;
                            Ok(input.trim().to_string())
                        }
                        _ => Err("input() expects 0 or 1 arguments".to_string())
                    }
                }
                
                // File I/O placeholders
                BuiltinImplementation::Read => {
                    // TODO: Implement file reading
                    Err("read() function not yet implemented".to_string())
                }
                
                BuiltinImplementation::Write => {
                    // TODO: Implement file writing
                    Err("write() function not yet implemented".to_string())
                }
                
                BuiltinImplementation::Open => {
                    // TODO: Implement file opening
                    Err("open() function not yet implemented".to_string())
                }
                
                BuiltinImplementation::Close => {
                    // TODO: Implement file closing
                    Err("close() function not yet implemented".to_string())
                }
            }
        } else {
            Err(format!("Unknown builtin function: '{}'", name))
        }
    }
    
    /// Get the registry
    pub fn registry(&self) -> &BuiltinRegistry {
        &self.registry
    }
}

/// Global functions for accessing the builtin registry
impl BuiltinRegistry {
    /// Get the global builtin registry
    pub fn global() -> Arc<Mutex<BuiltinRegistry>> {
        BUILTIN_REGISTRY.clone()
    }
    
    /// Initialize the global builtin registry
    pub fn initialize() {
        let _registry = BUILTIN_REGISTRY.clone();
        println!("✅ Builtin functions initialized: {:?}", 
            _registry.lock().unwrap().get_function_names());
    }
}

impl Default for BuiltinRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BuiltinRuntime {
    fn default() -> Self {
        Self::new()
    }
}