// src/backend/optimization/passes/dead_code_elimination.rs

use crate::backend::optimization::OptimizationPass;
use crate::backend::ir::{IRModule, IRFunction, BasicBlock, Instruction};
use crate::frontend::diagnostics::Diagnostics;
use std::collections::HashSet;

/// Dead Code Elimination optimization pass
/// Removes unreachable code and unused instructions
pub struct DeadCodeElimination {
    verbose: bool,
}

impl DeadCodeElimination {
    pub fn new() -> Self {
        DeadCodeElimination { verbose: false }
    }
    
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    /// Find all reachable basic blocks from the entry block
    fn find_reachable_blocks(&self, function: &IRFunction) -> HashSet<usize> {
        let mut reachable = HashSet::new();
        let mut worklist = Vec::new();
        
        if !function.basic_blocks.is_empty() {
            worklist.push(0); // Entry block is always reachable
        }
        
        while let Some(block_index) = worklist.pop() {
            if reachable.contains(&block_index) {
                continue;
            }
            
            reachable.insert(block_index);
            
            if let Some(block) = function.basic_blocks.get(block_index) {
                // Add successor blocks based on terminator
                if let Some(ref terminator) = block.terminator {
                    match terminator {
                        Instruction::Branch { target } => {
                            if let Ok(target_index) = target.parse::<usize>() {
                                worklist.push(target_index);
                            }
                        }
                        Instruction::BranchIf { true_target, false_target, .. } => {
                            if let Ok(true_index) = true_target.parse::<usize>() {
                                worklist.push(true_index);
                            }
                            if let Ok(false_index) = false_target.parse::<usize>() {
                                worklist.push(false_index);
                            }
                        }
                        _ => {} // Return, etc. don't have successors
                    }
                }
            }
        }
        
        reachable
    }
    
    /// Remove unreachable basic blocks from a function
    fn eliminate_unreachable_blocks(&self, function: &mut IRFunction) -> bool {
        let reachable = self.find_reachable_blocks(function);
        let original_count = function.basic_blocks.len();
        
        // Keep only reachable blocks, maintaining order
        let mut new_blocks = Vec::new();
        for (i, block) in function.basic_blocks.iter().enumerate() {
            if reachable.contains(&i) {
                new_blocks.push(block.clone());
            }
        }
        
        function.basic_blocks = new_blocks;
        let removed_count = original_count - function.basic_blocks.len();
        
        if self.verbose && removed_count > 0 {
            println!("   Removed {} unreachable blocks from function '{}'", 
                     removed_count, function.name);
        }
        
        removed_count > 0
    }
    
    /// Find used registers in a function
    fn find_used_registers(&self, function: &IRFunction) -> HashSet<String> {
        let mut used = HashSet::new();
        
        for block in &function.basic_blocks {
            for instruction in &block.instructions {
                // Add registers used by this instruction
                self.add_used_registers_from_instruction(instruction, &mut used);
            }
            
            if let Some(ref terminator) = block.terminator {
                self.add_used_registers_from_instruction(terminator, &mut used);
            }
        }
        
        used
    }
    
    /// Add registers used by an instruction to the used set
    fn add_used_registers_from_instruction(&self, instruction: &Instruction, used: &mut HashSet<String>) {
        match instruction {
            Instruction::Load { src, .. } => {
                if let crate::backend::ir::Operand::Register(reg) = src {
                    used.insert(reg.clone());
                }
            }
            Instruction::Store { dest, src } => {
                if let crate::backend::ir::Operand::Register(reg) = dest {
                    used.insert(reg.clone());
                }
                if let crate::backend::ir::Operand::Register(reg) = src {
                    used.insert(reg.clone());
                }
            }
            Instruction::Add { left, right, .. } |
            Instruction::Sub { left, right, .. } |
            Instruction::Mul { left, right, .. } |
            Instruction::Div { left, right, .. } => {
                if let crate::backend::ir::Operand::Register(reg) = left {
                    used.insert(reg.clone());
                }
                if let crate::backend::ir::Operand::Register(reg) = right {
                    used.insert(reg.clone());
                }
            }
            Instruction::Call { args, .. } => {
                for arg in args {
                    if let crate::backend::ir::Operand::Register(reg) = arg {
                        used.insert(reg.clone());
                    }
                }
            }
            Instruction::Return { value } => {
                if let Some(val) = value {
                    if let crate::backend::ir::Operand::Register(reg) = val {
                        used.insert(reg.clone());
                    }
                }
            }
            Instruction::BranchIf { condition, .. } => {
                if let crate::backend::ir::Operand::Register(reg) = condition {
                    used.insert(reg.clone());
                }
            }
            _ => {} // Other instructions
        }
    }
    
    /// Remove unused instructions (dead stores, unused computations)
    fn eliminate_dead_instructions(&self, function: &mut IRFunction) -> bool {
        let used_registers = self.find_used_registers(function);
        let mut changed = false;
        
        for block in &mut function.basic_blocks {
            let original_len = block.instructions.len();
            
            // Keep instructions that are either:
            // 1. Have side effects (calls, stores to memory)
            // 2. Define registers that are used
            block.instructions.retain(|instruction| {
                match instruction {
                    // Always keep instructions with side effects
                    Instruction::Call { .. } |
                    Instruction::Store { .. } => true,
                    
                    // Keep instructions that define used registers
                    Instruction::Alloca { dest, .. } |
                    Instruction::Load { dest, .. } |
                    Instruction::Add { dest, .. } |
                    Instruction::Sub { dest, .. } |
                    Instruction::Mul { dest, .. } |
                    Instruction::Div { dest, .. } => {
                        used_registers.contains(dest)
                    }
                    
                    // Keep other instructions for now
                    _ => true,
                }
            });
            
            if block.instructions.len() < original_len {
                changed = true;
                if self.verbose {
                    println!("   Removed {} dead instructions from block '{}'", 
                             original_len - block.instructions.len(), block.label);
                }
            }
        }
        
        changed
    }
}

impl OptimizationPass for DeadCodeElimination {
    fn name(&self) -> &'static str {
        "Dead Code Elimination"
    }
    
    fn run(&mut self, ir_module: &mut IRModule) -> Result<bool, Diagnostics> {
        let mut changed = false;
        
        for function in &mut ir_module.functions {
            // Remove unreachable blocks
            if self.eliminate_unreachable_blocks(function) {
                changed = true;
            }
            
            // Remove dead instructions
            if self.eliminate_dead_instructions(function) {
                changed = true;
            }
        }
        
        Ok(changed)
    }
}

impl Default for DeadCodeElimination {
    fn default() -> Self {
        Self::new()
    }
}
