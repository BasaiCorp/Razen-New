//! New command - Create new Razen source files
//!
//! This module handles creating new Razen source files with templates

use std::fs;
use std::path::Path;

/// Execute the new command
pub fn execute(name: String, main: bool, function: bool) -> Result<(), Box<dyn std::error::Error>> {
    let filename = if name.ends_with(".rzn") {
        name.clone()
    } else {
        format!("{}.rzn", name)
    };

    // Check if file already exists
    if Path::new(&filename).exists() {
        return Err(format!("File '{}' already exists", filename).into());
    }

    // Determine template content
    let content = if main {
        generate_main_template()
    } else if function {
        generate_function_template(&name)
    } else {
        generate_basic_template()
    };

    // Write the file
    fs::write(&filename, content)?;

    println!("[SUCCESS] Created new Razen file: {}", filename);
    println!("  Use 'razen run {}' to execute", filename);

    Ok(())
}

/// Generate basic template
fn generate_basic_template() -> String {
    r#"// New Razen file
// Add your code here

"#.to_string()
}

/// Generate main function template
fn generate_main_template() -> String {
    r#"// Razen program with main function

fun main() {
    println("Hello, Razen!")
    
    // Add your code here
}
"#.to_string()
}

/// Generate function template
fn generate_function_template(name: &str) -> String {
    let function_name = name.replace(".rzn", "").replace("-", "_");
    
    format!(r#"// Razen function module

fun {}() {{
    // Add your function implementation here
    println("Function {} called!")
}}

fun main() {{
    {}()
}}
"#, function_name, function_name, function_name)
}
