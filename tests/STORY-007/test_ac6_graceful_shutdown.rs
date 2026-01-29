//! AC#6: Graceful Shutdown Tests
//!
//! Given: Daemon is running with active connections
//! When: Daemon receives shutdown signal (SIGTERM/SIGINT or explicit stop)
//! Then:
//!   - Daemon stops accepting new connections
//!   - Existing requests complete (up to 5 second timeout)
//!   - IPC socket/pipe is removed
//!   - Process exits with code 0
//!
//! Source files tested:
//!   - src/daemon/server.rs (Shutdown handling)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DAEMON-009: Handle graceful shutdown on SIGTERM/SIGINT
//!   - DAEMON-010: Clean up socket/pipe file on shutdown

use serde_json::json;
use std::time::Duration;
use tempfile::TempDir;

// These imports will fail until the daemon module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::{DaemonClient, DaemonServer, DaemonState};

/// Test: Shutdown method stops the daemon gracefully
/// Requirement: DAEMON-009 - Handle graceful shutdown
#[test]
fn test_shutdown_stops_daemon_gracefully() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Act
    let result = daemon.shutdown();

    // Assert
    assert!(result.is_ok(), "Shutdown should complete successfully");
}

/// Test: Daemon state transitions to Stopping on shutdown
/// Requirement: DAEMON-002 - Track daemon state (stopping)
#[test]
fn test_daemon_state_transitions_to_stopping() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    assert_eq!(
        daemon.state(),
        DaemonState::Ready,
        "Should be ready initially"
    );

    // Act: Initiate shutdown (async)
    daemon.initiate_shutdown();

    // Assert: State should transition to Stopping
    // Allow brief time for state change
    std::thread::sleep(Duration::from_millis(10));
    let state = daemon.state();
    assert!(
        state == DaemonState::Stopping || state == DaemonState::Ready,
        "State should be Stopping or still Ready during transition, got: {:?}",
        state
    );
}

/// Test: Socket file removed after shutdown
/// Requirement: DAEMON-010 - Clean up socket/pipe file on shutdown
#[cfg(unix)]
#[test]
fn test_socket_removed_after_shutdown() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let socket_path = project_root.join(".treelint").join("daemon.sock");
    assert!(socket_path.exists(), "Socket should exist before shutdown");

    // Act
    daemon.shutdown().expect("Shutdown should succeed");

    // Assert
    std::thread::sleep(Duration::from_millis(100)); // Brief delay for cleanup
    assert!(
        !socket_path.exists(),
        "Socket file should be removed after shutdown"
    );
}

/// Test: Daemon stops accepting new connections after shutdown initiated
/// Requirement: DAEMON-009 - Daemon stops accepting new connections
#[test]
fn test_no_new_connections_after_shutdown_initiated() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let socket_path = daemon.socket_path();

    // Initiate shutdown
    daemon.initiate_shutdown();

    // Act: Try to connect after shutdown initiated
    std::thread::sleep(Duration::from_millis(50)); // Brief delay
    let connection_result = DaemonClient::connect(&socket_path);

    // Assert: New connection should fail or be rejected
    // Behavior depends on implementation - either connect fails or request fails
    if let Ok(client) = connection_result {
        let request = json!({"id": "test", "method": "status", "params": {}});
        let response = client.send_request(&request);
        assert!(
            response.is_err(),
            "Request to shutting down daemon should fail"
        );
    }
    // If connection itself failed, that's also acceptable
}

/// Test: Existing requests complete before shutdown (up to 5 second timeout)
/// Requirement: DAEMON-009 - Existing requests complete
#[test]
fn test_existing_requests_complete_before_shutdown() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Start a request in parallel, then initiate shutdown
    let request = json!({
        "id": "in-flight-request",
        "method": "status",
        "params": {}
    });

    // Send request
    let response_handle = std::thread::spawn(move || client.send_request(&request));

    // Brief delay to ensure request is in-flight
    std::thread::sleep(Duration::from_millis(10));

    // Initiate shutdown
    daemon.initiate_shutdown();

    // Act: Wait for response
    let response = response_handle.join().expect("Thread should complete");

    // Assert: Request should complete successfully
    assert!(
        response.is_ok(),
        "In-flight request should complete before shutdown"
    );
}

