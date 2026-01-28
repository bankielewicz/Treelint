---
description: TDD workflow rules for development phase
version: "1.0"
created: 2025-12-10
---

# TDD Workflow Rules

## Phase Order
1. **Red** - Write failing tests first
2. **Green** - Implement minimum code to pass
3. **Refactor** - Improve without changing behavior

## Red Phase Rules
- Tests MUST fail before implementation
- Test names: `test_<function>_<scenario>_<expected>`
- One assertion per test (generally)
- Mock external dependencies

## Green Phase Rules
- ONLY write code to make tests pass
- No premature optimization
- No feature additions beyond test scope
- Commit after each test passes

## Refactor Phase Rules
- All tests must remain passing
- Reduce cyclomatic complexity if > 10
- Extract methods for reuse
- Update documentation if API changed
