//! STORY-003: SQLite Index Storage - Integration Test Entry Point
//!
//! This file serves as the entry point for running all STORY-003 acceptance
//! criteria tests. Each AC is implemented in a separate module under
//! tests/STORY-003/.
//!
//! # Running Tests
//!
//! ```bash
//! # Run all STORY-003 tests
//! cargo test --test story_003
//!
//! # Run specific AC tests
//! cargo test --test story_003 ac1  # Schema creation
//! cargo test --test story_003 ac2  # Symbol CRUD
//! cargo test --test story_003 ac3  # File tracking
//! cargo test --test story_003 ac4  # Query operations
//! cargo test --test story_003 ac5  # Error handling
//! ```
//!
//! # Test Organization
//!
//! - **test_ac1_schema_creation.rs**: Database initialization, table/index creation, PRAGMA settings
//! - **test_ac2_symbol_crud.rs**: Insert, bulk insert, upsert, delete operations
//! - **test_ac3_file_tracking.rs**: File metadata storage, hash comparison, reindex detection
//! - **test_ac4_query_operations.rs**: Name/type/file queries, combined filters, performance
//! - **test_ac5_error_handling.rs**: StorageError variants, graceful error handling, no panics

#[path = "STORY-003/mod.rs"]
mod story_003_tests;

// Re-export all test modules for test discovery
pub use story_003_tests::*;
