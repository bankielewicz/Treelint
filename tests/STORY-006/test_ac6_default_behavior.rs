//! AC#6: Default Behavior Without Context Flag
//!
//! Tests that:
//! - No flags defaults to --context full (complete semantic unit)
//! - Default behavior produces complete function/class bodies
//! - query.context_mode is "full" in default mode
//!
//! TDD Phase: RED - These tests should FAIL until default context mode is implemented.
//!
//! Technical Specification Requirements (CTX-008, BR-003):
//! - Default context mode is 'full' when no flag specified

use assert_cmd::Command;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Create a test project with functions that have bodies
fn setup_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("example.py");
    fs::write(
        &sample_file,
        r#"
@deprecated
def example_function(x: int, y: int) -> int:
    """An example function with body."""
    result = x + y
    if result < 0:
        return 0
    return result

class ExampleClass:
    """An example class."""

    def __init__(self, value: int):
        """Initialize."""
        self.value = value

    def get_doubled(self) -> int:
        """Return doubled value."""
        return self.value * 2
"#,
    )
    .expect("Failed to write example.py");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// Default Context Mode Tests (BR-003)
// ──────────────────────────────────────────────────────────────────────────

/// Test: No context flag defaults to full semantic context
///
/// Given: A search without --context or --signatures
/// When: treelint search example_function --format json is executed
/// Then: Results include complete function body (equivalent to --context full)
#[test]
fn test_default_context_is_full_semantic_unit() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // NO --context flag
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "example_function", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(
        !results.is_empty(),
        "Expected results for 'example_function'"
    );

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present in default mode");

    // Should include complete function body (not just signature)
    assert!(
        body.contains("def example_function"),
        "Default mode should include function definition.\n\nActual body:\n{}",
        body
    );

    assert!(
        body.contains("result = x + y"),
        "Default mode should include function body.\n\nActual body:\n{}",
        body
    );

    assert!(
        body.contains("return result"),
        "Default mode should include full body to end.\n\nActual body:\n{}",
        body
    );
}

/// Test: Default mode includes decorators (full semantic unit)
///
/// Given: A decorated function
/// When: treelint search example_function --format json is executed (no flags)
/// Then: Results include the decorator
#[test]
fn test_default_context_includes_decorators() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "example_function", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(!results.is_empty(), "Expected results");

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Full semantic unit should include decorator
    assert!(
        body.contains("@deprecated"),
        "Default full mode should include decorators.\n\nActual body:\n{}",
        body
    );
}

/// Test: Default mode includes docstrings
///
/// Given: A function with docstring
/// When: treelint search example_function --format json is executed (no flags)
/// Then: Results include the docstring
#[test]
fn test_default_context_includes_docstrings() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "example_function", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(!results.is_empty(), "Expected results");

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Full semantic unit should include docstring
    assert!(
        body.contains("An example function with body"),
        "Default full mode should include docstrings.\n\nActual body:\n{}",
        body
    );
}

// ──────────────────────────────────────────────────────────────────────────
// JSON context_mode Field Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Default mode sets context_mode to "full" in JSON
///
/// Given: A search without --context or --signatures
/// When: treelint search example_function --format json is executed
/// Then: query.context_mode is "full"
#[test]
fn test_default_context_mode_is_full_in_json() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // NO --context flag
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "example_function", "--format", "json"])
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
        "query.context_mode must be 'full' by default.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

/// Test: context_mode is always present even in default mode
#[test]
fn test_default_context_mode_always_present() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "example_function", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let context_mode = json.get("query").and_then(|q| q.get("context_mode"));

    assert!(
        context_mode.is_some(),
        "query.context_mode must be present in default mode.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Equivalence Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Default behavior is equivalent to --context full
///
/// Given: Two searches - one with no flags, one with --context full
/// When: Both are executed
/// Then: Both produce identical context_mode and body content
#[test]
fn test_default_equivalent_to_context_full() {
    let temp_dir = setup_test_project();

    // Search with default (no flags)
    let mut cmd_default = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output_default = cmd_default
        .current_dir(temp_dir.path())
        .args(["search", "example_function", "--format", "json"])
        .assert()
        .success();

    let stdout_default = String::from_utf8_lossy(&output_default.get_output().stdout);
    let json_default: Value = serde_json::from_str(&stdout_default).expect("Valid JSON");

    // Search with explicit --context full
    let mut cmd_explicit = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output_explicit = cmd_explicit
        .current_dir(temp_dir.path())
        .args([
            "search",
            "example_function",
            "--context",
            "full",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout_explicit = String::from_utf8_lossy(&output_explicit.get_output().stdout);
    let json_explicit: Value = serde_json::from_str(&stdout_explicit).expect("Valid JSON");

    // Compare context_mode
    let mode_default = json_default
        .get("query")
        .and_then(|q| q.get("context_mode"))
        .and_then(|c| c.as_str());
    let mode_explicit = json_explicit
        .get("query")
        .and_then(|q| q.get("context_mode"))
        .and_then(|c| c.as_str());

    assert_eq!(
        mode_default, mode_explicit,
        "Default context_mode should equal explicit --context full"
    );

    // Compare body content
    let body_default = json_default
        .get("results")
        .and_then(|r| r.as_array())
        .and_then(|a| a.first())
        .and_then(|r| r.get("body"))
        .and_then(|b| b.as_str());

    let body_explicit = json_explicit
        .get("results")
        .and_then(|r| r.as_array())
        .and_then(|a| a.first())
        .and_then(|r| r.get("body"))
        .and_then(|b| b.as_str());

    assert_eq!(
        body_default, body_explicit,
        "Default body content should equal --context full body content"
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Default Text Output Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Default text output shows complete body
///
/// Given: A search without flags
/// When: treelint search example_function --format text is executed
/// Then: Text output includes complete function body
#[test]
fn test_default_text_output_shows_complete_body() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "example_function", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Should include function body in text output
    assert!(
        stdout.contains("result = x + y") || stdout.contains("return result"),
        "Default text output should include function body.\n\nActual output:\n{}",
        stdout
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Default Behavior with Classes
// ──────────────────────────────────────────────────────────────────────────

/// Test: Default mode for class returns complete class
///
/// Given: A class search without flags
/// When: treelint search ExampleClass --format json is executed
/// Then: Results include complete class with all methods
#[test]
fn test_default_context_class_complete() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "ExampleClass",
            "--type",
            "class",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(!results.is_empty(), "Expected results for 'ExampleClass'");

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Should include class definition
    assert!(
        body.contains("class ExampleClass"),
        "Default mode should include class definition.\n\nActual body:\n{}",
        body
    );

    // Should include all methods
    assert!(
        body.contains("def __init__"),
        "Default mode should include __init__.\n\nActual body:\n{}",
        body
    );

    assert!(
        body.contains("def get_doubled"),
        "Default mode should include get_doubled method.\n\nActual body:\n{}",
        body
    );
}

/// Test: Default mode for method returns only method (not class)
///
/// Given: A method search without flags
/// When: treelint search get_doubled --type method --format json is executed
/// Then: Results include only the method, not entire class
#[test]
fn test_default_context_method_not_entire_class() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "get_doubled",
            "--type",
            "method",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(
        !results.is_empty(),
        "Expected results for 'get_doubled' method"
    );

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Should include method
    assert!(
        body.contains("def get_doubled"),
        "Default mode should include method.\n\nActual body:\n{}",
        body
    );

    // Should NOT include class definition (method search should return just method)
    assert!(
        !body.contains("class ExampleClass"),
        "Default mode for method should NOT include class definition.\n\nActual body:\n{}",
        body
    );
}
