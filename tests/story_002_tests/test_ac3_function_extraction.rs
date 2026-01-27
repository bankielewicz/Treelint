//! AC#3: Symbol Extraction - Functions Tests
//!
//! Given: A source file containing function definitions
//! When: The parser extracts symbols from the file
//! Then: Each function is returned as a Symbol struct with:
//!       - name
//!       - SymbolType::Function
//!       - accurate line_start/line_end
//!       - signature
//!       - language
//!
//! Test file: tests/STORY-002/test_ac3_function_extraction.rs
//! Source files tested:
//!   - src/parser/symbols.rs (Symbol struct and extractor)
//!   - src/parser/queries/python.rs (Python queries)
//!   - src/parser/queries/typescript.rs (TypeScript queries)
//!   - src/parser/queries/rust.rs (Rust queries)
//! Coverage threshold: 95%

use std::path::Path;

// These imports will fail until the parser module is implemented
// This is expected behavior for TDD Red phase
use treelint::parser::{Language, Symbol, SymbolExtractor, SymbolType};

/// Helper to get fixture path
fn fixture_path(relative: &str) -> String {
    format!("tests/fixtures/{}", relative)
}

/// Test: Symbol struct has all required fields
/// Requirement: DM-004 - Define Symbol struct with all required fields
#[test]
fn test_symbol_struct_has_required_fields() {
    let symbol = Symbol {
        name: "test_function".to_string(),
        symbol_type: SymbolType::Function,
        visibility: None,
        file_path: "test.py".to_string(),
        line_start: 1,
        line_end: 5,
        signature: Some("def test_function(arg: str) -> None".to_string()),
        body: None,
        language: Language::Python,
    };

    assert_eq!(symbol.name, "test_function");
    assert_eq!(symbol.symbol_type, SymbolType::Function);
    assert_eq!(symbol.line_start, 1);
    assert_eq!(symbol.line_end, 5);
    assert!(symbol.signature.is_some());
    assert_eq!(symbol.language, Language::Python);
}

/// Test: SymbolType enum has Function variant
/// Requirement: DM-006 - Define SymbolType enum with Function variant
#[test]
fn test_symbol_type_has_function_variant() {
    let symbol_type = SymbolType::Function;

    assert_eq!(symbol_type, SymbolType::Function);
}

/// Test: Symbol can be serialized to JSON
/// Requirement: DM-005 - Derive Serialize, Clone, Debug for Symbol
#[test]
fn test_symbol_serializes_to_json() {
    let symbol = Symbol {
        name: "greet".to_string(),
        symbol_type: SymbolType::Function,
        visibility: None,
        file_path: "test.py".to_string(),
        line_start: 1,
        line_end: 3,
        signature: Some("def greet(name: str) -> str".to_string()),
        body: None,
        language: Language::Python,
    };

    let json = serde_json::to_string(&symbol);

    assert!(json.is_ok(), "Symbol should serialize to JSON");
    let json_str = json.unwrap();
    assert!(json_str.contains("greet"));
    assert!(json_str.contains("Function"));
}

/// Test: Extract functions from Python file
/// Requirement: CFG-001 - Define tree-sitter query for Python function_definition
#[test]
fn test_extract_functions_from_python_file() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/simple_functions.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let functions: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Function)
        .collect();

    // simple_functions.py has: greet, add, calculate_area, fetch_data
    assert!(
        functions.len() >= 4,
        "Expected at least 4 functions, found {}",
        functions.len()
    );

    // Verify specific function was extracted
    let greet = functions.iter().find(|s| s.name == "greet");
    assert!(greet.is_some(), "Should find 'greet' function");

    let greet = greet.unwrap();
    assert_eq!(greet.symbol_type, SymbolType::Function);
    assert_eq!(greet.language, Language::Python);
    assert!(greet.line_start > 0, "line_start should be positive");
    assert!(greet.line_end >= greet.line_start, "line_end >= line_start");
}

/// Test: Extract functions from TypeScript file
/// Requirement: CFG-004 - Define tree-sitter query for TypeScript function
#[test]
fn test_extract_functions_from_typescript_file() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/simple_functions.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let functions: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Function)
        .collect();

    // simple_functions.ts has: greet, add, multiply (arrow), fetchData, identity
    assert!(
        functions.len() >= 4,
        "Expected at least 4 functions, found {}",
        functions.len()
    );

    // Verify function was extracted
    let greet = functions.iter().find(|s| s.name == "greet");
    assert!(greet.is_some(), "Should find 'greet' function");

    let greet = greet.unwrap();
    assert_eq!(greet.language, Language::TypeScript);
}

/// Test: Extract functions from Rust file
/// Requirement: CFG-006 - Define tree-sitter query for Rust fn
#[test]
fn test_extract_functions_from_rust_file() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/simple_functions.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let functions: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Function)
        .collect();

    // simple_functions.rs has: greet, add, calculate_area, fetch_data, identity, longest
    assert!(
        functions.len() >= 5,
        "Expected at least 5 functions, found {}",
        functions.len()
    );

    // Verify function was extracted
    let greet = functions.iter().find(|s| s.name == "greet");
    assert!(greet.is_some(), "Should find 'greet' function");

    let greet = greet.unwrap();
    assert_eq!(greet.language, Language::Rust);
}

