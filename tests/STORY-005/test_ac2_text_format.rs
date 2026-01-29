//! AC#2: Text Output Format for Human Readability
//!
//! Tests that:
//! - Text output has a header line with name/type/file/lines
//! - Tree structure with signature on its own line
//! - Body indented 4 spaces
//! - Summary line with count/elapsed_ms/files stats
//! - Empty results produce appropriate text message
//!
//! TDD Phase: RED - These tests should FAIL until TextFormatter is implemented.
//!
//! Technical Specification Requirements:
//! - src/output/text.rs: TextFormatter with tree-style layout
//! - Summary line format: "Found N results in Xms (Y files searched)"
//! - Body lines indented with 4 spaces

use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test project with Python files for text output testing
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

/// Helper to create a project with multiple matching symbols for summary testing
fn setup_test_project_with_multiple_results() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let file_a = src_dir.join("module_a.py");
    fs::write(
        &file_a,
        r#"
def helper(x):
    """Helper in module a."""
    return x + 1
"#,
    )
    .expect("Failed to write module_a.py");

    let file_b = src_dir.join("module_b.py");
    fs::write(
        &file_b,
        r#"
def helper(x):
    """Helper in module b."""
    return x * 2
"#,
    )
    .expect("Failed to write module_b.py");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// Header Line Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Text output contains a header line with symbol type, name, file, and lines
