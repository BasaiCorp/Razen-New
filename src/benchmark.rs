// src/benchmark.rs
//! Comprehensive benchmarking system for RAJIT JIT performance validation

use std::time::{Duration, Instant};
use std::path::Path;
use crate::frontend::parser::{parse_source_with_name, format_parse_errors};
use crate::backend::execution::Compiler;
use crate::backend::{SemanticAnalyzer, NativeJIT};
use crate::frontend::diagnostics::display::render_diagnostics;

/// Benchmark result containing performance metrics
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub compilation_time: Duration,
    pub execution_time: Duration,
    pub total_time: Duration,
    pub strategy_used: String,
    pub cache_hits: usize,
    pub ir_instructions: usize,
    pub optimization_level: u8,
}

/// Benchmarking suite for JIT performance validation
pub struct BenchmarkSuite {
    results: Vec<BenchmarkResult>,
}

impl BenchmarkSuite {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }
    
    /// Run a single benchmark test
    pub fn run_benchmark(&mut self, name: &str, file_path: &str) -> Result<BenchmarkResult, String> {
        println!("Running benchmark: {}", name);
        
        // Read the source file
        let source = std::fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read benchmark file {}: {}", file_path, e))?;
        
        let compilation_start = Instant::now();
        
        // Parse the source code
        let filename = file_path.to_string();
        let (program, diagnostics) = parse_source_with_name(&source, &filename);
        
        if !diagnostics.is_empty() {
            let formatted_errors = format_parse_errors(&diagnostics, &source, &filename);
            return Err(format!("Parsing errors: {}", formatted_errors));
        }
        
        let program = program.ok_or("Failed to parse the source file")?;
        
        // Semantic analysis
        let base_dir = std::path::Path::new(file_path).parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();
        let mut semantic_analyzer = SemanticAnalyzer::with_module_support(base_dir, std::path::PathBuf::from(file_path));
        let semantic_diagnostics = semantic_analyzer.analyze_with_source(&program, &source);
        
        if semantic_diagnostics.has_errors() {
            let sources = vec![("source".to_string(), source.clone())];
            let rendered = render_diagnostics(&semantic_diagnostics, &sources);
            return Err(format!("Semantic errors: {}", rendered));
        }
        
        // Compile to IR
        let mut compiler = Compiler::new();
        compiler.set_clean_output(true); // Suppress debug output for benchmarks
        compiler.set_current_file(std::path::PathBuf::from(file_path));
        compiler.compile_program(program);
        
        if !compiler.errors.is_empty() {
            return Err(format!("Compilation failed: {}", compiler.errors.join("; ")));
        }
        
        let compilation_time = compilation_start.elapsed();
        let ir_count = compiler.ir.len();
        
        // JIT compilation and execution with optimization level 2 (standard)
        let mut jit = NativeJIT::with_optimization(2)
            .map_err(|e| format!("JIT initialization error: {}", e))?;
        jit.set_clean_output(true); // Suppress debug output for benchmarks
        
        // Register function parameter names
        for (func_name, params) in &compiler.function_param_names {
            jit.register_function_params(func_name.clone(), params.clone());
        }
        
        let execution_start = Instant::now();
        let _result = jit.compile_and_run(&compiler.ir)
            .map_err(|e| format!("JIT execution error: {}", e))?;
        let execution_time = execution_start.elapsed();
        
        let total_time = compilation_time + execution_time;
        let stats = jit.get_stats();
        
        // Determine strategy used based on execution counts
        let strategy_used = if stats.native_executions > 0 {
            "Native x86-64".to_string()
        } else if stats.bytecode_executions > 0 {
            "Bytecode".to_string()
        } else {
            "Runtime".to_string()
        };
        
        let result = BenchmarkResult {
            name: name.to_string(),
            compilation_time,
            execution_time,
            total_time,
            strategy_used,
            cache_hits: 0, // Will be updated in future iterations
            ir_instructions: ir_count,
            optimization_level: stats.optimization_level,
        };
        
        println!("  Strategy: {}", result.strategy_used);
        println!("  Compilation: {:?}", result.compilation_time);
        println!("  Execution: {:?}", result.execution_time);
        println!("  Total: {:?}", result.total_time);
        println!("  IR Instructions: {}", result.ir_instructions);
        
        self.results.push(result.clone());
        Ok(result)
    }
    
    /// Run multiple iterations of a benchmark for statistical accuracy
    pub fn run_benchmark_iterations(&mut self, name: &str, file_path: &str, iterations: usize) -> Result<BenchmarkStats, String> {
        let mut results = Vec::new();
        
        println!("Running {} iterations of benchmark: {}", iterations, name);
        
        for i in 1..=iterations {
            print!("  Iteration {}/{} ... ", i, iterations);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            
            let result = self.run_benchmark(&format!("{}_iter_{}", name, i), file_path)?;
            results.push(result);
            
            println!("Done");
        }
        
        Ok(BenchmarkStats::from_results(name, results))
    }
    
    /// Run all benchmark files in the benchmarks directory
    pub fn run_all_benchmarks(&mut self) -> Result<(), String> {
        let benchmark_dir = "benchmarks";
        
        if !Path::new(benchmark_dir).exists() {
            return Err("Benchmarks directory not found".to_string());
        }
        
        let benchmarks = [
            ("Arithmetic Heavy", "benchmarks/arithmetic_heavy.rzn"),
            ("Mixed Operations", "benchmarks/mixed_operations.rzn"),
            ("Simple Runtime", "benchmarks/simple_runtime.rzn"),
        ];
        
        println!("=== RAJIT JIT Performance Benchmarks ===\n");
        
        for (name, file_path) in &benchmarks {
            if Path::new(file_path).exists() {
                match self.run_benchmark_iterations(name, file_path, 3) {
                    Ok(stats) => {
                        println!("\n{}", stats);
                    }
                    Err(e) => {
                        println!("Benchmark {} failed: {}", name, e);
                    }
                }
                println!();
            } else {
                println!("Benchmark file not found: {}", file_path);
            }
        }
        
        self.print_summary();
        Ok(())
    }
    
    /// Print benchmark summary
    pub fn print_summary(&self) {
        if self.results.is_empty() {
            println!("No benchmark results available.");
            return;
        }
        
        println!("=== Benchmark Summary ===");
        
        let mut native_count = 0;
        let mut bytecode_count = 0;
        let mut runtime_count = 0;
        
        let mut total_compilation_time = Duration::new(0, 0);
        let mut total_execution_time = Duration::new(0, 0);
        
        for result in &self.results {
            match result.strategy_used.as_str() {
                "Native x86-64" => native_count += 1,
                "Bytecode" => bytecode_count += 1,
                "Runtime" => runtime_count += 1,
                _ => {}
            }
            
            total_compilation_time += result.compilation_time;
            total_execution_time += result.execution_time;
        }
        
        println!("Total benchmarks run: {}", self.results.len());
        println!("Strategy distribution:");
        println!("  Native x86-64: {} ({:.1}%)", native_count, (native_count as f64 / self.results.len() as f64) * 100.0);
        println!("  Bytecode: {} ({:.1}%)", bytecode_count, (bytecode_count as f64 / self.results.len() as f64) * 100.0);
        println!("  Runtime: {} ({:.1}%)", runtime_count, (runtime_count as f64 / self.results.len() as f64) * 100.0);
        println!("Average compilation time: {:?}", total_compilation_time / self.results.len() as u32);
        println!("Average execution time: {:?}", total_execution_time / self.results.len() as u32);
    }
}

