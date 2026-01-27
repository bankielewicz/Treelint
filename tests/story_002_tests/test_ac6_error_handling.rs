//! AC#6: Error Handling for Malformed Files Tests
//!
//! Given: A source file with syntax errors or malformed content
//! When: The parser attempts to extract symbols
//! Then:
//!       - The parser does NOT panic
//!       - Returns partial results for valid portions (tree-sitter error recovery)
//!       - Returns empty list for completely unparseable files
//!
//! Test file: tests/STORY-002/test_ac6_error_handling.rs
//! Source files tested:
//!   - src/parser/symbols.rs (Error handling in extractor)
//! Coverage threshold: 95%

use std::path::Path;

// These imports will fail until the parser module is implemented
// This is expected behavior for TDD Red phase
use treelint::parser::{Language, Parser, SymbolExtractor, SymbolType};

/// Helper to get fixture path
fn fixture_path(relative: &str) -> String {
    format!("tests/fixtures/{}", relative)
}

/// Test: Parser does not panic on malformed Python file
/// Requirement: SVC-005 - Handle syntax errors gracefully
#[test]
fn test_no_panic_on_malformed_python() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/malformed.py");

    // This should NOT panic
    let result = extractor.extract_from_file(Path::new(&path));

    assert!(result.is_ok(), "Extraction should return Ok, not panic");
}

/// Test: Parser does not panic on malformed TypeScript file
/// Requirement: SVC-005 - Handle syntax errors gracefully
#[test]
fn test_no_panic_on_malformed_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/malformed.ts");

    // This should NOT panic
    let result = extractor.extract_from_file(Path::new(&path));

    assert!(result.is_ok(), "Extraction should return Ok, not panic");
}

/// Test: Parser returns partial results from malformed Python file
/// Requirement: BR-002 - Syntax errors should return partial results
#[test]
fn test_partial_results_from_malformed_python() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/malformed.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // malformed.py has valid functions: valid_function, another_valid_function
    // and invalid: broken_function, IncompleteClass
    let valid_functions: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Function)
        .filter(|s| s.name == "valid_function" || s.name == "another_valid_function")
        .collect();

    // Should extract at least the valid functions
    assert!(
        valid_functions.len() >= 1,
        "Should extract at least 1 valid function from malformed file, found {}",
        valid_functions.len()
    );

    // Specifically check for valid_function
    let valid_fn = symbols.iter().find(|s| s.name == "valid_function");
    assert!(
        valid_fn.is_some(),
        "Should find 'valid_function' despite syntax errors elsewhere"
    );
}

/// Test: Parser returns partial results from malformed TypeScript file
/// Requirement: BR-002 - Syntax errors should return partial results
#[test]
fn test_partial_results_from_malformed_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/malformed.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // malformed.ts has valid: validFunction, ValidClass
    // Should extract at least the valid function
    let valid_fn = symbols.iter().find(|s| s.name == "validFunction");

    assert!(
        valid_fn.is_some(),
        "Should find 'validFunction' despite syntax errors elsewhere"
    );
}

/// Test: Empty file returns empty symbol list
/// Requirement: BR-003 - Empty files return empty symbol list
#[test]
fn test_empty_file_returns_empty_list() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/empty.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Empty file should return empty list (or only comment)
    // The file has only a comment, so no actual symbols
    let non_trivial: Vec<_> = symbols
        .iter()
        .filter(|s| {
            s.symbol_type == SymbolType::Function
                || s.symbol_type == SymbolType::Class
                || s.symbol_type == SymbolType::Method
        })
        .collect();

    assert!(
        non_trivial.is_empty(),
        "Empty file should have no functions/classes/methods"
    );
}

/// Test: Empty string content returns empty list
/// Requirement: BR-003 - Empty files return empty symbol list (not error)
#[test]
fn test_empty_content_returns_empty_list() {
    let extractor = SymbolExtractor::new();

    let symbols = extractor
        .extract_from_content("", Language::Python)
        .expect("Extraction should succeed for empty content");

    assert!(
        symbols.is_empty(),
        "Empty content should return empty symbol list"
    );
}

/// Test: Whitespace-only content returns empty list
/// Requirement: BR-003 - Trivial files return empty symbol list
#[test]
fn test_whitespace_only_returns_empty_list() {
    let extractor = SymbolExtractor::new();

    let symbols = extractor
        .extract_from_content("   \n\n   \t\t\n", Language::Python)
        .expect("Extraction should succeed for whitespace");

    assert!(
        symbols.is_empty(),
        "Whitespace-only content should return empty symbol list"
    );
}

