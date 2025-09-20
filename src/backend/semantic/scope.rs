// src/backend/semantic/scope.rs

use std::collections::HashMap;

/// Represents different types of scopes
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeType {
    Global,
    Function,
    Block,
    Loop,
    Conditional,
    Match,
    Try,
    Module,
}

/// Represents a scope in the program
#[derive(Debug, Clone)]
pub struct Scope {
    pub id: usize,
    pub scope_type: ScopeType,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub can_break: bool,
    pub can_continue: bool,
    pub can_return: bool,
    pub metadata: HashMap<String, String>,
}

impl Scope {
    pub fn new(id: usize, scope_type: ScopeType, parent: Option<usize>) -> Self {
        let (can_break, can_continue, can_return) = match scope_type {
            ScopeType::Global => (false, false, false),
            ScopeType::Function => (false, false, true),
            ScopeType::Block => (false, false, false),
            ScopeType::Loop => (true, true, false),
            ScopeType::Conditional => (false, false, false),
            ScopeType::Match => (false, false, false),
            ScopeType::Try => (false, false, false),
            ScopeType::Module => (false, false, false),
        };
        
        Scope {
            id,
            scope_type,
            parent,
            children: Vec::new(),
            can_break,
            can_continue,
            can_return,
            metadata: HashMap::new(),
        }
    }
    
    pub fn add_child(&mut self, child_id: usize) {
        self.children.push(child_id);
    }
    
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Manages scope hierarchy and control flow validation
#[derive(Debug, Clone)]
pub struct ScopeManager {
    scopes: HashMap<usize, Scope>,
    current_scope: usize,
    next_scope_id: usize,
}

impl ScopeManager {
    pub fn new() -> Self {
        let mut manager = ScopeManager {
            scopes: HashMap::new(),
            current_scope: 0,
            next_scope_id: 1,
        };
        
        // Create global scope
        let global_scope = Scope::new(0, ScopeType::Global, None);
        manager.scopes.insert(0, global_scope);
        
        manager
    }
    
    /// Enter a new scope
    pub fn enter_scope(&mut self, scope_type: ScopeType) -> usize {
        let scope_id = self.next_scope_id;
        self.next_scope_id += 1;
        
        let scope = Scope::new(scope_id, scope_type, Some(self.current_scope));
        
        // Add as child to current scope
        if let Some(current) = self.scopes.get_mut(&self.current_scope) {
            current.add_child(scope_id);
        }
        
        self.scopes.insert(scope_id, scope);
        self.current_scope = scope_id;
        
        scope_id
    }
    
    /// Exit the current scope
    pub fn exit_scope(&mut self) -> Option<usize> {
        if self.current_scope == 0 {
            return None; // Can't exit global scope
        }
        
        let old_scope = self.current_scope;
        
        if let Some(scope) = self.scopes.get(&self.current_scope) {
            if let Some(parent) = scope.parent {
                self.current_scope = parent;
                return Some(old_scope);
            }
        }
        
        None
    }
    
    /// Get the current scope
    pub fn current_scope(&self) -> usize {
        self.current_scope
    }
    
    /// Get a scope by ID
    pub fn get_scope(&self, scope_id: usize) -> Option<&Scope> {
        self.scopes.get(&scope_id)
    }
    
    /// Get a mutable reference to a scope by ID
    pub fn get_scope_mut(&mut self, scope_id: usize) -> Option<&mut Scope> {
        self.scopes.get_mut(&scope_id)
    }
    
    /// Check if break is allowed in the current context
    pub fn can_break(&self) -> bool {
        self.find_scope_with_capability(|scope| scope.can_break).is_some()
    }
    
    /// Check if continue is allowed in the current context
    pub fn can_continue(&self) -> bool {
        self.find_scope_with_capability(|scope| scope.can_continue).is_some()
    }
    
    /// Check if return is allowed in the current context
    pub fn can_return(&self) -> bool {
        self.find_scope_with_capability(|scope| scope.can_return).is_some()
    }
    
    /// Find the nearest scope that satisfies a condition
    fn find_scope_with_capability<F>(&self, predicate: F) -> Option<&Scope>
    where
        F: Fn(&Scope) -> bool,
    {
        let mut current = self.current_scope;
        
        loop {
            if let Some(scope) = self.scopes.get(&current) {
                if predicate(scope) {
                    return Some(scope);
                }
                
                if let Some(parent) = scope.parent {
                    current = parent;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        None
    }
    
    /// Find the nearest function scope
    pub fn find_function_scope(&self) -> Option<&Scope> {
        self.find_scope_with_capability(|scope| scope.scope_type == ScopeType::Function)
    }
    
    /// Find the nearest loop scope
    pub fn find_loop_scope(&self) -> Option<&Scope> {
        self.find_scope_with_capability(|scope| matches!(scope.scope_type, ScopeType::Loop))
    }
    
    /// Get all ancestor scopes of the current scope
    pub fn get_ancestor_scopes(&self) -> Vec<&Scope> {
        let mut ancestors = Vec::new();
        let mut current = self.current_scope;
        
        while let Some(scope) = self.scopes.get(&current) {
            ancestors.push(scope);
            if let Some(parent) = scope.parent {
                current = parent;
            } else {
                break;
            }
        }
        
        ancestors
    }
    
    /// Get the depth of the current scope (0 = global)
    pub fn get_scope_depth(&self) -> usize {
        self.get_ancestor_scopes().len() - 1 // Subtract 1 to exclude current scope
    }
    
    /// Check if we're in a specific type of scope
    pub fn in_scope_type(&self, scope_type: ScopeType) -> bool {
        self.find_scope_with_capability(|scope| scope.scope_type == scope_type).is_some()
    }
    
    /// Get all scopes
    pub fn all_scopes(&self) -> impl Iterator<Item = &Scope> {
        self.scopes.values()
    }
}

impl Default for ScopeManager {
    fn default() -> Self {
        Self::new()
    }
}
