use crate::value::Value;
use chrono::{Local, TimeZone, NaiveDateTime, Datelike};

/// Get the current timestamp
/// Example: now() => 1650123456789
pub fn now(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("Time.now takes no arguments".to_string());
    }
    
    let now = Local::now();
    let timestamp = now.timestamp() * 1000 + (now.timestamp_subsec_millis() as i64);
    
    Ok(Value::Int(timestamp))
}

/// Format a timestamp according to a format string
/// Example: format(1650123456789, "YYYY-MM-DD") => "2022-04-16"
pub fn format(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Time.format requires exactly 2 arguments: timestamp, format_string".to_string());
    }
    
    let timestamp = match &args[0] {
        Value::Int(ts) => ts / 1000, // Convert from milliseconds to seconds
        Value::Float(ts) => (*ts / 1000.0) as i64,
        _ => return Err(format!("First argument to format must be a timestamp, got {:?}", args[0])),
    };
    
    let format_string = args[1].as_string()?;
    
    // Convert timestamp to DateTime
    let dt = match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp)),
    };
    
    // Map Razen format to chrono format
    let chrono_format = format_string
        .replace("YYYY", "%Y")
        .replace("MM", "%m")
        .replace("DD", "%d")
        .replace("HH", "%H")
        .replace("mm", "%M")
        .replace("ss", "%S");
    
    let formatted = dt.format(&chrono_format).to_string();
    
    Ok(Value::String(formatted))
}

/// Parse a date string into a timestamp
/// Example: parse("2022-04-16", "YYYY-MM-DD") => 1650067200000
pub fn parse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Time.parse requires exactly 2 arguments: date_string, format_string".to_string());
    }
    
    let date_string = args[0].as_string()?;
    let format_string = args[1].as_string()?;
    
    // Map Razen format to chrono format
    let chrono_format = format_string
        .replace("YYYY", "%Y")
        .replace("MM", "%m")
        .replace("DD", "%d")
        .replace("HH", "%H")
        .replace("mm", "%M")
        .replace("ss", "%S");
    
    // Parse the date string
    let dt = match NaiveDateTime::parse_from_str(&date_string, &chrono_format) {
        Ok(dt) => dt,
        Err(e) => return Err(format!("Failed to parse date: {}", e)),
    };
    
    // Convert to timestamp (milliseconds)
    let timestamp = dt.and_utc().timestamp() * 1000;
    
    Ok(Value::Int(timestamp))
}

/// Add a duration to a timestamp
/// Example: add(1650067200000, 86400000) => 1650153600000 (add 1 day)
pub fn add(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Time.add requires exactly 2 arguments: timestamp, milliseconds".to_string());
    }
    
    let timestamp = match &args[0] {
        Value::Int(ts) => *ts,
        Value::Float(ts) => *ts as i64,
        _ => return Err(format!("First argument to add must be a timestamp, got {:?}", args[0])),
    };
    
    let milliseconds = match &args[1] {
        Value::Int(ms) => *ms,
        Value::Float(ms) => *ms as i64,
        _ => return Err(format!("Second argument to add must be milliseconds, got {:?}", args[1])),
    };
    
    Ok(Value::Int(timestamp + milliseconds))
}

/// Get the year from a timestamp
/// Example: year(1650067200000) => 2022
pub fn year(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Time.year requires exactly 1 argument: timestamp".to_string());
    }
    
    let timestamp = match &args[0] {
        Value::Int(ts) => ts / 1000, // Convert from milliseconds to seconds
        Value::Float(ts) => (*ts / 1000.0) as i64,
        _ => return Err(format!("Argument to year must be a timestamp, got {:?}", args[0])),
    };
    
    // Convert timestamp to DateTime
    let dt = match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp)),
    };
    
    Ok(Value::Int(dt.year() as i64))
}

/// Get the month from a timestamp (1-12)
/// Example: month(1650067200000) => 4 (April)
pub fn month(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Time.month requires exactly 1 argument: timestamp".to_string());
    }
    
    let timestamp = match &args[0] {
        Value::Int(ts) => ts / 1000, // Convert from milliseconds to seconds
        Value::Float(ts) => (*ts / 1000.0) as i64,
        _ => return Err(format!("Argument to month must be a timestamp, got {:?}", args[0])),
    };
    
    // Convert timestamp to DateTime
    let dt = match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp)),
    };
    
    Ok(Value::Int(dt.month() as i64))
}

/// Get the day of the month from a timestamp (1-31)
/// Example: day(1650067200000) => 16
pub fn day(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Time.day requires exactly 1 argument: timestamp".to_string());
    }
    
    let timestamp = match &args[0] {
        Value::Int(ts) => ts / 1000, // Convert from milliseconds to seconds
        Value::Float(ts) => (*ts / 1000.0) as i64,
        _ => return Err(format!("Argument to day must be a timestamp, got {:?}", args[0])),
    };
    
    // Convert timestamp to DateTime
    let dt = match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp)),
    };
    
    Ok(Value::Int(dt.day() as i64))
}
