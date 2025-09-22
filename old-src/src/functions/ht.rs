use crate::value::Value;
use rand::Rng;

/// Flips a coin, returns "head" or "tail"
/// Example: coin() => "head"
pub fn coin(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("HT.coin requires no arguments".to_string());
    }
    
    let mut rng = rand::thread_rng();
    let result = if rng.gen_bool(0.5) { "head" } else { "tail" };
    
    Ok(Value::String(result.to_string()))
}

/// Returns true or false randomly
/// Example: bool_tos() => true
pub fn bool_tos(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("HT.bool requires no arguments".to_string());
    }
    
    let mut rng = rand::thread_rng();
    let result = rng.gen_bool(0.5);
    
    Ok(Value::Bool(result))
}
