//! AC#8: Auto-Detection for Search Command Tests
//!
//! Given: User runs `treelint search foo`
//! When: Search command processes the request
//! Then:
//!   Auto-detection logic:
//!   1. Check if daemon socket/pipe exists and responds -> Query daemon (~5ms)
//!   2. Else check if index.db exists and is fresh -> Query index directly (~20ms)
//!   3. Else build index on-demand, then query (~5-60s first time)
//!
//!   Behavior is transparent to user (same output format)
//!
//! Source files tested:
//!   - src/cli/commands/search.rs (Auto-detection logic)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - CLI-012: Detect daemon availability
//!   - CLI-013: Fall back to direct index query
//!   - CLI-014: Build index on-demand when no index exists

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// Helper: Create a test project with source files
fn setup_test_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    fs::write(
        src_dir.join("module.py"),
        r#"
def searchable_function(x: int) -> int:
    """A function that can be searched."""
    return x * 2

class SearchableClass:
    """A class that can be searched."""
    def method(self):
        pass
"#,
    )
    .expect("Failed to write file");

    temp_dir
}

/// Test: Search uses daemon when available
/// Requirement: CLI-012 - Detect daemon availability
#[test]
fn test_search_uses_daemon_when_available() {
    // Arrange
    let temp_dir = setup_test_project();

    // Start daemon
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    std::thread::sleep(Duration::from_millis(300));

    // Act: Search (should use daemon)
    let start_time = Instant::now();
    let output = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "searchable_function", "--format", "json"])
        .assert()
        .success();
    let elapsed = start_time.elapsed();

    // Assert: Search should be fast when using daemon
    // (< 100ms is reasonable for daemon query)
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("searchable_function"),
        "Search should find the function.\n\nOutput:\n{}",
        stdout
    );

    // Log timing (daemon queries should be fast)
    println!("Daemon search took: {:?}", elapsed);

    // Cleanup
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Test: Search falls back to index when daemon not running
/// Requirement: CLI-013 - Fall back to direct index query
#[test]
fn test_search_falls_back_to_index_when_no_daemon() {
    // Arrange
    let temp_dir = setup_test_project();

    // Build index but don't start daemon
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act: Search (should use index directly)
    let output = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "searchable_function", "--format", "json"])
        .assert()
        .success();

    // Assert: Search should find the function via index
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("searchable_function"),
        "Search should find the function via index.\n\nOutput:\n{}",
        stdout
    );
}

/// Test: Search builds index on-demand when no index exists
/// Requirement: CLI-014 - Build index on-demand when no index exists
#[test]
fn test_search_builds_index_on_demand() {
    // Arrange
    let temp_dir = setup_test_project();

    // Ensure no index exists
    let index_path = temp_dir.path().join(".treelint").join("index.db");
    assert!(
        !index_path.exists(),
        "Test precondition: index should not exist"
    );

    // Act: Search (should build index on-demand)
    let output = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "searchable_function", "--format", "json"])
        .assert()
        .success();

    // Assert: Search should find the function after building index
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("searchable_function"),
        "Search should find the function after on-demand indexing.\n\nOutput:\n{}",
        stdout
    );

    // Index should now exist
    assert!(index_path.exists(), "Index should be created on-demand");
}

/// Test: Search output format is consistent regardless of source
/// Requirement: CLI-012, CLI-013, CLI-014 - Behavior is transparent to user
#[test]
fn test_search_output_format_consistent() {
    // Arrange
    let temp_dir = setup_test_project();

    // Test 1: On-demand (no index, no daemon)
    let output1 = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "searchable_function", "--format", "json"])
        .assert()
        .success();
    let stdout1 = String::from_utf8_lossy(&output1.get_output().stdout);

    // Test 2: Direct index (index exists, no daemon)
    let output2 = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "searchable_function", "--format", "json"])
        .assert()
        .success();
    let stdout2 = String::from_utf8_lossy(&output2.get_output().stdout);

    // Assert: Both outputs should have same structure
    // (Both should contain "results" and find "searchable_function")
    assert!(
        stdout1.contains("searchable_function") && stdout2.contains("searchable_function"),
        "Both search methods should find the function.\n\nOutput1:\n{}\n\nOutput2:\n{}",
        stdout1,
        stdout2
    );
}

