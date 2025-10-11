// src/backend/execution/raze/optimization/mod.rs
//! Optimization pipeline for RAZE
//! 
//! Provides multiple optimization passes:
//! - Constant folding
//! - Dead code elimination (DCE)
//! - Common subexpression elimination (CSE)
//! - Peephole optimizations

pub mod constant_fold;
pub mod dce;
pub mod cse;
pub mod peephole;

use crate::backend::execution::raze::mir::MIR;

/// Optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    /// No optimization (O0) - fastest compilation
    None,
    /// Basic optimization (O1) - constant folding, DCE
    Basic,
    /// Standard optimization (O2) - O1 + CSE, peephole
    Standard,
    /// Aggressive optimization (O3) - O2 + multiple passes
    Aggressive,
}

impl OptimizationLevel {
    pub fn from_u8(level: u8) -> Self {
        match level {
            0 => OptimizationLevel::None,
            1 => OptimizationLevel::Basic,
            2 => OptimizationLevel::Standard,
            3 | _ => OptimizationLevel::Aggressive,
        }
    }
    
    pub fn as_u8(&self) -> u8 {
        match self {
            OptimizationLevel::None => 0,
            OptimizationLevel::Basic => 1,
            OptimizationLevel::Standard => 2,
            OptimizationLevel::Aggressive => 3,
        }
    }
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::Standard
    }
}

/// Optimization pipeline
pub struct OptimizationPipeline {
    level: OptimizationLevel,
}

impl OptimizationPipeline {
    pub fn new(level: OptimizationLevel) -> Self {
        Self { level }
    }
    
    /// Run optimization pipeline on MIR
    pub fn optimize(&self, mut mir: Vec<MIR>) -> Vec<MIR> {
        match self.level {
            OptimizationLevel::None => mir,
            
            OptimizationLevel::Basic => {
                mir = constant_fold::fold(mir);
                mir = dce::eliminate(mir);
                mir
            }
            
            OptimizationLevel::Standard => {
                mir = constant_fold::fold(mir);
                mir = dce::eliminate(mir);
                mir = cse::eliminate(mir);
                mir = peephole::optimize(mir);
                mir
            }
            
            OptimizationLevel::Aggressive => {
                // Multiple passes for aggressive optimization
                for _ in 0..3 {
                    mir = constant_fold::fold(mir);
                    mir = dce::eliminate(mir);
                    mir = cse::eliminate(mir);
                    mir = peephole::optimize(mir);
                }
                mir
            }
        }
    }
}

impl Default for OptimizationPipeline {
    fn default() -> Self {
        Self::new(OptimizationLevel::Standard)
    }
}
