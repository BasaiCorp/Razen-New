// src/frontend/diagnostics/error.rs

use std::fmt;
use std::ops::Range;

/// Represents a position in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Position { line, column, offset }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Represents a span of source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
    pub source_id: Option<String>, // File path or identifier
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Span {
            start,
            end,
            source_id: None,
        }
    }

    pub fn with_source(mut self, source_id: String) -> Self {
        self.source_id = Some(source_id);
        self
    }

    pub fn single_char(pos: Position) -> Self {
        Span::new(pos, Position::new(pos.line, pos.column + 1, pos.offset + 1))
    }

    pub fn to_range(&self) -> Range<usize> {
        self.start.offset..self.end.offset
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref source_id) = self.source_id {
            write!(f, "{}:{}", source_id, self.start)
        } else {
            write!(f, "{}", self.start)
        }
    }
}

/// Severity levels for diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Help,
    Note,
    Warning,
    Error,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Help => "help",
            Severity::Note => "note",
            Severity::Warning => "warning",
            Severity::Error => "error",
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            Severity::Help => "\x1b[36m",      // Cyan
            Severity::Note => "\x1b[34m",      // Blue
            Severity::Warning => "\x1b[33m",   // Yellow
            Severity::Error => "\x1b[31m",     // Red
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Categories of diagnostic errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticKind {
    // Lexical errors
    UnterminatedString,
    InvalidNumber,
    UnknownCharacter,
    InvalidEscapeSequence { sequence: String },
    
    // Syntax errors
    UnexpectedToken { expected: Vec<String>, found: String },
    MissingToken { expected: String },
    InvalidExpression,
    InvalidStatement,
    MissingSemicolon,
    UnexpectedEof,
    InvalidFunctionSignature,
    InvalidVariableDeclaration,
    
    // Semantic errors
    UndefinedVariable { name: String },
    UndefinedFunction { name: String },
    TypeMismatch { expected: String, found: String },
    DuplicateDefinition { name: String },
    InvalidAssignment { reason: String },
    UnreachableCode,
    DeadCode { name: String },
    
    // Function-related errors
    WrongArgumentCount { expected: usize, found: usize },
    ArgumentCountMismatch { expected: usize, found: usize },
    InvalidReturnType { expected: String, found: String },
    MissingReturn { function_name: String },
    InvalidFunctionCall { reason: String },
    
    // Method-related errors
    UndefinedMethod { method: String, type_name: String },
    
    // Variable-related errors
    UninitializedVariable { name: String },
    ImmutableAssignment { name: String },
    UnusedVariable { name: String },
    ShadowedVariable { name: String, previous_line: usize },
    
    // Control flow errors
    BreakOutsideLoop,
    ContinueOutsideLoop,
    InvalidCondition { found: String },
    
    // Module system errors
    ModuleNotFound { name: String },
    CircularImport { path: Vec<String> },
    InvalidImport { reason: String },
    
    // Performance warnings
    InefficientCode { suggestion: String },
    LargeFunction { lines: usize },
    DeepNesting { depth: usize },
    
    // Style warnings
    NamingConvention { name: String, expected_style: String },
    MissingDocumentation { item: String },
    LongLine { length: usize, max: usize },
    
    // Custom error with message
    Custom { message: String },
}

