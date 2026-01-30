//! PageRank-style relevance scoring using tree-sitter reference extraction.
//!
//! This module provides relevance scoring for symbols based on how often they
//! are referenced by other symbols. References are detected using tree-sitter
//! AST queries for `call_expression` and `import_statement` nodes.
//!
//! # Reference Detection
//!
//! The module uses tree-sitter to detect:
//! - Function calls via `call_expression` nodes
//! - Import references via `import_statement` nodes
//!
//! # Relevance Formula
//!
//! The relevance score for a symbol is calculated as:
//! ```text
//! relevance = (incoming_references + 1) / max_score
//! ```
//!
//! This ensures:
//! - All symbols have a non-zero score (the +1)
//! - Scores are normalized to the 0.0-1.0 range
//! - Symbols referenced more frequently have higher scores

use std::collections::{HashMap, HashSet};

use crate::parser::{Language, Symbol, SymbolType};
use crate::TreelintError;

/// A reference from one symbol to another, detected via tree-sitter.
#[derive(Debug, Clone)]
pub struct Reference {
    /// The name of the symbol being referenced (callee)
    pub target_name: String,
    /// The file containing the reference
    pub source_file: String,
    /// The line number of the reference
    pub line: usize,
    /// The type of reference (call or import)
    pub ref_type: ReferenceType,
}

/// Type of reference detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceType {
    /// Function call (call_expression)
    Call,
    /// Import statement (import_statement)
    Import,
}

/// Relevance scorer using tree-sitter based reference extraction.
///
/// This service extracts references from source files using tree-sitter
/// AST queries and calculates relevance scores for symbols based on
/// how many incoming references they have.
pub struct RelevanceScorer {
    /// Set of known symbol names (targets for reference counting)
    known_symbols: HashSet<String>,
    /// Reference counts per symbol name
    reference_counts: HashMap<String, usize>,
}

impl RelevanceScorer {
    /// Create a new RelevanceScorer from a list of symbols.
    ///
    /// # Arguments
    ///
    /// * `symbols` - The symbols to track references for
    ///
    /// # Returns
    ///
    /// A new RelevanceScorer instance.
    pub fn new(symbols: &[Symbol]) -> Self {
        // Build set of callable symbol names (functions, methods, classes)
        let known_symbols: HashSet<String> = symbols
            .iter()
            .filter(|s| {
                matches!(
                    s.symbol_type,
                    SymbolType::Function | SymbolType::Method | SymbolType::Class
                )
            })
            .map(|s| s.name.clone())
            .collect();

        // Initialize reference counts to 0
        let reference_counts: HashMap<String, usize> =
            known_symbols.iter().map(|name| (name.clone(), 0)).collect();

        Self {
            known_symbols,
            reference_counts,
        }
    }

    /// Extract references from a source file using tree-sitter.
    ///
    /// This method parses the source file and extracts:
    /// - Function calls (call_expression nodes)
    /// - Import references (import_statement nodes)
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the source file
    /// * `content` - The source code content
    /// * `language` - The programming language
    ///
    /// # Returns
    ///
    /// A vector of detected references, or an error.
    pub fn extract_references(
        &self,
        file_path: &str,
        content: &str,
        language: Language,
    ) -> Result<Vec<Reference>, TreelintError> {
        let mut references = Vec::new();

        // Parse the source code
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&language.tree_sitter_language())
            .map_err(|e| TreelintError::Parse(format!("Failed to set language: {}", e)))?;

        let tree = parser
            .parse(content, None)
            .ok_or_else(|| TreelintError::Parse("Failed to parse source".to_string()))?;

        // Walk the AST to find call_expression and import_statement nodes
        self.walk_node_for_references(
            tree.root_node(),
            content.as_bytes(),
            file_path,
            language,
            &mut references,
        );

