//! Index storage implementation using SQLite.
//!
//! This module provides the [`IndexStorage`] struct for persisting and
//! querying extracted symbols.

use std::cell::RefCell;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection, OptionalExtension};

use super::schema;
use super::search::QueryFilters;
use crate::parser::{Language, Symbol, SymbolType, Visibility};

/// Error type for storage operations.
///
/// All storage errors are wrapped in this enum to provide clear error
/// categorization and helpful error messages.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// Database file is corrupted or invalid.
    #[error("Database corrupted: {0}")]
    DatabaseCorrupted(String),

    /// Permission denied when accessing database file.
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Disk is full, unable to write to database.
    #[error("Disk full - unable to write to database")]
    DiskFull,

    /// Failed to establish database connection.
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Query execution failed.
    #[error("Query failed: {0}")]
    QueryFailed(String),
}

impl From<rusqlite::Error> for StorageError {
    fn from(err: rusqlite::Error) -> Self {
        let msg = err.to_string();
        let msg_lower = msg.to_lowercase();

        // Categorize based on error message content
        if msg_lower.contains("permission denied")
            || msg_lower.contains("access denied")
            || msg_lower.contains("readonly")
            || msg_lower.contains("read-only")
        {
            StorageError::PermissionDenied(msg)
        } else if msg_lower.contains("disk full")
            || msg_lower.contains("no space")
            || msg_lower.contains("disk i/o")
        {
            StorageError::DiskFull
        } else if msg_lower.contains("not a database")
            || msg_lower.contains("corrupt")
            || msg_lower.contains("malformed")
        {
            StorageError::DatabaseCorrupted(msg)
        } else if msg_lower.contains("unable to open") {
            StorageError::ConnectionFailed(msg)
        } else {
            StorageError::QueryFailed(msg)
        }
    }
}

/// Information about a tracked file.
#[derive(Debug, Clone, PartialEq)]
pub struct FileInfo {
    /// Path to the file relative to project root.
    pub path: String,
    /// Programming language of the file.
    pub language: Language,
    /// Content hash for change detection.
    pub hash: String,
    /// Unix timestamp when the file was last indexed.
    pub indexed_at: u64,
}

/// SQLite-based storage for the symbol index.
///
/// `IndexStorage` provides persistent storage for extracted symbols with
/// efficient query operations and file tracking for incremental updates.
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use treelint::index::IndexStorage;
///
/// let storage = IndexStorage::new(Path::new("/path/to/project"))?;
/// # Ok::<(), treelint::index::StorageError>(())
/// ```
#[derive(Debug)]
pub struct IndexStorage {
    /// The underlying SQLite connection wrapped in RefCell for interior mutability.
    conn: RefCell<Option<Connection>>,
}

impl IndexStorage {
    /// Create or open the index database.
    ///
    /// Creates the `.treelint` directory and `index.db` file if they don't exist.
    /// Initializes the schema for new databases.
    ///
    /// # Arguments
    ///
    /// * `project_root` - Path to the project root directory
    ///
    /// # Returns
    ///
    /// `Ok(IndexStorage)` if successful, `Err(StorageError)` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Cannot create `.treelint` directory
    /// - Cannot open/create the database file
    /// - Database file is corrupted
    pub fn new(project_root: &Path) -> Result<Self, StorageError> {
        let treelint_dir = project_root.join(".treelint");

        // Create .treelint directory if it doesn't exist
        if !treelint_dir.exists() {
            std::fs::create_dir_all(&treelint_dir).map_err(|e| {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    StorageError::PermissionDenied(format!(
                        "Permission denied: Cannot create .treelint directory: {}",
                        e
                    ))
                } else {
                    StorageError::ConnectionFailed(format!(
                        "Failed to create .treelint directory: {}",
                        e
                    ))
                }
            })?;
        }

        let db_path = treelint_dir.join("index.db");

        // Check if directory is writable by checking if we can create/open the db file
        // This helps detect permission issues early
        if treelint_dir.exists() {
            let metadata = std::fs::metadata(&treelint_dir).map_err(|e| {
                StorageError::ConnectionFailed(format!("Failed to read directory metadata: {}", e))
            })?;

            if metadata.permissions().readonly() {
                return Err(StorageError::PermissionDenied(format!(
                    "Permission denied: Directory is read-only: {}",
                    treelint_dir.display()
                )));
            }
        }

