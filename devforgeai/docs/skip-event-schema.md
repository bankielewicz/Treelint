# Skip Event Schema

**STORY-009:** Skip Pattern Tracking - Skip Event Data Structure

## Overview

This document defines the skip event data structure used by the skip pattern tracking system. Skip events represent individual instances where a user skips feedback, and include metadata about the skip action and its consequences.

## Skip Event Structure

```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2025-11-09T14:45:32.123456Z",
  "operation_type": "skill_invocation",
  "skip_action": "skip_question",
  "consecutive_count": 3,
  "pattern_detected": true,
  "token_waste_estimate": 4500,
  "user_action": "disabled_feedback"
}
```

---

## Field Definitions

### event_id

**Type:** UUID (Universally Unique Identifier)

**Format:** Standard UUID v4 format
```
550e8400-e29b-41d4-a716-446655440000
```

**Purpose:** Unique identifier for this skip event (audit trail)

**Generation:**
```python
import uuid
event_id = uuid.uuid4()
```

**Use Cases:**
- Correlating skip events across log files
- Deduplication (preventing duplicate records)
- Audit trail tracking
- Cross-session event correlation

**Constraints:**
- Must be unique within the system
- Must be UUID v4 (random)
- Never reused (even if event is deleted)
- Immutable after creation

---

### timestamp

**Type:** ISO 8601 UTC Timestamp

**Format:**
```
2025-11-09T14:45:32.123456Z
```

**Pattern:** `YYYY-MM-DDTHH:MM:SS.ffffffZ`

**Purpose:** When the skip event occurred (precision to microseconds)

**Generation:**
```python
from datetime import datetime
timestamp = datetime.utcnow().isoformat() + 'Z'
```

**Use Cases:**
- Temporal analysis (when users skip most frequently)
- Cross-session tracking (maintains chronological order)
- Trend analysis (skip patterns over time)
- Audit compliance (when was a decision made)

**Constraints:**
- Must be UTC (Z suffix indicates UTC)
- Precision: Microseconds (6 decimal places)
- Immutable after creation
- Used for sorting and correlation

---

### operation_type

**Type:** String (enum-like, validated against whitelist)

**Allowed Values:**
```
"skill_invocation"
"subagent_invocation"
"command_execution"
"context_loading"
```

**Purpose:** Which type of operation triggered the feedback prompt that was skipped

**Semantics:**
- `skill_invocation`: User skipped feedback during skill execution
- `subagent_invocation`: User skipped feedback during subagent invocation
- `command_execution`: User skipped feedback during command execution
- `context_loading`: User skipped feedback during context file loading

**Validation Rules:**
1. Must be one of 4 allowed values (whitelist)
2. Case-sensitive: lowercase only
3. Cannot be null or empty string
4. Must match config file structure

**Storage:** Stored in `devforgeai/config/feedback-preferences.yaml` under `skip_counters[operation_type]`

**Extending:** To add new operation types:
1. Update this whitelist (4 values → 5 values)
2. Update schema version in config files
3. Add validation rules in code
4. Update documentation

---

### skip_action

**Type:** String (enum)

**Allowed Values:**
```
"skip_all"       # User skipped all feedback prompts
"skip_question"  # User skipped a specific question
```

**Purpose:** Type of skip action the user performed

**Semantics:**
- `skip_all`: User selected "Skip all feedback of this type" (AskUserQuestion)
- `skip_question`: User selected "Skip this question" or used default behavior

**Validation Rules:**
1. Must be one of 2 allowed values
2. Case-sensitive: lowercase with underscore
3. Cannot be null or empty

**Usage:**
- `skip_all` indicates strong user preference to disable feedback
- `skip_question` may be temporary/contextual
- Pattern detection triggered regardless of action type

**Example Sequence:**
```
Event 1: skip_action = "skip_question"  → count = 1
Event 2: skip_action = "skip_question"  → count = 2
Event 3: skip_action = "skip_all"       → count = 3 → Pattern detected!
```

---

### consecutive_count

**Type:** Integer

**Range:** 1-100

**Purpose:** Number of consecutive skips for this operation_type at time of skip event

**Semantics:**
- `1`: First skip (no pattern yet)
- `2`: Second consecutive skip
- `3`: Third consecutive skip (THRESHOLD REACHED, pattern detected)
- `4+`: Continued skipping

