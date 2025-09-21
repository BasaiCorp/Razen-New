// src/backend/linking/mod.rs

pub mod linker;
pub mod executable;
pub mod formats;

pub use linker::Linker;
pub use executable::{ExecutableFormat, ExecutableBuilder};
pub use formats::*;


/// Linking configuration
#[derive(Debug, Clone)]
pub struct LinkingConfig {
    pub output_format: ExecutableFormat,
    pub output_path: String,
    pub entry_point: String,
    pub static_linking: bool,
    pub debug_info: bool,
    pub strip_symbols: bool,
}

impl Default for LinkingConfig {
    fn default() -> Self {
        LinkingConfig {
            output_format: ExecutableFormat::Native,
            output_path: "output".to_string(),
            entry_point: "main".to_string(),
            static_linking: true,
            debug_info: false,
            strip_symbols: false,
        }
    }
}

/// Link result containing the final executable
#[derive(Debug)]
pub struct LinkResult {
    pub executable_path: String,
    pub size: usize,
    pub entry_point: usize,
    pub symbols: std::collections::HashMap<String, usize>,
}
