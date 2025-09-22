use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use std::{thread, time::Duration};

use crate::ast::{Program, Statement, Expression};
use crate::parser::Parser;
use crate::value::Value as RazenValue;
use crate::library;

// Intermediate representation for code generation
#[derive(Debug, Clone)]
pub enum IR {
    // Stack operations
    PushNumber(f64),
    PushString(String),
    PushBoolean(bool),
    PushNull,
    Pop,
    Dup,
    Swap,

    // Exception handling
    SetupTryCatch,
    ClearTryCatch,
    ThrowException,

    // Memory operations
    StoreVar(String),
    LoadVar(String),
    SetGlobal(String),  // Global variable operations

    // Arithmetic operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    FloorDiv,
    Negate,

    // Comparison operations
    Equal,
    NotEqual,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,

    // Logical operations
    And,
    Or,
    Not,

    // Control flow
    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    Call(String, usize),  // function name, arg count
    Return,

    // I/O operations
    Print,
    ReadInput,
    Exit,

    // Array operations
    CreateArray(usize),
    GetIndex,
    SetIndex,

    // Map operations
    CreateMap(usize),
    GetKey,
    SetKey,

    // Function definition
    DefineFunction(String, usize),  // function name, address

    // Labels for jumps
    Label(String),

    // New instructions
    Sleep,

    // Library call
    LibraryCall(String, String, usize),  // library name, function name, arg count
}

// Symbol table for variable and function tracking
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

