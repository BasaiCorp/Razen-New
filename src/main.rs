// src/main.rs

pub mod frontend;
pub mod backend;
pub mod utils;
pub mod commands;

use frontend::parser::{parse_source_with_name, format_parse_errors};
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
                    frontend::parser::ast::Statement::VariableDeclaration(_) |
                    frontend::parser::ast::Statement::ConstantDeclaration(_) => var_count += 1,
                    frontend::parser::ast::Statement::FunctionDeclaration(_) => func_count += 1,
                    frontend::parser::ast::Statement::StructDeclaration(_) => struct_count += 1,
                    frontend::parser::ast::Statement::EnumDeclaration(_) => enum_count += 1,
                    _ => {}
                }
            }
            
            println!("   - Variables/Constants: {}", var_count);
            println!("   - Functions: {}", func_count);
            println!("   - Structs: {}", struct_count);
            println!("   - Enums: {}", enum_count);
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
