# AC Verification Workflow Reference

**Purpose:** Explain the complete AC verification process used in Phase 4.5 and Phase 5.5 of the development workflow

**Token Cost:** ~1,000 tokens (loaded on-demand when verification phase begins)

---

## Overview

AC (Acceptance Criteria) Verification is a quality gate that ensures all story acceptance criteria have been correctly implemented before proceeding to subsequent phases. It uses a **fresh-context technique** to provide independent, unbiased verification.

**Key Principle:** ONE failed AC = workflow HALT. There is no majority voting or partial pass.

---

## When Executed

AC Verification occurs at two checkpoints in the development workflow:

| Phase | Name | Trigger | Purpose |
|-------|------|---------|---------|
| **Phase 4.5** | Post-Refactor AC Verification | After Phase 04 (Refactoring) completes | Verify ACs before integration testing |
| **Phase 5.5** | Post-Integration AC Verification | After Phase 05 (Integration Testing) completes | Verify ACs after integration changes |

**Workflow Position:**
```
Phase 04 (Refactor) --> Phase 4.5 (AC Verify) --> Phase 05 (Integration)
                                                        |
                                                        v
                              Phase 06 (Deferral) <-- Phase 5.5 (AC Verify)
```

---

## Fresh-Context Technique

**CRITICAL:** Verification must use fresh context with NO prior coding knowledge.

### What Fresh-Context Means

- **No prior knowledge:** Verifier does NOT rely on information from earlier conversation
- **Independent discovery:** Must locate source files by reading story, not memory
- **Source inspection required:** Must Read() actual files to verify behavior
- **Unbiased evaluation:** Cannot "remember" how features were coded

### Why Fresh-Context Matters

| Benefit | Explanation |
|---------|-------------|
| Eliminates confirmation bias | Coding agent may overlook gaps in their own work |
| Independent verification | Separate entity validates implementation |
| Catches edge cases | Fresh perspective notices missed scenarios |
| Quality gate integrity | Cannot "rubber stamp" own implementation |

### Enforcement

The ac-compliance-verifier subagent has **read-only tools** (Read, Grep, Glob only). It cannot Write, Edit, or execute Bash commands, ensuring it reports issues rather than auto-fixing them.

---

## Subagent Invocation

### Synchronous Execution Model

AC verification uses **synchronous (blocking) execution**. The workflow waits for verification to complete before proceeding.

```
Task(
  subagent_type="ac-compliance-verifier",
  description="Verify AC compliance for ${STORY_ID}",
  prompt="""
  Verify acceptance criteria fulfillment for story.

  **Story file path:** devforgeai/specs/Stories/${STORY_ID}*.story.md

  **TECHNIQUE INSTRUCTIONS (MANDATORY):**
  1. **Fresh-context:** Do NOT rely on any prior coding knowledge - verify with no prior context from this conversation
  2. **One-by-one:** Verify each AC individually against source code
  3. **Source inspection:** Read actual implementation files to verify behavior

  **Verification Process:**
  - Read story file to extract acceptance criteria
  - For each AC, locate relevant source files
  - Verify implementation matches AC requirements
  - Document evidence (file paths, line numbers)

  Return structured verification report with:
  - Per-AC pass/fail status
  - Evidence supporting determination
  - List of files inspected
  - Any issues found with line numbers
  """
)
# Note: No run_in_background parameter = synchronous execution
# Workflow blocks here until verification completes
```

### Key Parameters

| Parameter | Value | Explanation |
|-----------|-------|-------------|
| `subagent_type` | `"ac-compliance-verifier"` | Invokes the read-only verification specialist |
| `run_in_background` | NOT SET (omitted) | Synchronous execution - workflow waits |

**Never use `run_in_background=true`** for AC verification. The workflow must wait for results before proceeding.

---

## HALT Behavior on Failure

### 100% Stop Rate Guarantee

**CRITICAL:** When any AC fails verification, the workflow HALTs immediately.

- ONE failed AC = workflow HALT
- No automatic bypass allowed
- No majority voting (4 of 5 pass = still HALT)
- User must fix and re-run `/dev STORY-XXX`

### Failure Display Format

