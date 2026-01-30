# DevForgeAI CLI - Workflow Validators

**Version:** 0.1.0
**Purpose:** Automated validation tools for DevForgeAI spec-driven development
**Status:** Production ready

---

## Overview

DevForgeAI CLI provides fast, deterministic validators that prevent workflow violations and enforce quality gates in spec-driven development.

**Key Features:**
- ✅ **Autonomous deferral detection** - Prevents DoD items deferred without user approval
- ✅ **Pre-commit integration** - Blocks commits with violations
- ✅ **Fast validation** - <100ms per check (vs ~5,000 tokens for AI validation)
- ✅ **Git-aware** - Validates Git availability before workflows
- ✅ **Context enforcement** - Ensures 6 context files exist

**Based on industry research:**
- SpecDriven AI: spec_validator.py pattern
- GitHub DoD Checker: checkbox validation approach
- Pre-commit framework: standard hook integration

---

## Installation

### Quick Start

```bash
# Navigate to DevForgeAI project
cd /path/to/devforgeai

# Install CLI package
pip install --break-system-packages -e .claude/scripts/

# Install pre-commit hooks
bash .claude/scripts/install_hooks.sh

# Verify installation
devforgeai --version
```

### Requirements

- Python 3.8+
- PyYAML 6.0+
- Git (for pre-commit hooks)

---

## Commands

### validate-dod - DoD Completion Validator

**Purpose:** Detect autonomous deferrals and validate user approval markers

**Usage:**
```bash
devforgeai validate-dod devforgeai/specs/Stories/STORY-001.story.md

# JSON output
devforgeai validate-dod STORY-001.story.md --format=json
```

**What it validates:**
- ✅ All DoD `[x]` items have Implementation Notes entry
- ✅ Status consistency (DoD `[x]` must match Impl `[x]` or have justification)
- ✅ Deferred items have user approval markers
- ✅ Referenced stories/ADRs exist
- ❌ **BLOCKS: Autonomous deferrals** (DoD `[x]` + Impl `[ ]` without approval)

**Exit codes:**
- `0` - Valid (all DoD items complete or properly justified)
- `1` - Violations found (details in output)
- `2` - Error (file not found, invalid format, etc.)

---

### check-git - Git Availability Validator

**Purpose:** Validate Git repository before DevForgeAI workflows

**Usage:**
```bash
devforgeai check-git

# Check specific directory
devforgeai check-git --directory=/path/to/project

# JSON output
devforgeai check-git --format=json
```

**What it checks:**
- ✅ Directory is inside Git working tree
- ✅ Git command available
- ❌ **BLOCKS: Non-Git directories** (prevents RCA-006 errors)

**Exit codes:**
- `0` - Git available
- `1` - Git not available
- `2` - Error (Git not installed, etc.)

---

### validate-context - Context Files Validator

**Purpose:** Ensure all 6 DevForgeAI context files exist

**Usage:**
```bash
devforgeai validate-context

# Check specific directory
devforgeai validate-context --directory=/path/to/project

# JSON output
devforgeai validate-context --format=json
```

**What it validates:**
- ✅ All 6 context files exist: tech-stack.md, source-tree.md, dependencies.md, coding-standards.md, architecture-constraints.md, anti-patterns.md
- ✅ Files are non-empty (not placeholders)
- ⚠️ **WARNS:** Files <100 bytes (likely incomplete)

**Exit codes:**
- `0` - All files valid
- `1` - Missing or empty files
- `2` - Error

---

### ast-grep scan - Semantic Code Analysis (STORY-115)

**Purpose:** Detect security vulnerabilities and anti-patterns using AST-based analysis

**Usage:**
```bash
# Basic scan with grep fallback
devforgeai ast-grep scan ./src --fallback

# Scan with category filter
devforgeai ast-grep scan ./src --category security --format json

# Scan specific language
devforgeai ast-grep scan ./tests --language python --format markdown

# Full options
devforgeai ast-grep scan <path> \
  --category security \
  --language python \
  --format json \
  --fallback
```

**What it detects:**
- ✅ **SQL injection** - String concatenation in queries
- ✅ **Hardcoded secrets** - API keys, passwords, tokens in code
- ✅ **AWS credentials** - Access keys, secret keys
- ⚠️ **Accuracy:** 90-95% with ast-grep, 60-75% with grep fallback

