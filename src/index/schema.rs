//! Database schema definitions and initialization.
//!
//! This module contains the SQL DDL statements for creating the index database
//! schema, including tables, indexes, and PRAGMA settings.

use rusqlite::{Connection, OptionalExtension};

use super::StorageError;

/// Schema version for migration tracking.
///
/// This version is stored in the metadata table and can be used to detect
/// when schema migrations are needed.
pub const SCHEMA_VERSION: &str = "1.0.0";

/// SQL statement to create the symbols table.
const CREATE_SYMBOLS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS symbols (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    visibility TEXT,
    file_path TEXT NOT NULL,
    line_start INTEGER NOT NULL,
    line_end INTEGER NOT NULL,
    signature TEXT,
    body TEXT,
    language TEXT NOT NULL,
    updated_at INTEGER NOT NULL
)
"#;

/// SQL statement to create the files table.
const CREATE_FILES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS files (
    path TEXT PRIMARY KEY,
    language TEXT NOT NULL,
    hash TEXT NOT NULL,
    indexed_at INTEGER NOT NULL
)
"#;

/// SQL statement to create the metadata table.
const CREATE_METADATA_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
)
"#;

/// SQL statements to create indexes for faster queries.
const CREATE_INDEXES: &[&str] = &[
    "CREATE INDEX IF NOT EXISTS idx_symbols_name ON symbols(name)",
    "CREATE INDEX IF NOT EXISTS idx_symbols_type ON symbols(type)",
    "CREATE INDEX IF NOT EXISTS idx_symbols_file ON symbols(file_path)",
];

/// Apply PRAGMA settings for optimal performance and safety.
///
/// This function configures SQLite with:
/// - WAL mode for concurrent reads
/// - synchronous=NORMAL for balanced durability/performance
/// - foreign_keys=ON for referential integrity
///
/// # Arguments
///
/// * `conn` - The database connection to configure
///
/// # Returns
///
/// `Ok(())` if all PRAGMAs were applied successfully, `Err(StorageError)` otherwise.
pub fn apply_pragmas(conn: &Connection) -> Result<(), StorageError> {
    conn.pragma_update(None, "journal_mode", "WAL")
        .map_err(|e| StorageError::QueryFailed(format!("Failed to set journal_mode: {}", e)))?;

    conn.pragma_update(None, "synchronous", "NORMAL")
        .map_err(|e| StorageError::QueryFailed(format!("Failed to set synchronous: {}", e)))?;

    conn.pragma_update(None, "foreign_keys", "ON")
        .map_err(|e| StorageError::QueryFailed(format!("Failed to set foreign_keys: {}", e)))?;

    Ok(())
}

/// Create all tables in the database schema.
///
/// This function creates the symbols, files, and metadata tables if they
/// don't already exist.
///
/// # Arguments
///
/// * `conn` - The database connection
///
/// # Returns
///
/// `Ok(())` if tables were created successfully, `Err(StorageError)` otherwise.
pub fn create_tables(conn: &Connection) -> Result<(), StorageError> {
    conn.execute(CREATE_SYMBOLS_TABLE, [])
        .map_err(|e| StorageError::QueryFailed(format!("Failed to create symbols table: {}", e)))?;

    conn.execute(CREATE_FILES_TABLE, [])
        .map_err(|e| StorageError::QueryFailed(format!("Failed to create files table: {}", e)))?;

    conn.execute(CREATE_METADATA_TABLE, []).map_err(|e| {
        StorageError::QueryFailed(format!("Failed to create metadata table: {}", e))
    })?;

    Ok(())
}

/// Create all indexes for faster queries.
///
/// This function creates indexes on frequently queried columns in the
/// symbols table.
///
/// # Arguments
///
/// * `conn` - The database connection
///
/// # Returns
///
/// `Ok(())` if indexes were created successfully, `Err(StorageError)` otherwise.
pub fn create_indexes(conn: &Connection) -> Result<(), StorageError> {
    for index_sql in CREATE_INDEXES {
        conn.execute(index_sql, [])
            .map_err(|e| StorageError::QueryFailed(format!("Failed to create index: {}", e)))?;
    }
    Ok(())
}

/// Store the schema version in the metadata table.
///
/// # Arguments
///
/// * `conn` - The database connection
///
/// # Returns
///
/// `Ok(())` if version was stored successfully, `Err(StorageError)` otherwise.
pub fn store_schema_version(conn: &Connection) -> Result<(), StorageError> {
    conn.execute(
        "INSERT OR REPLACE INTO metadata (key, value) VALUES (?1, ?2)",
        rusqlite::params!["schema_version", SCHEMA_VERSION],
    )
    .map_err(|e| StorageError::QueryFailed(format!("Failed to store schema version: {}", e)))?;

    Ok(())
}

/// Check if schema is already initialized.
///
/// # Arguments
///
/// * `conn` - The database connection
///
/// # Returns
///
/// `Ok(true)` if schema exists, `Ok(false)` otherwise.
fn schema_exists(conn: &Connection) -> Result<bool, StorageError> {
    let result: Option<String> = conn
        .query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='symbols'",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| StorageError::QueryFailed(format!("Failed to check schema: {}", e)))?;

    Ok(result.is_some())
}

/// Initialize the complete database schema.
///
/// This function applies PRAGMAs, creates all tables and indexes,
/// and stores the schema version. If the schema already exists,
/// only PRAGMAs are applied.
///
/// # Arguments
///
/// * `conn` - The database connection
///
/// # Returns
///
/// `Ok(())` if schema was initialized successfully, `Err(StorageError)` otherwise.
pub fn initialize_schema(conn: &Connection) -> Result<(), StorageError> {
    apply_pragmas(conn)?;

    // Only create tables if schema doesn't exist yet
    if !schema_exists(conn)? {
        create_tables(conn)?;
        create_indexes(conn)?;
        store_schema_version(conn)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_schema_version_is_valid_semver() {
        let parts: Vec<&str> = SCHEMA_VERSION.split('.').collect();
        assert_eq!(parts.len(), 3, "Schema version should be semver");
        for part in parts {
            assert!(part.parse::<u32>().is_ok(), "Each part should be a number");
        }
    }

    #[test]
    fn test_apply_pragmas_succeeds() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = Connection::open(&db_path).unwrap();

        let result = apply_pragmas(&conn);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_tables_succeeds() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = Connection::open(&db_path).unwrap();

        let result = create_tables(&conn);
        assert!(result.is_ok());
    }
}
