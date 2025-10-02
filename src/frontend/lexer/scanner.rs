// src/frontend/lexer/scanner.rs

use std::collections::HashMap;

use super::token::{Token, TokenKind};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenKind>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        // Variable & Function Declaration
        keywords.insert("const".to_string(), TokenKind::Const);
        keywords.insert("var".to_string(), TokenKind::Var);
        keywords.insert("fun".to_string(), TokenKind::Fun);
        keywords.insert("type".to_string(), TokenKind::Type);

        // Data Structures
        keywords.insert("struct".to_string(), TokenKind::Struct);
        keywords.insert("enum".to_string(), TokenKind::Enum);
        keywords.insert("impl".to_string(), TokenKind::Impl);

        // Control Flow
        keywords.insert("if".to_string(), TokenKind::If);
        keywords.insert("else".to_string(), TokenKind::Else);
        keywords.insert("elif".to_string(), TokenKind::Elif);
        keywords.insert("while".to_string(), TokenKind::While);
        keywords.insert("for".to_string(), TokenKind::For);
        keywords.insert("in".to_string(), TokenKind::In);
        keywords.insert("return".to_string(), TokenKind::Return);
        keywords.insert("break".to_string(), TokenKind::Break);
        keywords.insert("continue".to_string(), TokenKind::Continue);
        keywords.insert("match".to_string(), TokenKind::Match);
        keywords.insert("try".to_string(), TokenKind::Try);
        keywords.insert("catch".to_string(), TokenKind::Catch);
        keywords.insert("throw".to_string(), TokenKind::Throw);

        // Module System
        keywords.insert("mod".to_string(), TokenKind::Mod);
        keywords.insert("use".to_string(), TokenKind::Use);
        keywords.insert("pub".to_string(), TokenKind::Pub);
        keywords.insert("from".to_string(), TokenKind::From);
        keywords.insert("as".to_string(), TokenKind::As);

        // Types
        keywords.insert("int".to_string(), TokenKind::Int);
        keywords.insert("str".to_string(), TokenKind::Str);
        keywords.insert("bool".to_string(), TokenKind::Bool);
        keywords.insert("char".to_string(), TokenKind::Char);
        keywords.insert("array".to_string(), TokenKind::Array);
        keywords.insert("map".to_string(), TokenKind::Map);
        keywords.insert("any".to_string(), TokenKind::Any);
        keywords.insert("float".to_string(), TokenKind::FloatType);

        // Literals
        keywords.insert("true".to_string(), TokenKind::True);
        keywords.insert("false".to_string(), TokenKind::False);
        keywords.insert("null".to_string(), TokenKind::Null);
        keywords.insert("self".to_string(), TokenKind::Self_);
        // Note: 'new' is not a keyword in Razen, it's just a regular method name

        // Note: I/O functions like print, println, input, etc. are treated as regular identifiers
        // They are registered as builtin functions in the semantic analyzer, not as keywords

        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenKind::Eof, "".to_string(), self.line));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            '[' => self.add_token(TokenKind::LeftBracket),
            ']' => self.add_token(TokenKind::RightBracket),
            ',' => self.add_token(TokenKind::Comma),
            ';' => self.add_token(TokenKind::Semicolon),
            '~' => self.add_token(TokenKind::Tilde),
            '^' => {
                let kind = if self.match_char('=') { TokenKind::CaretEqual } else { TokenKind::Caret };
                self.add_token(kind);
            }
            '#' => self.add_token(TokenKind::Hash),
            '@' => self.add_token(TokenKind::At),
            '?' => {
                let kind = if self.match_char('?') { TokenKind::QuestionQuestion } else { TokenKind::Question };
                self.add_token(kind);
            }
            '.' => {
                if self.match_char('.') {
                    let kind = if self.match_char('.') {
                        TokenKind::DotDotDot
                    } else if self.match_char('=') {
                        TokenKind::DotDotEqual
                    } else {
                        TokenKind::DotDot
                    };
                    self.add_token(kind);
                } else {
                    self.add_token(TokenKind::Dot);
                }
            }
            '-' => {
                let kind = if self.match_char('=') {
                    TokenKind::MinusEqual
                } else if self.match_char('-') {
                    TokenKind::MinusMinus
                } else if self.match_char('>') {
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                };
                self.add_token(kind);
            }
            '+' => {
                let kind = if self.match_char('=') {
                    TokenKind::PlusEqual
                } else if self.match_char('+') {
                    TokenKind::PlusPlus
                } else {
                    TokenKind::Plus
                };
                self.add_token(kind);
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                } else if self.match_char('*') {
                    // Block comment
                    while !(self.peek() == '*' && self.peek_next() == '/') && !self.is_at_end() {
                        if self.peek() == '\n' { self.line += 1; }
                        self.advance();
                    }
                    if !self.is_at_end() {
                        self.advance(); // consume '*'
                        self.advance(); // consume '/'
                    }
                } else {
                    let kind = if self.match_char('=') { TokenKind::SlashEqual } else { TokenKind::Slash };
                    self.add_token(kind);
                }
            }
            '*' => {
                 let kind = if self.match_char('=') {
                    TokenKind::StarEqual
                } else if self.match_char('*') {
                    TokenKind::StarStar
                } else {
                    TokenKind::Star
                };
                self.add_token(kind);
            }
            '%' => {
                let kind = if self.match_char('=') { TokenKind::PercentEqual } else { TokenKind::Percent };
                self.add_token(kind);
            }
            '!' => {
                let kind = if self.match_char('=') { TokenKind::BangEqual } else { TokenKind::Bang };
                self.add_token(kind);
            }
            '=' => {
                let kind = if self.match_char('=') {
                    TokenKind::EqualEqual
                } else if self.match_char('>') {
                    TokenKind::FatArrow
                } else {
                    TokenKind::Equal
                };
                self.add_token(kind);
            }
            '<' => {
                let kind = if self.match_char('=') {
                    TokenKind::LessEqual
                } else if self.match_char('<') {
                    if self.match_char('=') {
                        TokenKind::LessLessEqual
                    } else {
                        TokenKind::LessLess
                    }
                } else {
                    TokenKind::Less
                };
                self.add_token(kind);
            }
            '>' => {
                let kind = if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else if self.match_char('>') {
                    if self.match_char('=') {
                        TokenKind::GreaterGreaterEqual
                    } else {
                        TokenKind::GreaterGreater
                    }
                } else {
                    TokenKind::Greater
                };
                self.add_token(kind);
            }
            '&' => {
                let kind = if self.match_char('&') { 
                    TokenKind::AmpersandAmpersand 
                } else if self.match_char('=') {
                    TokenKind::AmpersandEqual
                } else { 
                    TokenKind::Ampersand 
                };
                self.add_token(kind);
            }
            '|' => {
                let kind = if self.match_char('|') { 
                    TokenKind::PipePipe 
                } else if self.match_char('=') {
                    TokenKind::PipeEqual
                } else { 
                    TokenKind::Pipe 
                };
                self.add_token(kind);
            }
            ':' => {
                let kind = if self.match_char(':') { TokenKind::ColonColon } else { TokenKind::Colon };
                self.add_token(kind);
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            c if c.is_digit(10) => self.number(),
            c if c.is_alphabetic() || c == '_' => {
                // Check for f-string
                if c == 'f' && self.peek() == '"' {
                    self.advance(); // consume 'f'
                    self.f_string();
                } else {
                    self.identifier();
                }
            }
            _ => self.add_token(TokenKind::Illegal),
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let kind = self.keywords.get(&text).cloned().unwrap_or(TokenKind::Identifier);
        self.add_token(kind);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        if text.contains('.') {
            self.add_token(TokenKind::Float(text.parse().unwrap()));
        } else {
            self.add_token(TokenKind::Integer(text.parse().unwrap()));
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            // Unterminated string.
            self.add_token(TokenKind::Illegal);
            return;
        }

        self.advance(); // The closing ".

        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token(TokenKind::String(value));
    }

    fn f_string(&mut self) {
        self.advance(); // consume opening "
        
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            // Unterminated f-string.
            self.add_token(TokenKind::Illegal);
            return;
        }

        self.advance(); // The closing ".

        // Extract the f-string content (without f" and ")
        let value: String = self.source[self.start + 2..self.current - 1].iter().collect();
        self.add_token(TokenKind::FString(value));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source[self.current] != expected { return false; }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0'; }
        self.source[self.current + 1]
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn add_token(&mut self, kind: TokenKind) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(kind, text, self.line));
    }
}
