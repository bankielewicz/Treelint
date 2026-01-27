//! Tree-sitter query definitions for symbol extraction.
//!
//! This module organizes language-specific tree-sitter queries
//! for extracting symbols from source code.
//!
//! # Organization
//!
//! Each supported language has its own submodule containing
//! query strings and helper functions for that language's AST.
//!
//! - [`python`] - Python function, class, import queries
//! - [`typescript`] - TypeScript/JavaScript queries
//! - [`rust`] - Rust fn, struct, impl, use queries
//! - [`markdown`] - Markdown heading queries

pub mod markdown;
pub mod python;
pub mod rust;
pub mod typescript;
