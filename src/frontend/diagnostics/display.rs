// src/frontend/diagnostics/display.rs

use super::error::{Diagnostic, Diagnostics, Label, Severity};
use std::collections::HashMap;

/// Configuration for diagnostic display
#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub use_colors: bool,
    pub show_line_numbers: bool,
    pub context_lines: usize,
    pub tab_width: usize,
    pub max_line_length: usize,
    pub show_source_path: bool,
    pub compact_mode: bool,
    pub show_suggestions: bool,
    pub unicode_symbols: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            use_colors: true,
            show_line_numbers: true,
            context_lines: 2,
            tab_width: 4,
            max_line_length: 120,
            show_source_path: true,
            compact_mode: false,
            show_suggestions: true,
            unicode_symbols: true,
        }
    }
}

impl DisplayConfig {
    /// Create a minimal configuration for CI/automated environments
    pub fn minimal() -> Self {
        DisplayConfig {
            use_colors: false,
            show_line_numbers: true,
            context_lines: 1,
            tab_width: 4,
            max_line_length: 80,
            show_source_path: true,
            compact_mode: true,
            show_suggestions: false,
            unicode_symbols: false,
        }
    }

    /// Create a rich configuration for interactive development
    pub fn rich() -> Self {
        DisplayConfig {
            use_colors: true,
            show_line_numbers: true,
            context_lines: 3,
            tab_width: 4,
            max_line_length: 120,
            show_source_path: true,
            compact_mode: false,
            show_suggestions: true,
            unicode_symbols: true,
        }
    }
}

/// Source code manager for diagnostic display
#[derive(Debug, Clone)]
pub struct SourceManager {
    sources: HashMap<String, SourceFile>,
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub content: String,
    pub lines: Vec<String>,
    pub name: String,
}

impl SourceFile {
    pub fn new(name: String, content: String) -> Self {
        let lines = content.lines().map(|s| s.to_string()).collect();
        SourceFile {
            content,
            lines,
            name,
        }
    }

