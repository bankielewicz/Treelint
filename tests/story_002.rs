//! Integration test entry point for STORY-002: tree-sitter Parser Integration
//!
//! This file serves as the integration test entry point that references
//! the STORY-002 test modules.
//!
//! To run these tests:
//!   cargo test --test story_002
//!
//! Note: These tests are designed to FAIL initially (TDD Red phase).
//! They will pass once the parser module is implemented.

// Include the STORY-002 test modules
#[path = "story_002_tests/test_ac1_grammar_loading.rs"]
mod test_ac1_grammar_loading;

#[path = "story_002_tests/test_ac2_language_detection.rs"]
mod test_ac2_language_detection;

#[path = "story_002_tests/test_ac3_function_extraction.rs"]
mod test_ac3_function_extraction;

#[path = "story_002_tests/test_ac4_class_method_extraction.rs"]
mod test_ac4_class_method_extraction;

#[path = "story_002_tests/test_ac5_variable_import_extraction.rs"]
mod test_ac5_variable_import_extraction;

#[path = "story_002_tests/test_ac6_error_handling.rs"]
mod test_ac6_error_handling;
