// src/backend/optimization/optimizer.rs

use super::{OptimizationPass, OptimizationLevel};
use super::passes::{DeadCodeElimination, ConstantFolding, UnusedVariableElimination};
use crate::backend::ir::IRModule;
use crate::frontend::diagnostics::Diagnostics;

/// Main optimizer that runs optimization passes
pub struct Optimizer {
    level: OptimizationLevel,
    passes: Vec<Box<dyn OptimizationPass>>,
    verbose: bool,
}

impl Optimizer {
    pub fn new(level: OptimizationLevel) -> Self {
        let mut optimizer = Optimizer {
            level,
            passes: Vec::new(),
            verbose: false,
        };
        
        optimizer.configure_passes();
        optimizer
    }
    
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    /// Configure optimization passes based on optimization level
    fn configure_passes(&mut self) {
        match self.level {
            OptimizationLevel::None => {
                // No optimizations
            }
            OptimizationLevel::Basic => {
                self.passes.push(Box::new(DeadCodeElimination::new()));
                self.passes.push(Box::new(UnusedVariableElimination::new()));
            }
            OptimizationLevel::Standard => {
                self.passes.push(Box::new(DeadCodeElimination::new()));
                self.passes.push(Box::new(ConstantFolding::new()));
                self.passes.push(Box::new(UnusedVariableElimination::new()));
            }
            OptimizationLevel::Aggressive => {
                self.passes.push(Box::new(DeadCodeElimination::new()));
                self.passes.push(Box::new(ConstantFolding::new()));
                self.passes.push(Box::new(UnusedVariableElimination::new()));
                // TODO: Add more aggressive passes like inlining, loop unrolling
            }
        }
    }
    
    /// Run all optimization passes on the IR module
    pub fn optimize(&mut self, mut ir_module: IRModule) -> Result<IRModule, Diagnostics> {
        if self.verbose {
            println!("🔧 Starting optimization with level: {:?}", self.level);
            println!("📊 Before optimization: {} functions, {} globals", 
                     ir_module.functions.len(), ir_module.globals.len());
        }
        
        let mut changed = true;
        let mut iteration = 0;
        let max_iterations = 10; // Prevent infinite loops
        
        // Run passes until no more changes or max iterations reached
        while changed && iteration < max_iterations {
            changed = false;
            iteration += 1;
            
            if self.verbose {
                println!("🔄 Optimization iteration {}", iteration);
            }
            
            for pass in &mut self.passes {
                if self.verbose {
                    println!("   Running pass: {}", pass.name());
                }
                
                match pass.run(&mut ir_module) {
                    Ok(pass_changed) => {
                        if pass_changed {
                            changed = true;
                            if self.verbose {
                                println!("   ✅ {} made changes", pass.name());
                            }
                        }
                    }
                    Err(diagnostics) => {
                        if self.verbose {
                            println!("   ❌ {} failed", pass.name());
                        }
                        return Err(diagnostics);
                    }
                }
            }
        }
        
        if self.verbose {
            println!("✅ Optimization completed after {} iterations", iteration);
            println!("📊 After optimization: {} functions, {} globals", 
                     ir_module.functions.len(), ir_module.globals.len());
        }
        
        Ok(ir_module)
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new(OptimizationLevel::default())
    }
}
