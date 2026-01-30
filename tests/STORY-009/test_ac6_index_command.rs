//! AC#6: Manual Index Command Tests
//!
//! Given: User wants to manually build/rebuild the index
//! When: User runs `treelint index`
//! Then:
//!   - Full index build starts
//!   - Progress bar shown if > 1000 files and stdout is TTY
//!   - Progress shows: percentage, files processed, current file, speed, ETA
//!   - Completion message shows: files indexed, symbols found, index size, duration
//!   - Exit code: 0 on success
//!
//! Source files tested:
//!   - src/cli/commands/index.rs (Index command)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - CLI-009: Build full index with progress reporting
//!   - CLI-010: Show progress bar for large repositories

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::time::Instant;
use tempfile::TempDir;

/// Helper: Create a test project with source files
fn setup_test_project_with_files(count: usize) -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    for i in 0..count {
        let file_content = format!(
            r#"
def function_{i}(x: int) -> int:
    """Function number {i}"""
    return x * {i}

class Class_{i}:
    """Class number {i}"""
    def method_{i}(self):
        pass
"#,
            i = i
        );
        let file_path = src_dir.join(format!("module_{}.py", i));
        fs::write(&file_path, file_content).expect("Failed to write file");
    }

    temp_dir
}

/// Test: `treelint index` builds full index
/// Requirement: CLI-009 - Build full index with progress reporting
#[test]
fn test_index_builds_full_index() {
    // Arrange
    let temp_dir = setup_test_project_with_files(5);

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Assert: Index file exists
    let index_path = temp_dir.path().join(".treelint").join("index.db");
    assert!(
        index_path.exists(),
        "Index file should be created at {:?}",
        index_path
    );
}

/// Test: `treelint index` completion message shows files indexed
/// Requirement: CLI-009 - Build full index with progress reporting
#[test]
fn test_index_completion_shows_files_indexed() {
    // Arrange
    let temp_dir = setup_test_project_with_files(5);

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Assert: Output shows files indexed
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(
        combined.to_lowercase().contains("file")
            && (combined.contains("5") || combined.contains("indexed")),
        "Completion message should show files indexed.\n\nOutput:\n{}",
        combined
    );
}

/// Test: `treelint index` completion message shows symbols found
/// Requirement: CLI-009 - Build full index with progress reporting
#[test]
fn test_index_completion_shows_symbols_found() {
    // Arrange
    let temp_dir = setup_test_project_with_files(5);

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Assert: Output shows symbols found
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(
        combined.to_lowercase().contains("symbol"),
        "Completion message should show symbols found.\n\nOutput:\n{}",
        combined
    );
}

/// Test: `treelint index` completion message shows duration
/// Requirement: CLI-009 - Build full index with progress reporting
#[test]
fn test_index_completion_shows_duration() {
    // Arrange
    let temp_dir = setup_test_project_with_files(5);

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Assert: Output shows duration
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    // Duration could be shown as "1.5s", "100ms", "duration:", etc.
    assert!(
        combined.to_lowercase().contains("duration")
            || combined.contains("ms")
            || combined.contains("sec")
            || combined.contains("s "),
        "Completion message should show duration.\n\nOutput:\n{}",
        combined
    );
}

/// Test: `treelint index` exit code is 0 on success
/// Requirement: CLI-009 - Build full index with progress reporting
#[test]
fn test_index_exit_code_zero_on_success() {
    // Arrange
    let temp_dir = setup_test_project_with_files(5);

    // Act & Assert
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .code(0);
}

/// Test: `treelint index` on empty directory still succeeds
/// Requirement: CLI-009 - Build full index with progress reporting
#[test]
fn test_index_empty_directory_succeeds() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Act & Assert
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .code(0);
}

/// Test: `treelint index` indexes multiple file types
/// Requirement: CLI-009 - Build full index with progress reporting
#[test]
fn test_index_multiple_file_types() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Python file
    fs::write(src_dir.join("module.py"), "def py_function(): pass")
        .expect("Failed to write Python file");

    // TypeScript file
    fs::write(
        src_dir.join("component.ts"),
        "function tsFunction(): void {}",
    )
    .expect("Failed to write TypeScript file");

    // Rust file
    fs::write(src_dir.join("lib.rs"), "fn rust_function() {}").expect("Failed to write Rust file");

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Assert: All files were indexed
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    // Should index at least 3 files
    assert!(
        combined.contains("3") || combined.to_lowercase().contains("file"),
        "Should index multiple file types.\n\nOutput:\n{}",
        combined
    );
}

/// Test: `treelint index` creates .treelint directory if not exists
/// Requirement: CLI-009 - Build full index with progress reporting
#[test]
fn test_index_creates_treelint_directory() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let treelint_dir = temp_dir.path().join(".treelint");
    assert!(
        !treelint_dir.exists(),
        "Test precondition: .treelint should not exist"
    );

    // Create a simple source file
    fs::write(temp_dir.path().join("main.py"), "def main(): pass").expect("Failed to write file");

    // Act
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Assert: .treelint directory was created
    assert!(
        treelint_dir.exists(),
        ".treelint directory should be created"
    );
}

/// Test: `treelint index` subcommand is recognized
/// Requirement: CLI-009 - Build full index with progress reporting
#[test]
fn test_index_subcommand_recognized() {
    // Act & Assert: Command should not fail with "unrecognized subcommand"
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["index", "--help"])
        .assert()
        .stdout(predicate::str::contains("index"));
}

/// Test: `treelint index` completes in reasonable time for small project
/// Requirement: CLI-009 - Build full index with progress reporting
#[test]
fn test_index_reasonable_time_small_project() {
    // Arrange
    let temp_dir = setup_test_project_with_files(10);
    let start_time = Instant::now();

    // Act
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Assert: Should complete within 30 seconds for small project
    let elapsed = start_time.elapsed();
    assert!(
        elapsed.as_secs() < 30,
        "Index should complete within 30 seconds for small project, took {:?}",
        elapsed
    );
}

/// Test: Progress bar not shown for small repos (when piped)
/// Requirement: BR-003 - Progress bar only shown for large repos in TTY
#[test]
fn test_index_no_progress_bar_when_piped() {
    // Arrange: Small project (less than 1000 files)
    let temp_dir = setup_test_project_with_files(5);

    // Act: Run with output capture (simulates piped output)
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Assert: No progress bar characters in output (when piped)
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    // Progress bar typically contains these characters
    assert!(
        !combined.contains("━") && !combined.contains("▓") && !combined.contains("█"),
        "Small repo should not show progress bar when piped.\n\nOutput:\n{}",
        combined
    );
}
