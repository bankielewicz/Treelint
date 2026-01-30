//! AC#3: JSON Graph Output Tests
//!
//! Given: User wants machine-readable graph output
//! When: User runs `treelint deps --calls --format json`
//! Then:
//!   - JSON output matches schema:
//!   - `graph_type`: "calls" or "imports"
//!   - `nodes`: array of {id, file, type}
//!   - `edges`: array of {from, to, count (for calls)}
//!
//! Source files tested:
//!   - src/output/json.rs (GraphJsonFormatter)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DEPS-005: Generate JSON matching specified schema

use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

/// Test: JSON output has graph_type field
/// Requirement: DEPS-005 - JSON has graph_type
#[test]
fn test_json_output_has_graph_type() {
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
        .args(["deps", "--calls", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("graph_type") || stdout.contains("\"graph_type\""),
        "JSON output should have graph_type field.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON graph_type is "calls" for call graph
/// Requirement: DEPS-005 - graph_type: "calls"
#[test]
fn test_json_graph_type_is_calls() {
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
        .args(["deps", "--calls", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("\"calls\"") || stdout.contains("\"graph_type\":\"calls\""),
        "JSON graph_type should be 'calls'.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON graph_type is "imports" for import graph
/// Requirement: DEPS-005 - graph_type: "imports"
#[test]
fn test_json_graph_type_is_imports() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("main.py"), "import os").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--imports", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("\"imports\"") || stdout.contains("\"graph_type\":\"imports\""),
        "JSON graph_type should be 'imports'.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON nodes have id field
/// Requirement: DEPS-005 - nodes array with id
#[test]
fn test_json_nodes_have_id() {
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
        .args(["deps", "--calls", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("\"id\"") || stdout.contains("\"id\":"),
        "JSON nodes should have id field.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON nodes have file field
/// Requirement: DEPS-005 - nodes array with file
#[test]
fn test_json_nodes_have_file() {
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
        .args(["deps", "--calls", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("\"file\"") || stdout.contains("example.py"),
        "JSON nodes should have file field.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON nodes have type field
/// Requirement: DEPS-005 - nodes array with type
#[test]
fn test_json_nodes_have_type() {
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
        .args(["deps", "--calls", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("\"type\"") || stdout.contains("function") || stdout.contains("Function"),
        "JSON nodes should have type field.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON edges have from field
/// Requirement: DEPS-005 - edges array with from
#[test]
fn test_json_edges_have_from() {
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
        .args(["deps", "--calls", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("\"from\"") || stdout.contains("\"from\":"),
        "JSON edges should have from field.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON edges have to field
/// Requirement: DEPS-005 - edges array with to
#[test]
fn test_json_edges_have_to() {
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
        .args(["deps", "--calls", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("\"to\"") || stdout.contains("\"to\":"),
        "JSON edges should have to field.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON call edges have count field
/// Requirement: DEPS-005 - edges array with count (for calls)
#[test]
fn test_json_call_edges_have_count() {
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
        .args(["deps", "--calls", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("\"count\"") || stdout.contains("\"count\":"),
        "JSON call edges should have count field.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON is valid and parseable
/// Requirement: DEPS-005 - Valid JSON output
#[test]
fn test_json_output_is_valid() {
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
        .args(["deps", "--calls", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let result: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(
        result.is_ok(),
        "Output should be valid JSON.\n\nActual output:\n{}\n\nError: {:?}",
        stdout,
        result.err()
    );
}
