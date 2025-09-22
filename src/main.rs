// src/main.rs

use razen_lang::frontend::parser::{parse_source_with_name, format_parse_errors};
use razen_lang::backend::execution::Compiler;
use std::fs;
use std::env;
use std::process;

/// Execution mode for the Razen compiler
#[derive(Debug, Clone, PartialEq)]
enum ExecutionMode {
    /// Clean execution (like go run)
    Run,
    /// Development mode with compiler messages
    Dev,
    /// Compile to machine code
    Compile(String), // Output path
    /// Test mode
    Test,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let (execution_mode, filename) = parse_args(&args);

    let source = match fs::read_to_string(&filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    let (program, diagnostics) = parse_source_with_name(&source, &filename);

    if !diagnostics.is_empty() {
        eprintln!("Parsing errors:");
        let formatted_errors = format_parse_errors(&diagnostics, &source, &filename);
        eprintln!("{}", formatted_errors);
        process::exit(1);
    }

    if let Some(program) = program {
        match execution_mode {
            ExecutionMode::Run => {
                execute_program(program, true); // Clean output by default
            }
            ExecutionMode::Dev => {
                execute_program(program, false); // Show compiler messages
            }
            ExecutionMode::Compile(output_path) => {
                compile_program(program, &output_path);
            }
            ExecutionMode::Test => {
                test_program(program);
            }
        }
    }
}

/// Parse command line arguments and determine execution mode
fn parse_args(args: &[String]) -> (ExecutionMode, String) {
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    match args[1].as_str() {
        "run" => {
            if args.len() < 3 {
                eprintln!("Error: Missing source file path");
                process::exit(1);
            }
            (ExecutionMode::Run, args[2].clone())
        }
        "dev" => {
            if args.len() < 3 {
                eprintln!("Error: Missing source file path");
                process::exit(1);
            }
            (ExecutionMode::Dev, args[2].clone())
        }
        "compile" => {
            if args.len() < 4 {
                eprintln!("Error: Missing source file or output path");
                process::exit(1);
            }
            (ExecutionMode::Compile(args[3].clone()), args[2].clone())
        }
        "test" => {
            if args.len() < 3 {
                eprintln!("Error: Missing test file path");
                process::exit(1);
            }
            (ExecutionMode::Test, args[2].clone())
        }
        "help" | "-h" | "--help" => {
            print_usage();
            process::exit(0);
        }
        filename => {
            // Default to run mode if just filename provided
            (ExecutionMode::Run, filename.to_string())
        }
    }
}

fn print_usage() {
    println!("Usage: razen <command> [args]\\n");
    println!("Commands:");
    println!("  run <file>             Compile and execute a Razen source file (clean output)");
    println!("  dev <file>             Development mode with compiler messages and debugging");
    println!("  compile <file> <out>   Compile a Razen source file to machine code");
    println!("  test <file>            Run a test file");
    println!("  help                   Display this help message");
}

/// Execute a Razen program
fn execute_program(program: razen_lang::frontend::parser::ast::Program, clean_output: bool) {
    // Create compiler with proper clean_output setting from the start
    let mut compiler = Compiler::new();
    compiler.set_clean_output(clean_output);
    compiler.compile_program(program);
    
    if !compiler.errors.is_empty() {
        eprintln!("Compilation error: {}", compiler.errors.join("; "));
        process::exit(1);
    }
    
    match compiler.execute() {
        Ok(_) => {
            // For dev mode, show completion message
            if !clean_output {
                println!("Execution completed successfully!");
            }
            // For run mode, silent success (like go run)
        }
        Err(e) => {
            eprintln!("Execution error: {}", e);
            process::exit(1);
        }
    }
}

/// Compile a Razen program to machine code
fn compile_program(program: razen_lang::frontend::parser::ast::Program, output_path: &str) {
    match Compiler::from_program(program) {
        Ok(compiler) => {
            match compiler.write_to_file(output_path) {
                Ok(_) => {
                    println!("Compilation successful! Object file written to: {}", output_path);
                }
                Err(e) => {
                    eprintln!("Compilation failed: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            process::exit(1);
        }
    }
}

/// Test a Razen program
fn test_program(program: razen_lang::frontend::parser::ast::Program) {
    println!("Running test...");
    
    match Compiler::from_program(program) {
        Ok(compiler) => {
            let mut test_compiler = compiler;
            test_compiler.set_clean_output(true);
            
            match test_compiler.execute() {
                Ok(_) => {
                    println!("PASS");
                }
                Err(e) => {
                    println!("FAIL: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            println!("FAIL: {}", e);
            process::exit(1);
        }
    }
}