//! Init command - Initialize razen.toml in existing directory
//!
//! This module handles initializing razen.toml configuration files

use std::env;
use std::fs;
use std::path::Path;

/// Execute the init command
pub fn execute(name: Option<String>, version: String) -> Result<(), Box<dyn std::error::Error>> {
    // Check if razen.toml already exists
    if Path::new("razen.toml").exists() {
        return Err("razen.toml already exists in current directory".into());
    }

    // Get project name (use directory name if not provided)
    let project_name = match name {
        Some(n) => n,
        None => {
            let current_dir = env::current_dir()?;
            current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("razen-project")
                .to_string()
        }
    };

    // Generate razen.toml content
    let toml_content = generate_razen_toml(&project_name, &version);
    fs::write("razen.toml", toml_content)?;
    println!("✓ Created razen.toml");

    // Create main.rzn if it doesn't exist
    if !Path::new("main.rzn").exists() {
        let main_content = generate_default_main();
        fs::write("main.rzn", main_content)?;
        println!("✓ Created main.rzn");
    }

    // Create .gitignore if it doesn't exist
    if !Path::new(".gitignore").exists() {
        let gitignore_content = generate_gitignore();
        fs::write(".gitignore", gitignore_content)?;
        println!("✓ Created .gitignore");
    }

    println!("\n🎉 Initialized Razen project '{}'!", project_name);
    println!("📁 Files created:");
    println!("   ✓ razen.toml - Project configuration");
    if !Path::new("main.rzn").exists() {
        println!("   ✓ main.rzn - Main source file");
    }
    if !Path::new(".gitignore").exists() {
        println!("   ✓ .gitignore - Git ignore rules");
    }
    println!("\n🚀 Next steps:");
    println!("   razen run main.rzn");

    Ok(())
}

/// Generate razen.toml content
fn generate_razen_toml(name: &str, version: &str) -> String {
    format!(r#"[project]
name = "{}"
version = "{}"
description = "A Razen project"
author = "Your Name <your.email@example.com>"
license = "MIT"

[build]
target = "native"
optimization = 2
debug = false

[dependencies]
# Add your dependencies here
# example_lib = "1.0.0"

[dev-dependencies]
# Add your development dependencies here
# test_framework = "0.5.0"

[scripts]
# Custom build scripts
# build = "custom_build.rzn"
# test = "run_tests.rzn"

[features]
# Feature flags
default = []
# web = ["http_server"]
# cli = ["argument_parser"]

[metadata]
# Additional metadata
keywords = ["razen", "programming"]
categories = ["development-tools"]
repository = "https://github.com/username/{}"
documentation = "https://docs.rs/{}"
homepage = "https://github.com/username/{}"

# Razen-specific configuration
[razen]
# Language version compatibility
min_version = "0.1.0"
max_version = "1.0.0"

# Compiler settings
[razen.compiler]
warnings_as_errors = false
strict_mode = false
unsafe_allowed = false

# Runtime settings
[razen.runtime]
stack_size = "1MB"
heap_size = "10MB"
gc_enabled = true
"#, name, version, name, name, name)
}

/// Generate default main.rzn
fn generate_default_main() -> String {
    r#"// Razen project main file

fun main() {
    println("Hello, Razen!")
    println("Project initialized successfully!")
    
    // Add your code here
}
"#.to_string()
}

/// Generate .gitignore content
fn generate_gitignore() -> String {
    r#"# Razen build artifacts
target/
*.o
*.exe
*.dll
*.so
*.dylib

# Razen cache
.razen/
.cache/

# IDE files
.vscode/
.idea/
*.swp
*.swo
*~

# OS files
.DS_Store
Thumbs.db

# Logs
*.log

# Temporary files
*.tmp
*.temp

# Environment files
.env
.env.local

# Dependencies
node_modules/
vendor/

# Build outputs
dist/
build/
out/
"#.to_string()
}
