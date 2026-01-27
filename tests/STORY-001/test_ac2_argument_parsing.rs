//! AC#2: Search Command Parses All Required Arguments
//!
//! Tests that:
//! - treelint search validateUser --type function -i -r --format json --context 10 --signatures parses correctly
//! - SearchArgs struct parses all options
//! - SymbolType enum has all variants
//! - OutputFormat enum has json/text variants
//!
//! TDD Phase: RED - These tests should FAIL until implementation is complete.

use assert_cmd::Command;
use predicates::prelude::*;

/// Test: Search command accepts symbol argument
#[test]
fn test_search_accepts_symbol_argument() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "validateUser"]).assert().success();
}

/// Test: Search command accepts --type function flag
#[test]
fn test_search_accepts_type_function_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "validateUser", "--type", "function"])
        .assert()
        .success();
}

/// Test: Search command accepts --type class flag
#[test]
fn test_search_accepts_type_class_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "AuthService", "--type", "class"])
        .assert()
        .success();
}

/// Test: Search command accepts --type method flag
#[test]
fn test_search_accepts_type_method_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "login", "--type", "method"])
        .assert()
        .success();
}

/// Test: Search command accepts --type variable flag
#[test]
fn test_search_accepts_type_variable_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "config", "--type", "variable"])
        .assert()
        .success();
}

/// Test: Search command accepts --type constant flag
#[test]
fn test_search_accepts_type_constant_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "MAX_RETRIES", "--type", "constant"])
        .assert()
        .success();
}

/// Test: Search command accepts --type import flag
#[test]
fn test_search_accepts_type_import_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "os", "--type", "import"])
        .assert()
        .success();
}

/// Test: Search command accepts --type export flag
#[test]
fn test_search_accepts_type_export_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "default", "--type", "export"])
        .assert()
        .success();
}

/// Test: Search command accepts -i (ignore-case) short flag
#[test]
fn test_search_accepts_ignore_case_short_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "validateUser", "-i"])
        .assert()
        .success();
}

/// Test: Search command accepts --ignore-case long flag
#[test]
fn test_search_accepts_ignore_case_long_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "validateUser", "--ignore-case"])
        .assert()
        .success();
}

/// Test: Search command accepts -r (regex) short flag
#[test]
fn test_search_accepts_regex_short_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "validate.*", "-r"]).assert().success();
}

/// Test: Search command accepts --regex long flag
#[test]
fn test_search_accepts_regex_long_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "validate.*", "--regex"])
        .assert()
        .success();
}

/// Test: Search command accepts --format json flag
#[test]
fn test_search_accepts_format_json_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();
}

/// Test: Search command accepts --format text flag
#[test]
fn test_search_accepts_format_text_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "validateUser", "--format", "text"])
        .assert()
        .success();
}

/// Test: Search command accepts --context flag with number
#[test]
fn test_search_accepts_context_flag_with_number() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "validateUser", "--context", "10"])
        .assert()
        .success();
}

/// Test: Search command accepts --signatures flag
#[test]
fn test_search_accepts_signatures_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "validateUser", "--signatures"])
        .assert()
        .success();
}

/// Test: Search command parses all flags together (AC2 full scenario)
#[test]
fn test_search_parses_all_flags_combined() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args([
        "search",
        "validateUser",
        "--type",
        "function",
        "-i",
        "-r",
        "--format",
        "json",
        "--context",
        "10",
        "--signatures",
    ])
    .assert()
    .success();
}

/// Test: Invalid --type value is rejected with error
#[test]
fn test_search_rejects_invalid_type_value() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "foo", "--type", "invalid_type"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid").or(predicate::str::contains("Invalid")));
}

/// Test: Invalid --format value is rejected with error
#[test]
fn test_search_rejects_invalid_format_value() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "foo", "--format", "xml"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid").or(predicate::str::contains("Invalid")));
}

/// Test: Empty symbol argument is rejected (BR-001)
#[test]
fn test_search_rejects_empty_symbol() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Empty string as symbol should fail
    cmd.args(["search", ""]).assert().failure();
}

/// Test: Missing symbol argument produces error
#[test]
fn test_search_requires_symbol_argument() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // search without symbol should fail
    cmd.args(["search"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

/// Test: --context accepts 0
#[test]
fn test_search_context_accepts_zero() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "foo", "--context", "0"])
        .assert()
        .success();
}

/// Test: --context rejects negative numbers
#[test]
fn test_search_context_rejects_negative() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "foo", "--context", "-1"])
        .assert()
        .failure();
}

/// Test: --context rejects non-numeric values
#[test]
fn test_search_context_rejects_non_numeric() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "foo", "--context", "abc"])
        .assert()
        .failure();
}
