//! AC#4: Symbol Query Operations Tests
//!
//! Given: The database contains indexed symbols
//! When: Search queries are executed
//! Then:
//!   - query_by_name() returns exact matches
//!   - query_by_name_case_insensitive() ignores case
//!   - query_by_type() filters by SymbolType
//!   - query_by_file() returns symbols in file
//!   - Combined filters work
//!   - All queries return Vec<Symbol> with fully populated fields
//!
//! Source files tested:
//!   - src/index/search.rs (Search query functions)
//! Coverage threshold: 95%

use tempfile::TempDir;

// These imports will fail until the index module is implemented
// This is expected behavior for TDD Red phase
use treelint::index::{IndexStorage, QueryFilters};
use treelint::parser::{Language, Symbol, SymbolType, Visibility};

/// Helper to create test symbols for queries
fn setup_test_symbols(storage: &IndexStorage) {
    let symbols = vec![
        Symbol {
            name: "validateUser".to_string(),
            symbol_type: SymbolType::Function,
            visibility: Some(Visibility::Public),
            file_path: "src/auth.rs".to_string(),
            line_start: 10,
            line_end: 25,
            signature: Some("fn validateUser(email: &str) -> bool".to_string()),
            body: None,
            language: Language::Rust,
        },
        Symbol {
            name: "ValidateUser".to_string(),
            symbol_type: SymbolType::Class,
            visibility: Some(Visibility::Public),
            file_path: "src/validators.rs".to_string(),
            line_start: 1,
            line_end: 50,
            signature: Some("struct ValidateUser".to_string()),
            body: None,
            language: Language::Rust,
        },
        Symbol {
            name: "process_request".to_string(),
            symbol_type: SymbolType::Function,
            visibility: Some(Visibility::Public),
            file_path: "src/handlers.rs".to_string(),
            line_start: 5,
            line_end: 30,
            signature: Some("fn process_request(req: Request) -> Response".to_string()),
            body: None,
            language: Language::Rust,
        },
        Symbol {
            name: "handle_error".to_string(),
            symbol_type: SymbolType::Method,
            visibility: Some(Visibility::Private),
            file_path: "src/handlers.rs".to_string(),
            line_start: 35,
            line_end: 45,
            signature: Some("fn handle_error(&self, err: Error)".to_string()),
            body: None,
            language: Language::Rust,
        },
        Symbol {
            name: "MAX_RETRIES".to_string(),
            symbol_type: SymbolType::Constant,
            visibility: Some(Visibility::Public),
            file_path: "src/config.rs".to_string(),
            line_start: 1,
            line_end: 1,
            signature: None,
            body: None,
            language: Language::Rust,
        },
        Symbol {
            name: "HashMap".to_string(),
            symbol_type: SymbolType::Import,
            visibility: None,
            file_path: "src/handlers.rs".to_string(),
            line_start: 1,
            line_end: 1,
            signature: None,
            body: None,
            language: Language::Rust,
        },
    ];

    storage
        .insert_symbols(&symbols)
        .expect("Setup symbols should succeed");
}

/// Test: query_by_name() returns exact matches
/// Requirement: SEARCH-001 - Implement query_by_name(name: &str)
#[test]
fn test_query_by_name_exact_match() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let results = storage
        .query_by_name("validateUser")
        .expect("Query should succeed");

    // Assert
    assert_eq!(results.len(), 1, "Should return exactly one match");
    assert_eq!(results[0].name, "validateUser");
    assert_eq!(results[0].symbol_type, SymbolType::Function);
}

/// Test: query_by_name() returns empty for no match
/// Requirement: SEARCH-001 - Returns empty Vec for no matches
#[test]
fn test_query_by_name_no_match() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let results = storage
        .query_by_name("nonexistent_function")
        .expect("Query should succeed");

    // Assert
    assert!(results.is_empty(), "Should return empty for no match");
}

/// Test: query_by_name() is case-sensitive by default
/// Requirement: SEARCH-001 - Exact name match
#[test]
fn test_query_by_name_case_sensitive() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let results = storage
        .query_by_name("VALIDATEUSER")
        .expect("Query should succeed");

    // Assert: Should not match due to case difference
    assert!(results.is_empty(), "Exact match should be case-sensitive");
}

