//! JSON output formatting for search results
//!
//! This module defines the JSON schema for search command output.

use serde::{Deserialize, Serialize};

/// Search output structure matching the expected JSON schema.
///
/// Schema:
/// ```json
/// {
///   "query": { "symbol": "...", "type": "..." },
///   "results": [...],
///   "stats": { "files_searched": 0, "elapsed_ms": 0 }
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchOutput {
    /// The query that was executed
    pub query: SearchQuery,
    /// The search results (empty for placeholder)
    pub results: Vec<SearchResult>,
    /// Statistics about the search operation
    pub stats: SearchStats,
}

/// Query information in the search output
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    /// The symbol that was searched for
    pub symbol: String,
    /// The symbol type filter, if specified
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_type: Option<String>,
}

/// A single search result
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    /// The type of symbol found
    #[serde(rename = "type")]
    pub symbol_type: String,
    /// The name of the symbol
    pub name: String,
    /// The file path containing the symbol
    pub file: String,
    /// Line range [start, end]
    pub lines: (usize, usize),
    /// The function/method signature
    pub signature: String,
    /// The full body of the symbol
    pub body: String,
}

/// Statistics about the search operation
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchStats {
    /// Number of files searched
    pub files_searched: u64,
    /// Time elapsed in milliseconds
    pub elapsed_ms: u64,
}

impl SearchOutput {
    /// Create a placeholder output for testing/initial implementation.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol that was searched for
    /// * `symbol_type` - Optional symbol type filter
    ///
    /// # Returns
    ///
    /// A placeholder SearchOutput with empty results and zero stats.
    pub fn placeholder(symbol: &str, symbol_type: Option<&str>) -> Self {
        SearchOutput {
            query: SearchQuery {
                symbol: symbol.to_string(),
                symbol_type: symbol_type.map(|s| s.to_string()),
            },
            results: Vec::new(),
            stats: SearchStats {
                files_searched: 0,
                elapsed_ms: 0,
            },
        }
    }
}
