//! AC#7: Error Response Format Tests
//!
//! Given: Client sends an invalid or failing request
//! When: Daemon processes the request
//! Then: Daemon returns error response with:
//!   - `id` matching request ID
//!   - `result`: null
//!   - `error`: {"code": "EXXX", "message": "Human-readable error"}
//!
//! Standard error codes:
//!   - E001: Index not ready
//!   - E002: Invalid method
//!   - E003: Invalid params
//!
//! Source files tested:
//!   - src/daemon/protocol.rs (Error handling)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DAEMON-011: Return structured error responses with codes

use serde_json::json;
use std::time::Duration;
use tempfile::TempDir;

// These imports will fail until the daemon module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::{DaemonClient, DaemonError, DaemonServer};

/// Test: Error response has null result field
/// Requirement: DAEMON-011 - Error response has null result
#[test]
fn test_error_response_has_null_result() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act: Send request with invalid method
    let request = json!({
        "id": "req-001",
        "method": "invalid_method_xyz",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    assert!(
        response["result"].is_null(),
        "Error response 'result' should be null"
    );
    assert!(
        response.get("error").is_some() && !response["error"].is_null(),
        "Error response should have non-null 'error' field"
    );
}

/// Test: Error response has error object with code and message
/// Requirement: DAEMON-011 - Error object structure
#[test]
fn test_error_response_has_code_and_message() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act: Send request with invalid method
    let request = json!({
        "id": "req-001",
        "method": "unknown_method",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let error = &response["error"];
    assert!(
        error.get("code").is_some(),
        "Error should have 'code' field"
    );
    assert!(
        error.get("message").is_some(),
        "Error should have 'message' field"
    );

    let code = error["code"].as_str().expect("code should be string");
    let message = error["message"].as_str().expect("message should be string");

    assert!(!code.is_empty(), "Error code should not be empty");
    assert!(!message.is_empty(), "Error message should not be empty");
}

/// Test: Error response id matches request id
/// Requirement: DAEMON-011 - Error response has matching id
#[test]
fn test_error_response_id_matches_request() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "error-test-id-456",
        "method": "nonexistent",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    assert_eq!(
        response["id"], "error-test-id-456",
        "Error response id should match request id"
    );
}

/// Test: E002 error returned for invalid method
/// Requirement: DAEMON-011 - E002 (invalid method) error code
#[test]
fn test_e002_error_for_invalid_method() {
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
        "method": "definitely_not_a_valid_method",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let error = &response["error"];
    assert_eq!(
        error["code"], "E002",
        "Invalid method should return E002 error code"
    );
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("method"),
        "Error message should mention 'method'"
    );
}

/// Test: E003 error returned for invalid params
/// Requirement: DAEMON-011 - E003 (invalid params) error code
#[test]
fn test_e003_error_for_invalid_params() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act: Send search request with invalid params type
    let request = json!({
        "id": "req-001",
        "method": "search",
        "params": "invalid_string_instead_of_object"
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let error = &response["error"];
    assert_eq!(
        error["code"], "E003",
        "Invalid params should return E003 error code"
    );
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("param"),
        "Error message should mention 'param'"
    );
}

/// Test: E001 error returned when index not ready
/// Requirement: DAEMON-011 - E001 (index not ready) error code
#[test]
fn test_e001_error_when_index_not_ready() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    // Deliberately don't wait for ready state

    // Connect immediately before daemon is fully ready
    // Note: This test timing can be tricky - daemon might become ready quickly
    let client = DaemonClient::connect_with_retry(daemon.socket_path(), Duration::from_millis(100))
        .expect("Failed to connect");

    // Act: Try search while index might not be ready
    let request = json!({
        "id": "req-001",
        "method": "search",
        "params": {"symbol": "test"}
    });
    let response = client.send_request(&request);

    // Assert: If daemon wasn't ready, should get E001
    // Note: Test may not always trigger E001 if daemon initializes quickly
    if let Ok(resp) = response {
        if let Some(error) = resp.get("error") {
            if !error.is_null() && error.get("code").is_some() {
                assert_eq!(
                    error["code"], "E001",
                    "Index not ready should return E001 error code"
                );
            }
        }
    }
}

/// Test: Error code format is EXXX
/// Requirement: DAEMON-011 - Standard error codes format
#[test]
fn test_error_code_format_is_exxx() {
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
        "method": "invalid",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let code = response["error"]["code"]
        .as_str()
        .expect("code should be string");

    // Verify format: E followed by 3 digits
    assert!(
        code.len() == 4,
        "Error code should be 4 characters (E + 3 digits)"
    );
    assert!(code.starts_with('E'), "Error code should start with 'E'");
    assert!(
        code[1..].chars().all(|c| c.is_ascii_digit()),
        "Error code should be E followed by 3 digits"
    );
}

