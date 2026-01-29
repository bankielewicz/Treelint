//! Output formatting module for Treelint
//!
//! This module provides output formatters for search results
//! in various formats (JSON, text) with automatic TTY detection
//! for selecting the appropriate format.

pub mod json;
pub mod text;

pub use json::SearchOutput;

use crate::cli::args::OutputFormat;

/// Output format router that selects the appropriate formatter
/// based on TTY detection and explicit format flags.
///
/// When no explicit format is specified:
/// - TTY (terminal): Text format with human-readable output
/// - Non-TTY (piped): JSON format for programmatic consumption
///
/// When an explicit `--format` flag is provided, it overrides
/// TTY auto-detection.
pub struct OutputRouter {
    /// Whether stdout is a TTY terminal
    is_tty: bool,
}

impl Default for OutputRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputRouter {
    /// Create a new OutputRouter with auto-detected TTY status.
    ///
    /// Uses `atty::is(atty::Stream::Stdout)` to detect whether
    /// stdout is connected to a terminal.
    pub fn new() -> Self {
        Self {
            is_tty: atty::is(atty::Stream::Stdout),
        }
    }

    /// Create an OutputRouter with explicit TTY status (for testing).
    ///
    /// # Arguments
    ///
    /// * `is_tty` - Whether to treat stdout as a TTY
    pub fn with_tty(is_tty: bool) -> Self {
        Self { is_tty }
    }

    /// Returns whether stdout is a TTY terminal.
    ///
    /// Used to enable colors in text output when running in a terminal.
    pub fn is_tty(&self) -> bool {
        self.is_tty
    }

    /// Resolve the output format based on TTY status and explicit override.
    ///
    /// # Arguments
    ///
    /// * `explicit` - An explicitly specified format flag, or None for auto-detection
    ///
    /// # Returns
    ///
    /// The resolved output format to use.
    pub fn resolve_format(&self, explicit: Option<OutputFormat>) -> OutputFormat {
        match explicit {
            Some(format) => format,
            None => {
                if self.is_tty {
                    OutputFormat::Text
                } else {
                    OutputFormat::Json
                }
            }
        }
    }
}
