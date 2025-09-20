// src/backend/semantic/type_system.rs

use std::collections::HashMap;
use std::fmt;

/// Represents all possible types in the Razen language
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // Primitive types
    Int,
    Float,
    String,
    Bool,
    Char,
    Null,
    
    // Composite types
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>), // Key type, Value type
    
    // Function type
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    
    // User-defined types
    Struct(String),
    Enum(String),
    
    // Special types
    Any,
    Void,
    Unknown,
    
    // Generic type parameter
    Generic(String),
}

impl Type {
    /// Check if this type is compatible with another type
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            // Exact matches
            (Type::Int, Type::Int) |
            (Type::Float, Type::Float) |
            (Type::String, Type::String) |
            (Type::Bool, Type::Bool) |
            (Type::Char, Type::Char) |
            (Type::Null, Type::Null) |
            (Type::Void, Type::Void) => true,
            
            // Any type is compatible with everything
            (Type::Any, _) | (_, Type::Any) => true,
            
            // Unknown type during inference
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            
            // Numeric compatibility (int can be promoted to float)
            (Type::Int, Type::Float) => true,
            
            // Array compatibility
            (Type::Array(a), Type::Array(b)) => a.is_compatible_with(b),
            
            // Map compatibility
            (Type::Map(k1, v1), Type::Map(k2, v2)) => {
                k1.is_compatible_with(k2) && v1.is_compatible_with(v2)
            }
            
            // Function compatibility
            (Type::Function { params: p1, return_type: r1 }, 
             Type::Function { params: p2, return_type: r2 }) => {
                p1.len() == p2.len() &&
                p1.iter().zip(p2.iter()).all(|(a, b)| a.is_compatible_with(b)) &&
                r1.is_compatible_with(r2)
            }
            
            // Struct/Enum compatibility (same name)
            (Type::Struct(a), Type::Struct(b)) => a == b,
            (Type::Enum(a), Type::Enum(b)) => a == b,
            
            // Generic types
            (Type::Generic(a), Type::Generic(b)) => a == b,
            
            _ => false,
        }
    }
    
    /// Check if this type can be implicitly converted to another type
    pub fn can_convert_to(&self, other: &Type) -> bool {
        if self.is_compatible_with(other) {
            return true;
        }
        
        match (self, other) {
            // Numeric conversions
            (Type::Int, Type::Float) => true,
            (Type::Char, Type::String) => true,
            
            // Null can be converted to any nullable type
            (Type::Null, Type::Array(_)) |
            (Type::Null, Type::Map(_, _)) |
            (Type::Null, Type::String) => true,
            
            _ => false,
        }
    }
    
    /// Get the default value for this type
    pub fn default_value(&self) -> String {
        match self {
            Type::Int => "0".to_string(),
            Type::Float => "0.0".to_string(),
            Type::String => "\"\"".to_string(),
            Type::Bool => "false".to_string(),
            Type::Char => "'\\0'".to_string(),
            Type::Null => "null".to_string(),
            Type::Array(_) => "[]".to_string(),
            Type::Map(_, _) => "{}".to_string(),
            _ => "null".to_string(),
        }
    }
    
    /// Check if this is a numeric type
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float)
    }
    
    /// Check if this is a primitive type
    pub fn is_primitive(&self) -> bool {
        matches!(self, Type::Int | Type::Float | Type::String | Type::Bool | Type::Char | Type::Null)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "str"),
            Type::Bool => write!(f, "bool"),
            Type::Char => write!(f, "char"),
            Type::Null => write!(f, "null"),
            Type::Array(inner) => write!(f, "[{}]", inner),
            Type::Map(key, value) => write!(f, "map<{}, {}>", key, value),
            Type::Function { params, return_type } => {
                write!(f, "fun(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", return_type)
            }
            Type::Struct(name) => write!(f, "{}", name),
            Type::Enum(name) => write!(f, "{}", name),
            Type::Any => write!(f, "any"),
            Type::Void => write!(f, "void"),
            Type::Unknown => write!(f, "?"),
            Type::Generic(name) => write!(f, "{}", name),
        }
    }
}

