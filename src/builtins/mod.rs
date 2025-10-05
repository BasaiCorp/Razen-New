// src/builtins/mod.rs
//! Standard library builtins - Native Rust implementations

pub mod string;
pub mod math;
pub mod array;
pub mod file;
pub mod system;

use crate::backend::execution::value::Value;

/// Check if a function exists in stdlib
pub fn has_function(name: &str) -> bool {
    matches!(name,
        // String functions
        "split" | "join" | "trim" | "upper" | "lower" | "replace" | 
        "contains" | "starts_with" | "ends_with" | "substring" | "repeat" | "reverse" |
        // Math functions
        "abs" | "sqrt" | "pow" | "floor" | "ceil" | "round" | 
        "min" | "max" | "sin" | "cos" | "tan" | "random" | "random_range" |
        // Array functions
        "push" | "pop" | "shift" | "unshift" | "array_reverse" | "includes" |
        // File functions
        "read" | "write" | "append" | "delete" | "exists" | 
        "is_file" | "is_dir" | "list_dir" | "create_dir" | "delete_dir" |
        // System functions
        "exit" | "sleep" | "time" | "env" | "set_env" | "args" | "platform"
    )
}

/// Call a stdlib function by name
pub fn call_function(name: &str, stack: &mut Vec<Value>) -> Result<(), String> {
    match name {
        // String library functions
        "split" => string::split(stack),
        "join" => string::join(stack),
        "trim" => string::trim(stack),
        "upper" => string::upper(stack),
        "lower" => string::lower(stack),
        "replace" => string::replace(stack),
        "contains" => string::contains(stack),
        "starts_with" => string::starts_with(stack),
        "ends_with" => string::ends_with(stack),
        "substring" => string::substring(stack),
        "repeat" => string::repeat(stack),
        "reverse" => string::reverse(stack),
        
        // Math library functions
        "abs" => math::abs(stack),
        "sqrt" => math::sqrt(stack),
        "pow" => math::pow(stack),
        "floor" => math::floor(stack),
        "ceil" => math::ceil(stack),
        "round" => math::round(stack),
        "min" => math::min(stack),
        "max" => math::max(stack),
        "sin" => math::sin(stack),
        "cos" => math::cos(stack),
        "tan" => math::tan(stack),
        "random" => math::random(stack),
        "random_range" => math::random_range(stack),
        
        // Array library functions
        "push" => array::push(stack),
        "pop" => array::pop(stack),
        "shift" => array::shift(stack),
        "unshift" => array::unshift(stack),
        "array_reverse" => array::reverse(stack),
        "includes" => array::includes(stack),
        
        // File library functions
        "read" => file::read(stack),
        "write" => file::write(stack),
        "append" => file::append(stack),
        "delete" => file::delete(stack),
        "exists" => file::exists(stack),
        "is_file" => file::is_file(stack),
        "is_dir" => file::is_dir(stack),
        "list_dir" => file::list_dir(stack),
        "create_dir" => file::create_dir(stack),
        "delete_dir" => file::delete_dir(stack),
        
        // System library functions
        "exit" => system::exit(stack),
        "sleep" => system::sleep(stack),
        "time" => system::time(stack),
        "env" => system::env(stack),
        "set_env" => system::set_env(stack),
        "args" => system::args(stack),
        "platform" => system::platform(stack),
        
        _ => Err(format!("[ERROR] Unknown stdlib function: {}", name))
    }
}
