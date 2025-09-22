// src/backend/mod.rs
//! Clean, professional backend implementation for the Razen language
//! Built from scratch for maximum control and performance

pub mod execution;

// Re-export the clean execution system
pub use execution::{Compiler, Runtime, IR};