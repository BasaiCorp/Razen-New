// src/backend/execution/compiler.rs
//! Clean compiler implementation based on the proven old design

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::frontend::parser::ast::{Program, Statement, Expression, InterpolatedString, InterpolationPart};
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
    _break_stack: Vec<Vec<usize>>,
    _continue_stack: Vec<Vec<usize>>,
    label_counter: usize,
    clean_output: bool,
    pub errors: Vec<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            ir: Vec::new(),
            symbol_table: SymbolTable::new(),
            function_table: FunctionTable::new(),
            function_param_names: HashMap::new(),
            current_function: None,
            _break_stack: Vec::new(),
            _continue_stack: Vec::new(),
            label_counter: 0,
            clean_output: false,
            errors: Vec::new(),
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

        // First pass: register all functions
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

        // Second pass: compile all statements
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
            _ => {
                // Handle other statement types as needed
                if !self.clean_output {
                    println!("Unhandled statement type");
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
        let end_label = self.generate_label("end_");

        let loop_start = self.emit_label(&loop_label);

        self.compile_expression(condition);
        self.emit(IR::JumpIfFalse(0));
        let jump_to_end_pos = self.ir.len() - 1;

        for stmt in body {
            self.compile_statement(stmt);
        }

        self.emit(IR::Jump(loop_start));

        let end_pos = self.emit_label(&end_label);
        self.replace_instruction(jump_to_end_pos, IR::JumpIfFalse(end_pos));
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

                // Get function name
                if let Expression::Identifier(ident) = *call_expr.callee {
                    self.emit(IR::Call(ident.name, call_expr.arguments.len()));
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
                    _ => {
                        self.errors.push(format!("Unknown operator: {:?}", bin_expr.operator));
                        return;
                    }
                };
            },
            Expression::UnaryExpression(unary_expr) => {
                self.compile_expression(*unary_expr.operand);
                
                match unary_expr.operator {
                    crate::frontend::parser::ast::UnaryOperator::Minus => self.emit(IR::Negate),
                    crate::frontend::parser::ast::UnaryOperator::Not => self.emit(IR::Not),
                    _ => {
                        self.errors.push(format!("Unknown unary operator: {:?}", unary_expr.operator));
                        return;
                    }
                };
            },
            Expression::InterpolatedString(interp_str) => {
                self.compile_interpolated_string(interp_str);
            },
            _ => {
                // Handle other expression types as needed
                self.emit(IR::PushNull);
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
