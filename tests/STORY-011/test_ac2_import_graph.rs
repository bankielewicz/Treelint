//! AC#2: Import Graph Command Tests
//!
//! Given: A repository with files that import/export modules
//! When: User runs `treelint deps --imports`
//! Then:
//!   - Output shows import relationships
//!   - Nodes: all files/modules
//!   - Edges: importer -> imported module relationships
//!   - Default format is JSON
//!
//! Source files tested:
//!   - src/cli/commands/deps.rs (DepsCommand implementation)
//!   - src/graph/imports.rs (ImportGraphExtractor)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DEPS-003: Execute import graph extraction with --imports flag
//!   - DEPS-004: Output import graph with file nodes and edges

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test: `treelint deps --imports` flag is recognized
/// Requirement: DEPS-003 - --imports flag parsing
#[test]
fn test_deps_imports_flag_recognized() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("main.py"), "import os").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["deps", "--imports"])
        .assert()
        .success();
}

/// Test: `treelint deps --imports` returns JSON with nodes array
/// Requirement: DEPS-004 - Output import graph with nodes
#[test]
fn test_deps_imports_output_includes_nodes() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("utils.py"), "def helper(): pass").unwrap();
    fs::write(temp_dir.path().join("main.py"), "from utils import helper").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--imports"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("nodes") || stdout.contains("\"nodes\""),
        "Import graph output should include nodes array.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: `treelint deps --imports` returns JSON with edges array
/// Requirement: DEPS-004 - Output import graph with edges
#[test]
fn test_deps_imports_output_includes_edges() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("utils.py"), "def helper(): pass").unwrap();
    fs::write(temp_dir.path().join("main.py"), "from utils import helper").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--imports"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("edges") || stdout.contains("\"edges\""),
        "Import graph output should include edges array.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Import graph edge shows importer to imported relationship
/// Requirement: DEPS-004 - Edges: importer -> imported module
#[test]
fn test_deps_imports_edge_shows_importer_imported() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("utils.py"), "def helper(): pass").unwrap();
    fs::write(temp_dir.path().join("main.py"), "from utils import helper").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--imports"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("main") && stdout.contains("utils"),
        "Import graph should show main importing utils.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Import graph nodes include file/module names
/// Requirement: DEPS-004 - Nodes: all files/modules
#[test]
fn test_deps_imports_nodes_include_file_names() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("module_a.py"), "X = 1").unwrap();
    fs::write(
        temp_dir.path().join("module_b.py"),
        "from module_a import X",
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
        .args(["deps", "--imports"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("module_a") || stdout.contains("module_b"),
        "Import graph nodes should include module names.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Import graph defaults to JSON format
/// Requirement: DEPS-004 - Default format is JSON
#[test]
fn test_deps_imports_defaults_to_json() {
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
        .args(["deps", "--imports"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let trimmed = stdout.trim();
    assert!(
        trimmed.starts_with('{') || trimmed.starts_with('['),
        "Default deps --imports output should be JSON.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Both --calls and --imports can be specified together
/// Requirement: DEPS-003, DEPS-004 - Combined usage
#[test]
fn test_deps_calls_and_imports_combined() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("main.py"),
        r#"
from utils import helper

def main():
    helper()
"#,
    )
    .unwrap();
    fs::write(temp_dir.path().join("utils.py"), "def helper(): pass").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["deps", "--calls", "--imports"])
        .assert()
        .success();
}
