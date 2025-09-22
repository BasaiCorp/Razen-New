use crate::value::Value;
use std::collections::HashMap;

/// Create a new parser configuration
/// Example: create_parser({"grammar": grammar, "rules": [rule1, rule2], "startSymbol": rule1}) => parser_config
pub fn create_parser(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("Parser.create_parser: Expected at least 1 argument (config)".to_string());
    }
    
    let config = args[0].clone();
    // Return the configuration as is for now
    // In a real implementation, this would create a parser instance
    Ok(config)
}

/// Parse tokens into an AST using the parser configuration
/// Example: parse(parser_config, tokens) => ast_node
pub fn parse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Parser.parse: Expected 2 arguments (parser, tokens)".to_string());
    }
    
    let _parser = args[0].clone();
    let tokens = args[1].clone();
    
    // For now, just return a simple AST
    // In a real implementation, this would use the parser to build an AST
    let mut ast = HashMap::new();
    ast.insert("type".to_string(), Value::String("Program".to_string()));
    ast.insert("body".to_string(), tokens);
    
    Ok(Value::Map(ast))
}

/// Define a grammar rule
/// Example: define_rule("expression", "term { ('+'|'-') term }", "ExpressionNode") => rule
pub fn define_rule(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Parser.define_rule: Expected at least 2 arguments (name, production, [node_type])".to_string());
    }
    
    let name = args[0].as_string()?;
    let production = args[1].as_string()?;
    let node_type = if args.len() > 2 { Some(args[2].as_string()?) } else { None };
    
    let mut rule = HashMap::new();
    rule.insert("name".to_string(), Value::String(name));
    rule.insert("production".to_string(), Value::String(production));
    
    if let Some(node) = node_type {
        rule.insert("astNode".to_string(), Value::String(node));
    }
    
    Ok(Value::Map(rule))
}

/// Create a grammar definition
/// Example: create_grammar("Calculator", {"version": "1.0", "description": "Simple calculator grammar"}) => grammar
pub fn create_grammar(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Parser.create_grammar: Expected at least 2 arguments (name, properties)".to_string());
    }
    
    let name = args[0].as_string()?;
    let properties = match &args[1] {
        Value::Map(props) => props.clone(),
        _ => return Err("Parser.create_grammar: Second argument must be a map of properties".to_string()),
    };
    
    let mut grammar = properties;
    grammar.insert("name".to_string(), Value::String(name));
    
    Ok(Value::Map(grammar))
}
