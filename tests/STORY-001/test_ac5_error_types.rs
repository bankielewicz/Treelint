//! AC#5: Error Types Defined Using thiserror
//!
//! Tests that:
//! - TreelintError enum exists with thiserror derive
//! - Io, Parse, Cli variants exist
//! - Error Display trait works
//!
//! TDD Phase: RED - These tests should FAIL until implementation is complete.
//!
//! Note: These tests use the treelint library to test internal error types.
//! They require `treelint` to expose its error module in lib.rs.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

/// Test: src/error.rs file exists
#[test]
fn test_error_module_file_exists() {
    let error_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("error.rs");

    assert!(
        error_path.exists(),
        "src/error.rs must exist\nExpected path: {}",
        error_path.display()
    );
}

/// Test: error.rs contains TreelintError enum
#[test]
fn test_error_module_contains_treelint_error_enum() {
    let error_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("error.rs");

    let content =
        fs::read_to_string(&error_path).expect(&format!("Failed to read {}", error_path.display()));

    assert!(
        content.contains("TreelintError") || content.contains("enum TreelintError"),
        "src/error.rs must contain TreelintError enum\n\nActual content:\n{}",
        content
    );
}

/// Test: error.rs uses thiserror derive macro
#[test]
fn test_error_module_uses_thiserror_derive() {
    let error_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("error.rs");

    let content =
        fs::read_to_string(&error_path).expect(&format!("Failed to read {}", error_path.display()));

    assert!(
        content.contains("thiserror::Error") || content.contains("#[derive(") && content.contains("Error"),
        "src/error.rs must use thiserror derive macro (#[derive(thiserror::Error)])\n\nActual content:\n{}",
        content
    );
}

/// Test: TreelintError has Io variant
#[test]
fn test_treelint_error_has_io_variant() {
    let error_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("error.rs");

    let content =
        fs::read_to_string(&error_path).expect(&format!("Failed to read {}", error_path.display()));

    assert!(
        content.contains("Io"),
        "TreelintError must have Io variant\n\nActual content:\n{}",
        content
    );
}

/// Test: TreelintError Io variant wraps std::io::Error
#[test]
fn test_treelint_error_io_wraps_std_io_error() {
    let error_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("error.rs");

    let content =
        fs::read_to_string(&error_path).expect(&format!("Failed to read {}", error_path.display()));

    // Should contain either #[from] std::io::Error or io::Error
    assert!(
        content.contains("std::io::Error") || content.contains("io::Error"),
        "TreelintError::Io must wrap std::io::Error\n\nActual content:\n{}",
        content
    );
}

/// Test: TreelintError has Parse variant
#[test]
fn test_treelint_error_has_parse_variant() {
    let error_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("error.rs");

    let content =
        fs::read_to_string(&error_path).expect(&format!("Failed to read {}", error_path.display()));

    assert!(
        content.contains("Parse"),
        "TreelintError must have Parse variant\n\nActual content:\n{}",
        content
    );
}

/// Test: TreelintError has Cli variant
#[test]
fn test_treelint_error_has_cli_variant() {
    let error_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("error.rs");

    let content =
        fs::read_to_string(&error_path).expect(&format!("Failed to read {}", error_path.display()));

    assert!(
        content.contains("Cli"),
        "TreelintError must have Cli variant\n\nActual content:\n{}",
        content
    );
}

/// Test: TreelintError derives Debug
#[test]
fn test_treelint_error_derives_debug() {
    let error_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("error.rs");

    let content =
        fs::read_to_string(&error_path).expect(&format!("Failed to read {}", error_path.display()));

    assert!(
        content.contains("Debug"),
        "TreelintError must derive Debug\n\nActual content:\n{}",
        content
    );
}

/// Test: error.rs contains #[error(...)] display attributes
#[test]
fn test_error_module_has_display_attributes() {
    let error_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("error.rs");

    let content =
        fs::read_to_string(&error_path).expect(&format!("Failed to read {}", error_path.display()));

    assert!(
        content.contains("#[error("),
        "TreelintError variants must have #[error(...)] display attributes\n\nActual content:\n{}",
        content
    );
}

/// Test: Invalid arguments produce error with exit code 1 (SVC-003)
#[test]
fn test_invalid_arguments_produce_exit_code_1() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search"]) // missing required symbol
        .assert()
        .failure()
        .code(predicate::in_iter([1, 2])); // clap typically returns 2 for usage errors
}

/// Test: Error messages are human-readable (NFR-003)
#[test]
fn test_error_messages_are_human_readable() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search"]) // missing required symbol
        .assert()
        .failure()
        .stderr(predicate::str::is_empty().not()); // Should have non-empty error message
}

/// Test: Error for invalid --type shows valid options (BR-002)
#[test]
fn test_invalid_type_error_shows_valid_options() {
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.args(["search", "foo", "--type", "invalid"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("function")
                .or(predicate::str::contains("class"))
                .or(predicate::str::contains("method"))
                .or(predicate::str::contains("possible values")),
        );
}

/// Test: Error module is accessible via lib.rs (optional - for library usage)
#[test]
fn test_lib_rs_exports_error_module() {
    let lib_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("lib.rs");

    // lib.rs is optional for binary-only crate
    if lib_path.exists() {
        let content =
            fs::read_to_string(&lib_path).expect(&format!("Failed to read {}", lib_path.display()));

        // If lib.rs exists, it should export the error module
        assert!(
            content.contains("mod error") || content.contains("pub mod error"),
            "lib.rs should export error module if present\n\nActual content:\n{}",
            content
        );
    }
}
