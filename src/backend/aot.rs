//! AOT (Ahead-of-Time) Compiler for Razen
//!
//! This module creates self-contained executables using Method 2: Embedded Runtime
//! The executable contains the Razen runtime + compiled IR, similar to Go binaries.
//! No external dependencies required!

use std::fs;
use std::path::Path;
use std::process::Command;
use super::execution::IR;

/// AOT Compiler for generating native executables
pub struct AOTCompiler {
    optimization_level: u8,
    debug_info: bool,
    _target_triple: String, // Reserved for future cross-platform support
}

impl AOTCompiler {
    /// Create a new AOT compiler
    pub fn new() -> Self {
        AOTCompiler {
            optimization_level: 2,
            debug_info: false,
            _target_triple: Self::get_target_triple(),
        }
    }

    /// Set optimization level (0-3)
    pub fn set_optimization_level(&mut self, level: u8) {
        self.optimization_level = level.min(3);
    }

    /// Enable or disable debug information
    pub fn set_debug_info(&mut self, debug: bool) {
        self.debug_info = debug;
    }

    /// Compile IR to self-contained executable using embedded runtime
    pub fn compile_to_executable(
        &self,
        ir: &[IR],
        output_path: &Path,
        project_name: &str,
    ) -> Result<(), String> {
        // Step 1: Create embedded runtime executable
        self.create_embedded_executable(ir, output_path, project_name)
    }

    /// Create a self-contained executable with embedded Razen runtime and IR
    fn create_embedded_executable(
        &self,
        ir: &[IR],
        output_path: &Path,
        project_name: &str,
    ) -> Result<(), String> {
        // Step 1: Generate Rust source code for the embedded executable
        let rust_code = self.generate_embedded_rust_code(ir, project_name)?;
        
        // Step 2: Create temporary Rust project
        let temp_dir = std::env::temp_dir().join(format!("razen_build_{}", project_name));
        let _ = fs::remove_dir_all(&temp_dir); // Clean up any previous builds
        fs::create_dir_all(&temp_dir)
            .map_err(|e| format!("Failed to create temp directory: {}", e))?;
        
        // Step 3: Write Cargo.toml with size optimizations
        let cargo_toml = self.generate_optimized_cargo_toml(project_name);
        fs::write(temp_dir.join("Cargo.toml"), cargo_toml)
            .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;
        
        // Step 4: Create src directory and main.rs
        let src_dir = temp_dir.join("src");
        fs::create_dir_all(&src_dir)
            .map_err(|e| format!("Failed to create src directory: {}", e))?;
        
        fs::write(src_dir.join("main.rs"), rust_code)
            .map_err(|e| format!("Failed to write main.rs: {}", e))?;
        
        // Step 5: Build the executable with cargo
        self.build_with_cargo(&temp_dir, output_path, project_name)?;
        
        // Step 6: Clean up temporary files
        let _ = fs::remove_dir_all(&temp_dir);
        
        Ok(())
    }

    /// Generate Rust code for embedded runtime executable
    fn generate_embedded_rust_code(&self, ir: &[IR], _project_name: &str) -> Result<String, String> {
        let mut rust_code = String::new();

        // Serialize IR instructions to Rust code
        rust_code.push_str("// Auto-generated Razen executable with embedded runtime\n");
        rust_code.push_str("// This is a self-contained binary with no external dependencies\n\n");
        
        // Add the embedded Razen runtime (simplified version)
        rust_code.push_str(&self.generate_embedded_runtime());
        
        // Add the main function with embedded IR
        rust_code.push_str("fn main() {\n");
        rust_code.push_str("    let mut runtime = RazenRuntime::new();\n");
        rust_code.push_str("    \n");
        rust_code.push_str("    // Embedded IR instructions\n");
        rust_code.push_str("    let ir_instructions = vec![\n");
        
        for instruction in ir {
            rust_code.push_str(&format!("        {},\n", self.serialize_ir_to_rust(instruction)));
        }
        
        rust_code.push_str("    ];\n");
        rust_code.push_str("    \n");
        rust_code.push_str("    // Execute the embedded IR\n");
        rust_code.push_str("    if let Err(e) = runtime.execute_ir(&ir_instructions) {\n");
        rust_code.push_str("        eprintln!(\"Runtime error: {}\", e);\n");
        rust_code.push_str("        std::process::exit(1);\n");
        rust_code.push_str("    }\n");
        rust_code.push_str("}\n");

        Ok(rust_code)
    }

