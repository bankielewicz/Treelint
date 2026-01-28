# Contributing to Treelint

Thank you for your interest in contributing to Treelint! This document provides guidelines for contributing.

---

## Development Setup

### Prerequisites

- **Rust 1.70.0+** (MSRV)
- **Git**
- **Cargo** (included with Rust)

### Getting Started

```bash
# Clone the repository
git clone https://github.com/treelint/treelint
cd treelint

# Build debug version
cargo build

# Run tests
cargo test

# Run linter
cargo clippy -- -D warnings

# Check formatting
cargo fmt --check
```

---

## Code Quality Standards

### Required Checks

All contributions must pass:

1. **Tests:** `cargo test` (100% pass rate)
2. **Linting:** `cargo clippy -- -D warnings` (0 warnings)
3. **Formatting:** `cargo fmt --check` (no changes)

### Coverage Targets

| Layer | Target |
|-------|--------|
| Business Logic | 95% |
| Application | 85% |
| Infrastructure | 80% |

---

## Architecture Rules

### Module Structure

```
src/
├── main.rs          # Entry point
├── lib.rs           # Library root
├── cli/             # CLI argument handling
├── parser/          # tree-sitter parsing (future)
├── index/           # SQLite indexing (future)
├── output/          # JSON/text formatting
└── error.rs         # Error types
```

### Layer Boundaries

- **CLI layer** calls Application layer (never Database directly)
- **Application layer** orchestrates Domain + Infrastructure
- **Domain layer** contains business logic (no IO)
- **Infrastructure layer** handles external systems

### Prohibited Patterns

- ❌ `.unwrap()` in production code (use `?` operator)
- ❌ `.expect()` in production code
- ❌ String error types (use `thiserror`)
- ❌ Circular dependencies between modules
- ❌ Files > 500 lines (split into smaller modules)

---

## Pull Request Process

### 1. Create a Branch

```bash
git checkout -b feature/STORY-XXX-description
```

### 2. Make Changes

- Follow the architecture rules above
- Add tests for new functionality
- Update documentation if needed

### 3. Verify Quality

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

### 4. Submit PR

- Use clear, descriptive title
- Reference related issues/stories
- Describe what changed and why

---

## Commit Messages

Follow conventional commits:

```
feat: Add symbol type filtering
fix: Handle empty search queries
docs: Update CLI reference
test: Add search command tests
refactor: Extract JSON output module
```

---

## Dependencies

**Approved dependencies** are listed in `devforgeai/specs/context/dependencies.md`.

To add a new dependency:
1. Check if similar functionality exists in approved deps
2. Create an ADR explaining the need
3. Get approval before adding

---

## Questions?

- Open an issue for bugs or feature requests
- Check existing issues before creating new ones

---

**Version:** 0.1.0
**Generated:** 2026-01-27
