//! Integration tests for STORY-001: Project Setup + CLI Skeleton
//!
//! This file serves as the entry point for STORY-001 acceptance criteria tests.
//! Tests are organized by acceptance criteria in the STORY-001 subdirectory.

// Each test module is included using the #[path] attribute
// The path is relative to this file's location (tests/ directory)

// AC#1: Cargo Project Compiles Successfully
#[path = "STORY-001/test_ac1_cargo_build.rs"]
mod test_ac1_cargo_build;

// AC#2: Search Command Parses All Required Arguments
#[path = "STORY-001/test_ac2_argument_parsing.rs"]
mod test_ac2_argument_parsing;

// AC#3: Help Text Displays Complete CLI Documentation
#[path = "STORY-001/test_ac3_help_text.rs"]
mod test_ac3_help_text;

// AC#4: Search Command Returns Placeholder JSON Output
#[path = "STORY-001/test_ac4_placeholder_output.rs"]
mod test_ac4_placeholder_output;

// AC#5: Error Types Defined Using thiserror
#[path = "STORY-001/test_ac5_error_types.rs"]
mod test_ac5_error_types;
