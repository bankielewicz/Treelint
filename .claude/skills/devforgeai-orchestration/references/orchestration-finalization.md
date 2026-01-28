# Phase 6: Orchestration Finalization

Final phase that generates completion summary and updates workflow history.

## Purpose

**Complete orchestration workflow** with comprehensive documentation:
1. Calculate orchestration metrics (duration, phases, retries)
2. Update story workflow history with complete timeline
3. Update story YAML frontmatter with completion metadata
4. Generate structured completion summary for command display
5. Provide next steps for production monitoring

**This phase moves finalization logic FROM /orchestrate command TO skill** for proper workflow state management (lean orchestration pattern).

---

## Step 1: Calculate Orchestration Metrics

### Collect Timeline Data

```
Orchestration start time:
  start_time = extracted from Phase 0 checkpoint or current session start

Current time:
  current_time = now (ISO 8601 timestamp)

Total duration:
  total_duration = current_time - start_time
  Format: "2h 34m 15s" or "45m 22s"
```

---

### Collect Phase Durations

```
Phase 2 (Development):
  dev_start = Phase 2 invocation timestamp
  dev_end = DEV_COMPLETE checkpoint timestamp
  dev_duration = dev_end - dev_start

Phase 3 (QA Validation):
  qa_start = Phase 3 invocation timestamp
  qa_end = QA_APPROVED checkpoint timestamp
  qa_duration = qa_end - qa_start
  qa_retry_count = count QA attempts (from Phase 3.5 if retries occurred)

Phase 4 (Staging Release):
  staging_start = Phase 4 invocation timestamp
  staging_end = STAGING_COMPLETE checkpoint timestamp
  staging_duration = staging_end - staging_start

Phase 5 (Production Release):
  production_start = Phase 5 invocation timestamp
  production_end = PRODUCTION_COMPLETE checkpoint timestamp
  production_duration = production_end - production_start
```

---

### Collect Phase Results

```
Development phase:
  dev_test_count = extract from dev skill result
  dev_pass_rate = extract from dev skill result (should be 100%)
  dev_coverage = extract from dev skill light QA results
  dev_commits = count git commits OR file-based changes

QA validation:
  Read(file_path="devforgeai/qa/reports/{STORY_ID}-qa-report.md")

  qa_coverage_business_logic = extract business logic coverage %
  qa_coverage_application = extract application coverage %
  qa_coverage_infrastructure = extract infrastructure coverage %
  qa_violations_critical = count CRITICAL violations (should be 0)
  qa_violations_high = count HIGH violations (should be 0)
  qa_violations_medium = count MEDIUM violations
  qa_violations_low = count LOW violations
  qa_overall_result = "PASSED" (if at this phase)

Staging release:
  staging_deployment_status = extract from release skill result
  staging_smoke_tests = extract test results
  staging_health_checks = extract health status

Production release:
  production_deployment_status = extract from release skill result
  production_smoke_tests = extract test results
  production_health_checks = extract health status
```

---

### Collect Checkpoints

```
checkpoints_created = [
  "DEV_COMPLETE" (created in Phase 2),
  "QA_APPROVED" (created in Phase 3 or 3.5 after retries),
  "STAGING_COMPLETE" (created in Phase 4),
  "PRODUCTION_COMPLETE" (created in Phase 5)
]

FOR each checkpoint:
  checkpoint_timestamp = extract from workflow history
  checkpoints_with_times.append({
    "name": checkpoint,
    "timestamp": checkpoint_timestamp
  })
```

---

## Step 2: Update Story Workflow History

### Append Orchestration Summary

```
Edit(file_path="devforgeai/specs/Stories/{STORY_ID}.story.md")

Find "## Workflow Status" section
Prepend comprehensive orchestration summary:

old_string: "## Workflow Status\n\n"
new_string: "## Workflow Status\n\n{orchestration_summary}\n\n"
```

**Orchestration summary template:**

