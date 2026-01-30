//! AC#4: Status Reflects Index State Tests
//!
//! Given: Daemon is running
//! When: Client sends status request during or after indexing
//! Then:
//!   - indexed_files: Actual count from IndexStorage
//!   - indexed_symbols: Actual symbol count
//!   - last_index_time: Timestamp of last completed index
//!   - Status reflects current state (ready, indexing, etc.)
//!
//! Source files tested:
//!   - src/daemon/server.rs (handle_status implementation)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - INT-006: Query IndexStorage for actual file and symbol counts
//!   - INT-007: Track and report last_index_time
//!
//! TDD Phase: RED
//!
//! These tests will FAIL because handle_index() is a stub that doesn't
//! actually index files, so status counts remain at 0 after indexing.

use assert_cmd::Command;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Helper: Create test project with known file/symbol counts
fn setup_test_project(temp_dir: &TempDir, file_count: usize) -> (usize, usize) {
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let mut total_symbols = 0;

    for i in 0..file_count {
        // Each file has 2 functions = 2 symbols
        let content = format!(
            r#"
def function_a_{i}():
    return {i}

def function_b_{i}():
    return {i} * 2
"#,
            i = i
        );
        fs::write(src_dir.join(format!("module_{}.py", i)), content)
            .expect("Failed to write file");
        total_symbols += 2;
    }

    (file_count, total_symbols)
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
        .set_read_timeout(Some(Duration::from_secs(5)))
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

/// Test: Status returns actual indexed_files count from IndexStorage
/// Requirement: INT-006 - Query IndexStorage for actual file and symbol counts
///
/// This test WILL FAIL because handle_index is a stub that doesn't update counts.
#[test]
fn test_status_returns_actual_indexed_files_count() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let (expected_files, _) = setup_test_project(&temp_dir, 5);
    let socket_path = start_daemon(&temp_dir);

    // Index the project
    let index_request = r#"{"id": "index-1", "method": "index", "params": {}}"#;
    let _ = send_daemon_request(&socket_path, index_request);

    // Act: Get status
    let status_request = r#"{"id": "status-1", "method": "status", "params": {}}"#;
    let response = send_daemon_request(&socket_path, status_request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: indexed_files matches actual count
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    // FAILS: Status doesn't reflect actual IndexStorage state
    let indexed_files = json["result"]["indexed_files"]
        .as_u64()
        .expect("indexed_files should be number");

    assert_eq!(
        indexed_files, expected_files as u64,
        "Status indexed_files should match actual count from IndexStorage.\n\
         Expected: {} files\n\
         Actual: {} files\n\
         Root cause: handle_index is stub - doesn't update counts\n\
         Response: {}",
        expected_files,
        indexed_files,
        response
    );
}

/// Test: Status returns actual indexed_symbols count from IndexStorage
/// Requirement: INT-006 - Query IndexStorage for actual file and symbol counts
///
/// This test WILL FAIL because handle_index is a stub.
#[test]
fn test_status_returns_actual_indexed_symbols_count() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let (_, expected_symbols) = setup_test_project(&temp_dir, 3);
    let socket_path = start_daemon(&temp_dir);

    // Index the project
    let index_request = r#"{"id": "index-1", "method": "index", "params": {}}"#;
    let _ = send_daemon_request(&socket_path, index_request);

    // Act: Get status
    let status_request = r#"{"id": "status-1", "method": "status", "params": {}}"#;
    let response = send_daemon_request(&socket_path, status_request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: indexed_symbols matches actual count
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    // FAILS: Status doesn't query IndexStorage
    let indexed_symbols = json["result"]["indexed_symbols"]
        .as_u64()
        .expect("indexed_symbols should be number");

    assert!(
        indexed_symbols > 0,
        "Status should report actual symbol count after indexing.\n\
         Expected: approximately {} symbols\n\
         Actual: {} symbols\n\
         Root cause: handle_index is stub\n\
         Response: {}",
        expected_symbols,
        indexed_symbols,
        response
    );

    // Allow some variance
    let min_expected = (expected_symbols as f64 * 0.5) as u64;
    assert!(
        indexed_symbols >= min_expected,
        "Status should report at least half expected symbols.\n\
         Expected: >= {} symbols\n\
         Actual: {} symbols\n\
         Response: {}",
        min_expected,
        indexed_symbols,
        response
    );
}

/// Test: Status last_index_time updates after indexing
/// Requirement: INT-007 - Track and report last_index_time
///
/// This test may PASS if last_index_time updates even without actual indexing.
#[test]
fn test_status_last_index_time_updates_after_indexing() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let _ = setup_test_project(&temp_dir, 2);
    let socket_path = start_daemon(&temp_dir);

    // Get status before indexing
    let status_before = r#"{"id": "status-1", "method": "status", "params": {}}"#;
    let before_response = send_daemon_request(&socket_path, status_before);
    let before_json: serde_json::Value = serde_json::from_str(&before_response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, before_response));

    let before_time = before_json["result"]["last_index_time"].as_str();

    // Index the project
    let index_request = r#"{"id": "index-1", "method": "index", "params": {}}"#;
    let _ = send_daemon_request(&socket_path, index_request);

    // Act: Get status after indexing
    let status_after = r#"{"id": "status-2", "method": "status", "params": {}}"#;
    let after_response = send_daemon_request(&socket_path, status_after);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: last_index_time should be set (not null)
    let after_json: serde_json::Value = serde_json::from_str(&after_response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, after_response));

    let after_time = after_json["result"]["last_index_time"].as_str();

    assert!(
        after_time.is_some() && !after_time.unwrap().is_empty(),
        "last_index_time should be set after indexing.\n\
         Before: {:?}\n\
         After: {:?}\n\
         Response: {}",
        before_time,
        after_time,
        after_response
    );

    // If before_time was null, after_time should now have a value
    if before_time.is_none() || before_time == Some("") {
        assert!(
            after_time.is_some(),
            "last_index_time should change from null to timestamp after indexing"
        );
    }
}

