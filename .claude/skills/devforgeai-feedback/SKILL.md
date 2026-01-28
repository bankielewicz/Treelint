---
name: devforgeai-feedback
description: Retrospective feedback system with event-driven hooks for DevForgeAI operations. Use after operations complete (dev, qa, release) to capture insights, challenges, and improvements.
version: 1.0
status: Implemented (Core), Integration Pending (TodoWrite)
model: claude-opus-4-5-20251101
---

# DevForgeAI Feedback Skill

Capture retrospective feedback from development workflows to improve processes, identify patterns, and enable continuous improvement.

---

## Purpose

Enable automated and on-demand feedback collection across DevForgeAI operations (commands, skills, subagents) to:
1. Capture insights immediately after operations while context is fresh
2. Identify patterns in challenges and successes
3. Enable data-driven process improvements
4. Build organizational knowledge base from retrospectives

---

## When to Use This Skill

**Auto-invoked (via hooks):**
- After /dev completes (if post-dev-feedback hook enabled)
- After /qa completes (if post-qa-retrospective hook enabled)
- After /release completes (if post-release-monitoring hook enabled)
- After sprint planning (if sprint-retrospective hook enabled)
- After /dev completes (if post-dev-ai-analysis hook enabled) - **AI Analysis**
- After /qa completes (if post-qa-ai-analysis hook enabled) - **AI Analysis**

**Manual invocation:**
```
Skill(command="devforgeai-feedback")
```

**Prerequisites:**
- Hook system implemented (STORY-018) ✅
- Hook registry configured (devforgeai/config/hooks.yaml) ✅
- TodoWrite integration (EPIC-004) ⏸️ Pending

---

## Core Capabilities

### 1. Feedback Conversations

Interactive retrospective sessions with structured questions.

**Example:**
```yaml
feedback_type: conversation
feedback_config:
  mode: comprehensive
  questions:
    - "What challenges did you encounter?"
    - "Were requirements clear?"
    - "What would you do differently?"
```

### 2. Feedback Summaries

Auto-generated summary of operation results.

**Example:**
```yaml
feedback_type: summary
feedback_config:
  summary_sections: [duration, test_results, deferrals, next_steps]
```

### 3. Metrics Collection

Quantitative data collection for analysis.

**Example:**
```yaml
feedback_type: metrics
feedback_config:
  metrics: [execution_time, token_usage, test_pass_rate]
  export_to: "devforgeai/metrics/feedback-metrics.json"
```

### 4. Checklist Validation

Interactive checklist for retrospectives.

**Example:**
```yaml
feedback_type: checklist
feedback_config:
  checklist_items:
    - "Sprint capacity realistic?"
    - "Dependencies identified?"
    - "Technical debt addressed?"
```

### 5. AI Architectural Analysis (NEW)

AI-generated framework improvement recommendations. This is NOT user-facing feedback - it captures Claude's architectural analysis after workflows.

**Example:**
```yaml
feedback_type: ai_analysis
feedback_config:
  mode: architectural
  analysis_prompts:
    - what_worked_well
    - areas_for_improvement
    - recommendations
    - patterns_observed
    - anti_patterns_detected
    - constraint_analysis
  constraint_check: claude-code-terminal
  storage_path: devforgeai/feedback/ai-analysis/
```

**Purpose:** Systematize the manual post-workflow prompt:
> "You are Claude - you provide architectural advice and guidance regarding improvements to DevForgeAI Spec-Driven Development Framework..."

**Output Structure:**
```json
{
  "ai_analysis": {
    "what_worked_well": ["..."],
    "areas_for_improvement": ["..."],
    "recommendations": [
      {
        "description": "...",
        "affected_files": ["..."],
        "implementation_notes": "...",
        "priority": "medium",
        "feasible_in_claude_code": true
      }
    ],
    "patterns_observed": ["..."],
    "anti_patterns_detected": ["..."],
    "constraint_analysis": "..."
  }
}
```

