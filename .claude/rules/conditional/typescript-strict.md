---
paths: "**/*.ts, **/*.tsx"
description: TypeScript strict mode rules
version: "1.0"
created: 2025-12-10
---

# TypeScript Rules

These rules apply to all TypeScript files.

## Strict Mode
- `strict: true` in tsconfig.json
- No `any` types (use `unknown` if truly unknown)
- Explicit return types on functions

## Type Safety
- Use type guards for narrowing
- Prefer interfaces for object shapes
- Use generics for reusable patterns

## Forbidden Patterns
- `any` type without justification
- Type assertions without guards
- Non-null assertions (`!`)
