use crate::value::Value;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom, BufReader, BufWriter};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use std::path::{Path, PathBuf};
use std::fmt;

// Custom error type for binary file operations
#[derive(Debug)]
enum BinaryError {
    FileNotFound(PathBuf),
    PermissionDenied(PathBuf),
    InvalidHandle(usize),
    InvalidMode(String),
    InvalidSeek { offset: i64, whence: String },
    ReadError { handle: usize, source: io::Error },
    WriteError { handle: usize, source: io::Error },
    SeekError { handle: usize, source: io::Error },
    InvalidArgument(String),
    EndOfFile { handle: usize, requested: usize, read: usize },
    Other(String),
}

impl fmt::Display for BinaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryError::FileNotFound(path) => write!(f, "File not found: {}", path.display()),
            BinaryError::PermissionDenied(path) => write!(f, "Permission denied: {}", path.display()),
            BinaryError::InvalidHandle(handle) => write!(f, "Invalid file handle: {}", handle),
            BinaryError::InvalidMode(mode) => write!(f, "Invalid file mode: {}", mode),
            BinaryError::InvalidSeek { offset, whence } => 
                write!(f, "Invalid seek parameters: offset={}, whence={}", offset, whence),
            BinaryError::ReadError { handle, source } => 
                write!(f, "Failed to read from handle {}: {}", handle, source),
            BinaryError::WriteError { handle, source } => 
                write!(f, "Failed to write to handle {}: {}", handle, source),
            BinaryError::SeekError { handle, source } => 
                write!(f, "Failed to seek in handle {}: {}", handle, source),
            BinaryError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            BinaryError::EndOfFile { handle, requested, read } => 
                write!(f, "End of file reached on handle {}: requested {} bytes, read {} bytes", handle, requested, read),
            BinaryError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

// Convert BinaryError to String for the Razen API
impl From<BinaryError> for String {
    fn from(error: BinaryError) -> Self {
        error.to_string()
    }
}

// Global file handle manager to track open file handles
lazy_static::lazy_static! {
    static ref FILE_MANAGER: Arc<Mutex<FileManager>> = Arc::new(Mutex::new(FileManager::new()));
}

// Enhanced file types for better performance
enum FileType {
    Raw(File),
    Buffered(BufWriter<File>),
    Readable(BufReader<File>),
}

// File statistics for tracking operations
struct FileStats {
    bytes_read: usize,
    bytes_written: usize,
    seek_operations: usize,
    path: PathBuf,
    mode: String,
}

// File manager to track open file handles with improved performance
struct FileManager {
    files: HashMap<usize, FileType>,
    stats: HashMap<usize, FileStats>,
    next_file_id: usize,
}

impl FileManager {
    fn new() -> Self {
        FileManager {
            files: HashMap::new(),
            stats: HashMap::new(),
            next_file_id: 1,
        }
    }
    
    // Get global file statistics as a map for external use
    fn get_global_stats(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        let mut total_files = 0;
        let mut total_bytes_read = 0;
        let mut total_bytes_written = 0;
        let mut total_seek_operations = 0;
        
        for stats in self.stats.values() {
            total_files += 1;
            total_bytes_read += stats.bytes_read;
            total_bytes_written += stats.bytes_written;
            total_seek_operations += stats.seek_operations;
        }
        
        map.insert("open_files".to_string(), Value::Int(total_files as i64));
        map.insert("total_bytes_read".to_string(), Value::Int(total_bytes_read as i64));
        map.insert("total_bytes_written".to_string(), Value::Int(total_bytes_written as i64));
        map.insert("total_seek_operations".to_string(), Value::Int(total_seek_operations as i64));
        
        // Add list of open files
        let mut open_files = Vec::new();
        for (id, stats) in &self.stats {
            let mut file_info = HashMap::new();
            file_info.insert("id".to_string(), Value::Int(*id as i64));
            file_info.insert("path".to_string(), Value::String(stats.path.to_string_lossy().to_string()));
            file_info.insert("bytes_read".to_string(), Value::Int(stats.bytes_read as i64));
            file_info.insert("bytes_written".to_string(), Value::Int(stats.bytes_written as i64));
            open_files.push(Value::Map(file_info));
        }
        map.insert("files".to_string(), Value::Array(open_files));
        
        map
    }

