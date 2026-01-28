# Template Examples

**Version:** 1.0
**Date:** 2025-11-10
**Status:** Active

This document provides complete examples of feedback templates for all operation types and statuses.

---

## Command Templates

### Command Success Template (command-passed.yaml)

**Use Case:** Collecting feedback after successful command execution

**Template:**
```yaml
---
template-id: command-passed
operation-type: command
success-status: passed
version: "1.0"
description: "Template for successful command execution"
---

# Template: Command Success Retrospective

This template collects feedback when a command completes successfully.

## Field Mappings

field-mappings:
  what-went-well:
    question-id: "cmd_passed_01"
    section: "## What Went Well"
    description: "Positive aspects of the command execution"

  what-could-improve:
    question-id: "cmd_passed_02"
    section: "## What Could Improve"
    description: "Areas where the workflow could be more efficient"

  efficiency-notes:
    question-id: "cmd_passed_03"
    section: "## Efficiency Notes"
    description: "Performance observations and optimization opportunities"

  suggestions:
    question-id: "cmd_passed_04"
    section: "## Suggestions for Next Time"
    description: "Specific recommendations for next execution"

## Template Sections

- # Retrospective: {operation}
- ## What Went Well
- ## What Could Improve
- ## Efficiency Notes
- ## Suggestions for Next Time
- ## Context (auto-populated)
- ## Performance Metrics (auto-populated)
- ## User Sentiment (auto-populated)
- ## Actionable Insights (auto-extracted)
```

**Sample Rendered Output:**
```markdown
---
operation: /dev STORY-042
type: command
status: passed
timestamp: 2025-11-10T10:30:00+00:00
story-id: STORY-042
template-version: "1.0"
---

# Retrospective: /dev STORY-042

## What Went Well
- TDD workflow was clear and well-structured
- Test-automator subagent generated comprehensive tests
- Coverage threshold (95%) achieved without issues

## What Could Improve
- Initial git setup was confusing (missing context about repository initialization)
- Deferral challenge checkpoint asked questions multiple times

## Efficiency Notes
- Test generation: 2 minutes (fast)
- Implementation: 8 minutes (reasonable)
- Refactoring: 3 minutes (quick)

## Suggestions for Next Time
- Provide clearer git initialization guidance at start of /dev
- Deduplicate deferral questions if user already answered in previous session

## Context
- **TodoWrite Status:** 4 of 4 tasks completed
- **Errors Encountered:** No
- **Performance Metrics:** Execution time: 12m 34s, Token usage: 87,500

## Performance Metrics
- Command execution time: 12 minutes 34 seconds
- Token usage: 87,500 tokens (main conversation)
- Subagent calls: 5 (test-automator, backend-architect, code-reviewer, refactoring-specialist, integration-tester)

## User Sentiment
Satisfied (4/5)

## Actionable Insights
1. Provide clearer git initialization guidance [Priority: Medium]
2. Deduplicate deferral questions [Priority: High]
```

---

### Command Failure Template (command-failed.yaml)

**Use Case:** Collecting feedback after command failure or error

**Template:**
```yaml
---
template-id: command-failed
operation-type: command
success-status: failed
version: "1.0"
description: "Template for failed command execution"
---

# Template: Command Failure Retrospective

This template collects feedback when a command fails or encounters errors.

## Field Mappings

field-mappings:
  what-happened:
    question-id: "cmd_failed_01"
    section: "## What Happened"
    description: "Description of the failure or error"

  root-cause:
    question-id: "cmd_failed_02"
    section: "## Root Cause Analysis"
    description: "Investigation of why the failure occurred"

  blockers:
    question-id: "cmd_failed_03"
    section: "## Blockers Encountered"
    description: "Obstacles that prevented completion"

  recovery:
    question-id: "cmd_failed_04"
    section: "## Recovery Steps Taken"
    description: "Actions attempted to resolve the issue"

  prevention:
    question-id: "cmd_failed_05"
    section: "## How to Prevent Next Time"
    description: "Strategies to avoid recurrence"

## Template Sections

- # Failure Retrospective: {operation}
- ## What Happened
- ## Root Cause Analysis
- ## Blockers Encountered
- ## Recovery Steps Taken
- ## How to Prevent Next Time
- ## Context (auto-populated)
- ## User Sentiment (auto-populated)
- ## Actionable Insights (auto-extracted)
```