        // Try to open the database
        let conn = Connection::open(&db_path).map_err(|e| {
            let msg = e.to_string();
            let msg_lower = msg.to_lowercase();
            if msg_lower.contains("not a database")
                || msg_lower.contains("corrupt")
                || msg_lower.contains("malformed")
            {
                StorageError::DatabaseCorrupted(format!("Database file is corrupted: {}", e))
            } else if msg_lower.contains("permission")
                || msg_lower.contains("access denied")
                || msg_lower.contains("readonly")
                || msg_lower.contains("read-only")
                || msg_lower.contains("unable to open")
            {
                // "unable to open" often means permission denied in practice
                StorageError::PermissionDenied(format!(
                    "Permission denied: Cannot open database file: {}",
                    db_path.display()
                ))
            } else {
                StorageError::ConnectionFailed(format!("Failed to open database: {}", e))
            }
        })?;

        // Initialize schema
        schema::initialize_schema(&conn)?;

        Ok(Self {
            conn: RefCell::new(Some(conn)),
        })
    }

    /// Get a reference to the connection, returning an error if closed.
    fn conn(&self) -> Result<std::cell::Ref<'_, Connection>, StorageError> {
        let borrow = self.conn.borrow();
        if borrow.is_none() {
            return Err(StorageError::ConnectionFailed(
                "Connection has been closed".to_string(),
            ));
        }
        Ok(std::cell::Ref::map(borrow, |opt| opt.as_ref().unwrap()))
    }

    /// Get a mutable reference to the connection, returning an error if closed.
    fn conn_mut(&self) -> Result<std::cell::RefMut<'_, Connection>, StorageError> {
        let borrow = self.conn.borrow_mut();
        if borrow.is_none() {
            return Err(StorageError::ConnectionFailed(
                "Connection has been closed".to_string(),
            ));
        }
        Ok(std::cell::RefMut::map(borrow, |opt| opt.as_mut().unwrap()))
    }

    /// Close the database connection.
    ///
    /// After calling this method, all subsequent operations will return an error.
    pub fn close(&self) -> Result<(), StorageError> {
        let mut conn_opt = self.conn.borrow_mut();
        *conn_opt = None;
        Ok(())
    }

    /// Get the current Unix timestamp.
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }

    // ========================================================================
    // Table/Index/Pragma Inspection Methods (for tests)
    // ========================================================================

    /// Get list of all tables in the database.
    pub fn get_tables(&self) -> Result<Vec<String>, StorageError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .map_err(StorageError::from)?;

        let tables = stmt
            .query_map([], |row| row.get(0))
            .map_err(StorageError::from)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(tables)
    }

    /// Get list of columns for a table.
    pub fn get_table_columns(&self, table: &str) -> Result<Vec<String>, StorageError> {
        let conn = self.conn()?;
        let sql = format!("PRAGMA table_info({})", table);
        let mut stmt = conn.prepare(&sql).map_err(StorageError::from)?;

        let columns = stmt
            .query_map([], |row| row.get::<_, String>(1))
            .map_err(StorageError::from)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(columns)
    }

    /// Get list of all indexes in the database.
    pub fn get_indexes(&self) -> Result<Vec<String>, StorageError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name NOT LIKE 'sqlite_%' ORDER BY name")
            .map_err(StorageError::from)?;

        let indexes = stmt
            .query_map([], |row| row.get(0))
            .map_err(StorageError::from)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(indexes)
    }

    /// Get a PRAGMA value.
    pub fn get_pragma(&self, name: &str) -> Result<String, StorageError> {
        let conn = self.conn()?;
        let sql = format!("PRAGMA {}", name);

        // PRAGMAs can return either string or integer values
        // Try string first, then fall back to integer
        let result = conn.query_row(&sql, [], |row| {
            // Try to get as string first
            if let Ok(s) = row.get::<_, String>(0) {
                return Ok(s);
            }
            // Fall back to integer
            if let Ok(i) = row.get::<_, i64>(0) {
                return Ok(i.to_string());
            }
            // Last resort: try bool
            if let Ok(b) = row.get::<_, bool>(0) {
                return Ok(if b { "1" } else { "0" }.to_string());
            }
            Err(rusqlite::Error::InvalidColumnType(
                0,
                name.to_string(),
                rusqlite::types::Type::Null,
            ))
        });

        result.map_err(StorageError::from)
    }

    /// Get a metadata value by key.
    pub fn get_metadata(&self, key: &str) -> Result<Option<String>, StorageError> {
        let conn = self.conn()?;
        let value: Option<String> = conn
            .query_row(
                "SELECT value FROM metadata WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .optional()
            .map_err(StorageError::from)?;
        Ok(value)
    }

    // ========================================================================
    // Symbol CRUD Operations
    // ========================================================================

    /// Insert a single symbol into the database.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to insert
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, `Err(StorageError)` otherwise.
    pub fn insert_symbol(&self, symbol: &Symbol) -> Result<(), StorageError> {
        let conn = self.conn()?;
        let timestamp = Self::current_timestamp();

        conn.execute(
            r#"
            INSERT INTO symbols (name, type, visibility, file_path, line_start, line_end, signature, body, language, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            params![
                symbol.name,
                symbol_type_to_str(symbol.symbol_type),
                symbol.visibility.map(visibility_to_str),
                symbol.file_path,
                symbol.line_start as i64,
                symbol.line_end as i64,
                symbol.signature,
                symbol.body,
                language_to_str(symbol.language),
                timestamp as i64,
            ],
        )
        .map_err(StorageError::from)?;

        Ok(())
    }

    /// Insert multiple symbols in a single transaction.
    ///
    /// This method is more efficient than calling `insert_symbol` repeatedly
    /// and ensures atomicity - either all symbols are inserted or none.
    ///
    /// # Arguments
    ///
    /// * `symbols` - Slice of symbols to insert
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, `Err(StorageError)` otherwise.
    pub fn insert_symbols(&self, symbols: &[Symbol]) -> Result<(), StorageError> {
        let mut conn = self.conn_mut()?;
        let timestamp = Self::current_timestamp();

        let tx = conn.transaction().map_err(StorageError::from)?;

        {
            let mut stmt = tx
                .prepare(
                    r#"
                    INSERT INTO symbols (name, type, visibility, file_path, line_start, line_end, signature, body, language, updated_at)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                    "#,
                )
                .map_err(StorageError::from)?;

            for symbol in symbols {
                stmt.execute(params![
                    symbol.name,
                    symbol_type_to_str(symbol.symbol_type),
                    symbol.visibility.map(visibility_to_str),
                    symbol.file_path,
                    symbol.line_start as i64,
                    symbol.line_end as i64,
                    symbol.signature,
                    symbol.body,
                    language_to_str(symbol.language),
                    timestamp as i64,
                ])
                .map_err(StorageError::from)?;
            }
        }

        tx.commit().map_err(StorageError::from)?;
        Ok(())
    }

    /// Delete all symbols for a specific file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file whose symbols should be deleted
    ///
    /// # Returns
    ///
    /// `Ok(usize)` with count of deleted symbols, `Err(StorageError)` otherwise.
    pub fn delete_symbols_for_file(&self, file_path: &str) -> Result<usize, StorageError> {
        let conn = self.conn()?;
        let count = conn
            .execute(
                "DELETE FROM symbols WHERE file_path = ?1",
                params![file_path],
            )
            .map_err(StorageError::from)?;
        Ok(count)
    }

    /// Upsert symbols for a file (delete existing, then insert new).
    ///
    /// This provides a simple re-index operation: all symbols for the file
    /// are replaced with the new set.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file being re-indexed
    /// * `symbols` - New symbols to insert
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, `Err(StorageError)` otherwise.
    pub fn upsert_symbols_for_file(
        &self,
        file_path: &str,
        symbols: &[Symbol],
    ) -> Result<(), StorageError> {
        let mut conn = self.conn_mut()?;
        let timestamp = Self::current_timestamp();

        let tx = conn.transaction().map_err(StorageError::from)?;

        // Delete existing symbols for this file
        tx.execute(
            "DELETE FROM symbols WHERE file_path = ?1",
            params![file_path],
        )
        .map_err(StorageError::from)?;

        // Insert new symbols
        {
            let mut stmt = tx
                .prepare(
                    r#"
                    INSERT INTO symbols (name, type, visibility, file_path, line_start, line_end, signature, body, language, updated_at)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                    "#,
                )
                .map_err(StorageError::from)?;

            for symbol in symbols {
                stmt.execute(params![
                    symbol.name,
                    symbol_type_to_str(symbol.symbol_type),
                    symbol.visibility.map(visibility_to_str),
                    symbol.file_path,
                    symbol.line_start as i64,
                    symbol.line_end as i64,
                    symbol.signature,
                    symbol.body,
                    language_to_str(symbol.language),
                    timestamp as i64,
                ])
                .map_err(StorageError::from)?;
            }
        }

        tx.commit().map_err(StorageError::from)?;
        Ok(())
    }

    /// Get the updated_at timestamp for a symbol by name (for tests).
    pub fn get_symbol_updated_at(&self, name: &str) -> Result<u64, StorageError> {
        let conn = self.conn()?;
        let timestamp: i64 = conn
            .query_row(
                "SELECT updated_at FROM symbols WHERE name = ?1",
                params![name],
                |row| row.get(0),
            )
            .map_err(StorageError::from)?;
        Ok(timestamp as u64)
    }

    // ========================================================================
    // File Tracking Operations
    // ========================================================================

    /// Record a file's metadata for change tracking.
    ///
    /// # Arguments
    ///
    /// * `path` - File path relative to project root
    /// * `language` - Programming language of the file
    /// * `hash` - Content hash for change detection
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, `Err(StorageError)` otherwise.
    pub fn record_file(
        &self,
        path: &str,
        language: Language,
        hash: &str,
    ) -> Result<(), StorageError> {
        let conn = self.conn()?;
        let timestamp = Self::current_timestamp();

        conn.execute(
            r#"
            INSERT OR REPLACE INTO files (path, language, hash, indexed_at)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            params![path, language_to_str(language), hash, timestamp as i64],
        )
        .map_err(StorageError::from)?;

        Ok(())
    }

    /// Get the stored hash for a file.
    ///
    /// # Arguments
    ///
    /// * `path` - File path to look up
    ///
    /// # Returns
    ///
    /// `Ok(Some(hash))` if file is tracked, `Ok(None)` if unknown.
    pub fn get_file_hash(&self, path: &str) -> Result<Option<String>, StorageError> {
        let conn = self.conn()?;
        let hash: Option<String> = conn
            .query_row(
                "SELECT hash FROM files WHERE path = ?1",
                params![path],
                |row| row.get(0),
            )
            .optional()
            .map_err(StorageError::from)?;
        Ok(hash)
    }

    /// Get complete file info.
    ///
    /// # Arguments
    ///
    /// * `path` - File path to look up
    ///
    /// # Returns
    ///
    /// `Ok(Some(FileInfo))` if file is tracked, `Ok(None)` if unknown.
    pub fn get_file_info(&self, path: &str) -> Result<Option<FileInfo>, StorageError> {
        let conn = self.conn()?;
        let info = conn
            .query_row(
                "SELECT path, language, hash, indexed_at FROM files WHERE path = ?1",
                params![path],
                |row| {
                    Ok(FileInfo {
                        path: row.get(0)?,
                        language: str_to_language(row.get::<_, String>(1)?.as_str()),
                        hash: row.get(2)?,
                        indexed_at: row.get::<_, i64>(3)? as u64,
                    })
                },
            )
            .optional()
            .map_err(StorageError::from)?;
        Ok(info)
    }

    /// Check if a file needs to be re-indexed.
    ///
    /// Returns true if the file is unknown or its hash differs from the stored hash.
    ///
    /// # Arguments
    ///
    /// * `path` - File path to check
    /// * `current_hash` - Current content hash of the file
    ///
    /// # Returns
    ///
    /// `Ok(true)` if re-indexing is needed, `Ok(false)` if file is up-to-date.
    pub fn needs_reindex(&self, path: &str, current_hash: &str) -> Result<bool, StorageError> {
        match self.get_file_hash(path)? {
            Some(stored_hash) => Ok(stored_hash != current_hash),
            None => Ok(true), // Unknown file needs indexing
        }
    }

    /// Get all tracked files.
    ///
    /// # Returns
    ///
    /// `Ok(Vec<FileInfo>)` with all tracked files.
    pub fn get_all_tracked_files(&self) -> Result<Vec<FileInfo>, StorageError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare("SELECT path, language, hash, indexed_at FROM files ORDER BY path")
            .map_err(StorageError::from)?;

        let files = stmt
            .query_map([], |row| {
                Ok(FileInfo {
                    path: row.get(0)?,
                    language: str_to_language(row.get::<_, String>(1)?.as_str()),
                    hash: row.get(2)?,
                    indexed_at: row.get::<_, i64>(3)? as u64,
                })
            })
            .map_err(StorageError::from)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(files)
    }

    /// Remove file tracking information.
    ///
    /// # Arguments
    ///
    /// * `path` - File path to untrack
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful.
    pub fn remove_file_tracking(&self, path: &str) -> Result<(), StorageError> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM files WHERE path = ?1", params![path])
            .map_err(StorageError::from)?;
        Ok(())
    }

    // ========================================================================
    // Query Operations
    // ========================================================================

    /// Query symbols by exact name match.
    ///
    /// # Arguments
    ///
    /// * `name` - Exact symbol name to search for
    ///
    /// # Returns
    ///
    /// `Ok(Vec<Symbol>)` with matching symbols, sorted by file and line.
    pub fn query_by_name(&self, name: &str) -> Result<Vec<Symbol>, StorageError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                r#"
                SELECT name, type, visibility, file_path, line_start, line_end, signature, body, language
                FROM symbols
                WHERE name = ?1
                ORDER BY file_path, line_start
                "#,
            )
            .map_err(StorageError::from)?;

        let symbols = stmt
            .query_map(params![name], row_to_symbol)
            .map_err(StorageError::from)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(symbols)
    }

    /// Query symbols by name, case-insensitive.
    ///
    /// # Arguments
    ///
    /// * `name` - Symbol name to search for (case-insensitive)
    ///
    /// # Returns
    ///
    /// `Ok(Vec<Symbol>)` with matching symbols, sorted by file and line.
    pub fn query_by_name_case_insensitive(&self, name: &str) -> Result<Vec<Symbol>, StorageError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                r#"
                SELECT name, type, visibility, file_path, line_start, line_end, signature, body, language
                FROM symbols
                WHERE LOWER(name) = LOWER(?1)
                ORDER BY file_path, line_start
                "#,
            )
            .map_err(StorageError::from)?;

        let symbols = stmt
            .query_map(params![name], row_to_symbol)
            .map_err(StorageError::from)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(symbols)
    }

    /// Query symbols by type.
    ///
    /// # Arguments
    ///
    /// * `symbol_type` - Type of symbols to search for
    ///
    /// # Returns
    ///
    /// `Ok(Vec<Symbol>)` with matching symbols, sorted by file and line.
    pub fn query_by_type(&self, symbol_type: SymbolType) -> Result<Vec<Symbol>, StorageError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                r#"
                SELECT name, type, visibility, file_path, line_start, line_end, signature, body, language
                FROM symbols
                WHERE type = ?1
                ORDER BY file_path, line_start
                "#,
            )
            .map_err(StorageError::from)?;

        let symbols = stmt
            .query_map(params![symbol_type_to_str(symbol_type)], row_to_symbol)
            .map_err(StorageError::from)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(symbols)
    }

    /// Query symbols by file path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file
    ///
    /// # Returns
    ///
    /// `Ok(Vec<Symbol>)` with symbols from that file, sorted by line.
    pub fn query_by_file(&self, file_path: &str) -> Result<Vec<Symbol>, StorageError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                r#"
                SELECT name, type, visibility, file_path, line_start, line_end, signature, body, language
                FROM symbols
                WHERE file_path = ?1
                ORDER BY line_start
                "#,
            )
            .map_err(StorageError::from)?;

        let symbols = stmt
            .query_map(params![file_path], row_to_symbol)
            .map_err(StorageError::from)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(symbols)
    }

    /// Query symbols with combined filters.
    ///
    /// # Arguments
    ///
    /// * `filters` - Query filters to apply
    ///
    /// # Returns
    ///
    /// `Ok(Vec<Symbol>)` with matching symbols, sorted by file and line.
    pub fn query(&self, filters: QueryFilters) -> Result<Vec<Symbol>, StorageError> {
        let conn = self.conn()?;

        // Build dynamic WHERE clause
        let mut conditions: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref name) = filters.name {
            conditions.push(format!("name = ?{}", param_values.len() + 1));
            param_values.push(Box::new(name.clone()));
        }

        if let Some(ref name) = filters.name_case_insensitive {
            conditions.push(format!("LOWER(name) = LOWER(?{})", param_values.len() + 1));
            param_values.push(Box::new(name.clone()));
        }

        if let Some(symbol_type) = filters.symbol_type {
            conditions.push(format!("type = ?{}", param_values.len() + 1));
            param_values.push(Box::new(symbol_type_to_str(symbol_type).to_string()));
        }

        if let Some(ref file_path) = filters.file_path {
            conditions.push(format!("file_path = ?{}", param_values.len() + 1));
            param_values.push(Box::new(file_path.clone()));
        }

        if let Some(visibility) = filters.visibility {
            conditions.push(format!("visibility = ?{}", param_values.len() + 1));
            param_values.push(Box::new(visibility_to_str(visibility).to_string()));
        }

        if let Some(language) = filters.language {
            conditions.push(format!("language = ?{}", param_values.len() + 1));
            param_values.push(Box::new(language_to_str(language).to_string()));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            r#"
            SELECT name, type, visibility, file_path, line_start, line_end, signature, body, language
            FROM symbols
            {}
            ORDER BY file_path, line_start
            "#,
            where_clause
        );

        let mut stmt = conn.prepare(&sql).map_err(StorageError::from)?;

        // Convert params for rusqlite
        let params_refs: Vec<&dyn rusqlite::ToSql> =
            param_values.iter().map(|b| b.as_ref()).collect();

        let symbols = stmt
            .query_map(params_refs.as_slice(), row_to_symbol)
            .map_err(StorageError::from)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(symbols)
    }

    // ========================================================================
    // Error Handling / Recovery / Test Helpers
    // ========================================================================

    /// Verify database integrity using PRAGMA integrity_check.
    ///
    /// # Returns
    ///
    /// `Ok(true)` if database is valid, `Ok(false)` if corrupted.
    pub fn verify_integrity(&self) -> Result<bool, StorageError> {
        let conn = self.conn()?;
        let result: String = conn
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))
            .map_err(StorageError::from)?;
        Ok(result == "ok")
    }

    /// Execute raw SQL (for testing error handling).
    ///
    /// # Safety
    ///
    /// This method should only be used in tests.
    pub fn execute_raw_sql(&self, sql: &str) -> Result<(), StorageError> {
        let conn = self.conn()?;
        conn.execute(sql, []).map_err(StorageError::from)?;
        Ok(())
    }

    /// Begin an exclusive transaction (for testing locking behavior).
    pub fn begin_exclusive_transaction(&self) -> Result<(), StorageError> {
        let conn = self.conn()?;
        conn.execute("BEGIN EXCLUSIVE TRANSACTION", [])
            .map_err(StorageError::from)?;
        Ok(())
    }

    /// Rollback the current transaction.
    pub fn rollback_transaction(&self) -> Result<(), StorageError> {
        let conn = self.conn()?;
        conn.execute("ROLLBACK", []).map_err(StorageError::from)?;
        Ok(())
    }

    /// Insert symbols with simulated error (for testing transaction rollback).
    ///
    /// # Arguments
    ///
    /// * `symbols` - Symbols to insert
    /// * `simulate_error` - If true, simulate an error after inserting
    ///
    /// # Returns
    ///
    /// `Err(StorageError)` if `simulate_error` is true.
    pub fn insert_symbols_with_error_simulation(
        &self,
        symbols: &[Symbol],
        simulate_error: bool,
    ) -> Result<(), StorageError> {
        let mut conn = self.conn_mut()?;
        let timestamp = Self::current_timestamp();

        let tx = conn.transaction().map_err(StorageError::from)?;

        {
            let mut stmt = tx
                .prepare(
                    r#"
                    INSERT INTO symbols (name, type, visibility, file_path, line_start, line_end, signature, body, language, updated_at)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                    "#,
                )
                .map_err(StorageError::from)?;

            for symbol in symbols {
                stmt.execute(params![
                    symbol.name,
                    symbol_type_to_str(symbol.symbol_type),
                    symbol.visibility.map(visibility_to_str),
                    symbol.file_path,
                    symbol.line_start as i64,
                    symbol.line_end as i64,
                    symbol.signature,
                    symbol.body,
                    language_to_str(symbol.language),
                    timestamp as i64,
                ])
                .map_err(StorageError::from)?;
            }
        }

        if simulate_error {
            // Rollback instead of commit
            tx.rollback().map_err(StorageError::from)?;
            return Err(StorageError::QueryFailed(
                "Simulated error for testing".to_string(),
            ));
        }

        tx.commit().map_err(StorageError::from)?;
        Ok(())
    }
}

