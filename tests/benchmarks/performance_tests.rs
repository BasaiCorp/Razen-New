// tests/benchmarks/performance_tests.rs

use razen_lang::backend::Backend;
use razen_lang::backend::optimization::OptimizationLevel;
use razen_lang::frontend::lexer::Lexer;
use razen_lang::frontend::parser::Parser;
use std::time::{Duration, Instant};

#[cfg(test)]
mod performance_benchmarks {
    use super::*;

    struct BenchmarkResult {
        name: String,
        duration: Duration,
        code_size: usize,
        success: bool,
    }

    impl BenchmarkResult {
        fn new(name: String, duration: Duration, code_size: usize, success: bool) -> Self {
            BenchmarkResult {
                name,
                duration,
                code_size,
                success,
            }
        }

        fn print(&self) {
            println!(
                "Benchmark: {} | Duration: {:?} | Code Size: {} bytes | Success: {}",
                self.name, self.duration, self.code_size, self.success
            );
        }
    }

    fn benchmark_compilation(name: &str, source: &str, opt_level: OptimizationLevel) -> BenchmarkResult {
        let start = Instant::now();
        
        // Tokenize
        let mut lexer = Lexer::new(source);
        let tokens_result = lexer.tokenize();
        
        if tokens_result.is_err() {
            return BenchmarkResult::new(name.to_string(), start.elapsed(), 0, false);
        }
        
        // Parse
        let mut parser = Parser::new(tokens_result.unwrap());
        let program_result = parser.parse();
        
        if program_result.is_err() {
            return BenchmarkResult::new(name.to_string(), start.elapsed(), 0, false);
        }
        
        // Compile
        let mut backend = Backend::new().with_optimization_level(opt_level);
        let compile_result = backend.compile(program_result.unwrap());
        
        let duration = start.elapsed();
        
        match compile_result {
            Ok(compiled_program) => BenchmarkResult::new(
                name.to_string(),
                duration,
                compiled_program.bytecode.len(),
                true,
            ),
            Err(_) => BenchmarkResult::new(name.to_string(), duration, 0, false),
        }
    }

    fn benchmark_jit_execution(name: &str, source: &str, entry_point: &str) -> BenchmarkResult {
        let start = Instant::now();
        
        // Tokenize
        let mut lexer = Lexer::new(source);
        let tokens_result = lexer.tokenize();
        
        if tokens_result.is_err() {
            return BenchmarkResult::new(name.to_string(), start.elapsed(), 0, false);
        }
        
        // Parse
        let mut parser = Parser::new(tokens_result.unwrap());
        let program_result = parser.parse();
        
        if program_result.is_err() {
            return BenchmarkResult::new(name.to_string(), start.elapsed(), 0, false);
        }
        
        // JIT compile and run
        let mut backend = Backend::new();
        let jit_result = backend.jit_compile_and_run(program_result.unwrap(), entry_point);
        
        let duration = start.elapsed();
        
        match jit_result {
            Ok(_) => BenchmarkResult::new(name.to_string(), duration, 0, true),
            Err(_) => BenchmarkResult::new(name.to_string(), duration, 0, false),
        }
    }

    #[test]
    fn benchmark_simple_program_compilation() {
        let source = r#"
            fun main() -> int {
                return 42;
            }
        "#;
        
        let result = benchmark_compilation("Simple Program", source, OptimizationLevel::None);
        result.print();
        
        assert!(result.success, "Simple program compilation should succeed");
        assert!(result.duration < Duration::from_secs(5), "Compilation should be fast");
        assert!(result.code_size > 0, "Should generate code");
    }

    #[test]
    fn benchmark_arithmetic_program_compilation() {
        let source = r#"
            fun add(a: int, b: int) -> int {
                return a + b
            }
            
            fun multiply(a: int, b: int) -> int {
                return a * b
            }
            
            fun main() -> int {
                var x: int = add(5, 3)
                var y: int = multiply(x, 2)
                return y
            }
        "#;
        
        let result = benchmark_compilation("Arithmetic Program", source, OptimizationLevel::Basic);
        result.print();
        
        assert!(result.success, "Arithmetic program compilation should succeed");
        assert!(result.duration < Duration::from_secs(10), "Compilation should be reasonably fast");
    }

