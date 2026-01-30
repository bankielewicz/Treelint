//! AC#1: Map Command Basic Output Tests
//!
//! Given: A repository with indexed symbols
//! When: User runs `treelint map`
//! Then:
//!   - Output includes total symbol count and file count
//!   - All symbols organized by file
//!   - Each symbol shows: name, type, line range
//!   - Default format is JSON (if piped) or text (if TTY)
//!
//! Source files tested:
//!   - src/cli/commands/map.rs (MapCommand implementation)
//!   - src/cli/args.rs (MapArgs)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - MAP-001: Generate comprehensive symbol listing from index
//!   - MAP-002: Auto-detect output format based on TTY

use assert_cmd::Command;
use predicates::boolean::PredicateBooleanExt;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test: `treelint map` command exists and is recognized
/// Requirement: MAP-001 - Map subcommand parsing
#[test]
fn test_map_subcommand_recognized() {
    // Act & Assert: Command should not fail with "unrecognized subcommand"
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["map", "--help"])
        .assert()
        .stdout(predicate::str::contains("map").or(predicate::str::contains("Map")));
}

/// Test: `treelint map` returns success when repository has indexed symbols
/// Requirement: MAP-001 - Generate comprehensive symbol listing from index
#[test]
fn test_map_returns_success_with_indexed_repo() {
    // Arrange: Create temp dir with Python file and build index
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let py_file = temp_dir.path().join("example.py");
    fs::write(
        &py_file,
        r#"
def hello():
    """A simple function."""
    pass

class Greeter:
    """A simple class."""
    def greet(self, name):
        return f"Hello, {name}"
"#,
    )
    .expect("Failed to write test file");

    // Build index first
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act: Run map command
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd.current_dir(temp_dir.path()).args(["map"]).assert();

    // Assert: Command succeeds
    output.success();
}

/// Test: `treelint map` output includes total symbol count
/// Requirement: MAP-001 - Generate comprehensive symbol listing from index
#[test]
fn test_map_output_includes_total_symbol_count() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let py_file = temp_dir.path().join("example.py");
    fs::write(
        &py_file,
        r#"
def func1():
    pass

def func2():
    pass

class MyClass:
    pass
"#,
    )
    .expect("Failed to write test file");

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert: Output contains symbol count (should have at least 3 symbols)
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("total_symbols") || stdout.contains("\"symbols\""),
        "Map output should include symbol count.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: `treelint map` output includes file count
/// Requirement: MAP-001 - Generate comprehensive symbol listing from index
#[test]
fn test_map_output_includes_file_count() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("file1.py"), "def foo(): pass").unwrap();
    fs::write(temp_dir.path().join("file2.py"), "def bar(): pass").unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert: Output contains file count
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("total_files") || stdout.contains("\"files\""),
        "Map output should include file count.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: `treelint map` shows symbols organized by file
/// Requirement: MAP-001 - Generate comprehensive symbol listing from index
#[test]
fn test_map_shows_symbols_by_file() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("alpha.py"), "def alpha_func(): pass").unwrap();
    fs::write(temp_dir.path().join("beta.py"), "def beta_func(): pass").unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert: Output shows by_file structure
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("by_file") || (stdout.contains("alpha.py") && stdout.contains("beta.py")),
        "Map output should organize symbols by file.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: `treelint map` shows symbol name, type, and line range
/// Requirement: MAP-001 - Each symbol shows: name, type, line range
#[test]
fn test_map_shows_symbol_details() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"def my_function():
    """Line 1-2"""
    pass
"#,
    )
    .unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert: Output shows symbol name, type, and lines
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("my_function"),
        "Map output should include symbol name.\n\nActual output:\n{}",
        stdout
    );
    assert!(
        stdout.contains("function") || stdout.contains("Function"),
        "Map output should include symbol type.\n\nActual output:\n{}",
        stdout
    );
    assert!(
        stdout.contains("lines") || stdout.contains("line"),
        "Map output should include line range.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: `treelint map` defaults to JSON when piped
/// Requirement: MAP-002 - Auto-detect output format based on TTY
#[test]
fn test_map_defaults_json_when_piped() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("example.py"), "def foo(): pass").unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act: Run without --format (piped, so should be JSON)
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["map"])
        .assert()
        .success();

    // Assert: Output is valid JSON (starts with { or [)
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let trimmed = stdout.trim();
    assert!(
        trimmed.starts_with('{') || trimmed.starts_with('['),
        "Piped map output should default to JSON.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: `treelint map` exit code is 0 on success
/// Requirement: MAP-001 - Generate comprehensive symbol listing from index
#[test]
fn test_map_exit_code_zero_on_success() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("example.py"), "def foo(): pass").unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act & Assert
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map"])
        .assert()
        .code(0);
}

/// Test: `treelint map` handles empty repository gracefully
/// Requirement: MAP-001 - Edge case handling
#[test]
fn test_map_empty_repository() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Build empty index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert: Output shows zero counts
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("0") || stdout.contains("\"total_symbols\":0"),
        "Empty repo map should show zero symbols.\n\nActual output:\n{}",
        stdout
    );
}