```
IF verification.status == "FAIL":
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "  AC COMPLIANCE VERIFICATION FAILED - HALT TRIGGERED"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "WORKFLOW STATUS: HALTED - DO NOT PROCEED"
    Display: "100% stop rate guarantee - workflow always halts on failure"

    FOR each failed_ac:
        Display: "┌─────────────────────────────────────────────────────"
        Display: "│ FAILED AC ID: AC#{failed_ac.id}"
        Display: "│ SPECIFIC ISSUE: {failed_ac.reason}"
        Display: "│ EVIDENCE: file:{source_file} line:{line_number}"
        Display: "│ ACTIONABLE PATH TO FIX: {fix_guidance}"
        Display: "└─────────────────────────────────────────────────────"

    Display: "NEXT STEPS:"
    Display: "  1. Review the failed AC(s) above"
    Display: "  2. Fix the issues in the source files"
    Display: "  3. Re-run /dev STORY-XXX to trigger fresh verification"

    HALT workflow
```

### Recovery Path

1. Review failed AC details in the display output
2. Fix issues in source files identified by evidence
3. Re-run `/dev STORY-XXX` to restart workflow
4. Workflow resumes with fresh-context verification (no cached results)

---

## Verification Process Steps

### Step 1: Read Story File

```
Read(file_path="devforgeai/specs/Stories/STORY-XXX-*.story.md")
```

Extract:
- All acceptance criteria (XML format required)
- Given/When/Then conditions for each AC
- Source file hints (if provided in `<verification>` block)

### Step 2: Parse XML Acceptance Criteria

Stories must use XML-tagged ACs (legacy markdown not supported):

```xml
<acceptance_criteria id="AC1">
  <given>Initial context or state</given>
  <when>Action or event that occurs</when>
  <then>Expected outcome or result</then>
  <verification>
    <source_files>
      - path/to/file1.py
    </source_files>
  </verification>
</acceptance_criteria>
```

### Step 3: Locate Source Files

**With hints:** Read files from `<source_files>` block
**Without hints:** Use Glob and Grep to discover relevant files

### Step 4: Verify Each AC

For each AC:
1. Verify Given condition (precondition setup exists)
2. Verify When condition (trigger mechanism exists)
3. Verify Then condition (expected outcome produced)
4. Document evidence (file paths, line numbers, snippets)

### Step 5: Generate Structured Report

Return JSON report with pass/fail per AC and overall status.

---

## Report Format

```json
{
  "story_id": "STORY-XXX",
  "verification_timestamp": "2026-01-20T12:00:00Z",
  "verifier": "ac-compliance-verifier",
  "technique": "fresh-context",
  "results": {
    "total_acs": 5,
    "passed": 5,
    "failed": 0,
    "skipped": 0
  },
  "details": [
    {
      "ac_id": "AC1",
      "title": "AC Title",
      "status": "PASS",
      "evidence": {
        "file": "path/to/file.md",
        "lines": "10-25",
        "snippet": "relevant code snippet"
      }
    }
  ],
  "overall_status": "PASS"
}
```

---

## Error Handling

### Timeout (5 minutes)

```
IF verification exceeds 300 seconds:
    Display: "AC verification timed out after 5 minutes"
    AskUserQuestion: "Continue without verification or retry?"
```

### Subagent Failure

```
IF ac-compliance-verifier fails:
    Retry Task() invocation (once)
    IF retry fails:
        HALT workflow with error details
```

### Missing XML Format

If story lacks `<acceptance_criteria>` blocks:
```
HALT: "Story lacks required XML AC format. Update story to XML format per EPIC-046."
```

---

## Record Subagent Invocation

After verification completes, record the invocation:

```bash
# Phase 4.5
devforgeai-validate phase-record ${STORY_ID} --phase=4.5 --subagent=ac-compliance-verifier

# Phase 5.5
devforgeai-validate phase-record ${STORY_ID} --phase=5.5 --subagent=ac-compliance-verifier
```

---

## Related Documentation

| Document | Path |
|----------|------|
| Phase 4.5 Definition | `.claude/skills/devforgeai-development/phases/phase-04.5-ac-verification.md` |
| Phase 5.5 Definition | `.claude/skills/devforgeai-development/phases/phase-05.5-ac-verification.md` |
| AC Compliance Verifier Subagent | `.claude/agents/ac-compliance-verifier.md` |
| SKILL.md (AC Verification sections) | `.claude/skills/devforgeai-development/SKILL.md` (lines 520-543, 551-553) |

---

**File Version:** 1.0
**Created:** 2026-01-20
**Stories:** STORY-277 (HALT behavior), STORY-275 (Phase 4.5), STORY-276 (Phase 5.5)
