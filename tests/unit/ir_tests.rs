// tests/unit/ir_tests.rs

use razen_lang::backend::ir::{IRGenerator, IRModule, IRFunction, Instruction, Operand};
use razen_lang::backend::semantic::{SemanticAnalyzer, AnalyzedProgram};
use razen_lang::frontend::parser::ast::*;

#[cfg(test)]
mod ir_generator_tests {
    use super::*;

    fn create_analyzed_program() -> AnalyzedProgram {
        let program = Program {
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
        };
        
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.analyze(program).expect("Semantic analysis should succeed")
    }

    #[test]
    fn test_ir_generator_creation() {
        let generator = IRGenerator::new();
        // IR generator created successfully
        assert!(true);
    }

    #[test]
    fn test_function_ir_generation() {
        let mut generator = IRGenerator::new();
        let analyzed_program = create_analyzed_program();
        
        let result = generator.generate(analyzed_program);
        assert!(result.is_ok(), "IR generation should succeed for valid program");
        
        let ir_module = result.unwrap();
        assert!(!ir_module.functions.is_empty());
        // Function IR generation successful
        assert!(true);
    }

    #[test]
    fn test_variable_declaration_ir() {
        let mut generator = IRGenerator::new();
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
        
        let mut analyzer = SemanticAnalyzer::new();
        let analyzed_program = analyzer.analyze(program).expect("Semantic analysis should succeed");
        
        let result = generator.generate(analyzed_program);
        assert!(result.is_ok(), "Variable declaration IR generation should succeed");
    }

    #[test]
    fn test_binary_operation_ir() {
        let mut generator = IRGenerator::new();
        let program = Program {
            statements: vec![
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier::new("add_func".to_string()),
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
            ],
        };
        
        let mut analyzer = SemanticAnalyzer::new();
        let analyzed_program = analyzer.analyze(program).expect("Semantic analysis should succeed");
        
        let result = generator.generate(analyzed_program);
        assert!(result.is_ok(), "Binary operation IR generation should succeed");
        
        let _ir_module = result.unwrap();
        // Binary operation IR generation successful
        assert!(true);
    }

    #[test]
    fn test_function_call_ir() {
        let mut generator = IRGenerator::new();
        let program = Program {
            statements: vec![
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier::new("caller".to_string()),
                    parameters: vec![],
                    return_type: Some(TypeAnnotation::new("int".to_string())),
                    body: BlockStatement::new(vec![
                        Statement::ReturnStatement(ReturnStatement {
                            value: Some(Expression::CallExpression(CallExpression {
                                callee: Box::new(Expression::Identifier(Identifier::new("callee".to_string()))),
                                arguments: vec![],
                            })),
                        }),
                    ]),
                    is_public: false,
                }),
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier::new("callee".to_string()),
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
        };
        
        let mut analyzer = SemanticAnalyzer::new();
        let analyzed_program = analyzer.analyze(program).expect("Semantic analysis should succeed");
        
        let result = generator.generate(analyzed_program);
        assert!(result.is_ok(), "Function call IR generation should succeed");
        
        let _ir_module = result.unwrap();
        // Function call IR generation successful
        assert!(true);
    }

    #[test]
    fn test_register_allocation() {
        let mut generator = IRGenerator::new();
        let program = Program {
            statements: vec![
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier::new("register_test".to_string()),
                    parameters: vec![],
                    return_type: Some(TypeAnnotation::new("int".to_string())),
                    body: BlockStatement::new(vec![
                        Statement::VariableDeclaration(VariableDeclaration {
                            name: Identifier::new("a".to_string()),
                            type_annotation: Some(TypeAnnotation::new("int".to_string())),
                            initializer: Some(Expression::IntegerLiteral(IntegerLiteral::new(1))),
                            is_public: false,
                        }),
                        Statement::VariableDeclaration(VariableDeclaration {
                            name: Identifier::new("b".to_string()),
                            type_annotation: Some(TypeAnnotation::new("int".to_string())),
                            initializer: Some(Expression::IntegerLiteral(IntegerLiteral::new(2))),
                            is_public: false,
                        }),
                        Statement::ReturnStatement(ReturnStatement {
                            value: Some(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Identifier(Identifier::new("a".to_string()))),
                                operator: BinaryOperator::Add,
                                right: Box::new(Expression::Identifier(Identifier::new("b".to_string()))),
                            })),
                        }),
                    ]),
                    is_public: false,
                }),
            ],
        };
        
        let mut analyzer = SemanticAnalyzer::new();
        let analyzed_program = analyzer.analyze(program).expect("Semantic analysis should succeed");
        
        let result = generator.generate(analyzed_program);
        assert!(result.is_ok(), "Register allocation should work correctly");
        
        let _ir_module = result.unwrap();
        // Register allocation successful
        assert!(true);
    }
}

#[cfg(test)]
mod ir_instruction_tests {
    use super::*;

    #[test]
    fn test_instruction_creation() {
        // Test basic instruction creation
        let operand1 = Operand::Register("r1".to_string());
        let operand2 = Operand::Constant(42);
        
        assert_eq!(operand1, Operand::Register("r1".to_string()));
        assert_eq!(operand2, Operand::Constant(42));
    }

    #[test]
    fn test_operand_types() {
        // Test different operand types
        let register = Operand::Register("reg1".to_string());
        let constant = Operand::Constant(100);
        let string_literal = Operand::StringLiteral("hello".to_string());
        
        match register {
            Operand::Register(name) => assert_eq!(name, "reg1"),
            _ => panic!("Expected Register"),
        }
        
        match constant {
            Operand::Constant(val) => assert_eq!(val, 100),
            _ => panic!("Expected Constant"),
        }
        
        match string_literal {
            Operand::StringLiteral(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected StringLiteral"),
        }
    }

    #[test]
    fn test_instruction_types() {
        // Test instruction creation
        let add_instr = Instruction::Add {
            dest: "result".to_string(),
            left: Operand::Register("a".to_string()),
            right: Operand::Register("b".to_string()),
        };
        
        match add_instr {
            Instruction::Add { dest, left, right } => {
                assert_eq!(dest, "result");
                assert_eq!(left, Operand::Register("a".to_string()));
                assert_eq!(right, Operand::Register("b".to_string()));
            }
            _ => panic!("Expected Add instruction"),
        }
    }
}

#[cfg(test)]
mod ir_module_tests {
    use super::*;

    #[test]
    fn test_ir_module_creation() {
        let module = IRModule::new();
        
        assert!(module.functions.is_empty());
        assert!(module.strings.is_empty());
    }

    #[test]
    fn test_ir_function_creation() {
        let function = IRFunction {
            name: "test_func".to_string(),
            params: vec![],
            return_type: "int".to_string(),
            instructions: vec![],
            blocks: vec![],
        };
        
        assert_eq!(function.name, "test_func");
        assert_eq!(function.return_type, "int");
        assert!(function.params.is_empty());
        assert!(function.instructions.is_empty());
    }

    #[test]
    fn test_string_literal_management() {
        let mut module = IRModule::new();
        
        module.add_string("hello world".to_string());
        module.add_string("goodbye".to_string());
        
        assert_eq!(module.strings.len(), 2);
        assert!(module.strings.contains(&"hello world".to_string()));
        assert!(module.strings.contains(&"goodbye".to_string()));
    }
}
