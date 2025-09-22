use crate::value::Value;
use std::collections::HashMap;

/// Define a type with operations
/// Example: define_type("Number", ["+", "-", "*", "/"]) => type_def
pub fn define_type(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Type.define_type: Expected at least 2 arguments (name, operations)".to_string());
    }
    
    let name = args[0].as_string()?;
    let operations = match &args[1] {
        Value::Array(ops) => ops.clone(),
        _ => return Err("Type.define_type: Second argument must be an array of operations".to_string()),
    };
    
    let mut type_def = HashMap::new();
    type_def.insert("name".to_string(), Value::String(name));
    type_def.insert("operations".to_string(), Value::Array(operations));
    
    Ok(Value::Map(type_def))
}

/// Check if a value is of a specific type
/// Example: check_type(value, "Number") => true/false
pub fn check_type(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Type.check_type: Expected at least 2 arguments (value, type_name)".to_string());
    }
    
    let value = args[0].clone();
    let type_name = args[1].as_string()?;
    
    // Simple type checking for demonstration
    let result = match (value, type_name.as_str()) {
        (Value::Int(_), "Number") => true,
        (Value::Float(_), "Number") => true,
        (Value::String(_), "String") => true,
        (Value::Bool(_), "Boolean") => true,
        (Value::Array(_), "Array") => true,
        (Value::Map(_), "Object") => true,
        (Value::Null, "Null") => true,
        _ => false,
    };
    
    Ok(Value::Bool(result))
}

/// Create a type system with types
/// Example: create_type_system([number_type, string_type]) => type_system
pub fn create_type_system(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("Type.create_type_system: Expected at least 1 argument (types)".to_string());
    }
    
    let types = match &args[0] {
        Value::Array(types) => types.clone(),
        _ => return Err("Type.create_type_system: First argument must be an array of types".to_string()),
    };
    
    let mut type_system = HashMap::new();
    type_system.insert("types".to_string(), Value::Array(types));
    
    Ok(Value::Map(type_system))
}

/// Infer the type of an expression
/// Example: infer_type(expression, type_system) => "Number"
pub fn infer_type(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Type.infer_type: Expected at least 2 arguments (expression, type_system)".to_string());
    }
    
    let expression = args[0].clone();
    let _type_system = args[1].clone();
    
    // Simple type inference for demonstration
    let inferred_type = match expression {
        Value::Int(_) => "Number",
        Value::Float(_) => "Number",
        Value::String(_) => "String",
        Value::Bool(_) => "Boolean",
        Value::Array(_) => "Array",
        Value::Map(_) => "Object",
        Value::Null => "Null",
        _ => "Unknown",
    };
    
    Ok(Value::String(inferred_type.to_string()))
}