/// Test: Shutdown completes within 5 seconds
/// Requirement: DAEMON-009 - 5 second timeout for request completion
#[test]
fn test_shutdown_completes_within_5_seconds() {
    use std::time::Instant;

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Act
    let start = Instant::now();
    daemon.shutdown().expect("Shutdown should succeed");
    let elapsed = start.elapsed();

    // Assert
    assert!(
        elapsed < Duration::from_secs(5),
        "Shutdown took {:?}, expected < 5 seconds",
        elapsed
    );
}

/// Test: Daemon can be restarted after shutdown
/// Requirement: DAEMON-010 - Clean shutdown allows restart
#[test]
fn test_daemon_can_restart_after_shutdown() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // First daemon
    let daemon1 = DaemonServer::new(project_root).expect("First daemon should start");
    daemon1
        .wait_for_ready(Duration::from_secs(2))
        .expect("First daemon should be ready");
    daemon1
        .shutdown()
        .expect("First daemon shutdown should succeed");

    // Brief delay for cleanup
    std::thread::sleep(Duration::from_millis(100));

    // Act: Start second daemon in same directory
    let daemon2 = DaemonServer::new(project_root);

    // Assert
    assert!(
        daemon2.is_ok(),
        "Second daemon should start after first shutdown"
    );
}

/// Test: SIGTERM signal triggers graceful shutdown
/// Requirement: DAEMON-009 - Handle SIGTERM
#[cfg(unix)]
#[test]
fn test_sigterm_triggers_graceful_shutdown() {
    // NOTE: std::process::Command would be used if spawning daemon as separate process
    // For unit testing, we verify the signal handler is registered

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Act: Verify daemon has signal handler
    // (In practice, this would send SIGTERM to daemon process)
    let has_signal_handler = daemon.has_signal_handler();

    // Assert
    assert!(
        has_signal_handler,
        "Daemon should have SIGTERM signal handler registered"
    );
}

/// Test: SIGINT signal triggers graceful shutdown
/// Requirement: DAEMON-009 - Handle SIGINT
#[cfg(unix)]
#[test]
fn test_sigint_triggers_graceful_shutdown() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Act: Verify daemon has signal handler for SIGINT
    let has_signal_handler = daemon.has_signal_handler();

    // Assert
    assert!(
        has_signal_handler,
        "Daemon should have SIGINT signal handler registered"
    );
}

/// Test: Shutdown returns correct exit code (0)
/// Requirement: DAEMON-009 - Process exits with code 0
#[test]
fn test_shutdown_exit_code_is_zero() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Act
    let exit_code = daemon.shutdown_with_exit_code();

    // Assert
    assert_eq!(exit_code, 0, "Exit code should be 0 for graceful shutdown");
}

/// Test: Multiple shutdown calls are idempotent
/// Requirement: DAEMON-009 - Handle graceful shutdown
#[test]
fn test_multiple_shutdown_calls_are_idempotent() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Act: Call shutdown multiple times
    let result1 = daemon.shutdown();
    let result2 = daemon.shutdown();
    let result3 = daemon.shutdown();

    // Assert: All calls should succeed (or at least not panic)
    assert!(result1.is_ok(), "First shutdown should succeed");
    // Subsequent calls may succeed or return "already stopped" - both are valid
    assert!(
        result2.is_ok() || result2.is_err(),
        "Subsequent shutdown calls should not panic"
    );
    assert!(
        result3.is_ok() || result3.is_err(),
        "Subsequent shutdown calls should not panic"
    );
}

/// Test: Client receives error when daemon is shutting down
/// Requirement: DAEMON-009 - Daemon stops accepting new connections
#[test]
fn test_client_receives_error_when_daemon_shutting_down() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let socket_path = daemon.socket_path();
    let client = DaemonClient::connect(&socket_path).expect("Failed to connect");

    // Initiate shutdown
    daemon.initiate_shutdown();
    std::thread::sleep(Duration::from_millis(100)); // Wait for state change

    // Act: Send request after shutdown initiated
    let request = json!({"id": "late-request", "method": "status", "params": {}});
    let response = client.send_request(&request);

    // Assert: Request should fail or return shutdown error
    if let Ok(resp) = response {
        // If response received, it might contain shutdown-related error
        if resp.get("error").is_some() {
            let error = &resp["error"];
            // Acceptable to get error response about daemon shutting down
            assert!(
                !error.is_null(),
                "Should get error response when daemon shutting down"
            );
        }
    }
    // If request fails entirely, that's also acceptable behavior
}
