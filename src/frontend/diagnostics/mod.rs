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
            diagnostic = diagnostic.with_help("Use `fun` instead of `function` in Razen");
        } else if expected_str.contains(&"var".to_string()) && found_str == "let" {
            diagnostic = diagnostic.with_help("Use `var` instead of `let` in Razen");
        } else if expected_str.contains(&"{".to_string()) {
            diagnostic = diagnostic.with_help("Expected opening brace `{` to start a block");
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
            ")" => diagnostic = diagnostic.with_help("Missing closing parenthesis"),
            "}" => diagnostic = diagnostic.with_help("Missing closing brace"),
            ";" => diagnostic = diagnostic.with_note("Semicolons are optional in Razen but can help with clarity"),
            _ => {}
        }

        diagnostic
    }

    /// Create an undefined variable diagnostic with smart suggestions
    pub fn undefined_variable<S: Into<String>>(name: S, span: Span) -> Diagnostic {
        undefined_variable_with_suggestions(name, span, &[])
    }

    /// Create an undefined variable diagnostic with similar names for suggestions
    pub fn undefined_variable_with_suggestions<S: Into<String>>(name: S, span: Span, similar_names: &[String]) -> Diagnostic {
        let name_str = name.into();
        let mut diagnostic = Diagnostic::new(DiagnosticKind::undefined_variable(name_str.clone()))
            .with_label(Label::primary(span).with_message(format!("cannot find value `{}` in this scope", name_str)))
            .with_code("E0004");

        // Suggest similar names using Levenshtein distance
        if !similar_names.is_empty() {
            let best_match = find_best_match(&name_str, similar_names);
            if let Some(suggestion) = best_match {
                diagnostic = diagnostic.with_help(format!("Did you mean `{}`?", suggestion));
                
                // Add note about case sensitivity if it's a case mismatch
                if name_str.to_lowercase() == suggestion.to_lowercase() {
                    diagnostic = diagnostic.with_note("Razen is case-sensitive");
                }
                return diagnostic;
            }
        }

        // Add smart suggestions based on variable name
        if name_str.chars().next().map_or(false, |c| c.is_uppercase()) {
            diagnostic = diagnostic.with_help("Variable names should start with lowercase in Razen");
        }
        
        // Check for common typos (snake_case to camelCase)
        if name_str.contains("_") {
            let camel_case = to_camel_case(&name_str);
            diagnostic = diagnostic.with_help(format!("Did you mean `{}`? Razen uses camelCase for variables", camel_case));
        }
        
        diagnostic = diagnostic.with_help(format!("Declare `{}` with `var {} = value` before using it", name_str, name_str));
        
        diagnostic
    }

    /// Create an undefined function diagnostic with suggestions
    pub fn undefined_function<S: Into<String>>(name: S, span: Span) -> Diagnostic {
        undefined_function_with_suggestions(name, span, &[])
    }

    /// Create an undefined function diagnostic with similar names for suggestions
    pub fn undefined_function_with_suggestions<S: Into<String>>(name: S, span: Span, similar_names: &[String]) -> Diagnostic {
        let name_str = name.into();
        let mut diagnostic = Diagnostic::new(DiagnosticKind::undefined_function(name_str.clone()))
            .with_label(Label::primary(span).with_message(format!("cannot find function `{}` in this scope", name_str)))
            .with_code("E0005");

        // Suggest similar function names
        if !similar_names.is_empty() {
            let best_match = find_best_match(&name_str, similar_names);
            if let Some(suggestion) = best_match {
                diagnostic = diagnostic.with_help(format!("Did you mean `{}`?", suggestion));
                return diagnostic;
            }
        }

        // Suggest common built-in functions if similar
        match name_str.as_str() {
            "print_line" | "printline" | "print_ln" => {
                diagnostic = diagnostic.with_help("Did you mean `println`?");
            }
            "printf" | "print_f" => {
                diagnostic = diagnostic.with_help("Use `print` or `println` for output in Razen");
            }
            "console.log" | "console_log" | "log" => {
                diagnostic = diagnostic.with_help("Use `println` for console output in Razen");
            }
            "puts" | "echo" => {
                diagnostic = diagnostic.with_help("Use `println` for output in Razen");
            }
            "str" | "string" | "String" => {
                diagnostic = diagnostic.with_help("Use `tostr()` to convert values to strings");
            }
            "int" | "integer" | "parseInt" => {
                diagnostic = diagnostic.with_help("Use `toint()` to convert values to integers");
            }
            "float" | "parseFloat" => {
                diagnostic = diagnostic.with_help("Use `tofloat()` to convert values to floats");
            }
            _ => {
                diagnostic = diagnostic.with_help(format!("Define function `{}` with `fun {}() {{ ... }}` or check if it's imported", name_str, name_str));
            }
        }

        diagnostic
    }

    /// Create a type mismatch diagnostic with conversion suggestions
    pub fn type_mismatch<S: Into<String>>(expected: S, found: S, span: Span) -> Diagnostic {
        let expected_str = expected.into();
        let found_str = found.into();
        
        let mut diagnostic = Diagnostic::new(DiagnosticKind::type_mismatch(expected_str.clone(), found_str.clone()))
            .with_label(Label::primary(span).with_message(format!("expected `{}`, found `{}`", expected_str, found_str)))
            .with_code("E0006");

        // Add conversion suggestions with examples
        match (expected_str.as_str(), found_str.as_str()) {
            ("str", "int") | ("string", "int") => {
                diagnostic = diagnostic.with_help("Convert with `tostr(value)` or use f-string: `f\"{value}\"`");
            }
            ("int", "str") | ("int", "string") => {
                diagnostic = diagnostic.with_help("Convert with `toint(value)`");
                diagnostic = diagnostic.with_note("Use `toint()` for string to integer conversion");
            }
            ("float", "int") => {
                diagnostic = diagnostic.with_help("Convert with `tofloat(value)` or add decimal: `5.0`");
            }
            ("int", "float") => {
                diagnostic = diagnostic.with_help("Convert with `toint(value)` to truncate decimal");
                diagnostic = diagnostic.with_note("This will discard the decimal part");
            }
            ("float", "str") | ("float", "string") => {
                diagnostic = diagnostic.with_help("Convert with `tofloat(value)`");
            }
            ("bool", "int") => {
                diagnostic = diagnostic.with_help("Use comparison: `value != 0` or `value > 0`");
            }
            ("bool", "str") | ("bool", "string") => {
                diagnostic = diagnostic.with_help("Use comparison: `value == \\\"true\\\"` or check if not empty");
            }
            ("bool", _) => {
                diagnostic = diagnostic.with_help("Use comparison operators (==, !=, <, >, <=, >=) to create boolean values");
            }
            ("str", "bool") | ("string", "bool") => {
                diagnostic = diagnostic.with_help("Convert with `tostr(value)` or use conditional: `value ? \\\"true\\\" : \\\"false\\\"`");
            }
            ("str", "float") | ("string", "float") => {
                diagnostic = diagnostic.with_help("Convert with `tostr(value)` or use f-string: `f\"{value}\"`");
            }
            _ => {
                diagnostic = diagnostic.with_help(format!("Cannot automatically convert from `{}` to `{}`", found_str, expected_str));
                diagnostic = diagnostic.with_note("Consider using explicit type conversion functions: toint(), tofloat(), tostr()");
            }
        }

        diagnostic
    }

    /// Create a duplicate definition diagnostic with clear context
    pub fn duplicate_definition<S: Into<String>>(name: S, span: Span, previous_span: Option<Span>) -> Diagnostic {
        let mut diagnostic = Diagnostic::new(DiagnosticKind::duplicate_definition(name))
            .with_label(Label::primary(span).with_message("redefined here"))
            .with_code("E0007")
            .with_help("Consider using a different name or removing one of the definitions");

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
            .with_help(format!("This function expects {} argument{}", 
                              expected, if expected == 1 { "" } else { "s" }))
    }

    /// Create an unused variable warning with suggestions
    pub fn unused_variable<S: Into<String>>(name: S, span: Span) -> Diagnostic {
        let name_str = name.into();
        let mut diagnostic = Diagnostic::new(DiagnosticKind::unused_variable(name_str.clone()))
            .with_label(Label::new(span).with_severity(Severity::Warning))
            .with_code("W0001")
            .with_help(format!("Prefix with underscore if intentional: `_{}`", name_str));
        
        // Add context-specific suggestions
        if name_str == "result" || name_str == "value" || name_str == "data" {
            diagnostic = diagnostic.with_note("Consider removing this variable if it's not needed");
        }
        
        diagnostic
    }

    /// Create a variable shadowing warning
    pub fn shadowed_variable<S: Into<String>>(name: S, span: Span, previous_line: usize) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::shadowed_variable(name, previous_line))
            .with_label(Label::new(span).with_severity(Severity::Warning))
            .with_code("W0002")
            .with_help("Consider using a different variable name to avoid confusion")
    }

    /// Create a naming convention warning
    pub fn naming_convention<S: Into<String>>(name: S, expected_style: S, span: Span) -> Diagnostic {
        let name_str = name.into();
        let style_str = expected_style.into();
        
        Diagnostic::new(DiagnosticKind::naming_convention(name_str.clone(), style_str.clone()))
            .with_label(Label::new(span).with_severity(Severity::Warning))
            .with_code("W0003")
            .with_help(format!("{} names should follow {} convention", 
                              if name_str.chars().next().map_or(false, |c| c.is_uppercase()) { "Type" } else { "Variable" },
                              style_str))
    }

    /// Create a performance warning for large functions
    pub fn large_function(lines: usize, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::large_function(lines))
            .with_label(Label::new(span).with_severity(Severity::Warning))
            .with_code("W0004")
            .with_help("Consider breaking this function into smaller, more focused functions")
    }

    /// Create a warning for deeply nested code
    pub fn deep_nesting(depth: usize, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::deep_nesting(depth))
            .with_label(Label::new(span).with_severity(Severity::Warning))
            .with_code("W0005")
            .with_help("Consider extracting nested logic into separate functions or using early returns")
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

    /// Create a type error diagnostic
    pub fn type_error<S: Into<String>>(message: S, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::custom(message))
            .with_label(Label::primary(span))
            .with_code("E0020")
    }

    /// Create a division by zero error
    pub fn division_by_zero(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::DivisionByZero)
            .with_label(Label::primary(span))
            .with_code("E0021")
            .with_help("Ensure the divisor is not zero before performing division")
            .with_note("Division by zero is undefined and will cause runtime errors")
    }

    /// Create an index out of bounds error
    pub fn index_out_of_bounds(index: i64, length: usize, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::IndexOutOfBounds { index, length })
            .with_label(Label::primary(span))
            .with_code("E0022")
            .with_help(format!("Valid indices are 0 to {}", length.saturating_sub(1)))
            .with_note("Array indices in Razen are zero-based")
    }

    /// Create a missing field error
    pub fn missing_field<S: Into<String>>(field: S, type_name: S, span: Span) -> Diagnostic {
        let field_str = field.into();
        let type_str = type_name.into();
        
        Diagnostic::new(DiagnosticKind::MissingField { 
            field: field_str.clone(), 
            type_name: type_str.clone() 
        })
            .with_label(Label::primary(span))
            .with_code("E0023")
            .with_help(format!("Check the definition of struct `{}` for available fields", type_str))
    }

    /// Create an immutable assignment error with context
    pub fn immutable_assignment<S: Into<String>>(name: S, span: Span) -> Diagnostic {
        let name_str = name.into();
        Diagnostic::new(DiagnosticKind::ImmutableAssignment { name: name_str.clone() })
            .with_label(Label::primary(span).with_message("cannot assign to immutable variable"))
            .with_code("E0011")
            .with_help(format!("Declare `{}` as mutable with `var {}` if you need to modify it", name_str, name_str))
            .with_note("Variables in Razen are immutable when declared with `const` unless declared with `var`")
    }

    /// Create a break outside loop error
    pub fn break_outside_loop(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::BreakOutsideLoop)
            .with_label(Label::primary(span))
            .with_code("E0009")
            .with_help("Use `break` only inside `for` or `while` statements")
    }

    /// Create a continue outside loop error
    pub fn continue_outside_loop(span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::ContinueOutsideLoop)
            .with_label(Label::primary(span))
            .with_code("E0010")
            .with_help("Use `continue` only inside `for` or `while` statements")
    }
    
    /// Create an invalid condition error
    pub fn invalid_condition<S: Into<String>>(found_type: S, span: Span) -> Diagnostic {
        let found_str = found_type.into();
        Diagnostic::new(DiagnosticKind::InvalidCondition { found: found_str.clone() })
            .with_label(Label::primary(span))
            .with_code("E0015")
            .with_help("Use comparison operators (==, !=, <, >, <=, >=) to create boolean conditions")
            .with_note(format!("Found type `{}` but expected `bool`", found_str))
    }
    
    /// Create an invalid lvalue error
    pub fn invalid_lvalue<S: Into<String>>(reason: S, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::InvalidLValue { reason: reason.into() })
            .with_label(Label::primary(span))
            .with_code("E0014")
            .with_help("Only variables, struct fields, and array elements can be assigned to")
    }
    
    /// Create an invalid operand error
    pub fn invalid_operand<S: Into<String>>(operator: S, operand_type: S, span: Span) -> Diagnostic {
        let op_str = operator.into();
        let type_str = operand_type.into();
        
        Diagnostic::new(DiagnosticKind::InvalidOperand { 
            operator: op_str.clone(), 
            operand_type: type_str.clone() 
        })
            .with_label(Label::primary(span))
            .with_code("E0016")
            .with_help(format!("Operator `{}` cannot be applied to type `{}`", op_str, type_str))
    }
    
    /// Create a type not found error
    pub fn type_not_found<S: Into<String>>(type_name: S, span: Span) -> Diagnostic {
        let name_str = type_name.into();
        let mut diagnostic = Diagnostic::new(DiagnosticKind::TypeNotFound { type_name: name_str.clone() })
            .with_label(Label::primary(span))
            .with_code("E0017");
        
        // Check for common case sensitivity mistakes
        let lowercase_name = name_str.to_lowercase();
        match lowercase_name.as_str() {
            "int" | "float" | "str" | "string" | "bool" | "char" => {
                let correct_name = if lowercase_name == "string" { "str" } else { &lowercase_name };
                diagnostic = diagnostic.with_help(format!("Did you mean `{}`? Type names in Razen are lowercase", correct_name));
            }
            _ => {
                diagnostic = diagnostic.with_help(format!("Define type `{}` with `struct {}` or `type {} = <type>`", name_str, name_str, name_str));
            }
        }
        
        diagnostic
    }
    
    /// Create an uninitialized variable error
    pub fn uninitialized_variable<S: Into<String>>(name: S, span: Span) -> Diagnostic {
        let name_str = name.into();
        Diagnostic::new(DiagnosticKind::UninitializedVariable { name: name_str.clone() })
            .with_label(Label::primary(span))
            .with_code("E0018")
            .with_help(format!("Initialize `{}` before using it: `var {} = value`", name_str, name_str))
    }
    
    /// Create a missing return statement error
    pub fn missing_return<S: Into<String>>(function_name: S, span: Span) -> Diagnostic {
        let func_str = function_name.into();
        Diagnostic::new(DiagnosticKind::MissingReturn { function_name: func_str.clone() })
            .with_label(Label::primary(span))
            .with_code("E0019")
            .with_help(format!("Add a return statement to function `{}`", func_str))
            .with_note("Functions with a return type must return a value in all code paths")
    }
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(
                    matrix[i][j + 1] + 1,
                    matrix[i + 1][j] + 1
                ),
                matrix[i][j] + cost
            );
        }
    }

    matrix[len1][len2]
}

/// Find the best match from a list of candidates
fn find_best_match(target: &str, candidates: &[String]) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }

    let mut best_match = None;
    let mut best_distance = usize::MAX;
    let max_distance = (target.len() / 2).max(2); // Allow up to half the length or 2 chars difference

    for candidate in candidates {
        let distance = levenshtein_distance(target, candidate);
        if distance < best_distance && distance <= max_distance {
            best_distance = distance;
            best_match = Some(candidate.clone());
        }
    }

    best_match
}

/// Convert snake_case to camelCase
fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// Check if a string is a valid type name (lowercase)
pub fn is_valid_type_name(name: &str) -> bool {
    matches!(name, "int" | "float" | "str" | "bool" | "char" | "any")
}

/// Get the correct type name for a possibly incorrect one
pub fn get_correct_type_name(name: &str) -> Option<&'static str> {
    match name.to_lowercase().as_str() {
        "int" => Some("int"),
        "float" => Some("float"),
        "str" | "string" => Some("str"),
        "bool" | "boolean" => Some("bool"),
        "char" | "character" => Some("char"),
        _ => None,
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