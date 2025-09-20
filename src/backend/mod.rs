// src/backend/mod.rs

pub mod semantic;
pub mod ir;
pub mod codegen;
pub mod builtins;
pub mod cranelift;

use crate::frontend::parser::ast::Program;
use crate::frontend::diagnostics::Diagnostics;

/// The main backend compiler pipeline
pub struct Backend {
    pub semantic_analyzer: semantic::SemanticAnalyzer,
    pub ir_generator: ir::IRGenerator,
    pub code_generator: codegen::CodeGenerator,
}

impl Backend {
    pub fn new() -> Self {
        Backend {
            semantic_analyzer: semantic::SemanticAnalyzer::new(),
            ir_generator: ir::IRGenerator::new(),
            code_generator: codegen::CodeGenerator::new(),
        }
    }

    /// Compile a program through the full backend pipeline
    pub fn compile(&mut self, program: Program) -> Result<CompiledProgram, Diagnostics> {
        // Phase 1: Semantic Analysis
        let analyzed_program = self.semantic_analyzer.analyze(program)?;
        
        // Phase 2: IR Generation
        let ir_module = self.ir_generator.generate(analyzed_program)?;
        
        // Phase 3: Code Generation
        let compiled = self.code_generator.generate(ir_module)?;
        
        Ok(compiled)
    }

    /// Compile and execute immediately (JIT)
    pub fn compile_and_run(&mut self, program: Program) -> Result<i32, Diagnostics> {
        let compiled = self.compile(program)?;
        Ok(compiled.execute())
    }
}

/// Represents a compiled Razen program
pub struct CompiledProgram {
    pub bytecode: Vec<u8>,
    pub entry_point: usize,
    pub symbols: std::collections::HashMap<String, usize>,
}

impl CompiledProgram {
    pub fn execute(&self) -> i32 {
        // TODO: Implement execution
        0
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, &self.bytecode)
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::new()
    }
}