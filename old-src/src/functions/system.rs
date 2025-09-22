use crate::value::Value;
use std::process::Command;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

/// Executes a system command and returns the output
/// Example: exec("ls") => "file1\nfile2"
pub fn exec(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("System.exec requires exactly 1 argument: command".to_string());
    }
    
    let cmd_str = args[0].as_string()?;
    
    // Split the command string into command and arguments
    let mut parts = cmd_str.split_whitespace();
    let command = parts.next().ok_or_else(|| "Empty command".to_string())?;
    let arguments: Vec<&str> = parts.collect();
    
    // Execute the command
    let output = Command::new(command)
        .args(arguments)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(Value::String(stdout))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Command failed: {}", stderr))
    }
}

/// Returns system uptime in seconds
/// Example: uptime() => 12345
pub fn uptime(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("System.uptime takes no arguments".to_string());
    }
    
    // This is a simplified implementation that returns the time since UNIX epoch
    // A real implementation would use OS-specific APIs to get the actual uptime
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => Ok(Value::Int(duration.as_secs() as i64)),
        Err(e) => Err(format!("Failed to get system time: {}", e)),
    }
}

/// Returns system information
/// Example: info() => {os: "linux", cpu: "x86_64"}
pub fn info(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("System.info takes no arguments".to_string());
    }
    
    let mut info_map = HashMap::new();
    
    // OS information
    #[cfg(target_os = "linux")]
    info_map.insert("os".to_string(), Value::String("linux".to_string()));
    #[cfg(target_os = "windows")]
    info_map.insert("os".to_string(), Value::String("windows".to_string()));
    #[cfg(target_os = "macos")]
    info_map.insert("os".to_string(), Value::String("macos".to_string()));
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    info_map.insert("os".to_string(), Value::String("unknown".to_string()));
    
    // CPU architecture
    #[cfg(target_arch = "x86_64")]
    info_map.insert("cpu".to_string(), Value::String("x86_64".to_string()));
    #[cfg(target_arch = "x86")]
    info_map.insert("cpu".to_string(), Value::String("x86".to_string()));
    #[cfg(target_arch = "aarch64")]
    info_map.insert("cpu".to_string(), Value::String("aarch64".to_string()));
    #[cfg(target_arch = "arm")]
    info_map.insert("cpu".to_string(), Value::String("arm".to_string()));
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64", target_arch = "arm")))]
    info_map.insert("cpu".to_string(), Value::String("unknown".to_string()));
    
    // Memory information (simplified)
    info_map.insert("memory_total".to_string(), Value::String("unknown".to_string()));
    
    // Host name (simplified)
    info_map.insert("hostname".to_string(), Value::String("unknown".to_string()));
    
    Ok(Value::Map(info_map))
}

/// Returns the current system time in milliseconds since epoch
/// Example: current_time() => 1621234567890
pub fn current_time(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("System.current_time takes no arguments".to_string());
    }
    
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let millis = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
            Ok(Value::Int(millis as i64))
        },
        Err(e) => Err(format!("Failed to get system time: {}", e)),
    }
}

/// Returns the system name (hostname)
/// Example: system_name() => "hostname"
pub fn system_name(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("System.system_name takes no arguments".to_string());
    }
    
    match env::var("HOSTNAME").or_else(|_| env::var("COMPUTERNAME")) {
        Ok(hostname) => Ok(Value::String(hostname)),
        Err(_) => {
            // Try to get hostname using the hostname command
            match Command::new("hostname").output() {
                Ok(output) if output.status.success() => {
                    let hostname = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    Ok(Value::String(hostname))
                },
                _ => Ok(Value::String("unknown".to_string())),
            }
        },
    }
}