impl DiagnosticKind {
    pub fn title(&self) -> String {
        match self {
            // Lexical errors
            DiagnosticKind::UnterminatedString => "unterminated string literal".to_string(),
            DiagnosticKind::InvalidNumber => "invalid number literal".to_string(),
            DiagnosticKind::UnknownCharacter => "unknown character".to_string(),
            DiagnosticKind::InvalidEscapeSequence { sequence } => {
                format!("invalid escape sequence `{}`", sequence)
            },
            
            // Syntax errors
            DiagnosticKind::UnexpectedToken { expected, found } => {
                if expected.len() == 1 {
                    format!("expected `{}`, found `{}`", expected[0], found)
                } else {
                    format!("expected one of {}, found `{}`", 
                           expected.iter().map(|s| format!("`{}`", s)).collect::<Vec<_>>().join(", "), 
                           found)
                }
            },
            DiagnosticKind::MissingToken { expected } => {
                format!("expected `{}`", expected)
            },
            DiagnosticKind::InvalidExpression => "invalid expression".to_string(),
            DiagnosticKind::InvalidStatement => "invalid statement".to_string(),
            DiagnosticKind::MissingSemicolon => "missing semicolon".to_string(),
            DiagnosticKind::UnexpectedEof => "unexpected end of file".to_string(),
            DiagnosticKind::InvalidFunctionSignature => "invalid function signature".to_string(),
            DiagnosticKind::InvalidVariableDeclaration => "invalid variable declaration".to_string(),
            
            // Semantic errors
            DiagnosticKind::UndefinedVariable { name } => {
                format!("cannot find value `{}` in this scope", name)
            },
            DiagnosticKind::UndefinedFunction { name } => {
                format!("cannot find function `{}` in this scope", name)
            },
            DiagnosticKind::TypeMismatch { expected, found } => {
                format!("mismatched types: expected `{}`, found `{}`", expected, found)
            },
            DiagnosticKind::DuplicateDefinition { name } => {
                format!("the name `{}` is defined multiple times", name)
            },
            DiagnosticKind::InvalidAssignment { reason } => {
                format!("invalid assignment: {}", reason)
            },
            DiagnosticKind::UnreachableCode => "unreachable code".to_string(),
            DiagnosticKind::DeadCode { name } => {
                format!("unused {}", name)
            },
            
            // Function-related errors
            DiagnosticKind::WrongArgumentCount { expected, found } => {
                format!("this function takes {} argument{} but {} {} supplied",
                       expected, if *expected == 1 { "" } else { "s" },
                       found, if *found == 1 { "was" } else { "were" })
            },
            DiagnosticKind::ArgumentCountMismatch { expected, found } => {
                format!("this method takes {} argument{} but {} {} supplied",
                       expected, if *expected == 1 { "" } else { "s" },
                       found, if *found == 1 { "was" } else { "were" })
            },
            DiagnosticKind::InvalidReturnType { expected, found } => {
                format!("mismatched return type: expected `{}`, found `{}`", expected, found)
            },
            DiagnosticKind::MissingReturn { function_name } => {
                format!("function `{}` is missing a return statement", function_name)
            },
            DiagnosticKind::InvalidFunctionCall { reason } => {
                format!("invalid function call: {}", reason)
            },
            
            // Method-related errors
            DiagnosticKind::UndefinedMethod { method, type_name } => {
                format!("no method named `{}` found for type `{}`", method, type_name)
            },
            
            // Variable-related errors
            DiagnosticKind::UninitializedVariable { name } => {
                format!("use of possibly-uninitialized variable `{}`", name)
            },
            DiagnosticKind::ImmutableAssignment { name } => {
                format!("cannot assign to immutable variable `{}`", name)
            },
            DiagnosticKind::UnusedVariable { name } => {
                format!("unused variable: `{}`", name)
            },
            DiagnosticKind::ShadowedVariable { name, previous_line } => {
                format!("variable `{}` shadows a previous declaration on line {}", name, previous_line)
            },
            
            // Control flow errors
            DiagnosticKind::BreakOutsideLoop => "`break` outside of loop".to_string(),
            DiagnosticKind::ContinueOutsideLoop => "`continue` outside of loop".to_string(),
            DiagnosticKind::InvalidCondition { found } => {
                format!("expected boolean condition, found `{}`", found)
            },
            
            // Module system errors
            DiagnosticKind::ModuleNotFound { name } => {
                format!("module `{}` not found", name)
            },
            DiagnosticKind::CircularImport { path } => {
                format!("circular import detected: {}", path.join(" -> "))
            },
            DiagnosticKind::InvalidImport { reason } => {
                format!("invalid import: {}", reason)
            },
            
            // Performance warnings
            DiagnosticKind::InefficientCode { suggestion } => {
                format!("inefficient code: {}", suggestion)
            },
            DiagnosticKind::LargeFunction { lines } => {
                format!("function is too large ({} lines), consider breaking it into smaller functions", lines)
            },
            DiagnosticKind::DeepNesting { depth } => {
                format!("deeply nested code ({} levels), consider refactoring", depth)
            },
            
            // Style warnings
            DiagnosticKind::NamingConvention { name, expected_style } => {
                format!("name `{}` should follow {} naming convention", name, expected_style)
            },
            DiagnosticKind::MissingDocumentation { item } => {
                format!("missing documentation for {}", item)
            },
            DiagnosticKind::LongLine { length, max } => {
                format!("line too long ({} characters, maximum is {})", length, max)
            },
            
            DiagnosticKind::Custom { message } => message.clone(),
        }
    }