---

## Workflow (Auto-Triggered via Hooks)

### Phase 1: Hook Trigger Detection

```
# When operation completes:
TodoWrite(todos=[{status: "completed", ...}])
  ↓
# Hook system detects completion
hook_system.operation_complete(context)
  ↓
# Matches hooks against operation pattern + conditions
matching_hooks = find_matching_hooks(context)
  ↓
# Invoke devforgeai-feedback skill for each hook
FOR hook in matching_hooks:
  Skill(command="devforgeai-feedback")
```

---

### Phase 2: Feedback Session Execution

**Based on feedback_type:**

**Type 1: Conversation**
```
1. Extract questions from hook.feedback_config.questions
2. Present questions to user via AskUserQuestion
3. Capture responses
4. Persist to devforgeai/feedback/{operation_id}-feedback.md
5. Return summary
```

**Type 2: Summary**
```
1. Extract summary_sections from hook.feedback_config
2. Generate summary from operation context
3. Format as markdown
4. Persist to devforgeai/feedback/{operation_id}-summary.md
5. Return summary
```

**Type 3: Metrics**
```
1. Extract metrics list from hook.feedback_config
2. Collect metric values from operation context
3. Export to JSON
4. Append to devforgeai/metrics/{metric_type}.json
5. Return metrics summary
```

**Type 4: Checklist**
```
1. Extract checklist_items from hook.feedback_config
2. Present as interactive checklist via AskUserQuestion
3. Capture checked items
4. Persist to devforgeai/feedback/{operation_id}-checklist.md
5. Return completion percentage
```

**Type 5: AI Analysis** (NEW)
```
1. Read workflow context (story file, phases completed, errors, deferrals)
2. Load analysis prompts from ai-analysis-questions.yaml
3. For each analysis_prompt:
   a. Generate AI response (what_worked_well, areas_for_improvement, etc.)
   b. Validate recommendations are feasible in Claude Code Terminal
   c. Structure output per schema
4. Persist to devforgeai/feedback/ai-analysis/{story_id}/{timestamp}-ai-analysis.json
5. Update ai-analysis/index.json
6. Update aggregated/recommendations-queue.json (if high priority)
7. Return structured analysis summary
```

**AI Analysis Constraint Check:**
```
# Before accepting a recommendation, verify feasibility:
IF recommendation.requires NOT IN [Read, Write, Edit, Glob, Grep, Bash, Task, AskUserQuestion]:
  WARN: "Recommendation may not be implementable in Claude Code Terminal"
  SET recommendation.feasible_in_claude_code = false
```

---

### Phase 3: Feedback Persistence

**Save feedback to filesystem:**

```
# Feedback file structure
devforgeai/feedback/
├── {operation_id}-feedback.md      # Conversation responses
├── {operation_id}-summary.md       # Auto-generated summaries
├── {operation_id}-checklist.md     # Checklist results
└── index.json                       # Feedback index for search
```

**Feedback file format:**
```markdown
---
operation_id: cmd-dev-001
operation_type: command
operation_name: dev
story_id: STORY-042
timestamp: 2025-11-11T14:30:00Z
feedback_type: conversation
hook_id: post-dev-feedback
---

# Feedback: /dev STORY-042

## Questions and Responses

### Q1: What challenges did you encounter during TDD?

**Response:**
{user_response_1}

### Q2: Were acceptance criteria clear and testable?

**Response:**
{user_response_2}

### Q3: Did you defer any DoD items? Why?

**Response:**
{user_response_3}

---

## Context

- Duration: 6 minutes 23 seconds
- Tests: 45/45 passing
- Coverage: 97%
- Deferrals: 2 items (config, docs)
```

---

### Phase 4: Index Update

**Update feedback index for searchability:**

