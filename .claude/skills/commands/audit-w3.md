---
description: Audit codebase for W3 violations (auto-skill chaining without user control)
argument-hint: [--verbose|--fix-hints|--quiet]
model: opus
allowed-tools: Read, Grep, Glob
---

# /audit-w3 - W3 Compliance Audit

Scans for W3 violations: auto-skill chaining without user control.

**W3 Definition:** Skills/commands that auto-invoke other skills without user approval,
causing token overflow and violating lean orchestration principles.

**Reference:** BRAINSTORM-001 (line 85), STORY-135

---

## Phase 0: Parse Arguments

```
MODE = "normal"
QUIET = false
FIX_HINTS = false

FOR arg in $ARGUMENTS:
    IF arg == "--verbose":
        MODE = "verbose"
    ELIF arg == "--quiet":
        QUIET = true
    ELIF arg == "--fix-hints":
        FIX_HINTS = true
```

---

## Phase 1: Scan for Violations

### 1.1 CRITICAL: Subagent Skill Invocation (FORBIDDEN)

**Rule:** Subagents CANNOT invoke skills (architecture-constraints.md)

```
subagent_violations = Grep(
    pattern='Skill\s*\(\s*command\s*=',
    path='.claude/agents/',
    glob='*.md',
    output_mode='content',
    -n=true
)

# Exclude backup files
subagent_violations = filter_out(subagent_violations, '*.backup*', '*.original*')

CRITICAL_COUNT = count(subagent_violations)
```

### 1.2 HIGH: Non-Orchestration Skill Auto-Chaining

**Rule:** Only devforgeai-orchestration may coordinate skills freely

```
skill_violations = Grep(
    pattern='Skill\s*\(\s*command\s*=',
    path='.claude/skills/',
    glob='*.md',
    output_mode='content',
    -n=true
)

# Exclude legitimate orchestration
skill_violations = filter_out(skill_violations,
    'devforgeai-orchestration/*',
    '*.backup*',
    '*.original*'
)

# Check if each has user approval pattern (AskUserQuestion before Skill)
# or is display-only recommendation
FOR violation in skill_violations:
    IF has_user_approval_before(violation) OR is_display_only(violation):
        REMOVE from skill_violations

HIGH_COUNT = count(skill_violations)
```

### 1.3 MEDIUM: Missing W3 Compliance Documentation

**Rule:** Files with Skill() calls should have W3 compliance notes

```
files_with_skill_calls = Grep(
    pattern='Skill\s*\(\s*command\s*=',
    path='.claude/skills/',
    glob='*.md',
    output_mode='files_with_matches'
)

# Exclude orchestration
files_with_skill_calls = filter_out(files_with_skill_calls, 'devforgeai-orchestration/*')

missing_w3_notes = []
FOR file in files_with_skill_calls:
    content = Read(file_path=file)
    IF "W3" NOT IN content AND "display-only" NOT IN content:
        missing_w3_notes.append(file)

MEDIUM_COUNT = count(missing_w3_notes)
```

### 1.4 INFO: Auto-Invoke Language Patterns

```
language_patterns = Grep(
    pattern='(auto.*invoke|then invoke|invoking.*skill|automatically)',
    path='.claude/',
    glob='*.md',
    -i=true,
    output_mode='content',
    -n=true
)

# Filter to only concerning patterns (exclude documentation)
```

---

## Phase 2: Generate Report

```
IF NOT QUIET:
    Display:
    "╔════════════════════════════════════════════════════════════════════╗"
    "║                    W3 COMPLIANCE AUDIT REPORT                      ║"
    "╠════════════════════════════════════════════════════════════════════╣"
    "║  Scanned: {total_files} files in .claude/                          ║"
    "║  Violations: {CRITICAL_COUNT + HIGH_COUNT + MEDIUM_COUNT}          ║"
    "║    CRITICAL: {CRITICAL_COUNT}                                      ║"
    "║    HIGH: {HIGH_COUNT}                                              ║"
    "║    MEDIUM: {MEDIUM_COUNT}                                          ║"
    "╚════════════════════════════════════════════════════════════════════╝"

IF CRITICAL_COUNT > 0:
    Display:
    "## ❌ CRITICAL Violations (Block Release)"
    ""
    "| File | Line | Issue |"
    "|------|------|-------|"
    FOR v in subagent_violations:
        "| {v.file} | {v.line} | Subagent invoking Skill() - FORBIDDEN |"
    ""
    "**Action Required:** Subagents CANNOT invoke skills per architecture-constraints.md"

IF HIGH_COUNT > 0:
    Display:
    "## ⚠️ HIGH Priority (Review Required)"
    ""
    "| File | Line | Issue |"
    "|------|------|-------|"
    FOR v in skill_violations:
        "| {v.file} | {v.line} | Auto-invokes skill without user approval |"
    ""
    "**Action Required:** Add AskUserQuestion before Skill() OR use display-only pattern"

IF MEDIUM_COUNT > 0:
    Display:
    "## ⚠️ MEDIUM Priority (Documentation Gap)"
    ""
    "| File | Issue |"
    "|------|-------|"
    FOR f in missing_w3_notes:
        "| {f} | Missing W3 compliance note |"
    ""
    "**Action Required:** Add W3 compliance note explaining invocation pattern"

IF FIX_HINTS:
    Display:
    "## Remediation Pattern (STORY-135)"
    ""
    "**BEFORE (W3 Violation):**"
    "```"
    "Skill(command=\"devforgeai-architecture\")"
    "```"
    ""
    "**AFTER (W3 Compliant):**"
    "```"
    "**Recommended Next Action (display-only, no auto-invocation):**"
    ""
    "Run `/create-context [project-name]`"
    ""
    "**NOTE:** Per W3 compliance, this skill does NOT auto-invoke other skills."
    "```"
```

---

## Phase 3: Exit Status

```
IF CRITICAL_COUNT > 0:
    Display: "❌ AUDIT FAILED: {CRITICAL_COUNT} CRITICAL violations"
    EXIT 1
ELIF HIGH_COUNT > 0:
    Display: "⚠️ AUDIT WARNING: {HIGH_COUNT} HIGH priority violations"
    EXIT 0
ELSE:
    Display: "✅ W3 AUDIT PASSED"
    EXIT 0
```

---

## Exclusion Patterns

| Pattern | Reason |
|---------|--------|
| `devforgeai-orchestration/*` | Legitimate skill coordinator |
| `*.backup` | Historical files, not active |
| `*.backup-*` | Historical files, not active |
| `*.original-*` | Template files |
| `*.md.bak` | Editor backup files |

---

## Integration Notes

**Invoked by:** Manual user command, /release command (Phase 0.5)
**Invokes:** None (read-only scanning)
**Output:** Display report to user, exit code for CI/CD

---

## Success Criteria

- [ ] Detects known violation: `.claude/agents/tech-stack-detector.md:582`
- [ ] Excludes legitimate: `.claude/skills/devforgeai-orchestration/`
- [ ] Excludes backup files: `*.backup`, `*.original-*`
- [ ] CRITICAL violations return exit code 1
- [ ] `--quiet` mode works for release integration
- [ ] Report format matches other audit commands
