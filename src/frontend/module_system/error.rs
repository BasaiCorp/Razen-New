//! Module system error types

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleError {
    /// Module file not found
    ModuleNotFound {
        path: String,
        searched_paths: Vec<String>,
    },
    /// Symbol not exported from module
    SymbolNotExported {
        symbol: String,
        module: String,
    },
    /// Circular dependency detected
    CircularDependency {
        cycle: Vec<String>,
    },
    /// Invalid module path
    InvalidPath {
        path: String,
        reason: String,
    },
    /// IO error when reading module file
    IoError {
        path: String,
        error: String,
    },
    /// Parse error in module file
    ParseError {
        path: String,
        error: String,
    },
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleError::ModuleNotFound { path, searched_paths } => {
                write!(f, "Module '{}' not found. Searched paths: {}", path, searched_paths.join(", "))
            }
            ModuleError::SymbolNotExported { symbol, module } => {
                write!(f, "Symbol '{}' is not exported from module '{}'", symbol, module)
            }
            ModuleError::CircularDependency { cycle } => {
                write!(f, "Circular dependency detected: {}", cycle.join(" -> "))
            }
            ModuleError::InvalidPath { path, reason } => {
                write!(f, "Invalid module path '{}': {}", path, reason)
            }
            ModuleError::IoError { path, error } => {
                write!(f, "IO error reading module '{}': {}", path, error)
            }
            ModuleError::ParseError { path, error } => {
                write!(f, "Parse error in module '{}': {}", path, error)
            }
        }
    }
}

impl std::error::Error for ModuleError {}
