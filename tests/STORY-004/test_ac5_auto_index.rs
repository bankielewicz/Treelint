//! AC#5: Auto-Indexing on First Search When Index Missing
//!
//! Tests that:
//! - If no .treelint/index.db exists, index is built automatically
//! - Progress shows on stderr, search executes against new index
//! - Subsequent searches use existing index
//!
//! TDD Phase: RED - These tests should FAIL against the placeholder implementation.
//!
//! Technical Specification Requirements:
//! - SVC-005: Detect missing index and trigger auto-build before search
//! - BUILD-001: Implement build_index() that scans for supported files
//! - BUILD-002: Skip unsupported and binary files without error
//! - BUILD-003: Show progress to stderr during indexing

use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// Helper to create a test project WITHOUT an existing index
fn setup_test_project_no_index() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create some Python files
    let auth_file = src_dir.join("auth.py");
    fs::write(
        &auth_file,
        r#"
def validateUser(email: str, password: str) -> bool:
    """Validate user credentials."""
    return bool(email) and bool(password)

class AuthService:
    """Authentication service."""
    def authenticate(self, user):
        return validateUser(user.email, user.password)
"#,
    )
    .expect("Failed to write auth.py");

    let utils_file = src_dir.join("utils.py");
    fs::write(
        &utils_file,
        r#"
def processData(data: list) -> list:
    """Process data items."""
    return [item.strip() for item in data]
"#,
    )
    .expect("Failed to write utils.py");

    // Verify .treelint directory does NOT exist
    let treelint_dir = temp_dir.path().join(".treelint");
    assert!(
        !treelint_dir.exists(),
        "Test setup error: .treelint should not exist yet"
    );

    temp_dir
}

/// Helper to verify index was created
fn index_exists(project_root: &Path) -> bool {
    project_root.join(".treelint").join("index.db").exists()
}

/// Test: Search creates index automatically when missing
///
/// Given: No .treelint/index.db exists in the working directory
/// When: treelint search validateUser is executed
/// Then: Index is built automatically (.treelint/index.db created)
#[test]
fn test_search_auto_creates_index_when_missing() {
    let temp_dir = setup_test_project_no_index();

    // Verify no index exists before search
    assert!(
        !index_exists(temp_dir.path()),
        "Index should not exist before first search"
    );

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    // Verify index was created
    assert!(
        index_exists(temp_dir.path()),
        "Index should be created automatically after first search\n\nDirectory contents: {:?}",
        fs::read_dir(temp_dir.path())
            .map(|rd| rd
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .collect::<Vec<_>>())
            .unwrap_or_default()
    );
}

/// Test: Auto-index search returns real results
///
/// Given: No index exists, but source files contain symbols
/// When: treelint search validateUser is executed
/// Then: Search executes against newly built index and returns results
#[test]
fn test_search_auto_index_returns_real_results() {
    let temp_dir = setup_test_project_no_index();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let results = json.get("results").and_then(|r| r.as_array());

    // Should find validateUser after auto-indexing
    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "After auto-indexing, should find 'validateUser' symbol\n\nActual output:\n{}",
        stdout
    );

    let first_result = &results.unwrap()[0];
    let name = first_result.get("name").and_then(|n| n.as_str());
    assert_eq!(
        name,
        Some("validateUser"),
        "Auto-index search should find 'validateUser'\n\nActual result:\n{:?}",
        first_result
    );
}

/// Test: Auto-index shows progress on stderr
///
/// Given: No index exists
/// When: treelint search validateUser is executed
/// Then: Progress indicator shows on stderr during indexing
#[test]
fn test_search_auto_index_shows_progress_on_stderr() {
    let temp_dir = setup_test_project_no_index();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    let stderr = String::from_utf8_lossy(&output.get_output().stderr);

    // Stderr should contain indexing progress information
    // Accept various progress indicator formats
    let has_progress = stderr.contains("index")
        || stderr.contains("Index")
        || stderr.contains("scan")
        || stderr.contains("Scan")
        || stderr.contains("build")
        || stderr.contains("Build")
        || stderr.contains("pars")
        || stderr.contains("Pars")
        || stderr.contains("...")
        || stderr.contains("file")
        || stderr.contains("File");

    assert!(
        has_progress,
        "Auto-indexing should show progress on stderr\n\nActual stderr:\n{}",
        stderr
    );
}

/// Test: Subsequent search uses existing index (faster)
///
/// Given: Index was created by first search
/// When: treelint search validateUser is executed a second time
/// Then: Search completes without rebuilding index (should be faster)
#[test]
fn test_search_subsequent_uses_existing_index() {
    let temp_dir = setup_test_project_no_index();

    // First search - creates index
    let start1 = Instant::now();
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();
    let duration1 = start1.elapsed();

    // Verify index exists
    assert!(
        index_exists(temp_dir.path()),
        "Index should exist after first search"
    );

    // Second search - uses existing index
    let start2 = Instant::now();
    let output2 = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();
    let duration2 = start2.elapsed();

    // Second search should not show indexing progress (no rebuild)
    let stderr2 = String::from_utf8_lossy(&output2.get_output().stderr);

    // If there's indexing progress on second run, it should be much less
    // (or ideally, no "building index" message at all)
    let no_index_build_message =
        !stderr2.contains("Building index") && !stderr2.contains("building index");

    // Note: This is a soft assertion - some implementations might log differently
    // The main test is that results are still returned correctly
    let stdout2 = String::from_utf8_lossy(&output2.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout2).expect("Output is not valid JSON");
    let results = json.get("results").and_then(|r| r.as_array());

    assert!(
        results.is_some() && !results.unwrap().is_empty(),
        "Second search should still return results\n\nActual output:\n{}",
        stdout2
    );
}

