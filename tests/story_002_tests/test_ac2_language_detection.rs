//! AC#2: Language Detection from File Extension Tests
//!
//! Given: A source file with a recognized extension
//! When: The parser determines the file's language
//! Then: The correct Language enum is returned:
//!       .py → Python
//!       .ts/.tsx → TypeScript
//!       .js/.jsx → TypeScript
//!       .rs → Rust
//!       .md/.markdown → Markdown
//!
//! Test file: tests/STORY-002/test_ac2_language_detection.rs
//! Source files tested:
//!   - src/parser/languages.rs (Extension to language mapping)
//! Coverage threshold: 95%

use std::path::Path;

// These imports will fail until the parser module is implemented
// This is expected behavior for TDD Red phase
use treelint::parser::Language;

/// Test: .py extension returns Language::Python
/// Requirement: DM-002 - Implement from_extension() for language detection
#[test]
fn test_py_extension_returns_python() {
    let result = Language::from_extension(".py");

    assert_eq!(result, Some(Language::Python));
}

/// Test: .ts extension returns Language::TypeScript
/// Requirement: DM-002 - Implement from_extension() for language detection
#[test]
fn test_ts_extension_returns_typescript() {
    let result = Language::from_extension(".ts");

    assert_eq!(result, Some(Language::TypeScript));
}

/// Test: .tsx extension returns Language::TypeScript
/// Requirement: DM-002 - Implement from_extension() for language detection
#[test]
fn test_tsx_extension_returns_typescript() {
    let result = Language::from_extension(".tsx");

    assert_eq!(result, Some(Language::TypeScript));
}

/// Test: .js extension returns Language::TypeScript (using TS grammar)
/// Requirement: DM-002 - Implement from_extension() for language detection
#[test]
fn test_js_extension_returns_typescript() {
    let result = Language::from_extension(".js");

    assert_eq!(result, Some(Language::TypeScript));
}

/// Test: .jsx extension returns Language::TypeScript (using TS grammar)
/// Requirement: DM-002 - Implement from_extension() for language detection
#[test]
fn test_jsx_extension_returns_typescript() {
    let result = Language::from_extension(".jsx");

    assert_eq!(result, Some(Language::TypeScript));
}

/// Test: .rs extension returns Language::Rust
/// Requirement: DM-002 - Implement from_extension() for language detection
#[test]
fn test_rs_extension_returns_rust() {
    let result = Language::from_extension(".rs");

    assert_eq!(result, Some(Language::Rust));
}

/// Test: .md extension returns Language::Markdown
/// Requirement: DM-002 - Implement from_extension() for language detection
#[test]
fn test_md_extension_returns_markdown() {
    let result = Language::from_extension(".md");

    assert_eq!(result, Some(Language::Markdown));
}

/// Test: .markdown extension returns Language::Markdown
/// Requirement: DM-002 - Implement from_extension() for language detection
#[test]
fn test_markdown_extension_returns_markdown() {
    let result = Language::from_extension(".markdown");

    assert_eq!(result, Some(Language::Markdown));
}

/// Test: Unknown extension returns None
/// Requirement: DM-002 - Implement from_extension() for language detection
#[test]
fn test_unknown_extension_returns_none() {
    let result = Language::from_extension(".unknown");

    assert_eq!(result, None);
}

/// Test: Empty extension returns None
/// Requirement: DM-002 - Implement from_extension() for language detection
#[test]
fn test_empty_extension_returns_none() {
    let result = Language::from_extension("");

    assert_eq!(result, None);
}

/// Test: Extension without dot returns None
/// Requirement: DM-002 - Implement from_extension() for language detection
#[test]
fn test_extension_without_dot_returns_none() {
    let result = Language::from_extension("py");

    // Should return None because proper extension format is ".py"
    assert_eq!(result, None);
}

/// Test: Case-insensitive extension matching for .PY
/// Requirement: DM-002 - from_extension should handle case variations
#[test]
fn test_extension_case_insensitive_py_uppercase() {
    let result = Language::from_extension(".PY");

    assert_eq!(result, Some(Language::Python));
}

/// Test: Case-insensitive extension matching for .Ts
/// Requirement: DM-002 - from_extension should handle case variations
#[test]
fn test_extension_case_insensitive_ts_mixed() {
    let result = Language::from_extension(".Ts");

    assert_eq!(result, Some(Language::TypeScript));
}

