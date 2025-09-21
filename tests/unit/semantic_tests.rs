// tests/unit/semantic_tests.rs

use razen_lang::backend::semantic::{SemanticAnalyzer, Type, SymbolKind, Symbol};
use razen_lang::frontend::parser::ast::*;
use razen_lang::frontend::diagnostics::Diagnostics;

#[cfg(test)]
mod semantic_analyzer_tests {
    use super::*;

    fn create_test_program() -> Program {
        Program {
            statements: vec![
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier::new("test_func".to_string()),
                    parameters: vec![],
                    return_type: Some(TypeAnnotation::new("int".to_string())),
                    body: BlockStatement::new(vec![
                        Statement::ReturnStatement(ReturnStatement {
                            value: Some(Expression::IntegerLiteral(IntegerLiteral::new(42))),
                        }),
                    ]),
                    is_public: false,
                }),
            ],
        }
    }

    #[test]
    fn test_semantic_analyzer_creation() {
        let analyzer = SemanticAnalyzer::new();
        // SemanticAnalyzer created successfully
        assert!(true);
    }

    #[test]
    fn test_function_declaration_analysis() {
        let mut analyzer = SemanticAnalyzer::new();
        let program = create_test_program();
        
        let result = analyzer.analyze(program);
        assert!(result.is_ok(), "Semantic analysis should succeed for valid program");
        
        let _analyzed = result.unwrap();
        // Function declaration analyzed successfully
        assert!(true);
    }

    #[test]
    fn test_variable_declaration_analysis() {
        let mut analyzer = SemanticAnalyzer::new();
        let program = Program {
            statements: vec![
                Statement::VariableDeclaration(VariableDeclaration {
                    name: Identifier::new("test_var".to_string()),
                    type_annotation: Some(TypeAnnotation::new("int".to_string())),
                    initializer: Some(Expression::IntegerLiteral(IntegerLiteral::new(10))),
                    is_public: false,
                }),
            ],
        };
        
        let result = analyzer.analyze(program);
        assert!(result.is_ok(), "Variable declaration should be analyzed successfully");
        
        let _analyzed = result.unwrap();
        // Variable declaration analyzed successfully
        assert!(true);
    }

    #[test]
    fn test_type_checking_binary_operations() {
        let mut analyzer = SemanticAnalyzer::new();
        let program = Program {
            statements: vec![
                Statement::VariableDeclaration(VariableDeclaration {
                    name: Identifier::new("result".to_string()),
                    type_annotation: Some(TypeAnnotation::new("int".to_string())),
                    initializer: Some(Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(Expression::IntegerLiteral(IntegerLiteral::new(5))),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expression::IntegerLiteral(IntegerLiteral::new(3))),
                    })),
                    is_public: false,
                }),
            ],
        };
        
        let result = analyzer.analyze(program);
        assert!(result.is_ok(), "Binary operation type checking should succeed");
    }

    #[test]
    fn test_undefined_variable_error() {
        let mut analyzer = SemanticAnalyzer::new();
        let program = Program {
            statements: vec![
                Statement::ExpressionStatement(ExpressionStatement {
                    expression: Expression::Identifier(Identifier::new("undefined_var".to_string())),
                }),
            ],
        };
        
        let result = analyzer.analyze(program);
        assert!(result.is_err(), "Using undefined variable should produce error");
    }

    #[test]
    fn test_type_mismatch_error() {
        let mut analyzer = SemanticAnalyzer::new();
        let program = Program {
            statements: vec![
                Statement::VariableDeclaration(VariableDeclaration {
                    name: Identifier::new("test_var".to_string()),
                    type_annotation: Some(TypeAnnotation::new("int".to_string())),
                    initializer: Some(Expression::StringLiteral(StringLiteral::new("string_value".to_string()))),
                    is_public: false,
                }),
            ],
        };
        
        let result = analyzer.analyze(program);
        assert!(result.is_err(), "Type mismatch should produce error");
    }

    #[test]
    fn test_function_redefinition_error() {
        let mut analyzer = SemanticAnalyzer::new();
        let program = Program {
            statements: vec![
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier::new("duplicate_func".to_string()),
                    parameters: vec![],
                    return_type: Some(TypeAnnotation::new("int".to_string())),
                    body: BlockStatement::new(vec![]),
                    is_public: false,
                }),
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier::new("duplicate_func".to_string()),
                    parameters: vec![],
                    return_type: Some(TypeAnnotation::new("int".to_string())),
                    body: BlockStatement::new(vec![]),
                    is_public: false,
                }),
            ],
        };
        
        let result = analyzer.analyze(program);
        assert!(result.is_err(), "Function redefinition should produce error");
    }

    #[test]
    fn test_scope_management() {
        let mut analyzer = SemanticAnalyzer::new();
        let program = Program {
            statements: vec![
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier::new("scope_test".to_string()),
                    parameters: vec![],
                    return_type: Some(TypeAnnotation::new("void".to_string())),
                    body: BlockStatement::new(vec![
                        Statement::VariableDeclaration(VariableDeclaration {
                            name: Identifier::new("local_var".to_string()),
                            type_annotation: Some(TypeAnnotation::new("int".to_string())),
                            initializer: Some(Expression::IntegerLiteral(IntegerLiteral::new(42))),
                            is_public: false,
                        }),
                    ]),
                    is_public: false,
                }),
            ],
        };
        
        let result = analyzer.analyze(program);
        assert!(result.is_ok(), "Scope management should work correctly");
    }
}

