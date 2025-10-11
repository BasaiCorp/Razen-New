// src/backend/execution/raze/codegen/common.rs
//! Common utilities for code generation

use crate::backend::execution::raze::mir::{Reg, Label};
use std::collections::HashMap;

/// Register allocator - maps virtual registers to physical registers
pub struct RegisterAllocator {
    /// Map from virtual register to physical register
    allocation: HashMap<Reg, PhysicalReg>,
    /// Available physical registers
    available: Vec<PhysicalReg>,
    /// Next spill slot
    next_spill: i32,
}

impl RegisterAllocator {
    pub fn new(physical_regs: Vec<PhysicalReg>) -> Self {
        Self {
            allocation: HashMap::new(),
            available: physical_regs,
            next_spill: 0,
        }
    }
    
    /// Allocate a physical register for a virtual register
    pub fn allocate(&mut self, vreg: Reg) -> PhysicalReg {
        if let Some(&preg) = self.allocation.get(&vreg) {
            return preg;
        }
        
        if let Some(preg) = self.available.pop() {
            self.allocation.insert(vreg, preg);
            preg
        } else {
            // Need to spill - for now, just reuse registers
            // TODO: Implement proper spilling
            PhysicalReg::new(0)
        }
    }
    
    /// Free a physical register
    pub fn free(&mut self, vreg: Reg) {
        if let Some(preg) = self.allocation.remove(&vreg) {
            self.available.push(preg);
        }
    }
    
    /// Get allocated physical register
    pub fn get(&self, vreg: Reg) -> Option<PhysicalReg> {
        self.allocation.get(&vreg).copied()
    }
}

/// Physical register representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalReg {
    pub id: u8,
}

impl PhysicalReg {
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

/// Label resolver - tracks label positions for jump instructions
pub struct LabelResolver {
    /// Map from label to position in code
    positions: HashMap<Label, usize>,
    /// Pending relocations (label, position in code)
    relocations: Vec<(Label, usize, RelocationType)>,
}

impl LabelResolver {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
            relocations: Vec::new(),
        }
    }
    
    /// Define a label at the current position
    pub fn define_label(&mut self, label: Label, position: usize) {
        self.positions.insert(label, position);
    }
    
    /// Add a relocation for a label reference
    pub fn add_relocation(&mut self, label: Label, position: usize, ty: RelocationType) {
        self.relocations.push((label, position, ty));
    }
    
    /// Resolve all relocations
    pub fn resolve(&self, code: &mut [u8]) -> Result<(), String> {
        for (label, position, ty) in &self.relocations {
            let target = self.positions.get(label)
                .ok_or_else(|| format!("Undefined label: {:?}", label))?;
            
            match ty {
                RelocationType::Rel32 => {
                    // Calculate relative offset (target - (position + 4))
                    let offset = (*target as i64) - ((position + 4) as i64);
                    if offset < i32::MIN as i64 || offset > i32::MAX as i64 {
                        return Err(format!("Relocation offset out of range: {}", offset));
                    }
                    
                    // Write 32-bit little-endian offset
                    let bytes = (offset as i32).to_le_bytes();
                    code[*position..*position + 4].copy_from_slice(&bytes);
                }
                
                RelocationType::Abs64 => {
                    // Write 64-bit absolute address
                    let bytes = (*target as u64).to_le_bytes();
                    code[*position..*position + 8].copy_from_slice(&bytes);
                }
            }
        }
        
        Ok(())
    }
    
    /// Get label position
    pub fn get_position(&self, label: Label) -> Option<usize> {
        self.positions.get(&label).copied()
    }
}

impl Default for LabelResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Relocation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelocationType {
    /// 32-bit relative offset (for jumps/calls)
    Rel32,
    /// 64-bit absolute address
    Abs64,
}

/// Calling convention abstraction
pub trait CallingConvention {
    /// Get registers used for integer arguments
    fn int_arg_regs(&self) -> &[PhysicalReg];
    
    /// Get registers used for float arguments
    fn float_arg_regs(&self) -> &[PhysicalReg];
    
    /// Get register used for return value
    fn return_reg(&self) -> PhysicalReg;
    
    /// Get callee-saved registers
    fn callee_saved_regs(&self) -> &[PhysicalReg];
    
    /// Get caller-saved registers
    fn caller_saved_regs(&self) -> &[PhysicalReg];
}