    // Register a file with appropriate buffering based on mode
    fn register_file(&mut self, file: File, path: PathBuf, mode: String) -> usize {
        let id = self.next_file_id;
        self.next_file_id += 1;
        
        // Create appropriate file type based on mode
        let file_type = match mode.as_str() {
            // Read modes use BufReader for better performance
            "rb" | "r+b" => FileType::Readable(BufReader::with_capacity(8192, file)),
            
            // Write/append modes use BufWriter for better performance
            "wb" | "w+b" | "ab" | "a+b" => FileType::Buffered(BufWriter::with_capacity(8192, file)),
            
            // Fallback to raw file
            _ => FileType::Raw(file),
        };
        
        // Initialize statistics
        let stats = FileStats {
            bytes_read: 0,
            bytes_written: 0,
            seek_operations: 0,
            path,
            mode,
        };
        
        self.files.insert(id, file_type);
        self.stats.insert(id, stats);
        id
    }

    // Get a file by handle with error handling (deprecated, use get_file_mut instead)
    fn get_file(&mut self, id: usize) -> Result<&mut FileType, BinaryError> {
        self.files.get_mut(&id)
            .ok_or(BinaryError::InvalidHandle(id))
    }
    
    // Get a file by handle with error handling (mutable access)
    fn get_file_mut(&mut self, id: usize) -> Result<&mut FileType, BinaryError> {
        // First collect the keys to avoid borrowing issues
        let available_handles: Vec<usize> = self.files.keys().cloned().collect();
        
        match self.files.get_mut(&id) {
            Some(file_type) => Ok(file_type),
            None => {
                println!("Invalid file handle: {}", id);
                println!("Available handles: {:?}", available_handles);
                Err(BinaryError::InvalidHandle(id))
            }
        }
    }

    // Close a file with proper cleanup
    fn close_file(&mut self, id: usize) -> Result<(), BinaryError> {
        // Get the file type
        let file_type = self.files.remove(&id).ok_or(BinaryError::InvalidHandle(id))?;
        
        // Flush buffers if needed before closing
        match file_type {
            FileType::Buffered(mut writer) => {
                if let Err(e) = writer.flush() {
                    return Err(BinaryError::WriteError { handle: id, source: e });
                }
            },
            _ => {},
        }
        
        // Remove stats
        self.stats.remove(&id);
        Ok(())
    }
    
    // Get file statistics
    fn get_stats(&self, id: usize) -> Result<&FileStats, BinaryError> {
        self.stats.get(&id)
            .ok_or(BinaryError::InvalidHandle(id))
    }
    
    // Update read statistics
    fn update_read_stats(&mut self, id: usize, bytes: usize) -> Result<(), BinaryError> {
        if let Some(stats) = self.stats.get_mut(&id) {
            stats.bytes_read += bytes;
            Ok(())
        } else {
            Err(BinaryError::InvalidHandle(id))
        }
    }
    
    // Update write statistics
    fn update_write_stats(&mut self, id: usize, bytes: usize) -> Result<(), BinaryError> {
        if let Some(stats) = self.stats.get_mut(&id) {
            stats.bytes_written += bytes;
            Ok(())
        } else {
            Err(BinaryError::InvalidHandle(id))
        }
    }
    
    // Update seek statistics
    fn update_seek_stats(&mut self, id: usize) -> Result<(), BinaryError> {
        if let Some(stats) = self.stats.get_mut(&id) {
            stats.seek_operations += 1;
            Ok(())
        } else {
            Err(BinaryError::InvalidHandle(id))
        }
    }
}

