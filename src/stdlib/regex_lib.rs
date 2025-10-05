// src/stdlib/regex_lib.rs
//! Regular expression operations standard library

use crate::backend::execution::value::Value;
use regex::Regex;

pub fn has_function(name: &str) -> bool {
    matches!(name, 
        "match" | "matches" | "find" | "find_all" | 
        "replace" | "replace_all" | "split" | "is_match"
    )
}

pub fn get_function_list() -> Vec<&'static str> {
    vec![
        "match", "matches", "find", "find_all",
        "replace", "replace_all", "split", "is_match"
    ]
}

pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "match" => regex_match(args),
        "matches" => matches(args),
        "find" => find(args),
        "find_all" => find_all(args),
        "replace" => replace(args),
        "replace_all" => replace_all(args),
        "split" => split(args),
        "is_match" => is_match(args),
        _ => Err(format!("Unknown regex function: {}", name)),
    }
}

fn regex_match(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("match() requires 2 arguments (pattern, text)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(pattern), Value::String(text)) => {
            match Regex::new(pattern) {
                Ok(re) => {
                    if let Some(captures) = re.captures(text) {
                        let matches: Vec<Value> = captures
                            .iter()
                            .filter_map(|m| m.map(|m| Value::String(m.as_str().to_string())))
                            .collect();
                        Ok(Value::Array(matches))
                    } else {
                        Ok(Value::Array(vec![]))
                    }
                }
                Err(e) => Err(format!("Invalid regex pattern: {}", e)),
            }
        }
        _ => Err("match() requires two string arguments".to_string()),
    }
}

fn matches(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("matches() requires 2 arguments (pattern, text)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(pattern), Value::String(text)) => {
            match Regex::new(pattern) {
                Ok(re) => {
                    let all_matches: Vec<Value> = re
                        .find_iter(text)
                        .map(|m| Value::String(m.as_str().to_string()))
                        .collect();
                    Ok(Value::Array(all_matches))
                }
                Err(e) => Err(format!("Invalid regex pattern: {}", e)),
            }
        }
        _ => Err("matches() requires two string arguments".to_string()),
    }
}

fn find(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("find() requires 2 arguments (pattern, text)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(pattern), Value::String(text)) => {
            match Regex::new(pattern) {
                Ok(re) => {
                    if let Some(m) = re.find(text) {
                        Ok(Value::String(m.as_str().to_string()))
                    } else {
                        Ok(Value::Null)
                    }
                }
                Err(e) => Err(format!("Invalid regex pattern: {}", e)),
            }
        }
        _ => Err("find() requires two string arguments".to_string()),
    }
}

fn find_all(args: Vec<Value>) -> Result<Value, String> {
    matches(args) // Same as matches
}

fn replace(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 3 {
        return Err("replace() requires 3 arguments (pattern, text, replacement)".to_string());
    }
    
    match (&args[0], &args[1], &args[2]) {
        (Value::String(pattern), Value::String(text), Value::String(replacement)) => {
            match Regex::new(pattern) {
                Ok(re) => {
                    let result = re.replace(text, replacement.as_str());
                    Ok(Value::String(result.to_string()))
                }
                Err(e) => Err(format!("Invalid regex pattern: {}", e)),
            }
        }
        _ => Err("replace() requires three string arguments".to_string()),
    }
}

fn replace_all(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 3 {
        return Err("replace_all() requires 3 arguments (pattern, text, replacement)".to_string());
    }
    
    match (&args[0], &args[1], &args[2]) {
        (Value::String(pattern), Value::String(text), Value::String(replacement)) => {
            match Regex::new(pattern) {
                Ok(re) => {
                    let result = re.replace_all(text, replacement.as_str());
                    Ok(Value::String(result.to_string()))
                }
                Err(e) => Err(format!("Invalid regex pattern: {}", e)),
            }
        }
        _ => Err("replace_all() requires three string arguments".to_string()),
    }
}

fn split(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("split() requires 2 arguments (pattern, text)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(pattern), Value::String(text)) => {
            match Regex::new(pattern) {
                Ok(re) => {
                    let parts: Vec<Value> = re
                        .split(text)
                        .map(|s| Value::String(s.to_string()))
                        .collect();
                    Ok(Value::Array(parts))
                }
                Err(e) => Err(format!("Invalid regex pattern: {}", e)),
            }
        }
        _ => Err("split() requires two string arguments".to_string()),
    }
}

fn is_match(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("is_match() requires 2 arguments (pattern, text)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(pattern), Value::String(text)) => {
            match Regex::new(pattern) {
                Ok(re) => Ok(Value::Boolean(re.is_match(text))),
                Err(e) => Err(format!("Invalid regex pattern: {}", e)),
            }
        }
        _ => Err("is_match() requires two string arguments".to_string()),
    }
}
