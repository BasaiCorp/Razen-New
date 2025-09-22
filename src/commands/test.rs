//! Test command implementation - run test files and report results

use std::path::PathBuf;
use std::fs;
use crate::frontend::parser::{parse_source_with_name, format_parse_errors};
use crate::backend::execution::Compiler;
use crate::backend::SemanticAnalyzer;
use crate::frontend::diagnostics::display::render_diagnostics;
use super::{validate_file_exists, handle_error, success_message, info_message};

/// Execute the test command - run test files and report results
pub fn execute(
    path: PathBuf, 
    verbose: bool, 
    filter: Option<String>
) -> Result<(), Box<dyn std::error::Error>> {
    
    if path.is_file() {
        // Single test file
        run_single_test(&path, verbose, &filter)
    } else if path.is_dir() {
        // Test directory
        run_test_directory(&path, verbose, &filter)
    } else {
        handle_error(&format!("Path does not exist: {}", path.display()));
    }
}

/// Run a single test file
fn run_single_test(
    file: &PathBuf, 
    verbose: bool, 
    filter: &Option<String>
) -> Result<(), Box<dyn std::error::Error>> {
    
    // Check if file matches filter
    if let Some(filter_str) = filter {
        let filename = file.file_name().unwrap().to_string_lossy();
        if !filename.contains(filter_str) {
            if verbose {
                info_message(&format!("Skipping {} (doesn't match filter)", filename));
            }
            return Ok(());
        }
    }
    
    if verbose {
        println!("=== Running Test: {} ===", file.display());
    }
    
    // Validate file exists
    if let Err(e) = validate_file_exists(file) {
        handle_error(&e);
    }
    
    // Read source file
    let source = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            handle_error(&format!("Failed to read test file '{}': {}", file.display(), e));
        }
    };
    
    // Parse the source code
    let filename = file.to_string_lossy().to_string();
    let (program, diagnostics) = parse_source_with_name(&source, &filename);
    
    if !diagnostics.is_empty() {
        if verbose {
            eprintln!("❌ Parsing errors in {}:", file.display());
            let formatted_errors = format_parse_errors(&diagnostics, &source, &filename);
            eprintln!("{}", formatted_errors);
        }
        println!("FAIL: {} (parsing errors)", file.file_name().unwrap().to_string_lossy());
        return Ok(());
    }
    
    if let Some(program) = program {
        // Run semantic analysis
        let mut semantic_analyzer = SemanticAnalyzer::new();
        let semantic_diagnostics = semantic_analyzer.analyze_with_source(&program, &source);
        
        if !semantic_diagnostics.is_empty() && semantic_diagnostics.has_errors() {
            if verbose {
                let sources = vec![("source".to_string(), source.clone())];
                let rendered = render_diagnostics(&semantic_diagnostics, &sources);
                eprintln!("{}", rendered);
            }
            println!("FAIL: {} (semantic errors)", file.file_name().unwrap().to_string_lossy());
            return Ok(());
        }
        
        // Compile and execute
        match Compiler::from_program(program) {
            Ok(compiler) => {
                let mut test_compiler = compiler;
                test_compiler.set_clean_output(true); // Clean output for tests
                
                match test_compiler.execute() {
                    Ok(_) => {
                        if verbose {
                            success_message(&format!("Test passed: {}", file.display()));
                        } else {
                            println!("PASS: {}", file.file_name().unwrap().to_string_lossy());
                        }
                    }
                    Err(e) => {
                        if verbose {
                            eprintln!("❌ Execution failed: {}", e);
                        }
                        println!("FAIL: {} (execution error)", file.file_name().unwrap().to_string_lossy());
                    }
                }
            }
            Err(e) => {
                if verbose {
                    eprintln!("❌ Compilation failed: {}", e);
                }
                println!("FAIL: {} (compilation error)", file.file_name().unwrap().to_string_lossy());
            }
        }
    } else {
        println!("FAIL: {} (failed to parse)", file.file_name().unwrap().to_string_lossy());
    }
    
    Ok(())
}

/// Run all test files in a directory
fn run_test_directory(
    dir: &PathBuf, 
    verbose: bool, 
    filter: &Option<String>
) -> Result<(), Box<dyn std::error::Error>> {
    
    println!("=== Running Tests in Directory: {} ===", dir.display());
    
    let mut test_files = Vec::new();
    let mut passed = 0;
    let mut failed = 0;
    
    // Find all .rzn and .razen files
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "rzn" || ext == "razen" {
                            test_files.push(path);
                        }
                    }
                }
            }
        }
    }
    
    if test_files.is_empty() {
        info_message("No test files found in directory");
        return Ok(());
    }
    
    test_files.sort();
    
    for test_file in &test_files {
        // Capture output to count pass/fail
        let result = std::panic::catch_unwind(|| {
            run_single_test(test_file, verbose, filter)
        });
        
        match result {
            Ok(_) => passed += 1,
            Err(_) => failed += 1,
        }
    }
    
    println!("\n=== Test Summary ===");
    println!("Total: {}", test_files.len());
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    
    if failed > 0 {
        println!("❌ Some tests failed");
        std::process::exit(1);
    } else {
        success_message("All tests passed!");
    }
    
    Ok(())
}
