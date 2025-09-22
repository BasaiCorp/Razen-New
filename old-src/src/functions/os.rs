use crate::value::Value;
use std::env;

/// Gets the value of an environment variable
/// Example: env("PATH") => "/usr/bin:/bin"
pub fn env_var(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("OS.env requires exactly 1 argument: variable_name".to_string());
    }
    
    let var_name = args[0].as_string()?;
    
    match env::var(&var_name) {
        Ok(value) => Ok(Value::String(value)),
        Err(_) => Ok(Value::Null),
    }
}

/// Gets the current working directory
/// Example: cwd() => "/home/user"
pub fn cwd(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("OS.cwd takes no arguments".to_string());
    }
    
    match env::current_dir() {
        Ok(path) => Ok(Value::String(path.to_string_lossy().to_string())),
        Err(e) => Err(format!("Failed to get current directory: {}", e)),
    }
}

/// Gets the platform name (e.g., "linux", "windows")
/// Example: platform() => "linux"
pub fn platform(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("OS.platform takes no arguments".to_string());
    }
    
    #[cfg(target_os = "linux")]
    return Ok(Value::String("linux".to_string()));
    
    #[cfg(target_os = "windows")]
    return Ok(Value::String("windows".to_string()));
    
    #[cfg(target_os = "macos")]
    return Ok(Value::String("macos".to_string()));
    
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    return Ok(Value::String("unknown".to_string()));
}
