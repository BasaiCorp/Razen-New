// src/main.rs
//! Professional Razen CLI Tool
//! 
//! This is the main entry point for the Razen programming language
//! command-line interface, providing professional compilation and
//! execution capabilities.

use razen_lang::commands::execute_cli;

fn main() {
    // Execute the professional CLI
    if let Err(e) = execute_cli() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}