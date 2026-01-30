//! AC#1: Call Graph Command Tests
//!
//! Given: A repository with indexed symbols containing function calls
//! When: User runs `treelint deps --calls`
//! Then:
//!   - Output shows function call relationships
//!   - Nodes: all functions/methods with their file paths
//!   - Edges: caller -> callee relationships with call count
//!   - Default format is JSON
//!
//! Source files tested:
//!   - src/cli/commands/deps.rs (DepsCommand implementation)
//!   - src/graph/calls.rs (CallGraphExtractor)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DEPS-001: Execute call graph extraction with --calls flag
//!   - DEPS-002: Output call graph with nodes and edges

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test: `treelint deps` subcommand is recognized
/// Requirement: DEPS-001 - Deps subcommand parsing
#[test]
fn test_deps_subcommand_recognized() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["deps", "--help"])
        .assert()
        .stdout(predicate::str::contains("deps").or(predicate::str::contains("Deps")));
}

/// Test: `treelint deps --calls` flag is recognized
/// Requirement: DEPS-001 - --calls flag parsing
#[test]
fn test_deps_calls_flag_recognized() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("example.py"), "def foo(): pass").unwrap();

    // Build index first
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act: Run deps --calls
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();
}

/// Test: `treelint deps --calls` returns JSON with nodes array
/// Requirement: DEPS-002 - Output call graph with nodes
#[test]
fn test_deps_calls_output_includes_nodes() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def caller():
    callee()

def callee():
    pass
"#,
    )
    .unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("nodes") || stdout.contains("\"nodes\""),
        "Call graph output should include nodes array.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: `treelint deps --calls` returns JSON with edges array
/// Requirement: DEPS-002 - Output call graph with edges
#[test]
fn test_deps_calls_output_includes_edges() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def caller():
    callee()

def callee():
    pass
"#,
    )
    .unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("edges") || stdout.contains("\"edges\""),
        "Call graph output should include edges array.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Call graph edge shows caller to callee relationship
/// Requirement: DEPS-002 - Edges: caller -> callee relationships
#[test]
fn test_deps_calls_edge_shows_caller_callee() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def caller():
    callee()

def callee():
    pass
"#,
    )
    .unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    // Edge should show caller -> callee
    assert!(
        stdout.contains("caller") && stdout.contains("callee"),
        "Call graph should show caller and callee.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Call graph edge includes call count
/// Requirement: DEPS-002 - Edges with call count
#[test]
fn test_deps_calls_edge_includes_count() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def caller():
    helper()
    helper()
    helper()

def helper():
    pass
"#,
    )
    .unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    // Edge should include count field (3 calls to helper)
    assert!(
        stdout.contains("count") || stdout.contains("\"count\""),
        "Call graph edges should include count.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Call graph nodes include file path
/// Requirement: DEPS-002 - Nodes: functions with file paths
#[test]
fn test_deps_calls_nodes_include_file_path() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("example.py"), "def my_func(): pass").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("example.py") || stdout.contains("file"),
        "Call graph nodes should include file path.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Call graph defaults to JSON format
/// Requirement: DEPS-002 - Default format is JSON
#[test]
fn test_deps_calls_defaults_to_json() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("example.py"), "def foo(): pass").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let trimmed = stdout.trim();
    assert!(
        trimmed.starts_with('{') || trimmed.starts_with('['),
        "Default deps --calls output should be JSON.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: BR-001 - Error when neither --calls nor --imports specified
/// Requirement: BR-001 - At least one of --calls or --imports must be specified
#[test]
fn test_deps_without_flags_shows_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("example.py"), "def foo(): pass").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["deps"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--calls").or(predicate::str::contains("--imports")));
}
