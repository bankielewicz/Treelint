//! JSON output formatting for search results
//!
//! This module defines the JSON schema for search command output,
//! including query metadata, results array, and search statistics.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Search output structure matching the expected JSON schema.
///
/// Schema:
/// ```json
/// {
///   "query": { "symbol": "...", "type": "...", "context_mode": "full" },
///   "results": [{ "type": "...", "name": "...", "body": "..." }],
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
    /// Whether case-insensitive search was used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_insensitive: Option<bool>,
    /// Whether regex search was used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<bool>,
    /// The context mode: "full" (default) or "signatures"
    pub context_mode: String,
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
    /// The full body of the symbol (null in signatures mode)
    pub body: Option<String>,
    /// The programming language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Statistics about the search operation
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchStats {
    /// Number of files searched
    pub files_searched: u64,
    /// Time elapsed in milliseconds
    pub elapsed_ms: u64,
    /// Number of files skipped during search
    pub files_skipped: u64,
    /// Count of files skipped by type (e.g., binary, unsupported)
    pub skipped_by_type: HashMap<String, u64>,
    /// Languages that were searched
    pub languages_searched: Vec<String>,
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
                case_insensitive: None,
                regex: None,
                context_mode: "full".to_string(),
            },
            results: Vec::new(),
            stats: SearchStats {
                files_searched: 0,
                elapsed_ms: 0,
                files_skipped: 0,
                skipped_by_type: HashMap::new(),
                languages_searched: Vec::new(),
            },
        }
    }

    /// Create a new search output from pre-built components.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query metadata
    /// * `results` - The search results
    /// * `stats` - The search statistics
    pub fn new(query: SearchQuery, results: Vec<SearchResult>, stats: SearchStats) -> Self {
        SearchOutput {
            query,
            results,
            stats,
        }
    }
}
