# Observation Capture Protocol

Capture framework improvement observations during /dev workflow execution for later synthesis into actionable recommendations.

---

## Purpose

Enable DevForgeAI self-improvement by capturing "friction notes" as Claude works through each phase. These observations become input for the `framework-analyst` subagent at Phase 09.

---

## Categories

| Category | When to Log | Examples |
|----------|-------------|----------|
| `friction` | Something slowed you down or required workaround | "Had to read 4 files to find naming convention", "Unclear which subagent to use" |
| `success` | Something worked well, saved time, or prevented error | "anti-patterns.md caught God Object before commit", "Phase state resume worked perfectly" |
| `pattern` | Noticed repeated behavior (good or bad) | "3rd time manually updating DoD - should automate", "Same validation logic in 3 phases" |
| `gap` | Missing documentation, tooling, or guidance | "No example for handling edge case X", "Missing error message for Y scenario" |
| `idea` | Improvement opportunity or enhancement | "Phase could run in parallel with previous", "Subagent could cache this result" |
| `bug` | Framework defect discovered during workflow | "CLI exits 0 even on validation failure", "Lock file not cleaned up on error" |

---

## Schema

Observations are stored in the `observations` array of `phase-state.json`:

```json
{
  "story_id": "STORY-XXX",
  "phases": { ... },
  "observations": [
    {
      "id": "obs-02-001",
      "phase": "02",
      "category": "friction",
      "note": "Unclear how to name test files for shell scripts - had to check 3 examples",
      "files": ["tests/STORY-XXX/", ".claude/agents/test-automator.md"],
      "severity": "medium"
    },
    {
      "id": "obs-03-001",
      "phase": "03",
      "category": "success",
      "note": "anti-patterns.md constraint prevented class from exceeding 500 lines",
      "files": ["devforgeai/specs/context/anti-patterns.md"],
      "severity": "high"
    }
  ]
}
```

### Field Definitions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Format: `obs-{phase}-{sequence}` (e.g., `obs-02-001`) |
| `phase` | string | Yes | Phase number where observation occurred (01-08) |
| `category` | enum | Yes | One of: friction, success, pattern, gap, idea, bug |
| `note` | string | Yes | Brief description (1-2 sentences, max 200 chars) |
| `files` | array | No | Related file paths (if applicable) |
| `severity` | enum | Yes | One of: low, medium, high |

### Severity Guidelines

| Severity | Criteria |
|----------|----------|
| `low` | Minor inconvenience, polish item |
| `medium` | Noticeable friction, should be addressed |
| `high` | Significant blocker, caused error, or prevented quality work |

---

## Capture Prompt

Add this prompt to the exit of each phase (01-08):

```markdown
### Observation Capture (Before Exit)

**Before marking this phase complete, reflect:**
1. Did I encounter any friction? (unclear docs, missing tools, workarounds needed)
2. Did anything work particularly well? (constraints that helped, patterns that fit)
3. Did I notice any repeated patterns across phases?
4. Are there gaps in tooling/docs that would help future stories?
5. Did I discover any bugs or unexpected behavior?

**If YES to any:** Append observation to phase-state.json `observations` array:
```json
{
  "id": "obs-{phase}-{seq}",
  "phase": "{current_phase}",
  "category": "{friction|success|pattern|gap|idea|bug}",
  "note": "{1-2 sentence description}",
  "files": ["{relevant files if any}"],
  "severity": "{low|medium|high}"
}
```

**If NO observations:** Continue to exit gate (no action needed).
```

---

## Examples

### Example 1: Friction Observation

**Scenario:** During Phase 02 (Test-First), Claude couldn't find the test naming convention.

```json
{
  "id": "obs-02-001",
  "phase": "02",
  "category": "friction",
  "note": "Test naming convention unclear - checked test-automator.md, tdd-patterns.md, and 2 example test files before finding pattern",
  "files": [".claude/agents/test-automator.md", ".claude/skills/devforgeai-development/references/tdd-patterns.md"],
  "severity": "medium"
}
```

### Example 2: Success Observation

**Scenario:** During Phase 03 (Implementation), a context file constraint prevented a mistake.

```json
{
  "id": "obs-03-001",
  "phase": "03",
  "category": "success",
  "note": "architecture-constraints.md Single Responsibility Principle reminded me to split service class before it grew too large",
  "files": ["devforgeai/specs/context/architecture-constraints.md"],
  "severity": "high"
}
```

### Example 3: Pattern Observation

**Scenario:** During Phase 07 (DoD Update), Claude notices repeated manual work.

```json
{
  "id": "obs-07-001",
  "phase": "07",
  "category": "pattern",
  "note": "This is the 3rd story where I manually updated Implementation Notes section - could be automated from phase-state.json",
  "files": [".claude/skills/devforgeai-development/references/dod-update-workflow.md"],
  "severity": "medium"
}
```

---

## Integration with Phase 09

At Phase 09, the `framework-analyst` subagent:

1. Reads `phase-state.json` including `observations` array
2. Expands terse notes into full recommendations
3. Validates each recommendation (file paths, effort, feasibility)
4. Applies merit filter (no duplicates, not already implemented)
5. Stores meritorious items to `devforgeai/feedback/ai-analysis/`

**Data Flow:**
```
Phases 01-08: Capture observations → phase-state.json
Phase 09: framework-analyst reads observations → synthesizes → stores recommendations
Later: /recommendations-triage → user selects → stories created
```

---

## Constraints

1. **Keep notes brief** - 1-2 sentences, max 200 characters
2. **Include files when relevant** - Helps subagent find context
3. **Don't force observations** - Only log when something notable occurred
4. **Severity matters** - High severity items get priority in synthesis
5. **Phase must be accurate** - Enables correlation with phase-specific issues
