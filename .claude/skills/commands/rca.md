---
description: Perform Root Cause Analysis with 5 Whys methodology
argument-hint: [issue-description] [severity]
model: opus
allowed-tools: Read, Skill, AskUserQuestion, Glob, Grep
---

# /rca - Root Cause Analysis Command

Perform systematic Root Cause Analysis (RCA) for DevForgeAI framework breakdowns using 5 Whys methodology. Auto-collects evidence, generates recommendations, and creates RCA documents.

---

## Quick Reference

```bash
# Perform RCA for framework breakdown
/rca "devforgeai-development didn't validate context files"

# Specify severity explicitly
/rca "QA skill created autonomous deferrals" CRITICAL

# Simple issue description
/rca "orchestration skipped checkpoint detection"

# Multi-word description (quotes optional)
/rca Command had business logic in Phase 2
```

---

## Command Workflow

### Phase 0: Argument Validation

**Validate issue description (required):**
```
IF $1 is empty OR $1 == "":
  AskUserQuestion:
    Question: "What framework breakdown should I analyze?"
    Header: "Issue Description"
    Options:
      - "Skill didn't follow intended workflow"
      - "Command violated lean orchestration pattern"
      - "Quality gate was bypassed"
      - "Context file constraint ignored"
    multiSelect: false

  Extract ISSUE_DESCRIPTION from user response

  IF user cancelled:
    Display: "RCA cancelled. Use: /rca \"[issue description]\" [severity]"
    EXIT
ELSE:
  ISSUE_DESCRIPTION = $1
```

**Parse severity (optional):**
```
IF $2 provided:
  SEVERITY = $2

  # Validate severity value
  VALID_SEVERITIES = [CRITICAL, HIGH, MEDIUM, LOW, critical, high, medium, low]

  IF SEVERITY not in VALID_SEVERITIES:
    AskUserQuestion:
      Question: "Invalid severity '$2'. What severity level?"
      Header: "Severity"
      Options:
        - "CRITICAL - Framework broken, blocking work"
        - "HIGH - Significant workflow impact"
        - "MEDIUM - Quality improvement opportunity"
        - "LOW - Minor issue or enhancement"
      multiSelect: false

    Extract SEVERITY from response
  ELSE:
    # Normalize to uppercase
    SEVERITY = uppercase($2)
ELSE:
  SEVERITY = "infer"
```

**Validation summary:**
```
Display:
"✓ Issue: ${ISSUE_DESCRIPTION}
✓ Severity: ${SEVERITY}
✓ Proceeding with RCA analysis..."
```

---

### Phase 1: Set Context Markers

**Set explicit context for skill extraction:**

```
**Issue Description:** ${ISSUE_DESCRIPTION}
**Severity:** ${SEVERITY}
**Command:** rca
```

These markers allow the devforgeai-rca skill to extract parameters from conversation context (skills cannot accept command-line parameters).

---

### Phase 2: Invoke devforgeai-rca Skill

**Invoke skill:**

```
Skill(command="devforgeai-rca")
```

**What the skill does:**

The devforgeai-rca skill executes 8 comprehensive phases:

1. **Phase 0:** Issue Clarification - Extract details, generate RCA number/title
2. **Phase 1:** Auto-Read Files - Read skills, commands, subagents, context files
3. **Phase 2:** 5 Whys Analysis - Progressive questioning to root cause
4. **Phase 3:** Evidence Collection - Organize excerpts, validate context files
5. **Phase 4:** Recommendation Generation - Prioritized fixes (CRITICAL → LOW)
6. **Phase 5:** RCA Document Creation - Write to devforgeai/RCA/RCA-XXX-slug.md
7. **Phase 6:** Validation & Self-Check - Verify completeness, self-heal issues
8. **Phase 7:** Completion Report - Return summary to this command

**Skill operates in isolated context:**
- Progressive reference loading (~4,000 lines of guides)
- Auto-reads relevant files (skills, commands, subagents, context)
- Generates complete RCA document
- Returns structured completion report

---

### Phase 3: Display Results

