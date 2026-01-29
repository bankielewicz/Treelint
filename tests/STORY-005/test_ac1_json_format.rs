//! AC#1: JSON Output Format Matches Schema
//!
//! Tests that:
//! - JSON output is valid JSON conforming to expected schema
//! - Query object contains: symbol, type, case_insensitive, regex, context_mode
//! - Results array contains: type, name, file, lines, signature, body, language per item
//! - Stats object contains: files_searched, files_skipped, skipped_by_type,
//!   languages_searched, elapsed_ms
//! - Special characters are properly escaped per RFC 8259
//!
//! TDD Phase: RED - These tests should FAIL until output formatting is implemented.
//!
//! Technical Specification Requirements:
//! - SearchQuery must include context_mode field ("full" by default)
//! - SearchStats must include files_skipped, skipped_by_type, languages_searched
//! - SearchResult.body must be Option<String> for signatures mode

use assert_cmd::Command;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test project with Python files for output format testing
fn setup_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("calculator.py");
    fs::write(
        &sample_file,
        r#"
def add(a: int, b: int) -> int:
    """Add two numbers together."""
    return a + b

def subtract(a: int, b: int) -> int:
    """Subtract b from a."""
    return a - b

class Calculator:
    """A simple calculator class."""

    def multiply(self, a: int, b: int) -> int:
        """Multiply two numbers."""
        return a * b
"#,
    )
    .expect("Failed to write calculator.py");

    temp_dir
}

/// Helper to create a test project with special characters in symbol names/paths
fn setup_test_project_with_special_chars() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("special.py");
    fs::write(
        &sample_file,
        r#"
def process_data(input_str: str) -> str:
    """Process data with "quotes" and \backslashes."""
    return input_str.replace("\n", "\\n")

MESSAGE = "Hello\tWorld\n"

def handle_unicode():
    """Handle unicode: cafe\u0301"""
    pass
"#,
    )
    .expect("Failed to write special.py");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// Schema Validation Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: JSON output is valid JSON
///
/// Given: A project with indexed source files
/// When: treelint search add --format json is executed
/// Then: stdout contains valid, parseable JSON
#[test]
fn test_json_output_is_valid_json() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    let json: Result<Value, _> = serde_json::from_str(&stdout);
    assert!(
        json.is_ok(),
        "Output must be valid JSON.\n\nActual stdout:\n{}",
        stdout
    );
}

/// Test: JSON output has top-level query object
///
/// Given: A project with indexed source files
/// When: treelint search add --format json is executed
/// Then: JSON root contains "query" object
#[test]
fn test_json_output_has_query_object() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    assert!(
        json.get("query").is_some(),
        "JSON output must contain 'query' object.\n\nActual JSON:\n{}",
        serde_json::to_string_pretty(&json).unwrap()
    );
}

/// Test: Query object contains context_mode field
///
/// Given: A project with indexed source files
/// When: treelint search add --format json is executed (without --signatures)
/// Then: query.context_mode is "full"
#[test]
fn test_json_query_has_context_mode_field() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

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

/// Test: Query object contains all required fields
///
/// Given: A project with indexed source files
/// When: treelint search add --format json --type function -i is executed
/// Then: query object has symbol, type, case_insensitive, regex, context_mode
#[test]
fn test_json_query_has_all_required_fields() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search", "add", "--format", "json", "--type", "function", "-i",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let query = json.get("query").expect("Missing 'query' object");

    // symbol field
    assert!(
        query.get("symbol").is_some(),
        "query must contain 'symbol' field.\n\nActual query:\n{:?}",
        query
    );
    assert_eq!(
        query.get("symbol").and_then(|s| s.as_str()),
        Some("add"),
        "query.symbol must match search term"
    );

    // type field
    assert!(
        query.get("type").is_some(),
        "query must contain 'type' field when --type specified.\n\nActual query:\n{:?}",
        query
    );

    // case_insensitive field
    assert!(
        query.get("case_insensitive").is_some(),
        "query must contain 'case_insensitive' field when -i specified.\n\nActual query:\n{:?}",
        query
    );

    // context_mode field
    assert!(
        query.get("context_mode").is_some(),
        "query must contain 'context_mode' field.\n\nActual query:\n{:?}",
        query
    );
}

