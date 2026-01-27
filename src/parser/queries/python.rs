//! Python tree-sitter queries for symbol extraction.
//!
//! This module contains query patterns for extracting symbols
//! from Python source code using tree-sitter.
//!
//! # Supported Node Types
//!
//! - `function_definition` - Regular and async function definitions
//! - `class_definition` - Class definitions with decorators
//! - `import_statement` - import os, import sys
//! - `import_from_statement` - from pathlib import Path

/// Query string for extracting Python function definitions.
///
/// Captures both regular functions and async functions.
///
/// # Example Matched Code
///
/// ```text
/// def greet(name: str) -> str:
///     return f"Hello, {name}!"
///
/// async def fetch_data(url: str) -> dict:
///     return {}
/// ```
pub const FUNCTION_QUERY: &str = r#"
(function_definition
  name: (identifier) @function.name
) @function.definition
"#;

/// Query string for extracting Python class definitions.
///
/// Captures class name and body, including decorated classes.
///
/// # Example Matched Code
///
/// ```text
/// class Calculator:
///     def __init__(self):
///         self.value = 0
///
/// @dataclass
/// class Point:
///     x: float
///     y: float
/// ```
pub const CLASS_QUERY: &str = r#"
(class_definition
  name: (identifier) @class.name
  body: (block) @class.body
) @class.definition
"#;

/// Query string for extracting Python import statements.
///
/// Captures both `import` and `from ... import` statements.
///
/// # Example Matched Code
///
/// ```text
/// import os
/// import sys as system
/// from pathlib import Path
/// from typing import Dict, List, Optional
/// from collections import defaultdict as dd
/// ```
pub const IMPORT_QUERY: &str = r#"
(import_statement
  name: (dotted_name) @import.name
) @import.statement

(import_from_statement
  module_name: (dotted_name) @import.module
) @import.from_statement

(aliased_import
  name: (dotted_name) @import.original
  alias: (identifier) @import.alias
) @import.aliased
"#;

/// Query string for extracting Python assignments (variables/constants).
///
/// Captures module-level assignments for variables and constants.
/// Constants are identified by UPPER_CASE naming convention.
///
/// # Example Matched Code
///
/// ```text
/// MAX_RETRIES = 3
/// DEFAULT_TIMEOUT = 30.0
/// _cache = {}
/// counter = 0
/// ```
pub const ASSIGNMENT_QUERY: &str = r#"
(expression_statement
  (assignment
    left: (identifier) @variable.name
    right: (_) @variable.value
  )
) @variable.assignment
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_query_is_valid() {
        let language = tree_sitter_python::language();
        let query = tree_sitter::Query::new(&language, FUNCTION_QUERY);
        assert!(query.is_ok(), "Function query should be valid");
    }

    #[test]
    fn test_class_query_is_valid() {
        let language = tree_sitter_python::language();
        let query = tree_sitter::Query::new(&language, CLASS_QUERY);
        assert!(query.is_ok(), "Class query should be valid");
    }

    #[test]
    fn test_import_query_is_valid() {
        let language = tree_sitter_python::language();
        let query = tree_sitter::Query::new(&language, IMPORT_QUERY);
        assert!(query.is_ok(), "Import query should be valid");
    }

    #[test]
    fn test_assignment_query_is_valid() {
        let language = tree_sitter_python::language();
        let query = tree_sitter::Query::new(&language, ASSIGNMENT_QUERY);
        assert!(query.is_ok(), "Assignment query should be valid");
    }
}
