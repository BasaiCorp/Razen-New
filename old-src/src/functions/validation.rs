use crate::value::Value;
use regex::Regex;

/// Validates if a string is a valid email
/// Example: email("a@b.com") => true
pub fn email(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Validation.email requires exactly 1 argument: email_string".to_string());
    }
    
    let email_str = args[0].as_string()?;
    
    // Simple email regex pattern
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .map_err(|e| format!("Invalid regex pattern: {}", e))?;
    
    Ok(Value::Bool(email_regex.is_match(&email_str)))
}

/// Validates if a string is a valid phone number
/// Example: phone("1234567890") => true
pub fn phone(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Validation.phone requires exactly 1 argument: phone_string".to_string());
    }
    
    let phone_str = args[0].as_string()?;
    
    // Simple phone regex pattern (supports various formats)
    let phone_regex = Regex::new(r"^(\+\d{1,3}[- ]?)?\d{10}$|^(\+\d{1,3}[- ]?)?\d{3}[- ]?\d{3}[- ]?\d{4}$")
        .map_err(|e| format!("Invalid regex pattern: {}", e))?;
    
    Ok(Value::Bool(phone_regex.is_match(&phone_str)))
}

/// Checks if a value is not null or empty
/// Example: required("abc") => true
pub fn required(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Validation.required requires exactly 1 argument: value".to_string());
    }
    
    match &args[0] {
        Value::Null => Ok(Value::Bool(false)),
        Value::String(s) => Ok(Value::Bool(!s.is_empty())),
        Value::Array(arr) => Ok(Value::Bool(!arr.is_empty())),
        Value::Map(map) => Ok(Value::Bool(!map.is_empty())),
        _ => Ok(Value::Bool(true)), // Other types are considered non-empty
    }
}

/// Checks if a string has at least the minimum length
/// Example: min_length("abc", 2) => true
pub fn min_length(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Validation.min_length requires exactly 2 arguments: string, min_length".to_string());
    }
    
    let string = args[0].as_string()?;
    
    let min_len = match &args[1] {
        Value::Int(n) => {
            if *n < 0 {
                return Err("Minimum length cannot be negative".to_string());
            }
            *n as usize
        },
        _ => return Err("Second argument must be an integer".to_string()),
    };
    
    Ok(Value::Bool(string.len() >= min_len))
}