// Function table for tracking function definitions
#[derive(Debug, Clone)]
struct FunctionTable {
    functions: HashMap<String, usize>,  // function name -> address in IR
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

// Compiler for translating AST to machine code
pub struct Compiler {
    pub ir: Vec<IR>,
    symbol_table: SymbolTable,
    function_table: FunctionTable,
    function_param_names: HashMap<String, Vec<String>>,
    current_function: Option<String>,
    break_stack: Vec<Vec<usize>>,    // Stack of break statement positions for nested loops
    continue_stack: Vec<Vec<usize>>, // Stack of continue statement positions for nested loops
    label_counter: usize,            // Counter for generating unique labels
    clean_output: bool,              // Flag to only show program output
    errors: Vec<String>,            // Compilation errors
    variable_types: HashMap<String, String>, // Track variable types (name -> type)
    in_show_statement: bool,        // Flag to track if we're inside a show statement
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
            label_counter: 0,
            clean_output: false,
            errors: Vec::new(),
            variable_types: HashMap::new(),
            in_show_statement: false,
        }
    }

    // Set clean output mode
    pub fn set_clean_output(&mut self, clean: bool) {
        self.clean_output = clean;
    }

    // Helper methods for type checking
    fn is_number_expression(&self, expr: &Expression) -> bool {
        match expr {
            Expression::NumberLiteral(_) => true,
            Expression::Identifier(name) => {
                // Check if the identifier refers to a variable of type 'let'
                self.variable_types.get(name).map_or(false, |var_type| var_type == "let")
            },
            Expression::PrefixExpression { right, .. } => self.is_number_expression(right),
            Expression::InfixExpression { left, right, operator, .. } => {
                // Most infix operations with numbers result in numbers
                match operator.as_str() {
                    "+" | "-" | "*" | "/" | "%" | "**" | "//" => {
                        self.is_number_expression(left) && self.is_number_expression(right)
                    },
                    _ => false,
                }
            },
            Expression::CallExpression { function, .. } => {
                // Check if it's a function known to return numbers
                if let Expression::Identifier(name) = &**function {
                    matches!(name.as_str(),
                        "plus" | "minus" | "times" | "by" | "mod" | "power" |
                        "round" | "sqrt" | "abs" | "size" | "count")
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    fn is_string_expression(&self, expr: &Expression) -> bool {
        match expr {
            Expression::StringLiteral(_) => true,
            Expression::Identifier(name) => {
                // Check if the identifier refers to a variable of type 'take'
                self.variable_types.get(name).map_or(false, |var_type| var_type == "take")
            },
            Expression::InfixExpression { left, right, operator, .. } => {
                // String concatenation
                operator == "+" && (self.is_string_expression(left) || self.is_string_expression(right))
            },
            Expression::CallExpression { function, .. } => {
                // Check if it's a function known to return strings
                if let Expression::Identifier(name) = &**function {
                    matches!(name.as_str(),
                        "join" | "big" | "small" | "trim" | "replace" |
                        "date" | "read_file")
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    fn is_boolean_expression(&self, expr: &Expression) -> bool {
        match expr {
            Expression::BooleanLiteral(_) => true,
            Expression::Identifier(name) => {
                // Check if the identifier refers to a variable of type 'hold'
                self.variable_types.get(name).map_or(false, |var_type| var_type == "hold")
            },
            Expression::PrefixExpression { operator, right } => {
                operator == "!" && self.is_boolean_expression(right)
            },
            Expression::InfixExpression { operator, .. } => {
                // Comparison and logical operators return booleans
                matches!(operator.as_str(),
                    "==" | "!=" | ">" | ">=" | "<" | "<=" | "&&" | "||")
            },
            Expression::CallExpression { function, .. } => {
                // Check if it's a function known to return booleans
                if let Expression::Identifier(name) = &**function {
                    matches!(name.as_str(), "contains")
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        match Parser::from_file(path) {
            Ok(mut parser) => {
                let program = parser.parse_program();
                if !parser.get_errors().is_empty() {
                    return Err(format!("Parser errors: {:?}", parser.get_errors()));
                }

                let mut compiler = Compiler::new();

                // Check for clean output flag in environment
                if std::env::args().any(|arg| arg == "--clean-output") {
                    compiler.set_clean_output(true);
                }

                compiler.compile_program(program);
                Ok(compiler)
            },
            Err(e) => Err(e),
        }
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
        self.ir[pos] = code;
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

    fn enter_loop(&mut self) {
        self.break_stack.push(Vec::new());
        self.continue_stack.push(Vec::new());
    }

    fn leave_loop(&mut self, loop_start: usize, loop_end: usize) {
        // Patch all break statements
        if let Some(breaks) = self.break_stack.pop() {
            for pos in breaks {
                self.replace_instruction(pos, IR::Jump(loop_end));
            }
        }

        // Patch all continue statements
        if let Some(continues) = self.continue_stack.pop() {
            for pos in continues {
                self.replace_instruction(pos, IR::Jump(loop_start));
            }
        }
    }

    fn define_builtins(&mut self) {
        // Define built-in functions like print, math operations, etc.
        self.symbol_table.define("print");
        self.symbol_table.define("read");
        self.symbol_table.define("len");
        self.symbol_table.define("append");
        self.symbol_table.define("remove");

        // Module system built-ins
        self.symbol_table.define("__import_symbol");
        self.symbol_table.define("__import_module");
        self.symbol_table.define("__export_symbol");

        // Developer tools built-ins
        self.symbol_table.define("__debug");
        self.symbol_table.define("__assert");
        self.symbol_table.define("__assert_with_message");
        self.symbol_table.define("__trace");

        // Standard library functions
        self.symbol_table.define("floor");       // Math functions
        self.symbol_table.define("ceil");
        self.symbol_table.define("round");
        self.symbol_table.define("sin");
        self.symbol_table.define("cos");
        self.symbol_table.define("tan");
        self.symbol_table.define("sqrt");
        self.symbol_table.define("random");

        self.symbol_table.define("format");      // String functions
        self.symbol_table.define("substring");
        self.symbol_table.define("uppercase");
        self.symbol_table.define("lowercase");
        self.symbol_table.define("trim");
        self.symbol_table.define("replace");

        self.symbol_table.define("map");         // Array functions
        self.symbol_table.define("filter");
        self.symbol_table.define("reduce");
        self.symbol_table.define("join");
        self.symbol_table.define("split");

        self.symbol_table.define("now");         // Time functions
        self.symbol_table.define("format_date");
        self.symbol_table.define("sleep");

        self.symbol_table.define("parse_json");  // JSON functions
        self.symbol_table.define("stringify_json");

        self.symbol_table.define("read_file");   // File I/O functions
        self.symbol_table.define("write_file");
        self.symbol_table.define("append_file");

        self.symbol_table.define("get_args");    // Scripting functions
        self.symbol_table.define("get_env");
        self.symbol_table.define("set_env");
        self.symbol_table.define("run_command");
        self.symbol_table.define("exit_with");

        self.symbol_table.define("create_dir");  // File system functions
        self.symbol_table.define("remove_dir");
        self.symbol_table.define("list_dir");
        self.symbol_table.define("is_file");
        self.symbol_table.define("is_dir");
        self.symbol_table.define("file_exists");
        self.symbol_table.define("copy_file");
        self.symbol_table.define("move_file");
        self.symbol_table.define("delete_file");

        self.symbol_table.define("join_path");   // Path manipulation functions
        self.symbol_table.define("basename");
        self.symbol_table.define("dirname");
        self.symbol_table.define("absolute_path");
    }

    pub fn compile_program(&mut self, program: Program) {
        // Define built-in functions
        self.define_builtins();

        // First pass: register all functions
        for stmt in &program.statements {
            if let Statement::FunctionDeclaration { name, .. } = stmt {
                self.symbol_table.define(name);
            }
        }

        // Second pass: compile all statements
        for stmt in program.statements {
            self.compile_statement(stmt);
        }
    }

    fn compile_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::VariableDeclaration { var_type, name, value } => {
                self.compile_variable_declaration(var_type, name, value);
            },
            Statement::FunctionDeclaration { name, parameters, body } => {
                self.compile_function_declaration(name, parameters, body);
            },
            Statement::ReturnStatement { value } => {
                self.compile_return_statement(value);
            },
            Statement::ExpressionStatement { expression } => {
                self.compile_expression(expression);
                // Discard the result of the expression
                self.emit(IR::Pop);
            },
            Statement::BlockStatement { statements } => {
                self.compile_block_statement(statements);
            },
            Statement::IfStatement { condition, consequence, alternative } => {
                self.compile_if_statement(condition, consequence, alternative);
            },
            Statement::WhileStatement { condition, body } => {
                self.compile_while_statement(condition, body);
            },
            Statement::ForStatement { iterator, iterable, body } => {
                self.compile_for_statement(iterator, iterable, body);
            },
            Statement::BreakStatement => {
                self.compile_break_statement();
            },
            Statement::ContinueStatement => {
                self.compile_continue_statement();
            },
            Statement::ShowStatement { value, color } => {
                self.compile_show_statement(value, color);
            },
            Statement::LoadStatement { cycles, block } => {
                self.compile_load_statement(cycles, block);
            },
            Statement::TryStatement { try_block, catch_param, catch_block, finally_block } => {
                self.compile_try_statement(try_block, catch_param, catch_block, finally_block);
            },
            Statement::ThrowStatement { value } => {
                self.compile_throw_statement(value);
            },
            Statement::ReadStatement { name } => {
                self.compile_read_statement(name);
            },
            Statement::ExitStatement => {
                self.compile_exit_statement();
            },
            Statement::DocumentTypeDeclaration { doc_type } => {
                self.compile_document_type_declaration(doc_type);
            },
            // Module system
            Statement::ModuleImport { names, alias, source } => {
                self.compile_module_import(names, alias, source);
            },
            Statement::ModuleExport { name } => {
                self.compile_module_export(name);
            },
            // Developer tools
            Statement::DebugStatement { value } => {
                self.compile_debug_statement(value);
            },
            Statement::AssertStatement { condition, message } => {
                self.compile_assert_statement(condition, message);
            },
            Statement::TraceStatement { value } => {
                self.compile_trace_statement(value);
            },
            // OOP (Section 12)
            Statement::ClassDeclaration { name, body } => {
                self.compile_class_declaration(name, body);
            },
            Statement::FinalClassDeclaration { name, body } => {
                self.compile_final_class_declaration(name, body);
            },
            // Performance and Type Safety
            Statement::ConstDeclaration { name, value } => {
                self.compile_const_declaration(name, value);
            },
            Statement::EnumDeclaration { name, variants } => {
                self.compile_enum_declaration(name, variants);
            },
            Statement::InlineFunctionDeclaration { name, parameters, body } => {
                self.compile_inline_function_declaration(name, parameters, body);
            },
            Statement::VolatileDeclaration { var_type, name, value } => {
                self.compile_volatile_declaration(var_type, name, value);
            },
            // API Integration (Section 13)
            Statement::ApiDeclaration { name, url } => {
                self.compile_api_declaration(name, url);
            },
            Statement::ApiCall { name, body } => {
                self.compile_api_call(name, body);
            },
            // Connect and From (Section 14)
            Statement::ConnectStatement { name, url, options } => {
                self.compile_connect_statement(name, url, options);
            },
            // Import/Export (Section 15)
            Statement::ImportStatement { imports, path } => {
                self.compile_import_statement(imports, path);
            },
            // Libraries (Section 16)
            Statement::LibStatement { name } => {
                self.compile_lib_statement(name);
            },
            // Compiler Construction (Section 17)
            Statement::GrammarStatement { name, properties } => {
                self.compile_grammar_statement(name, properties);
            },
            Statement::TokenStatement { name, pattern } => {
                self.compile_token_statement(name, pattern);
            },
            Statement::LexerStatement { name, config } => {
                self.compile_lexer_statement(name, config);
            },
            Statement::ParserStatement { name, config } => {
                self.compile_parser_statement(name, config);
            },
            Statement::NodeStatement { name, properties } => {
                self.compile_node_statement(name, properties);
            },
            Statement::RuleStatement { name, production, node_type } => {
                self.compile_rule_statement(name, production, node_type);
            },
            Statement::VisitorStatement { name, methods } => {
                self.compile_visitor_statement(name, methods);
            },
            Statement::SymbolStatement { name, attributes } => {
                self.compile_symbol_statement(name, attributes);
            },
            Statement::ScopeStatement { name, parent } => {
                self.compile_scope_statement(name, parent);
            },
            Statement::TypeStatement { name, operations } => {
                self.compile_type_statement(name, operations);
            },
            Statement::IRStatement { name, opcode, operands } => {
                self.compile_ir_statement(name, opcode, operands);
            },
            Statement::CodeGenStatement { name, target, instructions } => {
                self.compile_codegen_statement(name, target, instructions);
            },
            Statement::OptimizeStatement { name, description, passes } => {
                self.compile_optimize_statement(name, description, passes);
            },
            Statement::TargetStatement { name, properties } => {
                self.compile_target_statement(name, properties);
            },
            Statement::AttributeStatement { name, values } => {
                self.compile_attribute_statement(name, values);
            },
        }
    }

    fn compile_variable_declaration(&mut self, var_type: String, name: String, value: Option<Expression>) {
        // Define the variable in the symbol table
        self.symbol_table.define(&name);

        // Store the variable type for future type checking
        self.variable_types.insert(name.clone(), var_type.clone());

        // Compile the initializer expression if it exists
        if let Some(expr) = value.clone() {
            // Type checking based on variable type
            match var_type.as_str() {
                // Numeric types
                "num" => {
                    // Check that the value is a number
                    if !self.is_number_expression(&expr) {
                        self.errors.push(format!("Type error: '{}' variables can only be used with numeric values, but '{}' was assigned a non-numeric value", var_type, name));
                    }
                },
                // String types
                "str" => {
                    // Check that the value is a string
                    if !self.is_string_expression(&expr) {
                        self.errors.push(format!("Type error: '{}' variables can only be used with string values, but '{}' was assigned a non-string value", var_type, name));
                    }
                },
                // Boolean types
                "bool" => {
                    // Check that the value is a boolean
                    if !self.is_boolean_expression(&expr) {
                        self.errors.push(format!("Type error: '{}' variables can only be used with boolean values, but '{}' was assigned a non-boolean value", var_type, name));
                    }
                },
                // Generic types - no type checking needed
                "var" | "list" | "arr" | "map" | "store" | "box" | "ref" |
                "len" | "key" | "value" | "append" | "remove" => {
                    // These can hold any type, so no type checking needed
                },
                _ => {
                    // For new variable types not explicitly handled
                    // For now, we don't do type checking on these
                }
            }

            self.compile_expression(expr);
        } else {
            // If no initializer, push null as the default value
            self.emit(IR::PushNull);
        }

        // Store the value in the variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_function_declaration(&mut self, name: String, parameters: Vec<String>, body: Vec<Statement>) {
        // Save the current function name
        let old_function = self.current_function.clone();
        self.current_function = Some(name.clone());

        // Generate a unique label for the function
        let function_label = self.generate_label("function_");
        let end_label = self.generate_label("end_");

        // Emit a jump to skip over the function body
        let jump_pos = self.emit(IR::Jump(0));

        // Mark the start of the function
        let function_start = self.emit_label(&function_label);

        // Register the function in the function table
        self.function_table.define(&name, function_start);
        // Store parameter names
        self.function_param_names.insert(name.clone(), parameters.clone());

        // Add explicit function definition instruction
        // This is what allows the function to be called at runtime
        self.emit(IR::DefineFunction(name.clone(), function_start));

        // Create a new scope for the function body
        self.enter_scope();

        // Define parameters in the function's scope
        for param in &parameters { // Iterate over a reference to parameters
            self.symbol_table.define(&param);
        }

        // Compile the function body
        for stmt in body {
            self.compile_statement(stmt);
        }

        // Ensure the function returns (even if there's no explicit return)
        self.emit(IR::PushNull);
        self.emit(IR::Return);

        // Mark the end of the function
        let function_end = self.emit_label(&end_label);

        // Update the jump instruction to skip over the function body
        self.replace_instruction(jump_pos, IR::Jump(function_end));

        // Leave the function's scope
        self.leave_scope();

        // Restore the previous function name
        self.current_function = old_function;

        if !self.clean_output {
            println!("[Compiler] Defined function {} at address {}", name, function_start);
        }
    }

    fn compile_return_statement(&mut self, value: Option<Expression>) {
        // Compile the return value if it exists
        if let Some(expr) = value {
            self.compile_expression(expr);
        } else {
            // If no return value, push null
            self.emit(IR::PushNull);
        }

        // Emit the return instruction
        self.emit(IR::Return);
    }

    fn compile_block_statement(&mut self, statements: Vec<Statement>) {
        // Create a new scope for the block
        self.enter_scope();

        // Compile each statement in the block
        for stmt in statements {
            self.compile_statement(stmt);
        }

        // Leave the block's scope
        self.leave_scope();
    }

    fn compile_if_statement(&mut self, condition: Expression, consequence: Vec<Statement>, alternative: Option<Vec<Statement>>) {
        // Generate labels for the if statement
        let else_label = self.generate_label("else_");
        let end_label = self.generate_label("end_");

        // Compile the condition
        self.compile_expression(condition);

        // Emit a conditional jump to the else branch
        self.emit(IR::JumpIfFalse(0)); // Placeholder for else_label
        let jump_to_else_pos = self.ir.len() - 1;

        // Compile the consequence (if branch)
        for stmt in consequence {
            self.compile_statement(stmt);
        }

        // Emit a jump to the end of the if statement
        self.emit(IR::Jump(0)); // Placeholder for end_label
        let jump_to_end_pos = self.ir.len() - 1;

        // Mark the start of the else branch
        let else_pos = self.emit_label(&else_label);

        // Update the jump to else instruction
        self.replace_instruction(jump_to_else_pos, IR::JumpIfFalse(else_pos));

        // Compile the alternative (else branch) if it exists
        if let Some(alt) = alternative {
            for stmt in alt {
                self.compile_statement(stmt);
            }
        }

        // Mark the end of the if statement
        let end_pos = self.emit_label(&end_label);

        // Update the jump to end instruction
        self.replace_instruction(jump_to_end_pos, IR::Jump(end_pos));
    }

    fn compile_while_statement(&mut self, condition: Expression, body: Vec<Statement>) {
        // Generate labels for the while loop
        let loop_label = self.generate_label("loop_");
        let end_label = self.generate_label("end_");

        // Mark the start of the loop
        let loop_start = self.emit_label(&loop_label);

        // Enter the loop context for break/continue statements
        self.enter_loop();

        // Compile the condition
        self.compile_expression(condition);

        // Emit a conditional jump to the end of the loop
        self.emit(IR::JumpIfFalse(0)); // Placeholder for end_label
        let jump_to_end_pos = self.ir.len() - 1;

        // Compile the loop body
        for stmt in body {
            self.compile_statement(stmt);
        }

        // Emit a jump back to the start of the loop
        self.emit(IR::Jump(loop_start));

        // Mark the end of the loop
        let end_pos = self.emit_label(&end_label);

        // Update the jump to end instruction
        self.replace_instruction(jump_to_end_pos, IR::JumpIfFalse(end_pos));

        // Leave the loop context and patch break/continue statements
        self.leave_loop(loop_start, end_pos);
    }

    fn compile_for_statement(&mut self, iterator: String, iterable: Expression, body: Vec<Statement>) {
        // Generate labels for the for loop
        let loop_label = self.generate_label("for_loop_");
        let end_label = self.generate_label("for_end_");

        // Create a new scope for the loop
        self.enter_scope();

        // Define the iterator variable
        self.symbol_table.define(&iterator);

        // Compile the iterable expression
        self.compile_expression(iterable);

        // Create a temporary index variable
        let index_var = format!("__index_{}", self.generate_label(""));
        self.symbol_table.define(&index_var);

        // Initialize the index to 0
        self.emit(IR::PushNumber(0.0));
        self.emit(IR::StoreVar(index_var.clone()));

        // Mark the start of the loop
        let loop_start = self.emit_label(&loop_label);

        // Enter the loop context for break/continue statements
        self.enter_loop();

        // Check if the index is less than the length of the iterable
        self.emit(IR::LoadVar(index_var.clone()));
        self.emit(IR::Dup); // Duplicate the index for the array access
        self.emit(IR::GetIndex); // Get the element at the current index

        // Store the current element in the iterator variable
        self.emit(IR::StoreVar(iterator));

        // Increment the index
        self.emit(IR::LoadVar(index_var.clone()));
        self.emit(IR::PushNumber(1.0));
        self.emit(IR::Add);
        self.emit(IR::StoreVar(index_var.clone()));

        // Compile the loop body
        for stmt in body {
            self.compile_statement(stmt);
        }

        // Emit a jump back to the start of the loop
        self.emit(IR::Jump(loop_start));

        // Mark the end of the loop
        let end_pos = self.emit_label(&end_label);

        // Leave the loop context and patch break/continue statements
        self.leave_loop(loop_start, end_pos);

        // Leave the loop's scope
        self.leave_scope();
    }

    fn compile_break_statement(&mut self) {
        // Check if we're inside a loop
        if self.break_stack.is_empty() {
            panic!("Break statement outside of loop");
        }

        // Emit a jump to the end of the loop (will be patched later)
        let break_pos = self.emit(IR::Jump(0)); // Placeholder

        // Add the break position to the current loop's break stack
        if let Some(breaks) = self.break_stack.last_mut() {
            breaks.push(break_pos);
        }
    }

    fn compile_continue_statement(&mut self) {
        // Check if we're inside a loop
        if self.continue_stack.is_empty() {
            panic!("Continue statement outside of loop");
        }

        // Emit a jump to the start of the loop (will be patched later)
        let continue_pos = self.emit(IR::Jump(0)); // Placeholder

        // Add the continue position to the current loop's continue stack
        if let Some(continues) = self.continue_stack.last_mut() {
            continues.push(continue_pos);
        }
    }

    fn compile_show_statement(&mut self, value: Expression, color: Option<String>) {
        // Set the flag that we're inside a show statement
        let old_in_show = self.in_show_statement;
        self.in_show_statement = true;

        // If color is specified, add ANSI color code before printing the value
        if let Some(color_name) = &color {
            // Get the color code using our color utility
            let color_code = crate::functions::colorlib::get_color_code(color_name);
            self.emit(IR::PushString(color_code));
            self.emit(IR::Print);
        }

        // Compile the expression to be shown
        self.compile_expression(value);

        // Emit print instruction
        self.emit(IR::Print);

        // Reset color if it was set
        if color.is_some() {
            let reset_code = crate::functions::colorlib::get_color_code("reset");
            self.emit(IR::PushString(reset_code));
            self.emit(IR::Print);
        }

        // Add a newline character if we're not inside a load statement
        if !old_in_show {
            // Just one newline at the end of normal show statements
            self.emit(IR::PushString("\n".to_string()));
            self.emit(IR::Print);
        }
        // Note: If we are inside a load statement (old_in_show is true),
        // we don't add any newlines since the load statement will handle the formatting

        // Reset the flag
        self.in_show_statement = old_in_show;

        // No need to add a final newline - this was causing extra line spacing
        // Just ensure we're on a fresh line for the next output
        self.emit(IR::PushString("\r".to_string()));
        self.emit(IR::Print);
    }

    fn compile_try_statement(&mut self, try_block: Vec<Statement>, catch_param: Option<String>, catch_block: Option<Vec<Statement>>, finally_block: Option<Vec<Statement>>) {
        // Proper implementation of try/catch/finally with exception handling

        // Generate unique labels for the try-catch-finally blocks
        let try_end_label = self.generate_label("try_end");
        let catch_start_label = self.generate_label("catch_start");
        let catch_end_label = self.generate_label("catch_end");
        let finally_start_label = self.generate_label("finally_start");
        let finally_end_label = self.generate_label("finally_end");

        // Set up exception handling
        self.emit(IR::PushString(catch_start_label.clone())); // Push catch handler address
        self.emit(IR::SetupTryCatch); // Setup try-catch block

        // Compile the try block
        self.enter_scope();
        for stmt in try_block {
            self.compile_statement(stmt);
        }
        self.leave_scope();

        // End of try block - clear exception handler and jump to finally
        self.emit(IR::ClearTryCatch); // Clear the try-catch handler

        // Store the position for the jump to finally
        let jump_to_finally_pos = self.emit(IR::Jump(0)); // Placeholder, will be updated

        // Catch handler starts here
        self.emit_label(&catch_start_label);

        // Compile the catch block if it exists
        if let Some(catch) = catch_block {
            self.enter_scope();

            // If we have a catch parameter, store the exception in it
            if let Some(param_name) = catch_param {
                let index = self.symbol_table.define(&param_name);
                self.emit(IR::StoreVar(param_name)); // Store exception in the variable
            } else {
                self.emit(IR::Pop); // Pop the exception if no parameter to store it
            }

            for stmt in catch {
                self.compile_statement(stmt);
            }
            self.leave_scope();
        } else {
            // If no catch block, just pop the exception
            self.emit(IR::Pop);
        }

        // Store the position for the jump to finally after catch
        let jump_after_catch_pos = self.emit(IR::Jump(0)); // Placeholder, will be updated
        self.emit_label(&catch_end_label);

        // Finally block - always executed
        let finally_pos = self.ir.len();
        self.emit_label(&finally_start_label);
        if let Some(finally) = finally_block {
            self.enter_scope();
            for stmt in finally {
                self.compile_statement(stmt);
            }
            self.leave_scope();
        }
        self.emit_label(&finally_end_label);

        // Update the jump positions now that we know where the finally block is
        self.replace_instruction(jump_to_finally_pos, IR::Jump(finally_pos));
        self.replace_instruction(jump_after_catch_pos, IR::Jump(finally_pos));
    }

    fn compile_throw_statement(&mut self, value: Expression) {
        // Compile the value to throw
        self.compile_expression(value);

        // Throw the exception using the new exception handling mechanism
        self.emit(IR::ThrowException);
    }

    fn compile_read_statement(&mut self, name: String) {
        // Define the variable in the symbol table
        self.symbol_table.define(&name);

        // Add a custom IR operation for reading user input
        self.emit(IR::ReadInput);
        self.emit(IR::StoreVar(name));
    }

    fn compile_exit_statement(&mut self) {
        // Emit the exit instruction to terminate the program
        self.emit(IR::Exit);
    }

    fn compile_document_type_declaration(&mut self, doc_type: String) {
        // Store the document type in the compiler state
        // This doesn't generate any runtime code as it's a compile-time directive
        if !self.clean_output {
            println!("Document type set to: {}", doc_type);
        }
    }

    fn compile_expression(&mut self, expr: Expression) {
        match expr {
            Expression::Identifier(name) => {
                self.emit(IR::LoadVar(name));
            },
            Expression::StringLiteral(value) => {
                self.emit(IR::PushString(value));
            },
            Expression::NumberLiteral(value) => {
                self.emit(IR::PushNumber(value));
            },
            Expression::BooleanLiteral(value) => {
                self.emit(IR::PushBoolean(value));
            },
            Expression::NullLiteral => {
                self.emit(IR::PushNull);
            },
            Expression::PrefixExpression { operator, right } => {
                self.compile_prefix_expression(operator, *right);
            },
            Expression::InfixExpression { left, right, operator, .. } => {
                self.compile_infix_expression(*left, operator, *right);
            },
            Expression::AssignmentExpression { left, operator, right } => {
                self.compile_assignment_expression(*left, operator, *right);
            },
            Expression::CallExpression { function, arguments } => {
                self.compile_call_expression(*function, arguments);
            },
            Expression::ArrayLiteral { elements } => {
                self.compile_array_literal(elements);
            },
            Expression::IndexExpression { left, index } => {
                self.compile_index_expression(*left, *index);
            },
            Expression::MapLiteral { pairs } => {
                self.compile_map_literal(pairs);
            },
            Expression::LibraryCall { library, function, arguments } => {
                self.compile_library_call(*library, *function, arguments);
            },
            Expression::NamespaceCall { namespace, function, arguments } => {
                self.compile_namespace_call(namespace, function, arguments);
            },
        }
    }

    fn compile_prefix_expression(&mut self, operator: String, right: Expression) {
        // Compile the right operand
        self.compile_expression(right);

        // Apply the prefix operator
        match operator.as_str() {
            "-" => { self.emit(IR::Negate); },
            "!" => { self.emit(IR::Not); },
            _ => panic!("Unknown prefix operator: {}", operator),
        }
    }

    fn compile_infix_expression(&mut self, left: Expression, operator: String, right: Expression) {
        // Compile the left operand
        self.compile_expression(left);

        // Compile the right operand
        self.compile_expression(right);

        // Apply the infix operator
        match operator.as_str() {
            "+" => { self.emit(IR::Add); },
            "-" => { self.emit(IR::Subtract); },
            "*" => { self.emit(IR::Multiply); },
            "/" => { self.emit(IR::Divide); },
            "%" => { self.emit(IR::Modulo); },
            "**" => { self.emit(IR::Power); },
            "//" => { self.emit(IR::FloorDiv); },
            "==" => { self.emit(IR::Equal); },
            "!=" => { self.emit(IR::NotEqual); },
            ">" => { self.emit(IR::GreaterThan); },
            ">=" => { self.emit(IR::GreaterEqual); },
            "<" => { self.emit(IR::LessThan); },
            "<=" => { self.emit(IR::LessEqual); },
            "&&" => { self.emit(IR::And); },
            "||" => { self.emit(IR::Or); },
            _ => panic!("Unknown infix operator: {}", operator),
        }
    }

    fn compile_assignment_expression(&mut self, left: Expression, operator: String, right: Expression) {
        // For simple assignment (=), just compile the right expression and store it
        if operator == "=" {
            // Compile the right expression
            self.compile_expression(right);

            // Store the result in the variable
            if let Expression::Identifier(name) = left {
                self.emit(IR::StoreVar(name));
            } else if let Expression::IndexExpression { left: array, index } = left {
                // Compile the array and index
                self.compile_expression(*array);
                self.compile_expression(*index);

                // Store the value in the array at the given index
                self.emit(IR::SetIndex);
            } else {
                panic!("Invalid left-hand side in assignment");
            }
        } else {
            // For compound assignments (+=, -=, etc.), load the variable, apply the operation, and store it back
            if let Expression::Identifier(name) = left.clone() {
                // Load the current value
                self.emit(IR::LoadVar(name.clone()));

                // Compile the right expression
                self.compile_expression(right);

                // Apply the operation
                match operator.as_str() {
                    "+=" => { self.emit(IR::Add); },
                    "-=" => { self.emit(IR::Subtract); },
                    "*=" => { self.emit(IR::Multiply); },
                    "/=" => { self.emit(IR::Divide); },
                    "%=" => { self.emit(IR::Modulo); },
                    _ => panic!("Unknown assignment operator: {}", operator),
                }

                // Store the result back in the variable
                self.emit(IR::StoreVar(name));
            } else {
                panic!("Invalid left-hand side in compound assignment");
            }
        }
    }

    fn compile_call_expression(&mut self, function: Expression, arguments: Vec<Expression>) {
        // Compile each argument
        for arg in &arguments {
            self.compile_expression(arg.clone());
        }

        // Support function names like Identifier or InfixExpression (dot notation)
        let func_name = match function {
            Expression::Identifier(name) => name,
            Expression::InfixExpression { left, operator, right } if operator == "." => {
                // Only support left/right as Identifier for now
                if let (Expression::Identifier(left_name), Expression::Identifier(right_name)) = (*left, *right) {
                    format!("{}.{}", left_name, right_name)
                } else {
                    panic!("Dot expression must be identifiers on both sides");
                }
            },
            _ => panic!("Function call on non-identifier or unsupported expression"),
        };

        // Call the function with the given number of arguments
        self.emit(IR::Call(func_name, arguments.len()));

        // For show statements, we need to handle the return value
        if self.in_show_statement {
            // The return value is already on the stack
            // No need to do anything special
        }
    }

    fn compile_array_literal(&mut self, elements: Vec<Expression>) {
        // Compile each element
        for elem in &elements {
            self.compile_expression(elem.clone());
        }

        // Create the array with the given number of elements
        self.emit(IR::CreateArray(elements.len()));
    }

    fn compile_index_expression(&mut self, left: Expression, index: Expression) {
        // First compile the indexed expression (array, map, string, enum, etc.)
        self.compile_expression(left);

        // Special handling for enum access with a direct identifier
        match index {
            Expression::Identifier(name) => {
                // When accessing an enum value like Color[RED], we need to handle it specially
                // Just push the identifier name as a string
                self.emit(IR::PushString(name.clone()));
            },
            _ => {
                // For other index expressions (number literals, computed expressions, etc.)
                self.compile_expression(index);
            }
        }

        // Generate IR instruction for indexing
        self.emit(IR::GetIndex);
    }

    fn compile_map_literal(&mut self, pairs: Vec<(Expression, Expression)>) {
        // Compile each key-value pair
        for (key, value) in &pairs {
            self.compile_expression(key.clone());
            self.compile_expression(value.clone());
        }

        // Create the map with the given number of key-value pairs
        self.emit(IR::CreateMap(pairs.len()));
    }

    fn compile_library_call(&mut self, library: Expression, function: Expression, arguments: Vec<Expression>) {
        // Compile each argument
        for arg in &arguments {
            self.compile_expression(arg.clone());
        }

        // Get the library name
        let lib_name = match library {
            Expression::Identifier(name) => name,
            _ => panic!("Library call on non-identifier"),
        };

        // Get the function name
        let func_name = match function {
            Expression::Identifier(name) => name,
            Expression::IndexExpression { left, index } => {
                // Only support left as Identifier for now
                if let Expression::Identifier(left_name) = *left {
                    if let Expression::StringLiteral(index_str) = *index {
                        format!("{}.{}", left_name, index_str)
                    } else {
                        panic!("Bracket notation must be used with string literal");
                    }
                } else {
                    panic!("Bracket notation must be used with identifier on the left");
                }
            },
            _ => panic!("Function name must be an identifier or bracket notation"),
        };

        // Create the full function name in the format "LibName.FunctionName"
        let full_func_name = format!("{}.{}", lib_name, func_name);

        // Call the library function with the given number of arguments
        self.emit(IR::LibraryCall(lib_name, full_func_name, arguments.len()));

        // For show statements, we need to handle the return value
        if self.in_show_statement {
            // The return value is already on the stack
            // No need to do anything special
        }
    }

    fn compile_namespace_call(&mut self, namespace: String, function: String, arguments: Vec<Expression>) {
        // Compile each argument
        for arg in &arguments {
            self.compile_expression(arg.clone());
        }

        // Create the full function name in the format "namespace.function"
        let full_func_name = format!("{}.{}", namespace, function);

        // Call the library function with the given number of arguments
        self.emit(IR::LibraryCall(namespace, full_func_name, arguments.len()));

        // For show statements, we need to handle the return value
        if self.in_show_statement {
            // The return value is already on the stack
            // No need to do anything special
        }
    }

    fn compile_load_statement(&mut self, cycles: Expression, block: Vec<Statement>) {
        // Determine cycles count - default to 3 if not a literal
        let cycles_value = match cycles {
            Expression::NumberLiteral(num) => num as usize,
            _ => {
                // If not a number literal, we need to evaluate the expression at runtime
                self.compile_expression(cycles.clone());
                self.emit(IR::Pop); // We don't need the result, use default
                3 // Default cycles
            }
        };

        // Clamp to 1-10 cycles
        let cycles_count = cycles_value.clamp(1, 10);

        // Verify all statements in the block are 'show' statements
        for stmt in &block {
            if !matches!(stmt, Statement::ShowStatement { .. }) {
                self.errors.push("Only 'show' statements are allowed inside a 'load' block".to_string());
            }
        }

        // Get the number of show statements
        let show_count = block.len();

        if show_count == 0 {
            // Nothing to show
            return;
        }

        // Flag that we're in a loading animation
        let old_in_show = self.in_show_statement;
        self.in_show_statement = true;

        // Loop through the cycles
        for _ in 0..cycles_count {
            // Loop through each show statement in the block
            for stmt in &block {
                if let Statement::ShowStatement { value, color } = stmt {
                    // If color is specified, add ANSI color code before printing the value
                    if let Some(color_name) = color {
                        // Get the color code using our color utility
                        let color_code = crate::functions::colorlib::get_color_code(&color_name);
                        self.emit(IR::PushString(color_code));
                        self.emit(IR::Print);
                    }

                    // Compile and display the message
                    self.compile_expression(value.clone());
                    self.emit(IR::Print);

                    // Reset color if it was set
                    if color.is_some() {
                        let reset_code = crate::functions::colorlib::get_color_code("reset");
                        self.emit(IR::PushString(reset_code));
                        self.emit(IR::Print);
                    }

                    // Add a delay (sleep)
                    self.emit(IR::PushNumber(0.3)); // 300ms delay
                    self.emit(IR::Sleep); // Use Sleep instruction instead of Pop

                    // Clear the line with carriage return to prepare for next message
                    // Note: We don't add any newlines here, just carriage return to overwrite the current line
                    self.emit(IR::PushString("\r".to_string()));
                    self.emit(IR::Print);
                }
            }
        }

        // Reset show statement flag
        self.in_show_statement = old_in_show;

        // No need to add a final newline - this was causing extra line spacing
        // Just ensure we're on a fresh line for the next output
        self.emit(IR::PushString("\r".to_string()));
        self.emit(IR::Print);
    }

    // Generate machine code from the IR
    pub fn generate_machine_code(&self) -> Result<Vec<u8>, String> {
        // This is a simplified implementation that would be replaced with actual machine code generation
        // In a real implementation, this would translate the IR to native machine code

        // For now, we'll just return a placeholder binary
        let mut code = Vec::new();

        // Add a simple header
        code.extend_from_slice(b"\x7FELF"); // ELF magic number

        // Add placeholder code for each IR instruction
        for ir in &self.ir {
            match ir {
                IR::PushNumber(_) => code.push(0x01),
                IR::PushString(_) => code.push(0x02),
                IR::PushBoolean(_) => code.push(0x03),
                IR::PushNull => code.push(0x04),
                IR::Pop => code.push(0x05),
                IR::Dup => code.push(0x06),
                IR::Swap => code.push(0x07),
                IR::StoreVar(_) => code.push(0x08),
                IR::LoadVar(_) => code.push(0x09),
                IR::Add => code.push(0x0A),
                IR::Subtract => code.push(0x0B),
                IR::Multiply => code.push(0x0C),
                IR::Divide => code.push(0x0D),
                IR::Modulo => code.push(0x0E),
                IR::Power => code.push(0x0F),
                IR::FloorDiv => code.push(0x10),
                IR::Negate => code.push(0x11),
                IR::Equal => code.push(0x12),
                IR::NotEqual => code.push(0x13),
                IR::GreaterThan => code.push(0x14),
                IR::GreaterEqual => code.push(0x15),
                IR::LessThan => code.push(0x16),
                IR::LessEqual => code.push(0x17),
                IR::And => code.push(0x18),
                IR::Or => code.push(0x19),
                IR::Not => code.push(0x1A),
                IR::Jump(_) => code.push(0x1B),
                IR::JumpIfFalse(_) => code.push(0x1C),
                IR::JumpIfTrue(_) => code.push(0x1D),
                IR::Call(_, _) => code.push(0x1E),
                IR::Return => code.push(0x1F),
                IR::Print => code.push(0x20),
                IR::ReadInput => code.push(0x29),
                IR::Exit => code.push(0x2A),
                IR::CreateArray(_) => code.push(0x21),
                IR::GetIndex => code.push(0x22),
                IR::SetIndex => code.push(0x23),
                IR::CreateMap(_) => code.push(0x24),
                IR::GetKey => code.push(0x25),
                IR::SetKey => code.push(0x26),
                IR::DefineFunction(_, _) => code.push(0x27),
                IR::Label(_) => code.push(0x28),
                IR::SetGlobal(_) => code.push(0x2B), // Global variable operations
                IR::Sleep => code.push(0x2C),
                IR::LibraryCall(_, _, _) => code.push(0x2D),
                IR::SetupTryCatch => code.push(0x2E),
                IR::ClearTryCatch => code.push(0x2F),
                IR::ThrowException => code.push(0x30),
            }
        }

        Ok(code)
    }

    // Write the generated machine code to a file
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        match self.generate_machine_code() {
            Ok(code) => {
                match fs::write(path, code) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to write to file: {}", e)),
                }
            },
            Err(e) => Err(e),
        }
    }

    // Execute the compiled code directly
    // In compiler.rs

    // ... (keep the rest of the Compiler struct and its methods as they are) ...

    // Execute the compiled code directly
    pub fn execute(&self) -> Result<(), String> {
        if !self.clean_output {
            println!("Executing Razen program...");
            for (i, ir) in self.ir.iter().enumerate() {
                println!("{}: {:?}", i, ir);
            }
        }

        // Helper function for boolean logic
        fn is_truthy(s: &str) -> bool {
            !matches!(s, "false" | "0" | "" | "null" | "undefined" | "False")
        }

        let mut stack: Vec<String> = Vec::new();
        let mut variables: HashMap<String, String> = HashMap::new();
        let mut call_stack: Vec<(usize, HashMap<String, String>)> = Vec::new();
        let mut exception_handlers: Vec<(String, usize)> = Vec::new();

        // Simplified pre-pass: Just register function addresses.
        // Parameter binding is handled at call time.
        for ir in self.ir.iter() {
            if let IR::DefineFunction(name, address) = ir {
                variables.insert(name.clone(), address.to_string());
            }
        }

        let mut pc = 0;
        while pc < self.ir.len() {
            let ir = &self.ir[pc];
            match ir {
                IR::PushNumber(n) => stack.push(n.to_string()),
                IR::PushString(s) => stack.push(s.clone()),
                IR::PushBoolean(b) => stack.push(b.to_string()),
                IR::PushNull => stack.push("null".to_string()),
                IR::Pop => { stack.pop(); },
                IR::Dup => {
                    if let Some(value) = stack.last().cloned() {
                        stack.push(value);
                    }
                },
                IR::Swap => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        stack.push(b);
                        stack.push(a);
                    }
                },
                // **IMPROVED**: Handles function-local scope
                IR::StoreVar(name) => {
                    if let Some(value) = stack.pop() {
                        if let Some((_, func_vars)) = call_stack.last_mut() {
                            func_vars.insert(name.clone(), value);
                        } else {
                            variables.insert(name.clone(), value);
                        }
                    }
                },
                // **IMPROVED**: Handles function-local scope
                IR::LoadVar(name) => {
                    let value = if let Some((_, func_vars)) = call_stack.last() {
                        func_vars.get(name)
                    } else { None };

                    if let Some(val) = value {
                        stack.push(val.clone());
                    } else if let Some(val) = variables.get(name) {
                        stack.push(val.clone());
                    } else {
                        stack.push("undefined".to_string());
                    }
                },
                IR::SetGlobal(name) => {
                    if let Some(value) = stack.pop() {
                        variables.insert(name.clone(), value);
                    }
                },
                // **RESTORED**: Full set of operators
                IR::Add => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            stack.push((a_num + b_num).to_string());
                        } else {
                            stack.push(format!("{}{}", a, b));
                        }
                    }
                },
                IR::Subtract => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            stack.push((a_num - b_num).to_string());
                        }
                    }
                },
                IR::Multiply => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            stack.push((a_num * b_num).to_string());
                        }
                    }
                },
                IR::Divide => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            if b_num != 0.0 { stack.push((a_num / b_num).to_string()); }
                            else { return Err("Division by zero".to_string()); }
                        }
                    }
                },
                IR::Modulo => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            if b_num != 0.0 { stack.push((a_num % b_num).to_string()); }
                            else { return Err("Modulo by zero".to_string()); }
                        }
                    }
                },
                IR::Power => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            stack.push(a_num.powf(b_num).to_string());
                        }
                    }
                },
                IR::FloorDiv => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            if b_num != 0.0 { stack.push((a_num / b_num).floor().to_string()); }
                            else { return Err("Division by zero".to_string()); }
                        }
                    }
                },
                IR::Negate => {
                    if let Some(a) = stack.pop() {
                        if let Ok(a_num) = a.parse::<f64>() {
                            stack.push((-a_num).to_string());
                        }
                    }
                },
                IR::Equal => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            stack.push((a_num == b_num).to_string());
                        } else { stack.push((a == b).to_string()); }
                    }
                },
                IR::NotEqual => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            stack.push((a_num != b_num).to_string());
                        } else { stack.push((a != b).to_string()); }
                    }
                },
                IR::GreaterThan => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            stack.push((a_num > b_num).to_string());
                        } else { stack.push((a > b).to_string()); }
                    }
                },
                IR::GreaterEqual => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            stack.push((a_num >= b_num).to_string());
                        } else { stack.push((a >= b).to_string()); }
                    }
                },
                IR::LessThan => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            stack.push((a_num < b_num).to_string());
                        } else { stack.push((a < b).to_string()); }
                    }
                },
                IR::LessEqual => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        if let (Ok(a_num), Ok(b_num)) = (a.parse::<f64>(), b.parse::<f64>()) {
                            stack.push((a_num <= b_num).to_string());
                        } else { stack.push((a <= b).to_string()); }
                    }
                },
                IR::And => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        stack.push((is_truthy(&a) && is_truthy(&b)).to_string());
                    }
                },
                IR::Or => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        stack.push((is_truthy(&a) || is_truthy(&b)).to_string());
                    }
                },
                IR::Not => {
                    if let Some(a) = stack.pop() {
                        stack.push((!is_truthy(&a)).to_string());
                    }
                },
                IR::Jump(target) => { pc = *target; continue; },
                IR::JumpIfFalse(target) => {
                    if let Some(value) = stack.pop() {
                        if !is_truthy(&value) { pc = *target; continue; }
                    }
                },
                IR::JumpIfTrue(target) => {
                    if let Some(value) = stack.pop() {
                        if is_truthy(&value) { pc = *target; continue; }
                    }
                },
                // **IMPROVED**: Handles function returns correctly
                IR::Return => {
                    let return_value = stack.pop().unwrap_or_else(|| "null".to_string());
                    if let Some((return_addr, caller_variables)) = call_stack.pop() {
                        variables = caller_variables;
                        stack.push(return_value);
                        pc = return_addr;
                        continue;
                    } else {
                        stack.push(return_value);
                    }
                },
                // **THE ONLY MAJOR FIX**: Handles user-defined function calls correctly
                IR::Call(name, arg_count) => {
                    if !self.clean_output {
                        println!("Calling user function: {} with {} arguments", name, arg_count);
                    }
                    let mut args = Vec::new();
                    for _ in 0..*arg_count {
                        if let Some(arg) = stack.pop() { args.push(arg); }
                    }
                    args.reverse();

                    if let Some(func_addr_str) = variables.get(name) {
                        if let Ok(func_addr) = func_addr_str.parse::<usize>() {
                            let mut func_variables = variables.clone(); // Inherit globals
                            if let Some(param_names) = self.function_param_names.get(name) {
                                for (i, param_name) in param_names.iter().enumerate() {
                                    if i < args.len() {
                                        func_variables.insert(param_name.clone(), args[i].clone());
                                    } else {
                                        func_variables.insert(param_name.clone(), "undefined".to_string());
                                    }
                                }
                            }
                            call_stack.push((pc + 1, variables.clone()));
                            variables = func_variables;
                            pc = func_addr;
                            continue;
                        }
                    } else {
                        if !self.clean_output { println!("Unknown function: {}", name); }
                        stack.push("undefined".to_string());
                    }
                },
                // **RESTORED**: The original, full-featured GetIndex logic
                IR::GetIndex => {
                    if let (Some(index), Some(container)) = (stack.pop(), stack.pop()) {
                        let mut found = false;
                        // Check for enums/maps which are stored as "KEY:VALUE,KEY2:VALUE2"
                        let enum_key_str = format!("{}:", index);
                        if let Some(start_pos) = container.find(&enum_key_str) {
                            let after_key = &container[start_pos + enum_key_str.len()..];
                            let end_pos = after_key.find(',').unwrap_or(after_key.len());
                            let value = &after_key[0..end_pos];
                            stack.push(value.trim().to_string());
                            found = true;
                        }
                        // Handle array indexing
                        if !found && container.starts_with('[') && container.ends_with(']') {
                            if let Ok(idx) = index.parse::<usize>() {
                                let content = &container[1..container.len() - 1];
                                 // This is a simplified split, a full parser would be needed for nested arrays.
                                let elements: Vec<&str> = content.split(',').map(str::trim).collect();
                                if idx < elements.len() {
                                    stack.push(elements[idx].to_string());
                                    found = true;
                                }
                            }
                        }
                        if !found {
                            stack.push("undefined".to_string());
                        }
                    } else {
                        stack.push("undefined".to_string());
                    }
                },
                // **RESTORED**: The original, full-featured SetIndex logic
                IR::SetIndex => {
                     if let (Some(value), Some(index_str), Some(container_str)) = (stack.pop(), stack.pop(), stack.pop()) {
                        if container_str.starts_with('[') && container_str.ends_with(']') {
                            let content = &container_str[1..container_str.len() - 1];
                            let mut elements: Vec<String> = content.split(',').map(|s| s.trim().to_string()).collect();
                            if let Ok(idx) = index_str.parse::<usize>() {
                                while elements.len() <= idx {
                                    elements.push("null".to_string());
                                }
                                elements[idx] = value;
                                stack.push(format!("[{}]", elements.join(", ")));
                            }
                        } else {
                            // In a real implementation, you'd handle map setting here too.
                            stack.push(container_str); // No-op for now on non-arrays
                        }
                     }
                },
                // **RESTORED**: The original, full-featured LibraryCall logic
                IR::LibraryCall(lib_name, func_name, arg_count) => {
                    if !self.clean_output {
                        println!("Calling library function: {}.{} with {} arguments", lib_name, func_name, arg_count);
                    }
                    let function_name_only = func_name.split('.').last().unwrap_or(func_name);
                    let mut args = Vec::new();
                    for _ in 0..*arg_count {
                        if let Some(arg) = stack.pop() {
                            if let Ok(i) = arg.parse::<i64>() { args.push(crate::value::Value::Int(i)); }
                            else if let Ok(f) = arg.parse::<f64>() { args.push(crate::value::Value::Float(f)); }
                            else if arg == "true" { args.push(crate::value::Value::Bool(true)); }
                            else if arg == "false" { args.push(crate::value::Value::Bool(false)); }
                            else if arg == "null" || arg == "undefined" { args.push(crate::value::Value::Null); }
                            else if arg.starts_with('[') && arg.ends_with(']') {
                                let inner = &arg[1..arg.len()-1];
                                let elements: Vec<crate::value::Value> = inner.split(',')
                                    .map(|s| {
                                        let s = s.trim();
                                        if let Ok(i) = s.parse::<i64>() { crate::value::Value::Int(i) }
                                        else if let Ok(f) = s.parse::<f64>() { crate::value::Value::Float(f) }
                                        else { crate::value::Value::String(s.to_string()) }
                                    }).collect();
                                args.push(crate::value::Value::Array(elements));
                            } else {
                                 let mut final_arg_str = arg.clone();
                                 if final_arg_str.starts_with('"') && final_arg_str.ends_with('"') && final_arg_str.len() >= 2 {
                                     final_arg_str = final_arg_str[1..final_arg_str.len()-1].to_string();
                                 }
                                 args.push(crate::value::Value::String(final_arg_str));
                            }
                        }
                    }
                    args.reverse();

                    match crate::library::call_library(&lib_name.to_lowercase(), function_name_only, args) {
                        Ok(value) => stack.push(value.to_string()),
                        Err(e) => {
                            // Handle library errors by trying to throw an exception
                            if let Some((_, handler_pc)) = exception_handlers.pop() {
                                stack.push(e);
                                pc = handler_pc;
                                continue;
                            } else {
                                return Err(format!("Unhandled library exception: {}", e));
                            }
                        }
                    };
                },
                // **RESTORED**: Other necessary opcodes
                IR::CreateArray(count) => {
                    let mut array = Vec::new();
                    for _ in 0..*count {
                        if let Some(value) = stack.pop() {
                            array.push(value);
                        }
                    }
                    array.reverse();
                    stack.push(format!("[{}]", array.join(", ")));
                },
                IR::CreateMap(count) => {
                    let mut map_entries = Vec::new();
                    for _ in 0..*count {
                        if let (Some(value), Some(key)) = (stack.pop(), stack.pop()) {
                            map_entries.push(format!("{}:{}", key, value));
                        }
                    }
                    map_entries.reverse(); // Reverse to keep order from source code
                    stack.push(format!("{{{}}}", map_entries.join(", ")));
                },
                IR::Print => {
                    if let Some(value) = stack.pop() {
                        use std::io::{self, Write};
                        print!("{}", value);
                        io::stdout().flush().unwrap();
                    }
                },
                IR::ReadInput => {
                    use std::io::{self, BufRead};
                    let stdin = io::stdin();
                    let mut line = String::new();
                    stdin.lock().read_line(&mut line).expect("Failed to read line");
                    if line.ends_with('\n') { line.pop(); if line.ends_with('\r') { line.pop(); } }
                    stack.push(line);
                },
                IR::Exit => { return Ok(()); },
                IR::Sleep => {
                    if let Some(duration_str) = stack.pop() {
                        if let Ok(duration) = duration_str.parse::<f64>() {
                             thread::sleep(Duration::from_secs_f64(duration));
                        }
                    }
                },
                // **RESTORED**: Exception Handling
                IR::SetupTryCatch => {
                    if let Some(handler_label) = stack.pop() {
                       let handler_pc = self.ir.iter().position(|ir| matches!(ir, IR::Label(l) if l == &handler_label));
                       if let Some(pc) = handler_pc {
                           exception_handlers.push((handler_label, pc));
                       }
                   }
                },
                IR::ClearTryCatch => { exception_handlers.pop(); },
                IR::ThrowException => {
                    if let Some(error_message) = stack.pop() {
                         if let Some((_, handler_pc)) = exception_handlers.pop() {
                            stack.push(error_message);
                            pc = handler_pc;
                            continue;
                        } else {
                            return Err(format!("Unhandled exception: {}", error_message));
                        }
                    }
                },
                IR::DefineFunction(_, _) | IR::Label(_) => {}, // Ignored at runtime
                _ => {
                    if !self.clean_output {
                        println!("Unimplemented instruction: {:?}", ir);
                    }
                }
            }
            pc += 1;
        }

        if !self.clean_output {
            println!("Execution complete.");
        }
        Ok(())
    }

    // Module System Methods

    /// Compile module import statement
    fn compile_module_import(&mut self, names: Vec<String>, alias: Option<String>, source: String) {
        if !self.clean_output {
            println!("[Compiler] Importing module: {} from {}", names.join(", "), source);
        }

        // Load the module file
        let module_path = source.trim_matches('"');

        // Check if we need to add .rzn extension
        let module_file = if module_path.ends_with(".rzn") {
            module_path.to_string()
        } else {
            format!("{}.rzn", module_path)
        };

        // Try to find the module in standard library first, then relative to current file
        let module_content = match fs::read_to_string(&module_file) {
            Ok(content) => content,
            Err(_) => {
                // Try standard library path
                let std_lib_path = format!("stdlib/{}", module_file);
                match fs::read_to_string(&std_lib_path) {
                    Ok(content) => content,
                    Err(_) => {
                        self.errors.push(format!("Module not found: {}", module_file));
                        return;
                    }
                }
            }
        };

        // Parse the module
        let mut lexer = crate::lexer::Lexer::new(module_content);
        let mut parser = crate::parser::Parser::new(lexer);
        let module_program = parser.parse_program();

        // Check for parser errors
        if !parser.get_errors().is_empty() {
            for error in parser.get_errors() {
                self.errors.push(format!("Error parsing module {}: {}", module_file, error));
            }
            return;
        }

        // Create a new compiler for the module
        let mut module_compiler = Compiler::new();

        // Compile the module
        module_compiler.compile_program(module_program);

        // Import the exported symbols from the module
        if let Some(alias_name) = alias {
            // Import as namespace
            self.emit(IR::PushString(format!("Importing module {} as {}", source, alias_name)));
            self.emit(IR::Call("__import_module".to_string(), 1));
        } else {
            // Import specific names
            for name in names {
                // Define the symbol in current scope
                self.symbol_table.define(&name);

                // For constants like PI, we need to initialize them
                if name == "PI" {
                    self.emit(IR::PushNumber(3.14159265359));
                    self.emit(IR::SetGlobal(name.clone()));
                } else if name == "E" {
                    self.emit(IR::PushNumber(2.71828182846));
                    self.emit(IR::SetGlobal(name.clone()));
                } else {
                    // For functions, just import the symbol
                    self.emit(IR::PushString(format!("Importing {} from {}", name, source)));
                    self.emit(IR::Call("__import_symbol".to_string(), 1));
                }
            }
        }
    }

    /// Compile module export statement
    fn compile_module_export(&mut self, name: String) {
        if !self.clean_output {
            println!("[Compiler] Exporting symbol: {}", name);
        }

        // Check if the symbol exists in current scope
        if self.symbol_table.resolve(&name).is_none() && self.function_table.resolve(&name).is_none() {
            self.errors.push(format!("Cannot export undefined symbol: {}", name));
            return;
        }

        // Mark the symbol as exported
        self.emit(IR::PushString(name));
        self.emit(IR::Call("__export_symbol".to_string(), 1));
    }

    // Developer Tools Methods

    /// Compile debug statement
    fn compile_debug_statement(&mut self, value: Expression) {
        if !self.clean_output {
            println!("[Compiler] Debug statement");
        }

        // Compile the expression to debug
        self.compile_expression(value);

        // Call debug function
        self.emit(IR::Call("__debug".to_string(), 1));
    }

    /// Compile assert statement
    fn compile_assert_statement(&mut self, condition: Expression, message: Option<Expression>) {
        if !self.clean_output {
            println!("[Compiler] Assert statement");
        }

        // Compile the condition
        self.compile_expression(condition);

        // If there's a message, compile it too
        if let Some(msg) = message {
            self.compile_expression(msg);
            self.emit(IR::Call("__assert_with_message".to_string(), 2));
        } else {
            self.emit(IR::Call("__assert".to_string(), 1));
        }
    }

    /// Compile trace statement
    fn compile_trace_statement(&mut self, value: Expression) {
        if !self.clean_output {
            println!("[Compiler] Trace statement");
        }

        // Compile the expression to trace
        self.compile_expression(value);

        // Call trace function
        self.emit(IR::Call("__trace".to_string(), 1));
    }

    // OOP Methods (Section 12)

    // Compile class declaration
    fn compile_class_declaration(&mut self, name: String, body: Vec<Statement>) {
        if !self.clean_output {
            println!("[Compiler] Class declaration: {}", name);
        }

        // Create a new scope for the class
        self.enter_scope();

        // Store class name in symbol table
        self.symbol_table.define(&name);

        // Emit class definition
        self.emit(IR::PushString(name.clone()));

        // Compile class body
        for stmt in body {
            self.compile_statement(stmt);
        }

        // Leave class scope
        self.leave_scope();

        // Call class definition function
        self.emit(IR::Call("__define_class".to_string(), 1));
    }

    // API Integration Methods (Section 13)

    // Compile API declaration
    fn compile_api_declaration(&mut self, name: String, url: String) {
        if !self.clean_output {
            println!("[Compiler] API declaration: {} from {}", name, url);
        }

        // Store API name in symbol table
        self.symbol_table.define(&name);

        // Create API object
        self.emit(IR::CreateMap(0));

        // Add URL to API object
        self.emit(IR::PushString("url".to_string()));
        self.emit(IR::PushString(url.clone()));
        self.emit(IR::SetKey);

        // Store API object in global variable
        self.emit(IR::SetGlobal(name));
    }

    // Compile API call
    fn compile_api_call(&mut self, name: String, body: Vec<Statement>) {
        if !self.clean_output {
            println!("[Compiler] API call: {}", name);
        }

        // Create a new scope for the API call
        self.enter_scope();

        // Emit API call start
        self.emit(IR::PushString(name.clone()));

        // Compile API call body
        for stmt in body {
            self.compile_statement(stmt);
        }

        // Leave API call scope
        self.leave_scope();

        // Call API execution function
        self.emit(IR::Call("__execute_api_call".to_string(), 1));
    }

    // Connect Methods (Section 14)

    // Compile connect statement
    fn compile_connect_statement(&mut self, name: String, url: String, options: Vec<(String, Expression)>) {
        if !self.clean_output {
            println!("[Compiler] Connect statement: {} to {}", name, url);
        }

        // Store connection name in symbol table
        self.symbol_table.define(&name);

        // Emit connection definition
        self.emit(IR::PushString(name.clone()));
        self.emit(IR::PushString(url));

        // Create options map
        self.emit(IR::PushNumber(options.len() as f64));
        self.emit(IR::CreateMap(options.len()));

        // Compile connection options
        for (option_name, option_value) in options {
            self.emit(IR::PushString(option_name));
            self.compile_expression(option_value);
            self.emit(IR::SetKey);
        }

        // Call connection function
        self.emit(IR::Call("__connect".to_string(), 3));
    }

    // Import/Export Methods (Section 15)

    // Compile import statement
    fn compile_import_statement(&mut self, imports: Vec<String>, path: String) {
        if !self.clean_output {
            println!("[Compiler] Import statement: {:?} from {}", imports, path);
        }

        // Emit import path
        self.emit(IR::PushString(path));

        // Create array of imports
        self.emit(IR::PushNumber(imports.len() as f64));
        self.emit(IR::CreateArray(imports.len()));

        // Add each import to the array
        for import in &imports {
            self.emit(IR::PushString(import.clone()));
            self.emit(IR::SetIndex);
        }

        // Call import function
        self.emit(IR::Call("__import".to_string(), 2));

        // Add each import to symbol table
        for import in imports {
            self.symbol_table.define(&import);
        }
    }

    // Library Methods (Section 16)

    // Compile library import statement
    fn compile_lib_statement(&mut self, name: String) {
        if !self.clean_output {
            println!("[Compiler] Library import: {}", name);
        }

        // Emit library name
        self.emit(IR::PushString(name.clone()));

        // Call library import function
        self.emit(IR::Call("__import_lib".to_string(), 1));

        // Dynamically register all library functions for all libraries
        let libs_path = PathBuf::from("properties/libs");
        self.register_all_libraries(libs_path.to_str().unwrap_or("properties/libs"));

        // No need to match on specific library names anymore
        // Just register the specific library being imported
        if !self.clean_output {
            println!("[Compiler] Registered library: {}", name);
        }
    }

    // Compiler Construction Statement Compilation Functions

    fn compile_grammar_statement(&mut self, name: String, properties: Vec<(String, Expression)>) {
        // Create an empty map for the grammar
        self.emit(IR::CreateMap(0));

        // Add properties to the map
        for (key, value) in properties {
            // Push the key
            self.emit(IR::PushString(key));

            // Push the value
            self.compile_expression(value);

            // Set the key-value pair in the map
            self.emit(IR::SetKey);
        }

        // Store the grammar in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_token_statement(&mut self, name: String, pattern: String) {
        // Push the token pattern
        self.emit(IR::PushString(pattern));

        // Store the token in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_lexer_statement(&mut self, name: String, config: Vec<(String, Expression)>) {
        // Create an empty map for the lexer config
        self.emit(IR::CreateMap(0));

        // Add config properties to the map
        for (key, value) in config {
            // Push the key
            self.emit(IR::PushString(key));

            // Push the value
            self.compile_expression(value);

            // Set the key-value pair in the map
            self.emit(IR::SetKey);
        }

        // Store the lexer in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_parser_statement(&mut self, name: String, config: Vec<(String, Expression)>) {
        // Create an empty map for the parser config
        self.emit(IR::CreateMap(0));

        // Add config properties to the map
        for (key, value) in config {
            // Push the key
            self.emit(IR::PushString(key));

            // Push the value
            self.compile_expression(value);

            // Set the key-value pair in the map
            self.emit(IR::SetKey);
        }

        // Store the parser in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_node_statement(&mut self, name: String, properties: Vec<(String, Expression)>) {
        // Create an empty map for the node properties
        self.emit(IR::CreateMap(0));

        // Add properties to the map
        for (key, value) in properties {
            // Push the key
            self.emit(IR::PushString(key));

            // Push the value
            self.compile_expression(value);

            // Set the key-value pair in the map
            self.emit(IR::SetKey);
        }

        // Store the node in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_rule_statement(&mut self, name: String, production: String, node_type: Option<String>) {
        // Create an empty map for the rule
        self.emit(IR::CreateMap(0));

        // Add production to the map
        self.emit(IR::PushString("production".to_string()));
        self.emit(IR::PushString(production));
        self.emit(IR::SetKey);

        // Add node_type to the map if present
        if let Some(node) = node_type {
            self.emit(IR::PushString("astNode".to_string()));
            self.emit(IR::PushString(node));
            self.emit(IR::SetKey);
        }

        // Store the rule in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_visitor_statement(&mut self, name: String, methods: Vec<String>) {
        // Create an empty map for the visitor
        self.emit(IR::CreateMap(0));

        // Add methods array to the map
        self.emit(IR::PushString("methods".to_string()));

        // Create an array for methods
        self.emit(IR::CreateArray(methods.len()));

        // Add each method to the array
        for (i, method) in methods.iter().enumerate() {
            self.emit(IR::PushString(method.clone()));
            // Use i as the index for the array
            self.emit(IR::PushNumber(i as f64));
            self.emit(IR::SetIndex);
        }

        // Set the methods array in the map
        self.emit(IR::SetKey);

        // Store the visitor in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_symbol_statement(&mut self, name: String, attributes: Vec<String>) {
        // Create an empty map for the symbol
        self.emit(IR::CreateMap(0));

        // Add attributes array to the map
        self.emit(IR::PushString("attributes".to_string()));

        // Create an array for attributes
        self.emit(IR::CreateArray(attributes.len()));

        // Add each attribute to the array
        for (i, attr) in attributes.iter().enumerate() {
            self.emit(IR::PushString(attr.clone()));
            // Use i as the index for the array
            self.emit(IR::PushNumber(i as f64));
            self.emit(IR::SetIndex);
        }

        // Set the attributes array in the map
        self.emit(IR::SetKey);

        // Store the symbol in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_scope_statement(&mut self, name: String, parent: Option<String>) {
        // Create an empty map for the scope
        self.emit(IR::CreateMap(0));

        // Add parent to the map if present
        self.emit(IR::PushString("parent".to_string()));
        if let Some(p) = parent {
            self.emit(IR::PushString(p));
        } else {
            self.emit(IR::PushNull);
        }
        self.emit(IR::SetKey);

        // Store the scope in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_type_statement(&mut self, name: String, operations: Vec<String>) {
        // Create an empty map for the type
        self.emit(IR::CreateMap(0));

        // Add operations array to the map
        self.emit(IR::PushString("operations".to_string()));

        // Create an array for operations
        self.emit(IR::CreateArray(operations.len()));

        // Add each operation to the array
        for (i, op) in operations.iter().enumerate() {
            self.emit(IR::PushString(op.clone()));
            // Use i as the index for the array
            self.emit(IR::PushNumber(i as f64));
            self.emit(IR::SetIndex);
        }

        // Set the operations array in the map
        self.emit(IR::SetKey);

        // Store the type in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_ir_statement(&mut self, name: String, opcode: String, operands: Vec<String>) {
        // Create an empty map for the IR
        self.emit(IR::CreateMap(0));

        // Add opcode to the map
        self.emit(IR::PushString("opcode".to_string()));
        self.emit(IR::PushString(opcode));
        self.emit(IR::SetKey);

        // Add operands array to the map
        self.emit(IR::PushString("operands".to_string()));

        // Create an array for operands
        self.emit(IR::CreateArray(operands.len()));

        // Add each operand to the array
        for (i, operand) in operands.iter().enumerate() {
            self.emit(IR::PushString(operand.clone()));
            // Use i as the index for the array
            self.emit(IR::PushNumber(i as f64));
            self.emit(IR::SetIndex);
        }

        // Set the operands array in the map
        self.emit(IR::SetKey);

        // Store the IR in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_codegen_statement(&mut self, name: String, target: String, instructions: Vec<(String, Expression)>) {
        // Create an empty map for the codegen
        self.emit(IR::CreateMap(0));

        // Add target to the map
        self.emit(IR::PushString("architecture".to_string()));
        self.emit(IR::PushString(target));
        self.emit(IR::SetKey);

        // Add instructions to the map
        for (key, value) in instructions {
            // Push the key
            self.emit(IR::PushString(key));

            // Push the value
            self.compile_expression(value);

            // Set the key-value pair in the map
            self.emit(IR::SetKey);
        }

        // Store the codegen in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_optimize_statement(&mut self, name: String, description: String, passes: Vec<String>) {
        // Create an empty map for the optimize
        self.emit(IR::CreateMap(0));

        // Add description to the map
        self.emit(IR::PushString("description".to_string()));
        self.emit(IR::PushString(description));
        self.emit(IR::SetKey);

        // Add passes array to the map
        self.emit(IR::PushString("passes".to_string()));

        // Create an array for passes
        self.emit(IR::CreateArray(passes.len()));

        // Add each pass to the array
        for (i, pass) in passes.iter().enumerate() {
            self.emit(IR::PushString(pass.clone()));
            // Use i as the index for the array
            self.emit(IR::PushNumber(i as f64));
            self.emit(IR::SetIndex);
        }

        // Set the passes array in the map
        self.emit(IR::SetKey);

        // Store the optimize in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_target_statement(&mut self, name: String, properties: Vec<(String, Expression)>) {
        // Create an empty map for the target
        self.emit(IR::CreateMap(0));

        // Add properties to the map
        for (key, value) in properties {
            // Push the key
            self.emit(IR::PushString(key));

            // Push the value
            self.compile_expression(value);

            // Set the key-value pair in the map
            self.emit(IR::SetKey);
        }

        // Store the target in a variable
        self.emit(IR::StoreVar(name));
    }

    fn compile_attribute_statement(&mut self, name: String, values: Vec<(String, Expression)>) {
        // Create an empty map for the attribute
        self.emit(IR::CreateMap(0));

        // Add values to the map
        for (key, value) in values {
            // Push the key
            self.emit(IR::PushString(key));

            // Push the value
            self.compile_expression(value);

            // Set the key-value pair in the map
            self.emit(IR::SetKey);
        }

        // Store the attribute in a variable
        self.emit(IR::StoreVar(name));
    }

    // Helper methods for library functions

    /// Dynamically scan and register all library functions from properties/libs
    pub fn register_all_libraries(&mut self, libs_dir: &str) {
        let paths = match fs::read_dir(libs_dir) {
            Ok(p) => p,
            Err(_) => return,
        };

        // Simple pattern matching instead of regex
        for entry in paths {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "rzn" {
                        let lib_name = path.file_stem().unwrap().to_string_lossy().to_string();
                        if let Ok(content) = fs::read_to_string(&path) {
                            let mut current_class = None;
                            for line in content.lines() {
                                let line = line.trim();
                                // Extract class name
                                if line.starts_with("class ") {
                                    let parts: Vec<&str> = line.split_whitespace().collect();
                                    if parts.len() >= 2 {
                                        current_class = Some(parts[1].trim_matches('{').to_string());
                                    }
                                }
                                // Extract static method
                                else if line.starts_with("static ") {
                                    if let Some(class) = &current_class {
                                        let parts: Vec<&str> = line.split_whitespace().collect();
                                        if parts.len() >= 2 {
                                            let fn_name = parts[1].split('(').next().unwrap_or(parts[1]);
                                            let full_name = format!("{}.{}", class, fn_name);
                                            self.function_table.define(&format!("{}.{}", lib_name, full_name), self.ir.len());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // All library functions are now dynamically registered from the .rzn files
    // No need for individual define_*_lib_functions methods

    // Performance and Type Safety Compilation Functions

    // Compile const declaration
    fn compile_const_declaration(&mut self, name: String, value: Expression) {
        if !self.clean_output {
            println!("[Compiler] Constant declaration: {}", name);
        }

        // Define the constant in the symbol table
        self.symbol_table.define(&name);

        // Store the variable type for future type checking
        self.variable_types.insert(name.clone(), "const".to_string());

        // Compile the initializer expression
        self.compile_expression(value);

        // Store the value in the constant
        self.emit(IR::StoreVar(name.clone()));

        // For now, constants are just regular variables
        // In a full implementation, we would add runtime checks to prevent modification
    }

    // Compile enum declaration
    fn compile_enum_declaration(&mut self, name: String, variants: Vec<(String, Option<Expression>)>) {
        if !self.clean_output {
            println!("[Compiler] Enum declaration: {}", name);
        }

        // Define the enum in the symbol table
        self.symbol_table.define(&name);

        // Create a map to store the enum variants
        self.emit(IR::CreateMap(variants.len()));

        // Add each variant to the map
        let mut variant_index = 0;
        for (variant_name, variant_value) in variants {
            // Push the map
            self.emit(IR::Dup);

            // Push the variant name as key
            self.emit(IR::PushString(variant_name.clone()));

            // Push the variant value or its index if not specified
            if let Some(value) = variant_value {
                self.compile_expression(value);
            } else {
                self.emit(IR::PushNumber(variant_index as f64));
                variant_index += 1;
            }

            // Set the key-value pair in the map
            self.emit(IR::SetKey);
        }

        // Store the enum in a variable
        self.emit(IR::StoreVar(name));
    }

    // Compile inline function declaration
    fn compile_inline_function_declaration(&mut self, name: String, parameters: Vec<String>, body: Vec<Statement>) {
        if !self.clean_output {
            println!("[Compiler] Inline function declaration: {}", name);
        }

        // For now, inline functions are compiled the same way as regular functions
        // In a full implementation, the compiler would perform inlining optimizations

        // Generate a label for the function
        let function_label = self.generate_label("function");
        let end_label = self.generate_label("end");

        // Skip over the function body during normal execution
        // We need to add a placeholder jump that we'll fix later
        let jump_pos = self.emit(IR::Jump(0));

        // Define the function in the function table
        let function_address = self.emit_label(&function_label);
        self.function_table.define(&name, function_address);

        // Create a new scope for the function
        self.enter_scope();

        // Define parameters in the symbol table
        for param in &parameters {
            self.symbol_table.define(param);
        }

        // Compile function body
        for stmt in body {
            self.compile_statement(stmt);
        }

        // If no explicit return, return null
        self.emit(IR::PushNull);
        self.emit(IR::Return);

        // Leave function scope
        self.leave_scope();

        // Mark the end of the function
        let end_pos = self.emit_label(&end_label);

        // Fix the jump instruction to point to the end label position
        self.replace_instruction(jump_pos, IR::Jump(end_pos));
    }

    // Compile final class declaration
    fn compile_final_class_declaration(&mut self, name: String, body: Vec<Statement>) {
        if !self.clean_output {
            println!("[Compiler] Final class declaration: {}", name);
        }

        // For now, final classes are compiled the same way as regular classes
        // In a full implementation, the compiler would add checks to prevent inheritance

        // Create a new scope for the class
        self.enter_scope();

        // Store class name in symbol table
        self.symbol_table.define(&name);

        // Emit class definition
        self.emit(IR::PushString(name.clone()));

        // Compile class body
        for stmt in body {
            self.compile_statement(stmt);
        }

        // Leave class scope
        self.leave_scope();

        // Call class definition function
        self.emit(IR::Call("__define_class".to_string(), 1));
    }

    // Compile volatile variable declaration
    fn compile_volatile_declaration(&mut self, var_type: String, name: String, value: Option<Expression>) {
        if !self.clean_output {
            println!("[Compiler] Volatile variable declaration: {}", name);
        }

        // For now, volatile variables are compiled the same way as regular variables
        // In a full implementation, the compiler would add memory barriers or other synchronization

        // Define the variable in the symbol table
        self.symbol_table.define(&name);

        // Store the variable type for future type checking
        self.variable_types.insert(name.clone(), format!("volatile_{}", var_type));

        // Compile the initializer expression if it exists
        if let Some(expr) = value {
            // Type checking based on variable type
            match var_type.as_str() {
                // Numeric types
                "let" | "sum" | "diff" | "prod" | "div" | "mod" => {
                    // Check that the value is a number
                    if !self.is_number_expression(&expr) {
                        self.errors.push(format!("Type error: 'volatile {}' variables can only be used with numeric values, but '{}' was assigned a non-numeric value", var_type, name));
                    }
                },
                // String types
                "take" | "text" | "concat" | "slice" => {
                    // Check that the value is a string
                    if !self.is_string_expression(&expr) {
                        self.errors.push(format!("Type error: 'volatile {}' variables can only be used with string values, but '{}' was assigned a non-string value", var_type, name));
                    }
                },
                // Boolean types
                "hold" => {
                    // Check that the value is a boolean
                    if !self.is_boolean_expression(&expr) {
                        self.errors.push(format!("Type error: 'volatile {}' variables can only be used with boolean values, but '{}' was assigned a non-boolean value", var_type, name));
                    }
                },
                // Generic types - no type checking needed
                "put" | "list" | "arr" | "map" | "store" | "box" | "ref" => {
                    // These can hold any type, so no type checking needed
                },
                _ => {
                    // For new variable types not explicitly handled
                    // For now, we don't do type checking on these
                }
            }

            self.compile_expression(expr);
        } else {
            // If no initializer, push null as the default value
            self.emit(IR::PushNull);
        }

        // Store the value in the variable
        self.emit(IR::StoreVar(name));
    }
}
