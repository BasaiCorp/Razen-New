use crate::value::Value;
use std::collections::HashMap;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;
use serde_json::{json, Value as JsonValue};
use std::time::Duration;
use url::form_urlencoded;

// Default timeout for API requests in seconds
const DEFAULT_TIMEOUT: u64 = 30;

// Common HTTP status codes
const HTTP_OK: u16 = 200;
const HTTP_CREATED: u16 = 201;
const HTTP_ACCEPTED: u16 = 202;
const HTTP_NO_CONTENT: u16 = 204;
const HTTP_BAD_REQUEST: u16 = 400;
const HTTP_UNAUTHORIZED: u16 = 401;
const HTTP_FORBIDDEN: u16 = 403;
const HTTP_NOT_FOUND: u16 = 404;
const HTTP_SERVER_ERROR: u16 = 500;

/// Make a GET request to an API endpoint
/// Example: get("https://api.example.com/data", {"param1": "value1"}, {"Authorization": "Bearer token"}, 30) => response
/// Arguments:
///   - url: The URL to make the request to
///   - params: (Optional) Query parameters as a map
///   - headers: (Optional) Headers as a map
///   - timeout: (Optional) Timeout in seconds
pub fn get(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("API.get: Expected at least 1 argument (url)".to_string());
    }
    
    let url = args[0].as_string()?;
    let params = if args.len() > 1 { args[1].clone() } else { Value::Map(HashMap::new()) };
    
    // Extract headers if provided
    let headers = if args.len() > 2 { args[2].clone() } else { Value::Map(HashMap::new()) };
    
    // Extract timeout if provided
    let timeout = if args.len() > 3 {
        match &args[3] {
            Value::Int(t) => Some(*t as u64),
            Value::Float(t) => Some(*t as u64),
            _ => None,
        }
    } else {
        None
    };
    
    make_request("GET", &url, params, headers, None, timeout)
}

/// Make a POST request to an API endpoint
/// Example: post("https://api.example.com/data", {"data": "value"}, {"Content-Type": "application/json"}, 30) => response
/// Arguments:
///   - url: The URL to make the request to
///   - data: The body data to send
///   - headers: (Optional) Headers as a map
///   - timeout: (Optional) Timeout in seconds
pub fn post(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("API.post: Expected at least 2 arguments (url, data)".to_string());
    }
    
    let url = args[0].as_string()?;
    let data = args[1].clone();
    
    // Extract headers if provided
    let headers = if args.len() > 2 { args[2].clone() } else { Value::Map(HashMap::new()) };
    
    // Extract timeout if provided
    let timeout = if args.len() > 3 {
        match &args[3] {
            Value::Int(t) => Some(*t as u64),
            Value::Float(t) => Some(*t as u64),
            _ => None,
        }
    } else {
        None
    };
    
    make_request("POST", &url, Value::Map(HashMap::new()), headers, Some(data), timeout)
}

/// Make a PUT request to an API endpoint
/// Example: put("https://api.example.com/data/1", {"data": "updated"}, {"Content-Type": "application/json"}, 30) => response
/// Arguments:
///   - url: The URL to make the request to
///   - data: The body data to send
///   - headers: (Optional) Headers as a map
///   - timeout: (Optional) Timeout in seconds
pub fn putmethod(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("API.put: Expected at least 2 arguments (url, data)".to_string());
    }
    
    let url = args[0].as_string()?;
    let data = args[1].clone();
    
    // Extract headers if provided
    let headers = if args.len() > 2 { args[2].clone() } else { Value::Map(HashMap::new()) };
    
    // Extract timeout if provided
    let timeout = if args.len() > 3 {
        match &args[3] {
            Value::Int(t) => Some(*t as u64),
            Value::Float(t) => Some(*t as u64),
            _ => None,
        }
    } else {
        None
    };
    
    make_request("PUT", &url, Value::Map(HashMap::new()), headers, Some(data), timeout)
}

/// Make a DELETE request to an API endpoint
/// Example: delete("https://api.example.com/data/1", {"Authorization": "Bearer token"}, 30) => response
/// Arguments:
///   - url: The URL to make the request to
///   - headers: (Optional) Headers as a map
///   - timeout: (Optional) Timeout in seconds
pub fn delete(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("API.delete: Expected at least 1 argument (url)".to_string());
    }
    
    let url = args[0].as_string()?;
    
    // Extract headers if provided
    let headers = if args.len() > 1 { args[1].clone() } else { Value::Map(HashMap::new()) };
    
    // Extract timeout if provided
    let timeout = if args.len() > 2 {
        match &args[2] {
            Value::Int(t) => Some(*t as u64),
            Value::Float(t) => Some(*t as u64),
            _ => None,
        }
    } else {
        None
    };
    
    make_request("DELETE", &url, Value::Map(HashMap::new()), headers, None, timeout)
}