/// Test: query_by_name_case_insensitive() ignores case
/// Requirement: SEARCH-002 - Implement query_by_name_case_insensitive(name: &str)
#[test]
fn test_query_by_name_case_insensitive_match() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let results = storage
        .query_by_name_case_insensitive("VALIDATEUSER")
        .expect("Query should succeed");

    // Assert
    assert_eq!(
        results.len(),
        2,
        "Should match both validateUser and ValidateUser"
    );
    assert!(results.iter().any(|s| s.name == "validateUser"));
    assert!(results.iter().any(|s| s.name == "ValidateUser"));
}

/// Test: query_by_type() filters by SymbolType
/// Requirement: SEARCH-003 - Implement query_by_type(symbol_type: SymbolType)
#[test]
fn test_query_by_type_filters_correctly() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let functions = storage
        .query_by_type(SymbolType::Function)
        .expect("Query should succeed");

    // Assert
    assert_eq!(functions.len(), 2, "Should return 2 functions");
    assert!(functions
        .iter()
        .all(|s| s.symbol_type == SymbolType::Function));
}

/// Test: query_by_type() returns all symbols of type
/// Requirement: SEARCH-003 - Returns all symbols of specified type
#[test]
fn test_query_by_type_returns_all_of_type() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act: Query for classes
    let classes = storage
        .query_by_type(SymbolType::Class)
        .expect("Query should succeed");

    // Assert
    assert_eq!(classes.len(), 1, "Should return 1 class");
    assert_eq!(classes[0].name, "ValidateUser");
}

/// Test: query_by_file() returns all symbols in file
/// Requirement: SEARCH-004 - Implement query_by_file(file_path: &str)
#[test]
fn test_query_by_file_returns_all_symbols() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let results = storage
        .query_by_file("src/handlers.rs")
        .expect("Query should succeed");

    // Assert
    assert_eq!(results.len(), 3, "Should return 3 symbols from handlers.rs");
    assert!(results.iter().all(|s| s.file_path == "src/handlers.rs"));
}

/// Test: query_by_file() returns empty for unknown file
/// Requirement: SEARCH-004 - Returns empty for unknown file
#[test]
fn test_query_by_file_empty_for_unknown() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let results = storage
        .query_by_file("src/nonexistent.rs")
        .expect("Query should succeed");

    // Assert
    assert!(results.is_empty(), "Should return empty for unknown file");
}

/// Test: Combined filters (name + type)
/// Requirement: SEARCH-005 - Combined filters work
#[test]
fn test_combined_filters_name_and_type() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let filters = QueryFilters::new()
        .with_name("ValidateUser")
        .with_type(SymbolType::Class);

    let results = storage.query(filters).expect("Query should succeed");

    // Assert
    assert_eq!(results.len(), 1, "Should return exactly one match");
    assert_eq!(results[0].name, "ValidateUser");
    assert_eq!(results[0].symbol_type, SymbolType::Class);
}

/// Test: Combined filters (type + file)
/// Requirement: SEARCH-005 - Combined filters work
#[test]
fn test_combined_filters_type_and_file() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let filters = QueryFilters::new()
        .with_type(SymbolType::Function)
        .with_file("src/handlers.rs");

    let results = storage.query(filters).expect("Query should succeed");

    // Assert
    assert_eq!(
        results.len(),
        1,
        "Should return one function from handlers.rs"
    );
    assert_eq!(results[0].name, "process_request");
}

/// Test: Combined filters (name case-insensitive + type)
/// Requirement: SEARCH-005 - Combined filters work
#[test]
fn test_combined_filters_case_insensitive_name_and_type() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let filters = QueryFilters::new()
        .with_name_case_insensitive("validateuser")
        .with_type(SymbolType::Function);

    let results = storage.query(filters).expect("Query should succeed");

    // Assert
    assert_eq!(results.len(), 1, "Should return only function validateUser");
    assert_eq!(results[0].name, "validateUser");
}

/// Test: All queries return Symbol with fully populated fields
/// Requirement: All queries return Vec<Symbol> with fully populated fields
#[test]
fn test_query_returns_fully_populated_symbols() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let results = storage
        .query_by_name("validateUser")
        .expect("Query should succeed");

    // Assert: All fields populated
    let symbol = &results[0];
    assert_eq!(symbol.name, "validateUser");
    assert_eq!(symbol.symbol_type, SymbolType::Function);
    assert_eq!(symbol.visibility, Some(Visibility::Public));
    assert_eq!(symbol.file_path, "src/auth.rs");
    assert_eq!(symbol.line_start, 10);
    assert_eq!(symbol.line_end, 25);
    assert_eq!(
        symbol.signature,
        Some("fn validateUser(email: &str) -> bool".to_string())
    );
    assert_eq!(symbol.language, Language::Rust);
}

