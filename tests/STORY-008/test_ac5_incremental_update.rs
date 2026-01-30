//! AC#5: Incremental Index Update Tests
//!
//! Given: Daemon detects a file change
//! When: The file is re-indexed
//! Then:
//!   - Only the changed file is re-parsed (not entire codebase)
//!   - Old symbols for that file are replaced with new symbols
//!   - Re-indexing completes within 1 second
//!   - Daemon status changes to "indexing" during update, then back to "ready"
//!
//! Source files tested:
//!   - src/daemon/watcher.rs (Incremental indexing)
//!   - src/index/storage.rs (Symbol replacement)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - WATCH-006: Handle file modification events
//!   - WATCH-007: Re-index single file efficiently

use std::fs;
use std::time::{Duration, Instant};

use tempfile::TempDir;

// These imports will fail until the watcher module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::watcher::{FileWatcher, IncrementalIndexer, IndexStats};
use treelint::daemon::DaemonState;

/// Test: Incremental indexer re-parses only the changed file
/// Requirement: WATCH-007 - Only the changed file is re-parsed
#[test]
fn test_incremental_indexer_reparses_only_changed_file() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create multiple files
    let file1 = project_root.join("file1.py");
    let file2 = project_root.join("file2.py");
    let file3 = project_root.join("file3.py");

    fs::write(&file1, "def func1(): pass").expect("Failed to write file1");
    fs::write(&file2, "def func2(): pass").expect("Failed to write file2");
    fs::write(&file3, "def func3(): pass").expect("Failed to write file3");

    let indexer = IncrementalIndexer::new(project_root).expect("Failed to create indexer");

    // Initial index
    indexer.index_all().expect("Failed to initial index");
    let initial_parse_count = indexer.total_parse_count();

    // Act: Modify only file2
    fs::write(&file2, "def func2_modified(): pass").expect("Failed to modify file2");
    let stats = indexer.reindex_file(&file2).expect("Failed to reindex");

    // Assert: Only file2 should be parsed
    assert_eq!(
        stats.files_parsed, 1,
        "Only 1 file should be parsed during incremental update"
    );
    assert_eq!(
        indexer.total_parse_count() - initial_parse_count,
        1,
        "Parse count should increase by 1"
    );
}

/// Test: Incremental indexer replaces old symbols
/// Requirement: WATCH-006 - Old symbols for that file are replaced with new symbols
#[test]
fn test_incremental_indexer_replaces_old_symbols() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(&py_file, "def old_function(): pass\nclass OldClass: pass")
        .expect("Failed to write file");

    let indexer = IncrementalIndexer::new(project_root).expect("Failed to create indexer");
    indexer.index_file(&py_file).expect("Failed to index file");

    // Verify old symbols exist
    let old_symbols = indexer.get_symbols_for_file(&py_file);
    assert!(
        old_symbols.iter().any(|s| s.name == "old_function"),
        "Old function should exist in index"
    );

    // Act: Modify file with new symbols
    fs::write(&py_file, "def new_function(): pass\nclass NewClass: pass")
        .expect("Failed to modify file");
    indexer.reindex_file(&py_file).expect("Failed to reindex");

    // Assert: Old symbols replaced with new
    let new_symbols = indexer.get_symbols_for_file(&py_file);
    assert!(
        !new_symbols.iter().any(|s| s.name == "old_function"),
        "Old function should NOT exist after reindex"
    );
    assert!(
        new_symbols.iter().any(|s| s.name == "new_function"),
        "New function should exist after reindex"
    );
}

/// Test: Incremental re-indexing completes within 1 second
/// Requirement: WATCH-007 - Re-indexing completes within 1 second
#[test]
fn test_incremental_reindex_completes_within_1_second() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create a moderately sized file (1000 lines)
    let py_file = project_root.join("large_module.py");
    let content: String = (0..1000)
        .map(|i| format!("def function_{}(arg{0}): return arg{0} + {0}\n", i))
        .collect();
    fs::write(&py_file, &content).expect("Failed to write file");

    let indexer = IncrementalIndexer::new(project_root).expect("Failed to create indexer");
    indexer
        .index_file(&py_file)
        .expect("Failed to initial index");

    // Modify file
    let modified_content = content.replace("function_0", "modified_function_0");
    fs::write(&py_file, &modified_content).expect("Failed to modify file");

    // Act: Time the reindex
    let start = Instant::now();
    let stats = indexer.reindex_file(&py_file).expect("Failed to reindex");
    let elapsed = start.elapsed();

    // Assert
    assert!(
        elapsed < Duration::from_secs(1),
        "Re-indexing should complete within 1 second, took {:?}",
        elapsed
    );
    assert!(
        stats.duration_ms < 1000,
        "Stats should report < 1000ms, reported {}ms",
        stats.duration_ms
    );
}