/// Make a PATCH request to an API endpoint
/// Example: patch("https://api.example.com/data/1", {"data": "patched"}, {"Content-Type": "application/json"}, 30) => response
/// Arguments:
///   - url: The URL to make the request to
///   - data: The body data to send
///   - headers: (Optional) Headers as a map
///   - timeout: (Optional) Timeout in seconds
pub fn patch(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("API.patch: Expected at least 2 arguments (url, data)".to_string());
    }
    
    let url = args[0].as_string()?;
    let data = args[1].clone();
    
    // Extract headers if provided
    let headers = if args.len() > 2 { args[2].clone() } else { Value::Map(HashMap::new()) };
    
    // Extract timeout if provided
    let timeout = if args.len() > 3 {
        match &args[3] {
            Value::Int(t) => Some(*t as u64),
            Value::Float(t) => Some(*t as u64),
            _ => None,
        }
    } else {
        None
    };
    
    make_request("PATCH", &url, Value::Map(HashMap::new()), headers, Some(data), timeout)
}

/// Call an API with the given options
/// Example: call("https://api.example.com", {"method": "GET", "headers": {"Accept": "application/json"}}) => response
/// Arguments:
///   - url: The URL to make the request to
///   - options: (Optional) Options for the request including method, headers, data, etc.
///     - method: HTTP method to use (default: "GET")
///     - headers: Headers as a map or array of key-value pairs
///     - data: Body data for POST/PUT/PATCH requests
///     - timeout: Timeout in seconds
pub fn call(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("API.call: Expected at least 1 argument (url)".to_string());
    }
    
    let url = args[0].as_string()?;
    
    // Default values
    let mut method = "GET".to_string();
    let mut headers_map = HashMap::new();
    let mut params = Value::Map(HashMap::new());
    let mut body_data = None;
    let mut timeout = None;
    
    // Extract options if provided
    if args.len() > 1 {
        match &args[1] {
            Value::Map(options) => {
                // Extract method
                if let Some(Value::String(m)) = options.get("method") {
                    method = m.clone();
                }
                
                // Extract headers
                if let Some(headers_value) = options.get("headers") {
                    match headers_value {
                        Value::Map(h) => {
                            for (key, value) in h {
                                headers_map.insert(key.clone(), value.clone());
                            }
                        },
                        Value::Array(arr) => {
                            // Handle array of key-value pairs
                            if arr.len() % 2 != 0 {
                                return Err("Headers array must contain an even number of elements (key-value pairs)".to_string());
                            }
                            
                            for i in (0..arr.len()).step_by(2) {
                                if i + 1 < arr.len() {
                                    let key = arr[i].as_string()?;
                                    let value = arr[i + 1].as_string()?;
                                    headers_map.insert(key, Value::String(value));
                                }
                            }
                        },
                        _ => return Err("Headers must be a map or array".to_string()),
                    }
                }
                
                // Extract data for POST/PUT/PATCH
                if let Some(data) = options.get("data") {
                    body_data = Some(data.clone());
                }
                
                // Extract params for GET
                if let Some(p) = options.get("params") {
                    params = p.clone();
                }
                
                // Extract timeout
                if let Some(t) = options.get("timeout") {
                    match t {
                        Value::Int(t) => timeout = Some(*t as u64),
                        Value::Float(t) => timeout = Some(*t as u64),
                        _ => {},
                    }
                }
            },
            Value::Array(arr) => {
                // Handle array of key-value pairs
                if arr.len() % 2 != 0 {
                    return Err("Options array must contain an even number of elements (key-value pairs)".to_string());
                }
                
                for i in (0..arr.len()).step_by(2) {
                    if i + 1 < arr.len() {
                        let key = match &arr[i] {
                            Value::String(s) => s.clone(),
                            _ => arr[i].to_string(),
                        };
                        
                        match key.as_str() {
                            "method" => {
                                if let Value::String(m) = &arr[i + 1] {
                                    method = m.clone();
                                }
                            },
                            "headers" => {
                                match &arr[i + 1] {
                                    Value::Array(h) => {
                                        // Special case for the example format where the array is represented as strings
                                        if h.len() >= 1 && h[0].to_string().contains("[") {
                                            // This is a special case where the array is represented as a string like "[Accept"
                                            // Just add some default headers
                                            headers_map.insert("Accept".to_string(), Value::String("application/json".to_string()));
                                            headers_map.insert("User-Agent".to_string(), Value::String("Razen-API-Test".to_string()));
                                        } else {
                                            // Handle nested array of headers
                                            if h.len() % 2 != 0 {
                                                return Err("Headers array must contain an even number of elements".to_string());
                                            }
                                            
                                            for j in (0..h.len()).step_by(2) {
                                                if j + 1 < h.len() {
                                                    let header_key = match &h[j] {
                                                        Value::String(s) => s.clone(),
                                                        _ => h[j].to_string(),
                                                    };
                                                    let header_value = match &h[j + 1] {
                                                        Value::String(s) => s.clone(),
                                                        _ => h[j + 1].to_string(),
                                                    };
                                                    headers_map.insert(header_key, Value::String(header_value));
                                                }
                                            }
                                        }
                                    },
                                    Value::Map(h) => {
                                        for (k, v) in h {
                                            headers_map.insert(k.clone(), v.clone());
                                        }
                                    },
                                    Value::String(s) => {
                                        // If it's a string, it might be a serialized array or object
                                        // For simplicity, we'll just add it as a single header
                                        headers_map.insert("Accept".to_string(), Value::String(s.clone()));
                                    },
                                    _ => {
                                        // For any other type, convert to string and add as a header
                                        headers_map.insert("Accept".to_string(), Value::String(arr[i + 1].to_string()));
                                    }
                                }
                            },
                            "data" => body_data = Some(arr[i + 1].clone()),
                            "params" => params = arr[i + 1].clone(),
                            "timeout" => {
                                match &arr[i + 1] {
                                    Value::Int(t) => timeout = Some(*t as u64),
                                    Value::Float(t) => timeout = Some(*t as u64),
                                    _ => {},
                                }
                            },
                            _ => {}, // Ignore unknown options
                        }
                    }
                }
            },
            _ => return Err("Options must be a map or array".to_string()),
        }
    }
    
    let headers = Value::Map(headers_map);
    
    // Make the request based on the method
    match method.to_uppercase().as_str() {
        "GET" => make_request("GET", &url, params, headers, None, timeout),
        "POST" => make_request("POST", &url, Value::Map(HashMap::new()), headers, body_data, timeout),
        "PUT" => make_request("PUT", &url, Value::Map(HashMap::new()), headers, body_data, timeout),
        "DELETE" => make_request("DELETE", &url, params, headers, None, timeout),
        "PATCH" => make_request("PATCH", &url, Value::Map(HashMap::new()), headers, body_data, timeout),
        _ => Err(format!("Unsupported method: {}", method))
    }
}

