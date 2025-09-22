use std::collections::HashMap;
use std::fmt;

/// Value represents any value that can be manipulated in Razen
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Null,
}

impl Value {
    /// Convert a Value to a string representation
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Map(map) => {
                let entries: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", entries.join(", "))
            }
            Value::Null => "null".to_string(),
        }
    }

    /// Try to convert a Value to an i64
    pub fn as_int(&self) -> Result<i64, String> {
        match self {
            Value::Int(i) => Ok(*i),
            Value::Float(f) => Ok(*f as i64),
            Value::String(s) => s.parse::<i64>().map_err(|_| "Cannot convert string to integer".to_string()),
            _ => Err(format!("Cannot convert {:?} to integer", self)),
        }
    }

    /// Try to convert a Value to an f64
    pub fn as_float(&self) -> Result<f64, String> {
        match self {
            Value::Int(i) => Ok(*i as f64),
            Value::Float(f) => Ok(*f),
            Value::String(s) => s.parse::<f64>().map_err(|_| "Cannot convert string to float".to_string()),
            _ => Err(format!("Cannot convert {:?} to float", self)),
        }
    }

    /// Try to convert a Value to a bool
    pub fn as_bool(&self) -> Result<bool, String> {
        match self {
            Value::Bool(b) => Ok(*b),
            Value::Int(i) => Ok(*i != 0),
            Value::Float(f) => Ok(*f != 0.0),
            Value::String(s) => {
                match s.to_lowercase().as_str() {
                    "true" | "yes" | "1" => Ok(true),
                    "false" | "no" | "0" => Ok(false),
                    _ => Err(format!("Cannot convert string '{}' to boolean", s)),
                }
            }
            _ => Err(format!("Cannot convert {:?} to boolean", self)),
        }
    }

    /// Try to convert a Value to a String
    pub fn as_string(&self) -> Result<String, String> {
        match self {
            Value::String(s) => Ok(s.clone()),
            _ => Ok(self.to_string()),
        }
    }

    /// Try to convert a Value to a Vec<Value>
    pub fn as_array(&self) -> Result<Vec<Value>, String> {
        match self {
            Value::Array(arr) => Ok(arr.clone()),
            _ => Err(format!("Cannot convert {:?} to array", self)),
        }
    }

    /// Try to convert a Value to a HashMap<String, Value>
    pub fn as_map(&self) -> Result<HashMap<String, Value>, String> {
        match self {
            Value::Map(map) => Ok(map.clone()),
            _ => Err(format!("Cannot convert {:?} to map", self)),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
            (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