**Sample Rendered Output:**
```markdown
---
operation: /qa STORY-042 deep
type: command
status: failed
timestamp: 2025-11-10T11:15:00+00:00
story-id: STORY-042
template-version: "1.0"
---

# Failure Retrospective: /qa STORY-042 deep

## What Happened
QA validation failed due to coverage threshold not met (actual: 92%, required: 95%)

## Root Cause Analysis
Several edge cases in template_engine.py were not covered by tests:
- Error handling for malformed YAML templates
- UUID collision prevention logic
- Special character escaping in frontmatter

## Blockers Encountered
- Difficulty identifying which specific lines were uncovered
- Test fixtures didn't include malformed YAML cases
- Time constraint for adding comprehensive edge case tests

## Recovery Steps Taken
1. Ran coverage report to identify uncovered lines
2. Added 8 new test cases for edge cases
3. Re-ran QA validation

## How to Prevent Next Time
- Run coverage analysis during development (not just at QA phase)
- Include edge case tests during TDD Red phase
- Use coverage-guided test generation

## Context
- **TodoWrite Status:** 3 of 5 tasks completed (blocked on coverage threshold)
- **Errors Encountered:** Yes (coverage threshold not met)
- **Performance Metrics:** Execution time: 8m 12s, Token usage: 45,200

## User Sentiment
Frustrated (2/5)

## Actionable Insights
1. Run coverage analysis during development [Priority: High]
2. Include edge case tests during TDD Red phase [Priority: High]
3. Use coverage-guided test generation [Priority: Medium]
```

---

## Skill Templates

### Skill Success Template (skill-passed.yaml)

**Use Case:** Collecting feedback after successful skill execution

**Template:**
```yaml
---
template-id: skill-passed
operation-type: skill
success-status: passed
version: "1.0"
description: "Template for successful skill execution"
---

# Template: Skill Success Retrospective

This template collects feedback when a skill completes successfully.

## Field Mappings

field-mappings:
  workflow-clarity:
    question-id: "skill_passed_01"
    section: "## Workflow Clarity"
    description: "How clear and understandable the skill's workflow was"

  subagent-effectiveness:
    question-id: "skill_passed_02"
    section: "## Subagent Coordination"
    description: "Effectiveness of subagent delegation and coordination"

  output-quality:
    question-id: "skill_passed_03"
    section: "## Output Quality"
    description: "Quality and completeness of skill output"

  improvements:
    question-id: "skill_passed_04"
    section: "## Improvement Suggestions"
    description: "Ideas for enhancing the skill"

## Template Sections

- # Skill Retrospective: {operation}
- ## Workflow Clarity
- ## Subagent Coordination
- ## Output Quality
- ## Improvement Suggestions
- ## Context (auto-populated)
- ## Performance Metrics (auto-populated)
- ## User Sentiment (auto-populated)
- ## Actionable Insights (auto-extracted)
```

