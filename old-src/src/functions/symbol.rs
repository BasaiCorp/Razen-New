use crate::value::Value;
use std::collections::HashMap;

/// Create a symbol table
/// Example: create_symbol_table("global") => symbol_table
pub fn create_symbol_table(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("Symbol.create_symbol_table: Expected at least 1 argument (name)".to_string());
    }
    
    let name = args[0].as_string()?;
    
    let mut symbol_table = HashMap::new();
    symbol_table.insert("name".to_string(), Value::String(name));
    symbol_table.insert("symbols".to_string(), Value::Map(HashMap::new()));
    
    Ok(Value::Map(symbol_table))
}

/// Define a symbol with attributes
/// Example: define_symbol("Variable", ["name", "type", "value", "scope"]) => symbol_def
pub fn define_symbol(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Symbol.define_symbol: Expected at least 2 arguments (name, attributes)".to_string());
    }
    
    let name = args[0].as_string()?;
    let attributes = match &args[1] {
        Value::Array(attrs) => attrs.clone(),
        _ => return Err("Symbol.define_symbol: Second argument must be an array of attributes".to_string()),
    };
    
    let mut symbol = HashMap::new();
    symbol.insert("name".to_string(), Value::String(name));
    symbol.insert("attributes".to_string(), Value::Array(attributes));
    
    Ok(Value::Map(symbol))
}

/// Add a symbol to a symbol table
/// Example: add_symbol(symbol_table, "x", {"type": "int", "value": 5}) => updated_symbol_table
pub fn add_symbol(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 3 {
        return Err("Symbol.add_symbol: Expected at least 3 arguments (symbol_table, name, attributes)".to_string());
    }
    
    let mut symbol_table = match &args[0] {
        Value::Map(table) => table.clone(),
        _ => return Err("Symbol.add_symbol: First argument must be a symbol table".to_string()),
    };
    
    let name = args[1].as_string()?;
    let attributes = args[2].clone();
    
    // Get the symbols map from the symbol table
    let mut symbols = match symbol_table.get("symbols") {
        Some(Value::Map(syms)) => syms.clone(),
        _ => {
            // If no symbols map exists, create one
            HashMap::new()
        }
    };
    
    // Add the symbol to the symbols map
    symbols.insert(name, attributes);
    
    // Update the symbol table with the new symbols map
    symbol_table.insert("symbols".to_string(), Value::Map(symbols));
    
    Ok(Value::Map(symbol_table))
}

/// Look up a symbol in a symbol table
/// Example: lookup_symbol(symbol_table, "x") => symbol_attributes
pub fn lookup_symbol(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Symbol.lookup_symbol: Expected at least 2 arguments (symbol_table, name)".to_string());
    }
    
    let symbol_table = match &args[0] {
        Value::Map(table) => table,
        _ => return Err("Symbol.lookup_symbol: First argument must be a symbol table".to_string()),
    };
    
    let name = args[1].as_string()?;
    
    // Get the symbols map from the symbol table
    let symbols = match symbol_table.get("symbols") {
        Some(Value::Map(syms)) => syms,
        _ => return Err("Symbol.lookup_symbol: Symbol table has no symbols map".to_string()),
    };
    
    // Look up the symbol in the symbols map
    match symbols.get(&name) {
        Some(symbol) => Ok(symbol.clone()),
        None => Err(format!("Symbol '{}' not found in symbol table", name)),
    }
}
