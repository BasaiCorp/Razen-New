// src/backend/ir/mod.rs

pub mod generator;
pub mod instructions;
pub mod module;

pub use generator::IRGenerator;
pub use instructions::{Instruction, Operand, BasicBlock};
pub use module::IRModule;

use crate::backend::semantic::AnalyzedProgram;
use crate::frontend::diagnostics::Diagnostics;

// Re-export types from module.rs
pub use module::{IRFunction, IRParam, IRGlobal};