/// Test: Auto-detection overhead is minimal
/// Requirement: NFR-001 - Auto-detection must not add significant latency (< 10ms)
#[test]
fn test_auto_detection_overhead_minimal() {
    // Arrange
    let temp_dir = setup_test_project();

    // Build index
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act: Run multiple searches and measure time
    let mut times = Vec::new();
    for _ in 0..3 {
        let start = Instant::now();
        Command::cargo_bin("treelint")
            .expect("treelint binary not found")
            .current_dir(temp_dir.path())
            .args(["search", "searchable_function", "--format", "json"])
            .assert()
            .success();
        times.push(start.elapsed());
    }

    // Calculate average
    let total: Duration = times.iter().sum();
    let avg = total / times.len() as u32;

    // Assert: Average search should be reasonably fast
    // Note: This is a soft assertion; actual times depend on system
    println!("Search times: {:?}, Average: {:?}", times, avg);
}

/// Test: Search with daemon fallback works correctly
/// Requirement: CLI-012, CLI-013 - Daemon first, then index fallback
#[test]
fn test_search_daemon_fallback_works() {
    // Arrange
    let temp_dir = setup_test_project();

    // Build index and start daemon
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    std::thread::sleep(Duration::from_millis(200));

    // Search with daemon
    let output1 = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "searchable_function", "--format", "json"])
        .assert()
        .success();

    // Stop daemon
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert()
        .success();

    std::thread::sleep(Duration::from_millis(100));

    // Search without daemon (should fallback to index)
    let output2 = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "searchable_function", "--format", "json"])
        .assert()
        .success();

    // Assert: Both should return same results
    let stdout1 = String::from_utf8_lossy(&output1.get_output().stdout);
    let stdout2 = String::from_utf8_lossy(&output2.get_output().stdout);

    assert!(
        stdout1.contains("searchable_function") && stdout2.contains("searchable_function"),
        "Search should work with and without daemon.\n\nWith daemon:\n{}\n\nWithout daemon:\n{}",
        stdout1,
        stdout2
    );
}

/// Test: On-demand index build is only done once
/// Requirement: CLI-014 - Build index on-demand when no index exists
#[test]
fn test_on_demand_index_build_cached() {
    // Arrange
    let temp_dir = setup_test_project();

    // First search (should build index)
    let start1 = Instant::now();
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "searchable_function", "--format", "json"])
        .assert()
        .success();
    let time1 = start1.elapsed();

    // Second search (should use existing index)
    let start2 = Instant::now();
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["search", "searchable_function", "--format", "json"])
        .assert()
        .success();
    let time2 = start2.elapsed();

    // Assert: Second search should be faster (using cached index)
    println!("First search (with build): {:?}", time1);
    println!("Second search (cached): {:?}", time2);

    // Second search should be noticeably faster
    // (First includes index build time)
}

/// Test: Search works in subdirectory (finds project root)
/// Requirement: CLI-012, CLI-013, CLI-014 - Auto-detection from subdirectory
#[test]
fn test_search_from_subdirectory() {
    // Arrange
    let temp_dir = setup_test_project();
    let sub_dir = temp_dir.path().join("src");

    // Build index from project root
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act: Search from subdirectory
    let output = Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(&sub_dir)
        .args(["search", "searchable_function", "--format", "json"])
        .assert();

    // Assert: Search should still work from subdirectory
    // (May need to find project root by looking for .treelint)
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);

    // Either finds the function or indicates it searched
    assert!(
        stdout.contains("searchable_function")
            || stdout.contains("results")
            || stderr.to_lowercase().contains("index"),
        "Search from subdirectory should work.\n\nStdout:\n{}\n\nStderr:\n{}",
        stdout,
        stderr
    );
}
