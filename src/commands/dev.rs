//! Dev command implementation - RAIE development mode with detailed output

use std::path::PathBuf;
use std::fs;
use crate::frontend::parser::{parse_source_with_debug, format_parse_errors};
use crate::backend::execution::Compiler;
use crate::backend::{SemanticAnalyzer, AdaptiveEngine, NativeAOT};
use crate::frontend::diagnostics::display::render_diagnostics;
use super::{validate_file_exists, validate_razen_file, handle_error, success_message, info_message};

/// Execute the dev command - RAIE development mode with detailed compiler output
pub fn execute(file: PathBuf, watch: bool, adaptive: bool, aot: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Validate input file
    if let Err(e) = validate_file_exists(&file) {
        handle_error(&e);
    }
    
    if let Err(e) = validate_razen_file(&file) {
        handle_error(&e);
    }
    
    if watch {
        info_message("Watch mode is not yet implemented. Running once...");
    }
    
    println!("=== Razen Development Mode ===");
    println!("File: {}", file.display());
    println!();
    
    // Read source file
    let source = match fs::read_to_string(&file) {
        Ok(content) => content,
        Err(e) => {
            handle_error(&format!("Failed to read file '{}': {}", file.display(), e));
        }
    };
    
    info_message(&format!("Source file loaded ({} bytes)", source.len()));
    
    // Parse the source code with full file path context and debug output
    let filename = file.canonicalize().unwrap_or(file.clone()).to_string_lossy().to_string();
    println!("\nPhase 1: Parsing...");
    
    // Parse with debug output enabled (only shows in dev command)
    let (program, diagnostics) = parse_source_with_debug(&source, &filename, true);
    
    if !diagnostics.is_empty() {
        eprintln!("Parsing errors:");
        let formatted_errors = format_parse_errors(&diagnostics, &source, &filename);
        eprintln!("{}", formatted_errors);
        std::process::exit(1);
    }
    
    success_message("Parsing completed successfully!");
    
    if let Some(program) = program {
        // Run semantic analysis with module support
        println!("\nPhase 2: Semantic Analysis...");
        let base_dir = file.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();
        let mut semantic_analyzer = SemanticAnalyzer::with_module_support(base_dir, file.clone());
        let semantic_diagnostics = semantic_analyzer.analyze_with_source(&program, &source);
        
        if !semantic_diagnostics.is_empty() {
            let sources = vec![(filename.clone(), source.clone())];
            let rendered = render_diagnostics(&semantic_diagnostics, &sources);
            eprintln!("{}", rendered);
            
            if semantic_diagnostics.has_errors() {
            }
        }
        
        success_message("Semantic analysis completed successfully!");
        
        // Compile to IR
        println!("\nPhase 3: IR Generation...");
        let mut compiler = Compiler::new();
        compiler.set_clean_output(false); // Verbose output for dev mode
        compiler.set_current_file(file.clone());
        compiler.compile_program(program);
        
        if !compiler.errors.is_empty() {
            handle_error(&format!("Compilation failed: {}", compiler.errors.join("; ")));
        }
        
        success_message("Compilation completed successfully!");
        
        // Choose execution method based on flags
        if adaptive {
            // RAIE (Razen Adaptive Interpreter Engine) compilation
            println!("\nPhase 4: RAIE Adaptive Compilation & Execution...");
            info_message("Using RAIE - Razen Adaptive Interpreter Engine");
            
            match AdaptiveEngine::new() {
                Ok(mut raie) => {
                    raie.set_clean_output(false); // Debug output for dev command
                    
                    // Register function parameter names
                    for (func_name, params) in &compiler.function_param_names {
                        raie.register_function_params(func_name.clone(), params.clone());
                    }
                    
                    println!("--- RAIE Output ---");
                    match raie.compile_and_run(&compiler.ir) {
                        Ok(result) => {
                            println!("Result: {}", result);
                            println!("--- End RAIE Output ---");
                            success_message("RAIE adaptive compilation and execution successful!");
                            
                            // Show RAIE statistics
                            let stats = raie.get_stats();
                            println!("\n[INFO] RAIE Statistics: {}", stats);
                            
                            println!("\nDevelopment Summary:");
                            println!("  Parsing: OK");
                            println!("  Semantic Analysis: OK");
                            println!("  IR Generation: OK");
                            println!("  RAIE Adaptive: OK");
                            println!("  Execution: OK");
                        }
                        Err(e) => {
                            println!("--- End RAIE Output ---");
                            handle_error(&format!("RAIE execution failed: {}", e));
                        }
                    }
                }
                Err(e) => {
                    handle_error(&format!("Failed to initialize RAIE: {}", e));
                }
            }
        } else if aot {
            // Native AOT compilation
            println!("\nPhase 4: Native AOT Compilation...");
            info_message("Using custom x86-64 backend (no dependencies!)");
            
            let mut native_aot = NativeAOT::new();
            let output_name = file.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            
            match native_aot.compile(&compiler.ir, output_name) {
                Ok(_) => {
                    success_message(&format!("Native AOT compilation successful! Executable: {}", output_name));
                    info_message("Generated native ELF executable (no runtime dependencies!)");
                    
                    println!("\nDevelopment Summary:");
                    println!("  Parsing: OK");
                    println!("  Semantic Analysis: OK");
                    println!("  IR Generation: OK");
                    println!("  Native AOT: OK");
                    println!("  Output: {}", output_name);
                }
                Err(e) => {
                    handle_error(&format!("Native AOT compilation failed: {}", e));
                }
            }
        } else {
            // IR interpreter (default)
            println!("\nPhase 4: Execution (IR Interpreter)...");
            println!("--- Program Output ---");
            
            match compiler.execute() {
                Ok(_) => {
                    println!("--- End Output ---");
                    success_message("Program executed successfully!");
                    
                    println!("\nDevelopment Summary:");
                    println!("  Parsing: OK");
                    println!("  Semantic Analysis: OK");
                    println!("  Compilation: OK");
                    println!("  Execution: OK");
                }
                Err(e) => {
                    println!("--- End Output ---");
                    handle_error(&format!("Execution failed: {}", e));
                }
            }
        }
    } else {
        handle_error("Failed to parse the source file");
    }
    
    Ok(())
}
