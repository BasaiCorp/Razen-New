use std::collections::HashMap;
use std::path::Path;

use crate::ast::{Program, Statement, Expression};
use crate::token::{Token, TokenType};
use crate::lexer::Lexer;

// Define operator precedence levels
#[derive(PartialEq, PartialOrd, Debug)]
enum Precedence {
    Lowest,
    Assignment,  // =
    LogicalOr,   // ||
    LogicalAnd,  // &&
    Equals,      // ==, !=
    LessGreater, // >, <, >=, <=
    Sum,         // +, -
    Product,     // *, /, %
    Power,       // **
    Prefix,      // -X, !X
    Namespace,   // namespace::function
    Call,        // myFunction(X)
    Index,       // array[index]
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    // Maps for prefix and infix parsing functions
    prefix_parse_fns: HashMap<TokenType, fn(&mut Parser) -> Option<Expression>>,
    infix_parse_fns: HashMap<TokenType, fn(&mut Parser, Expression) -> Option<Expression>>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        
        let mut parser = Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        
        // Register prefix parse functions
        parser.register_prefix(TokenType::Identifier, Parser::parse_identifier);
        parser.register_prefix(TokenType::StringLiteral, Parser::parse_string_literal);
        parser.register_prefix(TokenType::NumberLiteral, Parser::parse_number_literal);
        parser.register_prefix(TokenType::True, Parser::parse_boolean_literal);
        parser.register_prefix(TokenType::False, Parser::parse_boolean_literal);
        parser.register_prefix(TokenType::Null, Parser::parse_null_literal);
        parser.register_prefix(TokenType::LeftParen, Parser::parse_grouped_expression);
        parser.register_prefix(TokenType::LeftBracket, Parser::parse_array_literal);
        parser.register_prefix(TokenType::LeftBrace, Parser::parse_map_literal);
        parser.register_prefix(TokenType::Minus, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::Not, Parser::parse_prefix_expression);
        
        // Register mathematical keywords as identifier parsers
        // Mathematical variable tokens removed
        // Using Num token instead for numeric variables
        
        // String operations are handled with built-in operators
        
        // Register list and array keywords as identifier parsers
        parser.register_prefix(TokenType::List, Parser::parse_identifier);
        parser.register_prefix(TokenType::Arr, Parser::parse_identifier);
        parser.register_prefix(TokenType::Append, Parser::parse_identifier);
        parser.register_prefix(TokenType::Remove, Parser::parse_identifier);
        
        // Register dictionary/map keywords as identifier parsers
        parser.register_prefix(TokenType::Map, Parser::parse_identifier);
        parser.register_prefix(TokenType::Key, Parser::parse_identifier);
        parser.register_prefix(TokenType::Value, Parser::parse_identifier);
        
        // Date and time operations are handled with libraries
        
        // Register user-defined keywords as identifier parsers
        parser.register_prefix(TokenType::Store, Parser::parse_identifier);
        parser.register_prefix(TokenType::Box, Parser::parse_identifier);
        parser.register_prefix(TokenType::Ref, Parser::parse_identifier);
        
        // Register logical keywords as identifier parsers
        parser.register_prefix(TokenType::Is, Parser::parse_identifier);
        parser.register_prefix(TokenType::When, Parser::parse_identifier);
        
        // Register control flow keywords as identifier parsers
        parser.register_prefix(TokenType::If, Parser::parse_identifier);
        parser.register_prefix(TokenType::Else, Parser::parse_identifier);
        parser.register_prefix(TokenType::For, Parser::parse_identifier);
        parser.register_prefix(TokenType::While, Parser::parse_identifier);
        parser.register_prefix(TokenType::Load, Parser::parse_identifier);
        
        // Register library tokens as identifiers
        parser.register_prefix(TokenType::Random, Parser::parse_identifier);
        parser.register_prefix(TokenType::HTLib, Parser::parse_identifier);
        parser.register_prefix(TokenType::Coin, Parser::parse_identifier);
        parser.register_prefix(TokenType::MathLib, Parser::parse_identifier);
        parser.register_prefix(TokenType::Ping, Parser::parse_identifier);
        parser.register_prefix(TokenType::Bolt, Parser::parse_identifier);
        parser.register_prefix(TokenType::Seed, Parser::parse_identifier);
        parser.register_prefix(TokenType::NetLib, Parser::parse_identifier);
        parser.register_prefix(TokenType::File, Parser::parse_identifier);
        parser.register_prefix(TokenType::Json, Parser::parse_identifier);
        parser.register_prefix(TokenType::Date, Parser::parse_identifier);
        parser.register_prefix(TokenType::StrLib, Parser::parse_identifier);
        parser.register_prefix(TokenType::ArrLib, Parser::parse_identifier);
        parser.register_prefix(TokenType::Os, Parser::parse_identifier);
        parser.register_prefix(TokenType::Regex, Parser::parse_identifier);
        parser.register_prefix(TokenType::Crypto, Parser::parse_identifier);
        parser.register_prefix(TokenType::Color, Parser::parse_identifier);
        parser.register_prefix(TokenType::System, Parser::parse_identifier);
        parser.register_prefix(TokenType::Ui, Parser::parse_identifier);
        parser.register_prefix(TokenType::Storage, Parser::parse_identifier);
        parser.register_prefix(TokenType::Audio, Parser::parse_identifier);
        parser.register_prefix(TokenType::Image, Parser::parse_identifier);
        parser.register_prefix(TokenType::Validation, Parser::parse_identifier);
        parser.register_prefix(TokenType::LogLib, Parser::parse_identifier);
        parser.register_prefix(TokenType::Uuid, Parser::parse_identifier);
        
        // Self-compilation library tokens
        parser.register_prefix(TokenType::MemoryLib, Parser::parse_identifier);    // Memory library
        parser.register_prefix(TokenType::BinaryLib, Parser::parse_identifier);    // Binary file operations library
        parser.register_prefix(TokenType::BitwiseLib, Parser::parse_identifier);   // Bitwise operations library
        parser.register_prefix(TokenType::SystemLib, Parser::parse_identifier);    // System operations library
        parser.register_prefix(TokenType::ProcessLib, Parser::parse_identifier);   // Process management library
        parser.register_prefix(TokenType::ThreadLib, Parser::parse_identifier);    // Thread management library
        parser.register_prefix(TokenType::CompilerLib, Parser::parse_identifier);  // Compiler operations library
        
        // 17 - Compiler Construction Keywords
        parser.register_prefix(TokenType::Token, Parser::parse_identifier);        // Token representation
        parser.register_prefix(TokenType::Lexer, Parser::parse_identifier);        // Lexical analyzer
        parser.register_prefix(TokenType::Parser, Parser::parse_identifier);       // Syntax analyzer
        parser.register_prefix(TokenType::AST, Parser::parse_identifier);          // Abstract syntax tree
        parser.register_prefix(TokenType::Node, Parser::parse_identifier);         // AST node
        parser.register_prefix(TokenType::Visitor, Parser::parse_identifier);      // AST visitor pattern
        parser.register_prefix(TokenType::Symbol, Parser::parse_identifier);       // Symbol table entry
        parser.register_prefix(TokenType::Scope, Parser::parse_identifier);        // Scope management
        parser.register_prefix(TokenType::Type, Parser::parse_identifier);         // Type checking
        parser.register_prefix(TokenType::IR, Parser::parse_identifier);           // Intermediate representation
        parser.register_prefix(TokenType::CodeGen, Parser::parse_identifier);      // Code generation
        parser.register_prefix(TokenType::Optimize, Parser::parse_identifier);     // Optimization
        parser.register_prefix(TokenType::Target, Parser::parse_identifier);       // Target code
        parser.register_prefix(TokenType::Grammar, Parser::parse_identifier);      // Grammar definition
        parser.register_prefix(TokenType::Rule, Parser::parse_identifier);         // Grammar rule
        parser.register_prefix(TokenType::Attribute, Parser::parse_identifier);    // Semantic attribute
        
        // 18 - Compiler Construction Libraries
        parser.register_prefix(TokenType::LexerLib, Parser::parse_identifier);     // Lexical analysis library
        parser.register_prefix(TokenType::ParserLib, Parser::parse_identifier);    // Syntax analysis library
        parser.register_prefix(TokenType::ASTLib, Parser::parse_identifier);       // AST manipulation library
        parser.register_prefix(TokenType::SymbolLib, Parser::parse_identifier);    // Symbol table library
        parser.register_prefix(TokenType::TypeLib, Parser::parse_identifier);      // Type checking library
        parser.register_prefix(TokenType::IRLib, Parser::parse_identifier);        // IR operations library
        parser.register_prefix(TokenType::CodeGenLib, Parser::parse_identifier);   // Code generation library
        parser.register_prefix(TokenType::OptimizeLib, Parser::parse_identifier);  // Optimization library
        
        parser.register_prefix(TokenType::Read, Parser::parse_identifier);
        parser.register_prefix(TokenType::Debug, Parser::parse_identifier);
        parser.register_prefix(TokenType::Assert, Parser::parse_identifier);
        parser.register_prefix(TokenType::Trace, Parser::parse_identifier);
        parser.register_prefix(TokenType::Show, Parser::parse_identifier);
        parser.register_prefix(TokenType::Exit, Parser::parse_identifier);
        parser.register_prefix(TokenType::Api, Parser::parse_identifier);
        parser.register_prefix(TokenType::Call, Parser::parse_identifier);
        parser.register_prefix(TokenType::Connect, Parser::parse_identifier);
        parser.register_prefix(TokenType::To, Parser::parse_identifier);
        parser.register_prefix(TokenType::Import, Parser::parse_identifier);
        parser.register_prefix(TokenType::Export, Parser::parse_identifier);
        parser.register_prefix(TokenType::From, Parser::parse_identifier);
        parser.register_prefix(TokenType::As, Parser::parse_identifier);
        parser.register_prefix(TokenType::Get, Parser::parse_identifier);
        parser.register_prefix(TokenType::Post, Parser::parse_identifier);
        
