// src/backend/execution/mod.rs
//! Unified Execution Backend for Razen Language
//! 
//! This module provides a complete execution system:
//! - Runtime: IR interpreter (proven, reliable, fast)
//! - Adaptive: Razen Adaptive Interpreter Engine (2-3x faster through specialization)
//! - RAZE: Razen Advanced Zero-overhead Engine (JIT/AOT native compilation)
//! 
//! All three share the same IR and runtime foundation!

pub mod ir;
pub mod compiler;
pub mod runtime;
pub mod value;
pub mod adaptive;
pub mod aot;
pub mod raze;

pub use ir::*;
pub use compiler::*;
pub use runtime::*;
pub use value::*;
pub use adaptive::AdaptiveEngine;
pub use aot::AOT;
pub use raze::{JITCompiler, AOTCompiler as RAZEAOTCompiler, RAZERuntime, CompilationMode};
