---
name: context-extraction
description: Pattern for extracting operation context from TodoWrite state for feedback conversations
version: "1.0"
created: 2025-12-18
story: STORY-103
---

# Context Extraction Pattern

Extract operation context from TodoWrite state to pre-populate feedback conversations with rich, specific information.

## When to Use

Use this pattern when:
1. Feedback skill needs operation details for conversation context
2. User completes a /dev, /qa, /release, or /orchestrate operation
3. Generating feedback summaries or analysis reports

---

## Data Models

### OperationContext

Primary container for extracted operation state.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| operation_id | str | Required, UUID format | Unique identifier for this operation |
| operation_type | str | Required, enum: dev\|qa\|release\|orchestrate | Type of DevForgeAI operation |
| story_id | str | Optional, STORY-NNN format | Associated story if applicable |
| start_time | datetime | Required, ISO 8601 | When operation started |
| end_time | datetime | Required, ISO 8601 | When operation ended |
| duration_seconds | float | Required, >= 0 | Total operation duration |
| status | str | Required, enum: success\|failure\|partial | Operation outcome |
| todos | List[TodoContext] | Required, may be empty | Extracted todo items |
| error | ErrorContext | Optional, present when status=failure | Error details if operation failed |
| phases | List[str] | Required, may be empty | List of completed phases |

### TodoContext

Represents a single todo item extracted from TodoWrite.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| content | str | Required | Todo item description |
| status | str | Required, enum: pending\|in_progress\|completed | Current todo status |
| start_time | datetime | Optional, ISO 8601 | When todo started |
| end_time | datetime | Optional, ISO 8601 | When todo completed |
| duration_seconds | float | Optional, >= 0 | Todo execution time |
| error_message | str | Optional | Error message if todo failed |

### ErrorContext

Captures error details when operation fails.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| message | str | Required | Primary error message |
| failed_todo | str | Optional | Which todo failed |
| stack_trace | str | Optional, max 5KB | Truncated stack trace |
| error_type | str | Optional | Exception type name |

---

## Context Extraction Pattern

Follow these steps to extract operation context from TodoWrite state.

### Step 1: Read TodoWrite State

Access the current TodoWrite state to retrieve all todo items.

```
1. Query TodoWrite for current todos list
2. Capture each todo's content, status, and timing metadata
3. Record timestamp of state capture as extraction_time
```

### Step 2: Extract Todo Items

Transform raw todos into TodoContext objects.

For each todo in TodoWrite state:
1. Extract `content` (the todo description text)
2. Extract `status` (pending, in_progress, completed)
3. Calculate `start_time` from first status change to in_progress
4. Calculate `end_time` from status change to completed
5. Calculate `duration_seconds` = end_time - start_time (if both present)
6. Extract `error_message` if todo has failure indicator

### Step 3: Determine Operation Status

Analyze todos to determine overall operation outcome.

| Condition | Result |
|-----------|--------|
| All todos status=completed, no errors | status = "success" |
| Any todo has error_message OR blocked | status = "failure" |
| Some completed, some pending/in_progress | status = "partial" |

Decision logic:
1. Count completed todos
2. Count todos with errors
3. Count pending/in_progress todos
4. Apply status rules from table above

### Step 4: Calculate Timing

Compute operation timing from todo timestamps.

1. **start_time**: Earliest todo start_time across all todos
2. **end_time**: Latest todo end_time across all todos (or current time if incomplete)
3. **duration_seconds**: end_time - start_time in seconds

If no todo timestamps available:
- Use conversation start as start_time
- Use current time as end_time
- Log warning: "No todo timestamps found, using conversation bounds"

### Step 5: Extract Error Context

When operation status = "failure", capture error details.

1. Identify failed todo(s) by error_message presence
2. Extract primary error message
3. Extract stack trace if available
4. Truncate stack trace to 5KB maximum (see Performance section)
5. Record error_type from exception class name

If multiple errors:
- Use most recent error as primary
- Note total error count in message: "Primary error (1 of N)"

### Step 6: Extract Phase Information

Identify which workflow phases completed.

For /dev operations:
- Phase 0: Pre-flight validation
- Phase 1: Context validation
- Phase 2: TDD Red
- Phase 3: TDD Green
- Phase 4: TDD Refactor
- Phase 5: Integration tests

Detect phase completion by:
1. Check todo content for phase markers
2. Check for phase-specific output patterns
3. Add completed phase names to `phases` list

---

## Performance Requirements

### Timing Target: 200ms

Context extraction must complete within 200ms.

| Threshold | Action |
|-----------|--------|
| < 150ms | Normal operation |
| 150-200ms | Log info: "Extraction approaching timeout" |
| > 200ms | Return partial context, log warning |

### Size Limit: 50KB

Maximum serialized context size is 50KB.

Measure after JSON serialization:
1. If size <= 50KB: Return full context
2. If size > 50KB: Apply truncation strategies below

### Summarization for Large Operations

