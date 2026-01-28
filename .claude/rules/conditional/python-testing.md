---
paths: "**/*.py, tests/**/*"
description: Python testing conventions
version: "1.0"
created: 2025-12-10
---

# Python Testing Rules

These rules apply to all Python files and test directories.

## Framework
- Use pytest (not unittest)
- Use pytest fixtures for setup/teardown
- Use pytest.mark for test categorization

## Naming Conventions
- Test files: `test_<module>.py`
- Test functions: `test_<function>_<scenario>_<expected>`
- Fixture functions: `<resource>_fixture`

## Coverage Requirements
- Business logic: 95%
- Application layer: 85%
- Infrastructure: 80%

## Mocking
- Use `pytest-mock` or `unittest.mock`
- Mock at boundaries (external APIs, databases)
- Never mock the system under test
