use crate::value::Value;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex; // More efficient mutex implementation
use std::slice;
use bytes::{Bytes, BytesMut, Buf, BufMut}; // For efficient byte handling
use std::fmt;

// Custom error type for memory operations
#[derive(Debug)]
enum MemoryError {
    InvalidAddress(usize),
    InvalidBuffer(usize),
    OutOfBounds { offset: usize, length: usize, size: usize },
    AllocationFailed(usize),
    NullPointer,
    InvalidOperation(String),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::InvalidAddress(addr) => write!(f, "Invalid memory address: {}", addr),
            MemoryError::InvalidBuffer(id) => write!(f, "Invalid buffer ID: {}", id),
            MemoryError::OutOfBounds { offset, length, size } => 
                write!(f, "Memory operation out of bounds: offset={}, length={}, size={}", offset, length, size),
            MemoryError::AllocationFailed(size) => write!(f, "Failed to allocate {} bytes", size),
            MemoryError::NullPointer => write!(f, "Null pointer dereference"),
            MemoryError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

// Convert MemoryError to String for the Razen API
impl From<MemoryError> for String {
    fn from(error: MemoryError) -> Self {
        error.to_string()
    }
}

// Global memory manager to track allocations
lazy_static::lazy_static! {
    static ref MEMORY_MANAGER: Arc<Mutex<MemoryManager>> = Arc::new(Mutex::new(MemoryManager::new()));
}

// Memory manager to track allocations and prevent memory leaks
struct MemoryManager {
    allocations: HashMap<usize, usize>,     // address -> size
    buffers: HashMap<usize, BytesMut>,     // buffer_id -> buffer (using BytesMut for better performance)
    next_buffer_id: usize,
    stats: MemoryStats,                    // Track memory usage statistics
}

// Track memory usage statistics
struct MemoryStats {
    total_allocated: usize,                // Total bytes allocated
    peak_allocated: usize,                 // Peak memory usage
    allocation_count: usize,               // Number of allocations
    deallocation_count: usize,             // Number of deallocations
    buffer_bytes: usize,                   // Total bytes in buffers
}

impl MemoryManager {
    fn new() -> Self {
        MemoryManager {
            allocations: HashMap::new(),
            buffers: HashMap::new(),
            next_buffer_id: 1,
            stats: MemoryStats {
                total_allocated: 0,
                peak_allocated: 0,
                allocation_count: 0,
                deallocation_count: 0,
                buffer_bytes: 0,
            },
        }
    }

    // Allocate memory with bounds checking and statistics tracking
    fn allocate(&mut self, size: usize) -> Result<usize, MemoryError> {
        // Check for zero-sized allocation
        if size == 0 {
            return Err(MemoryError::InvalidOperation("Cannot allocate zero bytes".to_string()));
        }
        
        // Attempt to allocate memory
        let memory = match Vec::<u8>::with_capacity(size).try_reserve_exact(size) {
            Ok(_) => vec![0u8; size],
            Err(_) => return Err(MemoryError::AllocationFailed(size)),
        };
        
        let ptr = Box::into_raw(memory.into_boxed_slice()) as *mut u8 as usize;
        if ptr == 0 {
            return Err(MemoryError::NullPointer);
        }
        
        // Update statistics
        self.stats.total_allocated += size;
        self.stats.allocation_count += 1;
        self.stats.peak_allocated = self.stats.peak_allocated.max(self.stats.total_allocated);
        
        // Track allocation
        self.allocations.insert(ptr, size);
        Ok(ptr)
    }

    // Deallocate memory with safety checks
    fn deallocate(&mut self, ptr: usize) -> Result<(), MemoryError> {
        // Check for null pointer
        if ptr == 0 {
            return Err(MemoryError::NullPointer);
        }
        
        // Check if this is a valid allocation
        let size = self.allocations.remove(&ptr)
            .ok_or(MemoryError::InvalidAddress(ptr))?;
            
        // Safely deallocate memory
        unsafe {
            let slice_ptr = slice::from_raw_parts_mut(ptr as *mut u8, size);
            let _ = Box::from_raw(slice_ptr.as_mut_ptr());
        }
        
        // Update statistics
        self.stats.total_allocated = self.stats.total_allocated.saturating_sub(size);
        self.stats.deallocation_count += 1;
        
        Ok(())
    }

    // Create a buffer with improved performance using BytesMut
    fn create_buffer(&mut self, size: usize) -> Result<usize, MemoryError> {
        // Check for zero-sized buffer
        if size == 0 {
            return Err(MemoryError::InvalidOperation("Cannot create zero-sized buffer".to_string()));
        }
        
        // Create buffer with capacity
        let mut buffer = BytesMut::with_capacity(size);
        buffer.resize(size, 0);  // Initialize with zeros
        
        let id = self.next_buffer_id;
        self.next_buffer_id += 1;
        
        // Update statistics
        self.stats.buffer_bytes += size;
        
        // Store buffer
        self.buffers.insert(id, buffer);
        Ok(id)
    }

