use crate::value::Value;
use std::collections::HashMap;

/// Create an IR instruction
/// Example: create_instruction("LOAD_CONST", ["value"]) => ir_instruction
pub fn create_instruction(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("IR.create_instruction: Expected at least 2 arguments (opcode, operands)".to_string());
    }
    
    let opcode = args[0].as_string()?;
    let operands = match &args[1] {
        Value::Array(ops) => ops.clone(),
        _ => return Err("IR.create_instruction: Second argument must be an array of operands".to_string()),
    };
    
    let mut instruction = HashMap::new();
    instruction.insert("opcode".to_string(), Value::String(opcode));
    instruction.insert("operands".to_string(), Value::Array(operands));
    
    Ok(Value::Map(instruction))
}

/// Generate IR code from an AST
/// Example: generate(ast) => ir_code
pub fn generate(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("IR.generate: Expected at least 1 argument (ast)".to_string());
    }
    
    let ast = args[0].clone();
    
    // Simple IR generation for demonstration
    // In a real implementation, this would traverse the AST and generate IR instructions
    let mut ir_code = Vec::new();
    
    match ast {
        Value::Map(node) => {
            if let Some(Value::String(node_type)) = node.get("type") {
                match node_type.as_str() {
                    "Program" => {
                        if let Some(Value::Array(body)) = node.get("body") {
                            for stmt in body {
                                // Generate IR for each statement
                                let mut load_const = HashMap::new();
                                load_const.insert("opcode".to_string(), Value::String("LOAD_CONST".to_string()));
                                load_const.insert("operands".to_string(), Value::Array(vec![stmt.clone()]));
                                ir_code.push(Value::Map(load_const));
                            }
                        }
                    },
                    _ => {
                        // Default handling for unknown node types
                        let mut nop = HashMap::new();
                        nop.insert("opcode".to_string(), Value::String("NOP".to_string()));
                        nop.insert("operands".to_string(), Value::Array(vec![]));
                        ir_code.push(Value::Map(nop));
                    }
                }
            }
        },
        _ => {
            // If not a node, just load it as a constant
            let mut load_const = HashMap::new();
            load_const.insert("opcode".to_string(), Value::String("LOAD_CONST".to_string()));
            load_const.insert("operands".to_string(), Value::Array(vec![ast]));
            ir_code.push(Value::Map(load_const));
        }
    }
    
    Ok(Value::Array(ir_code))
}

/// Optimize IR code
/// Example: optimize(ir_code, ["constant_folding"]) => optimized_ir_code
pub fn optimize(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("IR.optimize: Expected at least 2 arguments (ir_code, optimizations)".to_string());
    }
    
    let ir_code = args[0].clone();
    let _optimizations = args[1].clone();
    
    // For now, just return the IR code as is
    // In a real implementation, this would apply optimizations to the IR code
    Ok(ir_code)
}

/// Convert IR code to a string representation
/// Example: to_string(ir_code) => "LOAD_CONST 5\nADD\n..."
pub fn to_string(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("IR.to_string: Expected at least 1 argument (ir_code)".to_string());
    }
    
    let ir_code = match &args[0] {
        Value::Array(code) => code,
        _ => return Err("IR.to_string: First argument must be an array of IR instructions".to_string()),
    };
    
    let mut result = String::new();
    
    for (i, instruction) in ir_code.iter().enumerate() {
        if i > 0 {
            result.push_str("\n");
        }
        
        match instruction {
            Value::Map(instr) => {
                if let Some(Value::String(opcode)) = instr.get("opcode") {
                    result.push_str(opcode);
                    
                    if let Some(Value::Array(operands)) = instr.get("operands") {
                        for operand in operands {
                            result.push_str(" ");
                            result.push_str(&operand.to_string());
                        }
                    }
                }
            },
            _ => {
                result.push_str(&instruction.to_string());
            }
        }
    }
    
    Ok(Value::String(result))
}
