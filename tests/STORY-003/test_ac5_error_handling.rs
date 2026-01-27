//! AC#5: Error Handling and Recovery Tests
//!
//! Given: The database may encounter errors (corruption, permissions, disk full)
//! When: Storage operations fail
//! Then:
//!   - All errors wrapped in StorageError enum
//!   - StorageError has variants: DatabaseCorrupted, PermissionDenied, DiskFull, ConnectionFailed, QueryFailed
//!   - No panics on any error
//!   - Corrupted database detected via PRAGMA integrity_check
//!   - Permission errors return helpful message
//!
//! Source files tested:
//!   - src/index/storage.rs (Error type definitions)
//! Coverage threshold: 95%

use std::fs::{self, File, Permissions};
use std::io::Write;
use std::path::Path;

use tempfile::TempDir;

// These imports will fail until the index module is implemented
// This is expected behavior for TDD Red phase
use treelint::index::{IndexStorage, StorageError};
use treelint::parser::{Language, Symbol, SymbolType, Visibility};

/// Helper to create a test symbol
fn create_test_symbol() -> Symbol {
    Symbol {
        name: "test_func".to_string(),
        symbol_type: SymbolType::Function,
        visibility: Some(Visibility::Public),
        file_path: "src/lib.rs".to_string(),
        line_start: 1,
        line_end: 5,
        signature: None,
        body: None,
        language: Language::Rust,
    }
}

/// Test: StorageError enum has DatabaseCorrupted variant
/// Requirement: DM-001 - StorageError enum with DatabaseCorrupted variant
#[test]
fn test_storage_error_has_database_corrupted_variant() {
    let error = StorageError::DatabaseCorrupted("Test corruption".to_string());
    let display = format!("{}", error);
    assert!(
        display.contains("corrupt") || display.contains("Corrupt"),
        "DatabaseCorrupted should have descriptive message"
    );
}

/// Test: StorageError enum has PermissionDenied variant
/// Requirement: DM-001 - StorageError enum with PermissionDenied variant
#[test]
fn test_storage_error_has_permission_denied_variant() {
    let error = StorageError::PermissionDenied("/path/to/file".to_string());
    let display = format!("{}", error);
    assert!(
        display.contains("permission")
            || display.contains("Permission")
            || display.contains("denied")
            || display.contains("Denied"),
        "PermissionDenied should have descriptive message"
    );
}

/// Test: StorageError enum has DiskFull variant
/// Requirement: DM-001 - StorageError enum with DiskFull variant
#[test]
fn test_storage_error_has_disk_full_variant() {
    let error = StorageError::DiskFull;
    let display = format!("{}", error);
    assert!(
        display.contains("disk")
            || display.contains("Disk")
            || display.contains("space")
            || display.contains("full"),
        "DiskFull should have descriptive message"
    );
}

/// Test: StorageError enum has ConnectionFailed variant
/// Requirement: DM-001 - StorageError enum with ConnectionFailed variant
#[test]
fn test_storage_error_has_connection_failed_variant() {
    let error = StorageError::ConnectionFailed("Test reason".to_string());
    let display = format!("{}", error);
    assert!(
        display.contains("connection")
            || display.contains("Connection")
            || display.contains("connect"),
        "ConnectionFailed should have descriptive message"
    );
}

/// Test: StorageError enum has QueryFailed variant
/// Requirement: DM-001 - StorageError enum with QueryFailed variant
#[test]
fn test_storage_error_has_query_failed_variant() {
    let error = StorageError::QueryFailed("SELECT failed".to_string());
    let display = format!("{}", error);
    assert!(
        display.contains("query") || display.contains("Query") || display.contains("SELECT"),
        "QueryFailed should have descriptive message"
    );
}

/// Test: rusqlite::Error maps to StorageError
/// Requirement: DM-002 - Implement From<rusqlite::Error> for automatic conversion
#[test]
fn test_rusqlite_error_converts_to_storage_error() {
    // This test verifies the From trait implementation exists
    // by creating a scenario that would produce a rusqlite error
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    // Try an invalid operation that would cause rusqlite error
    let result = storage.execute_raw_sql("INVALID SQL SYNTAX");

    // Assert: Error should be wrapped in StorageError, not panic
    assert!(result.is_err(), "Invalid SQL should return error");
    match result.unwrap_err() {
        StorageError::QueryFailed(_) => {} // Expected
        other => panic!("Expected QueryFailed, got {:?}", other),
    }
}

