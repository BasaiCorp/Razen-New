// src/builtins/string.rs
//! String manipulation functions

use crate::backend::execution::value::Value;

/// Split string by delimiter
pub fn split(stack: &mut Vec<Value>) -> Result<(), String> {
    let delimiter = stack.pop().ok_or("[ERROR] split() requires delimiter")?;
    let string = stack.pop().ok_or("[ERROR] split() requires string")?;
    
    if let (Value::String(s), Value::String(delim)) = (string, delimiter) {
        let parts: Vec<Value> = s.split(&delim as &str)
            .map(|part| Value::String(part.to_string()))
            .collect();
        stack.push(Value::Array(parts));
        Ok(())
    } else {
        Err("[ERROR] split() requires string and delimiter as strings".to_string())
    }
}

/// Join array elements with separator
pub fn join(stack: &mut Vec<Value>) -> Result<(), String> {
    let separator = stack.pop().ok_or("[ERROR] join() requires separator")?;
    let array = stack.pop().ok_or("[ERROR] join() requires array")?;
    
    if let (Value::Array(arr), Value::String(sep)) = (array, separator) {
        let joined = arr.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(&sep);
        stack.push(Value::String(joined));
        Ok(())
    } else {
        Err("[ERROR] join() requires array and separator string".to_string())
    }
}

/// Trim whitespace from string
pub fn trim(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(s)) = stack.pop() {
        stack.push(Value::String(s.trim().to_string()));
        Ok(())
    } else {
        Err("[ERROR] trim() requires a string".to_string())
    }
}

/// Convert string to uppercase
pub fn upper(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(s)) = stack.pop() {
        stack.push(Value::String(s.to_uppercase()));
        Ok(())
    } else {
        Err("[ERROR] upper() requires a string".to_string())
    }
}

/// Convert string to lowercase
pub fn lower(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(s)) = stack.pop() {
        stack.push(Value::String(s.to_lowercase()));
        Ok(())
    } else {
        Err("[ERROR] lower() requires a string".to_string())
    }
}

/// Replace substring in string
pub fn replace(stack: &mut Vec<Value>) -> Result<(), String> {
    let new_str = stack.pop().ok_or("[ERROR] replace() requires new string")?;
    let old_str = stack.pop().ok_or("[ERROR] replace() requires old string")?;
    let string = stack.pop().ok_or("[ERROR] replace() requires string")?;
    
    if let (Value::String(s), Value::String(old), Value::String(new)) = (string, old_str, new_str) {
        stack.push(Value::String(s.replace(&old as &str, &new as &str)));
        Ok(())
    } else {
        Err("[ERROR] replace() requires all string arguments".to_string())
    }
}

/// Check if string contains substring
pub fn contains(stack: &mut Vec<Value>) -> Result<(), String> {
    let substring = stack.pop().ok_or("[ERROR] contains() requires substring")?;
    let string = stack.pop().ok_or("[ERROR] contains() requires string")?;
    
    if let (Value::String(s), Value::String(sub)) = (string, substring) {
        stack.push(Value::Boolean(s.contains(&sub as &str)));
        Ok(())
    } else {
        Err("[ERROR] contains() requires string arguments".to_string())
    }
}

/// Check if string starts with prefix
pub fn starts_with(stack: &mut Vec<Value>) -> Result<(), String> {
    let prefix = stack.pop().ok_or("[ERROR] starts_with() requires prefix")?;
    let string = stack.pop().ok_or("[ERROR] starts_with() requires string")?;
    
    if let (Value::String(s), Value::String(pre)) = (string, prefix) {
        stack.push(Value::Boolean(s.starts_with(&pre as &str)));
        Ok(())
    } else {
        Err("[ERROR] starts_with() requires string arguments".to_string())
    }
}

/// Check if string ends with suffix
pub fn ends_with(stack: &mut Vec<Value>) -> Result<(), String> {
    let suffix = stack.pop().ok_or("[ERROR] ends_with() requires suffix")?;
    let string = stack.pop().ok_or("[ERROR] ends_with() requires string")?;
    
    if let (Value::String(s), Value::String(suf)) = (string, suffix) {
        stack.push(Value::Boolean(s.ends_with(&suf as &str)));
        Ok(())
    } else {
        Err("[ERROR] ends_with() requires string arguments".to_string())
    }
}

/// Get substring
pub fn substring(stack: &mut Vec<Value>) -> Result<(), String> {
    let end = stack.pop().ok_or("[ERROR] substring() requires end index")?;
    let start = stack.pop().ok_or("[ERROR] substring() requires start index")?;
    let string = stack.pop().ok_or("[ERROR] substring() requires string")?;
    
    if let (Value::String(s), Value::Integer(st), Value::Integer(en)) = (string, start, end) {
        let start_idx = st.max(0) as usize;
        let end_idx = en.max(0) as usize;
        if start_idx <= s.len() && end_idx <= s.len() && start_idx <= end_idx {
            stack.push(Value::String(s[start_idx..end_idx].to_string()));
            Ok(())
        } else {
            Err(format!("[ERROR] substring indices out of bounds: {}..{} for string length {}", start_idx, end_idx, s.len()))
        }
    } else {
        Err("[ERROR] substring() requires string and two integers".to_string())
    }
}

/// Repeat string n times
pub fn repeat(stack: &mut Vec<Value>) -> Result<(), String> {
    let count = stack.pop().ok_or("[ERROR] repeat() requires count")?;
    let string = stack.pop().ok_or("[ERROR] repeat() requires string")?;
    
    if let (Value::String(s), Value::Integer(n)) = (string, count) {
        stack.push(Value::String(s.repeat(n.max(0) as usize)));
        Ok(())
    } else {
        Err("[ERROR] repeat() requires string and integer".to_string())
    }
}

/// Reverse string
pub fn reverse(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::String(s)) = stack.pop() {
        stack.push(Value::String(s.chars().rev().collect()));
        Ok(())
    } else {
        Err("[ERROR] reverse() requires a string".to_string())
    }
}