```markdown
## Orchestration Complete - {current_timestamp}

### Timeline
- **Started:** {start_time}
- **Completed:** {current_time}
- **Total Duration:** {total_duration}

### Phases Executed
- **Phase 2: Development** ✅ ({dev_duration})
  - Tests: {dev_test_count} (100% pass rate)
  - Commits: {commit_count}
  - Coverage: {dev_coverage}% (light QA)
  - Status: Dev Complete
  - Checkpoint: DEV_COMPLETE

- **Phase 3: QA Validation** ✅ ({qa_duration})
  - Mode: Deep
  - Coverage: Business {bl_coverage}%, Application {app_coverage}%, Infrastructure {infra_coverage}%
  - Violations: CRITICAL: 0, HIGH: 0, MEDIUM: {med_count}, LOW: {low_count}
  {IF qa_retry_count > 0:}
  - Retry Attempts: {qa_retry_count}
  - Issues Fixed: {retry_issues_summary}
  - Final Result: APPROVED (after {qa_retry_count} retries)
  {ELSE:}
  - Result: APPROVED (first attempt)
  {END IF}
  - Status: QA Approved
  - Checkpoint: QA_APPROVED

- **Phase 4: Staging Release** ✅ ({staging_duration})
  - Environment: Staging
  - Deployment: Successful
  - Smoke Tests: Passed ({smoke_test_count}/{smoke_test_count})
  - Health Checks: Green (CPU: {cpu}%, Memory: {mem}%, Response: {resp_time}ms)
  - Status: Staging Complete
  - Checkpoint: STAGING_COMPLETE

- **Phase 5: Production Release** ✅ ({production_duration})
  - Environment: Production
  - Deployment: Successful
  - Smoke Tests: Passed ({smoke_test_count}/{smoke_test_count})
  - Health Checks: Green (CPU: {cpu}%, Memory: {mem}%, Response: {resp_time}ms)
  - Status: Released
  - Checkpoint: PRODUCTION_COMPLETE

### Quality Gates Passed
- [x] Gate 1: Context Validation (all 6 context files exist and valid)
- [x] Gate 2: Test Passing (100% pass rate, build successful, light QA passed)
- [x] Gate 3: QA Approval (coverage thresholds met, zero CRITICAL/HIGH violations)
- [x] Gate 4: Release Readiness (staging validated, smoke tests passed, health green)

### Checkpoints Reached
{FOR checkpoint in checkpoints_with_times:}
  - **{checkpoint.name}**: {checkpoint.timestamp}
{END FOR}

### Metrics Summary
- **Total Tests:** {dev_test_count}
- **Coverage:** BL {bl_coverage}%, App {app_coverage}%, Infra {infra_coverage}%
- **Quality:** 0 CRITICAL, 0 HIGH violations
- **Deployments:** Staging ✅, Production ✅
- **Duration:** {total_duration}
{IF qa_retry_count > 0:}
- **QA Retries:** {qa_retry_count} (issues resolved successfully)
{END IF}

### Final Status
**Status:** Released ✅
**Deployment:** Production live {production_timestamp}
**Health:** All systems green
**Monitor:** Production metrics for 24-48 hours
**Next:** Close related issues, notify stakeholders
```

---

## Step 3: Update Story YAML Frontmatter

### Add Completion Metadata

```
Edit(file_path="devforgeai/specs/Stories/{STORY_ID}.story.md")

Current YAML frontmatter:
---
id: STORY-042
title: User Login with Email Authentication
epic: EPIC-001
sprint: Sprint-3
status: QA Approved
points: 5
priority: High
---

Update status:
  old_string: "status: QA Approved"
  new_string: "status: Released"

Add completion fields (insert before closing ---):
  old_string: "---"
  new_string: "completed_date: {current_timestamp}
released_by: orchestration
orchestration_duration: {total_duration}
orchestration_retries: {qa_retry_count}
deployment_staging: {staging_timestamp}
deployment_production: {production_timestamp}
---"

Final YAML frontmatter:
---
id: STORY-042
title: User Login with Email Authentication
epic: EPIC-001
sprint: Sprint-3
status: Released
points: 5
priority: High
completed_date: 2025-01-06T16:30:15Z
released_by: orchestration
orchestration_duration: 2h 34m 15s
orchestration_retries: 1
deployment_staging: 2025-01-06T16:10:32Z
deployment_production: 2025-01-06T16:30:15Z
---
```

