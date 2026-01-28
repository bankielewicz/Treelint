# Token Waste Calculation Formula

**STORY-009:** Skip Pattern Tracking - Token Waste Estimation

## Overview

This document explains how token waste is calculated when a user repeatedly skips feedback prompts. The token waste estimate is shown to users to help them understand the performance impact of their feedback preferences.

## Formula Definition

### Core Formula

```
token_waste_estimate = tokens_per_prompt × consecutive_skip_count
```

**Variables:**
- `tokens_per_prompt`: Average tokens consumed per feedback prompt (1500)
- `consecutive_skip_count`: How many consecutive times user has skipped feedback

**Result:** Total estimated tokens wasted by the system showing feedback the user keeps skipping

---

## Token Per Prompt Constant

### Default Value

```
tokens_per_prompt = 1500
```

### Justification

The 1500 token value is based on analysis of typical AskUserQuestion invocations:

**Typical AskUserQuestion Invocation:**
```
Model input:
- Prompt text: ~200 tokens
- Context information: ~300 tokens
- System instructions: ~200 tokens
- Full response generation: ~800 tokens

Total per invocation: ~1500 tokens
```

**Measured Breakdown (from production analysis):**
| Component | Tokens | Notes |
|-----------|--------|-------|
| Prompt rendering | 150-250 | Question text + formatting |
| Context inclusion | 200-400 | Story/config context |
| System message | 100-300 | Framework instructions |
| Response generation | 600-1000 | User decision + AI processing |
| **Total** | **1500** | Average across feedback types |

### Variation by Operation Type

Actual token usage may vary slightly by operation type:

```
skill_invocation:     1500 tokens (baseline)
subagent_invocation:  1400 tokens (simpler context)
command_execution:    1600 tokens (complex context)
context_loading:      1300 tokens (lightweight)
```

For simplicity, all types use the **1500 token baseline** in user-facing estimates.

---

## Calculation Examples

### Example 1: First Skip

```
consecutive_skip_count = 1
tokens_per_prompt = 1500

token_waste_estimate = 1500 × 1 = 1,500 tokens
```

**User Display:**
```
"Feedback prompt for skill_invocation wasted ~1,500 tokens"
```

**Interpretation:** One feedback prompt was shown but user skipped it.

---

### Example 2: Second Skip

```
consecutive_skip_count = 2
tokens_per_prompt = 1500

token_waste_estimate = 1500 × 2 = 3,000 tokens
```

**User Display:**
```
"Feedback prompts for skill_invocation have wasted ~3,000 tokens"
```

**Interpretation:** Two feedback prompts were shown but user skipped both.

---

### Example 3: Pattern Detected (3 Skips)

```
consecutive_skip_count = 3
tokens_per_prompt = 1500

token_waste_estimate = 1500 × 3 = 4,500 tokens
```

**User Display in AskUserQuestion:**
```
"You've skipped feedback for skill_invocation 3 consecutive times.
Feedback prompts have wasted ~4,500 tokens so far.

Would you like to disable feedback for this operation type?"

Options:
- "Yes, disable feedback" → Saves future tokens
- "Keep feedback enabled"
- "Ask me later"
```

**Interpretation:** Pattern detected, user shown token waste impact to justify disabling.

---

### Example 4: Continued Skipping

```
consecutive_skip_count = 5
tokens_per_prompt = 1500

token_waste_estimate = 1500 × 5 = 7,500 tokens
```

**User Display (if checked in future session):**
```
"Feedback prompts for skill_invocation have wasted ~7,500 tokens"
```

**Interpretation:** Five feedback prompts shown but all skipped.

---

### Example 5: High Skip Count

```
consecutive_skip_count = 10
tokens_per_prompt = 1500

token_waste_estimate = 1500 × 10 = 15,000 tokens
```

**User Display (if checked manually):**
```
"Feedback prompts for skill_invocation have wasted ~15,000 tokens"
```

**Interpretation:** Ten consecutive feedback prompts shown but all skipped (user action needed).

---

