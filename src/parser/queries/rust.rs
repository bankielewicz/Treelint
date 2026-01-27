//! Rust tree-sitter queries for symbol extraction.
//!
//! This module contains query patterns for extracting symbols
//! from Rust source code using tree-sitter.
//!
//! # Supported Node Types
//!
//! - `function_item` - Function definitions (fn)
//! - `struct_item` - Struct definitions
//! - `enum_item` - Enum definitions
//! - `trait_item` - Trait definitions
//! - `impl_item` - Implementation blocks
//! - `use_declaration` - Use statements
//! - `const_item` - Constants
//! - `static_item` - Static variables
//! - `type_item` - Type aliases

/// Query string for extracting Rust function items.
///
/// Captures function definitions including visibility modifiers
/// and generic parameters.
///
/// # Example Matched Code
///
/// ```text
/// fn greet(name: &str) -> String {
///     format!("Hello, {}!", name)
/// }
///
/// pub fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
///
/// fn identity<T>(x: T) -> T {
///     x
/// }
/// ```
pub const FUNCTION_QUERY: &str = r#"
(function_item
  name: (identifier) @function.name
  parameters: (parameters) @function.params
) @function.item
"#;

/// Query string for extracting Rust struct items.
///
/// Captures struct definitions with visibility and fields.
///
/// # Example Matched Code
///
/// ```text
/// struct Point {
///     x: f64,
///     y: f64,
/// }
///
/// pub struct Calculator {
///     value: f64,
/// }
/// ```
pub const STRUCT_QUERY: &str = r#"
(struct_item
  name: (type_identifier) @struct.name
) @struct.item
"#;

/// Query string for extracting Rust enum items.
///
/// Captures enum definitions with variants.
///
/// # Example Matched Code
///
/// ```text
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
///
/// pub enum Result<T, E> {
///     Ok(T),
///     Err(E),
/// }
/// ```
pub const ENUM_QUERY: &str = r#"
(enum_item
  name: (type_identifier) @enum.name
) @enum.item
"#;

/// Query string for extracting Rust trait items.
///
/// Captures trait definitions with methods.
///
/// # Example Matched Code
///
/// ```text
/// trait Shape {
///     fn area(&self) -> f64;
/// }
///
/// pub trait Display {
///     fn fmt(&self, f: &mut Formatter) -> Result;
/// }
/// ```
pub const TRAIT_QUERY: &str = r#"
(trait_item
  name: (type_identifier) @trait.name
) @trait.item
"#;

/// Query string for extracting Rust impl items.
///
/// Captures implementation blocks including trait implementations.
///
/// # Example Matched Code
///
/// ```text
/// impl Calculator {
///     pub fn new() -> Self {
///         Self { value: 0.0 }
///     }
///
///     pub fn add(&mut self, x: f64) {
///         self.value += x;
///     }
/// }
///
/// impl Display for Calculator {
///     fn fmt(&self, f: &mut Formatter) -> Result {
///         write!(f, "{}", self.value)
///     }
/// }
/// ```
pub const IMPL_QUERY: &str = r#"
(impl_item
  type: (type_identifier) @impl.type
  body: (declaration_list) @impl.body
) @impl.item
"#;

/// Query string for extracting Rust use declarations.
///
/// Captures use statements including aliased imports and re-exports.
///
/// # Example Matched Code
///
/// ```text
/// use std::collections::HashMap;
/// use std::io::{self, Read, Write};
/// use std::path::PathBuf;
/// pub use std::fs::File;
/// ```
pub const USE_QUERY: &str = r#"
(use_declaration) @use.declaration
"#;

/// Query string for extracting Rust const items.
///
/// Captures constant definitions with type and value.
///
/// # Example Matched Code
///
/// ```text
/// const MAX_RETRIES: u32 = 3;
/// pub const DEFAULT_TIMEOUT: u64 = 30;
/// ```
pub const CONST_QUERY: &str = r#"
(const_item
  name: (identifier) @const.name
  type: (_) @const.type
  value: (_) @const.value
) @const.item
"#;

/// Query string for extracting Rust static items.
///
/// Captures static variable definitions.
///
/// # Example Matched Code
///
/// ```text
/// static COUNTER: AtomicUsize = AtomicUsize::new(0);
/// pub static mut BUFFER: [u8; 1024] = [0; 1024];
/// ```
pub const STATIC_QUERY: &str = r#"
(static_item
  name: (identifier) @static.name
  type: (_) @static.type
) @static.item
"#;

/// Query string for extracting Rust type aliases.
///
/// Captures type alias definitions.
///
/// # Example Matched Code
///
/// ```text
/// type Result<T> = std::result::Result<T, Error>;
/// pub type Cache = HashMap<String, Vec<u8>>;
/// ```
pub const TYPE_QUERY: &str = r#"
(type_item
  name: (type_identifier) @type.name
  type: (_) @type.definition
) @type.item
"#;

/// Query string for extracting Rust mod items.
///
/// Captures module declarations and definitions.
///
/// # Example Matched Code
///
/// ```text
/// mod parser;
/// pub mod utils;
///
/// mod internal {
///     fn helper() {}
/// }
/// ```
pub const MOD_QUERY: &str = r#"
(mod_item
  name: (identifier) @mod.name
) @mod.item
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_query_is_valid() {
        let language = tree_sitter_rust::language();
        let query = tree_sitter::Query::new(&language, FUNCTION_QUERY);
        assert!(query.is_ok(), "Function query should be valid");
    }

    #[test]
    fn test_struct_query_is_valid() {
        let language = tree_sitter_rust::language();
        let query = tree_sitter::Query::new(&language, STRUCT_QUERY);
        assert!(query.is_ok(), "Struct query should be valid");
    }

    #[test]
    fn test_enum_query_is_valid() {
        let language = tree_sitter_rust::language();
        let query = tree_sitter::Query::new(&language, ENUM_QUERY);
        assert!(query.is_ok(), "Enum query should be valid");
    }

    #[test]
    fn test_trait_query_is_valid() {
        let language = tree_sitter_rust::language();
        let query = tree_sitter::Query::new(&language, TRAIT_QUERY);
        assert!(query.is_ok(), "Trait query should be valid");
    }

    #[test]
    fn test_impl_query_is_valid() {
        let language = tree_sitter_rust::language();
        let query = tree_sitter::Query::new(&language, IMPL_QUERY);
        assert!(query.is_ok(), "Impl query should be valid");
    }

    #[test]
    fn test_use_query_is_valid() {
        let language = tree_sitter_rust::language();
        let query = tree_sitter::Query::new(&language, USE_QUERY);
        assert!(query.is_ok(), "Use query should be valid");
    }

    #[test]
    fn test_const_query_is_valid() {
        let language = tree_sitter_rust::language();
        let query = tree_sitter::Query::new(&language, CONST_QUERY);
        assert!(query.is_ok(), "Const query should be valid");
    }

    #[test]
    fn test_static_query_is_valid() {
        let language = tree_sitter_rust::language();
        let query = tree_sitter::Query::new(&language, STATIC_QUERY);
        assert!(query.is_ok(), "Static query should be valid");
    }

    #[test]
    fn test_type_query_is_valid() {
        let language = tree_sitter_rust::language();
        let query = tree_sitter::Query::new(&language, TYPE_QUERY);
        assert!(query.is_ok(), "Type query should be valid");
    }
}
