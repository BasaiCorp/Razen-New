use crate::value::Value;
use std::process::{Command, Child, Stdio};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::{BufReader, BufRead};
use std::thread;

// Global process manager to track running processes
lazy_static::lazy_static! {
    static ref PROCESS_MANAGER: Arc<Mutex<ProcessManager>> = Arc::new(Mutex::new(ProcessManager::new()));
}

// Process manager to track running processes
struct ProcessManager {
    processes: HashMap<usize, Child>,
    next_process_id: usize,
}

impl ProcessManager {
    fn new() -> Self {
        ProcessManager {
            processes: HashMap::new(),
            next_process_id: 1,
        }
    }

    fn register_process(&mut self, process: Child) -> usize {
        let id = self.next_process_id;
        self.next_process_id += 1;
        self.processes.insert(id, process);
        id
    }

    fn get_process(&mut self, id: usize) -> Result<&mut Child, String> {
        self.processes.get_mut(&id)
            .ok_or_else(|| format!("Invalid process ID: {}", id))
    }

    fn remove_process(&mut self, id: usize) -> Result<Child, String> {
        self.processes.remove(&id)
            .ok_or_else(|| format!("Invalid process ID: {}", id))
    }
}

/// Create a new process
/// Example: create("ls -l") => 1
pub fn create(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Process.create requires exactly 1 argument: command".to_string());
    }
    
    let command_str = args[0].as_string()?;
    
    // Split the command into program and arguments
    let mut parts = command_str.split_whitespace();
    let program = parts.next().ok_or_else(|| "Empty command".to_string())?;
    let arguments: Vec<&str> = parts.collect();
    
    // Create the process
    let process = Command::new(program)
        .args(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to create process: {}", e))?;
    
    // Register the process
    let process_id = PROCESS_MANAGER.lock().unwrap().register_process(process);
    
    Ok(Value::Int(process_id as i64))
}

/// Wait for a process to complete
/// Example: wait(1) => 0 (exit status)
pub fn wait(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Process.wait requires exactly 1 argument: process_id".to_string());
    }
    
    let process_id = args[0].as_int()? as usize;
    
    // Remove the process from the manager
    let mut process = PROCESS_MANAGER.lock().unwrap().remove_process(process_id)?;
    
    // Wait for the process to complete
    match process.wait() {
        Ok(status) => {
            let exit_code = status.code().unwrap_or(-1);
            Ok(Value::Int(exit_code as i64))
        },
        Err(e) => Err(format!("Failed to wait for process: {}", e)),
    }
}

/// Check if a process is running
/// Example: is_running(1) => true
pub fn is_running(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Process.is_running requires exactly 1 argument: process_id".to_string());
    }
    
    let process_id = args[0].as_int()? as usize;
    
    // Try to get the process
    let mut process_manager = PROCESS_MANAGER.lock().unwrap();
    match process_manager.get_process(process_id) {
        Ok(process) => {
            // Check if the process is still running
            match process.try_wait() {
                Ok(None) => Ok(Value::Bool(true)), // Still running
                Ok(Some(_)) => Ok(Value::Bool(false)), // Exited
                Err(_) => Ok(Value::Bool(false)), // Error, assume not running
            }
        },
        Err(_) => Ok(Value::Bool(false)), // Process not found, assume not running
    }
}

/// Kill a process
/// Example: kill(1) => true
pub fn kill(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Process.kill requires exactly 1 argument: process_id".to_string());
    }
    
    let process_id = args[0].as_int()? as usize;
    
    // Try to get the process
    let mut process_manager = PROCESS_MANAGER.lock().unwrap();
    match process_manager.get_process(process_id) {
        Ok(process) => {
            // Kill the process
            match process.kill() {
                Ok(_) => Ok(Value::Bool(true)),
                Err(e) => Err(format!("Failed to kill process: {}", e)),
            }
        },
        Err(e) => Err(e),
    }
}

