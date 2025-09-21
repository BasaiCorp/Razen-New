// src/backend/semantic/symbol_table.rs

use std::collections::HashMap;
use crate::backend::semantic::type_system::Type;
use crate::backend::builtins::BuiltinRegistry;
use crate::frontend::diagnostics::Span;

/// Represents different kinds of symbols
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable {
        is_mutable: bool,
        is_initialized: bool,
    },
    Constant,
    Function {
        params: Vec<(String, Type)>,
        return_type: Type,
        is_builtin: bool,
    },
    Struct {
        fields: HashMap<String, Type>,
    },
    Enum {
        variants: HashMap<String, Option<Type>>,
    },
    Module,
    Parameter,
}

/// Represents a symbol in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub ty: Type,
    pub span: Option<Span>,
    pub scope_id: usize,
    pub is_used: bool,
}

impl Symbol {
    pub fn new(name: String, kind: SymbolKind, ty: Type, span: Option<Span>, scope_id: usize) -> Self {
        Symbol {
            name,
            kind,
            ty,
            span,
            scope_id,
            is_used: false,
        }
    }
    
    pub fn variable(name: String, ty: Type, is_mutable: bool, span: Option<Span>, scope_id: usize) -> Self {
        Symbol::new(
            name,
            SymbolKind::Variable {
                is_mutable,
                is_initialized: false,
            },
            ty,
            span,
            scope_id,
        )
    }
    
    pub fn constant(name: String, ty: Type, span: Option<Span>, scope_id: usize) -> Self {
        Symbol::new(name, SymbolKind::Constant, ty, span, scope_id)
    }
    
    pub fn function(
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        span: Option<Span>,
        scope_id: usize,
    ) -> Self {
        let func_type = Type::Function {
            params: params.iter().map(|(_, ty)| ty.clone()).collect(),
            return_type: Box::new(return_type.clone()),
        };
        
        Symbol::new(
            name,
            SymbolKind::Function {
                params,
                return_type,
                is_builtin: false,
            },
            func_type,
            span,
            scope_id,
        )
    }
    
    pub fn builtin_function(
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
    ) -> Self {
        let func_type = Type::Function {
            params: params.iter().map(|(_, ty)| ty.clone()).collect(),
            return_type: Box::new(return_type.clone()),
        };
        
        Symbol::new(
            name,
            SymbolKind::Function {
                params,
                return_type,
                is_builtin: true,
            },
            func_type,
            None,
            0, // Global scope
        )
    }
    
    pub fn struct_type(name: String, fields: HashMap<String, Type>, span: Option<Span>, scope_id: usize) -> Self {
        Symbol::new(
            name.clone(),
            SymbolKind::Struct { fields },
            Type::Struct(name),
            span,
            scope_id,
        )
    }
    
    pub fn enum_type(name: String, variants: HashMap<String, Option<Type>>, span: Option<Span>, scope_id: usize) -> Self {
        Symbol::new(
            name.clone(),
            SymbolKind::Enum { variants },
            Type::Enum(name),
            span,
            scope_id,
        )
    }
    
    pub fn mark_used(&mut self) {
        self.is_used = true;
    }
    
    pub fn is_mutable(&self) -> bool {
        match &self.kind {
            SymbolKind::Variable { is_mutable, .. } => *is_mutable,
            _ => false,
        }
    }
    
    pub fn is_initialized(&self) -> bool {
        match &self.kind {
            SymbolKind::Variable { is_initialized, .. } => *is_initialized,
            SymbolKind::Constant => true,
            SymbolKind::Function { .. } => true,
            _ => false,
        }
    }
    
    pub fn set_initialized(&mut self) {
        if let SymbolKind::Variable { ref mut is_initialized, .. } = self.kind {
            *is_initialized = true;
        }
    }
}