**Sample Rendered Output:**
```markdown
---
operation: devforgeai-development
type: skill
status: passed
timestamp: 2025-11-10T12:00:00+00:00
story-id: STORY-042
template-version: "1.0"
---

# Skill Retrospective: devforgeai-development

## Workflow Clarity
The TDD workflow (Red → Green → Refactor) was very clear. Each phase had explicit instructions and success criteria.

## Subagent Coordination
Excellent subagent coordination:
- test-automator generated 61 comprehensive tests
- backend-architect implemented clean, well-documented code
- code-reviewer provided actionable feedback (92/100 quality score)
- All subagent handoffs were smooth

## Output Quality
High-quality implementation:
- 55/61 tests passing (90% pass rate)
- Code review score: 92/100
- Type hints: 100% coverage
- Documentation: Complete docstrings for all public functions

## Improvement Suggestions
- Consider parallel subagent execution for test generation + implementation
- Provide real-time progress updates during long-running phases
- Add checkpoint resume capability for interrupted workflows

## Context
- **TodoWrite Status:** 5 of 5 tasks completed
- **Errors Encountered:** No
- **Performance Metrics:** Execution time: 45m 12s, Token usage: 125,000

## Performance Metrics
- Skill execution time: 45 minutes 12 seconds
- Subagent invocations: 8 total
- Token usage (isolated context): 125,000 tokens
- Main conversation impact: ~15,000 tokens

## User Sentiment
Delighted (5/5)

## Actionable Insights
1. Consider parallel subagent execution [Priority: Medium]
2. Provide real-time progress updates [Priority: Low]
3. Add checkpoint resume capability [Priority: High]
```

---

### Skill Failure Template (skill-failed.yaml)

**Use Case:** Collecting feedback after skill encounters errors

**Template:**
```yaml
---
template-id: skill-failed
operation-type: skill
success-status: failed
version: "1.0"
description: "Template for failed skill execution"
---

# Template: Skill Failure Retrospective

This template collects feedback when a skill encounters errors.

## Field Mappings

field-mappings:
  error-description:
    question-id: "skill_failed_01"
    section: "## Error Description"
    description: "What error occurred during skill execution"

  subagent-issues:
    question-id: "skill_failed_02"
    section: "## Subagent Issues"
    description: "Problems with subagent invocations or coordination"

  recovery-approach:
    question-id: "skill_failed_03"
    section: "## Recovery Approach"
    description: "How the skill attempted to recover from the error"

  prevention:
    question-id: "skill_failed_04"
    section: "## Improvements for Future"
    description: "How to prevent similar issues"

## Template Sections

- # Skill Failure: {operation}
- ## Error Description
- ## Subagent Issues
- ## Recovery Approach
- ## Improvements for Future
- ## Context (auto-populated)
- ## User Sentiment (auto-populated)
- ## Actionable Insights (auto-extracted)
```

**Sample Rendered Output:**
```markdown
---
operation: devforgeai-qa
type: skill
status: failed
timestamp: 2025-11-10T13:30:00+00:00
story-id: STORY-042
template-version: "1.0"
---

# Skill Failure: devforgeai-qa

## Error Description
QA skill failed during deferral validation phase. The deferral-validator subagent encountered a circular dependency between STORY-042 and STORY-045.

## Subagent Issues
- deferral-validator subagent correctly detected circular dependency
- Skill halted with error (expected behavior)
- Error message was clear and actionable
- No recovery mechanism for circular dependencies (requires manual intervention)

## Recovery Approach
Skill halted execution and displayed error:
"Circular deferral chain detected: STORY-042 → STORY-045 → STORY-042. Manual resolution required."

Provided remediation steps:
1. Review story dependencies
2. Break circular chain by removing one deferral
3. Create follow-up story for deferred work

## Improvements for Future
- Add automatic circular dependency detection during story creation (not just QA)
- Provide suggested break points in circular chains
- Create visualization of deferral graph

## Context
- **TodoWrite Status:** 3 of 7 tasks completed (blocked on deferral validation)
- **Errors Encountered:** Yes (circular deferral chain)
- **Performance Metrics:** Execution time: 5m 30s, Token usage: 28,000

## User Sentiment
Neutral (3/5)

## Actionable Insights
1. Add circular dependency detection during story creation [Priority: High]
2. Provide suggested break points in circular chains [Priority: Medium]
3. Create visualization of deferral graph [Priority: Low]
```

---

## Subagent Templates

### Subagent Success Template (subagent-passed.yaml)

**Use Case:** Collecting feedback after successful subagent execution

