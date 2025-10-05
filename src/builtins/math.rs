// src/builtins/math.rs
//! Math functions

use crate::backend::execution::value::Value;

/// Absolute value
pub fn abs(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(value) = stack.pop() {
        match value {
            Value::Integer(n) => stack.push(Value::Integer(n.abs())),
            Value::Number(n) => stack.push(Value::Number(n.abs())),
            _ => return Err("[ERROR] abs() requires a number".to_string())
        }
        Ok(())
    } else {
        Err("[ERROR] abs() requires an argument".to_string())
    }
}

/// Square root
pub fn sqrt(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(value) = stack.pop() {
        let num = match value {
            Value::Integer(n) => n as f64,
            Value::Number(n) => n,
            _ => return Err("[ERROR] sqrt() requires a number".to_string())
        };
        stack.push(Value::Number(num.sqrt()));
        Ok(())
    } else {
        Err("[ERROR] sqrt() requires an argument".to_string())
    }
}

/// Power (base^exponent)
pub fn pow(stack: &mut Vec<Value>) -> Result<(), String> {
    let exp = stack.pop().ok_or("[ERROR] pow() requires exponent")?;
    let base = stack.pop().ok_or("[ERROR] pow() requires base")?;
    
    let base_num = match base {
        Value::Integer(n) => n as f64,
        Value::Number(n) => n,
        _ => return Err("[ERROR] pow() requires numeric base".to_string())
    };
    
    let exp_num = match exp {
        Value::Integer(n) => n as f64,
        Value::Number(n) => n,
        _ => return Err("[ERROR] pow() requires numeric exponent".to_string())
    };
    
    stack.push(Value::Number(base_num.powf(exp_num)));
    Ok(())
}

/// Floor (round down)
pub fn floor(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(value) = stack.pop() {
        let num = match value {
            Value::Integer(n) => return { stack.push(Value::Integer(n)); Ok(()) },
            Value::Number(n) => n,
            _ => return Err("[ERROR] floor() requires a number".to_string())
        };
        stack.push(Value::Integer(num.floor() as i64));
        Ok(())
    } else {
        Err("[ERROR] floor() requires an argument".to_string())
    }
}

/// Ceiling (round up)
pub fn ceil(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(value) = stack.pop() {
        let num = match value {
            Value::Integer(n) => return { stack.push(Value::Integer(n)); Ok(()) },
            Value::Number(n) => n,
            _ => return Err("[ERROR] ceil() requires a number".to_string())
        };
        stack.push(Value::Integer(num.ceil() as i64));
        Ok(())
    } else {
        Err("[ERROR] ceil() requires an argument".to_string())
    }
}

/// Round to nearest integer
pub fn round(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(value) = stack.pop() {
        let num = match value {
            Value::Integer(n) => return { stack.push(Value::Integer(n)); Ok(()) },
            Value::Number(n) => n,
            _ => return Err("[ERROR] round() requires a number".to_string())
        };
        stack.push(Value::Integer(num.round() as i64));
        Ok(())
    } else {
        Err("[ERROR] round() requires an argument".to_string())
    }
}

/// Minimum of two numbers
pub fn min(stack: &mut Vec<Value>) -> Result<(), String> {
    let b = stack.pop().ok_or("[ERROR] min() requires two arguments")?;
    let a = stack.pop().ok_or("[ERROR] min() requires two arguments")?;
    
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => stack.push(Value::Integer(x.min(y))),
        (Value::Number(x), Value::Number(y)) => stack.push(Value::Number(x.min(y))),
        (Value::Integer(x), Value::Number(y)) => stack.push(Value::Number((x as f64).min(y))),
        (Value::Number(x), Value::Integer(y)) => stack.push(Value::Number(x.min(y as f64))),
        _ => return Err("[ERROR] min() requires numeric arguments".to_string())
    }
    Ok(())
}

/// Maximum of two numbers
pub fn max(stack: &mut Vec<Value>) -> Result<(), String> {
    let b = stack.pop().ok_or("[ERROR] max() requires two arguments")?;
    let a = stack.pop().ok_or("[ERROR] max() requires two arguments")?;
    
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => stack.push(Value::Integer(x.max(y))),
        (Value::Number(x), Value::Number(y)) => stack.push(Value::Number(x.max(y))),
        (Value::Integer(x), Value::Number(y)) => stack.push(Value::Number((x as f64).max(y))),
        (Value::Number(x), Value::Integer(y)) => stack.push(Value::Number(x.max(y as f64))),
        _ => return Err("[ERROR] max() requires numeric arguments".to_string())
    }
    Ok(())
}

/// Sine
pub fn sin(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(value) = stack.pop() {
        let num = match value {
            Value::Integer(n) => n as f64,
            Value::Number(n) => n,
            _ => return Err("[ERROR] sin() requires a number".to_string())
        };
        stack.push(Value::Number(num.sin()));
        Ok(())
    } else {
        Err("[ERROR] sin() requires an argument".to_string())
    }
}

/// Cosine
pub fn cos(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(value) = stack.pop() {
        let num = match value {
            Value::Integer(n) => n as f64,
            Value::Number(n) => n,
            _ => return Err("[ERROR] cos() requires a number".to_string())
        };
        stack.push(Value::Number(num.cos()));
        Ok(())
    } else {
        Err("[ERROR] cos() requires an argument".to_string())
    }
}

/// Tangent
pub fn tan(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(value) = stack.pop() {
        let num = match value {
            Value::Integer(n) => n as f64,
            Value::Number(n) => n,
            _ => return Err("[ERROR] tan() requires a number".to_string())
        };
        stack.push(Value::Number(num.tan()));
        Ok(())
    } else {
        Err("[ERROR] tan() requires an argument".to_string())
    }
}

/// Random number between 0.0 and 1.0
pub fn random(stack: &mut Vec<Value>) -> Result<(), String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let random_val = ((seed % 1000000) as f64) / 1000000.0;
    stack.push(Value::Number(random_val));
    Ok(())
}

/// Random integer in range [min, max]
pub fn random_range(stack: &mut Vec<Value>) -> Result<(), String> {
    let max = stack.pop().ok_or("[ERROR] random_range() requires max")?;
    let min = stack.pop().ok_or("[ERROR] random_range() requires min")?;
    
    if let (Value::Integer(min_val), Value::Integer(max_val)) = (min, max) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let range = (max_val - min_val + 1) as u128;
        let random_val = min_val + ((seed % range) as i64);
        stack.push(Value::Integer(random_val));
        Ok(())
    } else {
        Err("[ERROR] random_range() requires integer arguments".to_string())
    }
}
