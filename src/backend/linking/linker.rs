// src/backend/linking/linker.rs

use super::{LinkingConfig, LinkResult, ExecutableBuilder};
use crate::backend::CompiledProgram;
use crate::frontend::diagnostics::Diagnostics;
use std::collections::HashMap;

/// Main linker for creating executable files
pub struct Linker {
    config: LinkingConfig,
    verbose: bool,
}

impl Linker {
    pub fn new(config: LinkingConfig) -> Self {
        Linker {
            config,
            verbose: false,
        }
    }
    
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    /// Link a compiled program into an executable
    pub fn link(&mut self, compiled_program: CompiledProgram) -> Result<LinkResult, Diagnostics> {
        if self.verbose {
            println!("🔗 Starting linking process...");
            println!("   Output format: {:?}", self.config.output_format);
            println!("   Output path: {}", self.config.output_path);
            println!("   Entry point: {}", self.config.entry_point);
            println!("   Code size: {} bytes", compiled_program.bytecode.len());
        }
        
        // Prepare output path with correct extension
        let output_path = if self.config.output_path.contains('.') {
            self.config.output_path.clone()
        } else {
            format!("{}{}", self.config.output_path, self.config.output_format.extension())
        };
        
        // Create executable builder
        let mut builder = ExecutableBuilder::new(self.config.output_format)
            .with_code(compiled_program.bytecode.clone())
            .with_symbols(compiled_program.symbols.clone())
            .with_entry_point(compiled_program.entry_point)
            .with_debug_info(self.config.debug_info);
        
        // Add runtime if needed
        if self.config.static_linking {
            builder = self.add_runtime_support(builder)?;
        }
        
        // Build the executable
        match builder.build(&output_path) {
            Ok(size) => {
                if self.verbose {
                    println!("✅ Linking completed successfully!");
                    println!("   Executable: {}", output_path);
                    println!("   Size: {} bytes", size);
                }
                
                Ok(LinkResult {
                    executable_path: output_path,
                    size,
                    entry_point: compiled_program.entry_point,
                    symbols: compiled_program.symbols,
                })
            }
            Err(e) => {
                let mut diagnostics = Diagnostics::new();
                diagnostics.add(crate::frontend::diagnostics::Diagnostic::new(
                    crate::frontend::diagnostics::DiagnosticKind::Custom { 
                        message: format!("Failed to create executable: {}", e) 
                    }
                ));
                Err(diagnostics)
            }
        }
    }
    
    /// Add runtime support for standalone executables
    fn add_runtime_support(&self, mut builder: ExecutableBuilder) -> Result<ExecutableBuilder, Diagnostics> {
        if self.verbose {
            println!("   Adding runtime support...");
        }
        
        // Add minimal runtime for built-in functions
        let runtime_code = self.generate_runtime_code();
        
        // For now, just append runtime code
        // In a real implementation, this would properly link runtime libraries
        builder = builder.with_data(runtime_code);
        
        Ok(builder)
    }
    
    /// Generate minimal runtime code for built-in functions
    fn generate_runtime_code(&self) -> Vec<u8> {
        // Placeholder runtime code
        // In a real implementation, this would include:
        // - Built-in function implementations (println, etc.)
        // - Memory management routines
        // - Exception handling
        // - Standard library functions
        
        let mut runtime = Vec::new();
        
        // Add placeholder runtime functions
        runtime.extend_from_slice(b"RAZEN_RUNTIME_V1");
        
        // Placeholder for println implementation
        runtime.extend_from_slice(&[
            // Simple system call stub for println
            0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // mov rax, 1 (sys_write)
            0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00, // mov rdi, 1 (stdout)
            0x0f, 0x05,                                 // syscall
            0xc3,                                       // ret
        ]);
        
        runtime
    }
    
    /// Resolve symbols and addresses
    fn resolve_symbols(&self, symbols: &HashMap<String, usize>) -> Result<HashMap<String, usize>, Diagnostics> {
        let mut resolved = symbols.clone();
        
        // Add built-in function addresses
        resolved.insert("println".to_string(), 0x1000); // Placeholder address
        resolved.insert("print".to_string(), 0x1010);   // Placeholder address
        
        Ok(resolved)
    }
    
    /// Perform dead code elimination at link time
    fn eliminate_dead_code(&self, _code: &[u8]) -> Vec<u8> {
        // Placeholder for link-time dead code elimination
        // In a real implementation, this would:
        // - Analyze call graph
        // - Remove unreferenced functions
        // - Optimize cross-function calls
        
        // For now, just return the original code
        _code.to_vec()
    }
    
    /// Strip debug symbols if requested
    fn strip_symbols(&self, mut symbols: HashMap<String, usize>) -> HashMap<String, usize> {
        if self.config.strip_symbols {
            // Keep only essential symbols
            symbols.retain(|name, _| {
                name == &self.config.entry_point || name.starts_with("__")
            });
        }
        symbols
    }
}

impl Default for Linker {
    fn default() -> Self {
        Self::new(LinkingConfig::default())
    }
}
