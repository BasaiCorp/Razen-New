// src/frontend/diagnostics/mod.rs

pub mod error;
pub mod display;

// Re-export commonly used types
pub use error::{
    Diagnostic, DiagnosticBuilder, DiagnosticKind, Diagnostics,
    Label, Position, Severity, Span,
};

pub use display::{
    DiagnosticRenderer, DisplayConfig, SourceManager, SourceFile,
    render_diagnostic, render_diagnostics,
};

/// Convenience macro for creating diagnostics
#[macro_export]
macro_rules! diagnostic {
    ($kind:expr) => {
        $crate::frontend::diagnostics::Diagnostic::new($kind)
    };
    ($kind:expr, $span:expr) => {
        $crate::frontend::diagnostics::Diagnostic::new($kind)
            .with_label($crate::frontend::diagnostics::Label::primary($span))
    };
    ($kind:expr, $span:expr, $message:expr) => {
        $crate::frontend::diagnostics::Diagnostic::new($kind)
            .with_label($crate::frontend::diagnostics::Label::primary($span).with_message($message))
    };
}

/// Convenience macro for creating spans
#[macro_export]
macro_rules! span {
    ($start_line:expr, $start_col:expr, $end_line:expr, $end_col:expr) => {
        $crate::frontend::diagnostics::Span::new(
            $crate::frontend::diagnostics::Position::new($start_line, $start_col, 0),
            $crate::frontend::diagnostics::Position::new($end_line, $end_col, 0),
        )
    };
    ($line:expr, $start_col:expr, $end_col:expr) => {
        span!($line, $start_col, $line, $end_col)
    };
}

/// Helper functions for common diagnostic patterns
pub mod helpers {
    use super::*;

