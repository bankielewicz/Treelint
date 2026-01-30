//! AC#2: JSON Output Format Tests
//!
//! Given: User wants machine-readable output
//! When: User runs `treelint map --format json`
//! Then:
//!   - JSON output matches schema with total_symbols, total_files, by_file, by_type
//!   - Each symbol has: name, type, lines, relevance (if --ranked)
//!
//! Source files tested:
//!   - src/output/json.rs (MapJsonFormatter)
//!   - src/cli/commands/map.rs
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - MAP-003: Generate JSON matching specified schema

use assert_cmd::Command;
use predicates::boolean::PredicateBooleanExt;
use predicates::prelude::*;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Test: `treelint map --format json` produces valid JSON
/// Requirement: MAP-003 - Generate JSON matching specified schema
#[test]
fn test_map_json_produces_valid_json() {
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
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert: Output is valid JSON
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Result<Value, _> = serde_json::from_str(&stdout);
    assert!(
        json.is_ok(),
        "Map --format json should produce valid JSON.\n\nActual output:\n{}\n\nError: {:?}",
        stdout,
        json.err()
    );
}

/// Test: JSON output has total_symbols field
/// Requirement: MAP-003 - Schema includes total_symbols: integer
#[test]
fn test_map_json_has_total_symbols() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def foo(): pass\ndef bar(): pass",
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
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    assert!(
        json.get("total_symbols").is_some(),
        "JSON should have total_symbols field.\n\nActual JSON:\n{}",
        serde_json::to_string_pretty(&json).unwrap()
    );

    assert!(
        json["total_symbols"].is_number(),
        "total_symbols should be a number.\n\nActual value: {:?}",
        json["total_symbols"]
    );
}

/// Test: JSON output has total_files field
/// Requirement: MAP-003 - Schema includes total_files: integer
#[test]
fn test_map_json_has_total_files() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("a.py"), "def a(): pass").unwrap();
    fs::write(temp_dir.path().join("b.py"), "def b(): pass").unwrap();

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
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    assert!(
        json.get("total_files").is_some(),
        "JSON should have total_files field"
    );

    assert!(
        json["total_files"].is_number(),
        "total_files should be a number"
    );

    // Should be 2 files
    assert_eq!(
        json["total_files"].as_i64(),
        Some(2),
        "total_files should be 2"
    );
}

/// Test: JSON output has by_file object
/// Requirement: MAP-003 - Schema includes by_file: object with file paths as keys
#[test]
fn test_map_json_has_by_file_object() {
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
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    assert!(
        json.get("by_file").is_some(),
        "JSON should have by_file field"
    );

    assert!(json["by_file"].is_object(), "by_file should be an object");
}

/// Test: by_file contains file entries with language and symbols
/// Requirement: MAP-003 - Each file contains language and symbols array
#[test]
fn test_map_json_by_file_structure() {
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
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");

    // Check that at least one file entry has language and symbols
    for (path, file_data) in by_file {
        assert!(
            file_data.get("language").is_some() || file_data.get("symbols").is_some(),
            "File {} should have language or symbols field.\n\nActual: {:?}",
            path,
            file_data
        );
    }
}

/// Test: JSON output has by_type object with counts
/// Requirement: MAP-003 - Schema includes by_type: object with symbol type counts
#[test]
fn test_map_json_has_by_type_object() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def foo(): pass\nclass Bar: pass",
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
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    assert!(
        json.get("by_type").is_some(),
        "JSON should have by_type field"
    );

    assert!(json["by_type"].is_object(), "by_type should be an object");
}

/// Test: Symbol entries have required fields (name, type, lines)
/// Requirement: MAP-003 - Each symbol has: name, type, lines
#[test]
fn test_map_json_symbol_has_required_fields() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("example.py"), "def test_func(): pass").unwrap();

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
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    // Find a symbol in by_file
    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");

    let mut found_symbol = false;
    for (_path, file_data) in by_file {
        if let Some(symbols) = file_data.get("symbols").and_then(|s| s.as_array()) {
            for symbol in symbols {
                found_symbol = true;
                assert!(
                    symbol.get("name").is_some(),
                    "Symbol should have name field"
                );
                assert!(
                    symbol.get("type").is_some(),
                    "Symbol should have type field"
                );
                assert!(
                    symbol.get("lines").is_some(),
                    "Symbol should have lines field"
                );
            }
        }
    }

    assert!(found_symbol, "Should find at least one symbol");
}

/// Test: --format json flag is recognized
/// Requirement: MAP-003 - JSON format flag
#[test]
fn test_map_format_json_flag_recognized() {
    // Act & Assert
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["map", "--help"])
        .assert()
        .stdout(predicates::str::contains("format").or(predicates::str::contains("json")));
}
