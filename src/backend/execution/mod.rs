// src/backend/execution/mod.rs
//! Execution backend for Razen language
//! Based on the proven stack-based execution engine from the old implementation

pub mod ir;
pub mod compiler;
pub mod runtime;
pub mod value;

pub use ir::*;
pub use compiler::*;
pub use runtime::*;
pub use value::*;
