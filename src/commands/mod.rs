//! Professional CLI commands for the Razen programming language
//!
//! This module provides a clean, professional command-line interface
//! using clap for argument parsing and command handling.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod benchmark;
pub mod build;
pub mod compile;
pub mod create;
pub mod dev;
pub mod init;
pub mod new;
pub mod run;
pub mod test;

/// Professional Razen CLI Tool
#[derive(Parser)]
#[command(name = "razen")]
#[command(about = "A professional programming language compiler and runtime")]
#[command(version = "0.1-beta.9")]
#[command(author = "Prathmesh Barot (aka PrathmeshCodes)")]
#[command(
    long_about = "Razen is a modern, efficient programming language with clean syntax and powerful features."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Compile and run a Razen program (RAJIT mode)
    #[command(about = "Compile and execute a Razen source file with RAJIT JIT compiler")]
    Run {
        /// Path to the Razen source file
        #[arg(value_name = "FILE")]
        file: PathBuf,
        
        /// Enable optimizations (uses standard level 2 for best performance)
        #[arg(short = 'O', long = "optimize")]
        optimize: bool,
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

        /// Use native JIT compilation (custom x86-64 backend)
        #[arg(long)]
        jit: bool,

        /// Use native AOT compilation (custom x86-64 backend)
        #[arg(long)]
        aot: bool,
    },

    /// Build entire Razen project (reads razen.toml)
    #[command(about = "Build the entire Razen project into executable binary")]
    Build {
        /// Output executable path (defaults to project name)
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Optimization level (0-3, defaults to razen.toml setting)
        #[arg(short = 'O', long, value_name = "LEVEL")]
        optimization: Option<u8>,

        /// Enable debug information (overrides razen.toml setting)
        #[arg(short, long)]
        debug: bool,

        /// Release mode (enables optimizations)
        #[arg(short, long)]
        release: bool,
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

    /// Create a new Razen source file
    #[command(about = "Create a new Razen source file with template")]
    New {
        /// Name of the new file (without .rzn extension)
        #[arg(value_name = "NAME")]
        name: String,

        /// Create with main function template
        #[arg(short, long)]
        main: bool,

        /// Create with function template
        #[arg(short, long)]
        function: bool,
    },

    /// Create a new Razen project
    #[command(about = "Create a new Razen project with razen.toml")]
    Create {
        /// Name of the new project
        #[arg(value_name = "NAME")]
        name: String,

        /// Project template type
        #[arg(short, long, default_value = "basic")]
        template: String,
    },

    /// Initialize razen.toml in current directory
    #[command(about = "Initialize razen.toml configuration in existing directory")]
    Init {
        /// Project name (defaults to directory name)
        #[arg(short, long)]
        name: Option<String>,

        /// Project version
        #[arg(short, long, default_value = "0.1.0")]
        version: String,
    },

    /// Run JIT performance benchmarks
    #[command(about = "Run comprehensive JIT performance benchmarks")]
    Benchmark {
        /// Number of iterations per benchmark
        #[arg(short, long, default_value = "3")]
        iterations: usize,

        /// Specific benchmark to run (optional)
        #[arg(short, long)]
        name: Option<String>,

        /// Output results to file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

/// Execute the CLI command
pub fn execute_cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, optimize } => run::execute(file, optimize),
        Commands::Dev { file, watch, jit, aot } => dev::execute(file, watch, jit, aot),
        Commands::Build {
            output,
            optimization,
            debug,
            release,
        } => build::execute(output, optimization, debug, release),
        Commands::Compile {
            input,
            output,
            optimization,
            debug,
        } => compile::execute(input, output, optimization, debug),
        Commands::Test {
            path,
            verbose,
            filter,
        } => test::execute(path, verbose, filter),
        Commands::New { name, main, function } => new::execute(name, main, function),
        Commands::Create { name, template } => create::execute(name, template),
        Commands::Init { name, version } => init::execute(name, version),
        Commands::Benchmark { iterations, name, output } => benchmark::execute(iterations, name, output),
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
        Some(ext) => Err(format!(
            "Invalid file extension '{}'. Expected '.rzn' or '.razen'",
            ext
        )),
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
    println!("\x1b[32m[SUCCESS]\x1b[0m {}", message);
}

/// Info message with colored output
pub fn info_message(message: &str) {
    println!("\x1b[34m[INFO]\x1b[0m {}", message);
}
