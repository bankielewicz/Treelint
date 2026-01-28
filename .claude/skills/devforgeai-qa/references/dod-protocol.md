# Definition of Done Protocol

**CRITICAL requirements for DoD validation that CANNOT be skipped.**

---

## Constitutional Rule

**ALL validation steps are MANDATORY and CANNOT be skipped.**

If you are tempted to skip a validation step:
1. **STOP immediately**
2. **Use AskUserQuestion** to request permission from user
3. **Document deviation** in QA report with justification
4. **Mark QA status** as "PASSED WITH EXCEPTIONS"

---

## PROHIBITED Shortcuts

### ❌ "Manual validation is equivalent to subagent validation"

**Reality:** Manual validation misses multi-level deferral chains, circular deferrals, ADR requirement checks

**Evidence:** RCA-007 - Manual validation missed STORY-004 → STORY-005 → STORY-006 chain

### ❌ "Story already QA Approved, re-validation can be lighter"

**Reality:** Re-validation must be thorough to catch issues missed in previous attempts

**Evidence:** RCA-007 - "Already approved" status created false sense of security

### ❌ "Token optimization justifies skipping steps"

**Reality:** Quality gates cannot be bypassed for efficiency - integrity is paramount

**Evidence:** Token savings irrelevant if technical debt enters production

### ❌ "Deferral reason exists, therefore deferral is valid"

**Reality:** Deferral reason must be VALIDATED (blocker verification, story reference check, ADR requirement, chain detection)

**Evidence:** RCA-007 - Deferral reason existed but was invalid (chain to STORY-006 which didn't include work)

---

## Required Actions

### ✅ Invoke deferral-validator Subagent

**When deferrals exist:**
- Step 2.5 is MANDATORY
- Subagent detects issues manual validation misses (multi-level chains, missing ADRs)
- NO manual override allowed

**Why automated validation:**
- Detects circular chains (A→B→A)
- Detects multi-level chains (A→B→C)
- Verifies story references exist and include deferred work
- Validates ADR requirements for scope changes
- Checks technical blocker feasibility

### ✅ Fail QA on Deferral Violations

**If deferral-validator returns CRITICAL/HIGH violations:**
- Do not proceed to approval
- Report violations to user with remediation
- QA status = FAILED
- Developer must fix deferrals and re-run `/dev` then `/qa`

### ✅ Document All Validation Steps

**In QA report:**
- Include whether deferral-validator was invoked
- Include validation results with evidence
- Document all violations by severity
- Provide remediation guidance

### ✅ Use AskUserQuestion for Protocol Deviations

**If uncertain about protocol:**
- Example: "Should I skip deferral validation for this re-validation?"
- Do not make autonomous protocol deviation decisions
- User must explicitly approve any deviation

---

## Enforcement Mechanisms

### Verbal Reminder
- This protocol appears in skill prompt
- Makes deviation explicit, not implicit
- Active decision required to deviate (not passive shortcut)

### Paper Trail
- Deviation visible in conversation transcript
- QA Validation History tracks whether deferral-validator invoked
- Audit mechanisms via `/audit-deferrals` command catch protocol deviations

### Quality Gate
- CRITICAL/HIGH deferral violations BLOCK QA approval
- Story status cannot progress to "QA Approved" with violations
- Developer must resolve before release

---

## Rationale (Why This Rule Exists)

**Root Cause:** RCA-007 incident

**What Happened:**
- QA skill deviated from protocol
- Performed manual validation instead of invoking deferral-validator subagent
- Missed multi-level deferral chain (STORY-004 → STORY-005 → STORY-006)
- Technical debt went untracked
- Exit code handling work disappeared in broken chain
- No ADR documentation created

**Impact:**
- Quality gate bypassed
- Work lost
- Technical debt accumulated
- Framework integrity violated

**Prevention:**
- Make protocol adherence mandatory, not optional
- Automated validation prevents human error
- Subagent catches issues manual validation misses

---

## Step 2.5 Implementation

**Location in workflow:** Phase 3 (Spec Compliance Validation), Step 2.5

**When invoked:** After validating acceptance criteria, before validating API contracts

**Cannot be skipped because:**
- Deferral validation is spec compliance validation
- Deferred items affect story completeness
- Invalid deferrals = incomplete implementation
- Multi-level chains create technical debt
- Circular chains prevent work completion

**See:** `spec-compliance-workflow.md` for complete Step 2.5 implementation

---

## Protocol Compliance Checklist

Before marking QA as PASSED, verify:
- [ ] All validation phases executed (no skips)
- [ ] deferral-validator invoked if deferrals exist
- [ ] No autonomous validation shortcuts taken
- [ ] All violations documented in QA report
- [ ] QA Validation History updated with deferral validation status
- [ ] If protocol deviated: User approval obtained + documented

**This protocol is mandatory in both light and deep validation modes.**
