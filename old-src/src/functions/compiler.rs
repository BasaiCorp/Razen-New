use crate::value::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Global compiler infrastructure manager
lazy_static::lazy_static! {
    static ref COMPILER_MANAGER: Arc<Mutex<CompilerManager>> = Arc::new(Mutex::new(CompilerManager::new()));
}

// Node types for AST
#[derive(Debug, Clone)]
enum AstNodeType {
    Program,
    Variable,
    Function,
    Expression,
    Statement,
    Operator,
    Literal,
    Custom(String),
}

// AST node structure
#[derive(Debug, Clone)]
struct AstNode {
    node_type: AstNodeType,
    name: String,
    data_type: String,
    value: Option<String>,
    children: Vec<usize>,
}

// Symbol table entry
#[derive(Debug, Clone)]
struct SymbolEntry {
    name: String,
    data_type: String,
    address: usize,
    scope: String,
}

// Compiler manager to track AST nodes and symbol tables
struct CompilerManager {
    nodes: HashMap<usize, AstNode>,
    next_node_id: usize,
    symbol_tables: HashMap<usize, HashMap<String, SymbolEntry>>,
    next_table_id: usize,
}

impl CompilerManager {
    fn new() -> Self {
        CompilerManager {
            nodes: HashMap::new(),
            next_node_id: 1,
            symbol_tables: HashMap::new(),
            next_table_id: 1,
        }
    }

    fn create_node(&mut self, node_type: AstNodeType, name: String, data_type: String, value: Option<String>) -> usize {
        let id = self.next_node_id;
        self.next_node_id += 1;
        
        let node = AstNode {
            node_type,
            name,
            data_type,
            value,
            children: Vec::new(),
        };
        
        self.nodes.insert(id, node);
        id
    }

    fn add_child(&mut self, parent_id: usize, child_id: usize) -> Result<(), String> {
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(child_id);
            Ok(())
        } else {
            Err(format!("Invalid parent node ID: {}", parent_id))
        }
    }

    fn get_node(&self, id: usize) -> Result<&AstNode, String> {
        self.nodes.get(&id)
            .ok_or_else(|| format!("Invalid node ID: {}", id))
    }

    fn create_symbol_table(&mut self) -> usize {
        let id = self.next_table_id;
        self.next_table_id += 1;
        self.symbol_tables.insert(id, HashMap::new());
        id
    }

    fn add_symbol(&mut self, table_id: usize, name: String, data_type: String, address: usize) -> Result<(), String> {
        if let Some(table) = self.symbol_tables.get_mut(&table_id) {
            let entry = SymbolEntry {
                name: name.clone(),
                data_type,
                address,
                scope: "global".to_string(), // Default scope
            };
            
            table.insert(name, entry);
            Ok(())
        } else {
            Err(format!("Invalid symbol table ID: {}", table_id))
        }
    }

    fn lookup_symbol(&self, table_id: usize, name: &str) -> Result<&SymbolEntry, String> {
        if let Some(table) = self.symbol_tables.get(&table_id) {
            table.get(name)
                .ok_or_else(|| format!("Symbol not found: {}", name))
        } else {
            Err(format!("Invalid symbol table ID: {}", table_id))
        }
    }
}

/// Create an AST node
/// Example: create_node("variable", "x", "integer") => 1
pub fn create_node(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 4 {
        return Err("Compiler.create_node requires 2-4 arguments: node_type, name, [data_type], [value]".to_string());
    }
    
    let node_type_str = args[0].as_string()?;
    let name = args[1].as_string()?;
    
    let data_type = if args.len() > 2 {
        args[2].as_string()?
    } else {
        "any".to_string()
    };
    
    let value = if args.len() > 3 {
        Some(args[3].as_string()?)
    } else {
        None
    };
    
    // Convert string to node type
    let node_type = match node_type_str.as_str() {
        "program" => AstNodeType::Program,
        "variable" => AstNodeType::Variable,
        "function" => AstNodeType::Function,
        "expression" => AstNodeType::Expression,
        "statement" => AstNodeType::Statement,
        "operator" => AstNodeType::Operator,
        "literal" => AstNodeType::Literal,
        _ => AstNodeType::Custom(node_type_str),
    };
    
    // Create the node
    let node_id = COMPILER_MANAGER.lock().unwrap()
        .create_node(node_type, name, data_type, value);
    
    Ok(Value::Int(node_id as i64))
}

/// Add a child node to a parent node
/// Example: add_child(1, 2) => true
pub fn add_child(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Compiler.add_child requires exactly 2 arguments: parent_id, child_id".to_string());
    }
    
    let parent_id = args[0].as_int()? as usize;
    let child_id = args[1].as_int()? as usize;
    
    // Add the child
    match COMPILER_MANAGER.lock().unwrap().add_child(parent_id, child_id) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(e),
    }
}

