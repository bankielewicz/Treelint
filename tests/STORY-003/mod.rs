//! STORY-003: SQLite Index Storage Tests
//!
//! This module contains test suites for verifying SQLite-based symbol
//! storage functionality, including schema creation, CRUD operations,
//! file tracking, query operations, and error handling.
//!
//! # Test Organization
//!
//! - AC#1: Database Initialization and Schema Creation
//! - AC#2: Symbol CRUD Operations
//! - AC#3: File Tracking for Change Detection
//! - AC#4: Symbol Query Operations
//! - AC#5: Error Handling and Recovery
//!
//! # Dependencies
//!
//! - rusqlite 0.31 with bundled feature
//! - tempfile for isolated test databases
//! - pretty_assertions for diff output

pub mod test_ac1_schema_creation;
pub mod test_ac2_symbol_crud;
pub mod test_ac3_file_tracking;
pub mod test_ac4_query_operations;
pub mod test_ac5_error_handling;
