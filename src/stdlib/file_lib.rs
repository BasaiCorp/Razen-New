// src/stdlib/file_lib.rs
//! File I/O standard library - Native Rust implementation

use crate::backend::execution::value::Value;
use std::fs;
use std::path::Path;

pub fn has_function(name: &str) -> bool {
    matches!(name, "read" | "write" | "append" | "exists" | "delete" | "read_lines" | "write_lines")
}

pub fn get_function_list() -> Vec<&'static str> {
    vec!["read", "write", "append", "exists", "delete", "read_lines", "write_lines"]
}

pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "read" => read(args),
        "write" => write(args),
        "append" => append(args),
        "exists" => exists(args),
        "delete" => delete(args),
        "read_lines" => read_lines(args),
        "write_lines" => write_lines(args),
        _ => Err(format!("Unknown file function: {}", name)),
    }
}

fn read(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("read() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::String(path) => {
            fs::read_to_string(path)
                .map(Value::String)
                .map_err(|e| format!("Failed to read file: {}", e))
        }
        _ => Err("read() requires a string path".to_string()),
    }
}

fn write(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("write() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(content)) => {
            fs::write(path, content)
                .map(|_| Value::Boolean(true))
                .map_err(|e| format!("Failed to write file: {}", e))
        }
        _ => Err("write() requires path and content as strings".to_string()),
    }
}

fn append(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("append() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(content)) => {
            use std::fs::OpenOptions;
            use std::io::Write;
            
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .and_then(|mut file| file.write_all(content.as_bytes()))
                .map(|_| Value::Boolean(true))
                .map_err(|e| format!("Failed to append to file: {}", e))
        }
        _ => Err("append() requires path and content as strings".to_string()),
    }
}

fn exists(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("exists() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::String(path) => {
            Ok(Value::Boolean(Path::new(path).exists()))
        }
        _ => Err("exists() requires a string path".to_string()),
    }
}

fn delete(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("delete() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::String(path) => {
            fs::remove_file(path)
                .map(|_| Value::Boolean(true))
                .map_err(|e| format!("Failed to delete file: {}", e))
        }
        _ => Err("delete() requires a string path".to_string()),
    }
}

fn read_lines(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("read_lines() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::String(path) => {
            let content = fs::read_to_string(path)
                .map_err(|e| format!("Failed to read file: {}", e))?;
            
            let lines: Vec<Value> = content
                .lines()
                .map(|line| Value::String(line.to_string()))
                .collect();
            
            Ok(Value::Array(lines))
        }
        _ => Err("read_lines() requires a string path".to_string()),
    }
}

fn write_lines(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("write_lines() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::String(path), Value::Array(lines)) => {
            let content: Vec<String> = lines.iter()
                .map(|v| v.to_string())
                .collect();
            
            fs::write(path, content.join("\n"))
                .map(|_| Value::Boolean(true))
                .map_err(|e| format!("Failed to write lines: {}", e))
        }
        _ => Err("write_lines() requires path (string) and lines (array)".to_string()),
    }
}
