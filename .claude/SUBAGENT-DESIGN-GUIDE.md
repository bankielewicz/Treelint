---
title: Subagent Design Guide
status: LOCKED
version: "1.0"
created: 2026-01-12
last_updated: 2026-01-12
source: RCA-017
---

# Subagent Design Guide

This guide provides definitive guidance for creating DevForgeAI subagents with proper context file validation to prevent constraint violations.

**Motivation:** RCA-017 identified that test-automator generated files in wrong locations because it didn't validate against source-tree.md before Write() operations. This pattern can occur with ANY file-generation subagent.

---

## Context File Validation Checklist

### For ALL Subagents

Every subagent MUST read these context files before operations:

| Context File | Purpose | When to Read |
|--------------|---------|--------------|
| `tech-stack.md` | Technology constraints | Before suggesting any technology |

### For File-Generation Subagents

Subagents with Write/Edit tool access MUST read:

| Context File | Purpose | CRITICAL |
|--------------|---------|----------|
| `source-tree.md` | Validate output file paths | **YES - ALWAYS** |
| `dependencies.md` | Verify no forbidden dependencies | Yes |
| `tech-stack.md` | Validate technology choices | Yes |

**File-Generation subagents include:**
- test-automator (writes test files)
- api-designer (writes API specs)
- documentation-writer (writes docs)
- story-requirements-analyst (writes story sections)
- refactoring-specialist (uses Edit tool)

### For Code-Generation Subagents

Subagents that generate implementation code MUST read:

| Context File | Purpose |
|--------------|---------|
| `coding-standards.md` | Follow code patterns |
| `architecture-constraints.md` | Respect layer boundaries |
| `anti-patterns.md` | Avoid forbidden patterns |
| `source-tree.md` | Correct file placement |

**Code-Generation subagents include:**
- backend-architect
- frontend-developer
- refactoring-specialist

### For Specification/Documentation Subagents

Subagents generating specs or documentation MUST read:

| Context File | Purpose |
|--------------|---------|
| `tech-stack.md` | Aligned with locked technologies |
| `dependencies.md` | No external packages (if framework component) |
| `source-tree.md` | Correct file locations |

**Specification/Documentation subagents include:**
- requirements-analyst
- api-designer
- documentation-writer

---

## Critical Rule: Before ANY Write() Call

**ALWAYS read source-tree.md before Write()**

This is the single most important rule for file-generation subagents. The RCA-017 violation occurred because test-automator wrote files without validating the target directory.

### The Rule

```
BEFORE calling Write(file_path="..."):
  1. Read source-tree.md
  2. Extract correct directory pattern for file type
  3. Validate planned file_path matches constraints
  4. HALT if validation fails
```

### HALT Pattern for Violations

If a planned file path violates source-tree.md, the subagent MUST halt:

```
❌ SOURCE-TREE CONSTRAINT VIOLATION

Planned path: tests/installer/test_wizard.sh
Expected per source-tree.md: installer/tests/test_wizard.sh

Rule violated: "Module-specific tests go in {module}/tests/"
Source: devforgeai/specs/context/source-tree.md, lines 395-397

HALT: Cannot proceed with Write() operation.
Action: User must approve path correction or update source-tree.md.
```

---

## Pre-Generation Validation Template

Copy this template into any file-generation subagent's workflow:

```markdown
**Pre-Generation Validation:**

Read(file_path="devforgeai/specs/context/source-tree.md")

Determine correct directory for output files:
- Extract directory patterns from source-tree.md
- Validate all planned file_path parameters match constraints

HALT if validation fails:
"""
❌ SOURCE-TREE CONSTRAINT VIOLATION

Planned path: {planned_path}
Expected per source-tree.md: {expected_path}

Rule violated: "{rule_description}"
Source: devforgeai/specs/context/source-tree.md, lines {line_numbers}

HALT: Cannot proceed with Write() operation.
Action: User must approve path correction or update source-tree.md.
"""

IF validation passes:
  Proceed with file generation
```

---

## Examples

### Wrong: Write Without Validation (RCA-017 Pattern)

This pattern caused the original violation in test-automator:

```markdown
## Phase 2: Generate Tests

# Directly write tests without checking source-tree.md
Write(
  file_path="tests/installer/test_wizard.sh",  # WRONG PATH
  content="..."
)
```

**Why this is wrong:**
- No Read() of source-tree.md before Write()
- No validation of output path against constraints
- No HALT mechanism for violations
- Result: tests/installer/ instead of installer/tests/ (RCA-017)

### Correct: Read → Validate → Write

This is the correct pattern all file-generation subagents must follow:

```markdown
## Phase 2: Generate Tests

# Step 1: Read source-tree.md
Read(file_path="devforgeai/specs/context/source-tree.md")

# Step 2: Extract correct test directory
# From source-tree.md lines 395-397:
# "Module-specific tests: {module}/tests/"
# For installer module → installer/tests/

# Step 3: Validate planned path
planned_path = "installer/tests/test_wizard.sh"
# Matches source-tree.md pattern? YES

# Step 4: Write with validated path
Write(
  file_path="installer/tests/test_wizard.sh",  # CORRECT PATH
  content="..."
)
```

**Why this is correct:**
- Reads source-tree.md before any Write()
- Extracts correct directory pattern
- Validates planned path matches constraints
- Would HALT if validation failed

---

## Quick Reference Card

```
┌─────────────────────────────────────────────────────────────┐
│ SUBAGENT FILE GENERATION CHECKLIST                         │
├─────────────────────────────────────────────────────────────┤
│ □ Read source-tree.md BEFORE any Write()                   │
│ □ Extract correct directory pattern for file type          │
│ □ Validate planned file_path matches constraints           │
│ □ HALT with violation message if path incorrect            │
│ □ Only proceed with Write() after validation passes        │
└─────────────────────────────────────────────────────────────┘
```

---

## References

- **RCA-017:** test-automator Source Tree Constraint Violation
- **STORY-203:** Add source-tree.md Validation to test-automator Phase 2
- **STORY-204:** Update ALL File-Generation Subagents with source-tree.md Validation
- **source-tree.md:** devforgeai/specs/context/source-tree.md
- **architecture-constraints.md:** devforgeai/specs/context/architecture-constraints.md, lines 46-63 (Subagent Design Constraints)

---

**Document Version:** 1.0
**Created:** 2026-01-12
**Status:** LOCKED - Changes require ADR approval
