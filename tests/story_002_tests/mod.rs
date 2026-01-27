//! STORY-002: tree-sitter Parser Integration Tests
//!
//! Test modules for verifying tree-sitter parser integration with embedded grammars.
//! These tests validate all 6 acceptance criteria for the parser module.

mod test_ac1_grammar_loading;
mod test_ac2_language_detection;
mod test_ac3_function_extraction;
mod test_ac4_class_method_extraction;
mod test_ac5_variable_import_extraction;
mod test_ac6_error_handling;
