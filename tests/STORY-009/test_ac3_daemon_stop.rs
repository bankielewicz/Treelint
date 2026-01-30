//! AC#3: Daemon Stop Command Tests
//!
//! Given: Daemon is running
//! When: User runs `treelint daemon stop`
//! Then:
//!   - Command sends shutdown signal to daemon
//!   - Waits for daemon to stop (up to 5 seconds)
//!   - Prints success message
//!   - Exit code is 0
//!
//! Source files tested:
//!   - src/cli/commands/daemon.rs (Stop command)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - CLI-004: Send shutdown signal to running daemon
//!   - CLI-005: Wait for daemon shutdown with timeout

use assert_cmd::Command;
use predicates::prelude::*;
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// Helper: Start a daemon and wait for it to be ready
fn start_daemon_and_wait(temp_dir: &TempDir) {
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    std::thread::sleep(Duration::from_millis(200));
}

/// Test: `treelint daemon stop` stops a running daemon
/// Requirement: CLI-004 - Send shutdown signal to running daemon
#[test]
fn test_daemon_stop_stops_running_daemon() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    // Verify daemon is running
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert()
        .success();

    // Act: Stop the daemon
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert()
        .success();

    // Assert: Daemon is no longer running (status returns exit 1)
    std::thread::sleep(Duration::from_millis(100));
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert()
        .code(1);
}

/// Test: `treelint daemon stop` prints success message
/// Requirement: CLI-004 - Send shutdown signal to running daemon
#[test]
fn test_daemon_stop_prints_success_message() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert()
        .success();

    // Assert: Output contains success message
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(
        combined.to_lowercase().contains("stop")
            || combined.to_lowercase().contains("shutdown")
            || combined.to_lowercase().contains("terminated"),
        "Stop command should print success message.\n\nOutput:\n{}",
        combined
    );
}

/// Test: `treelint daemon stop` exit code is 0
/// Requirement: CLI-004 - Send shutdown signal to running daemon
#[test]
fn test_daemon_stop_exit_code_is_zero() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    // Act & Assert
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert()
        .code(0);
}

/// Test: `treelint daemon stop` completes within 5 seconds
/// Requirement: CLI-005 - Wait for daemon shutdown with timeout
#[test]
fn test_daemon_stop_completes_within_5_seconds() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    // Act
    let start_time = Instant::now();
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert()
        .success();
    let elapsed = start_time.elapsed();

    // Assert: Completed within 5 seconds
    assert!(
        elapsed.as_secs() < 5,
        "Stop command should complete within 5 seconds, took {:?}",
        elapsed
    );
}

/// Test: `treelint daemon stop` cleans up socket file
/// Requirement: CLI-004 - Send shutdown signal to running daemon
#[test]
#[cfg(unix)]
fn test_daemon_stop_cleans_up_socket_file() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    let socket_path = temp_dir.path().join(".treelint").join("daemon.sock");

    // Verify socket exists before stop (if daemon created it)
    // Note: Socket may not exist in early TDD phase

    // Act
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert()
        .success();

    std::thread::sleep(Duration::from_millis(200));

    // Assert: Socket file should be removed after stop
    assert!(
        !socket_path.exists(),
        "Socket file should be cleaned up after stop.\nSocket path: {:?}",
        socket_path
    );
}

/// Test: `treelint daemon stop` subcommand is recognized
/// Requirement: DaemonArgs - Subcommand parsing for daemon
#[test]
fn test_daemon_stop_subcommand_recognized() {
    // Act & Assert: Command should not fail with "unrecognized subcommand"
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["daemon", "stop", "--help"])
        .assert()
        .stdout(predicate::str::contains("daemon").or(predicate::str::contains("stop")));
}

/// Test: Daemon stop after immediate start works
/// Requirement: CLI-004 - Send shutdown signal to running daemon
#[test]
fn test_daemon_stop_after_immediate_start() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Start daemon
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    // Immediately stop (minimal delay)
    std::thread::sleep(Duration::from_millis(50));

    // Act & Assert: Stop should still succeed
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert()
        .code(0);
}
