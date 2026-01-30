//! AC#1: Daemon Search Returns Actual Results Tests
//!
//! Given: Daemon is running and index exists with symbols
//! When: Client sends search request via daemon protocol: {"method": "search", "params": {"symbol": "foo"}}
//! Then:
//!   - Daemon returns actual matching symbols from IndexStorage
//!   - Symbols array contains matches from index.db
//!   - Total count reflects actual match count
//!   - Each symbol has name, type, file, lines
//!   - Empty array only if no matches found
//!
//! Source files tested:
//!   - src/daemon/server.rs (handle_search implementation)
//!   - src/index/storage.rs (IndexStorage query)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - INT-001: Query IndexStorage using QueryFilters from request params
//!   - INT-002: Convert Symbol structs to JSON response format
//!
//! TDD Phase: RED
//!
//! These tests will FAIL because handle_index() is a stub that doesn't
//! actually index files, so IndexStorage remains empty and search
//! returns no results.
//!
//! Response format: {"id": "...", "result": [...]} where result is a direct array

use assert_cmd::Command;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Helper: Create a test project with known symbols
fn setup_test_project_with_symbols(temp_dir: &TempDir) {
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create Python file with searchable symbols
    let py_content = r#"
def foo_function():
    """A function named foo_function"""
    return "foo"

def bar_function():
    """A function named bar_function"""
    return "bar"

class FooClass:
    """A class with foo in the name"""
    def foo_method(self):
        pass
"#;
    fs::write(src_dir.join("module.py"), py_content).expect("Failed to write Python file");

    // Create Rust file with searchable symbols
    let rs_content = r#"
/// A function named foo_handler
pub fn foo_handler() -> String {
    "foo".to_string()
}

/// A function named bar_handler
fn bar_handler() -> String {
    "bar".to_string()
}

/// A struct with Foo in the name
pub struct FooConfig {
    pub value: i32,
}
"#;
    fs::write(src_dir.join("handlers.rs"), rs_content).expect("Failed to write Rust file");
}

/// Helper: Start daemon and return socket path
fn start_daemon(temp_dir: &TempDir) -> String {
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    // Give daemon time to initialize and create socket
    thread::sleep(Duration::from_millis(300));

    // Return socket path
    #[cfg(unix)]
    {
        temp_dir
            .path()
            .join(".treelint")
            .join("daemon.sock")
            .to_string_lossy()
            .to_string()
    }
    #[cfg(windows)]
    {
        format!(
            "\\\\.\\pipe\\treelint-daemon-{}",
            temp_dir
                .path()
                .to_string_lossy()
                .replace(['\\', '/', ':'], "-")
        )
    }
}

/// Helper: Stop daemon
fn stop_daemon(temp_dir: &TempDir) {
    let _ = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["daemon", "stop"])
        .assert();
}

/// Helper: Wait for socket to be available
#[cfg(unix)]
fn wait_for_socket(socket_path: &str, timeout_ms: u64) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed().as_millis() < timeout_ms as u128 {
        if std::path::Path::new(socket_path).exists() {
            return true;
        }
        thread::sleep(Duration::from_millis(50));
    }
    false
}

#[cfg(windows)]
fn wait_for_socket(_socket_path: &str, timeout_ms: u64) -> bool {
    thread::sleep(Duration::from_millis(timeout_ms));
    true
}

/// Helper: Send request to daemon and get response (Unix)
#[cfg(unix)]
fn send_daemon_request(socket_path: &str, request: &str) -> String {
    use std::os::unix::net::UnixStream;

    if !wait_for_socket(socket_path, 2000) {
        panic!(
            "Daemon socket not available at {} after 2 seconds",
            socket_path
        );
    }

    let mut stream = match UnixStream::connect(socket_path) {
        Ok(s) => s,
        Err(e) => panic!("Failed to connect to daemon at {}: {}", socket_path, e),
    };
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .expect("Failed to set timeout");

    writeln!(stream, "{}", request).expect("Failed to send request");
    stream.flush().expect("Failed to flush");

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader
        .read_line(&mut response)
        .expect("Failed to read response");

    response
}

/// Helper: Send request to daemon and get response (Windows)
#[cfg(windows)]
fn send_daemon_request(socket_path: &str, request: &str) -> String {
    use interprocess::local_socket::{GenericFilePath, Stream, ToFsName};

    let pipe_name = socket_path
        .to_fs_name::<GenericFilePath>()
        .expect("Invalid pipe name");

    let mut stream = Stream::connect(pipe_name).expect("Failed to connect to daemon");

    writeln!(stream, "{}", request).expect("Failed to send request");
    stream.flush().expect("Failed to flush");

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader
        .read_line(&mut response)
        .expect("Failed to read response");

    response
}

