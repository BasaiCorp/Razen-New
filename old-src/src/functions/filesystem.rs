use std::fs::{self, File, OpenOptions, Permissions, DirBuilder};
use std::path::{Path, PathBuf};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::DirBuilderExt;
use std::collections::HashMap;
use chrono::{DateTime, Local};
use rand::{random, Rng};
use base64;
use crate::value::Value;

// Helper function to safely convert a Value to a string
fn safe_to_string(value: &Value) -> Result<String, String> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Int(i) => Ok(i.to_string()),
        Value::Float(f) => Ok(f.to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Null => Ok("null".to_string()),
        Value::Array(arr) => {
            let mut result = "[".to_string();
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(&safe_to_string(item)?);
            }
            result.push_str("]");
            Ok(result)
        },
        Value::Map(map) => {
            let mut result = "{".to_string();
            let mut first = true;
            for (key, value) in map {
                if !first {
                    result.push_str(", ");
                }
                first = false;
                result.push_str(&format!("{}: {}", key, safe_to_string(value)?));
            }
            result.push_str("}");
            Ok(result)
        },
        _ => Err(format!("Cannot convert value to string: {:?}", value)),
    }
}

/// Checks if a path exists
/// Example: exists("path/to/file") => true
pub fn exists(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("filesystem.exists() takes exactly 1 argument (path)".to_string());
    }
    
    let path = args[0].as_string()?;
    Ok(Value::Bool(Path::new(&path).exists()))
}

/// Checks if a path is a file
/// Example: is_file("path/to/file") => true
pub fn is_file(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("filesystem.is_file() takes exactly 1 argument (path)".to_string());
    }
    
    let path = args[0].as_string()?;
    Ok(Value::Bool(Path::new(&path).is_file()))
}

/// Checks if a path is a directory
/// Example: is_dir("path/to/dir") => true
pub fn is_dir(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("filesystem.is_dir() takes exactly 1 argument (path)".to_string());
    }
    
    let path = args[0].as_string()?;
    Ok(Value::Bool(Path::new(&path).is_dir()))
}

/// Creates a new directory
/// Example: create_dir("path/to/new_dir", true) => true
pub fn create_dir(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() || args.len() > 3 {
        return Err("filesystem.create_dir() takes 1-3 arguments (path, recursive = false, mode = 0o755)".to_string());
    }
    
    let path = args[0].as_string()?;
    let recursive = if args.len() >= 2 { 
        match &args[1] {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::String(s) => s.to_lowercase() == "true" || s == "1",
            _ => false
        }
    } else { 
        false 
    };
    
    // Optional mode parameter (Unix permissions)
    let mode = if args.len() >= 3 {
        match &args[2] {
            Value::Int(i) => *i as u32,
            Value::Float(f) => *f as u32,
            Value::String(s) => {
                if s.starts_with("0o") || s.starts_with("0O") {
                    // Octal format like "0o755"
                    if let Ok(m) = u32::from_str_radix(&s[2..], 8) {
                        m
                    } else {
                        0o755 // Default if parsing fails
                    }
                } else if let Ok(m) = s.parse::<u32>() {
                    m
                } else {
                    0o755 // Default if parsing fails
                }
            },
            _ => 0o755 // Default mode
        }
    } else {
        0o755 // Default mode (rwxr-xr-x)
    };
    
    let result = if recursive {
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        builder.mode(mode);
        builder.create(&path)
    } else {
        fs::create_dir(&path)
    };
    
    match result {
        Ok(_) => {
            // Set permissions if not recursive or on non-Unix platforms
            if !recursive {
                #[cfg(unix)]
                if let Ok(metadata) = fs::metadata(&path) {
                    let mut perms = metadata.permissions();
                    perms.set_mode(mode);
                    let _ = fs::set_permissions(&path, perms);
                }
            }
            Ok(Value::Bool(true))
        },
        Err(e) => Err(format!("Failed to create directory '{}': {}", path, e)),
    }
}