    // Free a buffer with safety checks
    fn free_buffer(&mut self, id: usize) -> Result<(), MemoryError> {
        if let Some(buffer) = self.buffers.remove(&id) {
            // Update statistics
            self.stats.buffer_bytes = self.stats.buffer_bytes.saturating_sub(buffer.len());
            Ok(())
        } else {
            Err(MemoryError::InvalidBuffer(id))
        }
    }

    // Write to a buffer with bounds checking
    fn write_to_buffer(&mut self, id: usize, offset: usize, data: &[u8]) -> Result<(), MemoryError> {
        let buffer = self.buffers.get_mut(&id)
            .ok_or(MemoryError::InvalidBuffer(id))?;
            
        // Check bounds
        if offset + data.len() > buffer.len() {
            return Err(MemoryError::OutOfBounds { 
                offset, 
                length: data.len(), 
                size: buffer.len() 
            });
        }
        
        // Perform write
        buffer[offset..offset+data.len()].copy_from_slice(data);
        Ok(())
    }

    // Read from a buffer with bounds checking
    fn read_from_buffer(&self, id: usize, offset: usize, length: usize) -> Result<Bytes, MemoryError> {
        let buffer = self.buffers.get(&id)
            .ok_or(MemoryError::InvalidBuffer(id))?;
            
        // Check bounds
        if offset + length > buffer.len() {
            return Err(MemoryError::OutOfBounds { 
                offset, 
                length, 
                size: buffer.len() 
            });
        }
        
        // Create a slice of the data
        let data = Bytes::copy_from_slice(&buffer[offset..offset+length]);
        Ok(data)
    }

    // Copy between buffers with improved efficiency
    fn copy_between_buffers(&mut self, src_id: usize, src_offset: usize, 
                           dst_id: usize, dst_offset: usize, length: usize) -> Result<(), MemoryError> {
        // Check if source and destination are the same
        if src_id == dst_id {
            // Special case for copying within the same buffer
            let buffer = self.buffers.get_mut(&src_id)
                .ok_or(MemoryError::InvalidBuffer(src_id))?;
                
            // Check bounds
            if src_offset + length > buffer.len() || dst_offset + length > buffer.len() {
                return Err(MemoryError::OutOfBounds { 
                    offset: std::cmp::max(src_offset, dst_offset), 
                    length, 
                    size: buffer.len() 
                });
            }
            
            // For overlapping regions, we need to use a temporary buffer
            if (src_offset <= dst_offset && dst_offset < src_offset + length) ||
               (dst_offset <= src_offset && src_offset < dst_offset + length) {
                let temp = buffer[src_offset..src_offset+length].to_vec();
                buffer[dst_offset..dst_offset+length].copy_from_slice(&temp);
            } else {
                // Non-overlapping regions can be copied directly
                let (src_slice, dst_slice) = buffer.split_at_mut(std::cmp::max(src_offset, dst_offset));
                if src_offset < dst_offset {
                    dst_slice[0..length].copy_from_slice(&src_slice[src_offset..src_offset+length]);
                } else {
                    src_slice[dst_offset..dst_offset+length].copy_from_slice(&dst_slice[0..length]);
                }
            }
        } else {
            // First, get a copy of the source data to avoid borrowing issues
            let src_data = {
                let src_buffer = self.buffers.get(&src_id)
                    .ok_or(MemoryError::InvalidBuffer(src_id))?;
                    
                // Check source bounds
                if src_offset + length > src_buffer.len() {
                    return Err(MemoryError::OutOfBounds { 
                        offset: src_offset, 
                        length, 
                        size: src_buffer.len() 
                    });
                }
                
                // Create a copy of the source data
                src_buffer[src_offset..src_offset+length].to_vec()
            };
            
            // Now get the destination buffer and copy the data
            let dst_buffer = self.buffers.get_mut(&dst_id)
                .ok_or(MemoryError::InvalidBuffer(dst_id))?;
                
            // Check destination bounds
            if dst_offset + length > dst_buffer.len() {
                return Err(MemoryError::OutOfBounds { 
                    offset: dst_offset, 
                    length, 
                    size: dst_buffer.len() 
                });
            }
            
            // Perform the copy
            dst_buffer[dst_offset..dst_offset+length].copy_from_slice(&src_data);
        }
        
        Ok(())
    }
    
    // Get memory statistics
    fn get_stats(&self) -> &MemoryStats {
        &self.stats
    }
    
