// src/backend/execution/raze/jit.rs
//! JIT (Just-In-Time) compiler for RAZE
//! 
//! Compiles MIR to native machine code at runtime and executes it immediately.

use super::mir::MIRBuilder;
use super::memory::ExecutableMemory;
use super::codegen::{create_native_codegen, Architecture};
use super::optimization::{OptimizationPipeline, OptimizationLevel};
use super::{RAZEError, RAZEResult};
use crate::backend::execution::ir::IR;
use crate::backend::execution::value::Value;
use std::time::Instant;

// May be needed for multi-function compilation in the future
#[allow(unused_imports)]
use super::mir::MIRModule;

/// JIT compiler
pub struct JITCompiler {
    optimization_level: OptimizationLevel,
    architecture: Architecture,
    memory: ExecutableMemory,
    compile_time: f64,
    execution_count: usize,
}

impl JITCompiler {
    /// Create a new JIT compiler with default optimization
    pub fn new() -> RAZEResult<Self> {
        Ok(Self {
            optimization_level: OptimizationLevel::Standard,
            architecture: Architecture::current(),
            memory: ExecutableMemory::new(),
            compile_time: 0.0,
            execution_count: 0,
        })
    }
    
    /// Create JIT compiler with specific optimization level
    pub fn with_optimization(level: u8) -> RAZEResult<Self> {
        Ok(Self {
            optimization_level: OptimizationLevel::from_u8(level),
            architecture: Architecture::current(),
            memory: ExecutableMemory::new(),
            compile_time: 0.0,
            execution_count: 0,
        })
    }
    
    /// Compile IR to native code and execute
    pub fn compile_and_run(&mut self, ir: &[IR]) -> RAZEResult<Value> {
        let start = Instant::now();
        
        // Step 1: Translate IR to MIR
        let mut mir_builder = MIRBuilder::new();
        let mir_module = mir_builder.translate_ir(ir)
            .map_err(|e| RAZEError::CompilationError(e))?;
        
        // Step 2: Get main function
        let main_func = mir_module.functions.get("main")
            .ok_or_else(|| RAZEError::CompilationError("No main function found".to_string()))?;
        
        // Step 3: Optimize MIR
        let optimizer = OptimizationPipeline::new(self.optimization_level);
        let optimized_mir = optimizer.optimize(main_func.instructions.clone());
        
        // Step 4: Generate native code
        let mut codegen = create_native_codegen();
        let machine_code = codegen.generate(&optimized_mir)?;
        
        // Step 5: Allocate executable memory
        let executable = self.memory.allocate(machine_code)?;
        
        self.compile_time = start.elapsed().as_secs_f64();
        
        // Step 6: Execute
        let result = unsafe {
            let ret = executable.execute_i64();
            Value::Integer(ret)
        };
        
        self.execution_count += 1;
        
        Ok(result)
    }
    
    /// Get compilation statistics
    pub fn stats(&self) -> JITStats {
        JITStats {
            architecture: self.architecture,
            optimization_level: self.optimization_level,
            compile_time: self.compile_time,
            execution_count: self.execution_count,
            memory_allocated: self.memory.total_allocated(),
            allocation_count: self.memory.allocation_count(),
        }
    }
}

impl Default for JITCompiler {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// JIT compilation statistics
#[derive(Debug, Clone)]
pub struct JITStats {
    pub architecture: Architecture,
    pub optimization_level: OptimizationLevel,
    pub compile_time: f64,
    pub execution_count: usize,
    pub memory_allocated: usize,
    pub allocation_count: usize,
}

impl std::fmt::Display for JITStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[INFO] RAZE JIT Statistics:")?;
        writeln!(f, "  Architecture: {}", self.architecture)?;
        writeln!(f, "  Optimization: O{}", self.optimization_level.as_u8())?;
        writeln!(f, "  Compile Time: {:.3}ms", self.compile_time * 1000.0)?;
        writeln!(f, "  Executions: {}", self.execution_count)?;
        writeln!(f, "  Memory: {} KB ({} allocations)", 
                 self.memory_allocated / 1024, self.allocation_count)?;
        Ok(())
    }
}
