// src/backend/cranelift/mod.rs

pub mod codegen;
pub mod jit;
pub mod aot;

pub use codegen::CodeGenerator;
pub use jit::JITCompiler;
pub use aot::AOTCompiler;


// Cranelift-based code generation - Part 3 of the backend
// This module handles the final code generation using Cranelift