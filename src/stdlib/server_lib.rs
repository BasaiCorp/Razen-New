// src/stdlib/server_lib_v2.rs
//! Web server with proper routing - Like Go's net/http and Express.js
//! 
//! Features:
//! - Route registration: app.get("/path", handler)
//! - Path parameters: /users/{id}
//! - Static file serving
//! - Method-based routing (GET, POST, PUT, DELETE)

use crate::backend::execution::value::Value;
use std::collections::HashMap;
use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;
use std::fs;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

// Route storage structure
lazy_static::lazy_static! {
    static ref ROUTES: Arc<Mutex<HashMap<String, HashMap<String, String>>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref STATIC_DIR: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    static ref SERVER_RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

pub fn has_function(name: &str) -> bool {
    matches!(name, 
        "create" | "listen" | "serve_file" | "serve_dir" | 
        "get" | "post" | "put" | "delete" | "route" |
        "text" | "html" | "json" | "status"
    )
}

pub fn get_function_list() -> Vec<&'static str> {
    vec![
        "create", "listen", "serve_file", "serve_dir",
        "get", "post", "put", "delete", "route",
        "text", "html", "json", "status"
    ]
}

pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "create" => create(args),
        "listen" => listen(args),
        "serve_file" => serve_file(args),
        "serve_dir" => serve_dir(args),
        "get" => register_get(args),
        "post" => register_post(args),
        "put" => register_put(args),
        "delete" => register_delete(args),
        "route" => register_route(args),
        "text" => text_response(args),
        "html" => html_response(args),
        "json" => json_response(args),
        "status" => status_response(args),
        _ => Err(format!("Unknown server function: {}", name)),
    }
}

fn create(_args: Vec<Value>) -> Result<Value, String> {
    let mut server_config = HashMap::new();
    server_config.insert("host".to_string(), Value::String("127.0.0.1".to_string()));
    server_config.insert("port".to_string(), Value::Integer(8080));
    server_config.insert("running".to_string(), Value::Boolean(false));
    
    Ok(Value::Map(server_config))
}

// Register a GET route
fn register_get(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("server.get() requires 2 arguments (path, file_path)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(file_path)) => {
            let mut routes = ROUTES.lock().unwrap();
            let method_routes = routes.entry("GET".to_string()).or_insert_with(HashMap::new);
            method_routes.insert(path.clone(), file_path.clone());
            println!("[INFO] Registered route: GET {} -> {}", path, file_path);
            Ok(Value::Null)
        }
        _ => Err("server.get() requires two string arguments (path, file_path)".to_string()),
    }
}

// Register a POST route
fn register_post(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("server.post() requires 2 arguments (path, file_path)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(file_path)) => {
            let mut routes = ROUTES.lock().unwrap();
            let method_routes = routes.entry("POST".to_string()).or_insert_with(HashMap::new);
            method_routes.insert(path.clone(), file_path.clone());
            println!("[INFO] Registered route: POST {} -> {}", path, file_path);
            Ok(Value::Null)
        }
        _ => Err("server.post() requires two string arguments (path, file_path)".to_string()),
    }
}

// Register a PUT route
fn register_put(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("server.put() requires 2 arguments (path, file_path)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(file_path)) => {
            let mut routes = ROUTES.lock().unwrap();
            let method_routes = routes.entry("PUT".to_string()).or_insert_with(HashMap::new);
            method_routes.insert(path.clone(), file_path.clone());
            println!("[INFO] Registered route: PUT {} -> {}", path, file_path);
            Ok(Value::Null)
        }
        _ => Err("server.put() requires two string arguments (path, file_path)".to_string()),
    }
}

// Register a DELETE route
fn register_delete(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("server.delete() requires 2 arguments (path, file_path)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(file_path)) => {
            let mut routes = ROUTES.lock().unwrap();
            let method_routes = routes.entry("DELETE".to_string()).or_insert_with(HashMap::new);
            method_routes.insert(path.clone(), file_path.clone());
            println!("[INFO] Registered route: DELETE {} -> {}", path, file_path);
            Ok(Value::Null)
        }
        _ => Err("server.delete() requires two string arguments (path, file_path)".to_string()),
    }
}

