//! AC#6: Ranked Output Flag Tests
//!
//! Given: User wants to see relevance scores
//! When: User runs `treelint map --ranked`
//! Then:
//!   - Output includes relevance score for each symbol
//!   - Symbols sorted by relevance (highest first) within each file
//!   - JSON includes `relevance` field
//!   - Text shows stars with score (e.g., * 0.85)
//!
//! Source files tested:
//!   - src/cli/commands/map.rs (Ranked output)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - MAP-008: Include relevance scores with --ranked flag

use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Test: `treelint map --ranked` includes relevance in JSON
/// Requirement: MAP-008 - JSON includes relevance field
#[test]
fn test_map_ranked_json_has_relevance() {
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
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    // Find symbol and check for relevance field
    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");

    let mut found_relevance = false;
    for (_path, file_data) in by_file {
        if let Some(symbols) = file_data.get("symbols").and_then(|s| s.as_array()) {
            for symbol in symbols {
                if symbol.get("relevance").is_some() {
                    found_relevance = true;
                    break;
                }
            }
        }
    }

    assert!(
        found_relevance,
        "JSON output with --ranked should include relevance field.\n\nJSON:\n{}",
        serde_json::to_string_pretty(&json).unwrap()
    );
}

/// Test: Without --ranked, JSON does NOT include relevance
/// Requirement: MAP-008 - Relevance only with --ranked flag
#[test]
fn test_map_without_ranked_no_relevance() {
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

    // Act: Without --ranked
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .assert()
        .success();

    // Assert: Should NOT have relevance field
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Either no "relevance" field, or if present, it should be null/omitted
    assert!(
        !stdout.contains("\"relevance\"") || stdout.contains("\"relevance\":null"),
        "Without --ranked, output should not include relevance.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Symbols sorted by relevance (highest first) within file
/// Requirement: MAP-008 - Symbols sorted by relevance
#[test]
fn test_map_ranked_sorted_by_relevance() {
    // Arrange: Create symbols with different reference counts
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("functions.py"),
        r#"
def most_popular():
    """Called by everyone."""
    pass

def somewhat_popular():
    most_popular()

def least_popular():
    most_popular()
    somewhat_popular()
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("caller1.py"),
        "from functions import most_popular\nmost_popular()",
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("caller2.py"),
        "from functions import most_popular\nmost_popular()",
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

    // Assert: Within functions.py, symbols should be sorted by relevance
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");

    // Find functions.py and check ordering
    for (path, file_data) in by_file {
        if path.contains("functions.py") {
            if let Some(symbols) = file_data.get("symbols").and_then(|s| s.as_array()) {
                let relevances: Vec<f64> = symbols
                    .iter()
                    .filter_map(|s| s.get("relevance").and_then(|r| r.as_f64()))
                    .collect();

                // Check that relevances are in descending order
                for i in 1..relevances.len() {
                    assert!(
                        relevances[i - 1] >= relevances[i],
                        "Symbols should be sorted by relevance (descending).\n\nRelevances: {:?}",
                        relevances
                    );
                }
            }
        }
    }
}

/// Test: Text output with --ranked shows star rating
/// Requirement: MAP-008 - Text shows stars with score
#[test]
fn test_map_ranked_text_shows_stars() {
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
        .args(["map", "--format", "text", "--ranked"])
        .assert()
        .success();

    // Assert: Should show star or score indicator
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    assert!(
        stdout.contains("*")
            || stdout.contains("0.")
            || stdout.contains("relevance")
            || stdout.contains("score"),
        "Text output with --ranked should show stars or score.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --ranked flag is recognized
/// Requirement: MAP-008 - Ranked flag parsing
#[test]
fn test_map_ranked_flag_recognized() {
    // Act & Assert
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["map", "--help"])
        .assert()
        .stdout(predicates::str::contains("ranked"));
}

/// Test: --ranked can be combined with --format
/// Requirement: MAP-008 - Works with format flag
#[test]
fn test_map_ranked_with_format() {
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

    // Act: JSON with --ranked
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Act: Text with --ranked
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "text", "--ranked"])
        .assert()
        .success();
}

/// Test: --ranked can be combined with --type
/// Requirement: MAP-008 - Works with type filter
#[test]
fn test_map_ranked_with_type_filter() {
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

    // Act: Filter by type with --ranked
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--type", "function", "--ranked"])
        .assert()
        .success();

    // Assert: Should have relevance for filtered results
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("relevance") && stdout.contains("foo"),
        "Should show relevance for filtered function.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Relevance values are numeric in JSON
/// Requirement: MAP-008 - Relevance is a number
#[test]
fn test_map_ranked_relevance_is_numeric() {
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
        .args(["map", "--format", "json", "--ranked"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    let by_file = json["by_file"]
        .as_object()
        .expect("by_file should be object");

    for (_path, file_data) in by_file {
        if let Some(symbols) = file_data.get("symbols").and_then(|s| s.as_array()) {
            for symbol in symbols {
                if let Some(relevance) = symbol.get("relevance") {
                    assert!(
                        relevance.is_number(),
                        "Relevance should be a number, got: {:?}",
                        relevance
                    );
                }
            }
        }
    }
}
