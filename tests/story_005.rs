//! Integration tests for STORY-005: JSON/Text Output Formatting
//!
//! This file serves as the entry point for STORY-005 acceptance criteria tests.
//! Tests are organized by acceptance criteria in the STORY-005 subdirectory.
//!
//! TDD Phase: RED - These tests should FAIL until output formatting is implemented.
//!
//! Acceptance Criteria:
//! - AC#1: JSON Output Format Matches Schema
//! - AC#2: Text Output Format for Human Readability
//! - AC#3: TTY Auto-Detection
//! - AC#4: Format Flag Override
//! - AC#5: Signatures-Only Mode

// Each test module is included using the #[path] attribute
// The path is relative to this file's location (tests/ directory)

// AC#1: JSON Output Format Matches Schema
#[path = "STORY-005/test_ac1_json_format.rs"]
mod test_ac1_json_format;

// AC#2: Text Output Format for Human Readability
#[path = "STORY-005/test_ac2_text_format.rs"]
mod test_ac2_text_format;

// AC#3: TTY Auto-Detection
#[path = "STORY-005/test_ac3_tty_detection.rs"]
mod test_ac3_tty_detection;

// AC#4: Format Flag Override
#[path = "STORY-005/test_ac4_format_override.rs"]
mod test_ac4_format_override;

// AC#5: Signatures-Only Mode
#[path = "STORY-005/test_ac5_signatures_mode.rs"]
mod test_ac5_signatures_mode;
