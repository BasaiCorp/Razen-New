// src/backend/ir/mod.rs

pub mod generator;
pub mod instructions;
pub mod module;

pub use generator::IRGenerator;
pub use instructions::{Instruction, Operand, BasicBlock, MatchArm, InstructionBuilder};
pub use module::IRModule;


// Re-export types from module.rs
pub use module::{IRFunction, IRParam, IRGlobal};