/// Create a new binary file
/// Example: create("test.bin") => true
pub fn create(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(BinaryError::InvalidArgument("Binary.create requires exactly 1 argument: path".to_string()).into());
    }
    
    let path = args[0].as_string()?;
    let path_buf = PathBuf::from(&path);
    
    match File::create(&path) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => Err(BinaryError::FileNotFound(path_buf).into()),
                io::ErrorKind::PermissionDenied => Err(BinaryError::PermissionDenied(path_buf).into()),
                _ => Err(BinaryError::Other(format!("Failed to create binary file '{}': {}", path, e)).into()),
            }
        }
    }
}

/// Open a binary file
/// Example: open("test.bin", "rb") => file_handle
pub fn open(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(BinaryError::InvalidArgument("Binary.open requires exactly 2 arguments: path, mode".to_string()).into());
    }
    
    let path = args[0].as_string()?;
    let mode = args[1].as_string()?;
    let path_buf = PathBuf::from(&path);
    
    // Simplify mode validation to handle simpler modes
    let simplified_mode = match mode.as_str() {
        "r" => "rb",
        "w" => "wb",
        "a" => "ab",
        "r+" => "r+b",
        "w+" => "w+b",
        "a+" => "a+b",
        other => other,
    };
    
    // Validate mode
    if !matches!(simplified_mode, "rb" | "wb" | "ab" | "r+b" | "w+b" | "a+b") {
        return Err(BinaryError::InvalidMode(mode.clone()).into());
    }
    
    let file = match simplified_mode {
        "rb" => OpenOptions::new().read(true).open(&path),
        "wb" => OpenOptions::new().write(true).create(true).truncate(true).open(&path),
        "ab" => OpenOptions::new().write(true).append(true).create(true).open(&path),
        "r+b" => OpenOptions::new().read(true).write(true).open(&path),
        "w+b" => OpenOptions::new().read(true).write(true).create(true).truncate(true).open(&path),
        "a+b" => OpenOptions::new().read(true).write(true).append(true).create(true).open(&path),
        _ => unreachable!(), // We already validated the mode
    };
    
    match file {
        Ok(file) => {
            // Register the file and get its handle
            let id = FILE_MANAGER.lock().register_file(file, path_buf, mode.clone());
            
            // Log the file opening for debugging
            println!("Opened file '{}' with mode '{}' and assigned handle {}", path, mode, id);
            
            Ok(Value::Int(id as i64))
        },
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => Err(BinaryError::FileNotFound(path_buf).into()),
                io::ErrorKind::PermissionDenied => Err(BinaryError::PermissionDenied(path_buf).into()),
                _ => Err(BinaryError::Other(format!("Failed to open binary file '{}': {}", path, e)).into()),
            }
        }
    }
}

/// Close a binary file
/// Example: close(file_handle) => true
pub fn close(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(BinaryError::InvalidArgument("Binary.close requires exactly 1 argument: file_handle".to_string()).into());
    }
    
    let handle = args[0].as_int()? as usize;
    
    // Log the operation for debugging
    println!("Closing file handle {}", handle);
    
    // Get file manager and lock once to avoid temporary value being dropped while borrowed
    let mut file_manager = FILE_MANAGER.lock();
    
    // Get available file handles for debugging (avoid borrowing issues)
    let available_handles: Vec<usize> = file_manager.files.keys().cloned().collect();
    println!("Available file handles before closing: {:?}", available_handles);
    
    // Get file stats and clone them before closing to avoid borrowing conflicts
    let stats_option = match file_manager.get_stats(handle) {
        Ok(stats) => {
            // Clone the relevant stats we need for logging
            let path = stats.path.clone();
            let bytes_read = stats.bytes_read;
            let bytes_written = stats.bytes_written;
            Some((path, bytes_read, bytes_written))
        },
        Err(e) => {
            println!("Error getting stats for handle {}: {:?}", handle, e);
            None
        },
    };
    
    // Close the file with proper error handling
    match file_manager.close_file(handle) {
        Ok(_) => {
            // Log file statistics
            if let Some((path, bytes_read, bytes_written)) = stats_option {
                println!("Closed file: {}, Read: {} bytes, Written: {} bytes", 
                    path.display(), bytes_read, bytes_written);
            }
            
            // Get available file handles after closing for debugging (avoid borrowing issues)
            let available_handles_after: Vec<usize> = file_manager.files.keys().cloned().collect();
            println!("Available file handles after closing: {:?}", available_handles_after);
            
            Ok(Value::Bool(true))
        },
        Err(e) => {
            println!("Error closing file handle {}: {:?}", handle, e);
            Err(e.into())
        },
    }
}

