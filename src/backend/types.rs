// src/backend/types.rs
//! Core type system for the Razen programming language

use crate::frontend::parser::ast::*;
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
    
    // Special types
    Any,        // For flexible variables without type annotation
    Unknown,    // For unresolved types during inference
    
    // Composite types
    Array(Box<Type>),
    Function(Vec<Type>, Box<Type>), // (parameters, return_type)
    Custom(String),
}

impl Type {
    /// Check if this type can be assigned to another type
    pub fn can_assign_to(&self, target: &Type) -> bool {
        match (self, target) {
            // Exact matches
            (Type::Int, Type::Int) |
            (Type::Float, Type::Float) |
            (Type::String, Type::String) |
            (Type::Bool, Type::Bool) |
            (Type::Char, Type::Char) |
            (Type::Null, Type::Null) => true,
            
            // Any type is flexible with everything
            (Type::Any, _) | (_, Type::Any) => true,
            
            // Unknown type during inference
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            
            // Numeric coercions
            (Type::Int, Type::Float) | (Type::Float, Type::Int) => true,
            
            // Everything else is incompatible
            _ => false,
        }
    }
    
    /// Check if this type can be used in string concatenation (more permissive)
    pub fn can_concatenate_with_string(&self) -> bool {
        // For string concatenation, any type can be converted to string
        true
    }
    
    /// Convert TypeAnnotation to Type
    pub fn from_annotation(annotation: &TypeAnnotation) -> Type {
        match annotation {
            TypeAnnotation::Int => Type::Int,
            TypeAnnotation::Float => Type::Float,
            TypeAnnotation::String => Type::String,
            TypeAnnotation::Bool => Type::Bool,
            TypeAnnotation::Char => Type::Char,
            TypeAnnotation::Any => Type::Any,
            TypeAnnotation::Array(inner) => Type::Array(Box::new(Type::from_annotation(inner))),
            TypeAnnotation::Custom(ident) => Type::Custom(ident.name.clone()),
            _ => Type::Any,
        }
    }
    
    /// Infer type from a literal expression
    pub fn from_literal(expr: &Expression) -> Type {
        match expr {
            Expression::IntegerLiteral(_) => Type::Int,
            Expression::FloatLiteral(_) => Type::Float,
            Expression::StringLiteral(_) => Type::String,
            Expression::CharacterLiteral(_) => Type::Char,
            Expression::BooleanLiteral(_) => Type::Bool,
            Expression::NullLiteral(_) => Type::Null,
            _ => Type::Unknown,
        }
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
            Type::Any => write!(f, "any"),
            Type::Unknown => write!(f, "unknown"),
            Type::Array(inner) => write!(f, "[{}]", inner),
            Type::Function(params, ret) => {
                write!(f, "fun(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", ret)
            }
            Type::Custom(name) => write!(f, "{}", name),
        }
    }
}
