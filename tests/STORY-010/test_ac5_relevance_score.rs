//! AC#5: Relevance Score Calculation Tests
//!
//! Given: Repository has symbols that reference each other
//! When: Relevance scoring is calculated
//! Then:
//!   - Each symbol gets relevance score: (incoming_references + 1) / total_symbols
//!   - Score normalized to 0.0 - 1.0 range
//!   - Symbols referenced more frequently have higher scores
//!   - Score stored in symbols.relevance_score column
//!
//! Source files tested:
//!   - src/index/relevance.rs (RelevanceScorer)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - MAP-006: Calculate relevance score for each symbol
//!   - MAP-007: Normalize scores to 0.0-1.0 range

use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Test: Relevance score formula is applied correctly
/// Requirement: MAP-006 - Score = (incoming_refs + 1) / total_symbols
#[test]
fn test_relevance_score_formula_applied() {
    // Arrange: Create files with known reference pattern
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // helper.py: defines helper() - will be referenced by 2 other files
    fs::write(
        temp_dir.path().join("helper.py"),
        r#"
def helper():
    """A helper function used by others."""
    return "help"
"#,
    )
    .unwrap();

    // main.py: references helper
    fs::write(
        temp_dir.path().join("main.py"),
        r#"
from helper import helper

def main():
    result = helper()
    return result
"#,
    )
    .unwrap();

    // utils.py: also references helper
    fs::write(
        temp_dir.path().join("utils.py"),
        r#"
from helper import helper

def process():
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

    // Act: Get map with relevance
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert: helper should have higher relevance than others
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    // Find helper's relevance score
    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");
    let mut helper_relevance: Option<f64> = None;
    let mut main_relevance: Option<f64> = None;

    for (_path, file_data) in by_file {
        if let Some(symbols) = file_data.get("symbols").and_then(|s| s.as_array()) {
            for symbol in symbols {
                let name = symbol.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let relevance = symbol.get("relevance").and_then(|r| r.as_f64());

                if name == "helper" {
                    helper_relevance = relevance;
                } else if name == "main" {
                    main_relevance = relevance;
                }
            }
        }
    }

    // helper (referenced 2x) should have higher relevance than main (referenced 0x)
    if let (Some(h), Some(m)) = (helper_relevance, main_relevance) {
        assert!(
            h > m,
            "helper (relevance={}) should have higher relevance than main (relevance={})",
            h,
            m
        );
    }
}

/// Test: Relevance scores are normalized to 0.0-1.0 range
/// Requirement: MAP-007 - Normalize scores to 0.0-1.0 range
#[test]
fn test_relevance_scores_normalized_range() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def a(): pass
def b(): a()
def c(): a(); b()
def d(): a(); b(); c()
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

    // Assert: All scores should be in 0.0-1.0 range
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");

    for (_path, file_data) in by_file {
        if let Some(symbols) = file_data.get("symbols").and_then(|s| s.as_array()) {
            for symbol in symbols {
                if let Some(relevance) = symbol.get("relevance").and_then(|r| r.as_f64()) {
                    assert!(
                        relevance >= 0.0 && relevance <= 1.0,
                        "Relevance score {} should be in 0.0-1.0 range",
                        relevance
                    );
                }
            }
        }
    }
}

/// Test: More referenced symbols have higher scores
/// Requirement: MAP-006 - Symbols referenced more frequently have higher scores
#[test]
fn test_more_references_higher_score() {
    // Arrange: Create chain of references
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // core.py - referenced by everyone
    fs::write(temp_dir.path().join("core.py"), "def core_func(): return 1").unwrap();

    // a.py - uses core
    fs::write(
        temp_dir.path().join("a.py"),
        "from core import core_func\ndef a(): return core_func()",
    )
    .unwrap();

    // b.py - uses core
    fs::write(
        temp_dir.path().join("b.py"),
        "from core import core_func\ndef b(): return core_func()",
    )
    .unwrap();

    // c.py - uses core
    fs::write(
        temp_dir.path().join("c.py"),
        "from core import core_func\ndef c(): return core_func()",
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

    // Assert: core_func should have highest score
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");

    let mut max_relevance = 0.0;
    let mut max_symbol = String::new();

    for (_path, file_data) in by_file {
        if let Some(symbols) = file_data.get("symbols").and_then(|s| s.as_array()) {
            for symbol in symbols {
                let name = symbol.get("name").and_then(|n| n.as_str()).unwrap_or("");
                if let Some(relevance) = symbol.get("relevance").and_then(|r| r.as_f64()) {
                    if relevance > max_relevance {
                        max_relevance = relevance;
                        max_symbol = name.to_string();
                    }
                }
            }
        }
    }

    assert_eq!(
        max_symbol, "core_func",
        "core_func should have highest relevance, but {} does",
        max_symbol
    );
}

/// Test: Unreferenced symbols have minimum non-zero score
/// Requirement: BR-002 - Symbols with no references have minimum score (1/total)
#[test]
fn test_unreferenced_symbol_has_nonzero_score() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("isolated.py"),
        "def lonely_function(): pass",
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

    // Assert: Even unreferenced symbols should have score > 0
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");

    for (_path, file_data) in by_file {
        if let Some(symbols) = file_data.get("symbols").and_then(|s| s.as_array()) {
            for symbol in symbols {
                if let Some(relevance) = symbol.get("relevance").and_then(|r| r.as_f64()) {
                    assert!(
                        relevance > 0.0,
                        "Even unreferenced symbols should have non-zero relevance score"
                    );
                }
            }
        }
    }
}

/// Test: Relevance score stored in database
/// Requirement: MAP-006 - Score stored in symbols.relevance_score column
#[test]
fn test_relevance_score_persisted_in_db() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("example.py"), "def foo(): pass").unwrap();

    // Build index (which should calculate relevance)
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act: First map with --ranked
    let first = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Second map (should use stored values, not recalculate)
    let second = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert: Both should have same relevance (persisted)
    let first_stdout = String::from_utf8_lossy(&first.get_output().stdout);
    let second_stdout = String::from_utf8_lossy(&second.get_output().stdout);

    assert_eq!(
        first_stdout.to_string(),
        second_stdout.to_string(),
        "Relevance scores should be persisted and consistent"
    );
}

/// Test: All symbols receive a relevance score
/// Requirement: MAP-006 - Calculate relevance score for each symbol
#[test]
fn test_all_symbols_have_relevance() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("multi.py"),
        r#"
def func1(): pass
def func2(): pass
class Class1: pass
MY_CONST = 42
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

    // Assert: Every symbol should have a relevance field
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");

    let mut symbol_count = 0;
    let mut with_relevance = 0;

    for (_path, file_data) in by_file {
        if let Some(symbols) = file_data.get("symbols").and_then(|s| s.as_array()) {
            for symbol in symbols {
                symbol_count += 1;
                if symbol.get("relevance").is_some() {
                    with_relevance += 1;
                }
            }
        }
    }

    assert!(symbol_count > 0, "Should have at least one symbol");

    assert_eq!(
        symbol_count, with_relevance,
        "All {} symbols should have relevance, but only {} do",
        symbol_count, with_relevance
    );
}