**Options:**
- `--category` - Filter by: security, anti-patterns, complexity, architecture
- `--language` - Filter by: python, csharp, typescript, javascript
- `--format` - Output format: text (default), json, markdown
- `--fallback` - Force grep-based analysis (skip ast-grep)

**Auto-Install Behavior:**
```bash
# If ast-grep not installed, you'll be prompted:
ast-grep Not Found
==================================
Options:
  1) Install now (pip install ast-grep-cli)
  2) Use fallback (grep-based analysis)
  3) Skip

Select option [1-3]:
```

**Configuration:**
```yaml
# devforgeai/ast-grep/config.yaml
fallback_mode: false        # Use grep by default?
min_version: "0.40.0"       # Minimum ast-grep version
max_version: "1.0.0"        # Maximum version (exclusive)
allow_auto_install: false   # Skip prompt and auto-install?
```

**Exit codes:**
- `0` - No violations found (or scan skipped)
- `1` - Violations detected
- `2` - Error (invalid arguments, scan failed)

**Example Output (JSON):**
```json
{
  "violations": [
    {
      "file": "app.py",
      "line": 42,
      "column": 5,
      "rule_id": "SEC-001",
      "severity": "CRITICAL",
      "message": "Potential SQL injection via string concatenation",
      "evidence": "query = \"SELECT * FROM users WHERE id = \" + user_id",
      "analysis_method": "grep-fallback",
      "category": "security"
    }
  ],
  "analysis_method": "grep-fallback",
  "summary": {
    "total_violations": 1,
    "by_severity": {"CRITICAL": 1},
    "accuracy_note": "60-75% vs 90-95% with ast-grep"
  }
}
```

---

## Pre-Commit Integration

### How It Works

The pre-commit hook automatically runs `validate-dod` on all staged `.story.md` files before allowing commit.

**Installation:**
```bash
bash .claude/scripts/install_hooks.sh
```

**Workflow:**
```bash
# Developer makes changes
git add devforgeai/specs/Stories/STORY-042.story.md

# Attempt commit
git commit -m "feat: Implement feature"

# Hook runs automatically:
🔍 DevForgeAI Validators Running...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  📋 Validating: devforgeai/specs/Stories/STORY-042.story.md
     ✅ Passed
✅ All validators passed - commit allowed

# If violations:
❌ COMMIT BLOCKED - Fix violations
```

**Bypass (not recommended):**
```bash
git commit --no-verify
```

---

## Example Scenarios

### Scenario 1: Valid Story (All DoD Complete)

**Story file:**
```markdown
## Definition of Done
- [x] Unit tests written
- [x] Code reviewed

## Implementation Notes
- [x] Unit tests written - Completed: Created tests
- [x] Code reviewed - Completed: Passed review
```

**Validator result:**
```
✅ story.md: All DoD items validated
Exit code: 0
```

---

### Scenario 2: Autonomous Deferral (VIOLATION)

**Story file:**
```markdown
## Definition of Done
- [x] Unit tests written
- [x] Performance benchmarks  ← Marked complete

## Implementation Notes
- [x] Unit tests written - Completed
- [ ] Performance benchmarks - Deferred to STORY-XXX  ← Actually deferred
                                                        ← NO user approval!
```

**Validator result:**
```
❌ VALIDATION FAILED

CRITICAL VIOLATIONS:
  • Performance benchmarks
    Error: AUTONOMOUS DEFERRAL DETECTED
    DoD: [x] | Impl: [ ]
    Found: Deferred to STORY-XXX
    Fix: Add user approval marker

GIT COMMIT BLOCKED
Exit code: 1
```

---

### Scenario 3: Valid Deferral (User Approved)

**Story file:**
```markdown
## Definition of Done
- [x] Performance benchmarks

## Implementation Notes
- [ ] Performance benchmarks - Deferred to STORY-042
  **User Approved:** YES (via AskUserQuestion 2025-11-04)
  **Rationale:** Requires load test environment
```

**Validator result:**
```
✅ story.md: All DoD items validated
Exit code: 0
```

---

## Validation Rules

### What Triggers CRITICAL Violations

1. **Autonomous Deferral:**
   - DoD: `[x]` (marked complete)
   - Impl: `[ ]` (actually deferred)
   - Missing: User approval marker

2. **Missing from Implementation Notes:**
   - DoD: `[x]` (marked complete)
   - Impl: NOT FOUND (no entry)

### Valid Approval Markers

Deferred items MUST have one of:

1. **Explicit approval:**
   ```markdown
   **User approved:** YES
   ```