///
/// Given: A project with function "add" in calculator.py
/// When: treelint search add --format text is executed
/// Then: Output contains a header line with type/name/file/line information
#[test]
fn test_text_output_has_header_line_with_type_name_file_lines() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Header should contain the symbol type (function/Function)
    let has_type = stdout.to_lowercase().contains("function");
    assert!(
        has_type,
        "Text output header must indicate symbol type.\n\nActual output:\n{}",
        stdout
    );

    // Header should contain the symbol name
    assert!(
        stdout.contains("add"),
        "Text output header must include symbol name 'add'.\n\nActual output:\n{}",
        stdout
    );

    // Header should contain file path reference
    assert!(
        stdout.contains("calculator.py"),
        "Text output header must include file path.\n\nActual output:\n{}",
        stdout
    );

    // Header should contain line numbers
    let has_line_numbers = stdout.lines().any(|line| {
        // Look for line number patterns like "2-6" or ":2:" or "lines 2-6"
        line.chars().any(|c| c.is_ascii_digit()) && (line.contains('-') || line.contains(':'))
    });
    assert!(
        has_line_numbers,
        "Text output header must include line number information.\n\nActual output:\n{}",
        stdout
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Signature Display Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Text output includes function signature in tree structure
///
/// Given: A project with function "add(a: int, b: int) -> int"
/// When: treelint search add --format text is executed
/// Then: Output includes the function signature
#[test]
fn test_text_output_includes_signature() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Signature should contain the function definition
    assert!(
        stdout.contains("def add") || stdout.contains("add("),
        "Text output must include function signature.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Text output signature is on its own line (tree structure)
///
/// Given: A project with function "add"
/// When: treelint search add --format text is executed
/// Then: Signature appears on a separate line from the header
#[test]
fn test_text_output_signature_on_own_line() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    // There must be at least 2 lines (header + signature)
    assert!(
        lines.len() >= 2,
        "Text output must have at least header and signature lines.\n\nActual output ({} lines):\n{}",
        lines.len(),
        stdout
    );

    // Find the line with "add" in the header and check the next line has signature
    let header_idx = lines.iter().position(|line| {
        line.contains("add") && (line.contains("function") || line.contains("Function"))
    });

    if let Some(idx) = header_idx {
        assert!(
            idx + 1 < lines.len(),
            "Signature line must follow the header line"
        );
        // The signature line should be indented or prefixed with tree character
        let sig_line = lines[idx + 1];
        assert!(
            sig_line.starts_with(' ')
                || sig_line.starts_with('\t')
                || sig_line.starts_with("  ")
                || sig_line.contains("def add")
                || sig_line.contains("add("),
            "Signature should appear as a tree child (indented).\n\nHeader: {}\nNext line: {}",
            lines[idx],
            sig_line
        );
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Body Indentation Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Text output body lines are indented with 4 spaces
///
/// Given: A project with function "add" that has a body
/// When: treelint search add --format text is executed
/// Then: Body lines are indented with 4 spaces
#[test]
fn test_text_output_body_indented_4_spaces() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Body lines should contain the function body content
    // The function body is: return a + b
    // It should be indented with 4 spaces in the text output
    let has_indented_body = stdout.lines().any(|line| {
        line.starts_with("    ")
            && (line.trim_start().starts_with("return")
                || line.trim_start().starts_with("\"\"\"")
                || line.trim_start().contains("a + b"))
    });

    assert!(
        has_indented_body,
        "Body lines must be indented with 4 spaces.\n\nActual output:\n{}",
        stdout
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Summary Line Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Text output ends with a summary line
///
/// Given: A project with matching symbols
/// When: treelint search add --format text is executed
/// Then: Output ends with summary: "Found N results in Xms (Y files searched)"
#[test]
fn test_text_output_has_summary_line() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Summary line should contain "Found" and result count
    let has_summary = stdout.lines().any(|line| {
        let lower = line.to_lowercase();
        lower.contains("found") && lower.contains("result")
    });

    assert!(
        has_summary,
        "Text output must include a summary line with result count.\n\nExpected format: 'Found N results in Xms (Y files searched)'\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Summary line includes elapsed time in ms
///
/// Given: A project with matching symbols
/// When: treelint search add --format text is executed
/// Then: Summary line includes elapsed time (e.g., "in 42ms")
#[test]
fn test_text_summary_includes_elapsed_ms() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    let has_elapsed = stdout
        .lines()
        .any(|line| line.to_lowercase().contains("ms"));

    assert!(
        has_elapsed,
        "Summary line must include elapsed time in ms.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Summary line includes files searched count
///
/// Given: A project with source files
/// When: treelint search add --format text is executed
/// Then: Summary line includes file count (e.g., "(3 files searched)")
#[test]
fn test_text_summary_includes_files_searched() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    let has_files_count = stdout.lines().any(|line| {
        let lower = line.to_lowercase();
        lower.contains("file") && line.chars().any(|c| c.is_ascii_digit())
    });

    assert!(
        has_files_count,
        "Summary line must include files searched count.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Summary line format matches expected pattern
///
/// Given: A project with matching symbols
/// When: treelint search add --format text is executed
/// Then: Summary line matches "Found N results in Xms (Y files searched)"
#[test]
fn test_text_summary_format_matches_expected_pattern() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "add", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Look for the pattern: "Found N result(s) in Xms (Y file(s) searched)"
    let summary_pattern =
        regex::Regex::new(r"(?i)found\s+\d+\s+results?\s+in\s+\d+ms\s+\(\d+\s+files?\s+searched\)")
            .unwrap();

    let has_matching_summary = stdout.lines().any(|line| summary_pattern.is_match(line));

    assert!(
        has_matching_summary,
        "Summary line must match format: 'Found N results in Xms (Y files searched)'.\n\nActual output:\n{}",
        stdout
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Multiple Results Display Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Multiple results each get their own header/signature/body block
///
/// Given: A project with "helper" function in two different files
/// When: treelint search helper --format text is executed
/// Then: Output shows separate blocks for each result
#[test]
fn test_text_output_multiple_results_separate_blocks() {
    let temp_dir = setup_test_project_with_multiple_results();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "helper", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Should reference both files
    assert!(
        stdout.contains("module_a") && stdout.contains("module_b"),
        "Text output should show results from both files.\n\nActual output:\n{}",
        stdout
    );

    // Count header lines (each result gets a header with type/name)
    let header_count = stdout
        .lines()
        .filter(|line| {
            let lower = line.to_lowercase();
            lower.contains("function") && lower.contains("helper")
        })
        .count();

    assert!(
        header_count >= 2,
        "Expected at least 2 header lines for 2 results, got {}.\n\nActual output:\n{}",
        header_count,
        stdout
    );
}

/// Test: Summary line shows correct count for multiple results
///
/// Given: A project with "helper" in two files
/// When: treelint search helper --format text is executed
/// Then: Summary says "Found 2 results..."
#[test]
fn test_text_summary_shows_correct_count_multiple_results() {
    let temp_dir = setup_test_project_with_multiple_results();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "helper", "--format", "text"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Summary should say "Found 2 results" (or more)
    let summary_line = stdout
        .lines()
        .find(|line| line.to_lowercase().contains("found"));

    assert!(
        summary_line.is_some(),
        "Must have a summary line.\n\nActual output:\n{}",
        stdout
    );

    let summary = summary_line.unwrap();
    assert!(
        summary.contains("2"),
        "Summary should show 2 results for 'helper'.\n\nActual summary: {}",
        summary
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Empty Results Text Output Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: Empty results in text format show appropriate message
///
/// Given: A project with no matching symbols
/// When: treelint search nonExistentXYZ --format text is executed
/// Then: Output shows "No results found" or similar message
#[test]
fn test_text_empty_results_show_message() {
    let temp_dir = setup_test_project();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "nonExistentXYZ", "--format", "text"])
        .assert();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    let has_no_results_message = stdout.to_lowercase().contains("no results")
        || stdout.to_lowercase().contains("not found")
        || stdout.to_lowercase().contains("found 0");

    assert!(
        has_no_results_message,
        "Empty results in text mode should show a 'no results' message.\n\nActual output:\n{}",
        stdout
    );
}
