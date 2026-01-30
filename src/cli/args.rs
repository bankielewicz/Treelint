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
    /// Manage the background daemon
    Daemon(DaemonArgs),
    /// Build or rebuild the symbol index
    Index(IndexArgs),
}

/// Arguments for the daemon command
#[derive(Parser, Debug)]
pub struct DaemonArgs {
    /// Daemon action to perform
    #[command(subcommand)]
    pub action: Option<DaemonAction>,

    /// Run the daemon server (internal use only)
    #[arg(long = "run-server", hide = true)]
    pub run_server: bool,
}

/// Available daemon actions
#[derive(Subcommand, Debug, Clone)]
pub enum DaemonAction {
    /// Start the background daemon
    Start,
    /// Stop the running daemon
    Stop,
    /// Show daemon status
    Status,
}

/// Arguments for the index command
#[derive(Parser, Debug)]
pub struct IndexArgs {
    /// Force full re-index, bypassing hash cache
    #[arg(short, long)]
    pub force: bool,
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

    /// Output format (auto-detected if not specified: TTY=text, pipe=json)
    #[arg(long = "format", value_enum)]
    pub format: Option<OutputFormat>,

    /// Context mode: number of lines (e.g., "5") or "full" for complete semantic unit.
    /// Default is "full" when not specified.
    /// Cannot be used with --signatures.
    #[arg(long = "context", conflicts_with = "signatures", value_parser = parse_context_value)]
    pub context: Option<String>,

    /// Only return function/method signatures (body is omitted).
    /// Cannot be used with --context.
    #[arg(long = "signatures", conflicts_with = "context")]
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
#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
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

/// Parse and validate the --context argument value.
///
/// Accepts:
/// - A positive integer (e.g., "5") for N lines of context
/// - "full" (case-insensitive) for complete semantic unit
///
/// Rejects:
/// - Zero ("0") - context lines must be positive per BR-002
/// - Negative numbers
/// - Non-numeric, non-"full" values
fn parse_context_value(s: &str) -> Result<String, String> {
    let trimmed = s.trim();

    // Accept "full" keyword (case-insensitive)
    if trimmed.eq_ignore_ascii_case("full") {
        return Ok(trimmed.to_lowercase());
    }

    // Try to parse as integer
    match trimmed.parse::<i64>() {
        Ok(n) if n <= 0 => Err(format!(
            "invalid value '{}' for '--context <CONTEXT>': context lines must be a positive integer (greater than 0). \
             Use '--context 1' or higher, or '--context full' for complete semantic unit.",
            s
        )),
        Ok(_) => Ok(trimmed.to_string()),
        Err(_) => Err(format!(
            "invalid value '{}' for '--context <CONTEXT>': expected a positive integer or 'full'. \
             Examples: '--context 5' for 5 lines, '--context full' for complete semantic unit.",
            s
        )),
    }
}
