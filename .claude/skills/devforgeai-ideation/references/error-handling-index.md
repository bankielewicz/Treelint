# Error Handling Index

Quick reference for identifying and resolving ideation workflow errors.

---

## Decision Tree: Which Error Type Am I Experiencing?

```
START: What type of error are you experiencing?
│
├─► Vague/incomplete user answers?
│   └─► See: error-type-1-incomplete-answers.md
│       (Phase 2: Requirements Elicitation)
│
├─► Directory structure problems?
│   └─► See: error-type-6-directory-issues.md
│       (Phase 6.1: Artifact Generation)
│
├─► File creation/write failures?
│   └─► See: error-type-2-artifact-failures.md
│       (Phase 6.1: Artifact Generation)
│
├─► Invalid complexity scores?
│   └─► See: error-type-3-complexity-errors.md
│       (Phase 3: Complexity Assessment)
│
├─► Quality validation failures?
│   └─► See: error-type-4-validation-failures.md
│       (Phase 6.4: Self-Validation)
│
└─► Conflicts with existing constraints?
    └─► See: error-type-5-constraint-conflicts.md
        (Phase 5: Feasibility Analysis, brownfield projects)
```

---

## Quick Reference Table

| Phase | Error Type | Symptom | File | Severity |
|-------|------------|---------|------|----------|
| Phase 2 | Incomplete Answers | User says "I don't know" | [error-type-1](error-type-1-incomplete-answers.md) | Medium |
| Phase 2 | Incomplete Answers | Vague terms without quantification | [error-type-1](error-type-1-incomplete-answers.md) | Medium |
| Phase 3 | Complexity Errors | Score >60 or <0 | [error-type-3](error-type-3-complexity-errors.md) | Critical |
| Phase 3 | Complexity Errors | Tier doesn't match score | [error-type-3](error-type-3-complexity-errors.md) | High |
| Phase 5 | Constraint Conflicts | Tech not in tech-stack.md | [error-type-5](error-type-5-constraint-conflicts.md) | High |
| Phase 5 | Constraint Conflicts | Anti-pattern violation | [error-type-5](error-type-5-constraint-conflicts.md) | Critical |
| Phase 6.1 | Directory Issues | Directory not found | [error-type-6](error-type-6-directory-issues.md) | High |
| Phase 6.1 | Artifact Failures | Write() fails with permission error | [error-type-2](error-type-2-artifact-failures.md) | High |
| Phase 6.4 | Validation Failures | Missing YAML frontmatter fields | [error-type-4](error-type-4-validation-failures.md) | Critical |
| Phase 6.4 | Validation Failures | Too many TODO/TBD placeholders | [error-type-4](error-type-4-validation-failures.md) | High |

---

## Error Type Files

1. **[error-type-1-incomplete-answers.md](error-type-1-incomplete-answers.md)** — Phase 2: Requirements Elicitation
   - User gives vague responses or says "I don't know"
   - Missing quantification of non-functional requirements
   - Recovery: Follow-ups, examples, assumptions with validation flags

2. **[error-type-2-artifact-failures.md](error-type-2-artifact-failures.md)** — Phase 6.1: Artifact Generation
   - Epic or requirements specification files cannot be created
   - Write errors, permission problems, filesystem issues
   - Recovery: Create directories, retry, provide manual instructions

3. **[error-type-3-complexity-errors.md](error-type-3-complexity-errors.md)** — Phase 3: Complexity Assessment
   - Complexity scores out of valid range (0-60)
   - Missing dimensions or dimension overflow
   - Recovery: Recalculate using assessment matrix, update tier

4. **[error-type-4-validation-failures.md](error-type-4-validation-failures.md)** — Phase 6.4: Self-Validation
   - Generated artifacts fail quality checks
   - Missing YAML fields, too many placeholders, vague requirements
   - Recovery: Auto-correct MEDIUM issues, regenerate HIGH issues, report CRITICAL

5. **[error-type-5-constraint-conflicts.md](error-type-5-constraint-conflicts.md)** — Phase 5: Feasibility Analysis (Brownfield)
   - New requirements conflict with existing constraints
   - Technology stack mismatches, architecture violations
   - Recovery: User decision (update constraints, modify requirement, defer)

6. **[error-type-6-directory-issues.md](error-type-6-directory-issues.md)** — Phase 6.1: Artifact Generation
   - Required directories don't exist before file creation
   - Permission or filesystem structure issues
   - Recovery: Create with Write/.gitkeep pattern, verify permissions

---

## Recovery Strategy Overview

**Level 1: Auto-Correct** (No user interaction)
- Missing dates → Use today
- Missing status → Use default "Planning"
- ID mismatches → Extract from filename

**Level 2: Regenerate** (Use existing data)
- Missing sections → Regenerate from phase data
- Invalid calculations → Recalculate using matrix

**Level 3: User Resolution** (AskUserQuestion)
- Missing business value
- Conflicting requirements
- Constraint violations

**Level 4: Manual Intervention** (HALT)
- Persistent file system errors
- Critical validation failures after 2 attempts
- Irresolvable conflicts

---

## Max Retry Policy

| Error Type | Max Retries | Escalation |
|------------|-------------|------------|
| Incomplete Answers | 3 | Document as assumption |
| Artifact Generation | 2 | Manual instructions |
| Complexity Errors | 1 | HALT for manual review |
| Validation Failures | 2 | Report to user |
| Constraint Conflicts | N/A | User decision required |
| Directory Issues | 2 | Manual mkdir instructions |

---

## Token Budget by Error Type

| Error Type | Typical Budget |
|------------|----------------|
| Incomplete Answers | 500-1,000 tokens |
| Artifact Failures | 500-1,500 tokens |
| Complexity Errors | 1,000-2,000 tokens |
| Validation Failures | 2,000-4,000 tokens |
| Constraint Conflicts | 1,000-2,000 tokens |
| Directory Issues | 500-1,000 tokens |

---

**Load specific error type file for detailed recovery procedures.**
