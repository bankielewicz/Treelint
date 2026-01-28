# Phase 09: Feedback Hook Integration

**Entry Gate:**
```bash
devforgeai-validate phase-check ${STORY_ID} --from=08 --to=09

Examples (--project-root applies to phase-* commands only, not check-hooks/invoke-hooks):
 - Correct: devforgeai-validate phase-init ${STORY_ID} --project-root=.
 - Incorrect: python -m devforgeai.cli.devforgeai_validate phase-init ${STORY_ID} --project-root=.
# Exit code 0: Transition allowed
# Exit code 1: Phase 08 not complete - HALT (commit not done)
```

---

## Phase Workflow

**Purpose:** Invoke feedback hooks for retrospective insights and AI analysis

**Required Subagents:** `framework-analyst` (for AI analysis synthesis)

**Execution:** After Phase 08 (Git commit) completes

**Steps:**

### Step 1: User Feedback Hook (existing)

1. **Check user feedback hooks configuration**
   ```bash
   devforgeai-validate check-hooks --operation=dev --status=success --type=user
   # Exit code 0: User feedback hooks enabled
   # Exit code 1: Hooks disabled - skip step 2
   ```

2. **Invoke user feedback hooks if enabled**
   ```bash
   devforgeai-validate invoke-hooks --operation=dev --story=${STORY_ID} --type=user
   # Triggers devforgeai-feedback skill (conversation mode)
   ```

### Step 2: AI Analysis (Subagent Delegation)

**2.1 Read Observations from Phase State**

```
Read(file_path="devforgeai/workflows/${STORY_ID}-phase-state.json")
```

Extract the `observations` array captured during phases 01-08.

**2.2 Invoke Framework Analyst Subagent**

```
Task(
  subagent_type="framework-analyst",
  prompt="Analyze the ${STORY_ID} workflow execution and generate framework improvement recommendations.

INPUT:
- Story ID: ${STORY_ID}
- Workflow Type: dev
- Phase State Path: devforgeai/workflows/${STORY_ID}-phase-state.json
- Observations: [observations from phase-state.json]

INSTRUCTIONS:
1. Read phase-state.json and extract observations array
2. Read each file mentioned in observations to gather context
3. Check recommendations-queue.json for duplicates
4. Check recent git commits for already-implemented items
5. Expand terse observations into structured recommendations
6. Return ONLY valid JSON matching the required schema

Return ONLY valid JSON - no markdown, no explanation text."
)
```

**2.3 Validate Subagent Output (BLOCKING)**

If any validation check fails, log error and skip storage:

**2.3.1 JSON Schema Validation**
- Parse output as JSON
- Validate against `devforgeai/feedback/schema.json` ai_analysis structure
- FAIL if invalid JSON or missing required fields

**2.3.2 Aspirational Language Check**
- Scan all string fields for forbidden words:
  - "could", "might", "consider", "should explore", "potentially", "possibly", "maybe", "perhaps"
- FAIL if any aspirational language detected

**2.3.3 Evidence Requirement Check**
- Each `what_worked_well` item must have non-empty `evidence`
- Each `areas_for_improvement` item must have non-empty `evidence` and `root_cause`
- Each recommendation must have non-empty `affected_files` array (1+ specific paths)
- FAIL if evidence missing

**2.3.4 Effort Estimate Check**
- Each recommendation must have valid `effort_estimate`
- Valid values: "15 min", "30 min", "45 min", "1 hour", "2 hours", "4 hours"
- FAIL if missing or invalid format

**2.3.5 Feasibility Check**
- Each recommendation must have `feasible_in_claude_code: true`
- FAIL if false or missing

**2.4 Apply Merit Filter (Post-Validation)**

For each recommendation that passed validation:

```
Read(file_path="devforgeai/feedback/ai-analysis/aggregated/recommendations-queue.json")
```

- **Duplicate Check:** Compare against existing queue items (>80% description similarity)
- **Already-Implemented Check:** Check if affected files modified in last 7 days with related commit message
- PASS: Include in storage
- FILTER: Log reason, exclude from storage

**2.5 Store Results (Only if Validation Passed)**

