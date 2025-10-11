// src/backend/execution/raze/codegen/aarch64.rs
//! ARM64/AArch64 code generation backend
//! 
//! Generates native AArch64 machine code from MIR.
//! Follows ARM64 AAPCS calling convention.

use super::assembler::Assembler;
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

/// AArch64 register encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AArch64Reg {
    X0 = 0, X1 = 1, X2 = 2, X3 = 3, X4 = 4, X5 = 5, X6 = 6, X7 = 7,
    X8 = 8, X9 = 9, X10 = 10, X11 = 11, X12 = 12, X13 = 13, X14 = 14, X15 = 15,
    X16 = 16, X17 = 17, X18 = 18, X19 = 19, X20 = 20, X21 = 21, X22 = 22, X23 = 23,
    X24 = 24, X25 = 25, X26 = 26, X27 = 27, X28 = 28, X29 = 29, X30 = 30, SP = 31,
}

impl AArch64Reg {
    fn from_id(id: u8) -> Self {
        match id {
            0 => AArch64Reg::X0, 1 => AArch64Reg::X1, 2 => AArch64Reg::X2, 3 => AArch64Reg::X3,
            4 => AArch64Reg::X4, 5 => AArch64Reg::X5, 6 => AArch64Reg::X6, 7 => AArch64Reg::X7,
            8 => AArch64Reg::X8, 9 => AArch64Reg::X9, 10 => AArch64Reg::X10, 11 => AArch64Reg::X11,
            12 => AArch64Reg::X12, 13 => AArch64Reg::X13, 14 => AArch64Reg::X14, 15 => AArch64Reg::X15,
            _ => AArch64Reg::X0,
        }
    }
    
    fn encoding(&self) -> u8 { *self as u8 }
}

/// ARM64 AAPCS calling convention
struct AAPCS;

impl CallingConvention for AAPCS {
    fn int_arg_regs(&self) -> &[PhysicalReg] {
        static REGS: &[PhysicalReg] = &[
            PhysicalReg { id: 0 }, PhysicalReg { id: 1 }, PhysicalReg { id: 2 }, PhysicalReg { id: 3 },
            PhysicalReg { id: 4 }, PhysicalReg { id: 5 }, PhysicalReg { id: 6 }, PhysicalReg { id: 7 }
        ];
        REGS
    }
    
    fn float_arg_regs(&self) -> &[PhysicalReg] { &[] }
    fn return_reg(&self) -> PhysicalReg { PhysicalReg::new(0) }
    
    fn callee_saved_regs(&self) -> &[PhysicalReg] {
        static REGS: &[PhysicalReg] = &[
            PhysicalReg { id: 19 }, PhysicalReg { id: 20 }, PhysicalReg { id: 21 }, PhysicalReg { id: 22 },
            PhysicalReg { id: 23 }, PhysicalReg { id: 24 }, PhysicalReg { id: 25 }, PhysicalReg { id: 26 },
            PhysicalReg { id: 27 }, PhysicalReg { id: 28 }
        ];
        REGS
    }
    
    fn caller_saved_regs(&self) -> &[PhysicalReg] {
        static REGS: &[PhysicalReg] = &[
            PhysicalReg { id: 0 }, PhysicalReg { id: 1 }, PhysicalReg { id: 2 }, PhysicalReg { id: 3 },
            PhysicalReg { id: 4 }, PhysicalReg { id: 5 }, PhysicalReg { id: 6 }, PhysicalReg { id: 7 },
            PhysicalReg { id: 8 }, PhysicalReg { id: 9 }, PhysicalReg { id: 10 }, PhysicalReg { id: 11 },
            PhysicalReg { id: 12 }, PhysicalReg { id: 13 }, PhysicalReg { id: 14 }, PhysicalReg { id: 15 },
            PhysicalReg { id: 16 }, PhysicalReg { id: 17 }, PhysicalReg { id: 18 }
        ];
        REGS
    }
}

/// AArch64 code generator
pub struct AArch64CodeGen {
    asm: Assembler,
    labels: LabelResolver,
    reg_alloc: RegisterAllocator,
    vreg_to_preg: HashMap<Reg, AArch64Reg>,
}

impl AArch64CodeGen {
    pub fn new() -> Self {
        let available = vec![
            PhysicalReg::new(0), PhysicalReg::new(1), PhysicalReg::new(2), PhysicalReg::new(3),
            PhysicalReg::new(4), PhysicalReg::new(5), PhysicalReg::new(6), PhysicalReg::new(7),
            PhysicalReg::new(8), PhysicalReg::new(9), PhysicalReg::new(10), PhysicalReg::new(11),
        ];
        
        Self {
            asm: Assembler::new(),
            labels: LabelResolver::new(),
            reg_alloc: RegisterAllocator::new(available),
            vreg_to_preg: HashMap::new(),
        }
    }
    
    fn map_reg(&mut self, vreg: Reg) -> AArch64Reg {
        if let Some(&preg) = self.vreg_to_preg.get(&vreg) {
            return preg;
        }
        let preg = self.reg_alloc.allocate(vreg);
        let aarch64_reg = AArch64Reg::from_id(preg.id);
        self.vreg_to_preg.insert(vreg, aarch64_reg);
        aarch64_reg
    }
    