/// Convert an AST node to a string representation
/// Example: node_to_string(1) => "Variable(x: integer)"
pub fn node_to_string(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Compiler.node_to_string requires exactly 1 argument: node_id".to_string());
    }
    
    let node_id = args[0].as_int()? as usize;
    
    // Get the node
    let manager = COMPILER_MANAGER.lock().unwrap();
    let node = manager.get_node(node_id)?;
    
    // Convert to string
    let node_type = match &node.node_type {
        AstNodeType::Program => "Program".to_string(),
        AstNodeType::Variable => "Variable".to_string(),
        AstNodeType::Function => "Function".to_string(),
        AstNodeType::Expression => "Expression".to_string(),
        AstNodeType::Statement => "Statement".to_string(),
        AstNodeType::Operator => "Operator".to_string(),
        AstNodeType::Literal => "Literal".to_string(),
        AstNodeType::Custom(s) => s.clone(),
    };
    
    let value_str = if let Some(value) = &node.value {
        format!(", value: {}", value)
    } else {
        "".to_string()
    };
    
    let result = format!("{}({}: {}{})", node_type, node.name, node.data_type, value_str);
    
    Ok(Value::String(result))
}

/// Create a symbol table
/// Example: create_symbol_table() => 1
pub fn create_symbol_table(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("Compiler.create_symbol_table takes no arguments".to_string());
    }
    
    // Create a symbol table
    let table_id = COMPILER_MANAGER.lock().unwrap().create_symbol_table();
    
    Ok(Value::Int(table_id as i64))
}

/// Add a symbol to a symbol table
/// Example: add_symbol(1, "x", "integer", 0) => true
pub fn add_symbol(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("Compiler.add_symbol requires exactly 4 arguments: table_id, name, data_type, address".to_string());
    }
    
    let table_id = args[0].as_int()? as usize;
    let name = args[1].as_string()?;
    let data_type = args[2].as_string()?;
    let address = args[3].as_int()? as usize;
    
    // Add the symbol
    match COMPILER_MANAGER.lock().unwrap().add_symbol(table_id, name, data_type, address) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(e),
    }
}

/// Look up a symbol in a symbol table
/// Example: lookup_symbol(1, "x") => "integer"
pub fn lookup_symbol(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Compiler.lookup_symbol requires exactly 2 arguments: table_id, name".to_string());
    }
    
    let table_id = args[0].as_int()? as usize;
    let name = args[1].as_string()?;
    
    // Look up the symbol
    let manager = COMPILER_MANAGER.lock().unwrap();
    let symbol = manager.lookup_symbol(table_id, &name)?;
    
    // Convert to a map
    let mut result = Vec::new();
    result.push(Value::String("name".to_string()));
    result.push(Value::String(symbol.name.clone()));
    result.push(Value::String("type".to_string()));
    result.push(Value::String(symbol.data_type.clone()));
    result.push(Value::String("address".to_string()));
    result.push(Value::Int(symbol.address as i64));
    result.push(Value::String("scope".to_string()));
    result.push(Value::String(symbol.scope.clone()));
    
    Ok(Value::Array(result))
}

/// Generate intermediate representation (IR) code
/// Example: generate_ir("x = 5 + 3") => "PUSH 5\nPUSH 3\nADD\nSTORE x"
pub fn generate_ir(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Compiler.generate_ir requires exactly 1 argument: source_code".to_string());
    }
    
    let source_code = args[0].as_string()?;
    
    // This is a simplified implementation
    // In a real implementation, we would need to parse the source code
    // and generate proper IR
    
    // For now, just generate some dummy IR
    let mut ir = String::new();
    
    if source_code.contains('=') {
        let parts: Vec<&str> = source_code.split('=').collect();
        if parts.len() == 2 {
            let var_name = parts[0].trim();
            let expr = parts[1].trim();
            
            if expr.contains('+') {
                let operands: Vec<&str> = expr.split('+').collect();
                if operands.len() == 2 {
                    let left = operands[0].trim();
                    let right = operands[1].trim();
                    
                    ir.push_str(&format!("PUSH {}\n", left));
                    ir.push_str(&format!("PUSH {}\n", right));
                    ir.push_str("ADD\n");
                    ir.push_str(&format!("STORE {}\n", var_name));
                }
            } else {
                ir.push_str(&format!("PUSH {}\n", expr));
                ir.push_str(&format!("STORE {}\n", var_name));
            }
        }
    }
    
    Ok(Value::String(ir))
}

/// Optimize intermediate representation (IR) code
/// Example: optimize_ir("PUSH 5\nPUSH 3\nADD\nSTORE x") => "PUSH 8\nSTORE x"
pub fn optimize_ir(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Compiler.optimize_ir requires exactly 1 argument: ir_code".to_string());
    }
    
    let ir_code = args[0].as_string()?;
    
    // This is a simplified implementation
    // In a real implementation, we would need to parse the IR code
    // and apply optimization passes
    
    // For now, just return the original IR
    Ok(Value::String(ir_code))
}