**Update Rules:**
1. Incremented by 1 on each skip for same operation_type
2. Reset to 1 if user answers a feedback (breaks consecutive sequence)
3. Maintained across sessions (survives terminal restart)
4. Independent per operation_type (skill skips don't affect subagent skips)

**Example Scenario:**
```
Time 1: User skips skill_invocation feedback → consecutive_count = 1
Time 2: User skips skill_invocation feedback → consecutive_count = 2
Time 3: User answers subagent_invocation feedback → doesn't affect skill counter
Time 4: User skips skill_invocation feedback → consecutive_count = 3 (consecutive!)
```

**Pattern Detection Trigger:**
- When `consecutive_count >= 3` → Pattern detected
- Trigger point: At count = 3 (first crossing of threshold)
- Shown to user: AskUserQuestion with "Disable feedback for [operation_type]?" option

---

### pattern_detected

**Type:** Boolean

**Allowed Values:**
```
true   # 3+ consecutive skips reached, pattern detected
false  # Threshold not reached yet
```

**Purpose:** Whether pattern detection was triggered for this skip event

**Semantics:**
- `true`: This skip triggered the pattern (consecutive_count reached 3)
- `false`: This skip occurred but pattern not yet triggered

**Trigger Conditions:**
1. `consecutive_count >= 3` (explicitly checked)
2. First time reaching threshold for this operation_type in this session
3. Pattern triggers only once per session (not on every skip after 3)

**User Experience:**
- When `pattern_detected = true`:
  - AskUserQuestion appears
  - Options: "Disable feedback", "Keep feedback", "Ask me later"
  - System waits for user decision (blocks progress until answered)

- When `pattern_detected = false`:
  - No AskUserQuestion
  - Skip logged silently
  - Operation continues normally

**Example Sequence:**
```
Skip 1: consecutive_count = 1, pattern_detected = false
Skip 2: consecutive_count = 2, pattern_detected = false
Skip 3: consecutive_count = 3, pattern_detected = true   ← First true!
Skip 4: consecutive_count = 4, pattern_detected = false  ← Already triggered
Skip 5: consecutive_count = 5, pattern_detected = false  ← Already triggered
```

---

### token_waste_estimate

**Type:** Integer (tokens)

**Unit:** Claude tokens (from /context command)

**Range:** 0-50000 (practical limits)

**Purpose:** Estimated tokens wasted by prompting user with feedback they keep skipping

**Calculation Formula:**
```
tokens_per_prompt × skip_count = token_waste_estimate
```

**Parameters:**
- `tokens_per_prompt = 1500` (default, based on AskUserQuestion analysis)
- `skip_count = consecutive_count`

**Examples:**
```
Skip 1: 1500 × 1 = 1,500 tokens
Skip 2: 1500 × 2 = 3,000 tokens
Skip 3: 1500 × 3 = 4,500 tokens (pattern detected!)
Skip 5: 1500 × 5 = 7,500 tokens
Skip 10: 1500 × 10 = 15,000 tokens
```

**Display to User:**
```
"Feedback prompts for [operation_type] have wasted ~{token_waste_estimate} tokens.
Disable feedback to reclaim these tokens for model operations."
```

**Precision:**
- Rounded to nearest 500 tokens for user display
- Exact value in logs (1500 × count)
- Used in AskUserQuestion context string

**Calculation Code:**
```python
tokens_per_prompt = 1500  # Average for AskUserQuestion
token_waste_estimate = tokens_per_prompt * consecutive_count
```

---

### user_action (Optional)

**Type:** String or null

**Allowed Values:**
```
null                  # No user action taken
"disabled_feedback"   # User chose to disable feedback
"kept_feedback"       # User chose to keep feedback
"ask_later"           # User deferred decision
```

**Purpose:** User's response to the pattern detection AskUserQuestion

**When Populated:**
- Only populated when `pattern_detected = true`
- Otherwise: `null` (no user action required)

**Semantics:**
- `disabled_feedback`: User confirmed "Disable feedback for [operation_type]"
  - Result: `disabled_feedback[operation_type] = true` in config
  - Result: Skip counter resets to 0
  - Result: No more prompts for this operation_type

- `kept_feedback`: User chose "Keep feedback enabled"
  - Result: Skip counter continues counting
  - Result: Prompts continue (user may skip more)
  - Result: If pattern continues, suggestion may appear again in future session

- `ask_later`: User chose "Ask me later"
  - Result: Skip counter resets to 0 (fresh start)
  - Result: User gets prompted again on next 3+ skips

---

## Complete Example Events

### Example 1: First Skip (No Pattern)

```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2025-11-09T14:45:32.123456Z",
  "operation_type": "skill_invocation",
  "skip_action": "skip_question",
  "consecutive_count": 1,
  "pattern_detected": false,
  "token_waste_estimate": 1500,
  "user_action": null
}
```

**Interpretation:** User skipped skill_invocation feedback for first time (1 skip, no pattern yet, 1500 tokens wasted)

---

### Example 2: Third Skip - Pattern Detected

```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440001",
  "timestamp": "2025-11-09T14:45:45.654321Z",
  "operation_type": "skill_invocation",
  "skip_action": "skip_all",
  "consecutive_count": 3,
  "pattern_detected": true,
  "token_waste_estimate": 4500,
  "user_action": "disabled_feedback"
}
```

**Interpretation:** User's 3rd consecutive skip of skill_invocation feedback. Pattern detected, AskUserQuestion shown, user chose to disable feedback.

---

### Example 3: Skip After Re-enabling Feedback

```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440002",
  "timestamp": "2025-11-09T15:00:00.000000Z",
  "operation_type": "skill_invocation",
  "skip_action": "skip_question",
  "consecutive_count": 1,
  "pattern_detected": false,
  "token_waste_estimate": 1500,
  "user_action": null
}
```

**Interpretation:** After user re-enabled skill_invocation feedback, counter reset to 0. This skip is the first new skip, so consecutive_count = 1 again.

---

## Event Logging

### Log Location

```
devforgeai/logs/skip-pattern-detection.log
```

### Log Format

```
2025-11-09T14:45:32.123456Z [INFO] Skip event recorded
  event_id: 550e8400-e29b-41d4-a716-446655440000
  operation_type: skill_invocation
  consecutive_count: 3
  pattern_detected: true
  token_waste_estimate: 4500
  user_action: disabled_feedback
```

### Log Levels

- **DEBUG:** Every skip (verbose)
- **INFO:** Pattern detected (normal operation)
- **WARNING:** User re-enables after disabled (user attention)
- **ERROR:** Config corruption (action needed)

---

## Related Documentation

- **Config Schema:** `config-schema-reference.md`
- **Token Waste Formula:** `token-waste-formula.md`
- **User Guide:** `user-guide-feedback-preferences.md`
- **Developer Guide:** `developer-guide-operation-types.md`
