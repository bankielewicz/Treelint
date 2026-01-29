//! Human-readable text output formatting for search results.
//!
//! This module provides the [`TextFormatter`] struct for rendering
//! search results as a tree-style text layout for terminal display.
//! Output includes a header with symbol metadata, indented signature
//! and body, and a summary line with statistics. Colors are applied
//! when output is to a TTY terminal.

use colored::Colorize;

use super::json::SearchResult;

/// Number of spaces for body indentation in text output.
const BODY_INDENT: &str = "    ";
/// Number of spaces for signature indentation in text output.
const SIGNATURE_INDENT: &str = "  ";

/// Formats search results as human-readable text.
///
/// Produces a tree-style layout with:
/// - Header line: `type name (file:start-end)`
/// - Signature line (indented 2 spaces)
/// - Body lines (indented 4 spaces, omitted in signatures mode)
/// - Summary line: `Found N results in Xms (Y files searched)`
///
/// When `use_colors` is true (TTY output), applies:
/// - Cyan for symbol names
/// - Green for file paths
/// - Yellow for line numbers
pub struct TextFormatter {
    /// Whether to omit body content (signatures-only mode)
    signatures_only: bool,
    /// Whether to apply terminal colors (TTY mode)
    use_colors: bool,
}

impl TextFormatter {
    /// Create a new TextFormatter.
    ///
    /// # Arguments
    ///
    /// * `signatures_only` - If true, omit body content from output
    /// * `use_colors` - If true, apply terminal colors (for TTY output)
    pub fn new(signatures_only: bool, use_colors: bool) -> Self {
        Self {
            signatures_only,
            use_colors,
        }
    }

    /// Format search results as human-readable text.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The search term
    /// * `results` - The search results to format
    /// * `elapsed_ms` - Search duration in milliseconds
    /// * `files_searched` - Number of files searched
    ///
    /// # Returns
    ///
    /// A formatted string containing all results and summary.
    pub fn format(
        &self,
        symbol: &str,
        results: &[SearchResult],
        elapsed_ms: u64,
        files_searched: u64,
    ) -> String {
        let mut output = String::new();

        if results.is_empty() {
            output.push_str(&format!("No results found for: {}\n", symbol));
            return output;
        }

        for result in results {
            self.format_result(&mut output, result);
        }

        // Summary line
        let result_word = if results.len() == 1 {
            "result"
        } else {
            "results"
        };
        let file_word = if files_searched == 1 { "file" } else { "files" };
        output.push_str(&format!(
            "\nFound {} {} in {}ms ({} {} searched)\n",
            results.len(),
            result_word,
            elapsed_ms,
            files_searched,
            file_word,
        ));

        output
    }

    /// Format a single search result entry.
    ///
    /// # Arguments
    ///
    /// * `output` - The string buffer to append to
    /// * `result` - The search result to format
    fn format_result(&self, output: &mut String, result: &SearchResult) {
        // Header line: type name (file:start-end)
        if self.use_colors {
            // Colored output for TTY
            let name_colored = result.name.cyan();
            let file_colored = result.file.green();
            let lines_colored = format!("{}-{}", result.lines.0, result.lines.1).yellow();
            output.push_str(&format!(
                "{} {} ({}:{})\n",
                result.symbol_type, name_colored, file_colored, lines_colored
            ));
        } else {
            // Plain output for pipes
            output.push_str(&format!(
                "{} {} ({}:{}-{})\n",
                result.symbol_type, result.name, result.file, result.lines.0, result.lines.1
            ));
        }

        // Signature line (indented 2 spaces)
        if !result.signature.is_empty() {
            output.push_str(&format!("{}{}\n", SIGNATURE_INDENT, result.signature));
        }

        // Body lines (indented 4 spaces, only in full mode)
        if !self.signatures_only {
            if let Some(ref body) = result.body {
                if !body.is_empty() {
                    for line in body.lines() {
                        output.push_str(&format!("{}{}\n", BODY_INDENT, line));
                    }
                }
            }
        }
    }
}
