// src/backend/cranelift/mod.rs

pub mod codegen;
pub mod jit;
pub mod aot;

pub use codegen::CodeGenerator;
pub use jit::JITCompiler;
pub use aot::AOTCompiler;

use crate::backend::ir::IRModule;
use crate::backend::CompiledProgram;
use crate::frontend::diagnostics::Diagnostics;

// Cranelift-based code generation - Part 3 of the backend
// This module handles the final code generation using Cranelift