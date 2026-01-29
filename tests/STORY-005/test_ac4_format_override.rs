//! AC#4: Format Flag Override
//!
//! Tests that:
//! - --format json explicitly forces JSON output regardless of TTY
//! - --format text explicitly forces text output even when piped
//! - Text without colors works when piped (non-TTY)
//! - Format flag is correctly reflected in output metadata
//!
//! TDD Phase: RED - These tests should FAIL until format override is implemented.
//!
//! Technical Specification Requirements:
//! - --format json produces JSON when stdout is TTY
//! - --format text works when piped (text without ANSI colors)

use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test project
fn setup_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("validator.py");
    fs::write(
        &sample_file,
        r#"
def validate(data: dict) -> bool:
    """Validate input data."""
    if not data:
        return False
    return True

class Validator:
    """Data validator class."""

    def check(self, field: str) -> bool:
        """Check if field is valid."""
        return len(field) > 0
"#,
    )
    .expect("Failed to write validator.py");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// --format json Override Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --format json flag produces valid JSON output
///
/// Given: A project with indexed source files
/// When: treelint search validate --format json is executed
/// Then: Output is valid JSON
#[test]
fn test_format_json_flag_produces_valid_json() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    let json: Result<Value, _> = serde_json::from_str(&stdout);
    assert!(
        json.is_ok(),
        "--format json must produce valid JSON output.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --format json overrides TTY to produce JSON (piped scenario)
///
/// Given: stdout is piped (via assert_cmd)
/// When: treelint search validate --format json is executed
/// Then: Output is valid JSON (explicit flag matches piped default)
#[test]
fn test_format_json_flag_works_when_piped() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    let json: Value = serde_json::from_str(&stdout).expect("--format json must produce valid JSON");

    // Verify it's properly formatted JSON with required fields
    assert!(json.get("query").is_some(), "JSON must have query object");
    assert!(
        json.get("results").is_some(),
        "JSON must have results array"
    );
    assert!(json.get("stats").is_some(), "JSON must have stats object");
}

// ──────────────────────────────────────────────────────────────────────────
// --format text Override Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --format text flag produces human-readable text output
///
/// Given: stdout is piped (via assert_cmd - normally would default to JSON)
/// When: treelint search validate --format text is executed
/// Then: Output is human-readable text (NOT JSON)
#[test]
fn test_format_text_flag_produces_text_output() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Text output should NOT be valid JSON (it's human-readable text)
    let json_result: Result<Value, _> = serde_json::from_str(&stdout);
    assert!(
        json_result.is_err(),
        "--format text should produce non-JSON text output when piped.\n\nActual output:\n{}",
        stdout
    );

    // Should contain human-readable elements
    assert!(
        stdout.contains("validate"),
        "Text output must contain the symbol name.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --format text when piped does not include ANSI color codes
///
/// Given: stdout is piped (non-TTY)
/// When: treelint search validate --format text is executed
/// Then: Output does not contain ANSI escape sequences
#[test]
fn test_format_text_piped_no_ansi_colors() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // ANSI escape sequences start with \x1b[ (ESC[)
    let has_ansi = stdout.contains("\x1b[");
    assert!(
        !has_ansi,
        "Text output when piped (non-TTY) must NOT contain ANSI color codes.\n\nActual output bytes contain ESC sequences."
    );
}

/// Test: --format text includes summary line even when piped
///
/// Given: stdout is piped
/// When: treelint search validate --format text is executed
/// Then: Output still includes summary line
#[test]
fn test_format_text_piped_includes_summary() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    let has_summary = stdout.lines().any(|line| {
        line.to_lowercase().contains("found") && line.to_lowercase().contains("result")
    });

    assert!(
        has_summary,
        "Text output must include summary even when piped.\n\nActual output:\n{}",
        stdout
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Format Consistency Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --format json produces same data as auto-detected JSON
///
/// Given: stdout is piped (auto-detects as JSON)
/// When: treelint search validate is executed with and without --format json
/// Then: Both outputs have the same results (ignoring elapsed_ms)
#[test]
fn test_format_json_explicit_matches_auto_detected() {
    let temp_dir = setup_test_project();

    // Run without --format (should auto-detect to JSON when piped)
    let mut cmd_auto = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output_auto = cmd_auto
        .current_dir(temp_dir.path())
        .args(["search", "validate"])
        .assert()
        .success();

    let stdout_auto = String::from_utf8_lossy(&output_auto.get_output().stdout);
    let json_auto: Value = serde_json::from_str(&stdout_auto)
        .expect("Auto-detected output should be valid JSON when piped");

    // Run with explicit --format json
    let mut cmd_explicit = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output_explicit = cmd_explicit
        .current_dir(temp_dir.path())
        .args(["search", "validate", "--format", "json"])
        .assert()
        .success();

    let stdout_explicit = String::from_utf8_lossy(&output_explicit.get_output().stdout);
    let json_explicit: Value = serde_json::from_str(&stdout_explicit)
        .expect("Explicit --format json should produce valid JSON");

    // Query and results should match (stats.elapsed_ms will differ)
    assert_eq!(
        json_auto.get("query"),
        json_explicit.get("query"),
        "Query metadata should be identical between auto and explicit JSON"
    );

    assert_eq!(
        json_auto.get("results"),
        json_explicit.get("results"),
        "Results should be identical between auto and explicit JSON"
    );
}

/// Test: --format text with no results shows no-results message (not JSON error)
///
/// Given: No matching symbols
/// When: treelint search nonExistentXYZ --format text is executed
/// Then: Shows human-readable "no results" message (not JSON)
#[test]
fn test_format_text_no_results_shows_text_message() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "nonExistentXYZ", "--format", "text"])
        .assert();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Should NOT be JSON
    let json_result: Result<Value, _> = serde_json::from_str(&stdout);
    assert!(
        json_result.is_err(),
        "--format text with no results should produce text, not JSON.\n\nActual output:\n{}",
        stdout
    );

    // Should contain human-readable no-results message
    let has_message = stdout.to_lowercase().contains("no result")
        || stdout.to_lowercase().contains("not found")
        || stdout.to_lowercase().contains("found 0");

    assert!(
        has_message,
        "Text format with no results should show human-readable message.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --format json with no results still produces valid JSON
///
/// Given: No matching symbols
/// When: treelint search nonExistentXYZ --format json is executed
/// Then: Output is valid JSON with empty results array
#[test]
fn test_format_json_no_results_produces_valid_json() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "nonExistentXYZ", "--format", "json"])
        .assert();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    let json: Value = serde_json::from_str(&stdout)
        .expect("--format json with no results must still produce valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some(),
        "JSON must contain 'results' array even when empty"
    );
    assert_eq!(
        results.unwrap().len(),
        0,
        "results array must be empty when no matches"
    );
}
