//! AC#3: File Creation Detection Tests
//!
//! Given: Daemon is running with active file watcher
//! When: A new supported source file is created
//! Then:
//!   - Watcher detects the new file
//!   - File is parsed and symbols are added to index
//!   - New symbols appear in subsequent search queries
//!
//! Source files tested:
//!   - src/daemon/watcher.rs (Create event handling)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - WATCH-004: Handle file creation events

use std::fs;
use std::time::Duration;

use tempfile::TempDir;

// These imports will fail until the watcher module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::watcher::{FileWatcher, WatcherEvent, WatcherEventKind};

/// Test: Watcher detects new file creation
/// Requirement: WATCH-004 - Handle file creation events
#[test]
fn test_watcher_detects_new_file_creation() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create a new Python file
    let new_file = project_root.join("new_module.py");
    fs::write(&new_file, "def hello(): return 'world'").expect("Failed to write new file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events
            .iter()
            .any(|e| e.path() == &new_file && e.kind() == WatcherEventKind::Create),
        "Watcher should detect new file creation"
    );
}

/// Test: Watcher queues new file for indexing
/// Requirement: WATCH-004 - New file indexed
#[test]
fn test_watcher_queues_new_file_for_indexing() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create a new file
    let new_file = project_root.join("new_module.py");
    fs::write(&new_file, "class NewClass: pass").expect("Failed to write new file");

    // Wait for debounce
    std::thread::sleep(Duration::from_millis(150));

    let queue = watcher.pending_index_queue();

    // Assert
    assert!(
        queue.contains(&new_file),
        "New file should be queued for indexing"
    );
}

/// Test: Watcher detects file creation in subdirectory
/// Requirement: WATCH-004 - Handle file creation in subdirectories
#[test]
fn test_watcher_detects_creation_in_subdirectory() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create subdirectory
    let sub_dir = project_root.join("src");
    fs::create_dir_all(&sub_dir).expect("Failed to create subdirectory");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create file in subdirectory
    let new_file = sub_dir.join("utils.py");
    fs::write(&new_file, "def utility(): pass").expect("Failed to write new file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events
            .iter()
            .any(|e| e.path() == &new_file && e.kind() == WatcherEventKind::Create),
        "Watcher should detect file creation in subdirectory"
    );
}

/// Test: Watcher detects TypeScript file creation
/// Requirement: WATCH-004 - Handle TypeScript file creation
#[test]
fn test_watcher_detects_typescript_file_creation() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create a new TypeScript file
    let ts_file = project_root.join("component.ts");
    fs::write(
        &ts_file,
        "export function greet(): string { return 'hello'; }",
    )
    .expect("Failed to write TypeScript file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events
            .iter()
            .any(|e| e.path() == &ts_file && e.kind() == WatcherEventKind::Create),
        "Watcher should detect TypeScript file creation"
    );
}

/// Test: Watcher detects Rust file creation
/// Requirement: WATCH-004 - Handle Rust file creation
#[test]
fn test_watcher_detects_rust_file_creation() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create a new Rust file
    let rs_file = project_root.join("lib.rs");
    fs::write(&rs_file, "pub fn main() {}").expect("Failed to write Rust file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events
            .iter()
            .any(|e| e.path() == &rs_file && e.kind() == WatcherEventKind::Create),
        "Watcher should detect Rust file creation"
    );
}

/// Test: Watcher handles empty file creation
/// Requirement: WATCH-004 - Handle empty file creation (edge case)
#[test]
fn test_watcher_handles_empty_file_creation() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create an empty file
    let empty_file = project_root.join("empty.py");
    fs::write(&empty_file, "").expect("Failed to write empty file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events
            .iter()
            .any(|e| e.path() == &empty_file && e.kind() == WatcherEventKind::Create),
        "Watcher should detect empty file creation"
    );
}

/// Test: Create event triggers within 1 second
/// Requirement: BR-001 - File changes must be reflected in index within 1 second
#[test]
fn test_watcher_create_event_triggers_within_1_second() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");
    let start = std::time::Instant::now();

    // Act: Create file
    let new_file = project_root.join("module.py");
    fs::write(&new_file, "def test(): pass").expect("Failed to write file");

    let events = watcher.poll_events(Duration::from_secs(1));
    let elapsed = start.elapsed();

    // Assert
    assert!(
        events.iter().any(|e| e.path() == &new_file),
        "File should be detected"
    );
    assert!(
        elapsed < Duration::from_secs(1),
        "Detection should happen within 1 second, took {:?}",
        elapsed
    );
}
