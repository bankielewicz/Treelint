//! STORY-011: Dependency Graph with Call and Import Extraction - Integration Test Entry Point
//!
//! This file serves as the entry point for running all STORY-011 acceptance
//! criteria tests. Each AC is implemented in a separate module under
//! tests/STORY-011/.
//!
//! # Running Tests
//!
//! ```bash
//! # Run all STORY-011 tests
//! cargo test --test story_011
//!
//! # Run specific AC tests
//! cargo test --test story_011 ac1  # Call graph command
//! cargo test --test story_011 ac2  # Import graph command
//! cargo test --test story_011 ac3  # JSON graph output
//! cargo test --test story_011 ac4  # Mermaid diagram output
//! cargo test --test story_011 ac5  # Symbol-specific graph
//! cargo test --test story_011 ac6  # Function call detection
//! cargo test --test story_011 ac7  # Import relationship detection
//! cargo test --test story_011 ac8  # Graph storage and caching
//! ```
//!
//! # Test Organization
//!
//! - **test_ac1_call_graph.rs**: `treelint deps --calls` command
//! - **test_ac2_import_graph.rs**: `treelint deps --imports` command
//! - **test_ac3_json_output.rs**: JSON graph format with schema validation
//! - **test_ac4_mermaid_output.rs**: Mermaid diagram syntax generation
//! - **test_ac5_symbol_filter.rs**: `--symbol` flag for graph filtering
//! - **test_ac6_call_detection.rs**: Tree-sitter call extraction (Python/TS/Rust)
//! - **test_ac7_import_detection.rs**: Tree-sitter import extraction
//! - **test_ac8_graph_storage.rs**: SQLite call_edges/import_edges tables
//!
//! # TDD Phase: RED
//!
//! These tests are designed to FAIL initially because:
//! - The `deps` subcommand does not exist yet in CLI args
//! - CallGraphExtractor service is not implemented
//! - ImportGraphExtractor service is not implemented
//! - MermaidFormatter does not exist
//! - call_edges and import_edges tables not in schema
//!
//! After implementation (TDD Green phase), all tests should pass.

#[path = "STORY-011/mod.rs"]
mod story_011_tests;

// Re-export all test modules for test discovery
pub use story_011_tests::*;
