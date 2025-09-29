// src/backend/execution/compiler.rs
//! Clean compiler implementation based on the proven old design

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::frontend::parser::ast::{Program, Statement, Expression, InterpolatedString, InterpolationPart, UseStatement};
use crate::frontend::parser::parse_source_with_name;
use super::ir::IR;
use super::runtime::Runtime;

/// Symbol table for variable and function tracking
#[derive(Debug, Clone)]
struct SymbolTable {
    symbols: HashMap<String, usize>,
    parent: Option<Box<SymbolTable>>,
    next_index: usize,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: None,
            next_index: 0,
        }
    }

    fn new_enclosed(parent: SymbolTable) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: Some(Box::new(parent)),
            next_index: 0,
        }
    }

    fn define(&mut self, name: &str) -> usize {
        let index = self.next_index;
        self.symbols.insert(name.to_string(), index);
        self.next_index += 1;
        index
    }

    #[allow(dead_code)]
    fn resolve(&self, name: &str) -> Option<usize> {
        match self.symbols.get(name) {
            Some(index) => Some(*index),
            None => {
                if let Some(parent) = &self.parent {
                    parent.resolve(name)
                } else {
                    None
                }
            }
        }
    }
}

/// Function table for tracking function definitions
#[derive(Debug, Clone)]
struct FunctionTable {
    functions: HashMap<String, usize>,
}

impl FunctionTable {
    fn new() -> Self {
        FunctionTable {
            functions: HashMap::new(),
        }
    }

    fn define(&mut self, name: &str, address: usize) {
        self.functions.insert(name.to_string(), address);
    }

    fn resolve(&self, name: &str) -> Option<usize> {
        self.functions.get(name).copied()
    }
}

