# Phase 06: Deferral Challenge Checkpoint

**Entry Gate:**
```bash
devforgeai-validate phase-check ${STORY_ID} --from=5.5 --to=06

Examples (--project-root applies to phase-* commands only, not check-hooks/invoke-hooks):
 - Correct: devforgeai-validate phase-init ${STORY_ID} --project-root=.
 - Incorrect: python -m devforgeai.cli.devforgeai_validate phase-init ${STORY_ID} --project-root=.
# Exit code 0: Transition allowed
# Exit code 1: Phase 5.5 not complete - HALT
# Exit code 2: Missing subagents from Phase 5.5 - HALT
```

---

## Phase Workflow

**Purpose:** Challenge ALL deferrals - prevent autonomous deferral approval

**Required Subagents:**
- deferral-validator (Deferral validation) [CONDITIONAL]

**Steps:**

1. **Review DoD for incomplete items**
   ```
   Grep(pattern="- \\[ \\]", path="${STORY_FILE}")
   # List all unchecked DoD items
   ```

2. **Detect deferrals**
   - Check for DEFERRED markers
   - Check for incomplete items without justification

3. **IF deferrals exist: Invoke deferral-validator**
   ```
   Task(
     subagent_type="deferral-validator",
     description="Validate deferrals for ${STORY_ID}",
     prompt="""
     Validate that all deferrals are properly justified.

     Story: ${STORY_FILE}

     For each deferral, check:
     1. Technical justification exists
     2. Follow-up story referenced
     3. Not a circular deferral
     4. Challenge with user
            AskUserQuestion(
            questions=[{
               question: "How should we handle this incomplete item?",
               header: "Deferral",
               options: [
                  {label: "HALT and implement NOW (Recommended)", description: "Stop and implement this item"},
                  {label: "Defer with follow-up story", description: "Create follow-up story and continue"},
                  {label: "Mark as out of scope", description: "Document as intentionally excluded"}
               ],
               multiSelect: false
            }]
            )
     """
   )
   ```

4. **Attempt implementation for each deferral**
   - "Attempt First, Defer Only If Blocked" pattern
   - Try to implement before accepting deferral

5. **If still blocked: Challenge with user**
   ```
   AskUserQuestion(
     questions=[{
       question: "How should we handle this incomplete item?",
       header: "Deferral",
       options: [
         {label: "HALT and implement NOW (Recommended)", description: "Stop and implement this item"},
         {label: "Defer with follow-up story", description: "Create follow-up story and continue"},
         {label: "Mark as out of scope", description: "Document as intentionally excluded"}
       ],
       multiSelect: false
     }]
   )
   ```

5.1. **IF user selects "HALT and implement NOW": Increment iteration counter** (RCA-013 REC-4)
   ```
   # User chose to loop back and implement - increment iteration counter
   iteration_count += 1
   last_iteration_date = CURRENT_DATE

   # Update phase-state.json with new iteration count
   Write iteration_count and last_iteration_date to phase-state.json

   # Display warning if approaching limit
   IF iteration_count >= 4:
       Display: "⚠️ TDD Iteration: {iteration_count}/5 - Approaching limit"

   # Block if max iterations reached
   IF iteration_count >= 5:
       HALT: "Maximum TDD iterations (5) reached. Story may need decomposition."

   # Loop back to Phase 02 (Test-First) for next iteration
   GOTO Phase 02
   ```

6. **AskUserQuestion for EVERY deferral** [ENFORCED]
   - First option MUST be "HALT and implement NOW"
   - No autonomous deferral approval

c.1. **Record user approval timestamp** [MANDATORY]
   ```
   Edit(
     file_path="${STORY_FILE}",
     old_string="DEFERRED: {item}",
     new_string="DEFERRED: {item}\n   User approved: ${CURRENT_DATE}"
   )
   ```

