// src/backend/cranelift/mod.rs

pub mod codegen;
pub mod jit; // New organized JIT module
pub mod aot;

// Keep the old jit.rs for reference (renamed to jit_old.rs)
#[allow(dead_code)]
mod jit_old;

pub use codegen::CodeGenerator;
pub use jit::RazenJIT; // Use the new organized JIT
pub use aot::AOTCompiler;

// Cranelift-based code generation - Part 3 of the backend
// This module handles the final code generation using Cranelift