//! AC#1: Line-Based Context Mode (--context N)
//!
//! Tests that:
//! - `--context N` extracts N lines before the symbol's start line
//! - `--context N` extracts N lines after the symbol's end line
//! - Edge cases: N exceeds file boundaries (clamped to file start/end)
//! - Edge cases: symbol at file start or end
//! - Context value must be positive integer (BR-002)
//!
//! TDD Phase: RED - These tests should FAIL until line-based context extraction is implemented.
//!
//! Technical Specification Requirements (CTX-001, CTX-002):
//! - Extract N lines before and after a symbol's line range
//! - Handle edge cases where N lines would exceed file boundaries

use assert_cmd::Command;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Create a test project with a Python file where we know exact line numbers
fn setup_test_project_with_known_lines() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Each line is numbered in comment for verification
    // validateUser function is at lines 10-15 (1-indexed)
    let sample_file = src_dir.join("validators.py");
    fs::write(
        &sample_file,
        r#"# Line 1
# Line 2
# Line 3
# Line 4
# Line 5
# Line 6
# Line 7
# Line 8
# Line 9
def validateUser(email: str, password: str) -> bool:
    """Validate user credentials."""
    if not email:
        return False
    return len(password) >= 8
# Line 16
# Line 17
# Line 18
# Line 19
# Line 20
"#,
    )
    .expect("Failed to write validators.py");

    temp_dir
}

/// Create a test project with a function at the start of file (edge case)
fn setup_test_project_function_at_start() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Function starts at line 1 - no lines before it
    let sample_file = src_dir.join("at_start.py");
    fs::write(
        &sample_file,
        r#"def first_function() -> str:
    """First function in file."""
    return "first"
# Line 4
# Line 5
"#,
    )
    .expect("Failed to write at_start.py");

    temp_dir
}

/// Create a test project with a function at the end of file (edge case)
fn setup_test_project_function_at_end() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Function is at end of file - no lines after it
    let sample_file = src_dir.join("at_end.py");
    fs::write(
        &sample_file,
        r#"# Line 1
# Line 2
# Line 3
def last_function() -> str:
    """Last function in file."""
    return "last"
"#,
    )
    .expect("Failed to write at_end.py");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// Basic Line Context Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context 5 extracts 5 lines before and after symbol
///
/// Given: A function "validateUser" at lines 10-15 in a file with 20 lines
/// When: treelint search validateUser --context 5 is executed
/// Then: Results include lines 5-9 (before) + symbol (10-15) + lines 16-20 (after)
#[test]
fn test_context_lines_extracts_n_lines_before_and_after() {
    let temp_dir = setup_test_project_with_known_lines();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validateUser",
            "--context",
            "5",
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
        "Expected at least one result for 'validateUser'"
    );

    // Verify body contains context lines
    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Body should include the 5 lines before (comments # Line 5-9)
    assert!(
        body.contains("# Line 5"),
        "Context should include 5 lines before symbol.\n\nActual body:\n{}",
        body
    );

    // Body should include the symbol itself
    assert!(
        body.contains("def validateUser"),
        "Context should include the symbol.\n\nActual body:\n{}",
        body
    );

    // Body should include the 5 lines after (comments # Line 16-20)
    assert!(
        body.contains("# Line 16"),
        "Context should include 5 lines after symbol.\n\nActual body:\n{}",
        body
    );
}

/// Test: --context 3 extracts exactly 3 lines before and after
///
/// Given: A function with plenty of lines before/after
/// When: treelint search validateUser --context 3 is executed
/// Then: Results include exactly 3 lines before and 3 after
#[test]
fn test_context_lines_exact_count() {
    let temp_dir = setup_test_project_with_known_lines();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validateUser",
            "--context",
            "3",
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

    assert!(!results.is_empty(), "Expected results");

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Should include Line 7, 8, 9 (3 lines before symbol at line 10)
    assert!(
        body.contains("# Line 7"),
        "Context should include line 7 (3 before).\n\nActual body:\n{}",
        body
    );

    // Should NOT include Line 6 (4 lines before)
    assert!(
        !body.contains("# Line 6"),
        "Context should NOT include line 6 (only 3 lines requested).\n\nActual body:\n{}",
        body
    );

    // Should include Line 16, 17, 18 (3 lines after symbol ending at line 15)
    assert!(
        body.contains("# Line 18"),
        "Context should include line 18 (3 after).\n\nActual body:\n{}",
        body
    );

    // Should NOT include Line 19 (4 lines after)
    assert!(
        !body.contains("# Line 19"),
        "Context should NOT include line 19 (only 3 lines requested).\n\nActual body:\n{}",
        body
    );
}

