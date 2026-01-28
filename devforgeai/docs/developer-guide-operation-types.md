# Developer Guide: Adding New Operation Types

**STORY-009:** Skip Pattern Tracking - Developer Guide for Extending Operation Types

## Overview

This guide explains how to add new operation types to the skip pattern tracking system. Operation types define which kinds of system operations can trigger feedback and be tracked for skipping patterns.

## Current Operation Types

The system currently tracks 4 operation types:

```python
VALID_OPERATION_TYPES = [
    'skill_invocation',      # When a skill is invoked
    'subagent_invocation',   # When a subagent is invoked
    'command_execution',     # When a command is executed
    'context_loading',       # When context files are loaded
]
```

---

## When to Add a New Operation Type

Add a new operation type when:

1. **New feedback interaction introduced:** You're adding a new place where the system prompts users
2. **Feedback can be independently disabled:** The feedback is separate from existing types
3. **Users might skip repeatedly:** Historical data suggests this interaction gets skipped
4. **Cross-session tracking needed:** Skips persist across terminal restarts

### Don't Add When:

- Feedback is part of existing operation type (use existing instead)
- One-time feedback (users won't skip multiple times)
- Not user-facing (internal system operations)

---

## Step-by-Step: Adding a New Operation Type

### Step 1: Define the Operation Type

Choose a descriptive name using `snake_case`:

**Examples:**
- ✅ `model_context_selection` (good - descriptive, lowercase, snake_case)
- ✅ `documentation_generation` (good - clear purpose)
- ❌ `ModelContext` (bad - CamelCase not allowed)
- ❌ `model-context` (bad - hyphen not allowed)
- ❌ `very_long_operation_type_with_too_many_words` (bad - over-complicated)

**Format:** `^[a-z_]+$` (lowercase letters and underscores only)

---

### Step 2: Update the Whitelist

**File:** `.claude/scripts/devforgeai_cli/feedback/skip_tracking.py`

**Find:**
```python
VALID_OPERATION_TYPES = [
    'skill_invocation',
    'subagent_invocation',
    'command_execution',
    'context_loading',
]
```

**Add your type:**
```python
VALID_OPERATION_TYPES = [
    'skill_invocation',
    'subagent_invocation',
    'command_execution',
    'context_loading',
    'model_context_selection',  # NEW
]
```

**Validation code:** (automatically validates against this list)
```python
def validate_operation_type(operation_type: str) -> bool:
    """
    Validate operation type is in whitelist.

    Args:
        operation_type: Type to validate

    Returns:
        True if valid, False otherwise
    """
    if not isinstance(operation_type, str):
        return False

    if operation_type not in VALID_OPERATION_TYPES:
        raise ValueError(
            f"Invalid operation type: {operation_type}. "
            f"Must be one of: {', '.join(VALID_OPERATION_TYPES)}"
        )

    return True
```

---

### Step 3: Update Config Schema

**File:** `devforgeai/config/feedback-preferences.yaml`

The config file automatically creates entries for new operation types. When the system encounters a skip for a new type, it adds:

```yaml
skip_counters:
  model_context_selection: 0

disabled_feedback:
  model_context_selection: false

disable_reasons:
  model_context_selection: null
```

**No manual schema update needed** - the code handles this automatically:

```python
def _load_config(config_file: Path) -> dict:
    """Load config from YAML file."""
    if not config_file.exists():
        return {
            'skip_counts': {},  # Automatically initialized
            'disabled_feedback': {},
            'disable_reasons': {},
        }
    # ... rest of loading
```

**Optional: Update documentation**
- Add to `config-schema-reference.md` for user reference
- Update operation type table with new type
- Add example usage

---

### Step 4: Update Integration Points

**Find all places that call skip tracking functions:**

```bash
grep -r "increment_skip\|get_skip_count\|check_skip_threshold" \
  .claude/scripts/devforgeai_cli --include="*.py"
```

**Check each location and test with new type:**

Example: Skill invocation code
```python
# Before
def invoke_skill(skill_name):
    if check_skip_threshold('skill_invocation'):
        # Show AskUserQuestion
        pass

# After (if adding feedback elsewhere)
def load_model_context():
    if check_skip_threshold('model_context_selection'):
        # Show AskUserQuestion
        pass
```

---

### Step 5: Write Tests

Add comprehensive tests for the new operation type:

**Unit Tests:**

```python
import pytest
from devforgeai_cli.feedback.skip_tracking import (
    increment_skip,
    get_skip_count,
    check_skip_threshold,
    reset_skip_count,
)

class TestNewOperationType:
    """Test new operation type"""

    @pytest.fixture
    def temp_config_dir(self):
        import tempfile
        import shutil
        from pathlib import Path

        temp_dir = tempfile.mkdtemp()
        yield Path(temp_dir)
        shutil.rmtree(temp_dir)

    def test_increment_new_operation_type(self, temp_config_dir):
        """
        GIVEN new operation type
        WHEN increment_skip is called
        THEN counter increments
        """
        # Arrange
        operation_type = 'model_context_selection'

        # Act
        count1 = increment_skip(operation_type, config_dir=temp_config_dir)
        count2 = increment_skip(operation_type, config_dir=temp_config_dir)

        # Assert
        assert count1 == 1
        assert count2 == 2

    def test_new_type_independent_from_others(self, temp_config_dir):
        """
        GIVEN multiple operation types
        WHEN one type is incremented
        THEN other types remain unaffected
        """
        # Arrange
        type1 = 'skill_invocation'
        type2 = 'model_context_selection'

        # Act
        increment_skip(type1, config_dir=temp_config_dir)
        increment_skip(type1, config_dir=temp_config_dir)
        increment_skip(type2, config_dir=temp_config_dir)

        # Assert
        assert get_skip_count(type1, config_dir=temp_config_dir) == 2
        assert get_skip_count(type2, config_dir=temp_config_dir) == 1

    def test_pattern_detection_for_new_type(self, temp_config_dir):
        """
        GIVEN 3+ consecutive skips
        WHEN check_skip_threshold is called
        THEN returns True
        """
        # Arrange
        operation_type = 'model_context_selection'
        for _ in range(3):
            increment_skip(operation_type, config_dir=temp_config_dir)

        # Act
        threshold_reached = check_skip_threshold(operation_type, config_dir=temp_config_dir)

        # Assert
        assert threshold_reached is True

    def test_new_type_persists_across_sessions(self, temp_config_dir):
        """
        GIVEN skip count for new type
        WHEN new session reads config
        THEN skip count persists
        """
        # Arrange
        operation_type = 'model_context_selection'
        increment_skip(operation_type, config_dir=temp_config_dir)
        increment_skip(operation_type, config_dir=temp_config_dir)

        # Act - simulate new session (re-read from disk)
        count = get_skip_count(operation_type, config_dir=temp_config_dir)

        # Assert
        assert count == 2
```

**Integration Tests:**

```python
def test_new_type_with_disable_feedback(temp_config_dir):
    """
    GIVEN new operation type with skip pattern
    WHEN user disables feedback
    THEN feedback is disabled for that type
    """
    from devforgeai_cli.feedback.skip_tracking import (
        increment_skip,
        check_skip_threshold,
        disable_feedback,
    )

    operation_type = 'model_context_selection'

    # Trigger pattern
    for _ in range(3):
        increment_skip(operation_type, config_dir=temp_config_dir)

    # Verify pattern detected
    assert check_skip_threshold(operation_type, config_dir=temp_config_dir)

    # Disable feedback
    disable_feedback(operation_type, config_dir=temp_config_dir)

    # Verify disabled
    from devforgeai_cli.feedback.skip_tracking import is_feedback_disabled
    assert is_feedback_disabled(operation_type, config_dir=temp_config_dir)
```

**Run tests:**
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/ -v
```

---

### Step 6: Update Documentation

**Files to update:**

1. **config-schema-reference.md**
   - Add new operation type to valid values table
   - Update schema example with new type

2. **skip-event-schema.md**
   - Add new type to operation_type allowed values
   - Example event with new type

3. **user-guide-feedback-preferences.md**
   - Add section explaining new operation type
   - When it's triggered
   - Common reason to disable

4. **This file (developer-guide-operation-types.md)**
   - Update current operation types list at top
   - Add to examples

---

### Step 7: Validation Testing

**Manual validation checklist:**

```bash
# 1. Test direct skip increment
python3 << 'EOF'
from devforgeai_cli.feedback.skip_tracking import increment_skip
from pathlib import Path
import tempfile

temp_dir = Path(tempfile.mkdtemp())
result = increment_skip('model_context_selection', config_dir=temp_dir)
print(f"Skip increment result: {result}")  # Should be 1
EOF

# 2. Test config file was created
ls -l devforgeai/config/feedback-preferences.yaml

# 3. Check config contains new type
cat devforgeai/config/feedback-preferences.yaml | grep model_context_selection

# 4. Run full test suite
pytest .claude/scripts/devforgeai_cli/tests/feedback/ -v --tb=short
```

---

### Step 8: Update Version / Create ADR

If adding operation types is significant change:

**Update schema version:**
```yaml
---
version: "1.1"  # Was 1.0, increment minor version
created_at: "..."
last_updated: "..."
---
```

**Create ADR if architectural impact:**

File: `devforgeai/adrs/ADR-NNN-add-model-context-selection-feedback.md`

```markdown
# ADR-XXX: Add model_context_selection Feedback Type

## Context
New feedback was introduced for model context selection during [feature].
Users skip this feedback repeatedly, indicating it should be tracked.

## Decision
Add `model_context_selection` as 5th operation type to skip tracking system.

## Rationale
- Feedback is independent and user-facing
- Historical data shows repeated skipping
- Users benefit from ability to disable

## Consequences
- Config schema increments to v1.1
- New operation type in whitelist
- 20 new tests added
- Backward compatible (old configs auto-updated)

## Alternatives Considered
1. Bundle with existing type (rejected - different feedback context)
2. Make non-disableable (rejected - users want control)
```

---

## Code Example: Using New Operation Type

### Calling Skip Tracking from Skill Code

```python
# In devforgeai_cli/feedback/adaptive_questioning_engine.py

def should_show_feedback_for_model_context_selection():
    """Check if feedback should be shown for model context selection"""
    from devforgeai_cli.feedback.skip_tracking import (
        is_feedback_disabled,
        check_skip_threshold,
    )

    operation_type = 'model_context_selection'

    # Check if user disabled this feedback type
    if is_feedback_disabled(operation_type):
        return False  # Feedback disabled

    # Feedback enabled
    return True

def track_model_context_selection_skip():
    """Record that user skipped model context selection feedback"""
    from devforgeai_cli.feedback.skip_tracking import (
        increment_skip,
        check_skip_threshold,
    )

    operation_type = 'model_context_selection'

    # Increment counter
    new_count = increment_skip(operation_type)

    # Check if pattern reached
    if check_skip_threshold(operation_type, threshold=3):
        # Show AskUserQuestion with disable option
        show_pattern_detected_dialog(operation_type, new_count)
```

### Implementation in Code

```python
# In skill/subagent that shows feedback

def show_model_context_selection():
    """Show model context selection feedback"""
    from devforgeai_cli.feedback.skip_tracking import is_feedback_disabled

    # Check if disabled
    if is_feedback_disabled('model_context_selection'):
        return  # Skip feedback entirely

    # Show AskUserQuestion
    response = AskUserQuestion(
        question="Select model context",
        options=["Option A", "Option B", "Option C"]
    )

    # Handle response
    if response == "skip":
        track_model_context_selection_skip()

    return response
```

---

## Validation Rules for New Types

When implementing new type, ensure:

### 1. Whitelist Validation
```python
# Must be in VALID_OPERATION_TYPES
if operation_type not in VALID_OPERATION_TYPES:
    raise ValueError(f"Invalid operation type: {operation_type}")
```

### 2. Format Validation
```python
import re

# Must match pattern: lowercase letters and underscores
pattern = r'^[a-z_]+$'
if not re.match(pattern, operation_type):
    raise ValueError(f"Invalid format: {operation_type}")
```

### 3. Length Validation
```python
# Should be reasonable length (8-40 characters)
if len(operation_type) < 8 or len(operation_type) > 40:
    raise ValueError(f"Operation type name out of range: {operation_type}")
```

---

## Testing Checklist

When adding new operation type:

- [ ] Whitelist updated (VALID_OPERATION_TYPES)
- [ ] Unit tests written (10+)
- [ ] Integration tests written (5+)
- [ ] Cross-session persistence tested
- [ ] Independence from other types verified
- [ ] Disable/re-enable tested
- [ ] Token waste calculation verified
- [ ] Config file updated and validated
- [ ] Documentation updated
- [ ] Manual validation completed
- [ ] All tests passing: `pytest ... -v`
- [ ] Coverage >95%: `pytest ... --cov`

---

## Common Issues and Solutions

### Issue: New type not recognized

**Symptom:** ValueError: "Invalid operation type"

**Cause:** Whitelist not updated

**Solution:** Add to VALID_OPERATION_TYPES in skip_tracking.py

---

### Issue: Config file not updating for new type

**Symptom:** New type in whitelist but not in config file

**Cause:** Config auto-update happens on next skip

**Solution:** Trigger skip: `increment_skip('new_type')`

---

### Issue: Tests failing for new type

**Symptom:** AssertionError in pattern detection test

**Cause:** Threshold default is 3, make sure test increments 3+ times

**Solution:**
```python
# Wrong - only increments 2 times
for _ in range(2):
    increment_skip(new_type)
assert check_skip_threshold(new_type)  # FAILS

# Right - increments 3 times
for _ in range(3):
    increment_skip(new_type)
assert check_skip_threshold(new_type)  # PASSES
```

---

## Related Documentation

- **Config Schema:** `config-schema-reference.md`
- **Skip Event Schema:** `skip-event-schema.md`
- **User Guide:** `user-guide-feedback-preferences.md`
- **Token Formula:** `token-waste-formula.md`
- **STORY-009:** Skip Pattern Tracking specification

---

## Support and Questions

For questions about adding operation types:
1. Review this guide's examples
2. Check existing implementations (skill_invocation, etc.)
3. Run test suite to verify
4. Consult STORY-009 specification for more context
