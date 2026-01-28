---
description: QA validation rules for story verification
version: "1.0"
created: 2025-12-10
---

# QA Validation Rules

## Validation Modes

### Light Mode (During Development)
- Run after each TDD phase
- Fast feedback on obvious issues
- Checks: syntax, imports, basic type safety

### Deep Mode (After Implementation)
- Comprehensive analysis
- Coverage verification
- Anti-pattern detection
- Security scanning

## Coverage Thresholds

| Layer | Minimum Coverage |
|-------|------------------|
| Business Logic | 95% |
| Application | 85% |
| Infrastructure | 80% |

## Blocking Violations

### Critical (Immediate Block)
- Security vulnerabilities
- Data exposure risks
- Authentication bypasses
- **Coverage below thresholds** (95%/85%/80% - NO DEFERRAL PATH)

### High (Block Before QA Approval)
- Unhandled error paths
- Missing input validation

## Non-Blocking (Warning Only)
- Style inconsistencies
- Documentation gaps
- Minor complexity issues

---

## Coverage Threshold Enforcement (ADR-010)

**Coverage gaps are CRITICAL blockers, NOT warnings.**

| Threshold | Layer | Enforcement |
|-----------|-------|-------------|
| 95% | Business Logic | CRITICAL - blocks QA |
| 85% | Application | CRITICAL - blocks QA |
| 80% | Infrastructure | CRITICAL - blocks QA |

**Rationale:** Coverage gaps cannot be deferred. If coverage is below threshold, QA result MUST be "FAILED", not "PASS WITH WARNINGS".

**Behavior:** test-automator returning WARN for coverage gaps → escalates to FAILED at Phase 3 result determination.