/// Helper: Index the project via daemon
fn index_via_daemon(socket_path: &str) {
    let request = r#"{"id": "idx-1", "method": "index", "params": {}}"#;
    let _ = send_daemon_request(socket_path, request);
    // Wait for indexing to complete
    thread::sleep(Duration::from_millis(500));
}

/// Helper: Extract symbols array from response
/// Handles both formats:
/// - Direct array: {"result": [...]}
/// - Nested: {"result": {"symbols": [...], "total": N}}
fn extract_symbols(json: &serde_json::Value, response: &str) -> Vec<serde_json::Value> {
    // Check for error first
    if let Some(error) = json.get("error") {
        if !error.is_null() {
            return Vec::new();
        }
    }

    // Try direct array format first (current daemon format)
    if let Some(arr) = json["result"].as_array() {
        return arr.clone();
    }

    // Try nested format
    if let Some(arr) = json["result"]["symbols"].as_array() {
        return arr.clone();
    }

    panic!(
        "Could not extract symbols from response.\n\
         Expected 'result' as array or 'result.symbols' as array.\n\
         Response: {}",
        response
    );
}

/// Test: Search returns actual symbols matching query
/// Requirement: INT-001 - Query IndexStorage using QueryFilters from request params
///
/// This test WILL FAIL because handle_index() is a stub that doesn't index,
/// so IndexStorage is empty and search returns no results.
#[test]
fn test_search_returns_actual_matching_symbols() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    setup_test_project_with_symbols(&temp_dir);
    let socket_path = start_daemon(&temp_dir);

    // Index the project first (via daemon - currently a stub)
    index_via_daemon(&socket_path);

    // Act: Search for "foo"
    let request = r#"{"id": "search-1", "method": "search", "params": {"symbol": "foo"}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Response contains actual symbols (not empty)
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    let symbols = extract_symbols(&json, &response);

    // FAILS: handle_index is stub, so IndexStorage is empty
    assert!(
        !symbols.is_empty(),
        "Search for 'foo' should return matching symbols.\n\
         Expected: symbols containing 'foo' (foo_function, FooClass, foo_method, foo_handler, FooConfig)\n\
         Actual: empty array\n\
         Root cause: handle_index is stub - doesn't actually index files\n\
         Response: {}",
        response
    );
}

/// Test: Search returns correct total count
/// Requirement: INT-001 - Query IndexStorage using QueryFilters from request params
///
/// This test WILL FAIL because handle_index() is a stub.
#[test]
fn test_search_returns_multiple_matching_symbols() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    setup_test_project_with_symbols(&temp_dir);
    let socket_path = start_daemon(&temp_dir);

    // Index the project first
    index_via_daemon(&socket_path);

    // Act: Search for "foo"
    let request = r#"{"id": "search-2", "method": "search", "params": {"symbol": "foo"}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Should find multiple 'foo' symbols
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    let symbols = extract_symbols(&json, &response);

    // FAILS: handle_index is stub - no symbols indexed
    assert!(
        symbols.len() >= 3,
        "Should find at least 3 'foo' symbols (foo_function, FooClass, foo_handler, FooConfig, etc.).\n\
         Expected: >= 3 symbols\n\
         Actual: {} symbols\n\
         Root cause: handle_index is stub\n\
         Response: {}",
        symbols.len(),
        response
    );
}

/// Test: Each symbol has required fields (name, type, file, lines)
/// Requirement: INT-002 - Convert Symbol structs to JSON response format
///
/// This test WILL FAIL because handle_index() is a stub.
#[test]
fn test_search_symbols_have_required_fields() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    setup_test_project_with_symbols(&temp_dir);
    let socket_path = start_daemon(&temp_dir);

    // Index the project first
    index_via_daemon(&socket_path);

    // Act: Search for "foo"
    let request = r#"{"id": "search-3", "method": "search", "params": {"symbol": "foo"}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Each symbol has name, type, file, lines
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    let symbols = extract_symbols(&json, &response);

    // FAILS: symbols array is empty due to stub handle_index
    assert!(
        !symbols.is_empty(),
        "Need at least one symbol to verify fields.\n\
         Root cause: handle_index is stub\n\
         Response: {}",
        response
    );

    for symbol in &symbols {
        assert!(
            symbol.get("name").is_some(),
            "Symbol should have 'name' field: {:?}",
            symbol
        );
        assert!(
            symbol.get("type").is_some(),
            "Symbol should have 'type' field: {:?}",
            symbol
        );
        assert!(
            symbol.get("file").is_some(),
            "Symbol should have 'file' field: {:?}",
            symbol
        );
        // Lines can be line_start/line_end or variations
        assert!(
            symbol.get("line_start").is_some()
                || symbol.get("start_line").is_some()
                || symbol.get("lines").is_some()
                || symbol.get("line").is_some(),
            "Symbol should have line information: {:?}",
            symbol
        );
    }
}

