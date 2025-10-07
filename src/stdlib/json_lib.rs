// src/stdlib/json_lib.rs
//! JSON parsing and serialization standard library - Native Rust implementation

use crate::backend::execution::value::Value;
use std::collections::HashMap;

pub fn has_function(name: &str) -> bool {
    matches!(name, "parse" | "stringify" | "is_valid")
}

pub fn get_function_list() -> Vec<&'static str> {
    vec!["parse", "stringify", "is_valid"]
}

pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "parse" => parse(args),
        "stringify" => stringify(args),
        "is_valid" => is_valid(args),
        _ => Err(format!("Unknown json function: {}", name)),
    }
}

fn parse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("parse() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::String(json_str) => {
            parse_json_value(json_str.trim())
        }
        _ => Err("parse() requires a string argument".to_string()),
    }
}

fn stringify(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("stringify() takes exactly 1 argument".to_string());
    }
    Ok(Value::String(value_to_json(&args[0])))
}

fn is_valid(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is_valid() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::String(json_str) => {
            Ok(Value::Boolean(parse_json_value(json_str.trim()).is_ok()))
        }
        _ => Err("is_valid() requires a string argument".to_string()),
    }
}

// Simple JSON parser (basic implementation)
fn parse_json_value(s: &str) -> Result<Value, String> {
    let s = s.trim();
    
    if s.is_empty() {
        return Err("Empty JSON string".to_string());
    }
    
    // Parse null
    if s == "null" {
        return Ok(Value::Null);
    }
    
    // Parse boolean
    if s == "true" {
        return Ok(Value::Boolean(true));
    }
    if s == "false" {
        return Ok(Value::Boolean(false));
    }
    
    // Parse number
    if let Ok(n) = s.parse::<i64>() {
        return Ok(Value::Integer(n));
    }
    if let Ok(n) = s.parse::<f64>() {
        return Ok(Value::Number(n));
    }
    
    // Parse string
    if s.starts_with('"') && s.ends_with('"') {
        let content = &s[1..s.len()-1];
        return Ok(Value::String(content.to_string()));
    }
    
    // Parse array
    if s.starts_with('[') && s.ends_with(']') {
        let content = &s[1..s.len()-1].trim();
        if content.is_empty() {
            return Ok(Value::Array(vec![]));
        }
        
        let mut elements = Vec::new();
        let mut current = String::new();
        let mut depth = 0;
        let mut in_string = false;
        
        for ch in content.chars() {
            match ch {
                '"' => in_string = !in_string,
                '[' | '{' if !in_string => depth += 1,
                ']' | '}' if !in_string => depth -= 1,
                ',' if depth == 0 && !in_string => {
                    elements.push(parse_json_value(current.trim())?);
                    current.clear();
                    continue;
                }
                _ => {}
            }
            current.push(ch);
        }
        
        if !current.trim().is_empty() {
            elements.push(parse_json_value(current.trim())?);
        }
        
        return Ok(Value::Array(elements));
    }
    
    // Parse object (as map)
    if s.starts_with('{') && s.ends_with('}') {
        let content = &s[1..s.len()-1].trim();
        if content.is_empty() {
            return Ok(Value::Map(HashMap::new()));
        }
        
        let mut map = HashMap::new();
        let mut current = String::new();
        let mut depth = 0;
        let mut in_string = false;
        let mut pairs = Vec::new();
        
        for ch in content.chars() {
            match ch {
                '"' => in_string = !in_string,
                '[' | '{' if !in_string => depth += 1,
                ']' | '}' if !in_string => depth -= 1,
                ',' if depth == 0 && !in_string => {
                    pairs.push(current.trim().to_string());
                    current.clear();
                    continue;
                }
                _ => {}
            }
            current.push(ch);
        }
        
        if !current.trim().is_empty() {
            pairs.push(current.trim().to_string());
        }
        
        for pair in pairs {
            if let Some(colon_pos) = pair.find(':') {
                let key_str = pair[..colon_pos].trim();
                let value_str = pair[colon_pos+1..].trim();
                
                let key = if key_str.starts_with('"') && key_str.ends_with('"') {
                    key_str[1..key_str.len()-1].to_string()
                } else {
                    key_str.to_string()
                };
                
                let value = parse_json_value(value_str)?;
                map.insert(key, value);
            }
        }
        
        return Ok(Value::Map(map));
    }
    
    Err(format!("Invalid JSON: {}", s))
}

// Convert Value to JSON string
fn value_to_json(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Integer(n) => n.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(value_to_json).collect();
            format!("[{}]", elements.join(","))
        }
        Value::Map(map) => {
            let pairs: Vec<String> = map.iter()
                .map(|(k, v)| format!("\"{}\":{}", k, value_to_json(v)))
                .collect();
            format!("{{{}}}", pairs.join(","))
        }
        Value::Struct { type_name, fields } => {
            let pairs: Vec<String> = fields.iter()
                .map(|(k, v)| format!("\"{}\":{}", k, value_to_json(v)))
                .collect();
            format!("{{\"_type\":\"{}\",{}}}", type_name, pairs.join(","))
        }
        Value::Result { is_ok, value } => {
            if *is_ok {
                format!("{{\"Ok\":{}}}", value_to_json(value))
            } else {
                format!("{{\"Err\":{}}}", value_to_json(value))
            }
        }
        Value::Option { is_some, value } => {
            if *is_some {
                format!("{{\"Some\":{}}}", value_to_json(value))
            } else {
                "null".to_string()
            }
        }
    }
}
