use crate::value::Value;
use std::process::Command;
use std::env;
use std::path::PathBuf;
use std::collections::HashMap;

/// Get the current process ID
/// Example: getpid() => 1234
pub fn getpid(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("Syscall.getpid takes no arguments".to_string());
    }
    
    // Get the current process ID using std::process
    let pid = std::process::id();
    
    Ok(Value::Int(pid as i64))
}

/// Get the current working directory
/// Example: getcwd() => "/home/user/projects"
pub fn getcwd(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("Syscall.getcwd takes no arguments".to_string());
    }
    
    // Get the current working directory
    match env::current_dir() {
        Ok(path) => {
            match path.to_str() {
                Some(path_str) => Ok(Value::String(path_str.to_string())),
                None => Err("Failed to convert path to string".to_string()),
            }
        },
        Err(e) => Err(format!("Failed to get current working directory: {}", e)),
    }
}

/// Execute a system command
/// Example: execute("echo Hello") => "Hello\n"
pub fn execute(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Syscall.execute requires exactly 1 argument: command".to_string());
    }
    
    let command = args[0].as_string()?;
    
    // Split the command into program and arguments
    let mut parts = command.split_whitespace();
    let program = parts.next().ok_or_else(|| "Empty command".to_string())?;
    let arguments: Vec<&str> = parts.collect();
    
    // Execute the command
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;
    
    // Convert the output to a string
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    
    Ok(Value::String(stdout))
}

/// Get an environment variable
/// Example: getenv("PATH") => "/usr/local/bin:/usr/bin:/bin"
pub fn getenv(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Syscall.getenv requires exactly 1 argument: variable_name".to_string());
    }
    
    let var_name = args[0].as_string()?;
    
    // Get the environment variable
    match env::var(&var_name) {
        Ok(value) => Ok(Value::String(value)),
        Err(_) => Ok(Value::Null), // Return null if the variable doesn't exist
    }
}

/// Set an environment variable
/// Example: setenv("MY_VAR", "value") => true
pub fn setenv(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Syscall.setenv requires exactly 2 arguments: variable_name, value".to_string());
    }
    
    let var_name = args[0].as_string()?;
    let var_value = args[1].as_string()?;
    
    // Set the environment variable
    env::set_var(&var_name, &var_value);
    
    Ok(Value::Bool(true))
}

/// Get all environment variables
/// Example: environ() => {"PATH": "/usr/bin", "HOME": "/home/user"}
pub fn environ(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("Syscall.environ takes no arguments".to_string());
    }
    
    // Get all environment variables
    let vars: HashMap<String, String> = env::vars().collect();
    
    // Convert to a Razen map (represented as an array of key-value pairs)
    let mut result = Vec::new();
    for (key, value) in vars {
        result.push(Value::String(key));
        result.push(Value::String(value));
    }
    
    Ok(Value::Array(result))
}

/// Get command line arguments
/// Example: args() => ["program_name", "arg1", "arg2"]
pub fn args(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("Syscall.args takes no arguments".to_string());
    }
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Convert to Razen array
    let result: Vec<Value> = args.into_iter()
        .map(Value::String)
        .collect();
    
    Ok(Value::Array(result))
}

/// Check if a path exists
/// Example: path_exists("/etc/passwd") => true
pub fn path_exists(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Syscall.path_exists requires exactly 1 argument: path".to_string());
    }
    
    let path = args[0].as_string()?;
    
    // Check if the path exists
    let exists = PathBuf::from(&path).exists();
    
    Ok(Value::Bool(exists))
}

/// Get the absolute path
/// Example: realpath("../file.txt") => "/absolute/path/to/file.txt"
pub fn realpath(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Syscall.realpath requires exactly 1 argument: path".to_string());
    }
    
    let path = args[0].as_string()?;
    
    // Get the absolute path
    match std::fs::canonicalize(&path) {
        Ok(abs_path) => {
            match abs_path.to_str() {
                Some(path_str) => Ok(Value::String(path_str.to_string())),
                None => Err("Failed to convert path to string".to_string()),
            }
        },
        Err(e) => Err(format!("Failed to get absolute path: {}", e)),
    }
}

/// Exit the program with a status code
/// Example: exit(0) => (program exits)
pub fn exit(args: Vec<Value>) -> Result<Value, String> {
    let status = if args.is_empty() {
        0 // Default exit status
    } else if args.len() == 1 {
        args[0].as_int()? as i32
    } else {
        return Err("Syscall.exit takes 0 or 1 argument: status_code".to_string());
    };
    
    // Exit the program
    std::process::exit(status);
}

/// Sleep for a specified Int of milliseconds
/// Example: sleep(1000) => true (sleeps for 1 second)
pub fn sleep(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Syscall.sleep requires exactly 1 argument: milliseconds".to_string());
    }
    
    let ms = args[0].as_int()? as u64;
    
    // Sleep for the specified duration
    std::thread::sleep(std::time::Duration::from_millis(ms));
    
    Ok(Value::Bool(true))
}

/// Get the hostname of the system
/// Example: hostname() => "computer-name"
pub fn hostname(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("Syscall.hostname takes no arguments".to_string());
    }
    
    // Get the hostname using the hostname command
    match Command::new("hostname").output() {
        Ok(output) => {
            let hostname = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            
            Ok(Value::String(hostname))
        },
        Err(e) => Err(format!("Failed to get hostname: {}", e)),
    }
}

/// Get the username of the current user
/// Example: username() => "user"
pub fn username(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("Syscall.username takes no arguments".to_string());
    }
    
    // Get the username from the USER or USERNAME environment variable
    if let Ok(user) = env::var("USER") {
        return Ok(Value::String(user));
    }
    
    if let Ok(user) = env::var("USERNAME") {
        return Ok(Value::String(user));
    }
    
    // If environment variables are not available, try using the whoami command
    match Command::new("whoami").output() {
        Ok(output) => {
            let username = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            
            Ok(Value::String(username))
        },
        Err(e) => Err(format!("Failed to get username: {}", e)),
    }
}
