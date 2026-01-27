//! Search command implementation
//!
//! This module implements the `treelint search` command which searches
//! for symbols in the codebase.

use crate::cli::args::{OutputFormat, SearchArgs};
use crate::output::json::SearchOutput;

/// Execute the search command with the given arguments.
///
/// # Arguments
///
/// * `args` - The parsed search command arguments
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the search fails.
pub fn execute(args: SearchArgs) -> anyhow::Result<()> {
    match args.format {
        OutputFormat::Json => {
            let output = SearchOutput::placeholder(
                &args.symbol,
                args.symbol_type.as_ref().map(|t| t.as_str()),
            );
            let json = serde_json::to_string_pretty(&output)?;
            println!("{}", json);
        }
        OutputFormat::Text => {
            println!("No results found for: {}", args.symbol);
        }
    }

    Ok(())
}
