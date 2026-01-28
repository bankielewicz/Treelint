# Subagent Creation Workflow

**Purpose:** Detailed workflow for creating DevForgeAI-aware Claude Code subagents

This workflow is executed by agent-generator subagent when invoked by devforgeai-subagent-creation skill.

---

## Workflow Overview

**6 Steps in agent-generator:**
1. Load framework references (Phase 0)
2. Check existing subagents (Step 1)
3. Generate subagent content (Steps 2-3)
4. Validate framework compliance (Step 3.6)
5. Write subagent file (Step 4)
6. Generate reference file (Step 4.5 - conditional)
7. Return summary report (Step 5)

---

## Mode-Specific Workflows

### Guided Interactive Mode

**When:** User provides name only, no options

**Process:**
1. AskUserQuestion for domain (8 options: backend, frontend, qa, security, deployment, architecture, documentation, general)
2. AskUserQuestion for detailed purpose (2-3 sentences)
3. AskUserQuestion for responsibilities (multi-select: code generation, analysis, testing, documentation, validation, coordination, decision-making)
4. Suggest tools based on domain + responsibilities
5. AskUserQuestion for tool selection (suggested, inherit all, or custom)
6. Estimate complexity from responsibilities + purpose
7. Suggest model (haiku if simple, sonnet if complex)
8. AskUserQuestion for model selection
9. Assemble specification from all responses
10. Generate subagent using specification

**Example interaction:**
```
Domain? → "qa"
Purpose? → "Validate code coverage meets framework thresholds"
Responsibilities? → ["Validation", "Analysis"]
Tools? → "Use suggested: Read, Grep, Glob"
Model? → "Use suggested: haiku (simple validation task)"
→ Generate coverage-validator subagent
```

---

### Template Mode

**When:** User provides `--template=[name]`

**Process:**
1. Validate template exists in `.claude/skills/devforgeai-subagent-creation/assets/templates/`
2. Read template file
3. Extract {placeholders} from template
4. For each placeholder:
   - If placeholder has obvious value (e.g., {name}), use it
   - If placeholder needs input (e.g., {detailed_purpose}), ask user OR infer from domain
5. Replace all {placeholders} with actual values
6. Validate customized content
7. Write subagent file

**Placeholder replacement logic:**
```
{name} → user-provided name
{display_name} → Title Case of name
{description} → Generate from domain + template purpose
{tools} → From template's suggested tools
{model} → From template's suggested model
{one_line_purpose} → Extract from template or infer
{detailed_purpose} → Ask user or use template default
{proactive_trigger_1/2/3} → Infer from domain + template role
{domain} → Infer from template type
```

---

### Domain Mode

**When:** User provides `--domain=[domain]`

**Process:**
1. Load domain presets from framework-integration-patterns.md
2. Apply domain configuration:
   - Suggested tools (from domain config)
   - Model (from domain config)
   - Context files (from domain config)
   - Integration skills (from domain config)
3. AskUserQuestion for purpose (custom within domain)
4. Generate subagent using domain presets + purpose

**Domain presets:**
```
backend:
  tools: [Read, Write, Edit, Grep, Glob, Bash(git:*|npm:*|pip:*|dotnet:*)]
  model: opus
  context_files: [all 6]
  integration_skills: [devforgeai-development, devforgeai-architecture]

frontend:
  tools: [Read, Write, Edit, Grep, Glob, Bash(npm:*)]
  model: opus
  context_files: [tech-stack, source-tree, coding-standards]
  integration_skills: [devforgeai-development, devforgeai-ui-generator]

qa:
  tools: [Read, Grep, Glob, Bash(pytest:*|npm:test|dotnet:test)]
  model: opus
  context_files: [anti-patterns, coding-standards]
  integration_skills: [devforgeai-qa, devforgeai-development]

[etc for all 7 domains]
```

---

### Custom Specification Mode

**When:** User provides `--spec=[file]`

**Process:**
1. Read specification file
2. Parse format (YAML or Markdown with frontmatter)
3. Extract: name, purpose, responsibilities, tools, model, workflow
4. Validate required fields present
5. Enrich with framework context:
   - Add context_files if missing (infer from domain)
   - Add integration_points if missing (infer from responsibilities)
   - Add token_efficiency if missing (add standard strategies)
