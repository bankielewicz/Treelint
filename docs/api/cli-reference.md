# Treelint CLI Reference

Complete command-line interface documentation for Treelint v0.2.0.

---

## Global Options

| Option | Description |
|--------|-------------|
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

---

## Commands

### `search`

Search for symbols in the codebase.

**Synopsis:**
```bash
treelint search <SYMBOL> [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `<SYMBOL>` | Yes | Symbol name or pattern to search for |

**Options:**

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--type <TYPE>` | - | Any | Filter by symbol type |
| `--ignore-case` | `-i` | false | Case-insensitive search |
| `--regex` | `-r` | false | Interpret symbol as a regex pattern |
| `--format <FORMAT>` | - | `text` | Output format (`json` or `text`) |
| `--context <N>` | - | `0` | Number of context lines to include |
| `--signatures` | - | false | Only return function/method signatures |

**Symbol Types:**

| Type | Description |
|------|-------------|
| `function` | Function definitions |
| `class` | Class definitions |
| `method` | Method definitions |
| `variable` | Variable declarations |
| `constant` | Constant declarations |
| `import` | Import statements |
| `export` | Export statements |

---

## Examples

### Basic Search

```bash
# Search for a symbol by name
treelint search validateUser

# Search for functions only
treelint search validateUser --type function

# Case-insensitive search
treelint search validateuser -i
```

### Output Formats

```bash
# JSON output (machine-readable)
treelint search validateUser --format json

# Text output (human-readable)
treelint search validateUser --format text
```

### Context Lines

```bash
# Include 10 lines of context around matches
treelint search validateUser --context 10

# Signatures only (no body)
treelint search validateUser --signatures
```

### Regex Search

```bash
# Search using regex pattern
treelint search "validate.*" -r

# Combine with type filter
treelint search "^handle" -r --type function
```

---

## Output Schema

### JSON Format

```json
{
  "query": {
    "symbol": "validateUser",
    "type": "function"
  },
  "results": [
    {
      "type": "function",
      "name": "validateUser",
      "file": "src/auth/validator.py",
      "lines": [10, 45],
      "signature": "def validateUser(email: str, password: str) -> bool",
      "body": "..."
    }
  ],
  "stats": {
    "files_searched": 150,
    "elapsed_ms": 47
  }
}
```

### Text Format

```
No results found for: validateUser
```

Or when results exist:
```
validateUser (function)
  File: src/auth/validator.py
  Lines: 10-45
  Signature: def validateUser(email: str, password: str) -> bool
```

---

## Supported Languages

Treelint uses tree-sitter with embedded grammars to parse the following languages:

| Language | File Extensions | Notes |
|----------|-----------------|-------|
| Python | `.py` | Functions, classes, methods, imports |
| TypeScript | `.ts`, `.tsx` | Full ES6+ support including exports |
| JavaScript | `.js`, `.jsx` | Uses TypeScript grammar |
| Rust | `.rs` | Functions, structs, impls, modules |
| Markdown | `.md`, `.markdown` | Extracts headings as symbols |

**Note:** Language is auto-detected from file extensions. Unsupported file types are skipped during search.

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | User error (invalid arguments) |
| `2` | No results found |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Set logging level (`debug`, `info`, `warn`, `error`) |

---

**Version:** 0.2.0
**Generated:** 2026-01-27
**Source:** STORY-001, STORY-002
