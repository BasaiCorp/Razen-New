use crate::value::Value;

/// Push a value to the end of an array
/// Example: push([1, 2, 3], 4) => [1, 2, 3, 4]
pub fn push(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Array.push requires exactly 2 arguments: array and value".to_string());
    }
    
    let mut array = match &args[0] {
        Value::Array(arr) => arr.clone(),
        _ => return Err(format!("First argument to push must be an array, got {:?}", args[0])),
    };
    
    array.push(args[1].clone());
    
    Ok(Value::Array(array))
}

/// Pop a value from the end of an array
/// Example: pop([1, 2, 3]) => 3
pub fn pop(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Array.pop requires exactly 1 argument: array".to_string());
    }
    
    let mut array = match &args[0] {
        Value::Array(arr) => arr.clone(),
        _ => return Err(format!("Argument to pop must be an array, got {:?}", args[0])),
    };
    
    if array.is_empty() {
        return Err("Cannot pop from empty array".to_string());
    }
    
    let popped = array.pop().unwrap();
    
    Ok(popped)
}

/// Join array elements with a separator
/// Example: join(["a", "b", "c"], "-") => "a-b-c"
pub fn join(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Array.join requires exactly 2 arguments: array and separator".to_string());
    }
    
    let array = match &args[0] {
        Value::Array(arr) => arr.clone(),
        _ => return Err(format!("First argument to join must be an array, got {:?}", args[0])),
    };
    
    let separator = args[1].as_string()?;
    
    let strings: Result<Vec<String>, String> = array.iter()
        .map(|v| v.as_string())
        .collect();
        
    match strings {
        Ok(s) => Ok(Value::String(s.join(&separator))),
        Err(e) => Err(e),
    }
}

/// Get the length of an array
/// Example: length([1, 2, 3]) => 3
pub fn length(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Array.length requires exactly 1 argument: array".to_string());
    }
    
    let array = match &args[0] {
        Value::Array(arr) => arr.clone(),
        _ => return Err(format!("Argument to length must be an array, got {:?}", args[0])),
    };
    
    Ok(Value::Int(array.len() as i64))
}

/// Get unique elements from an array
/// Example: unique([1, 2, 2, 3, 3, 3]) => [1, 2, 3]
pub fn unique(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Array.unique requires exactly 1 argument: array".to_string());
    }
    
    let array = match &args[0] {
        Value::Array(arr) => arr.clone(),
        _ => return Err(format!("Argument to unique must be an array, got {:?}", args[0])),
    };
    
    let mut result = Vec::new();
    
    for item in array {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    
    Ok(Value::Array(result))
}

/// Sort an array
/// Example: sort([3, 1, 2]) => [1, 2, 3]
pub fn sort(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Array.sort requires exactly 1 argument: array".to_string());
    }
    
    let array = match &args[0] {
        Value::Array(arr) => arr.clone(),
        _ => return Err(format!("Argument to sort must be an array, got {:?}", args[0])),
    };
    
    // Sort only works reliably on arrays of the same type
    // For simplicity, we'll convert everything to strings and sort lexicographically
    let mut result = array.clone();
    result.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
    
    Ok(Value::Array(result))
}

/// Reverse an array
/// Example: reverse([1, 2, 3]) => [3, 2, 1]
pub fn reverse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Array.reverse requires exactly 1 argument: array".to_string());
    }
    
    let array = match &args[0] {
        Value::Array(arr) => arr.clone(),
        _ => return Err(format!("Argument to reverse must be an array, got {:?}", args[0])),
    };
    
    let mut result = array.clone();
    result.reverse();
    
    Ok(Value::Array(result))
}

/// Get a slice of an array
/// Example: slice([1, 2, 3, 4, 5], 1, 3) => [2, 3]
pub fn slice(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("Array.slice requires exactly 3 arguments: array, start, end".to_string());
    }
    
    let array = match &args[0] {
        Value::Array(arr) => arr.clone(),
        _ => return Err(format!("First argument to slice must be an array, got {:?}", args[0])),
    };
    
    let start = match &args[1] {
        Value::Int(n) => *n as usize,
        Value::Float(n) => *n as usize,
        _ => return Err(format!("Second argument to slice must be a number, got {:?}", args[1])),
    };
    
    let end = match &args[2] {
        Value::Int(n) => *n as usize,
        Value::Float(n) => *n as usize,
        _ => return Err(format!("Third argument to slice must be a number, got {:?}", args[2])),
    };
    
    if start > array.len() || end > array.len() || start > end {
        return Err(format!("Invalid slice range: {}..{} for array of length {}", start, end, array.len()));
    }
    
    let result = array[start..end].to_vec();
    
    Ok(Value::Array(result))
}