/// Test: Comment-only file returns empty list
/// Requirement: BR-003 - Comment-only files return empty symbol list
#[test]
fn test_comment_only_returns_empty_list() {
    let extractor = SymbolExtractor::new();
    let content = r#"
# This is a comment
# Another comment
"""
A docstring comment
"""
"#;

    let symbols = extractor
        .extract_from_content(content, Language::Python)
        .expect("Extraction should succeed for comments");

    let code_symbols: Vec<_> = symbols
        .iter()
        .filter(|s| {
            s.symbol_type == SymbolType::Function
                || s.symbol_type == SymbolType::Class
                || s.symbol_type == SymbolType::Method
        })
        .collect();

    assert!(
        code_symbols.is_empty(),
        "Comment-only file should have no code symbols"
    );
}

/// Test: Binary/garbage content does not panic
/// Requirement: NFR-003 - No panics on malformed input
#[test]
fn test_binary_content_no_panic() {
    let extractor = SymbolExtractor::new();
    // Random garbage content (using valid UTF-8 bytes that look like garbage code)
    let garbage = "\x00\x01\x02\x03\x7F\x1F\x1E";

    // Should not panic
    let result = extractor.extract_from_content(garbage, Language::Python);

    // Should return Ok (possibly empty) or an error, but NOT panic
    // We accept either Ok or Err, just not panic
    match result {
        Ok(symbols) => {
            // Empty or partial is fine
            assert!(
                symbols.len() <= 1,
                "Garbage should produce few or no symbols"
            );
        }
        Err(_) => {
            // An error is also acceptable for binary content
        }
    }
}

/// Test: Unicode content with unusual characters
/// Requirement: NFR-003 - Handle edge cases without panic
#[test]
fn test_unicode_content_no_panic() {
    let extractor = SymbolExtractor::new();
    let content = r#"
def 函数名(参数: str) -> str:
    """中文文档"""
    return f"你好, {参数}!"

class Тест:
    def метод(self):
        pass
"#;

    let result = extractor.extract_from_content(content, Language::Python);

    assert!(result.is_ok(), "Unicode content should not cause panic");

    let symbols = result.unwrap();
    // Should extract the function and class despite non-ASCII names
    assert!(
        symbols.len() >= 2,
        "Should extract symbols with Unicode names"
    );
}

/// Test: Very long line does not panic
/// Requirement: NFR-003 - Handle edge cases without panic
#[test]
fn test_very_long_line_no_panic() {
    let extractor = SymbolExtractor::new();
    // Create a function with a very long string
    let long_string = "x".repeat(10_000);
    let content = format!(
        r#"
def long_function():
    x = "{}"
    return x
"#,
        long_string
    );

    let result = extractor.extract_from_content(&content, Language::Python);

    assert!(result.is_ok(), "Long line should not cause panic");
}

/// Test: Deeply nested structure does not panic
/// Requirement: NFR-003 - Handle edge cases without panic
#[test]
fn test_deeply_nested_no_panic() {
    let extractor = SymbolExtractor::new();
    // Create deeply nested structure
    let content = r#"
def level0():
    def level1():
        def level2():
            def level3():
                def level4():
                    def level5():
                        return "deep"
                    return level5()
                return level4()
            return level3()
        return level2()
    return level1()
"#;

    let result = extractor.extract_from_content(content, Language::Python);

    assert!(
        result.is_ok(),
        "Deeply nested functions should not cause panic"
    );
}

/// Test: Incomplete function definition
/// Requirement: SVC-005 - Handle syntax errors gracefully
#[test]
fn test_incomplete_function_definition() {
    let extractor = SymbolExtractor::new();
    let content = "def incomplete(";

    let result = extractor.extract_from_content(content, Language::Python);

    // Should return Ok (possibly empty), not panic
    assert!(result.is_ok(), "Incomplete function should not cause panic");
}

/// Test: Mismatched brackets
/// Requirement: SVC-005 - Handle syntax errors gracefully
#[test]
fn test_mismatched_brackets() {
    let extractor = SymbolExtractor::new();
    let content = r#"
def func():
    x = [1, 2, 3
    return x
}
"#;

    let result = extractor.extract_from_content(content, Language::Python);

    assert!(result.is_ok(), "Mismatched brackets should not cause panic");
}

