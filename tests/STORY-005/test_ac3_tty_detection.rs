//! AC#3: TTY Auto-Detection
//!
//! Tests that:
//! - When stdout is TTY: default output format is text
//! - When stdout is piped: default output format is JSON
//! - OutputRouter handles format selection based on TTY detection
//!
//! TDD Phase: RED - These tests should FAIL until OutputRouter and TTY detection
//! are implemented.
//!
//! Technical Specification Requirements:
//! - src/output/mod.rs: OutputRouter that selects format based on TTY/flag
//! - Uses atty crate (or std::io::IsTerminal) for TTY detection
//!
//! Note: Direct TTY testing is hard to mock in unit tests. These tests
//! validate the routing logic and piped behavior (which is non-TTY).

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Helper to create a minimal test project
fn setup_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("greeter.py");
    fs::write(
        &sample_file,
        r#"
def greet(name: str) -> str:
    """Greet someone by name."""
    return f"Hello, {name}!"
"#,
    )
    .expect("Failed to write greeter.py");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// Piped Output (Non-TTY) Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: When stdout is piped (not TTY), default output is JSON
///
/// Given: A project with indexed source files
/// When: treelint search greet is executed and stdout is piped (assert_cmd captures it)
/// Then: Output is valid JSON (auto-detected non-TTY -> JSON)
///
/// Note: assert_cmd captures stdout, so it is always piped (non-TTY).
/// This test validates that the auto-detection correctly selects JSON for piped output.
#[test]
fn test_piped_output_defaults_to_json() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Do NOT specify --format flag. Let auto-detection choose.
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "greet"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // When piped (non-TTY), default should be JSON
    let json: Result<Value, _> = serde_json::from_str(&stdout);
    assert!(
        json.is_ok(),
        "When stdout is piped (non-TTY), default output format should be JSON.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Piped JSON output has valid schema
///
/// Given: A project with indexed source files
/// When: treelint search greet is executed with piped output (no --format)
/// Then: Auto-detected JSON output has query, results, stats fields
#[test]
fn test_piped_output_json_has_valid_schema() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "greet"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout)
        .expect("Piped output should be valid JSON (auto-detected non-TTY)");

    assert!(
        json.get("query").is_some(),
        "Auto-detected JSON must contain 'query' object"
    );
    assert!(
        json.get("results").is_some(),
        "Auto-detected JSON must contain 'results' array"
    );
    assert!(
        json.get("stats").is_some(),
        "Auto-detected JSON must contain 'stats' object"
    );
}

/// Test: Piped output with no results defaults to JSON
///
/// Given: A project with no matching symbols
/// When: treelint search nonExistentXYZ is executed with piped output (no --format)
/// Then: Output is valid JSON even with no results
#[test]
fn test_piped_output_no_results_defaults_to_json() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "nonExistentXYZ"])
        .assert();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    let json: Result<Value, _> = serde_json::from_str(&stdout);
    assert!(
        json.is_ok(),
        "Piped output with no results should still be valid JSON.\n\nActual output:\n{}",
        stdout
    );
}

// ──────────────────────────────────────────────────────────────────────────
// OutputRouter Unit Tests (Library-Level)
// ──────────────────────────────────────────────────────────────────────────

/// Test: OutputRouter exists and can be imported
///
/// Given: The output module
/// When: Attempting to reference OutputRouter
/// Then: The type exists in treelint::output
///
/// This test verifies the OutputRouter type exists (compilation test).
#[test]
fn test_output_router_type_exists() {
    // This test will fail to compile if OutputRouter doesn't exist
    // We can't directly test TTY detection in automated tests,
    // but we can verify the routing struct/function exists.

    // OutputRouter should be accessible from the output module
    use treelint::output::OutputRouter;

    // Should be able to create an OutputRouter instance
    // The exact API will be defined during implementation
    let _router = OutputRouter::new();
}

/// Test: OutputRouter selects JSON format for non-TTY
///
/// Given: OutputRouter configured with is_tty = false
/// When: resolve_format(None) is called (no explicit format)
/// Then: Returns JSON format
#[test]
fn test_output_router_selects_json_for_non_tty() {
    use treelint::cli::args::OutputFormat;
    use treelint::output::OutputRouter;

    let router = OutputRouter::with_tty(false);

    // When no explicit format is specified and stdout is not a TTY
    let resolved = router.resolve_format(None);

    assert_eq!(
        resolved,
        OutputFormat::Json,
        "OutputRouter should select JSON when stdout is not a TTY and no format specified"
    );
}

/// Test: OutputRouter selects Text format for TTY
///
/// Given: OutputRouter configured with is_tty = true
/// When: resolve_format(None) is called (no explicit format)
/// Then: Returns Text format
#[test]
fn test_output_router_selects_text_for_tty() {
    use treelint::cli::args::OutputFormat;
    use treelint::output::OutputRouter;

    let router = OutputRouter::with_tty(true);

    // When no explicit format is specified and stdout is a TTY
    let resolved = router.resolve_format(None);

    assert_eq!(
        resolved,
        OutputFormat::Text,
        "OutputRouter should select Text when stdout is a TTY and no format specified"
    );
}

/// Test: OutputRouter respects explicit format over TTY detection
///
/// Given: OutputRouter configured with is_tty = true
/// When: resolve_format(Some(OutputFormat::Json)) is called
/// Then: Returns JSON format (explicit overrides TTY)
#[test]
fn test_output_router_explicit_format_overrides_tty() {
    use treelint::cli::args::OutputFormat;
    use treelint::output::OutputRouter;

    let router = OutputRouter::with_tty(true);

    // Explicit format should always win over TTY detection
    let resolved = router.resolve_format(Some(OutputFormat::Json));

    assert_eq!(
        resolved,
        OutputFormat::Json,
        "Explicit --format json must override TTY auto-detection"
    );
}

/// Test: OutputRouter resolve_format with explicit text on non-TTY
///
/// Given: OutputRouter configured with is_tty = false
/// When: resolve_format(Some(OutputFormat::Text)) is called
/// Then: Returns Text format (explicit overrides piped detection)
#[test]
fn test_output_router_explicit_text_overrides_piped() {
    use treelint::cli::args::OutputFormat;
    use treelint::output::OutputRouter;

    let router = OutputRouter::with_tty(false);

    let resolved = router.resolve_format(Some(OutputFormat::Text));

    assert_eq!(
        resolved,
        OutputFormat::Text,
        "Explicit --format text must override piped (non-TTY) auto-detection"
    );
}
