// src/backend/execution/raze/optimization/constant_fold.rs
//! Constant folding optimization
//! 
//! Evaluates constant expressions at compile time.

use crate::backend::execution::raze::mir::{MIR, MIRImmediate, MIRType};
use std::collections::HashMap;

pub fn fold(mir: Vec<MIR>) -> Vec<MIR> {
    let mut result = Vec::new();
    let mut constants: HashMap<u8, i64> = HashMap::new();
    
    for instr in mir {
        match instr {
            // Track constant loads
            MIR::LoadImm { dest, value: MIRImmediate::Int(i), .. } => {
                constants.insert(dest.0, i);
                result.push(instr);
            }
            
            // Fold constant additions
            MIR::AddInt { dest, left, right } => {
                if let (Some(&l), Some(&r)) = (constants.get(&left.0), constants.get(&right.0)) {
                    let sum = l.wrapping_add(r);
                    constants.insert(dest.0, sum);
                    result.push(MIR::LoadImm {
                        dest,
                        value: MIRImmediate::Int(sum),
                        ty: MIRType::Int64,
                    });
                } else {
                    constants.remove(&dest.0);
                    result.push(instr);
                }
            }
            
            // Fold constant subtractions
            MIR::SubInt { dest, left, right } => {
                if let (Some(&l), Some(&r)) = (constants.get(&left.0), constants.get(&right.0)) {
                    let diff = l.wrapping_sub(r);
                    constants.insert(dest.0, diff);
                    result.push(MIR::LoadImm {
                        dest,
                        value: MIRImmediate::Int(diff),
                        ty: MIRType::Int64,
                    });
                } else {
                    constants.remove(&dest.0);
                    result.push(instr);
                }
            }
            
            // Fold constant multiplications
            MIR::MulInt { dest, left, right } => {
                if let (Some(&l), Some(&r)) = (constants.get(&left.0), constants.get(&right.0)) {
                    let product = l.wrapping_mul(r);
                    constants.insert(dest.0, product);
                    result.push(MIR::LoadImm {
                        dest,
                        value: MIRImmediate::Int(product),
                        ty: MIRType::Int64,
                    });
                } else {
                    constants.remove(&dest.0);
                    result.push(instr);
                }
            }
            
            // Other instructions invalidate constant tracking
            _ => {
                result.push(instr);
            }
        }
    }
    
    result
}
