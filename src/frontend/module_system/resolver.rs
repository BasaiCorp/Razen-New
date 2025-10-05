//! Module resolver for file-based module discovery and loading

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;

use crate::frontend::parser::ast::{Program, Statement, UseStatement};
use crate::frontend::parser::{parse_source_with_name, format_parse_errors};
use super::error::ModuleError;

/// Represents a resolved module with its metadata
#[derive(Debug, Clone)]
pub struct ResolvedModule {
    pub name: String,           // Module name (last path component)
    pub path: String,           // Original import path
    pub file_path: PathBuf,     // Actual file system path
    pub program: Program,       // Parsed AST
    pub public_symbols: HashSet<String>, // Public symbols exported by this module
    pub dependencies: Vec<String>, // Other modules this module depends on
}

/// Module resolver that handles file-based module discovery
pub struct ModuleResolver {
    /// Cache of resolved modules
    modules: HashMap<String, ResolvedModule>,
    /// Current resolution stack for circular dependency detection
    resolution_stack: Vec<String>,
    /// Base directory for relative path resolution
    base_dir: PathBuf,
}

impl ModuleResolver {
    pub fn new(base_dir: PathBuf) -> Self {
        ModuleResolver {
            modules: HashMap::new(),
            resolution_stack: Vec::new(),
            base_dir,
        }
    }

    /// Resolve a module from its import path
    /// Handles both stdlib modules (no quotes) and file modules (with quotes)
    pub fn resolve_module(&mut self, import_path: &str, current_file: &Path) -> Result<ResolvedModule, ModuleError> {
        // Check if already resolved
        if let Some(module) = self.modules.get(import_path) {
            return Ok(module.clone());
        }

        // Check if this is a stdlib module (no path separators, no ./)
        if crate::stdlib::is_stdlib_module(import_path) {
            return self.resolve_stdlib_module(import_path);
        }

        // Check for circular dependency
        if self.resolution_stack.contains(&import_path.to_string()) {
            let mut cycle = self.resolution_stack.clone();
            cycle.push(import_path.to_string());
            return Err(ModuleError::CircularDependency { cycle });
        }

        // Add to resolution stack
        self.resolution_stack.push(import_path.to_string());

        // Resolve file path
        let file_path = self.resolve_file_path(import_path, current_file)?;

        // Extract module name (last path component)
        let module_name = self.extract_module_name(import_path);

        // Read and parse module file
        let source = fs::read_to_string(&file_path)
            .map_err(|e| ModuleError::IoError {
                path: import_path.to_string(),
                error: e.to_string(),
            })?;

        let filename = file_path.to_string_lossy().to_string();
        let (program, diagnostics) = parse_source_with_name(&source, &filename);

        if !diagnostics.is_empty() {
            let formatted_errors = format_parse_errors(&diagnostics, &source, &filename);
            return Err(ModuleError::ParseError {
                path: import_path.to_string(),
                error: formatted_errors,
            });
        }

        let program = program.ok_or_else(|| ModuleError::ParseError {
            path: import_path.to_string(),
            error: "Failed to parse module".to_string(),
        })?;

        // Extract public symbols and dependencies
        let (public_symbols, dependencies) = self.analyze_module(&program)?;

        // Create resolved module
        let resolved_module = ResolvedModule {
            name: module_name,
            path: import_path.to_string(),
            file_path,
            program,
            public_symbols,
            dependencies,
        };

        // Remove from resolution stack
        self.resolution_stack.pop();

        // Cache the resolved module
        self.modules.insert(import_path.to_string(), resolved_module.clone());

        Ok(resolved_module)
    }

    /// Resolve a stdlib module (built-in native module)
    fn resolve_stdlib_module(&mut self, module_name: &str) -> Result<ResolvedModule, ModuleError> {
        // Get stdlib module info
        let module_info = crate::stdlib::get_module_info(module_name)
            .ok_or_else(|| ModuleError::ModuleNotFound {
                path: module_name.to_string(),
                searched_paths: vec![format!("stdlib:{}", module_name)],
            })?;

        // Create public symbols from function list
        let public_symbols: HashSet<String> = module_info.functions
            .iter()
            .map(|&f| f.to_string())
            .collect();

        // Create a virtual resolved module for stdlib
        let resolved_module = ResolvedModule {
            name: module_name.to_string(),
            path: format!("stdlib:{}", module_name),
            file_path: PathBuf::from(format!("<stdlib:{}>", module_name)),
            program: Program { statements: vec![] }, // Empty program for stdlib
            public_symbols,
            dependencies: vec![],
        };

        // Cache the resolved module
        self.modules.insert(module_name.to_string(), resolved_module.clone());

        Ok(resolved_module)
    }

