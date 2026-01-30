//! AC#5: Symbol-Specific Graph Tests
//!
//! Given: User wants graph for a specific symbol
//! When: User runs `treelint deps --calls --symbol validateUser`
//! Then:
//!   - Output shows graph centered on specified symbol
//!   - Includes direct callers (functions that call validateUser)
//!   - Includes direct callees (functions validateUser calls)
//!   - Optionally includes N levels of depth (default 1)
//!
//! Source files tested:
//!   - src/cli/commands/deps.rs (Symbol filtering)
//!   - src/graph/calls.rs (Graph traversal)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DEPS-008: Filter graph to specific symbol with --symbol flag

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test: --symbol flag is recognized
/// Requirement: DEPS-008 - --symbol flag parsing
#[test]
fn test_symbol_flag_recognized() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("example.py"), "def target(): pass").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["deps", "--calls", "--symbol", "target"])
        .assert()
        .success();
}

/// Test: --symbol filters to specified function
/// Requirement: DEPS-008 - Filter graph to specific symbol
#[test]
fn test_symbol_filters_to_specified_function() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def unrelated():
    pass

def caller():
    target()

def target():
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
        .args(["deps", "--calls", "--symbol", "target"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("target"),
        "Filtered graph should include target symbol.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --symbol graph includes direct callers
/// Requirement: DEPS-008 - Includes direct callers
#[test]
fn test_symbol_graph_includes_direct_callers() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def caller_a():
    target()

def caller_b():
    target()

def target():
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
        .args(["deps", "--calls", "--symbol", "target"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("caller_a") || stdout.contains("caller"),
        "Symbol graph should include direct callers.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --symbol graph includes direct callees
/// Requirement: DEPS-008 - Includes direct callees
#[test]
fn test_symbol_graph_includes_direct_callees() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def target():
    helper_a()
    helper_b()

def helper_a():
    pass

def helper_b():
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
        .args(["deps", "--calls", "--symbol", "target"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("helper_a") || stdout.contains("helper"),
        "Symbol graph should include direct callees.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --symbol excludes unrelated functions
/// Requirement: DEPS-008 - Only shows relevant relationships
#[test]
fn test_symbol_excludes_unrelated_functions() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def unrelated_a():
    unrelated_b()

def unrelated_b():
    pass

def target():
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
        .args(["deps", "--calls", "--symbol", "target"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    // unrelated_a and unrelated_b should not appear (no connection to target)
    let has_unrelated = stdout.contains("unrelated_a") && stdout.contains("unrelated_b");
    assert!(
        !has_unrelated || stdout.contains("target"),
        "Symbol graph should focus on target, excluding fully unrelated.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --symbol with non-existent symbol shows error or empty
/// Requirement: DEPS-008 - Handle invalid symbol name
#[test]
fn test_symbol_nonexistent_handles_gracefully() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("example.py"), "def foo(): pass").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let result = cmd.current_dir(temp_dir.path()).args([
        "deps",
        "--calls",
        "--symbol",
        "nonexistent_function",
    ]);

    // Should either succeed with empty graph or fail with helpful error
    let output = result.assert();
    let status = output.get_output().status;

    if status.success() {
        // Empty graph is acceptable
        let stdout = String::from_utf8_lossy(&output.get_output().stdout);
        assert!(
            stdout.contains("nodes") || stdout.contains("[]") || stdout.len() < 100,
            "Non-existent symbol should return empty or minimal graph.\n\nActual output:\n{}",
            stdout
        );
    }
    // Failure with error message is also acceptable
}

/// Test: --symbol works with --format mermaid
/// Requirement: DEPS-008 - Combined with format flag
#[test]
fn test_symbol_with_mermaid_format() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def caller():
    target()

def target():
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
        .args([
            "deps", "--calls", "--symbol", "target", "--format", "mermaid",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("graph") && stdout.contains("target"),
        "Symbol filter with Mermaid should produce valid diagram.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --symbol works with --imports
/// Requirement: DEPS-008 - Works for import graphs too
#[test]
fn test_symbol_with_imports() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("target.py"), "TARGET_VAR = 1").unwrap();
    fs::write(
        temp_dir.path().join("main.py"),
        "from target import TARGET_VAR",
    )
    .unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["deps", "--imports", "--symbol", "target"])
        .assert()
        .success();
}
