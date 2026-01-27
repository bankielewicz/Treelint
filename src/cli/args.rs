//! CLI argument definitions using clap derive macros.
//!
//! This module defines all CLI arguments and subcommands for Treelint.

use clap::{Parser, Subcommand, ValueEnum};

/// Treelint 0.1.0 - AST-aware code search CLI
///
/// A code search tool that uses tree-sitter AST parsing to return
/// semantic code units (functions, classes) instead of raw text matches.
#[derive(Parser, Debug)]
#[command(name = "treelint")]
#[command(version, long_about = None)]
pub struct Args {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Search for symbols in the codebase
    Search(SearchArgs),
}

/// Arguments for the search command
#[derive(Parser, Debug)]
pub struct SearchArgs {
    /// Symbol name or pattern to search for
    #[arg(required = true, value_parser = non_empty_string)]
    pub symbol: String,

    /// Filter by symbol type
    #[arg(long = "type", value_enum)]
    pub symbol_type: Option<SymbolType>,

    /// Case-insensitive search
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Interpret symbol as a regex pattern
    #[arg(short = 'r', long = "regex")]
    pub regex: bool,

    /// Output format
    #[arg(long = "format", value_enum, default_value = "text")]
    pub format: OutputFormat,

    /// Number of context lines to include
    #[arg(long = "context", default_value = "0")]
    pub context: u32,

    /// Only return function/method signatures
    #[arg(long = "signatures")]
    pub signatures: bool,
}

/// Symbol types that can be searched
#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum SymbolType {
    /// Function definitions
    #[value(name = "function")]
    Function,
    /// Class definitions
    #[value(name = "class")]
    Class,
    /// Method definitions
    #[value(name = "method")]
    Method,
    /// Variable declarations
    #[value(name = "variable")]
    Variable,
    /// Constant declarations
    #[value(name = "constant")]
    Constant,
    /// Import statements
    #[value(name = "import")]
    Import,
    /// Export statements
    #[value(name = "export")]
    Export,
}

impl SymbolType {
    /// Convert to lowercase string for JSON output
    pub fn as_str(&self) -> &'static str {
        match self {
            SymbolType::Function => "function",
            SymbolType::Class => "class",
            SymbolType::Method => "method",
            SymbolType::Variable => "variable",
            SymbolType::Constant => "constant",
            SymbolType::Import => "import",
            SymbolType::Export => "export",
        }
    }
}

/// Output format options
#[derive(ValueEnum, Clone, Debug, Default, PartialEq)]
pub enum OutputFormat {
    /// JSON output format
    #[value(name = "json")]
    Json,
    /// Human-readable text output
    #[default]
    #[value(name = "text")]
    Text,
}

/// Validator function to reject empty strings
fn non_empty_string(s: &str) -> Result<String, String> {
    if s.is_empty() {
        Err("symbol cannot be empty".to_string())
    } else {
        Ok(s.to_string())
    }
}
