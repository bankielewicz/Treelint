//! AC#5: Context Modes Work with All Symbol Types
//!
//! Tests that:
//! - Context extraction works for functions, classes, methods, variables, constants
//! - Context modes work across Python, TypeScript, Rust, and Markdown
//! - All three context modes (lines, full, signatures) work with each symbol type
//!
//! TDD Phase: RED - These tests should FAIL until context modes are implemented for all types.
//!
//! Technical Specification Requirements (CTX-007):
//! - Context extraction works correctly for all symbol types across all supported languages

use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Create a comprehensive test project with all symbol types in Python
fn setup_python_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("all_types.py");
    fs::write(
        &sample_file,
        r#"# Header comment
# More header

import os
from typing import List

# Constant
MAX_RETRIES: int = 5

# Variable
current_count = 0

def standalone_function(x: int) -> int:
    """A standalone function."""
    return x * 2

class MyClass:
    """A sample class."""

    class_variable = "class level"

    def __init__(self, value: int):
        """Initialize."""
        self.value = value

    def instance_method(self) -> int:
        """An instance method."""
        return self.value

# Footer comment
"#,
    )
    .expect("Failed to write all_types.py");

    temp_dir
}

/// Create a comprehensive test project with all symbol types in TypeScript
fn setup_typescript_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("all_types.ts");
    fs::write(
        &sample_file,
        r#"// Header comment
// More header

import { something } from './other';

// Constant
export const MAX_SIZE: number = 100;

// Variable
let counter: number = 0;

/**
 * A standalone function.
 */
export function standaloneFunc(x: number): number {
    return x * 2;
}

/**
 * A sample class.
 */
export class SampleClass {
    private value: number;

    constructor(value: number) {
        this.value = value;
    }

    getValue(): number {
        return this.value;
    }
}

export default SampleClass;
"#,
    )
    .expect("Failed to write all_types.ts");

    temp_dir
}

/// Create a comprehensive test project with all symbol types in Rust
fn setup_rust_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("lib.rs");
    fs::write(
        &sample_file,
        r#"//! Module documentation

use std::collections::HashMap;

/// A constant value
pub const MAX_VALUE: u32 = 1000;

/// A standalone function
pub fn standalone_fn(x: i32) -> i32 {
    x * 2
}

/// A sample struct
pub struct SampleStruct {
    value: i32,
}

impl SampleStruct {
    /// Create a new instance
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    /// Get the value
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
"#,
    )
    .expect("Failed to write lib.rs");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// Python Symbol Types Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context full works with Python functions
#[test]
fn test_context_full_python_function() {
    let temp_dir = setup_python_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "standalone_function",
            "--context",
            "full",
            "--type",
            "function",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for Python function.\n\nOutput:\n{}",
        stdout
    );

    let body = results.unwrap()[0].get("body").and_then(|b| b.as_str());
    assert!(
        body.is_some() && body.unwrap().contains("def standalone_function"),
        "Full context should include function definition"
    );
}

/// Test: --context full works with Python classes
#[test]
fn test_context_full_python_class() {
    let temp_dir = setup_python_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "MyClass",
            "--context",
            "full",
            "--type",
            "class",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for Python class.\n\nOutput:\n{}",
        stdout
    );

    let body = results.unwrap()[0].get("body").and_then(|b| b.as_str());
    assert!(
        body.is_some() && body.unwrap().contains("class MyClass"),
        "Full context should include class definition"
    );
}

/// Test: --context full works with Python methods
#[test]
fn test_context_full_python_method() {
    let temp_dir = setup_python_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "instance_method",
            "--context",
            "full",
            "--type",
            "method",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for Python method.\n\nOutput:\n{}",
        stdout
    );
}

/// Test: --signatures works with Python constants
#[test]
fn test_signatures_python_constant() {
    let temp_dir = setup_python_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "MAX_RETRIES",
            "--signatures",
            "--type",
            "constant",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for Python constant.\n\nOutput:\n{}",
        stdout
    );
}

/// Test: --context N works with Python variables
#[test]
fn test_context_lines_python_variable() {
    let temp_dir = setup_python_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "current_count",
            "--context",
            "2",
            "--type",
            "variable",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for Python variable.\n\nOutput:\n{}",
        stdout
    );
}

// ──────────────────────────────────────────────────────────────────────────
// TypeScript Symbol Types Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context full works with TypeScript functions
#[test]
fn test_context_full_typescript_function() {
    let temp_dir = setup_typescript_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "standaloneFunc",
            "--context",
            "full",
            "--type",
            "function",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for TypeScript function.\n\nOutput:\n{}",
        stdout
    );
}

/// Test: --context full works with TypeScript classes
#[test]
fn test_context_full_typescript_class() {
    let temp_dir = setup_typescript_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "SampleClass",
            "--context",
            "full",
            "--type",
            "class",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for TypeScript class.\n\nOutput:\n{}",
        stdout
    );
}

