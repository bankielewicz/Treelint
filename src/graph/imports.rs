//! Import graph extraction and storage.
//!
//! This module provides functionality to extract import/module relationships
//! from source code using tree-sitter and store them in SQLite.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::index::storage::IndexStorage;
use crate::parser::Language;

/// An import edge representing an importer-imported relationship.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportEdge {
    /// Path of the file that imports
    pub from: String,
    /// Path or name of the imported module/file
    pub to: String,
    /// Type of import (direct, from, star)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_type: Option<String>,
}

/// A complete import graph with nodes and edges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportGraph {
    /// Type of graph (always "imports")
    pub graph_type: String,
    /// All file/module nodes
    pub nodes: Vec<super::GraphNode>,
    /// Import relationships between nodes
    pub edges: Vec<ImportEdge>,
}

impl Default for ImportGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl ImportGraph {
    /// Create an empty import graph.
    pub fn new() -> Self {
        Self {
            graph_type: "imports".to_string(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Filter the graph to show only relationships involving a specific file/module.
    pub fn filter_by_symbol(&self, symbol_name: &str) -> ImportGraph {
        let symbol_lower = symbol_name.to_lowercase();

        // Find edges where the symbol is either importer or imported
        let filtered_edges: Vec<ImportEdge> = self
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
            if node.name.to_lowercase().contains(&symbol_lower)
                || node.file.to_lowercase().contains(&symbol_lower)
            {
                involved_ids.insert(node.id.clone());
            }
        }

        let filtered_nodes: Vec<super::GraphNode> = self
            .nodes
            .iter()
            .filter(|n| involved_ids.contains(&n.id))
            .cloned()
            .collect();

        ImportGraph {
            graph_type: "imports".to_string(),
            nodes: filtered_nodes,
            edges: filtered_edges,
        }
    }
}

/// Service for extracting import relationships.
pub struct ImportGraphExtractor;

impl Default for ImportGraphExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl ImportGraphExtractor {
    /// Create a new ImportGraphExtractor.
    pub fn new() -> Self {
        Self
    }

    /// Extract import graph from the indexed files.
    ///
    /// # Arguments
    ///
    /// * `storage` - The index storage containing file information
    /// * `project_root` - The project root directory
    ///
    /// # Returns
    ///
    /// An `ImportGraph` containing all import relationships.
    pub fn extract(
        &self,
        storage: &IndexStorage,
        project_root: &Path,
    ) -> Result<ImportGraph, crate::index::StorageError> {
        let mut graph = ImportGraph::new();
        let mut file_nodes: HashMap<String, bool> = HashMap::new();

        // Get all symbols to determine which files exist
        let all_symbols = storage.get_all_symbols()?;

        // Get unique file paths from symbols
        let mut file_paths: std::collections::HashSet<String> = std::collections::HashSet::new();
        for symbol in &all_symbols {
            file_paths.insert(symbol.file_path.clone());
        }

        // Add nodes for all files with symbols
        for file_path_str in &file_paths {
            let node_id = file_path_str.clone();
            let name = Path::new(file_path_str)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(file_path_str)
                .to_string();

            graph.nodes.push(super::GraphNode {
                id: node_id.clone(),
                name,
                file: file_path_str.clone(),
                node_type: "file".to_string(),
            });

            file_nodes.insert(node_id, true);
        }

        // Extract imports from each file
        for file_path_str in &file_paths {
            let file_path = if Path::new(file_path_str).is_absolute() {
                Path::new(file_path_str).to_path_buf()
            } else {
                project_root.join(file_path_str)
            };

            if !file_path.exists() {
                continue;
            }

            // Determine language from file extension
            let language = match crate::parser::Language::from_path(&file_path) {
                Some(l) => l,
                None => continue,
            };

            let content = match std::fs::read_to_string(&file_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let imports = self.extract_imports_from_content(&content, language, file_path_str);

            for (imported_module, import_type) in imports {
                // Try to resolve the module to a file path
                let resolved_path =
                    self.resolve_import(&imported_module, file_path_str, project_root, language);

                // Add edge
                graph.edges.push(ImportEdge {
                    from: file_path_str.clone(),
                    to: resolved_path.unwrap_or(imported_module),
                    import_type: Some(import_type),
                });
            }
        }

        // Store edges in database
        self.store_import_edges(storage, &graph)?;

        Ok(graph)
    }

    /// Extract imports from source content.
    ///
    /// Returns a vector of (module_name, import_type) tuples.
    fn extract_imports_from_content(
        &self,
        content: &str,
        language: Language,
        _file_path: &str,
    ) -> Vec<(String, String)> {
        let mut imports = Vec::new();

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

        // Walk the AST to find import statements
        self.walk_for_imports(tree.root_node(), source_bytes, language, &mut imports);

        imports
    }

    /// Walk the AST to find import statements.
    fn walk_for_imports(
        &self,
        node: tree_sitter::Node,
        source: &[u8],
        language: Language,
        imports: &mut Vec<(String, String)>,
    ) {
        let kind = node.kind();

        match language {
            Language::Python => {
                if kind == "import_statement" {
                    // import os, sys
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        if child.kind() == "dotted_name" {
                            if let Some(name) = self.node_text(child, source) {
                                imports.push((name, "direct".to_string()));
                            }
                        } else if child.kind() == "aliased_import" {
                            if let Some(name_node) = child.child_by_field_name("name") {
                                if let Some(name) = self.node_text(name_node, source) {
                                    imports.push((name, "direct".to_string()));
                                }
                            }
                        }
                    }
                } else if kind == "import_from_statement" {
                    // from pathlib import Path
                    if let Some(module_node) = node.child_by_field_name("module_name") {
                        if let Some(name) = self.node_text(module_node, source) {
                            imports.push((name, "from".to_string()));
                        }
                    } else {
                        // Check for dotted_name after 'from'
                        let mut cursor = node.walk();
                        let mut found_from = false;
                        for child in node.children(&mut cursor) {
                            if child.kind() == "from" {
                                found_from = true;
                            } else if found_from && child.kind() == "dotted_name" {
                                if let Some(name) = self.node_text(child, source) {
                                    imports.push((name, "from".to_string()));
                                }
                                break;
                            } else if found_from && child.kind() == "relative_import" {
                                // Handle relative imports
                                if let Some(name) = self.node_text(child, source) {
                                    imports.push((name, "from".to_string()));
                                }
                                break;
                            }
                        }
                    }

                    // Check for star import
                    let text = self.node_text(node, source).unwrap_or_default();
                    if text.contains("*") {
                        // Already added the module, just update type if needed
                        if let Some(last) = imports.last_mut() {
                            last.1 = "star".to_string();
                        }
                    }
                }
            }
            Language::TypeScript => {
                if kind == "import_statement" {
                    // import { x } from 'module'
                    if let Some(source_node) = node.child_by_field_name("source") {
                        if let Some(name) = self.node_text(source_node, source) {
                            // Remove quotes
                            let clean_name = name.trim_matches(|c| c == '"' || c == '\'');
                            imports.push((clean_name.to_string(), "from".to_string()));
                        }
                    }
                } else if kind == "export_statement" {
                    // export { x } from 'module'
                    if let Some(source_node) = node.child_by_field_name("source") {
                        if let Some(name) = self.node_text(source_node, source) {
                            let clean_name = name.trim_matches(|c| c == '"' || c == '\'');
                            imports.push((clean_name.to_string(), "from".to_string()));
                        }
                    }
                }
            }
            Language::Rust => {
                if kind == "use_declaration" {
                    // use std::collections::HashMap
                    if let Some(path) = self.extract_rust_use_path(node, source) {
                        imports.push((path, "direct".to_string()));
                    }
                } else if kind == "mod_item" {
                    // mod utils;
                    if let Some(name_node) = node.child_by_field_name("name") {
                        if let Some(name) = self.node_text(name_node, source) {
                            imports.push((name, "direct".to_string()));
                        }
                    }
                }
            }
            Language::Markdown => {
                // No imports in Markdown
            }
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_for_imports(child, source, language, imports);
        }
    }

    /// Extract the use path from a Rust use declaration.
    fn extract_rust_use_path(&self, node: tree_sitter::Node, source: &[u8]) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "use_tree" || child.kind() == "scoped_identifier" {
                return self.node_text(child, source);
            }
        }
        None
    }

