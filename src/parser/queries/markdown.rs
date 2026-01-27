//! Markdown tree-sitter queries for symbol extraction.
//!
//! This module contains query patterns for extracting symbols
//! from Markdown documents using tree-sitter.
//!
//! # Supported Node Types
//!
//! - `atx_heading` - ATX-style headings (# Heading)
//! - `setext_heading` - Setext-style headings (Heading\n===)
//!
//! Markdown headings are extracted as symbols to support
//! repository map generation and document structure analysis.

/// Query string for extracting Markdown ATX headings.
///
/// Captures headings at all levels (h1-h6).
///
/// # Example Matched Code
///
/// ```text
/// # Main Title
/// ## Getting Started
/// ### Installation
/// #### Prerequisites
/// ##### Notes
/// ###### Footnotes
/// ```
pub const ATX_HEADING_QUERY: &str = r#"
(atx_heading) @heading.atx
"#;

/// Query string for extracting Markdown Setext headings.
///
/// Captures h1 and h2 setext-style headings.
///
/// # Example Matched Code
///
/// ```text
/// Main Title
/// ===========
///
/// Subtitle
/// --------
/// ```
pub const SETEXT_HEADING_QUERY: &str = r#"
(setext_heading) @heading.setext
"#;

/// Combined query for all Markdown headings.
///
/// This query captures both ATX and Setext style headings,
/// making it easier to extract all headings from a document.
pub const HEADING_QUERY: &str = r#"
(atx_heading) @heading

(setext_heading) @heading
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atx_heading_query_is_valid() {
        let language = tree_sitter_md::language();
        let query = tree_sitter::Query::new(&language, ATX_HEADING_QUERY);
        assert!(
            query.is_ok(),
            "ATX heading query should be valid: {:?}",
            query.err()
        );
    }

    #[test]
    fn test_setext_heading_query_is_valid() {
        let language = tree_sitter_md::language();
        let query = tree_sitter::Query::new(&language, SETEXT_HEADING_QUERY);
        assert!(
            query.is_ok(),
            "Setext heading query should be valid: {:?}",
            query.err()
        );
    }

    #[test]
    fn test_combined_heading_query_is_valid() {
        let language = tree_sitter_md::language();
        let query = tree_sitter::Query::new(&language, HEADING_QUERY);
        assert!(
            query.is_ok(),
            "Combined heading query should be valid: {:?}",
            query.err()
        );
    }
}
