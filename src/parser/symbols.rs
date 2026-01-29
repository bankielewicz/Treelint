//! Symbol extraction and representation.
//!
//! This module provides types for representing extracted symbols and
//! the [`SymbolExtractor`] service for parsing source files.

use std::path::Path;

use serde::{Deserialize, Serialize};
use tree_sitter::{Node, Tree};

use super::languages::Language;
use crate::TreelintError;

// Delimiter used for extracting signatures from code
const SIGNATURE_DELIMITER_BRACE: char = '{';
const SIGNATURE_DELIMITER_COLON: char = ':';

/// The type of a symbol extracted from source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolType {
    /// A function definition
    Function,
    /// A class, struct, or interface definition
    Class,
    /// A method within a class or impl block
    Method,
    /// A variable binding
    Variable,
    /// A constant value
    Constant,
    /// An import statement
    Import,
    /// An export statement (TypeScript)
    Export,
}

impl std::fmt::Display for SymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolType::Function => write!(f, "Function"),
            SymbolType::Class => write!(f, "Class"),
            SymbolType::Method => write!(f, "Method"),
            SymbolType::Variable => write!(f, "Variable"),
            SymbolType::Constant => write!(f, "Constant"),
            SymbolType::Import => write!(f, "Import"),
            SymbolType::Export => write!(f, "Export"),
        }
    }
}

/// Visibility modifier for a symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    /// Public visibility (e.g., `pub` in Rust, no underscore in Python)
    Public,
    /// Private visibility (e.g., no `pub` in Rust, leading underscore in Python)
    Private,
    /// Protected visibility (e.g., `protected` in TypeScript)
    Protected,
}

/// A symbol extracted from source code.
///
/// Represents a semantic unit such as a function, class, method, variable,
/// constant, import, or export.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    /// The name of the symbol
    pub name: String,
    /// The type of symbol (function, class, etc.)
    pub symbol_type: SymbolType,
    /// Visibility modifier (if applicable)
    pub visibility: Option<Visibility>,
    /// Path to the source file containing this symbol
    pub file_path: String,
    /// Starting line number (1-indexed)
    pub line_start: usize,
    /// Ending line number (1-indexed)
    pub line_end: usize,
    /// The symbol's signature (e.g., function signature)
    pub signature: Option<String>,
    /// The full body of the symbol (optional, for lazy loading)
    pub body: Option<String>,
    /// The language this symbol was extracted from
    pub language: Language,
}

/// Builder for creating Symbol instances with reduced boilerplate.
///
/// This builder provides a fluent interface for constructing Symbol
/// instances, reducing code duplication across language-specific
/// extraction methods.
struct SymbolBuilder {
    name: String,
    symbol_type: SymbolType,
    file_path: String,
    line_start: usize,
    line_end: usize,
    language: Language,
    visibility: Option<Visibility>,
    signature: Option<String>,
}

impl SymbolBuilder {
    /// Create a new SymbolBuilder with required fields.
    fn new(
        name: String,
        symbol_type: SymbolType,
        file_path: &str,
        node: Node,
        language: Language,
    ) -> Self {
        Self {
            name,
            symbol_type,
            file_path: file_path.to_string(),
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            language,
            visibility: None,
            signature: None,
        }
    }

    /// Set the visibility modifier.
    fn visibility(mut self, visibility: Option<Visibility>) -> Self {
        self.visibility = visibility;
        self
    }

    /// Set the signature.
    fn signature(mut self, signature: Option<String>) -> Self {
        self.signature = signature;
        self
    }

    /// Build the final Symbol.
    fn build(self) -> Symbol {
        Symbol {
            name: self.name,
            symbol_type: self.symbol_type,
            visibility: self.visibility,
            file_path: self.file_path,
            line_start: self.line_start,
            line_end: self.line_end,
            signature: self.signature,
            body: None,
            language: self.language,
        }
    }
}

/// A wrapper around tree-sitter's Parser for a specific language.
///
/// This provides a convenient interface for parsing source code with
/// embedded tree-sitter grammars.
pub struct Parser {
    language: Language,
}

impl Parser {
    /// Create a new parser for the specified language.
    ///
    /// # Arguments
    ///
    /// * `language` - The language to parse
    ///
    /// # Returns
    ///
    /// A `Result` containing the parser or an error if initialization fails.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::parser::{Parser, Language};
    ///
    /// let parser = Parser::new(Language::Python)?;
    /// # Ok::<(), treelint::TreelintError>(())
    /// ```
    pub fn new(language: Language) -> Result<Self, TreelintError> {
        // Validate that the grammar can be loaded
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&language.tree_sitter_language())
            .map_err(|e| TreelintError::Parse(format!("Failed to set language: {}", e)))?;

        Ok(Self { language })
    }

    /// Parse source code and return the syntax tree.
    ///
    /// # Arguments
    ///
    /// * `source` - The source code to parse
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed [`Tree`] or an error.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::parser::{Parser, Language};
    ///
    /// let parser = Parser::new(Language::Python)?;
    /// let tree = parser.parse("def hello(): pass")?;
    /// # Ok::<(), treelint::TreelintError>(())
    /// ```
    pub fn parse(&self, source: &str) -> Result<Tree, TreelintError> {
        // Note: tree_sitter::Parser::parse takes &mut self, so we need to clone
        // for immutable interface. In practice, we create a new parser for each parse.
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&self.language.tree_sitter_language())
            .map_err(|e| TreelintError::Parse(format!("Failed to set language: {}", e)))?;

        parser
            .parse(source, None)
            .ok_or_else(|| TreelintError::Parse("Failed to parse source".to_string()))
    }

    /// Get the language this parser is configured for.
    pub fn language(&self) -> Language {
        self.language
    }
}

