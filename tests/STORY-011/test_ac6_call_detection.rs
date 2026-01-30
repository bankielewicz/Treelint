//! AC#6: Function Call Detection Tests
//!
//! Given: Source files with function calls
//! When: Call graph extraction runs
//! Then:
//!   - Detects function calls via tree-sitter:
//!     - Python: call_expression nodes
//!     - TypeScript: call_expression nodes
//!     - Rust: call_expression, method_call_expression nodes
//!   - Resolves callee to indexed symbol when possible
//!   - Records call count (how many times A calls B)
//!
//! Source files tested:
//!   - src/graph/calls.rs (CallGraphExtractor)
//!   - src/parser/queries/python.rs
//!   - src/parser/queries/typescript.rs
//!   - src/parser/queries/rust.rs
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DEPS-009: Detect function calls via tree-sitter queries
//!   - DEPS-010: Track call count per edge

use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

/// Test: Python function calls detected
/// Requirement: DEPS-009 - Python call_expression nodes
#[test]
fn test_python_function_calls_detected() {
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
        stdout.contains("caller") && stdout.contains("callee"),
        "Python call from caller to callee should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Python method calls detected
/// Requirement: DEPS-009 - Python method calls
#[test]
fn test_python_method_calls_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
class MyClass:
    def method_a(self):
        self.method_b()

    def method_b(self):
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
        stdout.contains("method_a") || stdout.contains("method_b"),
        "Python method calls should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: TypeScript function calls detected
/// Requirement: DEPS-009 - TypeScript call_expression nodes
#[test]
fn test_typescript_function_calls_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.ts"),
        r#"
function caller(): void {
    callee();
}

function callee(): void {
    // do nothing
}
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
        stdout.contains("caller") && stdout.contains("callee"),
        "TypeScript call should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Rust function calls detected
/// Requirement: DEPS-009 - Rust call_expression nodes
#[test]
fn test_rust_function_calls_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.rs"),
        r#"
fn caller() {
    callee();
}

fn callee() {
    // do nothing
}
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
        stdout.contains("caller") && stdout.contains("callee"),
        "Rust call should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Rust method calls detected
/// Requirement: DEPS-009 - Rust method_call_expression nodes
#[test]
fn test_rust_method_calls_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.rs"),
        r#"
struct MyStruct;

impl MyStruct {
    fn method_a(&self) {
        self.method_b();
    }

    fn method_b(&self) {}
}
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
        stdout.contains("method_a") || stdout.contains("method_b"),
        "Rust method calls should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Call count tracked accurately
/// Requirement: DEPS-010 - Track call count per edge
#[test]
fn test_call_count_tracked() {
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
        .args(["deps", "--calls", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    // Should show count of 3 for caller->helper edge
    assert!(
        stdout.contains("3") || stdout.contains("\"count\":3"),
        "Call count should be tracked (expected 3).\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Recursive calls detected
/// Requirement: DEPS-009 - Self-calling function (recursion)
#[test]
fn test_recursive_calls_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)
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
    // Recursive call: factorial -> factorial
    assert!(
        stdout.contains("factorial"),
        "Recursive call should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Cross-file calls detected
/// Requirement: DEPS-009 - Calls across files
#[test]
fn test_cross_file_calls_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("utils.py"), "def helper(): pass").unwrap();
    fs::write(
        temp_dir.path().join("main.py"),
        r#"
from utils import helper

def main():
    helper()
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
        stdout.contains("main") && stdout.contains("helper"),
        "Cross-file call main->helper should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: BR-002 - Unresolved calls (external libraries) excluded
/// Requirement: BR-002 - External library calls not in graph
#[test]
fn test_external_library_calls_excluded() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
import os

def my_func():
    os.path.join("a", "b")  # External library call
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
    // os.path.join should NOT appear as an edge (unresolved)
    assert!(
        !stdout.contains("os.path.join") && !stdout.contains("join"),
        "External library calls should be excluded.\n\nActual output:\n{}",
        stdout
    );
}
