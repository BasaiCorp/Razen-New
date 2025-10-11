// src/backend/execution/raze/memory/mod.rs
//! Memory management for RAZE
//! 
//! Handles allocation of executable memory for JIT-compiled code.
//! Uses platform-specific APIs (mmap on Unix, VirtualAlloc on Windows).

pub mod executable;
pub mod pool;

pub use executable::{ExecutableMemory, ExecutableCode};
pub use pool::MemoryPool;