    /// Resolve the actual file path from an import path
    fn resolve_file_path(&self, import_path: &str, current_file: &Path) -> Result<PathBuf, ModuleError> {
        // Handle relative paths starting with "./"
        let base_path = if import_path.starts_with("./") {
            current_file.parent().unwrap_or(&self.base_dir).to_path_buf()
        } else {
            self.base_dir.clone()
        };

        // Remove "./" prefix if present
        let clean_path = import_path.strip_prefix("./").unwrap_or(import_path);

        // Try different file extensions and patterns
        let mut searched_paths = Vec::new();

        // 1. Try direct .rzn file
        let direct_path = base_path.join(format!("{}.rzn", clean_path));
        searched_paths.push(direct_path.to_string_lossy().to_string());
        if direct_path.exists() {
            return Ok(direct_path);
        }

        // 2. Try .razen file
        let razen_path = base_path.join(format!("{}.razen", clean_path));
        searched_paths.push(razen_path.to_string_lossy().to_string());
        if razen_path.exists() {
            return Ok(razen_path);
        }

        // 3. Try directory with mod.rzn
        let mod_rzn_path = base_path.join(clean_path).join("mod.rzn");
        searched_paths.push(mod_rzn_path.to_string_lossy().to_string());
        if mod_rzn_path.exists() {
            return Ok(mod_rzn_path);
        }

        // 4. Try directory with mod.razen
        let mod_razen_path = base_path.join(clean_path).join("mod.razen");
        searched_paths.push(mod_razen_path.to_string_lossy().to_string());
        if mod_razen_path.exists() {
            return Ok(mod_razen_path);
        }

        Err(ModuleError::ModuleNotFound {
            path: import_path.to_string(),
            searched_paths,
        })
    }

    /// Extract module name from import path (last component)
    fn extract_module_name(&self, import_path: &str) -> String {
        let clean_path = import_path.strip_prefix("./").unwrap_or(import_path);
        
        // Get the last component of the path
        if let Some(last_component) = clean_path.split('/').last() {
            last_component.to_string()
        } else {
            clean_path.to_string()
        }
    }

    /// Analyze a module to extract public symbols and dependencies
    fn analyze_module(&self, program: &Program) -> Result<(HashSet<String>, Vec<String>), ModuleError> {
        let mut public_symbols = HashSet::new();
        let mut dependencies = Vec::new();

        for statement in &program.statements {
            match statement {
                // Extract use statements as dependencies
                Statement::UseStatement(UseStatement { path, .. }) => {
                    dependencies.push(path.clone());
                }
                
                // Extract public symbols
                Statement::FunctionDeclaration(func) if func.is_public => {
                    public_symbols.insert(func.name.name.clone());
                }
                Statement::StructDeclaration(struct_decl) if struct_decl.is_public => {
                    public_symbols.insert(struct_decl.name.name.clone());
                }
                Statement::EnumDeclaration(enum_decl) if enum_decl.is_public => {
                    public_symbols.insert(enum_decl.name.name.clone());
                }
                Statement::ConstantDeclaration(const_decl) if const_decl.is_public => {
                    public_symbols.insert(const_decl.name.name.clone());
                }
                Statement::VariableDeclaration(var_decl) if var_decl.is_public => {
                    public_symbols.insert(var_decl.name.name.clone());
                }
                
                _ => {} // Ignore private items
            }
        }

        Ok((public_symbols, dependencies))
    }

    /// Get a resolved module by import path
    pub fn get_module(&self, import_path: &str) -> Option<&ResolvedModule> {
        self.modules.get(import_path)
    }

    /// Get all resolved modules
    pub fn get_all_modules(&self) -> &HashMap<String, ResolvedModule> {
        &self.modules
    }

    /// Clear the module cache
    pub fn clear_cache(&mut self) {
        self.modules.clear();
        self.resolution_stack.clear();
    }
}