---

## Step 4: Generate Completion Summary for Command Display

### Structured Result Object

**Return to /orchestrate command:**

```json
{
  "status": "ORCHESTRATION_COMPLETE",
  "story_id": "STORY-042",
  "story_title": "User Login with Email Authentication",
  "final_status": "Released",
  "timeline": {
    "start": "2025-01-06T14:00:00Z",
    "end": "2025-01-06T16:30:15Z",
    "duration": "2h 34m 15s"
  },
  "phases_completed": [2, 3, 4, 5],
  "phases_skipped": [],
  "retry_attempts": 1,
  "checkpoints": [
    "DEV_COMPLETE",
    "QA_APPROVED",
    "STAGING_COMPLETE",
    "PRODUCTION_COMPLETE"
  ],
  "metrics": {
    "tests": "174 (100% pass)",
    "coverage": "BL: 96%, App: 91%, Infra: 88%",
    "violations": "CRITICAL: 0, HIGH: 0",
    "deployments": "Staging ✅, Production ✅"
  },
  "summary_message": "🎉 Story STORY-042 Orchestration Complete!

✅ Development: Code implemented, 174 tests passing
✅ QA: All quality gates passed, 1 retry handled successfully
✅ Staging: Deployed and validated successfully
✅ Production: Live with green health checks

Duration: 2h 34m 15s
Status: Released

Monitor production metrics for 24 hours.
Close related issues and notify stakeholders.",
  "next_steps": [
    "Monitor production: View metrics and error rates",
    "Verify functionality: Run smoke tests in production",
    "Close related issues: Mark tickets as deployed",
    "Notify stakeholders: Feature is live",
    "Schedule retrospective: Team debrief on delivery"
  ]
}
```

**Command displays:** `summary_message` and `next_steps` from this result.

---

### Display Template

**Command outputs:**

```
🎉 Story STORY-042 Orchestration Complete!

✅ Development: Code implemented, 174 tests passing
✅ QA: All quality gates passed, 1 retry handled successfully
✅ Staging: Deployed and validated successfully
✅ Production: Live with green health checks

📊 Metrics:
- Duration: 2h 34m 15s
- Coverage: BL: 96%, App: 91%, Infra: 88%
- Tests: 174 (100% pass rate)
- Violations: 0 CRITICAL, 0 HIGH

🚀 Deployments:
- Staging: 2025-01-06T16:10:32Z ✅
- Production: 2025-01-06T16:30:15Z ✅

📋 Next Steps:
1. Monitor production metrics for 24 hours
2. Verify functionality with smoke tests
3. Close related issues
4. Notify stakeholders
5. Schedule retrospective
```

---

## Special Cases

### Orchestration with QA Retries

**Timeline includes retry cycles:**

```markdown
### Phases Executed (with Retries)
- **Phase 2: Development** ✅ (45m)
  - Initial implementation complete

- **Phase 3: QA Validation (Attempt 1)** ❌ (12m)
  - Result: FAILED (coverage 88% vs 95% threshold)

- **Phase 2: Development (Retry Fix)** ✅ (23m)
  - Added 12 unit tests for payment module

- **Phase 3: QA Validation (Attempt 2)** ✅ (14m)
  - Result: PASSED (coverage 96%)

- **Phase 4: Staging Release** ✅ (18m)
  - Deployed successfully

- **Phase 5: Production Release** ✅ (22m)
  - Deployed successfully

Total Duration: 2h 14m (includes 1 QA retry cycle)
```