c.2. **Technical Debt Register Update** [MANDATORY - UNCONDITIONAL] (STORY-286)
   ```
   # UNCONDITIONAL: When user selects "Keep deferred" OR "Update justification",
   # MUST trigger register update workflow (no opt-out, no conditions)
   # BR-001: Register update is UNCONDITIONAL
   # BR-003: User approval timestamp MUST exist before register write

   # TRIGGER CONDITIONS:
   # - "Keep deferred" selection → triggers technical debt register update
   # - "Update justification" selection → triggers technical debt register update
   # Both cases: register updated UNCONDITIONALLY (no opt-out)

   IF user_decision IN ["Keep deferred (blocker is valid)", "Update justification (blocker changed)"]:
       # Verify user approval timestamp exists (BR-003 - prevents autonomous writes)
       IF NOT deferral.user_approval_timestamp:
           HALT: "AUTONOMOUS WRITE ATTEMPTED - User approval timestamp missing"

       # Execute register update workflow (detailed in references/phase-06-deferral-challenge.md Step 6.6)
       Read(file_path=".claude/skills/devforgeai-development/references/phase-06-deferral-challenge.md")
       # Execute Step 6.6: Technical Debt Register Update Workflow

       # Display confirmation (AC#5)
       Display: ""
       Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
       Display: "📋 TECHNICAL DEBT REGISTERED"
       Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
       Display: "✓ ID: {DEBT_ID}"
       Display: "✓ Item: {deferral.text}"
       Display: "✓ Register: devforgeai/technical-debt-register.md"
       Display: "✓ Technical debt: {total_open} open items"
       Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
       Display: ""
   ```

7. **Update AC Checklist (deferral items)**

**Reference:** `references/phase-06-deferral-challenge.md` for complete workflow
    Read(file_path=".claude/skills/devforgeai-development/references/phase-06-deferral-challenge.md")

---

## Validation Checkpoint

**Before proceeding to Phase 07, verify:**

- [ ] DoD reviewed for incomplete items
- [ ] IF deferrals exist: deferral-validator invoked
- [ ] AskUserQuestion invoked for EVERY deferral
- [ ] User approval timestamp recorded for each kept deferral
- [ ] IF deferral approved: Technical debt register updated (STORY-286)
- [ ] IF deferral approved: Confirmation displayed with DEBT-NNN ID (STORY-286)
- [ ] AC Checklist (deferral items) updated if applicable ([ ] → [x])

### AC Checklist Update Verification (RCA-003)

After Step 7 completes (if deferrals were processed), verify AC Checklist was updated:
```
# Only required if deferrals were processed in this phase
IF deferrals_exist:
    Grep(pattern="- \\[x\\].*[Dd]eferr", path="${STORY_FILE}")
    # Should find checked deferral-related items
    # If no matches found AND deferrals were processed: AC Checklist update was skipped - HALT
```

**IF AskUserQuestion SKIPPED:**
- AUTONOMOUS DEFERRAL DETECTED - HALT
- Claude MUST use AskUserQuestion for EVERY deferral
- First option MUST be 'HALT and implement NOW'

**IF timestamp MISSING:**
- DEFERRAL WITHOUT USER APPROVAL - HALT
- Every kept deferral MUST have 'User approved: timestamp'

---

## Observation Capture

**Before exiting this phase, reflect:**
1. Did I encounter any friction? (unclear docs, missing tools, workarounds)
2. Did anything work particularly well? (constraints that helped, patterns that fit)
3. Did I notice any repeated patterns?
4. Are there gaps in tooling/docs?
5. Did I discover any bugs?

**If YES to any:** Append to phase-state.json `observations` array:
```json
{
  "id": "obs-06-{seq}",
  "phase": "06",
  "category": "{friction|success|pattern|gap|idea|bug}",
  "note": "{1-2 sentence description}",
  "files": ["{relevant files}"],
  "severity": "{low|medium|high}"
}
```

**Reference:** `references/observation-capture.md`
    Read(file_path=".claude/skills/devforgeai-development/references/observation-capture.md")

---

**Exit Gate:**
```bash
devforgeai-validate phase-complete ${STORY_ID} --phase=06 --checkpoint-passed
# Exit code 0: Phase complete, proceed to Phase 07
# Exit code 1: Cannot complete - deferrals not properly approved
```
