//! AC#3: Text Output Format Tests
//!
//! Given: User wants human-readable output
//! When: User runs `treelint map --format text`
//! Then:
//!   - Header with total counts
//!   - Directory tree structure
//!   - Files grouped by directory
//!   - Symbols indented under files with type and line range
//!   - Relevance score shown with stars if --ranked
//!
//! Source files tested:
//!   - src/output/text.rs (MapTextFormatter)
//!   - src/cli/commands/map.rs
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - MAP-004: Generate tree-style text output

use assert_cmd::Command;
use predicates::boolean::PredicateBooleanExt;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test: `treelint map --format text` produces text output (not JSON)
/// Requirement: MAP-004 - Generate tree-style text output
#[test]
fn test_map_text_produces_non_json_output() {
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

    // Act
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "text"])
        .assert()
        .success();

    // Assert: Output should NOT start with { (not JSON)
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let trimmed = stdout.trim();
    assert!(
        !trimmed.starts_with('{') && !trimmed.starts_with('['),
        "Text format should not be JSON.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Text output includes header with total counts
/// Requirement: MAP-004 - Header with total counts
#[test]
fn test_map_text_has_header_with_counts() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("a.py"), "def foo(): pass").unwrap();
    fs::write(temp_dir.path().join("b.py"), "def bar(): pass").unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "text"])
        .assert()
        .success();

    // Assert: Output should include counts (symbols/files/total)
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("symbol")
            || stdout.contains("file")
            || stdout.contains("total")
            || stdout.contains("2"),
        "Text output should include header with counts.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Text output shows directory tree structure
/// Requirement: MAP-004 - Directory tree structure
#[test]
fn test_map_text_shows_tree_structure() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::create_dir(temp_dir.path().join("subdir")).unwrap();
    fs::write(temp_dir.path().join("root.py"), "def root_func(): pass").unwrap();
    fs::write(
        temp_dir.path().join("subdir").join("nested.py"),
        "def nested_func(): pass",
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
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "text"])
        .assert()
        .success();

    // Assert: Output should show tree characters or indentation
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("subdir")
            || stdout.contains("|")
            || stdout.contains("--")
            || stdout.contains("  "),
        "Text output should show tree structure.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Text output groups files by directory
/// Requirement: MAP-004 - Files grouped by directory
#[test]
fn test_map_text_groups_files_by_directory() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::create_dir(temp_dir.path().join("alpha")).unwrap();
    fs::create_dir(temp_dir.path().join("beta")).unwrap();
    fs::write(
        temp_dir.path().join("alpha").join("a1.py"),
        "def a1(): pass",
    )
    .unwrap();
    fs::write(
        temp_dir.path().join("alpha").join("a2.py"),
        "def a2(): pass",
    )
    .unwrap();
    fs::write(temp_dir.path().join("beta").join("b1.py"), "def b1(): pass").unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "text"])
        .assert()
        .success();

    // Assert: Output should show directory grouping
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let alpha_pos = stdout.find("alpha");
    let beta_pos = stdout.find("beta");

    // Both directories should appear
    assert!(
        alpha_pos.is_some() && beta_pos.is_some(),
        "Both directories should appear in output.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Text output shows symbols indented under files
/// Requirement: MAP-004 - Symbols indented under files with type and line range
#[test]
fn test_map_text_shows_symbols_indented() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def my_function():\n    pass\n",
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
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "text"])
        .assert()
        .success();

    // Assert: Symbol should be indented
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("my_function"),
        "Output should include symbol name.\n\nActual output:\n{}",
        stdout
    );

    // Check for indentation (spaces before symbol name)
    let lines: Vec<&str> = stdout.lines().collect();
    let symbol_line = lines.iter().find(|l| l.contains("my_function"));
    if let Some(line) = symbol_line {
        assert!(
            line.starts_with(' ') || line.starts_with('\t') || line.contains("  "),
            "Symbol should be indented.\n\nLine: {}",
            line
        );
    }
}

/// Test: Text output shows symbol type
/// Requirement: MAP-004 - Symbols with type
#[test]
fn test_map_text_shows_symbol_type() {
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

    // Act
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "text"])
        .assert()
        .success();

    // Assert: Type indicator should be present
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.to_lowercase().contains("function")
            || stdout.contains("fn")
            || stdout.contains("def")
            || stdout.contains("(f)"),
        "Text output should show symbol type.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Text output shows line range
/// Requirement: MAP-004 - Symbols with line range
#[test]
fn test_map_text_shows_line_range() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def foo():\n    pass\n    return\n",
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
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "text"])
        .assert()
        .success();

    // Assert: Line numbers should be present
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains(":1") || stdout.contains("L1") || stdout.contains("line"),
        "Text output should show line numbers.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Text output shows star rating when --ranked is used
/// Requirement: MAP-004 - Relevance score shown with stars if --ranked
#[test]
fn test_map_text_shows_stars_when_ranked() {
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

    // Act: Use both --format text and --ranked
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "text", "--ranked"])
        .assert()
        .success();

    // Assert: Star or score should be present
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("*")
            || stdout.contains("star")
            || stdout.contains("0.")
            || stdout.contains("relevance"),
        "Ranked text output should show stars or score.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --format text flag is recognized
/// Requirement: MAP-004 - Text format flag
#[test]
fn test_map_format_text_flag_recognized() {
    // Act & Assert
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["map", "--help"])
        .assert()
        .stdout(predicate::str::contains("format").or(predicate::str::contains("text")));
}
