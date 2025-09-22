use crate::value::Value;
use std::fs;
use std::path::Path;

/// Read the contents of a file
/// Example: read("data.txt") => "file contents"
pub fn read(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("File.read requires exactly 1 argument: path".to_string());
    }
    
    let path = args[0].as_string()?;
    
    match fs::read_to_string(&path) {
        Ok(content) => Ok(Value::String(content)),
        Err(e) => Err(format!("Failed to read file '{}': {}", path, e)),
    }
}

/// Write content to a file (overwrites existing file)
/// Example: write("data.txt", "new content") => true
pub fn write(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("File.write requires exactly 2 arguments: path, content".to_string());
    }
    
    let path = args[0].as_string()?;
    let content = args[1].as_string()?;
    
    match fs::write(&path, content) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(format!("Failed to write to file '{}': {}", path, e)),
    }
}

/// Append content to a file
/// Example: append("data.txt", "additional content") => true
pub fn append(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("File.append requires exactly 2 arguments: path, content".to_string());
    }
    
    let path = args[0].as_string()?;
    let content = args[1].as_string()?;
    
    // Read existing content first
    let existing = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => String::new(), // If file doesn't exist, start with empty string
    };
    
    // Append new content and write back
    let new_content = existing + &content;
    match fs::write(&path, new_content) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(format!("Failed to append to file '{}': {}", path, e)),
    }
}

/// Check if a file exists
/// Example: exists("data.txt") => true
pub fn exists(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("File.exists requires exactly 1 argument: path".to_string());
    }
    
    let path = args[0].as_string()?;
    
    Ok(Value::Bool(Path::new(&path).exists()))
}

/// Delete a file
/// Example: delete("data.txt") => true
pub fn delete(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("File.delete requires exactly 1 argument: path".to_string());
    }
    
    let path = args[0].as_string()?;
    
    if !Path::new(&path).exists() {
        return Ok(Value::Bool(false)); // File doesn't exist, nothing to delete
    }
    
    match fs::remove_file(&path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(format!("Failed to delete file '{}': {}", path, e)),
    }
}
