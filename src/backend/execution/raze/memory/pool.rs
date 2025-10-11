// src/backend/execution/raze/memory/pool.rs
//! Memory pool for efficient allocation of small code blocks
//! 
//! Reduces overhead by allocating large blocks and subdividing them.

use super::executable::{ExecutableMemory, ExecutableCode};
use crate::backend::execution::raze::RAZEError;

const POOL_SIZE: usize = 64 * 1024; // 64KB per pool
const MIN_ALLOCATION: usize = 256;   // Minimum allocation size

/// Memory pool for efficient code allocation
pub struct MemoryPool {
    pools: Vec<Pool>,
    current_pool: usize,
}

struct Pool {
    memory: ExecutableMemory,
    used: usize,
    capacity: usize,
}

impl MemoryPool {
    pub fn new() -> Self {
        Self {
            pools: Vec::new(),
            current_pool: 0,
        }
    }
    
    /// Allocate code from the pool
    pub fn allocate(&mut self, code: Vec<u8>) -> Result<ExecutableCode, RAZEError> {
        let size = code.len();
        
        // For large allocations, use direct allocation
        if size > POOL_SIZE / 2 {
            let mut mem = ExecutableMemory::new();
            return mem.allocate(code);
        }
        
        // Try to allocate from current pool
        if let Some(pool) = self.pools.get_mut(self.current_pool) {
            if pool.used + size <= pool.capacity {
                // Allocate from current pool
                return pool.memory.allocate(code);
            }
        }
        
        // Need a new pool
        self.allocate_new_pool()?;
        
        // Allocate from new pool
        if let Some(pool) = self.pools.get_mut(self.current_pool) {
            pool.memory.allocate(code)
        } else {
            Err(RAZEError::MemoryError("Failed to allocate from pool".to_string()))
        }
    }
    
    fn allocate_new_pool(&mut self) -> Result<(), RAZEError> {
        let pool = Pool {
            memory: ExecutableMemory::new(),
            used: 0,
            capacity: POOL_SIZE,
        };
        
        self.pools.push(pool);
        self.current_pool = self.pools.len() - 1;
        
        Ok(())
    }
    
    /// Get total memory usage statistics
    pub fn stats(&self) -> PoolStats {
        let total_capacity: usize = self.pools.iter().map(|p| p.capacity).sum();
        let total_used: usize = self.pools.iter().map(|p| p.used).sum();
        
        PoolStats {
            pool_count: self.pools.len(),
            total_capacity,
            total_used,
            fragmentation: if total_capacity > 0 {
                (total_capacity - total_used) as f64 / total_capacity as f64
            } else {
                0.0
            },
        }
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub pool_count: usize,
    pub total_capacity: usize,
    pub total_used: usize,
    pub fragmentation: f64,
}

impl std::fmt::Display for PoolStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pools: {}, Capacity: {} KB, Used: {} KB, Fragmentation: {:.1}%",
            self.pool_count,
            self.total_capacity / 1024,
            self.total_used / 1024,
            self.fragmentation * 100.0
        )
    }
}
