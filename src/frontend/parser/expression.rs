// src/frontend/parser/expression.rs

use crate::frontend::lexer::token::{Token, TokenKind};
use crate::frontend::parser::ast::*;
use crate::frontend::diagnostics::{Diagnostic, Position, Span, helpers};

/// Parser error type - now wraps diagnostic
#[derive(Debug, Clone)]
pub struct ParseError {
    pub diagnostic: Diagnostic,
}

impl ParseError {
    pub fn new(message: String, line: usize) -> Self {
        let span = Span::new(
            Position::new(line, 1, 0),
            Position::new(line, 1, 0),
        );
        ParseError {
            diagnostic: helpers::syntax_error(message, span),
        }
    }

    pub fn from_diagnostic(diagnostic: Diagnostic) -> Self {
        ParseError { diagnostic }
    }

    pub fn unexpected_token<S: Into<String>>(expected: Vec<S>, found: S, span: Span) -> Self {
        ParseError {
            diagnostic: helpers::unexpected_token(expected, found, span),
        }
    }

    pub fn missing_token<S: Into<String>>(expected: S, span: Span) -> Self {
        ParseError {
            diagnostic: helpers::missing_token(expected, span),
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

/// Expression parser for Razen language
pub struct ExpressionParser<'a> {
    tokens: &'a [Token],
    pub current: usize,
}

impl<'a> ExpressionParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        ExpressionParser { tokens, current: 0 }
    }

    /// Parse an expression with operator precedence
    pub fn parse_expression(&mut self) -> ParseResult<Expression> {
        self.parse_assignment()
    }

    /// Parse assignment expressions (lowest precedence)
    fn parse_assignment(&mut self) -> ParseResult<Expression> {
        let expr = self.parse_logical_or()?;

        if self.match_tokens(&[
            TokenKind::Equal,
            TokenKind::PlusEqual,
            TokenKind::MinusEqual,
            TokenKind::StarEqual,
            TokenKind::SlashEqual,
            TokenKind::PercentEqual,
        ]) {
            let operator = self.previous().kind.clone();
            let right = self.parse_assignment()?;

            let assignment_op = match operator {
                TokenKind::Equal => AssignmentOperator::Assign,
                TokenKind::PlusEqual => AssignmentOperator::AddAssign,
                TokenKind::MinusEqual => AssignmentOperator::SubtractAssign,
                TokenKind::StarEqual => AssignmentOperator::MultiplyAssign,
                TokenKind::SlashEqual => AssignmentOperator::DivideAssign,
                TokenKind::PercentEqual => AssignmentOperator::ModuloAssign,
                _ => unreachable!(),
            };

            return Ok(Expression::AssignmentExpression(AssignmentExpression {
                left: Box::new(expr),
                operator: assignment_op,
                right: Box::new(right),
            }));
        }

        Ok(expr)
    }

