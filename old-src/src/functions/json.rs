use crate::value::Value;
use std::collections::HashMap;
use serde_json::{self, json, Value as JsonValue};

/// Parse a JSON string into a Razen value
/// Example: parse('{"name":"John","age":30}') => {name: "John", age: 30}
pub fn parse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("JSON.parse requires exactly 1 argument: json_string".to_string());
    }
    
    let json_string = args[0].as_string()?;
    
    // Parse the JSON string
    let parsed: Result<JsonValue, serde_json::Error> = serde_json::from_str(&json_string);
    
    match parsed {
        Ok(json_value) => json_to_razen_value(json_value),
        Err(e) => Err(format!("Failed to parse JSON: {}", e)),
    }
}

/// Convert a Razen value to a JSON string
/// Example: stringify({name: "John", age: 30}) => '{"name":"John","age":30}'
pub fn stringify(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("JSON.stringify requires exactly 1 argument: value".to_string());
    }
    
    let value = &args[0];
    
    // Convert Razen value to JSON value
    let json_value = razen_value_to_json(value)?;
    
    // Convert JSON value to string
    match serde_json::to_string(&json_value) {
        Ok(json_string) => Ok(Value::String(json_string)),
        Err(e) => Err(format!("Failed to stringify value: {}", e)),
    }
}

/// Helper function to convert a JSON value to a Razen value
fn json_to_razen_value(json_value: JsonValue) -> Result<Value, String> {
    match json_value {
        JsonValue::Null => Ok(Value::Null),
        JsonValue::Bool(b) => Ok(Value::Bool(b)),
        JsonValue::Number(n) => {
            if n.is_i64() {
                Ok(Value::Int(n.as_i64().unwrap()))
            } else {
                Ok(Value::Float(n.as_f64().unwrap()))
            }
        },
        JsonValue::String(s) => Ok(Value::String(s)),
        JsonValue::Array(arr) => {
            let mut razen_array = Vec::new();
            for item in arr {
                razen_array.push(json_to_razen_value(item)?);
            }
            Ok(Value::Array(razen_array))
        },
        JsonValue::Object(obj) => {
            let mut razen_map = HashMap::new();
            for (key, value) in obj {
                razen_map.insert(key, json_to_razen_value(value)?);
            }
            Ok(Value::Map(razen_map))
        },
    }
}

/// Helper function to convert a Razen value to a JSON value
pub fn razen_value_to_json(value: &Value) -> Result<JsonValue, String> {
    match value {
        Value::Null => Ok(JsonValue::Null),
        Value::Bool(b) => Ok(JsonValue::Bool(*b)),
        Value::Int(i) => Ok(json!(*i)),
        Value::Float(f) => Ok(json!(*f)),
        Value::String(s) => Ok(JsonValue::String(s.clone())),
        Value::Array(arr) => {
            let mut json_array = Vec::new();
            for item in arr {
                json_array.push(razen_value_to_json(item)?);
            }
            Ok(JsonValue::Array(json_array))
        },
        Value::Map(map) => {
            let mut json_object = serde_json::Map::new();
            for (key, value) in map {
                json_object.insert(key.clone(), razen_value_to_json(value)?);
            }
            Ok(JsonValue::Object(json_object))
        },
    }
}
