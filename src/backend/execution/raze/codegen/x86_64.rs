// src/backend/execution/raze/codegen/x86_64.rs
//! x86_64 code generation backend
//! 
//! Generates native x86_64 machine code from MIR.
//! Follows System V AMD64 ABI calling convention.

use super::assembler::{Assembler, modrm, rex};
use super::common::{RegisterAllocator, LabelResolver, PhysicalReg, CallingConvention};
use super::{CodeGenerator, Architecture};
use crate::backend::execution::raze::mir::{MIR, MIRFunction, Reg, MIRImmediate};
use crate::backend::execution::raze::RAZEError;
use std::collections::HashMap;

// May be needed for future label resolution improvements
#[allow(unused_imports)]
use super::common::RelocationType;
#[allow(unused_imports)]
use crate::backend::execution::raze::mir::Label;

/// x86_64 register encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum X86Reg {
    RAX = 0, RCX = 1, RDX = 2, RBX = 3,
    RSP = 4, RBP = 5, RSI = 6, RDI = 7,
    R8  = 8, R9  = 9, R10 = 10, R11 = 11,
    R12 = 12, R13 = 13, R14 = 14, R15 = 15,
}

impl X86Reg {
    fn from_id(id: u8) -> Self {
        match id {
            0 => X86Reg::RAX, 1 => X86Reg::RCX, 2 => X86Reg::RDX, 3 => X86Reg::RBX,
            4 => X86Reg::RSP, 5 => X86Reg::RBP, 6 => X86Reg::RSI, 7 => X86Reg::RDI,
            8 => X86Reg::R8, 9 => X86Reg::R9, 10 => X86Reg::R10, 11 => X86Reg::R11,
            12 => X86Reg::R12, 13 => X86Reg::R13, 14 => X86Reg::R14, 15 => X86Reg::R15,
            _ => X86Reg::RAX,
        }
    }
    
    fn needs_rex(&self) -> bool { (*self as u8) >= 8 }
    fn encoding(&self) -> u8 { (*self as u8) & 0x7 }
}

/// System V AMD64 calling convention
struct SystemVABI;

impl CallingConvention for SystemVABI {
    fn int_arg_regs(&self) -> &[PhysicalReg] {
        static REGS: &[PhysicalReg] = &[
            PhysicalReg { id: 7 }, PhysicalReg { id: 6 }, PhysicalReg { id: 2 },
            PhysicalReg { id: 1 }, PhysicalReg { id: 8 }, PhysicalReg { id: 9 }
        ];
        REGS
    }
    
    fn float_arg_regs(&self) -> &[PhysicalReg] { &[] }
    fn return_reg(&self) -> PhysicalReg { PhysicalReg::new(X86Reg::RAX as u8) }
    
    fn callee_saved_regs(&self) -> &[PhysicalReg] {
        static REGS: &[PhysicalReg] = &[
            PhysicalReg { id: 3 }, PhysicalReg { id: 12 }, PhysicalReg { id: 13 },
            PhysicalReg { id: 14 }, PhysicalReg { id: 15 }
        ];
        REGS
    }
    
    fn caller_saved_regs(&self) -> &[PhysicalReg] {
        static REGS: &[PhysicalReg] = &[
            PhysicalReg { id: 0 }, PhysicalReg { id: 1 }, PhysicalReg { id: 2 },
            PhysicalReg { id: 6 }, PhysicalReg { id: 7 }, PhysicalReg { id: 8 },
            PhysicalReg { id: 9 }, PhysicalReg { id: 10 }, PhysicalReg { id: 11 }
        ];
        REGS
    }
}

/// x86_64 code generator
pub struct X86_64CodeGen {
    asm: Assembler,
    labels: LabelResolver,
    reg_alloc: RegisterAllocator,
    vreg_to_preg: HashMap<Reg, X86Reg>,
}

impl X86_64CodeGen {
    pub fn new() -> Self {
        let available = vec![
            PhysicalReg::new(X86Reg::RAX as u8), PhysicalReg::new(X86Reg::RCX as u8),
            PhysicalReg::new(X86Reg::RDX as u8), PhysicalReg::new(X86Reg::RBX as u8),
            PhysicalReg::new(X86Reg::RSI as u8), PhysicalReg::new(X86Reg::RDI as u8),
            PhysicalReg::new(X86Reg::R8 as u8), PhysicalReg::new(X86Reg::R9 as u8),
            PhysicalReg::new(X86Reg::R10 as u8), PhysicalReg::new(X86Reg::R11 as u8),
        ];
        
        Self {
            asm: Assembler::new(),
            labels: LabelResolver::new(),
            reg_alloc: RegisterAllocator::new(available),
            vreg_to_preg: HashMap::new(),
        }
    }
    
    fn map_reg(&mut self, vreg: Reg) -> X86Reg {
        if let Some(&preg) = self.vreg_to_preg.get(&vreg) {
            return preg;
        }
        let preg = self.reg_alloc.allocate(vreg);
        let x86_reg = X86Reg::from_id(preg.id);
        self.vreg_to_preg.insert(vreg, x86_reg);
        x86_reg
    }
    
    fn emit_prologue(&mut self) {
        self.asm.emit_u8(0x55); // push rbp
        self.asm.emit_u8(rex(true, false, false, false));
        self.asm.emit_u8(0x89);
        self.asm.emit_u8(modrm(3, X86Reg::RSP.encoding(), X86Reg::RBP.encoding()));
    }
    
