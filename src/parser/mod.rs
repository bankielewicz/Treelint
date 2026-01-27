//! Symbol extraction from source files using tree-sitter.
//!
//! This module provides functionality to parse source files and extract
//! semantic symbols (functions, classes, methods, imports, etc.) using
//! tree-sitter with embedded grammars.
//!
//! # Supported Languages
//!
//! - Python (.py)
//! - TypeScript (.ts, .tsx, .js, .jsx)
//! - Rust (.rs)
//! - Markdown (.md, .markdown)
//!
//! # Example
//!
//! ```no_run
//! use std::path::Path;
//! use treelint::parser::{SymbolExtractor, Language};
//!
//! let extractor = SymbolExtractor::new();
//! let symbols = extractor.extract_from_file(Path::new("example.py"))?;
//!
//! for symbol in symbols {
//!     println!("{}: {} (lines {}-{})", symbol.symbol_type, symbol.name, symbol.line_start, symbol.line_end);
//! }
//! # Ok::<(), treelint::TreelintError>(())
//! ```

pub mod languages;
pub mod queries;
pub mod symbols;

pub use languages::Language;
pub use symbols::{Parser, Symbol, SymbolExtractor, SymbolType, Visibility};
