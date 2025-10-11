// src/backend/execution/raze/optimization/dce.rs
//! Dead code elimination
//! 
//! Removes instructions that have no effect on program output.

use crate::backend::execution::raze::mir::MIR;
use std::collections::HashSet;

pub fn eliminate(mir: Vec<MIR>) -> Vec<MIR> {
    let mut used_regs = HashSet::new();
    let mut result = Vec::new();
    
    // Mark registers that are used
    for instr in &mir {
        match instr {
            MIR::Move { src, .. } => { used_regs.insert(src.0); }
            MIR::AddInt { left, right, .. } | MIR::SubInt { left, right, .. } 
            | MIR::MulInt { left, right, .. } | MIR::DivInt { left, right, .. } => {
                used_regs.insert(left.0);
                used_regs.insert(right.0);
            }
            MIR::Return { value: Some(v) } => { used_regs.insert(v.0); }
            MIR::Print { value } => { used_regs.insert(value.0); }
            _ => {}
        }
    }
    
    // Keep only instructions that define used registers or have side effects
    for instr in mir {
        let keep = match &instr {
            MIR::LoadImm { dest, .. } => used_regs.contains(&dest.0),
            MIR::Move { dest, .. } => used_regs.contains(&dest.0),
            MIR::AddInt { dest, .. } | MIR::SubInt { dest, .. } 
            | MIR::MulInt { dest, .. } | MIR::DivInt { dest, .. } => used_regs.contains(&dest.0),
            // Always keep control flow and side effects
            MIR::Label(_) | MIR::Jump { .. } | MIR::JumpIfZero { .. } 
            | MIR::JumpIfNotZero { .. } | MIR::Return { .. } 
            | MIR::Print { .. } | MIR::Call { .. } => true,
            _ => true, // Conservative: keep unknown instructions
        };
        
        if keep {
            result.push(instr);
        }
    }
    
    result
}
