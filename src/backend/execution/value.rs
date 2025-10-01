// src/backend/execution/value.rs
//! High-performance value representation for runtime

use std::fmt;

/// Optimized value type for runtime - avoids string conversions
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Integer(i64),
    String(String),
    Boolean(bool),
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
            Value::Null => false,
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
            Value::Null => write!(f, "null"),
        }
    }
}
