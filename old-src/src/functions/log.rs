use crate::value::Value;
use std::time::SystemTime;
use std::io::{self, Write};

/// Logs an info message
/// Example: info("Started") => true
pub fn info(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Log.info requires exactly 1 argument: message".to_string());
    }
    
    let message = args[0].to_string();
    let timestamp = get_timestamp();
    
    println!("[INFO] {} - {}", timestamp, message);
    io::stdout().flush().unwrap();
    
    Ok(Value::Bool(true))
}

/// Logs a warning message
/// Example: warn("Be careful!") => true
pub fn warn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Log.warn requires exactly 1 argument: message".to_string());
    }
    
    let message = args[0].to_string();
    let timestamp = get_timestamp();
    
    println!("[WARN] {} - {}", timestamp, message);
    io::stdout().flush().unwrap();
    
    Ok(Value::Bool(true))
}

/// Logs an error message
/// Example: error("Something went wrong") => true
pub fn error(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Log.error requires exactly 1 argument: message".to_string());
    }
    
    let message = args[0].to_string();
    let timestamp = get_timestamp();
    
    eprintln!("[ERROR] {} - {}", timestamp, message);
    io::stderr().flush().unwrap();
    
    Ok(Value::Bool(true))
}

/// Logs a debug message
/// Example: debug("x=5") => true
pub fn debug(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Log.debug requires exactly 1 argument: message".to_string());
    }
    
    let message = args[0].to_string();
    let timestamp = get_timestamp();
    
    println!("[DEBUG] {} - {}", timestamp, message);
    io::stdout().flush().unwrap();
    
    Ok(Value::Bool(true))
}

// Helper function to get current timestamp
fn get_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let secs = now % 60;
    let mins = (now / 60) % 60;
    let hours = (now / 3600) % 24;
    
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}
