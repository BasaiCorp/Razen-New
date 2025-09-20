// src/frontend/lexer/mod.rs

pub mod scanner;
pub mod token;

use scanner::Scanner;
use token::Token;

/// The Lexer for the Razen language.
/// It takes source code as input and produces a stream of tokens.
pub struct Lexer;

impl Lexer {
    /// Creates a new `Lexer`.
    pub fn new() -> Self {
        Lexer
    }

    /// Tokenizes the given source code.
    ///
    /// # Arguments
    ///
    /// * `source` - A string slice that holds the source code to tokenize.
    ///
    /// # Returns
    ///
    /// A `Vec<Token>` containing the tokens generated from the source code.
    pub fn lex(&self, source: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(source.to_string());
        scanner.scan_tokens().clone()
    }
}
