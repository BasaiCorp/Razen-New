// src/backend/mod.rs
//! Clean, professional backend implementation for the Razen language
//! Built from scratch for maximum control and performance

pub mod execution;
pub mod semantic;

// Re-export the clean execution system and semantic analyzer
pub use execution::{Compiler, Runtime, IR};
pub use semantic::SemanticAnalyzer;