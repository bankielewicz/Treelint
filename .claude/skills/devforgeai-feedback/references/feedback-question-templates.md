# Feedback Question Templates

**Version:** 1.0
**Date:** 2025-11-20
**Status:** Active

This guide provides default feedback questions organized by operation type and status, designed for retrospective conversations in the DevForgeAI feedback system.

---

## Overview

Question templates define:
- **Context-specific questions** - Tailored to operation type (command, skill, subagent)
- **Status-aware prompts** - Different questions for success/failure/partial outcomes
- **Progressive disclosure** - Start broad, then drill into specifics
- **Actionable insights** - Questions designed to extract improvement opportunities

---

## Template Structure

### Question Format

```yaml
questions:
  - id: "{type}_{status}_{number}"
    text: "Question text here?"
    type: "open|rating|multiple-choice"
    context: "When to ask this question"
    example_response: "Sample answer"
```

### Question Types

- **open** - Free-text response (insights, explanations, suggestions)
- **rating** - Scale 1-5 (clarity, efficiency, satisfaction)
- **multiple-choice** - Predefined options (blockers, patterns, categories)

---

## Command Success Questions

**Operation Type:** `command`
**Status:** `passed`
**Use Case:** Successful command completion (e.g., `/dev STORY-001`)

### Core Questions

```yaml
questions:
  - id: "cmd_passed_01"
    text: "What went well during this command execution?"
    type: "open"
    context: "Identify positive patterns to reinforce"
    example_response: "TDD workflow was clear, tests caught 2 edge cases early"

  - id: "cmd_passed_02"
    text: "What could be improved for next time?"
    type: "open"
    context: "Surface friction points and inefficiencies"
    example_response: "Test generation took 15min, could use more examples"

  - id: "cmd_passed_03"
    text: "Were there any unexpected challenges?"
    type: "open"
    context: "Capture edge cases and surprising behaviors"
    example_response: "Database seeding failed first try, unclear error message"

  - id: "cmd_passed_04"
    text: "How efficient was the overall workflow? (1-5)"
    type: "rating"
    context: "Quantify user satisfaction for trend analysis"
    example_response: "4 (efficient, minor delays in dependency resolution)"
```

### Specialized Questions

```yaml
questions:
  - id: "cmd_passed_05"
    text: "Did the command output provide clear next steps?"
    type: "multiple-choice"
    options: ["Very clear", "Somewhat clear", "Unclear", "No next steps provided"]
    context: "Evaluate command UX and guidance quality"

  - id: "cmd_passed_06"
    text: "Were all acceptance criteria clearly testable?"
    type: "multiple-choice"
    options: ["All testable", "Most testable", "Some ambiguous", "Many unclear"]
    context: "For /dev command, assess AC quality"

  - id: "cmd_passed_07"
    text: "How long did the command take vs expected? (minutes)"
    type: "open"
    context: "Track performance expectations vs reality"
    example_response: "Expected 20min, took 35min (dependency issues)"
```

---

## Command Failure Questions

**Operation Type:** `command`
**Status:** `failed`
**Use Case:** Command failed with errors

### Core Questions

```yaml
questions:
  - id: "cmd_failed_01"
    text: "What was the primary blocker that caused failure?"
    type: "open"
    context: "Identify root cause for prevention"
    example_response: "Missing database migration script, tests failed"

  - id: "cmd_failed_02"
    text: "Was the error message clear and actionable?"
    type: "rating"
    context: "Evaluate error handling quality (1=cryptic, 5=very clear)"
    example_response: "2 - Error mentioned 'validation failed' without specifics"

  - id: "cmd_failed_03"
    text: "What information would have helped resolve this faster?"
    type: "open"
    context: "Improve error diagnostics and documentation"
    example_response: "Stack trace location, example of valid input format"

  - id: "cmd_failed_04"
    text: "Were you able to recover, or did you need external help?"
    type: "multiple-choice"
    options: ["Self-resolved", "Needed docs", "Needed teammate", "Blocked entirely"]
    context: "Measure self-service capability"
```

### Recovery Questions

```yaml
questions:
  - id: "cmd_failed_05"
    text: "What steps did you take to debug the failure?"
    type: "open"
    context: "Learn effective debugging patterns"
    example_response: "Checked logs, re-ran with --verbose, inspected DB state"

  - id: "cmd_failed_06"
    text: "Should this failure have been caught earlier in the workflow?"
    type: "multiple-choice"
    options: ["Yes, in preflight checks", "Yes, in Phase 1", "No, unavoidable", "Not sure"]
    context: "Identify opportunities for earlier validation"

  - id: "cmd_failed_07"
    text: "How can we prevent this failure in future runs?"
    type: "open"
    context: "Extract actionable prevention strategies"
    example_response: "Add migration check to Phase 0, validate DB schema"
```

