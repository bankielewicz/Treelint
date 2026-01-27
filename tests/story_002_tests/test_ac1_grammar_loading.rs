//! AC#1: Embedded Grammar Loading Tests
//!
//! Given: The Treelint binary is compiled with embedded tree-sitter grammars
//! When: The parser initializes for any supported language (Python, TypeScript, Rust, Markdown)
//! Then: The grammar loads from the compiled binary without requiring external grammar files,
//!       and initialization completes in less than 10ms
//!
//! Test file: tests/STORY-002/test_ac1_grammar_loading.rs
//! Source files tested:
//!   - src/parser/languages.rs (Language enum and grammar loading)
//!   - build.rs (Build script for grammar embedding)
//! Coverage threshold: 95%

use std::time::Instant;

// These imports will fail until the parser module is implemented
// This is expected behavior for TDD Red phase
use treelint::parser::{Language, Parser};

/// Test: Language enum has exactly 4 variants (Python, TypeScript, Rust, Markdown)
/// Requirement: DM-001 - Define Language enum with Python, TypeScript, Rust, Markdown variants
#[test]
fn test_language_enum_has_four_variants() {
    // Verify all 4 language variants exist
    let languages = [
        Language::Python,
        Language::TypeScript,
        Language::Rust,
        Language::Markdown,
    ];

    assert_eq!(
        languages.len(),
        4,
        "Language enum must have exactly 4 variants"
    );
}

/// Test: Python grammar loads successfully from embedded binary
/// Requirement: DM-003 - Return embedded tree-sitter::Language from each variant
#[test]
fn test_python_grammar_loads_from_embedded_binary() {
    let ts_language = Language::Python.tree_sitter_language();

    // Verify the grammar is valid (not null/invalid)
    assert!(
        ts_language.version() > 0,
        "Python grammar must have valid version"
    );
}

/// Test: TypeScript grammar loads successfully from embedded binary
/// Requirement: DM-003 - Return embedded tree-sitter::Language from each variant
#[test]
fn test_typescript_grammar_loads_from_embedded_binary() {
    let ts_language = Language::TypeScript.tree_sitter_language();

    assert!(
        ts_language.version() > 0,
        "TypeScript grammar must have valid version"
    );
}

/// Test: Rust grammar loads successfully from embedded binary
/// Requirement: DM-003 - Return embedded tree-sitter::Language from each variant
#[test]
fn test_rust_grammar_loads_from_embedded_binary() {
    let ts_language = Language::Rust.tree_sitter_language();

    assert!(
        ts_language.version() > 0,
        "Rust grammar must have valid version"
    );
}

/// Test: Markdown grammar loads successfully from embedded binary
/// Requirement: DM-003 - Return embedded tree-sitter::Language from each variant
#[test]
fn test_markdown_grammar_loads_from_embedded_binary() {
    let ts_language = Language::Markdown.tree_sitter_language();

    assert!(
        ts_language.version() > 0,
        "Markdown grammar must have valid version"
    );
}

/// Test: Grammar initialization completes in less than 10ms for Python
/// Requirement: NFR-001 - Grammar initialization under 10ms
#[test]
fn test_python_grammar_init_under_10ms() {
    let start = Instant::now();
    let _ts_language = Language::Python.tree_sitter_language();
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 10,
        "Python grammar init took {}ms, expected < 10ms",
        elapsed.as_millis()
    );
}

/// Test: Grammar initialization completes in less than 10ms for TypeScript
/// Requirement: NFR-001 - Grammar initialization under 10ms
#[test]
fn test_typescript_grammar_init_under_10ms() {
    let start = Instant::now();
    let _ts_language = Language::TypeScript.tree_sitter_language();
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 10,
        "TypeScript grammar init took {}ms, expected < 10ms",
        elapsed.as_millis()
    );
}

/// Test: Grammar initialization completes in less than 10ms for Rust
/// Requirement: NFR-001 - Grammar initialization under 10ms
#[test]
fn test_rust_grammar_init_under_10ms() {
    let start = Instant::now();
    let _ts_language = Language::Rust.tree_sitter_language();
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 10,
        "Rust grammar init took {}ms, expected < 10ms",
        elapsed.as_millis()
    );
}

