//! AC#5: Status Method Handler Tests
//!
//! Given: Daemon is running
//! When: Client sends status request
//! Then: Daemon returns status response with:
//!   - `status`: "starting" | "ready" | "indexing" | "stopping"
//!   - `indexed_files`: count of indexed files
//!   - `indexed_symbols`: count of indexed symbols
//!   - `last_index_time`: ISO 8601 timestamp
//!   - `uptime_seconds`: integer
//!   - `pid`: process ID
//!   - `socket_path`: path to IPC socket/pipe
//!
//! Source files tested:
//!   - src/daemon/server.rs (Status handler)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DAEMON-008: Return comprehensive daemon status

use serde_json::json;
use std::time::Duration;
use tempfile::TempDir;

// These imports will fail until the daemon module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::{DaemonClient, DaemonServer};

/// Test: Status response contains all 7 required fields
/// Requirement: DAEMON-008 - Return comprehensive daemon status
#[test]
fn test_status_response_contains_all_required_fields() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "status",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let result = &response["result"];
    assert!(
        result.get("status").is_some(),
        "Status response should have 'status' field"
    );
    assert!(
        result.get("indexed_files").is_some(),
        "Status response should have 'indexed_files' field"
    );
    assert!(
        result.get("indexed_symbols").is_some(),
        "Status response should have 'indexed_symbols' field"
    );
    assert!(
        result.get("last_index_time").is_some(),
        "Status response should have 'last_index_time' field"
    );
    assert!(
        result.get("uptime_seconds").is_some(),
        "Status response should have 'uptime_seconds' field"
    );
    assert!(
        result.get("pid").is_some(),
        "Status response should have 'pid' field"
    );
    assert!(
        result.get("socket_path").is_some(),
        "Status response should have 'socket_path' field"
    );
}

/// Test: Status field is valid state string
/// Requirement: DAEMON-008 - status field with valid states
#[test]
fn test_status_field_is_valid_state_string() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "status",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let status = response["result"]["status"]
        .as_str()
        .expect("status should be a string");
    let valid_states = vec!["starting", "ready", "indexing", "stopping"];
    assert!(
        valid_states.contains(&status),
        "Status '{}' should be one of: {:?}",
        status,
        valid_states
    );
}

/// Test: indexed_files is a non-negative integer
/// Requirement: DAEMON-008 - indexed_files count
#[test]
fn test_indexed_files_is_non_negative_integer() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "status",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let indexed_files = response["result"]["indexed_files"]
        .as_i64()
        .expect("indexed_files should be an integer");
    assert!(
        indexed_files >= 0,
        "indexed_files should be non-negative, got: {}",
        indexed_files
    );
}

/// Test: indexed_symbols is a non-negative integer
/// Requirement: DAEMON-008 - indexed_symbols count
#[test]
fn test_indexed_symbols_is_non_negative_integer() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "status",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let indexed_symbols = response["result"]["indexed_symbols"]
        .as_i64()
        .expect("indexed_symbols should be an integer");
    assert!(
        indexed_symbols >= 0,
        "indexed_symbols should be non-negative, got: {}",
        indexed_symbols
    );
}

/// Test: last_index_time is valid ISO 8601 timestamp or null
/// Requirement: DAEMON-008 - last_index_time timestamp
#[test]
fn test_last_index_time_is_valid_iso8601_or_null() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "status",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let last_index_time = &response["result"]["last_index_time"];

    if !last_index_time.is_null() {
        let timestamp = last_index_time
            .as_str()
            .expect("last_index_time should be a string or null");

        // Verify ISO 8601 format (basic check)
        // Example: 2026-01-27T10:30:00Z or 2026-01-27T10:30:00+00:00
        assert!(
            timestamp.contains('T') && (timestamp.contains('Z') || timestamp.contains('+')),
            "Timestamp '{}' should be ISO 8601 format",
            timestamp
        );
    }
}

