// src/backend/ir/instructions.rs

/// IR Instructions - Part 2 of the backend (placeholder)
#[derive(Debug, Clone)]
pub enum Instruction {
    // Placeholder instructions for Part 2
    Load { dest: String, src: Operand },
    Store { dest: Operand, src: Operand },
    Add { dest: String, left: Operand, right: Operand },
    Sub { dest: String, left: Operand, right: Operand },
    Mul { dest: String, left: Operand, right: Operand },
    Div { dest: String, left: Operand, right: Operand },
    Call { dest: Option<String>, func: String, args: Vec<Operand> },
    Return { value: Option<Operand> },
    Branch { target: String },
    BranchIf { condition: Operand, true_target: String, false_target: String },
    Label { name: String },
}

#[derive(Debug, Clone)]
pub enum Operand {
    Register(String),
    Immediate(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<Instruction>,
    pub terminator: Option<Instruction>,
}

impl BasicBlock {
    pub fn new(label: String) -> Self {
        BasicBlock {
            label,
            instructions: Vec::new(),
            terminator: None,
        }
    }
    
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
    
    pub fn set_terminator(&mut self, terminator: Instruction) {
        self.terminator = Some(terminator);
    }
}