/// Service for extracting symbols from source files.
///
/// This is the main entry point for symbol extraction. It handles language
/// detection, parsing, and query execution.
#[derive(Default)]
pub struct SymbolExtractor;

impl SymbolExtractor {
    /// Create a new SymbolExtractor.
    pub fn new() -> Self {
        Self
    }

    /// Extract symbols from a file path.
    ///
    /// Automatically detects the language from the file extension.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the source file
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of extracted symbols or an error.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use treelint::parser::SymbolExtractor;
    ///
    /// let extractor = SymbolExtractor::new();
    /// let symbols = extractor.extract_from_file(Path::new("example.py"))?;
    /// # Ok::<(), treelint::TreelintError>(())
    /// ```
    pub fn extract_from_file(&self, path: &Path) -> Result<Vec<Symbol>, TreelintError> {
        let language = Language::from_path(path).ok_or_else(|| {
            TreelintError::Parse(format!("Unsupported file type: {}", path.display()))
        })?;

        let content = std::fs::read_to_string(path)?;
        let file_path = path.to_string_lossy().to_string();

        let mut symbols = self.extract_with_path(&content, language, &file_path)?;

        // Populate body from source content using line ranges
        let lines: Vec<&str> = content.lines().collect();
        for symbol in &mut symbols {
            if symbol.body.is_none() && symbol.line_start > 0 && symbol.line_end > 0 {
                let start = symbol.line_start.saturating_sub(1); // Convert to 0-based
                let end = symbol.line_end.min(lines.len()); // Clamp to file length
                if start < end {
                    let body_text: String = lines[start..end].join("\n");
                    if !body_text.is_empty() {
                        symbol.body = Some(body_text);
                    }
                }
            }
        }

        Ok(symbols)
    }

    /// Extract symbols from source code content.
    ///
    /// # Arguments
    ///
    /// * `content` - The source code to parse
    /// * `language` - The language of the source code
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of extracted symbols or an error.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::parser::{SymbolExtractor, Language};
    ///
    /// let extractor = SymbolExtractor::new();
    /// let symbols = extractor.extract_from_content("def hello(): pass", Language::Python)?;
    /// # Ok::<(), treelint::TreelintError>(())
    /// ```
    pub fn extract_from_content(
        &self,
        content: &str,
        language: Language,
    ) -> Result<Vec<Symbol>, TreelintError> {
        self.extract_with_path(content, language, "<string>")
    }

    /// Internal method to extract symbols with a specified file path.
    fn extract_with_path(
        &self,
        content: &str,
        language: Language,
        file_path: &str,
    ) -> Result<Vec<Symbol>, TreelintError> {
        // Empty or whitespace-only content
        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        let parser = Parser::new(language)?;
        let tree = parser.parse(content)?;

        let mut symbols = Vec::new();
        let source_bytes = content.as_bytes();

        // Extract symbols based on language
        match language {
            Language::Python => {
                self.extract_python_symbols(&tree, source_bytes, file_path, &mut symbols);
            }
            Language::TypeScript => {
                self.extract_typescript_symbols(&tree, source_bytes, file_path, &mut symbols);
            }
            Language::Rust => {
                self.extract_rust_symbols(&tree, source_bytes, file_path, &mut symbols);
            }
            Language::Markdown => {
                self.extract_markdown_symbols(&tree, source_bytes, file_path, &mut symbols);
            }
        }

        // Sort by line number to maintain source order
        symbols.sort_by_key(|s| s.line_start);

        Ok(symbols)
    }

    /// Extract symbols from Python source code.
    fn extract_python_symbols(
        &self,
        tree: &Tree,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        let root = tree.root_node();
        self.walk_python_node(root, source, file_path, symbols, None);
    }

    /// Recursively walk Python AST nodes.
    fn walk_python_node(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
        parent_class: Option<&str>,
    ) {
        match node.kind() {
            "decorated_definition" => {
                // Handle decorated functions/classes - pass the outer node for correct line ranges
                if let Some(definition) = node.child_by_field_name("definition") {
                    match definition.kind() {
                        "function_definition" => {
                            if let Some(symbol) = self.extract_python_function_with_decorators(
                                node,
                                definition,
                                source,
                                file_path,
                                parent_class,
                            ) {
                                symbols.push(symbol);
                            }
                        }
                        "class_definition" => {
                            if let Some(class_symbol) = self.extract_python_class_with_decorators(
                                node, definition, source, file_path,
                            ) {
                                let class_name = class_symbol.name.clone();
                                symbols.push(class_symbol);

                                // Extract methods from class body
                                if let Some(body) = definition.child_by_field_name("body") {
                                    self.walk_python_node(
                                        body,
                                        source,
                                        file_path,
                                        symbols,
                                        Some(&class_name),
                                    );
                                }
                            }
                        }
                        _ => {}
                    }
                }
                return; // Don't recurse further - we've handled the definition
            }
            "function_definition" => {
                if let Some(symbol) =
                    self.extract_python_function(node, source, file_path, parent_class)
                {
                    symbols.push(symbol);
                }
            }
            "class_definition" => {
                if let Some(class_symbol) = self.extract_python_class(node, source, file_path) {
                    let class_name = class_symbol.name.clone();
                    symbols.push(class_symbol);

                    // Extract methods from class body
                    if let Some(body) = node.child_by_field_name("body") {
                        self.walk_python_node(body, source, file_path, symbols, Some(&class_name));
                    }
                    return; // Don't recurse into class body twice
                }
            }
            "import_statement" | "import_from_statement" => {
                self.extract_python_imports(node, source, file_path, symbols);
            }
            "expression_statement" => {
                // Check for module-level assignments (constants/variables)
                if parent_class.is_none() {
                    if let Some(assignment) = node.child(0) {
                        if assignment.kind() == "assignment" {
                            self.extract_python_assignment(assignment, source, file_path, symbols);
                        }
                    }
                }
            }
            _ => {}
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_python_node(child, source, file_path, symbols, parent_class);
        }
    }