/// Statistical analysis of benchmark results
#[derive(Debug)]
pub struct BenchmarkStats {
    pub name: String,
    pub iterations: usize,
    pub avg_compilation_time: Duration,
    pub avg_execution_time: Duration,
    pub avg_total_time: Duration,
    pub min_total_time: Duration,
    pub max_total_time: Duration,
    pub strategy_used: String,
}

impl BenchmarkStats {
    fn from_results(name: &str, results: Vec<BenchmarkResult>) -> Self {
        let iterations = results.len();
        
        let total_compilation: Duration = results.iter().map(|r| r.compilation_time).sum();
        let total_execution: Duration = results.iter().map(|r| r.execution_time).sum();
        let total_time: Duration = results.iter().map(|r| r.total_time).sum();
        
        let min_total_time = results.iter().map(|r| r.total_time).min().unwrap_or(Duration::new(0, 0));
        let max_total_time = results.iter().map(|r| r.total_time).max().unwrap_or(Duration::new(0, 0));
        
        let strategy_used = results.first().map(|r| r.strategy_used.clone()).unwrap_or_else(|| "Unknown".to_string());
        
        Self {
            name: name.to_string(),
            iterations,
            avg_compilation_time: total_compilation / iterations as u32,
            avg_execution_time: total_execution / iterations as u32,
            avg_total_time: total_time / iterations as u32,
            min_total_time,
            max_total_time,
            strategy_used,
        }
    }
}

impl std::fmt::Display for BenchmarkStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== {} Statistics ({} iterations) ===", self.name, self.iterations)?;
        writeln!(f, "Strategy: {}", self.strategy_used)?;
        writeln!(f, "Average compilation time: {:?}", self.avg_compilation_time)?;
        writeln!(f, "Average execution time: {:?}", self.avg_execution_time)?;
        writeln!(f, "Average total time: {:?}", self.avg_total_time)?;
        writeln!(f, "Min total time: {:?}", self.min_total_time)?;
        writeln!(f, "Max total time: {:?}", self.max_total_time)?;
        Ok(())
    }
}
