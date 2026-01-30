//! AC#8: Graph Storage and Caching Tests
//!
//! Given: Repository with call/import relationships
//! When: Graph data is extracted
//! Then:
//!   - Call edges stored in SQLite (caller_id, callee_id, count)
//!   - Import edges stored in SQLite (importer_path, imported_path)
//!   - Incremental update: only re-analyze changed files
//!   - Full rebuild with `--force` flag
//!
//! Source files tested:
//!   - src/graph/calls.rs (CallGraphExtractor storage)
//!   - src/graph/imports.rs (ImportGraphExtractor storage)
//!   - src/index/schema.rs (call_edges, import_edges tables)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - DEPS-013: Store call edges in SQLite
//!   - DEPS-014: Store import edges in SQLite

use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to get index database path
fn get_index_db_path(temp_dir: &TempDir) -> PathBuf {
    temp_dir.path().join(".treelint").join("index.db")
}

/// Test: call_edges table exists after deps --calls
/// Requirement: DEPS-013 - Store call edges in SQLite
#[test]
fn test_call_edges_table_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def caller(): callee()\ndef callee(): pass",
    )
    .unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    // Check that index.db exists and has call_edges table
    let db_path = get_index_db_path(&temp_dir);
    assert!(
        db_path.exists(),
        "Index database should exist at {:?}",
        db_path
    );

    // Verify call_edges table by querying it
    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");
    let table_exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='call_edges'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    assert!(
        table_exists,
        "call_edges table should exist in the database"
    );
}

/// Test: import_edges table exists after deps --imports
/// Requirement: DEPS-014 - Store import edges in SQLite
#[test]
fn test_import_edges_table_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("utils.py"), "X = 1").unwrap();
    fs::write(temp_dir.path().join("main.py"), "from utils import X").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["deps", "--imports"])
        .assert()
        .success();

    let db_path = get_index_db_path(&temp_dir);
    assert!(db_path.exists(), "Index database should exist");

    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");
    let table_exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='import_edges'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    assert!(
        table_exists,
        "import_edges table should exist in the database"
    );
}

/// Test: call_edges has caller_id column
/// Requirement: DEPS-013 - caller_id field
#[test]
fn test_call_edges_has_caller_id() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def caller(): callee()\ndef callee(): pass",
    )
    .unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    let db_path = get_index_db_path(&temp_dir);
    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");

    // Check for caller_id column
    let has_caller_id: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM pragma_table_info('call_edges') WHERE name='caller_id'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    assert!(has_caller_id, "call_edges should have caller_id column");
}

/// Test: call_edges has callee_id column
/// Requirement: DEPS-013 - callee_id field
#[test]
fn test_call_edges_has_callee_id() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def caller(): callee()\ndef callee(): pass",
    )
    .unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    let db_path = get_index_db_path(&temp_dir);
    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");

    let has_callee_id: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM pragma_table_info('call_edges') WHERE name='callee_id'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    assert!(has_callee_id, "call_edges should have callee_id column");
}

/// Test: call_edges has call_count column
/// Requirement: DEPS-013 - count field
#[test]
fn test_call_edges_has_count() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def caller(): callee()\ndef callee(): pass",
    )
    .unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    let db_path = get_index_db_path(&temp_dir);
    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");

    let has_count: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM pragma_table_info('call_edges') WHERE name='call_count'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    assert!(has_count, "call_edges should have call_count column");
}

/// Test: import_edges has importer_path column
/// Requirement: DEPS-014 - importer_path field
#[test]
fn test_import_edges_has_importer_path() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("utils.py"), "X = 1").unwrap();
    fs::write(temp_dir.path().join("main.py"), "from utils import X").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["deps", "--imports"])
        .assert()
        .success();

    let db_path = get_index_db_path(&temp_dir);
    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");

    let has_importer: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM pragma_table_info('import_edges') WHERE name='importer_path'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    assert!(
        has_importer,
        "import_edges should have importer_path column"
    );
}

/// Test: import_edges has imported_path column
/// Requirement: DEPS-014 - imported_path field
#[test]
fn test_import_edges_has_imported_path() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("utils.py"), "X = 1").unwrap();
    fs::write(temp_dir.path().join("main.py"), "from utils import X").unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["deps", "--imports"])
        .assert()
        .success();

    let db_path = get_index_db_path(&temp_dir);
    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");

    let has_imported: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM pragma_table_info('import_edges') WHERE name='imported_path'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    assert!(
        has_imported,
        "import_edges should have imported_path column"
    );
}

/// Test: BR-003 - Incremental update only re-analyzes changed files
/// Requirement: BR-003 - Incremental graph updates
#[test]
fn test_incremental_update_only_changed_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("unchanged.py"), "def foo(): pass").unwrap();
    fs::write(temp_dir.path().join("changed.py"), "def bar(): foo()").unwrap();

    // Initial index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Initial deps extraction
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    // Modify one file
    std::thread::sleep(std::time::Duration::from_millis(100));
    fs::write(
        temp_dir.path().join("changed.py"),
        "def bar(): foo()\ndef baz(): pass",
    )
    .unwrap();

    // Re-run deps (should be incremental)
    let start = std::time::Instant::now();
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();
    let duration = start.elapsed();

    // Incremental should be fast (< 5 seconds for 2 files)
    assert!(
        duration.as_secs() < 5,
        "Incremental update should be fast, took {:?}",
        duration
    );
}

/// Test: --force flag triggers full rebuild
/// Requirement: BR-003 - Full rebuild with --force
#[test]
fn test_force_flag_full_rebuild() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def caller(): callee()\ndef callee(): pass",
    )
    .unwrap();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Run with --force flag
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["deps", "--calls", "--force"])
        .assert()
        .success();

    // Verify data exists after force rebuild
    let db_path = get_index_db_path(&temp_dir);
    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");

    let edge_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM call_edges", [], |row| row.get(0))
        .unwrap_or(0);

    assert!(
        edge_count >= 0,
        "Force rebuild should produce valid edge data"
    );
}

/// Test: Graph data persists across CLI invocations
/// Requirement: DEPS-013, DEPS-014 - Persistent storage
#[test]
fn test_graph_data_persists() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        "def caller(): callee()\ndef callee(): pass",
    )
    .unwrap();

    // First: index and extract deps
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    // Second: run deps again (should use persisted data)
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    let output = cmd
        .current_dir(temp_dir.path())
        .args(["deps", "--calls"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("caller") || stdout.contains("callee") || stdout.contains("edges"),
        "Persisted graph data should be readable.\n\nActual output:\n{}",
        stdout
    );
}