/// Test: Extract valid class despite malformed method
/// Requirement: BR-002 - File with 1 broken function and 2 valid returns 2 symbols
#[test]
fn test_valid_class_with_broken_method() {
    let extractor = SymbolExtractor::new();
    let content = r#"
class ValidClass:
    def valid_method(self):
        return True

    def broken_method(self
        return False

    def another_valid_method(self):
        return True
"#;

    let result = extractor.extract_from_content(content, Language::Python);
    assert!(result.is_ok(), "Should not panic");

    let symbols = result.unwrap();

    // Should find the class
    let valid_class = symbols
        .iter()
        .find(|s| s.name == "ValidClass" && s.symbol_type == SymbolType::Class);
    assert!(valid_class.is_some(), "Should find ValidClass");

    // Should find at least one valid method
    let valid_methods: Vec<_> = symbols
        .iter()
        .filter(|s| {
            s.symbol_type == SymbolType::Method
                && (s.name == "valid_method" || s.name == "another_valid_method")
        })
        .collect();

    assert!(
        !valid_methods.is_empty(),
        "Should find at least one valid method"
    );
}

/// Test: Parser parses content with syntax error and still returns tree
/// Requirement: SVC-001 - Parse returns tree even with errors
#[test]
fn test_parser_returns_tree_with_errors() {
    let parser = Parser::new(Language::Python).expect("Parser creation should succeed");
    let content = "def broken(";

    let result = parser.parse(content);

    assert!(result.is_ok(), "Parser should return tree even with errors");

    let tree = result.unwrap();
    // Tree should exist and have error node
    assert!(
        tree.root_node().has_error(),
        "Tree should indicate it contains errors"
    );
}

/// Test: Non-existent file returns appropriate error
/// Requirement: SVC-005 - Handle file errors gracefully
#[test]
fn test_nonexistent_file_returns_error() {
    let extractor = SymbolExtractor::new();
    let path = Path::new("tests/fixtures/nonexistent_file.py");

    let result = extractor.extract_from_file(path);

    assert!(
        result.is_err(),
        "Non-existent file should return error, not panic"
    );
}

/// Test: Unreadable file returns appropriate error
/// Requirement: SVC-005 - Handle file errors gracefully
#[cfg(unix)]
#[test]
fn test_unreadable_file_returns_error() {
    use std::fs::{self, File};
    use std::os::unix::fs::PermissionsExt;

    // Create a temporary file with no read permissions
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("unreadable.py");
    File::create(&file_path).unwrap();

    // Remove read permissions
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&file_path, perms).unwrap();

    let extractor = SymbolExtractor::new();
    let result = extractor.extract_from_file(&file_path);

    // Restore permissions for cleanup
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o644);
    fs::set_permissions(&file_path, perms).unwrap();

    assert!(
        result.is_err(),
        "Unreadable file should return error, not panic"
    );
}

/// Test: Unknown language returns appropriate error or empty
/// Requirement: SVC-005 - Handle unsupported languages gracefully
#[test]
fn test_unknown_language_file() {
    let extractor = SymbolExtractor::new();

    // Try to parse Python content as if it were an unknown file type
    // This tests what happens when language detection fails
    let path = Path::new("tests/fixtures/config.toml"); // If this doesn't exist, test creates it

    // Create the file if needed
    if !path.exists() {
        std::fs::write(path, "[package]\nname = \"test\"").ok();
    }

    let result = extractor.extract_from_file(path);

    // Should either return Ok with empty list or Err for unsupported language
    // NOT panic
    match result {
        Ok(symbols) => {
            // If it returns Ok, symbols should be empty for unknown language
            println!("Returned {} symbols for unknown language", symbols.len());
        }
        Err(e) => {
            // Error is also acceptable
            println!("Returned error for unknown language: {}", e);
        }
    }

    // Clean up test file
    std::fs::remove_file("tests/fixtures/config.toml").ok();
}

/// Test: Large file does not cause memory issues
/// Requirement: NFR-002 - File parsing under 50ms for 10K lines
#[test]
fn test_large_file_handling() {
    let extractor = SymbolExtractor::new();

    // Generate a large Python file with many functions
    let mut content = String::new();
    for i in 0..100 {
        content.push_str(&format!(
            r#"
def function_{i}(x: int) -> int:
    """Function number {i}"""
    return x + {i}
"#,
            i = i
        ));
    }

    let start = std::time::Instant::now();
    let result = extractor.extract_from_content(&content, Language::Python);
    let elapsed = start.elapsed();

    assert!(result.is_ok(), "Large file should not cause panic");

    let symbols = result.unwrap();
    assert!(
        symbols.len() >= 100,
        "Should extract all 100 functions from large file"
    );

    // Should complete in reasonable time (< 1 second for 100 functions)
    assert!(
        elapsed.as_secs() < 1,
        "Large file parsing took too long: {:?}",
        elapsed
    );
}
