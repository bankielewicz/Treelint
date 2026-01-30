---
description: Create epic with feature breakdown
argument-hint: [epic-name]
model: opus
allowed-tools: AskUserQuestion, Skill
---

# Create Epic Command

Creates a new epic with feature breakdown by delegating to the devforgeai-orchestration skill.

---

## Quick Reference

```bash
# Create epic with interactive workflow
/create-epic User Authentication System

# Create epic for e-commerce platform
/create-epic Payment Processing Overhaul

# Create epic for analytics
/create-epic Real-time Analytics Dashboard
```

---

## Command Workflow

### Phase 0: Argument Validation

**Validate epic name provided:**

```bash
epic_name="$1"

if [ -z "$epic_name" ]; then
  echo "❌ Error: Epic name required"
  echo ""
  echo "Usage: /create-epic [epic-name]"
  echo ""
  echo "Examples:"
  echo "  /create-epic User Authentication System"
  echo "  /create-epic Payment Processing Overhaul"
  echo "  /create-epic Real-time Analytics Dashboard"
  echo ""
  exit 1
fi
```

**Validate epic name format:**

```
Epic name validation:
- Minimum length: 10 characters
- Maximum length: 100 characters
- Allowed characters: Alphanumeric, spaces, hyphens, underscores

if [ ${#epic_name} -lt 10 ] || [ ${#epic_name} -gt 100 ]; then
  echo "❌ Invalid epic name length"
  echo ""
  echo "Epic names must be 10-100 characters"
  echo ""
  echo "Current: '$epic_name' (${#epic_name} characters)"
  echo ""
  exit 1
fi
```

**Validation summary:**

```
✓ Epic name: $epic_name
✓ Length: Valid (${#epic_name} characters)
✓ Proceeding with epic creation...
```

---

### Phase 1: Set Context Markers

**Provide explicit context for orchestration skill:**

The skill operates in multiple modes (epic creation, sprint planning, story management). We set explicit markers so the skill knows to execute epic creation workflow.

```
**Epic name:** $epic_name
**Command:** create-epic
**Mode:** interactive
```

These markers trigger the skill's epic creation mode (Phase 4A in orchestration skill).

---

### Phase 2: Invoke Orchestration Skill

**Invoke skill and execute its expanded instructions:**

```
Skill(command="devforgeai-orchestration")
```

**After skill invocation:**
- Skill's SKILL.md content expands inline in conversation
- **YOU execute the skill's workflow phases** (not waiting for external result)
- Follow the skill's instructions phase by phase
- Produce output as skill instructs

**The skill instructs you to execute 8-phase epic creation workflow:**

1. **Epic Discovery** - Generate EPIC-ID (EPIC-001, EPIC-002, etc.), check for duplicate names via Grep, handle duplicates via AskUserQuestion
2. **Context Gathering** - Collect epic goal, timeline, priority, business value, stakeholders, success criteria (4 interactive AskUserQuestion flows)
3. **Feature Decomposition** - Invoke requirements-analyst subagent to generate 3-8 features, interactive review loop (accept/remove/add/modify)
4. **Technical Assessment** - Invoke architect-reviewer subagent for complexity scoring (0-10), risk identification, validate against context files (if exist)
5. **Epic File Creation** - Load epic-template.md, populate with gathered data, write to devforgeai/specs/Epics/{EPIC-ID}.epic.md
6. **Requirements Specification** - Optional: Ask if user wants detailed requirements spec, invoke requirements-analyst if yes
7. **Validation & Self-Healing** - Execute 9 validation checks, self-heal correctable issues (missing IDs, dates, defaults), HALT on critical failures
8. **Completion Summary** - Return structured JSON summary for display

**Reference files loaded progressively by skill:**
- epic-management.md (496 lines - Phases 1-2)
- feature-decomposition-patterns.md (850 lines - Phase 3)
- technical-assessment-guide.md (900 lines - Phase 4)
- epic-template.md (265 lines - Phase 5)
- epic-validation-checklist.md (800 lines - Phase 7)

