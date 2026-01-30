//! AC#3: Index Force Rebuild Option Tests
//!
//! Given: Daemon is running with existing index
//! When: Client sends index request with force flag: {"method": "index", "params": {"force": true}}
//! Then:
//!   - Clears existing index entries
//!   - Re-parses all source files (ignores file hashes)
//!   - Response includes full count of re-indexed files
//!
//! Source files tested:
//!   - src/daemon/server.rs (handle_index with force parameter)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - INT-005: Support force flag to trigger full rebuild
//!
//! TDD Phase: RED
//!
//! These tests will FAIL because handle_index() currently:
//! - Ignores the "force" parameter entirely
//! - Returns hardcoded response instead of actually re-indexing

use assert_cmd::Command;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Helper: Create initial test project
fn setup_initial_project(temp_dir: &TempDir) {
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let py_content = r#"
def original_function():
    """Original function from initial setup"""
    return "original"

class OriginalClass:
    """Original class from initial setup"""
    pass
"#;
    fs::write(src_dir.join("module.py"), py_content).expect("Failed to write Python file");
}

/// Helper: Modify project files (for testing re-index)
fn modify_project_files(temp_dir: &TempDir) {
    let src_dir = temp_dir.path().join("src");

    // Add new file with new symbols
    let new_content = r#"
def new_function_after_change():
    """New function added after initial index"""
    return "new"

class NewClassAfterChange:
    """New class added after initial index"""
    pass
"#;
    fs::write(src_dir.join("new_module.py"), new_content).expect("Failed to write new file");

    // Modify existing file
    let modified_content = r#"
def modified_function():
    """Modified function (was original_function)"""
    return "modified"

def another_new_function():
    """Another new function"""
    return "another"

class ModifiedClass:
    """Modified class (was OriginalClass)"""
    pass
"#;
    fs::write(src_dir.join("module.py"), modified_content).expect("Failed to modify file");
}

/// Helper: Start daemon
fn start_daemon(temp_dir: &TempDir) -> String {
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    thread::sleep(Duration::from_millis(300));

    #[cfg(unix)]
    {
        temp_dir
            .path()
            .join(".treelint")
            .join("daemon.sock")
            .to_string_lossy()
            .to_string()
    }
    #[cfg(windows)]
    {
        format!(
            "\\\\.\\pipe\\treelint-daemon-{}",
            temp_dir.path().to_string_lossy().replace(['\\', '/', ':'], "-")
        )
    }
}

/// Helper: Stop daemon
fn stop_daemon(temp_dir: &TempDir) {
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Helper: Wait for socket
#[cfg(unix)]
fn wait_for_socket(socket_path: &str, timeout_ms: u64) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed().as_millis() < timeout_ms as u128 {
        if std::path::Path::new(socket_path).exists() {
            return true;
        }
        thread::sleep(Duration::from_millis(50));
    }
    false
}

#[cfg(windows)]
fn wait_for_socket(_socket_path: &str, timeout_ms: u64) -> bool {
    thread::sleep(Duration::from_millis(timeout_ms));
    true
}

/// Helper: Send request to daemon (Unix)
#[cfg(unix)]
fn send_daemon_request(socket_path: &str, request: &str) -> String {
    use std::os::unix::net::UnixStream;

    if !wait_for_socket(socket_path, 2000) {
        panic!("Daemon socket not available at {}", socket_path);
    }

    let mut stream = match UnixStream::connect(socket_path) {
        Ok(s) => s,
        Err(e) => panic!("Failed to connect to daemon: {}", e),
    };
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .expect("Failed to set timeout");

    writeln!(stream, "{}", request).expect("Failed to send request");
    stream.flush().expect("Failed to flush");

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader
        .read_line(&mut response)
        .expect("Failed to read response");

    response
}

/// Helper: Send request to daemon (Windows)
#[cfg(windows)]
fn send_daemon_request(socket_path: &str, request: &str) -> String {
    use interprocess::local_socket::{GenericFilePath, Stream, ToFsName};

    let pipe_name = socket_path
        .to_fs_name::<GenericFilePath>()
        .expect("Invalid pipe name");

    let mut stream = Stream::connect(pipe_name).expect("Failed to connect to daemon");

    writeln!(stream, "{}", request).expect("Failed to send request");
    stream.flush().expect("Failed to flush");

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader
        .read_line(&mut response)
        .expect("Failed to read response");

    response
}