/// Removes a file or directory
/// Example: remove("path/to/file", true) => true
pub fn remove(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() || args.len() > 2 {
        return Err("filesystem.remove() takes 1 or 2 arguments (path, recursive = false)".to_string());
    }
    
    let path = args[0].as_string()?;
    let recursive = if args.len() == 2 { 
        match &args[1] {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::String(s) => s.to_lowercase() == "true" || s == "1",
            _ => false
        }
    } else { 
        false 
    };
    
    // Check if path exists
    if !Path::new(&path).exists() {
        // Return true if the path doesn't exist (idempotent operation)
        return Ok(Value::Bool(true));
    }
    
    let metadata = fs::metadata(&path).map_err(|e| format!("Failed to get metadata for '{}': {}", path, e))?;
    
    let result = if metadata.is_dir() {
        if recursive {
            fs::remove_dir_all(&path)
        } else {
            // Check if directory is empty first
            match fs::read_dir(&path) {
                Ok(mut entries) => {
                    if entries.next().is_some() {
                        return Err(format!("Directory '{}' is not empty. Use recursive=true to remove non-empty directories.", path));
                    }
                    fs::remove_dir(&path)
                },
                Err(e) => return Err(format!("Failed to read directory '{}': {}", path, e))
            }
        }
    } else {
        fs::remove_file(&path)
    };
    
    match result {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(format!("Failed to remove '{}': {}", path, e)),
    }
}

/// Reads the contents of a file
/// Example: read_file("path/to/file") => "file contents"
pub fn read_file(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 || args.len() > 2 {
        return Err("filesystem.read_file() takes 1 or 2 arguments (path, binary = false)".to_string());
    }
    
    let path = args[0].as_string()?;
    let path_obj = Path::new(&path);
    
    // Optional binary mode parameter
    let binary_mode = if args.len() == 2 {
        match &args[1] {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::String(s) => s.to_lowercase() == "true" || s == "1" || s.to_lowercase() == "binary",
            _ => false
        }
    } else {
        false // Default to text mode
    };
    
    // Check if path exists and is a file
    if !path_obj.exists() {
        return Err(format!("File '{}' does not exist", path));
    }
    
    if !path_obj.is_file() {
        return Err(format!("'{}' is not a file", path));
    }
    
    // Read file contents
    let file = File::open(&path).map_err(|e| format!("Failed to open file '{}': {}", path, e))?;
    let mut reader = BufReader::new(file);
    
    if binary_mode {
        // Read as binary data
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).map_err(|e| format!("Failed to read file '{}': {}", path, e))?;
        
        // Convert binary data to a base64-encoded string
        let base64_data = base64::encode(&buffer);
        Ok(Value::String(base64_data))
    } else {
        // Read as text
        let mut contents = String::new();
        reader.read_to_string(&mut contents).map_err(|e| format!("Failed to read file '{}': {}", path, e))?;
        Ok(Value::String(contents))
    }
}

/// Writes content to a file
/// Example: write_file("path/to/file", "content", false) => true
pub fn write_file(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 4 {
        return Err("filesystem.write_file() takes 2-4 arguments (path, content, append = false, binary = false)".to_string());
    }
    
    let path = args[0].as_string()?;
    let content = args[1].as_string()?;
    
    // Parse append flag (3rd argument)
    let append = if args.len() >= 3 { 
        match &args[2] {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::String(s) => s.to_lowercase() == "true" || s == "1",
            _ => false
        }
    } else { 
        false 
    };
    
    // Parse binary flag (4th argument)
    let binary = if args.len() >= 4 { 
        match &args[3] {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::String(s) => s.to_lowercase() == "true" || s == "1" || s.to_lowercase() == "binary",
            _ => false
        }
    } else { 
        false 
    };
    
    // Create parent directories if they don't exist
    if let Some(parent) = Path::new(&path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create parent directories for '{}': {}", path, e))?;
        }
    }
    
    let result = if binary {
        // Decode base64 content for binary mode
        let binary_data = match base64::decode(&content) {
            Ok(data) => data,
            Err(_) => content.into_bytes() // Fallback if not valid base64
        };
        
        if append {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(&path)
                .map_err(|e| format!("Failed to open file '{}' for appending: {}", path, e))?;
                
            file.write_all(&binary_data).map_err(|e| format!("Failed to write to file '{}': {}", path, e))
        } else {
            fs::write(&path, binary_data).map_err(|e| format!("Failed to write to file '{}': {}", path, e))
        }
    } else {
        // Text mode
        if append {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(&path)
                .map_err(|e| format!("Failed to open file '{}' for appending: {}", path, e))?;
                
            let mut writer = BufWriter::new(file);
            writer.write_all(content.as_bytes()).map_err(|e| format!("Failed to write to file '{}': {}", path, e))
        } else {
            fs::write(&path, content).map_err(|e| format!("Failed to write to file '{}': {}", path, e))
        }
    };
    
    match result {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(e),
    }
}