// ============================================================================
// Helper Functions for Type Conversion
// ============================================================================

/// Convert a SymbolType to its string representation.
fn symbol_type_to_str(st: SymbolType) -> &'static str {
    match st {
        SymbolType::Function => "Function",
        SymbolType::Class => "Class",
        SymbolType::Method => "Method",
        SymbolType::Variable => "Variable",
        SymbolType::Constant => "Constant",
        SymbolType::Import => "Import",
        SymbolType::Export => "Export",
    }
}

/// Convert a string to SymbolType.
fn str_to_symbol_type(s: &str) -> SymbolType {
    match s {
        "Function" => SymbolType::Function,
        "Class" => SymbolType::Class,
        "Method" => SymbolType::Method,
        "Variable" => SymbolType::Variable,
        "Constant" => SymbolType::Constant,
        "Import" => SymbolType::Import,
        "Export" => SymbolType::Export,
        _ => SymbolType::Function, // Default fallback
    }
}

/// Convert a Visibility to its string representation.
fn visibility_to_str(v: Visibility) -> &'static str {
    match v {
        Visibility::Public => "Public",
        Visibility::Private => "Private",
        Visibility::Protected => "Protected",
    }
}

/// Convert a string to Visibility.
fn str_to_visibility(s: &str) -> Option<Visibility> {
    match s {
        "Public" => Some(Visibility::Public),
        "Private" => Some(Visibility::Private),
        "Protected" => Some(Visibility::Protected),
        _ => None,
    }
}

