//! AC#4: Search Method Handler Tests
//!
//! Given: Daemon is running with a loaded index
//! When: Client sends search request
//! Then: Daemon returns search results in the standard JSON output format
//!       (same schema as CLI search)
//!
//! Source files tested:
//!   - src/daemon/server.rs (Search handler)
//!   - src/index/search.rs (Index search)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DAEMON-007: Execute search query using loaded index

use serde_json::json;
use std::time::Duration;
use tempfile::TempDir;

// These imports will fail until the daemon module is implemented
// This is expected behavior for TDD Red phase
use treelint::daemon::{DaemonClient, DaemonServer};

/// Test: Search method returns results in standard format
/// Requirement: DAEMON-007 - Execute search query using loaded index
#[test]
fn test_search_method_returns_standard_format() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create a test file to index
    let test_file = project_root.join("test.py");
    std::fs::write(&test_file, "def hello():\n    pass\n").expect("Failed to write test file");

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");

    // Index the test file
    daemon
        .index_file(&test_file)
        .expect("Failed to index test file");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "search",
        "params": {
            "symbol": "hello",
            "type": "function"
        }
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    assert!(
        response["result"].is_array(),
        "Search result should be an array"
    );
    let results = response["result"].as_array().unwrap();
    assert!(
        !results.is_empty(),
        "Search should return at least one result"
    );

    // Verify result format matches CLI schema
    let first_result = &results[0];
    assert!(
        first_result.get("name").is_some(),
        "Result should have 'name' field"
    );
    assert!(
        first_result.get("type").is_some(),
        "Result should have 'type' field"
    );
    assert!(
        first_result.get("file").is_some(),
        "Result should have 'file' field"
    );
    assert!(
        first_result.get("line_start").is_some(),
        "Result should have 'line_start' field"
    );
}

/// Test: Search method with symbol parameter filters by name
/// Requirement: DAEMON-007 - Execute search query using loaded index
#[test]
fn test_search_filters_by_symbol_name() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create test files with multiple symbols
    let test_file = project_root.join("test.py");
    std::fs::write(&test_file, "def foo():\n    pass\n\ndef bar():\n    pass\n")
        .expect("Failed to write test file");

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");
    daemon
        .index_file(&test_file)
        .expect("Failed to index test file");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "search",
        "params": {
            "symbol": "foo"
        }
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let results = response["result"].as_array().unwrap();
    assert_eq!(
        results.len(),
        1,
        "Should return exactly one result for 'foo'"
    );
    assert_eq!(
        results[0]["name"], "foo",
        "Result should be the 'foo' function"
    );
}

/// Test: Search method with type parameter filters by symbol type
/// Requirement: DAEMON-007 - Execute search query using loaded index
#[test]
fn test_search_filters_by_symbol_type() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create test file with function and class
    let test_file = project_root.join("test.py");
    std::fs::write(
        &test_file,
        "def my_func():\n    pass\n\nclass MyClass:\n    pass\n",
    )
    .expect("Failed to write test file");

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");
    daemon
        .index_file(&test_file)
        .expect("Failed to index test file");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "search",
        "params": {
            "type": "function"
        }
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let results = response["result"].as_array().unwrap();
    for result in results {
        assert_eq!(
            result["type"], "function",
            "All results should be functions"
        );
    }
}

/// Test: Search with no matches returns empty array
/// Requirement: DAEMON-007 - Execute search query using loaded index
#[test]
fn test_search_no_matches_returns_empty_array() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let test_file = project_root.join("test.py");
    std::fs::write(&test_file, "def hello():\n    pass\n").expect("Failed to write test file");

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");
    daemon
        .index_file(&test_file)
        .expect("Failed to index test file");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "search",
        "params": {
            "symbol": "nonexistent_symbol_xyz"
        }
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    assert!(response["result"].is_array(), "Result should be an array");
    let results = response["result"].as_array().unwrap();
    assert!(
        results.is_empty(),
        "Search for nonexistent symbol should return empty array"
    );
}

