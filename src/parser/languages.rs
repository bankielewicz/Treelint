//! Language detection and grammar loading for tree-sitter.
//!
//! This module provides the [`Language`] enum for representing supported
//! programming languages and methods for detecting language from file
//! extensions and loading the appropriate tree-sitter grammar.

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Supported programming languages for symbol extraction.
///
/// Each variant corresponds to a tree-sitter grammar that is embedded
/// in the binary at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    /// Python programming language (.py files)
    Python,
    /// TypeScript and JavaScript (.ts, .tsx, .js, .jsx files)
    TypeScript,
    /// Rust programming language (.rs files)
    Rust,
    /// Markdown documents (.md, .markdown files)
    Markdown,
}

impl Language {
    /// Detect language from a file extension.
    ///
    /// The extension should include the leading dot (e.g., ".py").
    /// Extension matching is case-insensitive.
    ///
    /// # Arguments
    ///
    /// * `extension` - The file extension including the leading dot
    ///
    /// # Returns
    ///
    /// `Some(Language)` if the extension is recognized, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::parser::Language;
    ///
    /// assert_eq!(Language::from_extension(".py"), Some(Language::Python));
    /// assert_eq!(Language::from_extension(".ts"), Some(Language::TypeScript));
    /// assert_eq!(Language::from_extension(".unknown"), None);
    /// ```
    pub fn from_extension(extension: &str) -> Option<Self> {
        // Require leading dot for proper extension format
        if !extension.starts_with('.') {
            return None;
        }

        match extension.to_lowercase().as_str() {
            ".py" => Some(Language::Python),
            ".ts" | ".tsx" | ".js" | ".jsx" => Some(Language::TypeScript),
            ".rs" => Some(Language::Rust),
            ".md" | ".markdown" => Some(Language::Markdown),
            _ => None,
        }
    }

    /// Detect language from a file path.
    ///
    /// Extracts the file extension from the path and uses [`from_extension`]
    /// to determine the language.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the source file
    ///
    /// # Returns
    ///
    /// `Some(Language)` if the file extension is recognized, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::Path;
    /// use treelint::parser::Language;
    ///
    /// let path = Path::new("src/main.py");
    /// assert_eq!(Language::from_path(path), Some(Language::Python));
    /// ```
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| Self::from_extension(&format!(".{}", ext)))
    }

    /// Get the tree-sitter Language for this language.
    ///
    /// This returns the embedded tree-sitter grammar, which is compiled
    /// into the binary and requires no external files.
    ///
    /// # Returns
    ///
    /// The tree-sitter [`tree_sitter::Language`] for parsing.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::parser::Language;
    ///
    /// let ts_lang = Language::Python.tree_sitter_language();
    /// assert!(ts_lang.version() > 0);
    /// ```
    pub fn tree_sitter_language(&self) -> tree_sitter::Language {
        match self {
            Language::Python => tree_sitter_python::language(),
            Language::TypeScript => tree_sitter_typescript::language_typescript(),
            Language::Rust => tree_sitter_rust::language(),
            Language::Markdown => tree_sitter_md::language(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_extension_python() {
        assert_eq!(Language::from_extension(".py"), Some(Language::Python));
    }

    #[test]
    fn test_from_extension_typescript() {
        assert_eq!(Language::from_extension(".ts"), Some(Language::TypeScript));
        assert_eq!(Language::from_extension(".tsx"), Some(Language::TypeScript));
    }

    #[test]
    fn test_from_extension_javascript_uses_typescript() {
        assert_eq!(Language::from_extension(".js"), Some(Language::TypeScript));
        assert_eq!(Language::from_extension(".jsx"), Some(Language::TypeScript));
    }

    #[test]
    fn test_from_extension_rust() {
        assert_eq!(Language::from_extension(".rs"), Some(Language::Rust));
    }

    #[test]
    fn test_from_extension_markdown() {
        assert_eq!(Language::from_extension(".md"), Some(Language::Markdown));
        assert_eq!(
            Language::from_extension(".markdown"),
            Some(Language::Markdown)
        );
    }

    #[test]
    fn test_from_extension_case_insensitive() {
        assert_eq!(Language::from_extension(".PY"), Some(Language::Python));
        assert_eq!(Language::from_extension(".Ts"), Some(Language::TypeScript));
        assert_eq!(Language::from_extension(".RS"), Some(Language::Rust));
    }

    #[test]
    fn test_from_extension_unknown() {
        assert_eq!(Language::from_extension(".unknown"), None);
        assert_eq!(Language::from_extension(".toml"), None);
    }

    #[test]
    fn test_from_extension_no_dot() {
        assert_eq!(Language::from_extension("py"), None);
    }

    #[test]
    fn test_from_extension_empty() {
        assert_eq!(Language::from_extension(""), None);
    }

    #[test]
    fn test_from_path() {
        assert_eq!(
            Language::from_path(Path::new("src/main.py")),
            Some(Language::Python)
        );
        assert_eq!(
            Language::from_path(Path::new("src/app.ts")),
            Some(Language::TypeScript)
        );
    }

    #[test]
    fn test_language_is_copy() {
        let original = Language::Python;
        let copied = original;
        let _also_original = original; // Should still work
        assert_eq!(copied, Language::Python);
    }
}
