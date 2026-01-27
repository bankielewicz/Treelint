//! Treelint - AST-aware code search library
//!
//! This library provides the core functionality for Treelint, an AST-aware
//! code search tool that uses tree-sitter to return semantic code units
//! (functions, classes) instead of raw text matches.

pub mod cli;
pub mod error;
pub mod output;

pub use error::TreelintError;
