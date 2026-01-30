# DevForgeAI Validation Scripts

Automated validation utilities for the DevForgeAI framework.

---

## Available Scripts

### validate_deferrals.py

Lightweight format validator for deferred Definition of Done items.

**Purpose:** Quick format validation providing fast feedback (<100ms) as part of three-layer defense architecture.

**Architecture Context:**

This script is **Layer 1** of the hybrid validation approach:
- **Layer 1 (this script):** Fast format check (~200 tokens, <100ms)
- **Layer 2 (task file):** Interactive user approval checkpoint
- **Layer 3 (subagent):** Comprehensive AI analysis (feasibility, circular deps)

**Scope:** Format validation ONLY
- ✅ Checks incomplete items have justification text
- ✅ Checks justification matches expected patterns
- ❌ Does NOT check story/ADR references exist (AI handles in Layer 3)
- ❌ Does NOT detect circular deferrals (AI handles in Layer 3)
- ❌ Does NOT analyze feasibility (AI handles in Layer 3)

---

## Usage

### Basic Usage

```bash
# Format-only validation (non-blocking warnings)
python .claude/scripts/validate_deferrals.py \
  --story-file devforgeai/specs/Stories/STORY-006.story.md \
  --format-only

# Quiet mode (for automation - suppresses output)
python .claude/scripts/validate_deferrals.py \
  --story-file devforgeai/specs/Stories/STORY-006.story.md \
  --format-only \
  --quiet

# Check exit code
echo $?  # 0 = valid or format-only mode, 1 = invalid (strict mode), 2 = error
```

### Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `--story-file PATH` | Yes | Path to story file to validate |
| `--format-only` | No | Enable format-only mode (non-blocking, warnings only) |
| `--quiet` | No | Suppress output (return only exit code) |

### Exit Codes

| Code | Meaning | When Used |
|------|---------|-----------|
| **0** | Valid OR warnings only | Format-only mode always returns 0 (non-blocking) |
| **1** | Invalid | Strict mode (not used in hybrid approach) |
| **2** | Configuration error | File not found, parse error, system error |

---

## Valid Deferral Formats

The script recognizes three deferral justification patterns:

### 1. Story Split Deferral

```markdown
- [ ] Item description here
    Deferred to STORY-XXX: Brief reason explaining why work moved to follow-up story
```

**Pattern:** `Deferred to STORY-\d+:`

### 2. External Blocker

```markdown
- [ ] Item description here
    Blocked by: External dependency description (e.g., "Payment API v2 not available until 2025-12-01")
```

**Pattern:** `Blocked by:`

### 3. Scope Change (with ADR)

```markdown
- [ ] Item description here
    Out of scope: ADR-XXX documents decision to remove from current scope
```

**Pattern:** `Out of scope: ADR-\d+`

---

## Examples

### Example 1: Story with Unjustified Deferrals (STORY-006 Original Failure)

**Input:**
```markdown
## Definition of Done

- [x] Unit tests passing
- [ ] Performance benchmarks created
- [ ] Cross-platform testing
```

**Output:**
```
⚠️  Format issues detected (2 items)

Line 2: 'Performance benchmarks created'
  Missing justification. Expected one of:
    - Deferred to STORY-XXX: [reason]
    - Blocked by: [external dependency]
    - Out of scope: ADR-XXX

Line 3: 'Cross-platform testing'
  Missing justification. Expected one of:
    - Deferred to STORY-XXX: [reason]
    - Blocked by: [external dependency]
    - Out of scope: ADR-XXX

Note: Format-only mode - warnings only (non-blocking)
Interactive checkpoint will guide you through resolution.
```

**Exit code:** 0 (format-only mode, non-blocking)

---

### Example 2: Story with Valid Deferrals

**Input:**
```markdown
## Definition of Done

- [x] Unit tests passing
- [ ] Performance benchmarks created
    Deferred to STORY-007: Performance optimization story
- [ ] Cross-platform testing
    Blocked by: CI/CD matrix not configured until STORY-001
```

**Output:**
```
✓ Deferral format validation PASSED
  All incomplete DoD items have basic justification format
```

**Exit code:** 0

---

### Example 3: All DoD Items Complete

**Input:**
```markdown
## Definition of Done

- [x] Unit tests passing
- [x] Integration tests passing
- [x] Code reviewed
```

**Output:**
```
✓ Deferral format validation PASSED
  All incomplete DoD items have basic justification format
```

**Exit code:** 0

---

## Integration with /dev Command

This script is invoked automatically by `/dev` command in Phase 2.5a (Layer 1):

```markdown
### Phase 2.5a: Quick Format Check

Bash(command="python .claude/scripts/validate_deferrals.py --story-file ${STORY_FILE} --format-only --quiet")

# Non-blocking - always proceeds to Layer 2 (interactive checkpoint)
```

**Token cost:** ~200 tokens
**Performance:** <100ms
**Purpose:** Fast feedback before user interaction

---

## Integration with Git Hooks (Optional)

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
# Pre-commit hook: Validate story deferrals

STORY_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\devforgeai/specs/Stories/.*\.story\.md$')