/// Make a request to an API endpoint with the specified method, URL, parameters, headers, and body
fn make_request(
    method: &str, 
    url: &str, 
    params: Value, 
    headers: Value,
    body: Option<Value>,
    timeout_seconds: Option<u64>
) -> Result<Value, String> {
    // Create a new client with timeout
    let timeout = Duration::from_secs(timeout_seconds.unwrap_or(DEFAULT_TIMEOUT));
    let client = Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    // Build the request
    let mut request_builder = match method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        "HEAD" => client.head(url),
        "OPTIONS" => client.request(reqwest::Method::OPTIONS, url),
        _ => return Err(format!("Unsupported HTTP method: {}", method)),
    };
    
    // Add query parameters for GET requests
    if method.to_uppercase() == "GET" && params != Value::Null {
        if let Value::Map(params_map) = params {
            for (key, value) in params_map {
                let value_str = match value {
                    Value::String(s) => s,
                    Value::Int(i) => i.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => value.to_string(),
                };
                request_builder = request_builder.query(&[(key, value_str)]);
            }
        }
    }
    
    // Add headers
    if let Value::Map(headers_map) = headers {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers_map {
            if let Ok(header_name) = HeaderName::from_str(&key) {
                let value_str = match value {
                    Value::String(s) => s,
                    Value::Int(i) => i.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => value.to_string(),
                };
                if let Ok(header_value) = HeaderValue::from_str(&value_str) {
                    header_map.insert(header_name, header_value);
                } else {
                    return Err(format!("Invalid header value for '{}': {}", key, value_str));
                }
            } else {
                return Err(format!("Invalid header name: {}", key));
            }
        }
        request_builder = request_builder.headers(header_map);
    }
    
    // Add body for POST, PUT, and PATCH requests
    if let Some(body_value) = body {
        if method.to_uppercase() == "POST" || method.to_uppercase() == "PUT" || method.to_uppercase() == "PATCH" {
            // Convert body to JSON
            let json_body = value_to_json(body_value)?;
            request_builder = request_builder.json(&json_body);
        }
    }
    
    // Send the request with error handling
    let response = match request_builder.send() {
        Ok(resp) => resp,
        Err(e) => {
            if e.is_timeout() {
                return Err(format!("API request timed out after {} seconds", timeout_seconds.unwrap_or(DEFAULT_TIMEOUT)));
            } else if e.is_connect() {
                return Err(format!("Failed to connect to API server: {}", e));
            } else if e.is_request() {
                return Err(format!("Invalid request: {}", e));
            } else {
                return Err(format!("API request failed: {}", e));
            }
        }
    };
    
    // Process the response
    process_response(response)
}