    #[test]
    fn benchmark_optimization_levels() {
        let source = r#"
            fun factorial(n: int) -> int {
                if n <= 1 {
                    return 1;
                } else {
                    return n * factorial(n - 1);
                }
            }
            
            fun main() -> int {
                let result: int = factorial(10);
                return result;
            }
        "#;
        
        let optimization_levels = [
            OptimizationLevel::None,
            OptimizationLevel::Basic,
            OptimizationLevel::Standard,
            OptimizationLevel::Aggressive,
        ];
        
        let mut results = Vec::new();
        
        for opt_level in optimization_levels {
            let name = format!("Factorial {:?}", opt_level);
            let result = benchmark_compilation(&name, source, opt_level);
            result.print();
            results.push(result);
        }
        
        // All optimization levels should succeed
        for result in &results {
            assert!(result.success, "Compilation with {} should succeed", result.name);
        }
        
        // Higher optimization levels might take longer but should still be reasonable
        for result in &results {
            assert!(result.duration < Duration::from_secs(30), "Compilation should not take too long");
        }
    }

    #[test]
    fn benchmark_large_program_compilation() {
        let source = r#"
            fun fibonacci(n: int) -> int {
                if n <= 1 {
                    return n;
                } else {
                    return fibonacci(n - 1) + fibonacci(n - 2);
                }
            }
            
            fun factorial(n: int) -> int {
                if n <= 1 {
                    return 1;
                } else {
                    return n * factorial(n - 1);
                }
            }
            
            fun gcd(a: int, b: int) -> int {
                while b != 0 {
                    let temp: int = b;
                    b = a % b;
                    a = temp;
                }
                return a;
            }
            
            fun lcm(a: int, b: int) -> int {
                return (a * b) / gcd(a, b);
            }
            
            fun is_prime(n: int) -> bool {
                if n <= 1 {
                    return false;
                }
                if n <= 3 {
                    return true;
                }
                if n % 2 == 0 || n % 3 == 0 {
                    return false;
                }
                
                let i: int = 5;
                while i * i <= n {
                    if n % i == 0 || n % (i + 2) == 0 {
                        return false;
                    }
                    i = i + 6;
                }
                return true;
            }
            
            fun sum_of_primes(limit: int) -> int {
                let sum: int = 0;
                let i: int = 2;
                while i < limit {
                    if is_prime(i) {
                        sum = sum + i;
                    }
                    i = i + 1;
                }
                return sum;
            }
            
            fun main() -> int {
                let fib_result: int = fibonacci(10);
                let fact_result: int = factorial(5);
                let gcd_result: int = gcd(48, 18);
                let lcm_result: int = lcm(12, 8);
                let prime_sum: int = sum_of_primes(20);
                
                return fib_result + fact_result + gcd_result + lcm_result + prime_sum;
            }
        "#;
        
        let result = benchmark_compilation("Large Program", source, OptimizationLevel::Standard);
        result.print();
        
        assert!(result.success, "Large program compilation should succeed");
        assert!(result.duration < Duration::from_secs(60), "Large program compilation should complete within reasonable time");
        assert!(result.code_size > 1000, "Large program should generate substantial code");
    }

    #[test]
    fn benchmark_jit_execution_simple() {
        let source = r#"
            fun main() -> int {
                return 42;
            }
        "#;
        
        let result = benchmark_jit_execution("JIT Simple", source, "main");
        result.print();
        
        assert!(result.success, "JIT execution should succeed");
        assert!(result.duration < Duration::from_secs(5), "JIT execution should be fast");
    }