/// Lists the contents of a directory
/// Example: list_dir("path/to/dir") => ["file1.txt", "file2.txt"]
pub fn list_dir(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 || args.len() > 2 {
        return Err("filesystem.list_dir() takes 1 or 2 arguments (path, detailed = false)".to_string());
    }
    
    let path = args[0].as_string()?;
    let path_obj = Path::new(&path);
    
    // Check if detailed mode is enabled (returns objects with metadata instead of just names)
    let detailed = if args.len() >= 2 {
        match &args[1] {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::String(s) => s.to_lowercase() == "true" || s == "1" || s.to_lowercase() == "detailed",
            _ => false
        }
    } else {
        false
    };
    
    // Check if path exists and is a directory
    if !path_obj.exists() {
        return Err(format!("Directory '{}' does not exist", path));
    }
    
    if !path_obj.is_dir() {
        return Err(format!("'{}' is not a directory", path));
    }
    
    // Read directory entries
    let entries = fs::read_dir(&path).map_err(|e| format!("Failed to read directory '{}': {}", path, e))?;
    
    let mut result = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let file_name = entry.file_name()
            .into_string()
            .map_err(|_| "Invalid UTF-8 in filename".to_string())?;
        
        if detailed {
            // Create a detailed entry with metadata
            let mut entry_data = HashMap::new();
            entry_data.insert("name".to_string(), Value::String(file_name));
            
            // Add file type
            if let Ok(file_type) = entry.file_type() {
                entry_data.insert("is_file".to_string(), Value::Bool(file_type.is_file()));
                entry_data.insert("is_dir".to_string(), Value::Bool(file_type.is_dir()));
                entry_data.insert("is_symlink".to_string(), Value::Bool(file_type.is_symlink()));
            }
            
            // Add metadata if available
            if let Ok(metadata) = entry.metadata() {
                entry_data.insert("size".to_string(), Value::Float(metadata.len() as f64));
                
                if let Ok(modified) = metadata.modified() {
                    let datetime: DateTime<Local> = modified.into();
                    entry_data.insert("modified".to_string(), Value::String(datetime.to_rfc3339()));
                }
                
                if let Ok(accessed) = metadata.accessed() {
                    let datetime: DateTime<Local> = accessed.into();
                    entry_data.insert("accessed".to_string(), Value::String(datetime.to_rfc3339()));
                }
                
                if let Ok(created) = metadata.created() {
                    let datetime: DateTime<Local> = created.into();
                    entry_data.insert("created".to_string(), Value::String(datetime.to_rfc3339()));
                }
                
                // Add permissions
                entry_data.insert("permissions".to_string(), Value::Float(metadata.permissions().mode() as f64));
            }
            
            result.push(Value::Map(entry_data));
        } else {
            // Just return the filename
            result.push(Value::String(file_name));
        }
    }
    
    Ok(Value::Array(result))
}

