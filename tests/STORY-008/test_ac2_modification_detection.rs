//! AC#2: File Modification Detection Tests
//!
//! Given: Daemon is running with active file watcher
//! When: A supported source file is modified and saved
//! Then:
//!   - Watcher detects the change within 500ms
//!   - File path is queued for re-indexing
//!   - Duplicate events for same file are debounced (100ms window)
//!
//! Source files tested:
//!   - src/daemon/watcher.rs (Event handling)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - WATCH-003: Debounce rapid file change events

use std::fs;
use std::time::{Duration, Instant};

use tempfile::TempDir;

// These imports will fail until the watcher module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::watcher::{FileWatcher, WatcherEvent, WatcherEventKind};

/// Test: Watcher detects file modification within 500ms
/// Requirement: WATCH-003 - Detect file modifications
#[test]
fn test_watcher_detects_modification_within_500ms() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create initial file
    let py_file = project_root.join("module.py");
    fs::write(&py_file, "def old(): pass").expect("Failed to write initial file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Modify the file
    let start = Instant::now();
    fs::write(&py_file, "def new(): pass").expect("Failed to modify file");

    let events = watcher.poll_events(Duration::from_millis(500));
    let elapsed = start.elapsed();

    // Assert
    assert!(
        events
            .iter()
            .any(|e| e.path() == &py_file && e.kind() == WatcherEventKind::Modify),
        "Watcher should detect file modification"
    );
    assert!(
        elapsed < Duration::from_millis(500),
        "Modification should be detected within 500ms, took {:?}",
        elapsed
    );
}

/// Test: Watcher queues modified file for re-indexing
/// Requirement: WATCH-003 - File path is queued for re-indexing
#[test]
fn test_watcher_queues_file_for_reindexing() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(&py_file, "def old(): pass").expect("Failed to write initial file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Modify the file
    fs::write(&py_file, "def new(): pass").expect("Failed to modify file");

    // Wait for debounce window to complete
    std::thread::sleep(Duration::from_millis(150));

    let queue = watcher.pending_reindex_queue();

    // Assert
    assert!(
        queue.contains(&py_file),
        "Modified file should be queued for re-indexing"
    );
}

/// Test: Watcher debounces rapid file changes (100ms window)
/// Requirement: WATCH-003 - Duplicate events for same file are debounced
#[test]
fn test_watcher_debounces_rapid_changes_100ms_window() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(&py_file, "version = 0").expect("Failed to write initial file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Simulate rapid typing (10 saves within 100ms)
    for i in 1..=10 {
        fs::write(&py_file, format!("version = {}", i)).expect("Failed to modify file");
        std::thread::sleep(Duration::from_millis(10));
    }

    // Wait for debounce window to complete
    std::thread::sleep(Duration::from_millis(150));

    let reindex_count = watcher.reindex_trigger_count();

    // Assert
    assert_eq!(
        reindex_count, 1,
        "Rapid changes should be debounced to single re-index, got {} triggers",
        reindex_count
    );
}

/// Test: Watcher handles multiple files modified simultaneously
/// Requirement: WATCH-003 - Handle multiple file changes
#[test]
fn test_watcher_handles_multiple_files_modified_simultaneously() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let file1 = project_root.join("file1.py");
    let file2 = project_root.join("file2.py");
    let file3 = project_root.join("file3.py");

    fs::write(&file1, "# file1").expect("Failed to write file1");
    fs::write(&file2, "# file2").expect("Failed to write file2");
    fs::write(&file3, "# file3").expect("Failed to write file3");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Modify all files
    fs::write(&file1, "# file1 modified").expect("Failed to modify file1");
    fs::write(&file2, "# file2 modified").expect("Failed to modify file2");
    fs::write(&file3, "# file3 modified").expect("Failed to modify file3");

    // Wait for debounce
    std::thread::sleep(Duration::from_millis(150));

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert: All files should have modification events
    let modified_paths: Vec<_> = events
        .iter()
        .filter(|e| e.kind() == WatcherEventKind::Modify)
        .map(|e| e.path().clone())
        .collect();

    assert!(
        modified_paths.contains(&file1),
        "file1.py should be detected"
    );
    assert!(
        modified_paths.contains(&file2),
        "file2.py should be detected"
    );
    assert!(
        modified_paths.contains(&file3),
        "file3.py should be detected"
    );
}

/// Test: Watcher debounces per-file independently
/// Requirement: WATCH-003 - Debounce is per-file, not global
#[test]
fn test_watcher_debounces_per_file_independently() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let file1 = project_root.join("file1.py");
    let file2 = project_root.join("file2.py");

    fs::write(&file1, "# file1").expect("Failed to write file1");
    fs::write(&file2, "# file2").expect("Failed to write file2");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Rapid changes to file1, then file2
    for i in 1..=5 {
        fs::write(&file1, format!("# file1 v{}", i)).expect("Failed to modify file1");
        std::thread::sleep(Duration::from_millis(10));
    }

    for i in 1..=5 {
        fs::write(&file2, format!("# file2 v{}", i)).expect("Failed to modify file2");
        std::thread::sleep(Duration::from_millis(10));
    }

    // Wait for debounce
    std::thread::sleep(Duration::from_millis(150));

    let queue = watcher.pending_reindex_queue();

    // Assert: Both files should be queued (debounced independently)
    assert!(queue.contains(&file1), "file1.py should be queued");
    assert!(queue.contains(&file2), "file2.py should be queued");
}