        // Register infix parse functions
        parser.register_infix(TokenType::Plus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Minus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Slash, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Asterisk, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Percent, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Power, Parser::parse_infix_expression);
        parser.register_infix(TokenType::FloorDiv, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Equal, Parser::parse_infix_expression);
        parser.register_infix(TokenType::NotEqual, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Less, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LessEqual, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Greater, Parser::parse_infix_expression);
        parser.register_infix(TokenType::GreaterEqual, Parser::parse_infix_expression);
        parser.register_infix(TokenType::And, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Or, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Assign, Parser::parse_assignment_expression);
        parser.register_infix(TokenType::PlusAssign, Parser::parse_assignment_expression);
        parser.register_infix(TokenType::MinusAssign, Parser::parse_assignment_expression);
        parser.register_infix(TokenType::AsteriskAssign, Parser::parse_assignment_expression);
        parser.register_infix(TokenType::SlashAssign, Parser::parse_assignment_expression);
        parser.register_infix(TokenType::PercentAssign, Parser::parse_assignment_expression);
        parser.register_infix(TokenType::LeftParen, Parser::parse_call_expression);
        parser.register_infix(TokenType::ColonColon, Parser::parse_namespace_expression);
        parser.register_infix(TokenType::LeftBracket, Parser::parse_index_expression);
        parser.register_infix(TokenType::Dot, Parser::parse_dot_expression);
        
