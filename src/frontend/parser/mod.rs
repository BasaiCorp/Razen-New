// src/frontend/parser/mod.rs

pub mod ast;
pub mod expression;
pub mod statement;

use crate::frontend::lexer::token::Token;
use crate::frontend::parser::ast::*;
use crate::frontend::parser::expression::{ParseError, ParseResult};
use crate::frontend::parser::statement::StatementParser;
use crate::frontend::diagnostics::{Diagnostics, render_diagnostics};

/// The main parser for the Razen language.
/// It takes a stream of tokens and produces an Abstract Syntax Tree (AST).
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>,
    debug: bool,
}

impl Parser {
    /// Creates a new parser with the given tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
            debug: false,
        }
    }
    
    /// Set debug mode for detailed parsing output
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    /// Parses the tokens into a Program AST node.
    /// Returns the Program and any parse errors encountered.
    pub fn parse(&mut self) -> (Option<Program>, Diagnostics) {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(error) => {
                    self.errors.push(error);
                    // Try to recover by synchronizing to the next statement
                    self.synchronize();
                }
            }
        }

        let program = if statements.is_empty() && !self.errors.is_empty() {
            None
        } else {
            Some(Program::new(statements))
        };

        // Convert parse errors to diagnostics
        let mut diagnostics = Diagnostics::new();
        for error in &self.errors {
            diagnostics.add(error.diagnostic.clone());
        }

        (program, diagnostics)
    }

    /// Parse a single statement using the statement parser
    fn parse_statement(&mut self) -> ParseResult<Statement> {
        let mut stmt_parser = StatementParser::new(&self.tokens[self.current..]);
        stmt_parser.set_debug(self.debug);
        let result = stmt_parser.parse_statement()?;
        
        // Update current position based on statement parser's progress
        self.current += stmt_parser.current_position();
        
        Ok(result)
    }

    /// Synchronize after a parse error by advancing to the next likely statement boundary
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == crate::frontend::lexer::token::TokenKind::Semicolon {
                return;
            }

            match self.peek().kind {
                crate::frontend::lexer::token::TokenKind::Mod
                | crate::frontend::lexer::token::TokenKind::Use
                | crate::frontend::lexer::token::TokenKind::Const
                | crate::frontend::lexer::token::TokenKind::Var
                | crate::frontend::lexer::token::TokenKind::Fun
                | crate::frontend::lexer::token::TokenKind::Struct
                | crate::frontend::lexer::token::TokenKind::Enum
                | crate::frontend::lexer::token::TokenKind::If
                | crate::frontend::lexer::token::TokenKind::While
                | crate::frontend::lexer::token::TokenKind::For
                | crate::frontend::lexer::token::TokenKind::Match
                | crate::frontend::lexer::token::TokenKind::Try
                | crate::frontend::lexer::token::TokenKind::Return
                | crate::frontend::lexer::token::TokenKind::Break
                | crate::frontend::lexer::token::TokenKind::Continue
                | crate::frontend::lexer::token::TokenKind::Throw => return,
                _ => {}
            }

            self.advance();
        }
    }

    /// Check if we're at the end of the token stream
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || 
        self.peek().kind == crate::frontend::lexer::token::TokenKind::Eof
    }

    /// Get the current token without consuming it
    fn peek(&self) -> &Token {
        if self.current >= self.tokens.len() {
            // Return the last token which should be EOF
            if !self.tokens.is_empty() {
                &self.tokens[self.tokens.len() - 1]
            } else {
                // This should not happen in normal operation since lexer always adds EOF
                panic!("No tokens available")
            }
        } else {
            &self.tokens[self.current]
        }
    }

    /// Get the previous token
    fn previous(&self) -> &Token {
        if self.current == 0 {
            self.peek()
        } else {
            &self.tokens[self.current - 1]
        }
    }

    /// Advance to the next token and return the previous one
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Get all parse errors
    pub fn get_errors(&self) -> &[ParseError] {
        &self.errors
    }

    /// Check if parsing was successful (no errors)
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get diagnostics from current errors
    pub fn get_diagnostics(&self) -> Diagnostics {
        let mut diagnostics = Diagnostics::new();
        for error in &self.errors {
            diagnostics.add(error.diagnostic.clone());
        }
        diagnostics
    }
}

/// Convenience function to parse source code directly
pub fn parse_source(source: &str) -> (Option<Program>, Diagnostics) {
    parse_source_with_name(source, "input")
}

/// Parse source code with a specific file name for better error reporting
pub fn parse_source_with_name(source: &str, filename: &str) -> (Option<Program>, Diagnostics) {
    parse_source_with_debug(source, filename, false)
}

