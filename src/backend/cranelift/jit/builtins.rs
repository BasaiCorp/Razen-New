// src/backend/cranelift/jit/builtins.rs
// Complete Builtin Functions Manager - Handles external function registration with proper C ABI

use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use cranelift_jit::JITBuilder;

/// Manages builtin functions for JIT compilation
/// Following Cranelift best practices for external function calls
pub struct BuiltinManager {
    // Can store builtin function metadata here
}

impl BuiltinManager {
    /// Create a new builtin manager
    pub fn new() -> Self {
        Self {}
    }

    /// Register all builtin functions as external symbols with proper C ABI
    /// This ensures proper execution order during JIT compilation
    pub fn register_symbols(&self, builder: &mut JITBuilder) {
        // Register all I/O builtin functions as external symbols
        builder.symbol("razen_println", razen_println as *const u8);
        builder.symbol("razen_print", razen_print as *const u8);
        builder.symbol("razen_input", razen_input as *const u8);
        builder.symbol("razen_input_prompt", razen_input_prompt as *const u8);
        
        if std::env::var("RAZEN_DEBUG").is_ok() {
            println!("🔧 Registered builtin functions as external symbols:");
            println!("   - razen_println (C ABI)");
            println!("   - razen_print (C ABI)");
            println!("   - razen_input (C ABI)");
            println!("   - razen_input_prompt (C ABI)");
        }
    }
}

// External function implementations with C ABI

/// Print a line to stdout
#[no_mangle]
extern "C" fn razen_println(msg_ptr: *const c_char) {
    if msg_ptr.is_null() {
        println!();
        return;
    }

    unsafe {
        let c_str = CStr::from_ptr(msg_ptr);
        if let Ok(rust_str) = c_str.to_str() {
            println!("{}", rust_str);
        } else {
            println!("[Invalid UTF-8]");
        }
    }
}

/// Print to stdout without newline
#[no_mangle]
extern "C" fn razen_print(msg_ptr: *const c_char) {
    if msg_ptr.is_null() {
        return;
    }

    unsafe {
        let c_str = CStr::from_ptr(msg_ptr);
        if let Ok(rust_str) = c_str.to_str() {
            print!("{}", rust_str);
        } else {
            print!("[Invalid UTF-8]");
        }
    }
}

/// Read input from stdin (no prompt)
#[no_mangle]
extern "C" fn razen_input() -> *const c_char {
    use std::io;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Remove trailing newline
            if input.ends_with('\n') {
                input.pop();
                if input.ends_with('\r') {
                    input.pop();
                }
            }

            // Convert to C string and leak it
            let c_string = CString::new(input).unwrap_or_else(|_| CString::new("").unwrap());
            c_string.into_raw()
        }
        Err(_) => {
            let c_string = CString::new("").unwrap();
            c_string.into_raw()
        }
    }
}

/// Read input from stdin with prompt
#[no_mangle]
extern "C" fn razen_input_prompt(prompt_ptr: *const c_char) -> *const c_char {
    use std::io::{self, Write};

    // Print the prompt
    if !prompt_ptr.is_null() {
        unsafe {
            let c_str = CStr::from_ptr(prompt_ptr);
            if let Ok(rust_str) = c_str.to_str() {
                print!("{}", rust_str);
                io::stdout().flush().unwrap_or(());
            }
        }
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Remove trailing newline
            if input.ends_with('\n') {
                input.pop();
                if input.ends_with('\r') {
                    input.pop();
                }
            }

            // Convert to C string and leak it
            let c_string = CString::new(input).unwrap_or_else(|_| CString::new("").unwrap());
            c_string.into_raw()
        }
        Err(_) => {
            let c_string = CString::new("").unwrap();
            c_string.into_raw()
        }
    }
}
