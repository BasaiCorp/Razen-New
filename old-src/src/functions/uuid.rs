use crate::value::Value;
use uuid::Uuid;
use std::collections::HashMap;

/// Generates a new UUID string
/// Example: generate() => "550e8400-e29b-41d4-a716-446655440000"
pub fn generate(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("UUID.generate takes no arguments".to_string());
    }
    
    let uuid = Uuid::new_v4();
    Ok(Value::String(uuid.to_string()))
}

/// Parses a UUID string and returns its components
/// Example: parse("550e8400-e29b-41d4-a716-446655440000") => {version: 4, ...}
pub fn parse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("UUID.parse requires exactly 1 argument: uuid_string".to_string());
    }
    
    let uuid_str = args[0].as_string()?;
    
    let uuid = Uuid::parse_str(&uuid_str)
        .map_err(|e| format!("Invalid UUID: {}", e))?;
    
    let mut components = HashMap::new();
    components.insert("version".to_string(), Value::Int(uuid.get_version_num() as i64));
    
    // Convert variant
    components.insert("variant".to_string(), Value::String(format!("{:?}", uuid.get_variant())));
    
    // Convert hyphenated string
    components.insert("hyphenated".to_string(), Value::String(uuid.hyphenated().to_string()));
    
    // Convert simple string
    components.insert("simple".to_string(), Value::String(uuid.simple().to_string()));
    
    // Convert bytes
    let bytes = uuid.as_bytes();
    let mut bytes_array = Vec::new();
    for byte in bytes {
        bytes_array.push(Value::Int(*byte as i64));
    }
    components.insert("bytes".to_string(), Value::Array(bytes_array));
    
    Ok(Value::Map(components))
}

/// Checks if a string is a valid UUID
/// Example: is_valid("550e8400-e29b-41d4-a716-446655440000") => true
pub fn is_valid(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("UUID.is_valid requires exactly 1 argument: uuid_string".to_string());
    }
    
    let uuid_str = args[0].as_string()?;
    
    match Uuid::parse_str(&uuid_str) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(_) => Ok(Value::Bool(false)),
    }
}
