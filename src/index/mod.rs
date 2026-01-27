//! SQLite-based index storage for Treelint symbols.
//!
//! This module provides persistent storage for extracted symbols using SQLite,
//! enabling fast queries without re-parsing source files.
//!
//! # Architecture
//!
//! The index module is part of the Domain Layer and provides:
//! - Symbol storage and retrieval
//! - File tracking for change detection
//! - Query operations with filtering
//!
//! # Usage
//!
//! ```no_run
//! use std::path::Path;
//! use treelint::index::{IndexStorage, QueryFilters, SCHEMA_VERSION};
//! use treelint::parser::{Symbol, SymbolType};
//!
//! // Create or open the index
//! let storage = IndexStorage::new(Path::new("/path/to/project"))?;
//!
//! // Query symbols
//! let functions = storage.query_by_type(SymbolType::Function)?;
//! let specific = storage.query_by_name("validateUser")?;
//!
//! // Combined filters
//! let filters = QueryFilters::new()
//!     .with_type(SymbolType::Function)
//!     .with_file("src/auth.rs");
//! let results = storage.query(filters)?;
//! # Ok::<(), treelint::index::StorageError>(())
//! ```

pub mod schema;
pub mod search;
pub mod storage;

pub use schema::SCHEMA_VERSION;
pub use search::QueryFilters;
pub use storage::{FileInfo, IndexStorage, StorageError};
