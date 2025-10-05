// src/stdlib/os_lib.rs
//! Operating system operations standard library

use crate::backend::execution::value::Value;
use std::env;
use std::process::Command;

pub fn has_function(name: &str) -> bool {
    matches!(name, 
        "env" | "set_env" | "remove_env" | "env_vars" |
        "current_dir" | "set_current_dir" | "home_dir" | "temp_dir" |
        "platform" | "arch" | "hostname" | "username" |
        "args" | "exit" | "exec" | "which"
    )
}

pub fn get_function_list() -> Vec<&'static str> {
    vec![
        "env", "set_env", "remove_env", "env_vars",
        "current_dir", "set_current_dir", "home_dir", "temp_dir",
        "platform", "arch", "hostname", "username",
        "args", "exit", "exec", "which"
    ]
}

pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "env" => get_env(args),
        "set_env" => set_env(args),
        "remove_env" => remove_env(args),
        "env_vars" => env_vars(args),
        "current_dir" => current_dir(args),
        "set_current_dir" => set_current_dir(args),
        "home_dir" => home_dir(args),
        "temp_dir" => temp_dir(args),
        "platform" => platform(args),
        "arch" => arch(args),
        "hostname" => hostname(args),
        "username" => username(args),
        "args" => get_args(args),
        "exit" => exit(args),
        "exec" => exec(args),
        "which" => which(args),
        _ => Err(format!("Unknown os function: {}", name)),
    }
}

fn get_env(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("env() requires 1 argument (variable name)".to_string());
    }
    
    match &args[0] {
        Value::String(var_name) => {
            match env::var(var_name) {
                Ok(value) => Ok(Value::String(value)),
                Err(_) => Ok(Value::Null),
            }
        }
        _ => Err("env() requires a string argument".to_string()),
    }
}

fn set_env(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("set_env() requires 2 arguments (name, value)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(name), Value::String(value)) => {
            unsafe {
                env::set_var(name, value);
            }
            Ok(Value::Null)
        }
        _ => Err("set_env() requires two string arguments".to_string()),
    }
}

fn remove_env(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("remove_env() requires 1 argument (variable name)".to_string());
    }
    
    match &args[0] {
        Value::String(var_name) => {
            unsafe {
                env::remove_var(var_name);
            }
            Ok(Value::Null)
        }
        _ => Err("remove_env() requires a string argument".to_string()),
    }
}

fn env_vars(_args: Vec<Value>) -> Result<Value, String> {
    let vars: Vec<Value> = env::vars()
        .map(|(key, value)| {
            let mut map = std::collections::HashMap::new();
            map.insert("key".to_string(), Value::String(key));
            map.insert("value".to_string(), Value::String(value));
            Value::Map(map)
        })
        .collect();
    Ok(Value::Array(vars))
}

fn current_dir(_args: Vec<Value>) -> Result<Value, String> {
    match env::current_dir() {
        Ok(path) => Ok(Value::String(path.to_string_lossy().to_string())),
        Err(e) => Err(format!("Failed to get current directory: {}", e)),
    }
}

fn set_current_dir(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("set_current_dir() requires 1 argument (path)".to_string());
    }
    
    match &args[0] {
        Value::String(path) => {
            match env::set_current_dir(path) {
                Ok(_) => Ok(Value::Null),
                Err(e) => Err(format!("Failed to set current directory: {}", e)),
            }
        }
        _ => Err("set_current_dir() requires a string argument".to_string()),
    }
}

fn home_dir(_args: Vec<Value>) -> Result<Value, String> {
    match dirs::home_dir() {
        Some(path) => Ok(Value::String(path.to_string_lossy().to_string())),
        None => Ok(Value::Null),
    }
}

fn temp_dir(_args: Vec<Value>) -> Result<Value, String> {
    let path = env::temp_dir();
    Ok(Value::String(path.to_string_lossy().to_string()))
}

fn platform(_args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::String(env::consts::OS.to_string()))
}

fn arch(_args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::String(env::consts::ARCH.to_string()))
}

fn hostname(_args: Vec<Value>) -> Result<Value, String> {
    match hostname::get() {
        Ok(name) => Ok(Value::String(name.to_string_lossy().to_string())),
        Err(e) => Err(format!("Failed to get hostname: {}", e)),
    }
}

fn username(_args: Vec<Value>) -> Result<Value, String> {
    match whoami::username() {
        name => Ok(Value::String(name)),
    }
}

fn get_args(_args: Vec<Value>) -> Result<Value, String> {
    let args: Vec<Value> = env::args()
        .map(|arg| Value::String(arg))
        .collect();
    Ok(Value::Array(args))
}

fn exit(args: Vec<Value>) -> Result<Value, String> {
    let code = if args.is_empty() {
        0
    } else {
        match &args[0] {
            Value::Integer(n) => *n as i32,
            _ => 0,
        }
    };
    
    std::process::exit(code);
}

fn exec(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("exec() requires at least 1 argument (command)".to_string());
    }
    
    match &args[0] {
        Value::String(cmd) => {
            let cmd_args: Vec<String> = args[1..]
                .iter()
                .map(|v| v.to_string())
                .collect();
            
            match Command::new(cmd).args(&cmd_args).output() {
                Ok(output) => {
                    let mut result = std::collections::HashMap::new();
                    result.insert("status".to_string(), Value::Integer(output.status.code().unwrap_or(-1) as i64));
                    result.insert("stdout".to_string(), Value::String(
                        String::from_utf8_lossy(&output.stdout).to_string()
                    ));
                    result.insert("stderr".to_string(), Value::String(
                        String::from_utf8_lossy(&output.stderr).to_string()
                    ));
                    Ok(Value::Map(result))
                }
                Err(e) => Err(format!("Failed to execute command: {}", e)),
            }
        }
        _ => Err("exec() requires a string command".to_string()),
    }
}

fn which(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("which() requires 1 argument (command name)".to_string());
    }
    
    match &args[0] {
        Value::String(cmd) => {
            match which::which(cmd) {
                Ok(path) => Ok(Value::String(path.to_string_lossy().to_string())),
                Err(_) => Ok(Value::Null),
            }
        }
        _ => Err("which() requires a string argument".to_string()),
    }
}
