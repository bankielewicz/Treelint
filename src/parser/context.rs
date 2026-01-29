//! Context extraction modes for symbol search output control.
//!
//! This module provides functionality to control how much context is returned
//! with symbol search results. It supports three modes:
//!
//! - **Lines(n)**: Include N lines before and after the symbol's line range
//! - **Full**: Include the complete semantic unit from tree-sitter node boundaries
//! - **Signatures**: Include only the symbol's signature, omitting the body
//!
//! # Example
//!
//! ```
//! use treelint::parser::context::ContextMode;
//!
//! // Parse from CLI argument
//! let mode = ContextMode::from_cli_value("5").unwrap();
//! assert!(matches!(mode, ContextMode::Lines(5)));
//!
//! let mode = ContextMode::from_cli_value("full").unwrap();
//! assert!(matches!(mode, ContextMode::Full));
//! ```

use serde::{Deserialize, Serialize};

use crate::TreelintError;

/// Represents the context extraction mode for symbol search results.
///
/// This enum defines how much context is included with each symbol in search output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextMode {
    /// Include N lines before and after the symbol's line range.
    /// The value must be a positive integer (> 0).
    Lines(usize),
    /// Include the complete semantic unit from tree-sitter node boundaries.
    /// This includes decorators, docstrings, and the full body.
    Full,
    /// Include only the symbol's signature, omitting the body.
    /// The body field will be set to None in the output.
    Signatures,
}

impl Default for ContextMode {
    /// Default context mode is Full (complete semantic unit).
    fn default() -> Self {
        ContextMode::Full
    }
}

impl ContextMode {
    /// Parse a ContextMode from a CLI argument value.
    ///
    /// Accepts:
    /// - A positive integer string (e.g., "5") → `Lines(5)`
    /// - The string "full" (case-insensitive) → `Full`
    ///
    /// # Arguments
    ///
    /// * `value` - The CLI argument value to parse
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `ContextMode` or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The value is "0" (zero is not allowed per BR-002)
    /// - The value is a negative number
    /// - The value is not a valid integer or "full"
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::parser::context::ContextMode;
    ///
    /// assert!(matches!(ContextMode::from_cli_value("5"), Ok(ContextMode::Lines(5))));
    /// assert!(matches!(ContextMode::from_cli_value("full"), Ok(ContextMode::Full)));
    /// assert!(matches!(ContextMode::from_cli_value("FULL"), Ok(ContextMode::Full)));
    /// assert!(ContextMode::from_cli_value("0").is_err());
    /// assert!(ContextMode::from_cli_value("-1").is_err());
    /// ```
    pub fn from_cli_value(value: &str) -> Result<Self, TreelintError> {
        let trimmed = value.trim();

        // Check for "full" keyword (case-insensitive)
        if trimmed.eq_ignore_ascii_case("full") {
            return Ok(ContextMode::Full);
        }

        // Try to parse as integer
        match trimmed.parse::<i64>() {
            Ok(n) if n <= 0 => Err(TreelintError::Cli(
                "Context lines must be a positive integer (greater than 0)".to_string(),
            )),
            Ok(n) => Ok(ContextMode::Lines(n as usize)),
            Err(_) => Err(TreelintError::Cli(format!(
                "Invalid value for --context: '{}'. Expected a positive integer or 'full'",
                value
            ))),
        }
    }

    /// Convert the ContextMode to a JSON-compatible string for output.
    ///
    /// Returns:
    /// - `"lines:N"` for `Lines(N)` mode
    /// - `"full"` for `Full` mode
    /// - `"signatures"` for `Signatures` mode
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::parser::context::ContextMode;
    ///
    /// assert_eq!(ContextMode::Lines(5).to_json_string(), "lines:5");
    /// assert_eq!(ContextMode::Full.to_json_string(), "full");
    /// assert_eq!(ContextMode::Signatures.to_json_string(), "signatures");
    /// ```
    pub fn to_json_string(&self) -> String {
        match self {
            ContextMode::Lines(n) => format!("lines:{}", n),
            ContextMode::Full => "full".to_string(),
            ContextMode::Signatures => "signatures".to_string(),
        }
    }

    /// Check if this mode is the signatures-only mode.
    pub fn is_signatures(&self) -> bool {
        matches!(self, ContextMode::Signatures)
    }

    /// Check if this mode is the full semantic mode.
    pub fn is_full(&self) -> bool {
        matches!(self, ContextMode::Full)
    }

    /// Check if this mode is the lines-based mode.
    pub fn is_lines(&self) -> bool {
        matches!(self, ContextMode::Lines(_))
    }

    /// Get the number of context lines if this is a Lines mode.
    pub fn lines_count(&self) -> Option<usize> {
        match self {
            ContextMode::Lines(n) => Some(*n),
            _ => None,
        }
    }
}

