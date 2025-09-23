// src/backend/type_checker.rs
//! Type checker for comprehensive type validation

use crate::backend::types::Type;
use crate::frontend::parser::ast::*;
use std::collections::HashMap;

/// Type context for tracking variable and function types
#[derive(Debug, Clone)]
pub struct TypeContext {
    variables: HashMap<String, Type>,
    functions: HashMap<String, (Vec<Type>, Type)>, // (params, return_type)
    scopes: Vec<HashMap<String, Type>>,
}

/// Main type checker
pub struct TypeChecker {
    context: TypeContext,
    errors: Vec<String>,
}

impl TypeContext {
    pub fn new() -> Self {
        let mut context = TypeContext {
            variables: HashMap::new(),
            functions: HashMap::new(),
            scopes: vec![HashMap::new()],
        };
        
        // Add builtin functions
        context.add_builtins();
        context
    }
    
    fn add_builtins(&mut self) {
        // I/O functions
        self.functions.insert("print".to_string(), (vec![Type::Any], Type::Null));
        self.functions.insert("println".to_string(), (vec![Type::Any], Type::Null));
        self.functions.insert("input".to_string(), (vec![], Type::String));
        self.functions.insert("read".to_string(), (vec![Type::String], Type::String));
        self.functions.insert("write".to_string(), (vec![Type::String, Type::String], Type::Bool));
    }
    
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
    