/// Test: Extracted function has accurate line numbers
/// Requirement: DM-004 - Symbol has accurate line_start/line_end
#[test]
fn test_function_has_accurate_line_numbers() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/simple_functions.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Find the 'greet' function (should be first function after docstring)
    let greet = symbols
        .iter()
        .find(|s| s.name == "greet" && s.symbol_type == SymbolType::Function)
        .expect("Should find 'greet' function");

    // greet function starts at line 4 (after module docstring)
    // and ends at line 6
    assert!(greet.line_start >= 4, "greet should start around line 4");
    assert!(
        greet.line_end >= greet.line_start,
        "line_end should be >= line_start"
    );
    assert!(
        greet.line_end - greet.line_start <= 5,
        "greet function should span about 3 lines"
    );
}

/// Test: Extracted function has signature
/// Requirement: DM-004 - Symbol has signature field
#[test]
fn test_function_has_signature() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/simple_functions.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let greet = symbols
        .iter()
        .find(|s| s.name == "greet" && s.symbol_type == SymbolType::Function)
        .expect("Should find 'greet' function");

    assert!(greet.signature.is_some(), "Function should have signature");
    let sig = greet.signature.as_ref().unwrap();
    assert!(
        sig.contains("greet") && sig.contains("name"),
        "Signature should contain function name and parameter: {}",
        sig
    );
}

/// Test: Extract async function from Python
/// Requirement: CFG-001 - Query captures async functions
#[test]
fn test_extract_async_function_python() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/simple_functions.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let fetch_data = symbols
        .iter()
        .find(|s| s.name == "fetch_data" && s.symbol_type == SymbolType::Function);

    assert!(
        fetch_data.is_some(),
        "Should find async 'fetch_data' function"
    );
}

/// Test: Extract async function from TypeScript
/// Requirement: CFG-004 - Query captures async functions
#[test]
fn test_extract_async_function_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/simple_functions.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let fetch_data = symbols
        .iter()
        .find(|s| s.name == "fetchData" && s.symbol_type == SymbolType::Function);

    assert!(
        fetch_data.is_some(),
        "Should find async 'fetchData' function"
    );
}

/// Test: Extract generic function from Rust
/// Requirement: CFG-006 - Query captures generic functions
#[test]
fn test_extract_generic_function_rust() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/simple_functions.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let identity = symbols
        .iter()
        .find(|s| s.name == "identity" && s.symbol_type == SymbolType::Function);

    assert!(
        identity.is_some(),
        "Should find generic 'identity' function"
    );
}

/// Test: Extract arrow functions from TypeScript
/// Requirement: CFG-004 - Query captures arrow functions
#[test]
fn test_extract_arrow_function_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/simple_functions.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // 'multiply' is defined as const multiply = (x, y) => ...
    let multiply = symbols.iter().find(|s| s.name == "multiply");

    assert!(
        multiply.is_some(),
        "Should find arrow function 'multiply' (may be Variable or Function type)"
    );
}

/// Test: Extract Markdown headings as Functions (for repository map)
/// Requirement: CFG-007 - Define tree-sitter query for Markdown headings
#[test]
fn test_extract_headings_from_markdown() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("markdown/document.md");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Markdown headings are extracted as Function symbols for repo map
    let headings: Vec<_> = symbols
        .iter()
        .filter(|s| s.language == Language::Markdown)
        .collect();

    // document.md has multiple headings: Main Title, Getting Started, etc.
    assert!(
        headings.len() >= 5,
        "Expected at least 5 headings, found {}",
        headings.len()
    );

    // Check for specific heading
    let main_title = headings.iter().find(|s| s.name.contains("Main Title"));
    assert!(main_title.is_some(), "Should find 'Main Title' heading");
}

/// Test: SymbolExtractor can extract from string content
/// Requirement: SVC-001 - Parse file content using tree-sitter
#[test]
fn test_extract_from_content_string() {
    let extractor = SymbolExtractor::new();
    let content = r#"
def hello(name: str) -> str:
    return f"Hello, {name}!"

def goodbye():
    pass
"#;

    let symbols = extractor
        .extract_from_content(content, Language::Python)
        .expect("Extraction should succeed");

    let functions: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Function)
        .collect();

    assert_eq!(functions.len(), 2, "Should extract 2 functions");
    assert!(functions.iter().any(|s| s.name == "hello"));
    assert!(functions.iter().any(|s| s.name == "goodbye"));
}

/// Test: Extract function with lifetime parameters (Rust)
/// Requirement: CFG-006 - Query handles lifetime parameters
#[test]
fn test_extract_function_with_lifetimes_rust() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/simple_functions.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let longest = symbols
        .iter()
        .find(|s| s.name == "longest" && s.symbol_type == SymbolType::Function);

    assert!(
        longest.is_some(),
        "Should find 'longest' function with lifetimes"
    );
}

/// Test: Symbols are returned in source order
/// Requirement: DM-004 - Symbols should maintain source order
#[test]
fn test_symbols_in_source_order() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/simple_functions.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let functions: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Function)
        .collect();

    // Verify line numbers are in ascending order
    for window in functions.windows(2) {
        assert!(
            window[0].line_start <= window[1].line_start,
            "Functions should be in source order: {} (line {}) should come before {} (line {})",
            window[0].name,
            window[0].line_start,
            window[1].name,
            window[1].line_start
        );
    }
}
