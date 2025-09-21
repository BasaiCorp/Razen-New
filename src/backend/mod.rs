// src/backend/mod.rs

pub mod semantic;
pub mod ir;
pub mod builtins;
pub mod cranelift;
pub mod optimization;
pub mod linking;

use crate::frontend::parser::ast::Program;
use crate::frontend::diagnostics::Diagnostics;

/// The main backend compiler pipeline
pub struct Backend {
    pub semantic_analyzer: semantic::SemanticAnalyzer,
    pub ir_generator: ir::IRGenerator,
    pub code_generator: cranelift::CodeGenerator,
    pub optimizer: optimization::Optimizer,
    pub linker: linking::Linker,
}

impl Backend {
    pub fn new() -> Self {
        Backend {
            semantic_analyzer: semantic::SemanticAnalyzer::new(),
            ir_generator: ir::IRGenerator::new(),
            code_generator: cranelift::CodeGenerator::new().expect("Failed to create CodeGenerator"),
            optimizer: optimization::Optimizer::new(optimization::OptimizationLevel::Basic),
            linker: linking::Linker::default(),
        }
    }
    
    pub fn with_optimization_level(mut self, level: optimization::OptimizationLevel) -> Self {
        self.optimizer = optimization::Optimizer::new(level);
        self
    }
    
    pub fn with_linking_config(mut self, config: linking::LinkingConfig) -> Self {
        self.linker = linking::Linker::new(config);
        self
    }

    /// Compile a program through the full backend pipeline
    pub fn compile(&mut self, program: Program) -> Result<CompiledProgram, Diagnostics> {
        // Phase 1: Semantic Analysis
        let analyzed_program = self.semantic_analyzer.analyze(program)?;
        
        // Phase 2: IR Generation
        let ir_module = self.ir_generator.generate(analyzed_program)?;
        
        // Phase 4: Optimization (before code generation for better results)
        let optimized_ir = self.optimizer.optimize(ir_module)?;
        
        // Phase 3: Code Generation
        let compiled = self.code_generator.generate(optimized_ir)?;
        
        Ok(compiled)
    }
    
    /// Compile and link a program to create an executable
    pub fn compile_and_link(&mut self, program: Program) -> Result<linking::LinkResult, Diagnostics> {
        // Compile through all phases
        let compiled_program = self.compile(program)?;
        
        // Phase 4: Linking
        let link_result = self.linker.link(compiled_program)?;
        
        Ok(link_result)
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