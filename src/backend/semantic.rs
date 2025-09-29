// src/backend/semantic.rs
//! Professional semantic analyzer for the Razen language
//! Performs type checking, scope analysis, and semantic validation

use crate::backend::type_checker::TypeChecker;
use crate::frontend::diagnostics::{helpers, Diagnostic, Diagnostics, Position, Span};
use crate::frontend::parser::ast::*;
use crate::frontend::module_system::{ModuleResolver, VisibilityChecker, ModuleError};
use crate::frontend::module_system::resolver::ResolvedModule;
use std::collections::HashMap;
use std::path::PathBuf;

/// Semantic analyzer that validates the AST and reports errors
pub struct SemanticAnalyzer {
    diagnostics: Diagnostics,
    symbol_table: SymbolTable,
    _type_checker: TypeChecker, // Reserved for future type checking integration
    current_function: Option<String>,
    in_loop: bool,
    source_lines: Vec<String>,
    module_resolver: Option<ModuleResolver>,
    visibility_checker: VisibilityChecker,
    current_file: Option<PathBuf>,
}

/// Symbol table for tracking variables and functions
#[derive(Debug, Clone)]
struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
    functions: HashMap<String, FunctionSymbol>,
    structs: HashMap<String, StructSymbol>,
    methods: HashMap<String, Vec<MethodSymbol>>, // type_name -> methods
}

#[derive(Debug, Clone)]
struct Symbol {
    _name: String,
    symbol_type: SymbolType,
    defined_at: Position,
    used: bool,
    mutable: bool,
}

#[derive(Debug, Clone)]
struct FunctionSymbol {
    _name: String,
    parameters: Vec<String>,
    return_type: Option<String>,
    defined_at: Position,
}

#[derive(Debug, Clone)]
struct StructSymbol {
    _name: String,
    _fields: HashMap<String, String>, // field_name -> type_name
    _defined_at: Position,
}

#[derive(Debug, Clone)]
struct MethodSymbol {
    _name: String,
    parameters: Vec<String>,
    return_type: Option<String>,
    is_static: bool,
    _defined_at: Position,
}

#[derive(Debug, Clone, PartialEq)]
enum SymbolType {
    Variable(String), // type name
    #[allow(dead_code)]
    Function,
    #[allow(dead_code)]
    Builtin,
    #[allow(dead_code)]
    Struct,
    #[allow(dead_code)]
    Method,
}

impl SemanticAnalyzer {
    fn get_type_name_from_annotation(annotation: &Option<TypeAnnotation>) -> String {
        match annotation {
            Some(TypeAnnotation::Int) => "int".to_string(),
            Some(TypeAnnotation::Float) => "float".to_string(),
            Some(TypeAnnotation::String) => "string".to_string(),
            Some(TypeAnnotation::Bool) => "bool".to_string(),
            Some(TypeAnnotation::Char) => "char".to_string(),
            Some(TypeAnnotation::Array(inner)) => {
                format!(
                    "array<{}>",
                    Self::get_type_name_from_annotation(&Some(*inner.clone()))
                )
            }
            Some(TypeAnnotation::Map(key, value)) => {
                format!(
                    "map<{}, {}>",
                    Self::get_type_name_from_annotation(&Some(*key.clone())),
                    Self::get_type_name_from_annotation(&Some(*value.clone()))
                )
            }
            Some(TypeAnnotation::Custom(ident)) => ident.name.clone(),
            Some(TypeAnnotation::Any) => "any".to_string(),
            None => "unknown".to_string(),
        }
    }

    fn get_type_name_from_type_annotation(annotation: &TypeAnnotation) -> String {
        match annotation {
            TypeAnnotation::Int => "int".to_string(),
            TypeAnnotation::Float => "float".to_string(),
            TypeAnnotation::String => "string".to_string(),
            TypeAnnotation::Bool => "bool".to_string(),
            TypeAnnotation::Char => "char".to_string(),
            TypeAnnotation::Array(inner) => {
                format!("array<{}>", Self::get_type_name_from_type_annotation(inner))
            }
            TypeAnnotation::Map(key, value) => {
                format!(
                    "map<{}, {}>",
                    Self::get_type_name_from_type_annotation(key),
                    Self::get_type_name_from_type_annotation(value)
                )
            }
            TypeAnnotation::Custom(ident) => ident.name.clone(),
            TypeAnnotation::Any => "any".to_string(),
        }
    }

    pub fn new() -> Self {
        let mut analyzer = SemanticAnalyzer {
            diagnostics: Diagnostics::new(),
            symbol_table: SymbolTable::new(),
            _type_checker: TypeChecker::new(), // Reserved for future integration
            current_function: None,
            in_loop: false,
            source_lines: Vec::new(),
            module_resolver: None,
            visibility_checker: VisibilityChecker::new(),
            current_file: None,
        };

        // Add built-in functions
        analyzer.add_builtins();
        analyzer
    }

    /// Create a new semantic analyzer with module support
    pub fn with_module_support(base_dir: PathBuf, current_file: PathBuf) -> Self {
        let mut analyzer = Self::new();
        analyzer.module_resolver = Some(ModuleResolver::new(base_dir));
        analyzer.current_file = Some(current_file);
        analyzer
    }

