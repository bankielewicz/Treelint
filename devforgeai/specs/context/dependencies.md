---
last_updated: 2026-01-27
status: LOCKED
requires_approval: Tech Lead
project: Treelint
---

# Approved Dependencies

**CRITICAL:** This document defines the ONLY approved Cargo dependencies for this project.

**AI agents MUST NOT add dependencies not listed here without explicit approval via AskUserQuestion.**

---

## Cargo.toml - Dependencies

### Core Dependencies

```toml
[dependencies]
# CLI Framework
clap = { version = "4.5", features = ["derive", "cargo"] }

# tree-sitter Parsing
tree-sitter = "0.22"
tree-sitter-python = "0.21"
tree-sitter-typescript = "0.21"
tree-sitter-rust = "0.21"
tree-sitter-md = "0.2"  # Markdown

# Database
rusqlite = { version = "0.31", features = ["bundled"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# Error Handling
thiserror = "1.0"
anyhow = "1.0"

# File System
walkdir = "2.5"
notify = "6.1"

# Regex
regex = "1.10"

# Terminal UI
indicatif = { version = "0.17", features = ["rayon"] }
colored = "2.1"

# Logging
log = "0.4"
env_logger = "0.11"

# TTY Detection
atty = "0.2"

# IPC (Daemon)
interprocess = "2.0"

# Cryptographic Hashing (ADR-002)
sha2 = "0.10"
```

### Development Dependencies

```toml
[dev-dependencies]
# Testing
pretty_assertions = "1.4"
tempfile = "3.10"
assert_cmd = "2.0"
predicates = "3.1"

# Benchmarking
criterion = { version = "0.5", features = ["html_reports"] }
```

### Build Dependencies

```toml
[build-dependencies]
# tree-sitter grammar compilation
cc = "1.0"
```

---

## Dependency Rules

### Rule 1: Version Pinning

```toml
✅ CORRECT:
clap = "4.5"           # Minor version pinned
rusqlite = "0.31"      # Minor version pinned

❌ FORBIDDEN:
clap = "*"             # Never use wildcard
clap = ">=4.0"         # Too permissive
```

### Rule 2: Feature Flags

Only enable features explicitly listed:

```toml
✅ CORRECT:
rusqlite = { version = "0.31", features = ["bundled"] }
serde = { version = "1.0", features = ["derive"] }

❌ FORBIDDEN:
rusqlite = { version = "0.31", features = ["bundled", "backup", "blob"] }  # Unnecessary features
```

### Rule 3: No Duplicate Functionality

```
✅ APPROVED:
  regex (pattern matching)

❌ FORBIDDEN:
  fancy-regex (overlaps with regex)
  pcre2 (overlaps with regex)
```

---

## Dependency Categories

### Category: CLI

| Crate | Version | Purpose | Status |
|-------|---------|---------|--------|
| clap | 4.5 | Argument parsing | ✅ Approved |

**Prohibited Alternatives:**
- ❌ structopt (deprecated)
- ❌ argh (less feature-rich)
- ❌ pico-args (too minimal)

### Category: Parsing

| Crate | Version | Purpose | Status |
|-------|---------|---------|--------|
| tree-sitter | 0.22 | AST parsing core | ✅ Approved |
| tree-sitter-python | 0.21 | Python grammar | ✅ Approved |
| tree-sitter-typescript | 0.21 | TypeScript grammar | ✅ Approved |
| tree-sitter-rust | 0.21 | Rust grammar | ✅ Approved |
| tree-sitter-md | 0.2 | Markdown grammar | ✅ Approved |

**Prohibited Alternatives:**
- ❌ syn (Rust-only)
- ❌ pest (less ecosystem)
- ❌ nom (too low-level for AST)

### Category: Database

| Crate | Version | Purpose | Status |
|-------|---------|---------|--------|
| rusqlite | 0.31 | SQLite bindings | ✅ Approved |

**Required Features:** `bundled`

**Prohibited Alternatives:**
- ❌ diesel (ORM overhead)
- ❌ sqlx (async complexity not needed)
- ❌ sled (not SQL)

### Category: Serialization

