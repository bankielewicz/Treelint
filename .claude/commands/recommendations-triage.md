---
description: Triage AI-generated framework improvement recommendations into stories
argument-hint: [--priority=HIGH|MEDIUM|LOW] [--limit=N]
model: opus
allowed-tools: Read, Glob, Write, Skill, AskUserQuestion, TodoWrite
---

# /recommendations-triage - Convert Framework Recommendations to Stories

Process accumulated AI-generated framework improvement recommendations and selectively convert to user stories.

---

## Quick Reference

```bash
# Show all pending recommendations
/recommendations-triage

# Filter by priority
/recommendations-triage --priority=HIGH

# Limit results
/recommendations-triage --limit=5

# Combined
/recommendations-triage --priority=HIGH --limit=10
```

---

## Phase 0: Parse Arguments

**Extract filters (optional):**
```
PRIORITY_FILTER = argument "--priority=" value (HIGH|MEDIUM|LOW)
LIMIT = argument "--limit=" value (integer, default: 20)
```

**Validate:**
- Priority: Must be HIGH, MEDIUM, or LOW (case-insensitive)
- Limit: Must be positive integer

---

## Phase 1: Read Recommendations Queue

**Load recommendations:**
```
Read(file_path="devforgeai/feedback/ai-analysis/aggregated/recommendations-queue.json")
```

**Extract recommendations by priority bucket:**
```json
{
  "high": [...],
  "medium": [...],
  "low": [...]
}
```

**Apply filters:**
- If PRIORITY_FILTER set: Only include that priority bucket
- Apply LIMIT to total results (across all priorities)

**If no recommendations found:**
```
No pending framework recommendations found.

Recommendations are generated during /dev workflow (Phase 09).
Run `/dev` on a story to capture framework improvement observations.

To manually trigger AI analysis: /DF:feedback --type=ai_analysis
```

---

## Phase 2: Display Recommendations

**Present grouped by priority (HIGH first):**

```markdown
## Framework Improvement Recommendations

**Total pending:** {count} | **Showing:** {displayed_count}

---

### HIGH Priority ({high_count} items)

| # | Title | Effort | Affected Files |
|---|-------|--------|----------------|
| 1 | {title} | {effort_estimate} | {affected_files[0]} |
| 2 | ... | ... | ... |

**Details:**

**[1] {title}**
- Description: {description}
- Files: {affected_files}
- Implementation: {implementation_code}
- Source: {story_id} (Phase {phase})

---

### MEDIUM Priority ({medium_count} items)

| # | Title | Effort | Affected Files |
|---|-------|--------|----------------|
| 3 | ... | ... | ... |

---

### LOW Priority ({low_count} items)

| # | Title | Effort | Affected Files |
|---|-------|--------|----------------|
| 5 | ... | ... | ... |
```

---

## Phase 3: User Selection

**Present multi-select for story conversion:**

```
AskUserQuestion(
  questions=[{
    question: "Which recommendations should be converted to stories?",
    header: "Select items",
    options: [
      {
        label: "[1] {title_truncated}",
        description: "Priority: HIGH | Effort: {effort} | Files: {file_count}"
      },
      {
        label: "[2] {title_truncated}",
        description: "Priority: MEDIUM | Effort: {effort} | Files: {file_count}"
      },
      ...
    ],
    multiSelect: true
  }]
)
```

**Also offer bulk actions:**
```
AskUserQuestion(
  questions=[{
    question: "Or select a bulk action:",
    header: "Bulk action",
    options: [
      {
        label: "All HIGH priority",
        description: "Convert all {high_count} HIGH priority items to stories"
      },
      {
        label: "Skip - just viewing",
        description: "Exit without creating stories"
      }
    ],
    multiSelect: false
  }]
)
```

**If "Skip" selected:**
```
No stories created.

To revisit later: /recommendations-triage
To search recommendations: /feedback-search --type=ai-analysis
```
→ Exit

---

## Phase 4: Story Creation

**For each selected recommendation:**

### 4.1 Prepare Story Context

**Set context markers for devforgeai-story-creation skill:**

```markdown
**Feature Description:** Framework Enhancement: {recommendation.title}

{recommendation.description}

**Implementation Approach:**
{recommendation.implementation_code}

**Affected Files:**
{recommendation.affected_files.join(", ")}

**Effort Estimate:** {recommendation.effort_estimate}

**Source:** AI Analysis from {recommendation.source_story_id}

**Feature Type:** Other
**Source:** framework-enhancement
**Priority:** {recommendation.priority}
```

