# Phase 5: Next Action Determination

Determines recommended next steps based on current story status and workflow stage.

## Purpose

**Provide clear guidance** on what should happen next:
- Recommend which skill to invoke
- Suggest manual actions if needed
- Identify blockers preventing progression
- Guide user to appropriate command

**When executed:**
- After each phase completion
- When story status changes
- When orchestration pauses
- At workflow decision points

---

## Decision Logic

### Status-Based Recommendations

```
Current Status → Recommended Next Action

Backlog → Architecture:
  Action: Invoke devforgeai-architecture
  Command: Automatic (orchestration handles)
  Reason: Context files needed before development

Architecture → Ready for Dev:
  Action: Context files created, ready for developer
  Command: /dev {STORY_ID} (manual) OR wait for sprint planning
  Reason: Development can begin when story assigned

Ready for Dev → In Development:
  Action: Developer starts work
  Command: /dev {STORY_ID}
  Reason: Story ready for TDD implementation

In Development → Dev Complete:
  Action: TDD workflow complete
  Command: Automatic (dev skill updates status)
  Reason: Implementation finished, ready for QA

Dev Complete → QA In Progress:
  Action: Invoke devforgeai-qa
  Command: Automatic (orchestration handles)
  Reason: Deep validation needed before release

QA In Progress → QA Approved:
  Action: Quality validation passed
  Command: Automatic (QA skill updates status)
  Reason: Ready for deployment

QA In Progress → QA Failed:
  Action: Developer fixes violations
  Command: /dev {STORY_ID} (to fix issues)
  Reason: QA found blocking violations

QA Approved → Releasing:
  Action: Invoke devforgeai-release
  Command: Automatic (orchestration handles)
  Reason: Deploy to staging and production

Releasing → Released:
  Action: Deployment complete
  Command: None (workflow complete)
  Reason: Story delivered to production

Released → [End State]:
  Action: No further action
  Command: None
  Reason: Story complete, monitor production
```

**For complete decision tree:** See `state-transitions.md`

---

## Blocker Detection

### Quality Gate Blocks

```
IF quality gate blocks progression:
  gate_number = identify which gate (1-4)
  requirements = load gate requirements from quality-gates.md

  Display:
  "⛔ Quality Gate {gate_number} Blocks Progression

  Current Status: {current_status}
  Attempted Transition: {current_status} → {next_status}

  Gate Requirements Not Met:
  {requirements}

  Action: {remediation_steps}

  See: quality-gates.md for complete requirements"

  Next Action: Fix gate requirements, then retry transition
```

---

### External Dependencies

```
IF story has external blockers (from deferred-tracking.md):
  blocker_list = load from technical-debt-register.md

  Display:
  "⏸️  External Blockers Detected

  Story: {STORY_ID}
  Blocked By:
  {FOR blocker in blocker_list:}
    - {blocker.description} (since {blocker.deferred_date})
  {END FOR}

  Next Action:
  1. Escalate blockers to {responsible_team}
  2. Monitor blocker status
  3. Resume workflow when blockers resolved"

  Next Action: Wait for blocker resolution
```

---

### Sprint Assignment

```
IF story status == "Ready for Dev" AND sprint == null:
  Display:
  "📋 Story Ready but Not Assigned to Sprint

  Story: {STORY_ID}
  Status: Ready for Dev
  Sprint: Not assigned

  Next Action:
  1. Run /create-sprint to plan sprint
  2. Include this story in sprint selection
  3. After sprint planned, run /dev {STORY_ID}"

  Next Action: Assign to sprint before development begins
```

---

## Context-Aware Recommendations

### After Development Complete

```
IF story status == "Dev Complete":
  # Automatic: Orchestration invokes QA
  # Manual: User can invoke /qa directly

  Recommendation:
  "✅ Development Complete

  Story: {STORY_ID}
  Status: Dev Complete
  Coverage: {coverage_percentage}%
  Tests: All passing ({test_count} tests)

  Next Action (Automatic):
  - Orchestration will invoke deep QA validation
  - QA skill will run comprehensive validation
  - Coverage, anti-patterns, spec compliance checked

  OR Manual:
  - Run: /qa {STORY_ID} (if outside orchestration)
  - Deep validation will execute"
```

---

### After QA Approved

```
IF story status == "QA Approved":
  Recommendation:
  "✅ QA Approved

  Story: {STORY_ID}
  Status: QA Approved
  Coverage: {coverage_percentage}%
  Violations: 0 CRITICAL, 0 HIGH

  Next Action (Automatic):
  - Orchestration will deploy to staging
  - Smoke tests will execute
  - If staging passes, production deployment follows

  OR Manual:
  - Run: /release {STORY_ID} staging (if outside orchestration)
  - Staging deployment will execute"
```

---

### After QA Failed

```
IF story status == "QA Failed":
  # Read QA report to understand failure
  Read(file_path="devforgeai/qa/reports/{STORY_ID}-qa-report.md")
  failure_type = parse failure type
  violations = parse violations

  Recommendation:
  "❌ QA Failed

  Story: {STORY_ID}
  Status: QA Failed
  Failure Type: {failure_type}
  Violations: {violations_summary}

  Next Action:
  1. Review QA report: devforgeai/qa/reports/{STORY_ID}-qa-report.md
  2. Fix violations in code
  3. Run: /dev {STORY_ID} (to apply fixes)
  4. Orchestration will automatically retry QA

  OR (if deferral failures):
  - Create follow-up stories for deferred work
  - Mark deferrals as justified
  - Proceed with justified deferrals"
```

---

### After Release Complete

```
IF story status == "Released":
  Recommendation:
  "✅ Story Released to Production

  Story: {STORY_ID}
  Status: Released
  Deployed: {completed_date}
  Environment: Production

  Next Actions:
  1. Monitor production metrics for 24-48 hours
  2. Review error logs and performance
  3. Update epic progress (if associated)
  4. Update sprint burndown
  5. Close story in project management tool

  No further orchestration needed - story complete!"
```

---

## Output

**Next Action Recommendation:**
```json
{
  "current_status": "Dev Complete",
  "next_status": "QA In Progress",
  "recommended_action": "Invoke devforgeai-qa for deep validation",
  "command": "Automatic (orchestration handles)",
  "manual_command": "/qa STORY-042",
  "reason": "Implementation finished, quality validation needed",
  "estimated_duration": "5-10 minutes"
}
```

---

## Related Files

- **workflow-states.md** - All 11 state definitions
- **state-transitions.md** - Valid transitions and prerequisites
- **quality-gates.md** - Gate requirements that can block
- **skill-invocation.md** - Phase 2, automatic skill coordination
- **checkpoint-detection.md** - Phase 0, determines starting point