/// Parse source code with debug output enabled (for dev command)
pub fn parse_source_with_debug(source: &str, filename: &str, debug: bool) -> (Option<Program>, Diagnostics) {
    use crate::frontend::lexer::Lexer;
    
    let lexer = Lexer::new();
    let tokens = lexer.lex(source);
    
    if debug {
        println!("[DEBUG] Generated {} tokens", tokens.len());
        // Show first few tokens for debugging
        for (i, token) in tokens.iter().take(20).enumerate() {
            println!("[DEBUG] Token {}: {:?} at line {}", i, token.kind, token.line);
        }
    }
    
    let mut parser = Parser::new(tokens);
    parser.set_debug(debug);
    let (program, mut diagnostics) = parser.parse();
    
    // Add source information to diagnostics
    for diagnostic in &mut diagnostics.diagnostics {
        for label in &mut diagnostic.labels {
            if label.span.source_id.is_none() {
                label.span.source_id = Some(filename.to_string());
            }
        }
    }
    
    (program, diagnostics)
}

/// Pretty print parse errors using the new diagnostic system
pub fn format_parse_errors(diagnostics: &Diagnostics, source: &str, filename: &str) -> String {
    render_diagnostics(diagnostics, &[(filename.to_string(), source.to_string())])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::Lexer;

    #[test]
    fn test_parse_simple_program() {
        let source = r#"
            var x: int = 42
            fun main() {
                println(x)
            }
        "#;
        
        let (program, errors) = parse_source(source);
        
        assert!(errors.is_empty(), "Expected no parse errors, got: {:?}", errors);
        assert!(program.is_some(), "Expected a program AST");
        
        let program = program.unwrap();
        assert_eq!(program.statements.len(), 2);
        
        // Check variable declaration
        match &program.statements[0] {
            Statement::VariableDeclaration(var_decl) => {
                assert_eq!(var_decl.name.name, "x");
            },
            _ => panic!("Expected variable declaration"),
        }
        
        // Check function declaration
        match &program.statements[1] {
            Statement::FunctionDeclaration(func_decl) => {
                assert_eq!(func_decl.name.name, "main");
            },
            _ => panic!("Expected function declaration"),
        }
    }

    #[test]
    fn test_parse_with_errors() {
        let source = "var x: = 42"; // Missing type
        
        let (program, errors) = parse_source(source);
        
        assert!(!errors.is_empty(), "Expected parse errors");
        // Program might still be partially parsed
    }

    #[test]
    fn test_parse_expressions() {
        let source = r#"
            var result: int = 1 + 2 * 3
            var name: str = "Hello, " + "World!"
            var flag: bool = true && false
        "#;
        
        let (program, errors) = parse_source(source);
        
        assert!(errors.is_empty(), "Expected no parse errors, got: {:?}", errors);
        assert!(program.is_some());
        
        let program = program.unwrap();
        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn test_parse_control_flow() {
        let source = r#"
            fun test() {
                if x > 0 {
                    println("positive")
                } elif x == 0 {
                    println("zero")
                } else {
                    println("negative")
                }
                
                while x > 0 {
                    x -= 1
                }
                
                for i in 0..10 {
                    println(i)
                }
            }
        "#;
        
        let (program, errors) = parse_source(source);
        
        assert!(errors.is_empty(), "Expected no parse errors, got: {:?}", errors);
        assert!(program.is_some());
    }

    #[test]
    fn test_parse_data_structures() {
        let source = r#"
            struct Person {
                name: str,
                age: int
            }
            
            enum Color {
                Red,
                Green,
                Blue
            }
        "#;
        
        let (program, errors) = parse_source(source);
        
        assert!(errors.is_empty(), "Expected no parse errors, got: {:?}", errors);
        assert!(program.is_some());
        
        let program = program.unwrap();
        assert_eq!(program.statements.len(), 2);
    }

    #[test]
    fn test_parse_module_system() {
        let source = r#"
            mod main
            
            use math
            use {sin, cos} from "trigonometry"
            use logger as log
        "#;
        
        let (program, errors) = parse_source(source);
        
        assert!(errors.is_empty(), "Expected no parse errors, got: {:?}", errors);
        assert!(program.is_some());
        
        let program = program.unwrap();
        assert_eq!(program.statements.len(), 4);
    }

    #[test]
    fn test_error_recovery() {
        let source = r#"
            var x: int = 42
            var y: = // Missing type and value
            var z: str = "hello"
        "#;
        
        let (program, errors) = parse_source(source);
        
        assert!(!errors.is_empty(), "Expected parse errors");
        
        // Should still parse the valid statements
        if let Some(program) = program {
            // Should have parsed at least the first and possibly the last statement
            assert!(program.statements.len() >= 1);
        }
    }
}