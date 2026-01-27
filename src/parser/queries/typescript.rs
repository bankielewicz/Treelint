//! TypeScript tree-sitter queries for symbol extraction.
//!
//! This module contains query patterns for extracting symbols
//! from TypeScript and JavaScript source code using tree-sitter.
//!
//! # Supported Node Types
//!
//! - `function_declaration` - Regular and async function declarations
//! - `class_declaration` - Class definitions with methods
//! - `interface_declaration` - TypeScript interfaces
//! - `type_alias_declaration` - Type aliases (type X = ...)
//! - `import_statement` - ES6 imports
//! - `export_statement` - Named and default exports

/// Query string for extracting TypeScript function declarations.
///
/// Captures function declarations including async functions.
///
/// # Example Matched Code
///
/// ```text
/// function greet(name: string): string {
///     return `Hello, ${name}!`;
/// }
///
/// async function fetchData(url: string): Promise<object> {
///     return {};
/// }
/// ```
pub const FUNCTION_QUERY: &str = r#"
(function_declaration
  name: (identifier) @function.name
) @function.declaration
"#;

/// Query string for extracting TypeScript class declarations.
///
/// Captures class name, body, and heritage (extends/implements).
///
/// # Example Matched Code
///
/// ```text
/// class Calculator {
///     private value: number = 0;
///
///     add(x: number): number {
///         return this.value += x;
///     }
/// }
///
/// abstract class Shape {
///     abstract area(): number;
/// }
/// ```
pub const CLASS_QUERY: &str = r#"
(class_declaration
  name: (type_identifier) @class.name
  body: (class_body) @class.body
) @class.declaration
"#;

/// Query string for extracting TypeScript interface declarations.
///
/// Captures interface name and body.
///
/// # Example Matched Code
///
/// ```text
/// interface User {
///     id: string;
///     name: string;
/// }
///
/// interface Point {
///     x: number;
///     y: number;
/// }
/// ```
pub const INTERFACE_QUERY: &str = r#"
(interface_declaration
  name: (type_identifier) @interface.name
  body: (interface_body) @interface.body
) @interface.declaration
"#;

/// Query string for extracting TypeScript type alias declarations.
///
/// Captures type name and definition.
///
/// # Example Matched Code
///
/// ```text
/// type UserId = string;
/// type Point = { x: number; y: number };
/// type Result<T> = T | Error;
/// ```
pub const TYPE_ALIAS_QUERY: &str = r#"
(type_alias_declaration
  name: (type_identifier) @type.name
  value: (_) @type.value
) @type.declaration
"#;

/// Query string for extracting TypeScript import statements.
///
/// Captures ES6 imports including named, default, and namespace imports.
///
/// # Example Matched Code
///
/// ```text
/// import { readFile } from "fs/promises";
/// import path from "path";
/// import * as fs from "fs";
/// import type { Buffer } from "node:buffer";
/// ```
pub const IMPORT_QUERY: &str = r#"
(import_statement
  (import_clause
    (named_imports
      (import_specifier
        name: (identifier) @import.name
      )
    )
  )
) @import.statement

(import_statement
  (import_clause
    (identifier) @import.default
  )
) @import.default_statement

(import_statement
  (import_clause
    (namespace_import
      (identifier) @import.namespace
    )
  )
) @import.namespace_statement
"#;

/// Query string for extracting TypeScript export statements.
///
/// Captures export declarations for functions, classes, types, and re-exports.
///
/// # Example Matched Code
///
/// ```text
/// export const MAX_RETRIES = 3;
/// export function greet(name: string) {}
/// export class Calculator {}
/// export type UserId = string;
/// export interface User {}
/// export default class ApiClient {}
/// export { readFile as read } from "fs/promises";
/// ```
pub const EXPORT_QUERY: &str = r#"
(export_statement
  (function_declaration
    name: (identifier) @export.function
  )
) @export.function_statement

(export_statement
  (class_declaration
    name: (type_identifier) @export.class
  )
) @export.class_statement

(export_statement
  (lexical_declaration
    (variable_declarator
      name: (identifier) @export.variable
    )
  )
) @export.variable_statement

(export_statement
  (type_alias_declaration
    name: (type_identifier) @export.type
  )
) @export.type_statement

(export_statement
  (interface_declaration
    name: (type_identifier) @export.interface
  )
) @export.interface_statement

(export_statement
  (export_clause
    (export_specifier
      name: (identifier) @export.specifier_name
      alias: (identifier)? @export.specifier_alias
    )
  )
) @export.reexport
"#;

/// Query string for extracting TypeScript method definitions.
///
/// Captures methods within class bodies including constructors.
///
/// # Example Matched Code
///
/// ```text
/// class Example {
///     constructor() {}
///     public add(x: number): number { return x; }
///     private helper(): void {}
///     protected reset(): void {}
/// }
/// ```
pub const METHOD_QUERY: &str = r#"
(method_definition
  name: (property_identifier) @method.name
) @method.definition

(method_definition
  name: "constructor" @method.constructor
) @method.constructor_definition
"#;

/// Query string for extracting TypeScript lexical declarations.
///
/// Captures const and let variable declarations.
///
/// # Example Matched Code
///
/// ```text
/// const MAX_RETRIES = 3;
/// let counter = 0;
/// const multiply = (x: number, y: number) => x * y;
/// ```
pub const LEXICAL_QUERY: &str = r#"
(lexical_declaration
  (variable_declarator
    name: (identifier) @variable.name
    value: (_)? @variable.value
  )
) @variable.lexical
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_query_is_valid() {
        let language = tree_sitter_typescript::language_typescript();
        let query = tree_sitter::Query::new(&language, FUNCTION_QUERY);
        assert!(query.is_ok(), "Function query should be valid");
    }

    #[test]
    fn test_class_query_is_valid() {
        let language = tree_sitter_typescript::language_typescript();
        let query = tree_sitter::Query::new(&language, CLASS_QUERY);
        assert!(query.is_ok(), "Class query should be valid");
    }

    #[test]
    fn test_interface_query_is_valid() {
        let language = tree_sitter_typescript::language_typescript();
        let query = tree_sitter::Query::new(&language, INTERFACE_QUERY);
        assert!(query.is_ok(), "Interface query should be valid");
    }

    #[test]
    fn test_import_query_is_valid() {
        let language = tree_sitter_typescript::language_typescript();
        let query = tree_sitter::Query::new(&language, IMPORT_QUERY);
        assert!(query.is_ok(), "Import query should be valid");
    }

    #[test]
    fn test_export_query_is_valid() {
        let language = tree_sitter_typescript::language_typescript();
        let query = tree_sitter::Query::new(&language, EXPORT_QUERY);
        assert!(query.is_ok(), "Export query should be valid");
    }
}