/// Get file statistics
/// Example: stats() => { open_files: 1, total_bytes_read: 100, ... }
pub fn stats(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(BinaryError::InvalidArgument("Binary.stats takes no arguments".to_string()).into());
    }
    
    let file_manager = FILE_MANAGER.lock();
    let stats_map = file_manager.get_global_stats();
    
    Ok(Value::Map(stats_map))
}

/// Write bytes to a binary file
/// Example: write_bytes(file_handle, [65, 66, 67]) => 3
pub fn write_bytes(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(BinaryError::InvalidArgument("Binary.write_bytes requires exactly 2 arguments: file_handle, bytes".to_string()).into());
    }
    
    let handle = args[0].as_int()? as usize;
    
    // Convert input to bytes
    let bytes = match &args[1] {
        Value::Array(arr) => {
            let mut bytes = Vec::with_capacity(arr.len());
            for val in arr {
                bytes.push(val.as_int()? as u8);
            }
            bytes
        },
        Value::String(s) => s.as_bytes().to_vec(),
        _ => return Err(BinaryError::InvalidArgument("Second argument must be an array of bytes or a string".to_string()).into()),
    };
    
    // Get file manager and lock once
    let mut file_manager = FILE_MANAGER.lock();
    
    // Log the operation for debugging
    println!("Writing {} bytes to file handle {}", bytes.len(), handle);
    
    // Get file and handle write based on file type
    let result = match file_manager.get_file_mut(handle) {
        Ok(file_type) => {
            match file_type {
                FileType::Raw(file) => file.write_all(&bytes),
                FileType::Buffered(writer) => writer.write_all(&bytes),
                FileType::Readable(_reader) => {
                    // Cannot write to a read-only file
                    return Err(BinaryError::InvalidArgument("Cannot write to a read-only file".to_string()).into());
                }
            }
        },
        Err(e) => return Err(e.into()),
    };
    
    // Handle write result
    match result {
        Ok(_) => {
            // Update statistics
            let _ = file_manager.update_write_stats(handle, bytes.len());
            
            // Flush the file to ensure data is written
            let flush_result = match file_manager.get_file_mut(handle) {
                Ok(file_type) => {
                    match file_type {
                        FileType::Raw(file) => file.flush(),
                        FileType::Buffered(writer) => writer.flush(),
                        FileType::Readable(_) => Ok(()),
                    }
                },
                Err(_) => Ok(()), // Already handled above
            };
            
            if let Err(e) = flush_result {
                return Err(BinaryError::WriteError { handle, source: e }.into());
            }
            
            Ok(Value::Int(bytes.len() as i64))
        },
        Err(e) => Err(BinaryError::WriteError { handle, source: e }.into()),
    }
}

