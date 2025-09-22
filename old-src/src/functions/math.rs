use crate::value::Value;

/// Add two numbers
/// Example: add(5, 3) => 8
pub fn add(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Math.add requires exactly 2 arguments: a, b".to_string());
    }
    let a = args[0].as_float()?;
    let b = args[1].as_float()?;
    Ok(Value::Float(a + b))
}

/// Subtract two numbers
/// Example: subtract(10, 4) => 6
pub fn subtract(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Math.subtract requires exactly 2 arguments: a, b".to_string());
    }
    let a = args[0].as_float()?;
    let b = args[1].as_float()?;
    Ok(Value::Float(a - b))
}

/// Multiply two numbers
/// Example: multiply(6, 7) => 42
pub fn multiply(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Math.multiply requires exactly 2 arguments: a, b".to_string());
    }
    let a = args[0].as_float()?;
    let b = args[1].as_float()?;
    Ok(Value::Float(a * b))
}

/// Divide two numbers
/// Example: divide(20, 5) => 4
pub fn divide(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Math.divide requires exactly 2 arguments: a, b".to_string());
    }
    let a = args[0].as_float()?;
    let b = args[1].as_float()?;
    
    if b == 0.0 {
        return Err("Division by zero".to_string());
    }
    
    Ok(Value::Float(a / b))
}

/// Raise a number to a power
/// Example: power(2, 3) => 8
pub fn power(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Math.power requires exactly 2 arguments: base, exponent".to_string());
    }
    let base = args[0].as_float()?;
    let exponent = args[1].as_float()?;
    
    Ok(Value::Float(base.powf(exponent)))
}

/// Calculate the square root of a number
/// Example: sqrt(16) => 4
pub fn sqrt(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Math.sqrt requires exactly 1 argument: value".to_string());
    }
    let value = args[0].as_float()?;
    
    if value < 0.0 {
        return Err("Cannot calculate square root of negative number".to_string());
    }
    
    Ok(Value::Float(value.sqrt()))
}

/// Calculate the absolute value of a number
/// Example: abs(-15) => 15
pub fn abs(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Math.abs requires exactly 1 argument: value".to_string());
    }
    let value = args[0].as_float()?;
    
    Ok(Value::Float(value.abs()))
}

/// Round a number to the nearest integer
/// Example: round(3.7) => 4
pub fn round(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Math.round requires exactly 1 argument: value".to_string());
    }
    let value = args[0].as_float()?;
    
    Ok(Value::Float(value.round()))
}

/// Round a number down to the nearest integer
/// Example: floor(3.7) => 3
pub fn floor(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Math.floor requires exactly 1 argument: value".to_string());
    }
    let value = args[0].as_float()?;
    
    Ok(Value::Float(value.floor()))
}

/// Round a number up to the nearest integer
/// Example: ceil(3.2) => 4
pub fn ceil(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Math.ceil requires exactly 1 argument: value".to_string());
    }
    let value = args[0].as_float()?;
    
    Ok(Value::Float(value.ceil()))
}

/// Calculate the sine of an angle (in radians)
/// Example: sin(0) => 0
pub fn sin(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Math.sin requires exactly 1 argument: angle".to_string());
    }
    let angle = args[0].as_float()?;
    
    Ok(Value::Float(angle.sin()))
}

/// Calculate the cosine of an angle (in radians)
/// Example: cos(0) => 1
pub fn cos(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Math.cos requires exactly 1 argument: angle".to_string());
    }
    let angle = args[0].as_float()?;
    
    Ok(Value::Float(angle.cos()))
}

/// Calculate the tangent of an angle (in radians)
/// Example: tan(0) => 0
pub fn tan(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Math.tan requires exactly 1 argument: angle".to_string());
    }
    let angle = args[0].as_float()?;
    
    Ok(Value::Float(angle.tan()))
}

/// Calculate the logarithm of a number with a given base
/// Example: log(100, 10) => 2
pub fn log(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Math.log requires exactly 2 arguments: value, base".to_string());
    }
    let value = args[0].as_float()?;
    let base = args[1].as_float()?;
    
    if value <= 0.0 || base <= 0.0 || base == 1.0 {
        return Err("Invalid arguments for logarithm".to_string());
    }
    
    Ok(Value::Float(value.log(base)))
}

/// Calculate e raised to the power of a number
/// Example: exp(1) => 2.718281828459045
pub fn exp(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Math.exp requires exactly 1 argument: value".to_string());
    }
    let value = args[0].as_float()?;
    
    Ok(Value::Float(value.exp()))
}

/// Generate a random number between 0 and 1
/// Example: random() => 0.123456789
pub fn random(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("Math.random takes no arguments".to_string());
    }
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    Ok(Value::Float(rng.gen::<f64>()))
}

/// Find the maximum value among a list of numbers
/// Example: max(3, 7, 2) => 7
pub fn max(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("Math.max requires at least one argument".to_string());
    }
    
    let mut max_value = args[0].as_float()?;
    
    for arg in &args[1..] {
        let value = arg.as_float()?;
        if value > max_value {
            max_value = value;
        }
    }
    
    Ok(Value::Float(max_value))
}

/// Find the minimum value among a list of numbers
/// Example: min(3, 7, 2) => 2
pub fn min(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("Math.min requires at least one argument".to_string());
    }
    
    let mut min_value = args[0].as_float()?;
    
    for arg in &args[1..] {
        let value = arg.as_float()?;
        if value < min_value {
            min_value = value;
        }
    }
    
    Ok(Value::Float(min_value))
}

/// Calculate the modulo (remainder) of a division
/// Example: modulo(10, 3) => 1
pub fn modulo(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Math.modulo requires exactly 2 arguments: a, b".to_string());
    }
    let a = args[0].as_float()?;
    let b = args[1].as_float()?;
    
    if b == 0.0 {
        return Err("Modulo by zero".to_string());
    }
    
    Ok(Value::Float(a % b))
}
