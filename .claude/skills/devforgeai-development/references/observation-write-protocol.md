# Observation Write Protocol

**Purpose:** Enable immediate disk-write of observations from subagents during /dev workflow execution. This ensures observations survive context exhaustion, mid-phase crashes, and session interruptions.

**Status:** Active
**Version:** 1.0
**Story:** STORY-FEEDBACK-002
**Implementation Date:** 2026-01-29

---

## Overview

This protocol complements `observation-capture.md` by specifying **how** subagents write observations to disk immediately, rather than relying on phase-state.json accumulation (which is lost on context exhaustion).

**Key Difference from observation-capture.md:**
- `observation-capture.md`: Defines observation schema and capture guidelines
- `observation-write-protocol.md`: Defines immediate disk-write mechanics for subagents

---

## File Naming Convention

**Pattern:** `{phase-type}-{phase-number}-{subagent-name}.json`

| Type | Phase | Subagent | Resulting Filename |
|------|-------|----------|-------------------|
| phase | 02 | test-automator | `phase-02-test-automator.json` |
| pre | 02 | api-spec | `pre-02-api-spec.json` |
| phase | 03 | backend-architect | `phase-03-backend-architect.json` |
| phase | 04 | code-reviewer | `phase-04-code-reviewer.json` |
| phase | 04 | refactoring-specialist | `phase-04-refactoring-specialist.json` |
| phase | 05 | integration-tester | `phase-05-integration-tester.json` |
| phase | 04.5 | ac-compliance-verifier | `phase-04.5-ac-compliance-verifier.json` |
| phase | 05.5 | ac-compliance-verifier | `phase-05.5-ac-compliance-verifier.json` |

---

## Write Location

**Path:** `devforgeai/feedback/ai-analysis/${STORY_ID}/`

**Example:** For STORY-305, files are written to:
```
devforgeai/feedback/ai-analysis/STORY-305/
├── phase-02-test-automator.json
├── pre-03-impl-plan.json
├── phase-03-backend-architect.json
├── phase-04-code-reviewer.json
├── phase-04-refactoring-specialist.json
└── phase-05-integration-tester.json
```

**Directory Creation:** If directory doesn't exist, create it before writing.

---

## JSON Schema

Each observation file MUST conform to this schema:

```json
{
  "subagent": "string (subagent name)",
  "phase": "string (NN or NN.N format)",
  "story_id": "string (STORY-XXX)",
  "timestamp": "string (ISO-8601 format)",
  "duration_ms": "number (optional, execution time)",
  "observations": [
    {
      "id": "string (obs-NN-NNN format)",
      "category": "enum (friction|success|pattern|gap|idea|bug|warning)",
      "note": "string (description, max 200 chars)",
      "severity": "enum (low|medium|high)",
      "files": ["string (optional, related file paths)"]
    }
  ],
  "metadata": {
    "version": "1.0",
    "write_timestamp": "string (ISO-8601)"
  }
}
```

### Field Definitions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subagent` | string | Yes | Name of the subagent writing observations |
| `phase` | string | Yes | Phase number (e.g., "02", "03", "04.5") |
| `story_id` | string | Yes | Story identifier (e.g., "STORY-305") |
| `timestamp` | string | Yes | ISO-8601 timestamp when subagent started |
| `duration_ms` | number | No | Execution time in milliseconds |
| `observations` | array | Yes | Array of observation objects (can be empty) |
| `metadata.version` | string | Yes | Protocol version ("1.0") |
| `metadata.write_timestamp` | string | Yes | ISO-8601 timestamp of file write |

### Observation Object Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Format: `obs-{phase}-{sequence}` (e.g., `obs-02-001`) |
| `category` | enum | Yes | One of: friction, success, pattern, gap, idea, bug, warning |
| `note` | string | Yes | Brief description (max 200 characters) |
| `severity` | enum | Yes | One of: low, medium, high |
| `files` | array | No | Related file paths (optional) |

---

## Categories Reference

| Category | When to Use | Example |
|----------|-------------|---------|
| `friction` | Pain points, workflow interruptions | "Had to read 4 files to find naming convention" |
| `success` | What worked well | "anti-patterns.md caught God Object" |
| `pattern` | Recurring approaches observed | "Same validation logic in 3 phases" |
| `gap` | Missing features, incomplete coverage | "No example for edge case X" |
| `idea` | Improvement suggestions | "Phase could run in parallel" |
| `bug` | Defects found | "CLI exits 0 on validation failure" |
| `warning` | Potential issues, risks | "Coverage close to threshold" |

---

## Write Workflow

### Step 1: Construct Observation JSON

Build the JSON object with all required fields:

