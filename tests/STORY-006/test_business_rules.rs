//! Business Rules Tests for STORY-006
//!
//! Tests for:
//! - BR-001: --signatures flag and --context flag are mutually exclusive
//! - BR-002: --context N must be a positive integer when using line mode
//!
//! TDD Phase: RED - These tests should FAIL until business rules are enforced.

use assert_cmd::Command;
use predicates::prelude::*;
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
def test_func() -> str:
    """Test function."""
    return "test"
"#,
    )
    .expect("Failed to write sample.py");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// BR-001: Mutual Exclusivity Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --signatures and --context N are mutually exclusive
///
/// Given: A search command with both --signatures and --context 5
/// When: treelint search foo --signatures --context 5 is executed
/// Then: Error is returned with message about mutual exclusivity
#[test]
fn test_br001_signatures_and_context_n_mutually_exclusive() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "test_func", "--signatures", "--context", "5"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Cannot use")
                .or(predicate::str::contains("cannot be used with"))
                .or(predicate::str::contains("mutually exclusive"))
                .or(predicate::str::contains("conflict")),
        );
}

/// Test: --signatures and --context full are mutually exclusive
///
/// Given: A search command with both --signatures and --context full
/// When: treelint search foo --signatures --context full is executed
/// Then: Error is returned
#[test]
fn test_br001_signatures_and_context_full_mutually_exclusive() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "test_func", "--signatures", "--context", "full"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Cannot use")
                .or(predicate::str::contains("cannot be used with"))
                .or(predicate::str::contains("mutually exclusive"))
                .or(predicate::str::contains("conflict")),
        );
}

/// Test: --context N alone is valid (no error)
#[test]
fn test_br001_context_n_alone_valid() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "test_func", "--context", "5", "--format", "json"])
        .assert()
        .success();
}

/// Test: --signatures alone is valid (no error)
#[test]
fn test_br001_signatures_alone_valid() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "test_func", "--signatures", "--format", "json"])
        .assert()
        .success();
}

/// Test: --context full alone is valid (no error)
#[test]
fn test_br001_context_full_alone_valid() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args([
            "search",
            "test_func",
            "--context",
            "full",
            "--format",
            "json",
        ])
        .assert()
        .success();
}

// ──────────────────────────────────────────────────────────────────────────
// BR-002: Positive Integer Validation Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context 0 is rejected (must be positive)
///
/// Given: A search command with --context 0
/// When: treelint search foo --context 0 is executed
/// Then: Error is returned with message about positive integer
#[test]
fn test_br002_context_zero_rejected() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "test_func", "--context", "0"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("positive")
                .or(predicate::str::contains("greater than"))
                .or(predicate::str::contains("must be"))
                .or(predicate::str::contains("invalid")),
        );
}

/// Test: --context -1 is rejected (negative)
///
/// Given: A search command with --context -1
/// When: treelint search foo --context -1 is executed
/// Then: Error is returned
#[test]
fn test_br002_context_negative_rejected() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "test_func", "--context", "-1"])
        .assert()
        .failure();
}

/// Test: --context -100 is rejected (large negative)
#[test]
fn test_br002_context_large_negative_rejected() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "test_func", "--context", "-100"])
        .assert()
        .failure();
}

/// Test: --context abc is rejected (non-numeric)
///
/// Given: A search command with --context abc (not a number)
/// When: treelint search foo --context abc is executed
/// Then: Error is returned
#[test]
fn test_br002_context_non_numeric_rejected() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "test_func", "--context", "abc"])
        .assert()
        .failure();
}

/// Test: --context 1.5 is rejected (float)
///
/// Given: A search command with --context 1.5 (float, not integer)
/// When: treelint search foo --context 1.5 is executed
/// Then: Error is returned
#[test]
fn test_br002_context_float_rejected() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "test_func", "--context", "1.5"])
        .assert()
        .failure();
}

/// Test: --context "" (empty) is rejected
#[test]
fn test_br002_context_empty_rejected() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "test_func", "--context", ""])
        .assert()
        .failure();
}

/// Test: --context 1 is valid (minimum positive)
#[test]
fn test_br002_context_one_valid() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "test_func", "--context", "1", "--format", "json"])
        .assert()
        .success();
}

/// Test: --context 100 is valid (large positive)
#[test]
fn test_br002_context_large_positive_valid() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args([
            "search",
            "test_func",
            "--context",
            "100",
            "--format",
            "json",
        ])
        .assert()
        .success();
}

/// Test: --context full is valid (special keyword, not number)
#[test]
fn test_br002_context_full_valid() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args([
            "search",
            "test_func",
            "--context",
            "full",
            "--format",
            "json",
        ])
        .assert()
        .success();
}

// ──────────────────────────────────────────────────────────────────────────
// Error Message Quality Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Mutual exclusivity error message is helpful
///
/// Given: --signatures and --context used together
/// When: Error is returned
/// Then: Error message mentions both flags
#[test]
fn test_error_message_mentions_both_flags() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "test_func", "--signatures", "--context", "5"])
        .assert()
        .failure();

    let stderr = String::from_utf8_lossy(&output.get_output().stderr);

    // Error should mention at least one of the conflicting flags
    assert!(
        stderr.contains("--signatures") || stderr.contains("--context"),
        "Error message should mention conflicting flag names.\n\nActual stderr:\n{}",
        stderr
    );
}

/// Test: Invalid context value error message is helpful
///
/// Given: --context 0 (invalid)
/// When: Error is returned
/// Then: Error message explains why value is invalid
#[test]
fn test_error_message_explains_invalid_value() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "test_func", "--context", "0"])
        .assert()
        .failure();

    let stderr = String::from_utf8_lossy(&output.get_output().stderr);

    // Error should be descriptive (not just "error")
    assert!(
        stderr.len() > 20,
        "Error message should be descriptive.\n\nActual stderr:\n{}",
        stderr
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Edge Case: Case Sensitivity Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context FULL (uppercase) is rejected or normalized
///
/// Note: Whether this passes depends on implementation choice.
/// If case-insensitive: test should pass
/// If case-sensitive: test should fail with error
#[test]
fn test_context_full_case_sensitivity() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Try uppercase "FULL"
    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "test_func",
            "--context",
            "FULL",
            "--format",
            "json",
        ])
        .assert();

    // Either it succeeds (case-insensitive) or fails with helpful error
    // The test verifies behavior is defined, not random
    let exit_code = output.get_output().status.code().unwrap_or(-1);

    if exit_code == 0 {
        // Case-insensitive implementation: verify context_mode is correct
        let stdout = String::from_utf8_lossy(&output.get_output().stdout);
        let json: Value = serde_json::from_str(&stdout).expect("Valid JSON");
        let context_mode = json
            .get("query")
            .and_then(|q| q.get("context_mode"))
            .and_then(|c| c.as_str());

        // Should normalize to "full"
        assert_eq!(
            context_mode,
            Some("full"),
            "If case-insensitive, FULL should normalize to 'full'"
        );
    } else {
        // Case-sensitive implementation: verify error is about invalid value
        let stderr = String::from_utf8_lossy(&output.get_output().stderr);
        assert!(
            stderr.contains("invalid") || stderr.contains("FULL") || stderr.contains("expected"),
            "If case-sensitive, error should explain 'FULL' is invalid"
        );
    }
}
