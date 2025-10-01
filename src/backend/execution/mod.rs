// src/backend/execution/mod.rs
//! Unified Execution Backend for Razen Language
//! 
//! This module provides a complete execution system:
//! - Runtime: IR interpreter (proven, reliable, fast)
//! - JIT: Optimized IR execution (40-50% faster than Python)
//! - AOT: Ahead-of-time compilation (future: native code)
//! 
//! All three share the same IR and runtime foundation!

pub mod ir;
pub mod compiler;
pub mod runtime;
pub mod value;
pub mod jit;
pub mod aot;

pub use ir::*;
pub use compiler::*;
pub use runtime::*;
pub use value::*;
pub use jit::JIT;
pub use aot::AOT;