    pub fn default_severity(&self) -> Severity {
        match self {
            // Critical errors that prevent compilation
            DiagnosticKind::UnterminatedString
            | DiagnosticKind::InvalidNumber
            | DiagnosticKind::UnknownCharacter
            | DiagnosticKind::InvalidEscapeSequence { .. }
            | DiagnosticKind::UnexpectedToken { .. }
            | DiagnosticKind::MissingToken { .. }
            | DiagnosticKind::InvalidExpression
            | DiagnosticKind::InvalidStatement
            | DiagnosticKind::UnexpectedEof
            | DiagnosticKind::InvalidFunctionSignature
            | DiagnosticKind::InvalidVariableDeclaration
            | DiagnosticKind::UndefinedVariable { .. }
            | DiagnosticKind::UndefinedFunction { .. }
            | DiagnosticKind::TypeMismatch { .. }
            | DiagnosticKind::DuplicateDefinition { .. }
            | DiagnosticKind::InvalidAssignment { .. }
            | DiagnosticKind::WrongArgumentCount { .. }
            | DiagnosticKind::ArgumentCountMismatch { .. }
            | DiagnosticKind::InvalidReturnType { .. }
            | DiagnosticKind::MissingReturn { .. }
            | DiagnosticKind::InvalidFunctionCall { .. }
            | DiagnosticKind::UndefinedMethod { .. }
            | DiagnosticKind::UninitializedVariable { .. }
            | DiagnosticKind::ImmutableAssignment { .. }
            | DiagnosticKind::BreakOutsideLoop
            | DiagnosticKind::ContinueOutsideLoop
            | DiagnosticKind::InvalidCondition { .. }
            | DiagnosticKind::ModuleNotFound { .. }
            | DiagnosticKind::CircularImport { .. }
            | DiagnosticKind::InvalidImport { .. } => Severity::Error,
            
            // Warnings for code quality and potential issues
            DiagnosticKind::UnreachableCode
            | DiagnosticKind::DeadCode { .. }
            | DiagnosticKind::UnusedVariable { .. }
            | DiagnosticKind::ShadowedVariable { .. }
            | DiagnosticKind::InefficientCode { .. }
            | DiagnosticKind::LargeFunction { .. }
            | DiagnosticKind::DeepNesting { .. }
            | DiagnosticKind::NamingConvention { .. }
            | DiagnosticKind::MissingDocumentation { .. }
            | DiagnosticKind::LongLine { .. } => Severity::Warning,
            
            // Style suggestions (optional semicolon in Razen)
            DiagnosticKind::MissingSemicolon => Severity::Note,
            
            DiagnosticKind::Custom { .. } => Severity::Error,
        }
    }

    // Convenience constructors for common error types
    pub fn custom<S: Into<String>>(message: S) -> Self {
        DiagnosticKind::Custom { message: message.into() }
    }

    pub fn unexpected_token<S: Into<String>>(expected: Vec<S>, found: S) -> Self {
        DiagnosticKind::UnexpectedToken {
            expected: expected.into_iter().map(|s| s.into()).collect(),
            found: found.into(),
        }
    }

    pub fn missing_token<S: Into<String>>(expected: S) -> Self {
        DiagnosticKind::MissingToken { expected: expected.into() }
    }

    pub fn undefined_variable<S: Into<String>>(name: S) -> Self {
        DiagnosticKind::UndefinedVariable { name: name.into() }
    }

    pub fn undefined_function<S: Into<String>>(name: S) -> Self {
        DiagnosticKind::UndefinedFunction { name: name.into() }
    }

    pub fn type_mismatch<S: Into<String>>(expected: S, found: S) -> Self {
        DiagnosticKind::TypeMismatch {
            expected: expected.into(),
            found: found.into(),
        }
    }

    pub fn duplicate_definition<S: Into<String>>(name: S) -> Self {
        DiagnosticKind::DuplicateDefinition { name: name.into() }
    }

    pub fn wrong_argument_count(expected: usize, found: usize) -> Self {
        DiagnosticKind::WrongArgumentCount { expected, found }
    }

    pub fn unused_variable<S: Into<String>>(name: S) -> Self {
        DiagnosticKind::UnusedVariable { name: name.into() }
    }