/// Test: Query latency under 50ms for single symbol lookup
/// Requirement: NFR-001 - Query latency under 50ms
#[test]
fn test_query_latency_under_50ms() {
    use std::time::Instant;

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let start = Instant::now();
    let _results = storage
        .query_by_name("validateUser")
        .expect("Query should succeed");
    let elapsed = start.elapsed();

    // Assert
    assert!(
        elapsed.as_millis() < 50,
        "Query took {}ms, expected < 50ms",
        elapsed.as_millis()
    );
}

/// Test: Empty filter returns all symbols
/// Requirement: SEARCH-005 - Empty filter behavior
#[test]
fn test_empty_filter_returns_all() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let filters = QueryFilters::new();
    let results = storage.query(filters).expect("Query should succeed");

    // Assert
    assert_eq!(results.len(), 6, "Empty filter should return all 6 symbols");
}

/// Test: Query with visibility filter
/// Requirement: SEARCH-005 - Combined filters work
#[test]
fn test_query_with_visibility_filter() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let filters = QueryFilters::new().with_visibility(Visibility::Private);

    let results = storage.query(filters).expect("Query should succeed");

    // Assert
    assert_eq!(results.len(), 1, "Should return 1 private symbol");
    assert_eq!(results[0].name, "handle_error");
}

/// Test: Query symbols sorted by file then line number
/// Requirement: Consistent ordering of results
#[test]
fn test_query_results_sorted() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act
    let results = storage
        .query_by_file("src/handlers.rs")
        .expect("Query should succeed");

    // Assert: Results should be sorted by line_start
    let line_numbers: Vec<_> = results.iter().map(|s| s.line_start).collect();
    let mut sorted = line_numbers.clone();
    sorted.sort();
    assert_eq!(
        line_numbers, sorted,
        "Results should be sorted by line number"
    );
}

/// Test: Query with language filter
/// Requirement: SEARCH-005 - Filter by language
#[test]
fn test_query_with_language_filter() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    // Insert symbols with different languages
    let rust_symbol = Symbol {
        name: "rust_func".to_string(),
        symbol_type: SymbolType::Function,
        visibility: Some(Visibility::Public),
        file_path: "src/lib.rs".to_string(),
        line_start: 1,
        line_end: 5,
        signature: None,
        body: None,
        language: Language::Rust,
    };

    let python_symbol = Symbol {
        name: "python_func".to_string(),
        symbol_type: SymbolType::Function,
        visibility: Some(Visibility::Public),
        file_path: "src/lib.py".to_string(),
        line_start: 1,
        line_end: 5,
        signature: None,
        body: None,
        language: Language::Python,
    };

    storage
        .insert_symbols(&[rust_symbol, python_symbol])
        .expect("Insert should succeed");

    // Act
    let filters = QueryFilters::new().with_language(Language::Rust);
    let results = storage.query(filters).expect("Query should succeed");

    // Assert
    assert_eq!(results.len(), 1, "Should return only Rust symbol");
    assert_eq!(results[0].language, Language::Rust);
}

/// Test: Parameterized query prevents SQL injection
/// Requirement: BR-001 - No SQL injection
#[test]
fn test_query_prevents_sql_injection() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    setup_test_symbols(&storage);

    // Act: Try SQL injection in name query
    let malicious_input = "'; DROP TABLE symbols; --";
    let result = storage.query_by_name(malicious_input);

    // Assert: Should not error and tables should still exist
    assert!(result.is_ok(), "Query should succeed (returning empty)");

    // Verify symbols table still exists
    let verify = storage.query_by_name("validateUser");
    assert!(verify.is_ok(), "Table should still exist");
    assert!(!verify.unwrap().is_empty(), "Data should still exist");
}

/// Test: Query handles special characters in names
/// Requirement: Edge case - Special characters
#[test]
fn test_query_handles_special_characters() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    let special_symbol = Symbol {
        name: "method_with_special$chars".to_string(),
        symbol_type: SymbolType::Method,
        visibility: Some(Visibility::Public),
        file_path: "src/lib.rs".to_string(),
        line_start: 1,
        line_end: 1,
        signature: None,
        body: None,
        language: Language::Rust,
    };

    storage
        .insert_symbol(&special_symbol)
        .expect("Insert should succeed");

    // Act
    let results = storage
        .query_by_name("method_with_special$chars")
        .expect("Query should succeed");

    // Assert
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "method_with_special$chars");
}
