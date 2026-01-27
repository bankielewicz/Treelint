---
last_updated: 2026-01-27
status: LOCKED
requires_approval: Tech Lead
project: Treelint
---

# Coding Standards

**CRITICAL:** This document defines the coding standards AI agents MUST follow.

**All code MUST comply with these standards. Non-compliant code will fail review.**

---

## Rust Style Guide

### General Rules

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) and standard Rust conventions.

**Formatting:**
- Use `rustfmt` with default settings
- Run `cargo fmt` before every commit
- Maximum line length: 100 characters

**Linting:**
- Use `cargo clippy` with default warnings
- All clippy warnings must be resolved (not suppressed)
- Exception: Documented `#[allow(...)]` with justification

---

## Naming Conventions

### Files and Modules

```rust
✅ CORRECT:
  src/parser/symbols.rs         // snake_case
  src/index/relevance.rs        // descriptive
  tests/search_tests.rs         // _tests suffix for test files

❌ FORBIDDEN:
  src/Parser/Symbols.rs         // No PascalCase
  src/symbolExtractor.rs        // No camelCase
  src/s.rs                      // No single letters
```

### Types

```rust
✅ CORRECT:
  struct Symbol { ... }         // PascalCase for types
  enum SymbolType { ... }       // PascalCase for enums
  trait Parser { ... }          // PascalCase for traits
  type SymbolId = u64;          // PascalCase for type aliases

❌ FORBIDDEN:
  struct symbol { ... }         // No snake_case types
  enum SYMBOL_TYPE { ... }      // No SCREAMING_CASE
```

### Functions and Methods

```rust
✅ CORRECT:
  fn extract_symbols(...) { }   // snake_case
  fn parse_file(...) { }        // verb_noun pattern
  impl Symbol {
      fn get_signature(&self) { }  // snake_case methods
  }

❌ FORBIDDEN:
  fn ExtractSymbols(...) { }    // No PascalCase
  fn extractSymbols(...) { }    // No camelCase
  fn do_stuff(...) { }          // Be specific
```

### Constants and Statics

```rust
✅ CORRECT:
  const MAX_FILE_SIZE: usize = 10_000_000;  // SCREAMING_SNAKE_CASE
  static DEFAULT_CONFIG: Config = Config::default();

❌ FORBIDDEN:
  const maxFileSize: usize = 10_000_000;    // No camelCase
  const max_file_size: usize = 10_000_000;  // No snake_case for constants
```

### Variables

```rust
✅ CORRECT:
  let file_path = Path::new("...");        // snake_case
  let mut symbol_count = 0;                // descriptive
  let db_connection = Connection::open()?; // abbreviations OK if common

❌ FORBIDDEN:
  let fp = Path::new("...");               // No single letters (except loops)
  let filePath = Path::new("...");         // No camelCase
  let x = get_symbols()?;                  // Be descriptive
```

---

## Documentation

### Public API Documentation

Every public item MUST have documentation:

```rust
✅ CORRECT:
/// Extracts symbols from a source file using tree-sitter.
///
/// # Arguments
///
/// * `path` - Path to the source file
/// * `language` - The programming language to parse
///
/// # Returns
///
/// A vector of extracted symbols, or an error if parsing fails.
///
/// # Example
///
/// ```
/// let symbols = extract_symbols(Path::new("main.py"), Language::Python)?;
/// ```
pub fn extract_symbols(path: &Path, language: Language) -> Result<Vec<Symbol>> {
    // ...
}

❌ FORBIDDEN:
pub fn extract_symbols(path: &Path, language: Language) -> Result<Vec<Symbol>> {
    // No documentation
}
```

### Module Documentation

Each module MUST have a top-level doc comment:

```rust
✅ CORRECT:
//! Symbol extraction from source files using tree-sitter.
//!
//! This module provides functionality to parse source files
//! and extract semantic symbols (functions, classes, etc.).

pub mod symbols;
pub mod languages;
```

### Internal Comments

Use `//` comments for implementation details:

```rust
✅ CORRECT:
// Skip files larger than 10MB to avoid memory issues
if file_size > MAX_FILE_SIZE {
    return Err(Error::FileTooLarge);
}

❌ FORBIDDEN:
// increment i    // Obvious, don't comment
i += 1;
```

---

## Error Handling

### Use Result for Fallible Operations

```rust
✅ CORRECT:
pub fn parse_file(path: &Path) -> Result<Ast, ParseError> {
    let content = std::fs::read_to_string(path)?;
    // ...
}

❌ FORBIDDEN:
pub fn parse_file(path: &Path) -> Ast {
    let content = std::fs::read_to_string(path).unwrap();  // No unwrap in prod
    // ...
}
```

