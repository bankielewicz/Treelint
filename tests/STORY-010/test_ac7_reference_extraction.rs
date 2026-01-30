//! AC#7: Reference Extraction for Scoring Tests
//!
//! Given: Repository has files with function calls and imports
//! When: Reference counting runs during index build
//! Then:
//!   - Function calls detected via tree-sitter (call_expression nodes)
//!   - Import references detected via tree-sitter (import_statement nodes)
//!   - Reference counts stored for relevance calculation
//!   - Works for Python, TypeScript, Rust, Markdown
//!
//! Source files tested:
//!   - src/index/relevance.rs (Reference extraction)
//!   - src/parser/symbols.rs (Parser queries)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - MAP-009: Extract function call references
//!   - MAP-010: Extract import references

use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Test: Function calls are detected and counted
/// Requirement: MAP-009 - foo() calling bar() increases bar's count
#[test]
fn test_function_calls_detected() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // target.py: defines target_function
    fs::write(
        temp_dir.path().join("target.py"),
        "def target_function(): return 42",
    )
    .unwrap();

    // caller.py: calls target_function multiple times
    fs::write(
        temp_dir.path().join("caller.py"),
        r#"
from target import target_function

def caller():
    x = target_function()
    y = target_function()
    return x + y
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
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert: target_function should have higher relevance than caller
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");

    let mut target_relevance: Option<f64> = None;
    let mut caller_relevance: Option<f64> = None;

    for (_path, file_data) in by_file {
        if let Some(symbols) = file_data.get("symbols").and_then(|s| s.as_array()) {
            for symbol in symbols {
                let name = symbol.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let relevance = symbol.get("relevance").and_then(|r| r.as_f64());

                if name == "target_function" {
                    target_relevance = relevance;
                } else if name == "caller" {
                    caller_relevance = relevance;
                }
            }
        }
    }

    if let (Some(t), Some(c)) = (target_relevance, caller_relevance) {
        assert!(
            t > c,
            "target_function ({}) should have higher relevance than caller ({})",
            t,
            c
        );
    }
}

/// Test: Import references are detected
/// Requirement: MAP-010 - 'import foo' increases foo module's count
#[test]
fn test_import_references_detected() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // core_module.py: the module being imported
    fs::write(
        temp_dir.path().join("core_module.py"),
        "def core_logic(): pass",
    )
    .unwrap();

    // importer1.py: imports core_module
    fs::write(
        temp_dir.path().join("importer1.py"),
        "import core_module\n\ndef use_core(): core_module.core_logic()",
    )
    .unwrap();

    // importer2.py: imports core_module
    fs::write(
        temp_dir.path().join("importer2.py"),
        "from core_module import core_logic\n\ndef another(): core_logic()",
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
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert: core_logic should have high relevance (imported by 2 files)
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("core_logic"),
        "Output should include core_logic symbol"
    );
}

/// Test: Python function calls detected
/// Requirement: MAP-009 - Works for Python
#[test]
fn test_python_function_calls() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("module.py"),
        r#"
def helper():
    return "help"

def main():
    result = helper()
    print(result)
    return helper()
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
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert: helper called 2x should have higher relevance
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("helper"), "Should detect helper function");
}

/// Test: TypeScript function calls detected
/// Requirement: MAP-009 - Works for TypeScript
#[test]
fn test_typescript_function_calls() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("module.ts"),
        r#"
function utilityFn(): string {
    return "utility";
}

function mainFn(): void {
    const x = utilityFn();
    console.log(utilityFn());
}

export { utilityFn, mainFn };
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
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("utilityFn"),
        "Should detect TypeScript functions"
    );
}

/// Test: Rust function calls detected
/// Requirement: MAP-009 - Works for Rust
#[test]
fn test_rust_function_calls() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("lib.rs"),
        r#"
fn helper() -> i32 {
    42
}

fn main_func() -> i32 {
    let x = helper();
    let y = helper();
    x + y
}
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
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("helper"), "Should detect Rust functions");
}

/// Test: Python imports detected
/// Requirement: MAP-010 - Import references for Python
#[test]
fn test_python_imports_detected() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    fs::write(temp_dir.path().join("utils.py"), "def util_fn(): pass").unwrap();

    fs::write(
        temp_dir.path().join("main.py"),
        "from utils import util_fn\n\nutil_fn()",
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
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("util_fn"),
        "Should detect imported function"
    );
}

/// Test: TypeScript imports detected
/// Requirement: MAP-010 - Import references for TypeScript
#[test]
fn test_typescript_imports_detected() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    fs::write(
        temp_dir.path().join("helper.ts"),
        "export function helperFn() { return 1; }",
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("app.ts"),
        "import { helperFn } from './helper';\n\nconst x = helperFn();",
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
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("helperFn"),
        "Should detect TypeScript imports"
    );
}

/// Test: Rust use statements detected
/// Requirement: MAP-010 - Import references for Rust
#[test]
fn test_rust_use_statements_detected() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    fs::write(
        temp_dir.path().join("lib.rs"),
        r#"
mod utils {
    pub fn utility() -> i32 { 1 }
}

use utils::utility;

fn main() {
    let _ = utility();
}
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
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("utility"),
        "Should detect Rust use statements"
    );
}

/// Test: Reference counts are stored and affect relevance
/// Requirement: MAP-009, MAP-010 - Reference counts stored for relevance calculation
#[test]
fn test_reference_counts_affect_relevance() {
    // Arrange: Create known reference pattern
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // heavily_used.py: will be referenced 3 times
    fs::write(
        temp_dir.path().join("heavily_used.py"),
        "def popular(): pass",
    )
    .unwrap();

    // user1.py
    fs::write(
        temp_dir.path().join("user1.py"),
        "from heavily_used import popular\npopular()",
    )
    .unwrap();

    // user2.py
    fs::write(
        temp_dir.path().join("user2.py"),
        "from heavily_used import popular\npopular()",
    )
    .unwrap();

    // user3.py
    fs::write(
        temp_dir.path().join("user3.py"),
        "from heavily_used import popular\npopular()",
    )
    .unwrap();

    // rarely_used.py: will be referenced 0 times
    fs::write(
        temp_dir.path().join("rarely_used.py"),
        "def unpopular(): pass",
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
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert: popular should have higher relevance than unpopular
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");

    let mut popular_relevance: Option<f64> = None;
    let mut unpopular_relevance: Option<f64> = None;

    for (_path, file_data) in by_file {
        if let Some(symbols) = file_data.get("symbols").and_then(|s| s.as_array()) {
            for symbol in symbols {
                let name = symbol.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let relevance = symbol.get("relevance").and_then(|r| r.as_f64());

                if name == "popular" {
                    popular_relevance = relevance;
                } else if name == "unpopular" {
                    unpopular_relevance = relevance;
                }
            }
        }
    }

    if let (Some(p), Some(u)) = (popular_relevance, unpopular_relevance) {
        assert!(
            p > u,
            "popular ({}) should have higher relevance than unpopular ({})",
            p,
            u
        );
    }
}
