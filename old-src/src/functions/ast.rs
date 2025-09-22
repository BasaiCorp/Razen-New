use crate::value::Value;
use std::collections::HashMap;

/// Create a new AST node
/// Example: create_node("BinaryExpression", {"left": left_node, "operator": "+", "right": right_node}) => node
pub fn create_node(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("AST.create_node: Expected at least 2 arguments (type, properties)".to_string());
    }
    
    let node_type = args[0].as_string()?;
    let properties = match &args[1] {
        Value::Map(props) => props.clone(),
        _ => return Err("AST.create_node: Second argument must be a map of properties".to_string()),
    };
    
    let mut node = properties;
    node.insert("type".to_string(), Value::String(node_type));
    
    Ok(Value::Map(node))
}

/// Define a node type with properties
/// Example: define_node_type("NumberNode", {"extends": "ExpressionNode", "fields": ["value"]}) => node_type
pub fn define_node_type(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("AST.define_node_type: Expected at least 2 arguments (name, properties)".to_string());
    }
    
    let name = args[0].as_string()?;
    let properties = match &args[1] {
        Value::Map(props) => props.clone(),
        _ => return Err("AST.define_node_type: Second argument must be a map of properties".to_string()),
    };
    
    let mut node_type = properties;
    node_type.insert("name".to_string(), Value::String(name));
    
    Ok(Value::Map(node_type))
}

/// Traverse an AST with a visitor
/// Example: traverse(ast, visitor) => result
pub fn traverse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("AST.traverse: Expected at least 2 arguments (ast, visitor)".to_string());
    }
    
    let ast = args[0].clone();
    let _visitor = args[1].clone();
    
    // For now, just return the AST as is
    // In a real implementation, this would traverse the AST with the visitor
    Ok(ast)
}

/// Create a visitor for AST traversal
/// Example: create_visitor("Evaluator", ["visitBinaryExpression", "visitLiteral"]) => visitor
pub fn create_visitor(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("AST.create_visitor: Expected at least 2 arguments (name, methods)".to_string());
    }
    
    let name = args[0].as_string()?;
    let methods = match &args[1] {
        Value::Array(methods) => methods.clone(),
        _ => return Err("AST.create_visitor: Second argument must be an array of method names".to_string()),
    };
    
    let mut visitor = HashMap::new();
    visitor.insert("name".to_string(), Value::String(name));
    visitor.insert("methods".to_string(), Value::Array(methods));
    
    Ok(Value::Map(visitor))
}
