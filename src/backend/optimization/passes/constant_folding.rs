// src/backend/optimization/passes/constant_folding.rs

use crate::backend::optimization::OptimizationPass;
use crate::backend::ir::{IRModule, IRFunction, Instruction, Operand};
use crate::frontend::diagnostics::Diagnostics;
use std::collections::HashMap;

/// Constant Folding optimization pass
/// Evaluates constant expressions at compile time
pub struct ConstantFolding {
    verbose: bool,
}

impl ConstantFolding {
    pub fn new() -> Self {
        ConstantFolding { verbose: false }
    }
    
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    /// Try to evaluate a constant expression
    #[allow(dead_code)]
    fn evaluate_constant_expression(&self, 
                                  instruction: &Instruction, 
                                  _constants: &HashMap<String, Operand>) -> Option<Operand> {
        match instruction {
            Instruction::Add { left, right, .. } => {
                if let (Operand::Immediate(a), Operand::Immediate(b)) = (left, right) {
                    Some(Operand::Immediate(a + b))
                } else {
                    None
                }
            }
            Instruction::Sub { left, right, .. } => {
                if let (Operand::Immediate(a), Operand::Immediate(b)) = (left, right) {
                    Some(Operand::Immediate(a - b))
                } else {
                    None
                }
            }
            Instruction::Mul { left, right, .. } => {
                if let (Operand::Immediate(a), Operand::Immediate(b)) = (left, right) {
                    Some(Operand::Immediate(a * b))
                } else {
                    None
                }
            }
            Instruction::Div { left, right, .. } => {
                if let (Operand::Immediate(a), Operand::Immediate(b)) = (left, right) {
                    if *b != 0 {
                        Some(Operand::Immediate(a / b))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    /// Perform constant folding on a function
    fn fold_constants_in_function(&self, function: &mut IRFunction) -> bool {
        let mut changed = false;
        let mut constants: HashMap<String, Operand> = HashMap::new();
        
        for block in &mut function.basic_blocks {
            let mut new_instructions = Vec::new();
            
            for instruction in &block.instructions {
                match instruction {
                    // Try to fold arithmetic operations
                    Instruction::Add { dest, left, right } => {
                        if let (Operand::Immediate(a), Operand::Immediate(b)) = (left, right) {
                            let result = Operand::Immediate(a + b);
                            new_instructions.push(Instruction::Load { 
                                dest: dest.clone(), 
                                src: result.clone() 
                            });
                            constants.insert(dest.clone(), result);
                            changed = true;
                            
                            if self.verbose {
                                println!("   Folded constant addition in register '{}'", dest);
                            }
                        } else {
                            new_instructions.push(instruction.clone());
                        }
                    }
                    
                    Instruction::Sub { dest, left, right } => {
                        if let (Operand::Immediate(a), Operand::Immediate(b)) = (left, right) {
                            let result = Operand::Immediate(a - b);
                            new_instructions.push(Instruction::Load { 
                                dest: dest.clone(), 
                                src: result.clone() 
                            });
                            constants.insert(dest.clone(), result);
                            changed = true;
                            
                            if self.verbose {
                                println!("   Folded constant subtraction in register '{}'", dest);
                            }
                        } else {
                            new_instructions.push(instruction.clone());
                        }
                    }
                    
                    Instruction::Mul { dest, left, right } => {
                        if let (Operand::Immediate(a), Operand::Immediate(b)) = (left, right) {
                            let result = Operand::Immediate(a * b);
                            new_instructions.push(Instruction::Load { 
                                dest: dest.clone(), 
                                src: result.clone() 
                            });
                            constants.insert(dest.clone(), result);
                            changed = true;
                            
                            if self.verbose {
                                println!("   Folded constant multiplication in register '{}'", dest);
                            }
                        } else {
                            new_instructions.push(instruction.clone());
                        }
                    }
                    
                    Instruction::Div { dest, left, right } => {
                        if let (Operand::Immediate(a), Operand::Immediate(b)) = (left, right) {
                            if *b != 0 {
                                let result = Operand::Immediate(a / b);
                                new_instructions.push(Instruction::Load { 
                                    dest: dest.clone(), 
                                    src: result.clone() 
                                });
                                constants.insert(dest.clone(), result);
                                changed = true;
                                
                                if self.verbose {
                                    println!("   Folded constant division in register '{}'", dest);
                                }
                            } else {
                                new_instructions.push(instruction.clone());
                            }
                        } else {
                            new_instructions.push(instruction.clone());
                        }
                    }
                    
                    // Track constant loads
                    Instruction::Load { dest, src } => {
                        match src {
                            Operand::Immediate(_) | Operand::String(_) | Operand::Bool(_) => {
                                constants.insert(dest.clone(), src.clone());
                            }
                            _ => {}
                        }
                        new_instructions.push(instruction.clone());
                    }
                    
                    // Other instructions
                    _ => {
                        new_instructions.push(instruction.clone());
                    }
                }
            }
            
            if changed {
                block.instructions = new_instructions;
            }
        }
        
        changed
    }
    
    /// Perform algebraic simplifications
    fn perform_algebraic_simplifications(&self, function: &mut IRFunction) -> bool {
        let mut changed = false;
        
        for block in &mut function.basic_blocks {
            let mut new_instructions = Vec::new();
            
            for instruction in &block.instructions {
                match instruction {
                    // x + 0 = x, 0 + x = x
                    Instruction::Add { dest, left, right } => {
                        match (left, right) {
                            (Operand::Immediate(0), _) => {
                                new_instructions.push(Instruction::Load { 
                                    dest: dest.clone(), 
                                    src: right.clone() 
                                });
                                changed = true;
                            }
                            (_, Operand::Immediate(0)) => {
                                new_instructions.push(Instruction::Load { 
                                    dest: dest.clone(), 
                                    src: left.clone() 
                                });
                                changed = true;
                            }
                            _ => {
                                new_instructions.push(instruction.clone());
                            }
                        }
                    }
                    
                    // x * 1 = x, 1 * x = x
                    // x * 0 = 0, 0 * x = 0
                    Instruction::Mul { dest, left, right } => {
                        match (left, right) {
                            (Operand::Immediate(1), _) => {
                                new_instructions.push(Instruction::Load { 
                                    dest: dest.clone(), 
                                    src: right.clone() 
                                });
                                changed = true;
                            }
                            (_, Operand::Immediate(1)) => {
                                new_instructions.push(Instruction::Load { 
                                    dest: dest.clone(), 
                                    src: left.clone() 
                                });
                                changed = true;
                            }
                            (Operand::Immediate(0), _) | (_, Operand::Immediate(0)) => {
                                new_instructions.push(Instruction::Load { 
                                    dest: dest.clone(), 
                                    src: Operand::Immediate(0) 
                                });
                                changed = true;
                            }
                            _ => {
                                new_instructions.push(instruction.clone());
                            }
                        }
                    }
                    
                    _ => {
                        new_instructions.push(instruction.clone());
                    }
                }
            }
            
            if changed {
                block.instructions = new_instructions;
            }
        }
        
        changed
    }
    
    /// Helper to get immediate value from a register if it's a constant
    #[allow(dead_code)]
    fn get_immediate_value(&self, _register: &str) -> Option<i64> {
        // This is a simplified version - in a real implementation,
        // we'd track constants across the function
        None
    }
}

impl OptimizationPass for ConstantFolding {
    fn name(&self) -> &'static str {
        "Constant Folding"
    }
    
    fn run(&mut self, ir_module: &mut IRModule) -> Result<bool, Diagnostics> {
        let mut changed = false;
        
        for function in &mut ir_module.functions {
            // Perform constant folding
            if self.fold_constants_in_function(function) {
                changed = true;
            }
            
            // Perform algebraic simplifications
            if self.perform_algebraic_simplifications(function) {
                changed = true;
            }
        }
        
        Ok(changed)
    }
}

impl Default for ConstantFolding {
    fn default() -> Self {
        Self::new()
    }
}