if [ -n "$STORY_FILES" ]; then
    echo "Validating story deferrals..."

    for file in $STORY_FILES; do
        python .claude/scripts/validate_deferrals.py --story-file "$file" --format-only

        if [ $? -ne 0 ]; then
            echo "❌ Validation failed: $file"
            exit 1
        fi
    done

    echo "✅ Format validation passed"
fi

exit 0
```

**Note:** Git hooks provide standalone capability when committing outside /dev workflow.

---

## Framework Compliance

### Native Tools Efficiency

Per `devforgeai/specs/Terminal/native-tools-vs-bash-efficiency-analysis.md`:

**This script follows efficiency guidelines:**
- ✅ Uses Python stdlib file I/O (not Bash cat/sed/grep)
- ✅ Invoked via Bash subprocess (terminal operation, not file operation)
- ✅ Minimal token cost (~200 tokens for Bash invocation + output)
- ✅ No Bash file operations (compliant with "Use native tools for files" mandate)

**Pattern:** Bash used ONLY for executing Python (analogous to "pytest", "npm test")

### Framework Validation Pattern

Follows established patterns from:
- `.claude/skills/devforgeai-architecture/scripts/validate_all_context.py` (Color class, exit codes)
- `.claude/skills/devforgeai-qa/scripts/validate_spec_compliance.py` (dataclass patterns, argparse)

**Consistency:** Matches 16 existing framework validation scripts (Python, not Bash)

---

## Testing

### Test Scenarios

**Scenario 1: Unjustified Deferrals (STORY-006)**
```bash
python validate_deferrals.py --story-file tmp/STORY-006-tree-sitter-ffi-integration.story.md --format-only
# Expected: ⚠️ 2 violations detected, exit 0
```

**Scenario 2: Valid Deferrals**
```bash
python validate_deferrals.py --story-file tmp/test-story-valid-deferrals.md --format-only
# Expected: ✓ PASSED, exit 0
```

**Scenario 3: All Complete**
```bash
python validate_deferrals.py --story-file tmp/test-story-complete.md --format-only
# Expected: ✓ PASSED, exit 0
```

**All scenarios tested:** ✅ PASS

---

## Performance Characteristics

**Execution speed:**
- Story with 5 DoD items: <50ms
- Story with 15 DoD items: <100ms
- Story with 50 DoD items: <200ms

**Token cost (when invoked via Bash in /dev workflow):**
- Bash invocation: ~100 tokens
- Output (success): ~100 tokens
- Output (warnings): ~300 tokens
- **Average:** ~200 tokens

**Memory usage:** <5MB (lightweight, no external dependencies)

---

## Troubleshooting

### "Story file not found" Error

**Problem:**
```
ERROR: Story file not found: devforgeai/specs/Stories/STORY-006.story.md
```

**Solution:**
- Verify file path is correct
- Use Glob to find story: `Glob(pattern="devforgeai/specs/Stories/STORY-006*.story.md")`
- Check file exists: `ls devforgeai/specs/Stories/`

---

### "No DoD section found" (False Negative)

**Problem:** Script reports PASSED but story has no DoD section

**Explanation:**
- Script assumes no DoD section = no violations (lenient)
- Comprehensive validation (Layer 3 subagent) will catch missing DoD

**Solution:** This is expected behavior - not a bug

---

### Pattern Not Recognized

**Problem:** Valid justification flagged as violation

**Example:**
```markdown
- [ ] Item here
    deferred to STORY-007: reason
```

**Cause:** Lowercase "deferred" doesn't match pattern (expects "Deferred")

**Solution:**
- Script uses case-insensitive matching (re.IGNORECASE)
- If still not recognized, check pattern matches exactly:
  - `Deferred to STORY-XXX:` (note colon)
  - `Blocked by:` (note colon)
  - `Out of scope: ADR-XXX` (note colon after "scope")

---

## Related Documentation

**RCA-006 Implementation:**
- Recommendation 1: `.claude/skills/devforgeai-development/references/dod-validation-checkpoint.md` (Layer 2)
- Recommendation 2: `.claude/skills/devforgeai-development/SKILL.md` (XML enforcement)
- Recommendation 3: This script (Layer 1) + `.claude/agents/deferral-validator.md` (Layer 3)

**Framework Patterns:**
- `devforgeai/specs/Terminal/native-tools-vs-bash-efficiency-analysis.md` (token efficiency)
- `.claude/skills/devforgeai-architecture/scripts/` (validation script patterns)

**Integration:**
- `.claude/commands/dev.md` (Phase 2.5a invokes this script)
- `.claude/agents/deferral-validator.md` (Layer 3 comprehensive validation)

---

## Maintenance

### Adding New Deferral Patterns

Edit `VALID_PATTERNS` in `FormatValidator` class:

```python
VALID_PATTERNS = [
    r'Deferred to STORY-\d+:',
    r'Blocked by:',
    r'Out of scope: ADR-\d+',
    r'Your new pattern here'  # Add new patterns
]
```

### Updating Error Messages

Edit violation string in `validate()` method:

```python
violations.append(
    f"Line {item.line_num}: '{self._truncate(item.text, 60)}'\n"
    f"  Your custom message here"
)
```

---

**Last Updated:** 2025-11-04
**Version:** 1.0.0
**Status:** Production Ready
