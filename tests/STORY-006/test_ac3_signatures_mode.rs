//! AC#3: Signatures Only Mode (--signatures)
//!
//! Tests that:
//! - `--signatures` returns only the signature field, body is null/omitted
//! - Multiple results all have null body in signatures mode
//! - Signature contains function/method name, parameters, and return type
//! - All metadata fields (type, name, file, lines, language) still present
//!
//! TDD Phase: RED - These tests should FAIL until signatures mode is implemented.
//!
//! Technical Specification Requirements (CTX-005):
//! - Extract signature only, omitting body content
//!
//! Note: Some basic signatures tests exist in STORY-005. These tests focus on
//! STORY-006 specific requirements for context modes integration.

use assert_cmd::Command;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Create a test project with Python functions having typed signatures
fn setup_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("functions.py");
    fs::write(
        &sample_file,
        r#"
from typing import List, Optional

def validate_email(email: str) -> bool:
    """Validate email format."""
    if not email:
        return False
    return "@" in email and "." in email.split("@")[1]

def process_items(items: List[str], limit: Optional[int] = None) -> List[str]:
    """Process a list of items with optional limit."""
    if limit is not None:
        items = items[:limit]
    return [item.strip() for item in items]

def simple_func():
    """A function without type hints."""
    pass

class DataProcessor:
    """Process data."""

    def transform(self, data: dict) -> dict:
        """Transform data."""
        return {k: v.upper() if isinstance(v, str) else v for k, v in data.items()}
"#,
    )
    .expect("Failed to write functions.py");

    temp_dir
}

/// Create a test project with TypeScript functions
fn setup_test_project_typescript() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("utils.ts");
    fs::write(
        &sample_file,
        r#"
export function formatDate(date: Date, format: string = "YYYY-MM-DD"): string {
    // Complex date formatting logic
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
}

export async function fetchData<T>(url: string, options?: RequestInit): Promise<T> {
    const response = await fetch(url, options);
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
}
"#,
    )
    .expect("Failed to write utils.ts");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// Basic Signatures Mode Tests (CTX-005)
// ──────────────────────────────────────────────────────────────────────────

/// Test: --signatures flag makes body null in JSON output
///
/// Given: A function "validate_email" with a multi-line body
/// When: treelint search validate_email --signatures --format json is executed
/// Then: result.body is null (not a string)
#[test]
fn test_signatures_mode_body_is_null() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validate_email",
            "--signatures",
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

    assert!(!results.is_empty(), "Expected results for 'validate_email'");

    for (i, result) in results.iter().enumerate() {
        let body = result.get("body");

        // Body should be null or absent
        let body_is_null_or_absent = body.is_none() || body.unwrap().is_null();

        assert!(
            body_is_null_or_absent,
            "Result[{}].body must be null or absent in signatures mode, got: {:?}",
            i, body
        );
    }
}

/// Test: --signatures flag includes signature field
///
/// Given: A function "validate_email(email: str) -> bool"
/// When: treelint search validate_email --signatures --format json is executed
/// Then: result.signature contains the function signature
#[test]
fn test_signatures_mode_includes_signature_field() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validate_email",
            "--signatures",
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

    let signature = results[0]
        .get("signature")
        .and_then(|s| s.as_str())
        .expect("signature field must be present");

    // Signature should contain function name
    assert!(
        signature.contains("validate_email"),
        "Signature should contain function name.\n\nActual signature:\n{}",
        signature
    );

    // Signature should contain parameter
    assert!(
        signature.contains("email"),
        "Signature should contain parameter name.\n\nActual signature:\n{}",
        signature
    );

    // Signature should contain return type
    assert!(
        signature.contains("bool"),
        "Signature should contain return type.\n\nActual signature:\n{}",
        signature
    );
}