### Define Custom Error Types

```rust
✅ CORRECT:
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Parse error at line {line}: {message}")]
    SyntaxError { line: usize, message: String },
}

❌ FORBIDDEN:
// Using String as error type
pub fn parse_file(path: &Path) -> Result<Ast, String> { ... }

// Using anyhow in library code
pub fn parse_file(path: &Path) -> anyhow::Result<Ast> { ... }
```

### Anyhow for Application Code

```rust
✅ CORRECT:
// In main.rs or CLI code
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run(args)?;
    Ok(())
}

// In library code - use custom errors
pub fn parse_file(...) -> Result<..., ParseError> { ... }
```

---

## Testing

### Test Organization

```rust
✅ CORRECT:
// Unit tests at bottom of source file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_function_symbol() {
        let code = "def foo(): pass";
        let symbols = extract_symbols(code, Language::Python).unwrap();
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "foo");
    }
}

// Integration tests in tests/ directory
// tests/search_tests.rs
```

### Test Naming

```rust
✅ CORRECT:
#[test]
fn test_extract_function_returns_name_and_lines() { }

#[test]
fn test_search_with_regex_matches_pattern() { }

#[test]
fn test_index_skips_unsupported_files() { }

❌ FORBIDDEN:
#[test]
fn test1() { }           // Not descriptive

#[test]
fn it_works() { }        // Not specific

#[test]
fn test() { }            // Meaningless
```

### Assertions

```rust
✅ CORRECT:
use pretty_assertions::assert_eq;

assert_eq!(result.len(), 3, "Expected 3 symbols");
assert!(result.contains(&expected_symbol));
assert!(result.is_ok(), "Parse should succeed");

❌ FORBIDDEN:
assert!(result.len() == 3);  // Use assert_eq! for equality
// No message for complex assertions
```

---

## Module Structure

### Single Responsibility

Each module should have one clear purpose:

```rust
✅ CORRECT:
// src/parser/symbols.rs - Only symbol extraction
// src/parser/languages.rs - Only language detection
// src/index/search.rs - Only search queries

❌ FORBIDDEN:
// src/parser/everything.rs - Multiple responsibilities
// src/utils.rs - Generic dumping ground
```

### Public API

Expose minimal public API:

```rust
✅ CORRECT:
// src/parser/mod.rs
mod symbols;
mod languages;
mod internal;  // Private

pub use symbols::Symbol;
pub use languages::Language;

// Don't expose internal details
```

---

## Performance Guidelines

### Avoid Unnecessary Allocations

```rust
✅ CORRECT:
fn process_symbols(symbols: &[Symbol]) { }     // Borrow slice
fn get_name(&self) -> &str { }                 // Return reference

❌ FORBIDDEN:
fn process_symbols(symbols: Vec<Symbol>) { }   // Takes ownership unnecessarily
fn get_name(&self) -> String { }               // Allocates unnecessarily
```

### Use Iterators

```rust
✅ CORRECT:
let names: Vec<_> = symbols.iter()
    .filter(|s| s.is_public())
    .map(|s| &s.name)
    .collect();

❌ FORBIDDEN:
let mut names = Vec::new();
for s in symbols {
    if s.is_public() {
        names.push(&s.name);
    }
}
```

---

## Unsafe Code

### Prohibited Without Approval

```rust
✅ REQUIRES ADR:
// Must document why unsafe is necessary
// Must have safety comment
unsafe {
    // SAFETY: We've verified the pointer is valid and properly aligned
    // because [detailed explanation]
    std::ptr::read(ptr)
}

❌ FORBIDDEN:
unsafe {
    // No safety comment
    std::ptr::read(ptr)
}
```

---

## Imports

### Grouping

```rust
✅ CORRECT:
// Standard library
use std::collections::HashMap;
use std::path::Path;

// External crates
use clap::Parser;
use serde::{Deserialize, Serialize};

// Internal modules
use crate::parser::Symbol;
use crate::error::ParseError;

❌ FORBIDDEN:
// Mixed imports
use std::path::Path;
use crate::parser::Symbol;
use clap::Parser;
use std::collections::HashMap;
```

---

## Related Documents

- [tech-stack.md](./tech-stack.md) - Approved technologies
- [source-tree.md](./source-tree.md) - File structure
- [anti-patterns.md](./anti-patterns.md) - Forbidden patterns

---

## Change Log

| Date | Change | Approver | Reason |
|------|--------|----------|--------|
| 2026-01-27 | Initial version | Bryan | Project setup |
