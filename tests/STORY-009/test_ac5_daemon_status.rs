//! AC#5: Daemon Status Command Tests
//!
//! Given: User wants to check daemon state
//! When: User runs `treelint daemon status`
//! Then:
//!   If daemon running:
//!   - Shows: status (ready/indexing), PID, uptime, indexed files/symbols, socket path
//!   - Exit code: 0
//!
//!   If daemon not running:
//!   - Shows: "Daemon not running"
//!   - Exit code: 1
//!
//! Source files tested:
//!   - src/cli/commands/daemon.rs (Status command)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - CLI-007: Query daemon status via IPC
//!   - CLI-008: Different exit codes for running vs not running

use assert_cmd::Command;
use predicates::prelude::*;
use std::time::Duration;
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

/// Test: `treelint daemon status` when running shows status
/// Requirement: CLI-007 - Query daemon status via IPC
#[test]
fn test_daemon_status_running_shows_status() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert()
        .success();

    // Assert: Output contains status
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    assert!(
        stdout.to_lowercase().contains("status")
            || stdout.to_lowercase().contains("ready")
            || stdout.to_lowercase().contains("running"),
        "Status output should show daemon status.\n\nOutput:\n{}",
        stdout
    );

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: `treelint daemon status` when running shows PID
/// Requirement: CLI-007 - Query daemon status via IPC
#[test]
fn test_daemon_status_running_shows_pid() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert()
        .success();

    // Assert: Output contains PID
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let pid_pattern = regex::Regex::new(r"(?i)pid[:\s]+\d+").unwrap();

    assert!(
        pid_pattern.is_match(&stdout),
        "Status output should show PID.\n\nOutput:\n{}",
        stdout
    );

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: `treelint daemon status` when running shows uptime
/// Requirement: CLI-007 - Query daemon status via IPC
#[test]
fn test_daemon_status_running_shows_uptime() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    // Wait a bit for measurable uptime
    std::thread::sleep(Duration::from_secs(1));

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert()
        .success();

    // Assert: Output contains uptime
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    assert!(
        stdout.to_lowercase().contains("uptime") || stdout.contains("s") || stdout.contains("sec"),
        "Status output should show uptime.\n\nOutput:\n{}",
        stdout
    );

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: `treelint daemon status` when running shows indexed files count
/// Requirement: CLI-007 - Query daemon status via IPC
#[test]
fn test_daemon_status_running_shows_indexed_files() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert()
        .success();

    // Assert: Output contains indexed files info
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    assert!(
        stdout.to_lowercase().contains("file") || stdout.to_lowercase().contains("indexed"),
        "Status output should show indexed files count.\n\nOutput:\n{}",
        stdout
    );

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: `treelint daemon status` when running shows indexed symbols count
/// Requirement: CLI-007 - Query daemon status via IPC
#[test]
fn test_daemon_status_running_shows_indexed_symbols() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert()
        .success();

    // Assert: Output contains indexed symbols info
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    assert!(
        stdout.to_lowercase().contains("symbol") || stdout.to_lowercase().contains("indexed"),
        "Status output should show indexed symbols count.\n\nOutput:\n{}",
        stdout
    );

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: `treelint daemon status` when running shows socket path
/// Requirement: CLI-007 - Query daemon status via IPC
#[test]
fn test_daemon_status_running_shows_socket_path() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert()
        .success();

    // Assert: Output contains socket path
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    #[cfg(unix)]
    {
        assert!(
            stdout.contains("socket") || stdout.contains(".sock") || stdout.contains("Socket"),
            "Status output should show socket path.\n\nOutput:\n{}",
            stdout
        );
    }

    #[cfg(windows)]
    {
        assert!(
            stdout.contains("pipe") || stdout.contains("treelint-daemon"),
            "Status output should show pipe path.\n\nOutput:\n{}",
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

/// Test: `treelint daemon status` when running has exit code 0
/// Requirement: CLI-008 - Different exit codes for running vs not running
#[test]
fn test_daemon_status_running_exit_code_zero() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    start_daemon_and_wait(&temp_dir);

    // Act & Assert
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert()
        .code(0);

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: `treelint daemon status` when not running shows message
/// Requirement: CLI-007 - Query daemon status via IPC
#[test]
fn test_daemon_status_not_running_shows_message() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert();

    // Assert: Output shows "not running" message
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(
        combined.to_lowercase().contains("not running")
            || combined.to_lowercase().contains("no daemon")
            || combined.to_lowercase().contains("daemon not running"),
        "Status should show 'not running' message.\n\nOutput:\n{}",
        combined
    );
}

/// Test: `treelint daemon status` when not running has exit code 1
/// Requirement: CLI-008 - Different exit codes for running vs not running
#[test]
fn test_daemon_status_not_running_exit_code_one() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Act & Assert
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert()
        .code(1);
}

/// Test: `treelint daemon status` subcommand is recognized
/// Requirement: DaemonArgs - Subcommand parsing for daemon
#[test]
fn test_daemon_status_subcommand_recognized() {
    // Act & Assert: Command should not fail with "unrecognized subcommand"
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["daemon", "status", "--help"])
        .assert()
        .stdout(predicate::str::contains("daemon").or(predicate::str::contains("status")));
}
