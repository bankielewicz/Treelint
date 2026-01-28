# Phase 04.5: AC Compliance Verification (Post-Refactor)

**Entry Gate:**
```bash
devforgeai-validate phase-check ${STORY_ID} --from=04 --to=4.5

Examples (--project-root applies to phase-* commands only, not check-hooks/invoke-hooks):
 - Correct: devforgeai-validate phase-init ${STORY_ID} --project-root=.
 - Incorrect: python -m devforgeai.cli.devforgeai_validate phase-init ${STORY_ID} --project-root=.
# Exit code 0: Transition allowed
# Exit code 1: Phase 04 not complete - HALT
# Exit code 2: Missing subagents from Phase 04 - HALT
```

---

## Phase Workflow

**Purpose:** Verify acceptance criteria fulfillment using fresh-context technique before integration testing

**Required Subagents:**
- ac-compliance-verifier (AC verification with fresh context)

**Execution Model:** SYNCHRONOUS (blocking) - workflow waits for verification to complete

**Steps:**

1. **Invoke AC Compliance Verifier**
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

2. **Record Subagent Invocation**
   ```bash
   devforgeai-validate phase-record ${STORY_ID} --phase=4.5 --subagent=ac-compliance-verifier
   ```

3. **Process Verification Results**
   ```
   IF verification.status == "PASS":
       Display: "✓ AC Compliance Verification passed"
       Display: "  All {count} acceptance criteria verified"
       Proceed to Phase 05

   IF verification.status == "FAIL":
       # CRITICAL: ANY single AC failure triggers immediate HALT (STORY-277)
       # Not majority voting: ONE failed AC = workflow must stop
       # IF any result = FAIL, the entire workflow HALTs immediately

       Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
       Display: "❌ AC COMPLIANCE VERIFICATION FAILED - HALT TRIGGERED"
       Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
       Display: ""
       Display: "WORKFLOW STATUS: HALTED - DO NOT PROCEED"
       Display: "100% stop rate guarantee - workflow always halts on failure"
       Display: ""

       # Display detailed failure report for EACH failed AC
       FOR each failed_ac in verification.failed_acs:
           Display: "┌─────────────────────────────────────────────────────"
           Display: "│ FAILED AC ID: AC#{failed_ac.id}"
           Display: "│"
           Display: "│ SPECIFIC ISSUE: {failed_ac.reason}"
           Display: "│   Why it failed: {failed_ac.detailed_explanation}"
           Display: "│"
           Display: "│ EVIDENCE: file:{failed_ac.source_file} line:{failed_ac.line_number}"
           Display: "│   Source location where issue was detected"
           Display: "│"
           Display: "│ ACTIONABLE PATH TO FIX:"
           Display: "│   How to fix this issue: {failed_ac.fix_guidance}"
           Display: "│   Direct path: Edit {failed_ac.source_file}"
           Display: "└─────────────────────────────────────────────────────"
           Display: ""

       Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
       Display: "ENFORCEMENT: No automatic bypass allowed - manual review required"
       Display: "This workflow must stop here. The workflow will never continue"
       Display: "past failed AC verification automatically."
       Display: ""
       Display: "NEXT STEPS:"
       Display: "  1. Review the failed AC(s) above"
       Display: "  2. Fix the issues in the source files"
       Display: "  3. Re-run /dev STORY-XXX to trigger fresh verification"
       Display: ""
       Display: "NOTE: Re-run uses fresh context evaluation with no prior context."
       Display: "      Each re-run starts with clean context - no cached verification."
       Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

       # HALT - Workflow stops here, no automatic progression
       HALT workflow
   ```

4. **Update Story File (if verification passes)**
   ```
   # Update AC Verification Checklist items verified in this phase
   Edit(
     file_path="${STORY_FILE}",
     old_string="- [ ] {verified_item}",
     new_string="- [x] {verified_item}"
   )
   ```

---

## Validation Checkpoint

**Before proceeding to Phase 05, verify:**

- [ ] ac-compliance-verifier subagent invoked
- [ ] Verification completed (not timed out)
- [ ] All ACs passed OR user override documented
- [ ] Subagent invocation recorded

**IF any checkbox UNCHECKED:** HALT workflow

---

## Observation Capture

**Before exiting this phase, reflect:**
1. Did verification catch issues missed during implementation?
2. Did fresh-context technique reveal gaps?
3. Were any ACs ambiguous or untestable?
4. Did verification complete within acceptable time (60-120s)?

**If observations exist:** Append to phase-state.json `observations` array:
```json
{
  "id": "obs-4.5-{seq}",
  "phase": "4.5",
  "category": "{friction|success|pattern|gap|idea|bug}",
  "note": "{1-2 sentence description}",
  "files": ["{relevant files}"],
  "severity": "{low|medium|high}"
}
```

---

## Error Handling

**Timeout (5 minutes):**
```
IF verification exceeds 300 seconds:
    Display: "⚠️ AC verification timed out after 5 minutes"
    Display: "Story may have too many ACs or complex verification"
    AskUserQuestion: "Continue without verification or retry?"
```

**Subagent Failure:**
```
IF ac-compliance-verifier fails:
    Display: "❌ AC compliance verifier encountered an error"
    # Retry once
    Retry Task() invocation
    IF retry fails:
        HALT workflow with error details
```

---

**Exit Gate:**
```bash
devforgeai-validate phase-complete ${STORY_ID} --phase=4.5 --checkpoint-passed
# Exit code 0: Phase complete, proceed to Phase 05
# Exit code 1: Cannot complete - verification failed or incomplete
```

