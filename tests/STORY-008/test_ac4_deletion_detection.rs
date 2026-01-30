//! AC#4: File Deletion Detection Tests
//!
//! Given: Daemon is running with indexed files
//! When: An indexed source file is deleted
//! Then:
//!   - Watcher detects the deletion
//!   - Symbols from deleted file are removed from index
//!   - Deleted symbols no longer appear in search results
//!
//! Source files tested:
//!   - src/daemon/watcher.rs (Delete event handling)
//!   - src/index/storage.rs (Index cleanup)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - WATCH-005: Handle file deletion events

use std::fs;
use std::time::Duration;

use tempfile::TempDir;

// These imports will fail until the watcher module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::watcher::{FileWatcher, WatcherEvent, WatcherEventKind};

/// Test: Watcher detects file deletion
/// Requirement: WATCH-005 - Handle file deletion events
#[test]
fn test_watcher_detects_file_deletion() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create file first
    let py_file = project_root.join("to_delete.py");
    fs::write(&py_file, "def old(): pass").expect("Failed to write file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Delete the file
    fs::remove_file(&py_file).expect("Failed to delete file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events
            .iter()
            .any(|e| e.path() == &py_file && e.kind() == WatcherEventKind::Delete),
        "Watcher should detect file deletion"
    );
}

/// Test: Watcher queues deleted file for index removal
/// Requirement: WATCH-005 - Symbols from deleted file are removed from index
#[test]
fn test_watcher_queues_deleted_file_for_removal() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(&py_file, "class ToDelete: pass").expect("Failed to write file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Delete file
    fs::remove_file(&py_file).expect("Failed to delete file");

    // Wait for debounce
    std::thread::sleep(Duration::from_millis(150));

    let removal_queue = watcher.pending_removal_queue();

    // Assert
    assert!(
        removal_queue.contains(&py_file),
        "Deleted file should be queued for index removal"
    );
}

/// Test: Watcher handles deletion in subdirectory
/// Requirement: WATCH-005 - Handle deletion in subdirectories
#[test]
fn test_watcher_detects_deletion_in_subdirectory() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let sub_dir = project_root.join("src");
    fs::create_dir_all(&sub_dir).expect("Failed to create subdirectory");

    let py_file = sub_dir.join("utils.py");
    fs::write(&py_file, "def util(): pass").expect("Failed to write file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Delete file
    fs::remove_file(&py_file).expect("Failed to delete file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events
            .iter()
            .any(|e| e.path() == &py_file && e.kind() == WatcherEventKind::Delete),
        "Watcher should detect file deletion in subdirectory"
    );
}

/// Test: Delete event triggers within 1 second
/// Requirement: BR-001 - File changes must be reflected in index within 1 second
#[test]
fn test_watcher_delete_event_triggers_within_1_second() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(&py_file, "def test(): pass").expect("Failed to write file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");
    let start = std::time::Instant::now();

    // Act: Delete file
    fs::remove_file(&py_file).expect("Failed to delete file");

    let events = watcher.poll_events(Duration::from_secs(1));
    let elapsed = start.elapsed();

    // Assert
    assert!(
        events
            .iter()
            .any(|e| e.path() == &py_file && e.kind() == WatcherEventKind::Delete),
        "Deletion should be detected"
    );
    assert!(
        elapsed < Duration::from_secs(1),
        "Detection should happen within 1 second, took {:?}",
        elapsed
    );
}

/// Test: Watcher correctly identifies deletion vs rename
/// Requirement: WATCH-005 - Distinguish deletion from rename
#[test]
fn test_watcher_distinguishes_deletion_from_rename() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let original = project_root.join("original.py");
    let renamed = project_root.join("renamed.py");
    fs::write(&original, "def func(): pass").expect("Failed to write file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Rename file
    fs::rename(&original, &renamed).expect("Failed to rename file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert: Should see delete of original and create of renamed
    let has_delete = events
        .iter()
        .any(|e| e.path() == &original && e.kind() == WatcherEventKind::Delete);
    let has_create = events
        .iter()
        .any(|e| e.path() == &renamed && e.kind() == WatcherEventKind::Create);

    // Note: Some platforms may emit Rename event instead
    let has_rename = events.iter().any(|e| e.kind() == WatcherEventKind::Rename);

    assert!(
        (has_delete && has_create) || has_rename,
        "Rename should be detected as delete+create or as rename event"
    );
}