When operation has more than 100 todos:

1. Keep first 50 todos (early operation context)
2. Keep last 10 todos (recent/relevant context)
3. Insert summary marker between: `{"content": "[SUMMARY] {N} todos omitted for brevity", "status": "completed"}`

Example: 150 todos becomes 61 items (50 + 1 summary + 10)

### Stack Trace Truncation

When stack_trace exceeds 5KB:

1. Keep first 2KB (beginning of trace with context)
2. Insert marker: `\n[...truncated {N} bytes...]\n`
3. Keep last 2KB (proximate cause)

This preserves:
- Initial exception context
- Final exception location
- Approximate size reduction

---

## Graceful Degradation

Extraction must never block feedback conversations.

### Return Partial Context

If some data is unavailable:
1. Include available fields with valid data
2. Set unavailable fields to null or empty
3. Add `extraction_warnings` list with missing data notes

Example:
```json
{
  "operation_id": "uuid-here",
  "todos": [],
  "extraction_warnings": ["TodoWrite state unavailable"]
}
```

### Warning Logging (Not Error)

Use warning level for extraction issues:
- TodoWrite state not accessible
- Timing data incomplete
- Unexpected data format

Never use error level for:
- Missing optional fields
- Normal empty states
- Performance near limits

### No-Exception Guarantee

Extraction must catch all exceptions internally.

Pattern:
1. Wrap entire extraction in try-catch
2. Log exception details at warning level
3. Return empty context dict on complete failure

Never propagate exceptions to caller.

### Empty Context Fallback

When extraction completely fails:

Return minimal valid context:
```json
{
  "operation_id": null,
  "operation_type": "unknown",
  "status": "unknown",
  "todos": [],
  "extraction_failed": true,
  "extraction_error": "Brief error description"
}
```

This ensures:
- Feedback conversation can still proceed
- Failure is logged for debugging
- No downstream exceptions

---

## Examples

### Example 1: Successful /dev Operation

Input: /dev STORY-042 completed with 8 todos, all passing

```json
{
  "operation_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "operation_type": "dev",
  "story_id": "STORY-042",
  "start_time": "2025-12-18T10:00:00Z",
  "end_time": "2025-12-18T10:45:00Z",
  "duration_seconds": 2700.0,
  "status": "success",
  "todos": [
    {"content": "Validate context files", "status": "completed", "duration_seconds": 5.2},
    {"content": "Write failing tests", "status": "completed", "duration_seconds": 300.5},
    {"content": "Implement feature", "status": "completed", "duration_seconds": 1800.3},
    {"content": "Run tests", "status": "completed", "duration_seconds": 45.8}
  ],
  "error": null,
  "phases": ["pre-flight", "context-validation", "red", "green", "refactor"]
}
```

### Example 2: Failed /qa Operation

Input: /qa STORY-042 failed on coverage check

```json
{
  "operation_id": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
  "operation_type": "qa",
  "story_id": "STORY-042",
  "start_time": "2025-12-18T11:00:00Z",
  "end_time": "2025-12-18T11:15:00Z",
  "duration_seconds": 900.0,
  "status": "failure",
  "todos": [
    {"content": "Load story file", "status": "completed", "duration_seconds": 2.1},
    {"content": "Run test suite", "status": "completed", "duration_seconds": 120.5},
    {"content": "Check coverage thresholds", "status": "completed", "duration_seconds": 5.0, "error_message": "Coverage 82% below 95% threshold"}
  ],
  "error": {
    "message": "Coverage 82% below 95% threshold for business logic layer",
    "failed_todo": "Check coverage thresholds",
    "stack_trace": null,
    "error_type": "CoverageThresholdError"
  },
  "phases": ["load", "test-run", "coverage-check"]
}
```

### Example 3: Large Operation with Summarization

Input: /orchestrate with 150 todos

```json
{
  "operation_id": "c3d4e5f6-a7b8-9012-cdef-123456789012",
  "operation_type": "orchestrate",
  "story_id": "STORY-100",
  "start_time": "2025-12-18T09:00:00Z",
  "end_time": "2025-12-18T12:30:00Z",
  "duration_seconds": 12600.0,
  "status": "success",
  "todos": [
    {"content": "Phase 0: Pre-flight", "status": "completed"},
    {"content": "...first 49 todos..."},
    {"content": "[SUMMARY] 89 todos omitted for brevity", "status": "completed"},
    {"content": "...last 10 todos..."},
    {"content": "Phase 7: Result interpretation", "status": "completed"}
  ],
  "error": null,
  "phases": ["pre-flight", "context", "dev", "qa", "release"]
}
```

---

## Related Documentation

- `context-sanitization.md` - Sanitization patterns for removing secrets
- `feedback-question-templates.md` - Questions using extracted context
- `feedback-analysis-patterns.md` - Analysis using context data

---

**Document Version:** 1.0
**Last Updated:** 2025-12-18
**Story:** STORY-103
