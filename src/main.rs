// src/main.rs

use razen_lang::frontend::parser::{parse_source_with_name, format_parse_errors};
use razen_lang::backend::Backend;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Default to syntax.rzn, but allow specifying a different file
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "src/tests/syntax.rzn"
    };

    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            return;
        }
    };

    println!("🔍 Parsing Razen file: {}\n", filename);

    let (program, diagnostics) = parse_source_with_name(&source, filename);

    if diagnostics.is_empty() {
        println!("✅ Parsing completed successfully!");
        if let Some(ref program) = program {
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
                    
                    // Test Part 2: IR Generation
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
    } else {
        println!("❌ Parsing completed with {} error(s) and {} warning(s):\n", 
                 diagnostics.error_count(), 
                 diagnostics.warning_count());
        
        // Display beautiful error messages
        let formatted_errors = format_parse_errors(&diagnostics, &source, filename);
        println!("{}", formatted_errors);
    }

    // Uncomment to see the full AST
    if env::var("RAZEN_DEBUG_AST").is_ok() {
        println!("\n🔧 Debug: Full AST");
        println!("{:#?}", program);
    }
}