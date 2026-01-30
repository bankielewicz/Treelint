//! STORY-012: Daemon-Index Integration - Integration Test Entry Point
//!
//! This file serves as the entry point for running all STORY-012 acceptance
//! criteria tests. Each AC is implemented in a separate module under
//! tests/STORY-012/.
//!
//! # Running Tests
//!
//! ```bash
//! # Run all STORY-012 tests
//! cargo test --test story_012
//!
//! # Run specific AC tests
//! cargo test --test story_012 ac1  # Daemon search returns actual results
//! cargo test --test story_012 ac2  # Daemon index triggers actual indexing
//! cargo test --test story_012 ac3  # Index force rebuild option
//! cargo test --test story_012 ac4  # Status reflects index state
//! ```
//!
//! # Test Organization
//!
//! - **test_ac1_daemon_search.rs**: Search request returns actual symbols from IndexStorage
//! - **test_ac2_daemon_index.rs**: Index request uses SymbolExtractor and stores in IndexStorage
//! - **test_ac3_force_index.rs**: Force=true clears and rebuilds index
//! - **test_ac4_status_accuracy.rs**: Status returns actual counts from IndexStorage
//!
//! # TDD Phase: RED
//!
//! These tests are designed to FAIL initially because:
//! - handle_search returns hardcoded empty results (line 974-982)
//! - handle_index returns stub response with 0 counts (line 985-996)
//! - Status counts come from atomic counters, not IndexStorage
//!
//! After implementation (TDD Green phase), all tests should pass.

#[path = "STORY-012/mod.rs"]
mod story_012_tests;

// Re-export all test modules for test discovery
pub use story_012_tests::*;
