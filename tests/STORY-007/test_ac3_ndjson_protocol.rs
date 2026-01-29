//! AC#3: NDJSON Message Protocol Tests
//!
//! Given: Daemon is running and a client connects
//! When: Client sends a newline-delimited JSON request
//! Then: Daemon parses the request, processes it, and returns a newline-delimited
//!       JSON response with:
//!       - `id` field matching request ID
//!       - `result` field with response data (or null on error)
//!       - `error` field with error details (or null on success)
//!
//! Source files tested:
//!   - src/daemon/protocol.rs (Protocol parsing)
//!   - src/daemon/server.rs (Request handling)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DAEMON-005: Parse NDJSON request messages
//!   - DAEMON-006: Serialize NDJSON response messages

use serde_json::{json, Value};
use std::time::Duration;
use tempfile::TempDir;

// These imports will fail until the daemon module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::{
    DaemonClient, DaemonRequest, DaemonResponse, DaemonServer, ProtocolHandler,
};

/// Test: Valid JSON request is parsed successfully
/// Requirement: DAEMON-005 - Parse NDJSON request messages
#[test]
fn test_valid_json_request_parsed_successfully() {
    // Arrange
    let json_str = r#"{"id": "req-001", "method": "status", "params": {}}"#;

    // Act
    let request: Result<DaemonRequest, _> = serde_json::from_str(json_str);

    // Assert
    assert!(request.is_ok(), "Valid JSON should parse successfully");
    let request = request.unwrap();
    assert_eq!(request.id, "req-001", "Request ID should be 'req-001'");
    assert_eq!(request.method, "status", "Method should be 'status'");
}

/// Test: Invalid JSON returns parse error
/// Requirement: DAEMON-005 - Parse NDJSON request messages
#[test]
fn test_invalid_json_returns_parse_error() {
    // Arrange
    let invalid_json = r#"{"id": "req-001", "method": incomplete"#;

    // Act
    let request: Result<DaemonRequest, _> = serde_json::from_str(invalid_json);

    // Assert
    assert!(request.is_err(), "Invalid JSON should return parse error");
}

/// Test: Request with missing id field fails parsing
/// Requirement: DAEMON-005 - Parse NDJSON request messages
#[test]
fn test_request_missing_id_fails_parsing() {
    // Arrange
    let json_without_id = r#"{"method": "status", "params": {}}"#;

    // Act
    let request: Result<DaemonRequest, _> = serde_json::from_str(json_without_id);

    // Assert
    assert!(
        request.is_err(),
        "Request without 'id' field should fail parsing"
    );
}

/// Test: Request with missing method field fails parsing
/// Requirement: DAEMON-005 - Parse NDJSON request messages
#[test]
fn test_request_missing_method_fails_parsing() {
    // Arrange
    let json_without_method = r#"{"id": "req-001", "params": {}}"#;

    // Act
    let request: Result<DaemonRequest, _> = serde_json::from_str(json_without_method);

    // Assert
    assert!(
        request.is_err(),
        "Request without 'method' field should fail parsing"
    );
}

/// Test: Request with missing params uses empty object
/// Requirement: DAEMON-005 - Parse NDJSON request messages
#[test]
fn test_request_missing_params_uses_empty_object() {
    // Arrange
    let json_without_params = r#"{"id": "req-001", "method": "status"}"#;

    // Act
    let request: Result<DaemonRequest, _> = serde_json::from_str(json_without_params);

    // Assert
    // Depending on implementation, missing params could be an error or default to {}
    // This test verifies the expected behavior
    if let Ok(request) = request {
        assert!(
            request.params.is_null() || request.params.is_object(),
            "Missing params should be null or empty object"
        );
    }
}

/// Test: Response contains id matching request
/// Requirement: DAEMON-006 - Serialize NDJSON response messages
#[test]
fn test_response_id_matches_request() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "test-id-123",
        "method": "status",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    assert_eq!(
        response["id"], "test-id-123",
        "Response ID should match request ID"
    );
}