**Receive completion report from skill:**

The skill returns a formatted completion report with:
- RCA number and title
- Severity level
- Root cause summary
- Recommendation counts by priority
- File path of created RCA document
- Next steps

**Display report to user:**

```
Output: {completion_report from skill}
```

The skill formats the report, this command simply displays it (lean orchestration - no parsing, no formatting in command).

---

## Integration with DevForgeAI Framework

### When to Use

**Use /rca when:**
- Framework process didn't work as expected
- Skill/command violated intended workflow
- Quality gate was bypassed unexpectedly
- Context file constraints were ignored
- Workflow state transition was invalid
- Autonomous operations occurred (without user approval)
- User is confused about framework behavior

**Examples:**
```bash
/rca "devforgeai-development didn't validate context files before TDD"
/rca "devforgeai-qa accepted pre-existing deferrals without challenge"
/rca "/dev command contains business logic in Phase 2"
/rca "Story transitioned to Released without QA Approved"
```

### Output

**Primary output:**
- RCA document in `devforgeai/RCA/RCA-{NNN}-{slug}.md`

**RCA document contains:**
- Issue description and metadata
- 5 Whys analysis with evidence
- Files examined (comprehensive excerpts)
- Recommendations by priority (CRITICAL/HIGH/MEDIUM/LOW)
- Exact implementation code/text (copy-paste ready)
- Testing procedures for each recommendation
- Implementation checklist
- Prevention strategy (short-term and long-term)
- Related RCAs

**Completion report displays:**
- RCA number and title
- Root cause (brief summary)
- Recommendation counts
- File path to full RCA document
- Next steps

### Framework-Aware Analysis

**The skill understands:**
- 6 immutable context files (tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns)
- 4 quality gates (Context Validation, Test Passing, QA Approval, Release Readiness)
- 11 workflow states (Backlog → Released)
- Lean orchestration pattern (command/skill/subagent responsibilities)
- DevForgeAI architectural principles (spec-driven, evidence-based, progressive disclosure)

### Evidence-Based Recommendations

**All recommendations include:**
- Exact file paths (`.claude/skills/{skill}/SKILL.md`)
- Specific sections (Phase X, Step Y, Lines Z-W)
- Copy-paste ready implementation (exact code/text to add or modify)
- Evidence-based rationale (references files examined)
- Testing procedures (how to verify fix works)
- Effort estimates (time and complexity)
- Impact analysis (benefit, risk, scope)

**No aspirational content:**
- ❌ "We should probably add validation" (vague)
- ✅ "Add context file validation to Phase 0, Step 8" (specific)
- ❌ "Improve error handling" (aspirational)
- ✅ "Add error message with file list and /create-context suggestion" (actionable)

---

## Success Criteria

RCA command successful when:
- [ ] Issue description captured (from argument or AskUserQuestion)
- [ ] Severity determined (explicit or inferred)
- [ ] devforgeai-rca skill invoked successfully
- [ ] Skill completion report received
- [ ] Report displayed to user
- [ ] RCA document created in devforgeai/RCA/
- [ ] Token usage <3K in main conversation (skill work isolated)
- [ ] Character budget <12K (lean orchestration compliance)

---

## Error Handling

### Missing Argument

**Error:** User runs `/rca` with no arguments

**Recovery:**
```
AskUserQuestion for issue description
Extract from response
Proceed with RCA
```

### Invalid Severity

**Error:** User provides invalid severity value

**Recovery:**
```
AskUserQuestion with CRITICAL/HIGH/MEDIUM/LOW options
Extract valid severity
Proceed with RCA
```

### Skill Execution Failure

**Error:** devforgeai-rca skill fails during execution

**Recovery:**
```
Display skill error message
Provide guidance:
  - Read skill output for details
  - Verify issue description was clear
  - Check if affected component exists
  - Retry with more specific description
```

### RCA Document Already Exists

**Error:** RCA file with same name already exists

**Recovery:**
```
Skill auto-increments RCA number
Retries write with new number
No user action needed
```

---

## Performance

