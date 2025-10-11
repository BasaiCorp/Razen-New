// src/backend/execution/raze/memory/executable.rs
//! Executable memory allocation and management
//! 
//! Provides safe wrappers around platform-specific executable memory APIs.
//! Uses mmap on Unix-like systems for RWX memory regions.

use std::ptr;
use crate::backend::execution::raze::RAZEError;

#[cfg(unix)]
use libc::{mmap, mprotect, munmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};

/// Manages executable memory allocations
pub struct ExecutableMemory {
    allocations: Vec<MemoryBlock>,
}

struct MemoryBlock {
    ptr: *mut u8,
    size: usize,
}

impl ExecutableMemory {
    pub fn new() -> Self {
        Self {
            allocations: Vec::new(),
        }
    }
    
    /// Allocate executable memory and copy code into it
    pub fn allocate(&mut self, code: Vec<u8>) -> Result<ExecutableCode, RAZEError> {
        let size = code.len();
        
        if size == 0 {
            return Err(RAZEError::MemoryError("Cannot allocate zero-sized memory".to_string()));
        }
        
        #[cfg(unix)]
        {
            self.allocate_unix(code, size)
        }
        
        #[cfg(windows)]
        {
            self.allocate_windows(code, size)
        }
        
        #[cfg(not(any(unix, windows)))]
        {
            Err(RAZEError::MemoryError("Unsupported platform for executable memory".to_string()))
        }
    }
    
    #[cfg(unix)]
    fn allocate_unix(&mut self, code: Vec<u8>, size: usize) -> Result<ExecutableCode, RAZEError> {
        // Allocate memory with RW permissions
        let ptr = unsafe {
            mmap(
                ptr::null_mut(),
                size,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            )
        };
        
        if ptr == libc::MAP_FAILED {
            return Err(RAZEError::MemoryError("Failed to allocate executable memory".to_string()));
        }
        
        // Copy code to allocated memory
        unsafe {
            ptr::copy_nonoverlapping(code.as_ptr(), ptr as *mut u8, size);
        }
        
        // Change permissions to RX (read + execute, no write for security)
        let result = unsafe {
            mprotect(ptr, size, PROT_READ | PROT_EXEC)
        };
        
        if result != 0 {
            // Cleanup on failure
            unsafe {
                munmap(ptr, size);
            }
            return Err(RAZEError::MemoryError("Failed to set executable permissions".to_string()));
        }
        
        let block = MemoryBlock {
            ptr: ptr as *mut u8,
            size,
        };
        
        self.allocations.push(block);
        
        Ok(ExecutableCode {
            ptr: ptr as *const u8,
            size,
            entry_point: ptr as *const u8,
        })
    }
    
    #[cfg(windows)]
    fn allocate_windows(&mut self, code: Vec<u8>, size: usize) -> Result<ExecutableCode, RAZEError> {
        use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
        use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_READWRITE};
        
        // Allocate memory with RW permissions
        let ptr = unsafe {
            VirtualAlloc(
                ptr::null_mut(),
                size,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            )
        };
        
        if ptr.is_null() {
            return Err(RAZEError::MemoryError("Failed to allocate executable memory".to_string()));
        }
        
        // Copy code to allocated memory
        unsafe {
            ptr::copy_nonoverlapping(code.as_ptr(), ptr as *mut u8, size);
        }
        
        // Change permissions to RX
        let mut old_protect = 0;
        let result = unsafe {
            VirtualProtect(ptr, size, PAGE_EXECUTE_READ, &mut old_protect)
        };
        
        if result == 0 {
            return Err(RAZEError::MemoryError("Failed to set executable permissions".to_string()));
        }
        
        let block = MemoryBlock {
            ptr: ptr as *mut u8,
            size,
        };
        
        self.allocations.push(block);
        
        Ok(ExecutableCode {
            ptr: ptr as *const u8,
            size,
            entry_point: ptr as *const u8,
        })
    }
    
    /// Get total allocated memory size
    pub fn total_allocated(&self) -> usize {
        self.allocations.iter().map(|b| b.size).sum()
    }
    
    /// Get number of allocations
    pub fn allocation_count(&self) -> usize {
        self.allocations.len()
    }
}