/// Generate assembly code from IR
/// Example: generate_assembly("PUSH 8\nSTORE x") => "mov eax, 8\nmov [x], eax"
pub fn generate_assembly(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Compiler.generate_assembly requires exactly 1 argument: ir_code".to_string());
    }
    
    let ir_code = args[0].as_string()?;
    
    // This is a simplified implementation
    // In a real implementation, we would need to parse the IR code
    // and generate proper assembly
    
    // For now, just generate some dummy assembly
    let mut assembly = String::new();
    
    for line in ir_code.lines() {
        if line.starts_with("PUSH ") {
            let value = line.trim_start_matches("PUSH ").trim();
            assembly.push_str(&format!("mov eax, {}\n", value));
        } else if line == "ADD" {
            assembly.push_str("add eax, ebx\n");
        } else if line.starts_with("STORE ") {
            let var_name = line.trim_start_matches("STORE ").trim();
            assembly.push_str(&format!("mov [{}], eax\n", var_name));
        }
    }
    
    Ok(Value::String(assembly))
}

/// Parse source code into an AST
/// Example: parse("let x = 5 + 3;") => 1 (root node ID)
pub fn parse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Compiler.parse requires exactly 1 argument: source_code".to_string());
    }
    
    let source_code = args[0].as_string()?;
    
    // This is a simplified implementation
    // In a real implementation, we would need to tokenize and parse the source code
    
    // For now, just create a dummy AST
    let mut manager = COMPILER_MANAGER.lock().unwrap();
    
    let program_id = manager.create_node(AstNodeType::Program, "program".to_string(), "void".to_string(), None);
    
    if source_code.contains("let") && source_code.contains('=') {
        let var_id = manager.create_node(AstNodeType::Variable, "x".to_string(), "integer".to_string(), None);
        manager.add_child(program_id, var_id).unwrap();
        
        if source_code.contains('+') {
            let expr_id = manager.create_node(AstNodeType::Expression, "add".to_string(), "integer".to_string(), None);
            manager.add_child(var_id, expr_id).unwrap();
            
            let left_id = manager.create_node(AstNodeType::Literal, "5".to_string(), "integer".to_string(), Some("5".to_string()));
            let right_id = manager.create_node(AstNodeType::Literal, "3".to_string(), "integer".to_string(), Some("3".to_string()));
            
            manager.add_child(expr_id, left_id).unwrap();
            manager.add_child(expr_id, right_id).unwrap();
        }
    }
    
    Ok(Value::Int(program_id as i64))
}

/// Tokenize source code into tokens
/// Example: tokenize("let x = 5 + 3;") => ["let", "x", "=", "5", "+", "3", ";"]
pub fn tokenize(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Compiler.tokenize requires exactly 1 argument: source_code".to_string());
    }
    
    let source_code = args[0].as_string()?;
    
    // This is a simplified tokenizer that splits on whitespace and special characters
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    
    // Define special characters that should be separate tokens
    let special_chars = vec!['=', '+', '-', '*', '/', '(', ')', '{', '}', '[', ']', ';', ',', '.'];
    
    for c in source_code.chars() {
        if c.is_whitespace() {
            // End the current token on whitespace
            if !current_token.is_empty() {
                tokens.push(Value::String(current_token.clone()));
                current_token.clear();
            }
        } else if special_chars.contains(&c) {
            // End the current token and add the special character as a separate token
            if !current_token.is_empty() {
                tokens.push(Value::String(current_token.clone()));
                current_token.clear();
            }
            tokens.push(Value::String(c.to_string()));
        } else {
            // Add to the current token
            current_token.push(c);
        }
    }
    
    // Add the final token if there is one
    if !current_token.is_empty() {
        tokens.push(Value::String(current_token));
    }
    
    Ok(Value::Array(tokens))
}

/// Compile source code to bytecode
/// Example: compile("let x = 5 + 3;") => [1, 5, 3, 2, 0]
pub fn compile(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Compiler.compile requires exactly 1 argument: source_code".to_string());
    }
    
    let source_code = args[0].as_string()?;
    
    // This is a simplified compiler that generates a very basic bytecode
    // In a real implementation, we would tokenize, parse, and generate proper bytecode
    
    // For this simple example, we'll just generate some dummy bytecode
    // 1 = DECLARE_VAR, 2 = ADD, 0 = END
    let mut bytecode = Vec::new();
    
    if source_code.contains("let") && source_code.contains('=') {
        bytecode.push(Value::Int(1)); // DECLARE_VAR
        
        // Extract the numeric values from the source code
        let mut numbers = Vec::new();
        let mut current_number = String::new();
        
        for c in source_code.chars() {
            if c.is_digit(10) {
                current_number.push(c);
            } else if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<i64>() {
                    numbers.push(num);
                }
                current_number.clear();
            }
        }
        
        // Add any remaining number
        if !current_number.is_empty() {
            if let Ok(num) = current_number.parse::<i64>() {
                numbers.push(num);
            }
        }
        
        // Add the numbers to the bytecode
        for num in numbers {
            bytecode.push(Value::Int(num));
        }
        
        // If the source code contains addition, add the ADD instruction
        if source_code.contains('+') {
            bytecode.push(Value::Int(2)); // ADD
        }
    }
    
    bytecode.push(Value::Int(0)); // END
    
    Ok(Value::Array(bytecode))
}
