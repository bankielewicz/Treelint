---
description: Quality gate definitions for story progression
version: "1.0"
created: 2025-12-10
---

# Quality Gates

## Gate 1: Context Validation
**Transition:** Architecture → Ready for Dev
**Enforced By:** devforgeai-orchestration skill
**Requirements:**
- All 6 context files present
- Valid YAML/Markdown syntax
- No conflicting dependencies

## Gate 2: Test Passing
**Transition:** Dev Complete → QA In Progress
**Enforced By:** devforgeai-qa skill
**Requirements:**
- All tests pass (exit code 0)
- Coverage meets thresholds (95%/85%/80%)
- No Critical/High anti-pattern violations

## Gate 3: QA Approval
**Transition:** QA Approved → Releasing
**Enforced By:** devforgeai-release skill
**Requirements:**
- Story file has "QA APPROVED" marker
- All acceptance criteria verified
- No deferred items without justification
- runtime smoke test passes (CLI/API must execute) <!-- RCA-002 -->

## Gate 4: Release Readiness
**Transition:** Releasing → Released
**Enforced By:** devforgeai-release skill
**Requirements:**
- Smoke tests pass
- Deployment verification complete
- Rollback plan documented
