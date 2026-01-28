---
name: devforgeai-subagent-creation
description: Create DevForgeAI-aware Claude Code subagents following framework patterns and official best practices. Use when user runs /create-agent command or requests custom subagent creation with framework integration. Orchestrates agent-generator subagent for generation while ensuring Claude Code compliance and DevForgeAI framework awareness.
model: claude-model: opus-4-5-20251001
---

# DevForgeAI Subagent Creation Skill

**Purpose:** Orchestrate the creation of DevForgeAI-aware Claude Code subagents that comply with both official Claude Code patterns and DevForgeAI framework constraints.

---

## Overview

This skill creates Claude Code subagents that:
- Follow official Claude Code subagent patterns (from claude-code-terminal-expert skill)
- Integrate with DevForgeAI framework (reference context files, quality gates, workflow states)
- Pass 12-point framework compliance validation
- Include reference files for framework guardrails (when applicable)
- Use native tools for token efficiency (40-73% savings)

**Architecture:**
```
/create-agent command → devforgeai-subagent-creation skill → agent-generator subagent v2.0
                        (orchestration)                       (generation)
```

---

## When to Use This Skill

**Triggered by:**
- User runs `/create-agent [name] [options]` command
- User requests: "Create a [domain] subagent for [purpose]"
- User requests: "Generate framework-aware subagent"
- Developer creating command refactoring subagents (lean orchestration pattern)

**Use cases:**
- Custom subagents for project-specific needs
- Domain-specific subagents (backend, frontend, qa, security, etc.)
- Command refactoring subagents (formatters, interpreters, orchestrators)
- Team workflow automation subagents

---

## Core Workflow

### Phase 1: Extract Context from Conversation

**Parse creation parameters from context markers set by /create-agent command:**

```
Extract from conversation:
- **Subagent Name:** [name]
- **Creation Mode:** [guided|template|domain|custom]
- **Domain:** [domain] (if domain mode)
- **Template:** [template-name] (if template mode)
- **Spec File:** [file-path] (if custom mode)
- **Framework:** DevForgeAI
- **Claude Code Guidance:** Available (from claude-code-terminal-expert skill invoked by command)
```

---

### Phase 2: Load DevForgeAI Framework References

**Load framework context for integration:**

```
Read(file_path=".claude/skills/devforgeai-subagent-creation/references/framework-integration-patterns.md")

# This reference provides:
# - DevForgeAI context files (6 immutable constraints)
# - Quality gates (4 gates with thresholds)
# - Workflow states (11-state progression)
# - Skill integration patterns
# - Token efficiency mandates
```

---

### Phase 3: Load Mode-Specific Templates (If Applicable)

**If template mode, load template:**

```
IF Creation Mode == "template":
  TEMPLATE_PATH = ".claude/skills/devforgeai-subagent-creation/assets/templates/{TEMPLATE_NAME}-template.md"

  Read(file_path=TEMPLATE_PATH)

  # Template has placeholders: {name}, {description}, {domain}, {purpose}, etc.
  # Will be provided to agent-generator for customization
```

---

### Phase 4: Prepare Specification for agent-generator

**Assemble specification based on mode:**

```
SPECIFICATION = {
  "name": extracted_name,
  "mode": extracted_mode,
  "framework": "DevForgeAI",
  "claude_code_patterns": "Available from claude-code-terminal-expert skill in conversation",
  "devforgeai_context": "Loaded from references/framework-integration-patterns.md",
  "template_content": template_content (if template mode),
  "domain": extracted_domain (if domain mode),
  "spec_file": extracted_spec_file (if custom mode)
}
```

---

### Phase 5: Invoke agent-generator Subagent

**Delegate generation to specialized subagent:**

```
Task(
  subagent_type="agent-generator",
  description="Generate {name} subagent",
  prompt="Generate DevForgeAI-aware Claude Code subagent following specification in conversation.

**Specification extracted:**
- Name: {name}
- Mode: {mode}
- Framework: DevForgeAI
- Claude Code patterns: Available in conversation

**Instructions:**
1. Execute Phase 0: Load framework references (already done if cached)
2. Execute mode-specific workflow:
   - guided: Interactive questions for domain, purpose, responsibilities, tools, model
   - template: Customize loaded template with {name} and specifics
   - domain: Apply domain presets from framework-integration-patterns.md
   - custom: Parse spec file and enrich with framework context
3. Generate system prompt using Step 3.3 (ENHANCED) with Claude Code + DevForgeAI patterns
4. Run Step 3.6: Validate framework compliance (12-point validation)
5. Generate reference file if needed (Step 4.5 conditional logic)
6. Write subagent file to .claude/agents/{name}.md
7. Return structured report with files, validation results, integration guidance

**Expected output:**
Structured JSON report with:
- generated_files: {subagent: path, reference: path (if applicable)}
- validation: {devforgeai_compliance: results, claude_code_compliance: results, overall_status}
- integration: {works_with: [], invoked_by: [], context_files: []}
- next_steps: []
"
)
```

**What agent-generator will do:**
- Load Phase 0 references (if not cached)
- Execute mode-specific generation
- Validate framework compliance (12 checks)
- Generate reference file (if command-related, domain-specific, or decision-making)
- Return complete report

---

### Phase 6: Process and Return Results

**Format agent-generator output for command:**

