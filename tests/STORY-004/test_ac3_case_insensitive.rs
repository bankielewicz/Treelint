//! AC#3: Case-Insensitive Search Works
//!
//! Tests that:
//! - treelint search validateuser -i returns case variants (ValidateUser, validateUser, VALIDATEUSER)
//! - Query metadata shows case_insensitive: true
//!
//! TDD Phase: RED - These tests should FAIL against the placeholder implementation.
//!
//! Technical Specification Requirements:
//! - SVC-003: Implement case-insensitive search using query_by_name_case_insensitive()
//! - SVC-006: Combine multiple filters with AND logic

use assert_cmd::Command;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test project with case-variant symbol names
fn setup_test_project_with_case_variants() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create multiple files with different case variants of "validateUser"
    let file1 = src_dir.join("auth_camel.py");
    fs::write(
        &file1,
        r#"
def validateUser(email: str, password: str) -> bool:
    """Camel case variant - validateUser."""
    return True
"#,
    )
    .expect("Failed to write auth_camel.py");

    let file2 = src_dir.join("auth_pascal.py");
    fs::write(
        &file2,
        r#"
def ValidateUser(email: str, password: str) -> bool:
    """Pascal case variant - ValidateUser."""
    return True
"#,
    )
    .expect("Failed to write auth_pascal.py");

    let file3 = src_dir.join("auth_upper.py");
    fs::write(
        &file3,
        r#"
def VALIDATEUSER(email: str, password: str) -> bool:
    """Upper case variant - VALIDATEUSER."""
    return True
"#,
    )
    .expect("Failed to write auth_upper.py");

    let file4 = src_dir.join("auth_lower.py");
    fs::write(
        &file4,
        r#"
def validateuser(email: str, password: str) -> bool:
    """Lower case variant - validateuser."""
    return True
"#,
    )
    .expect("Failed to write auth_lower.py");

    // Create a file with an unrelated function
    let file5 = src_dir.join("utils.py");
    fs::write(
        &file5,
        r#"
def processData(data):
    """Unrelated function."""
    return data
"#,
    )
    .expect("Failed to write utils.py");

    temp_dir
}

/// Test: Case-insensitive flag returns all case variants
///
/// Given: An index contains symbols "ValidateUser", "validateUser", and "VALIDATEUSER"
/// When: treelint search validateuser -i is executed
/// Then: All three symbols are returned regardless of case
#[test]
fn test_search_case_insensitive_returns_all_variants() {
    let temp_dir = setup_test_project_with_case_variants();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateuser", "-i", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && results.unwrap().len() >= 3,
        "Case-insensitive search should return at least 3 variants\n\nActual output:\n{}",
        stdout
    );

    // Collect the names found
    let names: Vec<&str> = results
        .unwrap()
        .iter()
        .filter_map(|r| r.get("name").and_then(|n| n.as_str()))
        .collect();

    // Check that we have multiple case variants
    let has_camel = names.iter().any(|n| *n == "validateUser");
    let has_pascal = names.iter().any(|n| *n == "ValidateUser");
    let has_upper = names.iter().any(|n| *n == "VALIDATEUSER");
    let has_lower = names.iter().any(|n| *n == "validateuser");

    assert!(
        has_camel || has_pascal || has_upper || has_lower,
        "Should find at least one case variant of validateuser\nFound names: {:?}",
        names
    );

    // Count how many variants we found
    let variant_count = [has_camel, has_pascal, has_upper, has_lower]
        .iter()
        .filter(|&&x| x)
        .count();

    assert!(
        variant_count >= 3,
        "Case-insensitive search should find at least 3 case variants\nFound {} variants: {:?}",
        variant_count,
        names
    );
}