    pub fn analyze(&mut self, program: &Program) -> Diagnostics {
        self.diagnostics = Diagnostics::new();

        // Module resolution pass: process use statements and resolve modules
        if self.module_resolver.is_some() {
            self.resolve_modules(program);
        }

        // First pass: collect all function declarations
        for stmt in &program.statements {
            if let Statement::FunctionDeclaration(func_decl) = stmt {
                self.declare_function(func_decl);
            }
        }

        // Second pass: analyze function bodies and perform type checking
        for stmt in &program.statements {
            self.analyze_statement(stmt);
        }

        // Note: Removed duplicate type checking pass to prevent duplicate errors
        // Type checking is now integrated into analyze_statement and analyze_expression

        // Check for unused variables (only warnings)
        self.check_unused_variables();

        self.diagnostics.clone()
    }

    /// Resolve modules from use statements
    fn resolve_modules(&mut self, program: &Program) {
        // Collect use statements first
        let use_statements: Vec<_> = program.statements.iter()
            .filter_map(|stmt| {
                if let Statement::UseStatement(use_stmt) = stmt {
                    Some(use_stmt.clone())
                } else {
                    None
                }
            })
            .collect();

        // Process each use statement
        for use_stmt in use_statements {
            self.resolve_single_use_statement(&use_stmt);
        }
    }

    /// Resolve a single use statement
    fn resolve_single_use_statement(&mut self, use_stmt: &UseStatement) {
        if let Some(ref mut resolver) = self.module_resolver {
            let current_file = self.current_file.as_ref().unwrap();
            
            match resolver.resolve_module(&use_stmt.path, current_file) {
                Ok(resolved_module) => {
                    // Register the module with the visibility checker
                    self.visibility_checker.register_module(&resolved_module);
                    
                    // Register the import
                    let module_name = if let Some(alias) = &use_stmt.alias {
                        alias.name.clone()
                    } else {
                        resolved_module.name.clone()
                    };
                    
                    self.visibility_checker.register_import(
                        &use_stmt.path,
                        use_stmt.alias.as_ref().map(|a| a.name.as_str()),
                        &resolved_module.name,
                    );
                    
                    // Register imported symbols in the symbol table
                    self.register_imported_symbols(&resolved_module, &module_name);
                }
                Err(module_error) => {
                    // Convert module error to diagnostic
                    let diagnostic = self.module_error_to_diagnostic(module_error);
                    self.diagnostics.add(diagnostic);
                }
            }
        }
    }

    /// Register imported symbols from a resolved module
    fn register_imported_symbols(&mut self, resolved_module: &ResolvedModule, module_name: &str) {
        // Process each statement in the imported module
        for stmt in &resolved_module.program.statements {
            match stmt {
                Statement::FunctionDeclaration(func_decl) => {
                    if func_decl.is_public {
                        let qualified_name = format!("{}.{}", module_name, func_decl.name.name);
                        
                        // Register the function in the symbol table
                        let func_symbol = FunctionSymbol {
                            _name: qualified_name.clone(),
                            parameters: func_decl.parameters.iter().map(|p| p.name.name.clone()).collect(),
                            return_type: func_decl.return_type.as_ref().map(|t| Self::get_type_name_from_type_annotation(t)),
                            defined_at: Position::new(1, 1, 0),
                        };
                        
                        self.symbol_table.functions.insert(qualified_name, func_symbol);
                    }
                },
                Statement::ConstantDeclaration(const_decl) => {
                    if const_decl.is_public {
                        let qualified_name = format!("{}.{}", module_name, const_decl.name.name);
                        
                        // Register the constant as a variable in the symbol table
                        let symbol = Symbol {
                            _name: qualified_name.clone(),
                            symbol_type: SymbolType::Variable("const".to_string()),
                            defined_at: Position::new(1, 1, 0),
                            used: false,
                            mutable: false,
                        };
                        
                        // Add to current scope
                        if let Some(current_scope) = self.symbol_table.scopes.last_mut() {
                            current_scope.insert(qualified_name, symbol);
                        }
                    }
                },
                Statement::VariableDeclaration(var_decl) => {
                    if var_decl.is_public {
                        let qualified_name = format!("{}.{}", module_name, var_decl.name.name);
                        
                        // Register the variable in the symbol table
                        let symbol = Symbol {
                            _name: qualified_name.clone(),
                            symbol_type: SymbolType::Variable("var".to_string()),
                            defined_at: Position::new(1, 1, 0),
                            used: false,
                            mutable: true,
                        };
                        
                        // Add to current scope
                        if let Some(current_scope) = self.symbol_table.scopes.last_mut() {
                            current_scope.insert(qualified_name, symbol);
                        }
                    }
                },
                Statement::StructDeclaration(struct_decl) => {
                    if struct_decl.is_public {
                        let qualified_name = format!("{}.{}", module_name, struct_decl.name.name);
                        
                        // Register the struct type
                        let struct_symbol = StructSymbol {
                            _name: qualified_name.clone(),
                            _fields: HashMap::new(), // TODO: populate with actual fields
                            _defined_at: Position::new(1, 1, 0),
                        };
                        
                        self.symbol_table.structs.insert(qualified_name, struct_symbol);
                    }
                },
                _ => {
                    // Skip other statement types
                }
            }
        }
    }

