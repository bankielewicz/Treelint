# Subagent Validation Checklist

**Purpose:** 12-point framework compliance validation for generated subagents

This checklist is used by agent-generator subagent (Step 3.6) to validate generated subagents against DevForgeAI constraints and Claude Code best practices.

---

## DevForgeAI Framework Compliance (6 Checks)

### 1. Tool Usage Validation ✅/❌

**Check:** Subagent uses native tools for file operations

**Validation:**
```
Grep(pattern="Bash\\(cat:|grep:|find:|sed:|awk:|echo >|head:|tail:)")

IF matches found:
  VIOLATION: "Subagent uses Bash for file operations"
  List violations with line numbers
  Auto-fix: Replace with native tools (Read/Grep/Glob/Edit/Write)
  Status: FAIL
ELSE:
  PASS: "✅ Tool usage follows native tools pattern"
```

### 2. Context File Awareness ✅/⚠️

**Check:** Subagent references appropriate context files for its domain

**Expected by domain:**
- Backend: All 6 files
- Frontend: 3 files (tech-stack, source-tree, coding-standards)
- QA: 2 files (anti-patterns, coding-standards)
- Architecture: All 6 files
- Security: 3 files
- Deployment: 3 files

**Validation:**
```
Grep(pattern="tech-stack\\.md|source-tree\\.md|dependencies\\.md|
              coding-standards\\.md|architecture-constraints\\.md|
              anti-patterns\\.md")

Count references
Compare to expected for domain

IF domain requires files AND none found:
  WARN: "Should reference [expected files]"
  Status: PASS WITH WARNINGS
ELSE:
  PASS: "✅ Context file awareness present"
```

### 3. Framework Integration Section ✅/❌

**Check:** ## Framework Integration section present and populated

**Validation:**
```
Grep(pattern="## Framework Integration")

IF not found:
  VIOLATION: "Missing Framework Integration section"
  Auto-fix: Add section with domain-appropriate content
  Status: FAIL
ELSE:
  Grep(pattern="devforgeai-[a-z]+")
  IF no skill references:
    WARN: "No DevForgeAI skill integration documented"
    Status: PASS WITH WARNINGS
  ELSE:
    PASS: "✅ Framework integration documented"
```

### 4. Tool Usage Protocol Section ✅/❌

**Check:** ## Tool Usage Protocol section present with native tools mandate

**Validation:**
```
Grep(pattern="## Tool Usage Protocol|ALWAYS use native tools")

IF not found:
  VIOLATION: "Missing Tool Usage Protocol section"
  Auto-fix: Add complete section
  Status: FAIL
ELSE:
  PASS: "✅ Tool Usage Protocol present"
```

### 5. Token Efficiency Section ✅/❌

**Check:** ## Token Efficiency section with strategies

**Validation:**
```
Grep(pattern="## Token Efficiency|40-73% token savings|native tools")

IF not found:
  VIOLATION: "Missing Token Efficiency section"
  Auto-fix: Add section with standard strategies
  Status: FAIL
ELSE:
  PASS: "✅ Token efficiency documented"
```

### 6. Lean Orchestration Compliance ✅/⚠️/N/A

**Check:** Command-related subagents have reference file generation planned

**Validation:**
```
IF subagent_purpose contains "formatter|interpreter|orchestrator|command":
  Grep(pattern="reference file|framework guardrails")

  IF not found:
    VIOLATION: "Command-related subagent must generate reference file"
    Flag: NEEDS_REFERENCE_FILE = true
    Status: FAIL
  ELSE:
    PASS: "✅ Reference file generation planned"
ELSE:
  PASS: "N/A - Not command-related"
```

---

## Claude Code Best Practice Compliance (6 Checks)

### 1. YAML Frontmatter Format ✅/❌

**Check:** Valid YAML with required fields

**Required fields:**
- name: lowercase-with-hyphens
- description: natural language with triggers
- tools: comma-separated OR omitted
- model: opus|haiku|opus|inherit OR omitted

**Validation:**
```
Parse YAML frontmatter

IF parse error:
  VIOLATION: "Invalid YAML syntax"
  Status: FAIL
ELSE:
  Check required fields present
  Check name format
  Check description has content
  PASS: "✅ YAML frontmatter valid"
```

### 2. Description Quality ✅/⚠️

**Check:** Description includes invocation triggers (esp. "proactively" for auto-invoked)

**Validation:**
```
IF subagent has auto-invoke triggers:
  Grep(pattern="proactively", path=description_field)

  IF not found:
    WARN: "Auto-invoked subagent should include 'proactively'"
    Suggestion: Add "Use proactively when [triggers]"
    Status: PASS WITH WARNINGS
  ELSE:
    PASS: "✅ Description follows trigger pattern"
```