/// Test: Case-insensitive flag with uppercase input
///
/// Given: An index contains "validateUser", "ValidateUser", "VALIDATEUSER"
/// When: treelint search VALIDATEUSER -i is executed
/// Then: All case variants are returned
#[test]
fn test_search_case_insensitive_with_uppercase_input() {
    let temp_dir = setup_test_project_with_case_variants();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "VALIDATEUSER", "-i", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && results.unwrap().len() >= 3,
        "Case-insensitive search with uppercase input should find all variants\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Case-insensitive flag with mixed case input
///
/// Given: An index contains case variants
/// When: treelint search VaLiDaTeUsEr -i is executed
/// Then: All case variants are returned
#[test]
fn test_search_case_insensitive_with_mixed_case_input() {
    let temp_dir = setup_test_project_with_case_variants();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "VaLiDaTeUsEr", "-i", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && results.unwrap().len() >= 3,
        "Case-insensitive search should work with any case input\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Query metadata shows case_insensitive flag
///
/// Given: Case-insensitive flag is used
/// When: treelint search validateuser -i is executed
/// Then: Query metadata shows case_insensitive: true
#[test]
fn test_search_query_metadata_shows_case_insensitive_true() {
    let temp_dir = setup_test_project_with_case_variants();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateuser", "-i", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let case_insensitive = json
        .get("query")
        .and_then(|q| q.get("case_insensitive"))
        .and_then(|c| c.as_bool());

    assert_eq!(
        case_insensitive,
        Some(true),
        "query.case_insensitive must be true when -i flag is used\n\nActual query:\n{:?}",
        json.get("query")
    );
}

/// Test: Without -i flag, search is case-sensitive
///
/// Given: An index contains "validateUser", "ValidateUser", "VALIDATEUSER"
/// When: treelint search validateUser is executed (no -i flag)
/// Then: Only exact case match "validateUser" is returned
#[test]
fn test_search_without_i_flag_is_case_sensitive() {
    let temp_dir = setup_test_project_with_case_variants();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    if results.is_some() && !results.unwrap().is_empty() {
        // All results must be exact case match
        for result in results.unwrap() {
            let name = result.get("name").and_then(|n| n.as_str());
            assert_eq!(
                name,
                Some("validateUser"),
                "Without -i flag, should only return exact case match\n\nActual result:\n{:?}",
                result
            );
        }
    }
}

/// Test: Case-insensitive search excludes non-matching names
///
/// Given: An index contains "validateUser" and "processData"
/// When: treelint search validateuser -i is executed
/// Then: "processData" is not returned (different base name)
#[test]
fn test_search_case_insensitive_excludes_different_names() {
    let temp_dir = setup_test_project_with_case_variants();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateuser", "-i", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    if let Some(results_arr) = results {
        for result in results_arr {
            let name = result.get("name").and_then(|n| n.as_str()).unwrap_or("");
            // All results should be case-variants of "validateuser"
            assert!(
                name.to_lowercase() == "validateuser",
                "Case-insensitive search should only return variants of 'validateuser', not '{}'\n\nAll results:\n{}",
                name,
                stdout
            );
        }
    }
}

/// Test: Long form --ignore-case flag works
///
/// Given: Case variants exist in index
/// When: treelint search validateuser --ignore-case is executed
/// Then: All case variants are returned
#[test]
fn test_search_long_form_ignore_case_flag_works() {
    let temp_dir = setup_test_project_with_case_variants();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validateuser",
            "--ignore-case",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && results.unwrap().len() >= 3,
        "--ignore-case long form should work like -i short form\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Case-insensitive combined with type filter
///
/// Given: Index contains function "validateUser" and class "ValidateUser"
/// When: treelint search validateuser -i --type function is executed
/// Then: Only function case variants are returned (AND logic)
#[test]
fn test_search_case_insensitive_combined_with_type_filter() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create a function and a class with case-variant names
    let file = src_dir.join("mixed.py");
    fs::write(
        &file,
        r#"
def validateUser(email):
    """Function validateUser."""
    return True

class ValidateUser:
    """Class ValidateUser."""
    pass

def VALIDATEUSER(x):
    """Function VALIDATEUSER."""
    return x
"#,
    )
    .expect("Failed to write mixed.py");

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "validateuser",
            "-i",
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
        "Should find function case variants\n\nActual output:\n{}",
        stdout
    );

    // All results should be functions (not classes)
    for result in results.unwrap() {
        let symbol_type = result.get("type").and_then(|t| t.as_str());
        assert_eq!(
            symbol_type,
            Some("function"),
            "Case-insensitive with type filter should only return functions\n\nActual result:\n{:?}",
            result
        );

        let name = result.get("name").and_then(|n| n.as_str()).unwrap_or("");
        assert!(
            name.to_lowercase() == "validateuser",
            "Should only return validateuser variants, not '{}'\n",
            name
        );
    }
}

/// Test: Query metadata shows case_insensitive: false when -i not used
///
/// Given: Case-insensitive flag is NOT used
/// When: treelint search validateUser is executed
/// Then: Query metadata shows case_insensitive: false (or not present)
#[test]
fn test_search_query_metadata_case_insensitive_false_without_flag() {
    let temp_dir = setup_test_project_with_case_variants();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let case_insensitive = json.get("query").and_then(|q| q.get("case_insensitive"));

    // Should be false or not present (null)
    if let Some(ci) = case_insensitive {
        if !ci.is_null() {
            let ci_bool = ci.as_bool();
            assert_eq!(
                ci_bool,
                Some(false),
                "query.case_insensitive should be false when -i is not used\n\nActual query:\n{:?}",
                json.get("query")
            );
        }
    }
    // If not present, that's also acceptable (implicit false)
}
