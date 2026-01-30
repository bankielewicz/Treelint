//! AC#2: Daemon Index Triggers Actual Indexing Tests
//!
//! Given: Daemon is running in a project directory
//! When: Client sends index request via daemon protocol: {"method": "index", "params": {}}
//! Then:
//!   - SymbolExtractor parses source files
//!   - Symbols stored in IndexStorage
//!   - Response includes actual files_indexed and symbols_found counts
//!   - Status is "completed" after indexing finishes
//!
//! Source files tested:
//!   - src/daemon/server.rs (handle_index implementation)
//!   - src/parser/symbols.rs (SymbolExtractor)
//!   - src/index/storage.rs (IndexStorage)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - INT-003: Discover source files and extract symbols using SymbolExtractor
//!   - INT-004: Store extracted symbols in IndexStorage
//!
//! TDD Phase: RED
//!
//! These tests will FAIL because handle_index() currently returns:
//!   {"status": "completed", "files_indexed": 0, "symbols_found": 0}
//! instead of actually indexing files.

use assert_cmd::Command;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Helper: Create a test project with known files and symbols
fn setup_test_project_with_files(temp_dir: &TempDir, file_count: usize) -> usize {
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let mut expected_symbols = 0;

    for i in 0..file_count {
        // Each Python file has 2 functions and 1 class with 1 method = 4 symbols
        let py_content = format!(
            r#"
def function_a_{i}():
    """Function A number {i}"""
    return {i}

def function_b_{i}():
    """Function B number {i}"""
    return {i} * 2

class Class_{i}:
    """Class number {i}"""
    def method_{i}(self):
        pass
"#,
            i = i
        );
        fs::write(src_dir.join(format!("module_{}.py", i)), py_content)
            .expect("Failed to write Python file");
        expected_symbols += 4; // 2 functions + 1 class + 1 method
    }

    expected_symbols
}

