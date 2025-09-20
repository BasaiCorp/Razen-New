// src/backend/ir/instructions.rs

use std::fmt;

/// Complete IR Instructions for Razen Language
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Memory operations
    Load { dest: String, src: Operand },
    Store { dest: Operand, src: Operand },
    Alloca { dest: String, ty: String, size: Option<Operand> },
    
    // Arithmetic operations
    Add { dest: String, left: Operand, right: Operand },
    Sub { dest: String, left: Operand, right: Operand },
    Mul { dest: String, left: Operand, right: Operand },
    Div { dest: String, left: Operand, right: Operand },
    Mod { dest: String, left: Operand, right: Operand },
    Pow { dest: String, left: Operand, right: Operand },
    
    // Bitwise operations
    And { dest: String, left: Operand, right: Operand },
    Or { dest: String, left: Operand, right: Operand },
    Xor { dest: String, left: Operand, right: Operand },
    Not { dest: String, operand: Operand },
    Shl { dest: String, left: Operand, right: Operand },
    Shr { dest: String, left: Operand, right: Operand },
    
    // Comparison operations
    Eq { dest: String, left: Operand, right: Operand },
    Ne { dest: String, left: Operand, right: Operand },
    Lt { dest: String, left: Operand, right: Operand },
    Le { dest: String, left: Operand, right: Operand },
    Gt { dest: String, left: Operand, right: Operand },
    Ge { dest: String, left: Operand, right: Operand },
    
    // Logical operations
    LogicalAnd { dest: String, left: Operand, right: Operand },
    LogicalOr { dest: String, left: Operand, right: Operand },
    LogicalNot { dest: String, operand: Operand },
    
    // Type conversion operations
    IntToFloat { dest: String, src: Operand },
    FloatToInt { dest: String, src: Operand },
    ToString { dest: String, src: Operand },
    ToBool { dest: String, src: Operand },
    
    // Control flow
    Call { dest: Option<String>, func: String, args: Vec<Operand> },
    Return { value: Option<Operand> },
    Branch { target: String },
    BranchIf { condition: Operand, true_target: String, false_target: String },
    Label { name: String },
    
    // Array operations
    ArrayNew { dest: String, element_type: String, size: Operand },
    ArrayGet { dest: String, array: Operand, index: Operand },
    ArraySet { array: Operand, index: Operand, value: Operand },
    ArrayLen { dest: String, array: Operand },
    
    // Map operations
    MapNew { dest: String, key_type: String, value_type: String },
    MapGet { dest: String, map: Operand, key: Operand },
    MapSet { map: Operand, key: Operand, value: Operand },
    MapHas { dest: String, map: Operand, key: Operand },
    MapRemove { map: Operand, key: Operand },
    
    // String operations
    StringConcat { dest: String, left: Operand, right: Operand },
    StringLen { dest: String, string: Operand },
    StringGet { dest: String, string: Operand, index: Operand },
    
    // Struct operations
    StructNew { dest: String, struct_type: String, fields: Vec<(String, Operand)> },
    StructGet { dest: String, struct_val: Operand, field: String },
    StructSet { struct_val: Operand, field: String, value: Operand },
    
    // Enum operations
    EnumNew { dest: String, enum_type: String, variant: String, value: Option<Operand> },
    EnumMatch { value: Operand, arms: Vec<MatchArm> },
    
    // Exception handling
    Throw { value: Operand },
    TryBegin { handler: String },
    TryEnd,
    
    // Variable assignment
    Assign { dest: String, src: Operand },
    
    // Phi node for SSA form
    Phi { dest: String, values: Vec<(Operand, String)> }, // (value, block_label)
    
    // Debug information
    DebugInfo { message: String },
    
    // No-op instruction
    Nop,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(String),
    Immediate(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Global(String),
    Local(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: String,
    pub target: String,
}

impl MatchArm {
    pub fn new(pattern: String, target: String) -> Self {
        MatchArm { pattern, target }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<Instruction>,
    pub terminator: Option<Instruction>,
    pub predecessors: Vec<String>,
    pub successors: Vec<String>,
}

impl BasicBlock {
    pub fn new(label: String) -> Self {
        BasicBlock {
            label,
            instructions: Vec::new(),
            terminator: None,
            predecessors: Vec::new(),
            successors: Vec::new(),
        }
    }
    
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
    
    pub fn set_terminator(&mut self, terminator: Instruction) {
        // Update successors based on terminator
        match &terminator {
            Instruction::Branch { target } => {
                self.successors.clear();
                self.successors.push(target.clone());
            }
            Instruction::BranchIf { true_target, false_target, .. } => {
                self.successors.clear();
                self.successors.push(true_target.clone());
                self.successors.push(false_target.clone());
            }
            _ => {}
        }
        self.terminator = Some(terminator);
    }
    
    pub fn add_predecessor(&mut self, pred: String) {
        if !self.predecessors.contains(&pred) {
            self.predecessors.push(pred);
        }
    }
    
    pub fn is_terminated(&self) -> bool {
        self.terminator.is_some()
    }
}

// Display implementations for better debugging
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Load { dest, src } => write!(f, "{} = load {}", dest, src),
            Instruction::Store { dest, src } => write!(f, "store {} = {}", dest, src),
            Instruction::Add { dest, left, right } => write!(f, "{} = add {}, {}", dest, left, right),
            Instruction::Call { dest, func, args } => {
                if let Some(dest) = dest {
                    write!(f, "{} = call {}({})", dest, func, 
                           args.iter().map(|a| format!("{}", a)).collect::<Vec<_>>().join(", "))
                } else {
                    write!(f, "call {}({})", func,
                           args.iter().map(|a| format!("{}", a)).collect::<Vec<_>>().join(", "))
                }
            }
            Instruction::Return { value } => {
                if let Some(val) = value {
                    write!(f, "return {}", val)
                } else {
                    write!(f, "return")
                }
            }
            Instruction::Label { name } => write!(f, "{}:", name),
            _ => write!(f, "{:?}", self), // Fallback for other instructions
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Register(name) => write!(f, "%{}", name),
            Operand::Immediate(val) => write!(f, "{}", val),
            Operand::Float(val) => write!(f, "{}", val),
            Operand::String(val) => write!(f, "\"{}\"", val),
            Operand::Bool(val) => write!(f, "{}", val),
            Operand::Null => write!(f, "null"),
            Operand::Global(name) => write!(f, "@{}", name),
            Operand::Local(name) => write!(f, "%{}", name),
        }
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.label)?;
        for instruction in &self.instructions {
            writeln!(f, "  {}", instruction)?;
        }
        if let Some(ref terminator) = self.terminator {
            writeln!(f, "  {}", terminator)?;
        }
        Ok(())
    }
}

/// Instruction builder for easier IR construction
pub struct InstructionBuilder {
    next_register: usize,
    next_label: usize,
}

impl InstructionBuilder {
    pub fn new() -> Self {
        InstructionBuilder {
            next_register: 0,
            next_label: 0,
        }
    }
    
    pub fn next_register(&mut self) -> String {
        let reg = format!("r{}", self.next_register);
        self.next_register += 1;
        reg
    }
    
    pub fn next_label(&mut self, prefix: &str) -> String {
        let label = format!("{}{}", prefix, self.next_label);
        self.next_label += 1;
        label
    }
    
    pub fn build_add(&mut self, left: Operand, right: Operand) -> (String, Instruction) {
        let dest = self.next_register();
        let instr = Instruction::Add { dest: dest.clone(), left, right };
        (dest, instr)
    }
    
    pub fn build_call(&mut self, func: String, args: Vec<Operand>, has_return: bool) -> (Option<String>, Instruction) {
        let dest = if has_return {
            Some(self.next_register())
        } else {
            None
        };
        let instr = Instruction::Call { dest: dest.clone(), func, args };
        (dest, instr)
    }
}

impl Default for InstructionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