/// Type checker for semantic analysis
pub struct TypeChecker {
    /// Type definitions for user-defined types
    type_definitions: HashMap<String, TypeDefinition>,
    
    /// Current generic type bindings
    generic_bindings: HashMap<String, Type>,
}

#[derive(Debug, Clone)]
pub enum TypeDefinition {
    Struct {
        fields: HashMap<String, Type>,
    },
    Enum {
        variants: HashMap<String, Option<Type>>,
    },
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            type_definitions: HashMap::new(),
            generic_bindings: HashMap::new(),
        }
    }
    
    /// Register a struct type definition
    pub fn register_struct(&mut self, name: String, fields: HashMap<String, Type>) {
        self.type_definitions.insert(name, TypeDefinition::Struct { fields });
    }
    
    /// Register an enum type definition
    pub fn register_enum(&mut self, name: String, variants: HashMap<String, Option<Type>>) {
        self.type_definitions.insert(name, TypeDefinition::Enum { variants });
    }
    
    /// Get a type definition
    pub fn get_type_definition(&self, name: &str) -> Option<&TypeDefinition> {
        self.type_definitions.get(name)
    }
    
    /// Check if two types are compatible for assignment
    pub fn check_assignment(&self, target: &Type, source: &Type) -> bool {
        source.can_convert_to(target)
    }
    
    /// Infer the result type of a binary operation
    pub fn infer_binary_op_type(&self, left: &Type, right: &Type, op: &str) -> Option<Type> {
        match op {
            // Arithmetic operations
            "+" | "-" | "*" | "/" | "%" => {
                match (left, right) {
                    (Type::Int, Type::Int) => Some(Type::Int),
                    (Type::Float, Type::Float) => Some(Type::Float),
                    (Type::Int, Type::Float) | (Type::Float, Type::Int) => Some(Type::Float),
                    (Type::String, Type::String) if op == "+" => Some(Type::String),
                    _ => None,
                }
            }
            
            // Comparison operations
            "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                if left.is_compatible_with(right) {
                    Some(Type::Bool)
                } else {
                    None
                }
            }
            
            // Logical operations
            "&&" | "||" => {
                match (left, right) {
                    (Type::Bool, Type::Bool) => Some(Type::Bool),
                    _ => None,
                }
            }
            
            // Bitwise operations
            "&" | "|" | "^" | "<<" | ">>" => {
                match (left, right) {
                    (Type::Int, Type::Int) => Some(Type::Int),
                    _ => None,
                }
            }
            
            _ => None,
        }
    }
    
    /// Infer the result type of a unary operation
    pub fn infer_unary_op_type(&self, operand: &Type, op: &str) -> Option<Type> {
        match op {
            "-" => {
                match operand {
                    Type::Int => Some(Type::Int),
                    Type::Float => Some(Type::Float),
                    _ => None,
                }
            }
            
            "!" => {
                match operand {
                    Type::Bool => Some(Type::Bool),
                    _ => None,
                }
            }
            
            "~" => {
                match operand {
                    Type::Int => Some(Type::Int),
                    _ => None,
                }
            }
            
            _ => None,
        }
    }
    
    /// Parse a type from a string representation
    pub fn parse_type(&self, type_str: &str) -> Option<Type> {
        match type_str {
            "int" => Some(Type::Int),
            "float" => Some(Type::Float),
            "str" => Some(Type::String),
            "bool" => Some(Type::Bool),
            "char" => Some(Type::Char),
            "any" => Some(Type::Any),
            "void" => Some(Type::Void),
            _ => {
                // Check if it's a user-defined type
                if self.type_definitions.contains_key(type_str) {
                    Some(Type::Struct(type_str.to_string()))
                } else {
                    None
                }
            }
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
