# Treelint CLI Reference

Complete command-line interface documentation for Treelint v0.8.0.

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
| `--format <FORMAT>` | - | auto | Output format (`json` or `text`). Auto-detects: TTY=text, pipe=json |
| `--context <N>` | - | `0` | Number of context lines to include |
| `--signatures` | - | false | Only return function/method signatures (omits body) |

**Format Auto-Detection (STORY-005):**

When `--format` is not specified, the output format is auto-detected:
- **Terminal (TTY):** Text format with human-readable tree-style output
- **Piped (non-TTY):** JSON format for programmatic consumption

Use `--format json` or `--format text` to override auto-detection.

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

# Auto-detect (JSON when piped, text when terminal)
treelint search validateUser | jq '.results'
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
    "type": "function",
    "case_insensitive": true,
    "regex": true,
    "context_mode": "full"
  },
  "results": [
    {
      "type": "function",
      "name": "validateUser",
      "file": "src/auth/validator.py",
      "lines": [10, 45],
      "signature": "def validateUser(email: str, password: str) -> bool",
      "body": "def validateUser(email: str, password: str) -> bool:\n    ...",
      "language": "python"
    }
  ],
  "stats": {
    "files_searched": 150,
    "elapsed_ms": 47,
    "files_skipped": 0,
    "skipped_by_type": {},
    "languages_searched": ["python", "rust", "typescript"]
  }
}
```

**Query Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `symbol` | string | Yes | The search term |
| `type` | string | No | Symbol type filter (omitted if not specified) |
| `case_insensitive` | boolean | No | Present and `true` when `-i` flag used |
| `regex` | boolean | No | Present and `true` when `-r` flag used |
| `context_mode` | string | Yes | `"full"` (default) or `"signatures"` (with `--signatures`) |

**Result Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | Symbol type (function, class, method, etc.) |
| `name` | string | Yes | Symbol name |
| `file` | string | Yes | File path containing the symbol |
| `lines` | [int, int] | Yes | Line range [start, end] |
| `signature` | string | Yes | Function/method signature |
| `body` | string\|null | Yes | Full symbol body (null in signatures mode) |
| `language` | string | No | Programming language |

**Stats Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `files_searched` | int | Yes | Number of files searched |
| `elapsed_ms` | int | Yes | Search duration in milliseconds |
| `files_skipped` | int | Yes | Number of files skipped |
| `skipped_by_type` | object | Yes | Files skipped by reason |
| `languages_searched` | [string] | Yes | Languages that were searched |

### Text Format

```
No results found for: validateUser
```

Or when results exist:
```
function validateUser (src/auth/validator.py:10-45)
  def validateUser(email: str, password: str) -> bool
    # function body indented 4 spaces
    ...

Found 1 result in 47ms (150 files searched)
```

With `--signatures` flag (body omitted):
```
function validateUser (src/auth/validator.py:10-45)
  def validateUser(email: str, password: str) -> bool

Found 1 result in 47ms (150 files searched)
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
| `0` | Success (results found) |
| `1` | User error (invalid arguments, invalid regex) |
| `2` | No results found |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Set logging level (`debug`, `info`, `warn`, `error`) |

---

---

## Background Features

### File Watcher (via Daemon)

When using the daemon for instant queries, the file watcher automatically keeps the index fresh:

- **Automatic updates** - Index updates within 1 second of file changes
- **No manual re-indexing** - Changes to `.py`, `.ts`, `.tsx`, `.rs`, `.md` files are detected automatically
- **Hash-based detection** - Avoids redundant re-indexing when content unchanged (e.g., `touch` command)
- **Gitignore support** - Respects project ignore patterns

See [Daemon API Reference](daemon-api.md) for daemon protocol details.

---

**Version:** 0.8.0
**Updated:** 2026-01-30
**Source:** STORY-001, STORY-002, STORY-005, STORY-008