        parser
    }
    
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        match Lexer::from_file(path) {
            Ok(lexer) => Ok(Parser::new(lexer)),
            Err(e) => Err(e),
        }
    }
    
    fn register_prefix(&mut self, token_type: TokenType, func: fn(&mut Parser) -> Option<Expression>) {
        self.prefix_parse_fns.insert(token_type, func);
    }
    
    fn register_infix(&mut self, token_type: TokenType, func: fn(&mut Parser, Expression) -> Option<Expression>) {
        self.infix_parse_fns.insert(token_type, func);
    }
    
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
    
    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }
    
    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }
    
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }
    
    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "Expected next token to be {:?}, got {:?} instead at line {}, column {}",
            token_type,
            self.peek_token.token_type,
            self.peek_token.line,
            self.peek_token.column
        );
        self.errors.push(msg);
    }
    
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }
    
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        
        while !self.current_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        
        program
    }
    
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            // Variable declaration keywords
            TokenType::Num | TokenType::Str | TokenType::Bool | TokenType::Var | TokenType::Const |
            TokenType::List | TokenType::Arr | TokenType::Append | TokenType::Remove |
            TokenType::Map | TokenType::Key | TokenType::Value |
            TokenType::Store | TokenType::Box | TokenType::Ref => self.parse_variable_declaration(),
            
            // 17 - Compiler Construction Keywords
            TokenType::Grammar => self.parse_grammar_statement(),
            TokenType::Token => self.parse_token_statement(),
            TokenType::Lexer => self.parse_lexer_statement(),
            TokenType::Parser => self.parse_parser_statement(),
            TokenType::AST => self.parse_ast_statement(),
            TokenType::Node => self.parse_node_statement(),
            TokenType::Visitor => self.parse_visitor_statement(),
            TokenType::Symbol => self.parse_symbol_statement(),
            TokenType::Scope => self.parse_scope_statement(),
            TokenType::Type => self.parse_type_statement(),
            TokenType::IR => self.parse_ir_statement(),
            TokenType::CodeGen => self.parse_codegen_statement(),
            TokenType::Optimize => self.parse_optimize_statement(),
            TokenType::Target => self.parse_target_statement(),
            TokenType::Rule => self.parse_rule_statement(),
            TokenType::Attribute => self.parse_attribute_statement(),
            
            TokenType::Fun => self.parse_function_declaration(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::Else => self.parse_else_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::Break => self.parse_break_statement(),
            TokenType::Continue => self.parse_continue_statement(),
            TokenType::Show => self.parse_show_statement(),
            TokenType::Read => self.parse_read_statement(),
            TokenType::Exit => self.parse_exit_statement(),
            TokenType::Load => self.parse_load_statement(),
            TokenType::Try => self.parse_try_statement(),
            TokenType::Throw => self.parse_throw_statement(),
            TokenType::DocumentType => self.parse_document_type_declaration(),
            TokenType::Is => self.parse_is_statement(),
            TokenType::When => self.parse_when_statement(),
            
            // Module system
            TokenType::Use => self.parse_module_import(),
            TokenType::Export => self.parse_module_export(),
            TokenType::Import => self.parse_import_statement(),
            
            // Developer tools
            TokenType::Debug => self.parse_debug_statement(),
            TokenType::Assert => self.parse_assert_statement(),
            TokenType::Trace => self.parse_trace_statement(),
            
            // OOP Keywords
            TokenType::Class => self.parse_class_declaration(),
            TokenType::Final => self.parse_final_class_declaration(),
            
            // Performance and Type Safety Keywords
            TokenType::Const => self.parse_const_declaration(),
            TokenType::Enum => self.parse_enum_declaration(),
            TokenType::Inline => self.parse_inline_function_declaration(),
            TokenType::Volatile => self.parse_volatile_declaration(),
            
            // API Keywords
            TokenType::Api => self.parse_api_declaration(),
            TokenType::Call => self.parse_api_call(),
            
            // Connection Keywords
            TokenType::Connect => self.parse_connect_statement(),
            
            // Library Keywords
            TokenType::Lib => self.parse_lib_statement(),
            
            TokenType::Comment => {
                // Skip comments and return None to continue parsing
                None
            },
            _ => self.parse_expression_statement(),
        }
    }
    
    fn parse_variable_declaration(&mut self) -> Option<Statement> {
        let var_type = self.current_token.literal.clone();
        let token_type = self.current_token.token_type.clone();
        let token_line = self.current_token.line;
        let token_column = self.current_token.column;
        
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        let value = self.parse_expression(Precedence::Lowest)?;
        
        // Type checking based on variable declaration token
        match token_type {
            // 1. General Purpose Variables
            TokenType::Num => {
                // num => for numeric variables (integers, floats)
                // Also allow function calls and expressions that might return numbers
                match value {
                    Expression::NumberLiteral(_) => {},
                    Expression::InfixExpression { .. } => {}, // Allow expressions that might result in numbers
                    Expression::PrefixExpression { .. } => {}, // Allow expressions that might result in numbers
                    Expression::Identifier(_) => {}, // Allow identifiers (runtime check needed)
                    Expression::CallExpression { .. } => {}, // Allow function calls (runtime check needed)
                    Expression::LibraryCall { .. } => {}, // Allow library function calls (like TimeLib[now]())
                    _ => {
                        // Only show warning for obvious mismatches like strings and booleans
                        if let Expression::StringLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: 'num' should be used for numeric values at line {}, column {}",
                                token_line, token_column
                            ));
                        } else if let Expression::BooleanLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: 'num' should be used for numeric values at line {}, column {}",
                                token_line, token_column
                            ));
                        }
                        // Allow other types to pass through for flexibility
                    }
                }
            },
            TokenType::Str => {
                // str => for string variables and text manipulation
                match value {
                    Expression::StringLiteral(_) => {},
                    Expression::InfixExpression { .. } => {}, // Allow expressions that might result in strings
                    Expression::Identifier(_) => {}, // Allow identifiers (runtime check needed)
                    Expression::CallExpression { .. } => {}, // Allow function calls (runtime check needed)
                    Expression::LibraryCall { .. } => {}, // Allow library function calls
                    _ => {
                        // Only show warning for obvious mismatches
                        if let Expression::NumberLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: 'str' should be used for string values at line {}, column {}",
                                token_line, token_column
                            ));
                        } else if let Expression::BooleanLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: 'str' should be used for string values at line {}, column {}",
                                token_line, token_column
                            ));
                        }
                        // Allow other types to pass through for flexibility
                    }
                }
            },
            TokenType::Bool => {
                // bool => for boolean variables and logical conditions
                match value {
                    Expression::BooleanLiteral(_) => {},
                    Expression::InfixExpression { .. } => {}, // Allow expressions that might result in booleans
                    Expression::PrefixExpression { .. } => {}, // Allow expressions that might result in booleans
                    Expression::Identifier(_) => {}, // Allow identifiers (runtime check needed)
                    Expression::CallExpression { .. } => {}, // Allow function calls (runtime check needed)
                    Expression::LibraryCall { .. } => {}, // Allow library function calls
                    _ => {
                        // Only show warning for obvious mismatches
                        if let Expression::NumberLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: 'bool' should be used for boolean values at line {}, column {}",
                                token_line, token_column
                            ));
                        } else if let Expression::StringLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: 'bool' should be used for boolean values at line {}, column {}",
                                token_line, token_column
                            ));
                        }
                        // Allow other types to pass through for flexibility
                    }
                }
            },
            TokenType::Var => {
                // var => for variables of any type (no restrictions)
            },
            
            // String operations are handled with built-in operators
            // Removed string-specific token handling
            
            
            // 5. List & Array Variables
            TokenType::List | TokenType::Arr | TokenType::Append | TokenType::Remove => {
                // Collection variables should be used with array literals or identifiers
                // These tokens are aliases for 'put' when used with collections
                match value {
                    Expression::ArrayLiteral { .. } => {},
                    Expression::Identifier(_) => {}, // Allow identifiers (runtime check needed)
                    Expression::CallExpression { .. } => {}, // Allow function calls (runtime check needed)
                    Expression::InfixExpression { .. } => {}, // Allow expressions that might result in arrays
                    Expression::LibraryCall { .. } => {}, // Allow library function calls
                    _ => {
                        // Only show warning for obvious mismatches like simple literals
                        if let Expression::NumberLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: '{}' should be used for collection values at line {}, column {}",
                                var_type, token_line, token_column
                            ));
                        } else if let Expression::StringLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: '{}' should be used for collection values at line {}, column {}",
                                var_type, token_line, token_column
                            ));
                        } else if let Expression::BooleanLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: '{}' should be used for collection values at line {}, column {}",
                                var_type, token_line, token_column
                            ));
                        }
                        // Allow other types to pass through for flexibility
                    }
                }
            },
            
            // 6. Dictionary/Map Variables
            TokenType::Map | TokenType::Key | TokenType::Value => {
                // Map variables should be used with map literals or identifiers
                // These tokens are aliases for 'put' when used with dictionaries/maps
                match value {
                    Expression::MapLiteral { .. } => {},
                    Expression::ArrayLiteral { .. } => {}, // Allow arrays for key-value pairs
                    Expression::Identifier(_) => {}, // Allow identifiers (runtime check needed)
                    Expression::CallExpression { .. } => {}, // Allow function calls (runtime check needed)
                    Expression::LibraryCall { .. } => {}, // Allow library function calls
                    _ => {
                        // Only show warning for obvious mismatches like simple literals
                        if let Expression::NumberLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: '{}' should be used for map/dictionary values at line {}, column {}",
                                var_type, token_line, token_column
                            ));
                        } else if let Expression::StringLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: '{}' should be used for map/dictionary values at line {}, column {}",
                                var_type, token_line, token_column
                            ));
                        } else if let Expression::BooleanLiteral(_) = value {
                            self.errors.push(format!(
                                "Type mismatch: '{}' should be used for map/dictionary values at line {}, column {}",
                                var_type, token_line, token_column
                            ));
                        }
                        // Allow other types to pass through for flexibility
                    }
                }
            },
            
            // Date & Time operations are handled with libraries
            
            
            // 8. User-Defined Variables
            TokenType::Store | TokenType::Box | TokenType::Ref => {
                // These can be used with any type, but we'll add specific validation for Ref
                if token_type == TokenType::Ref {
                    match value {
                        Expression::Identifier(_) => {}, // Ref should point to an existing variable
                        Expression::CallExpression { .. } => {}, // Allow function calls (runtime check needed)
                        Expression::LibraryCall { .. } => {}, // Allow library function calls
                        _ => {
                            // Only show warning for obvious mismatches like literals
                            if let Expression::NumberLiteral(_) = value {
                                self.errors.push(format!(
                                    "Type mismatch: 'ref' should be used with an identifier at line {}, column {}",
                                    token_line, token_column
                                ));
                            } else if let Expression::StringLiteral(_) = value {
                                self.errors.push(format!(
                                    "Type mismatch: 'ref' should be used with an identifier at line {}, column {}",
                                    token_line, token_column
                                ));
                            } else if let Expression::BooleanLiteral(_) = value {
                                self.errors.push(format!(
                                    "Type mismatch: 'ref' should be used with an identifier at line {}, column {}",
                                    token_line, token_column
                                ));
                            }
                            // Allow other types to pass through for flexibility
                        }
                    }
                }
                // Store and Box can hold any type, so no specific validation
            },
            
            // Default case for any other tokens
            _ => {}
        }
        
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::VariableDeclaration {
            var_type,
            name,
            value: Some(value),
        })
    }
    
    fn parse_function_declaration(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }
        
        let parameters = self.parse_function_parameters();
        
        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }
        
        let body = self.parse_block_statement();
        
        Some(Statement::FunctionDeclaration {
            name,
            parameters,
            body,
        })
    }
    
    fn parse_function_parameters(&mut self) -> Vec<String> {
        let mut parameters = Vec::new();
        
        if self.peek_token_is(TokenType::RightParen) {
            self.next_token();
            return parameters;
        }
        
        self.next_token();
        
        parameters.push(self.current_token.literal.clone());
        
        while self.peek_token_is(TokenType::Comma) {
            self.next_token(); // Skip comma
            self.next_token(); // Move to next parameter
            parameters.push(self.current_token.literal.clone());
        }
        
        if !self.expect_peek(TokenType::RightParen) {
            return Vec::new();
        }
        
        parameters
    }
    
    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        
        let value = if self.current_token_is(TokenType::Semicolon) {
            None
        } else {
            // Parse the expression for the return value
            let expr = self.parse_expression(Precedence::Lowest)?;
            Some(expr)
        };
        
        // Optional semicolon - consume if present but don't require it
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ReturnStatement { value })
    }
    
    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Optional semicolon - consume if present but don't require it
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ExpressionStatement { expression: expr })
    }
    
    fn parse_block_statement(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        
        // Consume the opening brace
        self.next_token();
        
        // Parse statements until we reach the closing brace or EOF
        while !self.current_token_is(TokenType::RightBrace) && !self.current_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            
            // Only advance if we're not at the end of the block
            // This helps prevent skipping over the closing brace
            if !self.peek_token_is(TokenType::RightBrace) {
                self.next_token();
            } else {
                // If next token is right brace, consume it and break
                self.next_token();
                break;
            }
        }
        
        statements
    }
    
    fn parse_if_statement(&mut self) -> Option<Statement> {
        self.next_token();
        
        // Parse the condition directly - no need for parentheses around the condition
        let condition = self.parse_expression(Precedence::Lowest)?;
        
        // Parse consequence
        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }
        let consequence = self.parse_block_statement();
        
        // Parse optional else clause
        let alternative = if self.peek_token_is(TokenType::Else) {
            self.next_token(); // consume 'else'
            if !self.expect_peek(TokenType::LeftBrace) {
                return None;
            }
            Some(self.parse_block_statement())
        } else {
            None
        };
        
        Some(Statement::IfStatement {
            condition,
            consequence,
            alternative,
        })
    }
    
    fn parse_while_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }
        
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        
        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }
        
        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }
        
        let body = self.parse_block_statement();
        
        Some(Statement::WhileStatement {
            condition,
            body,
        })
    }
    
    fn parse_for_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }
        
        self.next_token();
        let iterator = self.current_token.literal.clone();
        
        if !self.expect_peek(TokenType::In) {
            return None;
        }
        
        self.next_token();
        let iterable = self.parse_expression(Precedence::Lowest)?;
        
        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }
        
        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }
        
        let body = self.parse_block_statement();
        
        Some(Statement::ForStatement {
            iterator,
            iterable,
            body,
        })
    }
    
    fn parse_break_statement(&mut self) -> Option<Statement> {
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::BreakStatement)
    }
    
    fn parse_continue_statement(&mut self) -> Option<Statement> {
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ContinueStatement)
    }
    
    fn parse_show_statement(&mut self) -> Option<Statement> {
        self.next_token();
        
        // Check if there's a color parameter in parentheses
        let color = if self.current_token_is(TokenType::LeftParen) {
            self.next_token(); // Consume the left paren
            
            // Get the color name
            let color_name = if !self.current_token.literal.is_empty() {
                self.current_token.literal.clone()
            } else {
                self.current_token.token_type.to_string()
            };
            
            self.next_token(); // Consume the color name
            
            // Expect right paren
            if !self.current_token_is(TokenType::RightParen) {
                self.errors.push(format!("Expected right parenthesis after color name, got {:?}", self.current_token.token_type));
                return None;
            }
            
            self.next_token(); // Consume the right paren
            Some(color_name)
        } else {
            None
        };
        
        let value = self.parse_expression(Precedence::Lowest)?;
        
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ShowStatement { value, color })
    }
    
    fn parse_try_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }
        
        let try_block = self.parse_block_statement();
        
        let mut catch_param = None;
        let catch_block = if self.peek_token_is(TokenType::Catch) {
            self.next_token();
            
            // Parse the catch parameter if it exists
            if self.peek_token_is(TokenType::LeftParen) {
                self.next_token(); // consume '('
                
                if self.peek_token_is(TokenType::Identifier) {
                    self.next_token(); // consume identifier
                    catch_param = Some(self.current_token.literal.clone());
                    
                    if !self.expect_peek(TokenType::RightParen) {
                        return None;
                    }
                } else {
                    if !self.expect_peek(TokenType::RightParen) {
                        return None;
                    }
                }
            }
            
            if !self.expect_peek(TokenType::LeftBrace) {
                return None;
            }
            
            Some(self.parse_block_statement())
        } else {
            None
        };
        
        let finally_block = if self.peek_token_is(TokenType::Finally) {
            self.next_token();
            
            if !self.expect_peek(TokenType::LeftBrace) {
                return None;
            }
            
            Some(self.parse_block_statement())
        } else {
            None
        };
        
        Some(Statement::TryStatement {
            try_block,
            catch_param,
            catch_block,
            finally_block,
        })
    }
    
    fn parse_throw_statement(&mut self) -> Option<Statement> {
        self.next_token();
        
        let value = self.parse_expression(Precedence::Lowest)?;
        
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ThrowStatement { value })
    }
    
    // This is a placeholder to avoid duplicate function error
    // The actual implementation is at line ~1650
    
    // Compiler Construction Parsing Functions
    
    fn parse_grammar_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'grammar' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains grammar properties
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract properties from the map literal
        let properties = match expr {
            Expression::MapLiteral { pairs } => {
                // Convert the map pairs to (String, Expression) format
                pairs.into_iter()
                    .filter_map(|(key, value)| {
                        if let Expression::StringLiteral(key_str) = key {
                            Some((key_str, value))
                        } else {
                            None
                        }
                    })
                    .collect()
            },
            _ => Vec::new(),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::GrammarStatement {
            name,
            properties,
        })
    }
    
    fn parse_token_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'token' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the string literal that contains the token pattern
        let pattern = match self.parse_expression(Precedence::Lowest)? {
            Expression::StringLiteral(pattern) => pattern,
            _ => String::new(),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::TokenStatement {
            name,
            pattern,
        })
    }
    
    fn parse_lexer_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'lexer' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains lexer configuration
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract configuration from the map literal
        let config = match expr {
            Expression::MapLiteral { pairs } => {
                // Convert the map pairs to (String, Expression) format
                pairs.into_iter()
                    .filter_map(|(key, value)| {
                        if let Expression::StringLiteral(key_str) = key {
                            Some((key_str, value))
                        } else {
                            None
                        }
                    })
                    .collect()
            },
            _ => Vec::new(),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::LexerStatement {
            name,
            config,
        })
    }
    
    fn parse_parser_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'parser' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains parser configuration
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract configuration from the map literal
        let config = match expr {
            Expression::MapLiteral { pairs } => {
                // Convert the map pairs to (String, Expression) format
                pairs.into_iter()
                    .filter_map(|(key, value)| {
                        if let Expression::StringLiteral(key_str) = key {
                            Some((key_str, value))
                        } else {
                            None
                        }
                    })
                    .collect()
            },
            _ => Vec::new(),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ParserStatement {
            name,
            config,
        })
    }
    
    fn parse_ast_statement(&mut self) -> Option<Statement> {
        // For now, treat AST statements as variable declarations
        self.parse_variable_declaration()
    }
    
    fn parse_node_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'node' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains node properties
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract properties from the map literal
        let properties = match expr {
            Expression::MapLiteral { pairs } => {
                // Convert the map pairs to (String, Expression) format
                pairs.into_iter()
                    .filter_map(|(key, value)| {
                        if let Expression::StringLiteral(key_str) = key {
                            Some((key_str, value))
                        } else {
                            None
                        }
                    })
                    .collect()
            },
            _ => Vec::new(),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::NodeStatement {
            name,
            properties,
        })
    }
    
    fn parse_rule_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'rule' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal or string literal for the rule
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        let (production, node_type) = match expr {
            Expression::MapLiteral { pairs } => {
                // Extract production and node_type from the map
                let mut production = String::new();
                let mut node_type = None;
                
                for (key, value) in pairs {
                    if let Expression::StringLiteral(key_str) = key {
                        match key_str.as_str() {
                            "production" => {
                                if let Expression::StringLiteral(prod) = value {
                                    production = prod;
                                }
                            },
                            "astNode" => {
                                if let Expression::Identifier(node) = value {
                                    node_type = Some(node);
                                }
                            },
                            _ => {}
                        }
                    }
                }
                
                (production, node_type)
            },
            Expression::StringLiteral(prod) => {
                // If it's just a string literal, use it as the production
                (prod, None)
            },
            _ => (String::new(), None),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::RuleStatement {
            name,
            production,
            node_type,
        })
    }
    
    fn parse_visitor_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'visitor' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains visitor configuration
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract methods from the map literal
        let methods = match expr {
            Expression::MapLiteral { pairs } => {
                let mut methods_vec = Vec::new();
                
                for (key, value) in pairs {
                    if let Expression::StringLiteral(key_str) = key {
                        if key_str == "methods" {
                            if let Expression::ArrayLiteral { elements } = value {
                                for elem in elements {
                                    if let Expression::StringLiteral(method) = elem {
                                        methods_vec.push(method);
                                    }
                                }
                            }
                        }
                    }
                }
                
                methods_vec
            },
            _ => Vec::new(),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::VisitorStatement {
            name,
            methods,
        })
    }
    
    fn parse_symbol_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'symbol' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains symbol configuration
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract attributes from the map literal
        let attributes = match expr {
            Expression::MapLiteral { pairs } => {
                let mut attrs_vec = Vec::new();
                
                for (key, value) in pairs {
                    if let Expression::StringLiteral(key_str) = key {
                        if key_str == "attributes" {
                            if let Expression::ArrayLiteral { elements } = value {
                                for elem in elements {
                                    if let Expression::StringLiteral(attr) = elem {
                                        attrs_vec.push(attr);
                                    }
                                }
                            }
                        }
                    }
                }
                
                attrs_vec
            },
            _ => Vec::new(),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::SymbolStatement {
            name,
            attributes,
        })
    }
    
    fn parse_scope_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'scope' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains scope configuration
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract parent from the map literal
        let parent = match expr {
            Expression::MapLiteral { pairs } => {
                let mut parent_opt = None;
                
                for (key, value) in pairs {
                    if let Expression::StringLiteral(key_str) = key {
                        if key_str == "parent" {
                            match value {
                                Expression::Identifier(parent_name) => {
                                    parent_opt = Some(parent_name);
                                },
                                Expression::NullLiteral => {
                                    // Parent is explicitly null
                                    parent_opt = None;
                                },
                                _ => {}
                            }
                        }
                    }
                }
                
                parent_opt
            },
            _ => None,
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ScopeStatement {
            name,
            parent,
        })
    }
    
    fn parse_type_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'type' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains type configuration
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract operations from the map literal
        let operations = match expr {
            Expression::MapLiteral { pairs } => {
                let mut ops_vec = Vec::new();
                
                for (key, value) in pairs {
                    if let Expression::StringLiteral(key_str) = key {
                        if key_str == "operations" {
                            if let Expression::ArrayLiteral { elements } = value {
                                for elem in elements {
                                    if let Expression::StringLiteral(op) = elem {
                                        ops_vec.push(op);
                                    }
                                }
                            }
                        }
                    }
                }
                
                ops_vec
            },
            _ => Vec::new(),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::TypeStatement {
            name,
            operations,
        })
    }
    
    fn parse_ir_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'ir' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains IR configuration
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract opcode and operands from the map literal
        let (opcode, operands) = match expr {
            Expression::MapLiteral { pairs } => {
                let mut opcode_str = String::new();
                let mut operands_vec = Vec::new();
                
                for (key, value) in pairs {
                    if let Expression::StringLiteral(key_str) = key {
                        match key_str.as_str() {
                            "opcode" => {
                                if let Expression::StringLiteral(op) = value {
                                    opcode_str = op;
                                }
                            },
                            "operands" => {
                                if let Expression::ArrayLiteral { elements } = value {
                                    for elem in elements {
                                        if let Expression::StringLiteral(operand) = elem {
                                            operands_vec.push(operand);
                                        }
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
                
                (opcode_str, operands_vec)
            },
            _ => (String::new(), Vec::new()),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::IRStatement {
            name,
            opcode,
            operands,
        })
    }
    
    fn parse_codegen_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'codegen' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains codegen configuration
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract target and instructions from the map literal
        let (target, instructions) = match expr {
            Expression::MapLiteral { pairs } => {
                let mut target_str = String::new();
                let mut instructions_pairs = Vec::new();
                
                for (key, value) in pairs {
                    if let Expression::StringLiteral(key_str) = key {
                        match key_str.as_str() {
                            "architecture" => {
                                if let Expression::StringLiteral(arch) = value {
                                    target_str = arch;
                                }
                            },
                            _ => {
                                // Add all other key-value pairs to instructions
                                instructions_pairs.push((key_str, value));
                            }
                        }
                    }
                }
                
                (target_str, instructions_pairs)
            },
            _ => (String::new(), Vec::new()),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::CodeGenStatement {
            name,
            target,
            instructions,
        })
    }
    
    fn parse_optimize_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'optimize' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains optimize configuration
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract description and passes from the map literal
        let (description, passes) = match expr {
            Expression::MapLiteral { pairs } => {
                let mut desc_str = String::new();
                let mut passes_vec = Vec::new();
                
                for (key, value) in pairs {
                    if let Expression::StringLiteral(key_str) = key {
                        match key_str.as_str() {
                            "description" => {
                                if let Expression::StringLiteral(desc) = value {
                                    desc_str = desc;
                                }
                            },
                            "passes" => {
                                if let Expression::ArrayLiteral { elements } = value {
                                    for elem in elements {
                                        if let Expression::StringLiteral(pass) = elem {
                                            passes_vec.push(pass);
                                        }
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
                
                (desc_str, passes_vec)
            },
            _ => (String::new(), Vec::new()),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::OptimizeStatement {
            name,
            description,
            passes,
        })
    }
    
    fn parse_target_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'target' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains target configuration
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract properties from the map literal
        let properties = match expr {
            Expression::MapLiteral { pairs } => {
                // Convert the map pairs to (String, Expression) format
                pairs.into_iter()
                    .filter_map(|(key, value)| {
                        if let Expression::StringLiteral(key_str) = key {
                            Some((key_str, value))
                        } else {
                            None
                        }
                    })
                    .collect()
            },
            _ => Vec::new(),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::TargetStatement {
            name,
            properties,
        })
    }
    
    fn parse_attribute_statement(&mut self) -> Option<Statement> {
        // Expect identifier after 'attribute' keyword
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        
        // Parse the map literal that contains attribute values
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Extract values from the map literal
        let values = match expr {
            Expression::MapLiteral { pairs } => {
                // Convert the map pairs to (String, Expression) format
                pairs.into_iter()
                    .filter_map(|(key, value)| {
                        if let Expression::StringLiteral(key_str) = key {
                            Some((key_str, value))
                        } else {
                            None
                        }
                    })
                    .collect()
            },
            _ => Vec::new(),
        };
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::AttributeStatement {
            name,
            values,
        })
    }
    
    fn parse_read_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ReadStatement { name })
    }
    
    fn parse_exit_statement(&mut self) -> Option<Statement> {
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ExitStatement)
    }
    
    /// Parse document type declaration (type web; type script; type cli;)
    fn parse_document_type_declaration(&mut self) -> Option<Statement> {
        // Consume the 'type' token
        self.next_token();
        
        // Check if there's an equals sign
        let has_equals = self.current_token_is(TokenType::Assign);
        if has_equals {
            self.next_token(); // Consume the equals sign
        }
        
        // Get the document type (web, script, cli, freestyle)
        if self.current_token_is(TokenType::Identifier) {
            let doc_type = self.current_token.literal.clone();
            
            // Expect semicolon
            if self.peek_token_is(TokenType::Semicolon) {
                self.next_token();
            }
            
            Some(Statement::DocumentTypeDeclaration { doc_type })
        } else {
            self.errors.push(format!("Expected document type after 'type{}', got {:?}", 
                                    if has_equals { " =" } else { "" }, 
                                    self.current_token.token_type));
            None
        }
    }
    
    /// Parse module import statement (use name from "module"; or use name1, name2 from "module"; or use module as alias from "module";)
    fn parse_module_import(&mut self) -> Option<Statement> {
        // Skip 'use' token
        self.next_token();
        
        // Parse import names
        let mut names = Vec::new();
        let mut alias = None;
        
        // First name must be an identifier
        if !self.current_token_is(TokenType::Identifier) {
            self.errors.push(format!("Expected identifier after 'use', got {:?}", self.current_token.token_type));
            return None;
        }
        
        names.push(self.current_token.literal.clone());
        self.next_token();
        
        // Check for 'as' keyword for namespace alias
        if self.current_token_is(TokenType::As) {
            self.next_token(); // Skip 'as'
            
            if !self.current_token_is(TokenType::Identifier) {
                self.errors.push(format!("Expected identifier after 'as', got {:?}", self.current_token.token_type));
                return None;
            }
            
            alias = Some(self.current_token.literal.clone());
            self.next_token();
        } else {
            // Parse additional names if there's a comma
            while self.current_token_is(TokenType::Comma) {
                self.next_token(); // Skip comma
                
                if !self.current_token_is(TokenType::Identifier) {
                    self.errors.push(format!("Expected identifier after comma, got {:?}", self.current_token.token_type));
                    return None;
                }
                
                names.push(self.current_token.literal.clone());
                self.next_token();
            }
        }
        
        // Expect 'from' keyword
        if !self.current_token_is(TokenType::From) {
            self.errors.push(format!("Expected 'from' after import names, got {:?}", self.current_token.token_type));
            return None;
        }
        
        // Skip 'from'
        self.next_token();
        
        // Expect string literal for module path
        if !self.current_token_is(TokenType::StringLiteral) {
            self.errors.push(format!("Expected string literal for module path, got {:?}", self.current_token.token_type));
            return None;
        }
        
        let source = self.current_token.literal.clone();
        
        // Skip string literal
        self.next_token();
        
        // Expect semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ModuleImport { names, alias, source })
    }
    
    /// Parse module export statement (export name;)
    fn parse_module_export(&mut self) -> Option<Statement> {
        // Skip 'export' token
        self.next_token();
        
        if !self.current_token_is(TokenType::Identifier) {
            self.errors.push(format!("Expected identifier after 'export', got {:?}", self.current_token.token_type));
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Skip identifier
        self.next_token();
        
        // Expect semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ModuleExport { name })
    }
    
    /// Parse debug statement (debug expression;)
    fn parse_debug_statement(&mut self) -> Option<Statement> {
        // Skip 'debug' token
        self.next_token();
        
        let value = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };
        
        // Expect semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::DebugStatement { value })
    }
    
    /// Parse assert statement (assert(condition, message?);)
    fn parse_assert_statement(&mut self) -> Option<Statement> {
        // Skip 'assert' token
        self.next_token();
        
        // Expect left parenthesis
        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }
        
        // Skip left parenthesis and move to condition
        self.next_token();
        
        let condition = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };
        
        let mut message = None;
        
        // Check for comma (optional message)
        if self.peek_token_is(TokenType::Comma) {
            self.next_token(); // Move to comma
            self.next_token(); // Move past comma to message expression
            
            message = match self.parse_expression(Precedence::Lowest) {
                Some(expr) => Some(expr),
                None => return None,
            };
        }
        
        // Expect right parenthesis
        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }
        
        // Expect semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::AssertStatement { condition, message })
    }
    
    /// Parse trace statement (trace expression;)
    fn parse_trace_statement(&mut self) -> Option<Statement> {
        // Skip 'trace' token
        self.next_token();
        
        let value = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };
        
        // Expect semicolon
        if self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::TraceStatement { value })
    }
    
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // Skip comments
        if self.current_token.token_type == TokenType::Comment {
            return None;
        }
        
        // Try to get a prefix parse function for the current token
        let prefix = self.prefix_parse_fns.get(&self.current_token.token_type).cloned();
        
        if prefix.is_none() {
            self.errors.push(format!(
                "No prefix parse function for {:?} found at line {}, column {}",
                self.current_token.token_type,
                self.current_token.line,
                self.current_token.column
            ));
            return None;
        }
        
        let mut left_exp = prefix.unwrap()(self)?;
        
        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix = self.infix_parse_fns.get(&self.peek_token.token_type).cloned();
            
            if infix.is_none() {
                return Some(left_exp);
            }
            
            self.next_token();
            
            left_exp = infix.unwrap()(self, left_exp)?;
        }
        
        Some(left_exp)
    }
    
    fn parse_identifier(&mut self) -> Option<Expression> {
        // Convert library tokens to identifiers when used in expressions
        let identifier = match self.current_token.token_type {
            TokenType::Random | TokenType::HTLib | TokenType::Coin | TokenType::MathLib | 
            TokenType::Ping | TokenType::Bolt | TokenType::Seed | TokenType::NetLib | 
            TokenType::File | TokenType::Json | TokenType::Date | TokenType::StrLib | 
            TokenType::ArrLib | TokenType::Os | TokenType::Regex | TokenType::Crypto | 
            TokenType::Color | TokenType::System | TokenType::Ui | TokenType::Storage | 
            TokenType::Audio | TokenType::Image | TokenType::Validation | 
            TokenType::LogLib | TokenType::Uuid |
            // Self-compilation library tokens
            TokenType::MemoryLib | TokenType::BinaryLib | TokenType::BitwiseLib | 
            TokenType::SystemLib | TokenType::ProcessLib | TokenType::ThreadLib | 
            TokenType::CompilerLib => {
                self.current_token.literal.clone()
            },
            _ => self.current_token.literal.clone()
        };
        Some(Expression::Identifier(identifier))
    }
    
    fn parse_string_literal(&mut self) -> Option<Expression> {
        Some(Expression::StringLiteral(self.current_token.literal.clone()))
    }
    
    fn parse_number_literal(&mut self) -> Option<Expression> {
        match self.current_token.literal.parse::<f64>() {
            Ok(value) => Some(Expression::NumberLiteral(value)),
            Err(_) => {
                self.errors.push(format!(
                    "Could not parse {} as number at line {}, column {}",
                    self.current_token.literal,
                    self.current_token.line,
                    self.current_token.column
                ));
                None
            }
        }
    }
    
    fn parse_boolean_literal(&mut self) -> Option<Expression> {
        Some(Expression::BooleanLiteral(self.current_token_is(TokenType::True)))
    }
    
    fn parse_null_literal(&mut self) -> Option<Expression> {
        Some(Expression::NullLiteral)
    }
    
    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.current_token.literal.clone();
        
        self.next_token();
        
        let right = self.parse_expression(Precedence::Prefix)?;
        
        Some(Expression::PrefixExpression {
            operator,
            right: Box::new(right),
        })
    }
    
    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.current_token.literal.clone();
        let precedence = self.current_precedence();
        
        self.next_token();
        
        let right = self.parse_expression(precedence)?;
        
        Some(Expression::InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
    
    fn parse_assignment_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.current_token.literal.clone();
        
        self.next_token();
        
        let right = self.parse_expression(Precedence::Lowest)?;
        
        Some(Expression::AssignmentExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
    
    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }
        
        Some(expr)
    }
    
    fn parse_array_literal(&mut self) -> Option<Expression> {
        let elements = self.parse_expression_list(TokenType::RightBracket)?;
        
        Some(Expression::ArrayLiteral { elements })
    }
    
    fn parse_map_literal(&mut self) -> Option<Expression> {
        let mut pairs = Vec::new();
        
        while !self.peek_token_is(TokenType::RightBrace) {
            self.next_token();
            
            let key = self.parse_expression(Precedence::Lowest)?;
            
            if !self.expect_peek(TokenType::Colon) {
                return None;
            }
            
            self.next_token();
            
            let value = self.parse_expression(Precedence::Lowest)?;
            
            pairs.push((key, value));
            
            if !self.peek_token_is(TokenType::RightBrace) && !self.expect_peek(TokenType::Comma) {
                return None;
            }
        }
        
        if !self.expect_peek(TokenType::RightBrace) {
            return None;
        }
        
        Some(Expression::MapLiteral { pairs })
    }
    
    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_expression_list(TokenType::RightParen)?;
        
        Some(Expression::CallExpression {
            function: Box::new(function),
            arguments,
        })
    }
    
    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();
        
        // Handle function name inside brackets differently
        // For MemoryLib[addressof], the function name needs to be treated as an identifier
        let index = if self.current_token_is(TokenType::Identifier) {
            // If the current token is an identifier, create an identifier expression
            Expression::Identifier(self.current_token.literal.clone())
        } else {
            // Otherwise parse normally
            self.parse_expression(Precedence::Lowest)?
        };
        
        if !self.expect_peek(TokenType::RightBracket) {
            return None;
        }
        
        // Check if this is a library function call (e.g., MemoryLib[addressof](1, 2))
        if self.peek_token_is(TokenType::LeftParen) {
            // This is a library function call
            self.next_token(); // Consume the left paren
            
            // Parse arguments or empty arguments list
            let arguments = if self.peek_token_is(TokenType::RightParen) {
                self.next_token(); // Consume right paren for empty args
                vec![]
            } else {
                match self.parse_expression_list(TokenType::RightParen) {
                    Some(args) => args,
                    None => return None
                }
            };
            
            return Some(Expression::LibraryCall {
                library: Box::new(left),
                function: Box::new(index),
                arguments,
            });
        }
        
        Some(Expression::IndexExpression {
            left: Box::new(left),
            index: Box::new(index),
        })
    }
    
    fn parse_dot_expression(&mut self, left: Expression) -> Option<Expression> {
        self.next_token(); // Skip '.' token
        
        if !self.current_token_is(TokenType::Identifier) {
            // Handle library tokens as identifiers
            let is_library_token = match self.current_token.token_type {
                TokenType::Random | TokenType::HTLib | TokenType::Coin | TokenType::MathLib | 
                TokenType::Ping | TokenType::Bolt | TokenType::Seed | TokenType::NetLib | 
                TokenType::File | TokenType::Json | TokenType::Date | TokenType::StrLib | 
                TokenType::ArrLib | TokenType::Os | TokenType::Regex | TokenType::Crypto | 
                TokenType::Color | TokenType::System | TokenType::Ui | TokenType::Storage | 
                TokenType::Audio | TokenType::Image | TokenType::Validation | 
                TokenType::LogLib | TokenType::Uuid | TokenType::Get | TokenType::Post |
                TokenType::Read | TokenType::Debug | TokenType::Assert | TokenType::Trace |
                TokenType::Show | TokenType::Exit | TokenType::Api | TokenType::Call |
                TokenType::Connect | TokenType::To | TokenType::Import | TokenType::Export |
                TokenType::From | TokenType::As |
                // Self-compilation library tokens
                TokenType::MemoryLib | TokenType::BinaryLib | TokenType::BitwiseLib | 
                TokenType::SystemLib | TokenType::ProcessLib | TokenType::ThreadLib | 
                TokenType::CompilerLib => true,
                _ => false
            };
            
            if !is_library_token {
                self.errors.push(format!(
                    "Expected identifier after '.', got {:?} at line {}, column {}",
                    self.current_token.token_type, self.current_token.line, self.current_token.column
                ));
                return None;
            }
        }
        
        // Get the property name
        let property = self.current_token.literal.clone();
        
        // Create a property access expression
        Some(Expression::InfixExpression {
            left: Box::new(left),
            operator: ".".to_string(),
            right: Box::new(Expression::Identifier(property)),
        })
    }
    
    fn parse_namespace_expression(&mut self, left: Expression) -> Option<Expression> {
        // left should be the namespace identifier
        let namespace = match left {
            Expression::Identifier(name) => name,
            _ => return None,
        };
        
        self.next_token(); // Skip '::' token
        
        if !self.current_token_is(TokenType::Identifier) {
            return None;
        }
        
        let function = self.current_token.literal.clone();
        
        // Check if this is a function call (has parentheses)
        if self.peek_token_is(TokenType::LeftParen) {
            self.next_token(); // Move to '('
            
            // Parse arguments or empty arguments list
            let arguments = if self.peek_token_is(TokenType::RightParen) {
                self.next_token(); // Consume right paren for empty args
                vec![]
            } else {
                match self.parse_expression_list(TokenType::RightParen) {
                    Some(args) => args,
                    None => return None
                }
            };
            
            return Some(Expression::NamespaceCall {
                namespace,
                function,
                arguments,
            });
        }
        
        // If no parentheses, treat as namespace access (for future use)
        Some(Expression::InfixExpression {
            left: Box::new(Expression::Identifier(namespace)),
            operator: "::".to_string(),
            right: Box::new(Expression::Identifier(function)),
        })
    }
    
    fn parse_expression_list(&mut self, end: TokenType) -> Option<Vec<Expression>> {
        let mut list = Vec::new();
        
        if self.peek_token_is(end.clone()) {
            self.next_token();
            return Some(list);
        }
        
        self.next_token();
        
        list.push(self.parse_expression(Precedence::Lowest)?);
        
        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }
        
        if !self.expect_peek(end) {
            return None;
        }
        
        Some(list)
    }
    
    fn current_precedence(&self) -> Precedence {
        Self::token_precedence(&self.current_token.token_type)
    }
    
    fn peek_precedence(&self) -> Precedence {
        Self::token_precedence(&self.peek_token.token_type)
    }
    
    fn token_precedence(token_type: &TokenType) -> Precedence {
        match token_type {
            TokenType::Assign | TokenType::PlusAssign | TokenType::MinusAssign | 
            TokenType::AsteriskAssign | TokenType::SlashAssign | TokenType::PercentAssign => Precedence::Assignment,
            TokenType::Or => Precedence::LogicalOr,
            TokenType::And => Precedence::LogicalAnd,
            TokenType::Equal | TokenType::NotEqual => Precedence::Equals,
            TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk | TokenType::Percent | TokenType::FloorDiv => Precedence::Product,
            TokenType::Power => Precedence::Power,
            TokenType::LeftParen => Precedence::Call,
            TokenType::LeftBracket => Precedence::Index,
            TokenType::Dot => Precedence::Call, // Dot has same precedence as function call
            TokenType::ColonColon => Precedence::Namespace,
            _ => Precedence::Lowest,
        }
    }
    // Parse class declaration (class Name { ... })
    fn parse_class_declaration(&mut self) -> Option<Statement> {
        // Get the current token position for error reporting
        let token_line = self.current_token.line;
        let token_column = self.current_token.column;
        
        // Expect class name (identifier)
        if !self.expect_peek(TokenType::Identifier) {
            self.errors.push(format!(
                "Expected class name after 'class' keyword at line {}, column {}",
                token_line, token_column
            ));
            return None;
        }
        
        let class_name = self.current_token.literal.clone();
        
        // Expect opening brace
        if !self.expect_peek(TokenType::LeftBrace) {
            self.errors.push(format!(
                "Expected '{{' after class name at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Parse class body
        let body = self.parse_block_statement();
        
        Some(Statement::ClassDeclaration {
            name: class_name,
            body,
        })
    }
    
    // Parse API declaration (api name = from("url"))
    fn parse_api_declaration(&mut self) -> Option<Statement> {
        // Get the current token position for error reporting
        let token_line = self.current_token.line;
        let token_column = self.current_token.column;
        
        // Expect API name (identifier)
        if !self.expect_peek(TokenType::Identifier) {
            self.errors.push(format!(
                "Expected API name after 'api' keyword at line {}, column {}",
                token_line, token_column
            ));
            return None;
        }
        
        let api_name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            self.errors.push(format!(
                "Expected '=' after API name at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Expect 'from' keyword
        if !self.expect_peek(TokenType::From) {
            self.errors.push(format!(
                "Expected 'from' after '=' in API declaration at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Expect opening parenthesis
        if !self.expect_peek(TokenType::LeftParen) {
            self.errors.push(format!(
                "Expected '(' after 'from' in API declaration at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Expect URL string
        if !self.expect_peek(TokenType::StringLiteral) {
            self.errors.push(format!(
                "Expected string literal in API declaration, got {:?} instead at line {}, column {}",
                self.current_token.token_type,
                self.current_token.line,
                self.current_token.column
            ));
            return None;
        }
        
        let url = self.current_token.literal.clone();
        
        // Expect closing parenthesis
        if !self.expect_peek(TokenType::RightParen) {
            self.errors.push(format!(
                "Expected ')' after URL in API declaration at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Check for optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ApiDeclaration {
            name: api_name,
            url,
        })
    }
    
    // Parse API call (call api_name { ... })
    fn parse_api_call(&mut self) -> Option<Statement> {
        // Get the current token position for error reporting
        let token_line = self.current_token.line;
        let token_column = self.current_token.column;
        
        // Expect API name (identifier)
        if !self.expect_peek(TokenType::Identifier) {
            self.errors.push(format!(
                "Expected API name after 'call' keyword at line {}, column {}",
                token_line, token_column
            ));
            return None;
        }
        
        let api_name = self.current_token.literal.clone();
        
        // Expect opening brace
        if !self.expect_peek(TokenType::LeftBrace) {
            self.errors.push(format!(
                "Expected '{{' after API name in call statement at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Parse API call body
        let body = self.parse_block_statement();
        
        Some(Statement::ApiCall {
            name: api_name,
            body,
        })
    }
    
    // Parse connect statement (connect name = from("url") { options })
    fn parse_connect_statement(&mut self) -> Option<Statement> {
        // Get the current token position for error reporting
        let token_line = self.current_token.line;
        let token_column = self.current_token.column;
        
        // Expect connection name (identifier)
        if !self.expect_peek(TokenType::Identifier) {
            self.errors.push(format!(
                "Expected connection name after 'connect' keyword at line {}, column {}",
                token_line, token_column
            ));
            return None;
        }
        
        let connection_name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            self.errors.push(format!(
                "Expected '=' after connection name at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Expect 'from' keyword
        if !self.expect_peek(TokenType::From) {
            self.errors.push(format!(
                "Expected 'from' after '=' in connect statement, got {:?} instead at line {}, column {}",
                self.current_token.token_type,
                self.current_token.line,
                self.current_token.column
            ));
            return None;
        }
        
        // Expect opening parenthesis
        if !self.expect_peek(TokenType::LeftParen) {
            self.errors.push(format!(
                "Expected '(' after 'from' in connect statement at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Expect URL string
        if !self.expect_peek(TokenType::StringLiteral) {
            self.errors.push(format!(
                "Expected string literal in connect statement, got {:?} instead at line {}, column {}",
                self.current_token.token_type,
                self.current_token.line,
                self.current_token.column
            ));
            return None;
        }
        
        let url = self.current_token.literal.clone();
        
        // Expect closing parenthesis
        if !self.expect_peek(TokenType::RightParen) {
            self.errors.push(format!(
                "Expected ')' after URL in connect statement at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        let mut options = Vec::new();
        
        // Check for optional configuration block
        if self.peek_token_is(TokenType::LeftBrace) {
            self.next_token(); // Move to {
            
            // Parse configuration options
            if !self.expect_peek(TokenType::RightBrace) {
                // Parse options until we reach the closing brace
                loop {
                    // Expect option name (identifier)
                    if !self.current_token_is(TokenType::Identifier) {
                        self.errors.push(format!(
                            "Expected option name in connect configuration at line {}, column {}",
                            self.current_token.line, self.current_token.column
                        ));
                        return None;
                    }
                    
                    let option_name = self.current_token.literal.clone();
                    self.next_token();
                    
                    // Parse option value expression
                    let option_value = if let Some(expr) = self.parse_expression(Precedence::Lowest) {
                        expr
                    } else {
                        self.errors.push(format!(
                            "Expected expression for option '{}' at line {}, column {}",
                            option_name,
                            self.current_token.line,
                            self.current_token.column
                        ));
                        return None;
                    };
                    
                    options.push((option_name, option_value));
                    
                    // Skip semicolon if present
                    if self.current_token_is(TokenType::Semicolon) {
                        self.next_token();
                    }
                    
                    // Break if we've reached the end of the options block
                    if self.current_token_is(TokenType::RightBrace) {
                        break;
                    }
                    
                    // If we haven't reached the end, there should be more options
                    if self.current_token_is(TokenType::EOF) {
                        self.errors.push(format!(
                            "Unexpected end of file in connect configuration at line {}, column {}",
                            self.current_token.line, self.current_token.column
                        ));
                        return None;
                    }
                }
            }
        }
        
        // Check for optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ConnectStatement {
            name: connection_name,
            url,
            options,
        })
    }
    
    // Parse import statement (import {name} from(./path/to/file))
    fn parse_import_statement(&mut self) -> Option<Statement> {
        // Get the current token position for error reporting
        let token_line = self.current_token.line;
        let token_column = self.current_token.column;
        
        // Check for { to start import list
        if !self.expect_peek(TokenType::LeftBrace) {
            self.errors.push(format!(
                "Expected '{{' after 'import' keyword at line {}, column {}",
                token_line, token_column
            ));
            return None;
        }
        
        let mut imports = Vec::new();
        
        // Parse import names
        self.next_token(); // Move past {
        
        // Handle empty import list
        if self.current_token_is(TokenType::RightBrace) {
            self.errors.push(format!(
                "Empty import list at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Parse import names
        while !self.current_token_is(TokenType::RightBrace) && !self.current_token_is(TokenType::EOF) {
            if self.current_token_is(TokenType::Identifier) {
                imports.push(self.current_token.literal.clone());
            } else {
                self.errors.push(format!(
                    "Expected identifier in import list, got {:?} instead at line {}, column {}",
                    self.current_token.token_type,
                    self.current_token.line,
                    self.current_token.column
                ));
                return None;
            }
            
            // Move to next token
            self.next_token();
            
            // Check for comma or closing brace
            if self.current_token_is(TokenType::Comma) {
                self.next_token(); // Skip comma and continue
            } else if !self.current_token_is(TokenType::RightBrace) {
                self.errors.push(format!(
                    "Expected ',' or '}}' after import name, got {:?} instead at line {}, column {}",
                    self.current_token.token_type,
                    self.current_token.line,
                    self.current_token.column
                ));
                return None;
            }
        }
        
        // Check for from keyword
        if !self.expect_peek(TokenType::From) {
            self.errors.push(format!(
                "Expected 'from' after import list at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Check for opening parenthesis
        if !self.expect_peek(TokenType::LeftParen) {
            self.errors.push(format!(
                "Expected '(' after 'from' in import statement at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Parse path
        self.next_token(); // Move past (
        
        // Path can be either a string literal or an identifier (for relative paths)
        let path = if self.current_token_is(TokenType::StringLiteral) {
            // String literal path
            self.current_token.literal.clone()
        } else if self.current_token_is(TokenType::Identifier) || self.current_token_is(TokenType::Dot) {
            // Relative path starting with identifier or dot
            let mut path_str = self.current_token.literal.clone();
            
            // Continue parsing path components
            while self.peek_token_is(TokenType::Dot) || self.peek_token_is(TokenType::Slash) {
                self.next_token();
                path_str.push_str(&self.current_token.literal);
                
                // Check for path component after dot or slash
                if self.peek_token_is(TokenType::Identifier) {
                    self.next_token();
                    path_str.push_str(&self.current_token.literal);
                }
            }
            
            path_str
        } else {
            self.errors.push(format!(
                "Expected string literal or path in import statement, got {:?} instead at line {}, column {}",
                self.current_token.token_type,
                self.current_token.line,
                self.current_token.column
            ));
            return None;
        };
        
        // Check for closing parenthesis
        if !self.expect_peek(TokenType::RightParen) {
            self.errors.push(format!(
                "Expected ')' after path in import statement at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Check for optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ImportStatement {
            imports,
            path,
        })
    }
    
    // Parse library import statement (lib name)
    fn parse_lib_statement(&mut self) -> Option<Statement> {
        // Get the current token position for error reporting
        let token_line = self.current_token.line;
        let token_column = self.current_token.column;
        
        // Move to the next token
        self.next_token();
        
        // Check if the next token is a valid library name (either an identifier or a library token)
        let is_valid_library = match self.current_token.token_type {
            TokenType::Identifier | 
            TokenType::Random | TokenType::HTLib | TokenType::Coin | TokenType::MathLib | 
            TokenType::Ping | TokenType::Bolt | TokenType::Seed | TokenType::NetLib | 
            TokenType::File | TokenType::Json | TokenType::Date | TokenType::StrLib | 
            TokenType::ArrLib | TokenType::Os | TokenType::Regex | TokenType::Crypto | 
            TokenType::Color | TokenType::System | TokenType::Ui | TokenType::Storage | 
            TokenType::Audio | TokenType::Image | TokenType::Validation | 
            TokenType::LogLib | TokenType::Uuid | TokenType::BoxLib | TokenType::IOLib |
            TokenType::NumLib | TokenType::RefLib | TokenType::TimeLib | 
            TokenType::TypeCheckLib | TokenType::TypeConvertLib |
            // Self-compilation library tokens
            TokenType::MemoryLib | TokenType::BinaryLib | TokenType::BitwiseLib | 
            TokenType::SystemLib | TokenType::ProcessLib | TokenType::ThreadLib | 
            TokenType::CompilerLib |
            // 18 - Compiler Construction Libraries
            TokenType::LexerLib | TokenType::ParserLib | TokenType::ASTLib |
            TokenType::SymbolLib | TokenType::TypeLib | TokenType::IRLib |
            TokenType::CodeGenLib | TokenType::OptimizeLib => true,
            _ => false
        };
        
        if !is_valid_library {
            self.errors.push(format!(
                "Expected library name after 'lib' keyword at line {}, column {}",
                token_line, token_column
            ));
            return None;
        }
        
        // Get library name and convert to lowercase for case-insensitive comparison
        let lib_name = self.current_token.literal.clone();
        let lib_name_lower = lib_name.to_lowercase();
        
        // No need to validate library names anymore - the compiler will dynamically scan the properties/libs folder
        // and register all available libraries. This makes the system more extensible.
        
        // Check for optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::LibStatement {
            name: lib_name,
        })
    }
    
    fn parse_load_statement(&mut self) -> Option<Statement> {
        // Expect: load ( <number> ) { ... }
        
        // Expect opening parenthesis
        if !self.expect_peek(TokenType::LeftParen) {
            self.errors.push(format!(
                "Expected '(' after 'load' keyword at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Move past the '(' to the number
        self.next_token();
        
        // Parse the number of cycles expression
        let cycles = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => {
                self.errors.push(format!(
                    "Expected number after 'load(' at line {}, column {}",
                    self.current_token.line, self.current_token.column
                ));
                return None;
            }
        };
        
        // Expect closing parenthesis
        if !self.expect_peek(TokenType::RightParen) {
            self.errors.push(format!(
                "Expected ')' after load count at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Expect opening brace for block
        if !self.expect_peek(TokenType::LeftBrace) {
            self.errors.push(format!(
                "Expected '{{' after 'load()' at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Parse the block of statements inside the load
        let block = self.parse_block_statement();
        
        // Validate that all statements in the block are 'show' statements
        for stmt in &block {
            if !matches!(stmt, Statement::ShowStatement { .. }) {
                self.errors.push(format!(
                    "Only 'show' statements are allowed inside 'load' blocks at line {}, column {}",
                    self.current_token.line, self.current_token.column
                ));
                break;
            }
        }
        
        Some(Statement::LoadStatement { cycles, block })
    }
    
    fn parse_is_statement(&mut self) -> Option<Statement> {
        // 'is' is a comparison operator that works similar to '==' but can be used as a statement
        // Example: is x 5 (checks if x equals 5)
        
        self.next_token(); // Move past 'is' to the identifier

        if !self.current_token_is(TokenType::Identifier) {
            self.errors.push(format!("Expected identifier after 'is' at line {}, column {}", 
                self.current_token.line, self.current_token.column));
            return None;
        }
        
        let name = self.current_token.literal.clone();
        self.next_token(); // Move to the value
        
        let value = self.parse_expression(Precedence::Lowest)?;
        
        // Create an expression that compares name == value
        let left = Expression::Identifier(name);
        let operator = String::from("==");
        let right = value;
        
        let condition = Expression::InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
        
        // Parse the block if there's a brace
        let consequence = if self.peek_token_is(TokenType::LeftBrace) {
            self.next_token(); // Move to '{'
            self.parse_block_statement()
        } else {
            Vec::new()
        };
        
        // Check for else block
        let alternative = if self.peek_token_is(TokenType::Else) {
            self.next_token(); // Move to 'else'
            
            if !self.expect_peek(TokenType::LeftBrace) {
                return None;
            }
            
            Some(self.parse_block_statement())
        } else {
            None
        };
        
        // This is effectively an if statement with a different syntax
        Some(Statement::IfStatement {
            condition,
            consequence,
            alternative,
        })
    }
    
    fn parse_when_statement(&mut self) -> Option<Statement> {
        // 'when' is a pattern matching statement, similar to a switch/case
        // Example: when x { 1 => { ... }, 2 => { ... }, _ => { ... } }
        
        self.next_token(); // Move past 'when' to the expression
        
        let value = self.parse_expression(Precedence::Lowest)?;
        
        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }
        
        // TODO: Implement pattern matching statements
        // For now, we'll just parse a block and return it directly
        let block = self.parse_block_statement();
        
        // Return a simple block statement until pattern matching is implemented
        Some(Statement::BlockStatement {
            statements: block,
        })
    }
    
    fn parse_else_statement(&mut self) -> Option<Statement> {
        // 'else' should only appear after an 'if' statement
        // This is a syntax error if it appears standalone
        
        self.errors.push(format!("Unexpected 'else' statement without matching 'if' at line {}, column {}",
            self.current_token.line, self.current_token.column));
        None
    }
    
    // Parse const declaration (const NAME = value;)
    fn parse_const_declaration(&mut self) -> Option<Statement> {
        // Skip 'const' token
        self.next_token();
        
        // Expect identifier (constant name)
        if !self.current_token_is(TokenType::Identifier) {
            self.errors.push(format!(
                "Expected identifier after 'const' keyword at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        // Skip '=' token
        self.next_token();
        
        // Parse the value expression
        let value = self.parse_expression(Precedence::Lowest)?;
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::ConstDeclaration {
            name,
            value,
        })
    }
    
    // Parse enum declaration (enum NAME { VARIANT1, VARIANT2 = value, ... })
    fn parse_enum_declaration(&mut self) -> Option<Statement> {
        // Skip 'enum' token
        self.next_token();
        
        // Expect identifier (enum name)
        if !self.current_token_is(TokenType::Identifier) {
            self.errors.push(format!(
                "Expected identifier after 'enum' keyword at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect opening brace
        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }
        
        // Skip '{' token
        self.next_token();
        
        // Parse enum variants
        let mut variants = Vec::new();
        
        // Continue until we reach the closing brace
        while !self.current_token_is(TokenType::RightBrace) && !self.current_token_is(TokenType::EOF) {
            // Expect identifier (variant name)
            if !self.current_token_is(TokenType::Identifier) {
                self.errors.push(format!(
                    "Expected identifier for enum variant at line {}, column {}",
                    self.current_token.line, self.current_token.column
                ));
                return None;
            }
            
            let variant_name = self.current_token.literal.clone();
            let mut variant_value = None;
            
            // Check if this variant has an assigned value
            if self.peek_token_is(TokenType::Assign) {
                // Skip '=' token
                self.next_token();
                self.next_token();
                
                // Parse the value expression
                variant_value = Some(self.parse_expression(Precedence::Lowest)?);
            }
            
            // Add the variant to our list
            variants.push((variant_name, variant_value));
            
            // Skip comma if present
            if self.peek_token_is(TokenType::Comma) {
                self.next_token();
            }
            
            // Move to the next token
            self.next_token();
        }
        
        Some(Statement::EnumDeclaration {
            name,
            variants,
        })
    }
    
    // Parse inline function declaration (inline fun name(params) { body })
    fn parse_inline_function_declaration(&mut self) -> Option<Statement> {
        // Skip 'inline' token
        self.next_token();
        
        // Expect 'fun' keyword
        if !self.current_token_is(TokenType::Fun) {
            self.errors.push(format!(
                "Expected 'fun' keyword after 'inline' at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Skip 'fun' token
        self.next_token();
        
        // Expect identifier (function name)
        if !self.current_token_is(TokenType::Identifier) {
            self.errors.push(format!(
                "Expected function name after 'inline fun' at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect opening parenthesis
        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }
        
        // Parse function parameters
        let parameters = self.parse_function_parameters();
        
        // Expect opening brace
        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }
        
        // Parse function body
        let body = self.parse_block_statement();
        
        Some(Statement::InlineFunctionDeclaration {
            name,
            parameters,
            body,
        })
    }
    
    // Parse final class declaration (final class Name { ... })
    fn parse_final_class_declaration(&mut self) -> Option<Statement> {
        // Skip 'final' token
        self.next_token();
        
        // Expect 'class' keyword
        if !self.current_token_is(TokenType::Class) {
            self.errors.push(format!(
                "Expected 'class' keyword after 'final' at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        // Skip 'class' token
        self.next_token();
        
        // Expect identifier (class name)
        if !self.current_token_is(TokenType::Identifier) {
            self.errors.push(format!(
                "Expected class name after 'final class' at line {}, column {}",
                self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect opening brace
        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }
        
        // Parse class body
        let body = self.parse_block_statement();
        
        Some(Statement::FinalClassDeclaration {
            name,
            body,
        })
    }
    
    // Parse volatile variable declaration (volatile let/hold/etc name = value;)
    fn parse_volatile_declaration(&mut self) -> Option<Statement> {
        // Skip 'volatile' token
        self.next_token();
        
        // Expect a variable type keyword (let, hold, etc.)
        let var_type = match self.current_token.token_type {
            TokenType::Num | TokenType::Str | TokenType::Bool | TokenType::Var | TokenType::Const => {
                self.current_token.literal.clone()
            },
            _ => {
                self.errors.push(format!(
                    "Expected variable type keyword after 'volatile' at line {}, column {}",
                    self.current_token.line, self.current_token.column
                ));
                return None;
            }
        };
        
        // Skip variable type token
        self.next_token();
        
        // Expect identifier (variable name)
        if !self.current_token_is(TokenType::Identifier) {
            self.errors.push(format!(
                "Expected variable name after 'volatile {}' at line {}, column {}",
                var_type, self.current_token.line, self.current_token.column
            ));
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        // Expect assignment operator
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        // Skip '=' token
        self.next_token();
        
        // Parse the value expression
        let value = self.parse_expression(Precedence::Lowest)?;
        
        // Optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        
        Some(Statement::VolatileDeclaration {
            var_type,
            name,
            value: Some(value),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_variable_declaration() {
        let input = "let x = 5;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        
        let program = parser.parse_program();
        
        assert_eq!(parser.get_errors().len(), 0, "Parser errors: {:?}", parser.get_errors());
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Statement::VariableDeclaration { var_type, name, value } => {
                assert_eq!(var_type, "let");
                assert_eq!(name, "x");
                
                match value {
                    Some(Expression::NumberLiteral(val)) => assert_eq!(*val, 5.0),
                    _ => panic!("Expected NumberLiteral, got {:?}", value),
                }
            },
            _ => panic!("Expected VariableDeclaration, got {:?}", program.statements[0]),
        }
    }
    
    #[test]
    fn test_function_declaration() {
        let input = "fun add(x, y) { return x + y; }";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        
        let program = parser.parse_program();
        
        assert_eq!(parser.get_errors().len(), 0, "Parser errors: {:?}", parser.get_errors());
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Statement::FunctionDeclaration { name, parameters, body } => {
                assert_eq!(name, "add");
                assert_eq!(parameters, &vec!["x".to_string(), "y".to_string()]);
                assert_eq!(body.len(), 1);
                
                match &body[0] {
                    Statement::ReturnStatement { value } => {
                        match value {
                            Some(Expression::InfixExpression { left, operator, right }) => {
                                match **left {
                                    Expression::Identifier(ref id) => assert_eq!(id, "x"),
                                    _ => panic!("Expected Identifier, got {:?}", left),
                                }
                                
                                assert_eq!(operator, "+");
                                
                                match **right {
                                    Expression::Identifier(ref id) => assert_eq!(id, "y"),
                                    _ => panic!("Expected Identifier, got {:?}", right),
                                }
                            },
                            _ => panic!("Expected InfixExpression, got {:?}", value),
                        }
                    },
                    _ => panic!("Expected ReturnStatement, got {:?}", body[0]),
                }
            },
            _ => panic!("Expected FunctionDeclaration, got {:?}", program.statements[0]),
        }
    }
}