/// Read bytes from a binary file
/// Example: read_bytes(file_handle, 5) => [65, 66, 67, 68, 69]
pub fn read_bytes(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(BinaryError::InvalidArgument("Binary.read_bytes requires exactly 2 arguments: file_handle, count".to_string()).into());
    }
    
    let handle = args[0].as_int()? as usize;
    let count = args[1].as_int()? as usize;
    
    // Validate count
    if count == 0 {
        return Ok(Value::Array(Vec::new()));
    }
    
    if count > 1024 * 1024 * 10 { // 10MB limit for safety
        return Err(BinaryError::InvalidArgument(format!("Requested read size too large: {} bytes", count)).into());
    }
    
    // Log the operation for debugging
    println!("Reading {} bytes from file handle {}", count, handle);
    
    // Get file manager and lock once
    let mut file_manager = FILE_MANAGER.lock();
    
    // Get file and handle read based on file type
    let mut buffer = vec![0u8; count];
    let read_result = match file_manager.get_file(handle) {
        Ok(file_type) => {
            match file_type {
                FileType::Raw(file) => file.read_exact(&mut buffer),
                FileType::Buffered(writer) => {
                    // Flush any pending writes before reading
                    if let Err(e) = writer.flush() {
                        return Err(BinaryError::WriteError { handle, source: e }.into());
                    }
                    // Then read from the underlying file
                    let mut file_ref = writer.get_ref();
                    file_ref.read_exact(&mut buffer)
                },
                FileType::Readable(reader) => {
                    // Use the buffered reader
                    reader.get_mut().read_exact(&mut buffer)
                }
            }
        },
        Err(e) => return Err(e.into()),
    };
    
    // Handle read result
    match read_result {
        Ok(_) => {
            // Update statistics
            let _ = file_manager.update_read_stats(handle, count);
            
            // Convert bytes to array of values
            let values: Vec<Value> = buffer.into_iter()
                .map(|b| Value::Int(b as i64))
                .collect();
            
            Ok(Value::Array(values))
        },
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
            // Handle case where we hit EOF before reading 'count' bytes
            // Try to read as many bytes as possible
            let mut partial_buffer = Vec::new();
            if let Ok(file_type) = file_manager.get_file(handle) {
                match file_type {
                    FileType::Raw(file) => { let _ = file.read_to_end(&mut partial_buffer); },
                    FileType::Buffered(writer) => { let _ = writer.get_ref().read_to_end(&mut partial_buffer); },
                    FileType::Readable(reader) => { let _ = reader.get_mut().read_to_end(&mut partial_buffer); }
                }
            }
            
            // Update statistics with what we actually read
            let _ = file_manager.update_read_stats(handle, partial_buffer.len());
            
            // Return what we could read
            if !partial_buffer.is_empty() {
                let values: Vec<Value> = partial_buffer.into_iter()
                    .map(|b| Value::Int(b as i64))
                    .collect();
                
                Ok(Value::Array(values))
            } else {
                Err(BinaryError::EndOfFile { 
                    handle, 
                    requested: count, 
                    read: 0 
                }.into())
            }
        },
        Err(e) => Err(BinaryError::ReadError { handle, source: e }.into()),
    }
}

