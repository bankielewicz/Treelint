//! AC#6: Hash-Based Change Detection Tests
//!
//! Given: A file modification event is received
//! When: Daemon checks if file content actually changed
//! Then:
//!   - File hash (SHA-256) is compared to stored hash
//!   - If hash unchanged (e.g., touch command), no re-indexing occurs
//!   - If hash changed, file is re-indexed and new hash is stored
//!
//! Source files tested:
//!   - src/daemon/watcher.rs (Hash comparison)
//!   - src/index/storage.rs (Hash storage)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - WATCH-008: Skip re-indexing if file hash unchanged

use std::fs;
use std::time::Duration;

use tempfile::TempDir;

// These imports will fail until the watcher module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::watcher::{FileWatcher, HashCache};

/// Test: Hash is calculated using SHA-256
/// Requirement: WATCH-008 - File hash (SHA-256) is compared
#[test]
fn test_hash_calculated_using_sha256() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    let content = "def hello(): return 'world'";
    fs::write(&py_file, content).expect("Failed to write file");

    let hash_cache = HashCache::new();

    // Act
    let hash = hash_cache
        .compute_hash(&py_file)
        .expect("Failed to compute hash");

    // Assert: SHA-256 produces 64 hex characters
    assert_eq!(
        hash.len(),
        64,
        "SHA-256 hash should be 64 hex characters, got {} characters",
        hash.len()
    );
    assert!(
        hash.chars().all(|c| c.is_ascii_hexdigit()),
        "Hash should contain only hex characters"
    );
}

/// Test: Same content produces same hash
/// Requirement: WATCH-008 - Hash is deterministic
#[test]
fn test_same_content_produces_same_hash() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let content = "def identical(): pass";

    let file1 = project_root.join("file1.py");
    let file2 = project_root.join("file2.py");
    fs::write(&file1, content).expect("Failed to write file1");
    fs::write(&file2, content).expect("Failed to write file2");

    let hash_cache = HashCache::new();

    // Act
    let hash1 = hash_cache
        .compute_hash(&file1)
        .expect("Failed to compute hash1");
    let hash2 = hash_cache
        .compute_hash(&file2)
        .expect("Failed to compute hash2");

    // Assert
    assert_eq!(hash1, hash2, "Same content should produce same hash");
}

/// Test: Different content produces different hash
/// Requirement: WATCH-008 - Hash changes when content changes
#[test]
fn test_different_content_produces_different_hash() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(&py_file, "def original(): pass").expect("Failed to write file");

    let hash_cache = HashCache::new();
    let original_hash = hash_cache
        .compute_hash(&py_file)
        .expect("Failed to compute hash");

    // Act: Modify content
    fs::write(&py_file, "def modified(): pass").expect("Failed to modify file");
    let new_hash = hash_cache
        .compute_hash(&py_file)
        .expect("Failed to compute new hash");

    // Assert
    assert_ne!(
        original_hash, new_hash,
        "Different content should produce different hash"
    );
}

/// Test: Touch command does not trigger re-index
/// Requirement: WATCH-008 - If hash unchanged (e.g., touch command), no re-indexing occurs
#[test]
fn test_touch_command_does_not_trigger_reindex() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(&py_file, "def untouched(): pass").expect("Failed to write file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Initial index
    watcher
        .index_file(&py_file)
        .expect("Failed to initial index");
    let initial_index_count = watcher.reindex_trigger_count();

    // Act: Touch file (update mtime without changing content)
    // Simulate touch by opening and closing without writing
    let file = fs::OpenOptions::new()
        .write(true)
        .open(&py_file)
        .expect("Failed to open file");
    file.set_modified(std::time::SystemTime::now())
        .expect("Failed to touch file");
    drop(file);

    // Wait for watcher to process
    std::thread::sleep(Duration::from_millis(200));
    watcher.poll_events(Duration::from_millis(500));

    // Assert: No new re-index should occur
    assert_eq!(
        watcher.reindex_trigger_count(),
        initial_index_count,
        "Touch should NOT trigger re-index (hash unchanged)"
    );
}

/// Test: Hash is stored after indexing
/// Requirement: WATCH-008 - New hash is stored after re-index
#[test]
fn test_hash_stored_after_indexing() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(&py_file, "def func(): pass").expect("Failed to write file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Index file
    watcher.index_file(&py_file).expect("Failed to index file");

    // Assert: Hash should be stored
    let stored_hash = watcher.get_stored_hash(&py_file);
    assert!(
        stored_hash.is_some(),
        "Hash should be stored after indexing"
    );
    assert_eq!(
        stored_hash.as_ref().unwrap().len(),
        64,
        "Stored hash should be SHA-256 (64 hex chars)"
    );
}

/// Test: Hash is updated after re-indexing
/// Requirement: WATCH-008 - If hash changed, file is re-indexed and new hash is stored
#[test]
fn test_hash_updated_after_reindexing() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(&py_file, "def original(): pass").expect("Failed to write file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");
    watcher
        .index_file(&py_file)
        .expect("Failed to initial index");

    let original_stored = watcher
        .get_stored_hash(&py_file)
        .expect("Hash should exist");

    // Act: Modify and trigger reindex
    fs::write(&py_file, "def modified(): pass").expect("Failed to modify file");
    watcher.poll_events(Duration::from_millis(500));
    std::thread::sleep(Duration::from_millis(200));

    // Assert
    let new_stored = watcher
        .get_stored_hash(&py_file)
        .expect("Hash should exist");
    assert_ne!(
        original_stored, new_stored,
        "Stored hash should be updated after re-index"
    );
}

/// Test: Whitespace-only changes are detected
/// Requirement: WATCH-008 - Any content change should change hash
#[test]
fn test_whitespace_only_changes_detected() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(&py_file, "def func(): pass").expect("Failed to write file");

    let hash_cache = HashCache::new();
    let original_hash = hash_cache
        .compute_hash(&py_file)
        .expect("Failed to compute hash");

    // Act: Add whitespace
    fs::write(&py_file, "def func():  pass  ").expect("Failed to modify file");
    let new_hash = hash_cache
        .compute_hash(&py_file)
        .expect("Failed to compute new hash");

    // Assert: Whitespace changes should change hash
    assert_ne!(
        original_hash, new_hash,
        "Whitespace changes should produce different hash"
    );
}