    pub fn declare_variable(&mut self, name: String, var_type: Type) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, var_type);
        }
    }
    
    pub fn declare_function(&mut self, name: String, params: Vec<Type>, return_type: Type) {
        self.functions.insert(name, (params, return_type));
    }
    
    pub fn get_variable_type(&self, name: &str) -> Option<&Type> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(var_type) = scope.get(name) {
                return Some(var_type);
            }
        }
        None
    }
    
    pub fn get_function_signature(&self, name: &str) -> Option<&(Vec<Type>, Type)> {
        self.functions.get(name)
    }
    
    pub fn update_variable_type(&mut self, name: &str, new_type: Type) {
        // Update in the most recent scope where the variable exists
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), new_type);
                return;
            }
        }
    }
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            context: TypeContext::new(),
            errors: Vec::new(),
        }
    }
    
    pub fn check_program(&mut self, program: &Program) -> Vec<String> {
        self.errors.clear();
        
        // First pass: collect all function declarations
        for stmt in &program.statements {
            if let Statement::FunctionDeclaration(func_decl) = stmt {
                self.collect_function_declaration(func_decl);
            }
        }
        
        // Second pass: check all statements
        for stmt in &program.statements {
            self.check_statement(stmt);
        }
        
        self.errors.clone()
    }
    
    fn check_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDeclaration(var_decl) => {
                self.check_variable_declaration(var_decl);
            }
            Statement::FunctionDeclaration(func_decl) => {
                self.check_function_declaration(func_decl);
            }
            Statement::ExpressionStatement(expr_stmt) => {
                self.check_expression(&expr_stmt.expression);
            }
            Statement::BlockStatement(block_stmt) => {
                self.context.push_scope();
                for stmt in &block_stmt.statements {
                    self.check_statement(stmt);
                }
                self.context.pop_scope();
            }
            _ => {
                // Handle other statement types as needed
            }
        }
    }
    
    fn check_variable_declaration(&mut self, var_decl: &VariableDeclaration) {
        let var_name = &var_decl.name.name;
        
        // Get the declared type if any
        let declared_type = var_decl.type_annotation.as_ref()
            .map(|t| Type::from_annotation(t));
        
        // Get the inferred type from initializer if any
        let inferred_type = var_decl.initializer.as_ref()
            .map(|expr| self.check_expression(expr));
        
        match (declared_type, inferred_type) {
            // Both declared type and initializer present
            (Some(declared), Some(inferred)) => {
                if !inferred.can_assign_to(&declared) {
                    self.errors.push(format!(
                        "Type mismatch: variable '{}' declared as '{}' but assigned value of type '{}'",
                        var_name, declared, inferred
                    ));
                }
                self.context.declare_variable(var_name.clone(), declared);
            }
            
            // Only declared type, no initializer
            (Some(declared), None) => {
                self.context.declare_variable(var_name.clone(), declared);
            }
            
            // Only initializer, no declared type (use inferred type)
            (None, Some(inferred)) => {
                    self.context.declare_variable(var_name.clone(), inferred);
            }
            
            // Neither declared type nor initializer
            (None, None) => {
                self.context.declare_variable(var_name.clone(), Type::Any);
            }
        }
    }
    
    fn collect_function_declaration(&mut self, func_decl: &FunctionDeclaration) {
        let param_types: Vec<Type> = func_decl.parameters.iter()
            .map(|p| {
                if let Some(ref type_ann) = p.type_annotation {
                    Type::from_annotation(type_ann)
                } else {
                    Type::Any // Parameters without type annotations are flexible
                }
            })
            .collect();
        
        let return_type = func_decl.return_type.as_ref()
            .map(|t| Type::from_annotation(t))
            .unwrap_or(Type::Null);
        
        
        self.context.declare_function(
            func_decl.name.name.clone(),
            param_types,
            return_type
        );
    }
    
    fn check_function_declaration(&mut self, func_decl: &FunctionDeclaration) {
        // Create new scope for function
        self.context.push_scope();
        
        // Add parameters to scope
        for param in &func_decl.parameters {
            let param_type = if let Some(ref type_ann) = param.type_annotation {
                Type::from_annotation(type_ann)
            } else {
                Type::Any // Parameters without type annotations are flexible
            };
            self.context.declare_variable(param.name.name.clone(), param_type);
        }
        
        // Check function body
        for stmt in &func_decl.body.statements {
            self.check_statement(stmt);
        }
        
        self.context.pop_scope();
    }
    
    fn check_expression(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::IntegerLiteral(_) => Type::Int,
            Expression::FloatLiteral(_) => Type::Float,
            Expression::StringLiteral(_) => Type::String,
            Expression::BooleanLiteral(_) => Type::Bool,
            Expression::NullLiteral(_) => Type::Null,
            
            Expression::Identifier(ident) => {
                if let Some(var_type) = self.context.get_variable_type(&ident.name) {
                    var_type.clone()
                } else {
                    self.errors.push(format!("Undefined variable: '{}'", ident.name));
                    Type::Unknown
                }
            }
            
            Expression::BinaryExpression(bin_expr) => {
                let left_type = self.check_expression(&bin_expr.left);
                let right_type = self.check_expression(&bin_expr.right);
                
                self.check_binary_operation(&bin_expr.operator, &left_type, &right_type)
            }
            
            Expression::CallExpression(call_expr) => {
                self.check_call_expression(call_expr)
            }
            
            Expression::AssignmentExpression(assign_expr) => {
                self.check_assignment_expression(assign_expr)
            }
            
            _ => Type::Unknown,
        }
    }
    
    fn check_binary_operation(&mut self, op: &BinaryOperator, left: &Type, right: &Type) -> Type {
        match op {
            BinaryOperator::Add => {
                match (left, right) {
                    // String concatenation
                    (Type::String, _) | (_, Type::String) => Type::String,
                    // Numeric operations
                    (Type::Int, Type::Int) => Type::Int,
                    (Type::Float, Type::Float) => Type::Float,
                    (Type::Int, Type::Float) | (Type::Float, Type::Int) => Type::Float,
                    // Flexible types
                    (Type::Any, _) | (_, Type::Any) => Type::Any,
                    _ => {
                        self.errors.push(format!("Cannot add {} and {}", left, right));
                        Type::Unknown
                    }
                }
            }
            
            BinaryOperator::Subtract | BinaryOperator::Multiply | 
            BinaryOperator::Divide | BinaryOperator::Modulo => {
                match (left, right) {
                    (Type::Int, Type::Int) => Type::Int,
                    (Type::Float, Type::Float) => Type::Float,
                    (Type::Int, Type::Float) | (Type::Float, Type::Int) => Type::Float,
                    (Type::Any, _) | (_, Type::Any) => Type::Any,
                    _ => {
                        self.errors.push(format!("Cannot perform arithmetic on {} and {}", left, right));
                        Type::Unknown
                    }
                }
            }
            
            // Comparison operations always return bool
            BinaryOperator::Equal | BinaryOperator::NotEqual |
            BinaryOperator::Less | BinaryOperator::Greater |
            BinaryOperator::LessEqual | BinaryOperator::GreaterEqual => Type::Bool,
            
            // Logical operations
            BinaryOperator::And | BinaryOperator::Or => {
                if !matches!(left, Type::Bool | Type::Any) {
                    self.errors.push(format!("Left operand of logical operation must be bool, got {}", left));
                }
                if !matches!(right, Type::Bool | Type::Any) {
                    self.errors.push(format!("Right operand of logical operation must be bool, got {}", right));
                }
                Type::Bool
            }
            
            _ => Type::Unknown,
        }
    }
    
    fn check_call_expression(&mut self, call_expr: &CallExpression) -> Type {
        if let Expression::Identifier(func_name) = call_expr.callee.as_ref() {
            // Clone the function signature to avoid borrowing issues
            let func_signature = self.context.get_function_signature(&func_name.name).cloned();
            
            if let Some((param_types, return_type)) = func_signature {
                // Check argument count (flexible for input function)
                if func_name.name != "input" && param_types.len() != call_expr.arguments.len() {
                    self.errors.push(format!(
                        "Function '{}' expects {} arguments, got {}",
                        func_name.name, param_types.len(), call_expr.arguments.len()
                    ));
                }
                
                // Check argument types
                for (i, arg) in call_expr.arguments.iter().enumerate() {
                    let arg_type = self.check_expression(arg);
                    if i < param_types.len() {
                        let expected_type = &param_types[i];
                        if !arg_type.can_assign_to(expected_type) {
                            self.errors.push(format!(
                                "Argument {} of function '{}' expects type '{}', got '{}'",
                                i + 1, func_name.name, expected_type, arg_type
                            ));
                        }
                    }
                }
                
                return_type
            } else {
                self.errors.push(format!("Undefined function: '{}'", func_name.name));
                Type::Unknown
            }
        } else {
            // Complex callee expression
            self.check_expression(&call_expr.callee);
            Type::Unknown
        }
    }
    
    fn check_assignment_expression(&mut self, assign_expr: &AssignmentExpression) -> Type {
        if let Expression::Identifier(ident) = assign_expr.left.as_ref() {
            let value_type = self.check_expression(&assign_expr.right);
            
            if let Some(var_type) = self.context.get_variable_type(&ident.name) {
                // For flexible variables (Type::Any), update the type
                if matches!(var_type, Type::Any) {
                    self.context.update_variable_type(&ident.name, value_type.clone());
                } else if !value_type.can_assign_to(var_type) {
                    self.errors.push(format!(
                        "Cannot assign value of type '{}' to variable '{}' of type '{}'",
                        value_type, ident.name, var_type
                    ));
                }
            } else {
                self.errors.push(format!("Undefined variable: '{}'", ident.name));
            }
            
            value_type
        } else {
            self.check_expression(&assign_expr.left);
            self.check_expression(&assign_expr.right)
        }
    }
}