```json
{
  "subagent": "test-automator",
  "phase": "02",
  "story_id": "STORY-305",
  "timestamp": "2026-01-29T10:30:00Z",
  "duration_ms": 45000,
  "observations": [
    {
      "id": "obs-02-001",
      "category": "friction",
      "note": "Unclear test naming convention - checked 3 files before finding pattern",
      "severity": "medium",
      "files": [".claude/agents/test-automator.md", "tests/example.test.ts"]
    }
  ],
  "metadata": {
    "version": "1.0",
    "write_timestamp": "2026-01-29T10:30:45Z"
  }
}
```

### Step 2: Ensure Directory Exists

Before writing, verify the target directory exists:

```
devforgeai/feedback/ai-analysis/${STORY_ID}/
```

If not, create it.

### Step 3: Write to Disk

Use the Write tool to save the observation file:

```
Write(
  file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/phase-{NN}-{subagent}.json",
  content={observation_json}
)
```

### Step 4: Verify Write

Confirm the file was created successfully. If write fails:
1. Log error message
2. Continue execution (non-blocking)
3. Do NOT halt workflow for observation write failure

---

## Error Handling

### Non-Blocking Writes

Observation writes are **non-blocking**. If a write fails:

```
IF Write() fails:
    Log: "Warning: Failed to write observations to {path}: {error}"
    Continue with main task (do not halt)
```

**Rationale:** Observation capture is valuable but not critical. A failed observation write should never block story implementation.

### Directory Creation Failure

If directory creation fails:

```
IF mkdir fails:
    Log: "Warning: Cannot create observation directory {path}"
    Skip observation write
    Continue with main task
```

### Idempotent Overwrites

If file already exists:
- **Overwrite** the existing file (idempotent behavior)
- Do NOT append to existing file
- Each subagent invocation writes fresh observations

---

## Empty Observations

**Always write the file, even with empty observations.**

```json
{
  "subagent": "test-automator",
  "phase": "02",
  "story_id": "STORY-305",
  "timestamp": "2026-01-29T10:30:00Z",
  "duration_ms": 45000,
  "observations": [],
  "metadata": {
    "version": "1.0",
    "write_timestamp": "2026-01-29T10:30:45Z"
  }
}
```

**Rationale:** Empty observation files:
1. Confirm subagent executed (audit trail)
2. Distinguish "no observations" from "failed to write"
3. Provide duration_ms for performance analysis

---

## Subagent Implementation Template

Add this section to subagent specifications (before `## References`):

```markdown
## Observation Capture (MANDATORY - Final Step)

**Before returning, you MUST write observations to disk.**

### Step 1: Construct Observation JSON
```json
{
  "subagent": "{subagent-name}",
  "phase": "{phase-number}",
  "story_id": "${STORY_ID}",
  "timestamp": "{start-timestamp}",
  "duration_ms": {execution_time},
  "observations": [
    {
      "id": "obs-{phase}-{seq}",
      "category": "{friction|success|pattern|gap|idea|bug|warning}",
      "note": "{description, max 200 chars}",
      "severity": "{low|medium|high}",
      "files": ["{optional paths}"]
    }
  ],
  "metadata": {
    "version": "1.0",
    "write_timestamp": "{write-timestamp}"
  }
}
```

### Step 2: Write to Disk
```
Write(
  file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/phase-{NN}-{subagent}.json",
  content={observation_json}
)
```

### Step 3: Verify Write
Confirm file was created. If write fails, log error but continue (non-blocking).

**This write MUST happen even if the main task fails.**
```

---

## Phase 09 Integration

Phase 09 (Feedback) reads observation files from disk:

```
Glob(pattern="devforgeai/feedback/ai-analysis/${STORY_ID}/*.json")

FOR file in glob_results:
    IF file != "consolidated-analysis.json":
        Read(file_path=file)
        observations.extend(file_content.observations)

$CONSOLIDATED_OBSERVATIONS = sorted(observations, key=lambda x: x.id)
```

Phase 09 then passes `$CONSOLIDATED_OBSERVATIONS` to `framework-analyst` subagent.

---

## Validation Commands

### Check observation files exist
```bash
ls -la devforgeai/feedback/ai-analysis/STORY-XXX/
```

### Validate JSON syntax
```bash
for f in devforgeai/feedback/ai-analysis/STORY-XXX/*.json; do
  python3 -c "import json; json.load(open('$f'))" && echo "$f: valid"
done
```

### Count total observations
```bash
jq -s '[.[].observations | length] | add' devforgeai/feedback/ai-analysis/STORY-XXX/*.json
```

---

## References

- **Observation Schema:** `.claude/skills/devforgeai-development/references/observation-capture.md`
- **Phase 09 Workflow:** `.claude/skills/devforgeai-development/phases/phase-09-feedback.md`
- **Framework Analyst:** `.claude/agents/framework-analyst.md`
- **Pre-Phase Planning:** `devforgeai/config/pre-phase-planning.yaml`
