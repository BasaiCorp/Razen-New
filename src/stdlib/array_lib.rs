// src/stdlib/array_lib.rs
//! Array manipulation standard library - Native Rust implementation

use crate::backend::execution::value::Value;

pub fn has_function(name: &str) -> bool {
    matches!(name, "push" | "pop" | "first" | "last" | "reverse" | "contains" | 
             "sum" | "avg" | "max" | "min" | "sort" | "len")
}

pub fn get_function_list() -> Vec<&'static str> {
    vec!["push", "pop", "first", "last", "reverse", "contains", "sum", "avg", "max", "min", "sort", "len"]
}

pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "push" => push(args),
        "pop" => pop(args),
        "first" => first(args),
        "last" => last(args),
        "reverse" => reverse(args),
        "contains" => contains(args),
        "sum" => sum(args),
        "avg" => avg(args),
        "max" => max(args),
        "min" => min(args),
        "sort" => sort(args),
        "len" => len(args),
        _ => Err(format!("Unknown array function: {}", name)),
    }
}

fn push(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("push() takes exactly 2 arguments".to_string());
    }
    match &args[0] {
        Value::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(args[1].clone());
            Ok(Value::Array(new_arr))
        }
        _ => Err("push() requires an array as first argument".to_string()),
    }
}

fn pop(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("pop() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.pop().ok_or("Cannot pop from empty array".to_string())
        }
        _ => Err("pop() requires an array argument".to_string()),
    }
}

fn first(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("first() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Array(arr) => {
            arr.first().cloned().ok_or("Array is empty".to_string())
        }
        _ => Err("first() requires an array argument".to_string()),
    }
}

fn last(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("last() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Array(arr) => {
            arr.last().cloned().ok_or("Array is empty".to_string())
        }
        _ => Err("last() requires an array argument".to_string()),
    }
}

fn reverse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("reverse() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Array(arr) => {
            let mut reversed = arr.clone();
            reversed.reverse();
            Ok(Value::Array(reversed))
        }
        _ => Err("reverse() requires an array argument".to_string()),
    }
}

fn contains(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("contains() takes exactly 2 arguments".to_string());
    }
    match &args[0] {
        Value::Array(arr) => {
            let found = arr.iter().any(|v| v.to_string() == args[1].to_string());
            Ok(Value::Boolean(found))
        }
        _ => Err("contains() requires an array as first argument".to_string()),
    }
}

fn sum(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sum() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Array(arr) => {
            let mut total = 0.0;
            for val in arr {
                match val {
                    Value::Integer(n) => total += *n as f64,
                    Value::Number(n) => total += n,
                    _ => return Err("sum() requires numeric array elements".to_string()),
                }
            }
            Ok(Value::Number(total))
        }
        _ => Err("sum() requires an array argument".to_string()),
    }
}

fn avg(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("avg() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok(Value::Number(0.0));
            }
            let sum_result = sum(vec![Value::Array(arr.clone())])?;
            match sum_result {
                Value::Number(s) => Ok(Value::Number(s / arr.len() as f64)),
                _ => Err("avg() calculation error".to_string()),
            }
        }
        _ => Err("avg() requires an array argument".to_string()),
    }
}

fn max(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("max() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err("Cannot find max of empty array".to_string());
            }
            let mut max_val = match &arr[0] {
                Value::Integer(n) => *n as f64,
                Value::Number(n) => *n,
                _ => return Err("max() requires numeric array elements".to_string()),
            };
            for val in arr.iter().skip(1) {
                let num = match val {
                    Value::Integer(n) => *n as f64,
                    Value::Number(n) => *n,
                    _ => return Err("max() requires numeric array elements".to_string()),
                };
                if num > max_val {
                    max_val = num;
                }
            }
            Ok(Value::Number(max_val))
        }
        _ => Err("max() requires an array argument".to_string()),
    }
}

fn min(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("min() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err("Cannot find min of empty array".to_string());
            }
            let mut min_val = match &arr[0] {
                Value::Integer(n) => *n as f64,
                Value::Number(n) => *n,
                _ => return Err("min() requires numeric array elements".to_string()),
            };
            for val in arr.iter().skip(1) {
                let num = match val {
                    Value::Integer(n) => *n as f64,
                    Value::Number(n) => *n,
                    _ => return Err("min() requires numeric array elements".to_string()),
                };
                if num < min_val {
                    min_val = num;
                }
            }
            Ok(Value::Number(min_val))
        }
        _ => Err("min() requires an array argument".to_string()),
    }
}

fn sort(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sort() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Array(arr) => {
            let mut sorted = arr.clone();
            sorted.sort_by(|a, b| {
                match (a, b) {
                    (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
                    (Value::Number(x), Value::Number(y)) => {
                        x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    (Value::String(x), Value::String(y)) => x.cmp(y),
                    _ => std::cmp::Ordering::Equal,
                }
            });
            Ok(Value::Array(sorted))
        }
        _ => Err("sort() requires an array argument".to_string()),
    }
}

fn len(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("len() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Array(arr) => Ok(Value::Integer(arr.len() as i64)),
        _ => Err("len() requires an array argument".to_string()),
    }
}
