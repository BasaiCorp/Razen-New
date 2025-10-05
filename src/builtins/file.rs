// src/builtins/file.rs
//! File I/O operations - Clean function names

use crate::backend::execution::value::Value;
use std::fs;
use std::path::Path;

/// Read entire file as string
pub fn read(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(path)) = stack.pop() {
        match fs::read_to_string(&path) {
            Ok(content) => {
                stack.push(Value::String(content));
                Ok(())
            }
            Err(e) => Err(format!("[ERROR] Failed to read file '{}': {}", path, e))
        }
    } else {
        Err("[ERROR] read() requires a string path".to_string())
    }
}

/// Write content to file
pub fn write(stack: &mut Vec<Value>) -> Result<(), String> {
    let content = stack.pop().ok_or("[ERROR] write() requires content")?;
    let path = stack.pop().ok_or("[ERROR] write() requires path")?;
    
    if let (Value::String(p), Value::String(c)) = (path, content) {
        match fs::write(&p, &c) {
            Ok(_) => {
                stack.push(Value::Boolean(true));
                Ok(())
            }
            Err(e) => Err(format!("[ERROR] Failed to write file '{}': {}", p, e))
        }
    } else {
        Err("[ERROR] write() requires path and content as strings".to_string())
    }
}

/// Append content to file
pub fn append(stack: &mut Vec<Value>) -> Result<(), String> {
    let content = stack.pop().ok_or("[ERROR] append() requires content")?;
    let path = stack.pop().ok_or("[ERROR] append() requires path")?;
    
    if let (Value::String(p), Value::String(c)) = (path, content) {
        use std::fs::OpenOptions;
        use std::io::Write;
        
        match OpenOptions::new().create(true).append(true).open(&p) {
            Ok(mut file) => {
                match file.write_all(c.as_bytes()) {
                    Ok(_) => {
                        stack.push(Value::Boolean(true));
                        Ok(())
                    }
                    Err(e) => Err(format!("[ERROR] Failed to append to '{}': {}", p, e))
                }
            }
            Err(e) => Err(format!("[ERROR] Failed to open file '{}': {}", p, e))
        }
    } else {
        Err("[ERROR] append() requires path and content as strings".to_string())
    }
}

/// Delete file
pub fn delete(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(path)) = stack.pop() {
        match fs::remove_file(&path) {
            Ok(_) => {
                stack.push(Value::Boolean(true));
                Ok(())
            }
            Err(e) => Err(format!("[ERROR] Failed to delete file '{}': {}", path, e))
        }
    } else {
        Err("[ERROR] delete() requires a string path".to_string())
    }
}

/// Check if path exists
pub fn exists(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(path)) = stack.pop() {
        stack.push(Value::Boolean(Path::new(&path).exists()));
        Ok(())
    } else {
        Err("[ERROR] exists() requires a string path".to_string())
    }
}

/// Check if path is a file
pub fn is_file(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(path)) = stack.pop() {
        stack.push(Value::Boolean(Path::new(&path).is_file()));
        Ok(())
    } else {
        Err("[ERROR] is_file() requires a string path".to_string())
    }
}

/// Check if path is a directory
pub fn is_dir(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(path)) = stack.pop() {
        stack.push(Value::Boolean(Path::new(&path).is_dir()));
        Ok(())
    } else {
        Err("[ERROR] is_dir() requires a string path".to_string())
    }
}

/// List directory contents
pub fn list_dir(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(path)) = stack.pop() {
        match fs::read_dir(&path) {
            Ok(entries) => {
                let mut files = Vec::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(name) = entry.file_name().to_str() {
                            files.push(Value::String(name.to_string()));
                        }
                    }
                }
                stack.push(Value::Array(files));
                Ok(())
            }
            Err(e) => Err(format!("[ERROR] Failed to list directory '{}': {}", path, e))
        }
    } else {
        Err("[ERROR] list_dir() requires a string path".to_string())
    }
}

/// Create directory
pub fn create_dir(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(path)) = stack.pop() {
        match fs::create_dir_all(&path) {
            Ok(_) => {
                stack.push(Value::Boolean(true));
                Ok(())
            }
            Err(e) => Err(format!("[ERROR] Failed to create directory '{}': {}", path, e))
        }
    } else {
        Err("[ERROR] create_dir() requires a string path".to_string())
    }
}

/// Delete directory
pub fn delete_dir(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(path)) = stack.pop() {
        match fs::remove_dir_all(&path) {
            Ok(_) => {
                stack.push(Value::Boolean(true));
                Ok(())
            }
            Err(e) => Err(format!("[ERROR] Failed to delete directory '{}': {}", path, e))
        }
    } else {
        Err("[ERROR] delete_dir() requires a string path".to_string())
    }
}