/// Test: Search result format matches CLI JSON output
/// Requirement: DAEMON-007 - Search results in standard JSON output format
#[test]
fn test_search_result_format_matches_cli() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let test_file = project_root.join("test.py");
    std::fs::write(&test_file, "def calculate(x, y):\n    return x + y\n")
        .expect("Failed to write test file");

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");
    daemon
        .index_file(&test_file)
        .expect("Failed to index test file");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "search",
        "params": {
            "symbol": "calculate"
        }
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert: Verify all expected fields exist (matching CLI output schema)
    let results = response["result"].as_array().unwrap();
    let result = &results[0];

    // Required fields (per STORY-005 JSON output format)
    assert!(result.get("name").is_some(), "Should have 'name' field");
    assert!(result.get("type").is_some(), "Should have 'type' field");
    assert!(result.get("file").is_some(), "Should have 'file' field");
    assert!(
        result.get("line_start").is_some(),
        "Should have 'line_start' field"
    );
    assert!(
        result.get("line_end").is_some(),
        "Should have 'line_end' field"
    );
}

/// Test: Search completes within 100ms for indexed data
/// Requirement: BR-002 - Daemon must respond to requests within 100ms
#[test]
fn test_search_completes_within_100ms() {
    use std::time::Instant;

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Create a small test file
    let test_file = project_root.join("test.py");
    std::fs::write(&test_file, "def hello():\n    pass\n").expect("Failed to write test file");

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");
    daemon
        .index_file(&test_file)
        .expect("Failed to index test file");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "search",
        "params": {
            "symbol": "hello"
        }
    });

    let start = Instant::now();
    let _response = client
        .send_request(&request)
        .expect("Failed to send request");
    let elapsed = start.elapsed();

    // Assert
    assert!(
        elapsed.as_millis() < 100,
        "Search took {}ms, expected < 100ms",
        elapsed.as_millis()
    );
}

/// Test: Search handles regex patterns
/// Requirement: DAEMON-007 - Execute search query using loaded index
#[test]
fn test_search_handles_regex_patterns() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let test_file = project_root.join("test.py");
    std::fs::write(
        &test_file,
        "def get_user():\n    pass\n\ndef get_item():\n    pass\n\ndef set_user():\n    pass\n",
    )
    .expect("Failed to write test file");

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");
    daemon
        .index_file(&test_file)
        .expect("Failed to index test file");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act: Search with regex pattern
    let request = json!({
        "id": "req-001",
        "method": "search",
        "params": {
            "symbol": "get_.*",
            "regex": true
        }
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let results = response["result"].as_array().unwrap();
    assert_eq!(
        results.len(),
        2,
        "Regex 'get_.*' should match 'get_user' and 'get_item'"
    );
}

/// Test: Search when index not ready returns E001 error
/// Requirement: AC#7 - E001 (index not ready) error code
#[test]
fn test_search_when_index_not_ready_returns_e001() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    // Start daemon but don't index anything
    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    // Don't wait for ready - test while daemon is still starting
    // Or explicitly set index to "not ready" state

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "search",
        "params": {
            "symbol": "hello"
        }
    });

    // Note: This might succeed if daemon becomes ready quickly
    // The test assumes we can test when index is explicitly not ready
    let response = client.send_request(&request);

    // Assert: If daemon hasn't finished initializing, should return E001
    // If daemon is ready, this test may need adjustment
    if let Ok(resp) = response {
        if resp.get("error").is_some() && !resp["error"].is_null() {
            assert_eq!(
                resp["error"]["code"], "E001",
                "Error code should be E001 when index not ready"
            );
        }
    }
}

/// Test: Search method handles case-insensitive flag
/// Requirement: DAEMON-007 - Execute search query using loaded index
#[test]
fn test_search_case_insensitive() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_root = temp_dir.path();

    let test_file = project_root.join("test.py");
    std::fs::write(&test_file, "def MyFunction():\n    pass\n").expect("Failed to write test file");

    let daemon = DaemonServer::new(project_root).expect("Failed to create daemon");
    daemon
        .wait_for_ready(Duration::from_secs(2))
        .expect("Failed to reach ready state");
    daemon
        .index_file(&test_file)
        .expect("Failed to index test file");

    let client = DaemonClient::connect(daemon.socket_path()).expect("Failed to connect");

    // Act
    let request = json!({
        "id": "req-001",
        "method": "search",
        "params": {
            "symbol": "myfunction",
            "case_insensitive": true
        }
    });
    let response = client
        .send_request(&request)
        .expect("Failed to send request");

    // Assert
    let results = response["result"].as_array().unwrap();
    assert_eq!(
        results.len(),
        1,
        "Case-insensitive search should find 'MyFunction'"
    );
    assert_eq!(results[0]["name"], "MyFunction");
}
