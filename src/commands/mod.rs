//! Professional CLI commands for the Razen programming language
//! 
//! This module provides a clean, professional command-line interface
//! using clap for argument parsing and command handling.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod run;
pub mod dev;
pub mod compile;
pub mod test;

/// Professional Razen CLI Tool
#[derive(Parser)]
#[command(name = "razen")]
#[command(about = "A professional programming language compiler and runtime")]
#[command(version = "0.1.0")]
#[command(author = "Razen Team")]
#[command(long_about = "Razen is a modern, efficient programming language with clean syntax and powerful features.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Compile and run a Razen program (JIT mode)
    #[command(about = "Compile and execute a Razen source file with clean output")]
    Run {
        /// Path to the Razen source file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    
    /// Development mode with debugging and compiler messages
    #[command(about = "Run in development mode with detailed compiler output")]
    Dev {
        /// Path to the Razen source file
        #[arg(value_name = "FILE")]
        file: PathBuf,
        
        /// Watch for file changes and auto-reload
        #[arg(short, long)]
        watch: bool,
    },
    
    /// Compile to executable (AOT mode)
    #[command(about = "Compile a Razen source file to native executable")]
    Compile {
        /// Path to the Razen source file
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        
        /// Output executable path
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,
        
        /// Optimization level (0-3)
        #[arg(short = 'O', long, value_name = "LEVEL", default_value = "2")]
        optimization: u8,
        
        /// Enable debug information
        #[arg(short, long)]
        debug: bool,
    },
    
    /// Run test files
    #[command(about = "Execute Razen test files and report results")]
    Test {
        /// Path to the test file or directory
        #[arg(value_name = "PATH")]
        path: PathBuf,
        
        /// Run tests in verbose mode
        #[arg(short, long)]
        verbose: bool,
        
        /// Filter tests by name pattern
        #[arg(short, long)]
        filter: Option<String>,
    },
    
}

/// Execute the CLI command
pub fn execute_cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Run { file } => {
            run::execute(file)
        }
        Commands::Dev { file, watch } => {
            dev::execute(file, watch)
        }
        Commands::Compile { input, output, optimization, debug } => {
            compile::execute(input, output, optimization, debug)
        }
        Commands::Test { path, verbose, filter } => {
            test::execute(path, verbose, filter)
        }
    }
}

/// Utility function to validate file exists
pub fn validate_file_exists(path: &PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("File does not exist: {}", path.display()));
    }
    if !path.is_file() {
        return Err(format!("Path is not a file: {}", path.display()));
    }
    Ok(())
}

/// Utility function to validate Razen file extension
pub fn validate_razen_file(path: &PathBuf) -> Result<(), String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("rzn") | Some("razen") => Ok(()),
        Some(ext) => Err(format!("Invalid file extension '{}'. Expected '.rzn' or '.razen'", ext)),
        None => Err("File has no extension. Expected '.rzn' or '.razen'".to_string()),
    }
}

/// Professional error handling with colored output
pub fn handle_error(error: &str) -> ! {
    eprintln!("\x1b[31mError:\x1b[0m {}", error);
    std::process::exit(1);
}

/// Success message with colored output
pub fn success_message(message: &str) {
    println!("\x1b[32m✓\x1b[0m {}", message);
}

/// Info message with colored output
pub fn info_message(message: &str) {
    println!("\x1b[34mℹ\x1b[0m {}", message);
}