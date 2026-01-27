# CLAUDE.md

## What You're Building

**Treelint** is a Rust-based code search CLI that uses tree-sitter AST parsing to reduce AI coding assistant token consumption by 40-80%. It returns semantic code units (functions, classes) instead of raw text matches.

---

## Project Context

| Attribute | Value |
|-----------|-------|
| **Language** | Rust |
| **Parser** | tree-sitter (embedded grammars) |
| **Output** | JSON structured format |
| **Platforms** | Windows, macOS, Linux |
| **Timeline** | MVP in < 2 weeks |

### Supported Languages (v1)

- Python
- TypeScript
- Rust
- Markdown

---

## Key Files

| File | Purpose |
|------|---------|
| `src/main.rs` | CLI entry point |
| `src/parser/` | tree-sitter parsing logic |
| `src/index/` | Background indexing service |
| `src/output/` | JSON output formatting |
| `Cargo.toml` | Rust dependencies |
| `devforgeai/specs/brainstorms/BRAINSTORM-001-treelint-ast-code-search.brainstorm.md` | Requirements brainstorm |
| `devforgeai/specs/research/RESEARCH-001-claude-code-search-token-efficiency.research.md` | Market research |

---

## CLI Interface

```bash
# Symbol-based search (works with or without daemon)
treelint search validateUser --type function
treelint --function validateUser
treelint --class AuthService

# With context lines
treelint --function validateUser --context 10

# Repository map
treelint map --output repo-map.json

# Index management (hybrid: daemon or on-demand)
treelint index --force          # Manual re-index
treelint daemon start           # Start background daemon
treelint daemon stop            # Stop daemon
treelint daemon status          # Check daemon status
```

### Indexing Architecture (Hybrid)

```
CLI auto-detects daemon availability:
  ├─ Daemon running → Query live index (instant)
  └─ No daemon → Build index on-demand (still fast)

Environment recommendations:
  • Dev machine: Use daemon (always-fresh index)
  • CI/CD: Manual index (treelint index --force)
  • Containers: Manual (short-lived, no daemon overhead)
```

---

## Output Format

```json
{
  "query": { "symbol": "validateUser", "type": "function" },
  "results": [{
    "type": "function",
    "name": "validateUser",
    "file": "src/auth/validator.py",
    "lines": [10, 45],
    "signature": "def validateUser(email: str, password: str) -> bool",
    "body": "..."
  }],
  "stats": { "files_searched": 150, "elapsed_ms": 47 }
}
```

---

## Development Rules

### Always

- Use `cargo test` before committing
- Run `cargo clippy` for linting
- Keep binary size < 50MB
- Embed grammars (no external files)

### Never

- Add runtime dependencies
- Require external grammar files
- Break cross-platform compatibility

---

## DevForgeAI Framework

This project uses DevForgeAI for spec-driven development.

### Workflow

```
/brainstorm → /ideate → /create-context → /create-epic → /dev → /qa → /release
```

### Key Commands

| Command | Purpose |
|---------|---------|
| `/ideate` | Transform brainstorm into requirements |
| `/create-context` | Generate tech-stack.md, source-tree.md |
| `/create-story` | Create user stories with AC |
| `/dev` | TDD implementation workflow |
| `/qa` | Quality validation |

### Context Files

| File | Purpose |
|------|---------|
| `devforgeai/specs/context/tech-stack.md` | Allowed technologies |
| `devforgeai/specs/context/source-tree.md` | File/folder structure |
| `devforgeai/specs/context/dependencies.md` | Cargo dependencies |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Treelint CLI                         │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐ │
│  │   Parser    │  │   Index     │  │     Output      │ │
│  │ (tree-sitter)│  │  (daemon)   │  │   (JSON/Text)   │ │
│  └──────┬──────┘  └──────┬──────┘  └────────┬────────┘ │
│         │                │                   │          │
│  ┌──────▼──────────────────────────────────▼────────┐  │
│  │                  Symbol Store                     │  │
│  │  (functions, classes, methods, variables)         │  │
│  └───────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────┤
│  Embedded Grammars: Python, TypeScript, Rust, Markdown  │
└─────────────────────────────────────────────────────────┘
```

---

## Research Foundation

See `devforgeai/specs/research/RESEARCH-001-claude-code-search-token-efficiency.research.md`:

- Claude Code uses ripgrep (text-based, 20-30% context utilization)
- Aider uses tree-sitter (4.3% context utilization)
- Potential 40-83% token reduction with AST-aware search

---

## When in Doubt

1. Check brainstorm: `BRAINSTORM-001-treelint-ast-code-search.brainstorm.md`
2. Check research: `RESEARCH-001-claude-code-search-token-efficiency.research.md`
3. Ask the user with `AskUserQuestion`
