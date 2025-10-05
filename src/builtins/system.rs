// src/builtins/system.rs
//! System operations

use crate::backend::execution::value::Value;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

/// Exit program with code
pub fn exit(stack: &mut Vec<Value>) -> Result<(), String> {
    let code = stack.pop().unwrap_or(Value::Integer(0));
    if let Value::Integer(exit_code) = code {
        process::exit(exit_code as i32);
    } else {
        process::exit(0);
    }
}

/// Sleep for milliseconds
pub fn sleep(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::Integer(ms)) = stack.pop() {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
        stack.push(Value::Null);
        Ok(())
    } else {
        Err("[ERROR] sleep() requires integer milliseconds".to_string())
    }
}

/// Get current timestamp
pub fn time(stack: &mut Vec<Value>) -> Result<(), String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    stack.push(Value::Integer(timestamp));
    Ok(())
}

/// Get environment variable
pub fn env(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(key)) = stack.pop() {
        match std::env::var(&key) {
            Ok(value) => stack.push(Value::String(value)),
            Err(_) => stack.push(Value::Null)
        }
        Ok(())
    } else {
        Err("[ERROR] env() requires string key".to_string())
    }
}

/// Set environment variable
pub fn set_env(stack: &mut Vec<Value>) -> Result<(), String> {
    let value = stack.pop().ok_or("[ERROR] set_env() requires value")?;
    let key = stack.pop().ok_or("[ERROR] set_env() requires key")?;
    
    if let (Value::String(k), Value::String(v)) = (key, value) {
        unsafe { std::env::set_var(k, v); }
        stack.push(Value::Boolean(true));
        Ok(())
    } else {
        Err("[ERROR] set_env() requires string key and value".to_string())
    }
}

/// Get command line arguments
pub fn args(stack: &mut Vec<Value>) -> Result<(), String> {
    let args: Vec<Value> = std::env::args()
        .map(|arg| Value::String(arg))
        .collect();
    stack.push(Value::Array(args));
    Ok(())
}

/// Get platform/OS name
pub fn platform(stack: &mut Vec<Value>) -> Result<(), String> {
    stack.push(Value::String(std::env::consts::OS.to_string()));
    Ok(())
}
