mod token;
mod ast;
mod lexer;
mod parser;
mod compiler;
mod syntax;
mod value;
mod functions;
mod library;
mod llvm;

use std::env;
use std::path::Path;
use std::process;
use std::fs;
use std::io;
use std::time::Instant;

use crate::llvm::LlvmCompiler;
use crate::value::Value as RazenValue; // Assuming RazenValue is needed for return type
use inkwell::context::Context;

fn print_usage() {
    println!("Usage: razen <command> [args]\n");
    println!("Commands:");
    println!("  compile <file>     Compile a Razen source file to machine code");
    println!("  run <file>         Compile and execute a Razen source file");
    println!("  test [dir|file]    Run tests in the specified directory or file");
    println!("  help               Display this help message");
    println!("\nOptions:");
    println!("  --debug            Enable debug mode with additional output");
    println!("  --clean-output     Only show program output (no IR or debug info)");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Initialize the library system
    library::initialize();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    // Check for debug flag
    let debug_mode = args.iter().any(|arg| arg == "--debug");
    
    // Check for clean output flag (used by razen-run to only show program output)
    let clean_output = args.iter().any(|arg| arg == "--clean-output");
    
    // Filter out the special flags from arguments
    let filtered_args: Vec<String> = args.iter()
        .filter(|&arg| arg != "--debug" && arg != "--clean-output")
        .cloned()
        .collect();
    
    match filtered_args[1].as_str() {
        "compile" => {
            if filtered_args.len() < 3 {
                println!("Error: Missing source file path");
                process::exit(1);
            }
            
            let source_path_str = &filtered_args[2];
            let output_path_str = if filtered_args.len() > 3 {
                filtered_args[3].clone() // Clone to own the String for output_path_str
            } else {
                let source_path_obj = Path::new(source_path_str);
                let stem = source_path_obj.file_stem().unwrap_or_default().to_str().unwrap_or("output");
                format!("{}.o", stem) // Default to .o for object file
            };
            
            println!("Compiling {} to LLVM IR and then to {}", source_path_str, output_path_str);
            
            // 1. Compile Razen source to Razen IR
            match compiler::Compiler::from_file(source_path_str) {
                Ok(razen_compiler) => {
                    let razen_ir_sequence = razen_compiler.ir;

                    if debug_mode {
                        println!("Successfully parsed Razen source. Number of Razen IR instructions: {}", razen_ir_sequence.len());
                    }

                    // 2. Initialize LLVM Context and our LlvmCompiler
                    let context = Context::create();
                    let module_name = Path::new(source_path_str).file_stem().unwrap_or_default().to_str().unwrap_or("razen_module");
                    let mut llvm_compiler = LlvmCompiler::new(&context, module_name, !debug_mode); // Enable optimizations if not in debug mode

                    // 3. Compile Razen IR to LLVM IR (into a 'main' function)
                    match llvm_compiler.compile_function("main", vec![], RazenValue::Int(0), &razen_ir_sequence) {
                        Ok(_main_function) => {
                            if debug_mode {
                                println!("Successfully generated LLVM IR for 'main' function.");
                                llvm_compiler.dump_module(); // Print LLVM IR to stderr
                            }

                            // 4. Emit LLVM IR to a .ll file (placeholder for object file emission)
                            let ll_path = format!("{}.ll", Path::new(&output_path_str).file_stem().unwrap_or_default().to_str().unwrap_or("output"));
                            match llvm_compiler.module.print_to_file(&Path::new(&ll_path)) {
                                Ok(_) => {
                                    println!("LLVM IR written to {}", ll_path);
                                    println!("Compilation (to LLVM IR) successful!");
                                    println!("Next steps: Implement object file emission in llvm.rs and linking.");
                                }
                                Err(e) => {
                                    println!("Error writing LLVM IR to file: {:?}", e.to_string());
                                    process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            println!("LLVM Compilation Error: {}", e);
                            process::exit(1);
                        }
                    }
                },
                Err(e) => {
                    println!("Razen Compilation error: {}", e);
                    process::exit(1);
                }
            }
        },
        "run" => {
            if filtered_args.len() < 3 {
                println!("Error: Missing source file path");
                process::exit(1);
            }
            
            let source_path = &filtered_args[2];
            
            if !clean_output {
                println!("Running {}", source_path);
                
                if debug_mode {
                    println!("Debug mode enabled");
                }
            }
            
            match compiler::Compiler::from_file(source_path) {
                Ok(compiler) => {
                    match compiler.execute() {
                        Ok(_) => {
                            if !clean_output {
                                println!("Execution completed successfully!");
                            }
                        },
                        Err(e) => {
                            println!("Execution error: {}", e);
                            process::exit(1);
                        }
                    }
                },
                Err(e) => {
                    println!("Compilation error: {}", e);
                    process::exit(1);
                }
            }
        },
        "test" => {
            println!("Running tests");
            
            if debug_mode {
                println!("Debug mode enabled");
            }
            
            let test_path = if filtered_args.len() > 2 {
                &filtered_args[2]
            } else {
                // Default to razen_tests directory
                "razen_tests"
            };
            
            println!("Test path: {}", test_path);
            
            // Check if path is a directory or file
            let path = Path::new(test_path);
            if path.is_dir() {
                // Run all tests in directory
                println!("Running all tests in directory: {}", test_path);
                run_tests_in_directory(path, debug_mode);
            } else if path.is_file() {
                // Run single test file
                println!("Running test file: {}", test_path);
                run_test_file(path, debug_mode);
            } else {
                println!("Error: Test path '{}' does not exist", test_path);
                process::exit(1);
            }
        },
        "help" | "-h" | "--help" => {
            print_usage();
        },
        _ => {
            println!("Unknown command: {}", filtered_args[1]);
            print_usage();
            process::exit(1);
        }
    }
}

// Run all tests in a directory
fn run_tests_in_directory(dir_path: &Path, debug_mode: bool) -> io::Result<()> {
    let mut passed = 0;
    let mut failed = 0;
    let mut total_time = 0.0;
    
    // Collect all .rzn files in the directory
    let entries = fs::read_dir(dir_path)?;
    let mut test_files = Vec::new();
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rzn") {
            test_files.push(path);
        } else if path.is_dir() {
            // Recursively run tests in subdirectories
            if debug_mode {
                println!("Entering directory: {}", path.display());
            }
            run_tests_in_directory(&path, debug_mode)?;
        }
    }
    
    // Sort test files for consistent output
    test_files.sort();
    
    println!("Found {} test files in {}", test_files.len(), dir_path.display());
    
    for test_file in test_files {
        let (success, duration) = run_test_file(&test_file, debug_mode);
        
        if success {
            passed += 1;
        } else {
            failed += 1;
        }
        
        total_time += duration;
    }
    
    println!("\nTest Summary for {}:", dir_path.display());
    println!("  Passed: {}", passed);
    println!("  Failed: {}", failed);
    println!("  Total: {}", passed + failed);
    println!("  Time: {:.2}s", total_time);
    
    Ok(())
}

