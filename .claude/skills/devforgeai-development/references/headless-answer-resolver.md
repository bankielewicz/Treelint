# Headless Answer Resolver Reference

> Integration guide for resolving AskUserQuestion prompts in headless CI/CD mode (STORY-098)

---

## Overview

The `HeadlessAnswerResolver` service automatically resolves `AskUserQuestion` prompts when running in headless mode (CI pipelines, `-p` flag execution). This enables `/dev` and `/qa` workflows to run unattended.

---

## Quick Start

### Check if Headless Mode Active

```python
from devforgeai_cli.headless import HeadlessAnswerResolver

resolver = HeadlessAnswerResolver.get_instance()

if resolver.is_headless_mode():
    # Running in CI - use configured answers
    answer = resolver.resolve(prompt_text, options)
else:
    # Interactive mode - show prompt to user
    answer = AskUserQuestion(...)
```

### Configuration File Location

The resolver searches for `ci-answers.yaml` in these locations (in order):

1. `devforgeai/config/ci-answers.yaml` (preferred)
2. `devforgeai/config/ci/ci-answers.yaml` (fallback)
3. `~/devforgeai/config/ci-answers.yaml` (user home)

---

## Integration Patterns

### Pattern 1: Skill with AskUserQuestion

When a skill needs to ask the user a question:

```markdown
# In SKILL.md

**Before asking question, check headless mode:**

IF HeadlessAnswerResolver.is_headless_mode():
    answer = HeadlessAnswerResolver.resolve(
        prompt_text="What is the story priority?",
        options=["High", "Medium", "Low"]
    )
    IF answer is None:
        # Skip strategy - use default
        answer = "High"
ELSE:
    # Interactive mode
    AskUserQuestion(
        question="What is the story priority?",
        options=[
            {label: "High", description: "Critical path"},
            {label: "Medium", description: "Normal priority"},
            {label: "Low", description: "Nice to have"}
        ]
    )
```

### Pattern 2: Command with User Confirmation

```markdown
# In command.md

**Phase X: Deferral Approval**

IF HeadlessAnswerResolver.is_headless_mode():
    config = HeadlessAnswerResolver.load_configuration()
    IF config.defaults.unknown_prompt == "fail":
        # CI mode - no deferrals allowed
        HALT with error: "Deferrals not allowed in headless mode"
    ELSE:
        # Use configured deferral strategy
        proceed = HeadlessAnswerResolver.resolve(
            "Do you approve this deferral?",
            ["Yes", "No - implement now"]
        )
ELSE:
    # Interactive mode
    AskUserQuestion(...)
```

---

## Configuration Format

### Nested Format (Recommended)

```yaml
# devforgeai/config/ci-answers.yaml
headless_mode:
  enabled: true
  fail_on_unanswered: true  # AC#3
  log_matches: true         # AC#2

answers:
  priority:
    pattern: "What is the story priority"
    answer: "High"
  deferral_approval:
    pattern: "Do you approve this deferral"
    answer: "No - implement now"

defaults:
  unknown_prompt: fail  # fail | first_option | skip
```

### Pattern Syntax

- Patterns use Python regex (case-insensitive)
- First matching pattern wins
- Use `.*` for wildcards: `"Tests failed.*proceed"`

---

## Acceptance Criteria Coverage

| AC | Description | How It's Handled |
|----|-------------|------------------|
| AC#1 | Config file loading | `HeadlessAnswerResolver.load_configuration()` |
| AC#2 | Pattern matching with logging | `PromptPatternMatcher.match()` with `log_matches=true` |
| AC#3 | Fail-on-unanswered mode | `fail_on_unanswered: true` raises `HeadlessResolutionError` |
| AC#4 | Default answer fallback | `defaults.unknown_prompt: first_option` |
| AC#5 | Validation on load | `load_config()` validates YAML, required fields, enum values |

---

## Error Handling

### HeadlessResolutionError

Raised when `fail_on_unanswered: true` and no matching answer:

```
HeadlessResolutionError: Headless mode: No answer configured for prompt 'Unknown question'
```

**Resolution:**
1. Add pattern to `ci-answers.yaml`
2. Or set `fail_on_unanswered: false` with `defaults.unknown_prompt: first_option`

### ConfigurationError

Raised for invalid configuration:

```
ConfigurationError: YAML parsing error: ... (line 15)
ConfigurationError: Configuration missing required 'defaults' section
```

**Resolution:**
1. Fix YAML syntax errors
2. Add missing required sections

---

## Performance

| Metric | Target | Implementation |
|--------|--------|----------------|
| Config load | <100ms | YAML parsing with caching |
| Pattern match | <10ms | Pre-compiled regex patterns |

---

## Backward Compatibility

The resolver supports legacy flat format with auto-migration:

```yaml
# Legacy format (deprecated)
test_failure_action: fix-implementation
priority_default: high
```

This logs a deprecation warning and auto-converts to nested format internally.

---

## Testing

### Unit Tests

```bash
python -m pytest tests/headless/ -v
```

### Integration Test

```bash
# Set headless mode
export DEVFORGEAI_HEADLESS=true

# Run /dev with configured answers
claude -p "/dev STORY-001"
```

---

## Files Reference

| File | Purpose |
|------|---------|
| `.claude/scripts/devforgeai_cli/headless/answer_resolver.py` | Main service |
| `.claude/scripts/devforgeai_cli/headless/pattern_matcher.py` | Pattern matching |
| `.claude/scripts/devforgeai_cli/headless/answer_models.py` | Configuration models |
| `devforgeai/config/ci/ci-answers.yaml.example` | Configuration template |
| `tests/headless/` | Test suite |

---

**Story:** STORY-098 - Headless Mode Answer Configuration
**Epic:** EPIC-010 - Parallel Story Development