**Token Budget:**
- Command overhead: ~3K tokens
- Skill execution: ~50-80K tokens (isolated context)
- Total main conversation: ~3K (skill work isolated)

**Execution Time:**
- Simple RCA: 3-5 minutes
- Complex RCA: 5-10 minutes
- Depends on: Files to read, complexity of issue, recommendation count

**Character Budget:**
- Current: ~9,500 characters
- Target: <12K (within budget)
- Hard limit: <15K (compliant)

---

## Related Commands

**Framework Analysis:**
- `/audit-deferrals` - Audit deferred work in stories
- `/audit-budget` - Audit command character budgets

**Framework Development:**
- `/create-story` - Create implementation stories for RCA recommendations
- `/dev` - Implement RCA recommendations via TDD

**Framework Documentation:**
- Commands reference: `.claude/memory/commands-reference.md`
- Skills reference: `.claude/memory/skills-reference.md`
- Framework overview: `CLAUDE.md`

---

## Integration Pattern

**Typical RCA workflow:**

```
1. User encounters framework breakdown
   ↓
2. User runs: /rca "[issue description]" [severity]
   ↓
3. Command validates arguments
   ↓
4. Command sets context markers
   ↓
5. Command invokes: Skill(command="devforgeai-rca")
   ↓
6. Skill performs 8-phase RCA workflow (isolated context)
   ↓
7. Skill generates RCA document in devforgeai/RCA/
   ↓
8. Skill returns completion report
   ↓
9. Command displays report to user
   ↓
10. User reads full RCA document
    ↓
11. User implements CRITICAL recommendations
    ↓
12. User creates story for substantial work (>2 hours)
    ↓
13. User commits RCA and fixes to git
```

---

## Examples

### Example 1: Skill Breakdown

```bash
$ /rca "devforgeai-development didn't validate context files before TDD" CRITICAL

✓ Issue: devforgeai-development didn't validate context files before TDD
✓ Severity: CRITICAL
✓ Proceeding with RCA analysis...

[Skill executes 8 phases...]

═══════════════════════════════════════════════
RCA COMPLETE: RCA-010
═══════════════════════════════════════════════

Title: Context File Validation Missing
Severity: CRITICAL
File: devforgeai/RCA/RCA-010-context-file-validation-missing.md

ROOT CAUSE:
No pre-flight validation in development skill enforces context file existence before TDD begins

RECOMMENDATIONS:
- CRITICAL: 1 (implement immediately)
- HIGH: 2 (implement this sprint)
- MEDIUM: 1 (next sprint)
- LOW: 0 (backlog)

NEXT STEPS:
Review CRITICAL recommendations immediately. Create story for implementation if substantial work (>2 hours).

Read complete RCA: devforgeai/RCA/RCA-010-context-file-validation-missing.md

═══════════════════════════════════════════════
```

### Example 2: Command Breakdown

```bash
$ /rca "/qa command has business logic in Phase 2"

✓ Issue: /qa command has business logic in Phase 2
✓ Severity: infer
✓ Proceeding with RCA analysis...

[Skill infers severity as HIGH from "business logic" keyword]
[Skill executes 8 phases...]

═══════════════════════════════════════════════
RCA COMPLETE: RCA-011
═══════════════════════════════════════════════

Title: QA Command Business Logic Violation
Severity: HIGH
File: devforgeai/RCA/RCA-011-qa-command-business-logic-violation.md

ROOT CAUSE:
Command contains report parsing and display logic that should be in skill or subagent (lean orchestration violation)

RECOMMENDATIONS:
- CRITICAL: 0
- HIGH: 3 (implement this sprint)
- MEDIUM: 1 (next sprint)
- LOW: 1 (backlog)

NEXT STEPS:
Review HIGH recommendations. Plan implementation in current sprint.

Read complete RCA: devforgeai/RCA/RCA-011-qa-command-business-logic-violation.md

═══════════════════════════════════════════════
```

---

**End of /rca Command**

**Total: ~350 lines**
**Character count: ~9,500 characters (63% of 15K budget)**