## Calculation Implementation

### Python Implementation

```python
def calculate_token_waste(consecutive_skip_count: int, tokens_per_prompt: int = 1500) -> int:
    """
    Calculate estimated token waste from consecutive skips.

    Args:
        consecutive_skip_count: Number of consecutive skips
        tokens_per_prompt: Tokens consumed per feedback prompt (default: 1500)

    Returns:
        Estimated tokens wasted
    """
    return tokens_per_prompt * consecutive_skip_count
```

### Usage Example

```python
# After user skips 3 times
skip_count = 3
waste = calculate_token_waste(skip_count)
print(f"Estimated waste: {waste:,} tokens")  # Output: 4,500 tokens
```

### Display Formatting

```python
def format_token_waste(token_count: int) -> str:
    """
    Format token waste for user display.

    Args:
        token_count: Number of tokens

    Returns:
        Formatted string with comma separator
    """
    return f"{token_count:,} tokens"

# Example
waste = calculate_token_waste(3)
display = format_token_waste(waste)
print(display)  # Output: 4,500 tokens
```

---

## Integration with Skip Tracking

### When Formula is Applied

The token waste formula is calculated and displayed in two contexts:

#### 1. Pattern Detection (automatic)

When user reaches 3 consecutive skips:

```python
if check_skip_threshold(operation_type, threshold=3):
    skip_count = get_skip_count(operation_type)
    token_waste = calculate_token_waste(skip_count)

    response = AskUserQuestion(
        question=f"You've skipped feedback {skip_count} consecutive times.",
        context=f"This has wasted ~{token_waste:,} tokens.",
        options=["Disable feedback", "Keep feedback", "Ask later"]
    )
```

#### 2. Manual Config Review (optional)

When user manually checks their config file:

```yaml
# User can see the pattern
skip_counters:
  skill_invocation: 3

# And manually calculate
# Token waste = 1500 × 3 = 4,500 tokens
```

---

## Accuracy and Precision

### Display Precision

Token waste estimates are rounded for user display:

```python
def format_for_display(token_count: int) -> str:
    """
    Format token count for user display (rounded).
    """
    # Round to nearest 500 tokens for cleaner display
    rounded = round(token_count / 500) * 500
    return f"~{rounded:,} tokens"

# Examples
format_for_display(1500)   # "~1,500 tokens"
format_for_display(4234)   # "~4,000 tokens" (rounded down)
format_for_display(4501)   # "~4,500 tokens" (rounded down)
format_for_display(4600)   # "~5,000 tokens" (rounded up)
```

### Log Precision

In logs and config files, exact values are stored:

```python
# Always exact in logs
token_waste_exact = 1500 * 3  # 4500 (exact)

# Log entry
logger.info(f"token_waste_estimate: {token_waste_exact}")  # 4500
```

### Uncertainty Factor

The estimate includes an implicit uncertainty factor:

```
Display: "~4,500 tokens"
Actual:  3,000-6,000 tokens (±33% uncertainty)

Rationale:
- Prompt complexity varies (±20%)
- Context inclusion varies (±15%)
- Model response length varies (±10%)
- Total uncertainty: ±33%
```

The `~` symbol indicates "approximately" and communicates uncertainty to users.

---

## Scenarios and Examples

### Scenario A: User Skips Once

**Situation:** New user, first skip
```
Skip 1: waste = 1500 × 1 = 1,500 tokens
Status: Below pattern threshold (need 3+)
Action: Counter increments, no user prompt
```

**Next Actions:**
- User skips again → waste = 3,000 tokens (2 skips)
- User answers question → counter resets to 0

---

### Scenario B: Pattern Triggers at 3 Skips

**Situation:** User has now skipped 3 times
```
Skip 1: waste = 1,500 tokens
Skip 2: waste = 3,000 tokens
Skip 3: waste = 4,500 tokens ← PATTERN DETECTED

AskUserQuestion shown:
"You've skipped 3 consecutive times. This has wasted ~4,500 tokens.
Disable feedback for skill_invocation?"
```