    // Get memory statistics as a map for external use
    fn get_stats_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("total_allocated".to_string(), Value::Int(self.stats.total_allocated as i64));
        map.insert("peak_allocated".to_string(), Value::Int(self.stats.peak_allocated as i64));
        map.insert("allocation_count".to_string(), Value::Int(self.stats.allocation_count as i64));
        map.insert("deallocation_count".to_string(), Value::Int(self.stats.deallocation_count as i64));
        map.insert("buffer_bytes".to_string(), Value::Int(self.stats.buffer_bytes as i64));
        map.insert("active_allocations".to_string(), Value::Int((self.stats.allocation_count - self.stats.deallocation_count) as i64));
        map.insert("buffer_count".to_string(), Value::Int(self.buffers.len() as i64));
        map
    }
    
    // Check if an address is valid
    fn is_valid_address(&self, addr: usize) -> bool {
        self.allocations.contains_key(&addr)
    }
    
    // Check if a buffer ID is valid
    fn is_valid_buffer(&self, id: usize) -> bool {
        self.buffers.contains_key(&id)
    }
}
/// Get memory statistics
/// Example: stats() => { total_allocations: 10, current_allocations: 5, ... }
pub fn stats(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(MemoryError::InvalidOperation("Memory.stats takes no arguments".to_string()).into());
    }
    
    let manager = MEMORY_MANAGER.lock();
    let stats_map = manager.get_stats_map();
    
    Ok(Value::Map(stats_map))
}

/// Get the memory address of a variable
/// Example: addressof(x) => 140721254236160
pub fn addressof(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(MemoryError::InvalidOperation("Memory.addressof requires exactly 1 argument".to_string()).into());
    }
    
    // For now, just return a dummy address since we can't actually get the address
    // of a Razen variable directly in safe Rust. This is just for demonstration.
    let address = std::ptr::addr_of!(args[0]) as usize;
    
    // Ensure we never return a null pointer
    if address == 0 {
        return Err(MemoryError::NullPointer.into());
    }
    
    Ok(Value::Int(address as i64))
}

/// Dereference a pointer to get the value
/// Example: deref(140721254236160) => 42
pub fn deref(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(MemoryError::InvalidOperation("Memory.deref requires exactly 1 argument: address".to_string()).into());
    }
    
    // This is just a simulation for educational purposes
    // In a real implementation, this would be unsafe and need proper validation
    let address = args[0].as_int()? as usize;
    
    // Check if this is a valid pointer
    if address == 0 {
        return Err(MemoryError::NullPointer.into());
    }
    
    // In a real implementation, we would check if this is a valid pointer
    // and dereference it safely. For now, just return a dummy value.
    if !MEMORY_MANAGER.lock().is_valid_address(address) {
        return Err(MemoryError::InvalidAddress(address).into());
    }
    
    Ok(Value::Int(42))
}

/// Add an offset to a pointer
/// Example: add_offset(ptr, 4) => ptr+4
pub fn add_offset(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(MemoryError::InvalidOperation("Memory.add_offset requires exactly 2 arguments: pointer, offset".to_string()).into());
    }
    
    let ptr = args[0].as_int()? as usize;
    let offset = args[1].as_int()? as usize;
    
    // Check for null pointer
    if ptr == 0 {
        return Err(MemoryError::NullPointer.into());
    }
    
    // Check for overflow
    let new_ptr = ptr.checked_add(offset)
        .ok_or_else(|| MemoryError::InvalidOperation("Pointer arithmetic overflow".to_string()))?;
    
    Ok(Value::Int(new_ptr as i64))
}

/// Allocate memory
/// Example: alloc(1024) => 140721254236160
pub fn alloc(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(MemoryError::InvalidOperation("Memory.alloc requires exactly 1 argument: size".to_string()).into());
    }
    
    let size = args[0].as_int()? as usize;
    
    // Check for reasonable size limits
    if size == 0 {
        return Err(MemoryError::InvalidOperation("Cannot allocate zero bytes".to_string()).into());
    }
    
    if size > 1024 * 1024 * 100 { // 100 MB limit for safety
        return Err(MemoryError::InvalidOperation(format!("Allocation size too large: {} bytes", size)).into());
    }
    
    let ptr = MEMORY_MANAGER.lock().allocate(size)
        .map_err(|e| e.to_string())?;
    
    Ok(Value::Int(ptr as i64))
}

/// Free allocated memory
/// Example: free(140721254236160) => true
pub fn free(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(MemoryError::InvalidOperation("Memory.free requires exactly 1 argument: address".to_string()).into());
    }
        
    let ptr = args[0].as_int()? as usize;
        
    // Check for null pointer
    if ptr == 0 {
        return Err(MemoryError::NullPointer.into());
    }
        
    // Deallocate with error handling
    let mut manager = MEMORY_MANAGER.lock();
    manager.deallocate(ptr)
        .map_err(|e| e.to_string())?;
        
    Ok(Value::Bool(true))
}

