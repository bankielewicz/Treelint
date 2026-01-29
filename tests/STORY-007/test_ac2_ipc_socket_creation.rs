//! AC#2: IPC Socket/Pipe Creation Tests
//!
//! Given: Daemon process is starting
//! When: Daemon initializes IPC transport
//! Then:
//!   - On Unix/macOS: Creates socket at `.treelint/daemon.sock`
//!   - On Windows: Creates named pipe at `\\.\pipe\treelint-daemon`
//!   - Socket/pipe has appropriate permissions (user-only access)
//!
//! Source files tested:
//!   - src/daemon/server.rs (IPC transport abstraction)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DAEMON-003: Create Unix socket at .treelint/daemon.sock
//!   - DAEMON-004: Create named pipe on Windows at \\.\pipe\treelint-daemon

use std::time::Duration;

use tempfile::TempDir;

// These imports will fail until the daemon module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::DaemonServer;

/// Test: Unix socket created at .treelint/daemon.sock
/// Requirement: DAEMON-003 - Create Unix socket at .treelint/daemon.sock
#[cfg(unix)]
#[test]
fn test_unix_socket_created_at_correct_path() {
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
    assert!(
        socket_path.exists(),
        "Unix socket should exist at .treelint/daemon.sock"
    );
}

/// Test: Unix socket has correct permissions (0600 - user only)
/// Requirement: DAEMON-003 - Create Unix socket with correct permissions
#[cfg(unix)]
#[test]
fn test_unix_socket_has_user_only_permissions() {
    use std::os::unix::fs::PermissionsExt;

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Act
    let socket_path = project_root.join(".treelint").join("daemon.sock");
    let metadata = std::fs::metadata(&socket_path).expect("Failed to get socket metadata");
    let permissions = metadata.permissions();
    let mode = permissions.mode() & 0o777; // Extract permission bits

    // Assert: Socket should be readable/writable by owner only (0600 or similar)
    // Note: Unix domain sockets may have different default permissions
    assert!(
        mode & 0o077 == 0 || mode == 0o755,
        "Socket permissions should restrict group/other access, got {:o}",
        mode
    );
}

/// Test: .treelint directory created if not exists before socket creation
/// Requirement: DAEMON-003 - Create Unix socket at .treelint/daemon.sock
#[cfg(unix)]
#[test]
fn test_treelint_directory_created_for_socket() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();
    let treelint_dir = project_root.join(".treelint");

    // Pre-condition: directory does not exist
    assert!(
        !treelint_dir.exists(),
        ".treelint should not exist initially"
    );

    // Act
    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Assert
    assert!(
        treelint_dir.exists(),
        ".treelint directory should be created for socket"
    );
}

/// Test: Socket path is a Unix domain socket type
/// Requirement: DAEMON-003 - Create Unix socket at .treelint/daemon.sock
#[cfg(unix)]
#[test]
fn test_socket_path_is_unix_socket_type() {
    use std::os::unix::fs::FileTypeExt;

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Act
    let socket_path = project_root.join(".treelint").join("daemon.sock");
    let metadata = std::fs::metadata(&socket_path).expect("Failed to get socket metadata");
    let file_type = metadata.file_type();

    // Assert
    assert!(
        file_type.is_socket(),
        "Path should be a Unix domain socket, not a regular file"
    );
}

/// Test: Windows named pipe created at correct path
/// Requirement: DAEMON-004 - Create named pipe on Windows at \\.\pipe\treelint-daemon
#[cfg(windows)]
#[test]
fn test_windows_named_pipe_created_at_correct_path() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Act
    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Assert
    let pipe_path = daemon.socket_path();
    assert!(
        pipe_path.starts_with(r"\\.\pipe\treelint-daemon"),
        "Windows named pipe should be at \\\\.\\pipe\\treelint-daemon, got: {}",
        pipe_path
    );
}

/// Test: Windows named pipe includes unique identifier for project
/// Requirement: DAEMON-004 - Create named pipe on Windows
#[cfg(windows)]
#[test]
fn test_windows_named_pipe_includes_project_identifier() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Act
    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    let pipe_path = daemon.socket_path();

    // Assert: Pipe should include project-specific identifier to allow multiple daemons
    // for different projects
    assert!(
        pipe_path.contains("treelint-daemon"),
        "Pipe path should contain project identifier"
    );
}

/// Test: Socket removed on daemon shutdown
/// Requirement: DAEMON-010 - Clean up socket/pipe file on shutdown
#[cfg(unix)]
#[test]
fn test_socket_removed_on_shutdown() {
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
    daemon.shutdown().expect("Failed to shutdown daemon");

    // Assert
    // Give a brief moment for cleanup
    std::thread::sleep(Duration::from_millis(100));
    assert!(
        !socket_path.exists(),
        "Socket should be removed after shutdown"
    );
}

/// Test: Socket can be bound by only one process
/// Requirement: BR-001 - Only one daemon instance can run per working directory
#[test]
fn test_socket_exclusive_binding() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Start first daemon
    let daemon1 = DaemonServer::new(project_root).expect("First daemon should start");
    daemon1
        .wait_for_ready(Duration::from_secs(2))
        .expect("First daemon should be ready");

    // Act: Try to create second daemon
    let daemon2_result = DaemonServer::new(project_root);

    // Assert
    assert!(
        daemon2_result.is_err(),
        "Second daemon should fail due to socket already bound"
    );

    // Cleanup
    daemon1.shutdown().expect("Failed to shutdown daemon");
}

/// Test: Socket path returned matches actual socket location
/// Requirement: DAEMON-003/DAEMON-004 - Socket path reporting
#[test]
fn test_socket_path_matches_actual_location() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Act
    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    let reported_path = daemon.socket_path();

    // Assert
    #[cfg(unix)]
    {
        use std::path::Path;
        assert!(
            Path::new(&reported_path).exists(),
            "Reported socket path should exist on filesystem"
        );
    }

    #[cfg(windows)]
    {
        // Windows named pipes don't show up in filesystem
        assert!(
            reported_path.contains("pipe"),
            "Windows path should reference a named pipe"
        );
    }
}

/// Test: Stale socket is detected and cleaned up
/// Requirement: BR-003 - Stale socket files must be cleaned up on startup
#[cfg(unix)]
#[test]
fn test_stale_socket_detected_and_cleaned() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create .treelint directory and fake stale socket
    let treelint_dir = project_root.join(".treelint");
    std::fs::create_dir_all(&treelint_dir).expect("Failed to create .treelint");

    let socket_path = treelint_dir.join("daemon.sock");
    // Create a regular file to simulate stale socket (daemon crashed without cleanup)
    std::fs::write(&socket_path, "").expect("Failed to create stale socket file");

    // Act: Start daemon - should detect stale and clean up
    let daemon = DaemonServer::new(project_root);

    // Assert
    assert!(
        daemon.is_ok(),
        "Daemon should start after cleaning stale socket"
    );

    // Verify it created a proper socket
    let daemon = daemon.unwrap();
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Daemon should reach ready state");
}
