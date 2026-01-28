//! AC#1: Basic Exact Match Search Executes Against Index
//!
//! Tests that:
//! - treelint search validateUser returns all symbols where name exactly matches
//! - Output includes file path, line range, signature, and language
//! - Exit code 0 (success) or 2 (no results)
//!
//! TDD Phase: RED - These tests should FAIL against the placeholder implementation.
//!
//! Technical Specification Requirements:
//! - SVC-001: Replace placeholder logic with real index query execution
//! - BR-003: Exit codes: 0=results, 1=error, 2=no results

use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper to create a test project with Python files and index them
fn setup_test_project_with_symbols() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create a Python file with a validateUser function
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let auth_file = src_dir.join("auth.py");
    fs::write(
        &auth_file,
        r#"
def validateUser(email: str, password: str) -> bool:
    """Validate user credentials against the database."""
    if not email or not password:
        return False
    return check_credentials(email, password)

class AuthService:
    """Authentication service for user management."""
    def authenticate(self, user):
        return validateUser(user.email, user.password)
"#,
    )
    .expect("Failed to write auth.py");

    // Create another file with a different function
    let utils_file = src_dir.join("utils.py");
    fs::write(
        &utils_file,
        r#"
def processData(data: list) -> list:
    """Process a list of data items."""
    return [item.strip() for item in data]

def validateEmail(email: str) -> bool:
    """Validate email format."""
    return '@' in email
"#,
    )
    .expect("Failed to write utils.py");

    temp_dir
}

/// Test: Exact match search returns matching symbol from index
///
/// Given: A SQLite index exists with symbols (function validateUser, class AuthService)
/// When: treelint search validateUser is executed
/// Then: Returns all symbols where name exactly matches "validateUser"
#[test]
fn test_search_exact_match_returns_matching_symbol() {
    let temp_dir = setup_test_project_with_symbols();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    // Should return at least one result (the validateUser function)
    let results = json.get("results").and_then(|r| r.as_array());
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected at least one result for 'validateUser', but got empty results.\n\nActual output:\n{}",
        stdout
    );

    // Verify the result has the correct name
    let first_result = &results.unwrap()[0];
    let name = first_result.get("name").and_then(|n| n.as_str());
    assert_eq!(
        name,
        Some("validateUser"),
        "Result name should be 'validateUser'\n\nActual result:\n{:?}",
        first_result
    );
}

/// Test: Search result includes file path
///
/// Given: Index contains symbols with file path metadata
/// When: treelint search validateUser is executed
/// Then: Each result includes file path
#[test]
fn test_search_result_includes_file_path() {
    let temp_dir = setup_test_project_with_symbols();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(results.is_some() && !results.unwrap().is_empty());

    let first_result = &results.unwrap()[0];
    let file = first_result.get("file").and_then(|f| f.as_str());

    assert!(
        file.is_some() && !file.unwrap().is_empty(),
        "Result must include non-empty 'file' path\n\nActual result:\n{:?}",
        first_result
    );

    // File path should contain 'auth' since validateUser is in auth.py
    assert!(
        file.unwrap().contains("auth"),
        "File path should contain 'auth' for validateUser function\n\nActual file: {:?}",
        file
    );
}

/// Test: Search result includes line range
///
/// Given: Index contains symbols with line number metadata
/// When: treelint search validateUser is executed
/// Then: Each result includes line range [start, end]
#[test]
fn test_search_result_includes_line_range() {
    let temp_dir = setup_test_project_with_symbols();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(results.is_some() && !results.unwrap().is_empty());

    let first_result = &results.unwrap()[0];
    let lines = first_result.get("lines").and_then(|l| l.as_array());

    assert!(
        lines.is_some(),
        "Result must include 'lines' array\n\nActual result:\n{:?}",
        first_result
    );

    let lines_arr = lines.unwrap();
    assert_eq!(
        lines_arr.len(),
        2,
        "lines must be a tuple of [start, end]\n\nActual lines: {:?}",
        lines_arr
    );

    // Both start and end should be positive integers
    let start = lines_arr[0].as_u64();
    let end = lines_arr[1].as_u64();
    assert!(
        start.is_some() && end.is_some(),
        "lines values must be positive integers\n\nActual: {:?}",
        lines_arr
    );
    assert!(
        start.unwrap() > 0 && end.unwrap() >= start.unwrap(),
        "line_start must be positive and line_end >= line_start\n\nActual: start={:?}, end={:?}",
        start,
        end
    );
}

