use crate::value::Value;

/// Convert a string to uppercase
/// Example: upper("hello") => "HELLO"
pub fn upper(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("String.upper requires exactly 1 argument: string".to_string());
    }
    let string = args[0].as_string()?;
    Ok(Value::String(string.to_uppercase()))
}

/// Convert a string to lowercase
/// Example: lower("HELLO") => "hello"
pub fn lower(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("String.lower requires exactly 1 argument: string".to_string());
    }
    let string = args[0].as_string()?;
    Ok(Value::String(string.to_lowercase()))
}

/// Get a substring from a string
/// Example: substring("hello", 1, 3) => "el"
pub fn substring(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("String.substring requires exactly 3 arguments: string, start, end".to_string());
    }
    
    let string = args[0].as_string()?;
    
    let start = match &args[1] {
        Value::Int(n) => *n as usize,
        Value::Float(n) => *n as usize,
        _ => return Err(format!("Second argument to substring must be a number, got {:?}", args[1])),
    };
    
    let end = match &args[2] {
        Value::Int(n) => *n as usize,
        Value::Float(n) => *n as usize,
        _ => return Err(format!("Third argument to substring must be a number, got {:?}", args[2])),
    };
    
    if start > string.len() || end > string.len() || start > end {
        return Err(format!("Invalid substring range: {}..{} for string of length {}", start, end, string.len()));
    }
    
    // Safe substring operation (handles UTF-8)
    let chars: Vec<char> = string.chars().collect();
    let result: String = chars[start..end].iter().collect();
    
    Ok(Value::String(result))
}

/// Replace occurrences of a substring in a string
/// Example: replace("hello world", "world", "razen") => "hello razen"
pub fn replace(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("String.replace requires exactly 3 arguments: string, pattern, replacement".to_string());
    }
    
    let string = args[0].as_string()?;
    let pattern = args[1].as_string()?;
    let replacement = args[2].as_string()?;
    
    Ok(Value::String(string.replace(&pattern, &replacement)))
}

/// Get the length of a string
/// Example: length("hello") => 5
pub fn length(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("String.length requires exactly 1 argument: string".to_string());
    }
    
    let string = args[0].as_string()?;
    
    // Count Unicode characters, not bytes
    Ok(Value::Int(string.chars().count() as i64))
}

/// Split a string by a delimiter
/// Example: split("a,b,c", ",") => ["a", "b", "c"]
pub fn split(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("String.split requires exactly 2 arguments: string, delimiter".to_string());
    }
    
    let string = args[0].as_string()?;
    let delimiter = args[1].as_string()?;
    
    let parts: Vec<Value> = string.split(&delimiter)
        .map(|s| Value::String(s.to_string()))
        .collect();
    
    Ok(Value::Array(parts))
}

/// Trim whitespace from a string
/// Example: trim("  hello  ") => "hello"
pub fn trim(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("String.trim requires exactly 1 argument: string".to_string());
    }
    
    let string = args[0].as_string()?;
    
    Ok(Value::String(string.trim().to_string()))
}

/// Check if a string starts with a prefix
/// Example: starts_with("hello", "he") => true
pub fn starts_with(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("String.starts_with requires exactly 2 arguments: string, prefix".to_string());
    }
    
    let string = args[0].as_string()?;
    let prefix = args[1].as_string()?;
    
    Ok(Value::Bool(string.starts_with(&prefix)))
}

/// Check if a string ends with a suffix
/// Example: ends_with("hello", "lo") => true
pub fn ends_with(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("String.ends_with requires exactly 2 arguments: string, suffix".to_string());
    }
    
    let string = args[0].as_string()?;
    let suffix = args[1].as_string()?;
    
    Ok(Value::Bool(string.ends_with(&suffix)))
}

/// Check if a string contains a substring
/// Example: contains("hello", "ell") => true
pub fn contains(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("String.contains requires exactly 2 arguments: string, substring".to_string());
    }
    
    let string = args[0].as_string()?;
    let substring = args[1].as_string()?;
    
    Ok(Value::Bool(string.contains(&substring)))
}

/// Repeat a string multiple times
/// Example: repeat("abc", 3) => "abcabcabc"
pub fn repeat(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("String.repeat requires exactly 2 arguments: string, count".to_string());
    }
    
    let string = args[0].as_string()?;
    
    let count = match &args[1] {
        Value::Int(n) => *n as usize,
        Value::Float(n) => *n as usize,
        _ => return Err(format!("Second argument to repeat must be a number, got {:?}", args[1])),
    };
    
    Ok(Value::String(string.repeat(count)))
}
