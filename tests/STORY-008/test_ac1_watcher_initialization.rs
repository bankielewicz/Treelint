//! AC#1: File Watcher Initialization Tests
//!
//! Given: Daemon is starting in a project directory
//! When: Daemon initializes the file watcher
//! Then:
//!   - Watcher monitors project root recursively
//!   - Watcher respects .gitignore patterns (if present)
//!   - Watcher ignores .treelint/ directory
//!   - Watcher filters to supported file extensions (.py, .ts, .tsx, .rs, .md)
//!
//! Source files tested:
//!   - src/daemon/watcher.rs (FileWatcher implementation)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - WATCH-001: Initialize recursive file watcher on project root
//!   - WATCH-002: Filter events to supported file extensions

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use tempfile::TempDir;

// These imports will fail until the watcher module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::watcher::{FileWatcher, WatcherConfig, WatcherEvent};

/// Test: FileWatcher initializes with recursive watching on project root
/// Requirement: WATCH-001 - Initialize recursive file watcher on project root
#[test]
fn test_watcher_initializes_recursive_watching_on_project_root() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create nested directory structure
    let nested_dir = project_root.join("src").join("nested");
    fs::create_dir_all(&nested_dir).expect("Failed to create nested directory");

    // Act
    let watcher = FileWatcher::new(project_root);

    // Assert
    assert!(
        watcher.is_ok(),
        "FileWatcher should initialize successfully on project root"
    );
}

/// Test: FileWatcher watches all subdirectories recursively
/// Requirement: WATCH-001 - Initialize recursive file watcher on project root
#[test]
fn test_watcher_monitors_subdirectories_recursively() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create nested directory structure
    let nested_dir = project_root.join("src").join("deeply").join("nested");
    fs::create_dir_all(&nested_dir).expect("Failed to create nested directory");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create a file in deeply nested directory
    let nested_file = nested_dir.join("test.py");
    fs::write(&nested_file, "def test(): pass").expect("Failed to write file");

    // Wait for event with timeout
    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events.iter().any(|e| e.path() == &nested_file),
        "Watcher should detect file creation in deeply nested directory"
    );
}

/// Test: FileWatcher respects .gitignore patterns
/// Requirement: WATCH-002 - Filter events to supported file extensions
#[test]
fn test_watcher_respects_gitignore_patterns() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create .gitignore with patterns
    let gitignore_content = "node_modules/\n*.log\ntarget/\n";
    fs::write(project_root.join(".gitignore"), gitignore_content)
        .expect("Failed to write .gitignore");

    // Create ignored directories
    let node_modules = project_root.join("node_modules");
    fs::create_dir_all(&node_modules).expect("Failed to create node_modules");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create files in ignored directory
    let ignored_file = node_modules.join("package.json");
    fs::write(&ignored_file, "{}").expect("Failed to write ignored file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        !events.iter().any(|e| e.path().starts_with(&node_modules)),
        "Watcher should NOT detect changes in gitignored node_modules directory"
    );
}

/// Test: FileWatcher ignores .treelint directory
/// Requirement: WATCH-002 - Always ignore .treelint/ directory
#[test]
fn test_watcher_ignores_treelint_directory() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create .treelint directory
    let treelint_dir = project_root.join(".treelint");
    fs::create_dir_all(&treelint_dir).expect("Failed to create .treelint");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create file in .treelint directory
    let index_db = treelint_dir.join("index.db");
    fs::write(&index_db, "database content").expect("Failed to write index.db");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        !events.iter().any(|e| e.path().starts_with(&treelint_dir)),
        "Watcher should NOT detect changes in .treelint directory"
    );
}

/// Test: FileWatcher filters to supported file extensions (.py)
/// Requirement: WATCH-002 - Filter events to supported file extensions
#[test]
fn test_watcher_filters_supported_extensions_py() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create a Python file
    let py_file = project_root.join("module.py");
    fs::write(&py_file, "def main(): pass").expect("Failed to write .py file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events.iter().any(|e| e.path() == &py_file),
        "Watcher should detect .py file changes"
    );
}

/// Test: FileWatcher filters to supported file extensions (.ts)
/// Requirement: WATCH-002 - Filter events to supported file extensions
#[test]
fn test_watcher_filters_supported_extensions_ts() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create a TypeScript file
    let ts_file = project_root.join("component.ts");
    fs::write(&ts_file, "export function test() {}").expect("Failed to write .ts file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events.iter().any(|e| e.path() == &ts_file),
        "Watcher should detect .ts file changes"
    );
}

/// Test: FileWatcher filters to supported file extensions (.rs)
/// Requirement: WATCH-002 - Filter events to supported file extensions
#[test]
fn test_watcher_filters_supported_extensions_rs() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create a Rust file
    let rs_file = project_root.join("lib.rs");
    fs::write(&rs_file, "pub fn main() {}").expect("Failed to write .rs file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events.iter().any(|e| e.path() == &rs_file),
        "Watcher should detect .rs file changes"
    );
}

/// Test: FileWatcher filters to supported file extensions (.md)
/// Requirement: WATCH-002 - Filter events to supported file extensions
#[test]
fn test_watcher_filters_supported_extensions_md() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create a Markdown file
    let md_file = project_root.join("README.md");
    fs::write(&md_file, "# Title\n\nContent").expect("Failed to write .md file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        events.iter().any(|e| e.path() == &md_file),
        "Watcher should detect .md file changes"
    );
}

/// Test: FileWatcher ignores unsupported file extensions
/// Requirement: WATCH-002 - Filter events to supported file extensions
#[test]
fn test_watcher_ignores_unsupported_extensions() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create unsupported file types
    let json_file = project_root.join("config.json");
    let txt_file = project_root.join("notes.txt");
    let log_file = project_root.join("debug.log");

    fs::write(&json_file, "{}").expect("Failed to write .json file");
    fs::write(&txt_file, "notes").expect("Failed to write .txt file");
    fs::write(&log_file, "log entry").expect("Failed to write .log file");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        !events.iter().any(|e| e.path() == &json_file),
        "Watcher should NOT detect .json file changes"
    );
    assert!(
        !events.iter().any(|e| e.path() == &txt_file),
        "Watcher should NOT detect .txt file changes"
    );
    assert!(
        !events.iter().any(|e| e.path() == &log_file),
        "Watcher should NOT detect .log file changes"
    );
}

/// Test: FileWatcher ignores .git directory
/// Requirement: WATCH-002 - Always ignore .git/ directory
#[test]
fn test_watcher_ignores_git_directory() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create .git directory
    let git_dir = project_root.join(".git");
    fs::create_dir_all(&git_dir).expect("Failed to create .git");

    let watcher = FileWatcher::new(project_root).expect("Failed to create watcher");

    // Act: Create file in .git directory
    let git_config = git_dir.join("config");
    fs::write(&git_config, "[core]\n").expect("Failed to write git config");

    let events = watcher.poll_events(Duration::from_millis(500));

    // Assert
    assert!(
        !events.iter().any(|e| e.path().starts_with(&git_dir)),
        "Watcher should NOT detect changes in .git directory"
    );
}
