//! Dependency graph analysis module.
//!
//! This module provides functionality to extract and analyze function call
//! relationships and import dependencies across the codebase.
//!
//! # Architecture
//!
//! The graph module is part of the Domain Layer and provides:
//! - Call graph extraction (function-to-function relationships)
//! - Import graph extraction (file-to-file relationships)
//! - Graph storage and retrieval from SQLite
//!
//! # Example
//!
//! ```no_run
//! use std::path::Path;
//! use treelint::graph::{CallGraphExtractor, ImportGraphExtractor};
//! use treelint::index::IndexStorage;
//!
//! let project_root = Path::new("/path/to/project");
//! let storage = IndexStorage::new(project_root)?;
//! let call_extractor = CallGraphExtractor::new();
//! let call_graph = call_extractor.extract(&storage, project_root)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod calls;
pub mod imports;

pub use calls::{CallEdge, CallGraph, CallGraphExtractor};
pub use imports::{ImportEdge, ImportGraph, ImportGraphExtractor};

use serde::{Deserialize, Serialize};

/// A node in a dependency graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphNode {
    /// Unique identifier for the node
    pub id: String,
    /// Display name of the node
    pub name: String,
    /// File path where the node is defined
    pub file: String,
    /// Type of node (function, method, class, file, etc.)
    #[serde(rename = "type")]
    pub node_type: String,
}

/// Type of graph being represented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GraphType {
    /// Function call relationships
    Calls,
    /// Import/module relationships
    Imports,
}

/// Shared utility: Get text content of an AST node.
///
/// # Arguments
///
/// * `node` - The tree-sitter node to extract text from
/// * `source` - The source code bytes
///
/// # Returns
///
/// `Some(String)` with the node's text if valid UTF-8, `None` otherwise.
pub fn node_text(node: tree_sitter::Node, source: &[u8]) -> Option<String> {
    std::str::from_utf8(&source[node.start_byte()..node.end_byte()])
        .ok()
        .map(|s| s.to_string())
}
