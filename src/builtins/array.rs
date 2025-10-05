// src/builtins/array.rs
//! Array manipulation functions

use crate::backend::execution::value::Value;

/// Push element to array
pub fn push(stack: &mut Vec<Value>) -> Result<(), String> {
    let element = stack.pop().ok_or("[ERROR] push() requires element")?;
    let array = stack.pop().ok_or("[ERROR] push() requires array")?;
    
    if let Value::Array(mut arr) = array {
        arr.push(element);
        stack.push(Value::Array(arr));
        Ok(())
    } else {
        Err("[ERROR] push() requires an array".to_string())
    }
}

/// Pop element from array
pub fn pop(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::Array(mut arr)) = stack.pop() {
        if let Some(element) = arr.pop() {
            stack.push(Value::Array(arr));
            stack.push(element);
            Ok(())
        } else {
            Err("[ERROR] Cannot pop from empty array".to_string())
        }
    } else {
        Err("[ERROR] pop() requires an array".to_string())
    }
}

/// Remove first element from array
pub fn shift(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::Array(mut arr)) = stack.pop() {
        if !arr.is_empty() {
            let element = arr.remove(0);
            stack.push(Value::Array(arr));
            stack.push(element);
            Ok(())
        } else {
            Err("[ERROR] Cannot shift from empty array".to_string())
        }
    } else {
        Err("[ERROR] shift() requires an array".to_string())
    }
}

/// Add element to beginning of array
pub fn unshift(stack: &mut Vec<Value>) -> Result<(), String> {
    let element = stack.pop().ok_or("[ERROR] unshift() requires element")?;
    let array = stack.pop().ok_or("[ERROR] unshift() requires array")?;
    
    if let Value::Array(mut arr) = array {
        arr.insert(0, element);
        stack.push(Value::Array(arr));
        Ok(())
    } else {
        Err("[ERROR] unshift() requires an array".to_string())
    }
}

/// Reverse array
pub fn reverse(stack: &mut Vec<Value>) -> Result<(), String> {
    if let Some(Value::Array(mut arr)) = stack.pop() {
        arr.reverse();
        stack.push(Value::Array(arr));
        Ok(())
    } else {
        Err("[ERROR] reverse() requires an array".to_string())
    }
}

/// Check if array includes element
pub fn includes(stack: &mut Vec<Value>) -> Result<(), String> {
    let element = stack.pop().ok_or("[ERROR] includes() requires element")?;
    let array = stack.pop().ok_or("[ERROR] includes() requires array")?;
    
    if let Value::Array(arr) = array {
        let found = arr.iter().any(|v| v.to_string() == element.to_string());
        stack.push(Value::Boolean(found));
        Ok(())
    } else {
        Err("[ERROR] includes() requires an array".to_string())
    }
}