```
Write(file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/${TIMESTAMP}-ai-analysis.json")
```

Content structure:
```json
{
  "story_id": "${STORY_ID}",
  "timestamp": "${TIMESTAMP}",
  "validation_passed": true,
  "ai_analysis": {
    "what_worked_well": [...],
    "areas_for_improvement": [...],
    "recommendations": [...],
    "patterns_observed": [...],
    "anti_patterns_detected": [...],
    "constraint_analysis": "..."
  },
  "merit_filter_results": {
    "passed": 2,
    "filtered": 1,
    "filter_reasons": ["Duplicate of existing recommendation"]
  }
}
```

**2.6 Update Aggregated Queue (if HIGH priority recommendations)**

If any recommendation has `priority: "HIGH"`:
```
# Read existing queue
Read(file_path="devforgeai/feedback/ai-analysis/aggregated/recommendations-queue.json")

# Append HIGH priority recommendations
# Write updated queue
```

### Step 3: Handle Results

**3.1 Handle Validation Failure**
If validation failed:
- Log: "AI Analysis validation failed: [specific reason]"
- Do NOT store invalid output
- Continue to Phase 10 (non-blocking)
- Increment counter: `validation_failures` in metrics

**3.2 Handle Storage Success**
If storage succeeded:
- Log: "AI Analysis stored: ${STORY_ID}/${TIMESTAMP}-ai-analysis.json"
- Log: "Recommendations passed merit filter: X of Y"
- Continue to Phase 10

**3.3 Non-Blocking Behavior**
- Hook/analysis failures do NOT prevent workflow completion
- Log any errors for debugging
- Continue to Phase 10 regardless of outcome

**Reference:** See STORY-023 implementation notes, AI Analysis enhancement (2025-12-28)

---

## Progress Indicator

Display at start of Phase 09:
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 09/10: Feedback Hook (89% → 95% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## Validation Checkpoint

**Before proceeding to Phase 10, verify:**

- [ ] check-hooks command executed (user feedback)
- [ ] invoke-hooks command executed (if user hooks enabled)
- [ ] Phase-state.json observations array read
- [ ] framework-analyst subagent invoked
- [ ] Subagent output validated (JSON, aspirational check, evidence, effort, feasibility)
- [ ] Merit filter applied (duplicates, already-implemented)
- [ ] Results stored in `devforgeai/feedback/ai-analysis/${STORY_ID}/` (if validation passed)

**Note:** This checkpoint is NON-BLOCKING - validation failures are logged but don't halt workflow

**AI Analysis Output Example (Expanded Schema):**
```json
{
  "story_id": "STORY-XXX",
  "workflow_type": "dev",
  "analysis_date": "2025-12-29T10:00:00Z",
  "observations_processed": 3,
  "what_worked_well": [
    {
      "observation": "Phase state validation prevented skipping Phase 03",
      "evidence": ".claude/scripts/devforgeai_cli/commands/phase_commands.py:45",
      "impact": "Enforced TDD discipline, caught implementation without tests"
    }
  ],
  "areas_for_improvement": [
    {
      "issue": "Test naming convention for shell scripts unclear",
      "evidence": "Checked test-automator.md, tdd-patterns.md - no shell guidance",
      "root_cause": "Framework originally designed for Python/JS, shell testing added later"
    }
  ],
  "recommendations": [
    {
      "title": "Document shell script test naming convention",
      "description": "Add explicit guidance for shell script test naming",
      "affected_files": [".claude/agents/test-automator.md"],
      "implementation_code": "Add ### Shell Script Testing section with Bats/shell patterns",
      "effort_estimate": "15 min",
      "priority": "MEDIUM",
      "feasible_in_claude_code": true
    }
  ],
  "patterns_observed": ["Phase state enforcement working well across all 10 phases"],
  "anti_patterns_detected": [],
  "constraint_analysis": "Context files effectively prevented 2 anti-pattern violations"
}

---

**Exit Gate:**
```bash
devforgeai-validate phase-complete ${STORY_ID} --phase=09 --checkpoint-passed
# Exit code 0: Phase complete, proceed to Phase 10
# Note: Always succeeds (non-blocking phase)
```