2. **Story reference:**
   ```markdown
   Deferred to STORY-042
   ```

3. **ADR reference:**
   ```markdown
   Out of scope: ADR-023 documents scope change
   ```

4. **External blocker:**
   ```markdown
   Blocked by: API v2 release (external)
   ```

---

## Integration with DevForgeAI Workflows

### /dev Command Integration

The `/dev` slash command can call validators before invoking development skill:

```markdown
## Phase 0c: Validation
Bash(command="devforgeai check-git")
Bash(command="devforgeai validate-context")
```

### devforgeai-development Skill

The skill already has interactive checkpoints (Layer 2 - AskUserQuestion).
The CLI validator provides Layer 1 (fast deterministic checks).

**Three-layer defense:**
1. **Layer 1:** CLI validator (fast, <100ms, blocks commits)
2. **Layer 2:** AskUserQuestion (interactive, mandatory user approval)
3. **Layer 3:** deferral-validator subagent (comprehensive AI analysis)

Combined: 99% violation detection, zero autonomous deferrals possible.

---

## Troubleshooting

### "Implementation Notes section missing"

**Cause:** Story file doesn't have `## Implementation Notes` section

**Fix:**
```markdown
## Implementation Notes

**Developer:** DevForgeAI AI Agent
**Implemented:** 2025-XX-XX

- [x] DoD Item 1 - Completed: Description
- [ ] DoD Item 2 - Deferred to STORY-XXX
  **User approved:** YES
```

### "AUTONOMOUS DEFERRAL DETECTED"

**Cause:** DoD marked `[x]` but Implementation Notes shows `[ ]` without user approval

**Fix Option 1 - Add approval marker:**
```markdown
- [ ] Item - Deferred to STORY-XXX
  **User approved:** YES (via AskUserQuestion 2025-11-04)
  **Rationale:** [user-provided reason]
```

**Fix Option 2 - Complete the work:**
Change Implementation Notes to:
```markdown
- [x] Item - Completed: [description]
```

### "Referenced story STORY-XXX does not exist"

**Cause:** Justification references story that doesn't exist

**Fix:**
- Create the referenced story: `/create-story`
- OR update reference to existing story

---

## Development

### Running Tests

```bash
# Install test dependencies
pip install --break-system-packages pytest pytest-cov

# Run tests
pytest .claude/scripts/devforgeai_cli/tests/ -v

# With coverage
pytest .claude/scripts/devforgeai_cli/tests/ --cov=devforgeai_cli --cov-report=term
```

### Project Structure

```
.claude/scripts/devforgeai_cli/
├── __init__.py              # Package metadata
├── cli.py                   # CLI entry point
├── validators/
│   ├── __init__.py
│   ├── dod_validator.py     # DoD completion validator (200 lines)
│   ├── git_validator.py     # Git availability checker (107 lines)
│   └── context_validator.py # Context files validator (124 lines)
├── utils/
│   ├── __init__.py
│   ├── markdown_parser.py   # Markdown extraction (177 lines)
│   ├── yaml_parser.py       # YAML frontmatter parsing (133 lines)
│   └── story_analyzer.py    # Story analysis functions (147 lines)
└── tests/
    ├── __init__.py
    ├── test_dod_validator.py
    ├── test_git_validator.py
    ├── test_context_validator.py
    └── fixtures/
        ├── valid-story-complete.md
        ├── autonomous-deferral-story.md
        ├── valid-deferral-story.md
        └── missing-impl-notes.md
```

**Total:** ~1,100 lines of production code

---

## Contributing

### Adding New Validators

1. Create validator in `validators/`
2. Follow pattern: `validate()` returns `(is_valid, violations)`
3. Add to `cli.py` subparsers
4. Create tests in `tests/`
5. Update documentation

### Testing Validators

```python
def test_new_validator():
    from devforgeai_cli.validators.new_validator import validate_something

    is_valid, violations = validate_something(test_input)

    assert is_valid == expected_result
    assert len(violations) == expected_count
```

---

## License

See LICENSE.txt in DevForgeAI project root.

---

## Support

**Issues:** https://github.com/bankielewicz/DevForgeAI/issues
**Documentation:** See `devforgeai/specs/` in DevForgeAI repository
**Research:** `devforgeai/specs/research/validation-approaches-research-2024-2025.md`

---

**DevForgeAI CLI v0.1.0 - Preventing technical debt through automated validation**
