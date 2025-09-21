// src/backend/optimization/mod.rs

pub mod passes;
pub mod optimizer;

pub use optimizer::Optimizer;
pub use passes::*;

use crate::backend::ir::IRModule;
use crate::frontend::diagnostics::Diagnostics;

/// Optimization pass trait
pub trait OptimizationPass {
    fn name(&self) -> &'static str;
    fn run(&mut self, ir_module: &mut IRModule) -> Result<bool, Diagnostics>;
}

/// Optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    None,    // O0 - No optimizations
    Basic,   // O1 - Basic optimizations
    Standard, // O2 - Standard optimizations
    Aggressive, // O3 - Aggressive optimizations
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::Basic
    }
}

impl OptimizationLevel {
    pub fn from_level(level: u8) -> Self {
        match level {
            0 => OptimizationLevel::None,
            1 => OptimizationLevel::Basic,
            2 => OptimizationLevel::Standard,
            3 | _ => OptimizationLevel::Aggressive,
        }
    }
}
