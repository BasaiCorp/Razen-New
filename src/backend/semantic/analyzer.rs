// src/backend/semantic/analyzer.rs

use std::collections::HashMap;

use crate::frontend::parser::ast::*;
use crate::frontend::diagnostics::{Diagnostics, Diagnostic, DiagnosticKind, Severity, Label};
use crate::backend::semantic::{
    AnalyzedProgram, SymbolTable, Symbol, SymbolKind, Type, TypeChecker, ScopeManager
};
use crate::backend::semantic::scope::ScopeType;

/// Main semantic analyzer for the Razen language
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    type_checker: TypeChecker,
    scope_manager: ScopeManager,
    diagnostics: Diagnostics,
    current_function_return_type: Option<Type>,
    node_counter: usize,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: SymbolTable::new(),
            type_checker: TypeChecker::new(),
            scope_manager: ScopeManager::new(),
            diagnostics: Diagnostics::new(),
            current_function_return_type: None,
            node_counter: 0,
        }
    }
    
    /// Analyze a program and return the analyzed result or diagnostics
    pub fn analyze(&mut self, program: Program) -> Result<AnalyzedProgram, Diagnostics> {
        // Reset state
        self.diagnostics = Diagnostics::new();
        self.node_counter = 0;
        
        // Analyze all statements
        for statement in &program.statements {
            self.analyze_statement(statement);
        }
        
        // Check for unused variables (warnings)
        self.check_unused_symbols();
        
        // Return result
        if self.diagnostics.has_errors() {
            Err(std::mem::take(&mut self.diagnostics))
        } else {
            Ok(AnalyzedProgram::new(program, self.symbol_table.clone()))
        }
    }
    
    /// Analyze a statement
    fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VariableDeclaration(decl) => self.analyze_variable_declaration(decl),
            Statement::ConstantDeclaration(decl) => self.analyze_constant_declaration(decl),
            Statement::FunctionDeclaration(decl) => self.analyze_function_declaration(decl),
            Statement::StructDeclaration(decl) => self.analyze_struct_declaration(decl),
            Statement::EnumDeclaration(decl) => self.analyze_enum_declaration(decl),
            Statement::IfStatement(stmt) => self.analyze_if_statement(stmt),
            Statement::WhileStatement(stmt) => self.analyze_while_statement(stmt),
            Statement::ForStatement(stmt) => self.analyze_for_statement(stmt),
            Statement::MatchStatement(stmt) => self.analyze_match_statement(stmt),
            Statement::TryStatement(stmt) => self.analyze_try_statement(stmt),
            Statement::ReturnStatement(stmt) => self.analyze_return_statement(stmt),
            Statement::BreakStatement(_) => self.analyze_break_statement(),
            Statement::ContinueStatement(_) => self.analyze_continue_statement(),
            Statement::ThrowStatement(stmt) => self.analyze_throw_statement(stmt),
            Statement::ExpressionStatement(stmt) => {
                self.analyze_expression(&stmt.expression);
            }
            Statement::BlockStatement(stmt) => self.analyze_block_statement(stmt),
            Statement::ModuleDeclaration(_) => {
                // Module declarations are handled at a higher level
            }
            Statement::UseStatement(_) => {
                // Use statements are handled at a higher level
            }
        }
    }
    
    /// Analyze variable declaration
    fn analyze_variable_declaration(&mut self, decl: &VariableDeclaration) {
        let var_name = &decl.name.name;
        
        // Check if variable already exists in current scope
        if self.symbol_table.exists_in_current_scope(var_name) {
            self.error(
                format!("Variable '{}' is already declared in this scope", var_name),
                None, // TODO: Add span support to AST nodes
            );
            return;
        }
        
        // Determine the type
        let var_type = if let Some(ref type_annotation) = decl.type_annotation {
            self.resolve_type(type_annotation)
        } else if let Some(ref initializer) = decl.initializer {
            self.analyze_expression(initializer)
        } else {
            self.error(
                "Variable declaration must have either a type annotation or an initializer".to_string(),
                None, // TODO: Add span support to AST nodes
            );
            return;
        };
        
        // Check initializer type compatibility if both are present
        if let (Some(ref type_annotation), Some(ref initializer)) = (&decl.type_annotation, &decl.initializer) {
            let declared_type = self.resolve_type(type_annotation);
            let init_type = self.analyze_expression(initializer);
            
            if !self.type_checker.check_assignment(&declared_type, &init_type) {
                self.error(
                    format!(
                        "Cannot assign value of type '{}' to variable of type '{}'",
                        init_type, declared_type
                    ),
                    None, // TODO: Add span support to expressions
                );
            }
        }
        
        // Create and add symbol
        let mut symbol = Symbol::variable(
            var_name.clone(),
            var_type,
            true, // Variables are mutable by default in Razen
            None, // TODO: Add span support to AST nodes
            self.symbol_table.current_scope(),
        );
        
        if decl.initializer.is_some() {
            symbol.set_initialized();
        }
        
        self.symbol_table.add_symbol(symbol);
    }
    
    /// Analyze constant declaration
    fn analyze_constant_declaration(&mut self, decl: &ConstantDeclaration) {
        let const_name = &decl.name.name;
        
        // Check if constant already exists in current scope
        if self.symbol_table.exists_in_current_scope(const_name) {
            self.error(
                format!("Constant '{}' is already declared in this scope", const_name),
                None, // TODO: Add span support to AST nodes
            );
            return;
        }
        
        // Constants must have an initializer
        let init_type = self.analyze_expression(&decl.initializer);
        
        // Determine the type
        let const_type = if let Some(ref type_annotation) = decl.type_annotation {
            let declared_type = self.resolve_type(type_annotation);
            
            if !self.type_checker.check_assignment(&declared_type, &init_type) {
                self.error(
                    format!(
                        "Cannot assign value of type '{}' to constant of type '{}'",
                        init_type, declared_type
                    ),
                    None, // TODO: Add span support to expressions
                );
            }
            
            declared_type
        } else {
            init_type
        };
        
        // Create and add symbol
        let symbol = Symbol::constant(
            const_name.clone(),
            const_type,
            None, // TODO: Add span support to AST nodes
            self.symbol_table.current_scope(),
        );
        
        self.symbol_table.add_symbol(symbol);
    }
    
    /// Analyze function declaration
    fn analyze_function_declaration(&mut self, decl: &FunctionDeclaration) {
        let func_name = &decl.name.name;
        
        // Check if function already exists in current scope
        if self.symbol_table.exists_in_current_scope(func_name) {
            self.error(
                format!("Function '{}' is already declared in this scope", func_name),
                None, // TODO: Add span support to AST nodes
            );
            return;
        }
        
        // Resolve parameter types
        let mut params = Vec::new();
        for param in &decl.parameters {
            let param_type = self.resolve_type(&param.type_annotation);
            params.push((param.name.name.clone(), param_type));
        }
        
        // Resolve return type
        let return_type = if let Some(ref ret_type) = decl.return_type {
            self.resolve_type(ret_type)
        } else {
            Type::Void
        };
        
        // Create function symbol
        let symbol = Symbol::function(
            func_name.clone(),
            params.clone(),
            return_type.clone(),
            None, // TODO: Add span support to AST nodes
            self.symbol_table.current_scope(),
        );
        
        self.symbol_table.add_symbol(symbol);
        
        // Analyze function body in new scope
        self.symbol_table.push_scope();
        self.scope_manager.enter_scope(ScopeType::Function);
        
        let old_return_type = self.current_function_return_type.clone();
        self.current_function_return_type = Some(return_type);
        
        // Add parameters to function scope
        for (param_name, param_type) in params {
            let param_symbol = Symbol::new(
                param_name,
                SymbolKind::Parameter,
                param_type,
                None,
                self.symbol_table.current_scope(),
            );
            self.symbol_table.add_symbol(param_symbol);
        }
        
        // Analyze function body
        self.analyze_block_statement(&decl.body);
        
        // Restore previous state
        self.current_function_return_type = old_return_type;
        self.scope_manager.exit_scope();
        self.symbol_table.pop_scope();
    }
    
    /// Analyze struct declaration
    fn analyze_struct_declaration(&mut self, decl: &StructDeclaration) {
        let struct_name = &decl.name.name;
        
        // Check if struct already exists
        if self.symbol_table.exists_in_current_scope(struct_name) {
            self.error(
                format!("Struct '{}' is already declared in this scope", struct_name),
                None, // TODO: Add span support to AST nodes
            );
            return;
        }
        
        // Resolve field types
        let mut fields = HashMap::new();
        for field in &decl.fields {
            let field_type = self.resolve_type(&field.type_annotation);
            fields.insert(field.name.name.clone(), field_type);
        }
        
        // Register with type checker
        self.type_checker.register_struct(struct_name.clone(), fields.clone());
        
        // Create struct symbol
        let symbol = Symbol::struct_type(
            struct_name.clone(),
            fields,
            None, // TODO: Add span support to AST nodes
            self.symbol_table.current_scope(),
        );
        
        self.symbol_table.add_symbol(symbol);
    }
    
    /// Analyze enum declaration
    fn analyze_enum_declaration(&mut self, decl: &EnumDeclaration) {
        let enum_name = &decl.name.name;
        
        // Check if enum already exists
        if self.symbol_table.exists_in_current_scope(enum_name) {
            self.error(
                format!("Enum '{}' is already declared in this scope", enum_name),
                None, // TODO: Add span support to AST nodes
            );
            return;
        }
        
        // Resolve variant types
        let mut variants = HashMap::new();
        for variant in &decl.variants {
            let variant_type = variant.fields.as_ref()
                .map(|fields| {
                    if fields.len() == 1 {
                        self.resolve_type(&fields[0])
                    } else {
                        // Multiple fields - create a tuple type (simplified as Any for now)
                        Type::Any
                    }
                });
            variants.insert(variant.name.name.clone(), variant_type);
        }
        
        // Register with type checker
        self.type_checker.register_enum(enum_name.clone(), variants.clone());
        
        // Create enum symbol
        let symbol = Symbol::enum_type(
            enum_name.clone(),
            variants,
            None, // TODO: Add span support to AST nodes
            self.symbol_table.current_scope(),
        );
        
        self.symbol_table.add_symbol(symbol);
    }
    
    /// Analyze if statement
    fn analyze_if_statement(&mut self, stmt: &IfStatement) {
        // Analyze condition
        let condition_type = self.analyze_expression(&stmt.condition);
        if condition_type != Type::Bool {
            self.error(
                format!("If condition must be of type 'bool', found '{}'", condition_type),
                None, // TODO: Add span support to expressions
            );
        }
        
        // Analyze then branch
        self.symbol_table.push_scope();
        self.scope_manager.enter_scope(ScopeType::Conditional);
        self.analyze_statement(&stmt.then_branch);
        self.scope_manager.exit_scope();
        self.symbol_table.pop_scope();
        
        // Analyze elif branches
        for elif_branch in &stmt.elif_branches {
            let elif_condition_type = self.analyze_expression(&elif_branch.condition);
            if elif_condition_type != Type::Bool {
                self.error(
                    format!("Elif condition must be of type 'bool', found '{}'", elif_condition_type),
                    None,
                );
            }
            
            self.symbol_table.push_scope();
            self.scope_manager.enter_scope(ScopeType::Conditional);
            self.analyze_statement(&elif_branch.body);
            self.scope_manager.exit_scope();
            self.symbol_table.pop_scope();
        }
        
        // Analyze else branch if present
        if let Some(ref else_branch) = stmt.else_branch {
            self.symbol_table.push_scope();
            self.scope_manager.enter_scope(ScopeType::Conditional);
            self.analyze_statement(else_branch);
            self.scope_manager.exit_scope();
            self.symbol_table.pop_scope();
        }
    }
    
    /// Analyze while statement
    fn analyze_while_statement(&mut self, stmt: &WhileStatement) {
        // Analyze condition
        let condition_type = self.analyze_expression(&stmt.condition);
        if condition_type != Type::Bool {
            self.error(
                format!("While condition must be of type 'bool', found '{}'", condition_type),
                None,
            );
        }
        
        // Analyze body in loop scope
        self.symbol_table.push_scope();
        self.scope_manager.enter_scope(ScopeType::Loop);
        self.analyze_statement(&stmt.body);
        self.scope_manager.exit_scope();
        self.symbol_table.pop_scope();
    }
    
    /// Analyze for statement
    fn analyze_for_statement(&mut self, stmt: &ForStatement) {
        self.symbol_table.push_scope();
        self.scope_manager.enter_scope(ScopeType::Loop);
        
        // Analyze iterable
        let iterable_type = self.analyze_expression(&stmt.iterable);
        
        // Check if iterable is actually iterable
        match iterable_type {
            Type::Array(_) | Type::String => {
                // Valid iterable types
            }
            _ => {
                self.error(
                    format!("Cannot iterate over value of type '{}'", iterable_type),
                    None,
                );
            }
        }
        
        // Add loop variable to scope
        let loop_var_type = match iterable_type {
            Type::Array(ref inner) => (**inner).clone(),
            Type::String => Type::Char,
            _ => Type::Unknown,
        };
        
        let loop_var_symbol = Symbol::variable(
            stmt.variable.name.clone(),
            loop_var_type,
            false, // Loop variables are immutable
            None, // TODO: Add span support
            self.symbol_table.current_scope(),
        );
        
        self.symbol_table.add_symbol(loop_var_symbol);
        
        // Analyze body
        self.analyze_statement(&stmt.body);
        
        self.scope_manager.exit_scope();
        self.symbol_table.pop_scope();
    }
    
    /// Analyze match statement
    fn analyze_match_statement(&mut self, stmt: &MatchStatement) {
        let _expr_type = self.analyze_expression(&stmt.expression);
        
        for arm in &stmt.arms {
            // TODO: Implement pattern matching type checking
            self.symbol_table.push_scope();
            self.scope_manager.enter_scope(ScopeType::Match);
            self.analyze_expression(&arm.body);
            self.scope_manager.exit_scope();
            self.symbol_table.pop_scope();
        }
    }
    
    /// Analyze try statement
    fn analyze_try_statement(&mut self, stmt: &TryStatement) {
        // Analyze try block
        self.symbol_table.push_scope();
        self.scope_manager.enter_scope(ScopeType::Try);
        self.analyze_block_statement(&stmt.body);
        self.scope_manager.exit_scope();
        self.symbol_table.pop_scope();
        
        // Analyze catch clause if present
        if let Some(ref catch_clause) = stmt.catch_clause {
            self.symbol_table.push_scope();
            self.scope_manager.enter_scope(ScopeType::Block);
            
            // Add exception variable to scope if present
            if let Some(ref exception_var) = catch_clause.parameter {
                let exception_symbol = Symbol::variable(
                    exception_var.name.clone(),
                    Type::Any, // Default exception type
                    false,
                    None, // TODO: Add span support
                    self.symbol_table.current_scope(),
                );
                
                self.symbol_table.add_symbol(exception_symbol);
            }
            
            self.analyze_block_statement(&catch_clause.body);
            self.scope_manager.exit_scope();
            self.symbol_table.pop_scope();
        }
    }
    
    /// Analyze return statement
    fn analyze_return_statement(&mut self, stmt: &ReturnStatement) {
        if !self.scope_manager.can_return() {
            self.error(
                "Return statement outside of function".to_string(),
                None, // TODO: Add span support
            );
            return;
        }
        
        let return_type = if let Some(ref value) = stmt.value {
            self.analyze_expression(value)
        } else {
            Type::Void
        };
        
        if let Some(ref expected_type) = self.current_function_return_type {
            if !self.type_checker.check_assignment(expected_type, &return_type) {
                self.error(
                    format!(
                        "Cannot return value of type '{}' from function expecting '{}'",
                        return_type, expected_type
                    ),
                    None, // TODO: Add span support
                );
            }
        }
    }
    
    /// Analyze break statement
    fn analyze_break_statement(&mut self) {
        if !self.scope_manager.can_break() {
            self.error(
                "Break statement outside of loop".to_string(),
                None,
            );
        }
    }
    
    /// Analyze continue statement
    fn analyze_continue_statement(&mut self) {
        if !self.scope_manager.can_continue() {
            self.error(
                "Continue statement outside of loop".to_string(),
                None,
            );
        }
    }
    
    /// Analyze throw statement
    fn analyze_throw_statement(&mut self, stmt: &ThrowStatement) {
        // Analyze the thrown expression
        self.analyze_expression(&stmt.value);
    }
    
    /// Analyze block statement
    fn analyze_block_statement(&mut self, stmt: &BlockStatement) {
        for statement in &stmt.statements {
            self.analyze_statement(statement);
        }
    }
    
    /// Analyze an expression and return its type
    fn analyze_expression(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::IntegerLiteral(_) => Type::Int,
            Expression::FloatLiteral(_) => Type::Float,
            Expression::StringLiteral(_) => Type::String,
            Expression::BooleanLiteral(_) => Type::Bool,
            Expression::NullLiteral(_) => Type::Null,
            
            Expression::Identifier(ident) => {
                if let Some(symbol) = self.symbol_table.lookup(&ident.name) {
                    let symbol_type = symbol.ty.clone();
                    self.symbol_table.mark_used(&ident.name);
                    symbol_type
                } else {
                    self.error(
                        format!("Undefined variable '{}'", ident.name),
                        None, // TODO: Add span support to Identifier
                    );
                    Type::Unknown
                }
            }
            
            Expression::BinaryExpression(expr) => {
                let left_type = self.analyze_expression(&expr.left);
                let right_type = self.analyze_expression(&expr.right);
                
                let op_str = match expr.operator {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Modulo => "%",
                    BinaryOperator::Power => "**",
                    BinaryOperator::Equal => "==",
                    BinaryOperator::NotEqual => "!=",
                    BinaryOperator::Less => "<",
                    BinaryOperator::Greater => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
                    BinaryOperator::And => "&&",
                    BinaryOperator::Or => "||",
                    BinaryOperator::BitwiseAnd => "&",
                    BinaryOperator::BitwiseOr => "|",
                    BinaryOperator::BitwiseXor => "^",
                    BinaryOperator::LeftShift => "<<",
                    BinaryOperator::RightShift => ">>",
                    BinaryOperator::Range => "..",
                };
                
                if let Some(result_type) = self.type_checker.infer_binary_op_type(&left_type, &right_type, op_str) {
                    result_type
                } else {
                    self.error(
                        format!(
                            "Cannot apply operator '{}' to operands of type '{}' and '{}'",
                            op_str, left_type, right_type
                        ),
                        None,
                    );
                    Type::Unknown
                }
            }
            
            Expression::UnaryExpression(expr) => {
                let operand_type = self.analyze_expression(&expr.operand);
                
                let op_str = match expr.operator {
                    UnaryOperator::Not => "!",
                    UnaryOperator::Minus => "-",
                    UnaryOperator::Plus => "+",
                    UnaryOperator::BitwiseNot => "~",
                    UnaryOperator::PreIncrement => "++",
                    UnaryOperator::PostIncrement => "++",
                    UnaryOperator::PreDecrement => "--",
                    UnaryOperator::PostDecrement => "--",
                };
                
                if let Some(result_type) = self.type_checker.infer_unary_op_type(&operand_type, op_str) {
                    result_type
                } else {
                    self.error(
                        format!(
                            "Cannot apply unary operator '{}' to operand of type '{}'",
                            op_str, operand_type
                        ),
                        None,
                    );
                    Type::Unknown
                }
            }
            
            Expression::AssignmentExpression(expr) => {
                let target_type = self.analyze_expression(&expr.left);
                let value_type = self.analyze_expression(&expr.right);
                
                // Check if target is assignable
                if let Expression::Identifier(ident) = expr.left.as_ref() {
                    if let Some(symbol) = self.symbol_table.lookup(&ident.name) {
                        if !symbol.is_mutable() {
                            self.error(
                                format!("Cannot assign to immutable variable '{}'", ident.name),
                                None, // TODO: Add span support to expressions
                            );
                        }
                    }
                }
                
                if !self.type_checker.check_assignment(&target_type, &value_type) {
                    self.error(
                        format!(
                            "Cannot assign value of type '{}' to target of type '{}'",
                            value_type, target_type
                        ),
                        None,
                    );
                }
                
                target_type
            }
            
            Expression::CallExpression(expr) => {
                let func_type = self.analyze_expression(&expr.callee);
                
                // Analyze arguments
                let arg_types: Vec<Type> = expr.arguments.iter()
                    .map(|arg| self.analyze_expression(arg))
                    .collect();
                
                // Check function call
                match func_type {
                    Type::Function { params, return_type } => {
                        if params.len() != arg_types.len() {
                            self.error(
                                format!(
                                    "Function expects {} arguments, got {}",
                                    params.len(), arg_types.len()
                                ),
                                None,
                            );
                        } else {
                            for (i, (param_type, arg_type)) in params.iter().zip(arg_types.iter()).enumerate() {
                                if !self.type_checker.check_assignment(param_type, arg_type) {
                                    self.error(
                                        format!(
                                            "Argument {} has type '{}', expected '{}'",
                                            i + 1, arg_type, param_type
                                        ),
                                        None,
                                    );
                                }
                            }
                        }
                        
                        *return_type
                    }
                    _ => {
                        self.error(
                            format!("Cannot call value of type '{}'", func_type),
                            None,
                        );
                        Type::Unknown
                    }
                }
            }
            
            Expression::MemberExpression(expr) => {
                let _object_type = self.analyze_expression(&expr.object);
                
                // TODO: Implement member access type checking
                Type::Unknown
            }
            
            Expression::IndexExpression(expr) => {
                let object_type = self.analyze_expression(&expr.object);
                let index_type = self.analyze_expression(&expr.index);
                
                match object_type {
                    Type::Array(ref inner) => {
                        if index_type != Type::Int {
                            self.error(
                                format!("Array index must be of type 'int', found '{}'", index_type),
                                None,
                            );
                        }
                        (**inner).clone()
                    }
                    Type::Map(_, ref value_type) => {
                        (**value_type).clone()
                    }
                    Type::String => {
                        if index_type != Type::Int {
                            self.error(
                                format!("String index must be of type 'int', found '{}'", index_type),
                                None,
                            );
                        }
                        Type::Char
                    }
                    _ => {
                        self.error(
                            format!("Cannot index value of type '{}'", object_type),
                            None,
                        );
                        Type::Unknown
                    }
                }
            }
            
            Expression::ArrayLiteral(expr) => {
                if expr.elements.is_empty() {
                    Type::Array(Box::new(Type::Unknown))
                } else {
                    let first_type = self.analyze_expression(&expr.elements[0]);
                    
                    // Check that all elements have compatible types
                    for element in &expr.elements[1..] {
                        let element_type = self.analyze_expression(element);
                        if !first_type.is_compatible_with(&element_type) {
                            self.error(
                                format!(
                                    "Array elements must have compatible types, found '{}' and '{}'",
                                    first_type, element_type
                                ),
                                None,
                            );
                        }
                    }
                    
                    Type::Array(Box::new(first_type))
                }
            }
            
            Expression::InterpolatedString(expr) => {
                // Analyze all expressions in the interpolated string
                for part in &expr.parts {
                    if let InterpolationPart::Expression(ref expr) = part {
                        self.analyze_expression(expr);
                    }
                }
                Type::String
            }
            
            Expression::RangeExpression(expr) => {
                let start_type = self.analyze_expression(&expr.start);
                let end_type = self.analyze_expression(&expr.end);
                
                if start_type != Type::Int || end_type != Type::Int {
                    self.error(
                        "Range bounds must be integers".to_string(),
                        None,
                    );
                }
                
                Type::Array(Box::new(Type::Int))
            }
            
            Expression::GroupingExpression(expr) => {
                self.analyze_expression(&expr.expression)
            }
        }
    }
    
    /// Resolve a type annotation to a Type
    fn resolve_type(&self, type_annotation: &TypeAnnotation) -> Type {
        match type_annotation {
            TypeAnnotation::Int => Type::Int,
            TypeAnnotation::Float => Type::Float,
            TypeAnnotation::String => Type::String,
            TypeAnnotation::Bool => Type::Bool,
            TypeAnnotation::Char => Type::Char,
            TypeAnnotation::Any => Type::Any,
            TypeAnnotation::Custom(name) => {
                // Check if it's a user-defined type
                if self.type_checker.get_type_definition(&name.name).is_some() {
                    Type::Struct(name.name.clone())
                } else {
                    Type::Unknown
                }
            }
            TypeAnnotation::Array(inner) => {
                Type::Array(Box::new(self.resolve_type(inner)))
            }
            TypeAnnotation::Map(key, value) => {
                Type::Map(
                    Box::new(self.resolve_type(key)),
                    Box::new(self.resolve_type(value))
                )
            }
        }
    }
    
    /// Check for unused symbols and emit warnings
    fn check_unused_symbols(&mut self) {
        // Collect unused symbols first to avoid borrowing issues
        let unused_symbols: Vec<_> = self.symbol_table.get_unused_symbols()
            .into_iter()
            .filter(|symbol| matches!(symbol.kind, SymbolKind::Variable { .. } | SymbolKind::Constant))
            .map(|symbol| (
                match symbol.kind {
                    SymbolKind::Variable { .. } => "variable",
                    SymbolKind::Constant => "constant",
                    _ => "symbol",
                },
                symbol.name.clone(),
                symbol.span.clone(),
            ))
            .collect();
        
        // Now emit warnings
        for (symbol_type, name, span) in unused_symbols {
            self.warning(
                format!("Unused {}: '{}'", symbol_type, name),
                span,
            );
        }
    }
    
    /// Helper method to emit an error
    fn error(&mut self, message: String, span: Option<crate::frontend::diagnostics::Span>) {
        let mut diagnostic = Diagnostic::new(DiagnosticKind::custom(message))
            .with_severity(Severity::Error);
        
        if let Some(span) = span {
            diagnostic = diagnostic.with_label(Label::primary(span));
        }
        
        self.diagnostics.add(diagnostic);
    }
    
    /// Helper method to emit a warning
    fn warning(&mut self, message: String, span: Option<crate::frontend::diagnostics::Span>) {
        let mut diagnostic = Diagnostic::new(DiagnosticKind::custom(message))
            .with_severity(Severity::Warning);
        
        if let Some(span) = span {
            diagnostic = diagnostic.with_label(Label::primary(span));
        }
        
        self.diagnostics.add(diagnostic);
    }
    
    /// Get next node ID for type annotations
    fn next_node_id(&mut self) -> usize {
        let id = self.node_counter;
        self.node_counter += 1;
        id
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
