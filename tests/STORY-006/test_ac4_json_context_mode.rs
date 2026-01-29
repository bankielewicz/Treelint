//! AC#4: JSON Output Reflects Context Mode
//!
//! Tests that:
//! - query.context_mode field reflects the mode used
//! - "lines:N" format for line-based context
//! - "full" for semantic context mode
//! - "signatures" for minimal/signatures mode
//!
//! TDD Phase: RED - These tests should FAIL until context mode JSON output is implemented.
//!
//! Technical Specification Requirements (CTX-006):
//! - Include context_mode in query metadata

use assert_cmd::Command;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Create a simple test project
fn setup_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("sample.py");
    fs::write(
        &sample_file,
        r#"
def hello_world() -> str:
    """Say hello."""
    return "Hello, World!"
"#,
    )
    .expect("Failed to write sample.py");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// context_mode Field Format Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: context_mode is "lines:N" format for --context N
///
/// Given: A search with --context 5
/// When: treelint search hello_world --context 5 --format json is executed
/// Then: query.context_mode is "lines:5"
#[test]
fn test_json_context_mode_lines_format() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "hello_world",
            "--context",
            "5",
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
        Some("lines:5"),
        "query.context_mode must be 'lines:5' for --context 5.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

/// Test: context_mode is "lines:1" for --context 1
#[test]
fn test_json_context_mode_lines_1() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "hello_world",
            "--context",
            "1",
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
        Some("lines:1"),
        "query.context_mode must be 'lines:1' for --context 1.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

/// Test: context_mode is "lines:100" for large N
#[test]
fn test_json_context_mode_lines_large_n() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "hello_world",
            "--context",
            "100",
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
        Some("lines:100"),
        "query.context_mode must be 'lines:100' for --context 100.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

/// Test: context_mode is "full" for --context full
#[test]
fn test_json_context_mode_full() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "hello_world",
            "--context",
            "full",
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
        Some("full"),
        "query.context_mode must be 'full' for --context full.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

/// Test: context_mode is "signatures" for --signatures
#[test]
fn test_json_context_mode_signatures() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "hello_world", "--signatures", "--format", "json"])
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
        Some("signatures"),
        "query.context_mode must be 'signatures' for --signatures.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

// ──────────────────────────────────────────────────────────────────────────
// context_mode Field Presence Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: context_mode field is always present in query object
///
/// Given: Any search command with JSON output
/// When: treelint search hello_world --format json is executed
/// Then: query.context_mode field is present
#[test]
fn test_json_context_mode_field_always_present() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "hello_world", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let context_mode = json.get("query").and_then(|q| q.get("context_mode"));

    assert!(
        context_mode.is_some(),
        "query.context_mode must always be present in JSON output.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

/// Test: context_mode field is a string type
#[test]
fn test_json_context_mode_is_string() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "hello_world",
            "--context",
            "5",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let context_mode = json.get("query").and_then(|q| q.get("context_mode"));

    assert!(
        context_mode.is_some() && context_mode.unwrap().is_string(),
        "query.context_mode must be a string.\n\nActual type: {:?}",
        context_mode
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Empty Results Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: context_mode is present even with empty results
///
/// Given: A search for non-existent symbol
/// When: treelint search nonExistent --context 5 --format json is executed
/// Then: query.context_mode is still "lines:5"
#[test]
fn test_json_context_mode_present_with_empty_results() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "nonExistentXYZ",
            "--context",
            "5",
            "--format",
            "json",
        ])
        .assert();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let context_mode = json
        .get("query")
        .and_then(|q| q.get("context_mode"))
        .and_then(|c| c.as_str());

    assert_eq!(
        context_mode,
        Some("lines:5"),
        "query.context_mode must be 'lines:5' even with empty results.\n\nActual query:\n{:?}",
        json.get("query")
    );

    // Verify results is empty
    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && results.unwrap().is_empty(),
        "Results should be empty for non-existent symbol"
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Consistency Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: context_mode matches actual output behavior for lines mode
///
/// Given: A search with --context 3
/// When: Results are returned
/// Then: context_mode "lines:3" matches actual context extraction
#[test]
fn test_json_context_mode_matches_actual_behavior_lines() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create file with known structure
    let sample_file = src_dir.join("known_lines.py");
    fs::write(
        &sample_file,
        r#"# Line 1
# Line 2
# Line 3
def target_func():
    """Target function."""
    return "target"
# Line 7
# Line 8
# Line 9
"#,
    )
    .expect("Failed to write file");

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "target_func",
            "--context",
            "2",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    // Verify context_mode
    let context_mode = json
        .get("query")
        .and_then(|q| q.get("context_mode"))
        .and_then(|c| c.as_str());

    assert_eq!(context_mode, Some("lines:2"));

    // Verify body contains expected context
    let results = json.get("results").and_then(|r| r.as_array());
    if let Some(results) = results {
        if !results.is_empty() {
            let body = results[0].get("body").and_then(|b| b.as_str());
            if let Some(body) = body {
                // Should have 2 lines before (Line 2, Line 3)
                assert!(
                    body.contains("# Line 2") || body.contains("# Line 3"),
                    "Body should contain context lines as indicated by context_mode.\n\nBody:\n{}",
                    body
                );
            }
        }
    }
}

/// Test: context_mode matches actual output behavior for full mode
#[test]
fn test_json_context_mode_matches_actual_behavior_full() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "hello_world",
            "--context",
            "full",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    // Verify context_mode
    let context_mode = json
        .get("query")
        .and_then(|q| q.get("context_mode"))
        .and_then(|c| c.as_str());

    assert_eq!(context_mode, Some("full"));

    // Verify body contains complete function (semantic unit)
    let results = json.get("results").and_then(|r| r.as_array());
    if let Some(results) = results {
        if !results.is_empty() {
            let body = results[0].get("body").and_then(|b| b.as_str());
            if let Some(body) = body {
                assert!(
                    body.contains("def hello_world") && body.contains("return"),
                    "Full mode should include complete semantic unit.\n\nBody:\n{}",
                    body
                );
            }
        }
    }
}

/// Test: context_mode matches actual output behavior for signatures mode
#[test]
fn test_json_context_mode_matches_actual_behavior_signatures() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "hello_world", "--signatures", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    // Verify context_mode
    let context_mode = json
        .get("query")
        .and_then(|q| q.get("context_mode"))
        .and_then(|c| c.as_str());

    assert_eq!(context_mode, Some("signatures"));

    // Verify body is null/absent
    let results = json.get("results").and_then(|r| r.as_array());
    if let Some(results) = results {
        if !results.is_empty() {
            let body = results[0].get("body");
            let body_is_null_or_absent = body.is_none() || body.unwrap().is_null();
            assert!(
                body_is_null_or_absent,
                "Signatures mode should have null body as indicated by context_mode."
            );
        }
    }
}