/// Test: Case-insensitive extension matching for .RS
/// Requirement: DM-002 - from_extension should handle case variations
#[test]
fn test_extension_case_insensitive_rs_uppercase() {
    let result = Language::from_extension(".RS");

    assert_eq!(result, Some(Language::Rust));
}

/// Test: Case-insensitive extension matching for .MD
/// Requirement: DM-002 - from_extension should handle case variations
#[test]
fn test_extension_case_insensitive_md_uppercase() {
    let result = Language::from_extension(".MD");

    assert_eq!(result, Some(Language::Markdown));
}

/// Test: Language detection from file path
/// Requirement: DM-002 - Implement from_path() for language detection
#[test]
fn test_from_path_python_file() {
    let path = Path::new("src/module/file.py");
    let result = Language::from_path(path);

    assert_eq!(result, Some(Language::Python));
}

/// Test: Language detection from TypeScript file path
/// Requirement: DM-002 - Implement from_path() for language detection
#[test]
fn test_from_path_typescript_file() {
    let path = Path::new("src/components/Button.tsx");
    let result = Language::from_path(path);

    assert_eq!(result, Some(Language::TypeScript));
}

/// Test: Language detection from Rust file path
/// Requirement: DM-002 - Implement from_path() for language detection
#[test]
fn test_from_path_rust_file() {
    let path = Path::new("src/parser/mod.rs");
    let result = Language::from_path(path);

    assert_eq!(result, Some(Language::Rust));
}

/// Test: Language detection from Markdown file path
/// Requirement: DM-002 - Implement from_path() for language detection
#[test]
fn test_from_path_markdown_file() {
    let path = Path::new("docs/README.md");
    let result = Language::from_path(path);

    assert_eq!(result, Some(Language::Markdown));
}

/// Test: Language detection returns None for unknown path
/// Requirement: DM-002 - Implement from_path() for language detection
#[test]
fn test_from_path_unknown_extension() {
    let path = Path::new("config/settings.toml");
    let result = Language::from_path(path);

    assert_eq!(result, None);
}

/// Test: Language detection returns None for path without extension
/// Requirement: DM-002 - Implement from_path() for language detection
#[test]
fn test_from_path_no_extension() {
    let path = Path::new("Makefile");
    let result = Language::from_path(path);

    assert_eq!(result, None);
}

/// Test: All supported extensions are covered
/// Requirement: DM-002 - All AC#2 extensions must be supported
#[test]
fn test_all_supported_extensions() {
    let extensions = vec![
        (".py", Language::Python),
        (".ts", Language::TypeScript),
        (".tsx", Language::TypeScript),
        (".js", Language::TypeScript),
        (".jsx", Language::TypeScript),
        (".rs", Language::Rust),
        (".md", Language::Markdown),
        (".markdown", Language::Markdown),
    ];

    for (ext, expected) in extensions {
        let result = Language::from_extension(ext);
        assert_eq!(
            result,
            Some(expected),
            "Extension '{}' should map to {:?}",
            ext,
            expected
        );
    }
}

/// Test: Language enum implements PartialEq
/// Requirement: DM-001 - Language enum should be comparable
#[test]
fn test_language_partial_eq() {
    assert_eq!(Language::Python, Language::Python);
    assert_ne!(Language::Python, Language::TypeScript);
}

/// Test: Language enum implements Clone
/// Requirement: DM-001 - Language enum should be cloneable
#[test]
fn test_language_clone() {
    let original = Language::Python;
    let cloned = original.clone();

    assert_eq!(original, cloned);
}

/// Test: Language enum implements Debug
/// Requirement: DM-001 - Language enum should be debuggable
#[test]
fn test_language_debug() {
    let debug_str = format!("{:?}", Language::Python);

    assert!(debug_str.contains("Python"));
}

/// Test: Language enum implements Copy (if applicable)
/// Requirement: DM-001 - Language enum should be copyable (small type)
#[test]
fn test_language_copy() {
    let original = Language::Python;
    let copied = original; // Copy, not move
    let _also_original = original; // Should still be valid

    assert_eq!(copied, Language::Python);
}
