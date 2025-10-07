// src/backend/execution/value.rs
//! High-performance value representation for runtime

use std::fmt;
use std::collections::HashMap;

/// Optimized value type for runtime - avoids string conversions
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Integer(i64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Struct {
        type_name: String,
        fields: HashMap<String, Value>,
    },
    // Result type: Ok(value) or Err(error)
    Result {
        is_ok: bool,
        value: Box<Value>,
    },
    // Option type: Some(value) or None
    Option {
        is_some: bool,
        value: Box<Value>,
    },
    Null,
}

impl Value {
    /// Fast conversion to boolean for conditionals
    #[inline]
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Integer(i) => *i != 0,
            Value::String(s) => !s.is_empty() && s != "null" && s != "false" && s != "False",
            Value::Array(arr) => !arr.is_empty(),
            Value::Map(map) => !map.is_empty(),
            Value::Struct { .. } => true,
            Value::Result { is_ok, .. } => *is_ok,
            Value::Option { is_some, .. } => *is_some,
            Value::Null => false,
        }
    }

    /// Create an Ok Result value
    pub fn ok(value: Value) -> Value {
        Value::Result {
            is_ok: true,
            value: Box::new(value),
        }
    }

    /// Create an Err Result value
    pub fn err(error: Value) -> Value {
        Value::Result {
            is_ok: false,
            value: Box::new(error),
        }
    }

    /// Create a Some Option value
    pub fn some(value: Value) -> Value {
        Value::Option {
            is_some: true,
            value: Box::new(value),
        }
    }

    /// Create a None Option value
    pub fn none() -> Value {
        Value::Option {
            is_some: false,
            value: Box::new(Value::Null),
        }
    }

    /// Check if this is an Ok Result
    pub fn is_ok(&self) -> bool {
        matches!(self, Value::Result { is_ok: true, .. })
    }

    /// Check if this is an Err Result
    pub fn is_err(&self) -> bool {
        matches!(self, Value::Result { is_ok: false, .. })
    }

    /// Check if this is a Some Option
    pub fn is_some(&self) -> bool {
        matches!(self, Value::Option { is_some: true, .. })
    }

    /// Check if this is a None Option
    pub fn is_none(&self) -> bool {
        matches!(self, Value::Option { is_some: false, .. })
    }

    /// Unwrap a Result or Option, returning the inner value
    pub fn unwrap(&self) -> Result<Value, String> {
        match self {
            Value::Result { is_ok: true, value } => Ok((**value).clone()),
            Value::Result { is_ok: false, value } => Err(format!("Unwrap failed: {}", value)),
            Value::Option { is_some: true, value } => Ok((**value).clone()),
            Value::Option { is_some: false, .. } => Err("Unwrap failed: None".to_string()),
            _ => Err("Cannot unwrap non-Result/Option value".to_string()),
        }
    }

    /// Unwrap or return a default value
    pub fn unwrap_or(&self, default: Value) -> Value {
        match self {
            Value::Result { is_ok: true, value } | Value::Option { is_some: true, value } => {
                (**value).clone()
            }
            _ => default,
        }
    }

    /// Fast numeric addition
    #[inline]
    pub fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Integer(a), Value::Number(b)) => Value::Number(*a as f64 + b),
            (Value::Number(a), Value::Integer(b)) => Value::Number(a + *b as f64),
            _ => {
                // String concatenation fallback
                Value::String(format!("{}{}", self, other))
            }
        }
    }

    /// Fast numeric subtraction
    #[inline]
    pub fn subtract(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            (Value::Integer(a), Value::Number(b)) => Value::Number(*a as f64 - b),
            (Value::Number(a), Value::Integer(b)) => Value::Number(a - *b as f64),
            _ => Value::Null,
        }
    }

    /// Fast numeric multiplication
    #[inline]
    pub fn multiply(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (Value::Integer(a), Value::Number(b)) => Value::Number(*a as f64 * b),
            (Value::Number(a), Value::Integer(b)) => Value::Number(a * *b as f64),
            _ => Value::Null,
        }
    }

    /// Fast numeric division
    #[inline]
    pub fn divide(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Number(*a as f64 / *b as f64))
                }
            }
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            (Value::Integer(a), Value::Number(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Number(*a as f64 / b))
                }
            }
            (Value::Number(a), Value::Integer(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Number(a / *b as f64))
                }
            }
            _ => Ok(Value::Null),
        }
    }

    /// Fast comparison operations
    #[inline]
    pub fn less_than(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a < b,
            (Value::Number(a), Value::Number(b)) => a < b,
            (Value::Integer(a), Value::Number(b)) => (*a as f64) < *b,
            (Value::Number(a), Value::Integer(b)) => *a < (*b as f64),
            (Value::String(a), Value::String(b)) => a < b,
            _ => false,
        }
    }

    #[inline]
    pub fn less_equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a <= b,
            (Value::Number(a), Value::Number(b)) => a <= b,
            (Value::Integer(a), Value::Number(b)) => (*a as f64) <= *b,
            (Value::Number(a), Value::Integer(b)) => *a <= (*b as f64),
            (Value::String(a), Value::String(b)) => a <= b,
            _ => false,
        }
    }

    #[inline]
    pub fn greater_than(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a > b,
            (Value::Number(a), Value::Number(b)) => a > b,
            (Value::Integer(a), Value::Number(b)) => (*a as f64) > *b,
            (Value::Number(a), Value::Integer(b)) => *a > (*b as f64),
            (Value::String(a), Value::String(b)) => a > b,
            _ => false,
        }
    }

    #[inline]
    pub fn greater_equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a >= b,
            (Value::Number(a), Value::Number(b)) => a >= b,
            (Value::Integer(a), Value::Number(b)) => (*a as f64) >= *b,
            (Value::Number(a), Value::Integer(b)) => *a >= (*b as f64),
            (Value::String(a), Value::String(b)) => a >= b,
            _ => false,
        }
    }

    #[inline]
    pub fn equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Integer(a), Value::Number(b)) => (*a as f64) == *b,
            (Value::Number(a), Value::Integer(b)) => *a == (*b as f64),
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Result { is_ok: a_ok, value: a_val }, Value::Result { is_ok: b_ok, value: b_val }) => {
                a_ok == b_ok && a_val.equal(b_val)
            }
            (Value::Option { is_some: a_some, value: a_val }, Value::Option { is_some: b_some, value: b_val }) => {
                a_some == b_some && (*a_some == false || a_val.equal(b_val))
            }
            _ => false,
        }
    }

    /// Convert from string (for compatibility)
    pub fn from_string(s: String) -> Value {
        // Try integer first (faster)
        if let Ok(i) = s.parse::<i64>() {
            return Value::Integer(i);
        }
        // Try float
        if let Ok(f) = s.parse::<f64>() {
            return Value::Number(f);
        }
        // Try boolean
        match s.as_str() {
            "true" | "True" => Value::Boolean(true),
            "false" | "False" => Value::Boolean(false),
            "null" | "undefined" => Value::Null,
            _ => Value::String(s),
        }
    }

    /// Convert to f64 for numeric operations
    #[inline]
    pub fn to_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Integer(i) => Some(*i as f64),
            Value::String(s) => s.parse::<f64>().ok(),
            Value::Boolean(true) => Some(1.0),
            Value::Boolean(false) => Some(0.0),
            Value::Array(_) => None,
            Value::Map(_) => None,
            Value::Struct { .. } => None,
            Value::Result { .. } => None,
            Value::Option { .. } => None,
            Value::Null => None,
        }
    }

    /// Convert to i64 for integer operations
    #[inline]
    pub fn to_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            Value::Number(n) => Some(*n as i64),
            Value::String(s) => s.parse::<i64>().ok(),
            Value::Boolean(true) => Some(1),
            Value::Boolean(false) => Some(0),
            Value::Array(_) => None,
            Value::Map(_) => None,
            Value::Struct { .. } => None,
            Value::Result { .. } => None,
            Value::Option { .. } => None,
            Value::Null => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => {
                // Format numbers nicely - remove trailing .0 for whole numbers
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Integer(i) => write!(f, "{}", i),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", elements.join(", "))
            }
            Value::Map(map) => {
                let pairs: Vec<String> = map.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{{{}}}", pairs.join(", "))
            }
            Value::Struct { type_name, fields } => {
                let field_strs: Vec<String> = fields.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{} {{ {} }}", type_name, field_strs.join(", "))
            }
            Value::Result { is_ok, value } => {
                if *is_ok {
                    write!(f, "Ok({})", value)
                } else {
                    write!(f, "Err({})", value)
                }
            }
            Value::Option { is_some, value } => {
                if *is_some {
                    write!(f, "Some({})", value)
                } else {
                    write!(f, "None")
                }
            }
            Value::Null => write!(f, "null"),
        }
    }
}