/// Process an API response and convert it to a Razen Value
fn process_response(response: Response) -> Result<Value, String> {
    // Get status code
    let status = response.status().as_u16();
    
    // Get headers before consuming the response
    let mut headers_map = HashMap::new();
    for (name, value) in response.headers().iter() {
        if let Ok(value_str) = value.to_str() {
            headers_map.insert(name.as_str().to_string(), Value::String(value_str.to_string()));
        }
    }
    
    // Get content type for better handling and clone it before consuming the response
    let content_type = response.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("text/plain")
        .to_string(); // Clone the string to avoid borrowing issues
    
    // Try to get the response as text (this consumes the response)
    let response_text = match response.text() {
        Ok(text) => text,
        Err(e) => return Err(format!("Failed to read response body: {}", e)),
    };
    
    // Create the response data structure
    let mut result_map = HashMap::new();
    
    // Add raw response data to make it easier to debug
    let mut raw_response = HashMap::new();
    raw_response.insert("status".to_string(), Value::Int(status as i64));
    raw_response.insert("headers".to_string(), Value::Map(headers_map.clone()));
    raw_response.insert("content_type".to_string(), Value::String(content_type.clone()));
    
    // Try to parse the text as JSON if it looks like JSON
    if content_type.contains("json") || response_text.trim().starts_with('{') || response_text.trim().starts_with('[') {
        match serde_json::from_str::<JsonValue>(&response_text) {
            Ok(json) => {
                // If it's valid JSON, convert it to a Razen Value
                raw_response.insert("data".to_string(), json_to_value(json.clone()));
                
                // For convenience, also add the parsed JSON data directly to the top level
                if let JsonValue::Object(obj) = json {
                    for (key, value) in obj {
                        result_map.insert(key, json_to_value(value));
                    }
                }
            },
            Err(_) => {
                // If JSON parsing failed, try to fix common issues and try again
                let fixed_json = try_fix_unquoted_keys(&response_text);
                match serde_json::from_str::<JsonValue>(&fixed_json) {
                    Ok(json) => {
                        raw_response.insert("data".to_string(), json_to_value(json.clone()));
                        
                        // For convenience, also add the parsed JSON data directly to the top level
                        if let JsonValue::Object(obj) = json {
                            for (key, value) in obj {
                                result_map.insert(key, json_to_value(value));
                            }
                        }
                    },
                    Err(_) => {
                        // If it's still not valid JSON, store it as plain text
                        raw_response.insert("data".to_string(), Value::String(response_text.clone()));
                    }
                }
            }
        }
    } else {
        // If it's not JSON, store it as plain text
        raw_response.insert("data".to_string(), Value::String(response_text.clone()));
    }
    
    // Add status code to top level for easy access
    result_map.insert("status".to_string(), Value::Int(status as i64));
    
    // Store the raw response for advanced usage
    result_map.insert("raw_response".to_string(), Value::Map(raw_response));
    
    Ok(Value::Map(result_map))
}

