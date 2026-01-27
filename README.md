# Treelint

**AST-aware code search CLI that reduces AI coding assistant token consumption by 40-80%**

Treelint uses tree-sitter to provide semantic code navigation, returning functions and classes instead of raw line matches. Purpose-built for AI assistants like Claude Code.

## Why Treelint?

Text-based search tools (grep, ripgrep) return false positives from comments, strings, and variable names. This wastes 40-83% of an AI's context window on irrelevant results.

| Tool | Context Utilization | Approach |
|------|---------------------|----------|
| ripgrep (current) | 20-30% | Text search |
| Treelint | 5-10% | AST-aware search |
| **Savings** | **40-80%** | Semantic filtering |

## Features

- **Symbol-based search** - Find functions, classes, methods by name
- **JSON output** - Structured data optimized for AI consumption
- **Configurable context** - Return N lines around symbols
- **Background indexing** - Pre-index for instant queries
- **Repository map** - Generate symbol summaries (Aider-style)
- **Dependency graph** - Track function call relationships

## Installation

```bash
# From crates.io (coming soon)
cargo install treelint

# From source
git clone https://github.com/yourusername/treelint
cd treelint
cargo build --release
```

## Usage

```bash
# Search for a function
treelint --function validateUser

# Search for a class
treelint --class AuthService

# Search with type filter
treelint validateUser --type function

# With context lines
treelint --function validateUser --context 10

# JSON output (default)
treelint --function validateUser --format json

# Generate repository map
treelint map --output repo-map.json
```

## Output Format

```json
{
  "results": [
    {
      "type": "function",
      "name": "validateUser",
      "file": "src/auth/validator.py",
      "lines": [10, 45],
      "signature": "def validateUser(email: str, password: str) -> bool",
      "body": "..."
    }
  ]
}
```

## Supported Languages

| Language | Status |
|----------|--------|
| Python | ✅ v1 |
| TypeScript | ✅ v1 |
| Rust | ✅ v1 |
| Markdown | ✅ v1 |

## Architecture

- **Rust** - Single binary, no runtime dependencies
- **tree-sitter** - Embedded grammars for all supported languages
- **Cross-platform** - Windows, macOS, Linux

## Development

```bash
# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- --function main

# Build release
cargo build --release
```

## Research

This project is informed by research into AI coding assistant token efficiency:
- [RESEARCH-005: Claude Code Search Token Efficiency](devforgeai/specs/research/RESEARCH-001-claude-code-search-token-efficiency.research.md)

## License

MIT

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.
