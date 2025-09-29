//! Create command - Create new Razen projects
//!
//! This module handles creating new Razen projects with razen.toml

use std::fs;
use std::path::Path;

/// Execute the create command
pub fn execute(name: String, template: String) -> Result<(), Box<dyn std::error::Error>> {
    // Check if directory already exists
    if Path::new(&name).exists() {
        return Err(format!("Directory '{}' already exists", name).into());
    }

    // Create project directory
    fs::create_dir(&name)?;
    println!("Created project directory: {}", name);

    // Create razen.toml
    let toml_content = generate_razen_toml(&name, &template);
    fs::write(format!("{}/razen.toml", name), toml_content)?;
    println!("Created razen.toml configuration");

    // Create main source file based on template
    let main_content = match template.as_str() {
        "basic" => generate_basic_main(),
        "cli" => generate_cli_main(),
        "web" => generate_web_main(),
        "lib" => generate_lib_main(),
        _ => generate_basic_main(),
    };

    fs::write(format!("{}/main.rzn", name), main_content)?;
    println!("Created main.rzn with {} template", template);

    // Create additional files based on template
    match template.as_str() {
        "cli" => create_cli_files(&name)?,
        "web" => create_web_files(&name)?,
        "lib" => create_lib_files(&name)?,
        _ => {}
    }

    // Create README.md
    let readme_content = generate_readme(&name, &template);
    fs::write(format!("{}/README.md", name), readme_content)?;
    println!("Created README.md");

    println!("\nProject '{}' created successfully!", name);
    println!("Next steps:");
    println!("   cd {}", name);
    println!("   razen run main.rzn");

    Ok(())
}

/// Generate razen.toml content (same as init command)
fn generate_razen_toml(name: &str, _template: &str) -> String {
    format!(r#"[project]
name = "{}"
version = "0.1.0"
description = "A Razen project"

[build]
main = "main.rzn"
src_dir = "src"
optimization = 2
debug = false

[dependencies]
# Add your dependencies here
"#, name)
}

/// Generate basic main.rzn
fn generate_basic_main() -> String {
    r#"// Basic Razen application

fun main() {
    println("Hello, Razen!")
    println("Welcome to your new project!")
}
"#.to_string()
}
/// Generate CLI main.rzn
fn generate_cli_main() -> String {
    r#"// Razen CLI application

fun main() {
    println("Razen CLI Application")
    
    // For now, demonstrate with hardcoded command
    var command = "help"
    handle_command(command)
    
    // Show version as well
    println("")
    handle_command("version")
}

fun handle_command(command: str) {
    if command == "help" {
        show_help()
    } else if command == "version" {
        show_version()
    } else {
        println("Unknown command: " + command)
    }
}

fun show_help() {
    println("Available commands:")
    println("  help     - Show this help message")
    println("  version  - Show version information")
}

fun show_version() {
    println("Version 0.1.0")
}
"#.to_string()
}
/// Generate web main.rzn
fn generate_web_main() -> String {
    r#"// Razen web application

fun main() {
    println("Starting Razen Web Server...")
    
    var server = create_server()
    setup_routes(server)
    
    println("Server running on http://localhost:8080")
    server.listen(8080)
}

fun create_server() {
    // Placeholder for future web server support
    println("Creating web server...")
    return "server_instance"
}

fun setup_routes(server: str) {
    println("Setting up routes...")
    
    // GET /
    route_get(server, "/", handle_home)
    
    // GET /api/health
    route_get(server, "/api/health", handle_health)
}

fun handle_home() {
    return "Welcome to Razen Web App!"
}

fun handle_health() {
    return {"status": "ok", "message": "Server is healthy"}
}

fun route_get(server: str, path: str, handler: str) {
    // Placeholder for future routing support
    println("Route registered: GET " + path)
}
"#.to_string()
}

/// Generate library main.rzn
fn generate_lib_main() -> String {
    r#"// Razen library

// Library functions
fun add(a: int, b: int) -> int {
    return a + b
}

fun multiply(a: int, b: int) -> int {
    return a * b
}

fun greet(name: str) -> str {
    return "Hello, " + name + "!"
}

// Example usage
fun main() {
    println("Razen Library Example")
    
    var result1 = add(5, 3)
    var result2 = multiply(4, 7)
    var greeting = greet("World")
    
    println("5 + 3 = " + result1)
    println("4 * 7 = " + result2)
    println(greeting)
}
"#.to_string()
}

/// Create CLI-specific files
fn create_cli_files(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create commands directory
    fs::create_dir(format!("{}/commands", name))?;
    
    // Create example command
    let cmd_content = r#"// Example command module

fun execute_example() {
    println("Example command executed!")
}
"#;
    fs::write(format!("{}/commands/example.rzn", name), cmd_content)?;
    println!("Created commands/example.rzn");
    
    Ok(())
}

/// Create web-specific files
fn create_web_files(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create public directory
    fs::create_dir(format!("{}/public", name))?;
    
    // Create index.html
    let html_content = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Razen Web App</title>
</head>
<body>
    <h1>Welcome to Razen Web App!</h1>
    <p>This is a web application built with Razen.</p>
</body>
</html>
"#;
    fs::write(format!("{}/public/index.html", name), html_content)?;
    println!("Created public/index.html");
    
    Ok(())
}

/// Create library-specific files
fn create_lib_files(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create lib directory
    fs::create_dir(format!("{}/lib", name))?;
    
    // Create utils module
    let utils_content = r#"// Utility functions

fun is_even(n: int) -> bool {
    return n % 2 == 0
}

fun is_odd(n: int) -> bool {
    return n % 2 != 0
}

fun max(a: int, b: int) -> int {
    if a > b {
        return a
    } else {
        return b
    }
}

fun min(a: int, b: int) -> int {
    if a < b {
        return a
    } else {
        return b
    }
}
"#;
    fs::write(format!("{}/lib/utils.rzn", name), utils_content)?;
    println!("Created lib/utils.rzn");
    
    Ok(())
}

/// Generate README.md
fn generate_readme(name: &str, template: &str) -> String {
    format!(r#"# {}

A Razen project created with the `{}` template.

## Getting Started

### Prerequisites

- Razen programming language installed
- Basic knowledge of Razen syntax

### Running the Project

```bash
# Run the main application
razen run main.rzn

# Run in development mode with detailed output
razen dev main.rzn
```

### Project Structure

```
{}/
├── razen.toml          # Project configuration
├── main.rzn            # Main application file
├── README.md           # This file
{}
```

## Building

```bash
# Compile to executable
razen compile main.rzn -o {}
```

## Testing

```bash
# Run tests (when available)
razen test .
```

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License.
"#, name, template, name, 
    match template {
        "cli" => "└── commands/          # Command modules",
        "web" => "└── public/            # Static web files",
        "lib" => "└── lib/               # Library modules", 
        _ => ""
    }, name)
}