/// Symbol table for managing symbols and scopes
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// All symbols indexed by their unique ID
    symbols: HashMap<usize, Symbol>,
    
    /// Symbol lookup by name and scope
    scopes: HashMap<usize, HashMap<String, usize>>, // scope_id -> (name -> symbol_id)
    
    /// Scope hierarchy
    scope_parent: HashMap<usize, usize>, // scope_id -> parent_scope_id
    
    /// Next available IDs
    next_symbol_id: usize,
    next_scope_id: usize,
    
    /// Current scope
    current_scope: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = SymbolTable {
            symbols: HashMap::new(),
            scopes: HashMap::new(),
            scope_parent: HashMap::new(),
            next_symbol_id: 0,
            next_scope_id: 1, // 0 is reserved for global scope
            current_scope: 0,
        };
        
        // Initialize global scope
        table.scopes.insert(0, HashMap::new());
        table.add_builtin_functions();
        
        table
    }
    
    /// Add built-in functions to the global scope
    fn add_builtin_functions(&mut self) {
        // Get builtin functions from the registry
        if let Ok(registry) = BuiltinRegistry::global().lock() {
            for function_name in registry.get_function_names() {
                if let Some(builtin_func) = registry.get_function(&function_name) {
                    let symbol = Symbol::builtin_function(
                        builtin_func.name.clone(),
                        builtin_func.params.clone(),
                        builtin_func.return_type.clone(),
                    );
                    self.add_symbol(symbol);
                }
            }
        } else {
            // Fallback to hardcoded builtins if registry is not available
            let builtins = vec![
                ("println", vec![("value", Type::Any)], Type::Void),
                ("print", vec![("value", Type::Any)], Type::Void),
                ("input", vec![("prompt", Type::String)], Type::String),
                ("read", vec![("file", Type::String)], Type::String),
                ("write", vec![("file", Type::String), ("content", Type::String)], Type::Bool),
                ("open", vec![("file", Type::String), ("mode", Type::String)], Type::Any),
                ("close", vec![("handle", Type::Any)], Type::Bool),
            ];
            
            for (name, params, return_type) in builtins {
                let params: Vec<(String, Type)> = params.into_iter()
                    .map(|(n, t)| (n.to_string(), t))
                    .collect();
                
                let symbol = Symbol::builtin_function(name.to_string(), params, return_type);
                self.add_symbol(symbol);
            }
        }
    }
    
    /// Create a new scope
    pub fn push_scope(&mut self) -> usize {
        let scope_id = self.next_scope_id;
        self.next_scope_id += 1;
        
        self.scopes.insert(scope_id, HashMap::new());
        self.scope_parent.insert(scope_id, self.current_scope);
        self.current_scope = scope_id;
        
        scope_id
    }
    
    /// Exit the current scope
    pub fn pop_scope(&mut self) -> Option<usize> {
        if self.current_scope == 0 {
            return None; // Can't pop global scope
        }
        
        let old_scope = self.current_scope;
        self.current_scope = self.scope_parent[&self.current_scope];
        Some(old_scope)
    }
    
    /// Get the current scope ID
    pub fn current_scope(&self) -> usize {
        self.current_scope
    }
    
    /// Add a symbol to the current scope
    pub fn add_symbol(&mut self, symbol: Symbol) -> usize {
        let symbol_id = self.next_symbol_id;
        self.next_symbol_id += 1;
        
        let scope_id = symbol.scope_id;
        let name = symbol.name.clone();
        
        self.symbols.insert(symbol_id, symbol);
        
        if let Some(scope) = self.scopes.get_mut(&scope_id) {
            scope.insert(name, symbol_id);
        }
        
        symbol_id
    }
    
    /// Look up a symbol by name, searching through scope hierarchy
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.lookup_in_scope(name, self.current_scope)
    }
    
    /// Look up a symbol in a specific scope and its parents
    pub fn lookup_in_scope(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        // Check current scope
        if let Some(scope) = self.scopes.get(&scope_id) {
            if let Some(&symbol_id) = scope.get(name) {
                return self.symbols.get(&symbol_id);
            }
        }
        
        // Check parent scopes
        if let Some(&parent_scope) = self.scope_parent.get(&scope_id) {
            return self.lookup_in_scope(name, parent_scope);
        }
        
        None
    }
    
    /// Look up a symbol only in the current scope (no parent lookup)
    pub fn lookup_current_scope_only(&self, name: &str) -> Option<&Symbol> {
        if let Some(scope) = self.scopes.get(&self.current_scope) {
            if let Some(&symbol_id) = scope.get(name) {
                return self.symbols.get(&symbol_id);
            }
        }
        None
    }
    
    /// Get a symbol by its ID
    pub fn get_symbol(&self, symbol_id: usize) -> Option<&Symbol> {
        self.symbols.get(&symbol_id)
    }
    
    /// Get a mutable reference to a symbol by its ID
    pub fn get_symbol_mut(&mut self, symbol_id: usize) -> Option<&mut Symbol> {
        self.symbols.get_mut(&symbol_id)
    }
    
    /// Mark a symbol as used
    pub fn mark_used(&mut self, name: &str) {
        if let Some(symbol) = self.lookup(name) {
            let symbol_id = self.symbols.iter()
                .find(|(_, s)| s.name == name && s.scope_id == symbol.scope_id)
                .map(|(id, _)| *id);
            
            if let Some(id) = symbol_id {
                if let Some(symbol) = self.symbols.get_mut(&id) {
                    symbol.mark_used();
                }
            }
        }
    }
    
    /// Check if a symbol exists in the current scope
    pub fn exists_in_current_scope(&self, name: &str) -> bool {
        self.lookup_current_scope_only(name).is_some()
    }
    
    /// Get all symbols in a specific scope
    pub fn get_scope_symbols(&self, scope_id: usize) -> Vec<&Symbol> {
        if let Some(scope) = self.scopes.get(&scope_id) {
            scope.values()
                .filter_map(|&symbol_id| self.symbols.get(&symbol_id))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get all unused symbols (for warnings)
    pub fn get_unused_symbols(&self) -> Vec<&Symbol> {
        self.symbols.values()
            .filter(|symbol| !symbol.is_used && symbol.scope_id > 0) // Exclude builtins
            .collect()
    }
    
    /// Get all symbols
    pub fn all_symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.values()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
