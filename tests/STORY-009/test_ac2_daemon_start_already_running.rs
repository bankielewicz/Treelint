//! AC#2: Daemon Start When Already Running Tests
//!
//! Given: Daemon is already running
//! When: User runs `treelint daemon start`
//! Then:
//!   - Command detects existing daemon
//!   - Prints message: "Daemon already running (PID: XXXX)"
//!   - Does not start second daemon
//!   - Exit code is 0 (not an error)
//!
//! Source files tested:
//!   - src/cli/commands/daemon.rs (Already running check)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - CLI-003: Detect already-running daemon before start
//!   - BR-001: Daemon start is idempotent

use assert_cmd::Command;
use predicates::prelude::*;
use std::time::Duration;
use tempfile::TempDir;

/// Test: Second `treelint daemon start` shows "already running" message
/// Requirement: CLI-003 - Detect already-running daemon before start
#[test]
fn test_daemon_start_already_running_shows_message() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Start first daemon
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    // Give daemon time to initialize
    std::thread::sleep(Duration::from_millis(200));

    // Act: Try to start second daemon
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert();

    // Assert: Output indicates already running
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(
        combined.to_lowercase().contains("already running")
            || combined.to_lowercase().contains("already started"),
        "Second start should show 'already running' message.\n\nOutput:\n{}",
        combined
    );

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: Second `treelint daemon start` includes PID in message
/// Requirement: CLI-003 - Detect already-running daemon before start
#[test]
fn test_daemon_start_already_running_includes_pid() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Start first daemon
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    std::thread::sleep(Duration::from_millis(200));

    // Act: Try to start second daemon
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert();

    // Assert: Output includes PID of running daemon
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    // Look for PID pattern like "PID: 1234" or "(PID: 1234)"
    let pid_pattern = regex::Regex::new(r"(?i)pid[:\s]+\d+").unwrap();
    assert!(
        pid_pattern.is_match(&combined),
        "Already running message should include PID.\n\nOutput:\n{}",
        combined
    );

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: Second `treelint daemon start` does not create second daemon
/// Requirement: BR-001 - Only one daemon instance can run per working directory
#[test]
fn test_daemon_start_already_running_no_second_daemon() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Start first daemon and capture its PID
    let first_output = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    let first_stdout = String::from_utf8_lossy(&first_output.get_output().stdout);
    let pid_pattern = regex::Regex::new(r"(?i)pid[:\s]+(\d+)").unwrap();
    let first_pid = pid_pattern
        .captures(&first_stdout)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());

    std::thread::sleep(Duration::from_millis(200));

    // Act: Try to start second daemon
    let _ = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert();

    // Get status to verify only one daemon is running
    let status_output = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "status"])
        .assert();

    let status_stdout = String::from_utf8_lossy(&status_output.get_output().stdout);

    // Assert: The PID in status should match the first daemon's PID
    if let Some(first_pid_str) = first_pid {
        assert!(
            status_stdout.contains(&first_pid_str),
            "Status should show original daemon PID.\n\nFirst PID: {}\nStatus output:\n{}",
            first_pid_str,
            status_stdout
        );
    }

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: Second `treelint daemon start` exit code is 0 (idempotent)
/// Requirement: BR-001 - Daemon start is idempotent (no error if already running)
#[test]
fn test_daemon_start_already_running_exit_code_zero() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Start first daemon
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    std::thread::sleep(Duration::from_millis(200));

    // Act & Assert: Second start should also exit with 0
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
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

/// Test: Multiple consecutive starts are all idempotent
/// Requirement: BR-001 - Daemon start is idempotent
#[test]
fn test_daemon_start_multiple_consecutive_starts_idempotent() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Start daemon multiple times
    for i in 0..3 {
        let output = Command::cargo_bin("treelint")
            .expect("treelint binary not found")
            .current_dir(temp_dir.path())
            .args(["daemon", "start"])
            .assert()
            .code(0);

        if i > 0 {
            let stdout = String::from_utf8_lossy(&output.get_output().stdout);
            let stderr = String::from_utf8_lossy(&output.get_output().stderr);
            let combined = format!("{}{}", stdout, stderr);

            assert!(
                combined.to_lowercase().contains("already running")
                    || combined.to_lowercase().contains("already started"),
                "Start #{} should show already running message.\n\nOutput:\n{}",
                i + 1,
                combined
            );
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}
