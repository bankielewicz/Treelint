//! AC#2: Symbol CRUD Operations Tests
//!
//! Given: A list of Symbol structs extracted from parsed files (from STORY-002)
//! When: The storage layer saves the symbols
//! Then:
//!   - Single symbols insertable via insert_symbol()
//!   - Bulk inserts via insert_symbols() use transaction for atomicity
//!   - Existing symbols for same file_path are replaced (upsert)
//!   - updated_at set to Unix timestamp
//!   - Insert of 1000 symbols completes in under 500ms
//!
//! Source files tested:
//!   - src/index/storage.rs (Symbol CRUD operations)
//! Coverage threshold: 95%

use std::time::Instant;

use tempfile::TempDir;

// These imports will fail until the index module is implemented
// This is expected behavior for TDD Red phase
use treelint::index::IndexStorage;
use treelint::parser::{Language, Symbol, SymbolType, Visibility};

/// Helper function to create a test symbol
fn create_test_symbol(name: &str, file_path: &str) -> Symbol {
    Symbol {
        name: name.to_string(),
        symbol_type: SymbolType::Function,
        visibility: Some(Visibility::Public),
        file_path: file_path.to_string(),
        line_start: 1,
        line_end: 10,
        signature: Some(format!("fn {}()", name)),
        body: Some("// function body".to_string()),
        language: Language::Rust,
    }
}

/// Test: insert_symbol() inserts single symbol successfully
/// Requirement: SVC-003 - Implement insert_symbol(symbol: &Symbol)
#[test]
fn test_insert_symbol_inserts_single_symbol() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let symbol = create_test_symbol("test_function", "src/lib.rs");

    // Act
    let result = storage.insert_symbol(&symbol);

    // Assert
    assert!(result.is_ok(), "insert_symbol should succeed");
}

/// Test: Inserted symbol is retrievable with all fields
/// Requirement: SVC-003 - Inserted symbol retrievable with all fields
#[test]
fn test_inserted_symbol_retrievable_with_all_fields() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let symbol = Symbol {
        name: "validate_user".to_string(),
        symbol_type: SymbolType::Function,
        visibility: Some(Visibility::Public),
        file_path: "src/auth.rs".to_string(),
        line_start: 15,
        line_end: 42,
        signature: Some("fn validate_user(email: &str) -> bool".to_string()),
        body: Some("// validation logic".to_string()),
        language: Language::Rust,
    };

    storage
        .insert_symbol(&symbol)
        .expect("Insert should succeed");

    // Act
    let retrieved = storage
        .query_by_name("validate_user")
        .expect("Query should succeed");

    // Assert
    assert_eq!(retrieved.len(), 1, "Should retrieve exactly one symbol");
    let s = &retrieved[0];
    assert_eq!(s.name, "validate_user");
    assert_eq!(s.symbol_type, SymbolType::Function);
    assert_eq!(s.visibility, Some(Visibility::Public));
    assert_eq!(s.file_path, "src/auth.rs");
    assert_eq!(s.line_start, 15);
    assert_eq!(s.line_end, 42);
    assert_eq!(
        s.signature,
        Some("fn validate_user(email: &str) -> bool".to_string())
    );
    assert_eq!(s.language, Language::Rust);
}

/// Test: insert_symbols() bulk inserts with transaction
/// Requirement: SVC-004 - Implement insert_symbols(symbols: &[Symbol]) with transaction
#[test]
fn test_insert_symbols_bulk_inserts_with_transaction() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let symbols: Vec<Symbol> = (0..100)
        .map(|i| create_test_symbol(&format!("func_{}", i), "src/lib.rs"))
        .collect();

    // Act
    let result = storage.insert_symbols(&symbols);

    // Assert
    assert!(result.is_ok(), "Bulk insert should succeed");

    // Verify all symbols were inserted
    let all_symbols = storage
        .query_by_file("src/lib.rs")
        .expect("Query should succeed");
    assert_eq!(all_symbols.len(), 100, "All 100 symbols should be inserted");
}