6. Generate subagent from enriched specification

**Spec file formats supported:**
- YAML: Complete specification in YAML format
- Markdown: YAML frontmatter + workflow in markdown body

---

## System Prompt Generation (Step 3.3)

**Use loaded patterns:**
1. CLAUDE_CODE_PATTERNS (from claude-code-terminal-expert via conversation)
2. DEVFORGEAI_CONTEXT (from framework-integration-patterns.md)
3. LEAN_ORCHESTRATION_PROTOCOL (if command-related)

**Sections to generate:**
1. YAML frontmatter (name, description, tools, model)
2. # {Name} heading
3. ## Purpose (2-3 sentences with DevForgeAI context where applicable)
4. ## When Invoked (proactive, explicit, automatic triggers)
5. ## Workflow (numbered steps with tool usage examples)
6. ## Framework Integration (NEW - context files, quality gates, workflow states, skills)
7. ## Tool Usage Protocol (NEW - native tools mandate)
8. ## Success Criteria (measurable)
9. ## Principles (domain-specific + DevForgeAI alignment)
10. ## Token Efficiency (target + optimization strategies)
11. ## References (context files, framework integration)

**Key enhancement in Step 3.3:**
- Uses Claude Code official structure (from claude-code-terminal-expert)
- Adds DevForgeAI framework sections (Framework Integration, Tool Usage Protocol)
- References context files appropriately for domain
- Documents skill integration patterns
- Includes token efficiency strategies with evidence (40-73% savings)

---

## Validation (Step 3.6)

**12-point validation:**
- 6 DevForgeAI checks
- 6 Claude Code checks

**See:** validation-checklist.md for complete validation logic

**Outcomes:**
- PASS: Proceed to Step 4
- PASS WITH WARNINGS: Inform user, proceed
- FAIL: Auto-fix OR halt

---

## Reference File Generation (Step 4.5)

**Conditional logic:**
```
NEEDS_REFERENCE = false

IF command-refactoring:
  NEEDS_REFERENCE = true
  TYPE = "command-refactoring"
ELSE IF domain in [qa, architecture, security, deployment]:
  NEEDS_REFERENCE = true
  TYPE = "domain-constraints"
ELSE IF decision-making:
  NEEDS_REFERENCE = true
  TYPE = "decision-guidance"
ELSE IF user requested:
  NEEDS_REFERENCE = true
  TYPE = "custom"
```

**Reference content generation:**
1. Load template based on type
2. Customize for subagent's domain/purpose
3. Include DevForgeAI workflow context
4. Document framework constraints specific to domain
5. Provide task execution guidelines
6. List integration points
7. Define error scenarios
8. Create testing checklist

**Template sections:**
- DevForgeAI Context (workflow, quality gates, domain-specific)
- Framework Constraints (strict rules, deterministic logic)
- Task Guidelines (how to execute within constraints)
- Framework Integration Points (context files, related skills, tool patterns)
- Output Format (if structured data returned)
- Error Scenarios (detection, response, caller guidance)
- Testing Checklist (validation of framework compliance)

**Size target:** 200-600 lines (focused guardrails, not comprehensive docs)

---

## Summary Report Generation (Step 5)

**Report structure:**
```markdown
# Subagent Generation Report

**Generated:** {timestamp}
**Subagent:** {name}
**Mode:** {mode}

## Generated Files

✅ **Subagent:** .claude/agents/{name}.md ({lines} lines)
${if reference}: ✅ **Reference:** {path} ({lines} lines)

## Validation Results

**DevForgeAI:** {passes}/6 ✅
**Claude Code:** {passes}/6 ✅
**Overall:** {status}

${if warnings}: ⚠️ **Warnings:** {count}
${if failures}: ❌ **Failures:** {count}

## Integration

**Works with:** {skill_list}
**Invoked by:** {skill_name}, {phase}
**Context files:** {file_list}
**Domain:** {domain}

## Next Steps

1. Restart terminal
2. Verify: /agents
3. Test: Use {name} subagent to [task]
${if reference}: 4. Review: {reference_path}
```

---

**This guide ensures consistent, framework-compliant subagent generation across all creation modes.**
