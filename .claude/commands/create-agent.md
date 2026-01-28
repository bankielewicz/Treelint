---
description: Create DevForgeAI-aware Claude Code subagent
argument-hint: [name] [options]
model: opus
allowed-tools: Read, Glob, Grep, Skill, Task, AskUserQuestion
---

# /create-agent - DevForgeAI Subagent Creation

Create Claude Code subagents following DevForgeAI framework patterns and official Claude Code best practices.

---

## Quick Reference

```bash
# Guided mode (recommended)
/create-agent my-reviewer

# Domain mode
/create-agent backend-architect --domain=backend

# Template mode
/create-agent code-reviewer --template=code-reviewer

# Custom spec mode
/create-agent custom-agent --spec=specs/my-spec.md
```

**Domains:** backend, frontend, qa, security, deployment, architecture, documentation
**Templates:** code-reviewer, test-automator, documentation-writer, deployment-coordinator, requirements-analyst

---

## Command Workflow

### Phase 0: Argument Validation and Mode Detection

**Validate subagent name:**
```
IF $1 empty OR $1 NOT match "[a-z][a-z0-9-]*":
  AskUserQuestion:
    Question: "What should the subagent be named? (lowercase-with-hyphens)"
    Header: "Name"
    Options:
      - "Let me type a name"
      - "Cancel"
    multiSelect: false

  Extract NAME from response
  Validate format
ELSE:
  NAME = $1
```

**Detect mode:**
```
MODE = "guided"  # Default

IF $2 starts with "--template=":
  MODE = "template"
  TEMPLATE_NAME = substring after "="
  Validate template exists in .claude/skills/devforgeai-subagent-creation/assets/templates/

ELSE IF $2 starts with "--domain=":
  MODE = "domain"
  DOMAIN = substring after "="
  Validate DOMAIN in [backend, frontend, qa, security, deployment, architecture, documentation]

ELSE IF $2 starts with "--spec=":
  MODE = "custom"
  SPEC_FILE = substring after "="
  Validate file exists

ELSE IF $2 provided:
  Report: "Unknown option: $2"
  AskUserQuestion for mode selection
```

**Check existing:**
```
Glob(pattern=".claude/agents/${NAME}.md")

IF found:
  AskUserQuestion:
    Question: "Subagent '${NAME}' exists. Overwrite?"
    Header: "Exists"
    Options:
      - "Overwrite"
      - "Rename"
      - "Cancel"
    multiSelect: false
```

**Summary:**
```
✓ Name: ${NAME}
✓ Mode: ${MODE}
✓ Proceeding...
```

---

### Phase 1: Set Context Markers and Invoke Skill

**Prepare context for devforgeai-subagent-creation skill:**
```
**Subagent Name:** ${NAME}
**Creation Mode:** ${MODE}
**Framework:** DevForgeAI

IF MODE == "template":
  **Template:** ${TEMPLATE_NAME}
ELSE IF MODE == "domain":
  **Domain:** ${DOMAIN}
ELSE IF MODE == "custom":
  **Spec File:** ${SPEC_FILE}

Invoke skill:
Skill(command="devforgeai-subagent-creation")
```

**After skill invocation:**
- Skill's SKILL.md content expands inline
- **YOU execute the skill's workflow phases** (not waiting)
- Follow skill's 6-phase workflow
- Skill orchestrates agent-generator subagent for actual generation
- Return results as skill instructs

**The skill will:**
1. Extract context markers from conversation
2. Load DevForgeAI framework references
3. Load mode-specific templates (if template mode)
4. Prepare specification for agent-generator
5. Invoke agent-generator subagent v2.0 (framework-aware generation)
6. Process and return structured results

---

### Phase 2: Display Results

**Note:** This phase executes AFTER you complete the skill's workflow.

**Output skill results:**
```
Report: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Report: "✅ Subagent Generation Complete"
Report: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

Display: result.generated_files
Display: result.validation
Display: result.integration
```

---

### Phase 3: Display Next Steps

**Note:** This phase executes AFTER displaying results.

**Guide user on next steps:**
```
Display: result.next_steps (from skill)

# Skill provides formatted next steps including:
# 1. Restart terminal
# 2. Verify with /agents
# 3. Test invocation
# 4. Review reference file (if generated)
```

---

## Error Handling

### Invalid Name
```
Report: "❌ Name must be lowercase-with-hyphens"
Examples: code-reviewer, test-automator
AskUserQuestion for correction
```

### Template Not Found
```
Report: "❌ Template '${TEMPLATE_NAME}' not found"
List available templates
AskUserQuestion: Use guided mode or cancel?
```

### Invalid Domain
```
Report: "❌ Unknown domain: ${DOMAIN}"
List: backend, frontend, qa, security, deployment, architecture, documentation
AskUserQuestion for valid domain
```

### Spec File Missing
```
Report: "❌ Spec file not found: ${SPEC_FILE}"
AskUserQuestion: Guided mode, correct path, or cancel?
```

### Generation Failed
```
Report: "❌ Generation failed"
Display: error_details
AskUserQuestion: Show details, retry guided, or cancel?
```

---

## Success Criteria

- [ ] Subagent file created (.claude/agents/${NAME}.md)
- [ ] Reference file created (if applicable)
- [ ] Validation passed (12/12 checks)
- [ ] User guided on next steps
- [ ] Character budget <15K
- [ ] Token usage <5K main conversation

---

## Integration

**Invokes:**
- claude-code-terminal-expert skill (official patterns)
- agent-generator subagent v2.0 (generation + validation)

**Created subagents work with:**
- All DevForgeAI skills (framework-aware integration)
- Claude Code workflows (official pattern compliance)

**Use cases:**
- Custom subagents for project needs
- Command refactoring subagents (lean orchestration)
- Domain-specific subagents (backend, frontend, qa, etc.)
- Team workflow automation

---

## Performance

**Token Budget:**
- Command overhead: ~4K tokens
- Skill (isolated): ~2K tokens
- Subagent (isolated): ~30-50K tokens
- **Main conversation: ~4K** (92% isolated)

**Execution Time:**
- Guided: ~2-3 min
- Template: ~1-2 min
- Domain: ~1-2 min
- Custom: ~1-2 min

**Character Count:** 8,147 (54% of 15K budget) ✅ COMPLIANT