### 3. Tool Selection ✅/⚠️

**Check:** Tools appropriate for task complexity (principle of least privilege)

**Validation:**
```
Expected tools by task:
- Validation/Analysis: Read, Grep, Glob (view-only)
- Generation: Read, Write, Edit, Grep, Glob
- Testing: + Bash(pytest:*|npm:test)
- Deployment: + Bash(docker:*|kubectl:*)

IF tools_count > expected:
  WARN: "Tool access broader than needed"
  Suggest minimal set
  Status: PASS WITH WARNINGS
ELSE:
  PASS: "✅ Tool selection appropriate"
```

### 4. Model Selection ✅/⚠️

**Check:** Model appropriate for estimated token usage

**Validation:**
```
IF model == "haiku" AND estimated_tokens > 10K:
  WARN: "Haiku may be insufficient"
  Suggest: "Use sonnet"
  Status: PASS WITH WARNINGS
ELSE IF model == "sonnet" AND estimated_tokens < 5K:
  INFO: "Sonnet may be overqualified"
  Suggest: "Consider haiku"
  Status: PASS
ELSE:
  PASS: "✅ Model selection appropriate"
```

### 5. System Prompt Structure ✅/❌

**Check:** All required sections present

**Required sections:**
- ## Purpose
- ## When Invoked
- ## Workflow
- ## Success Criteria

**DevForgeAI additions:**
- ## Framework Integration
- ## Tool Usage Protocol
- ## Token Efficiency

**Validation:**
```
FOR section in required_sections:
  Grep(pattern=section)
  IF not found:
    missing_sections.append(section)

IF missing_sections:
  VIOLATION: "Missing sections: {list}"
  Auto-fix: Generate placeholders
  Status: FAIL
ELSE:
  PASS: "✅ All sections present"
```

### 6. Workflow Quality ✅/⚠️

**Check:** Workflow has appropriate number of steps (3-15)

**Validation:**
```
Count numbered workflow steps

IF < 3:
  WARN: "Workflow too simple (<3 steps)"
  Status: PASS WITH WARNINGS
ELSE IF > 15:
  WARN: "Workflow too complex (>15 steps)"
  Suggest: "Consider decomposition"
  Status: PASS WITH WARNINGS
ELSE:
  PASS: "✅ Workflow step count appropriate"
```

---

## Validation Report Format

```markdown
## Framework Compliance Validation Report

**Subagent:** {name}
**Date:** {timestamp}

### DevForgeAI Framework Compliance

| Check | Status | Details |
|-------|--------|---------|
| Tool usage | ✅/❌/⚠️ | {details} |
| Context files | ✅/⚠️ | {files_referenced or warning} |
| Framework integration | ✅/❌/⚠️ | {skills_documented or issue} |
| Tool protocol section | ✅/❌ | {present or missing} |
| Token efficiency | ✅/❌ | {documented or missing} |
| Lean orchestration | ✅/❌/⚠️/N/A | {status} |

### Claude Code Best Practice Compliance

| Check | Status | Details |
|-------|--------|---------|
| YAML frontmatter | ✅/❌ | {valid or error} |
| Description quality | ✅/⚠️ | {triggers or warning} |
| Tool selection | ✅/⚠️ | {appropriate or excessive} |
| Model selection | ✅/⚠️ | {appropriate or suboptimal} |
| Structure | ✅/❌ | {all sections or missing} |
| Workflow quality | ✅/⚠️ | {step_count steps} |

### Overall Status

**Result:** PASS | PASS WITH WARNINGS | FAIL

**Summary:**
- Passes: {count}/12
- Warnings: {count}/12
- Failures: {count}/12

**Critical Issues:** {list}
**Warnings:** {list}
**Recommended Actions:** {list}
```

---

## Auto-Fix Logic

**Auto-fixable violations:**
1. Missing Framework Integration section → Add with template
2. Missing Tool Usage Protocol section → Add complete section
3. Missing Token Efficiency section → Add with standard strategies
4. Bash file operations detected → Suggest native tool replacements
5. Missing required sections → Generate placeholder sections

**Require manual fix:**
1. YAML syntax errors
2. Invalid name format
3. Context file awareness (need domain context)
4. Skill integration documentation (need integration knowledge)

**Auto-fix process:**
```
IF status == FAIL AND auto_fixes_available:
  AskUserQuestion:
    Options:
      - "Apply auto-fixes automatically"
      - "Show issues first"
      - "Cancel generation"

  IF apply auto-fixes:
    Apply all auto-fixes
    Re-validate
    IF still failing:
      Require manual intervention
```

---

**This checklist ensures all generated subagents meet DevForgeAI and Claude Code standards.**
