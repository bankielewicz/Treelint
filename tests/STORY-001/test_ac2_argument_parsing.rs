//! AC#2: Search Command Parses All Required Arguments
//!
//! Tests that:
//! - treelint search validateUser --type function -i -r --format json --context 10 --signatures parses correctly
//! - SearchArgs struct parses all options
//! - SymbolType enum has all variants
//! - OutputFormat enum has json/text variants
//!
//! TDD Phase: RED - These tests should FAIL until implementation is complete.
//!
//! Note: These tests verify argument PARSING, not search results.
//! Exit code 0 = success with results
//! Exit code 2 = success but no results (valid argument parsing)
//! Both indicate valid argument parsing.

use assert_cmd::Command;
use predicates::prelude::*;

/// Helper to check that argument parsing succeeded.
/// Exit code 0 = success with results
/// Exit code 2 = success but no results found (Unix convention)
/// Either indicates valid argument parsing.
fn assert_arguments_accepted(cmd: &mut Command) {
    let assert = cmd.assert();
    let output = assert.get_output();
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 2,
        "Expected exit code 0 (success) or 2 (no results), got {}.\nstderr: {}",
        code,
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Test: Search command accepts symbol argument
#[test]
fn test_search_accepts_symbol_argument() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "validateUser"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --type function flag
#[test]
fn test_search_accepts_type_function_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "validateUser", "--type", "function"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --type class flag
#[test]
fn test_search_accepts_type_class_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "AuthService", "--type", "class"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --type method flag
#[test]
fn test_search_accepts_type_method_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "login", "--type", "method"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --type variable flag
#[test]
fn test_search_accepts_type_variable_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "config", "--type", "variable"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --type constant flag
#[test]
fn test_search_accepts_type_constant_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "MAX_RETRIES", "--type", "constant"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --type import flag
#[test]
fn test_search_accepts_type_import_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "os", "--type", "import"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --type export flag
#[test]
fn test_search_accepts_type_export_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "default", "--type", "export"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts -i (ignore-case) short flag
#[test]
fn test_search_accepts_ignore_case_short_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "validateUser", "-i"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --ignore-case long flag
#[test]
fn test_search_accepts_ignore_case_long_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "validateUser", "--ignore-case"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts -r (regex) short flag
#[test]
fn test_search_accepts_regex_short_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "validate.*", "-r"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --regex long flag
#[test]
fn test_search_accepts_regex_long_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "validate.*", "--regex"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --format json flag
#[test]
fn test_search_accepts_format_json_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "validateUser", "--format", "json"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --format text flag
#[test]
fn test_search_accepts_format_text_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "validateUser", "--format", "text"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --context flag with number
#[test]
fn test_search_accepts_context_flag_with_number() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "validateUser", "--context", "10"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command accepts --signatures flag
#[test]
fn test_search_accepts_signatures_flag() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["search", "validateUser", "--signatures"]);
    assert_arguments_accepted(&mut cmd);
}

/// Test: Search command parses all flags together (AC2 full scenario)
/// Note: --context and --signatures are mutually exclusive per STORY-006 BR-001
#[test]
fn test_search_parses_all_flags_combined() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    // Don't use both --context and --signatures (mutually exclusive per STORY-006)
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
    ]);
    assert_arguments_accepted(&mut cmd);
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

/// Test: --context rejects 0 (STORY-006 BR-002)
#[test]
fn test_search_context_rejects_zero() {
    // STORY-006 BR-002: --context N must be a positive integer (> 0)
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "foo", "--context", "0"])
        .assert()
        .failure();
}

/// Test: --context rejects negative numbers
#[test]
fn test_search_context_rejects_negative() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "foo", "--context", "-1"])
        .assert()
        .failure();
}

/// Test: --context rejects non-numeric values (except "full")
#[test]
fn test_search_context_rejects_non_numeric() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "foo", "--context", "abc"])
        .assert()
        .failure();
}