    fn emit_epilogue(&mut self) {
        self.asm.emit_u8(rex(true, false, false, false));
        self.asm.emit_u8(0x89);
        self.asm.emit_u8(modrm(3, X86Reg::RBP.encoding(), X86Reg::RSP.encoding()));
        self.asm.emit_u8(0x5D); // pop rbp
        self.asm.emit_u8(0xC3); // ret
    }
    
    fn emit_mov_reg_imm64(&mut self, dest: X86Reg, value: i64) {
        self.asm.emit_u8(rex(true, false, false, dest.needs_rex()));
        self.asm.emit_u8(0xB8 + dest.encoding());
        self.asm.emit_u64(value as u64);
    }
    
    fn emit_mov_reg_reg(&mut self, dest: X86Reg, src: X86Reg) {
        self.asm.emit_u8(rex(true, src.needs_rex(), false, dest.needs_rex()));
        self.asm.emit_u8(0x89);
        self.asm.emit_u8(modrm(3, src.encoding(), dest.encoding()));
    }
    
    fn emit_add_reg_reg(&mut self, dest: X86Reg, src: X86Reg) {
        self.asm.emit_u8(rex(true, src.needs_rex(), false, dest.needs_rex()));
        self.asm.emit_u8(0x01);
        self.asm.emit_u8(modrm(3, src.encoding(), dest.encoding()));
    }
    
    fn emit_sub_reg_reg(&mut self, dest: X86Reg, src: X86Reg) {
        self.asm.emit_u8(rex(true, src.needs_rex(), false, dest.needs_rex()));
        self.asm.emit_u8(0x29);
        self.asm.emit_u8(modrm(3, src.encoding(), dest.encoding()));
    }
    
    fn emit_imul_reg_reg(&mut self, dest: X86Reg, src: X86Reg) {
        self.asm.emit_u8(rex(true, dest.needs_rex(), false, src.needs_rex()));
        self.asm.emit_u8(0x0F);
        self.asm.emit_u8(0xAF);
        self.asm.emit_u8(modrm(3, dest.encoding(), src.encoding()));
    }
    
    fn generate_mir_instruction(&mut self, instr: &MIR) -> Result<(), RAZEError> {
        match instr {
            MIR::LoadImm { dest, value, .. } => {
                let dest_reg = self.map_reg(*dest);
                match value {
                    MIRImmediate::Int(i) => self.emit_mov_reg_imm64(dest_reg, *i),
                    MIRImmediate::Bool(b) => self.emit_mov_reg_imm64(dest_reg, if *b { 1 } else { 0 }),
                    MIRImmediate::Null => self.emit_mov_reg_imm64(dest_reg, 0),
                    _ => return Err(RAZEError::CodeGenError(format!("Unsupported immediate: {:?}", value))),
                }
            }
            
            MIR::Move { dest, src, .. } => {
                let dest_reg = self.map_reg(*dest);
                let src_reg = self.map_reg(*src);
                self.emit_mov_reg_reg(dest_reg, src_reg);
            }
            
            MIR::AddInt { dest, left, right } => {
                let dest_reg = self.map_reg(*dest);
                let left_reg = self.map_reg(*left);
                let right_reg = self.map_reg(*right);
                if dest_reg != left_reg {
                    self.emit_mov_reg_reg(dest_reg, left_reg);
                }
                self.emit_add_reg_reg(dest_reg, right_reg);
            }
            
            MIR::SubInt { dest, left, right } => {
                let dest_reg = self.map_reg(*dest);
                let left_reg = self.map_reg(*left);
                let right_reg = self.map_reg(*right);
                if dest_reg != left_reg {
                    self.emit_mov_reg_reg(dest_reg, left_reg);
                }
                self.emit_sub_reg_reg(dest_reg, right_reg);
            }
            
            MIR::MulInt { dest, left, right } => {
                let dest_reg = self.map_reg(*dest);
                let left_reg = self.map_reg(*left);
                let right_reg = self.map_reg(*right);
                if dest_reg != left_reg {
                    self.emit_mov_reg_reg(dest_reg, left_reg);
                }
                self.emit_imul_reg_reg(dest_reg, right_reg);
            }
            
            MIR::Label(label) => {
                self.labels.define_label(*label, self.asm.position());
            }
            
            MIR::Return { value } => {
                if let Some(val) = value {
                    let val_reg = self.map_reg(*val);
                    if val_reg != X86Reg::RAX {
                        self.emit_mov_reg_reg(X86Reg::RAX, val_reg);
                    }
                }
                self.emit_epilogue();
            }
            
            _ => {
                // Placeholder for other instructions
                return Err(RAZEError::CodeGenError(format!("Unimplemented MIR: {:?}", instr)));
            }
        }
        Ok(())
    }
}

impl CodeGenerator for X86_64CodeGen {
    fn generate(&mut self, mir: &[MIR]) -> Result<Vec<u8>, RAZEError> {
        self.emit_prologue();
        
        for instr in mir {
            self.generate_mir_instruction(instr)?;
        }
        
        let code = self.asm.buffer().to_vec();
        Ok(code)
    }
    
    fn generate_function(&mut self, func: &MIRFunction) -> Result<Vec<u8>, RAZEError> {
        self.generate(&func.instructions)
    }
    
    fn architecture(&self) -> Architecture {
        Architecture::X86_64
    }
}

impl Default for X86_64CodeGen {
    fn default() -> Self {
        Self::new()
    }
}
