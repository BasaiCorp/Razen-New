use crate::value::Value;
use regex::Regex;

/// Checks if a pattern matches a string
/// Example: match("abc123", "\\d+") => true
pub fn match_pattern(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Regex.match requires exactly 2 arguments: string, pattern".to_string());
    }
    
    let text = args[0].as_string()?;
    let pattern = args[1].as_string()?;
    
    let regex = Regex::new(&pattern)
        .map_err(|e| format!("Invalid regex pattern: {}", e))?;
    
    Ok(Value::Bool(regex.is_match(&text)))
}

/// Searches for a pattern in a string and returns the first match
/// Example: search("abc123", "\\d+") => "123"
pub fn search(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Regex.search requires exactly 2 arguments: string, pattern".to_string());
    }
    
    let text = args[0].as_string()?;
    let pattern = args[1].as_string()?;
    
    let regex = Regex::new(&pattern)
        .map_err(|e| format!("Invalid regex pattern: {}", e))?;
    
    match regex.find(&text) {
        Some(m) => Ok(Value::String(m.as_str().to_string())),
        None => Ok(Value::Null),
    }
}

/// Replaces all occurrences of a pattern in a string
/// Example: replace("foo123bar", "\\d+", "X") => "fooXbar"
pub fn replace(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("Regex.replace requires exactly 3 arguments: string, pattern, replacement".to_string());
    }
    
    let text = args[0].as_string()?;
    let pattern = args[1].as_string()?;
    let replacement = args[2].as_string()?;
    
    let regex = Regex::new(&pattern)
        .map_err(|e| format!("Invalid regex pattern: {}", e))?;
    
    Ok(Value::String(regex.replace_all(&text, replacement.as_str()).to_string()))
}