/// Helper: Extract symbols array from response
fn extract_symbols(json: &serde_json::Value, response: &str) -> Vec<serde_json::Value> {
    if let Some(error) = json.get("error") {
        if !error.is_null() {
            return Vec::new();
        }
    }

    if let Some(arr) = json["result"].as_array() {
        return arr.clone();
    }

    if let Some(arr) = json["result"]["symbols"].as_array() {
        return arr.clone();
    }

    panic!("Could not extract symbols from response: {}", response);
}

/// Test: Force index clears and rebuilds entire index
/// Requirement: INT-005 - Support force flag to trigger full rebuild
///
/// This test WILL FAIL because handle_index ignores force parameter.
#[test]
fn test_force_index_clears_and_rebuilds() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    setup_initial_project(&temp_dir);
    let socket_path = start_daemon(&temp_dir);

    // First index (normal)
    let first_index = r#"{"id": "index-1", "method": "index", "params": {}}"#;
    let first_response = send_daemon_request(&socket_path, first_index);

    // Modify files
    modify_project_files(&temp_dir);

    // Act: Force re-index
    let force_index = r#"{"id": "index-2", "method": "index", "params": {"force": true}}"#;
    let force_response = send_daemon_request(&socket_path, force_index);

    // Search for new symbol that was added after modification
    let search_request =
        r#"{"id": "search-1", "method": "search", "params": {"symbol": "new_function_after_change"}}"#;
    let search_response = send_daemon_request(&socket_path, search_request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Force index should find new symbols
    let search_json: serde_json::Value = serde_json::from_str(&search_response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, search_response));

    let symbols = extract_symbols(&search_json, &search_response);

    // FAILS: Force index doesn't actually re-index
    assert!(
        !symbols.is_empty(),
        "Force re-index should find symbols from modified files.\n\
         Root cause: handle_index is stub\n\
         First index response: {}\n\
         Force index response: {}\n\
         Search response: {}",
        first_response,
        force_response,
        search_response
    );
}

/// Test: Force index re-parses all files (ignores hash cache)
/// Requirement: INT-005 - Support force flag to trigger full rebuild
///
/// This test WILL FAIL because handle_index is a stub.
#[test]
fn test_force_index_ignores_file_hash_cache() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    setup_initial_project(&temp_dir);
    let socket_path = start_daemon(&temp_dir);

    // First index
    let first_index = r#"{"id": "index-1", "method": "index", "params": {}}"#;
    let first_response = send_daemon_request(&socket_path, first_index);
    let first_json: serde_json::Value = serde_json::from_str(&first_response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, first_response));

    // Act: Force index (should re-parse all files regardless of cache)
    let force_index = r#"{"id": "index-3", "method": "index", "params": {"force": true}}"#;
    let force_response = send_daemon_request(&socket_path, force_index);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Force index should report files re-indexed
    let force_json: serde_json::Value = serde_json::from_str(&force_response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, force_response));

    // FAILS: Current implementation returns 0 files
    let force_files = force_json["result"]["files_indexed"].as_u64().unwrap_or(0);
    let first_files = first_json["result"]["files_indexed"].as_u64().unwrap_or(0);

    assert!(
        force_files > 0,
        "Force index should report files re-indexed.\n\
         Expected: > 0 files\n\
         Actual: {} files\n\
         Root cause: handle_index is stub\n\
         Response: {}",
        force_files,
        force_response
    );

    // Force index should process at least as many files as first index
    assert!(
        force_files >= first_files || first_files == 0,
        "Force index should re-parse all files.\n\
         First index: {} files\n\
         Force index: {} files",
        first_files,
        force_files
    );
}