**Subagents invoked by skill:**
- requirements-analyst (feature decomposition, optional requirements spec)
- architect-reviewer (technical assessment)

**Framework validation (if context files exist):**
- Validates technologies against tech-stack.md
- Validates architecture against architecture-constraints.md
- Validates integrations against dependencies.md
- Validates patterns against anti-patterns.md
- HALTS on violations

---

### Phase 3: Display Results

**Display skill output:**

The skill returns a structured summary. Display it directly without modification:

```
✅ Epic Created Successfully

Epic Details:
  📋 ID: {epic_id}
  🎯 Title: {epic_name}
  🏆 Priority: {priority}
  📊 Business Value: {business_value}
  📅 Timeline: {timeline}

Features: {feature_count} features identified
  {for each feature:
    ✨ {feature.name} - {feature.complexity}
  }

Technical Assessment:
  🔧 Complexity Score: {complexity_score}/10
  ⚠️ Key Risks: {risk_count} identified
  📦 Prerequisites: {prerequisite_count}
  {if technology_conflicts:
    ⚠️ ADR Required: {adr_topics}
  }

Files Created:
  📁 {epic_file_path}
  {if requirements_created:
    📁 {requirements_file_path}
  }

{validation_note}
```

**If validation warnings:**

```
⚠️ Validation Warnings:
  - {warning_1} (self-healed)
  - {warning_2} (self-healed)

Epic created successfully but review warnings before implementation.
```

---

### Phase 3.5: Context Preservation Validation (STORY-299)

**Invoke context-preservation-validator for provenance chain validation:**

```
Task(
  subagent_type="context-preservation-validator",
  description="Validate context preservation for epic",
  prompt="Validate epic-to-brainstorm linkage for ${epic_file_path}. Check source_brainstorm field and verify brainstorm file exists."
)
```

**Behavior:** Non-blocking by default. Displays warning if brainstorm linkage missing or broken.

---

### Phase 4: Next Steps Guidance

**Provide actionable next steps:**

The skill provides context-aware next steps based on epic creation results.

```
Next Steps:
  1. Review epic document: {epic_file_path}
  2. {if greenfield_mode:
       ⚠️ Create architectural context: /create-context {project-name}
     }
  3. {if adr_required:
       ⚠️ Create ADRs for technology decisions: {adr_topics}
     }
  4. Create sprint: /create-sprint {sprint-number}
  5. Break features into stories during sprint planning
  6. Implement stories: /dev {STORY-ID}
```

**Additional guidance based on epic characteristics:**

```
{if greenfield_mode:
  📝 Greenfield Project Detected:
  - No context files found (devforgeai/specs/context/*.md)
  - Create architectural context before implementation
  - Run: /create-context {project-name}
  - This establishes tech stack, coding standards, and architecture constraints
}

{if complexity_score > 7:
  ⚠️ High Complexity Epic ({complexity_score}/10):
  - This epic may be large enough to split into multiple epics
  - Consider breaking into smaller initiatives
  - Review during sprint planning
}

{if feature_count > 8:
  ⚠️ Over-Scoped Epic ({feature_count} features):
  - Recommended: 3-8 features per epic
  - Consider splitting into multiple epics
  - Or defer some features to future epic
}
```

---

## Error Handling

### Error: Invalid Epic Name

**Condition:** Epic name is empty, too short (<10 chars), too long (>100 chars), or contains invalid characters

**Action:** Validation occurs in Phase 0 with clear error message and examples

**No fallback logic:** Command HALTs with validation error, user must provide valid name

---

### Error: Skill Invocation Failed

**Condition:** devforgeai-orchestration skill returns error or throws exception

**Action:**

```
❌ Epic creation failed

The orchestration skill encountered an issue:
  {skill_error_message}

Suggested actions:
  {skill_recovery_steps}

Do NOT attempt manual epic creation - the skill will handle error recovery.

If error persists, check:
  1. Skill exists: .claude/skills/devforgeai-orchestration/SKILL.md
  2. Reference files exist: .claude/skills/devforgeai-orchestration/references/
  3. Context markers set correctly
```

