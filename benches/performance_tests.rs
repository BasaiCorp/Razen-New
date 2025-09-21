// benches/performance_tests.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use razen_lang::backend::Backend;
use razen_lang::backend::optimization::OptimizationLevel;
use razen_lang::frontend::lexer::Lexer;
use razen_lang::frontend::parser::Parser;
use std::fs;

fn compile_razen_source(source: &str, opt_level: OptimizationLevel) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Tokenize
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    // Parse
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    // Compile
    let mut backend = Backend::new().with_optimization_level(opt_level);
    let compiled_program = backend.compile(program)?;
    
    Ok(compiled_program.code)
}

fn benchmark_simple_compilation(c: &mut Criterion) {
    let source = r#"
        fun main() -> int {
            return 42
        }
    "#;
    
    c.bench_function("simple_compilation", |b| {
        b.iter(|| compile_razen_source(black_box(source), OptimizationLevel::None))
    });
}

fn benchmark_arithmetic_compilation(c: &mut Criterion) {
    let source = r#"
        fun add(a: int, b: int) -> int {
            return a + b
        }
        
        fun main() -> int {
            var x: int = add(5, 3)
            var y: int = x * 2
            return y
        }
    "#;
    
    c.bench_function("arithmetic_compilation", |b| {
        b.iter(|| compile_razen_source(black_box(source), OptimizationLevel::Basic))
    });
}

fn benchmark_optimization_levels(c: &mut Criterion) {
    let source = r#"
        fun factorial(n: int) -> int {
            if n <= 1 {
                return 1
            } else {
                return n * factorial(n - 1)
            }
        }
        
        fun main() -> int {
            var result: int = factorial(5)
            return result
        }
    "#;
    
    let mut group = c.benchmark_group("optimization_levels");
    
    group.bench_function("none", |b| {
        b.iter(|| compile_razen_source(black_box(source), OptimizationLevel::None))
    });
    
    group.bench_function("basic", |b| {
        b.iter(|| compile_razen_source(black_box(source), OptimizationLevel::Basic))
    });
    
    group.bench_function("standard", |b| {
        b.iter(|| compile_razen_source(black_box(source), OptimizationLevel::Standard))
    });
    
    group.bench_function("aggressive", |b| {
        b.iter(|| compile_razen_source(black_box(source), OptimizationLevel::Aggressive))
    });
    
    group.finish();
}

fn benchmark_complex_program(c: &mut Criterion) {
    let source = r#"
        fun fibonacci(n: int) -> int {
            if n <= 1 {
                return n
            } else {
                return fibonacci(n - 1) + fibonacci(n - 2)
            }
        }
        
        fun factorial(n: int) -> int {
            if n <= 1 {
                return 1
            } else {
                return n * factorial(n - 1)
            }
        }
        
        fun gcd(a: int, b: int) -> int {
            while b != 0 {
                var temp: int = b
                b = a % b
                a = temp
            }
            return a
        }
        
        fun main() -> int {
            var fib_result: int = fibonacci(7)
            var fact_result: int = factorial(5)
            var gcd_result: int = gcd(48, 18)
            return fib_result + fact_result + gcd_result
        }
    "#;
    
    c.bench_function("complex_program", |b| {
        b.iter(|| compile_razen_source(black_box(source), OptimizationLevel::Standard))
    });
}

fn benchmark_jit_execution(c: &mut Criterion) {
    let source = r#"
        fun main() -> int {
            var a: int = 10
            var b: int = 20
            return a + b
        }
    "#;
    
    c.bench_function("jit_execution", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(source));
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let program = parser.parse().unwrap();
            let mut backend = Backend::new();
            backend.jit_compile_and_run(program, "main")
        })
    });
}

criterion_group!(
    benches,
    benchmark_simple_compilation,
    benchmark_arithmetic_compilation,
    benchmark_optimization_levels,
    benchmark_complex_program,
    benchmark_jit_execution
);
criterion_main!(benches);
