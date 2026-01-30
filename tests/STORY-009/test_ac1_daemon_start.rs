//! AC#1: Daemon Start Command Tests
//!
//! Given: No daemon is currently running
//! When: User runs `treelint daemon start`
//! Then:
//!   - Daemon process spawns in background (detached from terminal)
//!   - Command returns immediately with success message
//!   - Message includes PID and socket path
//!   - Exit code is 0
//!
//! Source files tested:
//!   - src/cli/commands/daemon.rs (DaemonCommand implementation)
//!   - src/cli/args.rs (DaemonArgs, DaemonAction)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - CLI-001: Spawn daemon process in background
//!   - CLI-002: Report daemon PID and socket path on start

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test: `treelint daemon start` spawns daemon when no daemon running
/// Requirement: CLI-001 - Spawn daemon process in background
#[test]
fn test_daemon_start_spawns_background_process() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert();

    // Assert: Command completes with exit code 0
    output.success();
}

/// Test: `treelint daemon start` returns immediately
/// Requirement: CLI-001 - Start daemon process and enter event loop
#[test]
fn test_daemon_start_returns_immediately() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let start_time = std::time::Instant::now();

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    // Assert: Command returns within 500ms (per NFR)
    let elapsed = start_time.elapsed();
    assert!(
        elapsed.as_millis() < 500,
        "Daemon start should return within 500ms, took {:?}",
        elapsed
    );

    // Cleanup: Stop the daemon
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: `treelint daemon start` output includes PID
/// Requirement: CLI-002 - Report daemon PID and socket path on start
#[test]
fn test_daemon_start_output_includes_pid() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    // Assert: Output contains PID
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("PID") || stdout.to_lowercase().contains("pid"),
        "Start output should include PID.\n\nActual output:\n{}",
        stdout
    );

    // Verify PID is a number (look for pattern like "PID: 1234" or "pid: 1234")
    let pid_pattern = regex::Regex::new(r"(?i)pid[:\s]+(\d+)").unwrap();
    assert!(
        pid_pattern.is_match(&stdout),
        "Start output should include PID as a number.\n\nActual output:\n{}",
        stdout
    );

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: `treelint daemon start` output includes socket path
/// Requirement: CLI-002 - Report daemon PID and socket path on start
#[test]
fn test_daemon_start_output_includes_socket_path() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    // Assert: Output contains socket path
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    #[cfg(unix)]
    {
        assert!(
            stdout.contains(".treelint/daemon.sock")
                || stdout.contains("socket")
                || stdout.contains("Socket"),
            "Start output should include socket path.\n\nActual output:\n{}",
            stdout
        );
    }

    #[cfg(windows)]
    {
        assert!(
            stdout.contains("pipe") || stdout.contains("treelint-daemon"),
            "Start output should include named pipe path.\n\nActual output:\n{}",
            stdout
        );
    }

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: `treelint daemon start` exit code is 0
/// Requirement: CLI-001 - Spawn daemon process in background
#[test]
fn test_daemon_start_exit_code_is_zero() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Act & Assert
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .code(0);

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: `treelint daemon start` creates socket file
/// Requirement: CLI-001 - Spawn daemon process in background
#[test]
#[cfg(unix)]
fn test_daemon_start_creates_socket_file() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    // Give daemon time to create socket
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Assert: Socket file exists
    let socket_path = temp_dir.path().join(".treelint").join("daemon.sock");
    assert!(
        socket_path.exists(),
        "Daemon socket file should exist at {:?}",
        socket_path
    );

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: `treelint daemon start` subcommand is recognized
/// Requirement: DaemonArgs - Subcommand parsing for daemon
#[test]
fn test_daemon_start_subcommand_recognized() {
    // Act & Assert: Command should not fail with "unrecognized subcommand"
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["daemon", "start", "--help"])
        .assert()
        .stdout(predicate::str::contains("daemon").or(predicate::str::contains("start")));
}
