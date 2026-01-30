//! AC#4: Mermaid Diagram Output Tests
//!
//! Given: User wants visual graph output
//! When: User runs `treelint deps --calls --format mermaid`
//! Then:
//!   - Mermaid output is valid diagram
//!   - Starts with `graph TD` (for calls) or `graph LR` (for imports)
//!   - Nodes include function/file name and path
//!   - Edges show relationship with optional count label
//!   - Output can be pasted into Mermaid-compatible renderer
//!
//! Source files tested:
//!   - src/output/graph.rs (MermaidFormatter)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DEPS-006: Generate valid Mermaid diagram syntax
//!   - DEPS-007: Include node labels with file paths

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test: Mermaid format flag is recognized
/// Requirement: DEPS-006 - Mermaid format parsing
#[test]
fn test_mermaid_format_flag_recognized() {
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
        .args(["deps", "--calls", "--format", "mermaid"])
        .assert()
        .success();
}

/// Test: Mermaid call graph starts with "graph TD"
/// Requirement: DEPS-006 - Starts with graph TD for calls
#[test]
fn test_mermaid_calls_starts_with_graph_td() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def caller(): callee()\ndef callee(): pass",
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
        .args(["deps", "--calls", "--format", "mermaid"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("graph TD") || stdout.contains("graph td"),
        "Mermaid call graph should start with 'graph TD'.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Mermaid import graph starts with "graph LR"
/// Requirement: DEPS-006 - Starts with graph LR for imports
#[test]
fn test_mermaid_imports_starts_with_graph_lr() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("utils.py"), "X = 1").unwrap();
    fs::write(temp_dir.path().join("main.py"), "from utils import X").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--imports", "--format", "mermaid"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("graph LR") || stdout.contains("graph lr"),
        "Mermaid import graph should start with 'graph LR'.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Mermaid nodes include function name
/// Requirement: DEPS-007 - Nodes include function name
#[test]
fn test_mermaid_nodes_include_function_name() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def my_function(): pass",
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
        .args(["deps", "--calls", "--format", "mermaid"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("my_function"),
        "Mermaid nodes should include function name.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Mermaid nodes include file path
/// Requirement: DEPS-007 - Include node labels with file paths
#[test]
fn test_mermaid_nodes_include_file_path() {
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
        .args(["deps", "--calls", "--format", "mermaid"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("example.py") || stdout.contains("example"),
        "Mermaid nodes should include file path.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Mermaid edges use arrow syntax
/// Requirement: DEPS-006 - Valid Mermaid edge syntax
#[test]
fn test_mermaid_edges_use_arrow_syntax() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def caller(): callee()\ndef callee(): pass",
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
        .args(["deps", "--calls", "--format", "mermaid"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("-->") || stdout.contains("->"),
        "Mermaid edges should use arrow syntax.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Mermaid call edges can show count label
/// Requirement: DEPS-006 - Edges with optional count label
#[test]
fn test_mermaid_call_edges_show_count() {
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
        .args(["deps", "--calls", "--format", "mermaid"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    // Edge label syntax: -->|label|
    assert!(
        stdout.contains("|") || stdout.contains("3") || stdout.contains("count"),
        "Mermaid call edges should show count label.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Mermaid output has no syntax errors (basic validation)
/// Requirement: DEPS-006 - Output can be pasted into Mermaid renderer
#[test]
fn test_mermaid_output_valid_syntax() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def caller(): callee()\ndef callee(): pass",
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
        .args(["deps", "--calls", "--format", "mermaid"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    // Basic validation: should start with graph and have at least one node or edge
    assert!(
        stdout.starts_with("graph") || stdout.trim().starts_with("graph"),
        "Mermaid output should start with 'graph'.\n\nActual output:\n{}",
        stdout
    );
    assert!(
        stdout.lines().count() >= 1,
        "Mermaid output should have content.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Invalid format value shows error
/// Requirement: DEPS-006 - Format validation
#[test]
fn test_invalid_format_shows_error() {
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
        .args(["deps", "--calls", "--format", "invalid"])
        .assert()
        .failure();
}