        Ok(references)
    }

    /// Walk AST nodes to find references.
    fn walk_node_for_references(
        &self,
        node: tree_sitter::Node,
        source: &[u8],
        file_path: &str,
        language: Language,
        references: &mut Vec<Reference>,
    ) {
        // Check node type for references
        match node.kind() {
            // Call expressions - function/method calls
            "call_expression" | "call" => {
                if let Some(callee_name) = self.extract_callee_name(node, source, language) {
                    if self.known_symbols.contains(&callee_name) {
                        references.push(Reference {
                            target_name: callee_name,
                            source_file: file_path.to_string(),
                            line: node.start_position().row + 1,
                            ref_type: ReferenceType::Call,
                        });
                    }
                }
            }
            // Import statements
            "import_statement" | "import_from_statement" => {
                let import_names = self.extract_import_names(node, source, language);
                for name in import_names {
                    if self.known_symbols.contains(&name) {
                        references.push(Reference {
                            target_name: name,
                            source_file: file_path.to_string(),
                            line: node.start_position().row + 1,
                            ref_type: ReferenceType::Import,
                        });
                    }
                }
            }
            // Rust use declarations
            "use_declaration" => {
                let import_names = self.extract_rust_use_names(node, source);
                for name in import_names {
                    if self.known_symbols.contains(&name) {
                        references.push(Reference {
                            target_name: name,
                            source_file: file_path.to_string(),
                            line: node.start_position().row + 1,
                            ref_type: ReferenceType::Import,
                        });
                    }
                }
            }
            _ => {}
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_node_for_references(child, source, file_path, language, references);
        }
    }

    /// Extract the callee name from a call expression node.
    fn extract_callee_name(
        &self,
        node: tree_sitter::Node,
        source: &[u8],
        language: Language,
    ) -> Option<String> {
        match language {
            Language::Python => {
                // Python: call has 'function' field
                if let Some(func_node) = node.child_by_field_name("function") {
                    return self.node_text(func_node, source);
                }
            }
            Language::TypeScript => {
                // TypeScript: call_expression has 'function' field
                if let Some(func_node) = node.child_by_field_name("function") {
                    // Handle member expressions like obj.method()
                    if func_node.kind() == "member_expression" {
                        if let Some(prop) = func_node.child_by_field_name("property") {
                            return self.node_text(prop, source);
                        }
                    }
                    return self.node_text(func_node, source);
                }
            }
            Language::Rust => {
                // Rust: call_expression has 'function' field
                if let Some(func_node) = node.child_by_field_name("function") {
                    // Handle path calls like module::function()
                    if func_node.kind() == "scoped_identifier" {
                        if let Some(name_node) = func_node.child_by_field_name("name") {
                            return self.node_text(name_node, source);
                        }
                    }
                    return self.node_text(func_node, source);
                }
            }
            Language::Markdown => {
                // Markdown doesn't have function calls
                return None;
            }
        }
        None
    }

    /// Extract imported names from an import statement.
    fn extract_import_names(
        &self,
        node: tree_sitter::Node,
        source: &[u8],
        language: Language,
    ) -> Vec<String> {
        let mut names = Vec::new();

        match language {
            Language::Python => {
                // Python: import os, from module import func
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "dotted_name" {
                        if let Some(name) = self.node_text(child, source) {
                            // Get the last part of dotted name
                            if let Some(last) = name.split('.').next_back() {
                                names.push(last.to_string());
                            }
                        }
                    } else if child.kind() == "aliased_import" {
                        if let Some(alias) = child.child_by_field_name("alias") {
                            if let Some(name) = self.node_text(alias, source) {
                                names.push(name);
                            }
                        } else if let Some(name_node) = child.child_by_field_name("name") {
                            if let Some(name) = self.node_text(name_node, source) {
                                names.push(name);
                            }
                        }
                    }
                }
            }
            Language::TypeScript => {
                // TypeScript: import { foo } from 'module'
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "import_clause" {
                        self.extract_ts_import_clause_names(child, source, &mut names);
                    }
                }
            }
            _ => {}
        }

        names
    }

    /// Extract names from TypeScript import clause.
    fn extract_ts_import_clause_names(
        &self,
        node: tree_sitter::Node,
        source: &[u8],
        names: &mut Vec<String>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "identifier" => {
                    if let Some(name) = self.node_text(child, source) {
                        names.push(name);
                    }
                }
                "named_imports" => {
                    let mut inner_cursor = child.walk();
                    for spec in child.children(&mut inner_cursor) {
                        if spec.kind() == "import_specifier" {
                            if let Some(alias) = spec.child_by_field_name("alias") {
                                if let Some(name) = self.node_text(alias, source) {
                                    names.push(name);
                                }
                            } else if let Some(name_node) = spec.child_by_field_name("name") {
                                if let Some(name) = self.node_text(name_node, source) {
                                    names.push(name);
                                }
                            }
                        }
                    }
                }
                "namespace_import" => {
                    // import * as foo
                    if let Some(name_node) = child.child(1) {
                        if let Some(name) = self.node_text(name_node, source) {
                            names.push(name);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Extract names from Rust use declaration.
    fn extract_rust_use_names(&self, node: tree_sitter::Node, source: &[u8]) -> Vec<String> {
        let mut names = Vec::new();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_rust_use_tree(child, source, &mut names);
        }

        names
    }

    /// Walk Rust use tree to extract imported names.
    fn walk_rust_use_tree(&self, node: tree_sitter::Node, source: &[u8], names: &mut Vec<String>) {
        match node.kind() {
            "identifier" => {
                if let Some(name) = self.node_text(node, source) {
                    if name != "self" {
                        names.push(name);
                    }
                }
            }
            "scoped_identifier" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    if let Some(name) = self.node_text(name_node, source) {
                        names.push(name);
                    }
                }
            }
            "use_as_clause" => {
                if let Some(alias) = node.child_by_field_name("alias") {
                    if let Some(name) = self.node_text(alias, source) {
                        names.push(name);
                    }
                }
            }
            _ => {
                // Recurse
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.walk_rust_use_tree(child, source, names);
                }
            }
        }
    }

    /// Get text content of a node.
    fn node_text(&self, node: tree_sitter::Node, source: &[u8]) -> Option<String> {
        std::str::from_utf8(&source[node.start_byte()..node.end_byte()])
            .ok()
            .map(|s| s.to_string())
    }

    /// Count a reference to a symbol.
    pub fn count_reference(&mut self, symbol_name: &str) {
        if let Some(count) = self.reference_counts.get_mut(symbol_name) {
            *count += 1;
        }
    }

    /// Process references and update counts.
    pub fn process_references(&mut self, references: &[Reference]) {
        for reference in references {
            self.count_reference(&reference.target_name);
        }
    }

    /// Calculate relevance scores for symbols.
    ///
    /// Returns a map from symbol key (file:name) to relevance score (0.0-1.0).
    ///
    /// # Arguments
    ///
    /// * `symbols` - The symbols to calculate scores for
    ///
    /// # Returns
    ///
    /// A HashMap mapping "file_path:symbol_name" to relevance score.
    pub fn calculate_scores(&self, symbols: &[Symbol]) -> HashMap<String, f64> {
        if symbols.is_empty() {
            return HashMap::new();
        }

        // Calculate raw scores: refs + 1 (to ensure non-zero)
        let mut raw_scores: HashMap<String, f64> = HashMap::new();
        let mut max_raw_score = 1.0f64;

        for symbol in symbols {
            let refs = *self.reference_counts.get(&symbol.name).unwrap_or(&0);
            let raw_score = refs as f64 + 1.0;
            let key = format!("{}:{}", symbol.file_path, symbol.name);
            raw_scores.insert(key.clone(), raw_score);
            if raw_score > max_raw_score {
                max_raw_score = raw_score;
            }
        }

        // Normalize to 0.0-1.0 range
        let mut scores: HashMap<String, f64> = HashMap::new();
        for symbol in symbols {
            let key = format!("{}:{}", symbol.file_path, symbol.name);
            let raw_score = *raw_scores.get(&key).unwrap_or(&1.0);
            let normalized_score = raw_score / max_raw_score;
            scores.insert(key, normalized_score);
        }

        scores
    }

    /// Get the reference count for a symbol.
    pub fn get_reference_count(&self, symbol_name: &str) -> usize {
        *self.reference_counts.get(symbol_name).unwrap_or(&0)
    }
}

