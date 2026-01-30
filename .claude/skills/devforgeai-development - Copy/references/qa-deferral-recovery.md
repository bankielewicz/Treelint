# QA Deferral Failure Recovery

**Purpose:** Resolution workflow when QA fails due to deferred Definition of Done items.

**Trigger:** Development skill invoked after QA failure with deferral violations

**Execution:** During Phase 01 Step h. detection, activates specialized recovery workflow

**Token Cost:** ~3,000 tokens (includes AskUserQuestion interactions)

---

## Background

When QA validation fails due to improperly justified deferrals, the development skill must guide resolution before allowing story progression.

**RCA-006 Compliance:** This workflow prevents autonomous deferrals and ensures user approval for all incomplete DoD items.

---

## Handling QA Deferral Failures

**When invoked after QA failure due to deferrals:**

### Step 1: Detect QA Failure Context

**Check for QA report:**

```
Glob(pattern="devforgeai/qa/reports/{story-id}-qa-report*.md")

IF multiple reports found (multiple QA attempts):
    Read most recent report

IF report status is "FAILED":
    Parse failure reasons

    IF failure includes "Deferral Validation FAILED":
        # This is a deferral-specific failure
        Extract deferral violations from report

        Display to user:
        "Previous QA attempt failed due to deferral issues:

         Unjustified Deferrals:
         1. '{item}': {violation_type}
            Current reason: '{reason}'
            Required: {required_action}

         2. '{item}': {violation_type}
            Current reason: '{reason}'
            Required: {required_action}

         Proceeding to resolve each deferral issue..."
```

### Step 2: Resolve Each Deferral Issue

```
FOR each deferral violation from QA report:
    AskUserQuestion:
        Question: "QA flagged deferral for '{item}'. How to resolve?"
        Header: "Deferral issue"
        Options:
            - "Complete the work now (implement {item})"
            - "Create follow-up story (proper tracking)"
            - "Create ADR (document scope change)"
            - "Document external blocker (with ETA)"
        multiSelect: false

    Based on user selection:
        Execute appropriate resolution (same as Phase 06 Step a. logic)
        Update Implementation Notes with proper justification
```

**Option 1: Complete the work now**
```
User chose: "Complete the work now"

Action:
1. Update story status back to "In Development"
2. Return to Phase 03 (Green) to implement deferred work
3. Run tests to verify new implementation
4. Proceed through Phase 04 (Refactor) and Phase 05 (Integration)
5. Return to Phase 08 DoD validation with completed work
```

**Option 2: Create follow-up story**
```
User chose: "Create follow-up story"

Action:
1. Invoke devforgeai-story-creation skill:

   # Set context markers for minimal user questions
   **Feature Description:** {deferred_work_description}
   **Epic:** {current_epic}
   **Sprint:** Backlog
   **Priority:** {current_priority}
   **Parent Story:** {current_story_id}
   **Story Type:** Deferral Tracking

   Skill(command="devforgeai-story-creation")

2. Skill generates complete follow-up story with:
   - Full acceptance criteria (not minimal placeholders)
   - Technical specification
   - UI specification (if applicable)
   - Self-validation (Phase 10)

3. After story created:
   Extract new STORY_ID from skill result

4. Update current story Implementation Notes:
   Edit(
       old_string="- [ ] {deferred_item}",
       new_string="- [ ] {deferred_item} - Deferred to {new_STORY_ID}: {reason}"
   )

5. Display: "✓ Follow-up story {new_STORY_ID} created for deferred work"
```

**Option 3: Create ADR for scope change**
```
User chose: "Create ADR"

Action:
1. Invoke architect-reviewer subagent to create ADR
2. Document scope change decision
3. Update current story Implementation Notes:
   - [ ] {item} - Out of scope: ADR-XXX (reason)
4. Link ADR in story
```

**Option 4: Document external blocker**
```
User chose: "Document external blocker"

Action:
1. Collect blocker details via AskUserQuestion:
   - Blocker description
   - Dependency (team, system, decision)
   - ETA or resolution date

2. Update story Implementation Notes:
   - [ ] {item} - Blocked by: {dependency} (ETA: {date})

3. Track in technical debt register
```

### Step 3: Run Light QA to Verify Fixes

```
After resolving all deferral issues:
    Display: "Deferral issues resolved. Running light QA validation..."

    # Don't need full deep QA, just validate deferrals fixed
    Read updated Implementation Notes
    Verify all incomplete items now have valid justifications

    IF validation passes:
        Display: "Deferral issues resolved ✓ Ready for QA re-evaluation"
        Update story status remains "Dev Complete"

    ELSE:
        Display: "Some deferral issues remain. Please review."
        List remaining issues
```

---

## Trigger Conditions

This workflow triggered when:
- Story status is "Dev Complete" or "QA Failed"
- Previous QA report shows "Deferral Validation FAILED"
- User runs /dev {story-id} after QA failure

**Detection:** Phase 01 Step h. sets `$QA_DEFERRAL_FAILURE = true`

---

## Exit Criteria

Deferral recovery succeeds when:
- [ ] All deferral violations from QA report resolved
- [ ] Implementation Notes updated with valid justifications
- [ ] Follow-up stories created (if deferrals chosen)
- [ ] ADRs created (if scope changes)
- [ ] Blockers documented (if external dependencies)
- [ ] Ready for QA re-validation

---

## Integration with devforgeai-story-creation

**Why use story-creation skill for follow-up stories:**

1. **Complete Specifications:** Generates full story with AC, tech spec, UI spec, NFRs
2. **Self-Validation:** Phase 10 validates quality before completion
3. **Prevents Technical Debt:** Stringent validation prevents incomplete tracking stories
4. **Consistency:** All stories created via same skill, same quality standard
5. **Framework-Aware:** Respects context files, integrates properly

**Context markers minimize user questions:**
- Epic/Sprint inherited from parent
- Priority inherited
- Story type marked as "Deferral Tracking"
- Parent story ID tracked for dependencies

---

## Next Steps After Recovery

After all deferral issues resolved:
1. Continue with current development workflow (if "Complete now" chosen)
2. Proceed to Phase 08 (Git/DoD) with resolved deferrals
3. Story ready for QA re-evaluation