---

## Skill Success Questions

**Operation Type:** `skill`
**Status:** `passed`
**Use Case:** Skill completed successfully (e.g., `devforgeai-development`)

### Core Questions

```yaml
questions:
  - id: "skill_passed_01"
    text: "Which phase of the skill workflow was most valuable?"
    type: "open"
    context: "Identify high-impact workflow stages"
    example_response: "Phase 2 (implementation) - clear structure, good examples"

  - id: "skill_passed_02"
    text: "Were the skill instructions clear and comprehensive?"
    type: "rating"
    context: "Evaluate skill documentation quality (1=confusing, 5=excellent)"
    example_response: "4 - Very clear, minor confusion in Phase 3 refactoring"

  - id: "skill_passed_03"
    text: "Did any phase take significantly longer than expected?"
    type: "open"
    context: "Identify workflow bottlenecks"
    example_response: "Phase 4 integration testing took 2x expected time"

  - id: "skill_passed_04"
    text: "Were subagent invocations seamless, or did you encounter issues?"
    type: "multiple-choice"
    options: ["Seamless", "Minor delays", "Confusing output", "Failed invocations"]
    context: "Assess subagent integration quality"
```

### Improvement Questions

```yaml
questions:
  - id: "skill_passed_05"
    text: "What additional guidance would have been helpful?"
    type: "open"
    context: "Surface missing documentation or examples"
    example_response: "More examples of complex mocking scenarios in Phase 1"

  - id: "skill_passed_06"
    text: "Did the skill enforce context file constraints correctly?"
    type: "multiple-choice"
    options: ["Yes, all validated", "Mostly", "Some violations missed", "No validation"]
    context: "Validate architectural constraint enforcement"

  - id: "skill_passed_07"
    text: "Would you use this skill again for similar work?"
    type: "rating"
    context: "Measure skill utility and satisfaction (1=no, 5=definitely)"
    example_response: "5 - Very effective, saved significant time"
```

---

## Skill Failure Questions

**Operation Type:** `skill`
**Status:** `failed`
**Use Case:** Skill failed to complete

### Core Questions

```yaml
questions:
  - id: "skill_failed_01"
    text: "At which phase did the skill fail?"
    type: "open"
    context: "Pinpoint failure location for debugging"
    example_response: "Phase 2, during implementation - test generation incomplete"

  - id: "skill_failed_02"
    text: "What was the immediate cause of failure?"
    type: "open"
    context: "Identify proximate cause"
    example_response: "Subagent returned empty result, skill couldn't proceed"

  - id: "skill_failed_03"
    text: "Did the skill provide a clear rollback or recovery path?"
    type: "multiple-choice"
    options: ["Yes, clear steps", "Partial guidance", "No guidance", "Made situation worse"]
    context: "Evaluate error recovery design"

  - id: "skill_failed_04"
    text: "Were you able to manually complete the failed phase?"
    type: "multiple-choice"
    options: ["Yes, easily", "Yes, with effort", "No, blocked", "Partially"]
    context: "Assess manual workaround feasibility"
```

---

## Subagent Success Questions

**Operation Type:** `subagent`
**Status:** `passed`
**Use Case:** Subagent completed task successfully

### Core Questions

```yaml
questions:
  - id: "subagent_passed_01"
    text: "Did the subagent produce the expected output format?"
    type: "multiple-choice"
    options: ["Exactly as expected", "Mostly correct", "Required edits", "Wrong format"]
    context: "Validate output contract adherence"

  - id: "subagent_passed_02"
    text: "How accurate was the subagent's analysis/generation?"
    type: "rating"
    context: "Measure output quality (1=poor, 5=excellent)"
    example_response: "4 - Good quality, minor fixes needed"

  - id: "subagent_passed_03"
    text: "Were there any hallucinations or incorrect assumptions?"
    type: "open"
    context: "Identify AI accuracy issues"
    example_response: "Assumed API endpoint existed that wasn't in spec"

  - id: "subagent_passed_04"
    text: "Did the subagent respect context file constraints?"
    type: "multiple-choice"
    options: ["Yes, all constraints", "Mostly", "Some violations", "Many violations"]
    context: "Validate architectural compliance"
```

