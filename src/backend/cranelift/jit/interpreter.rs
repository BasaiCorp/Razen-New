// src/backend/cranelift/jit/interpreter.rs
// IR Interpreter for JIT execution - Resolves values for builtin function calls

use std::collections::HashMap;
use crate::backend::ir::{IRModule, Instruction, Operand};
use crate::frontend::diagnostics::Diagnostics;

/// IR Value types for interpretation
#[derive(Debug, Clone)]
pub enum IRValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

/// Simple IR Interpreter for JIT execution
/// This interprets IR instructions to resolve values for builtin function calls
pub struct IRInterpreter<'a> {
    ir_module: &'a IRModule,
    pub registers: HashMap<String, IRValue>,
    memory: HashMap<String, IRValue>,
}

impl<'a> IRInterpreter<'a> {
    pub fn new(ir_module: &'a IRModule) -> Self {
        Self {
            ir_module,
            registers: HashMap::new(),
            memory: HashMap::new(),
        }
    }
    
    /// Execute a single IR instruction
    pub fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), Diagnostics> {
        match instruction {
            Instruction::Assign { dest, src } => {
                let value = self.resolve_operand(src)?;
                self.registers.insert(dest.clone(), value);
            }
            
            Instruction::Add { dest, left, right } => {
                let left_val = self.resolve_operand(left)?;
                let right_val = self.resolve_operand(right)?;
                let result = self.perform_add(left_val, right_val)?;
                self.registers.insert(dest.clone(), result);
            }
            
            Instruction::Sub { dest, left, right } => {
                let left_val = self.resolve_operand(left)?;
                let right_val = self.resolve_operand(right)?;
                let result = self.perform_sub(left_val, right_val)?;
                self.registers.insert(dest.clone(), result);
            }
            
            Instruction::Mul { dest, left, right } => {
                let left_val = self.resolve_operand(left)?;
                let right_val = self.resolve_operand(right)?;
                let result = self.perform_mul(left_val, right_val)?;
                self.registers.insert(dest.clone(), result);
            }
            
            Instruction::Div { dest, left, right } => {
                let left_val = self.resolve_operand(left)?;
                let right_val = self.resolve_operand(right)?;
                let result = self.perform_div(left_val, right_val)?;
                self.registers.insert(dest.clone(), result);
            }
            
            Instruction::Alloca { dest, .. } => {
                // Create a memory location
                self.memory.insert(dest.clone(), IRValue::Null);
            }
            
            Instruction::Store { dest, src } => {
                let value = self.resolve_operand(src)?;
                if let Operand::Register(reg) = dest {
                    self.memory.insert(reg.clone(), value);
                }
            }
            
            Instruction::Load { dest, src } => {
                if let Operand::Register(reg) = src {
                    if let Some(value) = self.memory.get(reg) {
                        self.registers.insert(dest.clone(), value.clone());
                    }
                }
            }
            
            Instruction::Call { dest, func: _, args: _ } => {
                // Handle function calls (this will be processed separately)
                // For now, just store a placeholder if there's a destination
                if let Some(dest_reg) = dest {
                    self.registers.insert(dest_reg.clone(), IRValue::Null);
                }
            }
            
            // For other instructions, we don't need to simulate them for builtin calls
            _ => {}
        }
        
        Ok(())
    }
    
    /// Resolve an operand to its actual value
    pub fn resolve_operand(&self, operand: &Operand) -> Result<IRValue, Diagnostics> {
        match operand {
            Operand::Immediate(i) => Ok(IRValue::Integer(*i)),
            Operand::Float(f) => Ok(IRValue::Float(*f)),
            Operand::Bool(b) => Ok(IRValue::Boolean(*b)),
            Operand::String(s) => {
                // Handle string literals like @str0, @str1, etc.
                if s.starts_with("@str") {
                    if let Ok(index) = s[4..].parse::<usize>() {
                        if index < self.ir_module.strings.len() {
                            return Ok(IRValue::String(self.ir_module.strings[index].clone()));
                        }
                    }
                }
                Ok(IRValue::String(s.clone()))
            }
            Operand::Register(reg) => {
                if let Some(value) = self.registers.get(reg) {
                    Ok(value.clone())
                } else {
                    Ok(IRValue::Null)
                }
            }
            _ => Ok(IRValue::Null),
        }
    }
    
    /// Resolve an operand to a string for builtin function calls
    pub fn resolve_operand_to_string(&self, operand: &Operand) -> String {
        match self.resolve_operand(operand) {
            Ok(value) => match value {
                IRValue::Integer(i) => i.to_string(),
                IRValue::Float(f) => f.to_string(),
                IRValue::String(s) => s,
                IRValue::Boolean(b) => b.to_string(),
                IRValue::Null => "null".to_string(),
            },
            Err(_) => "error".to_string(),
        }
    }
    
    /// Perform addition operation
    fn perform_add(&self, left: IRValue, right: IRValue) -> Result<IRValue, Diagnostics> {
        match (left, right) {
            (IRValue::Integer(a), IRValue::Integer(b)) => Ok(IRValue::Integer(a + b)),
            (IRValue::Float(a), IRValue::Float(b)) => Ok(IRValue::Float(a + b)),
            (IRValue::Integer(a), IRValue::Float(b)) => Ok(IRValue::Float(a as f64 + b)),
            (IRValue::Float(a), IRValue::Integer(b)) => Ok(IRValue::Float(a + b as f64)),
            (IRValue::String(a), IRValue::String(b)) => Ok(IRValue::String(format!("{}{}", a, b))),
            (IRValue::String(a), other) => {
                let b_str = match other {
                    IRValue::Integer(i) => i.to_string(),
                    IRValue::Float(f) => f.to_string(),
                    IRValue::Boolean(b) => b.to_string(),
                    _ => "null".to_string(),
                };
                Ok(IRValue::String(format!("{}{}", a, b_str)))
            }
            _ => Ok(IRValue::Null),
        }
    }
    
    /// Perform subtraction operation
    fn perform_sub(&self, left: IRValue, right: IRValue) -> Result<IRValue, Diagnostics> {
        match (left, right) {
            (IRValue::Integer(a), IRValue::Integer(b)) => Ok(IRValue::Integer(a - b)),
            (IRValue::Float(a), IRValue::Float(b)) => Ok(IRValue::Float(a - b)),
            (IRValue::Integer(a), IRValue::Float(b)) => Ok(IRValue::Float(a as f64 - b)),
            (IRValue::Float(a), IRValue::Integer(b)) => Ok(IRValue::Float(a - b as f64)),
            _ => Ok(IRValue::Null),
        }
    }
    
    /// Perform multiplication operation
    fn perform_mul(&self, left: IRValue, right: IRValue) -> Result<IRValue, Diagnostics> {
        match (left, right) {
            (IRValue::Integer(a), IRValue::Integer(b)) => Ok(IRValue::Integer(a * b)),
            (IRValue::Float(a), IRValue::Float(b)) => Ok(IRValue::Float(a * b)),
            (IRValue::Integer(a), IRValue::Float(b)) => Ok(IRValue::Float(a as f64 * b)),
            (IRValue::Float(a), IRValue::Integer(b)) => Ok(IRValue::Float(a * b as f64)),
            _ => Ok(IRValue::Null),
        }
    }
    
    /// Perform division operation
    fn perform_div(&self, left: IRValue, right: IRValue) -> Result<IRValue, Diagnostics> {
        match (left, right) {
            (IRValue::Integer(a), IRValue::Integer(b)) => {
                if b != 0 {
                    Ok(IRValue::Integer(a / b))
                } else {
                    Ok(IRValue::Null) // Division by zero
                }
            }
            (IRValue::Float(a), IRValue::Float(b)) => {
                if b != 0.0 {
                    Ok(IRValue::Float(a / b))
                } else {
                    Ok(IRValue::Null) // Division by zero
                }
            }
            (IRValue::Integer(a), IRValue::Float(b)) => {
                if b != 0.0 {
                    Ok(IRValue::Float(a as f64 / b))
                } else {
                    Ok(IRValue::Null)
                }
            }
            (IRValue::Float(a), IRValue::Integer(b)) => {
                if b != 0 {
                    Ok(IRValue::Float(a / b as f64))
                } else {
                    Ok(IRValue::Null)
                }
            }
            _ => Ok(IRValue::Null),
        }
    }
}
