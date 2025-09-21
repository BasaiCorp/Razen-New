// tests/integration/end_to_end_tests.rs

use razen_lang::frontend::lexer::Lexer;
use razen_lang::frontend::parser::Parser;
use razen_lang::backend::Backend;
use razen_lang::backend::optimization::OptimizationLevel;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod end_to_end_tests {
    use super::*;

    fn compile_razen_source(source: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Tokenize
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        
        // Parse
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        
        // Compile
        let mut backend = Backend::new();
        let compiled_program = backend.compile(program)?;
        
        Ok(compiled_program.bytecode)
    }

    fn jit_compile_and_run_razen_source(source: &str, entry_point: &str) -> Result<i32, Box<dyn std::error::Error>> {
        // Tokenize
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        
        // Parse
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        
        // JIT compile and run
        let mut backend = Backend::new();
        let exit_code = backend.jit_compile_and_run(program, entry_point)?;
        
        Ok(exit_code)
    }

    #[test]
    fn test_hello_world_program() {
        let source = r#"
            fun main() -> int {
                return 42
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_ok(), "Hello world program should compile successfully");
        
        let code = result.unwrap();
        assert!(!code.is_empty(), "Generated code should not be empty");
        assert!(code.len() > 50, "Generated code should be substantial");
    }

    #[test]
    fn test_arithmetic_program() {
        let source = r#"
            fun add(a: int, b: int) -> int {
                return a + b
            }
            
            fun main() -> int {
                var result = add(5, 3)
                return result
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_ok(), "Arithmetic program should compile successfully");
        
        let code = result.unwrap();
        assert!(!code.is_empty(), "Generated code should not be empty");
    }

    #[test]
    fn test_variable_declarations() {
        let source = r#"
            fun main() -> int {
                var a: int = 10
                var b: int = 20
                var c: int = a + b
                return c
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_ok(), "Variable declaration program should compile successfully");
    }

    #[test]
    fn test_function_calls() {
        let source = r#"
            fun multiply(x: int, y: int) -> int {
                return x * y
            }
            
            fun square(n: int) -> int {
                return multiply(n, n)
            }
            
            fun main() -> int {
                return square(7)
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_ok(), "Function call program should compile successfully");
    }

    #[test]
    fn test_complex_expressions() {
        let source = r#"
            fun main() -> int {
                var a: int = 5
                var b: int = 3
                var c: int = 2
                var result: int = (a + b) * c - 1
                return result
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_ok(), "Complex expression program should compile successfully");
    }

    #[test]
    fn test_jit_hello_world() {
        let source = r#"
            fun main() -> int {
                return 42
            }
        "#;
        
        let result = jit_compile_and_run_razen_source(source, "main");
        assert!(result.is_ok(), "JIT hello world should execute successfully");
        
        let exit_code = result.unwrap();
        assert_eq!(exit_code, 42, "JIT execution should return 42");
    }

    #[test]
    fn test_jit_arithmetic() {
        let source = r#"
            fn main() -> int {
                let a: int = 10;
                let b: int = 5;
                return a + b;
            }
        "#;
        
        let result = jit_compile_and_run_razen_source(source, "main");
        assert!(result.is_ok(), "JIT arithmetic should execute successfully");
        
        let exit_code = result.unwrap();
        assert_eq!(exit_code, 15, "JIT execution should return 10 + 5 = 15");
    }

    #[test]
    fn test_jit_function_calls() {
        let source = r#"
            fn add(x: int, y: int) -> int {
                return x + y;
            }
            
            fn main() -> int {
                return add(7, 8);
            }
        "#;
        
        let result = jit_compile_and_run_razen_source(source, "main");
        assert!(result.is_ok(), "JIT function calls should execute successfully");
        
        let exit_code = result.unwrap();
        assert_eq!(exit_code, 15, "JIT execution should return 7 + 8 = 15");
    }

    #[test]
    fn test_syntax_error_handling() {
        let source = r#"
            fn main() -> int {
                return 42
            // Missing semicolon
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_err(), "Syntax error should be caught");
    }

    #[test]
    fn test_type_error_handling() {
        let source = r#"
            fn main() -> int {
                let str_var: string = "hello";
                return str_var; // Type mismatch
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_err(), "Type error should be caught");
    }

    #[test]
    fn test_undefined_variable_error() {
        let source = r#"
            fn main() -> int {
                return undefined_var; // Undefined variable
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_err(), "Undefined variable error should be caught");
    }

    #[test]
    fn test_undefined_function_error() {
        let source = r#"
            fn main() -> int {
                return undefined_func(); // Undefined function
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_err(), "Undefined function error should be caught");
    }

    #[test]
    fn test_optimization_levels() {
        let source = r#"
            fn main() -> int {
                let a: int = 5 + 3; // Constant folding opportunity
                let b: int = 10 * 2; // Another constant folding opportunity
                let unused: int = 99; // Unused variable
                return a + b;
            }
        "#;
        
        // Test with different optimization levels
        for opt_level in [OptimizationLevel::None, OptimizationLevel::Basic, OptimizationLevel::Standard, OptimizationLevel::Aggressive] {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect("Tokenization should succeed");
            
            let mut parser = Parser::new(tokens);
            let program = parser.parse().expect("Parsing should succeed");
            
            let mut backend = Backend::new().with_optimization_level(opt_level);
            let result = backend.compile(program);
            
            assert!(result.is_ok(), "Compilation with optimization level {:?} should succeed", opt_level);
        }
    }

    #[test]
    fn test_large_program() {
        let source = r#"
            fn factorial(n: int) -> int {
                if n <= 1 {
                    return 1;
                } else {
                    return n * factorial(n - 1);
                }
            }
            
            fn fibonacci(n: int) -> int {
                if n <= 1 {
                    return n;
                } else {
                    return fibonacci(n - 1) + fibonacci(n - 2);
                }
            }
            
            fn gcd(a: int, b: int) -> int {
                while b != 0 {
                    let temp: int = b;
                    b = a % b;
                    a = temp;
                }
                return a;
            }
            
            fn main() -> int {
                let fact5: int = factorial(5);
                let fib7: int = fibonacci(7);
                let gcd_result: int = gcd(48, 18);
                return fact5 + fib7 + gcd_result;
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_ok(), "Large program should compile successfully");
        
        let code = result.unwrap();
        assert!(!code.is_empty(), "Generated code should not be empty");
        assert!(code.len() > 500, "Large program should generate substantial code");
    }

    #[test]
    fn test_string_operations() {
        let source = r#"
            fn main() -> int {
                let greeting: string = "Hello";
                let name: string = "World";
                let message: string = greeting + " " + name;
                return 42; // Return success code
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_ok(), "String operations should compile successfully");
    }

    #[test]
    fn test_boolean_operations() {
        let source = r#"
            fn main() -> int {
                let a: bool = true;
                let b: bool = false;
                let c: bool = a && b;
                let d: bool = a || b;
                let e: bool = !a;
                
                if c {
                    return 1;
                } else if d {
                    return 2;
                } else if e {
                    return 3;
                } else {
                    return 4;
                }
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_ok(), "Boolean operations should compile successfully");
    }

    #[test]
    fn test_comparison_operations() {
        let source = r#"
            fn main() -> int {
                let a: int = 10;
                let b: int = 5;
                
                if a > b {
                    return 1;
                } else if a < b {
                    return 2;
                } else if a == b {
                    return 3;
                } else if a != b {
                    return 4;
                } else if a >= b {
                    return 5;
                } else if a <= b {
                    return 6;
                } else {
                    return 0;
                }
            }
        "#;
        
        let result = compile_razen_source(source);
        assert!(result.is_ok(), "Comparison operations should compile successfully");
    }

    #[test]
    fn test_nested_function_calls() {
        let source = r#"
            fn add(a: int, b: int) -> int {
                return a + b;
            }
            
            fn multiply(a: int, b: int) -> int {
                return a * b;
            }
            
            fn complex_calculation(x: int, y: int, z: int) -> int {
                return add(multiply(x, y), z);
            }
            
            fn main() -> int {
                return complex_calculation(3, 4, 5); // Should return 3*4+5 = 17
            }
        "#;
        
        let result = jit_compile_and_run_razen_source(source, "main");
        assert!(result.is_ok(), "Nested function calls should execute successfully");
        
        let exit_code = result.unwrap();
        assert_eq!(exit_code, 17, "JIT execution should return 3*4+5 = 17");
    }
}
