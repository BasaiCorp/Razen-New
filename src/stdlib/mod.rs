// src/stdlib/mod.rs
//! Standard Library for Razen Language
//! 
//! Native Rust implementation of standard library functions
//! Usage in Razen:
//!   - use string  (without quotes - stdlib module)
//!   - use "path/to/module"  (with quotes - file module)
//! 
//! Call syntax: string.upper("razen")
//!
//! DYNAMIC MODULE SYSTEM:
//! To add a new stdlib module:
//! 1. Create a new file like `mymodule_lib.rs` in src/stdlib/
//! 2. Add it to the STDLIB_MODULES macro below
//! 3. That's it! The module will be automatically registered

pub mod string_lib;
pub mod math_lib;
pub mod array_lib;
pub mod file_lib;
pub mod json_lib;
pub mod time_lib;
pub mod http_lib;
pub mod os_lib;
pub mod regex_lib;
pub mod random_lib;
pub mod server_lib;

use crate::backend::execution::value::Value;

/// Macro to define stdlib modules dynamically
/// Add new modules here and they will be automatically registered everywhere
macro_rules! define_stdlib_modules {
    ($(($name:expr, $module:ident, $description:expr)),* $(,)?) => {
        /// Check if a module name is a standard library module
        pub fn is_stdlib_module(name: &str) -> bool {
            matches!(name, $($name)|*)
        }

        /// Check if a function call is a stdlib function
        pub fn is_stdlib_function(module: &str, function: &str) -> bool {
            match module {
                $($name => $module::has_function(function),)*
                _ => false,
            }
        }

        /// Call a stdlib function with arguments
        pub fn call_stdlib_function(module: &str, function: &str, args: Vec<Value>) -> Result<Value, String> {
            match module {
                $($name => $module::call_function(function, args),)*
                _ => Err(format!("Unknown stdlib module: {}", module)),
            }
        }

        /// Get all available stdlib module names
        pub fn get_stdlib_modules() -> Vec<&'static str> {
            vec![$($name),*]
        }

        /// Get stdlib module metadata
        pub fn get_module_info(name: &str) -> Option<ModuleInfo> {
            match name {
                $(
                    $name => Some(ModuleInfo {
                        name: $name,
                        description: $description,
                        version: "0.1.0",
                        functions: $module::get_function_list(),
                    }),
                )*
                _ => None,
            }
        }
    };
}

// STDLIB MODULE REGISTRY
// Add new modules here - they will be automatically registered everywhere!
define_stdlib_modules! {
    ("string", string_lib, "String manipulation and utilities"),
    ("math", math_lib, "Mathematical functions and constants"),
    ("arr", array_lib, "Array/List manipulation utilities"),
    ("file", file_lib, "File I/O operations"),
    ("json", json_lib, "JSON parsing and serialization"),
    ("time", time_lib, "Time and date operations"),
    ("http", http_lib, "HTTP client for web requests"),
    ("os", os_lib, "Operating system operations"),
    ("regex", regex_lib, "Regular expression matching"),
    ("random", random_lib, "Random number generation"),
    ("server", server_lib, "HTTP web server like Go and Node.js"),
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub version: &'static str,
    pub functions: Vec<&'static str>,
}
