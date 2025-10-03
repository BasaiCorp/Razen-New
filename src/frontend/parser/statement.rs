// src/frontend/parser/statement.rs

use crate::frontend::lexer::token::{Token, TokenKind};
use crate::frontend::parser::ast::*;
use crate::frontend::parser::expression::{ExpressionParser, ParseError, ParseResult};

/// Statement parser for Razen language
pub struct StatementParser<'a> {
    tokens: &'a [Token],
    current: usize,
    debug: bool,
}

impl<'a> StatementParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        StatementParser { tokens, current: 0, debug: false }
    }
    
    /// Set debug mode for detailed parsing output
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    
    /// Get current position for updating parent parser
    pub fn current_position(&self) -> usize {
        self.current
    }

    /// Parse a statement
    pub fn parse_statement(&mut self) -> ParseResult<Statement> {
        // Check for public modifier
        let is_public = if self.match_tokens(&[TokenKind::Pub]) {
            true
        } else {
            false
        };

        // Parse different statement types
        if self.check(&TokenKind::Mod) {
            self.parse_module_declaration()
        } else if self.check(&TokenKind::Use) {
            self.parse_use_statement()
        } else if self.check(&TokenKind::Type) {
            self.parse_type_alias_declaration(is_public)
        } else if self.check(&TokenKind::Const) {
            self.parse_constant_declaration(is_public)
        } else if self.check(&TokenKind::Var) {
            self.parse_variable_declaration(is_public)
        } else if self.check(&TokenKind::Fun) {
            self.parse_function_declaration(is_public)
        } else if self.check(&TokenKind::Struct) {
            self.parse_struct_declaration(is_public)
        } else if self.check(&TokenKind::Enum) {
            self.parse_enum_declaration(is_public)
        } else if self.check(&TokenKind::Impl) {
            self.parse_impl_block()
        } else if self.check(&TokenKind::If) {
            self.parse_if_statement()
        } else if self.check(&TokenKind::While) {
            self.parse_while_statement()
        } else if self.check(&TokenKind::For) {
            self.parse_for_statement()
        } else if self.check(&TokenKind::Match) {
            self.parse_match_statement()
        } else if self.check(&TokenKind::Try) {
            self.parse_try_statement()
        } else if self.check(&TokenKind::Return) {
            self.parse_return_statement()
        } else if self.check(&TokenKind::Break) {
            self.parse_break_statement()
        } else if self.check(&TokenKind::Continue) {
            self.parse_continue_statement()
        } else if self.check(&TokenKind::Throw) {
            self.parse_throw_statement()
        } else if self.check(&TokenKind::LeftBrace) {
            self.parse_block_statement()
        } else {
            // Expression statement
            self.parse_expression_statement()
        }
    }

    /// Parse module declaration: mod name
    fn parse_module_declaration(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::Mod, "Expected 'mod'")?;
        let name = self.consume_identifier("Expected module name")?;

        Ok(Statement::ModuleDeclaration(ModuleDeclaration {
            name: Identifier::new(name),
        }))
    }

    /// Parse use statement: use "./path/to/module" as alias
    fn parse_use_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::Use, "Expected 'use'")?;

        // Parse the module path (string literal)
        let path = if let Some(token) = self.match_token_kind(&TokenKind::String("".to_string())) {
            if let TokenKind::String(value) = &token.kind {
                value.clone()
            } else {
                return Err(ParseError::new(
                    "Expected string literal for module path".to_string(),
                    self.peek().line,
                ));
            }
        } else {
            return Err(ParseError::new(
                "Expected string literal for module path".to_string(),
                self.peek().line,
            ));
        };

        // Check for 'as' clause (optional alias)
        let alias = if self.match_tokens(&[TokenKind::As]) {
            let alias_name = self.consume_identifier("Expected alias name")?;
            Some(Identifier::new(alias_name))
        } else {
            None
        };

        Ok(Statement::UseStatement(UseStatement {
            path,
            alias,
        }))
    }

    /// Parse type alias declaration: type Name = TargetType
    fn parse_type_alias_declaration(&mut self, is_public: bool) -> ParseResult<Statement> {
        self.consume(TokenKind::Type, "Expected 'type'")?;
        let name = self.consume_identifier("Expected type alias name")?;

        self.consume(TokenKind::Equal, "Expected '=' after type alias name")?;
        let target_type = self.parse_type_annotation()?;

        Ok(Statement::TypeAliasDeclaration(TypeAliasDeclaration {
            name: Identifier::new(name),
            target_type,
            is_public,
        }))
    }

    /// Parse constant declaration: const name: type = value
    fn parse_constant_declaration(&mut self, is_public: bool) -> ParseResult<Statement> {
        self.consume(TokenKind::Const, "Expected 'const'")?;
        let name = self.consume_identifier("Expected constant name")?;

        let mut type_annotation = None;
        if self.match_tokens(&[TokenKind::Colon]) {
            type_annotation = Some(self.parse_type_annotation()?);
        }

        self.consume(TokenKind::Equal, "Expected '=' after constant declaration")?;
        let initializer = self.parse_expression()?;

        Ok(Statement::ConstantDeclaration(ConstantDeclaration {
            name: Identifier::new(name),
            type_annotation,
            initializer,
            is_public,
        }))
    }

    /// Parse variable declaration: var name: type = value
    fn parse_variable_declaration(&mut self, is_public: bool) -> ParseResult<Statement> {
        self.consume(TokenKind::Var, "Expected 'var'")?;
        let name = self.consume_identifier("Expected variable name")?;

        let mut type_annotation = None;
        if self.match_tokens(&[TokenKind::Colon]) {
            type_annotation = Some(self.parse_type_annotation()?);
        }

        let mut initializer = None;
        if self.match_tokens(&[TokenKind::Equal]) {
            initializer = Some(self.parse_expression()?);
        }

        Ok(Statement::VariableDeclaration(VariableDeclaration {
            name: Identifier::new(name),
            type_annotation,
            initializer,
            is_public,
        }))
    }

    /// Parse function declaration: fun name(params) -> return_type { body }
    fn parse_function_declaration(&mut self, is_public: bool) -> ParseResult<Statement> {
        self.consume(TokenKind::Fun, "Expected 'fun'")?;
        let name = self.consume_identifier("Expected function name")?;

        self.consume(TokenKind::LeftParen, "Expected '(' after function name")?;

        let mut parameters = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                let param_name = self.consume_identifier("Expected parameter name")?;

                // Make type annotation optional (like old implementation)
                let param_type = if self.match_tokens(&[TokenKind::Colon]) {
                    Some(self.parse_type_annotation()?)
                } else {
                    // No type annotation means flexible parameter
                    None
                };

                parameters.push(Parameter {
                    name: Identifier::new(param_name),
                    type_annotation: param_type,
                });

                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenKind::RightParen, "Expected ')' after parameters")?;

        let mut return_type = None;
        if self.match_tokens(&[TokenKind::Arrow]) {
            return_type = Some(self.parse_type_annotation()?);
        }

        let body = if let Statement::BlockStatement(block) = self.parse_block_statement()? {
            block
        } else {
            return Err(ParseError::new(
                "Expected block statement for function body".to_string(),
                self.peek().line,
            ));
        };

        Ok(Statement::FunctionDeclaration(FunctionDeclaration {
            name: Identifier::new(name),
            parameters,
            return_type,
            body,
            is_public,
        }))
    }

    /// Parse struct declaration: struct Name { fields }
    fn parse_struct_declaration(&mut self, is_public: bool) -> ParseResult<Statement> {
        self.consume(TokenKind::Struct, "Expected 'struct'")?;
        let name = self.consume_identifier("Expected struct name")?;

        self.consume(TokenKind::LeftBrace, "Expected '{' after struct name")?;

        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let field_name = self.consume_identifier("Expected field name")?;
            self.consume(TokenKind::Colon, "Expected ':' after field name")?;
            let field_type = self.parse_type_annotation()?;

            fields.push(StructField {
                name: Identifier::new(field_name),
                type_annotation: field_type,
            });

            // Optional comma
            self.match_tokens(&[TokenKind::Comma]);
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after struct fields")?;

        Ok(Statement::StructDeclaration(StructDeclaration {
            name: Identifier::new(name),
            fields,
            is_public,
        }))
    }

    /// Parse enum declaration: enum Name { variants }
    fn parse_enum_declaration(&mut self, is_public: bool) -> ParseResult<Statement> {
        self.consume(TokenKind::Enum, "Expected 'enum'")?;
        let name = self.consume_identifier("Expected enum name")?;

        self.consume(TokenKind::LeftBrace, "Expected '{' after enum name")?;

        let mut variants = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let variant_name = self.consume_identifier("Expected variant name")?;

            let mut fields = None;

            // Check for tuple-style variant: Variant(type1, type2, ...)
            if self.match_tokens(&[TokenKind::LeftParen]) {
                let mut variant_fields = Vec::new();

                while !self.check(&TokenKind::RightParen) && !self.is_at_end() {
                    let field_type = self.parse_type_annotation()?;
                    variant_fields.push(field_type);

                    if !self.check(&TokenKind::RightParen) {
                        self.consume(TokenKind::Comma, "Expected ',' between variant fields")?;
                    }
                }

                self.consume(TokenKind::RightParen, "Expected ')' after variant fields")?;
                fields = Some(variant_fields);
            }

            variants.push(EnumVariant {
                name: Identifier::new(variant_name),
                fields,
            });

            // Optional comma
            self.match_tokens(&[TokenKind::Comma]);
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after enum variants")?;

        Ok(Statement::EnumDeclaration(EnumDeclaration {
            name: Identifier::new(name),
            variants,
            is_public,
        }))
    }

    /// Parse impl block: impl TypeName { methods }
    fn parse_impl_block(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::Impl, "Expected 'impl'")?;
        let target_type = self.consume_identifier("Expected type name after 'impl'")?;

        self.consume(TokenKind::LeftBrace, "Expected '{' after impl type")?;

        let mut methods = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let method = self.parse_method_declaration()?;
            methods.push(method);
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after impl methods")?;

        Ok(Statement::ImplBlock(ImplBlock::new(
            Identifier::new(target_type),
            methods,
        )))
    }

    /// Parse method declaration within impl block
    fn parse_method_declaration(&mut self) -> ParseResult<MethodDeclaration> {
        self.consume(TokenKind::Fun, "Expected 'fun' for method declaration")?;
        let method_name = self.consume_identifier("Expected method name")?;

        self.consume(TokenKind::LeftParen, "Expected '(' after method name")?;

        let mut parameters = Vec::new();
        let mut is_static = true; // Assume static until we find 'self'

        // Check for 'self' parameter first
        if self.check(&TokenKind::Self_) {
            self.advance(); // consume 'self'
            is_static = false;

            // Add self parameter
            parameters.push(Parameter {
                name: Identifier::new("self".to_string()),
                type_annotation: Some(TypeAnnotation::Custom(Identifier::new("Self".to_string()))),
            });

            // Check for comma if there are more parameters
            if self.check(&TokenKind::Comma) {
                self.advance();
            }
        }

        // Parse remaining parameters
        while !self.check(&TokenKind::RightParen) && !self.is_at_end() {
            let param_name = self.consume_identifier("Expected parameter name")?;
            self.consume(TokenKind::Colon, "Expected ':' after parameter name")?;
            let param_type = self.parse_type_annotation()?;

            parameters.push(Parameter {
                name: Identifier::new(param_name),
                type_annotation: Some(param_type),
            });

            if !self.check(&TokenKind::RightParen) {
                self.consume(TokenKind::Comma, "Expected ',' between parameters")?;
            }
        }

        self.consume(TokenKind::RightParen, "Expected ')' after parameters")?;

        // Parse optional return type
        let mut return_type = None;
        if self.match_tokens(&[TokenKind::Arrow]) {
            return_type = Some(self.parse_type_annotation()?);
        }

        // Parse method body
        let body = self.parse_block_statement()?;

        Ok(MethodDeclaration::new(
            Identifier::new(method_name),
            parameters,
            return_type,
            BlockStatement::new(vec![body]),
            is_static,
        ))
    }

    /// Parse if statement: if condition { then } elif condition { then } else { else }
    fn parse_if_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::If, "Expected 'if'")?;
        let condition = self.parse_expression()?;
        let then_branch = Box::new(self.parse_statement()?);

        let mut elif_branches = Vec::new();
        while self.match_tokens(&[TokenKind::Elif]) {
            let elif_condition = self.parse_expression()?;
            let elif_body = Box::new(self.parse_statement()?);
            elif_branches.push(ElifBranch {
                condition: elif_condition,
                body: elif_body,
            });
        }

        let mut else_branch = None;
        if self.match_tokens(&[TokenKind::Else]) {
            else_branch = Some(Box::new(self.parse_statement()?));
        }

        Ok(Statement::IfStatement(IfStatement {
            condition,
            then_branch,
            elif_branches,
            else_branch,
        }))
    }

    /// Parse while statement: while condition { body }
    fn parse_while_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::While, "Expected 'while'")?;
        let condition = self.parse_expression()?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::WhileStatement(WhileStatement {
            condition,
            body,
        }))
    }

    /// Parse for statement: for variable in iterable { body }
    fn parse_for_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::For, "Expected 'for'")?;
        let variable = self.consume_identifier("Expected loop variable")?;
        self.consume(TokenKind::In, "Expected 'in' after loop variable")?;
        let iterable = self.parse_expression()?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::ForStatement(ForStatement {
            variable: Identifier::new(variable),
            iterable,
            body,
        }))
    }

    /// Parse match statement: match expression { pattern => body, ... }
    fn parse_match_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::Match, "Expected 'match'")?;
        let expression = self.parse_expression()?;

        self.consume(TokenKind::LeftBrace, "Expected '{' after match expression")?;

        let mut arms = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            self.consume(TokenKind::FatArrow, "Expected '=>' after match pattern")?;
            let body = self.parse_expression()?;

            arms.push(MatchArm { pattern, body });

            // Optional comma
            self.match_tokens(&[TokenKind::Comma]);
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after match arms")?;

        Ok(Statement::MatchStatement(MatchStatement {
            expression,
            arms,
        }))
    }

    /// Parse try statement: try { body } catch e { handler }
    fn parse_try_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::Try, "Expected 'try'")?;

        let body = if let Statement::BlockStatement(block) = self.parse_block_statement()? {
            block
        } else {
            return Err(ParseError::new(
                "Expected block statement for try body".to_string(),
                self.peek().line,
            ));
        };

        let mut catch_clause = None;
        if self.match_tokens(&[TokenKind::Catch]) {
            let mut parameter = None;
            if self.check(&TokenKind::Identifier) {
                parameter = Some(Identifier::new(
                    self.consume_identifier("Expected catch parameter")?,
                ));
            }

            let catch_body =
                if let Statement::BlockStatement(block) = self.parse_block_statement()? {
                    block
                } else {
                    return Err(ParseError::new(
                        "Expected block statement for catch body".to_string(),
                        self.peek().line,
                    ));
                };

            catch_clause = Some(CatchClause {
                parameter,
                body: catch_body,
            });
        }

        Ok(Statement::TryStatement(TryStatement { body, catch_clause }))
    }

    /// Parse return statement: return expression?
    fn parse_return_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::Return, "Expected 'return'")?;

        let mut value = None;
        if !self.check(&TokenKind::Semicolon) && !self.is_at_end() {
            value = Some(self.parse_expression()?);
        }

        Ok(Statement::ReturnStatement(ReturnStatement { value }))
    }

    /// Parse break statement: break
    fn parse_break_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::Break, "Expected 'break'")?;
        Ok(Statement::BreakStatement(BreakStatement))
    }

    /// Parse continue statement: continue
    fn parse_continue_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::Continue, "Expected 'continue'")?;
        Ok(Statement::ContinueStatement(ContinueStatement))
    }

    /// Parse throw statement: throw expression
    fn parse_throw_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::Throw, "Expected 'throw'")?;
        let value = self.parse_expression()?;
        Ok(Statement::ThrowStatement(ThrowStatement { value }))
    }

    /// Parse block statement: { statements }
    fn parse_block_statement(&mut self) -> ParseResult<Statement> {
        self.consume(TokenKind::LeftBrace, "Expected '{'")?;

        let mut statements = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
            // Optional semicolon
            self.match_tokens(&[TokenKind::Semicolon]);
        }

        self.consume(TokenKind::RightBrace, "Expected '}'")?;

        Ok(Statement::BlockStatement(BlockStatement::new(statements)))
    }

    /// Parse expression statement
    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let expression = self.parse_expression()?;
        Ok(Statement::ExpressionStatement(ExpressionStatement {
            expression,
        }))
    }

    /// Parse a pattern for match statements
    fn parse_pattern(&mut self) -> ParseResult<Pattern> {
        if self.match_tokens(&[TokenKind::Identifier]) {
            let name = self.previous().lexeme.clone();
            if name == "_" {
                Ok(Pattern::Wildcard)
            } else {
                Ok(Pattern::Identifier(Identifier::new(name)))
            }
        } else {
            // Try to parse as literal
            let expr = self.parse_expression()?;
            Ok(Pattern::Literal(expr))
        }
    }

    /// Parse type annotation
    fn parse_type_annotation(&mut self) -> ParseResult<TypeAnnotation> {
        // Handle array syntax: [type]
        if self.match_tokens(&[TokenKind::LeftBracket]) {
            let element_type = self.parse_type_annotation()?;
            self.consume(TokenKind::RightBracket, "Expected ']' after array element type")?;
            return Ok(TypeAnnotation::Array(Box::new(element_type)));
        }
        
        // Handle map syntax: {key_type: value_type}
        if self.match_tokens(&[TokenKind::LeftBrace]) {
            let key_type = self.parse_type_annotation()?;
            self.consume(TokenKind::Colon, "Expected ':' after map key type")?;
            let value_type = self.parse_type_annotation()?;
            self.consume(TokenKind::RightBrace, "Expected '}' after map value type")?;
            return Ok(TypeAnnotation::Map(Box::new(key_type), Box::new(value_type)));
        }
        
        // Handle basic types
        if self.match_tokens(&[TokenKind::Int]) {
            Ok(TypeAnnotation::Int)
        } else if self.match_tokens(&[TokenKind::FloatType]) {
            Ok(TypeAnnotation::Float)
        } else if self.match_tokens(&[TokenKind::Str]) {
            Ok(TypeAnnotation::String)
        } else if self.match_tokens(&[TokenKind::Bool]) {
            Ok(TypeAnnotation::Bool)
        } else if self.match_tokens(&[TokenKind::Char]) {
            Ok(TypeAnnotation::Char)
        } else if self.match_tokens(&[TokenKind::Any]) {
            Ok(TypeAnnotation::Any)
        } else if self.match_tokens(&[TokenKind::Array]) {
            // Legacy syntax: Array (without element type specification)
            Ok(TypeAnnotation::Array(Box::new(TypeAnnotation::Any)))
        } else if self.match_tokens(&[TokenKind::Map]) {
            // Legacy syntax: Map (without key/value type specification)
            Ok(TypeAnnotation::Map(
                Box::new(TypeAnnotation::Any),
                Box::new(TypeAnnotation::Any),
            ))
        } else if self.match_tokens(&[TokenKind::Identifier]) {
            let name = self.previous().lexeme.clone();
            Ok(TypeAnnotation::Custom(Identifier::new(name)))
        } else {
            Err(ParseError::new(
                "Expected type annotation".to_string(),
                self.peek().line,
            ))
        }
    }

    /// Parse expression using expression parser
    fn parse_expression(&mut self) -> ParseResult<Expression> {
        // Debug: Show tokens being parsed (only in dev mode)
        if self.debug {
            println!("[DEBUG] Parsing expression starting at token {}", self.current);
            if self.current < self.tokens.len() {
                println!("[DEBUG] Current token: {:?}", self.tokens[self.current].kind);
                if self.current + 1 < self.tokens.len() {
                    println!("[DEBUG] Next token: {:?}", self.tokens[self.current + 1].kind);
                }
            }
        }
        
        let mut expr_parser = ExpressionParser::new(&self.tokens[self.current..]);
        expr_parser.set_debug(self.debug);
        let result = expr_parser.parse_expression()?;

        // Update current position based on expression parser's progress
        self.current += expr_parser.current;

        Ok(result)
    }

    // Helper methods (similar to expression parser)
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
            Err(ParseError::new(message.to_string(), self.peek().line))
        }
    }

    fn consume_identifier(&mut self, message: &str) -> ParseResult<String> {
        if self.check(&TokenKind::Identifier) {
            Ok(self.advance().lexeme.clone())
        } else {
            Err(ParseError::new(message.to_string(), self.peek().line))
        }
    }
}