/// Helper: Start daemon and return socket path
fn start_daemon(temp_dir: &TempDir) -> String {
    Command::cargo_bin("treelint")
        .expect("treelint binary not found")
        .current_dir(temp_dir.path())
        .args(["daemon", "start"])
        .assert()
        .success();

    thread::sleep(Duration::from_millis(300));

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

/// Helper: Extract symbols array from search response
fn extract_symbols(json: &serde_json::Value, response: &str) -> Vec<serde_json::Value> {
    if let Some(error) = json.get("error") {
        if !error.is_null() {
            return Vec::new();
        }
    }

    if let Some(arr) = json["result"].as_array() {
        return arr.clone();
    }

    if let Some(arr) = json["result"]["symbols"].as_array() {
        return arr.clone();
    }

    panic!("Could not extract symbols from response: {}", response);
}

/// Test: Index request triggers actual file indexing
/// Requirement: INT-003 - Discover source files and extract symbols using SymbolExtractor
///
/// This test WILL FAIL because handle_index returns hardcoded files_indexed: 0.
#[test]
fn test_index_triggers_actual_file_indexing() {
    // Arrange: Create project with 5 files
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let _expected_symbols = setup_test_project_with_files(&temp_dir, 5);
    let socket_path = start_daemon(&temp_dir);

    // Act: Send index request
    let request = r#"{"id": "index-1", "method": "index", "params": {}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: files_indexed should be > 0
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    // FAILS: Current implementation returns files_indexed: 0
    let files_indexed = json["result"]["files_indexed"]
        .as_u64()
        .expect("files_indexed should be a number");

    assert!(
        files_indexed > 0,
        "Index should report actual files indexed.\n\
         Expected: 5 files (module_0.py through module_4.py)\n\
         Actual: {} files\n\
         Root cause: handle_index is stub\n\
         Response: {}",
        files_indexed,
        response
    );

    assert_eq!(
        files_indexed, 5,
        "Should index exactly 5 Python files.\n\
         Response: {}",
        response
    );
}

/// Test: Index request extracts symbols using SymbolExtractor
/// Requirement: INT-003 - Discover source files and extract symbols using SymbolExtractor
///
/// This test WILL FAIL because handle_index returns hardcoded symbols_found: 0.
#[test]
fn test_index_extracts_symbols_with_symbol_extractor() {
    // Arrange: Create project with known symbol count
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let expected_symbols = setup_test_project_with_files(&temp_dir, 3);
    let socket_path = start_daemon(&temp_dir);

    // Act: Send index request
    let request = r#"{"id": "index-2", "method": "index", "params": {}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: symbols_found should match expected count
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    // FAILS: Current implementation returns symbols_found: 0
    let symbols_found = json["result"]["symbols_found"]
        .as_u64()
        .expect("symbols_found should be a number");

    assert!(
        symbols_found > 0,
        "Index should report actual symbols found.\n\
         Expected: approximately {} symbols\n\
         Actual: {} symbols\n\
         Root cause: handle_index is stub\n\
         Response: {}",
        expected_symbols,
        symbols_found,
        response
    );

    // Allow some variance in symbol count
    let min_expected = (expected_symbols as f64 * 0.5) as u64;
    assert!(
        symbols_found >= min_expected,
        "Should extract at least half the expected symbols.\n\
         Expected: >= {} symbols\n\
         Actual: {} symbols\n\
         Response: {}",
        min_expected,
        symbols_found,
        response
    );
}

/// Test: Index stores symbols in IndexStorage
/// Requirement: INT-004 - Store extracted symbols in IndexStorage
///
/// This test WILL FAIL because handle_index doesn't actually store symbols.
#[test]
fn test_index_stores_symbols_in_index_storage() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let _ = setup_test_project_with_files(&temp_dir, 2);
    let socket_path = start_daemon(&temp_dir);

    // Act: Index the project
    let index_request = r#"{"id": "index-3", "method": "index", "params": {}}"#;
    let index_response = send_daemon_request(&socket_path, index_request);

    // Verify indexing completed
    let index_json: serde_json::Value = serde_json::from_str(&index_response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, index_response));
    assert_eq!(
        index_json["result"]["status"].as_str(),
        Some("completed"),
        "Index should complete successfully"
    );

    // Now search for symbols that should have been indexed
    let search_request =
        r#"{"id": "search-3", "method": "search", "params": {"symbol": "function_a"}}"#;
    let search_response = send_daemon_request(&socket_path, search_request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Search should find the indexed symbols
    let search_json: serde_json::Value = serde_json::from_str(&search_response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, search_response));

    let symbols = extract_symbols(&search_json, &search_response);

    // FAILS: Symbols not actually stored, search returns empty
    assert!(
        !symbols.is_empty(),
        "After indexing, search should find stored symbols.\n\
         Root cause: handle_index is stub\n\
         Index response: {}\n\
         Search response: {}",
        index_response,
        search_response
    );
}

/// Test: Index response has status "completed"
/// Requirement: AC#2 - Status is "completed" after indexing finishes
///
/// This test should PASS - verifies response format.
#[test]
fn test_index_response_has_completed_status() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let _ = setup_test_project_with_files(&temp_dir, 2);
    let socket_path = start_daemon(&temp_dir);

    // Act: Send index request
    let request = r#"{"id": "index-4", "method": "index", "params": {}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: status is "completed"
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    let status = json["result"]["status"]
        .as_str()
        .expect("status should be a string");

    assert_eq!(
        status, "completed",
        "Index response status should be 'completed'.\n\
         Response: {}",
        response
    );
}

/// Test: Index on empty directory succeeds with zero counts
/// Requirement: INT-003 - Discover source files
///
/// Edge case: indexing a directory with no supported files.
#[test]
fn test_index_empty_directory_succeeds() {
    // Arrange: Empty directory
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let socket_path = start_daemon(&temp_dir);

    // Act: Send index request
    let request = r#"{"id": "index-6", "method": "index", "params": {}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Should succeed with 0 files/symbols (not error)
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    assert!(
        json.get("error").is_none() || json["error"].is_null(),
        "Index on empty directory should not return error.\n\
         Response: {}",
        response
    );

    let status = json["result"]["status"].as_str().unwrap_or("");
    assert_eq!(
        status, "completed",
        "Index on empty directory should complete.\n\
         Response: {}",
        response
    );

    let files_indexed = json["result"]["files_indexed"].as_u64().unwrap_or(0);
    assert_eq!(
        files_indexed, 0,
        "Empty directory should have 0 files indexed.\n\
         Response: {}",
        response
    );
}

