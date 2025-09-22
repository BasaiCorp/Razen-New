use crate::value::Value;
use rand::Rng;

/// Generate a random integer between min and max (inclusive)
/// Example: int(1, 10) => 7
pub fn int(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Random.int requires exactly 2 arguments: min, max".to_string());
    }
    
    let min = match &args[0] {
        Value::Int(n) => *n,
        Value::Float(n) => *n as i64,
        _ => return Err(format!("First argument to int must be a number, got {:?}", args[0])),
    };
    
    let max = match &args[1] {
        Value::Int(n) => *n,
        Value::Float(n) => *n as i64,
        _ => return Err(format!("Second argument to int must be a number, got {:?}", args[1])),
    };
    
    if min > max {
        return Err(format!("Min value ({}) cannot be greater than max value ({})", min, max));
    }
    
    let mut rng = rand::thread_rng();
    let result = rng.gen_range(min..=max);
    
    Ok(Value::Int(result))
}

/// Generate a random float between min and max (inclusive)
/// Example: float(0, 1) => 0.42
pub fn float(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Random.float requires exactly 2 arguments: min, max".to_string());
    }
    
    let min = args[0].as_float()?;
    let max = args[1].as_float()?;
    
    if min > max {
        return Err(format!("Min value ({}) cannot be greater than max value ({})", min, max));
    }
    
    let mut rng = rand::thread_rng();
    let result = rng.gen_range(min..=max);
    
    Ok(Value::Float(result))
}

/// Choose a random element from an array
/// Example: choice(["apple", "banana", "cherry"]) => "banana"
pub fn choice(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Random.choice requires exactly 1 argument: array".to_string());
    }
    
    let array = match &args[0] {
        Value::Array(arr) => arr.clone(),
        _ => return Err(format!("Argument to choice must be an array, got {:?}", args[0])),
    };
    
    if array.is_empty() {
        return Err("Cannot choose from empty array".to_string());
    }
    
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..array.len());
    
    Ok(array[index].clone())
}

/// Shuffle an array
/// Example: shuffle([1, 2, 3, 4, 5]) => [3, 1, 5, 2, 4]
pub fn shuffle(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Random.shuffle requires exactly 1 argument: array".to_string());
    }
    
    let mut array = match &args[0] {
        Value::Array(arr) => arr.clone(),
        _ => return Err(format!("Argument to shuffle must be an array, got {:?}", args[0])),
    };
    
    let mut rng = rand::thread_rng();
    for i in (1..array.len()).rev() {
        let j = rng.gen_range(0..=i);
        array.swap(i, j);
    }
    
    Ok(Value::Array(array))
}