    /// Extract a Python function symbol.
    fn extract_python_function(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        parent_class: Option<&str>,
    ) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;

        let symbol_type = if parent_class.is_some() {
            SymbolType::Method
        } else {
            SymbolType::Function
        };

        let visibility = self.determine_python_visibility(&name);
        let signature = self.extract_python_function_signature(node, source);

        Some(
            SymbolBuilder::new(name, symbol_type, file_path, node, Language::Python)
                .visibility(visibility)
                .signature(signature)
                .build(),
        )
    }

    /// Extract a Python function symbol with decorators.
    ///
    /// Uses the outer decorated_definition node for line ranges to include decorators,
    /// but extracts the function name from the inner function_definition node.
    fn extract_python_function_with_decorators(
        &self,
        outer_node: Node,
        inner_node: Node,
        source: &[u8],
        file_path: &str,
        parent_class: Option<&str>,
    ) -> Option<Symbol> {
        let name_node = inner_node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;

        let symbol_type = if parent_class.is_some() {
            SymbolType::Method
        } else {
            SymbolType::Function
        };

        let visibility = self.determine_python_visibility(&name);
        let signature = self.extract_python_function_signature(inner_node, source);

        // Use outer_node for line ranges to include decorators
        Some(Symbol {
            name,
            symbol_type,
            visibility,
            file_path: file_path.to_string(),
            line_start: outer_node.start_position().row + 1,
            line_end: outer_node.end_position().row + 1,
            signature,
            body: None,
            language: Language::Python,
        })
    }

    /// Extract a Python class symbol with decorators.
    fn extract_python_class_with_decorators(
        &self,
        outer_node: Node,
        inner_node: Node,
        source: &[u8],
        file_path: &str,
    ) -> Option<Symbol> {
        let name_node = inner_node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;

        let visibility = self.determine_python_visibility(&name);
        let signature = self.extract_python_class_signature(inner_node, source);

        // Use outer_node for line ranges to include decorators
        Some(Symbol {
            name,
            symbol_type: SymbolType::Class,
            visibility,
            file_path: file_path.to_string(),
            line_start: outer_node.start_position().row + 1,
            line_end: outer_node.end_position().row + 1,
            signature,
            body: None,
            language: Language::Python,
        })
    }

    /// Extract Python function signature.
    ///
    /// For Python, we need to find the colon that ends the function signature,
    /// which comes after the closing parenthesis and optional return type annotation.
    /// Example: `def foo(x: int, y: str) -> bool:` should extract up to but not including the final `:`.
    fn extract_python_function_signature(&self, node: Node, source: &[u8]) -> Option<String> {
        let start = node.start_byte();
        let text = std::str::from_utf8(&source[start..]).ok()?;

        // Find the closing paren of the function parameters
        let mut paren_depth = 0;
        let mut after_paren_end = 0;

        for (i, ch) in text.char_indices() {
            match ch {
                '(' => paren_depth += 1,
                ')' => {
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        after_paren_end = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }

        if after_paren_end == 0 {
            // No closing paren found, fall back to simple extraction
            return self.extract_signature_until(node, source, SIGNATURE_DELIMITER_COLON);
        }

        // Now find the colon that comes after the closing paren (and optional return type)
        // Skip any return type annotation (-> Type)
        let after_paren = &text[after_paren_end..];
        if let Some(colon_pos) = after_paren.find(':') {
            let sig = text[..after_paren_end + colon_pos].trim();
            return Some(sig.to_string());
        }

        // Fallback: return up to the closing paren
        Some(text[..after_paren_end].trim().to_string())
    }

    /// Extract a Python class symbol.
    fn extract_python_class(&self, node: Node, source: &[u8], file_path: &str) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;

        let visibility = self.determine_python_visibility(&name);
        let signature = self.extract_python_class_signature(node, source);

        Some(
            SymbolBuilder::new(name, SymbolType::Class, file_path, node, Language::Python)
                .visibility(visibility)
                .signature(signature)
                .build(),
        )
    }

    /// Extract Python class signature.
    fn extract_python_class_signature(&self, node: Node, source: &[u8]) -> Option<String> {
        self.extract_signature_until(node, source, SIGNATURE_DELIMITER_COLON)
    }

    /// Extract Python imports.
    fn extract_python_imports(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        if node.kind() == "import_statement" {
            // import os, sys
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "dotted_name" {
                    if let Some(name) = self.node_text(child, source) {
                        symbols.push(self.make_bare_symbol(
                            name,
                            SymbolType::Import,
                            file_path,
                            node,
                            Language::Python,
                        ));
                    }
                } else if child.kind() == "aliased_import" {
                    // import os as operating_system
                    if let Some(alias) = child.child_by_field_name("alias") {
                        if let Some(name) = self.node_text(alias, source) {
                            symbols.push(self.make_bare_symbol(
                                name,
                                SymbolType::Import,
                                file_path,
                                node,
                                Language::Python,
                            ));
                        }
                    }
                }
            }
        } else if node.kind() == "import_from_statement" {
            // from pathlib import Path
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "dotted_name"
                    && child.parent().map(|p| p.kind()) == Some("import_from_statement")
                {
                    // This is the imported name, not the module
                    if let Some(name) = self.node_text(child, source) {
                        // Skip the module name (first dotted_name after 'from')
                        let is_module_name = {
                            let mut prev_cursor = node.walk();
                            let children: Vec<_> = node.children(&mut prev_cursor).collect();
                            children
                                .iter()
                                .position(|c| c.id() == child.id())
                                .map(|pos| pos < 2) // First dotted_name is module
                                .unwrap_or(false)
                        };

                        if !is_module_name {
                            symbols.push(self.make_bare_symbol(
                                name,
                                SymbolType::Import,
                                file_path,
                                node,
                                Language::Python,
                            ));
                        }
                    }
                } else if child.kind() == "aliased_import" {
                    // from collections import defaultdict as dd
                    if let Some(alias) = child.child_by_field_name("alias") {
                        if let Some(name) = self.node_text(alias, source) {
                            symbols.push(self.make_bare_symbol(
                                name,
                                SymbolType::Import,
                                file_path,
                                node,
                                Language::Python,
                            ));
                        }
                    } else if let Some(name_node) = child.child_by_field_name("name") {
                        if let Some(name) = self.node_text(name_node, source) {
                            symbols.push(self.make_bare_symbol(
                                name,
                                SymbolType::Import,
                                file_path,
                                node,
                                Language::Python,
                            ));
                        }
                    }
                }
            }
        }
    }

    /// Extract Python assignment (variable/constant).
    fn extract_python_assignment(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        if let Some(left) = node.child_by_field_name("left") {
            if left.kind() == "identifier" {
                if let Some(name) = self.node_text(left, source) {
                    let symbol_type = if self.is_python_constant(&name) {
                        SymbolType::Constant
                    } else {
                        SymbolType::Variable
                    };

                    let visibility = if name.starts_with('_') {
                        Some(Visibility::Private)
                    } else {
                        Some(Visibility::Public)
                    };

                    symbols.push(
                        SymbolBuilder::new(name, symbol_type, file_path, node, Language::Python)
                            .visibility(visibility)
                            .build(),
                    );
                }
            }
        }
    }

    /// Extract symbols from TypeScript source code.
    fn extract_typescript_symbols(
        &self,
        tree: &Tree,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        let root = tree.root_node();
        self.walk_typescript_node(root, source, file_path, symbols, None);
    }

    /// Recursively walk TypeScript AST nodes.
    fn walk_typescript_node(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
        parent_class: Option<&str>,
    ) {
        match node.kind() {
            "function_declaration" => {
                if let Some(symbol) =
                    self.extract_typescript_function(node, source, file_path, false)
                {
                    symbols.push(symbol);
                }
            }
            "method_definition" => {
                if let Some(symbol) = self.extract_typescript_method(node, source, file_path) {
                    symbols.push(symbol);
                }
            }
            "class_declaration" => {
                if let Some(class_symbol) = self.extract_typescript_class(node, source, file_path) {
                    let class_name = class_symbol.name.clone();
                    symbols.push(class_symbol);

                    // Extract methods from class body
                    if let Some(body) = node.child_by_field_name("body") {
                        self.walk_typescript_node(
                            body,
                            source,
                            file_path,
                            symbols,
                            Some(&class_name),
                        );
                    }
                    return;
                }
            }
            "interface_declaration" => {
                if let Some(symbol) = self.extract_typescript_interface(node, source, file_path) {
                    symbols.push(symbol);
                }
            }
            "type_alias_declaration" => {
                if let Some(symbol) = self.extract_typescript_type_alias(node, source, file_path) {
                    symbols.push(symbol);
                }
            }
            "import_statement" => {
                self.extract_typescript_imports(node, source, file_path, symbols);
            }
            "export_statement" => {
                self.extract_typescript_exports(node, source, file_path, symbols);
            }
            "lexical_declaration" => {
                // const/let declarations
                if parent_class.is_none() {
                    self.extract_typescript_lexical(node, source, file_path, symbols);
                }
            }
            "variable_declaration" => {
                // var declarations
                if parent_class.is_none() {
                    self.extract_typescript_variable(node, source, file_path, symbols);
                }
            }
            _ => {}
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() != "class_body" || parent_class.is_some() {
                self.walk_typescript_node(child, source, file_path, symbols, parent_class);
            }
        }
    }

    /// Extract a TypeScript function symbol.
    fn extract_typescript_function(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        is_exported: bool,
    ) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;

        let visibility = if is_exported {
            Some(Visibility::Public)
        } else {
            Some(Visibility::Private)
        };

        let signature = self.extract_typescript_function_signature(node, source);

        Some(
            SymbolBuilder::new(
                name,
                SymbolType::Function,
                file_path,
                node,
                Language::TypeScript,
            )
            .visibility(visibility)
            .signature(signature)
            .build(),
        )
    }

    /// Extract TypeScript function signature.
    fn extract_typescript_function_signature(&self, node: Node, source: &[u8]) -> Option<String> {
        self.extract_signature_until(node, source, SIGNATURE_DELIMITER_BRACE)
    }

    /// Extract a TypeScript method symbol.
    fn extract_typescript_method(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
    ) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;

        let visibility = self.get_typescript_visibility(node, source);
        let signature = self.extract_typescript_function_signature(node, source);

        Some(
            SymbolBuilder::new(
                name,
                SymbolType::Method,
                file_path,
                node,
                Language::TypeScript,
            )
            .visibility(visibility)
            .signature(signature)
            .build(),
        )
    }

    /// Get TypeScript visibility from modifiers.
    fn get_typescript_visibility(&self, node: Node, source: &[u8]) -> Option<Visibility> {
        let text = self.node_text(node, source)?;
        if text.contains("private") {
            Some(Visibility::Private)
        } else if text.contains("protected") {
            Some(Visibility::Protected)
        } else {
            // Public is explicit or default in TypeScript
            Some(Visibility::Public)
        }
    }

    /// Extract a TypeScript class symbol.
    fn extract_typescript_class(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
    ) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;

        let signature = self.extract_typescript_class_signature(node, source);

        Some(
            SymbolBuilder::new(
                name,
                SymbolType::Class,
                file_path,
                node,
                Language::TypeScript,
            )
            .visibility(Some(Visibility::Public))
            .signature(signature)
            .build(),
        )
    }

    /// Extract TypeScript class signature.
    fn extract_typescript_class_signature(&self, node: Node, source: &[u8]) -> Option<String> {
        self.extract_signature_until(node, source, SIGNATURE_DELIMITER_BRACE)
    }

    /// Extract a TypeScript interface symbol.
    fn extract_typescript_interface(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
    ) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;

        // Interfaces treated as Class for simplicity
        Some(
            SymbolBuilder::new(
                name,
                SymbolType::Class,
                file_path,
                node,
                Language::TypeScript,
            )
            .visibility(Some(Visibility::Public))
            .build(),
        )
    }

    /// Extract a TypeScript type alias symbol.
    fn extract_typescript_type_alias(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
    ) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;

        // Type aliases as Variable
        Some(
            SymbolBuilder::new(
                name,
                SymbolType::Variable,
                file_path,
                node,
                Language::TypeScript,
            )
            .visibility(Some(Visibility::Public))
            .build(),
        )
    }

    /// Extract TypeScript imports.
    fn extract_typescript_imports(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "import_clause" {
                self.extract_typescript_import_clause(child, source, file_path, node, symbols);
            }
        }
    }

    /// Extract imports from import clause.
    fn extract_typescript_import_clause(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        parent: Node,
        symbols: &mut Vec<Symbol>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "identifier" => {
                    // Default import
                    if let Some(name) = self.node_text(child, source) {
                        symbols.push(self.make_bare_symbol(
                            name,
                            SymbolType::Import,
                            file_path,
                            parent,
                            Language::TypeScript,
                        ));
                    }
                }
                "named_imports" => {
                    self.extract_typescript_named_imports(
                        child, source, file_path, parent, symbols,
                    );
                }
                "namespace_import" => {
                    if let Some(name_node) = child.child(1) {
                        if let Some(name) = self.node_text(name_node, source) {
                            symbols.push(self.make_bare_symbol(
                                name,
                                SymbolType::Import,
                                file_path,
                                parent,
                                Language::TypeScript,
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Extract named imports.
    fn extract_typescript_named_imports(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        parent: Node,
        symbols: &mut Vec<Symbol>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "import_specifier" {
                // Check for alias
                if let Some(alias) = child.child_by_field_name("alias") {
                    if let Some(name) = self.node_text(alias, source) {
                        symbols.push(self.make_bare_symbol(
                            name,
                            SymbolType::Import,
                            file_path,
                            parent,
                            Language::TypeScript,
                        ));
                    }
                } else if let Some(name_node) = child.child_by_field_name("name") {
                    if let Some(name) = self.node_text(name_node, source) {
                        symbols.push(self.make_bare_symbol(
                            name,
                            SymbolType::Import,
                            file_path,
                            parent,
                            Language::TypeScript,
                        ));
                    }
                }
            }
        }
    }

    /// Extract TypeScript exports.
    fn extract_typescript_exports(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "function_declaration" => {
                    if let Some(mut symbol) =
                        self.extract_typescript_function(child, source, file_path, true)
                    {
                        symbol.symbol_type = SymbolType::Function;
                        symbol.visibility = Some(Visibility::Public);
                        symbols.push(symbol);
                    }
                }
                "class_declaration" => {
                    if let Some(class_symbol) =
                        self.extract_typescript_class(child, source, file_path)
                    {
                        let class_name = class_symbol.name.clone();
                        symbols.push(class_symbol);

                        // Extract methods
                        if let Some(body) = child.child_by_field_name("body") {
                            self.walk_typescript_node(
                                body,
                                source,
                                file_path,
                                symbols,
                                Some(&class_name),
                            );
                        }
                    }
                }
                "interface_declaration" => {
                    if let Some(symbol) =
                        self.extract_typescript_interface(child, source, file_path)
                    {
                        symbols.push(symbol);
                    }
                }
                "type_alias_declaration" => {
                    if let Some(mut symbol) =
                        self.extract_typescript_type_alias(child, source, file_path)
                    {
                        symbol.symbol_type = SymbolType::Export;
                        symbols.push(symbol);
                    }
                }
                "lexical_declaration" => {
                    self.extract_typescript_exported_lexical(child, source, file_path, symbols);
                }
                "export_clause" => {
                    self.extract_typescript_export_clause(child, source, file_path, node, symbols);
                }
                _ => {}
            }
        }
    }

    /// Extract exported const/let declarations.
    fn extract_typescript_exported_lexical(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                if let Some(name_node) = child.child_by_field_name("name") {
                    if let Some(name) = self.node_text(name_node, source) {
                        // Exported items are marked as Export
                        // (we're already in an export statement context)
                        symbols.push(self.make_bare_symbol_with_visibility(
                            name,
                            SymbolType::Export,
                            Some(Visibility::Public),
                            file_path,
                            node,
                            Language::TypeScript,
                        ));
                    }
                }
            }
        }
    }

    /// Extract lexical declarations (non-exported).
    fn extract_typescript_lexical(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        let is_const = self
            .node_text(node, source)
            .map(|t| t.starts_with("const"))
            .unwrap_or(false);

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                if let Some(name_node) = child.child_by_field_name("name") {
                    if let Some(name) = self.node_text(name_node, source) {
                        let symbol_type = if is_const {
                            if name
                                .chars()
                                .all(|c| c.is_uppercase() || c == '_' || c.is_numeric())
                                && name.chars().any(|c| c.is_alphabetic())
                            {
                                SymbolType::Constant
                            } else {
                                SymbolType::Variable
                            }
                        } else {
                            SymbolType::Variable
                        };

                        symbols.push(self.make_bare_symbol_with_visibility(
                            name,
                            symbol_type,
                            Some(Visibility::Private),
                            file_path,
                            node,
                            Language::TypeScript,
                        ));
                    }
                }
            }
        }
    }

    /// Extract variable declarations.
    fn extract_typescript_variable(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "variable_declarator" {
                if let Some(name_node) = child.child_by_field_name("name") {
                    if let Some(name) = self.node_text(name_node, source) {
                        symbols.push(self.make_bare_symbol_with_visibility(
                            name,
                            SymbolType::Variable,
                            Some(Visibility::Private),
                            file_path,
                            node,
                            Language::TypeScript,
                        ));
                    }
                }
            }
        }
    }

    /// Extract export clause (re-exports).
    fn extract_typescript_export_clause(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        parent: Node,
        symbols: &mut Vec<Symbol>,
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "export_specifier" {
                // Check for alias
                if let Some(alias) = child.child_by_field_name("alias") {
                    if let Some(name) = self.node_text(alias, source) {
                        symbols.push(self.make_bare_symbol_with_visibility(
                            name,
                            SymbolType::Export,
                            Some(Visibility::Public),
                            file_path,
                            parent,
                            Language::TypeScript,
                        ));
                    }
                } else if let Some(name_node) = child.child_by_field_name("name") {
                    if let Some(name) = self.node_text(name_node, source) {
                        symbols.push(self.make_bare_symbol_with_visibility(
                            name,
                            SymbolType::Export,
                            Some(Visibility::Public),
                            file_path,
                            parent,
                            Language::TypeScript,
                        ));
                    }
                }
            }
        }
    }

    /// Extract symbols from Rust source code.
    fn extract_rust_symbols(
        &self,
        tree: &Tree,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        let root = tree.root_node();
        self.walk_rust_node(root, source, file_path, symbols, false);
    }

    /// Recursively walk Rust AST nodes.
    fn walk_rust_node(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
        in_impl: bool,
    ) {
        match node.kind() {
            "function_item" => {
                if let Some(symbol) = self.extract_rust_function(node, source, file_path, in_impl) {
                    symbols.push(symbol);
                }
            }
            "struct_item" => {
                if let Some(symbol) = self.extract_rust_struct(node, source, file_path) {
                    symbols.push(symbol);
                }
            }
            "enum_item" => {
                if let Some(symbol) = self.extract_rust_enum(node, source, file_path) {
                    symbols.push(symbol);
                }
            }
            "trait_item" => {
                if let Some(symbol) = self.extract_rust_trait(node, source, file_path) {
                    symbols.push(symbol);
                }
            }
            "impl_item" => {
                // Extract methods from impl block
                if let Some(body) = node.child_by_field_name("body") {
                    self.walk_rust_node(body, source, file_path, symbols, true);
                }
            }
            "use_declaration" => {
                self.extract_rust_use(node, source, file_path, symbols);
            }
            "const_item" => {
                if let Some(symbol) = self.extract_rust_const(node, source, file_path) {
                    symbols.push(symbol);
                }
            }
            "static_item" => {
                if let Some(symbol) = self.extract_rust_static(node, source, file_path) {
                    symbols.push(symbol);
                }
            }
            "type_item" => {
                if let Some(symbol) = self.extract_rust_type_alias(node, source, file_path) {
                    symbols.push(symbol);
                }
            }
            _ => {}
        }

        // Recurse into children (except for impl body which is handled above)
        if node.kind() != "impl_item" {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                self.walk_rust_node(child, source, file_path, symbols, in_impl);
            }
        }
    }

    /// Extract a Rust function symbol.
    fn extract_rust_function(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        is_method: bool,
    ) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;

        let symbol_type = if is_method {
            SymbolType::Method
        } else {
            SymbolType::Function
        };

        let visibility = self.get_rust_visibility(node, source);
        let signature = self.extract_rust_function_signature(node, source);

        Some(
            SymbolBuilder::new(name, symbol_type, file_path, node, Language::Rust)
                .visibility(visibility)
                .signature(signature)
                .build(),
        )
    }

    /// Get Rust visibility from node.
    fn get_rust_visibility(&self, node: Node, source: &[u8]) -> Option<Visibility> {
        // Check for visibility_modifier child
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "visibility_modifier" {
                let text = self.node_text(child, source)?;
                if text.starts_with("pub") {
                    return Some(Visibility::Public);
                }
            }
        }
        Some(Visibility::Private) // Default in Rust
    }

    /// Extract Rust function signature.
    fn extract_rust_function_signature(&self, node: Node, source: &[u8]) -> Option<String> {
        self.extract_signature_until(node, source, SIGNATURE_DELIMITER_BRACE)
    }

    /// Extract a Rust struct symbol.
    fn extract_rust_struct(&self, node: Node, source: &[u8], file_path: &str) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;
        let visibility = self.get_rust_visibility(node, source);

        // Structs as Class
        Some(
            SymbolBuilder::new(name, SymbolType::Class, file_path, node, Language::Rust)
                .visibility(visibility)
                .build(),
        )
    }

    /// Extract a Rust enum symbol.
    fn extract_rust_enum(&self, node: Node, source: &[u8], file_path: &str) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;
        let visibility = self.get_rust_visibility(node, source);

        // Enums as Class
        Some(
            SymbolBuilder::new(name, SymbolType::Class, file_path, node, Language::Rust)
                .visibility(visibility)
                .build(),
        )
    }

    /// Extract a Rust trait symbol.
    fn extract_rust_trait(&self, node: Node, source: &[u8], file_path: &str) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;
        let visibility = self.get_rust_visibility(node, source);

        // Traits as Class
        Some(
            SymbolBuilder::new(name, SymbolType::Class, file_path, node, Language::Rust)
                .visibility(visibility)
                .build(),
        )
    }

    /// Extract Rust use declarations.
    fn extract_rust_use(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        let visibility = self.get_rust_visibility(node, source);
        let is_pub = visibility == Some(Visibility::Public);

        // Extract the use path and name
        self.extract_rust_use_tree(node, source, file_path, symbols, is_pub);
    }

    /// Extract use tree items.
    fn extract_rust_use_tree(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
        is_pub: bool,
    ) {
        let symbol_type = if is_pub {
            SymbolType::Export
        } else {
            SymbolType::Import
        };

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "use_tree" => {
                    // Check if this is a simple path (no braces) or complex
                    let has_use_list = child
                        .children(&mut child.walk())
                        .any(|c| c.kind() == "use_list");

                    if has_use_list {
                        // Recurse into the use list
                        self.extract_rust_use_tree(child, source, file_path, symbols, is_pub);
                    } else {
                        // Simple path like std::collections::HashMap
                        // Get the last identifier (the actual name being imported)
                        let mut last_ident = None;
                        let mut inner_cursor = child.walk();
                        for inner_child in child.children(&mut inner_cursor) {
                            if inner_child.kind() == "identifier" {
                                last_ident = self.node_text(inner_child, source);
                            } else if inner_child.kind() == "scoped_identifier" {
                                // For paths like std::collections::HashMap
                                if let Some(name_node) = inner_child.child_by_field_name("name") {
                                    last_ident = self.node_text(name_node, source);
                                }
                            }
                        }

                        if let Some(name) = last_ident {
                            symbols.push(Symbol {
                                name,
                                symbol_type,
                                visibility: None,
                                file_path: file_path.to_string(),
                                line_start: node.start_position().row + 1,
                                line_end: node.end_position().row + 1,
                                signature: None,
                                body: None,
                                language: Language::Rust,
                            });
                        }
                    }
                }
                "use_list" => {
                    // List like {HashMap, HashSet} or {self, Read, Write}
                    let mut list_cursor = child.walk();
                    for list_item in child.children(&mut list_cursor) {
                        if list_item.kind() == "identifier" {
                            if let Some(name) = self.node_text(list_item, source) {
                                // Skip 'self' as it's not a real import name
                                if name != "self" {
                                    symbols.push(Symbol {
                                        name,
                                        symbol_type,
                                        visibility: None,
                                        file_path: file_path.to_string(),
                                        line_start: node.start_position().row + 1,
                                        line_end: node.end_position().row + 1,
                                        signature: None,
                                        body: None,
                                        language: Language::Rust,
                                    });
                                }
                            }
                        } else if list_item.kind() == "use_as_clause" {
                            // Aliased import in list
                            if let Some(alias) = list_item.child_by_field_name("alias") {
                                if let Some(name) = self.node_text(alias, source) {
                                    symbols.push(Symbol {
                                        name,
                                        symbol_type,
                                        visibility: None,
                                        file_path: file_path.to_string(),
                                        line_start: node.start_position().row + 1,
                                        line_end: node.end_position().row + 1,
                                        signature: None,
                                        body: None,
                                        language: Language::Rust,
                                    });
                                }
                            }
                        }
                    }
                }
                "scoped_identifier" => {
                    // Path like std::collections::HashMap at top level
                    if let Some(name_node) = child.child_by_field_name("name") {
                        if let Some(name) = self.node_text(name_node, source) {
                            symbols.push(Symbol {
                                name,
                                symbol_type,
                                visibility: None,
                                file_path: file_path.to_string(),
                                line_start: node.start_position().row + 1,
                                line_end: node.end_position().row + 1,
                                signature: None,
                                body: None,
                                language: Language::Rust,
                            });
                        }
                    }
                }
                "identifier" => {
                    // Simple identifier at root
                    if let Some(name) = self.node_text(child, source) {
                        symbols.push(Symbol {
                            name,
                            symbol_type,
                            visibility: None,
                            file_path: file_path.to_string(),
                            line_start: node.start_position().row + 1,
                            line_end: node.end_position().row + 1,
                            signature: None,
                            body: None,
                            language: Language::Rust,
                        });
                    }
                }
                "use_as_clause" => {
                    // Aliased import
                    if let Some(alias) = child.child_by_field_name("alias") {
                        if let Some(name) = self.node_text(alias, source) {
                            symbols.push(Symbol {
                                name,
                                symbol_type,
                                visibility: None,
                                file_path: file_path.to_string(),
                                line_start: node.start_position().row + 1,
                                line_end: node.end_position().row + 1,
                                signature: None,
                                body: None,
                                language: Language::Rust,
                            });
                        }
                    }
                }
                _ => {
                    // Recurse for other nodes
                    self.extract_rust_use_tree(child, source, file_path, symbols, is_pub);
                }
            }
        }
    }

    /// Extract a Rust const symbol.
    fn extract_rust_const(&self, node: Node, source: &[u8], file_path: &str) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;
        let visibility = self.get_rust_visibility(node, source);

        Some(
            SymbolBuilder::new(name, SymbolType::Constant, file_path, node, Language::Rust)
                .visibility(visibility)
                .build(),
        )
    }

    /// Extract a Rust static symbol.
    fn extract_rust_static(&self, node: Node, source: &[u8], file_path: &str) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;
        let visibility = self.get_rust_visibility(node, source);

        // Static as Variable
        Some(
            SymbolBuilder::new(name, SymbolType::Variable, file_path, node, Language::Rust)
                .visibility(visibility)
                .build(),
        )
    }

    /// Extract a Rust type alias symbol.
    fn extract_rust_type_alias(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
    ) -> Option<Symbol> {
        let name_node = node.child_by_field_name("name")?;
        let name = self.node_text(name_node, source)?;
        let visibility = self.get_rust_visibility(node, source);

        // Type aliases as Variable
        Some(
            SymbolBuilder::new(name, SymbolType::Variable, file_path, node, Language::Rust)
                .visibility(visibility)
                .build(),
        )
    }

    /// Extract symbols from Markdown source code.
    fn extract_markdown_symbols(
        &self,
        tree: &Tree,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        let root = tree.root_node();
        self.walk_markdown_node(root, source, file_path, symbols);
    }

    /// Recursively walk Markdown AST nodes.
    fn walk_markdown_node(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
    ) {
        // ATX headings: # Heading
        if node.kind() == "atx_heading" {
            if let Some(symbol) = self.extract_markdown_heading(node, source, file_path) {
                symbols.push(symbol);
            }
        }

        // Setext headings: Heading\n===
        if node.kind() == "setext_heading" {
            if let Some(symbol) = self.extract_markdown_heading(node, source, file_path) {
                symbols.push(symbol);
            }
        }

        // Recurse
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_markdown_node(child, source, file_path, symbols);
        }
    }

    /// Extract a Markdown heading symbol.
    fn extract_markdown_heading(
        &self,
        node: Node,
        source: &[u8],
        file_path: &str,
    ) -> Option<Symbol> {
        // Get the heading content
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "heading_content" || child.kind() == "inline" {
                if let Some(text) = self.node_text(child, source) {
                    let name = text.trim().to_string();
                    if !name.is_empty() {
                        return Some(Symbol {
                            name,
                            symbol_type: SymbolType::Function, // Headings as Function for repo map
                            visibility: Some(Visibility::Public),
                            file_path: file_path.to_string(),
                            line_start: node.start_position().row + 1,
                            line_end: node.end_position().row + 1,
                            signature: None,
                            body: None,
                            language: Language::Markdown,
                        });
                    }
                }
            }
        }

        // Fallback: try to get direct text
        if let Some(text) = self.node_text(node, source) {
            let name = text.trim_start_matches('#').trim().to_string();
            if !name.is_empty() {
                return Some(Symbol {
                    name,
                    symbol_type: SymbolType::Function,
                    visibility: Some(Visibility::Public),
                    file_path: file_path.to_string(),
                    line_start: node.start_position().row + 1,
                    line_end: node.end_position().row + 1,
                    signature: None,
                    body: None,
                    language: Language::Markdown,
                });
            }
        }

        None
    }

    /// Create a bare symbol for import/export extraction.
    ///
    /// This helper eliminates duplication in import/export extraction methods.
    /// The resulting symbol has no visibility, signature, or body.
    fn make_bare_symbol(
        &self,
        name: String,
        symbol_type: SymbolType,
        file_path: &str,
        node: Node,
        language: Language,
    ) -> Symbol {
        Symbol {
            name,
            symbol_type,
            visibility: None,
            file_path: file_path.to_string(),
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            signature: None,
            body: None,
            language,
        }
    }

    /// Create a bare symbol with explicit visibility.
    ///
    /// Similar to [`make_bare_symbol`] but allows setting visibility,
    /// used for exported symbols that need `Some(Visibility::Public)`.
    fn make_bare_symbol_with_visibility(
        &self,
        name: String,
        symbol_type: SymbolType,
        visibility: Option<Visibility>,
        file_path: &str,
        node: Node,
        language: Language,
    ) -> Symbol {
        Symbol {
            name,
            symbol_type,
            visibility,
            file_path: file_path.to_string(),
            line_start: node.start_position().row + 1,
            line_end: node.end_position().row + 1,
            signature: None,
            body: None,
            language,
        }
    }

    /// Get text content of a node.
    fn node_text(&self, node: Node, source: &[u8]) -> Option<String> {
        let text = std::str::from_utf8(&source[node.start_byte()..node.end_byte()]).ok()?;
        Some(text.to_string())
    }

    /// Extract signature from source code up to a delimiter.
    ///
    /// This is a common pattern used for extracting function/class signatures
    /// across Python (colon delimiter) and TypeScript/Rust (brace delimiter).
    fn extract_signature_until(
        &self,
        node: Node,
        source: &[u8],
        delimiter: char,
    ) -> Option<String> {
        let start = node.start_byte();
        let text = std::str::from_utf8(&source[start..]).ok()?;

        if let Some(delimiter_pos) = text.find(delimiter) {
            let sig = text[..delimiter_pos].trim();
            return Some(sig.to_string());
        }

        None
    }

    /// Determine Python visibility based on naming convention.
    ///
    /// In Python, names starting with a single underscore (but not dunder)
    /// are considered private by convention.
    fn determine_python_visibility(&self, name: &str) -> Option<Visibility> {
        if name.starts_with('_') && !name.starts_with("__") {
            Some(Visibility::Private)
        } else {
            Some(Visibility::Public)
        }
    }

    /// Check if a Python name represents a constant (UPPER_CASE convention).
    fn is_python_constant(&self, name: &str) -> bool {
        name.chars()
            .all(|c| c.is_uppercase() || c == '_' || c.is_numeric())
            && name.chars().any(|c| c.is_alphabetic())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_type_display() {
        assert_eq!(SymbolType::Function.to_string(), "Function");
        assert_eq!(SymbolType::Class.to_string(), "Class");
    }

    #[test]
    fn test_parser_creation() {
        let parser = Parser::new(Language::Python);
        assert!(parser.is_ok());
    }

    #[test]
    fn test_parser_parse() {
        let parser = Parser::new(Language::Python).unwrap();
        let tree = parser.parse("def hello(): pass");
        assert!(tree.is_ok());
    }

    #[test]
    fn test_symbol_extractor_empty_content() {
        let extractor = SymbolExtractor::new();
        let result = extractor.extract_from_content("", Language::Python);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_symbol_serialize() {
        let symbol = Symbol {
            name: "test".to_string(),
            symbol_type: SymbolType::Function,
            visibility: Some(Visibility::Public),
            file_path: "test.py".to_string(),
            line_start: 1,
            line_end: 2,
            signature: None,
            body: None,
            language: Language::Python,
        };

        let json = serde_json::to_string(&symbol);
        assert!(json.is_ok());
    }
}
