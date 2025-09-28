//! Module system for Razen programming language
//! 
//! Implements a Go-style file-based module system with Rust-level visibility controls.
//! Features:
//! - File-based modules (no mod declarations needed)
//! - Path-based imports: use "./utils" → utils.Function()
//! - Alias support: use "./utils" as util → util.Function()
//! - Private by default with pub keyword for exports
//! - Circular dependency detection

pub mod resolver;
pub mod visibility;
pub mod error;

pub use resolver::ModuleResolver;
pub use visibility::VisibilityChecker;
pub use error::ModuleError;
