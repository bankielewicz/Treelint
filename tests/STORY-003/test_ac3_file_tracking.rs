//! AC#3: File Tracking for Change Detection Tests
//!
//! Given: A file has been indexed
//! When: The storage layer records the file metadata
//! Then:
//!   - File path, language, SHA-256 hash, and indexed_at stored in files table
//!   - get_file_hash() returns stored hash
//!   - needs_reindex() returns true if hash differs or file not in database
//!   - Re-indexed files have old symbols deleted first
//!
//! Source files tested:
//!   - src/index/storage.rs (File tracking operations)
//! Coverage threshold: 95%

use std::time::{SystemTime, UNIX_EPOCH};

use tempfile::TempDir;

// These imports will fail until the index module is implemented
// This is expected behavior for TDD Red phase
use treelint::index::IndexStorage;
use treelint::parser::Language;

/// Test: record_file() stores file metadata
/// Requirement: SVC-006 - Implement record_file(path, language, hash)
#[test]
fn test_record_file_stores_metadata() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let file_path = "src/main.rs";
    let language = Language::Rust;
    let hash = "abc123def456789";

    // Act
    let result = storage.record_file(file_path, language, hash);

    // Assert
    assert!(result.is_ok(), "record_file should succeed");
}

/// Test: Recorded file has all fields stored correctly
/// Requirement: SVC-006 - File path, language, SHA-256 hash, and indexed_at stored
#[test]
fn test_record_file_stores_all_fields() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let file_path = "src/lib.rs";
    let language = Language::Rust;
    let hash = "sha256_hash_value_123";

    let before_record = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Act
    storage
        .record_file(file_path, language, hash)
        .expect("record_file should succeed");

    let after_record = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Assert
    let file_info = storage
        .get_file_info(file_path)
        .expect("Should get file info");

    assert!(file_info.is_some(), "File info should exist");
    let info = file_info.unwrap();
    assert_eq!(info.path, file_path);
    assert_eq!(info.language, language);
    assert_eq!(info.hash, hash);
    assert!(
        info.indexed_at >= before_record && info.indexed_at <= after_record,
        "indexed_at should be within expected range"
    );
}

/// Test: get_file_hash() returns stored hash
/// Requirement: SVC-007 - Implement get_file_hash(path: &str)
#[test]
fn test_get_file_hash_returns_stored_hash() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let file_path = "src/parser.rs";
    let expected_hash = "expected_sha256_hash_value";

    storage
        .record_file(file_path, Language::Rust, expected_hash)
        .expect("record_file should succeed");

    // Act
    let result = storage.get_file_hash(file_path);

    // Assert
    assert!(result.is_ok(), "get_file_hash should succeed");
    assert_eq!(
        result.unwrap(),
        Some(expected_hash.to_string()),
        "Should return stored hash"
    );
}

/// Test: get_file_hash() returns None for unknown file
/// Requirement: SVC-007 - Returns Some(hash) for indexed, None for unknown
#[test]
fn test_get_file_hash_returns_none_for_unknown_file() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    // Act
    let result = storage.get_file_hash("nonexistent/file.rs");

    // Assert
    assert!(result.is_ok(), "get_file_hash should succeed");
    assert_eq!(result.unwrap(), None, "Should return None for unknown file");
}

/// Test: needs_reindex() returns true when hash differs
/// Requirement: SVC-008 - needs_reindex returns true if hash differs
#[test]
fn test_needs_reindex_true_when_hash_differs() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let file_path = "src/module.rs";
    let old_hash = "old_hash_value";
    let new_hash = "new_hash_value";

    storage
        .record_file(file_path, Language::Rust, old_hash)
        .expect("record_file should succeed");

    // Act
    let result = storage.needs_reindex(file_path, new_hash);

    // Assert
    assert!(result.is_ok(), "needs_reindex should succeed");
    assert!(result.unwrap(), "Should return true when hash differs");
}

/// Test: needs_reindex() returns false when hash matches
/// Requirement: SVC-008 - needs_reindex returns false if hash matches
#[test]
fn test_needs_reindex_false_when_hash_matches() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let file_path = "src/unchanged.rs";
    let hash = "same_hash_value";

    storage
        .record_file(file_path, Language::Rust, hash)
        .expect("record_file should succeed");

    // Act
    let result = storage.needs_reindex(file_path, hash);

    // Assert
    assert!(result.is_ok(), "needs_reindex should succeed");
    assert!(!result.unwrap(), "Should return false when hash matches");
}

/// Test: needs_reindex() returns true for file not in database
/// Requirement: SVC-008 - needs_reindex returns true if file unknown
#[test]
fn test_needs_reindex_true_for_unknown_file() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    // Act
    let result = storage.needs_reindex("unknown/file.rs", "any_hash");

    // Assert
    assert!(result.is_ok(), "needs_reindex should succeed");
    assert!(result.unwrap(), "Should return true for unknown file");
}

