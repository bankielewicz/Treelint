# Phase 3A: Story Status Update

Updates story status and appends workflow history entries after phase completion.

## Purpose

**Maintain accurate story tracking** throughout orchestration:
1. Update story status in YAML frontmatter
2. Mark workflow checkboxes as complete
3. Append workflow history entries with timestamps
4. Create audit trail for all status transitions

**When executed:**
- After each major phase completion (Development, QA, Release)
- After sprint planning (Backlog → Ready for Dev)
- After quality gate passage

---

## Step 1: Update Status in Frontmatter

### Status Transition

```
Edit(
  file_path="devforgeai/specs/Stories/{story_id}.story.md",
  old_string="status: {old_status}",
  new_string="status: {new_status}"
)
```

**Valid transitions** (see `state-transitions.md` for complete rules):
- Backlog → Architecture
- Architecture → Ready for Dev
- Ready for Dev → In Development
- In Development → Dev Complete
- Dev Complete → QA In Progress
- QA In Progress → QA Approved OR QA Failed
- QA Approved → Releasing
- Releasing → Released

**Invalid transitions** (will HALT):
- Skipping states (Backlog → Dev Complete)
- Backward transitions without reason (Released → In Development)

---

## Step 2: Update Workflow Checkboxes

### Definition of Done Checkboxes

```
Checkboxes in Definition of Done section:
- [ ] All acceptance criteria implemented → - [x] (after Dev Complete)
- [ ] Unit tests written (95% coverage) → - [x] (after Dev Complete)
- [ ] Integration tests written → - [x] (after Dev Complete)
- [ ] Code reviewed → - [x] (after QA Approved)
- [ ] Documentation updated → - [x] (after QA Approved)
- [ ] QA approved → - [x] (after QA Approved)
- [ ] Deployed to staging → - [x] (after Staging Complete)
- [ ] Deployed to production → - [x] (after Released)
```

### Workflow Phase Checkboxes

```
Checkboxes in Workflow Status section:
- [ ] Architecture phase complete → - [x] (after context files created)
- [ ] Development phase complete → - [x] (after Dev Complete)
- [ ] QA phase complete → - [x] (after QA Approved)
- [ ] Released → - [x] (after Released)
```

### Update Logic

```
FOR each checkbox associated with completed phase:
  old_checkbox = "- [ ] {checkbox_text}"
  new_checkbox = "- [x] {checkbox_text}"

  Edit(
    file_path="devforgeai/specs/Stories/{story_id}.story.md",
    old_string=old_checkbox,
    new_string=new_checkbox
  )
```

**Note:** Only update checkboxes for **completed** phases. Don't check future phases.

---

## Step 3: Append Workflow History

### Workflow History Format

**Standard entry template:**
```markdown
### {ISO 8601 timestamp} - {New Status}

**Previous Status:** {old_status}
**Action Taken:** {description of what happened}
**Result:** {outcome summary}
**Next Steps:** {recommended actions}
```

**Example entry:**
```markdown
### 2025-01-06T14:23:45Z - Dev Complete

**Previous Status:** In Development
**Action Taken:** devforgeai-development skill executed TDD workflow (Red → Green → Refactor)
**Result:** All tests passing (174/174), coverage 91%, light QA passed
**Next Steps:** Ready for deep QA validation
**Checkpoint:** DEV_COMPLETE
```

---

### Checkpoint Markers

**Add checkpoint marker** for resume functionality:

```
Checkpoint: {CHECKPOINT_NAME}
Timestamp: {ISO 8601}
```

**Checkpoint types:**
- `DEV_COMPLETE` - After development skill completes
- `QA_APPROVED` - After QA skill approves
- `STAGING_COMPLETE` - After staging deployment succeeds
- `PRODUCTION_COMPLETE` - After production deployment succeeds

**Purpose:** Enable orchestration resume (see `checkpoint-detection.md`)

---

### Append Logic

```
# Read current workflow history section
Read(file_path="devforgeai/specs/Stories/{story_id}.story.md")
Extract: Workflow Status section

# Generate new entry
new_entry = format_workflow_entry(
  timestamp=current_timestamp,
  new_status=new_status,
  old_status=old_status,
  action=action_description,
  result=result_summary,
  next_steps=next_steps,
  checkpoint=checkpoint_name (if applicable)
)

# Append to workflow history
Edit(
  file_path="devforgeai/specs/Stories/{story_id}.story.md",
  old_string="## Workflow Status\n\n",
  new_string=f"## Workflow Status\n\n{new_entry}\n\n"
)
```

**Note:** New entries prepend to Workflow Status section (reverse chronological - latest first).

---

## Status Update Examples

### Development Complete

```markdown
### 2025-01-06T14:23:45Z - Dev Complete

**Previous Status:** In Development
**Action Taken:** devforgeai-development skill executed TDD workflow
**Phases Executed:**
- Red Phase: 12 failing tests generated from acceptance criteria
- Green Phase: Implementation written to pass all tests
- Refactor Phase: Code quality improved (complexity 8 → 5)
- Integration Phase: 3 cross-component tests added

**Result:**
- All tests passing (174/174)
- Coverage: 91% (exceeds 85% threshold)
- Light QA: PASSED
- Build: SUCCESS

**Next Steps:**
- Proceed to deep QA validation
- Review code quality metrics
- Validate against acceptance criteria

**Checkpoint:** DEV_COMPLETE
```

---

### QA Approved

```markdown
### 2025-01-06T15:45:22Z - QA Approved

**Previous Status:** QA In Progress
**Action Taken:** devforgeai-qa skill executed deep validation

**Validation Results:**
- Coverage Analysis: 91% (✓ exceeds 85% threshold)
- Anti-Pattern Detection: 0 violations found
- Spec Compliance: All 5 acceptance criteria implemented
- Code Quality: Maintainability index 78 (✓ ≥70)

**Result:** PASSED

**Violations:**
- CRITICAL: 0
- HIGH: 0
- MEDIUM: 2 (minor style issues - non-blocking)
- LOW: 1 (suggestion for optimization)

**Report:** devforgeai/qa/reports/STORY-042-qa-report.md

**Next Steps:**
- Proceed to staging deployment
- Monitor deployment smoke tests
- Prepare production release

**Checkpoint:** QA_APPROVED
```

---

### Released

```markdown
### 2025-01-06T16:30:15Z - Released

**Previous Status:** Releasing
**Action Taken:** devforgeai-release skill deployed to production

**Deployment Summary:**
- Staging: Deployed 2025-01-06T16:10:32Z (✓ smoke tests passed)
- Production: Deployed 2025-01-06T16:30:15Z (✓ smoke tests passed)

**Environments:**
- Staging: https://staging.example.com
- Production: https://example.com

**Result:** Production deployment successful

**Smoke Tests:**
- Health check: ✓ PASSED
- API endpoints: ✓ PASSED
- Database connection: ✓ PASSED

**Monitoring:**
- Application logs: /var/log/app/
- Error tracking: Sentry dashboard
- Performance metrics: Datadog

**Next Steps:**
- Monitor production metrics for 24 hours
- Review error logs
- Update epic progress (if associated)

**Checkpoint:** PRODUCTION_COMPLETE
```

---

## Related Files

- **checkpoint-detection.md** - Uses checkpoints created by this phase
- **story-validation.md** - Validates before status updates
- **orchestration-finalization.md** - Final status update in Phase 6
- **workflow-states.md** - Valid status values
- **state-transitions.md** - Valid transitions and prerequisites
- **story-management.md** - Complete story lifecycle procedures
