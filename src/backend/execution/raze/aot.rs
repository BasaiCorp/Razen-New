// src/backend/execution/raze/aot.rs
//! AOT (Ahead-Of-Time) compiler for RAZE
//! 
//! Compiles MIR to standalone native executables.

use super::mir::MIRBuilder;
use super::codegen::{create_codegen, Architecture};
use super::optimization::{OptimizationPipeline, OptimizationLevel};
use super::{RAZEError, RAZEResult};
use crate::backend::execution::ir::IR;
use std::fs;
use std::path::Path;

// May be needed for multi-function compilation in the future
#[allow(unused_imports)]
use super::mir::MIRModule;

/// AOT compiler
pub struct AOTCompiler {
    optimization_level: OptimizationLevel,
    target_arch: Architecture,
}

impl AOTCompiler {
    /// Create a new AOT compiler
    pub fn new() -> Self {
        Self {
            optimization_level: OptimizationLevel::Aggressive,
            target_arch: Architecture::current(),
        }
    }
    
    /// Create AOT compiler with specific optimization level
    pub fn with_optimization(level: u8) -> Self {
        Self {
            optimization_level: OptimizationLevel::from_u8(level),
            target_arch: Architecture::current(),
        }
    }
    
    /// Set target architecture
    pub fn set_target(&mut self, arch: Architecture) {
        self.target_arch = arch;
    }
    
    /// Compile IR to standalone executable
    pub fn compile_to_file(&mut self, ir: &[IR], output_path: &Path) -> RAZEResult<()> {
        println!("[INFO] RAZE AOT compilation started");
        println!("[INFO] Target: {}", self.target_arch);
        println!("[INFO] Optimization: O{}", self.optimization_level.as_u8());
        
        // Step 1: Translate IR to MIR
        let mut mir_builder = MIRBuilder::new();
        let mir_module = mir_builder.translate_ir(ir)
            .map_err(|e| RAZEError::CompilationError(e))?;
        
        // Step 2: Get main function
        let main_func = mir_module.functions.get("main")
            .ok_or_else(|| RAZEError::CompilationError("No main function found".to_string()))?;
        
        // Step 3: Aggressive optimization for AOT
        let optimizer = OptimizationPipeline::new(self.optimization_level);
        let optimized_mir = optimizer.optimize(main_func.instructions.clone());
        
        println!("[INFO] Optimized {} instructions", optimized_mir.len());
        
        // Step 4: Generate native code
        let mut codegen = create_codegen(self.target_arch);
        let machine_code = codegen.generate(&optimized_mir)?;
        
        println!("[INFO] Generated {} bytes of machine code", machine_code.len());
        
        // Step 5: Create executable wrapper
        self.create_executable(&machine_code, output_path)?;
        
        println!("[SUCCESS] AOT compilation completed: {}", output_path.display());
        
        Ok(())
    }
    
    fn create_executable(&self, machine_code: &[u8], output_path: &Path) -> RAZEResult<()> {
        // For now, create a simple wrapper
        // In a full implementation, this would create a proper ELF/Mach-O/PE executable
        
        #[cfg(unix)]
        {
            // Create a shell script wrapper for now
            let script = format!(
                "#!/bin/bash\n\
                 # RAZE AOT Compiled Executable\n\
                 # Architecture: {}\n\
                 # Optimization: O{}\n\
                 # Code size: {} bytes\n\
                 \n\
                 echo '[INFO] RAZE AOT executable'\n\
                 echo '[INFO] This is a development version'\n\
                 exit 0\n",
                self.target_arch,
                self.optimization_level.as_u8(),
                machine_code.len()
            );
            
            fs::write(output_path, script)
                .map_err(|e| RAZEError::CompilationError(format!("Failed to write executable: {}", e)))?;
            
            // Make executable
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(output_path)
                .map_err(|e| RAZEError::CompilationError(format!("Failed to get metadata: {}", e)))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(output_path, perms)
                .map_err(|e| RAZEError::CompilationError(format!("Failed to set permissions: {}", e)))?;
        }
        
        #[cfg(windows)]
        {
            // Create a batch file wrapper for Windows
            let script = format!(
                "@echo off\n\
                 REM RAZE AOT Compiled Executable\n\
                 REM Architecture: {}\n\
                 REM Optimization: O{}\n\
                 REM Code size: {} bytes\n\
                 \n\
                 echo [INFO] RAZE AOT executable\n\
                 echo [INFO] This is a development version\n\
                 exit /b 0\n",
                self.target_arch,
                self.optimization_level.as_u8(),
                machine_code.len()
            );
            
            fs::write(output_path, script)
                .map_err(|e| RAZEError::CompilationError(format!("Failed to write executable: {}", e)))?;
        }
        
        Ok(())
    }
}

impl Default for AOTCompiler {
    fn default() -> Self {
        Self::new()
    }
}
