// src/stdlib/random_lib.rs
//! Random number generation standard library

use crate::backend::execution::value::Value;
use rand::Rng;
use rand::seq::SliceRandom;

pub fn has_function(name: &str) -> bool {
    matches!(name, 
        "integer" | "number" | "range" | "choice" | "shuffle" | 
        "boolean" | "bytes" | "seed"
    )
}

pub fn get_function_list() -> Vec<&'static str> {
    vec![
        "integer", "number", "range", "choice", "shuffle",
        "boolean", "bytes", "seed"
    ]
}

pub fn call_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "integer" => random_int(args),
        "number" => random_float(args),
        "range" => random_range(args),
        "choice" => choice(args),
        "shuffle" => shuffle(args),
        "boolean" => random_bool(args),
        "bytes" => random_bytes(args),
        "seed" => set_seed(args),
        _ => Err(format!("Unknown random function: {}", name)),
    }
}

fn random_int(args: Vec<Value>) -> Result<Value, String> {
    let mut rng = rand::thread_rng();
    
    if args.is_empty() {
        // Generate random i64
        Ok(Value::Integer(rng.r#gen()))
    } else if args.len() == 1 {
        // Generate random int from 0 to max
        match &args[0] {
            Value::Integer(max) => {
                if *max <= 0 {
                    return Err("random.int() max must be positive".to_string());
                }
                Ok(Value::Integer(rng.gen_range(0..*max)))
            }
            _ => Err("random.int() requires an integer argument".to_string()),
        }
    } else {
        // Generate random int from min to max
        match (&args[0], &args[1]) {
            (Value::Integer(min), Value::Integer(max)) => {
                if min >= max {
                    return Err("random.int() min must be less than max".to_string());
                }
                Ok(Value::Integer(rng.gen_range(*min..*max)))
            }
            _ => Err("random.int() requires two integer arguments".to_string()),
        }
    }
}

fn random_float(args: Vec<Value>) -> Result<Value, String> {
    let mut rng = rand::thread_rng();
    
    if args.is_empty() {
        // Generate random float between 0.0 and 1.0
        Ok(Value::Number(rng.r#gen()))
    } else if args.len() == 1 {
        // Generate random float from 0.0 to max
        match &args[0] {
            Value::Number(max) => {
                if *max <= 0.0 {
                    return Err("random.float() max must be positive".to_string());
                }
                Ok(Value::Number(rng.gen_range(0.0..*max)))
            }
            Value::Integer(max) => {
                let max_f = *max as f64;
                if max_f <= 0.0 {
                    return Err("random.float() max must be positive".to_string());
                }
                Ok(Value::Number(rng.gen_range(0.0..max_f)))
            }
            _ => Err("random.float() requires a number argument".to_string()),
        }
    } else {
        // Generate random float from min to max
        let min_f = match &args[0] {
            Value::Number(n) => *n,
            Value::Integer(n) => *n as f64,
            _ => return Err("random.float() requires number arguments".to_string()),
        };
        
        let max_f = match &args[1] {
            Value::Number(n) => *n,
            Value::Integer(n) => *n as f64,
            _ => return Err("random.float() requires number arguments".to_string()),
        };
        
        if min_f >= max_f {
            return Err("random.float() min must be less than max".to_string());
        }
        
        Ok(Value::Number(rng.gen_range(min_f..max_f)))
    }
}

fn random_range(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("random.range() requires 2 arguments (min, max)".to_string());
    }
    
    random_int(args)
}

fn choice(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("random.choice() requires 1 argument (array)".to_string());
    }
    
    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err("random.choice() cannot choose from empty array".to_string());
            }
            
            let mut rng = rand::thread_rng();
            match arr.choose(&mut rng) {
                Some(value) => Ok(value.clone()),
                None => Ok(Value::Null),
            }
        }
        _ => Err("random.choice() requires an array argument".to_string()),
    }
}

fn shuffle(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("random.shuffle() requires 1 argument (array)".to_string());
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let mut shuffled = arr.clone();
            let mut rng = rand::thread_rng();
            shuffled.shuffle(&mut rng);
            Ok(Value::Array(shuffled))
        }
        _ => Err("random.shuffle() requires an array argument".to_string()),
    }
}

fn random_bool(_args: Vec<Value>) -> Result<Value, String> {
    let mut rng = rand::thread_rng();
    Ok(Value::Boolean(rng.r#gen()))
}

fn random_bytes(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("random.bytes() requires 1 argument (length)".to_string());
    }
    
    match &args[0] {
        Value::Integer(len) => {
            if *len < 0 {
                return Err("random.bytes() length must be non-negative".to_string());
            }
            
            let mut rng = rand::thread_rng();
            let bytes: Vec<Value> = (0..*len)
                .map(|_| Value::Integer(rng.gen_range(0..256)))
                .collect();
            
            Ok(Value::Array(bytes))
        }
        _ => Err("random.bytes() requires an integer argument".to_string()),
    }
}

fn set_seed(_args: Vec<Value>) -> Result<Value, String> {
    // Note: rand::thread_rng() doesn't support seeding directly
    // This is a placeholder for compatibility
    Ok(Value::Null)
}