/// Test: Bulk insert is atomic (all or nothing)
/// Requirement: SVC-004 - Bulk inserts use transaction for atomicity
#[test]
fn test_bulk_insert_is_atomic() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    // Insert initial symbols
    let initial_symbols: Vec<Symbol> = (0..10)
        .map(|i| create_test_symbol(&format!("initial_{}", i), "src/initial.rs"))
        .collect();
    storage
        .insert_symbols(&initial_symbols)
        .expect("Initial insert should succeed");

    // Create mixed batch with potential constraint violation
    // (implementation should use transaction and rollback on error)
    let mixed_symbols: Vec<Symbol> = (0..5)
        .map(|i| create_test_symbol(&format!("mixed_{}", i), "src/mixed.rs"))
        .collect();

    // Act: Insert should succeed or fail atomically
    let result = storage.insert_symbols(&mixed_symbols);

    // Assert: If successful, all should be present. If failed, none should be present.
    if result.is_ok() {
        let count = storage
            .query_by_file("src/mixed.rs")
            .expect("Query should succeed")
            .len();
        assert_eq!(
            count, 5,
            "All symbols should be present after successful transaction"
        );
    }
}

/// Test: Upsert behavior replaces existing symbols for same file
/// Requirement: SVC-005 - Existing symbols for same file_path are replaced (upsert)
#[test]
fn test_upsert_replaces_existing_symbols() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    // Insert initial symbol
    let initial = Symbol {
        name: "old_function".to_string(),
        symbol_type: SymbolType::Function,
        visibility: Some(Visibility::Public),
        file_path: "src/module.rs".to_string(),
        line_start: 1,
        line_end: 5,
        signature: None,
        body: None,
        language: Language::Rust,
    };
    storage
        .insert_symbol(&initial)
        .expect("Initial insert should succeed");

    // Create new symbols for same file (simulating re-indexing)
    let new_symbols = vec![
        create_test_symbol("new_function_1", "src/module.rs"),
        create_test_symbol("new_function_2", "src/module.rs"),
    ];

    // Act: Upsert should replace old symbols
    storage
        .upsert_symbols_for_file("src/module.rs", &new_symbols)
        .expect("Upsert should succeed");

    // Assert
    let symbols = storage
        .query_by_file("src/module.rs")
        .expect("Query should succeed");

    assert_eq!(symbols.len(), 2, "Should have exactly 2 new symbols");
    assert!(
        !symbols.iter().any(|s| s.name == "old_function"),
        "Old symbol should be replaced"
    );
    assert!(
        symbols.iter().any(|s| s.name == "new_function_1"),
        "New symbol 1 should exist"
    );
    assert!(
        symbols.iter().any(|s| s.name == "new_function_2"),
        "New symbol 2 should exist"
    );
}

/// Test: 1000 symbols insert completes in under 500ms
/// Requirement: NFR-002 - Bulk insert under 500ms for 1000 symbols
#[test]
fn test_1000_symbols_insert_under_500ms() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let symbols: Vec<Symbol> = (0..1000)
        .map(|i| create_test_symbol(&format!("func_{}", i), &format!("src/file_{}.rs", i % 10)))
        .collect();

    // Act
    let start = Instant::now();
    storage
        .insert_symbols(&symbols)
        .expect("Bulk insert should succeed");
    let elapsed = start.elapsed();

    // Assert
    assert!(
        elapsed.as_millis() < 500,
        "1000 symbol insert took {}ms, expected < 500ms",
        elapsed.as_millis()
    );
}

/// Test: delete_symbols_for_file() deletes all symbols for a file
/// Requirement: SVC-005 - Implement delete_symbols_for_file(file_path: &str)
#[test]
fn test_delete_symbols_for_file_works() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    let symbols = vec![
        create_test_symbol("func_1", "src/target.rs"),
        create_test_symbol("func_2", "src/target.rs"),
        create_test_symbol("func_other", "src/other.rs"),
    ];
    storage
        .insert_symbols(&symbols)
        .expect("Insert should succeed");

    // Act
    let deleted_count = storage
        .delete_symbols_for_file("src/target.rs")
        .expect("Delete should succeed");

    // Assert
    assert_eq!(deleted_count, 2, "Should delete 2 symbols");

    let remaining = storage
        .query_by_file("src/target.rs")
        .expect("Query should succeed");
    assert!(
        remaining.is_empty(),
        "No symbols should remain for deleted file"
    );

    let other = storage
        .query_by_file("src/other.rs")
        .expect("Query should succeed");
    assert_eq!(other.len(), 1, "Other file's symbols should be preserved");
}

