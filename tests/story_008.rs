//! STORY-008: File Watcher and Incremental Index Updates - Integration Test Entry Point
//!
//! This file serves as the entry point for running all STORY-008 acceptance
//! criteria tests. Each AC is implemented in a separate module under
//! tests/STORY-008/.
//!
//! # Running Tests
//!
//! ```bash
//! # Run all STORY-008 tests
//! cargo test --test story_008
//!
//! # Run specific AC tests
//! cargo test --test story_008 ac1  # File watcher initialization
//! cargo test --test story_008 ac2  # File modification detection
//! cargo test --test story_008 ac3  # File creation detection
//! cargo test --test story_008 ac4  # File deletion detection
//! cargo test --test story_008 ac5  # Incremental index update
//! cargo test --test story_008 ac6  # Hash-based change detection
//! cargo test --test story_008 ac7  # Watcher error recovery
//! ```
//!
//! # Test Organization
//!
//! - **test_ac1_watcher_initialization.rs**: Recursive watch, .gitignore, extension filtering
//! - **test_ac2_modification_detection.rs**: Detect changes, queue for re-indexing, debounce
//! - **test_ac3_creation_detection.rs**: Detect new files, parse and add to index
//! - **test_ac4_deletion_detection.rs**: Detect deletions, remove from index
//! - **test_ac5_incremental_update.rs**: Single file re-parse, < 1 second
//! - **test_ac6_hash_change_detection.rs**: SHA-256, skip if unchanged
//! - **test_ac7_error_recovery.rs**: Log errors, continue operating

#[path = "STORY-008/mod.rs"]
mod story_008_tests;

// Re-export all test modules for test discovery
pub use story_008_tests::*;