/// Test: --signatures mode preserves all metadata fields
///
/// Given: A function result
/// When: treelint search validate_email --signatures --format json is executed
/// Then: All metadata fields (type, name, file, lines, language) are present
#[test]
fn test_signatures_mode_preserves_metadata() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validate_email",
            "--signatures",
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

    let required_fields = ["type", "name", "file", "lines", "signature", "language"];

    for (i, result) in results.iter().enumerate() {
        for field in &required_fields {
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
// Multiple Results Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --signatures mode with multiple results all have null body
///
/// Given: Multiple functions matching a regex
/// When: treelint search "validate|process" -r --signatures --format json is executed
/// Then: All result bodies are null
#[test]
fn test_signatures_mode_multiple_results_all_null_body() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validate|process",
            "-r",
            "--signatures",
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
        results.len() >= 2,
        "Expected at least 2 results for regex 'validate|process', got {}.\n\nOutput:\n{}",
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
// Signature Content Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Signature for function with complex types
///
/// Given: A function "process_items(items: List[str], limit: Optional[int] = None) -> List[str]"
/// When: treelint search process_items --signatures --format json is executed
/// Then: Signature includes complex type annotations
#[test]
fn test_signatures_mode_complex_types() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "process_items",
            "--signatures",
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

    assert!(!results.is_empty(), "Expected results for 'process_items'");

    let signature = results[0]
        .get("signature")
        .and_then(|s| s.as_str())
        .expect("signature field must be present");

    // Should contain function name
    assert!(
        signature.contains("process_items"),
        "Signature should contain function name.\n\nActual: {}",
        signature
    );

    // Should contain List type
    assert!(
        signature.contains("List"),
        "Signature should contain List type.\n\nActual: {}",
        signature
    );

    // Should contain Optional type
    assert!(
        signature.contains("Optional"),
        "Signature should contain Optional type.\n\nActual: {}",
        signature
    );
}

/// Test: Signature for function without type hints
///
/// Given: A function "simple_func()" without type hints
/// When: treelint search simple_func --signatures --format json is executed
/// Then: Signature is present (even without types)
#[test]
fn test_signatures_mode_no_type_hints() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "simple_func", "--signatures", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(!results.is_empty(), "Expected results for 'simple_func'");

    let signature = results[0]
        .get("signature")
        .and_then(|s| s.as_str())
        .expect("signature field must be present");

    assert!(
        signature.contains("simple_func"),
        "Signature should contain function name even without type hints.\n\nActual: {}",
        signature
    );
}

/// Test: Signature for method in class
///
/// Given: A method "transform" in class "DataProcessor"
/// When: treelint search transform --signatures --type method --format json is executed
/// Then: Signature includes method with self parameter
#[test]
fn test_signatures_mode_method_signature() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "transform",
            "--signatures",
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
        "Expected results for 'transform' method"
    );

    let signature = results[0]
        .get("signature")
        .and_then(|s| s.as_str())
        .expect("signature field must be present");

    // Should contain method name
    assert!(
        signature.contains("transform"),
        "Signature should contain method name.\n\nActual: {}",
        signature
    );

    // Should contain self parameter
    assert!(
        signature.contains("self"),
        "Method signature should contain 'self'.\n\nActual: {}",
        signature
    );
}

// ──────────────────────────────────────────────────────────────────────────
// TypeScript Signatures Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: TypeScript function signature with generics
///
/// Given: A TypeScript function "fetchData<T>(url: string, options?: RequestInit): Promise<T>"
/// When: treelint search fetchData --signatures --format json is executed
/// Then: Signature includes generics
#[test]
fn test_signatures_mode_typescript_generics() {
    let temp_dir = setup_test_project_typescript();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "fetchData", "--signatures", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(!results.is_empty(), "Expected results for 'fetchData'");

    let signature = results[0]
        .get("signature")
        .and_then(|s| s.as_str())
        .expect("signature field must be present");

    // Should contain function name
    assert!(
        signature.contains("fetchData"),
        "Signature should contain function name.\n\nActual: {}",
        signature
    );

    // Should contain generic type parameter
    assert!(
        signature.contains("<T>") || signature.contains("< T >"),
        "Signature should contain generic type <T>.\n\nActual: {}",
        signature
    );

    // Should contain Promise return type
    assert!(
        signature.contains("Promise"),
        "Signature should contain Promise return type.\n\nActual: {}",
        signature
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Text Format Signatures Mode Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --signatures in text mode shows signature but not body
///
/// Given: A function with a multi-line body
/// When: treelint search validate_email --signatures --format text is executed
/// Then: Text output includes signature but NOT body code
#[test]
fn test_signatures_mode_text_no_body() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validate_email",
            "--signatures",
            "--format",
            "text",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Should include function name/signature
    assert!(
        stdout.contains("validate_email"),
        "Text output should include function name.\n\nActual:\n{}",
        stdout
    );

    // Should NOT include body logic
    assert!(
        !stdout.contains("return \"@\" in email"),
        "Signatures mode should NOT include function body.\n\nActual:\n{}",
        stdout
    );
}

// ──────────────────────────────────────────────────────────────────────────
// JSON context_mode Field Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --signatures sets context_mode to "signatures" in JSON
///
/// Given: A search with --signatures
/// When: treelint search foo --signatures --format json is executed
/// Then: query.context_mode is "signatures"
#[test]
fn test_signatures_mode_sets_context_mode() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validate_email",
            "--signatures",
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
        Some("signatures"),
        "query.context_mode must be 'signatures' when --signatures is used.\n\nActual query:\n{:?}",
        json.get("query")
    );
}