/// Convert a Language to its string representation.
fn language_to_str(l: Language) -> &'static str {
    match l {
        Language::Python => "Python",
        Language::TypeScript => "TypeScript",
        Language::Rust => "Rust",
        Language::Markdown => "Markdown",
    }
}

/// Convert a string to Language.
fn str_to_language(s: &str) -> Language {
    match s {
        "Python" => Language::Python,
        "TypeScript" => Language::TypeScript,
        "Rust" => Language::Rust,
        "Markdown" => Language::Markdown,
        _ => Language::Rust, // Default fallback
    }
}

/// Convert a database row to a Symbol.
fn row_to_symbol(row: &rusqlite::Row) -> rusqlite::Result<Symbol> {
    let visibility_str: Option<String> = row.get(2)?;

    Ok(Symbol {
        name: row.get(0)?,
        symbol_type: str_to_symbol_type(row.get::<_, String>(1)?.as_str()),
        visibility: visibility_str.as_deref().and_then(str_to_visibility),
        file_path: row.get(3)?,
        line_start: row.get::<_, i64>(4)? as usize,
        line_end: row.get::<_, i64>(5)? as usize,
        signature: row.get(6)?,
        body: row.get(7)?,
        language: str_to_language(row.get::<_, String>(8)?.as_str()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_storage_error_display() {
        let err = StorageError::QueryFailed("test".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Query") || display.contains("query"));
    }

    #[test]
    fn test_symbol_type_roundtrip() {
        let types = [
            SymbolType::Function,
            SymbolType::Class,
            SymbolType::Method,
            SymbolType::Variable,
            SymbolType::Constant,
            SymbolType::Import,
            SymbolType::Export,
        ];

        for st in types {
            let s = symbol_type_to_str(st);
            let back = str_to_symbol_type(s);
            assert_eq!(st, back);
        }
    }

    #[test]
    fn test_visibility_roundtrip() {
        let visibilities = [
            Visibility::Public,
            Visibility::Private,
            Visibility::Protected,
        ];

        for v in visibilities {
            let s = visibility_to_str(v);
            let back = str_to_visibility(s);
            assert_eq!(Some(v), back);
        }
    }

    #[test]
    fn test_language_roundtrip() {
        let languages = [
            Language::Python,
            Language::TypeScript,
            Language::Rust,
            Language::Markdown,
        ];

        for l in languages {
            let s = language_to_str(l);
            let back = str_to_language(s);
            assert_eq!(l, back);
        }
    }
}