// Additional methods for expression parser integration
impl<'a> StatementParser<'a> {
    pub fn set_current_position(&mut self, position: usize) {
        self.current = position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::Lexer;

    #[test]
    fn test_parse_variable_declaration() {
        let lexer = Lexer::new();
        let tokens = lexer.lex("var x: int = 42");
        let mut parser = StatementParser::new(&tokens);

        let result = parser.parse_statement().unwrap();
        match result {
            Statement::VariableDeclaration(var_decl) => {
                assert_eq!(var_decl.name.name, "x");
                assert!(var_decl.initializer.is_some());
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_function_declaration() {
        let lexer = Lexer::new();
        let tokens = lexer.lex("fun add(a: int, b: int) -> int { return a + b }");
        let mut parser = StatementParser::new(&tokens);

        let result = parser.parse_statement().unwrap();
        match result {
            Statement::FunctionDeclaration(func_decl) => {
                assert_eq!(func_decl.name.name, "add");
                assert_eq!(func_decl.parameters.len(), 2);
                assert!(func_decl.return_type.is_some());
            }
            _ => panic!("Expected function declaration"),
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let lexer = Lexer::new();
        let tokens = lexer.lex("if x > 0 { println(x) }");
        let mut parser = StatementParser::new(&tokens);

        let result = parser.parse_statement().unwrap();
        match result {
            Statement::IfStatement(_) => {}
            _ => panic!("Expected if statement"),
        }
    }
}