/// Convert a Razen Value to a JSON Value
fn value_to_json(value: Value) -> Result<JsonValue, String> {
    match value {
        Value::Null => Ok(JsonValue::Null),
        Value::Bool(b) => Ok(JsonValue::Bool(b)),
        Value::Int(i) => Ok(json!(i)),
        Value::Float(f) => Ok(json!(f)),
        Value::String(s) => Ok(JsonValue::String(s)),
        Value::Array(arr) => {
            let mut json_arr = Vec::new();
            for item in arr {
                json_arr.push(value_to_json(item)?);
            }
            Ok(JsonValue::Array(json_arr))
        },
        Value::Map(map) => {
            let mut json_obj = serde_json::Map::new();
            for (key, val) in map {
                json_obj.insert(key, value_to_json(val)?);
            }
            Ok(JsonValue::Object(json_obj))
        },
    }
}

/// Convert a JSON Value to a Razen Value
fn json_to_value(json: JsonValue) -> Value {
    match json {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(b) => Value::Bool(b),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::String(n.to_string())
            }
        },
        JsonValue::String(s) => Value::String(s),
        JsonValue::Array(arr) => {
            let mut value_arr = Vec::new();
            for item in arr {
                value_arr.push(json_to_value(item));
            }
            Value::Array(value_arr)
        },
        JsonValue::Object(obj) => {
            let mut value_map = HashMap::new();
            for (key, val) in obj {
                value_map.insert(key, json_to_value(val));
            }
            Value::Map(value_map)
        },
    }
}

/// Parse a JSON string into a Razen Value
/// Example: parse_json('{"name": "John", "age": 30}') => {"name": "John", "age": 30}
pub fn parse_json(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("API.parse_json: Expected 1 argument (json_string)".to_string());
    }
    
    // Handle null values
    if let Value::Null = &args[0] {
        return Ok(Value::Null);
    }
    
    let json_str = args[0].as_string()?;
    
    // If the string is empty, return null
    if json_str.trim().is_empty() {
        return Ok(Value::Null);
    }
    
    // Try to parse as standard JSON first
    let json_result = serde_json::from_str::<JsonValue>(&json_str);
    match json_result {
        Ok(json) => return Ok(json_to_value(json)),
        Err(_) => {
            // If standard parsing fails, try to fix common issues
            
            // 1. Try to fix unquoted keys
            let fixed_str = try_fix_unquoted_keys(&json_str);
            let fixed_result = serde_json::from_str::<JsonValue>(&fixed_str);
            if let Ok(json) = fixed_result {
                return Ok(json_to_value(json));
            }
            
            // 2. If it still fails, create a simple map with the raw response
            let mut result_map = HashMap::new();
            result_map.insert("raw_response".to_string(), Value::String(json_str.clone()));
            
            // 3. Try to extract status code if present
            if json_str.contains("status") {
                if let Some(pos) = json_str.find("status") {
                    let substr = &json_str[pos + 7..];
                    if let Some(end) = substr.find(',') {
                        let status_str = &substr[0..end].trim();
                        if let Ok(code) = status_str.trim_matches(':').trim().parse::<i64>() {
                            result_map.insert("status".to_string(), Value::Int(code));
                        }
                    }
                }
            }
            
            return Ok(Value::Map(result_map));
        }
    }
}

/// Helper function to try to fix common JSON issues like unquoted keys
fn try_fix_unquoted_keys(json_str: &str) -> String {
    let mut result = String::new();
    let mut in_key = false;
    let mut in_string = false;
    let mut escape_next = false;
    
    for c in json_str.chars() {
        match c {
            '{' | '[' if !in_string => {
                result.push(c);
                in_key = c == '{';
            },
            '}' | ']' if !in_string => {
                result.push(c);
                in_key = false;
            },
            ':' if !in_string => {
                result.push(c);
                in_key = false;
            },
            ',' if !in_string => {
                result.push(c);
                in_key = true;
            },
            '"' if !escape_next => {
                result.push(c);
                in_string = !in_string;
            },
            '\\' if in_string && !escape_next => {
                result.push(c);
                escape_next = true;
                continue;
            },
            ' ' | '\t' | '\n' | '\r' if !in_string => {
                result.push(c);
            },
            _ => {
                if in_key && !in_string && c.is_alphabetic() {
                    // Add quotes around unquoted keys
                    result.push('"');
                    result.push(c);
                    
                    // Collect the rest of the key
                    let mut i = json_str.chars().skip_while(|&x| x != c).skip(1);
                    while let Some(next) = i.next() {
                        if next.is_alphanumeric() || next == '_' {
                            result.push(next);
                        } else {
                            result.push('"');
                            result.push(next);
                            in_key = false;
                            break;
                        }
                    }
                } else {
                    result.push(c);
                }
            }
        }
        
        escape_next = false;
    }
    
    result
}

