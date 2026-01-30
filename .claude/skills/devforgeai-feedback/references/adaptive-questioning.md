---
name: adaptive-questioning
description: Pattern for selecting and formatting context-aware feedback questions
version: "1.0"
created: 2025-12-19
story: STORY-104
depends_on: STORY-103
---

# Adaptive Questioning Pattern

Select and format feedback questions based on operation context extracted via context-extraction.md. This pattern transforms generic feedback sessions into contextual, relevant conversations.

---

## Overview

Adaptive questioning improves feedback quality by:

1. **Selecting questions** based on operation outcome (success/failure/partial)
2. **Substituting context variables** into question templates
3. **Referencing specific todos, errors, and phases** in questions
4. **Falling back gracefully** when context is unavailable

**Integration:** This pattern consumes `OperationContext` from context-extraction.md (STORY-103) and passes context to the devforgeai-feedback skill for presentation via AskUserQuestion.

---

## Question Selection Pattern

### Decision Matrix

| Operation Status | Duration | Question Category |
|------------------|----------|-------------------|
| success | < 10 min (< 600s) | success_standard |
| success | >= 10 min (>= 600s) | success_long_running |
| failure | any | failure_focused |
| partial | any | partial_mixed |
| unknown | any | generic_fallback |

### Selection Algorithm

```
Step 1: Extract status from OperationContext
        - If status == "success" -> continue to Step 2
        - If status == "failure" -> select failure_focused category
        - If status == "partial" -> select partial_mixed category
        - If status unknown -> select generic_fallback

Step 2: Check duration_seconds for success operations
        - If duration_seconds >= 600 -> select success_long_running
        - Else -> select success_standard

Step 3: Check error context presence
        - If ErrorContext present -> include error-specific questions

Step 4: Select questions from category
        - Filter to questions with all required variables available

Step 5: Limit to 5-7 questions (BR-002)
        - Prioritize questions with most context variables substituted
        - Ensure at least 1 open-ended question included
```

---

## Template Variable Substitution

### Available Variables

| Variable | Source Field | Type | Example Value |
|----------|--------------|------|---------------|
| `{operation_type}` | OperationContext.operation_type | string | "dev" |
| `{duration}` | Formatted duration_seconds | string | "45 minutes" |
| `{story_id}` | OperationContext.story_id | string | "STORY-042" |
| `{todo_count}` | len(OperationContext.todos) | number | "8" |
| `{completed_count}` | count(todos where status=completed) | number | "7" |
| `{error_message}` | ErrorContext.message | string | "Coverage 82% below threshold" |
| `{failed_todo}` | ErrorContext.failed_todo | string | "Check coverage thresholds" |
| `{longest_phase}` | max(phases by duration) | string | "TDD Green" |

### Substitution Pattern

```
Step 1: Parse template for {variable_name} patterns
        - Use regex: \{([a-z_]+)\}

Step 2: For each variable, look up value in context
        - Check OperationContext fields first
        - Check ErrorContext if present
        - Calculate derived values (completed_count, duration)

Step 3: Substitute value if found
        - Replace {variable} with actual value

Step 4: Handle missing variables
        - If required: skip entire question
        - If optional: use fallback value or omit phrase

Step 5: Verify no unsubstituted variables remain
        - Log warning if {variable} pattern still present
```

### Duration Formatting

| Seconds | Formatted Output |
|---------|------------------|
| < 60 | "{N} seconds" |
| 60-3599 | "{N} minutes" |
| 3600-7199 | "about an hour" |
| >= 7200 | "{N} hours" |

### Variable Fallback Values

| Variable | Fallback Value | When to Use |
|----------|----------------|-------------|
| `{operation_type}` | "operation" | Generic reference |
| `{todo_count}` | "several" | Count unavailable |
| `{completed_count}` | Same as todo_count | Completion unknown |
| `{error_message}` | *skip question* | No error context |
| `{failed_todo}` | "a task" | Failed todo unknown |
| `{longest_phase}` | *omit phrase* | Phase timing unavailable |

---

## Success Question Templates

### Standard Success (< 10 min)

Questions focus on process improvements and patterns worth repeating.

| ID | Template | Required Variables |
|----|----------|-------------------|
| success_01 | "The {operation_type} for {story_id} completed in {duration}. What went particularly well?" | operation_type, story_id, duration |
| success_02 | "You completed {completed_count} tasks. Any patterns worth repeating?" | completed_count |
| success_03 | "All tests passed. Were there any edge cases that surprised you?" | *none* |
| success_04 | "What could make future {operation_type} operations even smoother?" | operation_type |

### Long-Running Success (>= 10 min)

Questions investigate time expectations and bottlenecks for operations taking 10+ minutes.

| ID | Template | Required Variables |
|----|----------|-------------------|
| success_long_01 | "This {operation_type} took {duration} - was this expected?" | operation_type, duration |
| success_long_02 | "The {longest_phase} phase took the longest. Any optimizations possible?" | longest_phase |
| success_long_03 | "Would you set a time expectation for future {operation_type} runs?" | operation_type |
| success_long_04 | "Were there any phases that felt slower than usual?" | *none* |

