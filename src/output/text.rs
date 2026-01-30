//! Human-readable text output formatting for search results.
//!
//! This module provides the [`TextFormatter`] struct for rendering
//! search results as a tree-style text layout for terminal display.
//! Output includes a header with symbol metadata, indented signature
//! and body, and a summary line with statistics. Colors are applied
//! when output is to a TTY terminal.

use colored::Colorize;

use super::json::{MapOutput, SearchResult};

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

// ============================================================================
// Map Text Formatting
// ============================================================================

/// Format the map output as human-readable text.
///
/// Produces a tree-style layout with:
/// - Header line with total counts
/// - Files grouped by directory
/// - Symbols indented under files with type and line range
/// - Relevance stars shown when ranked
///
/// # Arguments
///
/// * `map` - The map output structure
/// * `show_relevance` - Whether to show relevance scores with stars
///
/// # Returns
///
/// A formatted string containing the repository map.
pub fn format_map_text(map: &MapOutput, show_relevance: bool) -> String {
    let mut output = String::new();

    // Header with counts
    output.push_str(&format!(
        "Repository Map: {} symbols in {} files\n",
        map.total_symbols, map.total_files
    ));
    output.push_str(&format!("{}\n", "=".repeat(50)));

    // Group files by directory for tree structure
    let mut dirs: std::collections::BTreeMap<String, Vec<(&String, &super::json::FileSymbols)>> =
        std::collections::BTreeMap::new();

    for (path, file_symbols) in &map.by_file {
        let dir = std::path::Path::new(path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());
        dirs.entry(dir).or_default().push((path, file_symbols));
    }

    // Output tree structure
    for (dir, files) in &dirs {
        output.push_str(&format!("\n{}/\n", dir));

        for (path, file_symbols) in files {
            // Extract filename
            let filename = std::path::Path::new(path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string());

            output.push_str(&format!("  {}\n", filename));

            // Output symbols indented
            for symbol in &file_symbols.symbols {
                let line_range = if symbol.lines.len() >= 2 {
                    format!("L{}:{}", symbol.lines[0], symbol.lines[1])
                } else if !symbol.lines.is_empty() {
                    format!("L{}", symbol.lines[0])
                } else {
                    "L?".to_string()
                };

                if show_relevance {
                    if let Some(relevance) = symbol.relevance {
                        output.push_str(&format!(
                            "    {} {} [{}] * {:.2}\n",
                            symbol.symbol_type, symbol.name, line_range, relevance
                        ));
                    } else {
                        output.push_str(&format!(
                            "    {} {} [{}]\n",
                            symbol.symbol_type, symbol.name, line_range
                        ));
                    }
                } else {
                    output.push_str(&format!(
                        "    {} {} [{}]\n",
                        symbol.symbol_type, symbol.name, line_range
                    ));
                }
            }
        }
    }

    // Summary by type
    output.push_str(&format!("\n{}\n", "=".repeat(50)));
    output.push_str("By Type:\n");
    for (type_name, count) in &map.by_type {
        output.push_str(&format!("  {}: {}\n", type_name, count));
    }

    output
}