/// Calculate relevance scores for symbols using tree-sitter reference extraction.
///
/// This is the main entry point for relevance calculation. It:
/// 1. Creates a RelevanceScorer with known symbols
/// 2. Extracts references from source files using tree-sitter
/// 3. Also uses string pattern matching for module-level calls
/// 4. Calculates normalized relevance scores
///
/// # Arguments
///
/// * `symbols` - The symbols to score
///
/// # Returns
///
/// A HashMap mapping "file_path:symbol_name" to relevance score (0.0-1.0).
pub fn calculate_relevance_scores(symbols: &[Symbol]) -> HashMap<String, f64> {
    if symbols.is_empty() {
        return HashMap::new();
    }

    let mut scorer = RelevanceScorer::new(symbols);

    // Collect unique file paths and their content
    let mut file_contents: HashMap<String, (String, Language)> = HashMap::new();
    for symbol in symbols {
        if let Some(ref body) = symbol.body {
            // Use the symbol's body as part of the file content
            file_contents
                .entry(symbol.file_path.clone())
                .or_insert_with(|| (String::new(), symbol.language));

            // Also try to extract references from each symbol's body using tree-sitter
            if let Ok(refs) = scorer.extract_references(&symbol.file_path, body, symbol.language) {
                let filtered_refs: Vec<Reference> = refs
                    .into_iter()
                    .filter(|r| r.target_name != symbol.name)
                    .collect();
                scorer.process_references(&filtered_refs);
            }
        }
    }

    // Additionally, use string pattern matching to catch module-level calls
    // that aren't inside any function body (e.g., `popular()` at module level)
    for symbol in symbols {
        if let Some(ref body) = symbol.body {
            // Search for call patterns in the body using string matching
            for target_name in &scorer.known_symbols.clone() {
                // Skip self-references
                if target_name == &symbol.name {
                    continue;
                }

                // Look for function call pattern: "name("
                let call_pattern = format!("{}(", target_name);
                let count = body.matches(&call_pattern).count();

                for _ in 0..count {
                    scorer.count_reference(target_name);
                }
            }
        }
    }

    scorer.calculate_scores(symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relevance_scorer_creation() {
        let symbols = vec![Symbol {
            name: "test_func".to_string(),
            symbol_type: SymbolType::Function,
            visibility: None,
            file_path: "test.py".to_string(),
            line_start: 1,
            line_end: 2,
            signature: None,
            body: None,
            language: Language::Python,
        }];

        let scorer = RelevanceScorer::new(&symbols);
        assert!(scorer.known_symbols.contains("test_func"));
        assert_eq!(scorer.get_reference_count("test_func"), 0);
    }

    #[test]
    fn test_count_reference() {
        let symbols = vec![Symbol {
            name: "target".to_string(),
            symbol_type: SymbolType::Function,
            visibility: None,
            file_path: "test.py".to_string(),
            line_start: 1,
            line_end: 2,
            signature: None,
            body: None,
            language: Language::Python,
        }];

        let mut scorer = RelevanceScorer::new(&symbols);
        scorer.count_reference("target");
        scorer.count_reference("target");
        assert_eq!(scorer.get_reference_count("target"), 2);
    }

    #[test]
    fn test_calculate_scores_empty() {
        let symbols: Vec<Symbol> = vec![];
        let scores = calculate_relevance_scores(&symbols);
        assert!(scores.is_empty());
    }

    #[test]
    fn test_calculate_scores_normalized() {
        let symbols = vec![
            Symbol {
                name: "func_a".to_string(),
                symbol_type: SymbolType::Function,
                visibility: None,
                file_path: "test.py".to_string(),
                line_start: 1,
                line_end: 2,
                signature: None,
                body: None,
                language: Language::Python,
            },
            Symbol {
                name: "func_b".to_string(),
                symbol_type: SymbolType::Function,
                visibility: None,
                file_path: "test.py".to_string(),
                line_start: 3,
                line_end: 4,
                signature: None,
                body: None,
                language: Language::Python,
            },
        ];

        let scores = calculate_relevance_scores(&symbols);

        // Both should have scores in 0.0-1.0 range
        for (_key, score) in &scores {
            assert!(*score >= 0.0 && *score <= 1.0);
        }
    }
}