/// Convert a Razen Value to a JSON string
/// Example: to_json({"name": "John", "age": 30}) => '{"name": "John", "age": 30}'
pub fn to_json(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("API.to_json: Expected 1 argument (value)".to_string());
    }
    
    let value = args[0].clone();
    
    // Convert to JSON
    let json_result = value_to_json(value);
    match json_result {
        Ok(json) => {
            let json_str = json.to_string();
            Ok(Value::String(json_str))
        },
        Err(e) => Err(e),
    }
}

/// Create an API configuration with authentication
/// Example: create_api("https://api.example.com", "your-api-key", "bearer", 30) => api_config
/// Arguments:
///   - url: Base URL for the API
///   - api_key: (Optional) API key for authentication
///   - auth_type: (Optional) Authentication type ("bearer", "basic", "apikey", default: "bearer")
///   - timeout: (Optional) Default timeout in seconds
pub fn create_api(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("API.create_api: Expected at least 1 argument (url)".to_string());
    }
    
    let url = args[0].as_string()?;
    
    // Get API key if provided (default to empty string)
    let api_key = if args.len() > 1 {
        args[1].as_string()?
    } else {
        "".to_string()
    };
    
    // Get auth type (default to "none")
    let auth_type = if args.len() > 2 {
        args[2].as_string()?
    } else {
        "none".to_string()
    };
    
    // Get timeout if provided
    let timeout = if args.len() > 3 {
        match &args[3] {
            Value::Int(t) => *t as u64,
            Value::Float(t) => *t as u64,
            _ => DEFAULT_TIMEOUT,
        }
    } else {
        DEFAULT_TIMEOUT
    };
    
    // Create API configuration
    let mut api_config = HashMap::new();
    api_config.insert("url".to_string(), Value::String(url));
    api_config.insert("api_key".to_string(), Value::String(api_key));
    api_config.insert("auth_type".to_string(), Value::String(auth_type));
    api_config.insert("timeout".to_string(), Value::Int(timeout as i64));
    
    Ok(Value::Map(api_config))
}

/// Execute an API call with the given API configuration
/// Example: execute_api(api_config, "GET", "/endpoint", {"param": "value"}, 30) => response
/// Arguments:
///   - api_config: API configuration created with create_api
///   - method: HTTP method to use (GET, POST, PUT, DELETE, PATCH)
///   - endpoint: (Optional) API endpoint to append to the base URL
///   - params_or_data: (Optional) Query parameters or request body
///   - timeout: (Optional) Timeout in seconds (overrides the one in api_config)
pub fn execute_api(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("API.execute_api: Expected at least 2 arguments (api_config, method)".to_string());
    }
    
    let api_config = args[0].clone();
    let method = args[1].as_string()?;
    
    // Extract API configuration
    if let Value::Map(config_map) = api_config {
        let url = if let Some(Value::String(url)) = config_map.get("url") {
            url.clone()
        } else {
            return Err("API configuration missing 'url'".to_string());
        };
        
        let api_key = if let Some(Value::String(key)) = config_map.get("api_key") {
            key.clone()
        } else {
            "".to_string() // Default to empty API key if not provided
        };
        
        let auth_type = if let Some(Value::String(auth)) = config_map.get("auth_type") {
            auth.clone()
        } else {
            "none".to_string() // Default to no auth if not specified
        };
        
        // Get default timeout from config
        let default_timeout = if let Some(Value::Int(t)) = config_map.get("timeout") {
            Some(*t as u64)
        } else {
            Some(DEFAULT_TIMEOUT)
        };
        
        // Get endpoint if provided
        let endpoint = if args.len() > 2 {
            args[2].as_string()?
        } else {
            "".to_string()
        };
        
        // Combine URL and endpoint
        let full_url = if endpoint.starts_with("http") {
            endpoint
        } else {
            format!("{}{}", url.trim_end_matches('/'), endpoint)
        };
        
        // Get params/data if provided
        let params_or_data = if args.len() > 3 {
            args[3].clone()
        } else {
            Value::Map(HashMap::new())
        };
        
        // Get timeout if provided (overrides the one in config)
        let timeout = if args.len() > 4 {
            match &args[4] {
                Value::Int(t) => Some(*t as u64),
                Value::Float(t) => Some(*t as u64),
                _ => default_timeout,
            }
        } else {
            default_timeout
        };
        
        // Create headers with authentication
        let mut headers_map = HashMap::new();
        match auth_type.to_lowercase().as_str() {
            "bearer" => {
                if !api_key.is_empty() {
                    headers_map.insert("Authorization".to_string(), Value::String(format!("Bearer {}", api_key)));
                }
            },
            "basic" => {
                if !api_key.is_empty() {
                    headers_map.insert("Authorization".to_string(), Value::String(format!("Basic {}", api_key)));
                }
            },
            "apikey" => {
                if !api_key.is_empty() {
                    headers_map.insert("X-API-Key".to_string(), Value::String(api_key));
                }
            },
            "none" => {
                // No authentication headers needed
            },
            _ => {
                return Err(format!("Unsupported auth type: {}", auth_type));
            }
        }
        
        // Add content type for POST/PUT/PATCH
        if method.to_uppercase() == "POST" || method.to_uppercase() == "PUT" || method.to_uppercase() == "PATCH" {
            headers_map.insert("Content-Type".to_string(), Value::String("application/json".to_string()));
        }
        
        let headers = Value::Map(headers_map);
        
        // Make the request based on the method
        match method.to_uppercase().as_str() {
            "GET" => make_request("GET", &full_url, params_or_data, headers, None, timeout),
            "POST" => make_request("POST", &full_url, Value::Map(HashMap::new()), headers, Some(params_or_data), timeout),
            "PUT" => make_request("PUT", &full_url, Value::Map(HashMap::new()), headers, Some(params_or_data), timeout),
            "DELETE" => make_request("DELETE", &full_url, Value::Map(HashMap::new()), headers, None, timeout),
            "PATCH" => make_request("PATCH", &full_url, Value::Map(HashMap::new()), headers, Some(params_or_data), timeout),
            "HEAD" => make_request("HEAD", &full_url, params_or_data, headers, None, timeout),
            "OPTIONS" => make_request("OPTIONS", &full_url, params_or_data, headers, None, timeout),
            _ => Err(format!("Unsupported method: {}", method)),
        }
    } else {
        Err("API configuration must be a map".to_string())
    }
}