/// Gets file/directory metadata
/// Example: metadata("path/to/file") => { "size": 1024, "is_file": true, ... }
pub fn metadata(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("filesystem.metadata() takes exactly 1 argument (path)".to_string());
    }
    
    let path = args[0].as_string()?;
    let path_obj = Path::new(&path);
    
    // Check if path exists
    if !path_obj.exists() {
        return Err(format!("Path '{}' does not exist", path));
    }
    
    // Get metadata
    let metadata = fs::metadata(&path).map_err(|e| format!("Failed to get metadata: {}", e))?;
    
    // Create result object
    let mut result = HashMap::new();
    result.insert("size".to_string(), Value::Float(metadata.len() as f64));
    result.insert("is_file".to_string(), Value::Bool(metadata.is_file()));
    result.insert("is_dir".to_string(), Value::Bool(metadata.is_dir()));
    result.insert("permissions".to_string(), Value::Float(metadata.permissions().mode() as f64));
    
    // Add timestamps if available
    if let Ok(modified) = metadata.modified() {
        let datetime: DateTime<Local> = modified.into();
        result.insert("modified".to_string(), Value::String(datetime.to_rfc3339()));
    }
    
    if let Ok(created) = metadata.created() {
        let datetime: DateTime<Local> = created.into();
        result.insert("created".to_string(), Value::String(datetime.to_rfc3339()));
    }
    
    if let Ok(accessed) = metadata.accessed() {
        let datetime: DateTime<Local> = accessed.into();
        result.insert("accessed".to_string(), Value::String(datetime.to_rfc3339()));
    }
    
    Ok(Value::Map(result))
}

/// Gets the absolute path of a file or directory
/// Example: absolute_path("file.txt") => "/absolute/path/to/file.txt"
pub fn absolute_path(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("filesystem.absolute_path() takes exactly 1 argument (path)".to_string());
    }
    
    let path = args[0].as_string()?;
    let path_obj = Path::new(&path);
    
    let abs_path = fs::canonicalize(&path)
        .map_err(|e| format!("Failed to get absolute path: {}", e))?;
    
    Ok(Value::String(abs_path.to_string_lossy().to_string()))
}

/// Copies a file or directory
/// Example: copy_file("source.txt", "destination.txt") => true
pub fn copy_file(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("filesystem.copy() takes exactly 2 arguments (from, to)".to_string());
    }
    
    let from = args[0].as_string()?;
    let to = args[1].as_string()?;
    
    let from_path = Path::new(&from);
    
    // Check if source exists
    if !from_path.exists() {
        return Err(format!("Source path '{}' does not exist", from));
    }
    
    // Create parent directories for destination if they don't exist
    if let Some(parent) = Path::new(&to).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create parent directories: {}", e))?;
        }
    }
    
    let result = if from_path.is_file() {
        fs::copy(&from, &to).map(|_| ())
            .map_err(|e| format!("Failed to copy file: {}", e))
    } else if from_path.is_dir() {
        copy_dir_all(&from, &to)
            .map_err(|e| format!("Failed to copy directory: {}", e))
    } else {
        return Err(format!("Unsupported file type for '{}'", from));
    };
    
    match result {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => Err(e),
    }
}

/// Recursively copies a directory
fn copy_dir_all(src: &str, dst: &str) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = Path::new(dst).join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(
                src_path.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid source path"))?, 
                dst_path.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid destination path"))?
            )?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

/// Moves a file or directory
/// Example: move_file("old.txt", "new.txt") => true
pub fn move_file(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("filesystem.move() takes exactly 2 arguments (from, to)".to_string());
    }
    
    let from = args[0].as_string()?;
    let to = args[1].as_string()?;
    
    let from_path = Path::new(&from);
    
    // Check if source exists
    if !from_path.exists() {
        return Err(format!("Source path '{}' does not exist", from));
    }
    
    // Create parent directories for destination if they don't exist
    if let Some(parent) = Path::new(&to).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create parent directories: {}", e))?;
        }
    }
    
    fs::rename(&from, &to)
        .map(|_| Value::Bool(true))
        .map_err(|e| format!("Failed to move: {}", e))
}

/// Gets the file extension
/// Example: extension("file.txt") => "txt"
pub fn extension(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("filesystem.extension() takes exactly 1 argument (path)".to_string());
    }
    
    let path = args[0].as_string()?;
    let ext = Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    
    Ok(Value::String(ext))
}

