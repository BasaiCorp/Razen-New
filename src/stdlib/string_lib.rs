// src/stdlib/string_lib.rs
//! String manipulation standard library - Native Rust implementation

use crate::backend::execution::value::Value;

/// Check if function exists in this module
pub fn has_function(name: &str) -> bool {
    matches!(name, "upper" | "lower" | "trim" | "split" | "join" | "contains" | 
             "starts_with" | "ends_with" | "replace" | "reverse" | "repeat" | "char_at")
}

/// Get list of all functions
pub fn get_function_list() -> Vec<&'static str> {
    vec![
        "upper", "lower", "trim", "split", "join", "contains",
        "starts_with", "ends_with", "replace", "reverse", "repeat", "char_at"
    ]
}

/// Call a string library function
pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "upper" => upper(args),
        "lower" => lower(args),
        "trim" => trim(args),
        "split" => split(args),
        "join" => join(args),
        "contains" => contains(args),
        "starts_with" => starts_with(args),
        "ends_with" => ends_with(args),
        "replace" => replace(args),
        "reverse" => reverse(args),
        "repeat" => repeat(args),
        "char_at" => char_at(args),
        _ => Err(format!("Unknown string function: {}", name)),
    }
}

fn upper(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("upper() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err("upper() requires a string argument".to_string()),
    }
}

fn lower(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("lower() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err("lower() requires a string argument".to_string()),
    }
}

fn trim(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("trim() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        _ => Err("trim() requires a string argument".to_string()),
    }
}

fn split(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("split() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(delimiter)) => {
            let parts: Vec<Value> = s.split(delimiter.as_str())
                .map(|part| Value::String(part.to_string()))
                .collect();
            Ok(Value::Array(parts))
        }
        _ => Err("split() requires two string arguments".to_string()),
    }
}

fn join(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("join() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::Array(arr), Value::String(separator)) => {
            let strings: Vec<String> = arr.iter()
                .map(|v| v.to_string())
                .collect();
            Ok(Value::String(strings.join(separator)))
        }
        _ => Err("join() requires an array and a string separator".to_string()),
    }
}

fn contains(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("contains() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(substr)) => {
            Ok(Value::Boolean(s.contains(substr.as_str())))
        }
        _ => Err("contains() requires two string arguments".to_string()),
    }
}

fn starts_with(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("starts_with() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(prefix)) => {
            Ok(Value::Boolean(s.starts_with(prefix.as_str())))
        }
        _ => Err("starts_with() requires two string arguments".to_string()),
    }
}

fn ends_with(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("ends_with() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(suffix)) => {
            Ok(Value::Boolean(s.ends_with(suffix.as_str())))
        }
        _ => Err("ends_with() requires two string arguments".to_string()),
    }
}

fn replace(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("replace() takes exactly 3 arguments".to_string());
    }
    match (&args[0], &args[1], &args[2]) {
        (Value::String(s), Value::String(old), Value::String(new)) => {
            Ok(Value::String(s.replace(old.as_str(), new.as_str())))
        }
        _ => Err("replace() requires three string arguments".to_string()),
    }
}

fn reverse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("reverse() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::String(s) => {
            Ok(Value::String(s.chars().rev().collect()))
        }
        _ => Err("reverse() requires a string argument".to_string()),
    }
}

fn repeat(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("repeat() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::String(s), Value::Integer(count)) => {
            if *count < 0 {
                return Err("repeat() count must be non-negative".to_string());
            }
            Ok(Value::String(s.repeat(*count as usize)))
        }
        _ => Err("repeat() requires a string and an integer".to_string()),
    }
}

fn char_at(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("char_at() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::String(s), Value::Integer(index)) => {
            if *index < 0 || *index >= s.len() as i64 {
                return Err(format!("Index {} out of bounds for string of length {}", index, s.len()));
            }
            let ch = s.chars().nth(*index as usize)
                .ok_or("Invalid index".to_string())?;
            Ok(Value::String(ch.to_string()))
        }
        _ => Err("char_at() requires a string and an integer index".to_string()),
    }
}
