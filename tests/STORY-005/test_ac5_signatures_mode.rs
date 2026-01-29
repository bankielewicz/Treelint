//! AC#5: Signatures-Only Mode
//!
//! Tests that:
//! - --signatures flag causes JSON body to be null/omitted
//! - --signatures flag causes text to show only header and signature (no body)
//! - context_mode is "signatures" when --signatures is used
//! - context_mode is "full" when --signatures is NOT used
//!
//! TDD Phase: RED - These tests should FAIL until signatures mode is implemented.
//!
//! Technical Specification Requirements:
//! - SearchResult.body must be Option<String> (null when --signatures)
//! - SearchQuery.context_mode must be "signatures" when flag is active
//! - Text formatter omits body section with --signatures

use assert_cmd::Command;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test project with functions that have bodies
fn setup_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("math_utils.py");
    fs::write(
        &sample_file,
        r#"
def factorial(n: int) -> int:
    """Calculate the factorial of n."""
    if n <= 1:
        return 1
    return n * factorial(n - 1)

def fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number."""
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a

class MathHelper:
    """Helper class for math operations."""

    def power(self, base: int, exp: int) -> int:
        """Calculate base raised to exp."""
        return base ** exp
"#,
    )
    .expect("Failed to write math_utils.py");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// JSON Signatures Mode Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --signatures flag makes body null in JSON output
///
/// Given: A project with function "factorial" that has a body
/// When: treelint search factorial --format json --signatures is executed
/// Then: result.body is null (not a string)
#[test]
fn test_signatures_mode_json_body_is_null() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "json", "--signatures"])
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
        "Expected at least one result for 'factorial'"
    );

    for (i, result) in results.iter().enumerate() {
        let body = result.get("body");

        // Body should be null or absent (not a non-empty string)
        let body_is_null_or_absent = body.is_none() || body.unwrap().is_null();

        assert!(
            body_is_null_or_absent,
            "Result[{}].body must be null or absent in signatures mode, got: {:?}",
            i, body
        );
    }
}

/// Test: --signatures flag still includes signature in JSON
///
/// Given: A project with function "factorial"
/// When: treelint search factorial --format json --signatures is executed
/// Then: result.signature is still present (not null)
#[test]
fn test_signatures_mode_json_signature_still_present() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "json", "--signatures"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(!results.is_empty(), "Expected at least one result");

    for (i, result) in results.iter().enumerate() {
        let signature = result.get("signature").and_then(|s| s.as_str());
        assert!(
            signature.is_some() && !signature.unwrap().is_empty(),
            "Result[{}].signature must be present and non-empty in signatures mode.\n\nResult:\n{:?}",
            i,
            result
        );
    }
}

/// Test: --signatures flag still includes all metadata fields in JSON
///
/// Given: A project with function "factorial"
/// When: treelint search factorial --format json --signatures is executed
/// Then: All metadata fields (type, name, file, lines, language) are still present
#[test]
fn test_signatures_mode_json_metadata_still_present() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "json", "--signatures"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(!results.is_empty(), "Expected at least one result");

    let metadata_fields = ["type", "name", "file", "lines", "signature", "language"];

    for (i, result) in results.iter().enumerate() {
        for field in &metadata_fields {
            assert!(
                result.get(field).is_some(),
                "Result[{}] missing '{}' field in signatures mode.\n\nResult:\n{:?}",
                i,
                field,
                result
            );
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────
// context_mode Field Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: context_mode is "signatures" when --signatures flag is used
///
/// Given: A project with indexed source files
/// When: treelint search factorial --format json --signatures is executed
/// Then: query.context_mode is "signatures"
#[test]
fn test_signatures_mode_context_mode_is_signatures() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "json", "--signatures"])
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
        "query.context_mode must be 'signatures' when --signatures flag is used.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

/// Test: context_mode is "full" when --signatures flag is NOT used
///
/// Given: A project with indexed source files
/// When: treelint search factorial --format json is executed (without --signatures)
/// Then: query.context_mode is "full"
#[test]
fn test_full_mode_context_mode_is_full() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "json"])
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
        "query.context_mode must be 'full' when --signatures is NOT used.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Text Signatures Mode Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --signatures in text mode shows only header and signature (no body)
///
/// Given: A project with function "factorial" that has a multi-line body
/// When: treelint search factorial --format text --signatures is executed
/// Then: Output shows header and signature but NOT the body code
#[test]
fn test_signatures_mode_text_no_body() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "text", "--signatures"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Body content should NOT appear
    // factorial body contains: "if n <= 1:", "return 1", "return n * factorial(n - 1)"
    assert!(
        !stdout.contains("if n <= 1"),
        "Signatures mode text output must NOT include function body.\n\nActual output:\n{}",
        stdout
    );

    assert!(
        !stdout.contains("return n * factorial"),
        "Signatures mode text output must NOT include recursive call body.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --signatures in text mode still shows function signature
///
/// Given: A project with function "factorial(n: int) -> int"
/// When: treelint search factorial --format text --signatures is executed
/// Then: Output includes the function signature
#[test]
fn test_signatures_mode_text_shows_signature() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "text", "--signatures"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Signature should contain the function definition
    assert!(
        stdout.contains("factorial") && (stdout.contains("def") || stdout.contains("n: int")),
        "Signatures mode text output must still include function signature.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: --signatures in text mode still shows header with type/name/file
///
/// Given: A project with function "factorial"
/// When: treelint search factorial --format text --signatures is executed
/// Then: Header line with function type, name, and file is present
#[test]
fn test_signatures_mode_text_shows_header() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "text", "--signatures"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Header should contain function type
    assert!(
        stdout.to_lowercase().contains("function"),
        "Signatures mode text must show symbol type.\n\nActual output:\n{}",
        stdout
    );

    // Header should contain symbol name
    assert!(
        stdout.contains("factorial"),
        "Signatures mode text must show symbol name.\n\nActual output:\n{}",
        stdout
    );

    // Header should contain file path
    assert!(
        stdout.contains("math_utils.py"),
        "Signatures mode text must show file path.\n\nActual output:\n{}",
        stdout
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Signatures Mode with Multiple Results
// ──────────────────────────────────────────────────────────────────────────

/// Test: --signatures mode with multiple results omits all bodies in JSON
///
/// Given: A project with "factorial" and "fibonacci" functions
/// When: treelint search -r "factorial|fibonacci" --format json --signatures is executed
/// Then: All result bodies are null
#[test]
fn test_signatures_mode_json_multiple_results_all_bodies_null() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "factorial|fibonacci",
            "-r",
            "--format",
            "json",
            "--signatures",
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
        results.len() >= 2,
        "Expected at least 2 results for regex 'factorial|fibonacci', got {}.\n\nActual output:\n{}",
        results.len(),
        stdout
    );

    for (i, result) in results.iter().enumerate() {
        let body = result.get("body");
        let body_is_null_or_absent = body.is_none() || body.unwrap().is_null();

        assert!(
            body_is_null_or_absent,
            "Result[{}] body must be null in signatures mode. Got: {:?}",
            i, body
        );
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Signatures Mode Text Output Compactness
// ──────────────────────────────────────────────────────────────────────────

/// Test: --signatures text output is more compact than full output
///
/// Given: A function with a multi-line body
/// When: Comparing --signatures vs full text output
/// Then: Signatures mode produces fewer lines
#[test]
fn test_signatures_mode_text_is_more_compact_than_full() {
    let temp_dir = setup_test_project();

    // Full mode
    let mut cmd_full = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output_full = cmd_full
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "text"])
        .assert()
        .success();
    let stdout_full = String::from_utf8_lossy(&output_full.get_output().stdout);
    let full_line_count = stdout_full.lines().count();

    // Signatures mode
    let mut cmd_sig = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output_sig = cmd_sig
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "text", "--signatures"])
        .assert()
        .success();
    let stdout_sig = String::from_utf8_lossy(&output_sig.get_output().stdout);
    let sig_line_count = stdout_sig.lines().count();

    assert!(
        sig_line_count < full_line_count,
        "Signatures mode ({} lines) must be more compact than full mode ({} lines).\n\nFull output:\n{}\n\nSignatures output:\n{}",
        sig_line_count,
        full_line_count,
        stdout_full,
        stdout_sig
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Full Mode Body Contrast (verifies body IS present without --signatures)
// ──────────────────────────────────────────────────────────────────────────

/// Test: Without --signatures, body IS present in JSON (contrast test)
///
/// Given: A project with function "factorial" that has a body
/// When: treelint search factorial --format json is executed (NO --signatures)
/// Then: result.body is a non-null string containing the function body
#[test]
fn test_full_mode_json_body_is_present() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(!results.is_empty(), "Expected at least one result");

    for (i, result) in results.iter().enumerate() {
        let body = result.get("body");

        assert!(
            body.is_some() && body.unwrap().is_string(),
            "Result[{}].body must be a string in full mode (no --signatures).\n\nResult:\n{:?}",
            i,
            result
        );

        let body_str = body.unwrap().as_str().unwrap();
        assert!(
            !body_str.is_empty(),
            "Result[{}].body should contain function body in full mode",
            i
        );
    }
}

/// Test: Without --signatures, body IS present in text output (contrast test)
///
/// Given: A project with function "factorial" that has a body
/// When: treelint search factorial --format text is executed (NO --signatures)
/// Then: Output includes the function body
#[test]
fn test_full_mode_text_body_is_present() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "factorial", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Body content should appear in full mode
    let has_body_content = stdout.contains("if n <= 1")
        || stdout.contains("return n * factorial")
        || stdout.contains("return 1");

    assert!(
        has_body_content,
        "Full mode text output must include function body.\n\nActual output:\n{}",
        stdout
    );
}
