use chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Utc, Weekday};
use chrono::format::{DelayedFormat, StrftimeItems};
use crate::value::Value;
use std::collections::HashMap;

/// Get the current timestamp in seconds since the Unix epoch
pub fn now(_args: Vec<Value>) -> Result<Value, String> {
    let now = Utc::now().timestamp() * 1000; // Convert to milliseconds for compatibility with time.rs
    Ok(Value::Int(now))
}

/// Get the current year
pub fn year(_args: Vec<Value>) -> Result<Value, String> {
    let now = Local::now();
    Ok(Value::Int(now.year() as i64))
}

/// Get the current month (1-12)
pub fn month(_args: Vec<Value>) -> Result<Value, String> {
    let now = Local::now();
    Ok(Value::Int(now.month() as i64))
}

/// Get the current day of month
pub fn day(_args: Vec<Value>) -> Result<Value, String> {
    let now = Local::now();
    Ok(Value::Int(now.day() as i64))
}

/// Format a timestamp as a string
pub fn format(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("format() requires exactly 2 arguments: timestamp and format string".to_string());
    }

    let timestamp = match &args[0] {
        Value::Int(n) => n / 1000, // Convert from milliseconds to seconds
        Value::Float(n) => (*n / 1000.0) as i64,
        _ => return Err("First argument must be a timestamp (number)".to_string()),
    };

    let fmt = match &args[1] {
        Value::String(s) => s,
        _ => return Err("Second argument must be a format string".to_string()),
    };

    // Convert timestamp to DateTime
    let dt = match Utc.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp)),
    };

    // Convert format string from common patterns to strftime format
    let fmt = fmt
        .replace("YYYY", "%Y")
        .replace("MM", "%m")
        .replace("DD", "%d")
        .replace("HH", "%H")
        .replace("mm", "%M")
        .replace("ss", "%S");

    // Format the datetime
    let formatted = dt.format(&fmt).to_string();
    Ok(Value::String(formatted))
}

/// Parse a date string with a format
pub fn parse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("parse() requires exactly 2 arguments: date string and format string".to_string());
    }

    let date_str = match &args[0] {
        Value::String(s) => s,
        _ => return Err("First argument must be a date string".to_string()),
    };

    let fmt = match &args[1] {
        Value::String(s) => s,
        _ => return Err("Second argument must be a format string".to_string()),
    };

    // Convert format string from common patterns to strftime format
    let fmt = fmt
        .replace("YYYY", "%Y")
        .replace("MM", "%m")
        .replace("DD", "%d")
        .replace("HH", "%H")
        .replace("mm", "%M")
        .replace("ss", "%S");

    // Parse the date string
    let dt = match NaiveDateTime::parse_from_str(&format!("{} 00:00:00", date_str), &format!("{} %H:%M:%S", fmt)) {
        Ok(dt) => dt,
        Err(_) => {
            // Try parsing with just the date format
            match NaiveDateTime::parse_from_str(date_str, &fmt) {
                Ok(dt) => dt,
                Err(e) => return Err(format!("Failed to parse date: {}", e)),
            }
        }
    };

    // Convert to timestamp (milliseconds)
    let timestamp = dt.and_utc().timestamp() * 1000;
    Ok(Value::Int(timestamp))
}

/// Add days to a timestamp
pub fn add_days(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("add_days() requires exactly 2 arguments: timestamp and days".to_string());
    }

    let timestamp = match &args[0] {
        Value::Int(n) => *n / 1000, // Convert from milliseconds to seconds
        Value::Float(n) => (*n / 1000.0) as i64,
        _ => return Err("First argument must be a timestamp (number)".to_string()),
    };

    let days = match &args[1] {
        Value::Int(n) => *n,
        Value::Float(n) => *n as i64,
        _ => return Err("Second argument must be a number of days".to_string()),
    };

    // Convert timestamp to DateTime
    let dt = match Utc.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp)),
    };

    // Add days
    let new_dt = dt + chrono::Duration::days(days);
    let new_timestamp = new_dt.timestamp();

    Ok(Value::Int(new_timestamp * 1000))
}

/// Add months to a timestamp
pub fn add_months(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("add_months() requires exactly 2 arguments: timestamp and months".to_string());
    }

    let timestamp = match &args[0] {
        Value::Int(n) => *n / 1000, // Convert from milliseconds to seconds
        Value::Float(n) => (*n / 1000.0) as i64,
        _ => return Err("First argument must be a timestamp (number)".to_string()),
    };

    let months = match &args[1] {
        Value::Int(n) => *n as i32,
        Value::Float(n) => *n as i32,
        _ => return Err("Second argument must be a number of months".to_string()),
    };

    // Convert timestamp to DateTime
    let dt = match Utc.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp)),
    };

    // Convert to NaiveDate
    let naive_date = dt.naive_utc().date();
    
    // Add months (need to handle year changes)
    let mut year = naive_date.year();
    let mut month = naive_date.month() as i32 + months;
    
    // Adjust year if month overflows
    while month > 12 {
        month -= 12;
        year += 1;
    }
    
    // Adjust year if month underflows
    while month < 1 {
        month += 12;
        year -= 1;
    }
    
    // Create new date (clamping day if necessary)
    let day = naive_date.day();
    let days_in_month = get_days_in_month(year, month as u32);
    let adjusted_day = std::cmp::min(day, days_in_month);
    
    // Create new datetime
    let new_date = NaiveDateTime::new(
        match chrono::NaiveDate::from_ymd_opt(year, month as u32, adjusted_day) {
            Some(date) => date,
            None => return Err(format!("Invalid date: {}-{}-{}", year, month, adjusted_day)),
        },
        dt.naive_utc().time()
    );
    
    // Convert back to timestamp
    let new_timestamp = new_date.and_utc().timestamp();
    
    Ok(Value::Int(new_timestamp * 1000))
}