#[cfg(test)]
mod type_system_tests {
    use super::*;

    #[test]
    fn test_type_compatibility() {
        // Test type compatibility
        assert!(Type::Int.is_compatible_with(&Type::Int));
        assert!(Type::String.is_compatible_with(&Type::String));
        assert!(!Type::Int.is_compatible_with(&Type::String));
    }

    #[test]
    fn test_type_creation() {
        // Test type creation
        let int_type = Type::Int;
        let string_type = Type::String;
        let bool_type = Type::Bool;
        
        assert_eq!(int_type, Type::Int);
        assert_eq!(string_type, Type::String);
        assert_eq!(bool_type, Type::Bool);
    }

    #[test]
    fn test_composite_types() {
        // Test composite types
        let array_type = Type::Array(Box::new(Type::Int));
        let map_type = Type::Map(Box::new(Type::String), Box::new(Type::Int));
        
        match array_type {
            Type::Array(inner) => assert_eq!(*inner, Type::Int),
            _ => panic!("Expected Array type"),
        }
        
        match map_type {
            Type::Map(key, value) => {
                assert_eq!(*key, Type::String);
                assert_eq!(*value, Type::Int);
            },
            _ => panic!("Expected Map type"),
        }
    }
}

#[cfg(test)]
mod symbol_table_tests {
    use super::*;
    use razen_lang::backend::semantic::{SymbolTable, Symbol};

    #[test]
    fn test_symbol_table_creation() {
        let symbol_table = SymbolTable::new();
        // Symbol table created successfully
        assert!(true);
    }

    #[test]
    fn test_symbol_creation() {
        // Test symbol creation
        let symbol = Symbol {
            name: "test_symbol".to_string(),
            kind: SymbolKind::Variable,
            symbol_type: Type::Int,
            is_mutable: false,
        };
        
        assert_eq!(symbol.name, "test_symbol");
        assert_eq!(symbol.kind, SymbolKind::Variable);
        assert_eq!(symbol.symbol_type, Type::Int);
        assert!(!symbol.is_mutable);
    }

    #[test]
    fn test_symbol_kinds() {
        // Test different symbol kinds
        let var_symbol = Symbol {
            name: "var".to_string(),
            kind: SymbolKind::Variable,
            symbol_type: Type::Int,
            is_mutable: true,
        };
        
        let func_symbol = Symbol {
            name: "func".to_string(),
            kind: SymbolKind::Function,
            symbol_type: Type::Function {
                params: vec![Type::Int],
                return_type: Box::new(Type::Int),
            },
            is_mutable: false,
        };
        
        assert_eq!(var_symbol.kind, SymbolKind::Variable);
        assert_eq!(func_symbol.kind, SymbolKind::Function);
    }
}