    /// Resolve an import to a file path if possible.
    fn resolve_import(
        &self,
        module: &str,
        importer_path: &str,
        project_root: &Path,
        language: Language,
    ) -> Option<String> {
        let importer_dir = Path::new(importer_path).parent()?;

        match language {
            Language::Python => {
                // Handle relative imports (starting with .)
                let clean_module = module.trim_start_matches('.');
                let relative_depth = module.len() - clean_module.len();

                let search_dir = if relative_depth > 0 {
                    let mut dir = importer_dir.to_path_buf();
                    for _ in 1..relative_depth {
                        dir = dir.parent()?.to_path_buf();
                    }
                    dir
                } else {
                    project_root.to_path_buf()
                };

                // Convert module path to file path
                let module_path = clean_module.replace('.', "/");

                // Try .py extension
                let py_path = search_dir.join(format!("{}.py", module_path));
                if project_root.join(&py_path).exists() {
                    return Some(py_path.to_string_lossy().to_string());
                }

                // Try __init__.py in package
                let init_path = search_dir.join(&module_path).join("__init__.py");
                if project_root.join(&init_path).exists() {
                    return Some(init_path.to_string_lossy().to_string());
                }

                // Try just the module path
                let simple_path = format!("{}.py", module_path);
                if project_root.join(&simple_path).exists() {
                    return Some(simple_path);
                }
            }
            Language::TypeScript => {
                // Handle relative imports (starting with ./ or ../)
                if module.starts_with('.') {
                    let resolved = importer_dir.join(module);

                    // Try various extensions
                    for ext in &[".ts", ".tsx", ".js", ".jsx", "/index.ts", "/index.js"] {
                        let with_ext = format!("{}{}", resolved.to_string_lossy(), ext);
                        if project_root.join(&with_ext).exists() {
                            return Some(with_ext);
                        }
                    }
                }
            }
            Language::Rust => {
                // Handle Rust modules
                let module_parts: Vec<&str> = module.split("::").collect();
                if !module_parts.is_empty() {
                    // Try to find the module file
                    let first_part = module_parts[0];

                    // Check for mod.rs or direct file
                    let mod_rs = format!("{}/mod.rs", first_part);
                    if project_root.join(&mod_rs).exists() {
                        return Some(mod_rs);
                    }

                    let direct_rs = format!("{}.rs", first_part);
                    if project_root.join(&direct_rs).exists() {
                        return Some(direct_rs);
                    }
                }
            }
            Language::Markdown => {}
        }

        None
    }

    /// Get text content of a node (delegates to shared helper).
    fn node_text(&self, node: tree_sitter::Node, source: &[u8]) -> Option<String> {
        super::node_text(node, source)
    }

    /// Store import edges in the database using parameterized queries.
    fn store_import_edges(
        &self,
        storage: &IndexStorage,
        graph: &ImportGraph,
    ) -> Result<(), crate::index::StorageError> {
        // Ensure the import_edges table exists
        storage.ensure_import_edges_table()?;

        // Clear existing edges
        storage.clear_import_edges()?;

        // Insert new edges using parameterized queries
        for edge in &graph.edges {
            let import_type = edge.import_type.as_deref().unwrap_or("direct");
            storage.insert_import_edge(&edge.from, &edge.to, import_type)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_graph_new() {
        let graph = ImportGraph::new();
        assert_eq!(graph.graph_type, "imports");
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_import_edge_serialize() {
        let edge = ImportEdge {
            from: "main.py".to_string(),
            to: "utils.py".to_string(),
            import_type: Some("from".to_string()),
        };
        let json = serde_json::to_string(&edge);
        assert!(json.is_ok());
        let json_str = json.unwrap();
        assert!(json_str.contains("\"from\":\"main.py\""));
    }
}