/// Test: Results array items have all required fields
///
/// Given: A project with indexed source files
/// When: treelint search add --format json is executed
/// Then: Each result has type, name, file, lines, signature, body, language
#[test]
fn test_json_result_items_have_all_required_fields() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("JSON output must contain 'results' array");

    assert!(
        !results.is_empty(),
        "Expected at least one result for 'add'\n\nActual output:\n{}",
        stdout
    );

    let required_fields = [
        "type",
        "name",
        "file",
        "lines",
        "signature",
        "body",
        "language",
    ];

    for (i, result) in results.iter().enumerate() {
        for field in &required_fields {
            assert!(
                result.get(field).is_some(),
                "Result[{}] missing required field '{}'\n\nActual result:\n{:?}",
                i,
                field,
                result
            );
        }
    }
}

/// Test: Result body field is a string (not null) in full mode
///
/// Given: A project with indexed source files
/// When: treelint search add --format json is executed (no --signatures)
/// Then: Each result.body is a non-null string containing function body
#[test]
fn test_json_result_body_is_string_in_full_mode() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Missing 'results' array");

    assert!(!results.is_empty(), "Expected at least one result");

    for (i, result) in results.iter().enumerate() {
        let body = result.get("body");
        assert!(
            body.is_some(),
            "Result[{}] must have 'body' field\n\nResult:\n{:?}",
            i,
            result
        );

        // In full mode, body should be a string (not null)
        assert!(
            body.unwrap().is_string(),
            "Result[{}].body must be a string in full context mode, got: {:?}",
            i,
            body
        );

        let body_str = body.unwrap().as_str().unwrap();
        assert!(
            !body_str.is_empty(),
            "Result[{}].body should not be empty in full mode",
            i
        );
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Stats Object Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Stats object has files_searched field
///
/// Given: A project with indexed source files
/// When: treelint search add --format json is executed
/// Then: stats.files_searched is present and numeric
#[test]
fn test_json_stats_has_files_searched() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let files_searched = json
        .get("stats")
        .and_then(|s| s.get("files_searched"))
        .and_then(|f| f.as_u64());

    assert!(
        files_searched.is_some(),
        "stats.files_searched must be present and numeric.\n\nActual stats:\n{:?}",
        json.get("stats")
    );
}

/// Test: Stats object has elapsed_ms field
///
/// Given: A project with indexed source files
/// When: treelint search add --format json is executed
/// Then: stats.elapsed_ms is present and numeric
#[test]
fn test_json_stats_has_elapsed_ms() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let elapsed_ms = json
        .get("stats")
        .and_then(|s| s.get("elapsed_ms"))
        .and_then(|e| e.as_u64());

    assert!(
        elapsed_ms.is_some(),
        "stats.elapsed_ms must be present and numeric.\n\nActual stats:\n{:?}",
        json.get("stats")
    );
}

/// Test: Stats object has files_skipped field
///
/// Given: A project with indexed source files
/// When: treelint search add --format json is executed
/// Then: stats.files_skipped is present (new field for STORY-005)
#[test]
fn test_json_stats_has_files_skipped() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let files_skipped = json.get("stats").and_then(|s| s.get("files_skipped"));

    assert!(
        files_skipped.is_some(),
        "stats must contain 'files_skipped' field (STORY-005 requirement).\n\nActual stats:\n{:?}",
        json.get("stats")
    );
}

/// Test: Stats object has skipped_by_type field
///
/// Given: A project with indexed source files
/// When: treelint search add --format json is executed
/// Then: stats.skipped_by_type is present (new field for STORY-005)
#[test]
fn test_json_stats_has_skipped_by_type() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let skipped_by_type = json.get("stats").and_then(|s| s.get("skipped_by_type"));

    assert!(
        skipped_by_type.is_some(),
        "stats must contain 'skipped_by_type' field (STORY-005 requirement).\n\nActual stats:\n{:?}",
        json.get("stats")
    );
}

