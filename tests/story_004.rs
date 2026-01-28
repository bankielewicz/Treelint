//! Integration tests for STORY-004: Search Command Logic
//!
//! This file serves as the entry point for STORY-004 acceptance criteria tests.
//! Tests are organized by acceptance criteria in the STORY-004 subdirectory.
//!
//! TDD Phase: RED - These tests should FAIL until search command logic is implemented.
//!
//! Acceptance Criteria:
//! - AC#1: Basic Exact Match Search Executes Against Index
//! - AC#2: Type Filtering Narrows Results
//! - AC#3: Case-Insensitive Search Works
//! - AC#4: Regex Pattern Search Works
//! - AC#5: Auto-Indexing on First Search When Index Missing

// Each test module is included using the #[path] attribute
// The path is relative to this file's location (tests/ directory)

// AC#1: Basic Exact Match Search Executes Against Index
#[path = "STORY-004/test_ac1_exact_match.rs"]
mod test_ac1_exact_match;

// AC#2: Type Filtering Narrows Results
#[path = "STORY-004/test_ac2_type_filter.rs"]
mod test_ac2_type_filter;

// AC#3: Case-Insensitive Search Works
#[path = "STORY-004/test_ac3_case_insensitive.rs"]
mod test_ac3_case_insensitive;

// AC#4: Regex Pattern Search Works
#[path = "STORY-004/test_ac4_regex_search.rs"]
mod test_ac4_regex_search;

// AC#5: Auto-Indexing on First Search When Index Missing
#[path = "STORY-004/test_ac5_auto_index.rs"]
mod test_ac5_auto_index;