**No fallback to manual workflow:** Let skill handle all error recovery and edge cases

---

### Error: Epic Validation Failed

**Condition:** Skill detects critical failures during Phase 7 validation (circular dependencies, framework violations, missing required data)

**Action:**

```
❌ Epic Validation Failed

The skill detected critical issues that prevent epic creation:
  {validation_failures}

Self-healing attempted:
  {self_healed_issues}

Epic NOT created. Resolve critical issues:
  {failure_remediation_steps}

Retry: /create-epic {epic_name}
```

**Skill handles:** All validation logic, self-healing attempts, failure reporting

**Command handles:** Display error message only

---

## Success Criteria

- [x] Epic name validated (format, length)
- [x] Context markers set correctly
- [x] Skill invoked successfully
- [x] Results displayed from skill output
- [x] Next steps guidance provided
- [x] Character budget < 8,000 (target: 53% of 15K limit)
- [x] Token usage < 2,000 in main conversation
- [x] Zero business logic in command
- [x] Single skill invocation only
- [x] No direct subagent invocations

---

## Integration

**Invoked by:**
- User via `/create-epic [epic-name]` command

**Invokes:**
- devforgeai-orchestration skill (epic creation mode)

**Skill invokes:**
- requirements-analyst subagent (feature decomposition, optional requirements)
- architect-reviewer subagent (technical assessment)

**Prerequisites:**
- None (can create epics before context files exist)

**Enables:**
- /create-sprint command (requires epics for sprint planning)
- Epic → Sprint → Story workflow
- devforgeai-orchestration skill (epic management)

**Updates:**
- Creates: devforgeai/specs/Epics/{EPIC-ID}.epic.md
- Creates (optional): devforgeai/specs/requirements/{EPIC-ID}-requirements.md

---

## Performance

**Token Budget:**

| Component | Tokens |
|-----------|--------|
| Command overhead (validation, markers, display) | ~2,000 |
| Skill execution (isolated context) | ~125,000-146,000 |
| Total main conversation impact | ~2,000 |

**Estimated token breakdown (main conversation):**
- Phase 0: Argument validation (~200 tokens)
- Phase 1: Context markers (~100 tokens)
- Phase 2: Skill invocation (~500 tokens)
- Phase 3: Display results (~1,000 tokens)
- Phase 4: Next steps (~200 tokens)
- **Total: ~2,000 tokens**

**Compared to previous implementation:**
- Previous: ~10,000 tokens (all logic in command)
- Current: ~2,000 tokens (logic in isolated skill context)
- **Savings: 80% reduction in main conversation tokens**

**Execution Time:**
- Epic creation (full workflow): 3-5 minutes
- With requirements spec: 5-7 minutes
- Greenfield mode: 3-4 minutes (no context validation)
- Brownfield mode: 4-6 minutes (includes context validation)

**Character Budget:**
- Target: 6,000-8,000 characters (40-53% of 15K limit)
- Current estimate: ~7,500 characters (50% of limit)
- Status: ✅ COMPLIANT (well within budget)

---

## Reference Documentation

**For detailed epic creation guidance, see:**
- `.claude/memory/epic-creation-guide.md` - Epic lifecycle, best practices, troubleshooting, framework integration

**For implementation details, see:**
- `.claude/skills/devforgeai-orchestration/SKILL.md` (Phase 4A: Epic Creation Workflow)
- `devforgeai/protocols/lean-orchestration-pattern.md` - Command architecture pattern

---

**Character Budget:** 12,084 characters (81% of 15K limit) ✅ COMPLIANT
**Token Efficiency:** ~2,000 tokens in main conversation (80% reduction) ✅ EFFICIENT
**Pattern Compliance:** 5/5 responsibilities met, 0/5 violations ✅ COMPLIANT