    pub fn get_line(&self, line_number: usize) -> Option<&str> {
        if line_number == 0 || line_number > self.lines.len() {
            None
        } else {
            Some(&self.lines[line_number - 1])
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

impl SourceManager {
    pub fn new() -> Self {
        SourceManager {
            sources: HashMap::new(),
        }
    }

    pub fn add_source(&mut self, name: String, content: String) {
        let source_file = SourceFile::new(name.clone(), content);
        self.sources.insert(name, source_file);
    }

    pub fn get_source(&self, name: &str) -> Option<&SourceFile> {
        self.sources.get(name)
    }
}

/// Diagnostic renderer that produces Rust-style error messages
pub struct DiagnosticRenderer {
    config: DisplayConfig,
    source_manager: SourceManager,
}

impl DiagnosticRenderer {
    pub fn new(config: DisplayConfig) -> Self {
        DiagnosticRenderer {
            config,
            source_manager: SourceManager::new(),
        }
    }

    pub fn with_source_manager(mut self, source_manager: SourceManager) -> Self {
        self.source_manager = source_manager;
        self
    }

    pub fn add_source(&mut self, name: String, content: String) {
        self.source_manager.add_source(name, content);
    }

    /// Render a single diagnostic
    pub fn render_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        let mut output = String::new();

        // Header: severity and title
        self.write_header(&mut output, diagnostic);

        // Labels and source code snippets
        if !diagnostic.labels.is_empty() {
            self.write_labels(&mut output, &diagnostic.labels);
        }

        // Notes
        for note in &diagnostic.notes {
            self.write_note(&mut output, note);
        }

        // Help messages
        for help in &diagnostic.help {
            self.write_help(&mut output, help);
        }

        output
    }

    /// Render multiple diagnostics
    pub fn render_diagnostics(&self, diagnostics: &Diagnostics) -> String {
        let mut output = String::new();
        let mut sorted_diagnostics = diagnostics.clone();
        sorted_diagnostics.sort();

        for (i, diagnostic) in sorted_diagnostics.diagnostics.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            output.push_str(&self.render_diagnostic(diagnostic));
        }

        // Summary
        if !diagnostics.is_empty() {
            output.push('\n');
            self.write_summary(&mut output, diagnostics);
        }

        output
    }

    fn write_header(&self, output: &mut String, diagnostic: &Diagnostic) {
        let severity_color = if self.config.use_colors {
            diagnostic.severity.color_code()
        } else {
            ""
        };
        let reset_color = if self.config.use_colors { "\x1b[0m" } else { "" };
        let bold = if self.config.use_colors { "\x1b[1m" } else { "" };

        // Format: [ERROR] message (Razen clean style)
        let severity_label = match diagnostic.severity {
            Severity::Error => "ERROR",
            Severity::Warning => "WARNING",
            Severity::Note => "NOTE",
            Severity::Help => "HELP",
        };

        output.push_str(&format!(
            "{}{}[{}]{} {}\n",
            severity_color,
            bold,
            severity_label,
            reset_color,
            diagnostic.title()
        ));
    }

    fn write_labels(&self, output: &mut String, labels: &[Label]) {
        // Group labels by source file
        let mut labels_by_source: HashMap<Option<String>, Vec<&Label>> = HashMap::new();
        for label in labels {
            let source_id = label.span.source_id.clone();
            labels_by_source.entry(source_id).or_default().push(label);
        }

        for (source_id, source_labels) in labels_by_source {
            self.write_source_snippet(output, &source_id, source_labels);
        }
    }

    fn write_source_snippet(&self, output: &mut String, source_id: &Option<String>, labels: Vec<&Label>) {
        let source_file = match source_id {
            Some(id) => {
                // Try exact match first
                if let Some(sf) = self.source_manager.get_source(id) {
                    Some(sf)
                } else {
                    // Try all registered sources to find a match
                    // This handles cases where the path format might differ slightly
                    self.source_manager.sources.values()
                        .find(|sf| {
                            sf.name == *id || 
                            sf.name.ends_with(id) || 
                            id.ends_with(&sf.name)
                        })
                }
            },
            None => None,
        };

        if let Some(source_file) = source_file {
            self.write_source_with_file(output, source_file, labels);
        } else {
            // Fallback for when we don't have source file
            self.write_source_without_file(output, source_id, labels);
        }
    }

    fn write_source_with_file(&self, output: &mut String, source_file: &SourceFile, labels: Vec<&Label>) {
        if labels.is_empty() {
            return;
        }

        // Find the range of lines to display
        let min_line = labels.iter().map(|l| l.span.start.line).min().unwrap_or(1);
        let max_line = labels.iter().map(|l| l.span.end.line).max().unwrap_or(1);
        
        let start_line = min_line.saturating_sub(self.config.context_lines).max(1);
        let end_line = (max_line + self.config.context_lines).min(source_file.line_count());
        // Calculate line number width for alignment
        let line_num_width = end_line.to_string().len();

        // Colors
        let dim_gray = if self.config.use_colors { "\x1b[90m" } else { "" };
        let reset = if self.config.use_colors { "\x1b[0m" } else { "" };

        // File location header - clean Razen style
        output.push_str(&format!(
            "  {}:{}:{}\n",
            source_file.name,
            min_line,
            labels.first().map(|l| l.span.start.column).unwrap_or(1)
        ));

        output.push_str("\n");

        // Display lines with annotations
        for line_num in start_line..=end_line {
            if let Some(line_content) = source_file.get_line(line_num) {
                // Line number and content - simple format without | decoration
                output.push_str(&format!(
                    "{}    {:width$}  {}{}\n",
                    dim_gray,
                    line_num,
                    reset,
                    line_content,
                    width = line_num_width
                ));

                // Annotations for this line
                self.write_line_annotations(output, line_num, line_content, &labels, line_num_width);
            }
        }
    }

    fn write_line_annotations(&self, output: &mut String, line_num: usize, line_content: &str, labels: &[&Label], line_num_width: usize) {
        let mut annotations = Vec::new();

        // Collect all annotations for this line
        for label in labels {
            if label.span.start.line <= line_num && line_num <= label.span.end.line {
                let start_col = if label.span.start.line == line_num {
                    label.span.start.column.saturating_sub(1)
                } else {
                    0
                };
                
                let end_col = if label.span.end.line == line_num {
                    label.span.end.column.saturating_sub(1)
                } else {
                    line_content.len()
                };

                annotations.push((start_col, end_col, label));
            }
        }

        if annotations.is_empty() {
            return;
        }

        // Sort annotations by start position
        annotations.sort_by_key(|(start, _, _)| *start);

        // Colors
        let dim_gray = if self.config.use_colors { "\x1b[90m" } else { "" };
        let reset = if self.config.use_colors { "\x1b[0m" } else { "" };

        // Create annotation line - clean style without decorations
        let mut annotation_line = String::new();
        annotation_line.push_str(&format!("{}    {}", dim_gray, reset));
        annotation_line.push_str(&" ".repeat(line_num_width));
        annotation_line.push_str("  ");

        // Add spaces and carets
        let mut pos = 0;
        for (start_col, end_col, label) in &annotations {
            // Add spaces before the annotation
            while pos < *start_col {
                annotation_line.push(' ');
                pos += 1;
            }

            // Always use ^ for highlighting (simple and clean)
            let color = label.severity.color_code();
            let color_str = if self.config.use_colors { color } else { "" };

            if start_col == end_col || *end_col <= start_col + 1 {
                annotation_line.push_str(&format!("{}{}{}", color_str, '^', reset));
                pos += 1;
            } else {
                for _ in *start_col..*end_col {
                    annotation_line.push_str(&format!("{}{}{}", color_str, '^', reset));
                    pos += 1;
                }
            }
        }

        output.push_str(&annotation_line);
        
        // Add inline message if present
        for (_, _, label) in &annotations {
            if let Some(ref message) = label.message {
                output.push_str(&format!(" {}", message));
                break; // Only show first message inline
            }
        }
        
        output.push('\n');
    }

    fn write_source_without_file(&self, output: &mut String, source_id: &Option<String>, labels: Vec<&Label>) {
        // Fallback when we don't have the source file
        for label in labels {
            let location = if let Some(id) = source_id {
                format!("{}:{}", id, label.span.start)
            } else {
                format!("{}", label.span.start)
            };

            output.push_str(&format!("  --> {}\n", location));
            
            if let Some(ref message) = label.message {
                let color = if self.config.use_colors { label.severity.color_code() } else { "" };
                let reset = if self.config.use_colors { "\x1b[0m" } else { "" };
                output.push_str(&format!("   = {}{}{}\n", color, message, reset));
            }
        }
    }

    fn write_note(&self, output: &mut String, note: &str) {
        let blue = if self.config.use_colors { "\x1b[34m" } else { "" };
        let reset = if self.config.use_colors { "\x1b[0m" } else { "" };
        let bold = if self.config.use_colors { "\x1b[1m" } else { "" };

        output.push_str(&format!(
            "{}{}[NOTE]{} {}\n",
            blue, bold, reset, note
        ));
    }

    fn write_help(&self, output: &mut String, help: &str) {
        let cyan = if self.config.use_colors { "\x1b[36m" } else { "" };
        let reset = if self.config.use_colors { "\x1b[0m" } else { "" };
        let bold = if self.config.use_colors { "\x1b[1m" } else { "" };

        output.push_str(&format!(
            "{}{}[HELP]{} {}\n",
            cyan, bold, reset, help
        ));
    }

    fn write_summary(&self, output: &mut String, diagnostics: &Diagnostics) {
        let error_count = diagnostics.error_count();
        let warning_count = diagnostics.warning_count();

        let red = if self.config.use_colors { "\x1b[31m" } else { "" };
        let yellow = if self.config.use_colors { "\x1b[33m" } else { "" };
        let green = if self.config.use_colors { "\x1b[32m" } else { "" };
        let reset = if self.config.use_colors { "\x1b[0m" } else { "" };
        let bold = if self.config.use_colors { "\x1b[1m" } else { "" };

        if !self.config.compact_mode {
            output.push('\n');
        }

        if error_count > 0 {
            let error_plural = if error_count == 1 { "error" } else { "errors" };
            if warning_count > 0 {
                let warning_plural = if warning_count == 1 { "warning" } else { "warnings" };
                output.push_str(&format!(
                    "{}{}[INFO]{} Compilation failed with {} {}, {} {}\n",
                    red, bold, reset, error_count, error_plural, warning_count, warning_plural
                ));
            } else {
                output.push_str(&format!(
                    "{}{}[INFO]{} Compilation failed with {} {}\n",
                    red, bold, reset, error_count, error_plural
                ));
            }
        } else if warning_count > 0 {
            let warning_plural = if warning_count == 1 { "warning" } else { "warnings" };
            output.push_str(&format!(
                "{}{}[INFO]{} Compilation completed with {} {}\n",
                yellow, bold, reset, warning_count, warning_plural
            ));
        } else {
            output.push_str(&format!("{}{}[SUCCESS]{} Compilation completed\n", green, bold, reset));
        }
    }

    fn write_compilation_tips(&self, _output: &mut String, _diagnostics: &Diagnostics) {
        // Removed compilation tips for cleaner output
        // Tips are now integrated into individual error messages
    }
}

impl Default for DiagnosticRenderer {
    fn default() -> Self {
        DiagnosticRenderer::new(DisplayConfig::default())
    }
}

/// Convenience function to render diagnostics with clean, focused settings
pub fn render_diagnostics(diagnostics: &Diagnostics, sources: &[(String, String)]) -> String {
    let config = DisplayConfig {
        use_colors: true,
        show_line_numbers: true,
        context_lines: 1, // Reduced context for cleaner output
        tab_width: 4,
        max_line_length: 100,
        show_source_path: true,
        compact_mode: false,
        show_suggestions: true,
        unicode_symbols: false, // Simpler symbols for better compatibility
    };
    
    let mut renderer = DiagnosticRenderer::new(config);
    
    for (name, content) in sources {
        renderer.add_source(name.clone(), content.clone());
    }
    
    // Filter out duplicate diagnostics
    let mut unique_diagnostics = Diagnostics::new();
    let mut seen_errors = std::collections::HashSet::new();
    
    for diagnostic in &diagnostics.diagnostics {
        let error_key = format!("{:?}:{:?}", diagnostic.kind, diagnostic.labels.first().map(|l| &l.span));
        if !seen_errors.contains(&error_key) {
            seen_errors.insert(error_key);
            unique_diagnostics.add(diagnostic.clone());
        }
    }
    
    renderer.render_diagnostics(&unique_diagnostics)
}

/// Convenience function to render a single diagnostic
pub fn render_diagnostic(diagnostic: &Diagnostic, sources: &[(String, String)]) -> String {
    let mut renderer = DiagnosticRenderer::default();
    
    for (name, content) in sources {
        renderer.add_source(name.clone(), content.clone());
    }
    
    renderer.render_diagnostic(diagnostic)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::diagnostics::error::*;

    #[test]
    fn test_basic_error_display() {
        let diagnostic = Diagnostic::new(DiagnosticKind::custom("test error"))
            .with_label(Label::primary(Span::new(
                Position::new(1, 5, 4),
                Position::new(1, 10, 9),
            )).with_message("this is wrong"));

        let renderer = DiagnosticRenderer::default();
        let output = renderer.render_diagnostic(&diagnostic);
        
        assert!(output.contains("error"));
        assert!(output.contains("test error"));
    }

    #[test]
    fn test_source_code_display() {
        let source_code = "let x = 42;\nlet y = undefined_var;";
        let mut renderer = DiagnosticRenderer::default();
        renderer.add_source("test.rzn".to_string(), source_code.to_string());

        let diagnostic = Diagnostic::new(DiagnosticKind::undefined_variable("undefined_var"))
            .with_label(Label::primary(
                Span::new(
                    Position::new(2, 9, 20),
                    Position::new(2, 22, 33),
                ).with_source("test.rzn".to_string())
            ).with_message("not found in this scope"));

        let output = renderer.render_diagnostic(&diagnostic);
        
        // Debug: print the actual output to see what's wrong
        println!("Actual output: {}", output);
        
        assert!(output.contains("cannot find value `undefined_var`"));
        assert!(output.contains("test.rzn") || output.contains("2:"));
        assert!(output.contains("let y = undefined_var;"));
    }
}
