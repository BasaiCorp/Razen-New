// src/stdlib/server_lib.rs
//! Web server standard library - HTTP server like Go and Node.js
//! 
//! Features:
//! - Route-based request handling
//! - Serve static HTML files from disk
//! - Multi-threaded request processing
//! - Simple and intuitive API

use crate::backend::execution::value::Value;
use std::collections::HashMap;
use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;
use std::fs;
use std::path::Path;

pub fn has_function(name: &str) -> bool {
    matches!(name, 
        "create" | "listen" | "serve_file" | "serve_dir" | 
        "text" | "html" | "json" | "status"
    )
}

pub fn get_function_list() -> Vec<&'static str> {
    vec![
        "create", "listen", "serve_file", "serve_dir",
        "text", "html", "json", "status"
    ]
}

pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "create" => create(args),
        "listen" => listen(args),
        "serve_file" => serve_file(args),
        "serve_dir" => serve_dir(args),
        "text" => text_response(args),
        "html" => html_response(args),
        "json" => json_response(args),
        "status" => status_response(args),
        _ => Err(format!("Unknown server function: {}", name)),
    }
}

// Simple HTTP server implementation
fn create(_args: Vec<Value>) -> Result<Value, String> {
    let mut server_config = HashMap::new();
    server_config.insert("host".to_string(), Value::String("127.0.0.1".to_string()));
    server_config.insert("port".to_string(), Value::Integer(8080));
    server_config.insert("routes".to_string(), Value::Array(vec![]));
    server_config.insert("running".to_string(), Value::Boolean(false));
    
    Ok(Value::Map(server_config))
}

fn listen(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("listen() requires at least 1 argument (port)".to_string());
    }
    
    let port = match &args[0] {
        Value::Integer(p) => *p,
        _ => return Err("listen() requires a port number".to_string()),
    };
    
    // Optional: static directory to serve files from
    let static_dir = if args.len() > 1 {
        match &args[1] {
            Value::String(dir) => Some(dir.clone()),
            _ => None,
        }
    } else {
        None
    };
    
    let host = "127.0.0.1";
    let addr = format!("{}:{}", host, port);
    
    println!("[INFO] Starting server on {}", addr);
    if let Some(ref dir) = static_dir {
        println!("[INFO] Serving static files from: {}", dir);
    }
    
    match TcpListener::bind(&addr) {
        Ok(listener) => {
            println!("[SUCCESS] Server listening on http://{}", addr);
            println!("[INFO] Press Ctrl+C to stop");
            
            // Request handler with file serving
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let static_dir_clone = static_dir.clone();
                        thread::spawn(move || {
                            let mut buffer = [0; 4096];
                            if let Ok(size) = stream.read(&mut buffer) {
                                let request = String::from_utf8_lossy(&buffer[..size]);
                                
                                // Parse request line
                                let lines: Vec<&str> = request.lines().collect();
                                if let Some(request_line) = lines.first() {
                                    let parts: Vec<&str> = request_line.split_whitespace().collect();
                                    if parts.len() >= 2 {
                                        let method = parts[0];
                                        let mut path = parts[1].to_string();
                                        
                                        println!("[INFO] {} {}", method, path);
                                        
                                        // Serve file if static_dir is set
                                        if let Some(ref dir) = static_dir_clone {
                                            // Default to index.html for root path
                                            if path == "/" {
                                                path = "/index.html".to_string();
                                            }
                                            
                                            let file_path = format!("{}{}", dir, path);
                                            
                                            if let Ok(content) = fs::read_to_string(&file_path) {
                                                let content_type = get_content_type(&file_path);
                                                let response = format!(
                                                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                                                    content_type,
                                                    content.len(),
                                                    content
                                                );
                                                let _ = stream.write_all(response.as_bytes());
                                                let _ = stream.flush();
                                                return;
                                            } else {
                                                // File not found - send 404
                                                let not_found = format!(
                                                    "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\n404 Not Found: {}",
                                                    path
                                                );
                                                let _ = stream.write_all(not_found.as_bytes());
                                                let _ = stream.flush();
                                                return;
                                            }
                                        }
                                        
                                        // If no static_dir provided, return error
                                        let error_msg = "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\n\r\nNo static directory configured. Use: server.listen(port, \"./public\")";
                                        let _ = stream.write_all(error_msg.as_bytes());
                                        let _ = stream.flush();
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("[ERROR] Connection failed: {}", e);
                    }
                }
            }
            
            Ok(Value::Null)
        }
        Err(e) => Err(format!("Failed to bind server: {}", e)),
    }
}

// Helper function to determine content type from file extension
fn get_content_type(file_path: &str) -> &'static str {
    if file_path.ends_with(".html") || file_path.ends_with(".htm") {
        "text/html; charset=utf-8"
    } else if file_path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if file_path.ends_with(".js") {
        "application/javascript; charset=utf-8"
    } else if file_path.ends_with(".json") {
        "application/json; charset=utf-8"
    } else if file_path.ends_with(".png") {
        "image/png"
    } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
        "image/jpeg"
    } else if file_path.ends_with(".gif") {
        "image/gif"
    } else if file_path.ends_with(".svg") {
        "image/svg+xml"
    } else if file_path.ends_with(".ico") {
        "image/x-icon"
    } else {
        "text/plain; charset=utf-8"
    }
}

fn serve_file(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("serve_file() requires 1 argument (file_path)".to_string());
    }
    
    match &args[0] {
        Value::String(file_path) => {
            match fs::read_to_string(file_path) {
                Ok(content) => Ok(Value::String(content)),
                Err(e) => Err(format!("Failed to read file: {}", e)),
            }
        }
        _ => Err("serve_file() requires a string file path".to_string()),
    }
}

fn serve_dir(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("serve_dir() requires 2 arguments (port, directory)".to_string());
    }
    
    // This is essentially the same as listen with a directory
    listen(args)
}

fn text_response(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("text() requires 1 argument (text content)".to_string());
    }
    
    match &args[0] {
        Value::String(text) => {
            let mut response = HashMap::new();
            response.insert("type".to_string(), Value::String("text/plain".to_string()));
            response.insert("body".to_string(), Value::String(text.clone()));
            Ok(Value::Map(response))
        }
        _ => Err("text() requires a string argument".to_string()),
    }
}

fn html_response(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("html() requires 1 argument (html content)".to_string());
    }
    
    match &args[0] {
        Value::String(html) => {
            let mut response = HashMap::new();
            response.insert("type".to_string(), Value::String("text/html".to_string()));
            response.insert("body".to_string(), Value::String(html.clone()));
            Ok(Value::Map(response))
        }
        _ => Err("html() requires a string argument".to_string()),
    }
}

fn json_response(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("json() requires 1 argument (json content)".to_string());
    }
    
    match &args[0] {
        Value::String(json) => {
            let mut response = HashMap::new();
            response.insert("type".to_string(), Value::String("application/json".to_string()));
            response.insert("body".to_string(), Value::String(json.clone()));
            Ok(Value::Map(response))
        }
        _ => Err("json() requires a string argument".to_string()),
    }
}

fn status_response(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("status() requires 1 argument (status code)".to_string());
    }
    
    match &args[0] {
        Value::Integer(code) => {
            let mut response = HashMap::new();
            response.insert("status".to_string(), Value::Integer(*code));
            Ok(Value::Map(response))
        }
        _ => Err("status() requires an integer status code".to_string()),
    }
}
