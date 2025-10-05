//! Run command implementation - RAJIT compile and execute

use std::path::PathBuf;
use std::fs;
use std::time::Instant;
use crate::frontend::parser::{parse_source_with_name, format_parse_errors};
use crate::backend::execution::Compiler;
use crate::backend::{SemanticAnalyzer, NativeJIT};
use crate::frontend::diagnostics::display::render_diagnostics;
use super::{validate_file_exists, validate_razen_file, handle_error};

/// Execute the run command - compile and run a Razen program with RAJIT
pub fn execute(file: PathBuf, optimize: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Level 0 (no optimization) by default, Level 2 (standard) with -O flag
    let opt_level = if optimize { 2 } else { 0 };
    // Validate input file
    if let Err(e) = validate_file_exists(&file) {
        handle_error(&e);
    }
    
    if let Err(e) = validate_razen_file(&file) {
        handle_error(&e);
    }
    
    
    // Read source file
    let source = match fs::read_to_string(&file) {
        Ok(content) => content,
        Err(e) => {
            handle_error(&format!("Failed to read file '{}': {}", file.display(), e));
        }
    };
    
    // Parse the source code with full file path context
    let filename = file.canonicalize().unwrap_or(file.clone()).to_string_lossy().to_string();
    let (program, diagnostics) = parse_source_with_name(&source, &filename);
    
    if !diagnostics.is_empty() {
        eprintln!("Parsing errors:");
        let formatted_errors = format_parse_errors(&diagnostics, &source, &filename);
        eprintln!("{}", formatted_errors);
        std::process::exit(1);
    }
    
    if let Some(program) = program {
        // Run semantic analysis with module support
        let base_dir = file.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();
        let mut semantic_analyzer = SemanticAnalyzer::with_module_support(base_dir, file.clone());
        let semantic_diagnostics = semantic_analyzer.analyze_with_source(&program, &source);
        
        if !semantic_diagnostics.is_empty() {
            let sources = vec![(filename.clone(), source.clone())];
            let rendered = render_diagnostics(&semantic_diagnostics, &sources);
            eprintln!("{}", rendered);
            
            if semantic_diagnostics.has_errors() {
                std::process::exit(1);
            }
        }
        
        // Compile to IR
        let mut compiler = Compiler::new();
        compiler.set_clean_output(true); // Clean output for run command
        compiler.set_current_file(file.clone());
        compiler.compile_program(program);
        
        if !compiler.errors.is_empty() {
            handle_error(&format!("Compilation failed: {}", compiler.errors.join("; ")));
        }
        
        // Use RAJIT (Razen Adaptive JIT) with specified optimization level
        let start_time = Instant::now();
        
        match NativeJIT::with_optimization(opt_level) {
            Ok(mut jit) => {
                jit.set_clean_output(true); // Clean output for run command
                
                // Register function parameter names
                for (func_name, params) in &compiler.function_param_names {
                    jit.register_function_params(func_name.clone(), params.clone());
                }
                
                match jit.compile_and_run(&compiler.ir) {
                    Ok(_) => {
                        let duration = start_time.elapsed();
                        let time_secs = duration.as_secs_f64();
                        
                        // Show execution time with optimization info
                        let opt_name = if opt_level == 0 {
                            "none"
                        } else {
                            "standard"
                        };
                        
                        // Color based on execution time
                        let (color_code, time_str) = if time_secs < 3.0 {
                            ("\x1b[32m", format!("{:.3}s", time_secs)) // Green: < 3s (fast!)
                        } else if time_secs < 10.0 {
                            ("\x1b[33m", format!("{:.3}s", time_secs)) // Yellow: 3-10s (good)
                        } else if time_secs < 20.0 {
                            ("\x1b[38;5;208m", format!("{:.3}s", time_secs)) // Orange: 10-20s (okay)
                        } else {
                            ("\x1b[31m", format!("{:.3}s", time_secs)) // Red: > 20s (slow)
                        };
                        
                        eprintln!("\nRAJIT execution completed in {}{}\x1b[0m (optimization: {})", 
                                 color_code, time_str, opt_name);
                    }
                    Err(e) => {
                        handle_error(&format!("RAJIT execution failed: {}", e));
                    }
                }
            }
            Err(e) => {
                handle_error(&format!("Failed to initialize RAJIT: {}", e));
            }
        }
    } else {
        handle_error("Failed to parse the source file");
    }
    
    Ok(())
}