/// Test: Force=false parameter uses incremental indexing
/// Requirement: INT-005 - Support force flag to trigger full rebuild
///
/// This test should PASS - verifies force=false is accepted.
#[test]
fn test_force_false_uses_incremental_index() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    setup_initial_project(&temp_dir);
    let socket_path = start_daemon(&temp_dir);

    // First index
    let first_index = r#"{"id": "index-1", "method": "index", "params": {}}"#;
    let _ = send_daemon_request(&socket_path, first_index);

    // Act: Second index with force=false (explicit)
    let second_index = r#"{"id": "index-2", "method": "index", "params": {"force": false}}"#;
    let response = send_daemon_request(&socket_path, second_index);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Should complete successfully
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    assert!(
        json.get("error").is_none() || json["error"].is_null(),
        "Index with force=false should succeed.\n\
         Response: {}",
        response
    );

    assert_eq!(
        json["result"]["status"].as_str(),
        Some("completed"),
        "Index should complete.\n\
         Response: {}",
        response
    );
}

/// Test: Force index response includes accurate file count
/// Requirement: INT-005 - Response includes full count of re-indexed files
///
/// This test WILL FAIL because handle_index returns hardcoded 0 count.
#[test]
fn test_force_index_response_has_accurate_file_count() {
    // Arrange: Create project with known file count
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create exactly 4 Python files
    for i in 0..4 {
        let content = format!("def func_{}(): pass", i);
        fs::write(src_dir.join(format!("mod_{}.py", i)), content).expect("Failed to write file");
    }

    let socket_path = start_daemon(&temp_dir);

    // Act: Force index
    let request = r#"{"id": "index-1", "method": "index", "params": {"force": true}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Should report exactly 4 files
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    // FAILS: Current implementation returns 0
    let files_indexed = json["result"]["files_indexed"]
        .as_u64()
        .expect("files_indexed should be number");

    assert_eq!(
        files_indexed, 4,
        "Force index should report exact file count.\n\
         Expected: 4 files\n\
         Actual: {} files\n\
         Root cause: handle_index is stub\n\
         Response: {}",
        files_indexed,
        response
    );
}

/// Test: Force index clears index before rebuilding
/// Requirement: INT-005 - Clears existing index entries
///
/// This test WILL FAIL because handle_index is a stub.
#[test]
fn test_force_index_clears_index_before_rebuild() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Initial file
    fs::write(src_dir.join("initial.py"), "def initial_only(): pass")
        .expect("Failed to write file");

    let socket_path = start_daemon(&temp_dir);

    // First index
    let first_index = r#"{"id": "index-1", "method": "index", "params": {}}"#;
    let _ = send_daemon_request(&socket_path, first_index);

    // Remove the initial file
    fs::remove_file(src_dir.join("initial.py")).expect("Failed to remove file");

    // Create different file
    fs::write(src_dir.join("different.py"), "def different_only(): pass")
        .expect("Failed to write file");

    // Act: Force index (should clear index of initial_only)
    let force_index = r#"{"id": "index-2", "method": "index", "params": {"force": true}}"#;
    let _ = send_daemon_request(&socket_path, force_index);

    // Search for symbol from removed file
    let search_initial =
        r#"{"id": "search-1", "method": "search", "params": {"symbol": "initial_only"}}"#;
    let initial_response = send_daemon_request(&socket_path, search_initial);

    // Search for symbol from new file
    let search_different =
        r#"{"id": "search-2", "method": "search", "params": {"symbol": "different_only"}}"#;
    let different_response = send_daemon_request(&socket_path, search_different);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: initial_only should NOT be found (file removed, index cleared)
    let initial_json: serde_json::Value = serde_json::from_str(&initial_response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, initial_response));
    let initial_symbols = extract_symbols(&initial_json, &initial_response);

    // After force rebuild, removed file's symbols should be gone
    // (This would pass if indexing worked, but search returns empty anyway due to stub)

    // Assert: different_only SHOULD be found
    let different_json: serde_json::Value = serde_json::from_str(&different_response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, different_response));
    let different_symbols = extract_symbols(&different_json, &different_response);

    // FAILS: Force index doesn't actually rebuild
    assert!(
        !different_symbols.is_empty(),
        "Force index should find symbols from new files.\n\
         'different_only' should exist after force re-index.\n\
         Root cause: handle_index is stub\n\
         Search response: {}",
        different_response
    );

    // If force index worked, initial_only should be gone
    assert!(
        initial_symbols.is_empty(),
        "Force index should clear symbols from removed files.\n\
         'initial_only' should not exist after force re-index.\n\
         Search response: {}",
        initial_response
    );
}