    fn emit_prologue(&mut self) {
        // stp x29, x30, [sp, #-16]!
        let instr = 0xA9BF7BFD;
        self.asm.emit_u32(instr);
        
        // mov x29, sp
        let instr = 0x910003FD;
        self.asm.emit_u32(instr);
    }
    
    fn emit_epilogue(&mut self) {
        // ldp x29, x30, [sp], #16
        let instr = 0xA8C17BFD;
        self.asm.emit_u32(instr);
        
        // ret
        let instr = 0xD65F03C0;
        self.asm.emit_u32(instr);
    }
    
    fn emit_mov_reg_imm(&mut self, dest: AArch64Reg, value: i64) {
        // MOVZ Xd, #imm16, LSL #0
        let imm16 = (value & 0xFFFF) as u32;
        let instr = 0xD2800000 | (imm16 << 5) | (dest.encoding() as u32);
        self.asm.emit_u32(instr);
        
        // If value > 16 bits, use MOVK for upper bits
        if value > 0xFFFF || value < 0 {
            let imm16 = ((value >> 16) & 0xFFFF) as u32;
            let instr = 0xF2A00000 | (imm16 << 5) | (dest.encoding() as u32);
            self.asm.emit_u32(instr);
        }
    }
    
    fn emit_mov_reg_reg(&mut self, dest: AArch64Reg, src: AArch64Reg) {
        // MOV Xd, Xm (alias for ORR Xd, XZR, Xm)
        let instr = 0xAA0003E0 | ((src.encoding() as u32) << 16) | (dest.encoding() as u32);
        self.asm.emit_u32(instr);
    }
    
    fn emit_add_reg_reg(&mut self, dest: AArch64Reg, left: AArch64Reg, right: AArch64Reg) {
        // ADD Xd, Xn, Xm
        let instr = 0x8B000000 
            | ((right.encoding() as u32) << 16)
            | ((left.encoding() as u32) << 5)
            | (dest.encoding() as u32);
        self.asm.emit_u32(instr);
    }
    
    fn emit_sub_reg_reg(&mut self, dest: AArch64Reg, left: AArch64Reg, right: AArch64Reg) {
        // SUB Xd, Xn, Xm
        let instr = 0xCB000000
            | ((right.encoding() as u32) << 16)
            | ((left.encoding() as u32) << 5)
            | (dest.encoding() as u32);
        self.asm.emit_u32(instr);
    }
    
    fn emit_mul_reg_reg(&mut self, dest: AArch64Reg, left: AArch64Reg, right: AArch64Reg) {
        // MUL Xd, Xn, Xm (alias for MADD Xd, Xn, Xm, XZR)
        let instr = 0x9B007C00
            | ((right.encoding() as u32) << 16)
            | ((left.encoding() as u32) << 5)
            | (dest.encoding() as u32);
        self.asm.emit_u32(instr);
    }
    
    fn generate_mir_instruction(&mut self, instr: &MIR) -> Result<(), RAZEError> {
        match instr {
            MIR::LoadImm { dest, value, .. } => {
                let dest_reg = self.map_reg(*dest);
                match value {
                    MIRImmediate::Int(i) => self.emit_mov_reg_imm(dest_reg, *i),
                    MIRImmediate::Bool(b) => self.emit_mov_reg_imm(dest_reg, if *b { 1 } else { 0 }),
                    MIRImmediate::Null => self.emit_mov_reg_imm(dest_reg, 0),
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
                self.emit_add_reg_reg(dest_reg, left_reg, right_reg);
            }
            
            MIR::SubInt { dest, left, right } => {
                let dest_reg = self.map_reg(*dest);
                let left_reg = self.map_reg(*left);
                let right_reg = self.map_reg(*right);
                self.emit_sub_reg_reg(dest_reg, left_reg, right_reg);
            }
            
            MIR::MulInt { dest, left, right } => {
                let dest_reg = self.map_reg(*dest);
                let left_reg = self.map_reg(*left);
                let right_reg = self.map_reg(*right);
                self.emit_mul_reg_reg(dest_reg, left_reg, right_reg);
            }
            
            MIR::Label(label) => {
                self.labels.define_label(*label, self.asm.position());
            }
            
            MIR::Return { value } => {
                if let Some(val) = value {
                    let val_reg = self.map_reg(*val);
                    if val_reg != AArch64Reg::X0 {
                        self.emit_mov_reg_reg(AArch64Reg::X0, val_reg);
                    }
                }
                self.emit_epilogue();
            }
            
            _ => {
                return Err(RAZEError::CodeGenError(format!("Unimplemented MIR: {:?}", instr)));
            }
        }
        Ok(())
    }
}

impl CodeGenerator for AArch64CodeGen {
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
        Architecture::AArch64
    }
}

impl Default for AArch64CodeGen {
    fn default() -> Self {
        Self::new()
    }
}
