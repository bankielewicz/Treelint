# Treelint Library Reference

Complete API documentation for using Treelint as a Rust library.

---

## Overview

Treelint provides a Rust library for AST-aware code analysis using tree-sitter. The main entry points are:

- `SymbolExtractor` - Extract symbols from source files
- `Parser` - Low-level tree-sitter parsing
- `Language` - Language detection and grammar loading

---

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
treelint = "0.2.0"
```

Basic usage:

```rust
use std::path::Path;
use treelint::parser::{SymbolExtractor, Language};

fn main() -> Result<(), treelint::TreelintError> {
    let extractor = SymbolExtractor::new();

    // Extract from file (auto-detects language)
    let symbols = extractor.extract_from_file(Path::new("example.py"))?;

    for symbol in symbols {
        println!("{}: {} (lines {}-{})",
            symbol.symbol_type,
            symbol.name,
            symbol.line_start,
            symbol.line_end
        );
    }

    Ok(())
}
```

---

## Modules

### `treelint::parser`

Core parsing functionality.

**Re-exports:**
- `Language` - Language detection enum
- `Parser` - Tree-sitter parser wrapper
- `Symbol` - Extracted symbol data
- `SymbolExtractor` - Main extraction service
- `SymbolType` - Symbol type enum
- `Visibility` - Visibility modifier enum

### `treelint::error`

Error types.

**Re-exports:**
- `TreelintError` - Main error type

---

## Types

### `Language`

Supported programming languages for symbol extraction.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Python,
    TypeScript,
    Rust,
    Markdown,
}
```

**Methods:**

#### `from_extension(extension: &str) -> Option<Language>`

Detect language from a file extension.

| Parameter | Type | Description |
|-----------|------|-------------|
| `extension` | `&str` | File extension with leading dot (e.g., ".py") |

**Returns:** `Some(Language)` if recognized, `None` otherwise.

**Example:**

```rust
use treelint::parser::Language;

assert_eq!(Language::from_extension(".py"), Some(Language::Python));
assert_eq!(Language::from_extension(".ts"), Some(Language::TypeScript));
assert_eq!(Language::from_extension(".tsx"), Some(Language::TypeScript));
assert_eq!(Language::from_extension(".js"), Some(Language::TypeScript));
assert_eq!(Language::from_extension(".rs"), Some(Language::Rust));
assert_eq!(Language::from_extension(".md"), Some(Language::Markdown));
assert_eq!(Language::from_extension(".unknown"), None);
```

#### `from_path(path: &Path) -> Option<Language>`

Detect language from a file path.

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `&Path` | Path to the source file |

**Returns:** `Some(Language)` if the extension is recognized, `None` otherwise.

**Example:**

```rust
use std::path::Path;
use treelint::parser::Language;

let path = Path::new("src/main.py");
assert_eq!(Language::from_path(path), Some(Language::Python));
```

#### `tree_sitter_language(&self) -> tree_sitter::Language`

Get the embedded tree-sitter grammar for this language.

**Returns:** The `tree_sitter::Language` for parsing.

**Example:**

```rust
use treelint::parser::Language;

let ts_lang = Language::Python.tree_sitter_language();
assert!(ts_lang.version() > 0);
```

---

### `SymbolType`

The type of symbol extracted from source code.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolType {
    Function,   // Function definition
    Class,      // Class, struct, or interface
    Method,     // Method within a class
    Variable,   // Variable binding
    Constant,   // Constant value
    Import,     // Import statement
    Export,     // Export statement (TypeScript)
}
```

**Display Implementation:**

```rust
use treelint::parser::SymbolType;

let st = SymbolType::Function;
assert_eq!(format!("{}", st), "Function");
```

---

### `Visibility`

Visibility modifier for a symbol.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Visibility {
    Public,     // pub in Rust, no underscore in Python
    Private,    // no pub in Rust, leading underscore in Python
    Protected,  // protected in TypeScript
}
```

---

### `Symbol`

A symbol extracted from source code.

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub visibility: Option<Visibility>,
    pub file_path: String,
    pub line_start: usize,      // 1-indexed
    pub line_end: usize,        // 1-indexed
    pub signature: Option<String>,
    pub body: Option<String>,
    pub language: Language,
}
```

**Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | The name of the symbol |
| `symbol_type` | `SymbolType` | The type of symbol |
| `visibility` | `Option<Visibility>` | Visibility modifier (if applicable) |
| `file_path` | `String` | Path to the source file |
| `line_start` | `usize` | Starting line number (1-indexed) |
| `line_end` | `usize` | Ending line number (1-indexed) |
| `signature` | `Option<String>` | Symbol signature (e.g., function signature) |
| `body` | `Option<String>` | Full body (optional, for lazy loading) |
| `language` | `Language` | Language the symbol was extracted from |

**Serialization:**

Symbols implement `Serialize` and `Deserialize` for JSON output:

```rust
use treelint::parser::{Symbol, SymbolType, Language};

