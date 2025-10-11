// src/backend/execution/raze/runtime.rs
//! RAZE runtime support
//! 
//! Provides runtime services for JIT/AOT compiled code.

use super::{JITCompiler, AOTCompiler, CompilationMode};
use super::{RAZEError, RAZEResult};
use crate::backend::execution::ir::IR;
use crate::backend::execution::value::Value;
use std::path::Path;

/// Unified RAZE runtime
pub struct RAZERuntime {
    mode: CompilationMode,
    jit: Option<JITCompiler>,
    aot: Option<AOTCompiler>,
}

impl RAZERuntime {
    /// Create runtime with specified mode
    pub fn new(mode: CompilationMode) -> RAZEResult<Self> {
        let (jit, aot) = match mode {
            CompilationMode::JIT => (Some(JITCompiler::new()?), None),
            CompilationMode::AOT => (None, Some(AOTCompiler::new())),
            CompilationMode::Hybrid | CompilationMode::Adaptive => {
                (Some(JITCompiler::new()?), Some(AOTCompiler::new()))
            }
        };
        
        Ok(Self { mode, jit, aot })
    }
    
    /// Execute IR using the configured mode
    pub fn execute(&mut self, ir: &[IR]) -> RAZEResult<Value> {
        match self.mode {
            CompilationMode::JIT => {
                if let Some(ref mut jit) = self.jit {
                    jit.compile_and_run(ir)
                } else {
                    Err(RAZEError::RuntimeError("JIT not initialized".to_string()))
                }
            }
            _ => {
                // For other modes, fall back to JIT for now
                if let Some(ref mut jit) = self.jit {
                    jit.compile_and_run(ir)
                } else {
                    Err(RAZEError::RuntimeError("Runtime not initialized".to_string()))
                }
            }
        }
    }
    
    /// Compile to standalone executable (AOT mode)
    pub fn compile_to_file(&mut self, ir: &[IR], output_path: &Path) -> RAZEResult<()> {
        if let Some(ref mut aot) = self.aot {
            aot.compile_to_file(ir, output_path)
        } else {
            Err(RAZEError::RuntimeError("AOT not initialized".to_string()))
        }
    }
    
    /// Get runtime statistics
    pub fn stats(&self) -> String {
        if let Some(ref jit) = self.jit {
            format!("{}", jit.stats())
        } else {
            "[INFO] No statistics available".to_string()
        }
    }
}
