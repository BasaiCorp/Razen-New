// tests/integration/pipeline_tests.rs

use razen_lang::backend::Backend;
use razen_lang::backend::optimization::OptimizationLevel;
use razen_lang::backend::linking::{LinkingConfig, OutputFormat};
use razen_lang::frontend::parser::ast::*;

#[cfg(test)]
mod full_pipeline_tests {
    use super::*;

    fn create_simple_program() -> Program {
        Program {
            statements: vec![
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier::new("main".to_string()),
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

    fn create_arithmetic_program() -> Program {
        Program {
            statements: vec![
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier::new("add".to_string()),
                    parameters: vec![],
                    return_type: Some(TypeAnnotation::new("int".to_string())),
                    body: BlockStatement::new(vec![
                        Statement::ReturnStatement(ReturnStatement {
                            value: Some(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::IntegerLiteral(IntegerLiteral::new(5))),
                                operator: BinaryOperator::Add,
                                right: Box::new(Expression::IntegerLiteral(IntegerLiteral::new(3))),
                            })),
                        }),
                    ]),
                    is_public: false,
                }),
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: "main".to_string(),
                    parameters: vec![],
                    return_type: Some("int".to_string()),
                    body: vec![
                        Statement::Return(Some(Expression::FunctionCall {
                            name: "add".to_string(),
                            arguments: vec![],
                        })),
                    ],
                }),
            ],
        }
    }

    fn create_complex_program() -> Program {
        Program {
            statements: vec![
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: "fibonacci".to_string(),
                    parameters: vec![],
                    return_type: Some("int".to_string()),
                    body: vec![
                        Statement::VariableDeclaration(VariableDeclaration {
                            name: "n".to_string(),
                            var_type: Some("int".to_string()),
                            value: Some(Expression::Literal("10".to_string())),
                            is_mutable: false,
                        }),
                        Statement::VariableDeclaration(VariableDeclaration {
                            name: "a".to_string(),
                            var_type: Some("int".to_string()),
                            value: Some(Expression::Literal("0".to_string())),
                            is_mutable: true,
                        }),
                        Statement::VariableDeclaration(VariableDeclaration {
                            name: "b".to_string(),
                            var_type: Some("int".to_string()),
                            value: Some(Expression::Literal("1".to_string())),
                            is_mutable: true,
                        }),
                        Statement::Return(Some(Expression::Binary {
                            left: Box::new(Expression::Identifier("a".to_string())),
                            operator: BinaryOperator::Add,
                            right: Box::new(Expression::Identifier("b".to_string())),
                        })),
                    ],
                }),
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: "main".to_string(),
                    parameters: vec![],
                    return_type: Some("int".to_string()),
                    body: vec![
                        Statement::Return(Some(Expression::FunctionCall {
                            name: "fibonacci".to_string(),
                            arguments: vec![],
                        })),
                    ],
                }),
            ],
        }
    }

    #[test]
    fn test_complete_pipeline_simple_program() {
        let mut backend = Backend::new();
        let program = create_simple_program();
        
        let result = backend.compile(program);
        assert!(result.is_ok(), "Complete pipeline should succeed for simple program");
        
        let compiled_program = result.unwrap();
        assert!(!compiled_program.code.is_empty(), "Generated code should not be empty");
        assert!(compiled_program.code.len() > 100, "Generated code should be substantial");
    }

    #[test]
    fn test_complete_pipeline_arithmetic_program() {
        let mut backend = Backend::new();
        let program = create_arithmetic_program();
        
        let result = backend.compile(program);
        assert!(result.is_ok(), "Complete pipeline should succeed for arithmetic program");
        
        let compiled_program = result.unwrap();
        assert!(!compiled_program.code.is_empty(), "Generated code should not be empty");
        assert!(compiled_program.functions.contains_key("add"), "Compiled program should contain add function");
        assert!(compiled_program.functions.contains_key("main"), "Compiled program should contain main function");
    }

    #[test]
    fn test_complete_pipeline_complex_program() {
        let mut backend = Backend::new();
        let program = create_complex_program();
        
        let result = backend.compile(program);
        assert!(result.is_ok(), "Complete pipeline should succeed for complex program");
        
        let compiled_program = result.unwrap();
        assert!(!compiled_program.code.is_empty(), "Generated code should not be empty");
        assert!(compiled_program.functions.contains_key("fibonacci"), "Compiled program should contain fibonacci function");
        assert!(compiled_program.functions.contains_key("main"), "Compiled program should contain main function");
    }

    #[test]
    fn test_pipeline_with_basic_optimization() {
        let mut backend = Backend::new().with_optimization_level(OptimizationLevel::Basic);
        let program = create_arithmetic_program();
        
        let result = backend.compile(program);
        assert!(result.is_ok(), "Pipeline with basic optimization should succeed");
        
        let compiled_program = result.unwrap();
        assert!(!compiled_program.code.is_empty(), "Optimized code should not be empty");
    }

    #[test]
    fn test_pipeline_with_standard_optimization() {
        let mut backend = Backend::new().with_optimization_level(OptimizationLevel::Standard);
        let program = create_arithmetic_program();
        
        let result = backend.compile(program);
        assert!(result.is_ok(), "Pipeline with standard optimization should succeed");
        
        let compiled_program = result.unwrap();
        assert!(!compiled_program.code.is_empty(), "Optimized code should not be empty");
    }

    #[test]
    fn test_pipeline_with_aggressive_optimization() {
        let mut backend = Backend::new().with_optimization_level(OptimizationLevel::Aggressive);
        let program = create_arithmetic_program();
        
        let result = backend.compile(program);
        assert!(result.is_ok(), "Pipeline with aggressive optimization should succeed");
        
        let compiled_program = result.unwrap();
        assert!(!compiled_program.code.is_empty(), "Aggressively optimized code should not be empty");
    }

    #[test]
    fn test_pipeline_with_no_optimization() {
        let mut backend = Backend::new().with_optimization_level(OptimizationLevel::None);
        let program = create_simple_program();
        
        let result = backend.compile(program);
        assert!(result.is_ok(), "Pipeline with no optimization should succeed");
        
        let compiled_program = result.unwrap();
        assert!(!compiled_program.code.is_empty(), "Unoptimized code should not be empty");
    }

    #[test]
    fn test_pipeline_with_linking_config() {
        let linking_config = LinkingConfig {
            output_format: OutputFormat::ELF,
            debug_info: false,
            strip_symbols: true,
        };
        
        let mut backend = Backend::new().with_linking_config(linking_config);
        let program = create_simple_program();
        
        let result = backend.compile(program);
        assert!(result.is_ok(), "Pipeline with linking config should succeed");
    }

    #[test]
    fn test_compile_and_link() {
        let mut backend = Backend::new();
        let program = create_simple_program();
        
        let result = backend.compile_and_link(program, "test_output");
        assert!(result.is_ok(), "Compile and link should succeed");
        
        let executable_path = result.unwrap();
        assert!(executable_path.contains("test_output"), "Executable path should contain specified name");
    }

    #[test]
    fn test_jit_compilation() {
        let mut backend = Backend::new();
        let program = create_simple_program();
        
        let result = backend.jit_compile_and_run(program, "main");
        assert!(result.is_ok(), "JIT compilation should succeed");
        
        let exit_code = result.unwrap();
        assert_eq!(exit_code, 42, "JIT execution should return correct exit code");
    }

    #[test]
    fn test_jit_arithmetic_program() {
        let mut backend = Backend::new();
        let program = create_arithmetic_program();
        
        let result = backend.jit_compile_and_run(program, "main");
        assert!(result.is_ok(), "JIT compilation of arithmetic program should succeed");
        
        let exit_code = result.unwrap();
        assert_eq!(exit_code, 8, "JIT execution should return 5 + 3 = 8");
    }

    #[test]
    fn test_pipeline_error_handling() {
        let mut backend = Backend::new();
        
        // Create a program with semantic errors
        let invalid_program = Program {
            statements: vec![
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: "invalid_func".to_string(),
                    parameters: vec![],
                    return_type: Some("int".to_string()),
                    body: vec![
                        Statement::Return(Some(Expression::Identifier("undefined_var".to_string()))),
                    ],
                }),
            ],
        };
        
        let result = backend.compile(invalid_program);
        assert!(result.is_err(), "Pipeline should handle semantic errors gracefully");
    }

    #[test]
    fn test_pipeline_type_mismatch_error() {
        let mut backend = Backend::new();
        
        // Create a program with type errors
        let type_error_program = Program {
            statements: vec![
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: "type_error_func".to_string(),
                    parameters: vec![],
                    return_type: Some("int".to_string()),
                    body: vec![
                        Statement::VariableDeclaration(VariableDeclaration {
                            name: "str_var".to_string(),
                            var_type: Some("string".to_string()),
                            value: Some(Expression::Literal("\"hello\"".to_string())),
                            is_mutable: false,
                        }),
                        Statement::Return(Some(Expression::Identifier("str_var".to_string()))),
                    ],
                }),
            ],
        };
        
        let result = backend.compile(type_error_program);
        assert!(result.is_err(), "Pipeline should handle type mismatch errors gracefully");
    }

    #[test]
    fn test_pipeline_performance_comparison() {
        let program = create_complex_program();
        
        // Test different optimization levels
        let mut backend_none = Backend::new().with_optimization_level(OptimizationLevel::None);
        let mut backend_basic = Backend::new().with_optimization_level(OptimizationLevel::Basic);
        let mut backend_aggressive = Backend::new().with_optimization_level(OptimizationLevel::Aggressive);
        
        let result_none = backend_none.compile(program.clone());
        let result_basic = backend_basic.compile(program.clone());
        let result_aggressive = backend_aggressive.compile(program);
        
        assert!(result_none.is_ok(), "No optimization should succeed");
        assert!(result_basic.is_ok(), "Basic optimization should succeed");
        assert!(result_aggressive.is_ok(), "Aggressive optimization should succeed");
        
        let code_none = result_none.unwrap().code;
        let code_basic = result_basic.unwrap().code;
        let code_aggressive = result_aggressive.unwrap().code;
        
        // All should produce valid code
        assert!(!code_none.is_empty(), "Unoptimized code should not be empty");
        assert!(!code_basic.is_empty(), "Basic optimized code should not be empty");
        assert!(!code_aggressive.is_empty(), "Aggressively optimized code should not be empty");
    }
}
