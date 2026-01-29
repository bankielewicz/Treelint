//! Integration tests for STORY-006: Context Modes for Symbol Search Output Control
//!
//! This file serves as the entry point for STORY-006 acceptance criteria tests.
//! Tests are organized by acceptance criteria in the STORY-006 subdirectory.
//!
//! TDD Phase: RED - These tests should FAIL until context modes are implemented.
//!
//! Acceptance Criteria:
//! - AC#1: Line-Based Context Mode (--context N) - extracts N lines before/after symbol
//! - AC#2: Full Semantic Context Mode (--context full) - uses tree-sitter node boundaries
//! - AC#3: Signatures Only Mode (--signatures) - returns only signature, body is null/omitted
//! - AC#4: JSON Output Reflects Context Mode - query.context_mode field shows mode used
//! - AC#5: Context Modes Work with All Symbol Types - functions, classes, methods across languages
//! - AC#6: Default Behavior - no flags defaults to --context full
//!
//! Business Rules:
//! - BR-001: --signatures flag and --context flag are mutually exclusive
//! - BR-002: --context N must be a positive integer when using line mode

// Each test module is included using the #[path] attribute
// The path is relative to this file's location (tests/ directory)

// AC#1: Line-Based Context Mode (--context N)
#[path = "STORY-006/test_ac1_context_lines.rs"]
mod test_ac1_context_lines;

// AC#2: Full Semantic Context Mode (--context full)
#[path = "STORY-006/test_ac2_context_full.rs"]
mod test_ac2_context_full;

// AC#3: Signatures Only Mode (--signatures)
#[path = "STORY-006/test_ac3_signatures_mode.rs"]
mod test_ac3_signatures_mode;

// AC#4: JSON Output Reflects Context Mode
#[path = "STORY-006/test_ac4_json_context_mode.rs"]
mod test_ac4_json_context_mode;

// AC#5: Context Modes Work with All Symbol Types
#[path = "STORY-006/test_ac5_all_symbol_types.rs"]
mod test_ac5_all_symbol_types;

// AC#6: Default Behavior Without Context Flag
#[path = "STORY-006/test_ac6_default_behavior.rs"]
mod test_ac6_default_behavior;

// Business Rules Tests
#[path = "STORY-006/test_business_rules.rs"]
mod test_business_rules;