/// Add years to a timestamp
pub fn add_years(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("add_years() requires exactly 2 arguments: timestamp and years".to_string());
    }

    let timestamp = match &args[0] {
        Value::Int(n) => *n / 1000, // Convert from milliseconds to seconds
        Value::Float(n) => (*n / 1000.0) as i64,
        _ => return Err("First argument must be a timestamp (number)".to_string()),
    };

    let years = match &args[1] {
        Value::Int(n) => *n as i32,
        Value::Float(n) => *n as i32,
        _ => return Err("Second argument must be a number of years".to_string()),
    };

    // Convert timestamp to DateTime
    let dt = match Utc.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp)),
    };

    // Add years (need to handle leap years)
    let naive_date = dt.naive_utc().date();
    let new_year = naive_date.year() + years;
    
    // Handle Feb 29 in leap years
    let month = naive_date.month();
    let day = naive_date.day();
    let adjusted_day = if month == 2 && day == 29 && !check_leap_year(new_year) {
        28
    } else {
        day
    };
    
    // Create new datetime
    let new_date = NaiveDateTime::new(
        match chrono::NaiveDate::from_ymd_opt(new_year, month, adjusted_day) {
            Some(date) => date,
            None => return Err(format!("Invalid date: {}-{}-{}", new_year, month, adjusted_day)),
        },
        dt.naive_utc().time()
    );
    
    // Convert back to timestamp
    let new_timestamp = new_date.and_utc().timestamp();
    
    Ok(Value::Int(new_timestamp * 1000))
}

/// Get the day of week (0 = Sunday, 6 = Saturday)
pub fn weekday(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("weekday() requires exactly 1 argument: timestamp".to_string());
    }

    let timestamp = match &args[0] {
        Value::Int(n) => *n / 1000, // Convert from milliseconds to seconds
        Value::Float(n) => (*n / 1000.0) as i64,
        _ => return Err("Argument must be a timestamp (number)".to_string()),
    };

    // Convert timestamp to DateTime
    let dt = match Utc.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp)),
    };

    // Get weekday (0 = Sunday, 6 = Saturday)
    let weekday = match dt.weekday() {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 0,
    };

    Ok(Value::Int(weekday as i64))
}

/// Get the name of the weekday
pub fn weekday_name(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("weekday_name() requires exactly 1 argument: timestamp".to_string());
    }

    let timestamp = match &args[0] {
        Value::Int(n) => *n / 1000, // Convert from milliseconds to seconds
        Value::Float(n) => (*n / 1000.0) as i64,
        _ => return Err("Argument must be a timestamp (number)".to_string()),
    };

    // Convert timestamp to DateTime
    let dt = match Utc.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp)),
    };

    // Get weekday name
    let weekday_name = match dt.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };

    Ok(Value::String(weekday_name.to_string()))
}

/// Get the number of days in a month
pub fn days_in_month(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("days_in_month() requires exactly 2 arguments: year and month".to_string());
    }

    let year = match &args[0] {
        Value::Int(n) => *n as i32,
        Value::Float(n) => *n as i32,
        _ => return Err("First argument must be a year (number)".to_string()),
    };

    let month = match &args[1] {
        Value::Int(n) => *n as u32,
        Value::Float(n) => *n as u32,
        _ => return Err("Second argument must be a month (number)".to_string()),
    };

    if month < 1 || month > 12 {
        return Err(format!("Invalid month: {}", month));
    }

    let days = get_days_in_month(year, month);
    Ok(Value::Int(days as i64))
}

/// Check if a year is a leap year
pub fn is_leap_year(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is_leap_year() requires exactly 1 argument: year".to_string());
    }

    let year = match &args[0] {
        Value::Int(n) => *n as i32,
        Value::Float(n) => *n as i32,
        _ => return Err("Argument must be a year (number)".to_string()),
    };
    
    println!("Checking if year {} is a leap year", year);

    Ok(Value::Bool(check_leap_year(year)))
}

/// Get the difference in days between two dates
pub fn diff_days(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("diff_days() requires exactly 2 arguments: timestamp1 and timestamp2".to_string());
    }

    let timestamp1 = match &args[0] {
        Value::Int(n) => *n / 1000, // Convert from milliseconds to seconds
        Value::Float(n) => (*n / 1000.0) as i64,
        _ => return Err("First argument must be a timestamp (number)".to_string()),
    };

    let timestamp2 = match &args[1] {
        Value::Int(n) => *n / 1000, // Convert from milliseconds to seconds
        Value::Float(n) => (*n / 1000.0) as i64,
        _ => return Err("Second argument must be a timestamp (number)".to_string()),
    };

    // Convert timestamps to DateTime
    let dt1 = match Utc.timestamp_opt(timestamp1, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp1)),
    };

    let dt2 = match Utc.timestamp_opt(timestamp2, 0) {
        chrono::LocalResult::Single(dt) => dt,
        _ => return Err(format!("Invalid timestamp: {}", timestamp2)),
    };

    // Calculate difference in days
    let diff = (dt2.date_naive() - dt1.date_naive()).num_days();

    Ok(Value::Int(diff as i64))
}

// Helper function to get days in a month
fn get_days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if check_leap_year(year) { 29 } else { 28 },
        _ => 0, // Invalid month
    }
}

// Helper function to check if a year is a leap year
fn check_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