/// Seek to a position in a binary file
/// Example: seek(file_handle, offset) => new_position
pub fn seek(args: Vec<Value>) -> Result<Value, String> {
    // Simplify the interface to make it easier to use
    // Just take file handle and offset, assume from start
    if args.len() < 2 || args.len() > 3 {
        return Err(BinaryError::InvalidArgument("Binary.seek requires 2 or 3 arguments: file_handle, offset, [whence]".to_string()).into());
    }
    
    let handle = args[0].as_int()? as usize;
    let offset = args[1].as_int()? as i64;
    
    // Default whence to "start" if not provided
    let whence = if args.len() == 3 {
        args[2].as_string()?
    } else {
        "start".to_string()
    };
    
    // Log the operation for debugging
    println!("Seeking in file handle {} to offset {} from {}", handle, offset, whence);
    
    // Validate whence parameter
    let seek_from = match whence.as_str() {
        "start" => {
            if offset < 0 {
                return Err(BinaryError::InvalidSeek { 
                    offset, 
                    whence: whence.clone() 
                }.into());
            }
            SeekFrom::Start(offset as u64)
        },
        "current" => SeekFrom::Current(offset),
        "end" => SeekFrom::End(offset),
        _ => return Err(BinaryError::InvalidSeek { 
            offset, 
            whence: whence.clone() 
        }.into()),
    };
    
    // Get file manager and lock once
    let mut file_manager = FILE_MANAGER.lock();
    
    // Get available file handles for debugging (avoid borrowing issues)
    let available_handles: Vec<usize> = file_manager.files.keys().cloned().collect();
    println!("Available file handles: {:?}", available_handles);
    
    // Get file and handle seek based on file type
    let seek_result = match file_manager.get_file_mut(handle) {
        Ok(file_type) => {
            match file_type {
                FileType::Raw(file) => file.seek(seek_from),
                FileType::Buffered(writer) => {
                    // Flush any pending writes before seeking
                    if let Err(e) = writer.flush() {
                        return Err(BinaryError::WriteError { handle, source: e }.into());
                    }
                    writer.get_mut().seek(seek_from)
                },
                FileType::Readable(reader) => reader.get_mut().seek(seek_from),
            }
        },
        Err(e) => return Err(e.into()),
    };
    
    // Update seek statistics
    let _ = file_manager.update_seek_stats(handle);
    
    // Handle seek result
    match seek_result {
        Ok(position) => {
            println!("Successfully seeked to position {}", position);
            Ok(Value::Int(position as i64))
        },
        Err(e) => {
            println!("Error seeking: {}", e);
            Err(BinaryError::SeekError { handle, source: e }.into())
        },
    }
}

/// Get the current position in a binary file
/// Example: tell(file_handle) => 10
pub fn tell(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Binary.tell requires exactly 1 argument: file_handle".to_string());
    }
    
    let handle = args[0].as_int()? as usize;
    
    // Get current position
    let mut file_manager = FILE_MANAGER.lock();
    let file_type = file_manager.get_file(handle)?;
    
    // Get position based on file type
    let position_result = match file_type {
        FileType::Raw(file) => file.stream_position(),
        FileType::Buffered(writer) => {
            // Flush any pending writes before getting position
            if let Err(e) = writer.flush() {
                return Err(BinaryError::WriteError { handle, source: e }.into());
            }
            writer.get_mut().stream_position()
        },
        FileType::Readable(reader) => reader.get_mut().stream_position(),
    };
    
    match position_result {
        Ok(position) => Ok(Value::Int(position as i64)),
        Err(e) => Err(format!("Failed to get file position: {}", e)),
    }
}

/// Convert bytes to a string
/// Example: bytes_to_string([65, 66, 67]) => "ABC"
pub fn bytes_to_string(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Binary.bytes_to_string requires exactly 1 argument: bytes".to_string());
    }
    
    let bytes = args[0].as_array()?;
    
    // Convert array of values to bytes
    let mut buffer = Vec::with_capacity(bytes.len());
    for value in bytes {
        match value {
            Value::Int(n) => {
                if n < 0 || n > 255 {
                    return Err(format!("Invalid byte value: {}. Must be an integer between 0 and 255", n));
                }
                buffer.push(n as u8);
            },
            _ => return Err(format!("Invalid byte value: {:?}. Must be a Int between 0 and 255", value)),
        }
    }
    
    // Convert bytes to string
    match String::from_utf8(buffer) {
        Ok(s) => Ok(Value::String(s)),
        Err(_) => Err("Bytes contain invalid UTF-8 data".to_string()),
    }
}

/// Convert a string to bytes
/// Example: string_to_bytes("ABC") => [65, 66, 67]
pub fn string_to_bytes(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Binary.string_to_bytes requires exactly 1 argument: string".to_string());
    }
    
    let string = args[0].as_string()?;
    
    // Convert string to bytes
    let bytes = string.as_bytes();
    
    // Convert bytes to array of values
    let values: Vec<Value> = bytes.iter()
        .map(|&b| Value::Int(b as i64))
        .collect();
    
    Ok(Value::Array(values))
}
