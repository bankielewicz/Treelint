---
description: Input validation security rules
version: "1.0"
created: 2025-12-10
---

# Input Validation Rules

## Validation Requirements

### All User Input Must Be Validated
- Never trust user input
- Validate at system boundaries
- Sanitize before processing

### Validation Types
- **Type checking** - Ensure expected data type
- **Length limits** - Prevent buffer overflow
- **Format validation** - Regex for emails, phones, etc.
- **Range checking** - Numeric bounds
- **Allowlist validation** - Enum values

## SQL Injection Prevention
- Use parameterized queries ONLY
- Never concatenate user input into SQL
- Use ORM query builders

## XSS Prevention
- Escape HTML output
- Use Content Security Policy
- Sanitize rich text input

## Error Handling
- Never expose internal errors to users
- Log detailed errors server-side
- Return generic error messages to clients
- Include correlation IDs for debugging