// Run a single test file
fn run_test_file(file_path: &Path, debug_mode: bool) -> (bool, f64) {
    let start = Instant::now();
    let file_name = file_path.file_name().unwrap_or_default().to_string_lossy();
    
    print!("Testing {}... ", file_name);
    
    if debug_mode {
        println!();
    }
    
    match compiler::Compiler::from_file(file_path.to_str().unwrap()) {
        Ok(compiler) => {
            match compiler.execute() {
                Ok(_) => {
                    let duration = start.elapsed().as_secs_f64();
                    if !debug_mode {
                        println!("PASS ({:.2}s)", duration);
                    } else {
                        println!("\nPASS ({:.2}s)", duration);
                    }
                    (true, duration)
                },
                Err(e) => {
                    let duration = start.elapsed().as_secs_f64();
                    if !debug_mode {
                        println!("FAIL ({:.2}s)", duration);
                    } else {
                        println!("\nFAIL ({:.2}s): {}", duration, e);
                    }
                    (false, duration)
                }
            }
        },
        Err(e) => {
            let duration = start.elapsed().as_secs_f64();
            if !debug_mode {
                println!("FAIL ({:.2}s)", duration);
            } else {
                println!("\nFAIL ({:.2}s): {}", duration, e);
            }
            (false, duration)
        }
    }
}
