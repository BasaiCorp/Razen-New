// src/main.rs

use razen_lang::frontend::parser::{parse_source_with_name, format_parse_errors};
use razen_lang::backend::Backend;
use razen_lang::backend::builtins::BuiltinRegistry;
use razen_lang::backend::cranelift::{JITCompiler, AOTCompiler};
use std::fs;
use std::env;
use std::process;

/// Execution mode for the Razen compiler
#[derive(Debug, Clone, PartialEq)]
enum ExecutionMode {
    /// Standard compilation pipeline (semantic analysis + IR generation + code generation)
    Standard,
    /// JIT compilation and immediate execution
    JIT,
    /// AOT compilation to object file
    AOT(String), // Output path
}

fn main() {
    // Initialize builtin functions registry
    BuiltinRegistry::initialize();
    
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

    // Only show parsing info in standard mode
    if execution_mode == ExecutionMode::Standard {
        println!("🔍 Parsing Razen file: {}\n", filename);
    }

    let (program, diagnostics) = parse_source_with_name(&source, &filename);

    if diagnostics.is_empty() {
        if execution_mode == ExecutionMode::Standard {
            println!("✅ Parsing completed successfully!");
        }
        
        if let Some(ref program) = program {
            // Show program statistics only in standard mode
            if execution_mode == ExecutionMode::Standard {
                println!("📊 Program statistics:");
                println!("   - Statements: {}", program.statements.len());
                
                // Count different types of statements
                let mut var_count = 0;
                let mut func_count = 0;
                let mut struct_count = 0;
                let mut enum_count = 0;
                
                for stmt in &program.statements {
                    match stmt {
                        razen_lang::frontend::parser::ast::Statement::VariableDeclaration(_) |
                        razen_lang::frontend::parser::ast::Statement::ConstantDeclaration(_) => var_count += 1,
                        razen_lang::frontend::parser::ast::Statement::FunctionDeclaration(_) => func_count += 1,
                        razen_lang::frontend::parser::ast::Statement::StructDeclaration(_) => struct_count += 1,
                        razen_lang::frontend::parser::ast::Statement::EnumDeclaration(_) => enum_count += 1,
                        _ => {}
                    }
                }
                
                println!("   - Variables/Constants: {}", var_count);
                println!("   - Functions: {}", func_count);
                println!("   - Structs: {}", struct_count);
                println!("   - Enums: {}", enum_count);
            }
            
            // Execute based on mode
            match execution_mode {
                ExecutionMode::JIT => {
                    // JIT compilation and execution
                    execute_jit(program.clone());
                }
                ExecutionMode::AOT(output_path) => {
                    // AOT compilation to object file
                    execute_aot(program.clone(), &output_path);
                }
                ExecutionMode::Standard => {
                    // Standard compilation pipeline
                    execute_standard_pipeline(program.clone());
                }
            }
        }
    } else {
        // Show parsing errors (always show these regardless of mode)
        eprintln!("❌ Parsing completed with {} error(s) and {} warning(s):\n", 
                 diagnostics.error_count(), 
                 diagnostics.warning_count());
        
        // Display beautiful error messages
        let formatted_errors = format_parse_errors(&diagnostics, &source, &filename);
        eprintln!("{}", formatted_errors);
        process::exit(1);
    }

    // Uncomment to see the full AST
    if env::var("RAZEN_DEBUG_AST").is_ok() {
        println!("\n🔧 Debug: Full AST");
        println!("{:#?}", program);
    }
}

/// Parse command line arguments and determine execution mode
fn parse_args(args: &[String]) -> (ExecutionMode, String) {
    if args.len() < 2 {
        return (ExecutionMode::Standard, "src/tests/syntax.rzn".to_string());
    }
    
    let mut mode = ExecutionMode::Standard;
    let mut filename = String::new();
    let mut i = 1;
    
    while i < args.len() {
        match args[i].as_str() {
            "--jit" => {
                mode = ExecutionMode::JIT;
                i += 1;
                if i < args.len() {
                    filename = args[i].clone();
                } else {
                    eprintln!("Error: --jit requires a filename");
                    process::exit(1);
                }
            }
            "--aot" => {
                i += 1;
                if i + 1 < args.len() {
                    filename = args[i].clone();
                    let output_path = args[i + 1].clone();
                    mode = ExecutionMode::AOT(output_path);
                    i += 1;
                } else {
                    eprintln!("Error: --aot requires input filename and output path");
                    process::exit(1);
                }
            }
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            _ => {
                if filename.is_empty() {
                    filename = args[i].clone();
                } else {
                    eprintln!("Error: Unknown argument '{}'", args[i]);
                    print_help();
                    process::exit(1);
                }
            }
        }
        i += 1;
    }
    
    if filename.is_empty() {
        filename = "src/tests/syntax.rzn".to_string();
    }
    
    (mode, filename)
}

