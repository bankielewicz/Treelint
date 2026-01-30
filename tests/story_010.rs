//! STORY-010: Repository Map with Symbol Hierarchy and Relevance Scoring - Integration Test Entry Point
//!
//! This file serves as the entry point for running all STORY-010 acceptance
//! criteria tests. Each AC is implemented in a separate module under
//! tests/STORY-010/.
//!
//! # Running Tests
//!
//! ```bash
//! # Run all STORY-010 tests
//! cargo test --test story_010
//!
//! # Run specific AC tests
//! cargo test --test story_010 ac1  # Map command basic output
//! cargo test --test story_010 ac2  # JSON output format
//! cargo test --test story_010 ac3  # Text output format
//! cargo test --test story_010 ac4  # Symbol type filtering
//! cargo test --test story_010 ac5  # Relevance score calculation
//! cargo test --test story_010 ac6  # Ranked output flag
//! cargo test --test story_010 ac7  # Reference extraction
//! cargo test --test story_010 ac8  # Large repository performance
//! ```
//!
//! # Test Organization
//!
//! - **test_ac1_map_basic.rs**: Map command shows all symbols by file
//! - **test_ac2_json_output.rs**: JSON output matches schema
//! - **test_ac3_text_output.rs**: Tree structure text output
//! - **test_ac4_type_filter.rs**: --type flag filters symbols
//! - **test_ac5_relevance_score.rs**: Score calculation formula
//! - **test_ac6_ranked_output.rs**: --ranked sorts by relevance
//! - **test_ac7_reference_extraction.rs**: Function calls and imports detected
//! - **test_ac8_performance.rs**: < 10 seconds for 100K files
//!
//! # TDD Phase: RED
//!
//! These tests are designed to FAIL initially because:
//! - The `map` subcommand does not exist yet in CLI args
//! - The RelevanceScorer service is not implemented
//! - Reference extraction logic does not exist
//!
//! After implementation (TDD Green phase), all tests should pass.

#[path = "STORY-010/mod.rs"]
mod story_010_tests;

// Re-export all test modules for test discovery
pub use story_010_tests::*;