    /// Serialize IR instruction to Rust code representation
    fn serialize_ir_to_rust(&self, instruction: &IR) -> String {
        match instruction {
            IR::PushString(s) => format!("IR::PushString(\"{}\".to_string())", s),
            IR::PushNumber(n) => format!("IR::PushNumber({})", n),
            IR::PushBoolean(b) => format!("IR::PushBoolean({})", b),
            IR::PushNull => "IR::PushNull".to_string(),
            IR::Pop => "IR::Pop".to_string(),
            IR::Call(func_name, arg_count) => format!("IR::Call(\"{}\".to_string(), {})", func_name, arg_count),
            IR::Add => "IR::Add".to_string(),
            IR::Subtract => "IR::Subtract".to_string(),
            IR::Multiply => "IR::Multiply".to_string(),
            IR::Divide => "IR::Divide".to_string(),
            IR::Modulo => "IR::Modulo".to_string(),
            IR::Equal => "IR::Equal".to_string(),
            IR::NotEqual => "IR::NotEqual".to_string(),
            IR::LessThan => "IR::LessThan".to_string(),
            IR::LessEqual => "IR::LessEqual".to_string(),
            IR::GreaterThan => "IR::GreaterThan".to_string(),
            IR::GreaterEqual => "IR::GreaterEqual".to_string(),
            IR::StoreVar(name) => format!("IR::StoreVar(\"{}\".to_string())", name),
            IR::LoadVar(name) => format!("IR::LoadVar(\"{}\".to_string())", name),
            IR::Jump(addr) => format!("IR::Jump({})", addr),
            IR::JumpIfFalse(addr) => format!("IR::JumpIfFalse({})", addr),
            IR::Label(label) => format!("IR::Label(\"{}\".to_string())", label),
            IR::Return => "IR::Return".to_string(),
            _ => format!("// Unimplemented IR: {:?}", instruction),
        }
    }