/// Print help message
fn print_help() {
    println!("Razen Language Compiler");
    println!();
    println!("USAGE:");
    println!("    cargo run [OPTIONS] [FILE]");
    println!();
    println!("OPTIONS:");
    println!("    --jit <file>           Compile and run immediately using JIT");
    println!("    --aot <file> <output>  Compile to object file using AOT");
    println!("    -h, --help             Print this help message");
    println!();
    println!("EXAMPLES:");
    println!("    cargo run -- program.rzn                    # Standard compilation pipeline");
    println!("    cargo run -- --jit program.rzn              # JIT compile and run");
    println!("    cargo run -- --aot program.rzn program.o    # AOT compile to object file");
}

/// Execute JIT compilation and run immediately
fn execute_jit(program: razen_lang::frontend::parser::ast::Program) {
    let mut backend = Backend::new();
    
    // Run through semantic analysis and IR generation
    let analyzed_program = match backend.semantic_analyzer.analyze(program) {
        Ok(analyzed) => analyzed,
        Err(diagnostics) => {
            eprintln!("❌ Semantic analysis failed:");
            for diagnostic in &diagnostics.diagnostics {
                eprintln!("   - {}: {}", diagnostic.severity, diagnostic.kind.title());
            }
            process::exit(1);
        }
    };
    
    let ir_module = match backend.ir_generator.generate(analyzed_program) {
        Ok(ir) => ir,
        Err(diagnostics) => {
            eprintln!("❌ IR generation failed:");
            for diagnostic in &diagnostics.diagnostics {
                eprintln!("   - {}: {}", diagnostic.severity, diagnostic.kind.title());
            }
            process::exit(1);
        }
    };
    
    // JIT compile and execute
    let mut jit_compiler = match JITCompiler::new() {
        Ok(jit) => jit,
        Err(e) => {
            eprintln!("❌ Failed to create JIT compiler: {}", e);
            process::exit(1);
        }
    };
    
    match jit_compiler.compile_and_run(ir_module) {
        Ok(exit_code) => {
            process::exit(exit_code);
        }
        Err(diagnostics) => {
            eprintln!("❌ JIT execution failed:");
            for diagnostic in &diagnostics.diagnostics {
                eprintln!("   - {}: {}", diagnostic.severity, diagnostic.kind.title());
            }
            process::exit(1);
        }
    }
}

/// Execute AOT compilation to object file
fn execute_aot(program: razen_lang::frontend::parser::ast::Program, output_path: &str) {
    let mut backend = Backend::new();
    
    // Run through semantic analysis and IR generation
    let analyzed_program = match backend.semantic_analyzer.analyze(program) {
        Ok(analyzed) => analyzed,
        Err(diagnostics) => {
            eprintln!("❌ Semantic analysis failed:");
            for diagnostic in &diagnostics.diagnostics {
                eprintln!("   - {}: {}", diagnostic.severity, diagnostic.kind.title());
            }
            process::exit(1);
        }
    };
    
    let ir_module = match backend.ir_generator.generate(analyzed_program) {
        Ok(ir) => ir,
        Err(diagnostics) => {
            eprintln!("❌ IR generation failed:");
            for diagnostic in &diagnostics.diagnostics {
                eprintln!("   - {}: {}", diagnostic.severity, diagnostic.kind.title());
            }
            process::exit(1);
        }
    };
    
    // AOT compile to object file
    let mut aot_compiler = match AOTCompiler::new() {
        Ok(aot) => aot,
        Err(e) => {
            eprintln!("❌ Failed to create AOT compiler: {}", e);
            process::exit(1);
        }
    };
    
    match aot_compiler.compile_to_object(ir_module, output_path) {
        Ok(()) => {
            println!("✅ AOT compilation successful! Object file written to: {}", output_path);
        }
        Err(e) => {
            eprintln!("❌ AOT compilation failed: {}", e);
            process::exit(1);
        }
    }
}