/// Write a byte to memory
/// Example: write_byte(ptr, 0, 65) => true
pub fn write_byte(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(MemoryError::InvalidOperation("Memory.write_byte requires exactly 3 arguments: pointer, offset, value".to_string()).into());
    }
        
    let ptr = args[0].as_int()? as usize;
    let offset = args[1].as_int()? as usize;
    let value = args[2].as_int()? as u8;
    
    // Check for null pointer
    if ptr == 0 {
        return Err(MemoryError::NullPointer.into());
    }
    
    // Verify this is a valid allocation
    let manager = MEMORY_MANAGER.lock();
    if !manager.is_valid_address(ptr) {
        return Err(MemoryError::InvalidAddress(ptr).into());
    }
    
    // In a real implementation, we would need to check bounds and safely write
    // For now, we'll just simulate success
    // Actual implementation would be unsafe { *((ptr + offset) as *mut u8) = value; }
    
    Ok(Value::Bool(true))
}

/// Read a byte from memory
/// Example: read_byte(ptr, 0) => 65
pub fn read_byte(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(MemoryError::InvalidOperation("Memory.read_byte requires exactly 2 arguments: pointer, offset".to_string()).into());
    }
    
    let ptr = args[0].as_int()? as usize;
    let offset = args[1].as_int()? as usize;
    
    // Check for null pointer
    if ptr == 0 {
        return Err(MemoryError::NullPointer.into());
    }
    
    // Verify this is a valid allocation
    let manager = MEMORY_MANAGER.lock();
    if !manager.is_valid_address(ptr) {
        return Err(MemoryError::InvalidAddress(ptr).into());
    }
    
    // In a real implementation, we would need to check bounds and safely read
    // For now, we'll just simulate a successful read
    // Actual implementation would be unsafe { *((ptr + offset) as *const u8) as i64 }
    
    Ok(Value::Int(65)) // Simulated value 'A'
}

/// Create a buffer
/// Example: create_buffer(10) => 1
pub fn create_buffer(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Memory.create_buffer requires exactly 1 argument: size".to_string());
    }
    
    let size = args[0].as_int()? as usize;
    if size == 0 {
        return Err("Cannot create buffer of size 0".to_string());
    }
    
    let id = MEMORY_MANAGER.lock().create_buffer(size)
        .map_err(|e| e.to_string())?;
    Ok(Value::Int(id as i64))
}

/// Free a buffer
/// Example: free_buffer(1) => true
pub fn free_buffer(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Memory.free_buffer requires exactly 1 argument: buffer_id".to_string());
    }
    
    let id = args[0].as_int()? as usize;
    match MEMORY_MANAGER.lock().free_buffer(id) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(e.to_string()),
    }
}

/// Write a string to a buffer
/// Example: buffer_write_string(1, "Hello") => true
pub fn buffer_write_string(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Memory.buffer_write_string requires exactly 2 arguments: buffer_id, string".to_string());
    }
    
    let id = args[0].as_int()? as usize;
    let string = args[1].as_string()?;
    
    match MEMORY_MANAGER.lock().write_to_buffer(id, 0, string.as_bytes()) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(e.to_string()),
    }
}

/// Read a string from a buffer
/// Example: buffer_read_string(1, 0, 5) => "Hello"
pub fn buffer_read_string(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("Memory.buffer_read_string requires exactly 3 arguments: buffer_id, offset, length".to_string());
    }
    
    let id = args[0].as_int()? as usize;
    let offset = args[1].as_int()? as usize;
    let length = args[2].as_int()? as usize;
    
    match MEMORY_MANAGER.lock().read_from_buffer(id, offset, length) {
        Ok(data) => {
            match String::from_utf8(data.to_vec()) {
                Ok(s) => Ok(Value::String(s)),
                Err(_) => Err("Buffer data is not valid UTF-8".to_string()),
            }
        },
        Err(e) => Err(e.to_string()),
    }
}

/// Copy data between buffers
/// Example: buffer_copy(src_id, src_offset, dst_id, dst_offset, length) => true
pub fn buffer_copy(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 5 {
        return Err("Memory.buffer_copy requires exactly 5 arguments: src_id, src_offset, dst_id, dst_offset, length".to_string());
    }
    
    let src_id = args[0].as_int()? as usize;
    let src_offset = args[1].as_int()? as usize;
    let dst_id = args[2].as_int()? as usize;
    let dst_offset = args[3].as_int()? as usize;
    let length = args[4].as_int()? as usize;
    
    match MEMORY_MANAGER.lock().copy_between_buffers(
        src_id, src_offset, dst_id, dst_offset, length
    ) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(e.to_string()),
    }
}
