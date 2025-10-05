// src/stdlib/time_lib.rs
//! Time and date operations standard library

use crate::backend::execution::value::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Datelike, Timelike};

pub fn has_function(name: &str) -> bool {
    matches!(name, 
        "now" | "timestamp" | "sleep" | "format_timestamp" | 
        "year" | "month" | "day" | "hour" | "minute" | "second" |
        "add_seconds" | "add_minutes" | "add_hours" | "add_days"
    )
}

pub fn get_function_list() -> Vec<&'static str> {
    vec![
        "now", "timestamp", "sleep", "format_timestamp",
        "year", "month", "day", "hour", "minute", "second",
        "add_seconds", "add_minutes", "add_hours", "add_days"
    ]
}

pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "now" => now(args),
        "timestamp" => timestamp(args),
        "sleep" => sleep(args),
        "format_timestamp" => format_timestamp(args),
        "year" => year(args),
        "month" => month(args),
        "day" => day(args),
        "hour" => hour(args),
        "minute" => minute(args),
        "second" => second(args),
        "add_seconds" => add_seconds(args),
        "add_minutes" => add_minutes(args),
        "add_hours" => add_hours(args),
        "add_days" => add_days(args),
        _ => Err(format!("Unknown time function: {}", name)),
    }
}

fn now(_args: Vec<Value>) -> Result<Value, String> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => Ok(Value::Integer(duration.as_secs() as i64)),
        Err(_) => Err("Failed to get current time".to_string()),
    }
}

fn timestamp(_args: Vec<Value>) -> Result<Value, String> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => Ok(Value::Integer(duration.as_secs() as i64)),
        Err(_) => Err("Failed to get timestamp".to_string()),
    }
}

fn sleep(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sleep() takes exactly 1 argument (seconds)".to_string());
    }
    
    match &args[0] {
        Value::Integer(secs) => {
            std::thread::sleep(std::time::Duration::from_secs(*secs as u64));
            Ok(Value::Null)
        }
        Value::Number(secs) => {
            std::thread::sleep(std::time::Duration::from_secs_f64(*secs));
            Ok(Value::Null)
        }
        _ => Err("sleep() requires a number argument".to_string()),
    }
}

fn format_timestamp(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("format_timestamp() takes exactly 1 argument".to_string());
    }
    
    match &args[0] {
        Value::Integer(timestamp) => {
            let datetime = chrono::DateTime::from_timestamp(*timestamp, 0)
                .ok_or("Invalid timestamp")?;
            Ok(Value::String(datetime.format("%Y-%m-%d %H:%M:%S").to_string()))
        }
        _ => Err("format_timestamp() requires an integer timestamp".to_string()),
    }
}

fn year(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        let now = chrono::Local::now();
        return Ok(Value::Integer(now.year() as i64));
    }
    
    match &args[0] {
        Value::Integer(timestamp) => {
            let datetime = chrono::DateTime::from_timestamp(*timestamp, 0)
                .ok_or("Invalid timestamp")?;
            Ok(Value::Integer(datetime.year() as i64))
        }
        _ => Err("year() requires an integer timestamp or no arguments".to_string()),
    }
}

fn month(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        let now = chrono::Local::now();
        return Ok(Value::Integer(now.month() as i64));
    }
    
    match &args[0] {
        Value::Integer(timestamp) => {
            let datetime = chrono::DateTime::from_timestamp(*timestamp, 0)
                .ok_or("Invalid timestamp")?;
            Ok(Value::Integer(datetime.month() as i64))
        }
        _ => Err("month() requires an integer timestamp or no arguments".to_string()),
    }
}

fn day(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        let now = chrono::Local::now();
        return Ok(Value::Integer(now.day() as i64));
    }
    
    match &args[0] {
        Value::Integer(timestamp) => {
            let datetime = chrono::DateTime::from_timestamp(*timestamp, 0)
                .ok_or("Invalid timestamp")?;
            Ok(Value::Integer(datetime.day() as i64))
        }
        _ => Err("day() requires an integer timestamp or no arguments".to_string()),
    }
}

fn hour(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        let now = chrono::Local::now();
        return Ok(Value::Integer(now.hour() as i64));
    }
    
    match &args[0] {
        Value::Integer(timestamp) => {
            let datetime = chrono::DateTime::from_timestamp(*timestamp, 0)
                .ok_or("Invalid timestamp")?;
            Ok(Value::Integer(datetime.hour() as i64))
        }
        _ => Err("hour() requires an integer timestamp or no arguments".to_string()),
    }
}

fn minute(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        let now = chrono::Local::now();
        return Ok(Value::Integer(now.minute() as i64));
    }
    
    match &args[0] {
        Value::Integer(timestamp) => {
            let datetime = chrono::DateTime::from_timestamp(*timestamp, 0)
                .ok_or("Invalid timestamp")?;
            Ok(Value::Integer(datetime.minute() as i64))
        }
        _ => Err("minute() requires an integer timestamp or no arguments".to_string()),
    }
}

fn second(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        let now = chrono::Local::now();
        return Ok(Value::Integer(now.second() as i64));
    }
    
    match &args[0] {
        Value::Integer(timestamp) => {
            let datetime = chrono::DateTime::from_timestamp(*timestamp, 0)
                .ok_or("Invalid timestamp")?;
            Ok(Value::Integer(datetime.second() as i64))
        }
        _ => Err("second() requires an integer timestamp or no arguments".to_string()),
    }
}

fn add_seconds(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("add_seconds() takes exactly 2 arguments (timestamp, seconds)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::Integer(timestamp), Value::Integer(seconds)) => {
            Ok(Value::Integer(timestamp + seconds))
        }
        _ => Err("add_seconds() requires two integer arguments".to_string()),
    }
}

fn add_minutes(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("add_minutes() takes exactly 2 arguments (timestamp, minutes)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::Integer(timestamp), Value::Integer(minutes)) => {
            Ok(Value::Integer(timestamp + (minutes * 60)))
        }
        _ => Err("add_minutes() requires two integer arguments".to_string()),
    }
}

fn add_hours(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("add_hours() takes exactly 2 arguments (timestamp, hours)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::Integer(timestamp), Value::Integer(hours)) => {
            Ok(Value::Integer(timestamp + (hours * 3600)))
        }
        _ => Err("add_hours() requires two integer arguments".to_string()),
    }
}

fn add_days(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("add_days() takes exactly 2 arguments (timestamp, days)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::Integer(timestamp), Value::Integer(days)) => {
            Ok(Value::Integer(timestamp + (days * 86400)))
        }
        _ => Err("add_days() requires two integer arguments".to_string()),
    }
}
