//! Build command - Build entire Razen project from razen.toml
//!
//! This module handles building complete Razen projects by reading razen.toml
//! configuration, scanning source directories, and compiling all files.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use crate::frontend::parser::{parse_source_with_name, format_parse_errors};
use crate::backend::execution::Compiler;
use crate::backend::{SemanticAnalyzer, AOT};
use crate::frontend::diagnostics::display::render_diagnostics;
use super::{success_message, info_message};

/// Razen project configuration structure
#[derive(Debug, Deserialize, Serialize)]
pub struct RazenConfig {
    pub project: ProjectConfig,
    pub build: BuildConfig,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default)]
    pub razen: Option<RazenSpecificConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuildConfig {
    #[serde(default = "default_target")]
    pub target: String,
    #[serde(default = "default_main")]
    pub main: String,
    #[serde(default = "default_src_dir")]
    pub src_dir: String,
    #[serde(default = "default_optimization")]
    pub optimization: u8,
    #[serde(default)]
    pub debug: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RazenSpecificConfig {
    #[serde(default)]
    pub min_version: Option<String>,
    #[serde(default)]
    pub compiler: Option<CompilerConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompilerConfig {
    #[serde(default)]
    pub warnings_as_errors: bool,
    #[serde(default)]
    pub strict_mode: bool,
}

// Default values
fn default_target() -> String { "native".to_string() }
fn default_main() -> String { "main.rzn".to_string() }
fn default_src_dir() -> String { "src".to_string() }
fn default_optimization() -> u8 { 2 }

/// Execute the build command
pub fn execute(
    output: Option<PathBuf>,
    optimization: Option<u8>,
    debug: bool,
    release: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Building Razen project...");
    
    // Step 1: Read and parse razen.toml
    let config = read_razen_config()?;
    info_message(&format!("Found project: {} v{}", config.project.name, config.project.version));
    
    // Step 2: Determine build settings
    let build_optimization = if release {
        3 // Maximum optimization for release
    } else {
        optimization.unwrap_or(config.build.optimization)
    };
    
    let build_debug = debug || (!release && config.build.debug);
    
    info_message(&format!("Build mode: {} (optimization: {}, debug: {})", 
        if release { "release" } else { "debug" },
        build_optimization,
        build_debug
    ));
    
    // Step 3: Find main file and source files
    let main_file = find_main_file(&config)?;
    let source_files = scan_source_directory(&config)?;
    
    info_message(&format!("Main file: {}", main_file.display()));
    info_message(&format!("Found {} source files", source_files.len()));
    
    // Step 4: Parse and compile all source files
    let mut all_programs = Vec::new();
    let mut all_sources = Vec::new();
    
    // Parse main file first
    let main_source = fs::read_to_string(&main_file)?;
    let main_filename = main_file.to_string_lossy().to_string();
    let (main_program, main_diagnostics) = parse_source_with_name(&main_source, &main_filename);
    
    if !main_diagnostics.is_empty() {
        eprintln!("❌ Parsing errors in main file:");
        let formatted_errors = format_parse_errors(&main_diagnostics, &main_source, &main_filename);
        eprintln!("{}", formatted_errors);
        return Err("Failed to parse main file".into());
    }
    
    if let Some(program) = main_program {
        all_programs.push(program);
        all_sources.push((main_filename.clone(), main_source));
    }
    
    // Parse other source files (excluding main file)
    for source_file in &source_files {
        // Skip main file, already parsed
        if source_file.canonicalize().unwrap_or_else(|_| source_file.clone()) == 
           main_file.canonicalize().unwrap_or_else(|_| main_file.clone()) {
            continue;
        }
        
        let source = fs::read_to_string(source_file)?;
        let filename = source_file.to_string_lossy().to_string();
        let (program, diagnostics) = parse_source_with_name(&source, &filename);
        
        if !diagnostics.is_empty() {
            eprintln!("❌ Parsing errors in {}:", filename);
            let formatted_errors = format_parse_errors(&diagnostics, &source, &filename);
            eprintln!("{}", formatted_errors);
            return Err(format!("Failed to parse {}", filename).into());
        }
        
        if let Some(program) = program {
            all_programs.push(program);
            all_sources.push((filename, source));
        }
    }
    
    success_message(&format!("Parsed {} files successfully", all_programs.len()));
    
    // Step 5: Semantic analysis
    info_message("Running semantic analysis...");
    let base_dir = std::env::current_dir()?;
    let mut semantic_analyzer = SemanticAnalyzer::with_module_support(base_dir, main_file.clone());
    
    for (i, program) in all_programs.iter().enumerate() {
        let semantic_diagnostics = semantic_analyzer.analyze_with_source(program, &all_sources[i].1);
        
        if !semantic_diagnostics.is_empty() {
            let rendered = render_diagnostics(&semantic_diagnostics, &all_sources);
            eprintln!("{}", rendered);
            
            if semantic_diagnostics.has_errors() {
                return Err("Semantic analysis failed".into());
            }
        }
    }
    
    success_message("Semantic analysis completed");
    
    // Step 6: Compile to executable
    info_message("Compiling to executable...");
    let mut compiler = Compiler::new();
    compiler.set_clean_output(true);
    compiler.set_current_file(main_file.clone());
    
    // Compile main program (others are handled via module system)
    if let Some(main_program) = all_programs.first() {
        compiler.compile_program(main_program.clone());
    }
    
    if !compiler.errors.is_empty() {
        return Err(format!("Compilation failed: {}", compiler.errors.join("; ")).into());
    }
    
    // Step 7: Determine output path
    let output_path = output.unwrap_or_else(|| {
        let mut path = PathBuf::from(&config.project.name);
        if cfg!(windows) {
            path.set_extension("exe");
        }
        path
    });
    
    // Step 8: Generate native executable using AOT compilation
    info_message(&format!("Generating native executable: {}", output_path.display()));
    
    // Create AOT compiler
    let mut aot_compiler = AOT::with_optimization(build_optimization);
    
    // Get compiled IR from the compiler
    let ir = compiler.get_ir();
    
    // Compile to native executable
    aot_compiler.compile(&ir, output_path.to_str().unwrap())
        .map_err(|e| format!("AOT compilation failed: {}", e))?;
    
    success_message(&format!("Built executable: {}", output_path.display()));
    println!("\nBuild completed successfully!");
    println!("Project: {} v{}", config.project.name, config.project.version);
    println!("Target: {}", output_path.display());
    println!("Optimization: Level {}", build_optimization);
    
    Ok(())
}

/// Read and parse razen.toml configuration
fn read_razen_config() -> Result<RazenConfig, Box<dyn std::error::Error>> {
    let config_path = Path::new("razen.toml");
    
    if !config_path.exists() {
        return Err("razen.toml not found. Run 'razen init' to create a new project.".into());
    }
    
    let config_content = fs::read_to_string(config_path)?;
    let config: RazenConfig = toml::from_str(&config_content)
        .map_err(|e| format!("Failed to parse razen.toml: {}", e))?;
    
    Ok(config)
}

/// Find the main file specified in configuration
fn find_main_file(config: &RazenConfig) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let main_path = Path::new(&config.build.main);
    
    if main_path.exists() {
        return Ok(main_path.to_path_buf());
    }
    
    // Try in src directory
    let src_main = Path::new(&config.build.src_dir).join(&config.build.main);
    if src_main.exists() {
        return Ok(src_main);
    }
    
    Err(format!("Main file not found: {} (also checked in {})", 
        config.build.main, config.build.src_dir).into())
}

/// Scan source directory for all .rzn files
fn scan_source_directory(config: &RazenConfig) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut source_files = Vec::new();
    let src_dir = Path::new(&config.build.src_dir);
    
    // If src directory doesn't exist, just look in current directory
    let search_dir = if src_dir.exists() { src_dir } else { Path::new(".") };
    
    scan_directory_recursive(search_dir, &mut source_files)?;
    
    Ok(source_files)
}

/// Recursively scan directory for .rzn files
fn scan_directory_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Skip common non-source directories
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                if matches!(dir_name, "target" | ".git" | "node_modules" | ".razen") {
                    continue;
                }
            }
            scan_directory_recursive(&path, files)?;
        } else if let Some(ext) = path.extension() {
            if ext == "rzn" || ext == "razen" {
                files.push(path);
            }
        }
    }
    
    Ok(())
}