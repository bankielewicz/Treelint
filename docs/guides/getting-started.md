# Getting Started with Treelint

A quick guide to installing and using Treelint for AST-aware code search.

---

## Prerequisites

- **Rust 1.70.0+** (for building from source)
- **Git** (for cloning the repository)

---

## Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/treelint/treelint
cd treelint

# Build release binary
cargo build --release

# Binary location: target/release/treelint
```

### Add to PATH (Optional)

```bash
# Linux/macOS
cp target/release/treelint ~/.local/bin/

# Windows (PowerShell)
Copy-Item target\release\treelint.exe $env:USERPROFILE\.local\bin\
```

---

## Quick Start

### 1. Verify Installation

```bash
treelint --version
# Output: treelint 0.8.0

treelint --help
# Shows available commands and options
```

### 2. Search for a Symbol

```bash
# Basic search
treelint search main

# Search for functions only
treelint search main --type function

# Get JSON output
treelint search main --format json
```

### 3. Understanding Output

**Text Output (default):**
```
No results found for: main
```

**JSON Output:**
```json
{
  "query": {"symbol": "main"},
  "results": [],
  "stats": {"files_searched": 0, "elapsed_ms": 0}
}
```

> **Note:** v0.8.0 includes full AST parsing, background daemon, and file watching.

---

## Common Use Cases

### AI Assistant Integration

Treelint is optimized for AI coding assistants like Claude Code:

```bash
# From Claude Code's Bash tool
treelint search validateUser --type function --format json
```

The JSON output can be parsed directly by the AI for context-efficient code navigation.

### Finding Functions

```bash
# Find a specific function
treelint search handleRequest --type function

# Find functions matching a pattern
treelint search "handle*" -r --type function
```

### Case-Insensitive Search

```bash
# Find regardless of case
treelint search validateuser -i
```

---

## Feature Status (v0.8.0)

| Feature | Status |
|---------|--------|
| CLI argument parsing | ✅ Complete |
| JSON/text output formats | ✅ Complete |
| AST parsing (tree-sitter) | ✅ Complete |
| SQLite index storage | ✅ Complete |
| Background daemon | ✅ Complete |
| File watcher | ✅ Complete |
| Repository mapping | Coming soon |

---

## Next Steps

1. **Explore the CLI:** Try different search options
2. **Read the API Reference:** See [CLI Reference](../api/cli-reference.md)
3. **Follow Development:** Watch for updates with actual AST parsing

---

## Troubleshooting

### "Command not found"

Ensure the binary is in your PATH or use the full path:
```bash
./target/release/treelint --version
```

### Build Failures

Ensure Rust 1.70.0+ is installed:
```bash
rustc --version
# Should be 1.70.0 or higher
```

---

**Version:** 0.8.0
**Generated:** 2026-01-30
**Source:** STORY-001 through STORY-008