---

## Failure Question Templates

Questions focus on root cause analysis and prevention strategies.

| ID | Template | Required Variables |
|----|----------|-------------------|
| failure_01 | "The operation failed: {error_message}. What caused this?" | error_message |
| failure_02 | "The '{failed_todo}' task could not complete. What would have prevented this?" | failed_todo |
| failure_03 | "What additional information would have helped avoid this failure?" | *none* |
| failure_04 | "Were you able to diagnose the issue, or did you need external help?" | *none* |
| failure_05 | "Should this failure have been caught earlier in the workflow?" | *none* |
| failure_06 | "How can we prevent this type of failure in the future?" | *none* |

---

## Partial Completion Templates

Questions address what succeeded and what blocked progress.

| ID | Template | Required Variables |
|----|----------|-------------------|
| partial_01 | "The {operation_type} completed {completed_count} of {todo_count} tasks. What blocked the rest?" | operation_type, completed_count, todo_count |
| partial_02 | "Which incomplete task is the highest priority to resolve?" | *none* |
| partial_03 | "Can you continue from where you left off, or is a restart needed?" | *none* |
| partial_04 | "What would help complete the remaining tasks?" | *none* |

---

## Generic Fallback Templates

When context is unavailable or extraction failed, use these context-free questions.

| ID | Template | Required Variables |
|----|----------|-------------------|
| generic_01 | "How did this operation go overall?" | *none* |
| generic_02 | "What challenges or blockers did you encounter?" | *none* |
| generic_03 | "What would you do differently next time?" | *none* |
| generic_04 | "Any process improvements worth noting?" | *none* |

---

## Graceful Fallback Pattern

### When to Fall Back

Use generic fallback questions when:

1. **Context extraction failed completely** - OperationContext is null/empty
2. **Status field is "unknown"** - Cannot determine outcome
3. **Required variables missing** - Fewer than 3 contextual questions possible
4. **Context sanitization removed critical fields** - Sensitive data redacted

### Fallback Decision Logic

```
Step 1: Attempt to select contextual questions from appropriate category

Step 2: For each question, verify all required variables are available
        - Check OperationContext fields
        - Check ErrorContext if status == failure

Step 3: Count questions with all variables available
        - If >= 3 contextual questions possible -> use them
        - If < 3 contextual questions possible -> use generic_fallback

Step 4: Log which fields were available vs missing
        - Helps diagnose context extraction issues
```

### Logging Pattern

```
[ADAPTIVE_QUESTIONING] Context available: {available_fields}
[ADAPTIVE_QUESTIONING] Context missing: {missing_fields}
[ADAPTIVE_QUESTIONING] Selected category: {category}
[ADAPTIVE_QUESTIONING] Questions selected: {count}
[ADAPTIVE_QUESTIONING] Fallback used: {yes/no}
```

**Example log output:**
```
[ADAPTIVE_QUESTIONING] Context available: operation_type, status, duration_seconds, todos
[ADAPTIVE_QUESTIONING] Context missing: error, phases
[ADAPTIVE_QUESTIONING] Selected category: success_standard
[ADAPTIVE_QUESTIONING] Questions selected: 5
[ADAPTIVE_QUESTIONING] Fallback used: no
```

### No-Error Guarantee

When context is partial:
- Do not throw errors or exceptions
- Use available context for questions that can be populated
- Fall back gracefully for questions requiring missing fields
- Always present at least the generic fallback questions

---

## Context Metadata Pre-Population

### Metadata Inclusion Pattern

Feedback templates include operation context in their metadata section:

```yaml
context:
  operation_type: "{operation_type}"
  story_id: "{story_id}"
  status: "{status}"
  duration_seconds: {duration_seconds}
  todo_count: {todo_count}
  completed_count: {completed_count}
  error_message: "{error_message}"  # Only when status=failure
  phases: ["{phase_1}", "{phase_2}", ...]
```

### Pre-Fill Pattern

1. Extract OperationContext from session state
2. Calculate derived fields (completed_count, formatted duration)
3. Identify longest-running phase from phase timing data
4. Include error context when status = failure
5. Populate template metadata section

### Longest-Running Phase Identification

```
Step 1: Extract phases array from OperationContext

Step 2: For each phase, calculate duration
        - end_time - start_time = duration

Step 3: Find phase with maximum duration
        - Store as longest_phase variable

Step 4: Use in templates referencing {longest_phase}
```

---

## AskUserQuestion Integration

### Variable Usage in Prompts

The AskUserQuestion tool receives pre-populated question text:

```yaml
questions:
  - question: "The dev for STORY-042 completed in 45 minutes. What went particularly well?"
    header: "Success"
    options:
      - label: "Tests caught edge cases"
        description: "TDD workflow identified issues early"
      - label: "Clear acceptance criteria"
        description: "Story requirements were unambiguous"
      - label: "Good documentation"
        description: "Context files provided guidance"
    multiSelect: false
```

