// src/backend/execution/raze/mod.rs
//! RAZE - Razen Advanced Zero-overhead Engine
//! 
//! A lightweight, custom JIT/AOT compilation system with:
//! - Minimal dependencies (only libc)
//! - Full x86_64 and ARM64/AArch64 support
//! - Fast compilation with aggressive optimization
//! - Native machine code generation
//! 
//! Architecture:
//! - MIR: Mid-level Intermediate Representation (register-based, strongly-typed)
//! - JIT: Just-In-Time compilation with tiered optimization
//! - AOT: Ahead-Of-Time compilation to standalone executables
//! - Codegen: Multi-architecture native code generation
//! - Optimization: Constant folding, DCE, CSE, register allocation

pub mod mir;
pub mod memory;
pub mod codegen;
pub mod optimization;
pub mod jit;
pub mod aot;
pub mod runtime;

// Re-exports
pub use mir::{MIR, MIRBuilder, MIRFunction, MIRModule};
pub use jit::JITCompiler;
pub use aot::AOTCompiler;
pub use runtime::RAZERuntime;
pub use codegen::{Architecture, CodeGenerator};
pub use optimization::OptimizationLevel;

/// RAZE version
pub const VERSION: &str = "0.1.0";

/// RAZE compilation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationMode {
    /// Just-In-Time compilation
    JIT,
    /// Ahead-Of-Time compilation
    AOT,
    /// Hybrid mode (JIT for hot paths, AOT for cold paths)
    Hybrid,
    /// Adaptive mode (automatically choose best strategy)
    Adaptive,
}

impl Default for CompilationMode {
    fn default() -> Self {
        CompilationMode::JIT
    }
}

/// RAZE error types
#[derive(Debug, Clone)]
pub enum RAZEError {
    CompilationError(String),
    CodeGenError(String),
    OptimizationError(String),
    MemoryError(String),
    RuntimeError(String),
    UnsupportedArchitecture(String),
}

impl std::fmt::Display for RAZEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RAZEError::CompilationError(msg) => write!(f, "[ERROR] Compilation: {}", msg),
            RAZEError::CodeGenError(msg) => write!(f, "[ERROR] Code Generation: {}", msg),
            RAZEError::OptimizationError(msg) => write!(f, "[ERROR] Optimization: {}", msg),
            RAZEError::MemoryError(msg) => write!(f, "[ERROR] Memory: {}", msg),
            RAZEError::RuntimeError(msg) => write!(f, "[ERROR] Runtime: {}", msg),
            RAZEError::UnsupportedArchitecture(msg) => write!(f, "[ERROR] Unsupported Architecture: {}", msg),
        }
    }
}

impl std::error::Error for RAZEError {}

pub type RAZEResult<T> = Result<T, RAZEError>;
