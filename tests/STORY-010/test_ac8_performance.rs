//! AC#8: Large Repository Performance Tests
//!
//! Given: Repository has 100,000+ files
//! When: User runs `treelint map`
//! Then:
//!   - Map generation completes within 10 seconds
//!   - Memory usage bounded (streaming output if needed)
//!   - Progress indicator shown if generation takes > 2 seconds
//!
//! Source files tested:
//!   - src/cli/commands/map.rs (Performance optimization)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - MAP-011: Complete within 10 seconds for large repos
//!   - NFR-001: Map generation must be fast for large repos

use assert_cmd::Command;
use std::fs;
use std::time::Instant;
use tempfile::TempDir;

/// Test: Map command completes within 10 seconds for indexed repository
/// Requirement: MAP-011 - < 10 seconds for 100K file repository
///
/// Note: This test uses a smaller set (1000 files) for CI performance.
/// A full 100K file benchmark would be run separately in perf testing.
#[test]
fn test_map_completes_within_time_limit() {
    // Arrange: Create 1000 small files (scaled-down test for CI)
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let file_count = 1000;
    for i in 0..file_count {
        let dir = temp_dir.path().join(format!("dir_{}", i / 100));
        fs::create_dir_all(&dir).ok();
        fs::write(
            dir.join(format!("file_{}.py", i)),
            format!("def func_{}(): pass\n", i),
        )
        .expect("Failed to write test file");
    }

    // Build index first
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success();

    // Act: Time the map command
    let start = Instant::now();
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .timeout(std::time::Duration::from_secs(10))
        .assert()
        .success();
    let elapsed = start.elapsed();

    // Assert: Should complete within 10 seconds
    assert!(
        elapsed.as_secs() < 10,
        "Map command should complete within 10 seconds, took {:?}",
        elapsed
    );
}

/// Test: Map with --ranked also completes within time limit
/// Requirement: MAP-011 - Performance with relevance calculation
#[test]
fn test_map_ranked_completes_within_time_limit() {
    // Arrange: Create smaller test set
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let file_count = 500;
    for i in 0..file_count {
        fs::write(
            temp_dir.path().join(format!("file_{}.py", i)),
            format!("def func_{}(): pass\n", i),
        )
        .expect("Failed to write test file");
    }

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success();

    // Act: Time the ranked map command
    let start = Instant::now();
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--ranked"])
        .timeout(std::time::Duration::from_secs(10))
        .assert()
        .success();
    let elapsed = start.elapsed();

    // Assert
    assert!(
        elapsed.as_secs() < 10,
        "Map --ranked should complete within 10 seconds, took {:?}",
        elapsed
    );
}

/// Test: Map with type filter is fast
/// Requirement: MAP-011 - Filtering should not slow down significantly
#[test]
fn test_map_type_filter_performance() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let file_count = 500;
    for i in 0..file_count {
        fs::write(
            temp_dir.path().join(format!("file_{}.py", i)),
            format!("def func_{}(): pass\nclass Class_{}(): pass\n", i, i),
        )
        .expect("Failed to write test file");
    }

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success();

    // Act: Time filtered map
    let start = Instant::now();
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--type", "function"])
        .timeout(std::time::Duration::from_secs(10))
        .assert()
        .success();
    let elapsed = start.elapsed();

    // Assert
    assert!(
        elapsed.as_secs() < 10,
        "Map with --type filter should complete within 10 seconds, took {:?}",
        elapsed
    );
}

/// Test: Text output is also performant
/// Requirement: MAP-011 - Performance for text format
#[test]
fn test_map_text_format_performance() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let file_count = 500;
    for i in 0..file_count {
        fs::write(
            temp_dir.path().join(format!("file_{}.py", i)),
            format!("def func_{}(): pass\n", i),
        )
        .expect("Failed to write test file");
    }

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success();

    // Act
    let start = Instant::now();
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "text"])
        .timeout(std::time::Duration::from_secs(10))
        .assert()
        .success();
    let elapsed = start.elapsed();

    // Assert
    assert!(
        elapsed.as_secs() < 10,
        "Map text format should complete within 10 seconds, took {:?}",
        elapsed
    );
}

/// Test: Deep directory structure does not slow down map
/// Requirement: MAP-011 - Performance with nested directories
#[test]
fn test_map_deep_directory_performance() {
    // Arrange: Create deeply nested structure
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create 10 levels deep, 10 files each
    for depth in 0..10 {
        let mut path = temp_dir.path().to_path_buf();
        for d in 0..=depth {
            path = path.join(format!("level_{}", d));
        }
        fs::create_dir_all(&path).ok();

        for i in 0..10 {
            fs::write(
                path.join(format!("file_{}.py", i)),
                format!("def func_{}_{}(): pass\n", depth, i),
            )
            .ok();
        }
    }

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .timeout(std::time::Duration::from_secs(30))
        .assert()
        .success();

    // Act
    let start = Instant::now();
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .timeout(std::time::Duration::from_secs(10))
        .assert()
        .success();
    let elapsed = start.elapsed();

    // Assert
    assert!(
        elapsed.as_secs() < 10,
        "Map on deep directories should complete within 10 seconds, took {:?}",
        elapsed
    );
}

/// Test: Map from pre-built index is fast (no re-indexing)
/// Requirement: MAP-011 - Read from index, don't rebuild
#[test]
fn test_map_uses_existing_index() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    for i in 0..100 {
        fs::write(
            temp_dir.path().join(format!("file_{}.py", i)),
            format!("def func_{}(): pass\n", i),
        )
        .expect("Failed to write test file");
    }

    // Build index once
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act: Run map twice, second should be at least as fast
    let start1 = Instant::now();
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .assert()
        .success();
    let elapsed1 = start1.elapsed();

    let start2 = Instant::now();
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .assert()
        .success();
    let elapsed2 = start2.elapsed();

    // Assert: Both should be fast (using index, not rebuilding)
    assert!(
        elapsed1.as_millis() < 5000 && elapsed2.as_millis() < 5000,
        "Map should be fast when using existing index. First: {:?}, Second: {:?}",
        elapsed1,
        elapsed2
    );
}

/// Test: Map handles large output without memory issues
/// Requirement: NFR - Memory usage bounded
#[test]
fn test_map_handles_large_output() {
    // Arrange: Create files with many symbols each
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    for i in 0..50 {
        let mut content = String::new();
        for j in 0..20 {
            content.push_str(&format!("def func_{}_{}(): pass\n", i, j));
            content.push_str(&format!("class Class_{}_{}(): pass\n", i, j));
        }
        fs::write(temp_dir.path().join(format!("file_{}.py", i)), content)
            .expect("Failed to write test file");
    }

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success();

    // Act: Map should complete without OOM
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .timeout(std::time::Duration::from_secs(30))
        .assert()
        .success();
}

/// Test: Progress indicator shown for slow operations (placeholder)
/// Requirement: MAP-011 - Progress indicator shown if generation takes > 2 seconds
///
/// Note: This is a placeholder test. Full progress indicator testing
/// would require TTY simulation which is complex in integration tests.
#[test]
#[ignore = "Progress indicator requires TTY simulation - manual testing recommended"]
fn test_map_shows_progress_for_slow_operation() {
    // This test is marked as ignored because:
    // 1. Progress indicators typically only show on TTY
    // 2. CI runs without TTY
    // 3. Manual testing is more appropriate for UI/progress features

    // The actual verification would be:
    // - Run map on large repo with TTY attached
    // - Verify spinner/progress bar appears after 2 seconds
    // - Verify progress updates as work completes
}
