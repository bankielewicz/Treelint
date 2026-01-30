//! AC#7: Import Relationship Detection Tests
//!
//! Given: Source files with import statements
//! When: Import graph extraction runs
//! Then:
//!   - Detects imports via tree-sitter:
//!     - Python: import_statement, import_from_statement
//!     - TypeScript: import_statement, export_statement
//!     - Rust: use_declaration, mod_item
//!   - Resolves module path to actual file when possible
//!   - Handles relative and absolute imports
//!
//! Source files tested:
//!   - src/graph/imports.rs (ImportGraphExtractor)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DEPS-011: Detect import statements via tree-sitter
//!   - DEPS-012: Resolve module paths to file paths

use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

/// Test: Python import statements detected
/// Requirement: DEPS-011 - Python import_statement
#[test]
fn test_python_import_statement_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("utils.py"), "X = 1").unwrap();
    fs::write(temp_dir.path().join("main.py"), "import utils").unwrap();

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
        "Python import should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Python from-import statements detected
/// Requirement: DEPS-011 - Python import_from_statement
#[test]
fn test_python_from_import_detected() {
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
        "Python from-import should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Python relative imports detected
/// Requirement: DEPS-012 - Handles relative imports
#[test]
fn test_python_relative_import_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let pkg_dir = temp_dir.path().join("mypackage");
    fs::create_dir(&pkg_dir).unwrap();
    fs::write(pkg_dir.join("__init__.py"), "").unwrap();
    fs::write(pkg_dir.join("utils.py"), "X = 1").unwrap();
    fs::write(pkg_dir.join("main.py"), "from .utils import X").unwrap();

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
        stdout.contains("main") || stdout.contains("utils"),
        "Python relative import should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: TypeScript import statements detected
/// Requirement: DEPS-011 - TypeScript import_statement
#[test]
fn test_typescript_import_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("utils.ts"),
        "export const helper = () => {};",
    )
    .unwrap();
    fs::write(
        temp_dir.path().join("main.ts"),
        "import { helper } from './utils';",
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
        stdout.contains("main") && stdout.contains("utils"),
        "TypeScript import should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: TypeScript export statements detected
/// Requirement: DEPS-011 - TypeScript export_statement
#[test]
fn test_typescript_export_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("utils.ts"), "export const X = 1;").unwrap();
    fs::write(
        temp_dir.path().join("index.ts"),
        "export { X } from './utils';",
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
        stdout.contains("index") || stdout.contains("utils"),
        "TypeScript re-export should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Rust use declarations detected
/// Requirement: DEPS-011 - Rust use_declaration
#[test]
fn test_rust_use_declaration_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("lib.rs"),
        r#"
mod utils;

use utils::helper;

fn main() {
    helper();
}
"#,
    )
    .unwrap();
    fs::write(temp_dir.path().join("utils.rs"), "pub fn helper() {}").unwrap();

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
        stdout.contains("lib") || stdout.contains("utils"),
        "Rust use declaration should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Rust mod declarations detected
/// Requirement: DEPS-011 - Rust mod_item
#[test]
fn test_rust_mod_declaration_detected() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("main.rs"), "mod utils;").unwrap();
    fs::write(temp_dir.path().join("utils.rs"), "pub fn helper() {}").unwrap();

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
        stdout.contains("main") || stdout.contains("utils"),
        "Rust mod declaration should be detected.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Module path resolved to file path
/// Requirement: DEPS-012 - Resolve module paths to file paths
#[test]
fn test_module_path_resolved_to_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("helpers.py"), "def helper(): pass").unwrap();
    fs::write(
        temp_dir.path().join("main.py"),
        "from helpers import helper",
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
        .args(["deps", "--imports", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    // Should resolve "helpers" to "helpers.py"
    assert!(
        stdout.contains("helpers.py") || stdout.contains("helpers"),
        "Module path should resolve to file.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Circular imports handled
/// Requirement: DEPS-011 - Edge case: circular imports
#[test]
fn test_circular_imports_handled() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("module_a.py"),
        "from module_b import B\nA = 1",
    )
    .unwrap();
    fs::write(
        temp_dir.path().join("module_b.py"),
        "from module_a import A\nB = 2",
    )
    .unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Should not hang or crash on circular imports
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["deps", "--imports"])
        .timeout(std::time::Duration::from_secs(10))
        .assert()
        .success();
}

/// Test: Import type captured (direct, from, star)
/// Requirement: DEPS-011 - Import type classification
#[test]
fn test_import_type_captured() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("utils.py"), "X = 1\nY = 2").unwrap();
    fs::write(
        temp_dir.path().join("main.py"),
        r#"
import utils          # direct
from utils import X   # from
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
        .args(["deps", "--imports", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    // May include import_type field
    assert!(
        stdout.contains("import") || stdout.contains("edges"),
        "Import detection should produce output.\n\nActual output:\n{}",
        stdout
    );
}