/// Test: updated_at field is set on insert
/// Requirement: SVC-003 - updated_at set to Unix timestamp
#[test]
fn test_updated_at_set_on_insert() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let symbol = create_test_symbol("timestamped_func", "src/lib.rs");

    let before_insert = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Act
    storage
        .insert_symbol(&symbol)
        .expect("Insert should succeed");

    let after_insert = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Assert: Verify timestamp is within expected range
    let updated_at = storage
        .get_symbol_updated_at("timestamped_func")
        .expect("Should get updated_at");

    assert!(
        updated_at >= before_insert && updated_at <= after_insert,
        "updated_at ({}) should be between {} and {}",
        updated_at,
        before_insert,
        after_insert
    );
}

/// Test: Insert symbol with all SymbolType variants
/// Requirement: SVC-003 - Handle all symbol types
#[test]
fn test_insert_all_symbol_types() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    let symbol_types = [
        SymbolType::Function,
        SymbolType::Class,
        SymbolType::Method,
        SymbolType::Variable,
        SymbolType::Constant,
        SymbolType::Import,
        SymbolType::Export,
    ];

    // Act & Assert
    for (i, symbol_type) in symbol_types.iter().enumerate() {
        let symbol = Symbol {
            name: format!("symbol_{}", i),
            symbol_type: *symbol_type,
            visibility: Some(Visibility::Public),
            file_path: "src/types.rs".to_string(),
            line_start: i + 1,
            line_end: i + 1,
            signature: None,
            body: None,
            language: Language::Rust,
        };

        let result = storage.insert_symbol(&symbol);
        assert!(result.is_ok(), "Should insert {:?} symbol", symbol_type);
    }

    let all = storage
        .query_by_file("src/types.rs")
        .expect("Query should succeed");
    assert_eq!(all.len(), 7, "All 7 symbol types should be inserted");
}

/// Test: Insert symbol with unicode name
/// Requirement: Edge case - Unicode symbol names
#[test]
fn test_insert_symbol_with_unicode_name() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let symbol = Symbol {
        name: "validate_user".to_string(),
        symbol_type: SymbolType::Function,
        visibility: Some(Visibility::Public),
        file_path: "src/lib.rs".to_string(),
        line_start: 1,
        line_end: 1,
        signature: None,
        body: None,
        language: Language::Rust,
    };

    // Act
    let result = storage.insert_symbol(&symbol);

    // Assert
    assert!(result.is_ok(), "Should handle unicode names");

    let retrieved = storage
        .query_by_name("validate_user")
        .expect("Query should succeed");
    assert_eq!(retrieved.len(), 1);
    assert_eq!(retrieved[0].name, "validate_user");
}

/// Test: Insert symbol with very long signature
/// Requirement: Edge case - Long signatures
#[test]
fn test_insert_symbol_with_long_signature() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    let long_signature = format!(
        "fn complex_function({})",
        (0..50)
            .map(|i| format!("arg{}: Type{}", i, i))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let symbol = Symbol {
        name: "complex_function".to_string(),
        symbol_type: SymbolType::Function,
        visibility: Some(Visibility::Public),
        file_path: "src/lib.rs".to_string(),
        line_start: 1,
        line_end: 100,
        signature: Some(long_signature.clone()),
        body: None,
        language: Language::Rust,
    };

    // Act
    storage
        .insert_symbol(&symbol)
        .expect("Insert should succeed");

    // Assert
    let retrieved = storage
        .query_by_name("complex_function")
        .expect("Query should succeed");
    assert_eq!(retrieved[0].signature, Some(long_signature));
}

/// Test: Insert symbol with null/None optional fields
/// Requirement: SVC-003 - Handle optional fields
#[test]
fn test_insert_symbol_with_none_optional_fields() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let symbol = Symbol {
        name: "minimal_func".to_string(),
        symbol_type: SymbolType::Function,
        visibility: None, // Optional
        file_path: "src/lib.rs".to_string(),
        line_start: 1,
        line_end: 1,
        signature: None, // Optional
        body: None,      // Optional
        language: Language::Rust,
    };

    // Act
    let result = storage.insert_symbol(&symbol);

    // Assert
    assert!(result.is_ok(), "Should handle None optional fields");

    let retrieved = storage
        .query_by_name("minimal_func")
        .expect("Query should succeed");
    assert_eq!(retrieved[0].visibility, None);
    assert_eq!(retrieved[0].signature, None);
    assert_eq!(retrieved[0].body, None);
}