/// Test: Index uses SymbolExtractor (not hardcoded data)
/// Requirement: BR-002 - Index requests must use SymbolExtractor for parsing
///
/// This test WILL FAIL because handle_index doesn't actually use SymbolExtractor.
#[test]
fn test_index_uses_symbol_extractor_not_hardcoded() {
    // Arrange: Create a specific file with unique symbol names
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create file with uniquely named symbols
    let py_content = r#"
def unique_test_function_xyz_123():
    """A uniquely named function"""
    return "unique"

class UniqueTestClassABC789:
    """A uniquely named class"""
    pass
"#;
    fs::write(src_dir.join("unique_module.py"), py_content).expect("Failed to write Python file");

    let socket_path = start_daemon(&temp_dir);

    // Act: Index and then search for unique symbol
    let index_request = r#"{"id": "index-7", "method": "index", "params": {}}"#;
    let _index_response = send_daemon_request(&socket_path, index_request);

    let search_request = r#"{"id": "search-7", "method": "search", "params": {"symbol": "unique_test_function_xyz_123"}}"#;
    let search_response = send_daemon_request(&socket_path, search_request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Should find the unique symbol (proves SymbolExtractor was used)
    let json: serde_json::Value = serde_json::from_str(&search_response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, search_response));

    let symbols = extract_symbols(&json, &search_response);

    // FAILS: Symbols not actually extracted
    assert!(
        !symbols.is_empty(),
        "Should find unique symbol after indexing.\n\
         This proves SymbolExtractor actually parsed the file.\n\
         Root cause: handle_index is stub\n\
         Search response: {}",
        search_response
    );

    // Verify we found the exact symbol
    let found_unique = symbols
        .iter()
        .any(|s| s["name"].as_str() == Some("unique_test_function_xyz_123"));

    assert!(
        found_unique,
        "Should find exactly 'unique_test_function_xyz_123'.\n\
         Found symbols: {:?}",
        symbols
    );
}

/// Test: Index handles multiple file types
/// Requirement: INT-003 - Discover source files
///
/// This test WILL FAIL because handle_index doesn't actually index.
#[test]
fn test_index_handles_multiple_file_types() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Python file
    fs::write(
        src_dir.join("module.py"),
        "def py_function(): pass\nclass PyClass: pass",
    )
    .expect("Failed to write Python file");

    // Rust file
    fs::write(
        src_dir.join("lib.rs"),
        "fn rust_function() {}\npub struct RustStruct {}",
    )
    .expect("Failed to write Rust file");

    // TypeScript file
    fs::write(
        src_dir.join("component.ts"),
        "function tsFunction(): void {}\nclass TsClass {}",
    )
    .expect("Failed to write TypeScript file");

    let socket_path = start_daemon(&temp_dir);

    // Act: Index all files
    let request = r#"{"id": "index-8", "method": "index", "params": {}}"#;
    let response = send_daemon_request(&socket_path, request);

    // Cleanup
    stop_daemon(&temp_dir);

    // Assert: Should index all 3 files
    let json: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_else(|e| panic!("Invalid JSON: {}\nResponse: '{}'", e, response));

    // FAILS: Current implementation returns files_indexed: 0
    let files_indexed = json["result"]["files_indexed"]
        .as_u64()
        .expect("files_indexed should be a number");

    assert!(
        files_indexed >= 3,
        "Should index at least 3 files (py, rs, ts).\n\
         Actual: {} files\n\
         Root cause: handle_index is stub\n\
         Response: {}",
        files_indexed,
        response
    );

    let symbols_found = json["result"]["symbols_found"]
        .as_u64()
        .expect("symbols_found should be a number");

    // Each file has 2 symbols = 6 total minimum
    assert!(
        symbols_found >= 6,
        "Should find at least 6 symbols (2 per file).\n\
         Actual: {} symbols\n\
         Response: {}",
        symbols_found,
        response
    );
}
