# Sprint Planner Subagent

Specialized subagent for DevForgeAI sprint planning operations. Handles story selection validation, capacity planning, sprint document creation, and story status updates in isolated context.

---

## Quick Start

### Invoke Sprint Planner

```
Task(
  subagent_type="sprint-planner",
  description="Create Sprint-N with selected stories",
  prompt="Create sprint with:
    - Sprint name: {sprint_name}
    - Selected stories: {story_ids}
    - Duration: {duration_days}
    - Epic: {epic_id}
    - Start date: {start_date}

    Execute complete sprint planning workflow and return summary."
)
```

### Expected Response

```json
{
  "success": true,
  "sprint_id": "SPRINT-1",
  "sprint_name": "User Authentication",
  "file_path": "devforgeai/specs/Sprints/Sprint-1.md",
  "capacity": {
    "total_points": 16,
    "total_stories": 3,
    "status": "optimal"
  },
  "stories_added": [...],
  "stories_updated_count": 3,
  "next_steps": [...]
}
```

---

## How It Works

1. **Discovers** next sprint number by reading existing sprint files
2. **Validates** all selected stories are in Backlog status
3. **Calculates** sprint capacity and metrics
4. **Generates** sprint markdown document with proper structure
5. **Updates** story statuses (Backlog → Ready for Dev)
6. **Returns** structured JSON summary

---

## Integration Points

**Command-level:** `/create-sprint` command delegates to subagent
- Command handles user interaction (story selection, metadata)
- Subagent handles document generation and updates

**Skill-level:** `devforgeai-orchestration` skill invokes during sprint planning
- Skill orchestrates workflow
- Subagent executes sprint operations

**Direct use:** Can be invoked directly via Task tool

---

## What It Creates

**Sprint File:** `devforgeai/specs/Sprints/Sprint-{N}.md`
- YAML frontmatter (id, name, epic, dates, capacity, stories)
- Markdown content (overview, goals, stories by status, metrics)
- Ready for development workflow

**Story Updates:** Each selected story file updated
- Status: Backlog → Ready for Dev
- Sprint: Added/updated to SPRINT-{N}
- Workflow History: Entry added with timestamp

---

## Specifications

| Aspect | Value |
|--------|-------|
| **Model** | Sonnet |
| **Tools** | Read, Write, Edit, Glob, Grep |
| **Token Target** | < 40K per invocation |
| **Capacity Planning** | 20-40 points (2-week sprint) |
| **Framework-Aware** | Yes (11-state workflow) |
| **Context Isolation** | Yes (isolated invocation) |

---

## For More Information

**Subagent definition:** `sprint-planner.md`
**Sprint planning guidelines:** `.claude/skills/devforgeai-orchestration/references/sprint-planning-guide.md`
**Implementation details:** `devforgeai/SPRINT-PLANNER-GENERATION-SUMMARY.md`
**Verification steps:** `devforgeai/SPRINT-PLANNER-VERIFICATION.md`
**Command refactoring:** `devforgeai/COMMAND-REFACTORING-GUIDE-CREATE-SPRINT.md`

---

## Success Criteria

Sprint planner has successfully:
- [x] Generated valid sprint documents
- [x] Updated story statuses correctly
- [x] Calculated capacity accurately
- [x] Maintained workflow history
- [x] Respected framework constraints
- [x] Operated within token budget
- [x] Enabled lean orchestration pattern

---

**Status:** Production Ready
**Generated:** 2025-11-05
**Framework:** DevForgeAI 1.0.1
