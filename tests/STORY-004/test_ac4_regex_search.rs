//! AC#4: Regex Pattern Search Works
//!
//! Tests that:
//! - treelint search 'validate.*' -r matches validateUser, validateEmail, validatePassword
//! - Invalid regex returns exit code 1 with helpful message
//!
//! TDD Phase: RED - These tests should FAIL against the placeholder implementation.
//!
//! Technical Specification Requirements:
//! - SVC-004: Implement regex search by compiling pattern and filtering results
//! - SVC-006: Combine multiple filters with AND logic
//! - BR-002: Invalid regex returns error (exit 1), not panic
//! - NFR-003: Regex DoS prevention - timeout at 1 second

use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test project with validate* pattern symbols
fn setup_test_project_with_validate_pattern() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create file with multiple validate* functions
    let validators_file = src_dir.join("validators.py");
    fs::write(
        &validators_file,
        r#"
def validateUser(email: str, password: str) -> bool:
    """Validate user credentials."""
    return bool(email) and bool(password)

def validateEmail(email: str) -> bool:
    """Validate email format."""
    return '@' in email

def validatePassword(password: str) -> bool:
    """Validate password strength."""
    return len(password) >= 8

def validateAge(age: int) -> bool:
    """Validate age is positive."""
    return age > 0

def processUser(user):
    """Process user - should NOT match validate.* pattern."""
    return user

def userValidator(user):
    """User validator - should NOT match ^validate.* pattern."""
    return True
"#,
    )
    .expect("Failed to write validators.py");

    temp_dir
}

/// Test: Regex pattern search matches expected symbols
///
/// Given: An index contains symbols "validateUser", "validateEmail", "processUser", "validatePassword"
/// When: treelint search 'validate.*' -r is executed
/// Then: Symbols matching regex are returned (validateUser, validateEmail, validatePassword)
#[test]
fn test_search_regex_pattern_matches_validate_prefix() {
    let temp_dir = setup_test_project_with_validate_pattern();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate.*", "-r", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Regex 'validate.*' should match validate* symbols\n\nActual output:\n{}",
        stdout
    );

    // Should find at least validateUser, validateEmail, validatePassword, validateAge
    let results_arr = results.unwrap();
    assert!(
        results_arr.len() >= 4,
        "Should find at least 4 validate* functions\n\nFound {} results:\n{}",
        results_arr.len(),
        stdout
    );

    // All results should start with "validate"
    for result in results_arr {
        let name = result.get("name").and_then(|n| n.as_str()).unwrap_or("");
        assert!(
            name.starts_with("validate"),
            "Regex 'validate.*' should only match names starting with 'validate', not '{}'\n\nAll results:\n{}",
            name,
            stdout
        );
    }
}

/// Test: Regex pattern excludes non-matching symbols
///
/// Given: An index contains "validateUser" and "processUser"
/// When: treelint search 'validate.*' -r is executed
/// Then: "processUser" is NOT included in results
#[test]
fn test_search_regex_excludes_non_matching() {
    let temp_dir = setup_test_project_with_validate_pattern();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate.*", "-r", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    if let Some(results_arr) = results {
        for result in results_arr {
            let name = result.get("name").and_then(|n| n.as_str()).unwrap_or("");
            assert_ne!(
                name, "processUser",
                "Regex 'validate.*' should not match 'processUser'\n\nAll results:\n{}",
                stdout
            );
            assert_ne!(
                name, "userValidator",
                "Regex 'validate.*' should not match 'userValidator'\n\nAll results:\n{}",
                stdout
            );
        }
    }
}

/// Test: Regex with anchors works correctly
///
/// Given: An index contains "validateUser" and "userValidator"
/// When: treelint search '^validate' -r is executed
/// Then: Only "validate*" symbols are returned, not "userValidator"
#[test]
fn test_search_regex_with_start_anchor() {
    let temp_dir = setup_test_project_with_validate_pattern();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "^validate", "-r", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    if let Some(results_arr) = results {
        for result in results_arr {
            let name = result.get("name").and_then(|n| n.as_str()).unwrap_or("");
            assert!(
                name.starts_with("validate"),
                "Regex '^validate' should only match names starting with 'validate', not '{}'\n",
                name
            );
        }
    }
}

/// Test: Invalid regex returns exit code 1
///
/// Given: An invalid regex pattern
/// When: treelint search '[invalid' -r is executed
/// Then: Exit code is 1 (error)
#[test]
fn test_search_invalid_regex_returns_exit_1() {
    let temp_dir = setup_test_project_with_validate_pattern();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "[invalid", "-r", "--format", "json"])
        .assert()
        .code(1);
}

/// Test: Invalid regex shows helpful error message
///
/// Given: An invalid regex pattern
/// When: treelint search '[invalid' -r is executed
/// Then: Error message is helpful (mentions regex, parse, error, or pattern)
#[test]
fn test_search_invalid_regex_shows_helpful_message() {
    let temp_dir = setup_test_project_with_validate_pattern();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "[invalid", "-r", "--format", "json"])
        .assert()
        .code(1)
        .stderr(
            predicate::str::contains("regex")
                .or(predicate::str::contains("Regex"))
                .or(predicate::str::contains("pattern"))
                .or(predicate::str::contains("Pattern"))
                .or(predicate::str::contains("parse"))
                .or(predicate::str::contains("Parse"))
                .or(predicate::str::contains("invalid"))
                .or(predicate::str::contains("Invalid"))
                .or(predicate::str::contains("error"))
                .or(predicate::str::contains("Error")),
        );
}

