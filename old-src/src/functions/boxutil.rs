use crate::value::Value;
use std::collections::HashMap;

/// Stores a value in a box and returns a boxed representation.
/// Example: Box.put(123) => boxed value
pub fn put(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Box.put requires exactly 1 argument: value".to_string());
    }
    
    // Create a boxed representation of the value
    let mut boxed = HashMap::new();
    boxed.insert("type".to_string(), Value::String("box".to_string()));
    boxed.insert("value".to_string(), args[0].clone());
    
    Ok(Value::Map(boxed))
}

/// Returns the value stored in the box.
/// Example: Box.get(box) => previously boxed value
pub fn get(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Box.get requires exactly 1 argument: box".to_string());
    }
    
    // Extract the value from the box
    match &args[0] {
        Value::Map(map) => {
            // Check if it's a valid box
            match map.get("type") {
                Some(Value::String(t)) if t == "box" => {
                    // Return the boxed value
                    match map.get("value") {
                        Some(value) => Ok(value.clone()),
                        None => Err("Invalid box: missing value".to_string())
                    }
                },
                _ => Err("Invalid box: not a box type".to_string())
            }
        },
        _ => Err("Invalid box: expected a box object".to_string())
    }
}

/// Check if a value is a box
/// Example: Box.is_box(value) => true/false
pub fn is_box(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Box.is_box requires exactly 1 argument: value".to_string());
    }
    
    // Check if the value is a box
    match &args[0] {
        Value::Map(map) => {
            match map.get("type") {
                Some(Value::String(t)) if t == "box" => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false))
            }
        },
        _ => Ok(Value::Bool(false))
    }
}
