// src/backend/semantic/mod.rs

pub mod symbol_table;
pub mod type_system;
pub mod analyzer;
pub mod scope;

pub use analyzer::SemanticAnalyzer;
pub use symbol_table::{Symbol, SymbolTable, SymbolKind};
pub use type_system::{Type, TypeChecker};
pub use scope::{Scope, ScopeManager};

use crate::frontend::parser::ast::Program;
use crate::frontend::diagnostics::Diagnostics;

/// Represents a semantically analyzed program
#[derive(Debug, Clone)]
pub struct AnalyzedProgram {
    pub program: Program,
    pub symbol_table: SymbolTable,
    pub type_annotations: std::collections::HashMap<usize, Type>, // Node ID -> Type
}

impl AnalyzedProgram {
    pub fn new(program: Program, symbol_table: SymbolTable) -> Self {
        AnalyzedProgram {
            program,
            symbol_table,
            type_annotations: std::collections::HashMap::new(),
        }
    }

    pub fn get_type(&self, node_id: usize) -> Option<&Type> {
        self.type_annotations.get(&node_id)
    }

    pub fn set_type(&mut self, node_id: usize, ty: Type) {
        self.type_annotations.insert(node_id, ty);
    }
}
