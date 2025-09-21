// src/backend/optimization/passes/unused_variable_elimination.rs

use crate::backend::optimization::OptimizationPass;
use crate::backend::ir::{IRModule, IRFunction, Instruction};
use crate::frontend::diagnostics::Diagnostics;
use std::collections::{HashMap, HashSet};

/// Unused Variable Elimination optimization pass
/// Removes variables that are allocated but never used
pub struct UnusedVariableElimination {
    verbose: bool,
}

impl UnusedVariableElimination {
    pub fn new() -> Self {
        UnusedVariableElimination { verbose: false }
    }
    
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    /// Analyze variable usage in a function
    fn analyze_variable_usage(&self, function: &IRFunction) -> (HashSet<String>, HashSet<String>) {
        let mut allocated_vars = HashSet::new();
        let mut used_vars = HashSet::new();
        
        for block in &function.basic_blocks {
            for instruction in &block.instructions {
                match instruction {
                    // Track variable allocations
                    Instruction::Alloca { dest, .. } => {
                        allocated_vars.insert(dest.clone());
                    }
                    
                    // Track variable uses
                    Instruction::Load { dest: _, src } => {
                        if let crate::backend::ir::Operand::Register(reg) = src {
                            used_vars.insert(reg.clone());
                        }
                    }
                    Instruction::Store { dest, src: _ } => {
                        if let crate::backend::ir::Operand::Register(reg) = dest {
                            used_vars.insert(reg.clone());
                        }
                    }
                    
                    // Track uses in arithmetic operations
                    Instruction::Add { left, right, .. } |
                    Instruction::Sub { left, right, .. } |
                    Instruction::Mul { left, right, .. } |
                    Instruction::Div { left, right, .. } => {
                        if let crate::backend::ir::Operand::Register(reg) = left {
                            used_vars.insert(reg.clone());
                        }
                        if let crate::backend::ir::Operand::Register(reg) = right {
                            used_vars.insert(reg.clone());
                        }
                    }
                    
                    // Track uses in function calls
                    Instruction::Call { args, .. } => {
                        for arg in args {
                            if let crate::backend::ir::Operand::Register(reg) = arg {
                                used_vars.insert(reg.clone());
                            }
                        }
                    }
                    
                    // Track uses in returns
                    Instruction::Return { value } => {
                        if let Some(val) = value {
                            if let crate::backend::ir::Operand::Register(reg) = val {
                                used_vars.insert(reg.clone());
                            }
                        }
                    }
                    
                    // Track uses in branches
                    Instruction::BranchIf { condition, .. } => {
                        if let crate::backend::ir::Operand::Register(reg) = condition {
                            used_vars.insert(reg.clone());
                        }
                    }
                    
                    _ => {}
                }
            }
            
            // Check terminator for uses
            if let Some(ref terminator) = block.terminator {
                match terminator {
                    Instruction::Return { value } => {
                        if let Some(val) = value {
                            if let crate::backend::ir::Operand::Register(reg) = val {
                                used_vars.insert(reg.clone());
                            }
                        }
                    }
                    Instruction::BranchIf { condition, .. } => {
                        if let crate::backend::ir::Operand::Register(reg) = condition {
                            used_vars.insert(reg.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        
        (allocated_vars, used_vars)
    }
    
    /// Remove unused variable allocations
    fn remove_unused_allocations(&self, function: &mut IRFunction) -> bool {
        let (allocated_vars, used_vars) = self.analyze_variable_usage(function);
        
        // Find unused variables
        let unused_vars: HashSet<_> = allocated_vars.difference(&used_vars).collect();
        
        if unused_vars.is_empty() {
            return false;
        }
        
        let mut changed = false;
        
        // Remove allocation instructions for unused variables
        for block in &mut function.basic_blocks {
            let original_len = block.instructions.len();
            
            block.instructions.retain(|instruction| {
                match instruction {
                    Instruction::Alloca { dest, .. } => {
                        if unused_vars.contains(dest) {
                            if self.verbose {
                                println!("   Removed unused variable allocation: {}", dest);
                            }
                            false
                        } else {
                            true
                        }
                    }
                    _ => true,
                }
            });
            
            if block.instructions.len() < original_len {
                changed = true;
            }
        }
        
        changed
    }
    
    /// Remove redundant assignments (assignments to variables that are never read)
    fn remove_redundant_assignments(&self, function: &mut IRFunction) -> bool {
        let mut assignments: HashMap<String, Vec<usize>> = HashMap::new();
        let mut reads: HashSet<String> = HashSet::new();
        
        // First pass: collect all assignments and reads
        for (block_idx, block) in function.basic_blocks.iter().enumerate() {
            for (instr_idx, instruction) in block.instructions.iter().enumerate() {
                match instruction {
                    Instruction::Load { dest, .. } |
                    Instruction::Add { dest, .. } |
                    Instruction::Sub { dest, .. } |
                    Instruction::Mul { dest, .. } |
                    Instruction::Div { dest, .. } => {
                        assignments.entry(dest.clone())
                                  .or_insert_with(Vec::new)
                                  .push(block_idx * 1000 + instr_idx); // Simple encoding
                    }
                    
                    // Track reads
                    Instruction::Store { src, .. } => {
                        if let crate::backend::ir::Operand::Register(reg) = src {
                            reads.insert(reg.clone());
                        }
                    }
                    Instruction::Return { value: Some(val) } => {
                        if let crate::backend::ir::Operand::Register(reg) = val {
                            reads.insert(reg.clone());
                        }
                    }
                    Instruction::Call { args, .. } => {
                        for arg in args {
                            if let crate::backend::ir::Operand::Register(reg) = arg {
                                reads.insert(reg.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        
        // Find variables that are assigned but never read
        let mut unused_assignments = HashSet::new();
        for (var, _) in &assignments {
            if !reads.contains(var) {
                unused_assignments.insert(var.clone());
            }
        }
        
        if unused_assignments.is_empty() {
            return false;
        }
        
        let mut changed = false;
        
        // Remove assignments to variables that are never read
        for block in &mut function.basic_blocks {
            let original_len = block.instructions.len();
            
            block.instructions.retain(|instruction| {
                match instruction {
                    Instruction::Load { dest, .. } |
                    Instruction::Add { dest, .. } |
                    Instruction::Sub { dest, .. } |
                    Instruction::Mul { dest, .. } |
                    Instruction::Div { dest, .. } => {
                        if unused_assignments.contains(dest) {
                            if self.verbose {
                                println!("   Removed redundant assignment to: {}", dest);
                            }
                            false
                        } else {
                            true
                        }
                    }
                    _ => true,
                }
            });
            
            if block.instructions.len() < original_len {
                changed = true;
            }
        }
        
        changed
    }
}

impl OptimizationPass for UnusedVariableElimination {
    fn name(&self) -> &'static str {
        "Unused Variable Elimination"
    }
    
    fn run(&mut self, ir_module: &mut IRModule) -> Result<bool, Diagnostics> {
        let mut changed = false;
        
        for function in &mut ir_module.functions {
            // Remove unused allocations
            if self.remove_unused_allocations(function) {
                changed = true;
            }
            
            // Remove redundant assignments
            if self.remove_redundant_assignments(function) {
                changed = true;
            }
        }
        
        Ok(changed)
    }
}

impl Default for UnusedVariableElimination {
    fn default() -> Self {
        Self::new()
    }
}
