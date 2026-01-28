//! AC#2: Type Filtering Narrows Results
//!
//! Tests that:
//! - treelint search process --type function filters to only functions
//! - Query metadata reflects type filter applied
//!
//! TDD Phase: RED - These tests should FAIL against the placeholder implementation.
//!
//! Technical Specification Requirements:
//! - SVC-002: Implement type filtering by passing SymbolType to IndexSearch::query()
//! - SVC-006: Combine multiple filters with AND logic
//! - BR-001: Filters combine with AND logic (type AND case AND regex)

use assert_cmd::Command;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test project with multiple symbol types sharing the same name
fn setup_test_project_with_multiple_types() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create a Python file with function, method, and variable all named "process"
    let processor_file = src_dir.join("processor.py");
    fs::write(
        &processor_file,
        r#"
# Global variable named 'process'
process = "default_processor"

def process(data: list) -> list:
    """Process function that transforms data."""
    return [item.upper() for item in data]

class DataProcessor:
    """Data processor class."""

    def process(self, data):
        """Process method on DataProcessor."""
        return process(data)

class AnotherProcess:
    """Another class with process in the name."""
    pass
"#,
    )
    .expect("Failed to write processor.py");

    // Create another file with more 'process' symbols
    let utils_file = src_dir.join("utils.py");
    fs::write(
        &utils_file,
        r#"
PROCESS_CONSTANT = 42

def process_items(items):
    """Helper function that processes items."""
    return items

def validate(x):
    """A function that doesn't have 'process' in its name."""
    return x is not None
"#,
    )
    .expect("Failed to write utils.py");

    temp_dir
}

/// Test: Type filter returns only functions
///
/// Given: An index contains multiple symbol types (function process, method process, variable process)
/// When: treelint search process --type function is executed
/// Then: Only symbols with type "function" are returned
#[test]
fn test_search_type_filter_function_only_returns_functions() {
    let temp_dir = setup_test_project_with_multiple_types();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search", "process", "--type", "function", "--format", "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some(),
        "Expected results array in output\n\nActual output:\n{}",
        stdout
    );

    let results_arr = results.unwrap();

    // Should have at least one function result
    assert!(
        !results_arr.is_empty(),
        "Expected at least one function named 'process'\n\nActual output:\n{}",
        stdout
    );

    // All results must be of type "function"
    for result in results_arr {
        let symbol_type = result.get("type").and_then(|t| t.as_str());
        assert_eq!(
            symbol_type,
            Some("function"),
            "All results must be of type 'function' when --type function is specified\n\nActual result:\n{:?}",
            result
        );

        let name = result.get("name").and_then(|n| n.as_str());
        assert_eq!(
            name,
            Some("process"),
            "Result name should be 'process'\n\nActual result:\n{:?}",
            result
        );
    }
}

/// Test: Type filter excludes methods when searching for functions
///
/// Given: An index contains "process" as both function and method
/// When: treelint search process --type function is executed
/// Then: Methods named "process" are excluded
#[test]
fn test_search_type_filter_excludes_methods_when_filtering_functions() {
    let temp_dir = setup_test_project_with_multiple_types();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search", "process", "--type", "function", "--format", "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    if let Some(results_arr) = results {
        // No result should have type "method"
        for result in results_arr {
            let symbol_type = result.get("type").and_then(|t| t.as_str());
            assert_ne!(
                symbol_type,
                Some("method"),
                "Type filter 'function' should exclude methods\n\nOffending result:\n{:?}",
                result
            );
        }
    }
}

/// Test: Type filter excludes variables when searching for functions
///
/// Given: An index contains "process" as both function and variable
/// When: treelint search process --type function is executed
/// Then: Variables named "process" are excluded
#[test]
fn test_search_type_filter_excludes_variables_when_filtering_functions() {
    let temp_dir = setup_test_project_with_multiple_types();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search", "process", "--type", "function", "--format", "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    if let Some(results_arr) = results {
        // No result should have type "variable"
        for result in results_arr {
            let symbol_type = result.get("type").and_then(|t| t.as_str());
            assert_ne!(
                symbol_type,
                Some("variable"),
                "Type filter 'function' should exclude variables\n\nOffending result:\n{:?}",
                result
            );
        }
    }
}

