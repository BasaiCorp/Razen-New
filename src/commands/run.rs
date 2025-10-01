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
pub fn execute(file: PathBuf, opt_level: u8) -> Result<(), Box<dyn std::error::Error>> {
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
            let sources = vec![("source".to_string(), source.clone())];
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
                match jit.compile_and_run(&compiler.ir) {
                    Ok(_) => {
                        let duration = start_time.elapsed();
                        
                        // Show execution time with optimization info
                        let opt_name = match opt_level {
                            0 => "none",
                            1 => "basic",
                            2 => "standard",
                            3 => "aggressive",
                            4 => "maximum",
                            _ => "unknown",
                        };
                        
                        eprintln!("\nRAJIT execution completed in {:.3}s (optimization: {})", 
                                 duration.as_secs_f64(), opt_name);
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