    /// Create a syntax error diagnostic
    pub fn syntax_error<S: Into<String>>(message: S, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::custom(message))
            .with_label(Label::primary(span))
            .with_code("E0001")
    }

    /// Create an unexpected token diagnostic with helpful suggestions
    pub fn unexpected_token<S: Into<String>>(expected: Vec<S>, found: S, span: Span) -> Diagnostic {
        let expected_str: Vec<String> = expected.into_iter().map(|s| s.into()).collect();
        let found_str = found.into();
        
        let mut diagnostic = Diagnostic::new(DiagnosticKind::unexpected_token(expected_str.clone(), found_str.clone()))
            .with_label(Label::primary(span))
            .with_code("E0002");

        // Add helpful suggestions based on common mistakes
        if expected_str.contains(&"fun".to_string()) && found_str == "function" {
            diagnostic = diagnostic.with_help("help: use `fun` instead of `function` in Razen");
        } else if expected_str.contains(&"var".to_string()) && found_str == "let" {
            diagnostic = diagnostic.with_help("help: use `var` instead of `let` in Razen");
        } else if expected_str.contains(&"{".to_string()) {
            diagnostic = diagnostic.with_help("help: expected opening brace `{` to start a block");
        }

        diagnostic
    }

    /// Create a missing token diagnostic with context
    pub fn missing_token<S: Into<String>>(expected: S, span: Span) -> Diagnostic {
        let expected_str = expected.into();
        let mut diagnostic = Diagnostic::new(DiagnosticKind::missing_token(expected_str.clone()))
            .with_label(Label::primary(span))
            .with_code("E0003");

        // Add context-specific help
        match expected_str.as_str() {
            ")" => diagnostic = diagnostic.with_help("help: missing closing parenthesis"),
            "}" => diagnostic = diagnostic.with_help("help: missing closing brace"),
            ";" => diagnostic = diagnostic.with_note("note: semicolons are optional in Razen but can help with clarity"),
            _ => {}
        }

        diagnostic
    }

    /// Create an undefined variable diagnostic with smart suggestions
    pub fn undefined_variable<S: Into<String>>(name: S, span: Span) -> Diagnostic {
        let name_str = name.into();
        let mut diagnostic = Diagnostic::new(DiagnosticKind::undefined_variable(name_str.clone()))
            .with_label(Label::primary(span).with_message("not found in this scope"))
            .with_code("E0004");

        // Add smart suggestions based on variable name
        if name_str.chars().next().map_or(false, |c| c.is_uppercase()) {
            diagnostic = diagnostic.with_help("help: variable names should start with lowercase in Razen");
        }
        
        diagnostic = diagnostic.with_help(format!("help: consider declaring `{}` before using it", name_str));
        
        diagnostic
    }

    /// Create an undefined function diagnostic with suggestions
    pub fn undefined_function<S: Into<String>>(name: S, span: Span) -> Diagnostic {
        let name_str = name.into();
        let mut diagnostic = Diagnostic::new(DiagnosticKind::undefined_function(name_str.clone()))
            .with_label(Label::primary(span).with_message("not found in this scope"))
            .with_code("E0005");

        // Suggest common built-in functions if similar
        match name_str.as_str() {
            "print_line" | "printline" => {
                diagnostic = diagnostic.with_help("help: did you mean `println`?");
            }
            "printf" | "print_f" => {
                diagnostic = diagnostic.with_help("help: use `print` or `println` for output in Razen");
            }
            _ => {
                diagnostic = diagnostic.with_help(format!("help: define function `{}` or check if it's imported", name_str));
            }
        }

        diagnostic
    }

    /// Create a type mismatch diagnostic with conversion suggestions
    pub fn type_mismatch<S: Into<String>>(expected: S, found: S, span: Span) -> Diagnostic {
        let expected_str = expected.into();
        let found_str = found.into();
        
        let mut diagnostic = Diagnostic::new(DiagnosticKind::type_mismatch(expected_str.clone(), found_str.clone()))
            .with_label(Label::primary(span))
            .with_code("E0006");

        // Add conversion suggestions
        match (expected_str.as_str(), found_str.as_str()) {
            ("str", "int") => {
                diagnostic = diagnostic.with_help("help: use string interpolation or `to_string()` to convert");
            }
            ("int", "str") => {
                diagnostic = diagnostic.with_help("help: use `parse()` or `to_int()` to convert string to integer");
            }
            ("bool", _) => {
                diagnostic = diagnostic.with_help("help: use comparison operators (==, !=, <, >) to create boolean values");
            }
            _ => {}
        }

        diagnostic
    }

    /// Create a duplicate definition diagnostic with clear context
    pub fn duplicate_definition<S: Into<String>>(name: S, span: Span, previous_span: Option<Span>) -> Diagnostic {
        let mut diagnostic = Diagnostic::new(DiagnosticKind::duplicate_definition(name))
            .with_label(Label::primary(span).with_message("redefined here"))
            .with_code("E0007")
            .with_help("help: consider using a different name or removing one of the definitions");

        if let Some(prev_span) = previous_span {
            diagnostic = diagnostic.with_label(
                Label::secondary(prev_span).with_message("previous definition here")
            );
        }

        diagnostic
    }

    /// Create a function argument count mismatch diagnostic
    pub fn wrong_argument_count(expected: usize, found: usize, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::wrong_argument_count(expected, found))
            .with_label(Label::primary(span))
            .with_code("E0008")
            .with_help(format!("help: this function expects {} argument{}", 
                              expected, if expected == 1 { "" } else { "s" }))
    }

    /// Create an unused variable warning with suggestions
    pub fn unused_variable<S: Into<String>>(name: S, span: Span) -> Diagnostic {
        let name_str = name.into();
        Diagnostic::new(DiagnosticKind::unused_variable(name_str.clone()))
            .with_label(Label::primary(span))
            .with_code("W0001")
            .with_help(format!("help: if this is intentional, prefix the name with `_` (e.g., `_{}`)", name_str))
    }

    /// Create a variable shadowing warning
    pub fn shadowed_variable<S: Into<String>>(name: S, span: Span, previous_line: usize) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::shadowed_variable(name, previous_line))
            .with_label(Label::primary(span))
            .with_code("W0002")
            .with_help("help: consider using a different variable name to avoid confusion")
    }

    /// Create a naming convention warning
    pub fn naming_convention<S: Into<String>>(name: S, expected_style: S, span: Span) -> Diagnostic {
        let name_str = name.into();
        let style_str = expected_style.into();
        
        Diagnostic::new(DiagnosticKind::naming_convention(name_str.clone(), style_str.clone()))
            .with_label(Label::primary(span))
            .with_code("W0003")
            .with_help(format!("help: {} names should follow {} convention", 
                              if name_str.chars().next().map_or(false, |c| c.is_uppercase()) { "Type" } else { "Variable" },
                              style_str))
    }

    /// Create a performance warning for large functions
    pub fn large_function(lines: usize, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::large_function(lines))
            .with_label(Label::primary(span))
            .with_code("W0004")
            .with_help("help: consider breaking this function into smaller, more focused functions")
    }

    /// Create a warning for deeply nested code
    pub fn deep_nesting(depth: usize, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::deep_nesting(depth))
            .with_label(Label::primary(span))
            .with_code("W0005")
            .with_help("help: consider extracting nested logic into separate functions or using early returns")
    }

    /// Create a general warning diagnostic
    pub fn warning<S: Into<String>>(message: S, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::custom(message))
            .with_severity(Severity::Warning)
            .with_label(Label::new(span).with_severity(Severity::Warning))
            .with_code("W0099")
    }

    /// Create a note diagnostic
    pub fn note<S: Into<String>>(message: S, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::custom(message))
            .with_severity(Severity::Note)
            .with_label(Label::new(span).with_severity(Severity::Note))
    }

    /// Create a helpful tip diagnostic
    pub fn tip<S: Into<String>>(message: S, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::custom(message))
            .with_severity(Severity::Help)
            .with_label(Label::new(span).with_severity(Severity::Help))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_creation() {
        let span = Span::new(
            Position::new(1, 1, 0),
            Position::new(1, 5, 4),
        );

        let diagnostic = helpers::syntax_error("test error", span);
        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.code, Some("E0001".to_string()));
    }

    #[test]
    fn test_diagnostic_macro() {
        let span = Span::new(
            Position::new(1, 1, 0),
            Position::new(1, 5, 4),
        );

        let diagnostic = diagnostic!(DiagnosticKind::custom("test"), span, "test message");
        assert_eq!(diagnostic.labels.len(), 1);
        assert_eq!(diagnostic.labels[0].message, Some("test message".to_string()));
    }

    #[test]
    fn test_span_macro() {
        let span = span!(1, 5, 10);
        assert_eq!(span.start.line, 1);
        assert_eq!(span.start.column, 5);
        assert_eq!(span.end.line, 1);
        assert_eq!(span.end.column, 10);
    }
}