### 4.2 Invoke Story Creation

```
Skill(command="devforgeai-story-creation")
```

**Let skill handle:**
- Story ID generation (gap-aware)
- Full 8-phase workflow
- Epic/sprint association (prompt user)
- File creation

### 4.3 Track Created Stories

**After each story created, record:**
```json
{
  "recommendation_id": "{recommendation.id}",
  "story_id": "{created_story_id}",
  "converted_at": "{timestamp}",
  "converted_by": "recommendations-triage"
}
```

---

## Phase 5: Update Recommendations Queue

**Mark selected items as converted:**

```
Read(file_path="devforgeai/feedback/ai-analysis/aggregated/recommendations-queue.json")
```

**For each converted recommendation:**
1. Remove from priority bucket (high/medium/low)
2. Add to `implemented` array with conversion metadata:
```json
{
  "original_recommendation": { ... },
  "converted_to_story": "STORY-XXX",
  "converted_at": "2025-12-29T10:00:00Z",
  "status": "converted"
}
```

**Write updated queue:**
```
Write(file_path="devforgeai/feedback/ai-analysis/aggregated/recommendations-queue.json")
```

---

## Phase 6: Completion Summary

**Present summary:**

```markdown
## Triage Complete

**Stories Created:** {created_count}
**Recommendations Remaining:** {remaining_count}

### Created Stories

| Story ID | Title | Priority | Effort |
|----------|-------|----------|--------|
| STORY-XXX | {title} | HIGH | 30 min |
| STORY-YYY | {title} | MEDIUM | 1 hour |

### Queue Status

- HIGH priority remaining: {high_remaining}
- MEDIUM priority remaining: {medium_remaining}
- LOW priority remaining: {low_remaining}

### Next Steps

- `/dev STORY-XXX` - Start development on first created story
- `/recommendations-triage` - Process more recommendations
- `/feedback-search --type=ai-analysis` - View all AI analyses
```

---

## Error Handling

### Queue File Not Found
```
Error: recommendations-queue.json not found

Expected location: devforgeai/feedback/ai-analysis/aggregated/recommendations-queue.json

This file is created during /dev workflow (Phase 09) when AI analysis is enabled.

Resolution:
1. Run /dev on a story to generate recommendations
2. Or create empty queue: /recommendations-triage --init
```

### Story Creation Failed
```
Warning: Failed to create story for recommendation "{title}"

Error: {error_message}

The recommendation remains in queue and can be retried.
Continuing with remaining selections...
```

### Queue Write Failed
```
Error: Cannot update recommendations-queue.json

Cause: {permission_error or disk_space}

Stories were created but queue not updated.
Manual fix: Edit recommendations-queue.json to move items to "implemented" array
```

---

## Success Criteria

- [ ] Queue file read successfully
- [ ] Recommendations displayed by priority
- [ ] User selection captured (multi-select)
- [ ] Stories created with framework-enhancement tag
- [ ] Queue updated (converted items moved)
- [ ] Summary displayed with next steps
- [ ] Exit code 0 (success) or 1 (error)

---

## Integration

**Workflow:** /dev (Phase 09) captures → **recommendations-triage** → /create-story → /dev

**Data flow:**
1. /dev workflow Phase 09 invokes framework-analyst subagent
2. Subagent generates recommendations → stored in queue
3. /recommendations-triage reads queue → user selects
4. Selected items → devforgeai-story-creation skill
5. Stories created with `source: framework-enhancement` tag
6. Queue updated (items moved to implemented)

**Related commands:**
- `/DF:feedback` - Manual feedback trigger
- `/feedback-search` - Search all feedback types
- `/dev` - Development workflow (generates recommendations)
- `/create-story` - Story creation (invoked by this command)

---

## Performance

**Target Response Time:** <500ms for queue read and display
**Story Creation:** 2-5 min per story (delegated to skill)

**Token Budget:** ~5K tokens (command) + ~90K tokens/story (skill)

---

## References

- `.claude/agents/framework-analyst.md` - Subagent that generates recommendations
- `.claude/skills/devforgeai-development/phases/phase-09-feedback.md` - Where recommendations are captured
- `.claude/skills/devforgeai-development/references/observation-capture.md` - Observation capture protocol
- `devforgeai/feedback/ai-analysis/aggregated/recommendations-queue.json` - Queue file
- `devforgeai/feedback/schema.json` - AI analysis schema

---

**Command follows lean orchestration pattern: Queue read → User selection → Skill invocation → Queue update → Summary**
