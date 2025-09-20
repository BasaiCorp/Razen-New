// src/backend/ir/module.rs

// Remove circular import - define types here instead
use crate::backend::ir::BasicBlock;

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<IRParam>,
    pub return_type: String,
    pub basic_blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone)]
pub struct IRParam {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub struct IRGlobal {
    pub name: String,
    pub ty: String,
    pub initializer: Option<String>,
}

/// IR Module representation - Part 2 of the backend (placeholder)
#[derive(Debug, Clone)]
pub struct IRModule {
    pub name: String,
    pub functions: Vec<IRFunction>,
    pub globals: Vec<IRGlobal>,
    pub strings: Vec<String>,
}

impl IRModule {
    pub fn new() -> Self {
        IRModule {
            name: "main".to_string(),
            functions: Vec::new(),
            globals: Vec::new(),
            strings: Vec::new(),
        }
    }
    
    pub fn with_name(name: String) -> Self {
        IRModule {
            name,
            functions: Vec::new(),
            globals: Vec::new(),
            strings: Vec::new(),
        }
    }
    
    pub fn add_function(&mut self, function: IRFunction) {
        self.functions.push(function);
    }
    
    pub fn add_global(&mut self, global: IRGlobal) {
        self.globals.push(global);
    }
    
    pub fn add_string(&mut self, string: String) -> usize {
        let index = self.strings.len();
        self.strings.push(string);
        index
    }
    
    pub fn get_function(&self, name: &str) -> Option<&IRFunction> {
        self.functions.iter().find(|f| f.name == name)
    }
    
    pub fn get_global(&self, name: &str) -> Option<&IRGlobal> {
        self.globals.iter().find(|g| g.name == name)
    }
}

impl Default for IRModule {
    fn default() -> Self {
        Self::new()
    }
}
