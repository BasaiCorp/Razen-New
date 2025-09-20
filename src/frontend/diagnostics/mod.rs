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

    /// Create an unexpected token diagnostic
    pub fn unexpected_token<S: Into<String>>(expected: Vec<S>, found: S, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::unexpected_token(expected, found))
            .with_label(Label::primary(span))
            .with_code("E0002")
    }

    /// Create a missing token diagnostic
    pub fn missing_token<S: Into<String>>(expected: S, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::missing_token(expected))
            .with_label(Label::primary(span))
            .with_code("E0003")
    }

    /// Create an undefined variable diagnostic
    pub fn undefined_variable<S: Into<String>>(name: S, span: Span) -> Diagnostic {
        let name_str = name.into();
        Diagnostic::new(DiagnosticKind::undefined_variable(&name_str))
            .with_label(Label::primary(span).with_message("not found in this scope"))
            .with_code("E0004")
            .with_help(format!("help: consider importing `{}` or defining it in this scope", name_str))
    }

    /// Create an undefined function diagnostic
    pub fn undefined_function<S: Into<String>>(name: S, span: Span) -> Diagnostic {
        let name_str = name.into();
        Diagnostic::new(DiagnosticKind::undefined_function(&name_str))
            .with_label(Label::primary(span).with_message("not found in this scope"))
            .with_code("E0005")
            .with_help(format!("help: consider importing `{}` or defining it in this scope", name_str))
    }

    /// Create a type mismatch diagnostic
    pub fn type_mismatch<S: Into<String>>(expected: S, found: S, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::type_mismatch(expected, found))
            .with_label(Label::primary(span))
            .with_code("E0006")
    }

    /// Create a duplicate definition diagnostic
    pub fn duplicate_definition<S: Into<String>>(name: S, span: Span, previous_span: Option<Span>) -> Diagnostic {
        let mut diagnostic = Diagnostic::new(DiagnosticKind::duplicate_definition(name))
            .with_label(Label::primary(span).with_message("redefined here"))
            .with_code("E0007");

        if let Some(prev_span) = previous_span {
            diagnostic = diagnostic.with_label(
                Label::secondary(prev_span).with_message("previous definition here")
            );
        }

        diagnostic
    }

    /// Create a warning diagnostic
    pub fn warning<S: Into<String>>(message: S, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::custom(message))
            .with_severity(Severity::Warning)
            .with_label(Label::new(span).with_severity(Severity::Warning))
            .with_code("W0001")
    }

    /// Create a note diagnostic
    pub fn note<S: Into<String>>(message: S, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::custom(message))
            .with_severity(Severity::Note)
            .with_label(Label::new(span).with_severity(Severity::Note))
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