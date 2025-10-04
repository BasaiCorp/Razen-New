// src/stdlib/mod.rs
//! Standard Library for Razen Language
//! 
//! Native Rust implementation of standard library functions
//! Usage in Razen: use string, use math, use array, use file, use json
//! (without quotes - these are built-in stdlib modules)

pub mod string_lib;
pub mod math_lib;
pub mod array_lib;
pub mod file_lib;
pub mod json_lib;

use crate::backend::execution::value::Value;

/// Check if a module name is a standard library module
pub fn is_stdlib_module(name: &str) -> bool {
    matches!(name, "string" | "math" | "array" | "file" | "json")
}

/// Check if a function call is a stdlib function
pub fn is_stdlib_function(module: &str, function: &str) -> bool {
    match module {
        "string" => string_lib::has_function(function),
        "math" => math_lib::has_function(function),
        "array" => array_lib::has_function(function),
        "file" => file_lib::has_function(function),
        "json" => json_lib::has_function(function),
        _ => false,
    }
}

/// Call a stdlib function with arguments
pub fn call_stdlib_function(module: &str, function: &str, args: Vec<Value>) -> Result<Value, String> {
    match module {
        "string" => string_lib::call_function(function, args),
        "math" => math_lib::call_function(function, args),
        "array" => array_lib::call_function(function, args),
        "file" => file_lib::call_function(function, args),
        "json" => json_lib::call_function(function, args),
        _ => Err(format!("Unknown stdlib module: {}", module)),
    }
}

/// Get all available stdlib module names
pub fn get_stdlib_modules() -> Vec<&'static str> {
    vec!["string", "math", "array", "file", "json"]
}

/// Get stdlib module metadata
pub fn get_module_info(name: &str) -> Option<ModuleInfo> {
    match name {
        "string" => Some(ModuleInfo {
            name: "string",
            description: "String manipulation and utilities",
            version: "0.1.0",
            functions: string_lib::get_function_list(),
        }),
        "math" => Some(ModuleInfo {
            name: "math",
            description: "Mathematical functions and constants",
            version: "0.1.0",
            functions: math_lib::get_function_list(),
        }),
        "array" => Some(ModuleInfo {
            name: "array",
            description: "Array/List manipulation utilities",
            version: "0.1.0",
            functions: array_lib::get_function_list(),
        }),
        "file" => Some(ModuleInfo {
            name: "file",
            description: "File I/O operations",
            version: "0.1.0",
            functions: file_lib::get_function_list(),
        }),
        "json" => Some(ModuleInfo {
            name: "json",
            description: "JSON parsing and serialization",
            version: "0.1.0",
            functions: json_lib::get_function_list(),
        }),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub version: &'static str,
    pub functions: Vec<&'static str>,
}