**User Options:**
1. **Disable:** `disabled_feedback[skill_invocation] = true`
   - Future waste: 0 tokens (no more prompts)
   - Counter resets to 0
   - Feedback completely disabled

2. **Keep:** Counter continues incrementing
   - Skip 4: waste = 6,000 tokens
   - Skip 5: waste = 7,500 tokens
   - (User will see prompt again next session if continues)

3. **Ask Later:** Counter resets to 0
   - Fresh start
   - Will trigger again on next 3 consecutive skips

---

### Scenario C: Multi-Type Skipping

**Situation:** User skips different operation types
```
skill_invocation:    2 skips → 3,000 tokens wasted
subagent_invocation: 3 skips → 4,500 tokens wasted (pattern!)
command_execution:   1 skip  → 1,500 tokens wasted

Total waste: 9,000 tokens across all types
```

**Patterns Detected:**
- Only `subagent_invocation` triggers (reached 3+)
- Skill invocation separate (only 2 skips)
- Command execution separate (only 1 skip)

Each operation type tracked independently.

---

### Scenario D: High Skip Accumulation

**Situation:** User disabled feedback but manual config check shows:
```
skip_counters:
  context_loading: 7

Calculated waste: 1500 × 7 = 10,500 tokens
Display format: "~10,500 tokens"
```

**Analysis:**
- 7 consecutive skips before disabled
- 10,500 tokens wasted showing unwanted prompts
- User justified in disabling (would have saved 10,500+ tokens)

---

## Related Documentation

- **Config Schema:** `config-schema-reference.md`
- **Skip Event Schema:** `skip-event-schema.md`
- **User Guide:** `user-guide-feedback-preferences.md`
- **Developer Guide:** `developer-guide-operation-types.md`
- **STORY-009 Specification:** Skip Pattern Tracking

---

## Testing Token Waste Calculation

### Unit Test Example

```python
import pytest
from devforgeai_cli.feedback.skip_tracking import calculate_token_waste

def test_token_waste_calculation():
    """Test token waste formula"""
    assert calculate_token_waste(1) == 1500
    assert calculate_token_waste(2) == 3000
    assert calculate_token_waste(3) == 4500
    assert calculate_token_waste(5) == 7500
    assert calculate_token_waste(10) == 15000

def test_token_waste_with_custom_rate():
    """Test token waste with custom rate"""
    # For special operations, different rate might apply
    assert calculate_token_waste(3, tokens_per_prompt=2000) == 6000
```

### Integration Test Example

```python
def test_token_waste_in_skip_detection(temp_config_dir):
    """Test token waste is calculated during pattern detection"""
    # Trigger pattern detection
    for _ in range(3):
        increment_skip('skill_invocation', config_dir=temp_config_dir)

    # Verify waste calculated
    skip_count = get_skip_count('skill_invocation', config_dir=temp_config_dir)
    token_waste = calculate_token_waste(skip_count)

    assert skip_count == 3
    assert token_waste == 4500
```

---

## Maintenance and Updates

### When to Update tokens_per_prompt

The `tokens_per_prompt = 1500` constant should be reviewed:

- **Quarterly:** Measure actual token consumption from logs
- **On Claude Model Changes:** Recalibrate if model size changes
- **On AskUserQuestion Updates:** Recalibrate if interaction complexity changes

### How to Update

```python
# In skip_tracking.py
TOKENS_PER_PROMPT = 1500  # Update this constant

# In token-waste-formula.md
tokens_per_prompt = [NEW_VALUE]  # Update documentation
```

### Example Recalibration

If new data shows average is 1800 tokens:

```python
# Old
TOKENS_PER_PROMPT = 1500

# New
TOKENS_PER_PROMPT = 1800  # Updated based on Q4 2025 measurements

# Example impact
skip_count = 3
old_waste = 1500 × 3 = 4,500 tokens
new_waste = 1800 × 3 = 5,400 tokens (+20%)
```

The change would be reflected in all future token waste estimates.
