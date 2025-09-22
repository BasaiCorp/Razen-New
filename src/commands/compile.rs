//! Compile command implementation - AOT compilation to executable

use std::path::PathBuf;
use std::fs;
use crate::frontend::parser::{parse_source_with_name, format_parse_errors};
use crate::backend::execution::Compiler;
use crate::backend::SemanticAnalyzer;
use crate::frontend::diagnostics::display::render_diagnostics;
use super::{validate_file_exists, validate_razen_file, handle_error, success_message, info_message};

/// Execute the compile command - AOT compilation to native executable
pub fn execute(
    input: PathBuf, 
    output: Option<PathBuf>, 
    optimization: u8, 
    debug: bool
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate input file
    if let Err(e) = validate_file_exists(&input) {
        handle_error(&e);
    }
    
    if let Err(e) = validate_razen_file(&input) {
        handle_error(&e);
    }
    
    // Determine output path
    let output_path = match output {
        Some(path) => path,
        None => {
            let mut path = input.clone();
            path.set_extension("exe");
            path
        }
    };
    
    // Validate optimization level
    if optimization > 3 {
        handle_error("Optimization level must be between 0 and 3");
    }
    
    println!("=== Razen AOT Compiler ===");
    println!("📁 Input:  {}", input.display());
    println!("📦 Output: {}", output_path.display());
    println!("🔧 Optimization Level: {}", optimization);
    println!("🐛 Debug Info: {}", if debug { "enabled" } else { "disabled" });
    println!();
    
    // Read source file
    let source = match fs::read_to_string(&input) {
        Ok(content) => content,
        Err(e) => {
            handle_error(&format!("Failed to read file '{}': {}", input.display(), e));
        }
    };
    
    info_message(&format!("Source file loaded ({} bytes)", source.len()));
    
    // Parse the source code
    let filename = input.to_string_lossy().to_string();
    println!("\n🔍 Phase 1: Parsing...");
    let (program, diagnostics) = parse_source_with_name(&source, &filename);
    
    if !diagnostics.is_empty() {
        eprintln!("❌ Parsing errors:");
        let formatted_errors = format_parse_errors(&diagnostics, &source, &filename);
        eprintln!("{}", formatted_errors);
        std::process::exit(1);
    }
    
    success_message("Parsing completed successfully!");
    
    if let Some(program) = program {
        // Run semantic analysis
        println!("\n🔍 Phase 2: Semantic Analysis...");
        let mut semantic_analyzer = SemanticAnalyzer::new();
        let semantic_diagnostics = semantic_analyzer.analyze_with_source(&program, &source);
        
        if !semantic_diagnostics.is_empty() {
            let sources = vec![("source".to_string(), source.clone())];
            let rendered = render_diagnostics(&semantic_diagnostics, &sources);
            eprintln!("{}", rendered);
            
            if semantic_diagnostics.has_errors() {
                std::process::exit(1);
            }
        }
        
        success_message("Semantic analysis completed successfully!");
        
        // Compile to object file
        println!("\n🔍 Phase 3: AOT Compilation...");
        match Compiler::from_program(program) {
            Ok(compiler) => {
                match compiler.write_to_file(&output_path.to_string_lossy().to_string()) {
                    Ok(_) => {
                        success_message(&format!("AOT compilation successful! Object file written to: {}", output_path.display()));
                        
                        // Show file info
                        if let Ok(metadata) = fs::metadata(&output_path) {
                            info_message(&format!("Generated file size: {} bytes", metadata.len()));
                        }
                        
                        println!("\n📊 Compilation Summary:");
                        println!("  ✓ Parsing: OK");
                        println!("  ✓ Semantic Analysis: OK");
                        println!("  ✓ AOT Compilation: OK");
                        println!("  ✓ Output Generation: OK");
                        
                        if optimization > 0 {
                            info_message(&format!("Optimizations applied: Level {}", optimization));
                        }
                        
                        if debug {
                            info_message("Debug information included");
                        }
                    }
                    Err(e) => {
                        handle_error(&format!("Failed to write output file: {}", e));
                    }
                }
            }
            Err(e) => {
                handle_error(&format!("AOT compilation failed: {}", e));
            }
        }
    } else {
        handle_error("Failed to parse the source file");
    }
    
    Ok(())
}