let symbol = Symbol {
    name: "hello".to_string(),
    symbol_type: SymbolType::Function,
    visibility: None,
    file_path: "example.py".to_string(),
    line_start: 1,
    line_end: 2,
    signature: Some("def hello() -> None".to_string()),
    body: None,
    language: Language::Python,
};

let json = serde_json::to_string(&symbol)?;
```

---

## Services

### `SymbolExtractor`

Main service for extracting symbols from source files.

```rust
#[derive(Default)]
pub struct SymbolExtractor;
```

**Methods:**

#### `new() -> Self`

Create a new SymbolExtractor.

```rust
use treelint::parser::SymbolExtractor;

let extractor = SymbolExtractor::new();
```

#### `extract_from_file(&self, path: &Path) -> Result<Vec<Symbol>, TreelintError>`

Extract symbols from a file path. Automatically detects language from extension.

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `&Path` | Path to the source file |

**Returns:** `Result<Vec<Symbol>, TreelintError>`

**Errors:**
- `TreelintError::Parse` - Unsupported file type
- `TreelintError::Io` - File read error

**Example:**

```rust
use std::path::Path;
use treelint::parser::SymbolExtractor;

let extractor = SymbolExtractor::new();
let symbols = extractor.extract_from_file(Path::new("src/main.py"))?;

for symbol in symbols {
    println!("{}: {}", symbol.symbol_type, symbol.name);
}
```

#### `extract_from_content(&self, content: &str, language: Language) -> Result<Vec<Symbol>, TreelintError>`

Extract symbols from source code content.

| Parameter | Type | Description |
|-----------|------|-------------|
| `content` | `&str` | The source code to parse |
| `language` | `Language` | The language of the source code |

**Returns:** `Result<Vec<Symbol>, TreelintError>`

**Example:**

```rust
use treelint::parser::{SymbolExtractor, Language};

let extractor = SymbolExtractor::new();
let code = r#"
def hello():
    print("Hello, world!")

class Greeter:
    def greet(self, name):
        print(f"Hello, {name}!")
"#;

let symbols = extractor.extract_from_content(code, Language::Python)?;

// symbols contains:
// - Function "hello"
// - Class "Greeter"
// - Method "greet"
```

---

### `Parser`

Low-level wrapper around tree-sitter's Parser for a specific language.

```rust
pub struct Parser {
    language: Language,
}
```

**Methods:**

#### `new(language: Language) -> Result<Self, TreelintError>`

Create a new parser for the specified language.

| Parameter | Type | Description |
|-----------|------|-------------|
| `language` | `Language` | The language to parse |

**Returns:** `Result<Parser, TreelintError>`

**Example:**

```rust
use treelint::parser::{Parser, Language};

let parser = Parser::new(Language::Python)?;
```

#### `parse(&self, source: &str) -> Result<Tree, TreelintError>`

Parse source code and return the syntax tree.

| Parameter | Type | Description |
|-----------|------|-------------|
| `source` | `&str` | The source code to parse |

**Returns:** `Result<tree_sitter::Tree, TreelintError>`

**Example:**

```rust
use treelint::parser::{Parser, Language};

let parser = Parser::new(Language::Python)?;
let tree = parser.parse("def hello(): pass")?;

let root = tree.root_node();
println!("Root node: {}", root.kind());
```

#### `language(&self) -> Language`

Get the language this parser is configured for.

**Returns:** `Language`

---

## Error Handling

### `TreelintError`

Main error type for Treelint operations.

```rust
#[derive(Debug, Error)]
pub enum TreelintError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("CLI error: {0}")]
    Cli(String),
}
```

**Variants:**

| Variant | Description |
|---------|-------------|
| `Io` | I/O error (file system operations) |
| `Parse` | Parsing error (tree-sitter or validation) |
| `Cli` | CLI error (argument validation) |

**Example:**

```rust
use treelint::TreelintError;
use treelint::parser::SymbolExtractor;
use std::path::Path;

