// src/backend/execution/raze/codegen/assembler.rs
//! Low-level assembler utilities for encoding machine instructions

/// Assembler buffer for building machine code
pub struct Assembler {
    buffer: Vec<u8>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }
    
    /// Emit a single byte
    pub fn emit_u8(&mut self, byte: u8) {
        self.buffer.push(byte);
    }
    
    /// Emit a 16-bit value (little-endian)
    pub fn emit_u16(&mut self, value: u16) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }
    
    /// Emit a 32-bit value (little-endian)
    pub fn emit_u32(&mut self, value: u32) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }
    
    /// Emit a 64-bit value (little-endian)
    pub fn emit_u64(&mut self, value: u64) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }
    
    /// Emit multiple bytes
    pub fn emit_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }
    
    /// Get current position in buffer
    pub fn position(&self) -> usize {
        self.buffer.len()
    }
    
    /// Reserve space for later patching
    pub fn reserve(&mut self, size: usize) -> usize {
        let pos = self.position();
        self.buffer.resize(self.buffer.len() + size, 0);
        pos
    }
    
    /// Patch bytes at a specific position
    pub fn patch(&mut self, position: usize, bytes: &[u8]) {
        self.buffer[position..position + bytes.len()].copy_from_slice(bytes);
    }
    
    /// Patch a 32-bit value at a specific position
    pub fn patch_u32(&mut self, position: usize, value: u32) {
        self.patch(position, &value.to_le_bytes());
    }
    
    /// Align to a specific boundary
    pub fn align(&mut self, alignment: usize) {
        let pos = self.position();
        let padding = (alignment - (pos % alignment)) % alignment;
        self.buffer.resize(pos + padding, 0x90); // NOP padding
    }
    
    /// Get the assembled code
    pub fn code(self) -> Vec<u8> {
        self.buffer
    }
    
    /// Get a reference to the buffer
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
    
    /// Get a mutable reference to the buffer
    pub fn buffer_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper for encoding ModR/M byte (x86_64)
pub fn modrm(mode: u8, reg: u8, rm: u8) -> u8 {
    ((mode & 0x3) << 6) | ((reg & 0x7) << 3) | (rm & 0x7)
}

/// Helper for encoding SIB byte (x86_64)
pub fn sib(scale: u8, index: u8, base: u8) -> u8 {
    ((scale & 0x3) << 6) | ((index & 0x7) << 3) | (base & 0x7)
}

/// Helper for encoding REX prefix (x86_64)
pub fn rex(w: bool, r: bool, x: bool, b: bool) -> u8 {
    0x40 | ((w as u8) << 3) | ((r as u8) << 2) | ((x as u8) << 1) | (b as u8)
}