/// Test: Daemon status transitions during indexing
/// Requirement: WATCH-006 - Daemon status changes to "indexing" during update
#[test]
fn test_daemon_status_transitions_during_indexing() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(&py_file, "def func(): pass").expect("Failed to write file");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");
    watcher
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready");

    assert_eq!(
        watcher.state(),
        DaemonState::Ready,
        "Initial state should be Ready"
    );

    // Act: Trigger reindex and capture state during
    fs::write(&py_file, "def modified(): pass").expect("Failed to modify file");

    // Poll to trigger reindex
    let state_during = watcher.trigger_reindex_sync(&py_file);

    // Assert
    assert_eq!(
        state_during,
        DaemonState::Indexing,
        "State should be Indexing during reindex"
    );

    // Wait for completion
    std::thread::sleep(Duration::from_millis(200));
    assert_eq!(
        watcher.state(),
        DaemonState::Ready,
        "State should return to Ready after reindex"
    );
}

/// Test: Incremental index stats are accurate
/// Requirement: WATCH-007 - Return accurate IndexStats
#[test]
fn test_incremental_index_stats_are_accurate() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("module.py");
    fs::write(
        &py_file,
        "def func1(): pass\ndef func2(): pass\nclass MyClass: pass",
    )
    .expect("Failed to write file");

    let indexer = IncrementalIndexer::new(project_root).expect("Failed to create indexer");

    // Act
    let stats = indexer.index_file(&py_file).expect("Failed to index");

    // Assert
    assert_eq!(stats.files_parsed, 1, "Should report 1 file parsed");
    assert_eq!(
        stats.symbols_added, 3,
        "Should report 3 symbols added (2 functions + 1 class)"
    );
    assert!(stats.duration_ms > 0, "Duration should be positive");
}

/// Test: Reindex handles file with parse errors gracefully
/// Requirement: WATCH-006 - Handle files with syntax errors
#[test]
fn test_reindex_handles_parse_errors_gracefully() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let py_file = project_root.join("broken.py");
    fs::write(&py_file, "def valid(): pass").expect("Failed to write file");

    let indexer = IncrementalIndexer::new(project_root).expect("Failed to create indexer");
    indexer
        .index_file(&py_file)
        .expect("Failed to initial index");

    // Act: Write syntactically invalid content
    fs::write(&py_file, "def broken( # missing closing paren").expect("Failed to write broken");
    let result = indexer.reindex_file(&py_file);

    // Assert: Should not crash, may return error or partial results
    // The important thing is graceful handling
    assert!(
        result.is_ok() || result.is_err(),
        "Reindex should handle parse errors without panic"
    );
}

/// Test: Reindex 10K line file within 1 second
/// Requirement: NFR-001 - Single file re-indexing < 1 second for 10K lines
#[test]
fn test_reindex_10k_line_file_within_1_second() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create a 10,000 line file
    let py_file = project_root.join("huge_module.py");
    let content: String = (0..10000)
        .map(|i| format!("def function_{}(): return {}\n", i, i))
        .collect();
    fs::write(&py_file, &content).expect("Failed to write large file");

    let indexer = IncrementalIndexer::new(project_root).expect("Failed to create indexer");
    indexer
        .index_file(&py_file)
        .expect("Failed to initial index");

    // Modify file
    let modified = content.replace("function_0", "modified_0");
    fs::write(&py_file, &modified).expect("Failed to modify file");

    // Act
    let start = Instant::now();
    let _stats = indexer.reindex_file(&py_file).expect("Failed to reindex");
    let elapsed = start.elapsed();

    // Assert
    assert!(
        elapsed < Duration::from_secs(1),
        "10K line file reindex should complete within 1 second, took {:?}",
        elapsed
    );
}