impl Drop for ExecutableMemory {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            for block in &self.allocations {
                unsafe {
                    munmap(block.ptr as *mut libc::c_void, block.size);
                }
            }
        }
        
        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualFree;
            use winapi::um::winnt::MEM_RELEASE;
            
            for block in &self.allocations {
                unsafe {
                    VirtualFree(block.ptr as *mut winapi::ctypes::c_void, 0, MEM_RELEASE);
                }
            }
        }
    }
}

impl Default for ExecutableMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents executable code in memory
pub struct ExecutableCode {
    ptr: *const u8,
    size: usize,
    entry_point: *const u8,
}

impl ExecutableCode {
    /// Get pointer to executable code
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }
    
    /// Get size of executable code
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Execute the code with no arguments, returning i64
    /// 
    /// # Safety
    /// This is unsafe because it executes arbitrary machine code.
    /// The caller must ensure the code is valid and safe to execute.
    pub unsafe fn execute_i64(&self) -> i64 {
        let func: extern "C" fn() -> i64 = unsafe { std::mem::transmute(self.entry_point) };
        func()
    }
    
    /// Execute the code with no arguments, returning f64
    pub unsafe fn execute_f64(&self) -> f64 {
        let func: extern "C" fn() -> f64 = unsafe { std::mem::transmute(self.entry_point) };
        func()
    }
    
    /// Execute the code with no arguments, returning void
    pub unsafe fn execute_void(&self) {
        let func: extern "C" fn() = unsafe { std::mem::transmute(self.entry_point) };
        func()
    }
    
    /// Execute the code with one i64 argument, returning i64
    pub unsafe fn execute_i64_i64(&self, arg: i64) -> i64 {
        let func: extern "C" fn(i64) -> i64 = unsafe { std::mem::transmute(self.entry_point) };
        func(arg)
    }
    
    /// Execute the code with two i64 arguments, returning i64
    pub unsafe fn execute_i64_i64_i64(&self, arg1: i64, arg2: i64) -> i64 {
        let func: extern "C" fn(i64, i64) -> i64 = unsafe { std::mem::transmute(self.entry_point) };
        func(arg1, arg2)
    }
    
    /// Disassemble the code (for debugging)
    #[cfg(feature = "disassemble")]
    pub fn disassemble(&self) -> String {
        // This would require a disassembler library
        // For now, just return hex dump
        self.hex_dump()
    }
    
    /// Get hex dump of the code
    pub fn hex_dump(&self) -> String {
        let mut result = String::new();
        let bytes = unsafe { std::slice::from_raw_parts(self.ptr, self.size) };
        
        for (i, chunk) in bytes.chunks(16).enumerate() {
            result.push_str(&format!("{:08x}: ", i * 16));
            
            for byte in chunk {
                result.push_str(&format!("{:02x} ", byte));
            }
            
            result.push('\n');
        }
        
        result
    }
}

// ExecutableCode is Send + Sync because it's just a pointer to executable memory
unsafe impl Send for ExecutableCode {}
unsafe impl Sync for ExecutableCode {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_executable_memory_x86_64() {
        let mut mem = ExecutableMemory::new();
        
        // Simple x86_64 code: mov rax, 42; ret
        let code = vec![
            0x48, 0xc7, 0xc0, 0x2a, 0x00, 0x00, 0x00,  // mov rax, 42
            0xc3,                                        // ret
        ];
        
        let executable = mem.allocate(code).unwrap();
        let result = unsafe { executable.execute_i64() };
        
        assert_eq!(result, 42);
    }
    
    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_executable_memory_aarch64() {
        let mut mem = ExecutableMemory::new();
        
        // Simple ARM64 code: mov x0, #42; ret
        let code = vec![
            0x40, 0x05, 0x80, 0xd2,  // mov x0, #42
            0xc0, 0x03, 0x5f, 0xd6,  // ret
        ];
        
        let executable = mem.allocate(code).unwrap();
        let result = unsafe { executable.execute_i64() };
        
        assert_eq!(result, 42);
    }
}
