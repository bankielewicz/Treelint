//! AC#4: Daemon Stop When Not Running Tests
//!
//! Given: No daemon is running
//! When: User runs `treelint daemon stop`
//! Then:
//!   - Command detects no daemon
//!   - Prints message: "No daemon running"
//!   - Exit code is 0 (not an error)
//!
//! Source files tested:
//!   - src/cli/commands/daemon.rs (Not running check)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - CLI-006: Handle stop when no daemon running
//!   - BR-002: Daemon stop is idempotent

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

/// Test: `treelint daemon stop` when no daemon running shows message
/// Requirement: CLI-006 - Handle stop when no daemon running
#[test]
fn test_daemon_stop_no_daemon_shows_message() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Ensure no daemon is running (fresh temp directory)

    // Act
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();

    // Assert: Output indicates no daemon running
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(
        combined.to_lowercase().contains("no daemon")
            || combined.to_lowercase().contains("not running")
            || combined.to_lowercase().contains("no running daemon"),
        "Stop should show 'no daemon running' message.\n\nOutput:\n{}",
        combined
    );
}

/// Test: `treelint daemon stop` when no daemon running has exit code 0
/// Requirement: BR-002 - Daemon stop is idempotent (no error if not running)
#[test]
fn test_daemon_stop_no_daemon_exit_code_zero() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Act & Assert
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert()
        .code(0);
}

/// Test: Multiple consecutive stops are all idempotent
/// Requirement: BR-002 - Daemon stop is idempotent
#[test]
fn test_daemon_stop_multiple_consecutive_stops_idempotent() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Stop daemon multiple times
    for i in 0..3 {
        let output = Command::cargo_bin("treelint")
            .expect("treelint binary not found")
            .current_dir(temp_dir.path())
            .args(["daemon", "stop"])
            .assert()
            .code(0);

        let stdout = String::from_utf8_lossy(&output.get_output().stdout);
        let stderr = String::from_utf8_lossy(&output.get_output().stderr);
        let combined = format!("{}{}", stdout, stderr);

        assert!(
            combined.to_lowercase().contains("no daemon")
                || combined.to_lowercase().contains("not running")
                || combined.to_lowercase().contains("no running daemon"),
            "Stop #{} should show 'no daemon running' message.\n\nOutput:\n{}",
            i + 1,
            combined
        );
    }
}

/// Test: Stop in fresh directory with no .treelint folder
/// Requirement: CLI-006 - Handle stop when no daemon running
#[test]
fn test_daemon_stop_fresh_directory_no_treelint_folder() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Ensure .treelint directory doesn't exist
    let treelint_dir = temp_dir.path().join(".treelint");
    assert!(
        !treelint_dir.exists(),
        "Test precondition: .treelint should not exist"
    );

    // Act & Assert: Stop should succeed even without .treelint directory
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert()
        .code(0);
}

/// Test: Stop after daemon has already been stopped
/// Requirement: BR-002 - Daemon stop is idempotent
#[test]
fn test_daemon_stop_after_already_stopped() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Start and then stop daemon
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    std::thread::sleep(std::time::Duration::from_millis(200));

    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert()
        .success();

    std::thread::sleep(std::time::Duration::from_millis(100));

    // Act: Try to stop again
    let output = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert()
        .code(0);

    // Assert: Shows "no daemon running" message
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(
        combined.to_lowercase().contains("no daemon")
            || combined.to_lowercase().contains("not running")
            || combined.to_lowercase().contains("no running daemon"),
        "Stop after already stopped should show 'no daemon running' message.\n\nOutput:\n{}",
        combined
    );
}