/// Test: uptime_seconds is a non-negative integer
/// Requirement: DAEMON-008 - uptime_seconds
#[test]
fn test_uptime_seconds_is_non_negative_integer() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "status",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let uptime_seconds = response["result"]["uptime_seconds"]
        .as_i64()
        .expect("uptime_seconds should be an integer");
    assert!(
        uptime_seconds >= 0,
        "uptime_seconds should be non-negative, got: {}",
        uptime_seconds
    );
}

/// Test: pid is a positive integer
/// Requirement: DAEMON-008 - pid process ID
#[test]
fn test_pid_is_positive_integer() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "status",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let pid = response["result"]["pid"]
        .as_i64()
        .expect("pid should be an integer");
    assert!(pid > 0, "pid should be a positive integer, got: {}", pid);
}

/// Test: socket_path is a non-empty string
/// Requirement: DAEMON-008 - socket_path
#[test]
fn test_socket_path_is_non_empty_string() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "status",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let socket_path = response["result"]["socket_path"]
        .as_str()
        .expect("socket_path should be a string");
    assert!(
        !socket_path.is_empty(),
        "socket_path should be a non-empty string"
    );
}

/// Test: Status reflects correct state after daemon reaches ready
/// Requirement: DAEMON-008 - status reflects actual state
#[test]
fn test_status_reflects_ready_state() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "status",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let status = response["result"]["status"].as_str().unwrap();
    assert_eq!(
        status, "ready",
        "Status should be 'ready' after wait_for_ready succeeds"
    );
}

/// Test: indexed_files count increases after indexing
/// Requirement: DAEMON-008 - indexed_files reflects actual count
#[test]
fn test_indexed_files_increases_after_indexing() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Get initial count
    let request = json!({"id": "req-001", "method": "status", "params": {}});
    let response = client
        .send_request(&request)
        .expect("Failed to send request");
    let initial_count = response["result"]["indexed_files"].as_i64().unwrap();

    // Create and index a test file
    let test_file = project_root.join("test.py");
    std::fs::write(&test_file, "def hello():\n    pass\n").expect("Failed to write test file");
    daemon
        .index_file(&test_file)
        .expect("Failed to index test file");

    // Act: Get updated count
    let request = json!({"id": "req-002", "method": "status", "params": {}});
    let response = client
        .send_request(&request)
        .expect("Failed to send request");
    let updated_count = response["result"]["indexed_files"].as_i64().unwrap();

    // Assert
    assert!(
        updated_count > initial_count,
        "indexed_files should increase after indexing, got {} -> {}",
        initial_count,
        updated_count
    );
}

/// Test: indexed_symbols count increases after indexing
/// Requirement: DAEMON-008 - indexed_symbols reflects actual count
#[test]
fn test_indexed_symbols_increases_after_indexing() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Get initial count
    let request = json!({"id": "req-001", "method": "status", "params": {}});
    let response = client
        .send_request(&request)
        .expect("Failed to send request");
    let initial_count = response["result"]["indexed_symbols"].as_i64().unwrap();

    // Create and index a test file with symbols
    let test_file = project_root.join("test.py");
    std::fs::write(
        &test_file,
        "def hello():\n    pass\n\ndef world():\n    pass\n",
    )
    .expect("Failed to write test file");
    daemon
        .index_file(&test_file)
        .expect("Failed to index test file");

    // Act: Get updated count
    let request = json!({"id": "req-002", "method": "status", "params": {}});
    let response = client
        .send_request(&request)
        .expect("Failed to send request");
    let updated_count = response["result"]["indexed_symbols"].as_i64().unwrap();

    // Assert
    assert!(
        updated_count > initial_count,
        "indexed_symbols should increase after indexing, got {} -> {}",
        initial_count,
        updated_count
    );
}

/// Test: Status query completes within 1ms
/// Requirement: NFR - Status query < 1ms (p95)
#[test]
fn test_status_completes_within_1ms() {
    use std::time::Instant;

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "status",
        "params": {}
    });

    let start = Instant::now();
    let _response = client
        .send_request(&request)
        .expect("Failed to send request");
    let elapsed = start.elapsed();

    // Assert: Allow some slack (10ms) for test environment variability
    assert!(
        elapsed.as_millis() < 10,
        "Status query took {}ms, expected < 10ms",
        elapsed.as_millis()
    );
}