/// Execute standard compilation pipeline (for debugging/development)
fn execute_standard_pipeline(program: razen_lang::frontend::parser::ast::Program) {
    // Test Part 1: Semantic Analysis
    println!("\n🔍 Testing Part 1: Semantic Analysis...");
    let mut backend = Backend::new();
    
    match backend.semantic_analyzer.analyze(program.clone()) {
        Ok(analyzed_program) => {
            println!("✅ Semantic analysis completed successfully!");
            println!("📊 Semantic analysis results:");
            println!("   - Symbols in table: {}", analyzed_program.symbol_table.all_symbols().count());
            println!("   - Type annotations: {}", analyzed_program.type_annotations.len());
            
            // Show some symbol information
            let mut builtin_count = 0;
            let mut user_defined_count = 0;
            
            for symbol in analyzed_program.symbol_table.all_symbols() {
                match &symbol.kind {
                    razen_lang::backend::semantic::SymbolKind::Function { is_builtin, .. } => {
                        if *is_builtin {
                            builtin_count += 1;
                        } else {
                            user_defined_count += 1;
                        }
                    }
                    _ => user_defined_count += 1,
                }
            }
            
            println!("   - Built-in functions: {}", builtin_count);
            println!("   - User-defined symbols: {}", user_defined_count);
            
            // Check for unused symbols
            let unused_symbols = analyzed_program.symbol_table.get_unused_symbols();
            if !unused_symbols.is_empty() {
                println!("⚠️  Unused symbols: {}", unused_symbols.len());
            }
                    
            // Test Phase 2: IR Generation
            println!("\n🔍 Testing Part 2: IR Generation...");
            match backend.ir_generator.generate(analyzed_program) {
                Ok(ir_module) => {
                    println!("✅ IR Generation completed successfully!");
                    println!("📊 IR Module results:");
                    println!("   - Module name: {}", ir_module.name);
                    println!("   - Functions: {}", ir_module.functions.len());
                    println!("   - Globals: {}", ir_module.globals.len());
                    println!("   - String literals: {}", ir_module.strings.len());
                    
                    // Display IR for each function
                    for function in &ir_module.functions {
                        println!("\n🔧 Function: {} -> {}", function.name, function.return_type);
                        println!("   Parameters: {}", function.params.len());
                        println!("   Basic blocks: {}", function.basic_blocks.len());
                        
                        // Show first few instructions of each block
                        for (i, block) in function.basic_blocks.iter().enumerate() {
                            println!("   Block {}: {} ({} instructions)", 
                                     i, block.label, block.instructions.len());
                            
                            // Show first 3 instructions
                            for (j, instr) in block.instructions.iter().take(3).enumerate() {
                                println!("     {}: {}", j, instr);
                            }
                            if block.instructions.len() > 3 {
                                println!("     ... ({} more)", block.instructions.len() - 3);
                            }
                            
                            if let Some(ref terminator) = block.terminator {
                                println!("     terminator: {}", terminator);
                            }
                        }
                    }
                    
                    // Test Phase 3: Cranelift Code Generation
                    println!("\n🔍 Testing Phase 3: Cranelift Code Generation...");
                    match backend.code_generator.generate(ir_module) {
                        Ok(compiled_program) => {
                            println!("✅ Cranelift Code Generation completed successfully!");
                            println!("📊 Compiled Program results:");
                            println!("   - Native code size: {} bytes", compiled_program.bytecode.len());
                            println!("   - Entry point: {}", compiled_program.entry_point);
                            println!("   - Symbols: {}", compiled_program.symbols.len());
                            
                            println!("\n🎉 **COMPLETE COMPILATION PIPELINE WORKING!**");
                            println!("✅ Phase 1: Semantic Analysis");
                            println!("✅ Phase 2: IR Generation");
                            println!("✅ Phase 3: Cranelift Code Generation");
                            println!("🚀 Your Razen language can now compile to native code!");
                        }
                        Err(cranelift_diagnostics) => {
                            println!("❌ Cranelift Code Generation failed with {} error(s):", cranelift_diagnostics.error_count());
                            for diagnostic in &cranelift_diagnostics.diagnostics {
                                println!("   - {}: {}", diagnostic.severity, diagnostic.kind.title());
                            }
                        }
                    }
                }
                Err(ir_diagnostics) => {
                    println!("❌ IR Generation failed with {} error(s):", ir_diagnostics.error_count());
                    for diagnostic in &ir_diagnostics.diagnostics {
                        println!("   - {}: {}", diagnostic.severity, diagnostic.kind.title());
                    }
                }
            }
        }
        Err(semantic_diagnostics) => {
            println!("❌ Semantic analysis failed with {} error(s) and {} warning(s):", 
                     semantic_diagnostics.error_count(), 
                     semantic_diagnostics.warning_count());
            
            // Display semantic errors
            for diagnostic in &semantic_diagnostics.diagnostics {
                println!("   - {}: {}", diagnostic.severity, diagnostic.kind.title());
            }
        }
    }
}