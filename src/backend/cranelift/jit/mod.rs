// src/backend/cranelift/jit/mod.rs
// Professional JIT Compiler Module for Razen Language
// Organized following Cranelift best practices

pub mod compiler;
pub mod runtime;
pub mod builtins;
pub mod instruction_compiler;
pub mod interpreter;

pub use compiler::JITCompiler;
pub use runtime::JITRuntime;

/// JIT compilation result
pub type JITResult<T> = Result<T, String>;

/// Main JIT interface for Razen language
/// Provides a clean, simple API like `go run`
pub struct RazenJIT {
    compiler: JITCompiler,
    #[allow(dead_code)]
    runtime: JITRuntime,
}

impl RazenJIT {
    /// Create a new JIT instance
    pub fn new() -> JITResult<Self> {
        Ok(Self {
            compiler: JITCompiler::new()?,
            runtime: JITRuntime::new(),
        })
    }

    /// Compile and run a Razen program immediately (like `go run`)
    pub fn run(&mut self, ir_module: crate::backend::ir::IRModule) -> JITResult<i32> {
        // Use the complete JIT compiler implementation
        match self.compiler.compile_and_run(ir_module) {
            Ok(exit_code) => Ok(exit_code),
            Err(diagnostics) => {
                // Convert diagnostics to string error
                let error_msg = diagnostics.diagnostics.iter()
                    .map(|d| d.kind.title())
                    .collect::<Vec<_>>()
                    .join(", ");
                Err(format!("JIT compilation failed: {}", error_msg))
            }
        }
    }
}

impl Default for RazenJIT {
    fn default() -> Self {
        Self::new().expect("Failed to create RazenJIT")
    }
}
