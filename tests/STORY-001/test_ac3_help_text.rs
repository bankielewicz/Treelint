//! AC#3: Help Text Displays Complete CLI Documentation
//!
//! Tests that:
//! - treelint --help displays version from Cargo.toml
//! - treelint search --help shows all options
//!
//! TDD Phase: RED - These tests should FAIL until implementation is complete.

use assert_cmd::Command;
use predicates::prelude::*;

/// Test: treelint --help exits successfully
#[test]
fn test_help_exits_successfully() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.arg("--help").assert().success();
}

/// Test: treelint --help displays program name
#[test]
fn test_help_displays_program_name() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("treelint"));
}

/// Test: treelint --help displays version (SVC-001)
#[test]
fn test_help_displays_version() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

/// Test: treelint --version outputs version from Cargo.toml
#[test]
fn test_version_outputs_cargo_version() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

/// Test: treelint --help describes the search command
#[test]
fn test_help_describes_search_command() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("search"));
}

/// Test: treelint search --help exits successfully
#[test]
fn test_search_help_exits_successfully() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "--help"]).assert().success();
}

/// Test: treelint search --help describes symbol argument
#[test]
fn test_search_help_describes_symbol_argument() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("symbol").or(predicate::str::contains("SYMBOL")));
}

/// Test: treelint search --help documents --type option
#[test]
fn test_search_help_documents_type_option() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--type"));
}

/// Test: treelint search --help documents -i/--ignore-case option
#[test]
fn test_search_help_documents_ignore_case_option() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-i").or(predicate::str::contains("--ignore-case")));
}

/// Test: treelint search --help documents -r/--regex option
#[test]
fn test_search_help_documents_regex_option() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-r").or(predicate::str::contains("--regex")));
}

/// Test: treelint search --help documents --format option
#[test]
fn test_search_help_documents_format_option() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--format"));
}

/// Test: treelint search --help documents --context option
#[test]
fn test_search_help_documents_context_option() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--context"));
}

/// Test: treelint search --help documents --signatures option
#[test]
fn test_search_help_documents_signatures_option() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--signatures"));
}

/// Test: treelint search --help lists valid --type values
#[test]
fn test_search_help_lists_valid_type_values() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd.args(["search", "--help"]).assert().success();

    // Help should list at least some of the valid type values
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    assert!(
        stdout.contains("function") || stdout.contains("class") || stdout.contains("method"),
        "Help should list valid --type values (function, class, method, etc.)\n\nActual output:\n{}",
        stdout
    );
}

/// Test: treelint search --help lists valid --format values
#[test]
fn test_search_help_lists_valid_format_values() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd.args(["search", "--help"]).assert().success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    assert!(
        stdout.contains("json") && stdout.contains("text"),
        "Help should list valid --format values (json, text)\n\nActual output:\n{}",
        stdout
    );
}

/// Test: treelint help command works (alternative to --help)
#[test]
fn test_help_command_works() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.arg("help")
        .assert()
        .success()
        .stdout(predicate::str::contains("treelint"));
}

/// Test: treelint help search works
#[test]
fn test_help_search_command_works() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["help", "search"])
        .assert()
        .success()
        .stdout(predicate::str::contains("search"));
}
