//! Dev command implementation - development mode with detailed output

use std::path::PathBuf;
use std::fs;
use crate::frontend::parser::{parse_source_with_name, format_parse_errors};
use crate::backend::execution::Compiler;
use crate::backend::SemanticAnalyzer;
use crate::frontend::diagnostics::display::render_diagnostics;
use super::{validate_file_exists, validate_razen_file, handle_error, success_message, info_message};

/// Execute the dev command - development mode with detailed compiler output
pub fn execute(file: PathBuf, watch: bool) -> Result<(), Box<dyn std::error::Error>> {
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
    println!("📁 File: {}", file.display());
    println!();
    
    // Read source file
    let source = match fs::read_to_string(&file) {
        Ok(content) => content,
        Err(e) => {
            handle_error(&format!("Failed to read file '{}': {}", file.display(), e));
        }
    };
    
    info_message(&format!("Source file loaded ({} bytes)", source.len()));
    
    // Parse the source code
    let filename = file.to_string_lossy().to_string();
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
        
        // Compile and execute
        println!("\n🔍 Phase 3: Compilation...");
        let mut compiler = Compiler::new();
        compiler.set_clean_output(false); // Always show compiler messages in dev mode
        compiler.compile_program(program);
        
        if !compiler.errors.is_empty() {
            handle_error(&format!("Compilation failed: {}", compiler.errors.join("; ")));
        }
        
        success_message("Compilation completed successfully!");
        
        println!("\n🚀 Phase 4: Execution...");
        println!("--- Program Output ---");
        
        match compiler.execute() {
            Ok(_) => {
                println!("--- End Output ---");
                success_message("Program executed successfully!");
                
                println!("\n📊 Development Summary:");
                println!("  ✓ Parsing: OK");
                println!("  ✓ Semantic Analysis: OK");
                println!("  ✓ Compilation: OK");
                println!("  ✓ Execution: OK");
            }
            Err(e) => {
                println!("--- End Output ---");
                handle_error(&format!("Execution failed: {}", e));
            }
        }
    } else {
        handle_error("Failed to parse the source file");
    }
    
    Ok(())
}
