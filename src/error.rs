//! Error types for Treelint
//!
//! This module defines custom error types using thiserror for
//! ergonomic error handling throughout the application.

use thiserror::Error;

/// Main error type for Treelint operations.
///
/// This enum captures all possible error conditions that can occur
/// during CLI execution, parsing, and I/O operations.
#[derive(Debug, Error)]
pub enum TreelintError {
    /// I/O error wrapper for file system operations.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Parse error for tree-sitter or other parsing failures.
    #[error("Parse error: {0}")]
    Parse(String),

    /// CLI error for argument validation or command execution failures.
    #[error("CLI error: {0}")]
    Cli(String),

    /// I/O error (string variant for daemon module).
    #[error("I/O error: {0}")]
    IoError(String),

    /// Parse error (string variant for daemon module).
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Daemon error for IPC and daemon-related failures.
    #[error("Daemon error: {0}")]
    DaemonError(String),
}
