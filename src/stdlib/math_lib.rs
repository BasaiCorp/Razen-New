// src/stdlib/math_lib.rs
//! Math functions standard library - Native Rust implementation

use crate::backend::execution::value::Value;

pub fn has_function(name: &str) -> bool {
    matches!(name, "abs" | "max" | "min" | "pow" | "sqrt" | "floor" | "ceil" | 
             "round" | "sin" | "cos" | "tan" | "pi" | "e")
}

pub fn get_function_list() -> Vec<&'static str> {
    vec!["abs", "max", "min", "pow", "sqrt", "floor", "ceil", "round", "sin", "cos", "tan", "pi", "e"]
}

pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "abs" => abs(args),
        "max" => max(args),
        "min" => min(args),
        "pow" => pow(args),
        "sqrt" => sqrt(args),
        "floor" => floor(args),
        "ceil" => ceil(args),
        "round" => round(args),
        "sin" => sin(args),
        "cos" => cos(args),
        "tan" => tan(args),
        "pi" => Ok(Value::Number(std::f64::consts::PI)),
        "e" => Ok(Value::Number(std::f64::consts::E)),
        _ => Err(format!("Unknown math function: {}", name)),
    }
}

fn abs(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("abs() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Integer(n) => Ok(Value::Integer(n.abs())),
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err("abs() requires a numeric argument".to_string()),
    }
}

fn max(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("max() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(*a.max(b))),
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.max(*b))),
        _ => Err("max() requires two numeric arguments".to_string()),
    }
}

fn min(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("min() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(*a.min(b))),
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.min(*b))),
        _ => Err("min() requires two numeric arguments".to_string()),
    }
}

fn pow(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("pow() takes exactly 2 arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Value::Number(base), Value::Number(exp)) => Ok(Value::Number(base.powf(*exp))),
        (Value::Integer(base), Value::Integer(exp)) => {
            Ok(Value::Number((*base as f64).powf(*exp as f64)))
        }
        _ => Err("pow() requires two numeric arguments".to_string()),
    }
}

fn sqrt(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sqrt() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.sqrt())),
        Value::Integer(n) => Ok(Value::Number((*n as f64).sqrt())),
        _ => Err("sqrt() requires a numeric argument".to_string()),
    }
}

fn floor(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("floor() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Number(n) => Ok(Value::Integer(n.floor() as i64)),
        Value::Integer(n) => Ok(Value::Integer(*n)),
        _ => Err("floor() requires a numeric argument".to_string()),
    }
}

fn ceil(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("ceil() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Number(n) => Ok(Value::Integer(n.ceil() as i64)),
        Value::Integer(n) => Ok(Value::Integer(*n)),
        _ => Err("ceil() requires a numeric argument".to_string()),
    }
}

fn round(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("round() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Number(n) => Ok(Value::Integer(n.round() as i64)),
        Value::Integer(n) => Ok(Value::Integer(*n)),
        _ => Err("round() requires a numeric argument".to_string()),
    }
}

fn sin(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sin() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.sin())),
        Value::Integer(n) => Ok(Value::Number((*n as f64).sin())),
        _ => Err("sin() requires a numeric argument".to_string()),
    }
}

fn cos(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("cos() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.cos())),
        Value::Integer(n) => Ok(Value::Number((*n as f64).cos())),
        _ => Err("cos() requires a numeric argument".to_string()),
    }
}

fn tan(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("tan() takes exactly 1 argument".to_string());
    }
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.tan())),
        Value::Integer(n) => Ok(Value::Number((*n as f64).tan())),
        _ => Err("tan() requires a numeric argument".to_string()),
    }
}