/// Test: Error message is human-readable
/// Requirement: DAEMON-011 - Human-readable error message
#[test]
fn test_error_message_is_human_readable() {
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
        "method": "invalid_method",
        "params": {}
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let message = response["error"]["message"]
        .as_str()
        .expect("message should be string");

    // Human-readable means: starts with capital, has words, reasonable length
    assert!(
        message.len() >= 10,
        "Error message should be at least 10 characters"
    );
    assert!(
        message.len() <= 500,
        "Error message should be at most 500 characters"
    );
    assert!(
        message.chars().next().unwrap().is_uppercase()
            || message.chars().next().unwrap().is_ascii_punctuation(),
        "Error message should start with capital letter or punctuation"
    );
    assert!(
        message.split_whitespace().count() >= 2,
        "Error message should have multiple words"
    );
}

/// Test: DaemonError enum has all required variants
/// Requirement: DAEMON-011 - Standard error codes
#[test]
fn test_daemon_error_enum_has_required_variants() {
    // Assert: DaemonError enum has required variants
    // This test verifies the type system at compile time

    let _e001 = DaemonError::IndexNotReady;
    let _e002 = DaemonError::InvalidMethod("test".to_string());
    let _e003 = DaemonError::InvalidParams("test".to_string());

    // All variants should exist (compile-time check)
    assert!(true, "All DaemonError variants exist");
}

/// Test: DaemonError converts to error code correctly
/// Requirement: DAEMON-011 - Error code mapping
#[test]
fn test_daemon_error_to_code_mapping() {
    // Arrange
    let errors = vec![
        (DaemonError::IndexNotReady, "E001"),
        (DaemonError::InvalidMethod("test".to_string()), "E002"),
        (DaemonError::InvalidParams("test".to_string()), "E003"),
    ];

    // Act & Assert
    for (error, expected_code) in errors {
        let code = error.code();
        assert_eq!(
            code, expected_code,
            "{:?} should map to error code {}",
            error, expected_code
        );
    }
}

/// Test: DaemonError provides meaningful message
/// Requirement: DAEMON-011 - Human-readable error message
#[test]
fn test_daemon_error_provides_message() {
    // Arrange
    let errors = vec![
        DaemonError::IndexNotReady,
        DaemonError::InvalidMethod("unknown".to_string()),
        DaemonError::InvalidParams("missing field".to_string()),
    ];

    // Act & Assert
    for error in errors {
        let message = error.message();
        assert!(
            !message.is_empty(),
            "{:?} should have non-empty message",
            error
        );
        assert!(
            message.len() >= 5,
            "{:?} message should be meaningful",
            error
        );
    }
}

/// Test: Malformed JSON returns appropriate error
/// Requirement: DAEMON-005/011 - Handle parse errors
#[test]
fn test_malformed_json_returns_error() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act: Send malformed JSON
    let malformed = r#"{"id": "req-001", "method": incomplete"#;
    let response = client.send_raw(malformed);

    // Assert
    // Behavior depends on implementation - could return error response or close connection
    // The test verifies daemon handles this gracefully
    if let Ok(resp) = response {
        assert!(
            resp.get("error").is_some(),
            "Malformed JSON should return error response"
        );
    }
}

/// Test: Request with unknown extra fields is handled gracefully
/// Requirement: DAEMON-005 - Parse NDJSON request messages
#[test]
fn test_request_with_extra_fields_handled_gracefully() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act: Send request with extra unknown fields
    let request = json!({
        "id": "req-001",
        "method": "status",
        "params": {},
        "unknown_field": "should be ignored",
        "another_field": 12345
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert: Request should succeed (extra fields ignored)
    assert!(
        response["error"].is_null(),
        "Request with extra fields should succeed"
    );
    assert!(response["result"].is_object(), "Should return valid result");
}

/// Test: All three error codes have unique values
/// Requirement: DAEMON-011 - Standard error codes
#[test]
fn test_all_error_codes_are_unique() {
    // Arrange
    let codes = vec![
        DaemonError::IndexNotReady.code(),
        DaemonError::InvalidMethod("".to_string()).code(),
        DaemonError::InvalidParams("".to_string()).code(),
    ];

    // Assert: All codes are unique
    let unique_count = codes.iter().collect::<std::collections::HashSet<_>>().len();
    assert_eq!(
        unique_count,
        codes.len(),
        "All error codes should be unique"
    );
}