/// Test: Stats object has languages_searched field
///
/// Given: A project with Python source files
/// When: treelint search add --format json is executed
/// Then: stats.languages_searched is present and contains searched languages
#[test]
fn test_json_stats_has_languages_searched() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let languages_searched = json.get("stats").and_then(|s| s.get("languages_searched"));

    assert!(
        languages_searched.is_some(),
        "stats must contain 'languages_searched' field (STORY-005 requirement).\n\nActual stats:\n{:?}",
        json.get("stats")
    );

    // languages_searched should be an array
    assert!(
        languages_searched.unwrap().is_array(),
        "stats.languages_searched must be an array.\n\nActual value:\n{:?}",
        languages_searched
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Empty Results Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Empty results produce valid JSON with empty array
///
/// Given: A project with no matching symbols
/// When: treelint search nonExistentXYZ --format json is executed
/// Then: JSON output has empty results array and valid schema
#[test]
fn test_json_empty_results_valid_schema() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "nonExistentXYZ", "--format", "json"])
        .assert();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value =
        serde_json::from_str(&stdout).expect("Output must be valid JSON even with no results");

    // Query must still be present
    assert!(
        json.get("query").is_some(),
        "JSON must contain 'query' even with no results"
    );

    // Results must be an empty array
    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some(),
        "JSON must contain 'results' array even when empty"
    );
    assert_eq!(
        results.unwrap().len(),
        0,
        "results array must be empty when no matches found"
    );

    // Stats must still be present
    assert!(
        json.get("stats").is_some(),
        "JSON must contain 'stats' even with no results"
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Special Character Escaping Tests (RFC 8259)
// ──────────────────────────────────────────────────────────────────────────

/// Test: Special characters in file paths are properly escaped in JSON
///
/// Given: Source files exist and contain symbols
/// When: treelint search process_data --format json is executed
/// Then: File paths are properly JSON-escaped (backslashes, etc.)
#[test]
fn test_json_special_characters_escaped_in_paths() {
    let temp_dir = setup_test_project_with_special_chars();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "process_data", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // The entire output must be valid JSON - this validates escaping
    let json: Value = serde_json::from_str(&stdout).expect(
        "JSON output must be valid - special characters must be properly escaped per RFC 8259",
    );

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for 'process_data'"
    );

    // Verify file path is a valid string
    let file = results.unwrap()[0].get("file").and_then(|f| f.as_str());
    assert!(
        file.is_some(),
        "File path must be a properly escaped string"
    );
}

/// Test: Special characters in body content are properly escaped in JSON
///
/// Given: Source files contain strings with escape sequences
/// When: treelint search process_data --format json is executed
/// Then: Body content with quotes, backslashes are properly JSON-escaped
#[test]
fn test_json_special_characters_escaped_in_body() {
    let temp_dir = setup_test_project_with_special_chars();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "process_data", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // The entire output must be valid JSON - validates all escaping
    let json: Value = serde_json::from_str(&stdout).expect(
        "JSON with special characters in body must still be valid JSON (RFC 8259 compliance)",
    );

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for 'process_data'"
    );

    let body = results.unwrap()[0].get("body").and_then(|b| b.as_str());
    assert!(
        body.is_some(),
        "Body must be a properly escaped string even with special characters"
    );
}

/// Test: Unicode in symbol names is handled correctly in JSON
///
/// Given: Source files may contain unicode identifiers
/// When: treelint search handle_unicode --format json is executed
/// Then: JSON output handles unicode correctly
#[test]
fn test_json_unicode_in_symbol_names_handled() {
    let temp_dir = setup_test_project_with_special_chars();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "handle_unicode", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    let json: Value =
        serde_json::from_str(&stdout).expect("JSON output must be valid even with unicode content");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected results for 'handle_unicode'"
    );

    let name = results.unwrap()[0].get("name").and_then(|n| n.as_str());
    assert_eq!(
        name,
        Some("handle_unicode"),
        "Unicode symbol name must be correctly represented in JSON"
    );
}

/// Test: Result lines field is a two-element array
///
/// Given: A project with indexed source files
/// When: treelint search add --format json is executed
/// Then: Each result.lines is [start, end] with two numeric elements
#[test]
fn test_json_result_lines_is_two_element_array() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Missing results array");

    assert!(!results.is_empty(), "Expected at least one result");

    for (i, result) in results.iter().enumerate() {
        let lines = result
            .get("lines")
            .and_then(|l| l.as_array())
            .unwrap_or_else(|| panic!("Result[{}].lines must be an array", i));

        assert_eq!(
            lines.len(),
            2,
            "Result[{}].lines must be [start, end] tuple, got {} elements",
            i,
            lines.len()
        );

        assert!(
            lines[0].is_number() && lines[1].is_number(),
            "Result[{}].lines values must be numbers, got {:?}",
            i,
            lines
        );
    }
}