// Register a generic route
fn register_route(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 3 {
        return Err("server.route() requires 3 arguments (method, path, file_path)".to_string());
    }
    
    match (&args[0], &args[1], &args[2]) {
        (Value::String(method), Value::String(path), Value::String(file_path)) => {
            let mut routes = ROUTES.lock().unwrap();
            let method_upper = method.to_uppercase();
            let method_routes = routes.entry(method_upper.clone()).or_insert_with(HashMap::new);
            method_routes.insert(path.clone(), file_path.clone());
            println!("[INFO] Registered route: {} {} -> {}", method_upper, path, file_path);
            Ok(Value::Null)
        }
        _ => Err("server.route() requires three string arguments (method, path, file_path)".to_string()),
    }
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
    if args.len() > 1 {
        if let Value::String(dir) = &args[1] {
            let mut static_dir = STATIC_DIR.lock().unwrap();
            *static_dir = Some(dir.clone());
            println!("[INFO] Static files directory: {}", dir);
        }
    }
    
    let host = "127.0.0.1";
    let addr = format!("{}:{}", host, port);
    
    println!("[INFO] Starting server on {}", addr);
    
    // Set up Ctrl+C handler
    SERVER_RUNNING.store(true, Ordering::SeqCst);
    let running = SERVER_RUNNING.clone();
    ctrlc::set_handler(move || {
        println!("\n[INFO] Shutting down server...");
        running.store(false, Ordering::SeqCst);
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
    
    match TcpListener::bind(&addr) {
        Ok(listener) => {
            // Set non-blocking mode for graceful shutdown
            listener.set_nonblocking(true).ok();
            
            println!("[SUCCESS] Server listening on http://{}", addr);
            println!("[INFO] Press Ctrl+C to stop");
            
            // Request handler with routing
            for stream in listener.incoming() {
                // Check if server should stop
                if !SERVER_RUNNING.load(Ordering::SeqCst) {
                    println!("[INFO] Server stopped");
                    break;
                }
                
                match stream {
                    Ok(mut stream) => {
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
                                        
                                        // Check registered routes first
                                        let routes = ROUTES.lock().unwrap();
                                        if let Some(method_routes) = routes.get(method) {
                                            // Try exact path match first
                                            let mut file_path_opt = method_routes.get(&path).cloned();
                                            
                                            // If no exact match, try with .html extension
                                            if file_path_opt.is_none() && !path.ends_with(".html") && !path.contains('.') {
                                                let path_with_html = format!("{}.html", path);
                                                file_path_opt = method_routes.get(&path_with_html).cloned();
                                            }
                                            
                                            if let Some(file_path) = file_path_opt {
                                                // Serve the file mapped to this route
                                                match fs::read_to_string(&file_path) {
                                                    Ok(content) => {
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
                                                    }
                                                    Err(e) => {
                                                        eprintln!("[ERROR] Failed to read route file '{}': {}", file_path, e);
                                                    }
                                                }
                                            }
                                        }
                                        drop(routes);
                                        
                                        // If no route matched, try static file serving
                                        let static_dir = STATIC_DIR.lock().unwrap();
                                        if let Some(ref dir) = *static_dir {
                                            // Default to index.html for root path
                                            if path == "/" {
                                                path = "/index.html".to_string();
                                            }
                                            
                                            let file_path = format!("{}{}", dir, path);
                                            
                                            match fs::read_to_string(&file_path) {
                                                Ok(content) => {
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
                                                }
                                                Err(e) => {
                                                    eprintln!("[ERROR] Failed to read static file '{}': {}", file_path, e);
                                                    eprintln!("[DEBUG] Current directory: {:?}", std::env::current_dir());
                                                }
                                            }
                                        }
                                        
                                        // File not found - send 404
                                        let not_found = format!(
                                            "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html>
<html><head><title>404 Not Found</title></head>
<body style='font-family: sans-serif; text-align: center; padding: 50px;'>
<h1>404 Not Found</h1>
<p>The requested path <strong>{}</strong> was not found.</p>
<p>No route registered and file not found in static directory.</p>
<a href='/'>Go Home</a>
</body></html>",
                                            path
                                        );
                                        let _ = stream.write_all(not_found.as_bytes());
                                        let _ = stream.flush();
                                    }
                                }
                            }
                        });
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // Non-blocking mode, no connection available
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        continue;
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