/// Send a signal to a process
/// Example: signal(1, "SIGTERM") => true
pub fn signal(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Process.signal requires exactly 2 arguments: process_id, signal".to_string());
    }
    
    let process_id = args[0].as_int()? as usize;
    let signal_name = args[1].as_string()?;
    
    // This is a simplified implementation that just kills the process
    // In a real implementation, we would need to map signal names to signal Ints
    // and use the appropriate system calls
    
    // For now, just kill the process
    let mut process_manager = PROCESS_MANAGER.lock().unwrap();
    match process_manager.get_process(process_id) {
        Ok(process) => {
            // Kill the process
            match process.kill() {
                Ok(_) => Ok(Value::Bool(true)),
                Err(e) => Err(format!("Failed to send signal to process: {}", e)),
            }
        },
        Err(e) => Err(e),
    }
}

/// Get information about a process
/// Example: info() => {"pid": 1234, "ppid": 1233}
pub fn info(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("Process.info takes no arguments".to_string());
    }
    
    // Get the current process ID
    let pid = std::process::id();
    
    // Create a map with process information
    let mut info = Vec::new();
    info.push(Value::String("pid".to_string()));
    info.push(Value::Int(pid as i64));
    
    // Try to get the parent process ID using the ps command
    match Command::new("ps")
        .args(["-o", "ppid=", "-p", &pid.to_string()])
        .output() {
        Ok(output) => {
            let ppid_str = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            
            if let Ok(ppid) = ppid_str.parse::<u32>() {
                info.push(Value::String("ppid".to_string()));
                info.push(Value::Int(ppid as i64));
            }
        },
        Err(_) => {
            // Ignore errors
        },
    }
    
    Ok(Value::Array(info))
}

/// Read the standard output of a process
/// Example: read_stdout(1) => "output"
pub fn read_stdout(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Process.read_stdout requires exactly 1 argument: process_id".to_string());
    }
    
    let process_id = args[0].as_int()? as usize;
    
    // Try to get the process
    let mut process_manager = PROCESS_MANAGER.lock().unwrap();
    match process_manager.get_process(process_id) {
        Ok(process) => {
            // Get the stdout handle
            let stdout = process.stdout.take()
                .ok_or_else(|| "Failed to get process stdout".to_string())?;
            
            // Read all output
            let mut reader = BufReader::new(stdout);
            let mut output = String::new();
            reader.read_line(&mut output)
                .map_err(|e| format!("Failed to read process stdout: {}", e))?;
            
            // Put the stdout handle back
            process.stdout = Some(reader.into_inner());
            
            Ok(Value::String(output))
        },
        Err(e) => Err(e),
    }
}

/// Read the standard error of a process
/// Example: read_stderr(1) => "error"
pub fn read_stderr(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Process.read_stderr requires exactly 1 argument: process_id".to_string());
    }
    
    let process_id = args[0].as_int()? as usize;
    
    // Try to get the process
    let mut process_manager = PROCESS_MANAGER.lock().unwrap();
    match process_manager.get_process(process_id) {
        Ok(process) => {
            // Get the stderr handle
            let stderr = process.stderr.take()
                .ok_or_else(|| "Failed to get process stderr".to_string())?;
            
            // Read all output
            let mut reader = BufReader::new(stderr);
            let mut output = String::new();
            reader.read_line(&mut output)
                .map_err(|e| format!("Failed to read process stderr: {}", e))?;
            
            // Put the stderr handle back
            process.stderr = Some(reader.into_inner());
            
            Ok(Value::String(output))
        },
        Err(e) => Err(e),
    }
}

/// Write to the standard input of a process
/// Example: write_stdin(1, "input") => true
pub fn write_stdin(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Process.write_stdin requires exactly 2 arguments: process_id, data".to_string());
    }
    
    let process_id = args[0].as_int()? as usize;
    let data = args[1].as_string()?;
    
    // Try to get the process
    let mut process_manager = PROCESS_MANAGER.lock().unwrap();
    match process_manager.get_process(process_id) {
        Ok(process) => {
            // Get the stdin handle
            let mut stdin = process.stdin.take()
                .ok_or_else(|| "Failed to get process stdin".to_string())?;
            
            // Write to stdin
            use std::io::Write;
            match stdin.write_all(data.as_bytes()) {
                Ok(_) => {
                    // Put the stdin handle back
                    process.stdin = Some(stdin);
                    Ok(Value::Bool(true))
                },
                Err(e) => {
                    // Put the stdin handle back
                    process.stdin = Some(stdin);
                    Err(format!("Failed to write to process stdin: {}", e))
                },
            }
        },
        Err(e) => Err(e),
    }
}