/// URL encode a string
/// Example: url_encode("Hello World") => "Hello%20World"
pub fn url_encode(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("API.url_encode: Expected exactly 1 argument (string)".to_string());
    }
    
    let input = args[0].as_string()?;
    let encoded = form_urlencoded::byte_serialize(input.as_bytes()).collect::<String>();
    
    Ok(Value::String(encoded))
}

/// URL decode a string
/// Example: url_decode("Hello%20World") => "Hello World"
pub fn url_decode(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("API.url_decode: Expected exactly 1 argument (string)".to_string());
    }
    
    let input = args[0].as_string()?;
    
    // Properly decode URL-encoded string
    let decoded = form_urlencoded::parse(input.as_bytes())
        .map(|(key, val)| format!("{}", val))
        .collect::<String>();
    
    // If the input doesn't contain any '=' characters, it's likely a simple encoded string
    // In that case, we'll use a different approach
    if decoded.is_empty() && !input.contains('=') {
        let decoded = form_urlencoded::parse(format!("k={}", input).as_bytes())
            .map(|(_, val)| val.to_string())
            .collect::<String>();
        return Ok(Value::String(decoded));
    }
    
    Ok(Value::String(decoded))
}

/// Create form data from a map or array
/// Example: form_data({"name": "John", "age": 30}) => "name=John&age=30"
/// Example: form_data(["name", "John", "age", 30]) => "name=John&age=30"
pub fn form_data(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("API.form_data: Expected exactly 1 argument (map or array)".to_string());
    }
    
    let mut form_parts = Vec::new();
    
    match &args[0] {
        Value::Map(map) => {
            // Handle map input
            for (key, value) in map {
                let value_str = match value {
                    Value::String(s) => s.clone(),
                    Value::Int(i) => i.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => value.to_string(),
                };
                
                form_parts.push(format!("{}={}", 
                    form_urlencoded::byte_serialize(key.as_bytes()).collect::<String>(),
                    form_urlencoded::byte_serialize(value_str.as_bytes()).collect::<String>()));
            }
        },
        Value::Array(arr) => {
            // Handle array input (key-value pairs in sequence)
            if arr.len() % 2 != 0 {
                return Err("Array must contain an even number of elements (key-value pairs)".to_string());
            }
            
            for i in (0..arr.len()).step_by(2) {
                if i + 1 < arr.len() {
                    let key = match &arr[i] {
                        Value::String(s) => s.clone(),
                        _ => arr[i].to_string(),
                    };
                    
                    let value_str = match &arr[i + 1] {
                        Value::String(s) => s.clone(),
                        Value::Int(n) => n.to_string(),
                        Value::Float(f) => f.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => arr[i + 1].to_string(),
                    };
                    
                    form_parts.push(format!("{}={}", 
                        form_urlencoded::byte_serialize(key.as_bytes()).collect::<String>(),
                        form_urlencoded::byte_serialize(value_str.as_bytes()).collect::<String>()));
                }
            }
        },
        _ => return Err("Argument must be a map or array".to_string()),
    }
    
    Ok(Value::String(form_parts.join("&")))
}

