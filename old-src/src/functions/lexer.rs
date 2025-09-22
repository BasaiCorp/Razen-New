use crate::value::Value;
use std::collections::HashMap;

/// Create a new lexer configuration
/// Example: create_lexer({"tokens": ["INTEGER", "PLUS"], "ignore": ["WHITESPACE"]}) => lexer_config
pub fn create_lexer(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("Lexer.create_lexer: Expected at least 1 argument (config)".to_string());
    }
    
    let config = args[0].clone();
    // Return the configuration as is for now
    // In a real implementation, this would create a lexer instance
    Ok(config)
}

/// Tokenize input text using the lexer configuration
/// Example: tokenize(lexer_config, "2 + 3") => [{"type":"INTEGER","value":"2"}, {"type":"PLUS","value":"+"}, {"type":"INTEGER","value":"3"}]
pub fn tokenize(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Lexer.tokenize: Expected 2 arguments (lexer, input)".to_string());
    }
    
    let _lexer = args[0].clone();
    let input = args[1].as_string()?;
    
    // For now, just return a simple array of tokens
    // In a real implementation, this would use the lexer to tokenize the input
    let mut tokens = Vec::new();
    
    // Simple tokenization for demonstration
    let mut i = 0;
    while i < input.len() {
        let c = input.chars().nth(i).unwrap();
        
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        
        if c.is_digit(10) {
            let mut num = String::new();
            while i < input.len() && input.chars().nth(i).unwrap().is_digit(10) {
                num.push(input.chars().nth(i).unwrap());
                i += 1;
            }
            let mut token = HashMap::new();
            token.insert("type".to_string(), Value::String("INTEGER".to_string()));
            token.insert("value".to_string(), Value::String(num));
            tokens.push(Value::Map(token));
            continue;
        }
        
        if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
            let mut token = HashMap::new();
            let token_type = match c {
                '+' => "PLUS",
                '-' => "MINUS",
                '*' => "MULTIPLY",
                '/' => "DIVIDE",
                '(' => "LPAREN",
                ')' => "RPAREN",
                _ => "UNKNOWN",
            };
            token.insert("type".to_string(), Value::String(token_type.to_string()));
            token.insert("value".to_string(), Value::String(c.to_string()));
            tokens.push(Value::Map(token));
            i += 1;
            continue;
        }
        
        // Skip unknown characters
        i += 1;
    }
    
    Ok(Value::Array(tokens))
}

/// Define a new token with a name and pattern
/// Example: define_token("INTEGER", "[0-9]+") => {"name":"INTEGER","pattern":"[0-9]+"}
pub fn define_token(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Lexer.define_token: Expected 2 arguments (name, pattern)".to_string());
    }
    
    let name = args[0].as_string()?;
    let pattern = args[1].as_string()?;
    
    let mut token = HashMap::new();
    token.insert("name".to_string(), Value::String(name));
    token.insert("pattern".to_string(), Value::String(pattern));
    
    Ok(Value::Map(token))
}