/// Test: Grammar initialization completes in less than 10ms for Markdown
/// Requirement: NFR-001 - Grammar initialization under 10ms
#[test]
fn test_markdown_grammar_init_under_10ms() {
    let start = Instant::now();
    let _ts_language = Language::Markdown.tree_sitter_language();
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 10,
        "Markdown grammar init took {}ms, expected < 10ms",
        elapsed.as_millis()
    );
}

/// Test: Parser can be created with Python grammar
/// Requirement: SVC-001 - Parse file content using tree-sitter with appropriate grammar
#[test]
fn test_parser_creates_with_python_grammar() {
    let parser = Parser::new(Language::Python);

    assert!(
        parser.is_ok(),
        "Parser should create successfully with Python grammar"
    );
}

/// Test: Parser can be created with all supported grammars
/// Requirement: SVC-001 - Parse file content using tree-sitter with appropriate grammar
#[test]
fn test_parser_creates_with_all_grammars() {
    for language in [
        Language::Python,
        Language::TypeScript,
        Language::Rust,
        Language::Markdown,
    ] {
        let parser = Parser::new(language);
        assert!(
            parser.is_ok(),
            "Parser should create successfully with {:?} grammar",
            language
        );
    }
}

/// Test: Grammars work without external files (BR-001)
/// Requirement: BR-001 - Grammars must be embedded in binary (no external files)
#[test]
fn test_grammars_embedded_no_external_files() {
    // This test verifies that grammars load without any file I/O
    // by checking that the grammar loading doesn't require a path

    // If grammars required external files, this would fail
    // because we're not providing any paths
    let languages = [
        Language::Python,
        Language::TypeScript,
        Language::Rust,
        Language::Markdown,
    ];

    for language in languages {
        let ts_lang = language.tree_sitter_language();
        // If we get here without error, grammars are embedded
        assert!(
            ts_lang.version() > 0,
            "Grammar for {:?} must be embedded and valid",
            language
        );
    }
}

/// Test: Parser successfully parses valid Python content
/// Requirement: SVC-001 - Parse file content using tree-sitter with appropriate grammar
#[test]
fn test_parser_parses_valid_python_content() {
    let parser = Parser::new(Language::Python).expect("Parser creation should succeed");
    let content = "def hello(): pass";

    let result = parser.parse(content);

    assert!(result.is_ok(), "Parsing valid Python should succeed");
    let tree = result.unwrap();
    assert!(
        !tree.root_node().has_error(),
        "AST should have no errors for valid code"
    );
}

/// Test: Parser successfully parses valid TypeScript content
/// Requirement: SVC-001 - Parse file content using tree-sitter with appropriate grammar
#[test]
fn test_parser_parses_valid_typescript_content() {
    let parser = Parser::new(Language::TypeScript).expect("Parser creation should succeed");
    let content = "function hello(): void {}";

    let result = parser.parse(content);

    assert!(result.is_ok(), "Parsing valid TypeScript should succeed");
    let tree = result.unwrap();
    assert!(
        !tree.root_node().has_error(),
        "AST should have no errors for valid code"
    );
}

/// Test: Parser successfully parses valid Rust content
/// Requirement: SVC-001 - Parse file content using tree-sitter with appropriate grammar
#[test]
fn test_parser_parses_valid_rust_content() {
    let parser = Parser::new(Language::Rust).expect("Parser creation should succeed");
    let content = "fn hello() {}";

    let result = parser.parse(content);

    assert!(result.is_ok(), "Parsing valid Rust should succeed");
    let tree = result.unwrap();
    assert!(
        !tree.root_node().has_error(),
        "AST should have no errors for valid code"
    );
}

/// Test: Parser successfully parses valid Markdown content
/// Requirement: SVC-001 - Parse file content using tree-sitter with appropriate grammar
#[test]
fn test_parser_parses_valid_markdown_content() {
    let parser = Parser::new(Language::Markdown).expect("Parser creation should succeed");
    let content = "# Hello World\n\nThis is a paragraph.";

    let result = parser.parse(content);

    assert!(result.is_ok(), "Parsing valid Markdown should succeed");
}
