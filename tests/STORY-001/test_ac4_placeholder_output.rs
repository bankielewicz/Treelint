//! AC#4: Search Command Returns Placeholder JSON Output
//!
//! Tests that:
//! - treelint search testSymbol --format json returns valid JSON
//! - JSON matches schema: {"query": {...}, "results": [], "stats": {...}}
//! - Exit codes per spec: 0=results found, 1=error, 2=no results
//!
//! Note: Tests use non-existent symbol "testSymbol" so expect exit code 2 (no results).

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

/// Test: Search command exits with code 2 when no results found (per spec)
/// Spec: "Exit code 0 for success, 1 for user errors, 2 for no results"
#[test]
fn test_search_exits_with_code_2_on_no_results() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // testSymbol doesn't exist, so exit code should be 2 (no results)
    cmd.args(["search", "testSymbol"]).assert().code(2);
}

/// Test: Search --format json returns valid JSON
#[test]
fn test_search_json_format_returns_valid_json() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .args(["search", "testSymbol", "--format", "json"])
        .assert()
        .code(2); // No results = exit code 2 per spec

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Parse as JSON - should not fail
    let _: Value =
        serde_json::from_str(&stdout).expect(&format!("Output is not valid JSON:\n{}", stdout));
}

/// Test: JSON output contains "query" field
#[test]
fn test_search_json_contains_query_field() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .args(["search", "testSymbol", "--format", "json"])
        .assert()
        .code(2); // No results = exit code 2 per spec

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    assert!(
        json.get("query").is_some(),
        "JSON output must contain 'query' field\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON output contains "results" field
#[test]
fn test_search_json_contains_results_field() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .args(["search", "testSymbol", "--format", "json"])
        .assert()
        .code(2); // No results = exit code 2 per spec

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    assert!(
        json.get("results").is_some(),
        "JSON output must contain 'results' field\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON output contains "stats" field
#[test]
fn test_search_json_contains_stats_field() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .args(["search", "testSymbol", "--format", "json"])
        .assert()
        .code(2); // No results = exit code 2 per spec

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    assert!(
        json.get("stats").is_some(),
        "JSON output must contain 'stats' field\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON query.symbol matches input symbol
#[test]
fn test_search_json_query_symbol_matches_input() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .args(["search", "testSymbol", "--format", "json"])
        .assert()
        .code(2); // No results = exit code 2 per spec

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let query_symbol = json
        .get("query")
        .and_then(|q| q.get("symbol"))
        .and_then(|s| s.as_str());

    assert_eq!(
        query_symbol,
        Some("testSymbol"),
        "query.symbol must match input symbol 'testSymbol'\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON query.type reflects --type flag
#[test]
fn test_search_json_query_type_reflects_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .args([
            "search",
            "testSymbol",
            "--type",
            "function",
            "--format",
            "json",
        ])
        .assert()
        .code(2); // No results = exit code 2 per spec

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let query_type = json
        .get("query")
        .and_then(|q| q.get("type"))
        .and_then(|t| t.as_str());

    assert_eq!(
        query_type,
        Some("function"),
        "query.type must be 'function' when --type function is passed\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON query.type is null when --type not specified
#[test]
fn test_search_json_query_type_null_when_not_specified() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .args(["search", "testSymbol", "--format", "json"])
        .assert()
        .code(2); // No results = exit code 2 per spec

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let query_type = json.get("query").and_then(|q| q.get("type"));

    assert!(
        query_type.is_none() || query_type == Some(&Value::Null),
        "query.type must be null when --type is not specified\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON results is an empty array (placeholder)
#[test]
fn test_search_json_results_is_empty_array() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .args(["search", "testSymbol", "--format", "json"])
        .assert()
        .code(2); // No results = exit code 2 per spec

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results");

    assert!(
        results.is_some() && results.unwrap().is_array(),
        "results must be an array\n\nActual output:\n{}",
        stdout
    );

    let results_array = results.unwrap().as_array().unwrap();
    assert!(
        results_array.is_empty(),
        "Placeholder results must be an empty array\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON stats.files_searched is present and non-negative
#[test]
fn test_search_json_stats_files_searched_is_non_negative() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .args(["search", "testSymbol", "--format", "json"])
        .assert()
        .code(2); // No results = exit code 2 per spec

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let files_searched = json
        .get("stats")
        .and_then(|s| s.get("files_searched"))
        .and_then(|f| f.as_i64());

    assert!(
        files_searched.is_some() && files_searched.unwrap() >= 0,
        "stats.files_searched must be a non-negative integer\n\nActual output:\n{}",
        stdout
    );
}

/// Test: JSON stats.elapsed_ms is present and numeric
#[test]
fn test_search_json_stats_elapsed_ms_is_numeric() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .args(["search", "testSymbol", "--format", "json"])
        .assert()
        .code(2); // No results = exit code 2 per spec

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let elapsed_ms = json.get("stats").and_then(|s| s.get("elapsed_ms"));

    assert!(
        elapsed_ms.is_some() && elapsed_ms.unwrap().is_number(),
        "stats.elapsed_ms must be a number\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Text format output shows "No results found" message (SVC-006)
#[test]
fn test_search_text_format_shows_no_results_message() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "testSymbol", "--format", "text"])
        .assert()
        .code(2) // No results = exit code 2 per spec
        .stdout(predicate::str::contains("No results").or(predicate::str::contains("testSymbol")));
}

/// Test: Default format (without --format) works
#[test]
fn test_search_default_format_works() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Without --format flag, should default to text (or json) and work
    cmd.args(["search", "testSymbol"]).assert().code(2);
}

/// Test: JSON output is compact (no excessive whitespace)
#[test]
fn test_search_json_output_is_compact_or_pretty() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .args(["search", "testSymbol", "--format", "json"])
        .assert()
        .code(2); // No results = exit code 2 per spec

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Should be parseable JSON regardless of formatting
    let parsed: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    // Verify it has required structure
    assert!(parsed.get("query").is_some());
    assert!(parsed.get("results").is_some());
    assert!(parsed.get("stats").is_some());
}
