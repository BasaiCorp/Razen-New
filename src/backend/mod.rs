//! Unified Backend Implementation for Razen Language
//!
//! Architecture:
//! - execution/: Complete execution system (Runtime, Adaptive, AOT)
//! - semantic/: Semantic analysis and type checking
//! - types/: Type system

pub mod execution;
pub mod semantic;
pub mod types;
pub mod type_checker;

// Re-export unified execution system
pub use execution::{Compiler, Runtime, IR, AdaptiveEngine, AOT};
pub use semantic::SemanticAnalyzer;
pub use types::Type;
pub use type_checker::TypeChecker;
// Aliases for backward compatibility with commands
pub use execution::AdaptiveEngine as NativeJIT; // Backward compatibility
pub use execution::AOT as NativeAOT;