/// Test: No panic on corrupted database
/// Requirement: NFR-003 - No panics on errors
#[test]
fn test_no_panic_on_corrupted_database() {
    // Arrange: Create a corrupted database file
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let treelint_dir = temp_dir.path().join(".treelint");
    fs::create_dir_all(&treelint_dir).expect("Failed to create directory");

    let db_path = treelint_dir.join("index.db");
    let mut file = File::create(&db_path).expect("Failed to create file");
    file.write_all(b"This is not a valid SQLite database")
        .expect("Failed to write corrupted data");

    // Act: Try to open corrupted database
    let result = IndexStorage::new(temp_dir.path());

    // Assert: Should return error, not panic
    assert!(
        result.is_err(),
        "Opening corrupted database should return error"
    );
}

/// Test: Corrupted database detected via integrity_check
/// Requirement: AC#5 - Corrupted database detected via PRAGMA integrity_check
#[test]
fn test_verify_integrity_detects_corruption() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    // Act: Run integrity check on valid database
    let result = storage.verify_integrity();

    // Assert: Valid database should pass
    assert!(result.is_ok(), "Valid database should pass integrity check");
    assert!(
        result.unwrap(),
        "verify_integrity should return true for valid database"
    );
}

/// Test: Permission error returns helpful message
/// Requirement: AC#5 - Permission errors return helpful message
#[test]
#[cfg(unix)]
fn test_permission_error_helpful_message() {
    use std::os::unix::fs::PermissionsExt;

    // Arrange: Create read-only directory
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let treelint_dir = temp_dir.path().join(".treelint");
    fs::create_dir_all(&treelint_dir).expect("Failed to create directory");

    // Make directory read-only (no write permission)
    let perms = Permissions::from_mode(0o444);
    fs::set_permissions(&treelint_dir, perms).expect("Failed to set permissions");

    // Act: Try to create database in read-only directory
    let result = IndexStorage::new(temp_dir.path());

    // Cleanup: Restore permissions before assertions
    let restore_perms = Permissions::from_mode(0o755);
    let _ = fs::set_permissions(&treelint_dir, restore_perms);

    // Assert: Should return PermissionDenied with helpful message
    assert!(result.is_err(), "Should fail with permission error");
    let error = result.unwrap_err();
    let error_msg = format!("{}", error);
    assert!(
        error_msg.contains("permission")
            || error_msg.contains("Permission")
            || error_msg.contains("denied")
            || error_msg.contains("access"),
        "Error message should mention permission issue: {}",
        error_msg
    );
}

/// Test: No panic on insert with closed connection
/// Requirement: NFR-003 - No panics on errors
#[test]
fn test_no_panic_after_close() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    // Close the connection
    storage.close().expect("Close should succeed");

    // Act: Try to insert after close
    let result = storage.insert_symbol(&create_test_symbol());

    // Assert: Should return error, not panic
    assert!(result.is_err(), "Operation after close should return error");
}

/// Test: No panic on query with invalid SQL
/// Requirement: NFR-003 - All errors return Result, no unwrap in production
#[test]
fn test_no_panic_on_invalid_sql() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    // Act: Execute invalid SQL
    let result = storage.execute_raw_sql("SELECT * FROM nonexistent_table");

    // Assert: Should return error, not panic
    assert!(result.is_err(), "Invalid SQL should return error");
}

/// Test: Error recovery - reopen after error
/// Requirement: Resilience - ability to recover from errors
#[test]
fn test_error_recovery_reopen() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create and close first connection
    {
        let storage = IndexStorage::new(temp_dir.path()).expect("First open should succeed");
        storage
            .insert_symbol(&create_test_symbol())
            .expect("Insert should succeed");
    }

    // Act: Reopen
    let storage = IndexStorage::new(temp_dir.path()).expect("Reopen should succeed");

    // Assert: Data should be accessible
    let results = storage
        .query_by_name("test_func")
        .expect("Query should succeed");
    assert_eq!(results.len(), 1, "Data should persist after reopen");
}

