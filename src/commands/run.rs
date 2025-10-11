//! Run command implementation - RAIE (Razen Adaptive Interpreter Engine) and RAZE

use std::path::PathBuf;
use std::fs;
use std::time::Instant;
use crate::frontend::parser::{parse_source_with_name, format_parse_errors};
use crate::backend::execution::Compiler;
use crate::backend::{SemanticAnalyzer, AdaptiveEngine};
use crate::backend::execution::raze::JITCompiler;
use crate::frontend::diagnostics::display::render_diagnostics;
use super::{validate_file_exists, validate_razen_file, handle_error};

/// Execute the run command - compile and run a Razen program with RAIE or RAZE
pub fn execute(file: PathBuf, optimize: bool, raze: bool, raze_mode: String, raze_opt: u8) -> Result<(), Box<dyn std::error::Error>> {
    // Level 0 (no optimization) by default, Level 2 (full optimization) with -O flag
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
        
        // Choose execution engine: RAZE or RAIE
        let start_time = Instant::now();
        
        if raze {
            // Use RAZE (Razen Advanced Zero-overhead Engine)
            println!("[INFO] Using RAZE JIT compiler");
            println!("[INFO] Mode: {}", raze_mode);
            println!("[INFO] Optimization: O{}", raze_opt);
            
            match JITCompiler::with_optimization(raze_opt) {
                Ok(mut jit) => {
                    match jit.compile_and_run(&compiler.ir) {
                        Ok(result) => {
                            let duration = start_time.elapsed();
                            let time_secs = duration.as_secs_f64();
                            
                            // Color based on execution time
                            let (color_code, time_str) = if time_secs < 0.1 {
                                ("\x1b[32m", format!("{:.3}ms", time_secs * 1000.0)) // Green: < 100ms (blazing fast!)
                            } else if time_secs < 1.0 {
                                ("\x1b[32m", format!("{:.3}ms", time_secs * 1000.0)) // Green: < 1s (fast!)
                            } else if time_secs < 3.0 {
                                ("\x1b[33m", format!("{:.3}s", time_secs)) // Yellow: 1-3s (good)
                            } else {
                                ("\x1b[31m", format!("{:.3}s", time_secs)) // Red: > 3s (slow)
                            };
                            
                            eprintln!("\n[SUCCESS] RAZE execution completed in {}{}\x1b[0m", color_code, time_str);
                            
                            // Show statistics
                            let stats = jit.stats();
                            eprintln!("{}", stats);
                        }
                        Err(e) => {
                            handle_error(&format!("RAZE execution failed: {}", e));
                        }
                    }
                }
                Err(e) => {
                    handle_error(&format!("Failed to initialize RAZE: {}", e));
                }
            }
        } else {
            // Use RAIE (Razen Adaptive Interpreter Engine) with specified optimization level
            match AdaptiveEngine::with_optimization(opt_level) {
            Ok(mut raie) => {
                raie.set_clean_output(true); // Clean output for run command
                
                // Register function parameter names
                for (func_name, params) in &compiler.function_param_names {
                    raie.register_function_params(func_name.clone(), params.clone());
                }
                
                match raie.compile_and_run(&compiler.ir) {
                    Ok(_) => {
                        let duration = start_time.elapsed();
                        let time_secs = duration.as_secs_f64();
                        
                        // Show execution time with optimization info
                        let tier_name = match opt_level {
                            0 => "Tier 0 (baseline)",
                            1 => "Tier 1 (adaptive)",
                            2 => "Tier 2 (optimized)",
                            _ => "unknown",
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
                        
                        eprintln!("\n[SUCCESS] RAIE execution completed in {}{}\x1b[0m ({})", 
                                 color_code, time_str, tier_name);
                    }
                    Err(e) => {
                        handle_error(&format!("RAIE execution failed: {}", e));
                    }
                }
            }
            Err(e) => {
                handle_error(&format!("Failed to initialize RAIE: {}", e));
            }
            }
        }
    } else {
        handle_error("Failed to parse the source file");
    }
    
    Ok(())
}