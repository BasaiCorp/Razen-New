//! Visibility checker for module system - implements Rust-level access control

use std::collections::HashMap;
use crate::frontend::parser::ast::Statement;
use super::resolver::ResolvedModule;
use super::error::ModuleError;

/// Visibility levels for symbols
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    /// Public - accessible from anywhere
    Public,
    /// Private - only accessible within the same module
    Private,
}

/// Symbol information with visibility
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub visibility: Visibility,
    pub module_path: String,
    pub symbol_type: SymbolType,
}

/// Types of symbols that can be exported/imported
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Function,
    Struct,
    Enum,
    Constant,
    Variable,
}

/// Visibility checker that validates symbol access across modules
pub struct VisibilityChecker {
    /// Symbol table mapping symbol names to their info
    symbols: HashMap<String, SymbolInfo>,
    /// Module imports mapping alias/module name to import path
    imports: HashMap<String, String>,
}

impl VisibilityChecker {
    pub fn new() -> Self {
        VisibilityChecker {
            symbols: HashMap::new(),
            imports: HashMap::new(),
        }
    }

    /// Register symbols from a resolved module
    pub fn register_module(&mut self, module: &ResolvedModule) {
        // Register all symbols from the module
        for statement in &module.program.statements {
            match statement {
                Statement::FunctionDeclaration(func) => {
                    let visibility = if func.is_public { Visibility::Public } else { Visibility::Private };
                    let symbol_key = format!("{}::{}", module.name, func.name.name);
                    
                    self.symbols.insert(symbol_key, SymbolInfo {
                        name: func.name.name.clone(),
                        visibility,
                        module_path: module.path.clone(),
                        symbol_type: SymbolType::Function,
                    });
                }
                
                Statement::StructDeclaration(struct_decl) => {
                    let visibility = if struct_decl.is_public { Visibility::Public } else { Visibility::Private };
                    let symbol_key = format!("{}::{}", module.name, struct_decl.name.name);
                    
                    self.symbols.insert(symbol_key, SymbolInfo {
                        name: struct_decl.name.name.clone(),
                        visibility,
                        module_path: module.path.clone(),
                        symbol_type: SymbolType::Struct,
                    });
                }
                
                Statement::EnumDeclaration(enum_decl) => {
                    let visibility = if enum_decl.is_public { Visibility::Public } else { Visibility::Private };
                    let symbol_key = format!("{}::{}", module.name, enum_decl.name.name);
                    
                    self.symbols.insert(symbol_key, SymbolInfo {
                        name: enum_decl.name.name.clone(),
                        visibility,
                        module_path: module.path.clone(),
                        symbol_type: SymbolType::Enum,
                    });
                }
                
                Statement::ConstantDeclaration(const_decl) => {
                    let visibility = if const_decl.is_public { Visibility::Public } else { Visibility::Private };
                    let symbol_key = format!("{}::{}", module.name, const_decl.name.name);
                    
                    self.symbols.insert(symbol_key, SymbolInfo {
                        name: const_decl.name.name.clone(),
                        visibility,
                        module_path: module.path.clone(),
                        symbol_type: SymbolType::Constant,
                    });
                }
                
                Statement::VariableDeclaration(var_decl) => {
                    let visibility = if var_decl.is_public { Visibility::Public } else { Visibility::Private };
                    let symbol_key = format!("{}::{}", module.name, var_decl.name.name);
                    
                    self.symbols.insert(symbol_key, SymbolInfo {
                        name: var_decl.name.name.clone(),
                        visibility,
                        module_path: module.path.clone(),
                        symbol_type: SymbolType::Variable,
                    });
                }
                
                _ => {} // Ignore other statement types
            }
        }
    }

    pub fn register_import(&mut self, import_path: &str, alias: Option<&str>, module_name: &str) {
        let import_key = alias.unwrap_or(module_name).to_string();
        self.imports.insert(import_key, import_path.to_string());
    }

    /// Check if a symbol can be accessed from a module
    pub fn check_symbol_access(&self, module_ref: &str, symbol_name: &str) -> Result<&SymbolInfo, ModuleError> {
        // Check if this is a stdlib module - stdlib modules always allow access to their functions
        if crate::stdlib::is_stdlib_module(module_ref) {
            // Verify the function exists in the stdlib module
            if crate::stdlib::is_stdlib_function(module_ref, symbol_name) {
                // For stdlib modules, we bypass the symbol table check
                // Return a reference to a static dummy symbol
                use std::sync::OnceLock;
                static STDLIB_SYMBOL: OnceLock<SymbolInfo> = OnceLock::new();
                return Ok(STDLIB_SYMBOL.get_or_init(|| SymbolInfo {
                    name: "stdlib_function".to_string(),
                    visibility: Visibility::Public,
                    module_path: "stdlib".to_string(),
                    symbol_type: SymbolType::Function,
                }));
            } else {
                return Err(ModuleError::SymbolNotExported {
                    symbol: symbol_name.to_string(),
                    module: module_ref.to_string(),
                });
            }
        }

        // Resolve the module reference to actual import path
        let import_path = self.imports.get(module_ref)
            .ok_or_else(|| ModuleError::InvalidPath {
                path: module_ref.to_string(),
                reason: "Module not imported".to_string(),
            })?;

        // Extract module name from import path
        let module_name = self.extract_module_name(import_path);
        
        // Build symbol key
        let symbol_key = format!("{}::{}", module_name, symbol_name);
        
        // Find the symbol
        let symbol_info = self.symbols.get(&symbol_key)
            .ok_or_else(|| ModuleError::SymbolNotExported {
                symbol: symbol_name.to_string(),
                module: module_ref.to_string(),
            })?;

        // Check visibility
        if symbol_info.visibility == Visibility::Private {
            return Err(ModuleError::SymbolNotExported {
                symbol: symbol_name.to_string(),
                module: module_ref.to_string(),
            });
        }

        Ok(symbol_info)
    }

    /// Extract module name from import path (last component)
    fn extract_module_name(&self, import_path: &str) -> String {
        let clean_path = import_path.strip_prefix("./").unwrap_or(import_path);
        
        if let Some(last_component) = clean_path.split('/').last() {
            last_component.to_string()
        } else {
            clean_path.to_string()
        }
    }

    /// Get all registered symbols
    pub fn get_symbols(&self) -> &HashMap<String, SymbolInfo> {
        &self.symbols
    }

    /// Get all registered imports
    pub fn get_imports(&self) -> &HashMap<String, String> {
        &self.imports
    }

    /// Clear all registered symbols and imports
    pub fn clear(&mut self) {
        self.symbols.clear();
        self.imports.clear();
    }
}