**Template:**
```yaml
---
template-id: subagent-passed
operation-type: subagent
success-status: passed
version: "1.0"
description: "Template for successful subagent execution"
---

# Template: Subagent Success Retrospective

This template collects feedback when a subagent completes successfully.

## Field Mappings

field-mappings:
  task-clarity:
    question-id: "subagent_passed_01"
    section: "## Task Clarity"
    description: "How clear the task prompt was for the subagent"

  output-quality:
    question-id: "subagent_passed_02"
    section: "## Output Quality"
    description: "Quality and completeness of subagent output"

  specialization:
    question-id: "subagent_passed_03"
    section: "## Domain Expertise"
    description: "How well the subagent's specialization fit the task"

  suggestions:
    question-id: "subagent_passed_04"
    section: "## Suggestions for Improvement"
    description: "Ideas for enhancing subagent capabilities"

## Template Sections

- # Subagent Retrospective: {operation}
- ## Task Clarity
- ## Output Quality
- ## Domain Expertise
- ## Suggestions for Improvement
- ## Context (auto-populated)
- ## User Sentiment (auto-populated)
- ## Actionable Insights (auto-extracted)
```

**Sample Rendered Output:**
```markdown
---
operation: test-automator
type: subagent
status: passed
timestamp: 2025-11-10T14:00:00+00:00
story-id: STORY-042
template-version: "1.0"
---

# Subagent Retrospective: test-automator

## Task Clarity
Task prompt was very clear:
- Acceptance criteria provided from story
- Testing framework specified (pytest)
- Test structure requirements explicit
- Expected test count defined (60+ tests)

## Output Quality
Excellent output:
- 61 comprehensive test cases generated
- All tests followed AAA pattern (Arrange, Act, Assert)
- Test fixtures properly structured
- Edge cases well covered
- Documentation included in test docstrings

## Domain Expertise
test-automator demonstrated strong expertise:
- Proper pytest fixtures and parametrization
- Comprehensive mocking strategies
- Thoughtful edge case identification
- Realistic test data generation

## Suggestions for Improvement
- Could parallelize test generation for large test suites
- Consider generating property-based tests (hypothesis) for complex logic
- Add performance benchmarking tests automatically

## Context
- **TodoWrite Status:** N/A (subagent context)
- **Errors Encountered:** No
- **Performance Metrics:** Execution time: 3m 15s, Token usage: 35,000 (isolated)

## User Sentiment
Satisfied (4/5)

## Actionable Insights
1. Parallelize test generation for large test suites [Priority: Medium]
2. Generate property-based tests for complex logic [Priority: Low]
3. Add performance benchmarking tests automatically [Priority: Low]
```

---

## Generic Template

### Generic Fallback Template (generic.yaml)

**Use Case:** Fallback for any operation type or status when specific template not found

**Template:**
```yaml
---
template-id: generic
operation-type: generic
success-status: generic
version: "1.0"
description: "Fallback template for any operation"
---

# Template: Generic Operation Retrospective

This is a fallback template used when a specific template is not found.

## Field Mappings

field-mappings:
  what-went-well:
    question-id: "generic_01"
    section: "## What Went Well"
    description: "Positive outcomes from the operation"

  what-could-improve:
    question-id: "generic_02"
    section: "## What Could Improve"
    description: "Areas for improvement"

  suggestions:
    question-id: "generic_03"
    section: "## Suggestions for Improvement"
    description: "Specific recommendations"

  additional-notes:
    question-id: "generic_04"
    section: "## Additional Notes"
    description: "Any other relevant feedback"

## Template Sections

- # Operation Retrospective: {operation}
- ## What Went Well
- ## What Could Improve
- ## Suggestions for Improvement
- ## Additional Notes
- ## Context (auto-populated)
- ## User Sentiment (auto-populated)
- ## Actionable Insights (auto-extracted)
```