    /// Parse logical OR expressions
    fn parse_logical_or(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_logical_and()?;

        while self.match_tokens(&[TokenKind::PipePipe]) {
            let right = self.parse_logical_and()?;
            expr = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse logical AND expressions
    fn parse_logical_and(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_bitwise_or()?;

        while self.match_tokens(&[TokenKind::AmpersandAmpersand]) {
            let right = self.parse_bitwise_or()?;
            expr = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse bitwise OR expressions
    fn parse_bitwise_or(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_bitwise_xor()?;

        while self.match_tokens(&[TokenKind::Pipe]) {
            let right = self.parse_bitwise_xor()?;
            expr = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(expr),
                operator: BinaryOperator::BitwiseOr,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse bitwise XOR expressions
    fn parse_bitwise_xor(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_bitwise_and()?;

        while self.match_tokens(&[TokenKind::Caret]) {
            let right = self.parse_bitwise_and()?;
            expr = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(expr),
                operator: BinaryOperator::BitwiseXor,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse bitwise AND expressions
    fn parse_bitwise_and(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_equality()?;

        while self.match_tokens(&[TokenKind::Ampersand]) {
            let right = self.parse_equality()?;
            expr = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(expr),
                operator: BinaryOperator::BitwiseAnd,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse equality expressions
    fn parse_equality(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_comparison()?;

        while self.match_tokens(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
            let operator = self.previous().kind.clone();
            let right = self.parse_comparison()?;

            let binary_op = match operator {
                TokenKind::EqualEqual => BinaryOperator::Equal,
                TokenKind::BangEqual => BinaryOperator::NotEqual,
                _ => unreachable!(),
            };

            expr = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(expr),
                operator: binary_op,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse comparison expressions
    fn parse_comparison(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_shift()?;

        while self.match_tokens(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous().kind.clone();
            let right = self.parse_shift()?;

            let binary_op = match operator {
                TokenKind::Greater => BinaryOperator::Greater,
                TokenKind::GreaterEqual => BinaryOperator::GreaterEqual,
                TokenKind::Less => BinaryOperator::Less,
                TokenKind::LessEqual => BinaryOperator::LessEqual,
                _ => unreachable!(),
            };

            expr = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(expr),
                operator: binary_op,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse shift expressions
    fn parse_shift(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_range()?;

        while self.match_tokens(&[TokenKind::LessLess, TokenKind::GreaterGreater]) {
            let operator = self.previous().kind.clone();
            let right = self.parse_range()?;

            let binary_op = match operator {
                TokenKind::LessLess => BinaryOperator::LeftShift,
                TokenKind::GreaterGreater => BinaryOperator::RightShift,
                _ => unreachable!(),
            };

            expr = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(expr),
                operator: binary_op,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse range expressions
    fn parse_range(&mut self) -> ParseResult<Expression> {
        let expr = self.parse_term()?;

        if self.match_tokens(&[TokenKind::DotDot]) {
            let end = self.parse_term()?;
            return Ok(Expression::RangeExpression(RangeExpression {
                start: Box::new(expr),
                end: Box::new(end),
                inclusive: false,
            }));
        }

        Ok(expr)
    }

    /// Parse term expressions (addition and subtraction)
    fn parse_term(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_factor()?;

        while self.match_tokens(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous().kind.clone();
            let right = self.parse_factor()?;

            let binary_op = match operator {
                TokenKind::Minus => BinaryOperator::Subtract,
                TokenKind::Plus => BinaryOperator::Add,
                _ => unreachable!(),
            };

            expr = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(expr),
                operator: binary_op,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse factor expressions (multiplication, division, modulo)
    fn parse_factor(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_power()?;

        while self.match_tokens(&[TokenKind::Slash, TokenKind::Star, TokenKind::Percent]) {
            let operator = self.previous().kind.clone();
            let right = self.parse_power()?;

            let binary_op = match operator {
                TokenKind::Slash => BinaryOperator::Divide,
                TokenKind::Star => BinaryOperator::Multiply,
                TokenKind::Percent => BinaryOperator::Modulo,
                _ => unreachable!(),
            };

            expr = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(expr),
                operator: binary_op,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse power expressions (exponentiation)
    fn parse_power(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_unary()?;

        while self.match_tokens(&[TokenKind::StarStar]) {
            let right = self.parse_unary()?;
            expr = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(expr),
                operator: BinaryOperator::Power,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse unary expressions
    fn parse_unary(&mut self) -> ParseResult<Expression> {
        if self.match_tokens(&[
            TokenKind::Bang,
            TokenKind::Minus,
            TokenKind::Plus,
            TokenKind::Tilde,
            TokenKind::PlusPlus,
            TokenKind::MinusMinus,
        ]) {
            let operator = self.previous().kind.clone();
            let right = self.parse_unary()?;

            let unary_op = match operator {
                TokenKind::Bang => UnaryOperator::Not,
                TokenKind::Minus => UnaryOperator::Minus,
                TokenKind::Plus => UnaryOperator::Plus,
                TokenKind::Tilde => UnaryOperator::BitwiseNot,
                TokenKind::PlusPlus => UnaryOperator::PreIncrement,
                TokenKind::MinusMinus => UnaryOperator::PreDecrement,
                _ => unreachable!(),
            };

            return Ok(Expression::UnaryExpression(UnaryExpression {
                operator: unary_op,
                operand: Box::new(right),
            }));
        }

        self.parse_postfix()
    }

    /// Parse postfix expressions (function calls, member access, array indexing)
    fn parse_postfix(&mut self) -> ParseResult<Expression> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_tokens(&[TokenKind::LeftParen]) {
                // Function call
                let mut arguments = Vec::new();
                
                if !self.check(&TokenKind::RightParen) {
                    loop {
                        arguments.push(self.parse_expression()?);
                        if !self.match_tokens(&[TokenKind::Comma]) {
                            break;
                        }
                    }
                }

                self.consume(TokenKind::RightParen, "Expected ')' after arguments")?;
                
                expr = Expression::CallExpression(CallExpression {
                    callee: Box::new(expr),
                    arguments,
                });
            } else if self.match_tokens(&[TokenKind::Dot]) {
                // Member access
                let name = self.consume_identifier("Expected property name after '.'")?;
                expr = Expression::MemberExpression(MemberExpression {
                    object: Box::new(expr),
                    property: Identifier::new(name),
                    computed: false,
                });
            } else if self.match_tokens(&[TokenKind::LeftBracket]) {
                // Array indexing
                let index = self.parse_expression()?;
                self.consume(TokenKind::RightBracket, "Expected ']' after array index")?;
                
                expr = Expression::IndexExpression(IndexExpression {
                    object: Box::new(expr),
                    index: Box::new(index),
                });
            } else if self.match_tokens(&[TokenKind::PlusPlus, TokenKind::MinusMinus]) {
                // Postfix increment/decrement
                let operator = self.previous().kind.clone();
                let unary_op = match operator {
                    TokenKind::PlusPlus => UnaryOperator::PostIncrement,
                    TokenKind::MinusMinus => UnaryOperator::PostDecrement,
                    _ => unreachable!(),
                };

                expr = Expression::UnaryExpression(UnaryExpression {
                    operator: unary_op,
                    operand: Box::new(expr),
                });
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Parse primary expressions (literals, identifiers, grouping)
    fn parse_primary(&mut self) -> ParseResult<Expression> {
        if self.match_tokens(&[TokenKind::True]) {
            return Ok(Expression::BooleanLiteral(BooleanLiteral::new(true)));
        }

        if self.match_tokens(&[TokenKind::False]) {
            return Ok(Expression::BooleanLiteral(BooleanLiteral::new(false)));
        }

        if self.match_tokens(&[TokenKind::Null]) {
            return Ok(Expression::NullLiteral(NullLiteral));
        }

        if let Some(token) = self.match_token_kind(&TokenKind::Integer(0)) {
            if let TokenKind::Integer(value) = &token.kind {
                return Ok(Expression::IntegerLiteral(IntegerLiteral::new(*value)));
            }
        }

        if let Some(token) = self.match_token_kind(&TokenKind::Float(0.0)) {
            if let TokenKind::Float(value) = &token.kind {
                return Ok(Expression::FloatLiteral(FloatLiteral::new(*value)));
            }
        }

        if let Some(token) = self.match_token_kind(&TokenKind::String("".to_string())) {
            if let TokenKind::String(value) = &token.kind {
                return Ok(Expression::StringLiteral(StringLiteral::new(value.clone())));
            }
        }
        
        // Handle f-strings
        if let Some(token) = self.match_token_kind(&TokenKind::FString("".to_string())) {
            if let TokenKind::FString(value) = &token.kind {
                return Ok(self.parse_f_string(value)?);
            }
        }

        if self.match_tokens(&[TokenKind::Identifier]) {
            let name = self.previous().lexeme.clone();
            
            // f-strings are now handled by the FString token type above
            
            return Ok(Expression::Identifier(Identifier::new(name)));
        }

        // Handle built-in function identifiers
        if self.match_tokens(&[
            TokenKind::Print,
            TokenKind::Println,
            TokenKind::Input,
            TokenKind::Read,
            TokenKind::Write,
            TokenKind::Open,
            TokenKind::Close,
        ]) {
            let name = self.previous().lexeme.clone();
            return Ok(Expression::Identifier(Identifier::new(name)));
        }

        if self.match_tokens(&[TokenKind::LeftParen]) {
            let expr = self.parse_expression()?;
            self.consume(TokenKind::RightParen, "Expected ')' after expression")?;
            return Ok(Expression::GroupingExpression(GroupingExpression {
                expression: Box::new(expr),
            }));
        }

        if self.match_tokens(&[TokenKind::LeftBracket]) {
            // Array literal
            let mut elements = Vec::new();
            
            if !self.check(&TokenKind::RightBracket) {
                loop {
                    elements.push(self.parse_expression()?);
                    if !self.match_tokens(&[TokenKind::Comma]) {
                        break;
                    }
                }
            }

            self.consume(TokenKind::RightBracket, "Expected ']' after array elements")?;
            return Ok(Expression::ArrayLiteral(ArrayLiteral { elements }));
        }

        Err(ParseError::new(
            "Expected expression".to_string(),
            self.peek().line,
        ))
    }

    // Helper methods
    fn match_tokens(&mut self, types: &[TokenKind]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_token_kind(&mut self, kind: &TokenKind) -> Option<Token> {
        if std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind) {
            Some(self.advance().clone())
        } else {
            None
        }
    }

    fn check(&self, token_type: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(token_type)
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenKind, message: &str) -> ParseResult<&Token> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError::new(
                message.to_string(),
                self.peek().line,
            ))
        }
    }

    fn consume_identifier(&mut self, _message: &str) -> ParseResult<String> {
        if self.check(&TokenKind::Identifier) {
            Ok(self.advance().lexeme.clone())
        } else {
            Err(ParseError::new("Expected identifier".to_string(), self.peek().line))
        }
    }

    /// Convert a token to a span
    fn token_to_span(&self, token: &Token) -> Span {
        let start = Position::new(token.line, 1, 0); // We don't have column info yet
        let end = Position::new(token.line, token.lexeme.len() + 1, token.lexeme.len());
        Span::new(start, end)
    }
    
    /// Parse f-string with interpolation support
    fn parse_f_string(&mut self, content: &str) -> ParseResult<Expression> {
        let mut parts = Vec::new();
        let mut current_text = String::new();
        let mut chars = content.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '{' {
                // Check for escaped brace {{
                if chars.peek() == Some(&'{') {
                    chars.next(); // consume second {
                    current_text.push('{');
                    continue;
                }
                
                // Save any accumulated text
                if !current_text.is_empty() {
                    parts.push(InterpolationPart::Text(current_text.clone()));
                    current_text.clear();
                }
                
                // Parse expression inside {}
                let mut expr_content = String::new();
                let mut brace_count = 1;
                
                while let Some(expr_ch) = chars.next() {
                    if expr_ch == '{' {
                        brace_count += 1;
                    } else if expr_ch == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            break;
                        }
                    }
                    expr_content.push(expr_ch);
                }
                
                if brace_count > 0 {
                    return Err(ParseError::new("Unclosed brace in f-string".to_string(), self.peek().line));
                }
                
                // Parse the expression content
                if !expr_content.trim().is_empty() {
                    let expr = self.parse_f_string_expression(&expr_content)?;
                    parts.push(InterpolationPart::Expression(expr));
                }
            } else if ch == '}' {
                // Check for escaped brace }}
                if chars.peek() == Some(&'}') {
                    chars.next(); // consume second }
                    current_text.push('}');
                } else {
                    return Err(ParseError::new("Unmatched '}' in f-string".to_string(), self.peek().line));
                }
            } else {
                current_text.push(ch);
            }
        }
        
        // Add any remaining text
        if !current_text.is_empty() {
            parts.push(InterpolationPart::Text(current_text));
        }
        
        Ok(Expression::InterpolatedString(InterpolatedString { parts }))
    }
    
    /// Parse an expression inside f-string braces
    fn parse_f_string_expression(&mut self, expr_content: &str) -> ParseResult<Expression> {
        // For now, we'll handle simple variable names and basic expressions
        let trimmed = expr_content.trim();
        
        // Handle simple identifiers
        if trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') && !trimmed.is_empty() {
            return Ok(Expression::Identifier(Identifier::new(trimmed.to_string())));
        }
        
        // Handle string concatenation like "Hello, " + name
        if let Some(plus_pos) = trimmed.find(" + ") {
            let left_part = trimmed[..plus_pos].trim();
            let right_part = trimmed[plus_pos + 3..].trim();
            
            let left_expr = if left_part.starts_with('"') && left_part.ends_with('"') {
                let string_content = left_part[1..left_part.len()-1].to_string();
                Expression::StringLiteral(StringLiteral::new(string_content))
            } else {
                Expression::Identifier(Identifier::new(left_part.to_string()))
            };
            
            let right_expr = if right_part.starts_with('"') && right_part.ends_with('"') {
                let string_content = right_part[1..right_part.len()-1].to_string();
                Expression::StringLiteral(StringLiteral::new(string_content))
            } else {
                Expression::Identifier(Identifier::new(right_part.to_string()))
            };
            
            return Ok(Expression::BinaryExpression(BinaryExpression {
                left: Box::new(left_expr),
                operator: BinaryOperator::Add,
                right: Box::new(right_expr),
            }));
        }
        
        // Fallback: treat as identifier
        Ok(Expression::Identifier(Identifier::new(trimmed.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::Lexer;

    #[test]
    fn test_parse_integer_literal() {
        let lexer = Lexer::new();
        let tokens = lexer.lex("42");
        let mut parser = ExpressionParser::new(&tokens);
        
        let result = parser.parse_expression().unwrap();
        match result {
            Expression::IntegerLiteral(lit) => assert_eq!(lit.value, 42),
            _ => panic!("Expected integer literal"),
        }
    }

    #[test]
    fn test_parse_binary_expression() {
        let lexer = Lexer::new();
        let tokens = lexer.lex("1 + 2");
        let mut parser = ExpressionParser::new(&tokens);
        
        let result = parser.parse_expression().unwrap();
        match result {
            Expression::BinaryExpression(expr) => {
                assert_eq!(expr.operator, BinaryOperator::Add);
            },
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_parse_function_call() {
        let lexer = Lexer::new();
        let tokens = lexer.lex("foo(1, 2)");
        let mut parser = ExpressionParser::new(&tokens);
        
        let result = parser.parse_expression().unwrap();
        match result {
            Expression::CallExpression(call) => {
                assert_eq!(call.arguments.len(), 2);
            },
            _ => panic!("Expected call expression"),
        }
    }
}