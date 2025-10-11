// src/backend/execution/raze/codegen/mod.rs
//! Code generation backends for multiple architectures
//! 
//! Provides native machine code generation for:
//! - x86_64 (Intel/AMD 64-bit)
//! - AArch64 (ARM 64-bit)

pub mod common;
pub mod x86_64;
pub mod aarch64;
pub mod assembler;

pub use common::*;
pub use assembler::Assembler;

use crate::backend::execution::raze::mir::{MIR, MIRFunction};
use crate::backend::execution::raze::RAZEError;

/// Supported target architectures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    AArch64,
}

impl Architecture {
    /// Detect current architecture
    pub fn current() -> Self {
        #[cfg(target_arch = "x86_64")]
        return Architecture::X86_64;
        
        #[cfg(target_arch = "aarch64")]
        return Architecture::AArch64;
        
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        compile_error!("Unsupported architecture");
    }
    
    /// Get architecture name
    pub fn name(&self) -> &'static str {
        match self {
            Architecture::X86_64 => "x86_64",
            Architecture::AArch64 => "aarch64",
        }
    }
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Code generator trait
pub trait CodeGenerator {
    /// Generate machine code from MIR
    fn generate(&mut self, mir: &[MIR]) -> Result<Vec<u8>, RAZEError>;
    
    /// Generate code for a function
    fn generate_function(&mut self, func: &MIRFunction) -> Result<Vec<u8>, RAZEError>;
    
    /// Get target architecture
    fn architecture(&self) -> Architecture;
}

/// Create a code generator for the target architecture
pub fn create_codegen(arch: Architecture) -> Box<dyn CodeGenerator> {
    match arch {
        Architecture::X86_64 => Box::new(x86_64::X86_64CodeGen::new()),
        Architecture::AArch64 => Box::new(aarch64::AArch64CodeGen::new()),
    }
}

/// Create a code generator for the current architecture
pub fn create_native_codegen() -> Box<dyn CodeGenerator> {
    create_codegen(Architecture::current())
}