```
Extract from agent-generator result:
- generated_files
- validation
- integration
- next_steps

Return to command:
{
  "generated_files": {
    "subagent": ".claude/agents/{name}.md ({lines} lines)",
    "reference": "{path} ({lines} lines)" (if generated)
  },
  "validation": {
    "devforgeai_compliance": "{passes}/6 checks passed",
    "claude_code_compliance": "{passes}/6 checks passed",
    "overall_status": "PASS|PASS WITH WARNINGS|FAIL",
    "issues": []
  },
  "integration": {
    "works_with": [skill names],
    "invoked_by": [skill names with phases],
    "context_files": [referenced files],
    "domain": "{domain}"
  },
  "next_steps": [
    "Restart terminal to load subagent",
    "Verify with /agents command",
    "Test with: Use {name} subagent to [task]",
    "Review reference: {path}" (if applicable)
  ],
  "has_reference": true|false,
  "reference_path": "{path}" (if applicable)
}
```

---

## Internal References

This skill uses progressive disclosure to load framework context as needed:

**Framework Integration Patterns:**
```
Read(file_path=".claude/skills/devforgeai-subagent-creation/references/framework-integration-patterns.md")
```
Provides: Context files, quality gates, workflow states, skill integration patterns

**Validation Checklist:**
```
Read(file_path=".claude/skills/devforgeai-subagent-creation/references/validation-checklist.md")
```
Provides: 12-point validation criteria, auto-fix guidance

**Reference File Generation Guide:**
```
Read(file_path=".claude/skills/devforgeai-subagent-creation/references/reference-file-generation-guide.md")
```
Provides: When to create reference files, template structure, framework guardrails

**Subagent Creation Workflow:**
```
Read(file_path=".claude/skills/devforgeai-subagent-creation/references/subagent-creation-workflow.md")
```
Provides: Detailed step-by-step workflow, mode-specific instructions

---

## External Knowledge Integration

**Claude Code Official Patterns:**

This skill references `claude-code-terminal-expert` skill for official Claude Code best practices:

```
# Command already invoked claude-code-terminal-expert before invoking this skill
# Official patterns available in conversation context:
# - File format requirements
# - YAML frontmatter fields
# - Tool selection principles (principle of least privilege)
# - Model selection guidelines (haiku <10K, sonnet 10-50K, opus >50K)
# - System prompt structure
# - Best practices for subagent creation

# This skill doesn't re-invoke claude-code-terminal-expert
# It uses the patterns already loaded by the command
```

**DevForgeAI Framework Context:**

Loaded from this skill's own references:
- 6 immutable context files (tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns)
- 4 quality gates with thresholds
- 11 workflow states
- Skill integration patterns
- Token efficiency mandates (native tools 40-73% savings)

---

## Templates

Templates with {placeholders} for quick subagent creation:

**Available templates:**
1. `code-reviewer-template.md` - Code quality, security, best practices review
2. `test-automator-template.md` - TDD test generation (unit, integration, E2E)
3. `documentation-writer-template.md` - Technical docs, API specs, user guides
4. `deployment-coordinator-template.md` - Infrastructure, CI/CD, release management
5. `requirements-analyst-template.md` - User story creation, acceptance criteria

**Template structure:**
```markdown
---
name: {name}
description: {description_with_proactive_triggers}
tools: {suggested_tools_for_domain}
model: {suggested_model_for_complexity}
---

# {Name}

{one_line_purpose}

## Purpose

{detailed_purpose_2-3_sentences}

## When Invoked

**Proactive triggers:**
- {trigger_1_based_on_domain}
- {trigger_2_based_on_responsibilities}

... [continues with all sections]
```

**Placeholder replacement:**
- agent-generator receives template content
- Replaces {placeholders} with actual values
- Validates result
- Writes to disk

---

## Success Criteria

- [ ] Subagent created in `.claude/agents/{name}.md`
- [ ] YAML frontmatter valid (name, description, tools, model)
- [ ] System prompt >200 lines
- [ ] Framework compliance: 12/12 checks passed (or approved warnings)
- [ ] Reference file generated (if applicable)
- [ ] Integration points documented
- [ ] Next steps provided to user

---

## Token Efficiency

**Skill execution:**
- Load references: ~10K tokens (one-time, progressive)
- Mode-specific logic: ~5K tokens
- agent-generator invocation: ~2K tokens (overhead, execution in isolated context)
- Result processing: ~2K tokens
- **Total in main conversation: ~19K tokens**

**agent-generator execution (isolated):**
- ~30-50K tokens (generation, validation, reference file creation)
- Doesn't impact main conversation

**Overall efficiency:** ~60% of work in isolated context

---

## Integration with DevForgeAI

**This skill is invoked by:**
- `/create-agent` command (primary use case)
- Can be invoked directly: `Skill(command="devforgeai-subagent-creation")`

**This skill invokes:**
- agent-generator subagent v2.0 (for actual generation)

**Created subagents work with:**
- All DevForgeAI skills (devforgeai-development, devforgeai-qa, devforgeai-architecture, etc.)
- Claude Code workflows (official pattern compliance)

---

## Progressive Disclosure

Load reference files as needed during workflow:

**Always load:**
- `framework-integration-patterns.md` (Phase 2)

**Conditionally load:**
- `validation-checklist.md` (if validation details needed)
- `reference-file-generation-guide.md` (if reference file creation needed)
- `subagent-creation-workflow.md` (if detailed workflow guidance needed)

**Templates loaded on demand:**
- Only load requested template in template mode
- Other templates remain unloaded (token efficiency)

---

**Token Budget:** <20K per invocation (skill overhead in main conversation)
**Execution Time:** 1-3 minutes (varies by mode)
**Complexity:** Medium (orchestrates agent-generator, doesn't do generation itself)
**Pattern:** Lean orchestration skill (delegates to specialized subagent)