    pub fn shadowed_variable<S: Into<String>>(name: S, previous_line: usize) -> Self {
        DiagnosticKind::ShadowedVariable {
            name: name.into(),
            previous_line,
        }
    }

    pub fn naming_convention<S: Into<String>>(name: S, expected_style: S) -> Self {
        DiagnosticKind::NamingConvention {
            name: name.into(),
            expected_style: expected_style.into(),
        }
    }

    pub fn large_function(lines: usize) -> Self {
        DiagnosticKind::LargeFunction { lines }
    }

    pub fn deep_nesting(depth: usize) -> Self {
        DiagnosticKind::DeepNesting { depth }
    }
}

/// A label that points to a specific part of the source code
#[derive(Debug, Clone)]
pub struct Label {
    pub span: Span,
    pub message: Option<String>,
    pub severity: Severity,
}

impl Label {
    pub fn new(span: Span) -> Self {
        Label {
            span,
            message: None,
            severity: Severity::Error,
        }
    }

    pub fn with_message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub fn primary(span: Span) -> Self {
        Label::new(span).with_severity(Severity::Error)
    }

    pub fn secondary(span: Span) -> Self {
        Label::new(span).with_severity(Severity::Note)
    }

    pub fn help(span: Span) -> Self {
        Label::new(span).with_severity(Severity::Help)
    }
}

/// A complete diagnostic with all information needed for display
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub severity: Severity,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
    pub help: Vec<String>,
    pub code: Option<String>, // Error code like E0001
}

impl Diagnostic {
    pub fn new(kind: DiagnosticKind) -> Self {
        let severity = kind.default_severity();
        Diagnostic {
            kind,
            severity,
            labels: Vec::new(),
            notes: Vec::new(),
            help: Vec::new(),
            code: None,
        }
    }

    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    pub fn with_labels(mut self, labels: Vec<Label>) -> Self {
        self.labels.extend(labels);
        self
    }

    pub fn with_note<S: Into<String>>(mut self, note: S) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn with_help<S: Into<String>>(mut self, help: S) -> Self {
        self.help.push(help.into());
        self
    }

    pub fn with_code<S: Into<String>>(mut self, code: S) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn title(&self) -> String {
        self.kind.title()
    }

    /// Get the primary span (first error-level label)
    pub fn primary_span(&self) -> Option<&Span> {
        self.labels
            .iter()
            .find(|label| label.severity == Severity::Error)
            .map(|label| &label.span)
    }
}

/// Builder for creating diagnostics
pub struct DiagnosticBuilder {
    diagnostic: Diagnostic,
}

impl DiagnosticBuilder {
    pub fn new(kind: DiagnosticKind) -> Self {
        DiagnosticBuilder {
            diagnostic: Diagnostic::new(kind),
        }
    }

    pub fn severity(mut self, severity: Severity) -> Self {
        self.diagnostic.severity = severity;
        self
    }

    pub fn span(self, span: Span) -> Self {
        self.label(Label::primary(span))
    }

    pub fn label(mut self, label: Label) -> Self {
        self.diagnostic.labels.push(label);
        self
    }

    pub fn note<S: Into<String>>(mut self, note: S) -> Self {
        self.diagnostic.notes.push(note.into());
        self
    }

    pub fn help<S: Into<String>>(mut self, help: S) -> Self {
        self.diagnostic.help.push(help.into());
        self
    }

    pub fn code<S: Into<String>>(mut self, code: S) -> Self {
        self.diagnostic.code = Some(code.into());
        self
    }

    pub fn build(self) -> Diagnostic {
        self.diagnostic
    }
}

/// Collection of diagnostics
#[derive(Debug, Clone, Default)]
pub struct Diagnostics {
    pub diagnostics: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Diagnostics {
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn extend(&mut self, other: Diagnostics) {
        self.diagnostics.extend(other.diagnostics);
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Error).count()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Warning).count()
    }

    /// Sort diagnostics by position
    pub fn sort(&mut self) {
        self.diagnostics.sort_by(|a, b| {
            match (a.primary_span(), b.primary_span()) {
                (Some(span_a), Some(span_b)) => span_a.start.cmp(&span_b.start),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
    }
}

impl IntoIterator for Diagnostics {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.into_iter()
    }
}

impl<'a> IntoIterator for &'a Diagnostics {
    type Item = &'a Diagnostic;
    type IntoIter = std::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.iter()
    }
}

// Additional convenience functions for diagnostics