---

## Subagent Failure Questions

**Operation Type:** `subagent`
**Status:** `failed`
**Use Case:** Subagent failed or produced unusable output

### Core Questions

```yaml
questions:
  - id: "subagent_failed_01"
    text: "What caused the subagent to fail or produce bad output?"
    type: "open"
    context: "Identify root cause (prompt, context, tool limitations)"
    example_response: "Insufficient context - didn't load tech-stack.md"

  - id: "subagent_failed_02"
    text: "Could the failure have been prevented with better input?"
    type: "multiple-choice"
    options: ["Yes, clearer prompt", "Yes, more context", "No, tool limitation", "Not sure"]
    context: "Determine if prompt engineering can help"

  - id: "subagent_failed_03"
    text: "Did the parent skill/command handle the failure gracefully?"
    type: "rating"
    context: "Evaluate error handling quality (1=crash, 5=smooth recovery)"
    example_response: "3 - Logged error but workflow stopped abruptly"

  - id: "subagent_failed_04"
    text: "How should this subagent be improved?"
    type: "open"
    context: "Extract actionable improvement suggestions"
    example_response: "Add validation step to check context files loaded"
```

---

## Generic Fallback Questions

**Operation Type:** `generic`
**Status:** `any`
**Use Case:** When specific template not found

### Universal Questions

```yaml
questions:
  - id: "generic_01"
    text: "Overall, how did this operation go?"
    type: "rating"
    context: "Baseline satisfaction metric"
    example_response: "3 - Mixed results, some good, some friction"

  - id: "generic_02"
    text: "What was the most challenging aspect?"
    type: "open"
    context: "Identify pain points"
    example_response: "Understanding error messages, not clear what failed"

  - id: "generic_03"
    text: "What would you change to improve this workflow?"
    type: "open"
    context: "Collect improvement ideas"
    example_response: "Add progress indicators, better documentation links"

  - id: "generic_04"
    text: "Would you recommend this workflow to a colleague?"
    type: "multiple-choice"
    options: ["Yes, as-is", "Yes, with improvements", "Maybe", "No"]
    context: "Net Promoter Score proxy"
```

---

## Question Selection Strategy

### Failures-Only Mode
When `trigger_mode: failures-only` in config:
- Ask 5-7 questions (focused diagnostic)
- Prioritize root cause and recovery questions
- Include "How can we prevent this?" question

### Always Mode
When `trigger_mode: always`:
- Ask 3-5 questions (lighter touch)
- Balance positive + improvement questions
- Skip redundant questions for frequent operations

### Specific-Operations Mode
When `trigger_mode: specific-operations`:
- Customized question sets per operation
- Deeper inquiry for critical operations (/qa, /release)
- Lighter for routine operations (/dev)

---

## Customization Guide

### Adding New Questions

1. **Choose unique ID:** Follow pattern `{type}_{status}_{number}`
2. **Write clear question:** Specific, actionable, unambiguous
3. **Select type:** open/rating/multiple-choice
4. **Define context:** When/why to ask this question
5. **Add example:** Show expected response format

### Modifying Existing Questions

1. **Preserve question ID:** Changing ID breaks trend analysis
2. **Update text carefully:** May affect historical data comparability
3. **Document changes:** Note in template version history
4. **Test with users:** Validate question clarity

### Reusing Question IDs Across Templates

**✅ Good practice:**
- Same question asked in success and failure contexts
- Generic questions applicable to all operation types
- Enables cross-template trend analysis

**❌ Avoid:**
- Reusing IDs for completely different questions
- Changing question meaning while keeping ID

---

## Integration with Templates

Questions from this guide are referenced in YAML templates:

```yaml
# In command-passed.yaml
field-mappings:
  what-went-well:
    question-id: "cmd_passed_01"  # References question from this guide
    section: "## What Went Well"
```

Template engine:
1. Loads question template by ID
2. Presents question during conversation
3. Maps response to section using field mapping
4. Renders in output markdown

---

## Question Evolution

### Metrics to Track
- **Skip rate:** If >50% skip a question, consider removing
- **Response length:** Short answers may indicate unclear questions
- **Actionability:** Do responses lead to improvements?
- **Redundancy:** Multiple questions yielding same insights

### Review Cycle
- **Quarterly:** Review skip rates and response patterns
- **After major changes:** Add questions for new workflows
- **User feedback:** Incorporate suggestions from retrospectives