**Sample Rendered Output:**
```markdown
---
operation: custom-workflow
type: generic
status: passed
timestamp: 2025-11-10T15:00:00+00:00
template-version: "1.0"
---

# Operation Retrospective: custom-workflow

## What Went Well
- Workflow completed successfully
- All steps executed as expected
- Output met requirements

## What Could Improve
- Documentation could be more detailed
- Error handling could be more robust

## Suggestions for Improvement
- Add progress indicators for long-running steps
- Create specific template for this workflow type

## Additional Notes
This workflow used the generic template because no workflow-specific template exists yet.

## Context
- **TodoWrite Status:** 3 of 3 tasks completed
- **Errors Encountered:** No
- **Performance Metrics:** Execution time: 10m 00s, Token usage: 50,000

## User Sentiment
Satisfied (4/5)

## Actionable Insights
1. Add progress indicators for long-running steps [Priority: Medium]
2. Create specific template for this workflow type [Priority: High]
```

---

## Custom Template Example

### Project-Specific Template

**Use Case:** Organization wants custom template for their CI/CD workflow

**Template:**
```yaml
---
template-id: cicd-deployment
operation-type: workflow
success-status: passed
version: "1.0"
description: "Custom template for CI/CD deployment workflow"
---

# Template: CI/CD Deployment Retrospective

Custom template for collecting feedback on CI/CD deployments.

## Field Mappings

field-mappings:
  build-success:
    question-id: "cicd_01"
    section: "## Build Phase"
    description: "Feedback on build process"

  test-coverage:
    question-id: "cicd_02"
    section: "## Test Phase"
    description: "Test execution and coverage"

  deployment-process:
    question-id: "cicd_03"
    section: "## Deployment Phase"
    description: "Deployment execution feedback"

  smoke-tests:
    question-id: "cicd_04"
    section: "## Smoke Tests"
    description: "Post-deployment smoke test results"

  rollback-readiness:
    question-id: "cicd_05"
    section: "## Rollback Readiness"
    description: "Assessment of rollback preparedness"

## Template Sections

- # CI/CD Deployment: {operation}
- ## Build Phase
- ## Test Phase
- ## Deployment Phase
- ## Smoke Tests
- ## Rollback Readiness
- ## Context (auto-populated)
- ## User Sentiment (auto-populated)
- ## Actionable Insights (auto-extracted)
```

**Sample Rendered Output:**
```markdown
---
operation: deploy-production-v2.1.0
type: workflow
status: passed
timestamp: 2025-11-10T16:00:00+00:00
template-version: "1.0"
---

# CI/CD Deployment: deploy-production-v2.1.0

## Build Phase
- Build completed in 3 minutes 42 seconds
- All dependencies resolved successfully
- Docker image built and tagged: v2.1.0

## Test Phase
- Unit tests: 1,234 passed, 0 failed
- Integration tests: 87 passed, 0 failed
- E2E tests: 23 passed, 0 failed
- Coverage: 96.5% (exceeds 95% threshold)

## Deployment Phase
- Blue-green deployment strategy used
- New version deployed to green environment
- Traffic gradually shifted from blue to green over 10 minutes
- Zero downtime achieved

## Smoke Tests
- All critical endpoints responding correctly
- Database migrations applied successfully
- Cache warming completed
- Health checks passing

## Rollback Readiness
- Blue environment kept running for 1 hour
- Rollback tested in staging (successful)
- Rollback SOP documented and accessible
- On-call team notified

## Context
- **TodoWrite Status:** 8 of 8 tasks completed
- **Errors Encountered:** No
- **Performance Metrics:** Total deployment time: 15m 30s

## User Sentiment
Delighted (5/5)

## Actionable Insights
1. Document blue-green deployment best practices [Priority: Medium]
2. Automate rollback readiness validation [Priority: Low]
```

---

## References

- **Template Format Specification:** `template-format-specification.md`
- **Field Mapping Guide:** `field-mapping-guide.md`
- **User Customization Guide:** `user-customization-guide.md`

---

**Version History:**
- **1.0 (2025-11-10):** Initial examples for STORY-010