    /// Generate embedded Razen runtime in Rust
    fn generate_embedded_runtime(&self) -> String {
        r#"use std::collections::HashMap;
use std::io::{self, Write};

// Embedded IR enum (copy of the original)
#[derive(Debug, Clone)]
pub enum IR {
    PushNumber(f64),
    PushString(String),
    PushBoolean(bool),
    PushNull,
    Pop,
    StoreVar(String),
    LoadVar(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,
    Jump(usize),
    JumpIfFalse(usize),
    Call(String, usize),
    Return,
    Label(String),
}

// Embedded Razen Runtime
pub struct RazenRuntime {
    stack: Vec<String>,
    variables: HashMap<String, String>,
}

impl RazenRuntime {
    pub fn new() -> Self {
        RazenRuntime {
            stack: Vec::new(),
            variables: HashMap::new(),
        }
    }

    pub fn execute_ir(&mut self, instructions: &[IR]) -> Result<(), String> {
        let mut pc = 0; // Program counter
        
        while pc < instructions.len() {
            match &instructions[pc] {
                IR::PushString(s) => {
                    self.stack.push(s.clone());
                }
                IR::PushNumber(n) => {
                    self.stack.push(n.to_string());
                }
                IR::PushBoolean(b) => {
                    self.stack.push(b.to_string());
                }
                IR::PushNull => {
                    self.stack.push("null".to_string());
                }
                IR::Pop => {
                    self.stack.pop();
                }
                IR::StoreVar(name) => {
                    if let Some(value) = self.stack.pop() {
                        self.variables.insert(name.clone(), value);
                    }
                }
                IR::LoadVar(name) => {
                    let value = self.variables.get(name).cloned().unwrap_or("null".to_string());
                    self.stack.push(value);
                }
                IR::Add => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        let a_num: f64 = a.parse().unwrap_or(0.0);
                        let b_num: f64 = b.parse().unwrap_or(0.0);
                        self.stack.push((a_num + b_num).to_string());
                    }
                }
                IR::Subtract => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        let a_num: f64 = a.parse().unwrap_or(0.0);
                        let b_num: f64 = b.parse().unwrap_or(0.0);
                        self.stack.push((a_num - b_num).to_string());
                    }
                }
                IR::Multiply => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        let a_num: f64 = a.parse().unwrap_or(0.0);
                        let b_num: f64 = b.parse().unwrap_or(0.0);
                        self.stack.push((a_num * b_num).to_string());
                    }
                }
                IR::Divide => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        let a_num: f64 = a.parse().unwrap_or(0.0);
                        let b_num: f64 = b.parse().unwrap_or(1.0);
                        if b_num != 0.0 {
                            self.stack.push((a_num / b_num).to_string());
                        } else {
                            return Err("Division by zero".to_string());
                        }
                    }
                }
                IR::Call(func_name, _arg_count) => {
                    match func_name.as_str() {
                        "println" => {
                            if let Some(value) = self.stack.pop() {
                                println!("{}", value);
                            }
                        }
                        "print" => {
                            if let Some(value) = self.stack.pop() {
                                print!("{}", value);
                                io::stdout().flush().unwrap();
                            }
                        }
                        "printc" => {
                            if let (Some(color), Some(text)) = (self.stack.pop(), self.stack.pop()) {
                                self.print_colored(&text, &color);
                            }
                        }
                        "printlnc" => {
                            if let (Some(color), Some(text)) = (self.stack.pop(), self.stack.pop()) {
                                self.print_colored(&text, &color);
                                println!();
                            }
                        }
                        _ => {
                            return Err(format!("Unknown function: {}", func_name));
                        }
                    }
                }
                IR::Return => break,
                _ => {
                    // Skip unimplemented instructions
                }
            }
            pc += 1;
        }
        Ok(())
    }

    fn print_colored(&self, text: &str, color: &str) {
        let color_code = match color {
            "red" => "\x1b[31m",
            "green" => "\x1b[32m",
            "blue" => "\x1b[34m",
            "yellow" => "\x1b[33m",
            "magenta" | "purple" => "\x1b[35m",
            "cyan" => "\x1b[36m",
            "white" => "\x1b[37m",
            "bright_red" => "\x1b[91m",
            "bright_green" => "\x1b[92m",
            "bright_blue" => "\x1b[94m",
            "pink" => "\x1b[95m",
            "orange" => "\x1b[38;5;208m",
            _ => "",
        };
        print!("{}{}\x1b[0m", color_code, text);
        io::stdout().flush().unwrap();
    }
}
"#.to_string()
    }

    /// Generate optimized Cargo.toml for size and speed
    fn generate_optimized_cargo_toml(&self, project_name: &str) -> String {
        let opt_level = if self.optimization_level >= 3 {
            "\"z\"".to_string()  // String values need quotes
        } else {
            self.optimization_level.to_string()  // Numeric values don't need quotes
        };
        let codegen_units = if self.optimization_level >= 3 { 1 } else { 4 };
        
        format!(r#"[package]
name = "{}"
version = "1.0.0"
edition = "2021"

[profile.release]
opt-level = {}
lto = true
codegen-units = {}
panic = "abort"
strip = true

[dependencies]
"#, project_name, opt_level, codegen_units)
    }

    /// Build the executable using cargo
    fn build_with_cargo(&self, temp_dir: &Path, output_path: &Path, project_name: &str) -> Result<(), String> {
        // Build the project in release mode with optimizations
        let output = Command::new("cargo")
            .current_dir(temp_dir)
            .args(&["build", "--release"])
            .output()
            .map_err(|e| format!("Failed to run cargo build: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Cargo build failed: {}", stderr));
        }

        // Copy the built executable to the desired output path
        // The executable name comes from the Cargo.toml [package] name, not the temp directory name
        let mut built_exe = temp_dir.join("target/release").join(project_name);
        
        // On Windows, add .exe extension
        if cfg!(windows) {
            built_exe.set_extension("exe");
        }
        
        if built_exe.exists() {
            std::fs::copy(&built_exe, output_path)
                .map_err(|e| format!("Failed to copy executable: {}", e))?;
        } else {
            // Debug: List what files are actually in target/release
            let release_dir = temp_dir.join("target/release");
            if release_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&release_dir) {
                    let files: Vec<String> = entries
                        .filter_map(|e| e.ok())
                        .map(|e| e.file_name().to_string_lossy().to_string())
                        .collect();
                    return Err(format!("Built executable not found at {:?}. Files in target/release: {:?}", built_exe, files));
                }
            }
            return Err(format!("Built executable not found at {:?}", built_exe));
        }

        Ok(())
    }

    /// Get the target triple for the current platform
    fn get_target_triple() -> String {
        if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
            "x86_64-unknown-linux-gnu".to_string()
        } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
            "x86_64-apple-darwin".to_string()
        } else if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
            "aarch64-apple-darwin".to_string()
        } else if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
            "x86_64-pc-windows-msvc".to_string()
        } else {
            "unknown".to_string()
        }
    }
}

impl Default for AOTCompiler {
    fn default() -> Self {
        Self::new()
    }
}