let extractor = SymbolExtractor::new();
match extractor.extract_from_file(Path::new("nonexistent.py")) {
    Ok(symbols) => println!("Found {} symbols", symbols.len()),
    Err(TreelintError::Io(e)) => println!("File error: {}", e),
    Err(TreelintError::Parse(msg)) => println!("Parse error: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

---

## Supported Languages

| Language | File Extensions | Symbol Types |
|----------|-----------------|--------------|
| Python | `.py` | Function, Class, Method, Variable, Constant, Import |
| TypeScript | `.ts`, `.tsx`, `.js`, `.jsx` | Function, Class, Method, Variable, Constant, Import, Export |
| Rust | `.rs` | Function, Class (struct/impl), Method, Variable, Constant, Import |
| Markdown | `.md`, `.markdown` | Function (headings) |

---

## Performance

- Grammar initialization: < 10ms per language
- File parsing: < 50ms for files up to 10,000 lines
- Memory-safe: No panics on malformed input

---

## Thread Safety

- `Language` is `Copy` and thread-safe
- `SymbolExtractor` is stateless and can be shared across threads
- `Parser` creates a new tree-sitter parser per call (no shared state)

---

---

## Daemon Module

### `treelint::daemon`

Background daemon functionality including file watching and incremental indexing.

**Re-exports:**
- `DaemonServer` - Main daemon server
- `FileWatcher` - File change monitoring
- `IncrementalIndexer` - Single-file re-indexing
- `WatcherStatus` - Watcher state information
- `WatcherEvent` - File change event
- `WatcherEventKind` - Event type enum

### `FileWatcher`

Cross-platform file watcher that monitors source files for changes.

```rust
pub struct FileWatcher {
    // ... internal state
}
```

**Constructor:**

#### `new(project_root: &Path) -> Result<Self, TreelintError>`

Create a new file watcher for the given project directory.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project_root` | `&Path` | Root directory to monitor |

**Returns:** `Result<FileWatcher, TreelintError>`

**Example:**

```rust
use std::path::Path;
use treelint::daemon::FileWatcher;

let watcher = FileWatcher::new(Path::new("/path/to/project"))?;
```

**Methods:**

#### `poll_events(&self, timeout: Duration) -> Vec<WatcherEvent>`

Poll for pending file change events.

| Parameter | Type | Description |
|-----------|------|-------------|
| `timeout` | `Duration` | Maximum time to wait for events |

**Returns:** Vector of `WatcherEvent` representing file changes.

#### `status(&self) -> WatcherStatus`

Get current watcher status including event counts and error information.

**Returns:** `WatcherStatus` struct.

#### `state(&self) -> DaemonState`

Get current daemon state (starting, ready, indexing, stopping).

**Returns:** `DaemonState` enum.

---

### `IncrementalIndexer`

Service for re-indexing individual files efficiently.

```rust
pub struct IncrementalIndexer {
    // ... internal state
}
```

**Constructor:**

#### `new(project_root: &Path) -> Result<Self, TreelintError>`

Create a new incremental indexer.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project_root` | `&Path` | Root directory of the project |

**Methods:**

#### `reindex_file(&self, path: &Path) -> Result<IndexStats, TreelintError>`

Re-index a single file, replacing its symbols in the index.

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `&Path` | Path to the file to re-index |

**Returns:** `Result<IndexStats, TreelintError>` with statistics about the operation.

**Example:**

```rust
use std::path::Path;
use treelint::daemon::IncrementalIndexer;

let indexer = IncrementalIndexer::new(Path::new("/path/to/project"))?;
let stats = indexer.reindex_file(Path::new("src/main.py"))?;

println!("Added {} symbols, removed {}", stats.symbols_added, stats.symbols_removed);
```

---

### `WatcherEventKind`

Type of file system event detected.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatcherEventKind {
    Create,  // New file created
    Modify,  // File content changed
    Delete,  // File deleted
    Rename,  // File renamed
}
```

---

### `WatcherStatus`

Current state of the file watcher.

```rust
pub struct WatcherStatus {
    pub enabled: bool,          // Whether watching is active
    pub watching_paths: usize,  // Number of monitored paths
    pub events_processed: u64,  // Total events handled
    pub errors_count: u32,      // Number of errors encountered
    pub last_event: Option<String>, // ISO timestamp of last event
}
```

---

### `IndexStats`

Statistics from an indexing operation.

```rust
pub struct IndexStats {
    pub symbols_added: usize,   // New symbols added
    pub symbols_removed: usize, // Old symbols removed
    pub parse_time_ms: u64,     // Time spent parsing
}
```

---

### `HashCache`

Thread-safe cache for file content hashes (SHA-256).

```rust
pub struct HashCache {
    // ... internal state
}
```

**Methods:**

#### `compute_hash(&self, path: &Path) -> Result<String, TreelintError>`

Compute SHA-256 hash of a file's contents.

#### `sha256_hash(data: &[u8]) -> String`

Compute SHA-256 hash of raw bytes (static method).

**Example:**

```rust
use treelint::daemon::HashCache;

let cache = HashCache::new();
let hash = cache.compute_hash(Path::new("src/main.py"))?;
println!("File hash: {}", hash);

// Static usage
let hash = HashCache::sha256_hash(b"Hello, world!");
```

---

**Version:** 0.8.0
**Generated:** 2026-01-30
**Source:** STORY-002, STORY-008