### Context Passing to devforgeai-feedback Skill

```
Step 1: Skill receives operation completion event

Step 2: Skill invokes context extraction (STORY-103)
        - Returns OperationContext with all fields

Step 3: Skill invokes adaptive questioning (this pattern)
        - Returns selected questions with variables substituted

Step 4: Skill presents questions via AskUserQuestion
        - Questions reference specific context from operation

Step 5: Skill persists responses
        - Includes original context for analysis
```

---

## Performance Requirements

### NFR-002: Question Selection < 100ms

Target: Complete question selection and variable substitution in under 100ms.

| Threshold | Action |
|-----------|--------|
| < 50ms | Normal operation |
| 50-100ms | Log info: "Selection approaching limit" |
| > 100ms | Log warning, return cached/default questions |

### Optimization Strategies

1. **Pre-compile regex patterns** - Variable matching pattern compiled once
2. **Cache question templates** - Load at skill initialization
3. **Early exit** - Stop when 7 questions selected
4. **Skip validation for fallback** - Generic questions need no variable check
5. **Lazy phase calculation** - Only compute longest_phase if needed

---

## Question Count Limit (BR-002)

Maximum 7 questions per session to avoid survey fatigue.

### Selection Priority

1. **Most relevant to operation outcome** - Category-specific questions first
2. **Questions with most variables substituted** - More contextual = higher priority
3. **Balance of improvement + diagnostic** - Mix forward-looking and retrospective
4. **At least 1 open-ended question** - Ensure qualitative feedback captured

### Minimum Questions

Always present at least 3 questions:
- If category has fewer than 3 applicable questions, supplement with generic

---

## Natural Language Guidelines (BR-004)

Questions should feel conversational, not templated.

### DO

- Use contractions ("What's" not "What is")
- Reference specifics naturally ("The 45-minute duration" not "Duration: 45 minutes")
- Vary question structure (not all starting with "What")
- Use action-oriented language ("How can we improve" vs "What improvements exist")

### DON'T

- Start multiple questions with the same word
- Use technical jargon in questions (no "OperationContext", "ErrorContext")
- Make questions sound robotic or formulaic
- Include variable markers in displayed text (no "{todo_count}" shown to user)

---

## Examples

### Example 1: Successful /dev Operation

**Context:**
```json
{
  "operation_type": "dev",
  "story_id": "STORY-042",
  "duration_seconds": 2700,
  "status": "success",
  "todos": [...],  // 8 items, all completed
  "error": null,
  "phases": ["pre-flight", "context-validation", "red", "green", "refactor"]
}
```

**Selected Questions (5 - success_long_running category):**

1. "This dev took 45 minutes - was this expected?"
2. "You completed 8 tasks. Any patterns worth repeating?"
3. "The TDD Green phase took the longest. Any optimizations possible?"
4. "All tests passed. Were there any edge cases that surprised you?"
5. "What could make future dev operations even smoother?"

### Example 2: Failed /qa Operation

**Context:**
```json
{
  "operation_type": "qa",
  "story_id": "STORY-043",
  "duration_seconds": 180,
  "status": "failure",
  "todos": [...],  // 5 items, 3 completed
  "error": {
    "message": "Coverage 82% below 95% threshold",
    "failed_todo": "Check coverage thresholds"
  },
  "phases": ["validation", "coverage-check"]
}
```

**Selected Questions (5 - failure_focused category):**

1. "The operation failed: Coverage 82% below 95% threshold. What caused this?"
2. "The 'Check coverage thresholds' task could not complete. What would have prevented this?"
3. "What additional information would have helped avoid this failure?"
4. "Should this failure have been caught earlier in the workflow?"
5. "Were you able to diagnose the issue, or did you need external help?"

### Example 3: Missing Context (Fallback)

**Context:**
```json
{
  "extraction_failed": true,
  "operation_type": null,
  "status": "unknown"
}
```

**Log Output:**
```
[ADAPTIVE_QUESTIONING] Context available: none
[ADAPTIVE_QUESTIONING] Context missing: operation_type, status, duration_seconds, todos, error
[ADAPTIVE_QUESTIONING] Selected category: generic_fallback
[ADAPTIVE_QUESTIONING] Questions selected: 4
[ADAPTIVE_QUESTIONING] Fallback used: yes
```

**Selected Questions (4 - generic_fallback category):**

1. "How did this operation go overall?"
2. "What challenges or blockers did you encounter?"
3. "What would you do differently next time?"
4. "Any process improvements worth noting?"

---

## Related Documentation

- `context-extraction.md` - Source of OperationContext data model (STORY-103)
- `context-sanitization.md` - Pre-processing before question selection (STORY-103)
- `feedback-question-templates.md` - Full template library with context variables
- `../SKILL.md` - Feedback skill main documentation

---

**Document Version:** 1.0
**Last Updated:** 2025-12-19
**Story:** STORY-104
**Status:** Active
