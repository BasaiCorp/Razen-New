use crate::value::Value;
use std::process::Command;
use std::collections::HashMap;
use reqwest::blocking::Client;
use std::time::Duration;
use url::Url;

/// Ping a host to check connectivity
/// Example: ping("google.com") => true
/// Example: ping("https://google.com") => true
pub fn ping(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Net.ping requires exactly 1 argument: host".to_string());
    }
    
    let url = args[0].as_string()?;
    
    // Extract hostname from URL if needed
    let host = match Url::parse(&url) {
        Ok(parsed_url) => {
            parsed_url.host_str().unwrap_or("").to_string()
        }
        Err(_) => {
            // Fallback to simple host extraction
            if url.starts_with("http://") || url.starts_with("https://") {
                url.trim_start_matches("http://")
                   .trim_start_matches("https://")
                   .split('/')
                   .next()
                   .unwrap_or("")
                   .split(':')
                   .next()
                   .unwrap_or("")
                   .to_string()
            } else {
                url.to_string()
            }
        }
    };
    
    if host.is_empty() {
        return Ok(Value::Bool(false));
    }

    let ping_cmd = if cfg!(target_os = "windows") {
        Command::new("ping")
            .arg("-n")
            .arg("1")
            .arg("-w")
            .arg("1000")
            .arg(&host)
            .output()
    } else {
        Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg("-W")
            .arg("1")
            .arg(&host)
            .output()
    };

    match ping_cmd {
        Ok(output) => {
            let success = output.status.success();
            Ok(Value::Bool(success))
        }
        Err(_) => Ok(Value::Bool(false)),
    }
}

/// Sends a GET request to the given URL
/// Example: get("https://api.com") => "response data"
pub fn get(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Net.get requires exactly 1 argument: url".to_string());
    }
    
    let url = args[0].as_string()?;
    
    // Create a client with a timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    // Send the GET request
    let response = client.get(&url)
        .send()
        .map_err(|e| format!("Failed to execute HTTP GET request: {}", e))?;
    
    if response.status().is_success() {
        let text = response.text()
            .map_err(|e| format!("Failed to read response body: {}", e))?;
        Ok(Value::String(text))
    } else {
        Err(format!("HTTP GET request failed with status: {}", response.status()))
    }
}

/// Sends a POST request to the given URL with data
/// Example: post("https://api.com", {a:1}) => "response data"
pub fn post(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Net.post requires exactly 2 arguments: url, data".to_string());
    }
    
    let url = args[0].as_string()?;
    let data = args[1].to_string();
    
    // Create a client with a timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    // Send the POST request
    let response = client.post(&url)
        .body(data)
        .send()
        .map_err(|e| format!("Failed to execute HTTP POST request: {}", e))?;
    
    if response.status().is_success() {
        let text = response.text()
            .map_err(|e| format!("Failed to read response body: {}", e))?;
        Ok(Value::String(text))
    } else {
        Err(format!("HTTP POST request failed with status: {}", response.status()))
    }
}