---

## Context Variable Support (STORY-104)

Questions can include context variables that are substituted with operation-specific values at runtime. This enables adaptive, contextual feedback questions.

### Variable Syntax

Use `{variable_name}` syntax in question text. Variables are replaced with actual values from the operation context before presenting to users.

**Example:**
```
Template: "The {operation_type} for {story_id} took {duration}."
Result:   "The dev for STORY-042 took 45 minutes."
```

### Available Variables

| Variable | Source | Example Value | Fallback |
|----------|--------|---------------|----------|
| `{operation_type}` | OperationContext.operation_type | "dev", "qa", "release" | "operation" |
| `{story_id}` | OperationContext.story_id | "STORY-042" | *skip question* |
| `{duration}` | Formatted duration_seconds | "45 minutes" | "unknown time" |
| `{todo_count}` | len(todos) | "8" | "several" |
| `{completed_count}` | count(status=completed) | "7" | same as todo_count |
| `{error_message}` | ErrorContext.message | "Coverage 82% below threshold" | *skip question* |
| `{failed_todo}` | ErrorContext.failed_todo | "Check coverage thresholds" | "a task" |
| `{longest_phase}` | max(phases by duration) | "TDD Green" | *omit phrase* |

### Duration Formatting

| Seconds | Formatted Output |
|---------|------------------|
| < 60 | "{N} seconds" |
| 60-3599 | "{N} minutes" |
| 3600-7199 | "about an hour" |
| >= 7200 | "{N} hours" |

### Variable Fallback Rules

1. **Required variable missing:** Skip the entire question
2. **Optional variable missing:** Use fallback value from table above
3. **All variables missing:** Use generic fallback question set

### Context-Aware Question Templates

#### Success Templates with Variables

```yaml
questions:
  - id: "ctx_success_01"
    text: "The {operation_type} for {story_id} completed in {duration}. What went particularly well?"
    type: "open"
    required_vars: ["operation_type", "story_id", "duration"]
    context: "Contextual success opener with specific operation details"

  - id: "ctx_success_02"
    text: "You completed {completed_count} tasks. Any patterns worth repeating?"
    type: "open"
    required_vars: ["completed_count"]
    context: "Quantified success reflection"

  - id: "ctx_success_03"
    text: "The {longest_phase} phase took the longest. Any optimizations possible?"
    type: "open"
    required_vars: ["longest_phase"]
    context: "Phase-specific improvement inquiry"
```

#### Failure Templates with Variables

```yaml
questions:
  - id: "ctx_failure_01"
    text: "The operation failed: {error_message}. What caused this?"
    type: "open"
    required_vars: ["error_message"]
    context: "Error-specific root cause analysis"

  - id: "ctx_failure_02"
    text: "The '{failed_todo}' task could not complete. What would have prevented this?"
    type: "open"
    required_vars: ["failed_todo"]
    context: "Todo-specific prevention inquiry"
```

#### Long-Running Templates with Variables

```yaml
questions:
  - id: "ctx_long_01"
    text: "This {operation_type} took {duration} - was this expected?"
    type: "open"
    required_vars: ["operation_type", "duration"]
    context: "Duration expectation check for operations >= 10 minutes"

  - id: "ctx_long_02"
    text: "Would you set a time expectation for future {operation_type} runs?"
    type: "open"
    required_vars: ["operation_type"]
    context: "Future timing guidance"
```

#### Partial Completion Templates

```yaml
questions:
  - id: "ctx_partial_01"
    text: "The {operation_type} completed {completed_count} of {todo_count} tasks. What blocked the rest?"
    type: "open"
    required_vars: ["operation_type", "completed_count", "todo_count"]
    context: "Partial completion analysis"
```

### Integration with Adaptive Questioning

Context-aware templates are selected based on operation outcome. See `adaptive-questioning.md` (STORY-104) for the complete selection algorithm and fallback patterns.

---

## Related Documentation

- **Template Format Specification:** `template-format-specification.md`
- **Field Mapping Guide:** `field-mapping-guide.md`
- **User Customization Guide:** `user-customization-guide.md`
- **Feedback Persistence Guide:** `feedback-persistence-guide.md`
- **Adaptive Questioning Pattern:** `adaptive-questioning.md` (STORY-104)

---

**The question templates serve as defaults. Users can customize via `devforgeai/feedback/config.yaml` or create custom templates for specific operations. The goal is actionable insights, not exhaustive surveys—balance depth with user patience.**
