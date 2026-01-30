# Reference File Generation Guide

**Purpose:** Guide for when and how to generate reference files for framework guardrails

Reference files prevent "bull in china shop" behavior by providing explicit framework constraints, decision boundaries, and integration patterns.

---

## When to Generate Reference Files

### Mandatory (MUST Generate)

**1. Command Refactoring Subagents**

**Triggers:**
- Subagent name contains: `formatter`, `interpreter`, `orchestrator`, `result`, `display`
- Purpose mentions: "command refactoring", "slash command", "/[a-z]"

**Reasoning:** Lean orchestration protocol requires reference files for command-related subagents

**Examples:**
- qa-result-interpreter (interprets QA reports for /qa command)
- ui-spec-formatter (formats UI specs for /create-ui command)
- sprint-planner (generates sprint files for /create-sprint command)

---

### Recommended (SHOULD Generate)

**2. Domain-Specific Subagents with Constraints**

**Triggers:**
- Domain in: qa, architecture, security, deployment
- Has strict thresholds or rules from context files

**Reasoning:** Domains have framework constraints requiring explicit guardrails

**Examples:**
- security-auditor (OWASP Top 10 rules, violation severity definitions)
- architect-reviewer (architecture patterns, ADR requirements)

---

**3. Decision-Making Subagents**

**Triggers:**
- Responsibilities include: decision, determine, select, choose, evaluate, analyze, assess

**Reasoning:** Decisions require framework boundaries to prevent autonomous behavior

**Examples:**
- requirements-analyst (story format decisions, AC quality assessment)
- deployment-engineer (deployment strategy selection, rollback decisions)

---

### Optional (MAY Generate)

**4. User Explicitly Requests**

**Triggers:**
- User specifies: `--with-reference` option
- Conversation contains: `**Generate Reference:** true`

**Reasoning:** User knows their use case needs framework guardrails

---

## Reference File Structure

### Location

**Preferred:** `.claude/skills/{related-skill}/references/{subagent-topic}-guide.md`

**Fallback:** `.claude/skills/devforgeai-subagent-creation/references/{subagent-name}-guide.md`

**Determine related skill:**
```
IF subagent integrates primarily with one skill:
  related_skill = that skill name (e.g., devforgeai-qa for qa-result-interpreter)
ELSE:
  Use devforgeai-subagent-creation as default location
```

---

### Template Structure (200-600 lines target)

```markdown
# {Subagent Topic} Guide

**Purpose:** Framework guardrails for {subagent_name} subagent

Prevents autonomous behavior by providing:
- DevForgeAI workflow context
- Immutable constraints
- Decision boundaries
- Integration patterns

**Reference Type:** {command-refactoring|domain-constraints|decision-guidance|custom}

---

## DevForgeAI Context

### Workflow States (11-State Progression)

[Include workflow diagram]

**{subagent_name}'s role:**
[Describe at which states this operates]

### Quality Gates (4 Gates)

[List all 4 gates]

**{subagent_name}'s participation:**
[Which gates, what it validates]

### {Domain-Specific Context}

[Customize based on domain:]
- Backend: Clean architecture layers
- QA: Coverage thresholds (95%/85%/80%)
- Architecture: Technology decision process
- Security: OWASP Top 10, violation severity
- Deployment: Deployment strategies
- Documentation: Coverage requirements

---

## Framework Constraints

### 1. {Constraint Category 1} (Strict, Immutable)

**Rules:**
- {rule_1 from context files}
- {rule_2 from context files}

**Example:**
[Concrete example showing constraint]

**Never say:** {relaxation examples}
**Always enforce:** {strict enforcement examples}

### 2. {Constraint Category 2} (Deterministic)

**Decision tree:**
```
IF {objective_condition} THEN {outcome}
ELSE IF {objective_condition} THEN {outcome}
ELSE {default}
```

**No subjective interpretation allowed.**

---

## {Subagent Task} Guidelines

### Task Execution Within Framework

1. **{Task Step} with Constraint**
   - Check: {which context file}
   - Action: {what to do}
   - Constraint: {which rule}
   - Output: {expected result}

**Output Template:**
[Exact template subagent should follow]

**Anti-patterns to avoid:**
[From anti-patterns.md for domain]

**Correct patterns:**
[From coding-standards.md for domain]

---

## Framework Integration Points

### Context Files to Reference

**When to check:**
- tech-stack.md: {when}
- anti-patterns.md: {when}
[etc for domain]

**How to reference:**
```
Read(file_path="devforgeai/specs/context/{file}.md")
Extract rules
Apply to analysis
Report violations with reference
```

### Related Skills/Subagents

**{Skill/Subagent Name}:**
- When: {phase/step/condition}
- Expect: {input/output}
- Handle: {processing}
- Error: {what if fails}

### Tool Usage Patterns

**Mandated:** Native tools for files (40-73% savings)
**Rationale:** 274K (Bash) → 108K (Native) = 61% savings

---

## Output Format (If Subagent Returns Data)

**Structured JSON contract:**

```json
{
  "status": "SUCCESS|ERROR|PARTIAL",
  "display": {"template": "...", "title": "...", "sections": [...]},
  "data": {...},
  "validation": {"passes": [], "warnings": [], "failures": []},
  "recommendations": {"next_steps": [], "remediation": [], "priority": "..."},
  "metadata": {"timestamp": "...", "subagent": "...", "framework_version": "..."}
}
```

---

## Error Scenarios

### {Error Type 1}

**Detection:** {how to detect}
**Response:** {JSON response}
**Caller guidance:** {how caller handles}

### Framework Constraint Violation

**Detection:**
```
IF violates {context-file}.md rule:
  CONSTRAINT_VIOLATION = true
```

**Response:**
```json
{
  "status": "ERROR",
  "result_type": "constraint_violation",
  "data": {"violated_constraint": "...", "framework_rule": "..."},
  "recommendations": {"remediation": ["Review {file}", "Correct {rule}"], "priority": "HIGH"}
}
```

**Caller guidance:** HALT, display violation, guide to correct approach

---

## Testing Checklist

- [ ] Respects constraint 1
- [ ] Respects constraint 2
- [ ] Output format matches schema
- [ ] Framework-aware (not siloed)
- [ ] Integration tested
- [ ] Error handling comprehensive
- [ ] No autonomous decisions

---

**Target:** 200-600 lines
**Update:** When framework constraints change
**Owner:** DevForgeAI framework team