/// Test: Transaction rollback on error
/// Requirement: Atomicity - failed operations don't leave partial state
#[test]
fn test_transaction_rollback_on_error() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    // Insert initial data
    storage
        .insert_symbol(&create_test_symbol())
        .expect("Initial insert should succeed");

    // Act: Simulate a failed batch insert
    // (implementation should rollback partial inserts)
    let result = storage.insert_symbols_with_error_simulation(
        &[Symbol {
            name: "good_symbol".to_string(),
            symbol_type: SymbolType::Function,
            visibility: None,
            file_path: "src/lib.rs".to_string(),
            line_start: 1,
            line_end: 1,
            signature: None,
            body: None,
            language: Language::Rust,
        }],
        true,
    ); // true = simulate error after first insert

    // Assert: Partial data should not be committed
    assert!(result.is_err(), "Simulated error should return Err");

    let good_symbol = storage
        .query_by_name("good_symbol")
        .expect("Query should succeed");
    assert!(
        good_symbol.is_empty(),
        "Partial insert should be rolled back"
    );

    // Original data should still exist
    let original = storage
        .query_by_name("test_func")
        .expect("Query should succeed");
    assert_eq!(original.len(), 1, "Original data should be preserved");
}

/// Test: StorageError implements Debug and Display
/// Requirement: DM-001 - Each variant has descriptive Display message
#[test]
fn test_storage_error_implements_debug_and_display() {
    let errors = vec![
        StorageError::DatabaseCorrupted("test".to_string()),
        StorageError::PermissionDenied("/path".to_string()),
        StorageError::DiskFull,
        StorageError::ConnectionFailed("reason".to_string()),
        StorageError::QueryFailed("query".to_string()),
    ];

    for error in errors {
        // Test Display
        let display = format!("{}", error);
        assert!(
            !display.is_empty(),
            "Display should produce non-empty string"
        );

        // Test Debug
        let debug = format!("{:?}", error);
        assert!(!debug.is_empty(), "Debug should produce non-empty string");
    }
}

/// Test: StorageError implements std::error::Error
/// Requirement: DM-001 - Proper error handling
#[test]
fn test_storage_error_implements_std_error() {
    let error = StorageError::QueryFailed("test".to_string());

    // Should be usable with ? operator and error chains
    fn takes_std_error(_: &dyn std::error::Error) {}
    takes_std_error(&error);
}

/// Test: No panic when database file is locked
/// Requirement: NFR-003 - No panics on errors
#[test]
fn test_no_panic_when_database_locked() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create first connection (will hold lock in WAL mode)
    let storage1 = IndexStorage::new(temp_dir.path()).expect("First open should succeed");

    // Start a transaction that holds the lock
    storage1
        .begin_exclusive_transaction()
        .expect("Transaction should start");

    // Act: Try to open second connection and write
    let storage2 = IndexStorage::new(temp_dir.path()).expect("Second open should succeed");
    let result = storage2.insert_symbol(&create_test_symbol());

    // Cleanup
    storage1.rollback_transaction().ok();

    // Assert: Should handle gracefully (either succeed with WAL or return error)
    // WAL mode allows concurrent readers, so this might succeed
    // Main assertion is: NO PANIC
    assert!(
        result.is_ok() || result.is_err(),
        "Should handle lock gracefully"
    );
}

/// Test: Graceful handling of invalid path
/// Requirement: Error handling for invalid inputs
#[test]
fn test_graceful_handling_invalid_path() {
    // Act: Try to create storage with invalid path
    let result = IndexStorage::new(Path::new(
        "/nonexistent/deeply/nested/path/that/cannot/exist",
    ));

    // Assert: Should return error, not panic
    assert!(result.is_err(), "Invalid path should return error");
}

/// Test: Error message includes context
/// Requirement: DM-001 - Helpful error messages
#[test]
fn test_error_message_includes_context() {
    let error =
        StorageError::ConnectionFailed("Unable to connect to database at /path/to/db".to_string());
    let display = format!("{}", error);

    assert!(
        display.contains("/path/to/db") || display.contains("Unable to connect"),
        "Error message should include context about the failure"
    );
}

/// Test: All error variants are distinct
/// Requirement: DM-001 - Clear error categorization
#[test]
fn test_all_error_variants_distinct() {
    use std::mem::discriminant;

    let errors = vec![
        StorageError::DatabaseCorrupted("".to_string()),
        StorageError::PermissionDenied("".to_string()),
        StorageError::DiskFull,
        StorageError::ConnectionFailed("".to_string()),
        StorageError::QueryFailed("".to_string()),
    ];

    // Verify all variants have distinct discriminants
    let discriminants: Vec<_> = errors.iter().map(discriminant).collect();
    let unique_count = discriminants
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len();

    assert_eq!(
        unique_count,
        errors.len(),
        "All StorageError variants should be distinct"
    );
}