/// Test: Status reflects correct state during indexing
/// Requirement: AC#4 - Status reflects current state (ready, indexing, etc.)
///
/// This test should PASS - verifies basic status response.
#[test]
fn test_status_reflects_ready_state() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let socket_path = start_daemon(&temp_dir);

    // Give daemon time to fully start
    thread::sleep(Duration::from_millis(100));

    // Act: Get status when idle (ready state)
    let request = r#"{"id": "status-1", "method": "status", "params": {}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Status should be "ready" when idle
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    let status = json["result"]["status"]
        .as_str()
        .expect("status should be string");

    assert_eq!(
        status, "ready",
        "Status should be 'ready' when daemon is idle.\n\
         Response: {}",
        response
    );
}

/// Test: Status before any indexing shows zero counts
/// Requirement: BR-003 - Return 0 counts if storage not initialized
///
/// This test should PASS - verifies initial state.
#[test]
fn test_status_before_indexing_shows_zero_counts() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let socket_path = start_daemon(&temp_dir);

    // Act: Get status immediately (no indexing yet)
    let request = r#"{"id": "status-1", "method": "status", "params": {}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Counts should be 0
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    let indexed_files = json["result"]["indexed_files"]
        .as_u64()
        .expect("indexed_files should be number");

    assert_eq!(
        indexed_files, 0,
        "Before indexing, indexed_files should be 0.\n\
         Response: {}",
        response
    );

    let indexed_symbols = json["result"]["indexed_symbols"]
        .as_u64()
        .expect("indexed_symbols should be number");

    assert_eq!(
        indexed_symbols, 0,
        "Before indexing, indexed_symbols should be 0.\n\
         Response: {}",
        response
    );
}

/// Test: Status includes required fields
/// Requirement: AC#4 - Status response includes accurate index state
///
/// This test should PASS - verifies response structure.
#[test]
fn test_status_includes_all_required_fields() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let socket_path = start_daemon(&temp_dir);

    // Act: Get status
    let request = r#"{"id": "status-1", "method": "status", "params": {}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: All required fields present
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    let result = &json["result"];

    assert!(
        result.get("status").is_some(),
        "Status response should include 'status' field.\n\
         Response: {}",
        response
    );

    assert!(
        result.get("indexed_files").is_some(),
        "Status response should include 'indexed_files' field.\n\
         Response: {}",
        response
    );

    assert!(
        result.get("indexed_symbols").is_some(),
        "Status response should include 'indexed_symbols' field.\n\
         Response: {}",
        response
    );

    assert!(
        result.get("last_index_time").is_some(),
        "Status response should include 'last_index_time' field.\n\
         Response: {}",
        response
    );

    assert!(
        result.get("uptime_seconds").is_some(),
        "Status response should include 'uptime_seconds' field.\n\
         Response: {}",
        response
    );

    assert!(
        result.get("pid").is_some(),
        "Status response should include 'pid' field.\n\
         Response: {}",
        response
    );
}

/// Test: Status indexed_files matches IndexStorage after indexing
/// Requirement: INT-006 - indexed_files matches storage.get_all_tracked_files().len()
///
/// This test WILL FAIL because handle_index is a stub.
#[test]
fn test_status_indexed_files_matches_storage_query() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create exactly 7 files
    for i in 0..7 {
        fs::write(src_dir.join(format!("file_{}.py", i)), format!("def f{}(): pass", i))
            .expect("Failed to write");
    }

    let socket_path = start_daemon(&temp_dir);

    // Index
    let index_request = r#"{"id": "index-1", "method": "index", "params": {}}"#;
    let _ = send_daemon_request(&socket_path, index_request);

    // Act: Get status
    let status_request = r#"{"id": "status-1", "method": "status", "params": {}}"#;
    let response = send_daemon_request(&socket_path, status_request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Exact match with expected count
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    // FAILS: Status doesn't query storage
    let indexed_files = json["result"]["indexed_files"]
        .as_u64()
        .expect("indexed_files should be number");

    assert_eq!(
        indexed_files, 7,
        "indexed_files should exactly match storage.get_all_tracked_files().len().\n\
         Expected: 7\n\
         Actual: {}\n\
         Root cause: handle_index is stub\n\
         Response: {}",
        indexed_files,
        response
    );
}

/// Test: Status last_index_time is null before first index
/// Requirement: INT-007 - last_index_time is None before first index, Some after
///
/// This test should PASS - verifies initial state.
#[test]
fn test_status_last_index_time_null_before_first_index() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let socket_path = start_daemon(&temp_dir);

    // Act: Get status immediately (no index yet)
    let request = r#"{"id": "status-1", "method": "status", "params": {}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: last_index_time should be null
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    let last_index_time = &json["result"]["last_index_time"];

    assert!(
        last_index_time.is_null() || last_index_time.as_str() == Some(""),
        "last_index_time should be null before first index.\n\
         Actual: {:?}\n\
         Response: {}",
        last_index_time,
        response
    );
}
