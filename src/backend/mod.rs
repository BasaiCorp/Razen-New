// src/backend/mod.rs
//! Clean, professional backend implementation for the Razen language
//! Built from scratch for maximum control and performance

pub mod aot;
pub mod execution;
pub mod semantic;
pub mod types;
pub mod type_checker;

// Re-export the clean execution system and semantic analyzer
pub use aot::AOTCompiler;
pub use execution::{Compiler, Runtime, IR};
pub use semantic::SemanticAnalyzer;
pub use types::Type;
pub use type_checker::TypeChecker;