/// Test: --context 1 extracts exactly 1 line before and after
///
/// Minimal context case
#[test]
fn test_context_lines_one_line() {
    let temp_dir = setup_test_project_with_known_lines();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validateUser",
            "--context",
            "1",
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

    assert!(!results.is_empty(), "Expected results");

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Should include exactly 1 line before (line 9)
    assert!(
        body.contains("# Line 9"),
        "Context should include 1 line before.\n\nActual body:\n{}",
        body
    );

    // Should NOT include line 8
    assert!(
        !body.contains("# Line 8"),
        "Context should NOT include line 8 (only 1 line requested).\n\nActual body:\n{}",
        body
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Edge Case: Boundary Clamping Tests (CTX-002)
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context N at file start clamps to line 1
///
/// Given: A function "first_function" starting at line 1
/// When: treelint search first_function --context 10 is executed
/// Then: Context starts from line 1 (not negative), no error
#[test]
fn test_context_lines_clamps_at_file_start() {
    let temp_dir = setup_test_project_function_at_start();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Requesting 10 lines before, but function is at line 1
    // Should not crash or return negative lines
    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "first_function",
            "--context",
            "10",
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

    assert!(!results.is_empty(), "Expected results for 'first_function'");

    // The body should start with the function definition (no lines before it exist)
    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    assert!(
        body.contains("def first_function"),
        "Body should contain function.\n\nActual body:\n{}",
        body
    );

    // Should include available lines after
    assert!(
        body.contains("# Line 4") || body.contains("# Line 5"),
        "Body should include lines after function.\n\nActual body:\n{}",
        body
    );
}

/// Test: --context N at file end clamps to last line
///
/// Given: A function "last_function" at end of file
/// When: treelint search last_function --context 10 is executed
/// Then: Context ends at last line (not beyond), no error
#[test]
fn test_context_lines_clamps_at_file_end() {
    let temp_dir = setup_test_project_function_at_end();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Requesting 10 lines after, but function is at end of file
    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "last_function",
            "--context",
            "10",
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

    assert!(!results.is_empty(), "Expected results for 'last_function'");

    // The body should contain the function
    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    assert!(
        body.contains("def last_function"),
        "Body should contain function.\n\nActual body:\n{}",
        body
    );

    // Should include available lines before
    assert!(
        body.contains("# Line 1") || body.contains("# Line 2"),
        "Body should include lines before function.\n\nActual body:\n{}",
        body
    );
}

// ──────────────────────────────────────────────────────────────────────────
// JSON context_mode Field Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context N sets context_mode to "lines:N" in JSON
///
/// Given: A search with --context 5
/// When: treelint search validateUser --context 5 --format json is executed
/// Then: query.context_mode is "lines:5"
#[test]
fn test_context_lines_sets_context_mode_lines_n() {
    let temp_dir = setup_test_project_with_known_lines();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validateUser",
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
        "query.context_mode must be 'lines:5' when --context 5 is used.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

/// Test: --context 10 sets context_mode to "lines:10"
#[test]
fn test_context_lines_sets_context_mode_lines_10() {
    let temp_dir = setup_test_project_with_known_lines();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validateUser",
            "--context",
            "10",
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
        Some("lines:10"),
        "query.context_mode must be 'lines:10'.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Text Format with Line Context
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context N in text mode includes context lines
///
/// Given: A function with context lines
/// When: treelint search validateUser --context 3 --format text is executed
/// Then: Text output includes the context lines before and after
#[test]
fn test_context_lines_text_format_shows_context() {
    let temp_dir = setup_test_project_with_known_lines();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validateUser",
            "--context",
            "3",
            "--format",
            "text",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Should include context lines in text output
    assert!(
        stdout.contains("# Line 7") || stdout.contains("# Line 8") || stdout.contains("# Line 9"),
        "Text output should include context lines before symbol.\n\nActual output:\n{}",
        stdout
    );

    assert!(
        stdout.contains("# Line 16")
            || stdout.contains("# Line 17")
            || stdout.contains("# Line 18"),
        "Text output should include context lines after symbol.\n\nActual output:\n{}",
        stdout
    );
}