    /// Convert module error to diagnostic
    fn module_error_to_diagnostic(&self, error: ModuleError) -> Diagnostic {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 1, 0));
        
        match error {
            ModuleError::ModuleNotFound { path, searched_paths } => {
                helpers::syntax_error(
                    format!("Module '{}' not found. Searched: {}", path, searched_paths.join(", ")),
                    span,
                )
            }
            ModuleError::SymbolNotExported { symbol, module } => {
                helpers::syntax_error(
                    format!("Symbol '{}' is not exported from module '{}'", symbol, module),
                    span,
                )
            }
            ModuleError::CircularDependency { cycle } => {
                helpers::syntax_error(
                    format!("Circular dependency detected: {}", cycle.join(" -> ")),
                    span,
                )
            }
            _ => {
                helpers::syntax_error(format!("Module error: {}", error), span)
            }
        }
    }

    pub fn analyze_with_source(&mut self, program: &Program, source: &str) -> Diagnostics {
        // Store source lines for accurate position reporting
        self.source_lines = source.lines().map(|s| s.to_string()).collect();
        self.analyze(program)
    }

    fn add_builtins(&mut self) {
        let builtins = vec![
            ("print", vec!["value"]),
            ("println", vec!["value"]),
            ("printc", vec!["text", "color"]), // Colored print
            ("printlnc", vec!["text", "color"]), // Colored println
            ("input", vec![]), // input() can take 0 or 1 parameters
            ("read", vec!["filename"]),
            ("write", vec!["filename", "content"]),
            ("len", vec!["value"]),
            // Dot notation type conversion methods
            ("toint", vec!["value"]),
            ("tofloat", vec!["value"]),
            ("tostr", vec!["value"]),
            ("tobool", vec!["value"]),
            ("append", vec!["list", "value"]),
            ("remove", vec!["list", "index"]),
        ];

        for (name, params) in builtins {
            self.symbol_table.functions.insert(
                name.to_string(),
                FunctionSymbol {
                    _name: name.to_string(),
                    parameters: params.into_iter().map(|s| s.to_string()).collect(),
                    return_type: None,
                    defined_at: Position::new(0, 0, 0),
                },
            );
        }
    }

    fn declare_function(&mut self, func_decl: &FunctionDeclaration) {
        let func_name = &func_decl.name.name;

        // Check for duplicate function definitions
        if self.symbol_table.functions.contains_key(func_name) {
            let existing = &self.symbol_table.functions[func_name];
            let diagnostic = helpers::duplicate_definition(
                func_name,
                self.create_span_from_identifier(&func_decl.name),
                Some(Span::new(existing.defined_at, existing.defined_at)),
            );
            self.diagnostics.add(diagnostic);
            return;
        }

        let params: Vec<String> = func_decl
            .parameters
            .iter()
            .map(|p| p.name.name.clone())
            .collect();

        self.symbol_table.functions.insert(
            func_name.clone(),
            FunctionSymbol {
                _name: func_name.clone(),
                parameters: params,
                return_type: func_decl
                    .return_type
                    .as_ref()
                    .map(|_| "unknown".to_string()),
                defined_at: Position::new(1, 1, 0), // TODO: get actual position
            },
        );
    }

    fn analyze_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::FunctionDeclaration(func_decl) => {
                self.analyze_function_declaration(func_decl);
            }
            Statement::VariableDeclaration(var_decl) => {
                self.analyze_variable_declaration(var_decl);
            }
            Statement::ExpressionStatement(expr_stmt) => {
                self.analyze_expression(&expr_stmt.expression);
            }
            Statement::ReturnStatement(ret_stmt) => {
                if self.current_function.is_none() {
                    let diagnostic = helpers::syntax_error(
                        "return statement outside function",
                        Span::new(Position::new(1, 1, 0), Position::new(1, 6, 5)),
                    );
                    self.diagnostics.add(diagnostic);
                }

                if let Some(ref expr) = ret_stmt.value {
                    self.analyze_expression(expr);
                }
            }
            Statement::BlockStatement(block_stmt) => {
                self.symbol_table.push_scope();
                for stmt in &block_stmt.statements {
                    self.analyze_statement(stmt);
                }
                self.symbol_table.pop_scope();
            }
            Statement::IfStatement(if_stmt) => {
                self.analyze_expression(&if_stmt.condition);
                self.analyze_statement(&if_stmt.then_branch);

                // Analyze elif branches
                for elif_branch in &if_stmt.elif_branches {
                    self.analyze_expression(&elif_branch.condition);
                    self.analyze_statement(&elif_branch.body);
                }

                // Analyze else branch
                if let Some(ref else_branch) = if_stmt.else_branch {
                    self.analyze_statement(else_branch);
                }
            }
            Statement::WhileStatement(while_stmt) => {
                self.analyze_expression(&while_stmt.condition);
                let was_in_loop = self.in_loop;
                self.in_loop = true;
                self.analyze_statement(&while_stmt.body);
                self.in_loop = was_in_loop;
            }
            Statement::BreakStatement(_) => {
                if !self.in_loop {
                    let diagnostic = Diagnostic::new(
                        crate::frontend::diagnostics::DiagnosticKind::BreakOutsideLoop,
                    )
                    .with_code("E0009");
                    self.diagnostics.add(diagnostic);
                }
            }
            Statement::ContinueStatement(_) => {
                if !self.in_loop {
                    let diagnostic = Diagnostic::new(
                        crate::frontend::diagnostics::DiagnosticKind::ContinueOutsideLoop,
                    )
                    .with_code("E0010");
                    self.diagnostics.add(diagnostic);
                }
            }
            // Handle other statement types (not yet implemented)
            Statement::ModuleDeclaration(_) => {
                // TODO: Implement module analysis
            }
            Statement::UseStatement(_) => {
                // TODO: Implement use statement analysis
            }
            Statement::ConstantDeclaration(const_decl) => {
                // Similar to variable declaration but immutable
                let const_name = &const_decl.name.name;

                if let Some(existing) = self.symbol_table.lookup_in_current_scope(const_name) {
                    let diagnostic = helpers::shadowed_variable(
                        const_name,
                        self.create_span_from_identifier(&const_decl.name),
                        existing.defined_at.line,
                    );
                    self.diagnostics.add(diagnostic);
                }

                // Analyze initializer and infer type
                let inferred_type = self
                    .analyze_expression(&const_decl.initializer)
                    .unwrap_or_else(|| "any".to_string());

                // Declare the constant with the inferred type (immutable)
                self.declare_variable(const_name, &inferred_type, Position::new(1, 1, 0), false);
            }
            Statement::StructDeclaration(struct_decl) => {
                // Register struct type in symbol table
                self.declare_variable(
                    &struct_decl.name.name,
                    "type",
                    Position::new(1, 1, 0),
                    false,
                );
            }
            Statement::EnumDeclaration(enum_decl) => {
                // Register enum type in symbol table
                self.declare_variable(&enum_decl.name.name, "type", Position::new(1, 1, 0), false);
            }
            Statement::ImplBlock(impl_block) => {
                self.analyze_impl_block(impl_block);
            }
            Statement::ForStatement(for_stmt) => {
                // Analyze iterable
                self.analyze_expression(&for_stmt.iterable);

                // Create new scope for loop variable
                self.symbol_table.push_scope();

                // Determine loop variable type based on iterable
                let loop_var_type = match &for_stmt.iterable {
                    Expression::RangeExpression(_) => "int", // Range produces integers
                    Expression::ArrayLiteral(array) => {
                        // Infer type from first array element if possible
                        if !array.elements.is_empty() {
                            match &array.elements[0] {
                                Expression::IntegerLiteral(_) => "int",
                                Expression::FloatLiteral(_) => "float",
                                Expression::StringLiteral(_) => "str",
                                Expression::BooleanLiteral(_) => "bool",
                                _ => "var",
                            }
                        } else {
                            "var"
                        }
                    }
                    _ => "var", // Default to var for other types
                };

                self.declare_variable(
                    &for_stmt.variable.name,
                    loop_var_type,
                    Position::new(1, 1, 0),
                    true,
                );

                let was_in_loop = self.in_loop;
                self.in_loop = true;
                self.analyze_statement(&for_stmt.body);
                self.in_loop = was_in_loop;

                self.symbol_table.pop_scope();
            }
            Statement::MatchStatement(match_stmt) => {
                self.analyze_expression(&match_stmt.expression);
                for arm in &match_stmt.arms {
                    self.analyze_expression(&arm.body);
                }
            }
            Statement::TryStatement(try_stmt) => {
                self.analyze_statement(&Statement::BlockStatement(try_stmt.body.clone()));
                if let Some(ref catch_clause) = try_stmt.catch_clause {
                    self.symbol_table.push_scope();
                    if let Some(ref param) = catch_clause.parameter {
                        self.declare_variable(
                            &param.name,
                            "exception",
                            Position::new(1, 1, 0),
                            true,
                        );
                    }
                    self.analyze_statement(&Statement::BlockStatement(catch_clause.body.clone()));
                    self.symbol_table.pop_scope();
                }
            }
            Statement::ThrowStatement(throw_stmt) => {
                self.analyze_expression(&throw_stmt.value);
            }
        }
    }

    fn analyze_function_declaration(&mut self, func_decl: &FunctionDeclaration) {
        let old_function = self.current_function.clone();
        self.current_function = Some(func_decl.name.name.clone());

        // Create new scope for function
        self.symbol_table.push_scope();

        // Add parameters to scope with their proper types
        for param in &func_decl.parameters {
            let param_type = if let Some(ref type_ann) = param.type_annotation {
                match type_ann {
                    TypeAnnotation::Int => "int",
                    TypeAnnotation::Float => "float",
                    TypeAnnotation::String => "str",
                    TypeAnnotation::Bool => "bool",
                    TypeAnnotation::Char => "char",
                    TypeAnnotation::Any => "any",
                    TypeAnnotation::Custom(id) => &id.name,
                    _ => "any",
                }
            } else {
                "any" // Parameters without type annotations are flexible
            };
            self.declare_variable(&param.name.name, param_type, Position::new(1, 1, 0), true);
        }

        // Analyze function body
        for stmt in &func_decl.body.statements {
            self.analyze_statement(stmt);
        }

        self.symbol_table.pop_scope();
        self.current_function = old_function;
    }

    fn analyze_variable_declaration(&mut self, var_decl: &VariableDeclaration) {
        let var_name = &var_decl.name.name;

        // Check for variable shadowing
        if let Some(existing) = self.symbol_table.lookup_in_current_scope(var_name) {
            let diagnostic = helpers::shadowed_variable(
                var_name,
                self.create_span_from_identifier(&var_decl.name),
                existing.defined_at.line,
            );
            self.diagnostics.add(diagnostic);
        }

        // Analyze initializer and infer type if present
        let inferred_type = if let Some(ref expr) = var_decl.initializer {
            self.analyze_expression(expr)
                .unwrap_or_else(|| "any".to_string())
        } else {
            "any".to_string()
        };

        // Use explicit type annotation if provided, otherwise use inferred type
        let var_type = if let Some(ref type_ann) = var_decl.type_annotation {
            let declared_type = match type_ann {
                TypeAnnotation::Int => "int",
                TypeAnnotation::Float => "float",
                TypeAnnotation::String => "str",
                TypeAnnotation::Bool => "bool",
                TypeAnnotation::Char => "char",
                TypeAnnotation::Any => "any",
                TypeAnnotation::Custom(id) => &id.name,
                _ => "any",
            };

            // Check type compatibility if both declared type and initializer exist
            if let Some(ref _expr) = var_decl.initializer {
                if !self.types_compatible(&inferred_type, declared_type) {
                    let diagnostic = helpers::type_mismatch(
                        declared_type,
                        &inferred_type,
                        self.create_span_from_identifier(&var_decl.name),
                    );
                    self.diagnostics.add(diagnostic);
                }
            }

            declared_type
        } else {
            &inferred_type
        };

        // Declare the variable
        self.declare_variable(
            var_name,
            var_type,
            Position::new(1, 1, 0), // TODO: get actual position
            true,
        );
    }

    fn analyze_expression(&mut self, expr: &Expression) -> Option<String> {
        match expr {
            Expression::Identifier(ident) => self.analyze_identifier(ident),
            Expression::IntegerLiteral(_) => Some("int".to_string()),
            Expression::StringLiteral(_) => Some("str".to_string()),
            Expression::BooleanLiteral(_) => Some("bool".to_string()),
            Expression::BinaryExpression(bin_expr) => {
                let left_type = self.analyze_expression(&bin_expr.left);
                let right_type = self.analyze_expression(&bin_expr.right);

                // Simple type checking for binary operations
                match &bin_expr.operator {
                    BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo => {
                        if let (Some(left), Some(right)) = (&left_type, &right_type) {
                            // Allow "any" type to be compatible with anything
                            if left == "any" || right == "any" {
                                Some("any".to_string()) // Any type is flexible
                            } else if left == "str" || right == "str" {
                                Some("str".to_string()) // String concatenation
                            } else if left == "int" && right == "int" {
                                Some("int".to_string())
                            } else if left == "float" || right == "float" {
                                Some("float".to_string()) // Promote to float
                            } else {
                                let diagnostic = helpers::type_mismatch(
                                    "compatible types",
                                    &format!("{} and {}", left, right),
                                    Span::new(Position::new(1, 1, 0), Position::new(1, 1, 0)),
                                );
                                self.diagnostics.add(diagnostic);
                                None
                            }
                        } else {
                            None
                        }
                    }
                    BinaryOperator::Equal
                    | BinaryOperator::NotEqual
                    | BinaryOperator::Less
                    | BinaryOperator::Greater
                    | BinaryOperator::LessEqual
                    | BinaryOperator::GreaterEqual => Some("bool".to_string()),
                    BinaryOperator::And | BinaryOperator::Or => {
                        // Check that both operands are boolean
                        if let (Some(left), Some(right)) = (&left_type, &right_type) {
                            if left != "bool" || right != "bool" {
                                let diagnostic = helpers::type_mismatch(
                                    "bool",
                                    &format!("{} and {}", left, right),
                                    Span::new(Position::new(1, 1, 0), Position::new(1, 1, 0)),
                                );
                                self.diagnostics.add(diagnostic);
                            }
                        }
                        Some("bool".to_string())
                    }
                    BinaryOperator::BitwiseAnd
                    | BinaryOperator::BitwiseOr
                    | BinaryOperator::BitwiseXor
                    | BinaryOperator::LeftShift
                    | BinaryOperator::RightShift => {
                        // Check that both operands are integers
                        if let (Some(left), Some(right)) = (&left_type, &right_type) {
                            if left != "int" || right != "int" {
                                let diagnostic = helpers::type_mismatch(
                                    "int",
                                    &format!("{} and {}", left, right),
                                    Span::new(Position::new(1, 1, 0), Position::new(1, 1, 0)),
                                );
                                self.diagnostics.add(diagnostic);
                            }
                        }
                        Some("int".to_string())
                    }
                    _ => None,
                }
            }
            Expression::UnaryExpression(unary_expr) => {
                let operand_type = self.analyze_expression(&unary_expr.operand);
                match &unary_expr.operator {
                    UnaryOperator::Not => {
                        if let Some(ref op_type) = operand_type {
                            if op_type != "bool" {
                                let diagnostic = helpers::type_mismatch(
                                    "bool",
                                    &op_type,
                                    Span::new(Position::new(1, 1, 0), Position::new(1, 1, 0)),
                                );
                                self.diagnostics.add(diagnostic);
                            }
                        }
                        Some("bool".to_string())
                    }
                    UnaryOperator::Minus | UnaryOperator::Plus => {
                        if let Some(ref op_type) = operand_type {
                            if op_type != "int" {
                                let diagnostic = helpers::type_mismatch(
                                    "int",
                                    &op_type,
                                    Span::new(Position::new(1, 1, 0), Position::new(1, 1, 0)),
                                );
                                self.diagnostics.add(diagnostic);
                            }
                        }
                        Some("int".to_string())
                    }
                    UnaryOperator::BitwiseNot => {
                        if let Some(ref op_type) = operand_type {
                            if op_type != "int" {
                                let diagnostic = helpers::type_mismatch(
                                    "int",
                                    &op_type,
                                    Span::new(Position::new(1, 1, 0), Position::new(1, 1, 0)),
                                );
                                self.diagnostics.add(diagnostic);
                            }
                        }
                        Some("int".to_string())
                    }
                    UnaryOperator::PreIncrement
                    | UnaryOperator::PostIncrement
                    | UnaryOperator::PreDecrement
                    | UnaryOperator::PostDecrement => {
                        if let Some(ref op_type) = operand_type {
                            if op_type != "int" {
                                let diagnostic = helpers::type_mismatch(
                                    "int",
                                    &op_type,
                                    Span::new(Position::new(1, 1, 0), Position::new(1, 1, 0)),
                                );
                                self.diagnostics.add(diagnostic);
                            }
                        }
                        Some("int".to_string())
                    }
                }
            }
            Expression::CallExpression(call_expr) => self.analyze_call_expression(call_expr),
            Expression::AssignmentExpression(assign_expr) => {
                // Check if the target is a valid lvalue
                if let Expression::Identifier(ident) = assign_expr.left.as_ref() {
                    let is_mutable = if let Some(symbol) = self.symbol_table.lookup(&ident.name) {
                        if !symbol.mutable {
                            let diagnostic = Diagnostic::new(
                                crate::frontend::diagnostics::DiagnosticKind::ImmutableAssignment {
                                    name: ident.name.clone(),
                                },
                            )
                            .with_code("E0011");
                            self.diagnostics.add(diagnostic);
                        }
                        true
                    } else {
                        let diagnostic = helpers::undefined_variable(
                            &ident.name,
                            self.create_span_from_identifier(&ident),
                        );
                        self.diagnostics.add(diagnostic);
                        false
                    };

                    // Mark as used after borrowing is done
                    if is_mutable {
                        self.symbol_table.mark_used(&ident.name);
                    }
                }

                self.analyze_expression(&assign_expr.right)
            }
            // Handle other expression types
            Expression::FloatLiteral(_) => Some("float".to_string()),
            Expression::NullLiteral(_) => Some("null".to_string()),
            Expression::MemberExpression(member_expr) => {
                self.analyze_expression(&member_expr.object);
                // For now, assume member access returns the same type
                None
            }
            Expression::MethodCallExpression(method_call) => self.analyze_method_call(method_call),
            Expression::SelfExpression(_) => {
                // Return the type of the current context (if we're in a method)
                // For now, we'll return None and let the type checker handle it
                None
            }
            Expression::IndexExpression(index_expr) => {
                self.analyze_expression(&index_expr.object);
                self.analyze_expression(&index_expr.index);
                // For now, assume index access returns the element type
                None
            }
            Expression::ArrayLiteral(array_lit) => {
                for element in &array_lit.elements {
                    self.analyze_expression(element);
                }
                Some("array".to_string())
            }
            Expression::MapLiteral(map_lit) => {
                for pair in &map_lit.pairs {
                    self.analyze_expression(&pair.key);
                    self.analyze_expression(&pair.value);
                }
                Some("map".to_string())
            }
            Expression::StructInstantiation(struct_inst) => {
                // Analyze all field values
                for field in &struct_inst.fields {
                    self.analyze_expression(&field.value);
                }
                // Return the struct type name
                Some(struct_inst.name.name.clone())
            }
            Expression::InterpolatedString(interp_str) => {
                for part in &interp_str.parts {
                    if let InterpolationPart::Expression(expr) = part {
                        self.analyze_expression(expr);
                    }
                }
                Some("str".to_string())
            }
            Expression::RangeExpression(range_expr) => {
                self.analyze_expression(&range_expr.start);
                self.analyze_expression(&range_expr.end);
                Some("range".to_string())
            }
            Expression::GroupingExpression(group_expr) => {
                self.analyze_expression(&group_expr.expression)
            }
            Expression::ModuleCallExpression(module_call) => {
                self.analyze_module_call(module_call)
            }
        }
    }

    fn analyze_identifier(&mut self, ident: &Identifier) -> Option<String> {
        let result = if let Some(symbol) = self.symbol_table.lookup(&ident.name) {
            match &symbol.symbol_type {
                SymbolType::Variable(type_name) => Some(type_name.clone()),
                SymbolType::Function => Some("function".to_string()),
                SymbolType::Builtin => Some("builtin".to_string()),
                SymbolType::Struct => Some("struct".to_string()),
                SymbolType::Method => Some("method".to_string()),
            }
        } else {
            let diagnostic =
                helpers::undefined_variable(&ident.name, self.create_span_from_identifier(ident));
            self.diagnostics.add(diagnostic);
            None
        };

        // Mark as used after borrowing is done
        self.symbol_table.mark_used(&ident.name);
        result
    }

    fn analyze_call_expression(&mut self, call_expr: &CallExpression) -> Option<String> {
        if let Expression::Identifier(func_name) = call_expr.callee.as_ref() {
            // Check if function exists and get info
            let func_info =
                if let Some(func_symbol) = self.symbol_table.functions.get(&func_name.name) {
                    Some((
                        func_symbol.parameters.len(),
                        func_symbol.return_type.clone(),
                    ))
                } else {
                    None
                };

            if let Some((expected_args, return_type)) = func_info {
                // Check argument count
                let provided_args = call_expr.arguments.len();

                if expected_args != provided_args {
                    let diagnostic = helpers::wrong_argument_count(
                        expected_args,
                        provided_args,
                        self.create_span_from_identifier(&func_name),
                    );
                    self.diagnostics.add(diagnostic);
                }

                // Analyze arguments
                for arg in &call_expr.arguments {
                    self.analyze_expression(arg);
                }

                return_type
            } else {
                let diagnostic = helpers::undefined_function(
                    &func_name.name,
                    self.create_span_from_identifier(&func_name),
                );
                self.diagnostics.add(diagnostic);
                None
            }
        } else {
            // Complex callee expression
            self.analyze_expression(&call_expr.callee);
            for arg in &call_expr.arguments {
                self.analyze_expression(arg);
            }
            None
        }
    }

    fn declare_variable(&mut self, name: &str, var_type: &str, pos: Position, mutable: bool) {
        let symbol = Symbol {
            _name: name.to_string(),
            symbol_type: SymbolType::Variable(var_type.to_string()),
            defined_at: pos,
            used: false,
            mutable,
        };
        self.symbol_table.declare(name.to_string(), symbol);
    }

    fn check_unused_variables(&mut self) {
        for scope in &self.symbol_table.scopes {
            for (name, symbol) in scope {
                if !symbol.used && matches!(symbol.symbol_type, SymbolType::Variable(_)) {
                    let diagnostic = helpers::unused_variable(
                        name,
                        Span::new(symbol.defined_at, symbol.defined_at),
                    );
                    self.diagnostics.add(diagnostic);
                }
            }
        }
    }

    fn create_span_from_identifier(&self, ident: &Identifier) -> Span {
        // Use actual span information from the identifier if available
        if let Some(ref span) = ident.span {
            span.clone()
        } else {
            // Find the actual position of the identifier in source
            let (line, column) = self.find_identifier_position(&ident.name);
            let start_pos = Position::new(line, column, 0);
            let end_pos = Position::new(line, column + ident.name.len(), ident.name.len());
            Span::new(start_pos, end_pos)
                .with_source(self.current_file.as_ref().map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|| "source".to_string()))
        }
    }

    #[allow(dead_code)]
    fn estimate_line_number(&self, identifier: &str) -> usize {
        // Search for the identifier in the source lines
        for (line_idx, line) in self.source_lines.iter().enumerate() {
            if line.contains(identifier) {
                return line_idx + 1; // Convert to 1-based line number
            }
        }
        1 // Fallback to line 1
    }

    fn find_identifier_position(&self, identifier: &str) -> (usize, usize) {
        // Search for the identifier in the source lines with better accuracy
        for (line_idx, line) in self.source_lines.iter().enumerate() {
            // Skip comment lines
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }

            // Find all occurrences of the identifier in this line
            let mut start_pos = 0;
            while let Some(col_idx) = line[start_pos..].find(identifier) {
                let actual_col = start_pos + col_idx;
                
                // Check if this occurrence is in a comment
                let before_identifier = &line[..actual_col];
                if !before_identifier.contains("//") {
                    // Check if it's a whole word (not part of another identifier)
                    let is_word_boundary = {
                        let before_char = if actual_col > 0 {
                            line.chars().nth(actual_col - 1)
                        } else {
                            None
                        };
                        let after_char = line.chars().nth(actual_col + identifier.len());
                        
                        let before_ok = before_char.map_or(true, |c| !c.is_alphanumeric() && c != '_');
                        let after_ok = after_char.map_or(true, |c| !c.is_alphanumeric() && c != '_');
                        
                        before_ok && after_ok
                    };
                    
                    if is_word_boundary {
                        return (line_idx + 1, actual_col + 1); // Convert to 1-based
                    }
                }
                
                start_pos = actual_col + 1;
            }
        }
        
        // Fallback to first occurrence if no perfect match found
        for (line_idx, line) in self.source_lines.iter().enumerate() {
            if let Some(col_idx) = line.find(identifier) {
                return (line_idx + 1, col_idx + 1); // Convert to 1-based
            }
        }
        (1, 1) // Final fallback
    }

    /// Check if two types are compatible for assignment
    fn types_compatible(&self, from_type: &str, to_type: &str) -> bool {
        match (from_type, to_type) {
            // Exact matches
            (a, b) if a == b => true,

            // Any type is flexible
            ("any", _) | (_, "any") => true,

            // Numeric coercions
            ("int", "float") | ("float", "int") => true,

            // String concatenation flexibility (but not for explicit type declarations)
            // For explicit declarations, we want strict typing
            _ => false,
        }
    }

    /// Analyze impl block and register methods
    fn analyze_impl_block(&mut self, impl_block: &ImplBlock) {
        let type_name = &impl_block.target_type.name;

        // Check if the target type exists
        if !self.symbol_table.structs.contains_key(type_name) {
            // For now, we'll allow impl blocks for any type (including built-in types)
            // In a more complete implementation, we'd validate the type exists
        }

        let mut methods = Vec::new();

        for method in &impl_block.methods {
            // Analyze method parameters and body
            self.symbol_table.push_scope();

            // If not static, add 'self' parameter to scope
            if !method.is_static {
                self.declare_variable("self", type_name, Position::new(1, 1, 0), false);
            }

            // Add method parameters to scope
            for param in &method.parameters {
                if param.name.name != "self" {
                    let type_name = Self::get_type_name_from_annotation(&param.type_annotation);
                    self.declare_variable(
                        &param.name.name,
                        &type_name,
                        Position::new(1, 1, 0),
                        true,
                    );
                }
            }

            // Analyze method body
            self.analyze_statement(&Statement::BlockStatement(method.body.clone()));

            // Register method
            let method_symbol = MethodSymbol {
                _name: method.name.name.clone(),
                parameters: method
                    .parameters
                    .iter()
                    .map(|p| Self::get_type_name_from_annotation(&p.type_annotation))
                    .collect(),
                return_type: method
                    .return_type
                    .as_ref()
                    .map(|t| Self::get_type_name_from_type_annotation(t)),
                is_static: method.is_static,
                _defined_at: Position::new(1, 1, 0),
            };

            methods.push(method_symbol);
            self.symbol_table.pop_scope();
        }

        // Register all methods for this type
        self.symbol_table.methods.insert(type_name.clone(), methods);
    }

    /// Analyze method call expression
    fn analyze_method_call(&mut self, method_call: &MethodCallExpression) -> Option<String> {
        // Analyze the object being called on
        let object_type = self.analyze_expression(&method_call.object);

        // Analyze method arguments
        for arg in &method_call.arguments {
            self.analyze_expression(arg);
        }

        // Check if the method exists for this type
        if let Some(object_type_name) = &object_type {
            if let Some(methods) = self.symbol_table.methods.get(object_type_name) {
                let method_name = &method_call.method.name;

                // Find the method
                if let Some(method) = methods.iter().find(|m| m._name == *method_name) {
                    // Validate argument count (excluding self parameter for non-static methods)
                    let expected_args = if method.is_static {
                        method.parameters.len()
                    } else {
                        method.parameters.len().saturating_sub(1) // Subtract 1 for self
                    };

                    if method_call.arguments.len() != expected_args {
                        let diagnostic = Diagnostic::new(
                            crate::frontend::diagnostics::DiagnosticKind::ArgumentCountMismatch {
                                expected: expected_args,
                                found: method_call.arguments.len(),
                            },
                        )
                        .with_code("E0012");
                        self.diagnostics.add(diagnostic);
                    }

                    return method.return_type.clone();
                } else {
                    // Method not found
                    let diagnostic = Diagnostic::new(
                        crate::frontend::diagnostics::DiagnosticKind::UndefinedMethod {
                            method: method_name.clone(),
                            type_name: object_type_name.clone(),
                        },
                    )
                    .with_code("E0013");
                    self.diagnostics.add(diagnostic);
                }
            }
        }

        None
    }

    /// Analyze module call expression (e.g., utils.Function())
    fn analyze_module_call(&mut self, module_call: &ModuleCallExpression) -> Option<String> {
        // Analyze arguments
        for arg in &module_call.arguments {
            self.analyze_expression(arg);
        }

        // Check if the module call is valid using visibility checker
        match self.visibility_checker.check_symbol_access(
            &module_call.module.name,
            &module_call.function.name,
        ) {
            Ok(_symbol_info) => {
                // Module call is valid, return unknown type for now
                // TODO: Get actual return type from symbol info
                Some("unknown".to_string())
            }
            Err(module_error) => {
                // Module call is invalid, report error
                let diagnostic = self.module_error_to_diagnostic(module_error);
                self.diagnostics.add(diagnostic);
                None
            }
        }
    }
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable {
            scopes: vec![HashMap::new()], // Global scope
            functions: HashMap::new(),
            structs: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: String, symbol: Symbol) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, symbol);
        }
    }

    fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    fn lookup_in_current_scope(&self, name: &str) -> Option<&Symbol> {
        self.scopes.last()?.get(name)
    }

    fn mark_used(&mut self, name: &str) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.get_mut(name) {
                symbol.used = true;
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::parser::ast::*;

    #[test]
    fn test_undefined_variable() {
        let mut analyzer = SemanticAnalyzer::new();

        // Create a simple program with undefined variable
        let program = Program {
            statements: vec![Statement::FunctionDeclaration(FunctionDeclaration {
                name: Identifier::new("main".to_string()),
                parameters: vec![],
                return_type: None,
                body: BlockStatement {
                    statements: vec![Statement::ExpressionStatement(ExpressionStatement {
                        expression: Expression::Identifier(Identifier::new(
                            "undefined_var".to_string(),
                        )),
                    })],
                },
                is_public: false,
            })],
        };

        let diagnostics = analyzer.analyze(&program);
        assert!(!diagnostics.is_empty());
        assert!(diagnostics.has_errors());
    }

    #[test]
    fn test_function_call_validation() {
        let mut analyzer = SemanticAnalyzer::new();

        // Test calling println with correct arguments
        let program = Program {
            statements: vec![Statement::FunctionDeclaration(FunctionDeclaration {
                name: Identifier::new("main".to_string()),
                parameters: vec![],
                return_type: None,
                body: BlockStatement {
                    statements: vec![Statement::ExpressionStatement(ExpressionStatement {
                        expression: Expression::CallExpression(CallExpression {
                            callee: Box::new(Expression::Identifier(Identifier::new(
                                "println".to_string(),
                            ))),
                            arguments: vec![Expression::StringLiteral(StringLiteral {
                                value: "Hello, world!".to_string(),
                            })],
                        }),
                    })],
                },
                is_public: false,
            })],
        };

        let diagnostics = analyzer.analyze(&program);
        // println should be recognized as a builtin
        assert!(!diagnostics.has_errors());
    }
}
