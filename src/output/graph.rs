//! Graph output formatting (Mermaid diagrams).
//!
//! This module provides formatters for outputting dependency graphs
//! in Mermaid diagram format.

use crate::graph::{CallGraph, ImportGraph};

/// Format a call graph as a Mermaid diagram.
///
/// Produces a top-down directed graph showing function call relationships.
///
/// # Example Output
///
/// ```text
/// graph TD
///     caller["caller<br/>example.py"] --> callee["callee<br/>example.py"]
/// ```
pub fn format_call_graph_mermaid(graph: &CallGraph) -> String {
    let mut output = String::from("graph TD\n");

    // Create a mapping from node IDs to sanitized Mermaid IDs
    let mut node_ids: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for (i, node) in graph.nodes.iter().enumerate() {
        let mermaid_id = format!("n{}", i);
        node_ids.insert(node.id.clone(), mermaid_id.clone());

        // Create node with label showing name and file
        let label = format!("{}<br/>{}", node.name, sanitize_path(&node.file));
        output.push_str(&format!("    {}[\"{}\"]\n", mermaid_id, label));
    }

    // Add edges
    for edge in &graph.edges {
        if let (Some(from_id), Some(to_id)) = (node_ids.get(&edge.from), node_ids.get(&edge.to)) {
            if edge.count > 1 {
                // Add edge with count label
                output.push_str(&format!("    {} -->|{}| {}\n", from_id, edge.count, to_id));
            } else {
                output.push_str(&format!("    {} --> {}\n", from_id, to_id));
            }
        }
    }

    output
}

/// Format an import graph as a Mermaid diagram.
///
/// Produces a left-to-right directed graph showing import relationships.
///
/// # Example Output
///
/// ```text
/// graph LR
///     main["main.py"] --> utils["utils.py"]
/// ```
pub fn format_import_graph_mermaid(graph: &ImportGraph) -> String {
    let mut output = String::from("graph LR\n");

    // Create a mapping from node IDs to sanitized Mermaid IDs
    let mut node_ids: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for (i, node) in graph.nodes.iter().enumerate() {
        let mermaid_id = format!("n{}", i);
        node_ids.insert(node.id.clone(), mermaid_id.clone());

        // Create node with label showing file name
        let label = sanitize_path(&node.file);
        output.push_str(&format!("    {}[\"{}\"]\n", mermaid_id, label));
    }

    // Track which external modules we've already added
    let mut external_ids: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut external_counter = graph.nodes.len();

    // Add edges
    for edge in &graph.edges {
        let from_id = node_ids.get(&edge.from).cloned();

        let to_id = if let Some(id) = node_ids.get(&edge.to) {
            Some(id.clone())
        } else {
            // Create a node for external module if not already created
            if let Some(id) = external_ids.get(&edge.to) {
                Some(id.clone())
            } else {
                let mermaid_id = format!("n{}", external_counter);
                external_counter += 1;
                external_ids.insert(edge.to.clone(), mermaid_id.clone());

                // Add node definition for external module
                let label = sanitize_path(&edge.to);
                output = output.replace(
                    "graph LR\n",
                    &format!("graph LR\n    {}[\"{}\"]\n", mermaid_id, label),
                );

                Some(mermaid_id)
            }
        };

        if let (Some(from), Some(to)) = (from_id, to_id) {
            output.push_str(&format!("    {} --> {}\n", from, to));
        }
    }

    output
}

/// Sanitize a file path for Mermaid labels.
///
/// Removes problematic characters and shortens long paths.
fn sanitize_path(path: &str) -> String {
    // Get just the filename or last part of the path
    let short = std::path::Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(path);

    // Replace characters that might cause issues in Mermaid
    short
        .replace('"', "'")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{CallEdge, GraphNode, ImportEdge};

    #[test]
    fn test_call_graph_mermaid_basic() {
        let graph = CallGraph {
            graph_type: "calls".to_string(),
            nodes: vec![
                GraphNode {
                    id: "example.py:caller".to_string(),
                    name: "caller".to_string(),
                    file: "example.py".to_string(),
                    node_type: "function".to_string(),
                },
                GraphNode {
                    id: "example.py:callee".to_string(),
                    name: "callee".to_string(),
                    file: "example.py".to_string(),
                    node_type: "function".to_string(),
                },
            ],
            edges: vec![CallEdge {
                from: "example.py:caller".to_string(),
                to: "example.py:callee".to_string(),
                count: 1,
            }],
        };

        let mermaid = format_call_graph_mermaid(&graph);
        assert!(mermaid.starts_with("graph TD"));
        assert!(mermaid.contains("caller"));
        assert!(mermaid.contains("callee"));
        assert!(mermaid.contains("-->"));
    }

    #[test]
    fn test_import_graph_mermaid_basic() {
        let graph = ImportGraph {
            graph_type: "imports".to_string(),
            nodes: vec![
                GraphNode {
                    id: "main.py".to_string(),
                    name: "main".to_string(),
                    file: "main.py".to_string(),
                    node_type: "file".to_string(),
                },
                GraphNode {
                    id: "utils.py".to_string(),
                    name: "utils".to_string(),
                    file: "utils.py".to_string(),
                    node_type: "file".to_string(),
                },
            ],
            edges: vec![ImportEdge {
                from: "main.py".to_string(),
                to: "utils.py".to_string(),
                import_type: Some("from".to_string()),
            }],
        };

        let mermaid = format_import_graph_mermaid(&graph);
        assert!(mermaid.starts_with("graph LR"));
        assert!(mermaid.contains("main.py"));
        assert!(mermaid.contains("utils.py"));
        assert!(mermaid.contains("-->"));
    }

    #[test]
    fn test_call_graph_mermaid_with_count() {
        let graph = CallGraph {
            graph_type: "calls".to_string(),
            nodes: vec![
                GraphNode {
                    id: "a".to_string(),
                    name: "caller".to_string(),
                    file: "test.py".to_string(),
                    node_type: "function".to_string(),
                },
                GraphNode {
                    id: "b".to_string(),
                    name: "helper".to_string(),
                    file: "test.py".to_string(),
                    node_type: "function".to_string(),
                },
            ],
            edges: vec![CallEdge {
                from: "a".to_string(),
                to: "b".to_string(),
                count: 3,
            }],
        };

        let mermaid = format_call_graph_mermaid(&graph);
        assert!(mermaid.contains("|3|"));
    }
}
