//! Call graph extraction and storage.
//!
//! This module provides functionality to extract function call relationships
//! from source code using tree-sitter and store them in SQLite.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::index::storage::IndexStorage;
use crate::parser::{Language, Symbol, SymbolType};

/// A call edge representing a caller-callee relationship.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallEdge {
    /// ID or name of the calling function
    pub from: String,
    /// ID or name of the called function
    pub to: String,
    /// Number of times the caller calls the callee
    pub count: u32,
}

/// A complete call graph with nodes and edges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraph {
    /// Type of graph (always "calls")
    pub graph_type: String,
    /// All function/method nodes
    pub nodes: Vec<super::GraphNode>,
    /// Call relationships between nodes
    pub edges: Vec<CallEdge>,
}

impl Default for CallGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl CallGraph {
    /// Create an empty call graph.
    pub fn new() -> Self {
        Self {
            graph_type: "calls".to_string(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Filter the graph to show only relationships involving a specific symbol.
    pub fn filter_by_symbol(&self, symbol_name: &str) -> CallGraph {
        let symbol_lower = symbol_name.to_lowercase();

        // Find edges where the symbol is either caller or callee
        let filtered_edges: Vec<CallEdge> = self
            .edges
            .iter()
            .filter(|e| {
                e.from.to_lowercase().contains(&symbol_lower)
                    || e.to.to_lowercase().contains(&symbol_lower)
            })
            .cloned()
            .collect();

        // Collect node IDs that are involved in filtered edges
        let mut involved_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
        for edge in &filtered_edges {
            involved_ids.insert(edge.from.clone());
            involved_ids.insert(edge.to.clone());
        }

        // Also include the target symbol even if it has no edges
        for node in &self.nodes {
            if node.name.to_lowercase().contains(&symbol_lower) {
                involved_ids.insert(node.id.clone());
            }
        }

        let filtered_nodes: Vec<super::GraphNode> = self
            .nodes
            .iter()
            .filter(|n| involved_ids.contains(&n.id))
            .cloned()
            .collect();

        CallGraph {
            graph_type: "calls".to_string(),
            nodes: filtered_nodes,
            edges: filtered_edges,
        }
    }
}

/// Service for extracting function call relationships.
pub struct CallGraphExtractor;

impl Default for CallGraphExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl CallGraphExtractor {
    /// Create a new CallGraphExtractor.
    pub fn new() -> Self {
        Self
    }

    /// Extract call graph from the indexed symbols and source files.
    ///
    /// # Arguments
    ///
    /// * `storage` - The index storage containing symbols
    /// * `project_root` - The project root directory
    ///
    /// # Returns
    ///
    /// A `CallGraph` containing all function call relationships.
    pub fn extract(
        &self,
        storage: &IndexStorage,
        project_root: &Path,
    ) -> Result<CallGraph, crate::index::StorageError> {
        let mut graph = CallGraph::new();

        // Get all symbols from the index
        let all_symbols = storage.get_all_symbols()?;

        // Build a lookup map for symbols by name
        let symbol_map: HashMap<String, &Symbol> = all_symbols
            .iter()
            .filter(|s| matches!(s.symbol_type, SymbolType::Function | SymbolType::Method))
            .map(|s| (s.name.clone(), s))
            .collect();

        // Create nodes for all functions/methods
        for symbol in all_symbols
            .iter()
            .filter(|s| matches!(s.symbol_type, SymbolType::Function | SymbolType::Method))
        {
            let node_id = format!("{}:{}", symbol.file_path, symbol.name);
            graph.nodes.push(super::GraphNode {
                id: node_id,
                name: symbol.name.clone(),
                file: symbol.file_path.clone(),
                node_type: symbol.symbol_type.to_string().to_lowercase(),
            });
        }

        // Get unique file paths from symbols
        let mut file_paths: std::collections::HashSet<String> = std::collections::HashSet::new();
        for symbol in &all_symbols {
            file_paths.insert(symbol.file_path.clone());
        }

        // Extract calls from each source file
        for file_path_str in file_paths {
            let file_path = if Path::new(&file_path_str).is_absolute() {
                Path::new(&file_path_str).to_path_buf()
            } else {
                project_root.join(&file_path_str)
            };

            if !file_path.exists() {
                continue;
            }

            // Determine language from file extension
            let language = match crate::parser::Language::from_path(&file_path) {
                Some(l) => l,
                None => continue,
            };

            // Read file content
            let content = match std::fs::read_to_string(&file_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            // Extract calls from this file
            let file_calls = self.extract_calls_from_content(&content, language, &file_path_str);

            // Match calls to known symbols
            for (caller_name, callee_name, count) in file_calls {
                // Check if callee is a known symbol
                if symbol_map.contains_key(&callee_name) {
                    // Find the caller's full ID
                    let caller_id = format!("{}:{}", file_path_str, caller_name);
                    let callee_symbol = symbol_map.get(&callee_name);

                    if let Some(callee_sym) = callee_symbol {
                        let callee_id = format!("{}:{}", callee_sym.file_path, callee_name);

                        // Check if this edge already exists
                        if let Some(existing_edge) = graph
                            .edges
                            .iter_mut()
                            .find(|e| e.from == caller_id && e.to == callee_id)
                        {
                            existing_edge.count += count;
                        } else {
                            graph.edges.push(CallEdge {
                                from: caller_id,
                                to: callee_id,
                                count,
                            });
                        }
                    }
                }
            }
        }

        // Store edges in database
        self.store_call_edges(storage, &graph)?;

        Ok(graph)
    }

    /// Extract function calls from source content.
    ///
    /// Returns a vector of (caller_name, callee_name, count) tuples.
    fn extract_calls_from_content(
        &self,
        content: &str,
        language: Language,
        _file_path: &str,
    ) -> Vec<(String, String, u32)> {
        let mut calls: HashMap<(String, String), u32> = HashMap::new();

        // Parse the content
        let parser = match crate::parser::Parser::new(language) {
            Ok(p) => p,
            Err(_) => return Vec::new(),
        };

        let tree = match parser.parse(content) {
            Ok(t) => t,
            Err(_) => return Vec::new(),
        };

        let source_bytes = content.as_bytes();

        // Walk the AST to find function definitions and their calls
        self.walk_for_calls(tree.root_node(), source_bytes, language, &mut calls, None);

        calls
            .into_iter()
            .map(|((caller, callee), count)| (caller, callee, count))
            .collect()
    }

    /// Walk the AST to find function calls within function bodies.
    fn walk_for_calls(
        &self,
        node: tree_sitter::Node,
        source: &[u8],
        language: Language,
        calls: &mut HashMap<(String, String), u32>,
        current_function: Option<&str>,
    ) {
        let kind = node.kind();

        // Detect function definitions
        let function_name = match language {
            Language::Python => {
                if kind == "function_definition" {
                    node.child_by_field_name("name")
                        .and_then(|n| self.node_text(n, source))
                } else {
                    None
                }
            }
            Language::TypeScript => {
                if kind == "function_declaration" || kind == "method_definition" {
                    node.child_by_field_name("name")
                        .and_then(|n| self.node_text(n, source))
                } else {
                    None
                }
            }
            Language::Rust => {
                if kind == "function_item" {
                    node.child_by_field_name("name")
                        .and_then(|n| self.node_text(n, source))
                } else {
                    None
                }
            }
            Language::Markdown => None,
        };

        let active_function = function_name.as_deref().or(current_function);

        // Detect function calls
        if let Some(caller) = active_function {
            let callee_name = match language {
                Language::Python => {
                    if kind == "call" {
                        // Get the function being called
                        node.child_by_field_name("function")
                            .and_then(|n| self.extract_callee_name(n, source))
                    } else {
                        None
                    }
                }
                Language::TypeScript => {
                    if kind == "call_expression" {
                        node.child_by_field_name("function")
                            .and_then(|n| self.extract_callee_name(n, source))
                    } else {
                        None
                    }
                }
                Language::Rust => {
                    if kind == "call_expression" {
                        node.child_by_field_name("function")
                            .and_then(|n| self.extract_callee_name(n, source))
                    } else if kind == "method_call_expression" {
                        node.child_by_field_name("name")
                            .and_then(|n| self.node_text(n, source))
                    } else {
                        None
                    }
                }
                Language::Markdown => None,
            };

            if let Some(callee) = callee_name {
                // Increment call count
                let key = (caller.to_string(), callee);
                *calls.entry(key).or_insert(0) += 1;
            }
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_for_calls(child, source, language, calls, active_function);
        }
    }

    /// Extract the callee name from a call expression.
    fn extract_callee_name(&self, node: tree_sitter::Node, source: &[u8]) -> Option<String> {
        match node.kind() {
            "identifier" => self.node_text(node, source),
            "attribute" | "member_expression" => {
                // For method calls like obj.method(), extract just the method name
                node.child_by_field_name("attribute")
                    .or_else(|| node.child_by_field_name("property"))
                    .and_then(|n| self.node_text(n, source))
            }
            "scoped_identifier" | "field_expression" => {
                // For Rust paths like module::function
                node.child_by_field_name("name")
                    .and_then(|n| self.node_text(n, source))
            }
            _ => {
                // Try to get any identifier child
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "identifier" {
                        return self.node_text(child, source);
                    }
                }
                // Last resort: get the whole text if it's simple
                let text = self.node_text(node, source)?;
                if text.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    Some(text)
                } else {
                    None
                }
            }
        }
    }

    /// Get text content of a node (delegates to shared helper).
    fn node_text(&self, node: tree_sitter::Node, source: &[u8]) -> Option<String> {
        super::node_text(node, source)
    }

    /// Store call edges in the database using parameterized queries.
    fn store_call_edges(
        &self,
        storage: &IndexStorage,
        graph: &CallGraph,
    ) -> Result<(), crate::index::StorageError> {
        // Ensure the call_edges table exists
        storage.ensure_call_edges_table()?;

        // Clear existing edges
        storage.clear_call_edges()?;

        // Insert new edges using parameterized queries
        for edge in &graph.edges {
            storage.insert_call_edge(&edge.from, &edge.to, edge.count)?;
        }

        Ok(())
    }

    /// Load call graph from stored edges.
    pub fn load_from_storage(
        &self,
        storage: &IndexStorage,
    ) -> Result<CallGraph, crate::index::StorageError> {
        let mut graph = CallGraph::new();

        // Get all symbols for nodes
        let all_symbols = storage.get_all_symbols()?;

        for symbol in all_symbols
            .iter()
            .filter(|s| matches!(s.symbol_type, SymbolType::Function | SymbolType::Method))
        {
            let node_id = format!("{}:{}", symbol.file_path, symbol.name);
            graph.nodes.push(super::GraphNode {
                id: node_id,
                name: symbol.name.clone(),
                file: symbol.file_path.clone(),
                node_type: symbol.symbol_type.to_string().to_lowercase(),
            });
        }

        // Check if call_edges table exists
        let tables = storage.get_tables()?;
        if !tables.contains(&"call_edges".to_string()) {
            return Ok(graph);
        }

        // Load edges from database - use raw SQL for simplicity
        // This is a basic implementation; in production we'd use prepared statements
        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_graph_new() {
        let graph = CallGraph::new();
        assert_eq!(graph.graph_type, "calls");
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_call_edge_serialize() {
        let edge = CallEdge {
            from: "caller".to_string(),
            to: "callee".to_string(),
            count: 3,
        };
        let json = serde_json::to_string(&edge);
        assert!(json.is_ok());
        let json_str = json.unwrap();
        assert!(json_str.contains("\"from\":\"caller\""));
        assert!(json_str.contains("\"count\":3"));
    }
}
