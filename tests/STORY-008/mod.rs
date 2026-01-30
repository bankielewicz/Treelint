//! STORY-008: File Watcher and Incremental Index Updates Tests
//!
//! This module contains test suites for verifying the file watcher
//! functionality, including initialization, event detection, incremental
//! indexing, hash-based change detection, and error recovery.
//!
//! # Test Organization
//!
//! - AC#1: File Watcher Initialization
//! - AC#2: File Modification Detection
//! - AC#3: File Creation Detection
//! - AC#4: File Deletion Detection
//! - AC#5: Incremental Index Update
//! - AC#6: Hash-Based Change Detection
//! - AC#7: Watcher Error Recovery
//!
//! # Dependencies
//!
//! - notify for cross-platform file watching
//! - tempfile for isolated test directories
//! - std::time for timing validation

pub mod test_ac1_watcher_initialization;
pub mod test_ac2_modification_detection;
pub mod test_ac3_creation_detection;
pub mod test_ac4_deletion_detection;
pub mod test_ac5_incremental_update;
pub mod test_ac6_hash_change_detection;
pub mod test_ac7_error_recovery;