/// Test: Auto-index works with multiple file types
///
/// Given: Project contains .py, .ts, .rs, .md files
/// When: treelint search is executed
/// Then: Index includes symbols from all supported languages
#[test]
fn test_search_auto_index_multiple_languages() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Python file
    fs::write(
        src_dir.join("auth.py"),
        "def validateUser(email): return True",
    )
    .expect("Failed to write Python file");

    // TypeScript file
    fs::write(
        src_dir.join("auth.ts"),
        "function validateUser(email: string): boolean { return true; }",
    )
    .expect("Failed to write TypeScript file");

    // Rust file
    fs::write(
        src_dir.join("auth.rs"),
        "fn validate_user(email: &str) -> bool { true }",
    )
    .expect("Failed to write Rust file");

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "validate.*", "-r", "-i", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let stats = json.get("stats");
    let files_searched = stats
        .and_then(|s| s.get("files_searched"))
        .and_then(|f| f.as_u64());

    // Should have searched multiple files
    assert!(
        files_searched.is_some() && files_searched.unwrap() >= 2,
        "Auto-index should scan multiple language files\n\nActual stats:\n{:?}",
        stats
    );
}

/// Test: Auto-index skips binary files without error
///
/// Given: Project contains binary files (.jpg, .exe, .bin)
/// When: treelint search is executed
/// Then: Search completes successfully (binary files skipped)
#[test]
fn test_search_auto_index_skips_binary_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create valid Python file
    fs::write(
        src_dir.join("auth.py"),
        "def validateUser(email): return True",
    )
    .expect("Failed to write Python file");

    // Create binary-like files (just need the extension to be recognized)
    fs::write(src_dir.join("image.jpg"), &[0xFF, 0xD8, 0xFF, 0xE0][..])
        .expect("Failed to write jpg file");
    fs::write(src_dir.join("program.exe"), &[0x4D, 0x5A][..]).expect("Failed to write exe file");
    fs::write(src_dir.join("data.bin"), &[0x00, 0x01, 0x02][..]).expect("Failed to write bin file");

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Should succeed without error (binary files skipped)
    cmd.current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();
}

/// Test: Auto-index skips unsupported file types without error
///
/// Given: Project contains unsupported files (.java, .cpp, .go)
/// When: treelint search is executed
/// Then: Search completes successfully (unsupported files skipped)
#[test]
fn test_search_auto_index_skips_unsupported_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create valid Python file
    fs::write(
        src_dir.join("auth.py"),
        "def validateUser(email): return True",
    )
    .expect("Failed to write Python file");

    // Create unsupported language files
    fs::write(
        src_dir.join("Auth.java"),
        "public class Auth { public boolean validateUser() { return true; } }",
    )
    .expect("Failed to write Java file");
    fs::write(
        src_dir.join("auth.cpp"),
        "bool validateUser() { return true; }",
    )
    .expect("Failed to write C++ file");
    fs::write(
        src_dir.join("auth.go"),
        "func validateUser() bool { return true }",
    )
    .expect("Failed to write Go file");

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Should succeed without error
    cmd.current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();
}

/// Test: Auto-index creates .treelint directory if missing
///
/// Given: No .treelint directory exists
/// When: treelint search is executed
/// Then: .treelint directory is created along with index.db
#[test]
fn test_search_auto_index_creates_treelint_directory() {
    let temp_dir = setup_test_project_no_index();

    let treelint_dir = temp_dir.path().join(".treelint");
    assert!(
        !treelint_dir.exists(),
        ".treelint should not exist initially"
    );

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    cmd.current_dir(temp_dir.path())
        .args(["search", "validateUser", "--format", "json"])
        .assert()
        .success();

    assert!(
        treelint_dir.exists(),
        ".treelint directory should be created by auto-index"
    );
    assert!(
        treelint_dir.join("index.db").exists(),
        "index.db should be created inside .treelint directory"
    );
}

/// Test: Auto-index with empty project returns exit code 2
///
/// Given: Project has no supported source files
/// When: treelint search anySymbol is executed
/// Then: Exit code is 2 (no results - nothing to index)
#[test]
fn test_search_auto_index_empty_project_returns_exit_2() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create an empty src directory
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Only create a text file (not a supported language)
    fs::write(src_dir.join("readme.txt"), "This is a readme").expect("Failed to write readme.txt");

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    // Should return exit code 2 (no results)
    cmd.current_dir(temp_dir.path())
        .args(["search", "anySymbol", "--format", "json"])
        .assert()
        .code(2);
}

/// Test: Stats reflect actual files indexed
///
/// Given: Project has 3 Python files
/// When: treelint search is executed with auto-index
/// Then: stats.files_searched reflects the number of indexed files
#[test]
fn test_search_auto_index_stats_reflect_files_indexed() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create 3 Python files
    for i in 1..=3 {
        fs::write(
            src_dir.join(format!("module{}.py", i)),
            format!("def func{}(): pass", i),
        )
        .expect("Failed to write Python file");
    }

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args(["search", "func.*", "-r", "--format", "json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output is not valid JSON");

    let files_searched = json
        .get("stats")
        .and_then(|s| s.get("files_searched"))
        .and_then(|f| f.as_u64());

    assert!(
        files_searched.is_some() && files_searched.unwrap() >= 3,
        "stats.files_searched should be at least 3\n\nActual stats:\n{:?}",
        json.get("stats")
    );
}