/// Test: Success response has result field and null error
/// Requirement: DAEMON-006 - Serialize NDJSON response messages
#[test]
fn test_success_response_has_result_and_null_error() {
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
    assert!(
        response.get("result").is_some(),
        "Success response should have 'result' field"
    );
    assert!(
        response.get("error").is_none() || response["error"].is_null(),
        "Success response should have null 'error' field"
    );
}

/// Test: Response is terminated with newline (NDJSON format)
/// Requirement: DAEMON-006 - Serialize NDJSON response messages
#[test]
fn test_response_terminated_with_newline() {
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
    let raw_response = client
        .send_request_raw(&request)
        .expect("Failed to send request");

    // Assert
    assert!(
        raw_response.ends_with('\n'),
        "NDJSON response should end with newline character"
    );
}

/// Test: Response is valid JSON
/// Requirement: DAEMON-006 - Serialize NDJSON response messages
#[test]
fn test_response_is_valid_json() {
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
    let raw_response = client
        .send_request_raw(&request)
        .expect("Failed to send request");

    // Assert
    let parsed: Result<Value, _> = serde_json::from_str(raw_response.trim());
    assert!(
        parsed.is_ok(),
        "Response should be valid JSON, got: {}",
        raw_response
    );
}

/// Test: Multiple requests on same connection work
/// Requirement: DAEMON-005/006 - Protocol supports streaming
#[test]
fn test_multiple_requests_on_same_connection() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act & Assert: Send multiple requests
    for i in 1..=3 {
        let request = json!({
            "id": format!("req-{:03}", i),
            "method": "status",
            "params": {}
        });
        let response = client
            .send_request(&request)
            .expect("Failed to send request");
        assert_eq!(
            response["id"],
            format!("req-{:03}", i),
            "Response {} should match request ID",
            i
        );
    }
}

/// Test: DaemonRequest struct has required fields
/// Requirement: DAEMON-005 - Parse NDJSON request messages
#[test]
fn test_daemon_request_struct_has_required_fields() {
    // Arrange & Act
    let request = DaemonRequest {
        id: "test-id".to_string(),
        method: "status".to_string(),
        params: json!({}),
    };

    // Assert
    assert_eq!(request.id, "test-id");
    assert_eq!(request.method, "status");
    assert!(request.params.is_object());
}

/// Test: DaemonResponse struct has required fields
/// Requirement: DAEMON-006 - Serialize NDJSON response messages
#[test]
fn test_daemon_response_struct_has_required_fields() {
    // Arrange & Act
    let response = DaemonResponse {
        id: "test-id".to_string(),
        result: Some(json!({"status": "ready"})),
        error: None,
    };

    // Assert
    assert_eq!(response.id, "test-id");
    assert!(response.result.is_some());
    assert!(response.error.is_none());
}

/// Test: Empty request body returns error
/// Requirement: DAEMON-005 - Parse NDJSON request messages
#[test]
fn test_empty_request_body_returns_error() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act: Send empty line
    let response = client.send_raw("");

    // Assert
    // Behavior depends on implementation - could return error response or close connection
    // The test verifies the daemon handles this gracefully without crashing
    assert!(
        response.is_err() || response.as_ref().unwrap().get("error").is_some(),
        "Empty request should result in error"
    );
}

/// Test: ProtocolHandler trait can be implemented
/// Requirement: DAEMON-005/006 - Protocol abstraction
#[test]
fn test_protocol_handler_trait_exists() {
    // This test verifies the trait exists at compile time
    // The actual implementation is tested via DaemonServer

    // If this compiles, the trait exists
    fn _assert_trait_exists<T: ProtocolHandler>(_: T) {}

    // Test passes if compilation succeeds
    assert!(true, "ProtocolHandler trait should exist");
}
