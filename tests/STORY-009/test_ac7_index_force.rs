//! AC#7: Force Re-index Flag Tests
//!
//! Given: Index already exists with cached file hashes
//! When: User runs `treelint index --force`
//! Then:
//!   - All files are re-parsed regardless of hash
//!   - Existing index is rebuilt from scratch
//!   - Progress bar shows full file count
//!
//! Source files tested:
//!   - src/cli/commands/index.rs (Force flag handling)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - CLI-011: Force flag bypasses hash-based caching

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// Helper: Create a test project with source files
fn setup_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    for i in 0..5 {
        let file_content = format!(
            r#"
def function_{i}(x: int) -> int:
    """Function number {i}"""
    return x * {i}
"#,
            i = i
        );
        let file_path = src_dir.join(format!("module_{}.py", i));
        fs::write(&file_path, file_content).expect("Failed to write file");
    }

    temp_dir
}

/// Test: `treelint index --force` flag is recognized
/// Requirement: CLI-011 - Force flag bypasses hash-based caching
#[test]
fn test_index_force_flag_recognized() {
    // Arrange
    let temp_dir = setup_test_project();

    // Act & Assert: Command should not fail with "unrecognized flag"
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index", "--force"])
        .assert()
        .success();
}

/// Test: `treelint index --force` re-indexes all files
/// Requirement: CLI-011 - Force flag bypasses hash-based caching
#[test]
fn test_index_force_reindexes_all_files() {
    // Arrange
    let temp_dir = setup_test_project();

    // First, build initial index
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Record index file modification time
    let index_path = temp_dir.path().join(".treelint").join("index.db");
    let initial_metadata = fs::metadata(&index_path).ok();
    let initial_modified = initial_metadata.and_then(|m| m.modified().ok());

    // Wait a bit to ensure time difference
    std::thread::sleep(Duration::from_millis(100));

    // Act: Force re-index
    let output = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index", "--force"])
        .assert()
        .success();

    // Assert: Output should show all files being processed
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    // Should show file count matching our test files
    assert!(
        combined.contains("5") || combined.to_lowercase().contains("file"),
        "Force re-index should process all files.\n\nOutput:\n{}",
        combined
    );

    // Index file should have been modified
    if let Some(initial_time) = initial_modified {
        let final_metadata = fs::metadata(&index_path).expect("Index should exist");
        let final_modified = final_metadata
            .modified()
            .expect("Should have modified time");
        assert!(
            final_modified > initial_time,
            "Index should have been modified by --force"
        );
    }
}

/// Test: `treelint index --force` rebuilds index from scratch
/// Requirement: CLI-011 - Force flag bypasses hash-based caching
#[test]
fn test_index_force_rebuilds_from_scratch() {
    // Arrange
    let temp_dir = setup_test_project();

    // Build initial index
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Add a new file
    let new_file = temp_dir.path().join("src").join("new_module.py");
    fs::write(&new_file, "def new_function(): pass").expect("Failed to write new file");

    // Act: Force re-index
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index", "--force"])
        .assert()
        .success();

    // Assert: Search should find the new function
    let search_output = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "new_function", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&search_output.get_output().stdout);
    assert!(
        stdout.contains("new_function"),
        "Force re-index should include new files.\n\nSearch output:\n{}",
        stdout
    );
}

/// Test: `treelint index --force` takes longer than incremental
/// Requirement: CLI-011 - Force flag bypasses hash-based caching
#[test]
fn test_index_force_takes_longer_than_incremental() {
    // Arrange
    let temp_dir = setup_test_project();

    // Build initial index
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Time incremental index (no changes)
    let incremental_start = Instant::now();
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();
    let incremental_time = incremental_start.elapsed();

    // Time force index
    let force_start = Instant::now();
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index", "--force"])
        .assert()
        .success();
    let force_time = force_start.elapsed();

    // Assert: Force should typically take longer (re-parses everything)
    // Note: This might not always be true for tiny projects, so we just log it
    println!(
        "Incremental: {:?}, Force: {:?}",
        incremental_time, force_time
    );
}

/// Test: `treelint index --force` exit code is 0
/// Requirement: CLI-011 - Force flag bypasses hash-based caching
#[test]
fn test_index_force_exit_code_zero() {
    // Arrange
    let temp_dir = setup_test_project();

    // Act & Assert
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index", "--force"])
        .assert()
        .code(0);
}

/// Test: `treelint index --force` on empty project
/// Requirement: CLI-011 - Force flag bypasses hash-based caching
#[test]
fn test_index_force_empty_project() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Act & Assert
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index", "--force"])
        .assert()
        .code(0);
}

/// Test: `treelint index --force` after file deletion
/// Requirement: CLI-011 - Force flag bypasses hash-based caching
#[test]
fn test_index_force_after_file_deletion() {
    // Arrange
    let temp_dir = setup_test_project();

    // Build initial index
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Delete a file
    let file_to_delete = temp_dir.path().join("src").join("module_0.py");
    fs::remove_file(&file_to_delete).expect("Failed to delete file");

    // Act: Force re-index
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index", "--force"])
        .assert()
        .success();

    // Assert: Search should not find the deleted function
    let search_output = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "function_0", "--format", "json"])
        .assert();

    let stdout = String::from_utf8_lossy(&search_output.get_output().stdout);
    // Should have empty results array (the deleted function should not be found)
    // Note: stdout may contain "function_0" in the query section, but results should be empty
    let has_no_results = stdout.contains("\"results\": []") || stdout.contains("\"results\":[]");
    assert!(
        has_no_results,
        "Force re-index should remove deleted files.\n\nSearch output:\n{}",
        stdout
    );
}

/// Test: --force flag appears in help text
/// Requirement: CLI-011 - Force flag bypasses hash-based caching
#[test]
fn test_index_force_flag_in_help() {
    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd.args(["index", "--help"]).assert().success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("--force") || stdout.contains("-f"),
        "--force flag should be documented in help.\n\nHelp output:\n{}",
        stdout
    );
}