| Crate | Version | Purpose | Status |
|-------|---------|---------|--------|
| serde | 1.0 | Serialization framework | ✅ Approved |
| serde_json | 1.0 | JSON support | ✅ Approved |
| toml | 0.8 | TOML config support | ✅ Approved |

**Prohibited Alternatives:**
- ❌ simd-json (premature optimization)
- ❌ json (less feature-rich)

### Category: Error Handling

| Crate | Version | Purpose | Status |
|-------|---------|---------|--------|
| thiserror | 1.0 | Error derive macros | ✅ Approved |
| anyhow | 1.0 | Application errors | ✅ Approved |

**Prohibited Alternatives:**
- ❌ error-chain (deprecated pattern)
- ❌ failure (deprecated)

### Category: File System

| Crate | Version | Purpose | Status |
|-------|---------|---------|--------|
| walkdir | 2.5 | Directory traversal | ✅ Approved |
| notify | 6.1 | File watching | ✅ Approved |

**Prohibited Alternatives:**
- ❌ glob (walkdir is more efficient)
- ❌ hotwatch (wrapper around notify)

### Category: Cryptography

| Crate | Version | Purpose | Status |
|-------|---------|---------|--------|
| sha2 | 0.10 | SHA-256 hashing (file change detection) | ✅ Approved |

**Prohibited Alternatives:**
- ❌ ring (heavier than needed)
- ❌ md5 (deprecated for security)
- ❌ openssl (external C dependency)

### Category: Terminal UI

| Crate | Version | Purpose | Status |
|-------|---------|---------|--------|
| indicatif | 0.17 | Progress bars | ✅ Approved |
| colored | 2.1 | Colored output | ✅ Approved |
| atty | 0.2 | TTY detection | ✅ Approved |

**Prohibited Alternatives:**
- ❌ termion (less cross-platform)
- ❌ crossterm (more than needed)
- ❌ tui/ratatui (full TUI not needed)

### Category: Testing

| Crate | Version | Purpose | Status |
|-------|---------|---------|--------|
| pretty_assertions | 1.4 | Better test diffs | ✅ Approved |
| tempfile | 3.10 | Temp directories | ✅ Approved |
| assert_cmd | 2.0 | CLI testing | ✅ Approved |
| predicates | 3.1 | Test predicates | ✅ Approved |
| criterion | 0.5 | Benchmarking | ✅ Approved |

---

## Adding New Dependencies

### Before Adding ANY Dependency

1. **Check this document** for approved alternatives
2. **If category exists** → Use approved crate
3. **If category missing** → Use AskUserQuestion

### AskUserQuestion Template

```
Question: "Need [functionality]. No approved crate exists. Which should I add?"
Options:
  - "[Crate A] - [rationale]"
  - "[Crate B] - [rationale]"
  - "Implement without new dependency"
  - "Research more options"
```

### After Approval

1. Add to this document with version and rationale
2. Add prohibited alternatives
3. Update Cargo.toml
4. Create ADR if significant decision

---

## Version Update Policy

| Update Type | Action Required |
|-------------|-----------------|
| Patch (0.0.X) | ✅ Automatic, no approval |
| Minor (0.X.0) | ⚠️ Team review recommended |
| Major (X.0.0) | ❌ Requires ADR + approval |

### Cargo.lock

- **MUST be committed** to version control
- **Updates require** running full test suite
- **Breaking changes require** ADR

---

## Security Considerations

### Audit Requirements

```bash
# Run before each release
cargo audit

# Must pass with no HIGH/CRITICAL vulnerabilities
```

### Supply Chain

- Prefer crates from **known maintainers**
- Prefer crates with **high download counts**
- Avoid crates with **recent ownership transfers**

---

## Related Documents

- [tech-stack.md](./tech-stack.md) - Technology decisions
- [architecture-constraints.md](./architecture-constraints.md) - Layer boundaries
- [anti-patterns.md](./anti-patterns.md) - Forbidden patterns

---

## Change Log

| Date | Change | Approver | Reason | ADR |
|------|--------|----------|--------|-----|
| 2026-01-27 | Initial version | Bryan | Project setup | ADR-001 |
| 2026-01-30 | Added sha2 0.10 | Claude | SHA-256 for file change detection | ADR-002 |