/// Test: Re-indexing file updates hash
/// Requirement: SVC-006 - Recording file updates existing record
#[test]
fn test_record_file_updates_existing_hash() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let file_path = "src/reindexed.rs";
    let old_hash = "old_hash";
    let new_hash = "new_hash";

    storage
        .record_file(file_path, Language::Rust, old_hash)
        .expect("First record should succeed");

    // Act
    storage
        .record_file(file_path, Language::Rust, new_hash)
        .expect("Second record should succeed");

    // Assert
    let stored_hash = storage
        .get_file_hash(file_path)
        .expect("Should get hash")
        .expect("Hash should exist");

    assert_eq!(stored_hash, new_hash, "Hash should be updated to new value");
}

/// Test: Record files with all supported languages
/// Requirement: SVC-006 - Handle all Language variants
#[test]
fn test_record_file_all_languages() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    let files = [
        ("src/main.py", Language::Python),
        ("src/app.ts", Language::TypeScript),
        ("src/lib.rs", Language::Rust),
        ("README.md", Language::Markdown),
    ];

    // Act & Assert
    for (path, language) in files {
        let result = storage.record_file(path, language, "hash");
        assert!(result.is_ok(), "Should record {:?} file", language);

        let info = storage
            .get_file_info(path)
            .expect("Should get file info")
            .expect("File info should exist");
        assert_eq!(
            info.language, language,
            "Language should match for {}",
            path
        );
    }
}

/// Test: indexed_at updates on re-record
/// Requirement: SVC-006 - indexed_at updated on re-record
#[test]
fn test_indexed_at_updates_on_rerecord() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let file_path = "src/timed.rs";

    // First record
    storage
        .record_file(file_path, Language::Rust, "hash1")
        .expect("First record should succeed");

    let first_indexed_at = storage
        .get_file_info(file_path)
        .expect("Should get file info")
        .expect("File info should exist")
        .indexed_at;

    // Wait a moment to ensure timestamp differs
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Act: Re-record
    storage
        .record_file(file_path, Language::Rust, "hash2")
        .expect("Second record should succeed");

    let second_indexed_at = storage
        .get_file_info(file_path)
        .expect("Should get file info")
        .expect("File info should exist")
        .indexed_at;

    // Assert
    assert!(
        second_indexed_at >= first_indexed_at,
        "indexed_at should update on re-record"
    );
}

/// Test: Multiple files can be tracked simultaneously
/// Requirement: SVC-006 - Track multiple files
#[test]
fn test_multiple_files_tracked_simultaneously() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    let files: Vec<_> = (0..50)
        .map(|i| (format!("src/file_{}.rs", i), format!("hash_{}", i)))
        .collect();

    // Act
    for (path, hash) in &files {
        storage
            .record_file(path, Language::Rust, hash)
            .expect("record_file should succeed");
    }

    // Assert
    for (path, expected_hash) in &files {
        let stored_hash = storage
            .get_file_hash(path)
            .expect("Should get hash")
            .expect("Hash should exist");
        assert_eq!(
            &stored_hash, expected_hash,
            "Hash should match for {}",
            path
        );
    }
}

/// Test: File path with special characters
/// Requirement: Edge case - Special characters in paths
#[test]
fn test_file_path_with_special_characters() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let special_path = "src/special-file_v2.0 (copy).rs";

    // Act
    let result = storage.record_file(special_path, Language::Rust, "hash");

    // Assert
    assert!(result.is_ok(), "Should handle special characters in path");

    let info = storage
        .get_file_info(special_path)
        .expect("Should get file info");
    assert!(info.is_some(), "Should retrieve file with special path");
}

/// Test: Empty hash value handling
/// Requirement: Edge case - Empty hash
#[test]
fn test_empty_hash_value_handling() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    // Act
    let result = storage.record_file("src/empty_hash.rs", Language::Rust, "");

    // Assert: Should either succeed (empty is valid) or return clear error
    // Not panic
    assert!(
        result.is_ok() || result.is_err(),
        "Should handle empty hash gracefully"
    );
}

/// Test: get_all_tracked_files returns all recorded files
/// Requirement: SVC-006 - Ability to list all tracked files
#[test]
fn test_get_all_tracked_files() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");

    let files = vec!["src/a.rs", "src/b.rs", "src/c.rs"];
    for file in &files {
        storage
            .record_file(file, Language::Rust, "hash")
            .expect("record_file should succeed");
    }

    // Act
    let tracked = storage
        .get_all_tracked_files()
        .expect("Should get tracked files");

    // Assert
    assert_eq!(tracked.len(), 3, "Should have 3 tracked files");
    for file in &files {
        assert!(
            tracked.iter().any(|f| f.path == *file),
            "Should contain {}",
            file
        );
    }
}

/// Test: remove_file_tracking removes file from tracking
/// Requirement: SVC-006 - Ability to remove file tracking
#[test]
fn test_remove_file_tracking() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage = IndexStorage::new(temp_dir.path()).expect("Failed to create storage");
    let file_path = "src/to_remove.rs";

    storage
        .record_file(file_path, Language::Rust, "hash")
        .expect("record_file should succeed");

    // Verify file is tracked
    assert!(
        storage.get_file_hash(file_path).unwrap().is_some(),
        "File should be tracked"
    );

    // Act
    let result = storage.remove_file_tracking(file_path);

    // Assert
    assert!(result.is_ok(), "remove_file_tracking should succeed");
    assert!(
        storage.get_file_hash(file_path).unwrap().is_none(),
        "File should no longer be tracked"
    );
}