    #[test]
    fn benchmark_jit_execution_arithmetic() {
        let source = r#"
            fun main() -> int {
                let a: int = 10;
                let b: int = 20;
                let c: int = 30;
                let result: int = (a + b) * c - 5;
                return result;
            }
        "#;
        
        let result = benchmark_jit_execution("JIT Arithmetic", source, "main");
        result.print();
        
        assert!(result.success, "JIT arithmetic execution should succeed");
        assert!(result.duration < Duration::from_secs(10), "JIT arithmetic should be reasonably fast");
    }

    #[test]
    fn benchmark_compilation_phases() {
        let source = r#"
            fun test_func(x: int, y: int) -> int {
                let a: int = x + y;
                let b: int = x * y;
                let c: int = a - b;
                return c;
            }
            
            fun main() -> int {
                return test_func(5, 3);
            }
        "#;
        
        // Benchmark individual phases
        let start_total = Instant::now();
        
        // Phase 1: Lexing
        let start_lex = Instant::now();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Tokenization should succeed");
        let lex_duration = start_lex.elapsed();
        
        // Phase 2: Parsing
        let start_parse = Instant::now();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().expect("Parsing should succeed");
        let parse_duration = start_parse.elapsed();
        
        // Phase 3: Compilation
        let start_compile = Instant::now();
        let mut backend = Backend::new();
        let compiled_program = backend.compile(program).expect("Compilation should succeed");
        let compile_duration = start_compile.elapsed();
        
        let total_duration = start_total.elapsed();
        
        println!("Phase Benchmarks:");
        println!("  Lexing: {:?}", lex_duration);
        println!("  Parsing: {:?}", parse_duration);
        println!("  Compilation: {:?}", compile_duration);
        println!("  Total: {:?}", total_duration);
        println!("  Code Size: {} bytes", compiled_program.bytecode.len());
        
        // Assertions
        assert!(lex_duration < Duration::from_millis(100), "Lexing should be very fast");
        assert!(parse_duration < Duration::from_millis(500), "Parsing should be fast");
        assert!(compile_duration < Duration::from_secs(5), "Compilation should be reasonable");
        assert!(total_duration < Duration::from_secs(10), "Total compilation should be fast");
    }

    #[test]
    fn benchmark_memory_usage() {
        let source = r#"
            fun recursive_func(n: int) -> int {
                if n <= 0 {
                    return 0;
                } else {
                    return n + recursive_func(n - 1);
                }
            }
            
            fun main() -> int {
                return recursive_func(100);
            }
        "#;
        
        // This is a basic memory usage test - in a real scenario you'd use more sophisticated memory profiling
        let start = Instant::now();
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Tokenization should succeed");
        
        let mut parser = Parser::new(tokens);
        let program = parser.parse().expect("Parsing should succeed");
        
        let mut backend = Backend::new();
        let compiled_program = backend.compile(program).expect("Compilation should succeed");
        
        let duration = start.elapsed();
        
        println!("Memory Usage Benchmark:");
        println!("  Duration: {:?}", duration);
        println!("  Code Size: {} bytes", compiled_program.bytecode.len());
        
        assert!(duration < Duration::from_secs(15), "Recursive function compilation should complete");
        assert!(compiled_program.bytecode.len() > 0, "Should generate code");
    }

    #[test]
    fn benchmark_stress_test() {
        // Generate a large program programmatically
        let mut source = String::new();
        
        // Generate many functions
        for i in 0..50 {
            source.push_str(&format!(
                "fun func_{i}(x: int) -> int {{ return x + {i}; }}\n"
            ));
        }
        
        // Generate main function that calls all of them
        source.push_str("fun main() -> int {\n");
        source.push_str("    let result: int = 0;\n");
        for i in 0..50 {
            source.push_str(&format!("    result = result + func_{i}({i});\n"));
        }
        source.push_str("    return result;\n");
        source.push_str("}\n");
        
        let result = benchmark_compilation("Stress Test", &source, OptimizationLevel::Basic);
        result.print();
        
        assert!(result.success, "Stress test compilation should succeed");
        assert!(result.duration < Duration::from_secs(120), "Stress test should complete within reasonable time");
        assert!(result.code_size > 2000, "Stress test should generate substantial code");
    }
}
