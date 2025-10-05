// src/stdlib/http_lib.rs
//! HTTP client standard library

use crate::backend::execution::value::Value;
use std::collections::HashMap;

pub fn has_function(name: &str) -> bool {
    matches!(name, 
        "get" | "post" | "put" | "delete" | "patch" |
        "request" | "parse_url" | "encode_url" | "decode_url"
    )
}

pub fn get_function_list() -> Vec<&'static str> {
    vec![
        "get", "post", "put", "delete", "patch",
        "request", "parse_url", "encode_url", "decode_url"
    ]
}

pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "get" => get(args),
        "post" => post(args),
        "put" => put(args),
        "delete" => delete(args),
        "patch" => patch(args),
        "request" => request(args),
        "parse_url" => parse_url(args),
        "encode_url" => encode_url(args),
        "decode_url" => decode_url(args),
        _ => Err(format!("Unknown http function: {}", name)),
    }
}

fn get(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("get() requires at least 1 argument (url)".to_string());
    }
    
    match &args[0] {
        Value::String(url) => {
            match ureq::get(url).call() {
                Ok(response) => {
                    match response.into_string() {
                        Ok(body) => {
                            let mut result = HashMap::new();
                            result.insert("status".to_string(), Value::Integer(200));
                            result.insert("body".to_string(), Value::String(body));
                            Ok(Value::Map(result))
                        }
                        Err(e) => Err(format!("Failed to read response: {}", e)),
                    }
                }
                Err(e) => Err(format!("HTTP GET failed: {}", e)),
            }
        }
        _ => Err("get() requires a string URL".to_string()),
    }
}

fn post(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("post() requires 2 arguments (url, body)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(url), Value::String(body)) => {
            match ureq::post(url)
                .set("Content-Type", "application/json")
                .send_string(body)
            {
                Ok(response) => {
                    match response.into_string() {
                        Ok(response_body) => {
                            let mut result = HashMap::new();
                            result.insert("status".to_string(), Value::Integer(200));
                            result.insert("body".to_string(), Value::String(response_body));
                            Ok(Value::Map(result))
                        }
                        Err(e) => Err(format!("Failed to read response: {}", e)),
                    }
                }
                Err(e) => Err(format!("HTTP POST failed: {}", e)),
            }
        }
        _ => Err("post() requires string arguments (url, body)".to_string()),
    }
}

fn put(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("put() requires 2 arguments (url, body)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(url), Value::String(body)) => {
            match ureq::put(url)
                .set("Content-Type", "application/json")
                .send_string(body)
            {
                Ok(response) => {
                    match response.into_string() {
                        Ok(response_body) => {
                            let mut result = HashMap::new();
                            result.insert("status".to_string(), Value::Integer(200));
                            result.insert("body".to_string(), Value::String(response_body));
                            Ok(Value::Map(result))
                        }
                        Err(e) => Err(format!("Failed to read response: {}", e)),
                    }
                }
                Err(e) => Err(format!("HTTP PUT failed: {}", e)),
            }
        }
        _ => Err("put() requires string arguments (url, body)".to_string()),
    }
}

fn delete(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("delete() requires at least 1 argument (url)".to_string());
    }
    
    match &args[0] {
        Value::String(url) => {
            match ureq::delete(url).call() {
                Ok(response) => {
                    match response.into_string() {
                        Ok(body) => {
                            let mut result = HashMap::new();
                            result.insert("status".to_string(), Value::Integer(200));
                            result.insert("body".to_string(), Value::String(body));
                            Ok(Value::Map(result))
                        }
                        Err(e) => Err(format!("Failed to read response: {}", e)),
                    }
                }
                Err(e) => Err(format!("HTTP DELETE failed: {}", e)),
            }
        }
        _ => Err("delete() requires a string URL".to_string()),
    }
}

fn patch(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("patch() requires 2 arguments (url, body)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(url), Value::String(body)) => {
            match ureq::patch(url)
                .set("Content-Type", "application/json")
                .send_string(body)
            {
                Ok(response) => {
                    match response.into_string() {
                        Ok(response_body) => {
                            let mut result = HashMap::new();
                            result.insert("status".to_string(), Value::Integer(200));
                            result.insert("body".to_string(), Value::String(response_body));
                            Ok(Value::Map(result))
                        }
                        Err(e) => Err(format!("Failed to read response: {}", e)),
                    }
                }
                Err(e) => Err(format!("HTTP PATCH failed: {}", e)),
            }
        }
        _ => Err("patch() requires string arguments (url, body)".to_string()),
    }
}

fn request(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("request() requires at least 2 arguments (method, url)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::String(method), Value::String(url)) => {
            let method_upper = method.to_uppercase();
            match method_upper.as_str() {
                "GET" => get(vec![Value::String(url.clone())]),
                "POST" => {
                    if args.len() >= 3 {
                        post(vec![Value::String(url.clone()), args[2].clone()])
                    } else {
                        post(vec![Value::String(url.clone()), Value::String(String::new())])
                    }
                }
                "PUT" => {
                    if args.len() >= 3 {
                        put(vec![Value::String(url.clone()), args[2].clone()])
                    } else {
                        put(vec![Value::String(url.clone()), Value::String(String::new())])
                    }
                }
                "DELETE" => delete(vec![Value::String(url.clone())]),
                "PATCH" => {
                    if args.len() >= 3 {
                        patch(vec![Value::String(url.clone()), args[2].clone()])
                    } else {
                        patch(vec![Value::String(url.clone()), Value::String(String::new())])
                    }
                }
                _ => Err(format!("Unsupported HTTP method: {}", method)),
            }
        }
        _ => Err("request() requires string arguments (method, url)".to_string()),
    }
}

fn parse_url(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("parse_url() requires 1 argument (url)".to_string());
    }
    
    match &args[0] {
        Value::String(url_str) => {
            match url::Url::parse(url_str) {
                Ok(url) => {
                    let mut result = HashMap::new();
                    result.insert("scheme".to_string(), Value::String(url.scheme().to_string()));
                    result.insert("host".to_string(), Value::String(url.host_str().unwrap_or("").to_string()));
                    result.insert("port".to_string(), Value::Integer(url.port().unwrap_or(0) as i64));
                    result.insert("path".to_string(), Value::String(url.path().to_string()));
                    result.insert("query".to_string(), Value::String(url.query().unwrap_or("").to_string()));
                    Ok(Value::Map(result))
                }
                Err(e) => Err(format!("Failed to parse URL: {}", e)),
            }
        }
        _ => Err("parse_url() requires a string argument".to_string()),
    }
}

fn encode_url(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("encode_url() requires 1 argument".to_string());
    }
    
    match &args[0] {
        Value::String(s) => {
            let encoded = urlencoding::encode(s);
            Ok(Value::String(encoded.to_string()))
        }
        _ => Err("encode_url() requires a string argument".to_string()),
    }
}

fn decode_url(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("decode_url() requires 1 argument".to_string());
    }
    
    match &args[0] {
        Value::String(s) => {
            match urlencoding::decode(s) {
                Ok(decoded) => Ok(Value::String(decoded.to_string())),
                Err(e) => Err(format!("Failed to decode URL: {}", e)),
            }
        }
        _ => Err("decode_url() requires a string argument".to_string()),
    }
}