/// Gets the file name without extension
/// Example: file_stem("archive.tar.gz") => "archive.tar"
pub fn file_stem(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("filesystem.file_stem() takes exactly 1 argument (path)".to_string());
    }
    
    let path = args[0].as_string()?;
    let stem = Path::new(&path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    
    Ok(Value::String(stem))
}

/// Gets the parent directory
/// Example: parent_dir("/path/to/file.txt") => "/path/to"
pub fn parent_dir(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("filesystem.parent_dir() takes exactly 1 argument (path)".to_string());
    }
    
    let path = args[0].as_string()?;
    let parent = Path::new(&path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or("")
        .to_string();
    
    Ok(Value::String(parent))
}

/// Joins path components
/// Example: join_path(["path", "to", "file.txt"]) => "path/to/file.txt"
pub fn join_path(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("filesystem.join_path() takes at least 1 argument".to_string());
    }
    
    // If first argument is an array, use its elements as path components
    if let Value::Array(array) = &args[0] {
        let mut path = PathBuf::new();
        
        for component in array {
            let component_str = component.as_string()?;
            path.push(component_str);
        }
        
        return Ok(Value::String(path.to_string_lossy().to_string()));
    }
    
    // Otherwise, use all arguments as path components
    let mut path = PathBuf::new();
    
    for arg in &args {
        let component = arg.as_string()?;
        path.push(component);
    }
    
    Ok(Value::String(path.to_string_lossy().to_string()))
}

/// Changes the current working directory
/// Example: change_dir("/path/to/dir") => true
pub fn change_dir(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("filesystem.change_dir() takes exactly 1 argument (path)".to_string());
    }
    
    let path = args[0].as_string()?;
    let path_obj = Path::new(&path);
    
    // Check if path exists and is a directory
    if !path_obj.exists() {
        return Err(format!("Directory '{}' does not exist", path));
    }
    
    if !path_obj.is_dir() {
        return Err(format!("'{}' is not a directory", path));
    }
    
    std::env::set_current_dir(&path)
        .map(|_| Value::Bool(true))
        .map_err(|e| format!("Failed to change directory: {}", e))
}

/// Gets the current working directory
/// Example: current_dir() => "/current/working/directory"
pub fn current_dir(_args: Vec<Value>) -> Result<Value, String> {
    std::env::current_dir()
        .map(|p| Value::String(p.to_string_lossy().to_string()))
        .map_err(|e| format!("Failed to get current directory: {}", e))
}

/// Creates a temporary file
/// Example: temp_file("prefix") => "/tmp/prefix_123456"
pub fn temp_file(args: Vec<Value>) -> Result<Value, String> {
    let prefix = if !args.is_empty() {
        args[0].as_string()?
    } else {
        "tmp_".to_string()
    };
    
    let temp_dir = std::env::temp_dir();
    let mut temp_file = temp_dir.join(prefix);
    
    // Add a random suffix to avoid collisions
    let suffix: String = random::<u32>().to_string();
    temp_file.set_extension(&suffix);
    
    // Create the file to ensure it exists
    File::create(&temp_file)
        .map_err(|e| format!("Failed to create temp file: {}", e))?;
    
    Ok(Value::String(temp_file.to_string_lossy().to_string()))
}

/// Creates a temporary directory
/// Example: temp_dir("prefix") => "/tmp/prefix_123456"
pub fn temp_dir(args: Vec<Value>) -> Result<Value, String> {
    let prefix = if !args.is_empty() {
        args[0].as_string()?
    } else {
        "tmp_".to_string()
    };
    
    let temp_dir = std::env::temp_dir();
    let mut temp_dir_path = temp_dir.join(prefix);
    
    // Add a random suffix to avoid collisions
    let suffix: String = random::<u32>().to_string();
    temp_dir_path.push(&suffix);
    
    // Create the directory
    fs::create_dir_all(&temp_dir_path)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    Ok(Value::String(temp_dir_path.to_string_lossy().to_string()))
}