/// Test: Search result includes signature
///
/// Given: Index contains symbols with signature metadata
/// When: treelint search validateUser is executed
/// Then: Each result includes function/method signature
#[test]
fn test_search_result_includes_signature() {
    let temp_dir = setup_test_project_with_symbols();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());
    assert!(results.is_some() && !results.unwrap().is_empty());

    let first_result = &results.unwrap()[0];
    let signature = first_result.get("signature").and_then(|s| s.as_str());

    assert!(
        signature.is_some() && !signature.unwrap().is_empty(),
        "Result must include non-empty 'signature'\n\nActual result:\n{:?}",
        first_result
    );

    // Signature should contain function definition elements
    let sig = signature.unwrap();
    assert!(
        sig.contains("validateUser") || sig.contains("def"),
        "Signature should contain function name or 'def' keyword\n\nActual signature: {}",
        sig
    );
}

/// Test: Search result includes language field
///
/// Given: Index contains symbols with language metadata
/// When: treelint search validateUser is executed
/// Then: Each result includes programming language
#[test]
fn test_search_result_includes_language() {
    let temp_dir = setup_test_project_with_symbols();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    // Check if language is included in query metadata or results
    // The story specifies "language" should be in output
    if results.is_some() && !results.unwrap().is_empty() {
        let first_result = &results.unwrap()[0];

        // Language could be in the result or inferred from file extension
        let has_language = first_result.get("language").is_some()
            || first_result
                .get("file")
                .and_then(|f| f.as_str())
                .map(|f| f.ends_with(".py"))
                .unwrap_or(false);

        assert!(
            has_language,
            "Result should include language information or have identifiable file extension\n\nActual result:\n{:?}",
            first_result
        );
    }
}

/// Test: Exit code 0 when results found
///
/// Given: Index contains matching symbols
/// When: treelint search validateUser is executed
/// Then: Exit code is 0 (success with results)
#[test]
fn test_search_exit_code_0_when_results_found() {
    let temp_dir = setup_test_project_with_symbols();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    // If results are found, exit code should be 0
    if results.is_some() && !results.unwrap().is_empty() {
        output.code(0);
    }
}

/// Test: Exit code 2 when no results found
///
/// Given: Index does not contain matching symbols
/// When: treelint search nonExistentSymbol is executed
/// Then: Exit code is 2 (success but no results)
#[test]
fn test_search_exit_code_2_when_no_results() {
    let temp_dir = setup_test_project_with_symbols();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Search for a symbol that definitely doesn't exist
    cmd.current_dir(temp_dir.path())
        .args(["search", "nonExistentSymbolXYZ123", "--format", "json"])
        .assert()
        .code(2);
}

/// Test: Exact match does not return partial matches
///
/// Given: Index contains symbols "validateUser", "validateEmail", "User"
/// When: treelint search User is executed (exact match)
/// Then: Only returns "User", not "validateUser" or "validateEmail"
#[test]
fn test_search_exact_match_no_partial_matches() {
    let temp_dir = setup_test_project_with_symbols();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate", "--format", "json"])
        .assert();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    // "validate" is not an exact symbol name, so should return no results (exit 2)
    // or if partial matching is accidentally enabled, this test will catch it
    if let Some(results_arr) = results {
        for result in results_arr {
            let name = result.get("name").and_then(|n| n.as_str()).unwrap_or("");
            assert_eq!(
                name, "validate",
                "Exact match should only return symbols named exactly 'validate', not partial matches like '{}'\n\nAll results:\n{}",
                name, stdout
            );
        }
    }
}

/// Test: Search returns stats with files_searched > 0
///
/// Given: A project with indexed files
/// When: treelint search validateUser is executed
/// Then: stats.files_searched reflects actual files searched
#[test]
fn test_search_stats_files_searched_nonzero() {
    let temp_dir = setup_test_project_with_symbols();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let files_searched = json
        .get("stats")
        .and_then(|s| s.get("files_searched"))
        .and_then(|f| f.as_u64());

    assert!(
        files_searched.is_some() && files_searched.unwrap() > 0,
        "stats.files_searched must be > 0 when searching against real index\n\nActual stats:\n{:?}",
        json.get("stats")
    );
}

/// Test: Search query metadata reflects search parameters
///
/// Given: A project with indexed files
/// When: treelint search validateUser is executed
/// Then: Query metadata correctly reflects the search symbol
#[test]
fn test_search_query_metadata_correct() {
    let temp_dir = setup_test_project_with_symbols();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let query_symbol = json
        .get("query")
        .and_then(|q| q.get("symbol"))
        .and_then(|s| s.as_str());

    assert_eq!(
        query_symbol,
        Some("validateUser"),
        "query.symbol must match the searched term\n\nActual query:\n{:?}",
        json.get("query")
    );
}