/// Test: Type filter for class returns only classes
///
/// Given: An index contains multiple symbol types
/// When: treelint search AnotherProcess --type class is executed
/// Then: Only class symbols are returned
#[test]
fn test_search_type_filter_class_returns_only_classes() {
    let temp_dir = setup_test_project_with_multiple_types();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "DataProcessor",
            "--type",
            "class",
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
        "Expected at least one class result for 'DataProcessor'\n\nActual output:\n{}",
        stdout
    );

    // All results must be of type "class"
    for result in results.unwrap() {
        let symbol_type = result.get("type").and_then(|t| t.as_str());
        assert_eq!(
            symbol_type,
            Some("class"),
            "All results must be of type 'class'\n\nActual result:\n{:?}",
            result
        );
    }
}

/// Test: Type filter for method returns only methods
///
/// Given: An index contains "process" as function, method, and variable
/// When: treelint search process --type method is executed
/// Then: Only method symbols are returned
#[test]
fn test_search_type_filter_method_returns_only_methods() {
    let temp_dir = setup_test_project_with_multiple_types();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "process", "--type", "method", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Expected at least one method result for 'process'\n\nActual output:\n{}",
        stdout
    );

    // All results must be of type "method"
    for result in results.unwrap() {
        let symbol_type = result.get("type").and_then(|t| t.as_str());
        assert_eq!(
            symbol_type,
            Some("method"),
            "All results must be of type 'method'\n\nActual result:\n{:?}",
            result
        );
    }
}

/// Test: Query metadata reflects type filter
///
/// Given: A type filter is applied
/// When: treelint search process --type function is executed
/// Then: Query metadata shows type: "function"
#[test]
fn test_search_query_metadata_reflects_type_filter() {
    let temp_dir = setup_test_project_with_multiple_types();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search", "process", "--type", "function", "--format", "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let query_type = json
        .get("query")
        .and_then(|q| q.get("type"))
        .and_then(|t| t.as_str());

    assert_eq!(
        query_type,
        Some("function"),
        "query.type must reflect the --type filter value\n\nActual query:\n{:?}",
        json.get("query")
    );
}

/// Test: No type filter returns all symbol types
///
/// Given: An index contains multiple symbol types named "process"
/// When: treelint search process is executed (no --type filter)
/// Then: Returns all symbol types (function, method, variable)
#[test]
fn test_search_no_type_filter_returns_all_types() {
    let temp_dir = setup_test_project_with_multiple_types();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "process", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && results.unwrap().len() > 1,
        "Without type filter, should return multiple symbol types\n\nActual output:\n{}",
        stdout
    );

    // Collect unique types
    let mut types_found: Vec<String> = results
        .unwrap()
        .iter()
        .filter_map(|r| r.get("type").and_then(|t| t.as_str()).map(String::from))
        .collect();
    types_found.sort();
    types_found.dedup();

    // Should have at least 2 different types (function and method, or function and variable)
    assert!(
        types_found.len() >= 2,
        "Expected multiple symbol types without filter, found only: {:?}\n\nActual output:\n{}",
        types_found,
        stdout
    );
}

/// Test: Type filter with no matching results returns exit code 2
///
/// Given: An index does not contain variables named "validate"
/// When: treelint search validate --type variable is executed
/// Then: Exit code is 2 (no results)
#[test]
fn test_search_type_filter_no_match_returns_exit_2() {
    let temp_dir = setup_test_project_with_multiple_types();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // "validate" exists as a function but not as a variable
    cmd.current_dir(temp_dir.path())
        .args([
            "search", "validate", "--type", "variable", "--format", "json",
        ])
        .assert()
        .code(2);
}

/// Test: Type filter combined with exact match uses AND logic
///
/// Given: An index contains function "process" and method "process"
/// When: treelint search process --type method is executed
/// Then: Only returns the method (exact name AND type filter)
#[test]
fn test_search_type_filter_combined_with_name_uses_and_logic() {
    let temp_dir = setup_test_project_with_multiple_types();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "process", "--type", "method", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Should find method named 'process'\n\nActual output:\n{}",
        stdout
    );

    // All results must match BOTH name AND type (AND logic)
    for result in results.unwrap() {
        let name = result.get("name").and_then(|n| n.as_str());
        let symbol_type = result.get("type").and_then(|t| t.as_str());

        assert_eq!(name, Some("process"), "Name must match exactly");
        assert_eq!(symbol_type, Some("method"), "Type must match filter");
    }
}