/// Extract context lines from source code around a symbol.
///
/// Given a symbol's line range and the source content, extracts N lines
/// before the symbol's start line and N lines after the symbol's end line,
/// plus the symbol itself.
///
/// # Arguments
///
/// * `source` - The full source code content
/// * `line_start` - The symbol's starting line (1-indexed)
/// * `line_end` - The symbol's ending line (1-indexed)
/// * `context_lines` - Number of context lines to include before/after
///
/// # Returns
///
/// The extracted text with context lines, or None if the source is empty
/// or the line range is invalid.
///
/// # Example
///
/// ```
/// use treelint::parser::context::extract_lines_context;
///
/// let source = "line 1\nline 2\nline 3\nline 4\nline 5";
/// let result = extract_lines_context(source, 3, 3, 1);
/// assert_eq!(result, Some("line 2\nline 3\nline 4".to_string()));
/// ```
pub fn extract_lines_context(
    source: &str,
    line_start: usize,
    line_end: usize,
    context_lines: usize,
) -> Option<String> {
    if source.is_empty() {
        return None;
    }

    let lines: Vec<&str> = source.lines().collect();
    let total_lines = lines.len();

    if total_lines == 0 || line_start == 0 || line_end == 0 {
        return None;
    }

    // Convert to 0-indexed and clamp to valid range
    let start_idx = line_start.saturating_sub(1);
    let end_idx = (line_end.saturating_sub(1)).min(total_lines.saturating_sub(1));

    if start_idx > end_idx || start_idx >= total_lines {
        return None;
    }

    // Calculate context range with clamping at file boundaries
    let context_start = start_idx.saturating_sub(context_lines);
    let context_end = (end_idx + context_lines).min(total_lines.saturating_sub(1));

    // Extract the lines in the range (inclusive)
    let extracted: Vec<&str> = lines[context_start..=context_end].to_vec();

    if extracted.is_empty() {
        None
    } else {
        Some(extracted.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_mode_default() {
        assert!(matches!(ContextMode::default(), ContextMode::Full));
    }

    #[test]
    fn test_from_cli_value_positive_integer() {
        assert!(matches!(
            ContextMode::from_cli_value("5"),
            Ok(ContextMode::Lines(5))
        ));
        assert!(matches!(
            ContextMode::from_cli_value("1"),
            Ok(ContextMode::Lines(1))
        ));
        assert!(matches!(
            ContextMode::from_cli_value("100"),
            Ok(ContextMode::Lines(100))
        ));
    }

    #[test]
    fn test_from_cli_value_full() {
        assert!(matches!(
            ContextMode::from_cli_value("full"),
            Ok(ContextMode::Full)
        ));
        assert!(matches!(
            ContextMode::from_cli_value("FULL"),
            Ok(ContextMode::Full)
        ));
        assert!(matches!(
            ContextMode::from_cli_value("Full"),
            Ok(ContextMode::Full)
        ));
    }

    #[test]
    fn test_from_cli_value_zero_rejected() {
        assert!(ContextMode::from_cli_value("0").is_err());
    }

    #[test]
    fn test_from_cli_value_negative_rejected() {
        assert!(ContextMode::from_cli_value("-1").is_err());
        assert!(ContextMode::from_cli_value("-5").is_err());
    }

    #[test]
    fn test_from_cli_value_invalid() {
        assert!(ContextMode::from_cli_value("abc").is_err());
        assert!(ContextMode::from_cli_value("").is_err());
    }

    #[test]
    fn test_to_json_string() {
        assert_eq!(ContextMode::Lines(5).to_json_string(), "lines:5");
        assert_eq!(ContextMode::Lines(1).to_json_string(), "lines:1");
        assert_eq!(ContextMode::Full.to_json_string(), "full");
        assert_eq!(ContextMode::Signatures.to_json_string(), "signatures");
    }

    #[test]
    fn test_extract_lines_context_basic() {
        let source = "line 1\nline 2\nline 3\nline 4\nline 5";
        let result = extract_lines_context(source, 3, 3, 1);
        assert_eq!(result, Some("line 2\nline 3\nline 4".to_string()));
    }

    #[test]
    fn test_extract_lines_context_clamps_at_start() {
        let source = "line 1\nline 2\nline 3\nline 4\nline 5";
        // Line 1 with 3 lines context - should start at line 1
        let result = extract_lines_context(source, 1, 1, 3);
        assert_eq!(result, Some("line 1\nline 2\nline 3\nline 4".to_string()));
    }

    #[test]
    fn test_extract_lines_context_clamps_at_end() {
        let source = "line 1\nline 2\nline 3\nline 4\nline 5";
        // Line 5 with 3 lines context - should end at line 5
        let result = extract_lines_context(source, 5, 5, 3);
        assert_eq!(result, Some("line 2\nline 3\nline 4\nline 5".to_string()));
    }

    #[test]
    fn test_extract_lines_context_multi_line_symbol() {
        let source = "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7";
        // Symbol spans lines 3-5 with 1 line context
        let result = extract_lines_context(source, 3, 5, 1);
        assert_eq!(
            result,
            Some("line 2\nline 3\nline 4\nline 5\nline 6".to_string())
        );
    }

    #[test]
    fn test_extract_lines_context_empty_source() {
        assert_eq!(extract_lines_context("", 1, 1, 1), None);
    }
}