/// Test: Search with no matches returns empty array (not error)
/// Requirement: INT-001 - Query IndexStorage using QueryFilters from request params
///
/// This test should PASS - verifies edge case handling.
#[test]
fn test_search_no_matches_returns_empty_array() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    setup_test_project_with_symbols(&temp_dir);
    let socket_path = start_daemon(&temp_dir);

    // Index the project first
    index_via_daemon(&socket_path);

    // Act: Search for something that doesn't exist
    let request =
        r#"{"id": "search-4", "method": "search", "params": {"symbol": "nonexistent_xyz_123"}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Returns empty array, not error
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    assert!(
        json.get("error").is_none() || json["error"].is_null(),
        "Search with no matches should not return error.\n\
         Response: {}",
        response
    );

    let symbols = extract_symbols(&json, &response);

    assert!(
        symbols.is_empty(),
        "Search for nonexistent symbol should return empty array.\n\
         Response: {}",
        response
    );
}

/// Test: Search queries actual IndexStorage, not hardcoded data
/// Requirement: BR-001 - Search requests must return actual index results, not stub data
///
/// This test WILL FAIL because handle_index() is a stub.
#[test]
fn test_search_queries_actual_index_storage() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    setup_test_project_with_symbols(&temp_dir);
    let socket_path = start_daemon(&temp_dir);

    // Index the project
    index_via_daemon(&socket_path);

    // Act: Search for "bar" (we have bar_function and bar_handler)
    let request = r#"{"id": "search-5", "method": "search", "params": {"symbol": "bar"}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Should find bar_function and bar_handler
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    let symbols = extract_symbols(&json, &response);

    // FAILS: handle_index is stub
    assert!(
        symbols.len() >= 2,
        "Should find at least 2 'bar' symbols (bar_function, bar_handler).\n\
         Expected: >= 2 symbols\n\
         Actual: {} symbols\n\
         Root cause: handle_index is stub\n\
         Response: {}",
        symbols.len(),
        response
    );

    // Verify we got the expected symbol names
    let symbol_names: Vec<&str> = symbols.iter().filter_map(|s| s["name"].as_str()).collect();

    assert!(
        symbol_names.iter().any(|n| n.contains("bar")),
        "Results should contain symbols with 'bar' in name.\n\
         Found names: {:?}\n\
         Response: {}",
        symbol_names,
        response
    );
}

/// Test: Search handles type filter parameter
/// Requirement: INT-001 - Query IndexStorage using QueryFilters from request params
///
/// This test WILL FAIL because handle_index() is a stub.
#[test]
fn test_search_with_type_filter() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    setup_test_project_with_symbols(&temp_dir);
    let socket_path = start_daemon(&temp_dir);

    // Index the project
    index_via_daemon(&socket_path);

    // Act: Search for "foo" with type filter "function"
    let request = r#"{"id": "search-6", "method": "search", "params": {"symbol": "foo", "type": "function"}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Should only return functions, not classes
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    let symbols = extract_symbols(&json, &response);

    // FAILS: handle_index is stub
    assert!(
        !symbols.is_empty(),
        "Should find function symbols matching 'foo'.\n\
         Root cause: handle_index is stub\n\
         Response: {}",
        response
    );

    // All returned symbols should be functions (or methods)
    for symbol in &symbols {
        let sym_type = symbol["type"].as_str().unwrap_or("");
        assert!(
            sym_type == "function" || sym_type == "method",
            "With type='function' filter, should only return functions/methods.\n\
             Got type: {}\n\
             Symbol: {:?}",
            sym_type,
            symbol
        );
    }
}

/// Test: Daemon auto-indexes on startup, search finds symbols immediately
/// Requirement: The daemon indexes automatically on startup per run_server()
///
/// This test verifies that the daemon's auto-indexing feature works correctly.
/// When the daemon starts, it automatically indexes source files in the project,
/// so search should find symbols immediately without a separate index request.
#[test]
fn test_search_after_daemon_autoindex_finds_symbols() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    setup_test_project_with_symbols(&temp_dir);
    let socket_path = start_daemon(&temp_dir);

    // The daemon auto-indexes on startup (see run_server() in daemon.rs)
    // No explicit index request needed

    // Act: Search immediately after daemon start
    let request = r#"{"id": "search-7", "method": "search", "params": {"symbol": "foo"}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Auto-indexing should have populated the index
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    let symbols = extract_symbols(&json, &response);

    // With auto-indexing, we should find the foo symbols from setup
    assert!(
        !symbols.is_empty(),
        "Daemon auto-indexes on startup, should find symbols immediately.\n\
         Response: {}",
        response
    );
}