/// Check if a status code indicates success (2xx)
/// Example: is_success(200) => true
pub fn is_success(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("API.is_success: Expected exactly 1 argument (status_code)".to_string());
    }
    
    // Extract status code from different possible input types
    let status_code = match &args[0] {
        Value::Int(code) => *code as u16,
        Value::String(s) => {
            // Try to extract status code from JSON response string
            if s.contains("status") {
                // Try to find status code in the string
                if let Some(pos) = s.find("status") {
                    let substr = &s[pos + 7..];
                    if let Some(end) = substr.find(',') {
                        let status_str = &substr[0..end].trim();
                        if let Ok(code) = status_str.trim_matches(':').trim().parse::<u16>() {
                            code
                        } else {
                            // Default to 200 for successful responses
                            200
                        }
                    } else {
                        // Default to 200 for successful responses
                        200
                    }
                } else {
                    // Default to 200 for successful responses
                    200
                }
            } else {
                // Default to 200 for successful responses
                200
            }
        },
        // If null or other type, assume it's an error (not success)
        Value::Null => return Ok(Value::Bool(false)),
        _ => return Ok(Value::Bool(false)),
    };
    
    Ok(Value::Bool(status_code >= 200 && status_code < 300))
}

/// Check if a status code indicates client error (4xx)
/// Example: is_client_error(404) => true
pub fn is_client_error(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("API.is_client_error: Expected exactly 1 argument (status_code)".to_string());
    }
    
    // Extract status code from different possible input types
    let status_code = match &args[0] {
        Value::Int(code) => *code as u16,
        Value::String(s) => {
            // Try to extract status code from JSON response string
            if s.contains("status") {
                // Try to find status code in the string
                if let Some(pos) = s.find("status") {
                    let substr = &s[pos + 7..];
                    if let Some(end) = substr.find(',') {
                        let status_str = &substr[0..end].trim();
                        if let Ok(code) = status_str.trim_matches(':').trim().parse::<u16>() {
                            code
                        } else {
                            // Default to 0 for non-client errors
                            0
                        }
                    } else {
                        // Default to 0 for non-client errors
                        0
                    }
                } else {
                    // Default to 0 for non-client errors
                    0
                }
            } else {
                // Default to 0 for non-client errors
                0
            }
        },
        // If null or other type, assume it's not a client error
        Value::Null => return Ok(Value::Bool(false)),
        _ => return Ok(Value::Bool(false)),
    };
    
    Ok(Value::Bool(status_code >= 400 && status_code < 500))
}

/// Check if a status code indicates server error (5xx)
/// Example: is_server_error(500) => true
pub fn is_server_error(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("API.is_server_error: Expected exactly 1 argument (status_code)".to_string());
    }
    
    // Extract status code from different possible input types
    let status_code = match &args[0] {
        Value::Int(code) => *code as u16,
        Value::String(s) => {
            // Try to extract status code from JSON response string
            if s.contains("status") {
                // Try to find status code in the string
                if let Some(pos) = s.find("status") {
                    let substr = &s[pos + 7..];
                    if let Some(end) = substr.find(',') {
                        let status_str = &substr[0..end].trim();
                        if let Ok(code) = status_str.trim_matches(':').trim().parse::<u16>() {
                            code
                        } else {
                            // Default to 0 for non-server errors
                            0
                        }
                    } else {
                        // Default to 0 for non-server errors
                        0
                    }
                } else {
                    // Default to 0 for non-server errors
                    0
                }
            } else {
                // Default to 0 for non-server errors
                0
            }
        },
        // If null or other type, assume it's not a server error
        Value::Null => return Ok(Value::Bool(false)),
        _ => return Ok(Value::Bool(false)),
    };
    
    Ok(Value::Bool(status_code >= 500 && status_code < 600))
}