/// Test: --signatures works with TypeScript methods
#[test]
fn test_signatures_typescript_method() {
    let temp_dir = setup_typescript_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "getValue",
            "--signatures",
            "--type",
            "method",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for TypeScript method.\n\nOutput:\n{}",
        stdout
    );

    // Verify body is null in signatures mode
    let body = results.unwrap()[0].get("body");
    let body_is_null = body.is_none() || body.unwrap().is_null();
    assert!(body_is_null, "Signatures mode should have null body");
}

/// Test: --context N works with TypeScript exports
#[test]
fn test_context_lines_typescript_export() {
    let temp_dir = setup_typescript_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Search for MAX_SIZE which is `export const MAX_SIZE` - an actual Export type
    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "MAX_SIZE",
            "--context",
            "3",
            "--type",
            "export",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let context_mode = json
        .get("query")
        .and_then(|q| q.get("context_mode"))
        .and_then(|c| c.as_str());

    assert_eq!(
        context_mode,
        Some("lines:3"),
        "context_mode should be 'lines:3' for TypeScript export search"
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Rust Symbol Types Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context full works with Rust functions
#[test]
fn test_context_full_rust_function() {
    let temp_dir = setup_rust_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "standalone_fn",
            "--context",
            "full",
            "--type",
            "function",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for Rust function.\n\nOutput:\n{}",
        stdout
    );

    let body = results.unwrap()[0].get("body").and_then(|b| b.as_str());
    assert!(
        body.is_some() && body.unwrap().contains("pub fn standalone_fn"),
        "Full context should include Rust function definition"
    );
}

/// Test: --context full works with Rust impl methods
#[test]
fn test_context_full_rust_method() {
    let temp_dir = setup_rust_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "get_value",
            "--context",
            "full",
            "--type",
            "method",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for Rust method.\n\nOutput:\n{}",
        stdout
    );
}

/// Test: --signatures works with Rust constants
#[test]
fn test_signatures_rust_constant() {
    let temp_dir = setup_rust_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "MAX_VALUE",
            "--signatures",
            "--type",
            "constant",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for Rust constant.\n\nOutput:\n{}",
        stdout
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Cross-Language Consistency Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: All three context modes return consistent JSON structure across languages
#[test]
fn test_context_modes_consistent_json_structure() {
    let python_project = setup_python_project();
    let typescript_project = setup_typescript_project();
    let rust_project = setup_rust_project();

    let projects = vec![
        (&python_project, "standalone_function", "function"),
        (&typescript_project, "standaloneFunc", "function"),
        (&rust_project, "standalone_fn", "function"),
    ];

    for (project, symbol, symbol_type) in projects {
        // Test --context full
        let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
        let output = cmd
            .current_dir(project.path())
            .args([
                "search",
                symbol,
                "--context",
                "full",
                "--type",
                symbol_type,
                "--format",
                "json",
            ])
            .assert()
            .success();

        let stdout = String::from_utf8_lossy(&output.get_output().stdout);
        let json: Value = serde_json::from_str(&stdout)
            .expect(&format!("Output must be valid JSON for {}", symbol));

        // Verify consistent structure
        assert!(json.get("query").is_some(), "Missing query for {}", symbol);
        assert!(
            json.get("results").is_some(),
            "Missing results for {}",
            symbol
        );
        assert!(json.get("stats").is_some(), "Missing stats for {}", symbol);

        // Verify context_mode is present
        let context_mode = json.get("query").and_then(|q| q.get("context_mode"));
        assert!(
            context_mode.is_some(),
            "Missing context_mode in query for {}",
            symbol
        );
    }
}

/// Test: --signatures mode body null across all languages
#[test]
fn test_signatures_mode_body_null_all_languages() {
    let python_project = setup_python_project();
    let typescript_project = setup_typescript_project();
    let rust_project = setup_rust_project();

    let projects = vec![
        (&python_project, "standalone_function"),
        (&typescript_project, "standaloneFunc"),
        (&rust_project, "standalone_fn"),
    ];

    for (project, symbol) in projects {
        let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
        let output = cmd
            .current_dir(project.path())
            .args(["search", symbol, "--signatures", "--format", "json"])
            .assert()
            .success();

        let stdout = String::from_utf8_lossy(&output.get_output().stdout);
        let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

        let results = json.get("results").and_then(|r| r.as_array());
        if let Some(results) = results {
            if !results.is_empty() {
                let body = results[0].get("body");
                let body_is_null = body.is_none() || body.unwrap().is_null();
                assert!(
                    body_is_null,
                    "Signatures mode should have null body for {}.\n\nBody: {:?}",
                    symbol, body
                );
            }
        }
    }
}
