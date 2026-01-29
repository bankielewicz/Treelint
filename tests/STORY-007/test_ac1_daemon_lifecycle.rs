//! AC#1: Daemon Process Lifecycle Tests
//!
//! Given: No daemon process is running
//! When: The daemon is started programmatically or via internal API
//! Then: A background daemon process spawns, creates the IPC socket/pipe,
//!       and enters ready state within 2 seconds
//!
//! Source files tested:
//!   - src/daemon/server.rs (DaemonServer implementation)
//!   - src/daemon/mod.rs (Daemon module)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DAEMON-001: Start daemon process and enter event loop
//!   - DAEMON-002: Track daemon state (starting, ready, indexing, stopping)

use std::time::{Duration, Instant};

use tempfile::TempDir;

// These imports will fail until the daemon module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::{DaemonServer, DaemonState};

/// Test: Daemon starts successfully when no existing daemon is running
/// Requirement: DAEMON-001 - Start daemon process and enter event loop
#[test]
fn test_daemon_starts_successfully_when_no_existing_daemon() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Act
    let daemon = DaemonServer::new(project_root);

    // Assert
    assert!(
        daemon.is_ok(),
        "Daemon should start successfully when no existing daemon is running"
    );
}

/// Test: Daemon enters ready state within 2 seconds
/// Requirement: DAEMON-001 - Start daemon process and enter event loop
#[test]
fn test_daemon_enters_ready_state_within_2_seconds() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();
    let start_time = Instant::now();

    // Act
    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    let state = daemon.wait_for_ready(Duration::from_secs(2));

    // Assert
    let elapsed = start_time.elapsed();
    assert!(
        state.is_ok(),
        "Daemon should enter ready state within 2 seconds"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "Ready state achieved in {:?}, expected < 2 seconds",
        elapsed
    );
}

/// Test: Daemon state transitions from starting to ready
/// Requirement: DAEMON-002 - Track daemon state (starting, ready, indexing, stopping)
#[test]
fn test_daemon_state_transitions_from_starting_to_ready() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");

    // Act
    let initial_state = daemon.state();
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");
    let ready_state = daemon.state();

    // Assert
    assert_eq!(
        initial_state,
        DaemonState::Starting,
        "Initial state should be Starting"
    );
    assert_eq!(
        ready_state,
        DaemonState::Ready,
        "Final state should be Ready"
    );
}

/// Test: Daemon state enum has all required variants
/// Requirement: DAEMON-002 - Track daemon state (starting, ready, indexing, stopping)
#[test]
fn test_daemon_state_enum_has_all_required_variants() {
    // Assert: DaemonState enum has required variants
    // This test verifies the type system at compile time
    let _starting = DaemonState::Starting;
    let _ready = DaemonState::Ready;
    let _indexing = DaemonState::Indexing;
    let _stopping = DaemonState::Stopping;

    // All variants should exist (compile-time check)
    assert!(true, "All DaemonState variants exist");
}

/// Test: Daemon creates IPC socket during startup
/// Requirement: DAEMON-001 - Start daemon process and enter event loop
#[test]
fn test_daemon_creates_ipc_socket_during_startup() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Act
    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Assert
    let socket_path = project_root.join(".treelint").join("daemon.sock");
    #[cfg(unix)]
    assert!(
        socket_path.exists(),
        "Unix socket should be created at .treelint/daemon.sock"
    );

    // On Windows, check for named pipe existence differently
    #[cfg(windows)]
    {
        let pipe_path = daemon.socket_path();
        assert!(
            pipe_path.contains("treelint-daemon"),
            "Named pipe should contain 'treelint-daemon'"
        );
    }
}

/// Test: Daemon reports correct socket path
/// Requirement: DAEMON-001 - Start daemon process and enter event loop
#[test]
fn test_daemon_reports_correct_socket_path() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Act
    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    let socket_path = daemon.socket_path();

    // Assert
    #[cfg(unix)]
    {
        let expected = project_root
            .join(".treelint")
            .join("daemon.sock")
            .to_string_lossy()
            .to_string();
        assert_eq!(
            socket_path, expected,
            "Socket path should be .treelint/daemon.sock"
        );
    }

    #[cfg(windows)]
    assert!(
        socket_path.starts_with(r"\\.\pipe\"),
        "Windows pipe path should start with \\\\.\\pipe\\"
    );
}

/// Test: Daemon fails to start if another daemon is already running
/// Requirement: BR-001 - Only one daemon instance can run per working directory
#[test]
fn test_daemon_fails_if_another_daemon_running() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Start first daemon
    let daemon1 = DaemonServer::new(project_root).expect("First daemon should start");
    daemon1
        .wait_for_ready(Duration::from_secs(2))
        .expect("First daemon should reach ready state");

    // Act: Try to start second daemon
    let daemon2_result = DaemonServer::new(project_root);

    // Assert
    assert!(
        daemon2_result.is_err(),
        "Second daemon should fail to start when one is already running"
    );
}

/// Test: Daemon startup is idempotent for stale socket files
/// Requirement: BR-003 - Stale socket files must be cleaned up on startup
#[test]
fn test_daemon_cleans_stale_socket_on_startup() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create a stale socket file
    let treelint_dir = project_root.join(".treelint");
    std::fs::create_dir_all(&treelint_dir).expect("Failed to create .treelint directory");

    #[cfg(unix)]
    {
        let socket_path = treelint_dir.join("daemon.sock");
        // Create a regular file to simulate stale socket
        std::fs::write(&socket_path, "stale").expect("Failed to create stale socket file");
    }

    // Act: Start daemon (should clean up stale socket)
    let daemon = DaemonServer::new(project_root);

    // Assert
    assert!(
        daemon.is_ok(),
        "Daemon should start successfully after cleaning stale socket"
    );
}

/// Test: Daemon accepts connections after reaching ready state
/// Requirement: DAEMON-001 - Start daemon process and enter event loop
#[test]
fn test_daemon_accepts_connections_after_ready() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Act: Try to connect
    let client_result = daemon.test_connection();

    // Assert
    assert!(
        client_result.is_ok(),
        "Should be able to connect to daemon after ready state"
    );
}

/// Test: Daemon PID is available after startup
/// Requirement: AC#5 - Status includes pid field
#[test]
fn test_daemon_pid_available_after_startup() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let daemon = DaemonServer::new(temp_dir.path()).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Act
    let pid = daemon.pid();

    // Assert
    assert!(pid > 0, "Daemon PID should be a positive integer");
}