```
# Read existing index
Read(file_path="devforgeai/feedback/index.json")

# Parse JSON
index = json.loads(content)

# Add new entry
index["feedback_sessions"].append({
  "id": "{operation_id}",
  "timestamp": "{iso_timestamp}",
  "operation": "{operation_name}",
  "story": "{story_id}",
  "feedback_file": "{operation_id}-feedback.md",
  "tags": hook['tags']
})

# Write updated index
Write(file_path="devforgeai/feedback/index.json", content=json.dumps(index, indent=2))
```

---

## Hook Integration

This skill works with the event-driven hook system (STORY-018):

```yaml
# Hook triggers this skill automatically
hooks:
  - id: post-dev-feedback
    feedback_type: conversation
    # ... hook config ...
    # When triggered → invokes devforgeai-feedback skill
```

**Integration status:**
- ✅ Hook system implemented (src/hook_*.py)
- ✅ Hook registry schema defined
- ✅ Default hooks.yaml template created
- ⏸️ TodoWrite integration pending (EPIC-004)
- ⏸️ Feedback persistence pending (EPIC-004)

---

## Manual Usage

**Invoke skill directly for on-demand feedback:**

```
# Set context
**Operation:** dev
**Story:** STORY-042
**Feedback Mode:** comprehensive

Skill(command="devforgeai-feedback")
```

**The skill will:**
1. Detect operation context from conversation
2. Load default feedback questions for that operation type
3. Present feedback session
4. Persist responses
5. Return summary

---

## Reference Files

**When fully implemented, this skill will load:**
- `references/feedback-question-templates.md` - Default questions by operation type
- `references/feedback-persistence-guide.md` - Storage patterns and indexing
- `references/feedback-analysis-patterns.md` - Analyzing feedback trends
- `references/feedback-export-formats.md` - Export to JSON, CSV, markdown
    Read(file_path=".claude/skills/devforgeai-feedback/references/feedback-question-templates.md")
    Read(file_path=".claude/skills/devforgeai-feedback/references/feedback-persistence-guide.md")
    Read(file_path=".claude/skills/devforgeai-feedback/references/feedback-analysis-patterns.md")
    Read(file_path=".claude/skills/devforgeai-feedback/references/feedback-export-formats.md")

**Current status:** Reference files pending (EPIC-004)

---

## Success Criteria

This skill succeeds when:
- [ ] Feedback session executed (conversation/summary/metrics/checklist)
- [ ] User responses captured (if interactive)
- [ ] Feedback persisted to filesystem
- [ ] Index updated with new feedback entry
- [ ] Summary returned to caller
- [ ] Token usage <30K (isolated context)

---

## Integration Points

**From:** Hook system (auto-trigger on operation completion)
**To:** Feedback storage (persistence layer)
**Invokes:** AskUserQuestion (for interactive sessions)
**Updates:** devforgeai/feedback/ (feedback files), devforgeai/metrics/ (metrics)

---

## Current Status

**STORY-018 Deliverables:** ✅ Complete
- Hook system architecture documented (HOOK-SYSTEM.md)
- Hook registry schema defined
- Default hooks.yaml template created
- /audit-hooks command implemented

**Future Work (EPIC-004):**
- Complete feedback skill implementation (Phase 1-4 workflows)
- Integrate with TodoWrite tool (auto-trigger)
- Implement feedback persistence layer
- Create feedback analysis tools
- Build feedback export/import

---

## Related Documentation

- **Hook System:** `.claude/skills/devforgeai-feedback/HOOK-SYSTEM.md`
- **Hook Configuration:** `devforgeai/config/hooks.yaml`
- **Hook Implementation:** `src/hook_*.py` (6 modules, 1,300 LOC)
- **Hook Tests:** `tests/test_hook_*.py` (175 tests)
- **Audit Command:** `.claude/commands/audit-hooks.md`
- **Story:** `devforgeai/specs/Stories/STORY-018-event-driven-hook-system.story.md`
