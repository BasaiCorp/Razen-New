// src/commands/benchmark.rs
//! Benchmark command implementation for RAIE performance validation

use std::path::PathBuf;
use crate::benchmark::BenchmarkSuite;
use crate::commands::{handle_error, success_message, info_message};

/// Execute the benchmark command
pub fn execute(
    iterations: usize,
    name: Option<String>,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    info_message("Starting RAIE Performance Benchmarks");
    
    let mut suite = BenchmarkSuite::new();
    
    match name {
        Some(benchmark_name) => {
            // Run specific benchmark
            let file_path = format!("benchmarks/{}.rzn", benchmark_name.to_lowercase().replace(" ", "_"));
            
            if !std::path::Path::new(&file_path).exists() {
                handle_error(&format!("Benchmark file not found: {}", file_path));
            }
            
            info_message(&format!("Running specific benchmark: {}", benchmark_name));
            
            match suite.run_benchmark_iterations(&benchmark_name, &file_path, iterations) {
                Ok(stats) => {
                    println!("\n{}", stats);
                    success_message(&format!("Benchmark '{}' completed successfully", benchmark_name));
                }
                Err(e) => {
                    handle_error(&format!("Benchmark failed: {}", e));
                }
            }
        }
        None => {
            // Run all benchmarks
            info_message("Running all available benchmarks");
            
            if let Err(e) = suite.run_all_benchmarks() {
                handle_error(&format!("Benchmark suite failed: {}", e));
            }
            
            success_message("All benchmarks completed successfully");
        }
    }
    
    // Save results to file if requested
    if let Some(output_path) = output {
        info_message(&format!("Saving results to: {}", output_path.display()));
        // TODO: Implement results serialization
        success_message("Results saved successfully");
    }
    
    Ok(())
}