/// Clean compiler for translating AST to IR
pub struct Compiler {
    pub ir: Vec<IR>,
    symbol_table: SymbolTable,
    function_table: FunctionTable,
    function_param_names: HashMap<String, Vec<String>>,
    current_function: Option<String>,
    break_stack: Vec<Vec<usize>>,
    continue_stack: Vec<Vec<usize>>,
    loop_stack: Vec<(String, String)>, // (continue_label, break_label)
    continue_positions: Vec<usize>, // Store actual continue positions
    label_counter: usize,
    clean_output: bool,
    pub errors: Vec<String>,
    current_file_path: Option<std::path::PathBuf>,
    imported_modules: HashMap<String, String>, // module_name -> module_path
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            ir: Vec::new(),
            symbol_table: SymbolTable::new(),
            function_table: FunctionTable::new(),
            function_param_names: HashMap::new(),
            current_function: None,
            break_stack: Vec::new(),
            continue_stack: Vec::new(),
            loop_stack: Vec::new(),
            continue_positions: Vec::new(),
            label_counter: 0,
            clean_output: false,
            errors: Vec::new(),
            current_file_path: None,
            imported_modules: HashMap::new(),
        }
    }

    pub fn from_program(program: Program) -> Result<Self, String> {
        let mut compiler = Compiler::new();
        compiler.set_clean_output(true); // Default to clean output
        compiler.compile_program(program);
        
        if !compiler.errors.is_empty() {
            return Err(compiler.errors.join("; "));
        }
        
        Ok(compiler)
    }

    pub fn set_clean_output(&mut self, clean: bool) {
        self.clean_output = clean;
    }

    /// Get the compiled IR instructions
    pub fn get_ir(&self) -> &[IR] {
        &self.ir
    }

    pub fn set_current_file(&mut self, file_path: std::path::PathBuf) {
        self.current_file_path = Some(file_path);
    }

    fn generate_label(&mut self, prefix: &str) -> String {
        let label = format!("{}{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    fn emit(&mut self, code: IR) -> usize {
        let pos = self.ir.len();
        self.ir.push(code);
        pos
    }

    fn emit_label(&mut self, label: &str) -> usize {
        self.emit(IR::Label(label.to_string()))
    }

    fn replace_instruction(&mut self, pos: usize, code: IR) {
        if pos < self.ir.len() {
            self.ir[pos] = code;
        }
    }

    fn enter_scope(&mut self) {
        let new_table = SymbolTable::new_enclosed(self.symbol_table.clone());
        self.symbol_table = new_table;
    }

    fn leave_scope(&mut self) {
        if let Some(parent) = self.symbol_table.parent.take() {
            self.symbol_table = *parent;
        }
    }

    fn define_builtins(&mut self) {
        // Define built-in functions
        self.symbol_table.define("print");
        self.symbol_table.define("println");
        self.symbol_table.define("printc");   // Colored print
        self.symbol_table.define("printlnc"); // Colored println
        self.symbol_table.define("input");
        self.symbol_table.define("read");
        self.symbol_table.define("write");
        self.symbol_table.define("len");
        self.symbol_table.define("append");
        self.symbol_table.define("remove");
    }

    pub fn compile_program(&mut self, program: Program) {
        if !self.clean_output {
            println!("Starting compilation...");
            println!("Processing {} statements", program.statements.len());
        }
        
        // Define built-in functions
        self.define_builtins();
        if !self.clean_output {
            println!("Defined built-in functions");
        }

        // First pass: process use statements and load imported modules
        for stmt in &program.statements {
            if let Statement::UseStatement(use_stmt) = stmt {
                self.process_use_statement(use_stmt);
            }
        }

        // Second pass: register all functions (including imported ones)
        let mut function_count = 0;
        for stmt in &program.statements {
            if let Statement::FunctionDeclaration(func_decl) = stmt {
                self.symbol_table.define(&func_decl.name.name);
                function_count += 1;
                if !self.clean_output {
                    println!("Found function: {}", func_decl.name.name);
                }
            }
        }
        
        if !self.clean_output && function_count > 0 {
            println!("Registered {} user functions", function_count);
        }

        // Third pass: compile all statements
        if !self.clean_output {
            println!("Compiling statements...");
        }
        for stmt in program.statements {
            self.compile_statement(stmt);
        }
        
        // Automatically call main function if it exists (like your old implementation)
        if self.function_table.resolve("main").is_some() {
            if !self.clean_output {
                println!("Auto-calling main function");
            }
            self.emit(IR::Call("main".to_string(), 0));
            self.emit(IR::Pop); // Discard return value
        }
        
        if !self.clean_output {
            println!("Compilation completed - generated {} IR instructions", self.ir.len());
        }
    }

    fn process_use_statement(&mut self, use_stmt: &UseStatement) {
        if let Some(ref current_file) = self.current_file_path {
            let module_path = self.resolve_module_path(&use_stmt.path, current_file);
            
            if let Some(path) = module_path {
                if let Ok(source) = fs::read_to_string(&path) {
                    let (program, _diagnostics) = parse_source_with_name(&source, &path.to_string_lossy());
                    
                    if let Some(program) = program {
                        // Extract module name from path or alias
                        let module_name = if let Some(alias) = &use_stmt.alias {
                            alias.name.clone()
                        } else {
                            path.file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("unknown")
                                .to_string()
                        };
                        
                        // Store the module mapping
                        self.imported_modules.insert(module_name.clone(), path.to_string_lossy().to_string());
                        
                        // Compile all public items from the imported module with qualified names
                        for stmt in program.statements {
                            match stmt {
                                Statement::FunctionDeclaration(func_decl) => {
                                    // Check if function is public
                                    if func_decl.is_public {
                                        let qualified_name = format!("{}.{}", module_name, func_decl.name.name);
                                        let parameters: Vec<String> = func_decl.parameters.iter()
                                            .map(|p| p.name.name.clone()).collect();
                                        
                                        // Compile the function with qualified name
                                        self.compile_function_declaration(qualified_name.clone(), parameters, func_decl.body.statements);
                                        
                                        if !self.clean_output {
                                            println!("Imported function: {} -> {}", func_decl.name.name, qualified_name);
                                        }
                                    }
                                },
                                Statement::ConstantDeclaration(const_decl) => {
                                    // Check if constant is public
                                    if const_decl.is_public {
                                        let qualified_name = format!("{}.{}", module_name, const_decl.name.name);
                                        
                                        // Register the constant in symbol table
                                        self.symbol_table.define(&qualified_name);
                                        
                                        // Compile the constant value and store it
                                        self.compile_expression(const_decl.initializer);
                                        self.emit(IR::StoreVar(qualified_name.clone()));
                                        
                                        if !self.clean_output {
                                            println!("Imported constant: {} -> {}", const_decl.name.name, qualified_name);
                                        }
                                    }
                                },
                                Statement::VariableDeclaration(var_decl) => {
                                    // Check if variable is public
                                    if var_decl.is_public {
                                        let qualified_name = format!("{}.{}", module_name, var_decl.name.name);
                                        
                                        // Register the variable in symbol table
                                        self.symbol_table.define(&qualified_name);
                                        
                                        // Compile the variable value and store it
                                        if let Some(expr) = var_decl.initializer {
                                            self.compile_expression(expr);
                                        } else {
                                            self.emit(IR::PushNull);
                                        }
                                        self.emit(IR::StoreVar(qualified_name.clone()));
                                        
                                        if !self.clean_output {
                                            println!("Imported variable: {} -> {}", var_decl.name.name, qualified_name);
                                        }
                                    }
                                },
                                Statement::StructDeclaration(struct_decl) => {
                                    // Check if struct is public
                                    if struct_decl.is_public {
                                        let qualified_name = format!("{}.{}", module_name, struct_decl.name.name);
                                        
                                        // Register the struct type (for future struct instantiation)
                                        // For now, just log it
                                        if !self.clean_output {
                                            println!("Imported struct: {} -> {}", struct_decl.name.name, qualified_name);
                                        }
                                    }
                                },
                                _ => {
                                    // Skip other statement types
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn resolve_module_path(&self, module_path: &str, current_file: &std::path::Path) -> Option<std::path::PathBuf> {
        let current_dir = current_file.parent()?;
        let mut path = current_dir.join(module_path);
        
        // Try with .rzn extension
        if !path.extension().is_some() {
            path.set_extension("rzn");
        }
        
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    fn compile_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::VariableDeclaration(var_decl) => {
                self.symbol_table.define(&var_decl.name.name);
                
                if let Some(expr) = var_decl.initializer {
                    self.compile_expression(expr);
                } else {
                    self.emit(IR::PushNull);
                }
                
                self.emit(IR::StoreVar(var_decl.name.name));
            },
            Statement::FunctionDeclaration(func_decl) => {
                let name = func_decl.name.name;
                let parameters: Vec<String> = func_decl.parameters.iter()
                    .map(|p| p.name.name.clone()).collect();
                
                self.compile_function_declaration(name, parameters, func_decl.body.statements);
            },
            Statement::ReturnStatement(ret_stmt) => {
                if let Some(expr) = ret_stmt.value {
                    self.compile_expression(expr);
                } else {
                    self.emit(IR::PushNull);
                }
                self.emit(IR::Return);
            },
            Statement::ExpressionStatement(expr_stmt) => {
                self.compile_expression(expr_stmt.expression);
                self.emit(IR::Pop); // Discard result
            },
            Statement::BlockStatement(block_stmt) => {
                self.enter_scope();
                for stmt in block_stmt.statements {
                    self.compile_statement(stmt);
                }
                self.leave_scope();
            },
            Statement::IfStatement(if_stmt) => {
                self.compile_if_elif_else_statement(if_stmt);
            },
            Statement::WhileStatement(while_stmt) => {
                let body_statements = if let Statement::BlockStatement(block) = *while_stmt.body {
                    block.statements
                } else {
                    vec![*while_stmt.body]
                };
                
                self.compile_while_statement(while_stmt.condition, body_statements);
            },
            Statement::ForStatement(for_stmt) => {
                let body_statements = if let Statement::BlockStatement(block) = *for_stmt.body {
                    block.statements
                } else {
                    vec![*for_stmt.body]
                };
                
                self.compile_for_statement(for_stmt.variable.name, for_stmt.iterable, body_statements);
            },
            Statement::BreakStatement(_) => {
                self.compile_break_statement();
            },
            Statement::ContinueStatement(_) => {
                self.compile_continue_statement();
            },
            Statement::MatchStatement(match_stmt) => {
                self.compile_match_statement(match_stmt);
            },
            Statement::ConstantDeclaration(const_decl) => {
                // Handle const declarations similar to variable declarations
                // but mark them as immutable in the symbol table
                self.symbol_table.define(&const_decl.name.name);
                
                // Compile the initializer expression
                self.compile_expression(const_decl.initializer);
                
                // Store the constant value
                self.emit(IR::StoreVar(const_decl.name.name));
            },
            Statement::StructDeclaration(struct_decl) => {
                self.compile_struct_declaration(struct_decl);
            },
            Statement::EnumDeclaration(enum_decl) => {
                self.compile_enum_declaration(enum_decl);
            },
            Statement::ImplBlock(impl_block) => {
                self.compile_impl_block(impl_block);
            },
            _ => {
                // Handle other statement types as needed
                if !self.clean_output {
                    println!("Unhandled statement type: {:?}", std::mem::discriminant(&stmt));
                }
            }
        }
    }

    fn compile_function_declaration(&mut self, name: String, parameters: Vec<String>, body: Vec<Statement>) {
        let old_function = self.current_function.clone();
        self.current_function = Some(name.clone());

        let function_label = self.generate_label("function_");
        let end_label = self.generate_label("end_");

        // Skip over function body
        let jump_pos = self.emit(IR::Jump(0));

        // Mark function start
        let function_start = self.emit_label(&function_label);
        self.function_table.define(&name, function_start);
        self.function_param_names.insert(name.clone(), parameters.clone());

        self.emit(IR::DefineFunction(name.clone(), function_start));

        // Create function scope
        self.enter_scope();

        // Define parameters
        for param in &parameters {
            self.symbol_table.define(param);
        }

        // Compile function body
        for stmt in body {
            self.compile_statement(stmt);
        }

        // Ensure function returns
        self.emit(IR::PushNull);
        self.emit(IR::Return);

        let function_end = self.emit_label(&end_label);
        self.replace_instruction(jump_pos, IR::Jump(function_end));

        self.leave_scope();
        self.current_function = old_function;
    }

    fn compile_if_elif_else_statement(&mut self, if_stmt: crate::frontend::parser::ast::IfStatement) {
        let end_label = self.generate_label("end_");
        let mut jump_to_end_positions = Vec::new();
        
        // Compile the main if condition
        self.compile_expression(if_stmt.condition);
        self.emit(IR::JumpIfFalse(0));
        let jump_to_next_pos = self.ir.len() - 1;
        
        // Compile the then branch
        self.compile_statement(*if_stmt.then_branch);
        
        // Jump to end after executing then branch
        self.emit(IR::Jump(0));
        jump_to_end_positions.push(self.ir.len() - 1);
        
        // Handle elif branches
        let mut current_jump_pos = jump_to_next_pos;
        for elif_branch in if_stmt.elif_branches {
            // Mark position for the previous condition to jump to
            let elif_pos = self.ir.len();
            self.replace_instruction(current_jump_pos, IR::JumpIfFalse(elif_pos));
            
            // Compile elif condition
            self.compile_expression(elif_branch.condition);
            self.emit(IR::JumpIfFalse(0));
            current_jump_pos = self.ir.len() - 1;
            
            // Compile elif body
            self.compile_statement(*elif_branch.body);
            
            // Jump to end after executing elif branch
            self.emit(IR::Jump(0));
            jump_to_end_positions.push(self.ir.len() - 1);
        }
        
        // Handle else branch
        if let Some(else_branch) = if_stmt.else_branch {
            // Mark position for the last condition to jump to
            let else_pos = self.ir.len();
            self.replace_instruction(current_jump_pos, IR::JumpIfFalse(else_pos));
            
            // Compile else body
            self.compile_statement(*else_branch);
        } else {
            // No else branch, last condition jumps to end
            let end_pos = self.ir.len();
            self.replace_instruction(current_jump_pos, IR::JumpIfFalse(end_pos));
        }
        
        // Mark the end position and patch all jumps to end
        let end_pos = self.emit_label(&end_label);
        for jump_pos in jump_to_end_positions {
            self.replace_instruction(jump_pos, IR::Jump(end_pos));
        }
    }

    fn compile_while_statement(&mut self, condition: Expression, body: Vec<Statement>) {
        let loop_label = self.generate_label("loop_");
        let continue_label = self.generate_label("continue_");
        let end_label = self.generate_label("end_");

        // Push loop context for break/continue
        self.loop_stack.push((continue_label.clone(), end_label.clone()));
        self.break_stack.push(Vec::new());
        self.continue_stack.push(Vec::new());

        let loop_start = self.emit_label(&loop_label);
        let continue_pos = self.emit_label(&continue_label);
        self.continue_positions.push(continue_pos);

        self.compile_expression(condition);
        self.emit(IR::JumpIfFalse(0));
        let jump_to_end_pos = self.ir.len() - 1;

        // Compile loop body
        self.enter_scope();
        for stmt in body {
            self.compile_statement(stmt);
        }
        self.leave_scope();

        self.emit(IR::Jump(loop_start));

        let end_pos = self.emit_label(&end_label);
        self.replace_instruction(jump_to_end_pos, IR::JumpIfFalse(end_pos));

        // Patch break and continue statements
        if let (Some(break_positions), Some(continue_positions)) = 
            (self.break_stack.pop(), self.continue_stack.pop()) {
            let actual_continue_pos = self.continue_positions.pop().unwrap_or(continue_pos);
            for pos in break_positions {
                self.replace_instruction(pos, IR::Jump(end_pos));
            }
            for pos in continue_positions {
                self.replace_instruction(pos, IR::Jump(actual_continue_pos));
            }
        }
        
        self.loop_stack.pop();
    }

    fn compile_for_statement(&mut self, variable: String, iterable: Expression, body: Vec<Statement>) {
        let loop_label = self.generate_label("for_loop_");
        let continue_label = self.generate_label("for_continue_");
        let end_label = self.generate_label("for_end_");
        let iter_var = self.generate_label("iter_");
        let index_var = self.generate_label("index_");

        // Push loop context for break/continue
        self.loop_stack.push((continue_label.clone(), end_label.clone()));
        self.break_stack.push(Vec::new());
        self.continue_stack.push(Vec::new());

        self.enter_scope();
        
        // Handle different types of iterables
        match &iterable {
            Expression::RangeExpression(range) => {
                // For range expressions like 1..10
                self.compile_expression(*range.start.clone());
                self.emit(IR::StoreVar(index_var.clone()));
                
                self.compile_expression(*range.end.clone());
                self.emit(IR::StoreVar(iter_var.clone()));
                
                let loop_start = self.emit_label(&loop_label);
                let continue_pos = self.emit_label(&continue_label);
                self.continue_positions.push(continue_pos);
                
                // Check if index < end
                self.emit(IR::LoadVar(index_var.clone()));
                self.emit(IR::LoadVar(iter_var.clone()));
                if range.inclusive {
                    self.emit(IR::LessEqual);
                } else {
                    self.emit(IR::LessThan);
                }
                self.emit(IR::JumpIfFalse(0));
                let jump_to_end_pos = self.ir.len() - 1;
                
                // Set loop variable to current index
                self.emit(IR::LoadVar(index_var.clone()));
                self.emit(IR::StoreVar(variable.clone()));
                
                // Compile loop body
                for stmt in body {
                    self.compile_statement(stmt);
                }
                
                // This is where continue should jump to - the increment part
                let actual_continue_pos = self.ir.len();
                self.continue_positions.pop(); // Remove the old position
                self.continue_positions.push(actual_continue_pos); // Add the correct position
                
                // Increment index
                self.emit(IR::LoadVar(index_var.clone()));
                self.emit(IR::PushNumber(1.0));
                self.emit(IR::Add);
                self.emit(IR::StoreVar(index_var));
                
                self.emit(IR::Jump(loop_start));
                
                let end_pos = self.emit_label(&end_label);
                self.replace_instruction(jump_to_end_pos, IR::JumpIfFalse(end_pos));
            },
            Expression::ArrayLiteral(array) => {
                // For array literals like [1, 2, 3] - simpler approach
                // Just iterate through each element directly
                for (_i, element) in array.elements.iter().enumerate() {
                    // Set loop variable to current element
                    self.compile_expression(element.clone());
                    self.emit(IR::StoreVar(variable.clone()));
                    
                    // Compile loop body for this iteration
                    for stmt in body.clone() {
                        self.compile_statement(stmt);
                    }
                }
                
                // No need for complex loop logic - we've already iterated through all elements
                let _loop_start = self.emit_label(&loop_label);
                let _continue_pos = self.emit_label(&continue_label);
                let _end_pos = self.emit_label(&end_label);
            },
            _ => {
                // For other expressions, treat as single value iteration
                self.compile_expression(iterable);
                self.emit(IR::StoreVar(variable.clone()));
                
                let _loop_start = self.emit_label(&loop_label);
                let _continue_pos = self.emit_label(&continue_label);
                
                // Compile loop body (executes once)
                for stmt in body {
                    self.compile_statement(stmt);
                }
                
                let _end_pos = self.emit_label(&end_label);
            }
        }
        
        // Patch break and continue statements
        if let (Some(break_positions), Some(continue_positions)) = 
            (self.break_stack.pop(), self.continue_stack.pop()) {
            let end_pos = self.ir.len();
            let actual_continue_pos = self.continue_positions.pop().unwrap_or(end_pos);
            for pos in break_positions {
                self.replace_instruction(pos, IR::Jump(end_pos));
            }
            for pos in continue_positions {
                self.replace_instruction(pos, IR::Jump(actual_continue_pos));
            }
        }
        
        self.leave_scope();
        self.loop_stack.pop();
    }

    fn compile_break_statement(&mut self) {
        if !self.break_stack.is_empty() {
            self.emit(IR::Jump(0)); // Placeholder, will be patched
            let pos = self.ir.len() - 1;
            if let Some(break_positions) = self.break_stack.last_mut() {
                break_positions.push(pos);
            }
        } else {
            self.errors.push("Break statement outside of loop".to_string());
        }
    }

    fn compile_continue_statement(&mut self) {
        if !self.continue_stack.is_empty() {
            self.emit(IR::Jump(0)); // Placeholder, will be patched
            let pos = self.ir.len() - 1;
            if let Some(continue_positions) = self.continue_stack.last_mut() {
                continue_positions.push(pos);
            }
        } else {
            self.errors.push("Continue statement outside of loop".to_string());
        }
    }

    fn compile_match_statement(&mut self, match_stmt: crate::frontend::parser::ast::MatchStatement) {
        // Compile the match expression
        self.compile_expression(match_stmt.expression);
        
        let mut jump_to_end_positions = Vec::new();
        
        // Compile each match arm
        for arm in match_stmt.arms {
            // Duplicate the match value for comparison
            self.emit(IR::Dup);
            
            // Compile the pattern
            self.compile_pattern(arm.pattern);
            
            // Compare the values
            self.emit(IR::Equal);
            
            // Jump to next arm if not equal
            self.emit(IR::JumpIfFalse(0));
            let jump_to_next_pos = self.ir.len() - 1;
            
            // Pop the duplicated match value since we found a match
            self.emit(IR::Pop);
            
            // Compile the arm body
            self.compile_expression(arm.body);
            self.emit(IR::Pop); // Discard result
            
            // Jump to end
            self.emit(IR::Jump(0));
            jump_to_end_positions.push(self.ir.len() - 1);
            
            // Update the jump to next arm
            let next_arm_pos = self.ir.len();
            self.replace_instruction(jump_to_next_pos, IR::JumpIfFalse(next_arm_pos));
        }
        
        // Pop the match value if no arm matched
        self.emit(IR::Pop);
        
        // Update all jumps to end
        let end_pos = self.ir.len();
        for jump_pos in jump_to_end_positions {
            self.replace_instruction(jump_pos, IR::Jump(end_pos));
        }
    }

    fn compile_pattern(&mut self, pattern: crate::frontend::parser::ast::Pattern) {
        match pattern {
            crate::frontend::parser::ast::Pattern::Literal(expr) => {
                self.compile_expression(expr);
            },
            crate::frontend::parser::ast::Pattern::Identifier(ident) => {
                self.emit(IR::LoadVar(ident.name));
            },
            crate::frontend::parser::ast::Pattern::Wildcard => {
                // Wildcard matches anything, push true
                self.emit(IR::PushBoolean(true));
            },
        }
    }

    fn compile_struct_declaration(&mut self, struct_decl: crate::frontend::parser::ast::StructDeclaration) {
        // For now, struct declarations are compile-time only
        // We register the struct type in the symbol table for future use
        if !self.clean_output {
            println!("Registering struct type: {}", struct_decl.name.name);
        }
        
        // Store struct metadata for future instantiation
        // This is a placeholder - in a full implementation, we'd store field information
        self.symbol_table.define(&format!("struct_{}", struct_decl.name.name));
    }

    fn compile_enum_declaration(&mut self, enum_decl: crate::frontend::parser::ast::EnumDeclaration) {
        // Register the enum type name as a variable so it can be used in expressions
        if !self.clean_output {
            println!("Registering enum type: {}", enum_decl.name.name);
        }
        
        // Register the enum type itself
        let enum_name = enum_decl.name.name.clone();
        self.symbol_table.define(&enum_name);
        self.emit(IR::PushString(format!("enum_{}", enum_name)));
        self.emit(IR::StoreVar(enum_name.clone()));
        
        // Store enum metadata for future instantiation
        self.symbol_table.define(&format!("enum_{}", enum_name));
    }

    fn compile_impl_block(&mut self, impl_block: crate::frontend::parser::ast::ImplBlock) {
        let type_name = impl_block.target_type.name;
        
        if !self.clean_output {
            println!("Compiling impl block for type: {}", type_name);
        }
        
        // Compile each method in the impl block
        for method in impl_block.methods {
            let method_name = format!("{}::{}", type_name, method.name.name);
            let mut parameters: Vec<String> = Vec::new();
            
            // Add parameters (including self if not static)
            for param in &method.parameters {
                parameters.push(param.name.name.clone());
            }
            
            // Compile the method as a function
            self.compile_function_declaration(method_name, parameters, method.body.statements);
        }
    }

    fn compile_expression(&mut self, expr: Expression) {
        match expr {
            Expression::Identifier(ident) => {
                self.emit(IR::LoadVar(ident.name));
            },
            Expression::StringLiteral(str_lit) => {
                self.emit(IR::PushString(str_lit.value));
            },
            Expression::IntegerLiteral(int_lit) => {
                self.emit(IR::PushNumber(int_lit.value as f64));
            },
            Expression::FloatLiteral(float_lit) => {
                self.emit(IR::PushNumber(float_lit.value));
            },
            Expression::BooleanLiteral(bool_lit) => {
                self.emit(IR::PushBoolean(bool_lit.value));
            },
            Expression::NullLiteral(_) => {
                self.emit(IR::PushNull);
            },
            Expression::CallExpression(call_expr) => {
                // Compile arguments
                for arg in &call_expr.arguments {
                    self.compile_expression(arg.clone());
                }

                // Handle different types of callees
                match *call_expr.callee {
                    Expression::Identifier(ident) => {
                        // Regular function call
                        self.emit(IR::Call(ident.name, call_expr.arguments.len()));
                    },
                    Expression::MemberExpression(member_expr) => {
                        // Method call like input().toint()
                        self.compile_method_call(member_expr, call_expr.arguments.len());
                    },
                    _ => {
                        self.errors.push("Invalid function call".to_string());
                    }
                }
            },
            Expression::BinaryExpression(bin_expr) => {
                self.compile_expression(*bin_expr.left);
                self.compile_expression(*bin_expr.right);

                match bin_expr.operator {
                    crate::frontend::parser::ast::BinaryOperator::Add => self.emit(IR::Add),
                    crate::frontend::parser::ast::BinaryOperator::Subtract => self.emit(IR::Subtract),
                    crate::frontend::parser::ast::BinaryOperator::Multiply => self.emit(IR::Multiply),
                    crate::frontend::parser::ast::BinaryOperator::Divide => self.emit(IR::Divide),
                    crate::frontend::parser::ast::BinaryOperator::Modulo => self.emit(IR::Modulo),
                    crate::frontend::parser::ast::BinaryOperator::Equal => self.emit(IR::Equal),
                    crate::frontend::parser::ast::BinaryOperator::NotEqual => self.emit(IR::NotEqual),
                    crate::frontend::parser::ast::BinaryOperator::Greater => self.emit(IR::GreaterThan),
                    crate::frontend::parser::ast::BinaryOperator::GreaterEqual => self.emit(IR::GreaterEqual),
                    crate::frontend::parser::ast::BinaryOperator::Less => self.emit(IR::LessThan),
                    crate::frontend::parser::ast::BinaryOperator::LessEqual => self.emit(IR::LessEqual),
                    crate::frontend::parser::ast::BinaryOperator::And => self.emit(IR::And),
                    crate::frontend::parser::ast::BinaryOperator::Or => self.emit(IR::Or),
                    crate::frontend::parser::ast::BinaryOperator::BitwiseAnd => self.emit(IR::BitwiseAnd),
                    crate::frontend::parser::ast::BinaryOperator::BitwiseOr => self.emit(IR::BitwiseOr),
                    crate::frontend::parser::ast::BinaryOperator::BitwiseXor => self.emit(IR::BitwiseXor),
                    crate::frontend::parser::ast::BinaryOperator::LeftShift => self.emit(IR::LeftShift),
                    crate::frontend::parser::ast::BinaryOperator::RightShift => self.emit(IR::RightShift),
                    _ => {
                        self.errors.push(format!("Unknown operator: {:?}", bin_expr.operator));
                        return;
                    }
                };
            },
            Expression::UnaryExpression(unary_expr) => {
                match unary_expr.operator {
                    crate::frontend::parser::ast::UnaryOperator::Minus => {
                        self.compile_expression(*unary_expr.operand);
                        self.emit(IR::Negate);
                    },
                    crate::frontend::parser::ast::UnaryOperator::Not => {
                        self.compile_expression(*unary_expr.operand);
                        self.emit(IR::Not);
                    },
                    crate::frontend::parser::ast::UnaryOperator::Plus => {
                        // Unary plus is a no-op, just compile the operand
                        self.compile_expression(*unary_expr.operand);
                    },
                    crate::frontend::parser::ast::UnaryOperator::BitwiseNot => {
                        self.compile_expression(*unary_expr.operand);
                        self.emit(IR::BitwiseNot);
                    },
                    crate::frontend::parser::ast::UnaryOperator::PreIncrement => {
                        // ++var: increment then use
                        if let Expression::Identifier(ident) = *unary_expr.operand {
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.emit(IR::PushNumber(1.0));
                            self.emit(IR::Add);
                            self.emit(IR::Dup); // Duplicate for return value
                            self.emit(IR::StoreVar(ident.name));
                        } else {
                            self.errors.push("Pre-increment can only be applied to variables".to_string());
                        }
                    },
                    crate::frontend::parser::ast::UnaryOperator::PostIncrement => {
                        // var++: use then increment
                        if let Expression::Identifier(ident) = *unary_expr.operand {
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.emit(IR::Dup); // Keep original value for return
                            self.emit(IR::PushNumber(1.0));
                            self.emit(IR::Add);
                            self.emit(IR::StoreVar(ident.name));
                        } else {
                            self.errors.push("Post-increment can only be applied to variables".to_string());
                        }
                    },
                    crate::frontend::parser::ast::UnaryOperator::PreDecrement => {
                        // --var: decrement then use
                        if let Expression::Identifier(ident) = *unary_expr.operand {
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.emit(IR::PushNumber(1.0));
                            self.emit(IR::Subtract);
                            self.emit(IR::Dup); // Duplicate for return value
                            self.emit(IR::StoreVar(ident.name));
                        } else {
                            self.errors.push("Pre-decrement can only be applied to variables".to_string());
                        }
                    },
                    crate::frontend::parser::ast::UnaryOperator::PostDecrement => {
                        // var--: use then decrement
                        if let Expression::Identifier(ident) = *unary_expr.operand {
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.emit(IR::Dup); // Keep original value for return
                            self.emit(IR::PushNumber(1.0));
                            self.emit(IR::Subtract);
                            self.emit(IR::StoreVar(ident.name));
                        } else {
                            self.errors.push("Post-decrement can only be applied to variables".to_string());
                        }
                    }
                };
            },
            Expression::InterpolatedString(interp_str) => {
                self.compile_interpolated_string(interp_str);
            },
            Expression::RangeExpression(range_expr) => {
                // For range expressions like 1..10, we create a simple range object
                // This is mainly used in for loops, so we'll store start and end
                self.compile_expression(*range_expr.start);
                self.compile_expression(*range_expr.end);
                self.emit(IR::PushBoolean(range_expr.inclusive)); // Store inclusivity flag
                // Create a simple range representation on stack
                self.emit(IR::Call("create_range".to_string(), 3)); // start, end, inclusive
            },
            Expression::ArrayLiteral(array_lit) => {
                // Compile all array elements
                for element in &array_lit.elements {
                    self.compile_expression(element.clone());
                }
                // Create array with the specified number of elements
                self.emit(IR::CreateArray(array_lit.elements.len()));
            },
            Expression::MapLiteral(map_lit) => {
                // Compile all map key-value pairs
                for pair in &map_lit.pairs {
                    self.compile_expression(pair.key.clone());
                    self.compile_expression(pair.value.clone());
                }
                // Create map with the specified number of pairs
                self.emit(IR::CreateMap(map_lit.pairs.len()));
            },
            Expression::StructInstantiation(struct_inst) => {
                // For now, compile struct instantiation as a map-like structure
                // Push struct type name first
                self.emit(IR::PushString(struct_inst.name.name.clone()));
                
                // Compile all field values
                for field in &struct_inst.fields {
                    self.emit(IR::PushString(field.name.name.clone())); // field name
                    self.compile_expression(field.value.clone()); // field value
                }
                
                // Create struct with the specified number of fields
                self.emit(IR::CreateMap(struct_inst.fields.len() + 1)); // +1 for type name
            },
            Expression::AssignmentExpression(assign_expr) => {
                // Handle assignment to identifier
                if let Expression::Identifier(ident) = *assign_expr.left {
                    match assign_expr.operator {
                        crate::frontend::parser::ast::AssignmentOperator::Assign => {
                            // Simple assignment: var = value
                            self.compile_expression(*assign_expr.right);
                            self.emit(IR::StoreVar(ident.name));
                        },
                        crate::frontend::parser::ast::AssignmentOperator::AddAssign => {
                            // Addition assignment: var += value
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.compile_expression(*assign_expr.right);
                            self.emit(IR::Add);
                            self.emit(IR::StoreVar(ident.name));
                        },
                        crate::frontend::parser::ast::AssignmentOperator::SubtractAssign => {
                            // Subtraction assignment: var -= value
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.compile_expression(*assign_expr.right);
                            self.emit(IR::Subtract);
                            self.emit(IR::StoreVar(ident.name));
                        },
                        crate::frontend::parser::ast::AssignmentOperator::MultiplyAssign => {
                            // Multiplication assignment: var *= value
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.compile_expression(*assign_expr.right);
                            self.emit(IR::Multiply);
                            self.emit(IR::StoreVar(ident.name));
                        },
                        crate::frontend::parser::ast::AssignmentOperator::DivideAssign => {
                            // Division assignment: var /= value
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.compile_expression(*assign_expr.right);
                            self.emit(IR::Divide);
                            self.emit(IR::StoreVar(ident.name));
                        },
                        crate::frontend::parser::ast::AssignmentOperator::ModuloAssign => {
                            // Modulo assignment: var %= value
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.compile_expression(*assign_expr.right);
                            self.emit(IR::Modulo);
                            self.emit(IR::StoreVar(ident.name));
                        },
                        crate::frontend::parser::ast::AssignmentOperator::BitwiseAndAssign => {
                            // Bitwise AND assignment: var &= value
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.compile_expression(*assign_expr.right);
                            self.emit(IR::BitwiseAnd);
                            self.emit(IR::StoreVar(ident.name));
                        },
                        crate::frontend::parser::ast::AssignmentOperator::BitwiseOrAssign => {
                            // Bitwise OR assignment: var |= value
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.compile_expression(*assign_expr.right);
                            self.emit(IR::BitwiseOr);
                            self.emit(IR::StoreVar(ident.name));
                        },
                        crate::frontend::parser::ast::AssignmentOperator::BitwiseXorAssign => {
                            // Bitwise XOR assignment: var ^= value
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.compile_expression(*assign_expr.right);
                            self.emit(IR::BitwiseXor);
                            self.emit(IR::StoreVar(ident.name));
                        },
                        crate::frontend::parser::ast::AssignmentOperator::LeftShiftAssign => {
                            // Left shift assignment: var <<= value
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.compile_expression(*assign_expr.right);
                            self.emit(IR::LeftShift);
                            self.emit(IR::StoreVar(ident.name));
                        },
                        crate::frontend::parser::ast::AssignmentOperator::RightShiftAssign => {
                            // Right shift assignment: var >>= value
                            self.emit(IR::LoadVar(ident.name.clone()));
                            self.compile_expression(*assign_expr.right);
                            self.emit(IR::RightShift);
                            self.emit(IR::StoreVar(ident.name));
                        },
                    }
                } else {
                    self.errors.push("Invalid assignment target".to_string());
                }
            },
            Expression::MethodCallExpression(method_call) => {
                // Compile the object being called on
                self.compile_expression(*method_call.object);
                
                // Compile arguments
                for arg in &method_call.arguments {
                    self.compile_expression(arg.clone());
                }
                
                // Call the method (object is already on stack as first argument)
                self.emit(IR::MethodCall(method_call.method.name, method_call.arguments.len() + 1)); // +1 for self
            },
            Expression::SelfExpression(_) => {
                // Load the 'self' variable from current scope
                self.emit(IR::LoadVar("self".to_string()));
            },
            Expression::MemberExpression(member_expr) => {
                // Check if this is module member access (module.constant or module.variable)
                if let Expression::Identifier(module_ident) = &*member_expr.object {
                    let qualified_name = format!("{}.{}", module_ident.name, member_expr.property.name);
                    
                    // Check if this qualified name exists in our symbol table (imported from module)
                    if self.symbol_table.resolve(&qualified_name).is_some() {
                        // This is a module member access: load the qualified variable/constant
                        self.emit(IR::LoadVar(qualified_name));
                        return;
                    }
                    
                    // Check if this looks like an enum type (starts with uppercase)
                    if module_ident.name.chars().next().unwrap_or('a').is_uppercase() {
                        // This is likely enum variant access: EnumType.Variant
                        let enum_variant = format!("{}::{}", module_ident.name, member_expr.property.name);
                        self.emit(IR::PushString(enum_variant));
                        return;
                    }
                }
                
                // Regular member access: object.property
                self.compile_expression(*member_expr.object);
                self.emit(IR::PushString(member_expr.property.name));
                self.emit(IR::GetKey); // Use map-like access for structs
            },
            Expression::ModuleCallExpression(module_call) => {
                // Compile arguments
                for arg in &module_call.arguments {
                    self.compile_expression(arg.clone());
                }
                
                // Create a qualified function name: module.function
                let qualified_name = format!("{}.{}", module_call.module.name, module_call.function.name);
                
                // Call the function with the qualified name
                self.emit(IR::Call(qualified_name, module_call.arguments.len()));
            },
            _ => {
                // Handle other expression types as needed
                if !self.clean_output {
                    println!("Unhandled expression type: {:?}", std::mem::discriminant(&expr));
                }
                self.emit(IR::PushNull);
            }
        }
    }
    
    /// Compile method call like input().toint()
    fn compile_method_call(&mut self, member_expr: crate::frontend::parser::ast::MemberExpression, _arg_count: usize) {
        // First, compile the object (e.g., input())
        self.compile_expression(*member_expr.object);
        
        // Then handle the method call based on the method name
        let method_name = &member_expr.property.name;
        
        match method_name.as_str() {
            "toint" => {
                // Convert the value on stack to integer
                self.emit(IR::Call("toint".to_string(), 1)); // 1 argument (the object itself)
            },
            "tofloat" => {
                // Convert the value on stack to float
                self.emit(IR::Call("tofloat".to_string(), 1));
            },
            "tostr" => {
                // Convert the value on stack to string
                self.emit(IR::Call("tostr".to_string(), 1));
            },
            "tobool" => {
                // Convert the value on stack to boolean
                self.emit(IR::Call("tobool".to_string(), 1));
            },
            _ => {
                self.errors.push(format!("Unknown method: {}", method_name));
            }
        }
    }
    
    /// Compile interpolated string (f-string) into IR
    fn compile_interpolated_string(&mut self, interp_str: InterpolatedString) {
        if interp_str.parts.is_empty() {
            self.emit(IR::PushString("".to_string()));
            return;
        }
        
        // Compile each part and concatenate them
        let mut first = true;
        for part in interp_str.parts {
            match part {
                InterpolationPart::Text(text) => {
                    self.emit(IR::PushString(text));
                },
                InterpolationPart::Expression(expr) => {
                    self.compile_expression(expr);
                    // Convert the expression result to string if needed
                    // For now, assume it's already a string or will be converted during runtime
                }
            }
            
            // If this is not the first part, concatenate with the previous result
            if !first {
                self.emit(IR::Add); // String concatenation
            }
            first = false;
        }
    }

    /// Execute the compiled IR
    pub fn execute(&self) -> Result<(), String> {
        let mut runtime = Runtime::new();
        runtime.set_clean_output(self.clean_output);
        
        // Register function parameter names with runtime
        for (func_name, params) in &self.function_param_names {
            runtime.register_function_params(func_name.clone(), params.clone());
        }
        
        runtime.execute(&self.ir)
    }

    /// Write compiled machine code to file
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let machine_code = self.generate_machine_code()?;
        fs::write(path, machine_code).map_err(|e| format!("Failed to write file: {}", e))
    }

    fn generate_machine_code(&self) -> Result<Vec<u8>, String> {
        let mut code = Vec::new();
        
        // Simple bytecode generation
        for ir in &self.ir {
            match ir {
                IR::PushNumber(_) => code.push(0x01),
                IR::PushString(_) => code.push(0x02),
                IR::PushBoolean(_) => code.push(0x03),
                IR::PushNull => code.push(0x04),
                IR::Pop => code.push(0x05),
                IR::StoreVar(_) => code.push(0x08),
                IR::LoadVar(_) => code.push(0x09),
                IR::Add => code.push(0x0A),
                IR::Subtract => code.push(0x0B),
                IR::Multiply => code.push(0x0C),
                IR::Divide => code.push(0x0D),
                IR::Call(_, _) => code.push(0x1E),
                IR::Return => code.push(0x1F),
                IR::Print => code.push(0x20),
                _ => code.push(0x00), // NOP for unhandled instructions
            }
        }
        
        Ok(code)
    }
}