/// Test: Invalid regex does not cause panic
///
/// Given: An invalid regex pattern (unclosed bracket)
/// When: treelint search '[' -r is executed
/// Then: Command exits gracefully (no panic), exit code 1
#[test]
fn test_search_invalid_regex_no_panic() {
    let temp_dir = setup_test_project_with_validate_pattern();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Various invalid regex patterns that might cause panics in bad implementations
    let invalid_patterns = ["[", "(", "(?", "*", "+?+", "\\"];

    for pattern in invalid_patterns {
        let result = Command::cargo_bin("treelint")
            .expect("treelint binary not found")
            .current_dir(temp_dir.path())
            .args(["search", pattern, "-r", "--format", "json"])
            .assert();

        // Should exit with code 1 (error), not panic
        result.code(1);
    }
}

/// Test: Query metadata reflects regex flag
///
/// Given: Regex flag is used
/// When: treelint search 'validate.*' -r is executed
/// Then: Query metadata shows regex: true
#[test]
fn test_search_query_metadata_shows_regex_true() {
    let temp_dir = setup_test_project_with_validate_pattern();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate.*", "-r", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let regex_flag = json
        .get("query")
        .and_then(|q| q.get("regex"))
        .and_then(|r| r.as_bool());

    assert_eq!(
        regex_flag,
        Some(true),
        "query.regex must be true when -r flag is used\n\nActual query:\n{:?}",
        json.get("query")
    );
}

/// Test: Long form --regex flag works
///
/// Given: Regex symbols in index
/// When: treelint search 'validate.*' --regex is executed
/// Then: Regex matching is performed
#[test]
fn test_search_long_form_regex_flag_works() {
    let temp_dir = setup_test_project_with_validate_pattern();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate.*", "--regex", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "--regex long form should work like -r short form\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Regex combined with type filter uses AND logic
///
/// Given: Index contains function "validateUser" and class "ValidateService"
/// When: treelint search 'validate.*' -r --type function is executed
/// Then: Only functions matching the regex are returned
#[test]
fn test_search_regex_combined_with_type_filter() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let file = src_dir.join("mixed.py");
    fs::write(
        &file,
        r#"
def validateUser(email):
    """Function validateUser."""
    return True

class ValidateService:
    """Class ValidateService - should not match with --type function."""
    pass

def validateEmail(email):
    """Function validateEmail."""
    return '@' in email
"#,
    )
    .expect("Failed to write mixed.py");

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "[Vv]alidate.*",
            "-r",
            "--type",
            "function",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Should find function validate* symbols\n\nActual output:\n{}",
        stdout
    );

    // All results should be functions, not classes
    for result in results.unwrap() {
        let symbol_type = result.get("type").and_then(|t| t.as_str());
        assert_eq!(
            symbol_type,
            Some("function"),
            "Regex with type filter should only return functions\n\nActual result:\n{:?}",
            result
        );
    }
}

/// Test: Regex combined with case-insensitive flag
///
/// Given: Index contains "validateUser" and "ValidateEmail"
/// When: treelint search 'validate.*' -r -i is executed
/// Then: Both case variants matching the regex are returned
#[test]
fn test_search_regex_combined_with_case_insensitive() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let file = src_dir.join("validators.py");
    fs::write(
        &file,
        r#"
def validateUser(email):
    """Lowercase prefix - validateUser."""
    return True

def ValidateEmail(email):
    """Uppercase prefix - ValidateEmail."""
    return '@' in email

def VALIDATEPASSWORD(password):
    """All caps - VALIDATEPASSWORD."""
    return len(password) >= 8
"#,
    )
    .expect("Failed to write validators.py");

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate.*", "-r", "-i", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && results.unwrap().len() >= 3,
        "Regex with case-insensitive should match all case variants\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Regex no match returns exit code 2
///
/// Given: No symbols match the regex pattern
/// When: treelint search 'zzz.*' -r is executed
/// Then: Exit code is 2 (no results)
#[test]
fn test_search_regex_no_match_returns_exit_2() {
    let temp_dir = setup_test_project_with_validate_pattern();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "zzz.*nonexistent", "-r", "--format", "json"])
        .assert()
        .code(2);
}

/// Test: Special regex characters work
///
/// Given: A regex with special characters
/// When: treelint search 'validate(User|Email)' -r is executed
/// Then: Only validateUser and validateEmail are matched
#[test]
fn test_search_regex_special_characters() {
    let temp_dir = setup_test_project_with_validate_pattern();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate(User|Email)", "-r", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Regex with alternation should match\n\nActual output:\n{}",
        stdout
    );

    // All results should be either validateUser or validateEmail
    let names: Vec<&str> = results
        .unwrap()
        .iter()
        .filter_map(|r| r.get("name").and_then(|n| n.as_str()))
        .collect();

    for name in &names {
        assert!(
            *name == "validateUser" || *name == "validateEmail",
            "Regex 'validate(User|Email)' should only match 'validateUser' or 'validateEmail', not '{}'\n\nAll names: {:?}",
            name,
            names
        );
    }
}