**Metrics show:**
- QA Retry Attempts: 1
- Issues Fixed: Coverage gap (88% → 96%)
- Final Result: All gates passed

---

### Orchestration with Checkpoint Resume

**Timeline shows skipped phases:**

```markdown
### Phases Executed (Resumed from QA_APPROVED Checkpoint)
- **Phase 2: Development** ⏭️ Skipped (checkpoint resume)

- **Phase 3: QA Validation** ⏭️ Skipped (checkpoint resume)

- **Phase 4: Staging Release** ✅ (18m)
  - Resumed from checkpoint
  - Deployment successful

- **Phase 5: Production Release** ✅ (22m)
  - Deployment successful

Total Duration: 40m (resumed from QA_APPROVED checkpoint)
Phases Skipped: 2 (Development, QA - already complete)
```

**Metrics show:**
- Resumed from: QA_APPROVED
- Phases executed: 2 of 4 (Staging, Production)
- Phases skipped: 2 of 4 (Development, QA)

---

## Output

**Orchestration Finalization Result:**

```json
{
  "status": "ORCHESTRATION_COMPLETE",
  "story_id": "STORY-042",
  "story_title": "User Login with Email Authentication",
  "final_status": "Released",
  "timeline": {
    "start": "2025-01-06T14:00:00Z",
    "end": "2025-01-06T16:30:15Z",
    "duration": "2h 34m 15s"
  },
  "phases_completed": [2, 3, 4, 5],
  "phases_skipped": [],
  "retry_attempts": 1,
  "checkpoints": ["DEV_COMPLETE", "QA_APPROVED", "STAGING_COMPLETE", "PRODUCTION_COMPLETE"],
  "metrics": {
    "tests": "174 (100% pass)",
    "coverage": "BL: 96%, App: 91%, Infra: 88%",
    "violations": "CRITICAL: 0, HIGH: 0",
    "deployments": "Staging ✅, Production ✅"
  },
  "summary_message": "...",
  "next_steps": [...]
}
```

**This result** is returned to /orchestrate command for display to user.

---

## Epic and Sprint Progress Updates

### Update Epic Progress (If Associated)

```
IF story.epic != null:
  Read epic file:
    Read(file_path="devforgeai/specs/Epics/{story.epic}.epic.md")

  Count epic stories:
    Grep(pattern="STORY-[0-9]+", path=epic_file, output_mode="count")
    total_stories = count

  Count completed stories:
    FOR each story in epic:
      IF story.status == "Released":
        completed_stories += 1

  Calculate progress:
    epic_progress = (completed_stories / total_stories) * 100

  Update epic file:
    Edit(
      file_path=epic_file,
      old_string="Progress: {old_progress}%",
      new_string="Progress: {epic_progress}%"
    )

  Display: "Updated epic {story.epic}: {epic_progress}% complete ({completed_stories}/{total_stories})"
```

---

### Update Sprint Burndown (If Associated)

```
IF story.sprint != null:
  Read sprint file:
    Read(file_path="devforgeai/specs/Sprints/{story.sprint}.md")

  Calculate sprint progress:
    total_points = sum all story points in sprint
    completed_points = sum points for Released stories
    remaining_points = total_points - completed_points

  Update sprint burndown:
    Edit(
      file_path=sprint_file,
      old_string="Remaining Points: {old_remaining}",
      new_string="Remaining Points: {remaining_points}"
    )

  Display: "Updated sprint {story.sprint}: {remaining_points} points remaining"
```

---

## Related Files

- **checkpoint-detection.md** - Phase 0, checkpoints created by this phase used for resume
- **qa-retry-workflow.md** - Phase 3.5, retry metrics included in summary
- **story-status-update.md** - Phase 3A, individual phase updates vs final summary
- **skill-invocation.md** - Phase 2, skills invoked that generate metrics
- **quality-gates.md** - All 4 gates validated and documented in summary
