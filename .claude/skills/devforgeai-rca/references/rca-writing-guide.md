# RCA Writing Guide for DevForgeAI

**Purpose:** Standards for writing clear, actionable RCA documents

**Target Audience:** devforgeai-rca skill (Claude executing RCA Phase 5)

---

## Table of Contents

1. [RCA Document Structure](#rca-document-structure)
2. [Title Conventions](#title-conventions)
3. [Issue Description Clarity](#issue-description-clarity)
4. [5 Whys Formatting](#5-whys-formatting)
5. [Evidence Section Organization](#evidence-section-organization)
6. [Recommendation Prioritization](#recommendation-prioritization)
7. [Implementation Checklist Generation](#implementation-checklist-generation)
8. [Prevention Strategy Formulation](#prevention-strategy-formulation)

---

## RCA Document Structure

### Required Sections (In Order)

1. **Header** - RCA number, date, metadata
2. **Issue Description** - What happened, when, where
3. **5 Whys Analysis** - Progressive questioning to root cause
4. **Evidence Collected** - Files examined, excerpts, validation
5. **Recommendations** - Prioritized fixes (CRITICAL → LOW)
6. **Implementation Checklist** - Action items
7. **Prevention Strategy** - Short-term and long-term
8. **Related RCAs** - Pattern linking

**All 8 sections REQUIRED** - No exceptions

### Optional Sections

**May include if relevant:**
- **Timeline of Events** - Chronological incident progression
- **Contributing Factors** - Additional causes beyond root
- **Metrics** - Quantitative impact (if measurable)
- **Stakeholder Impact** - Who was affected

### Section Dependencies

```
Header → Issue Description → 5 Whys → Evidence
                                ↓
                         Recommendations
                                ↓
                    Implementation Checklist
                                ↓
                      Prevention Strategy
```

**Evidence must exist before Recommendations** (can't recommend without proof)
**Recommendations must exist before Checklist** (checklist derived from recommendations)

---

## Title Conventions

### Title Format

**Pattern:** `RCA-{NUMBER}: {Brief Description}`

**Examples:**
```
✅ GOOD:
RCA-010: Context File Validation Missing
RCA-011: Autonomous Deferrals Without Approval
RCA-012: Skill Execution Incomplete Workflow

❌ BAD:
RCA-010: There was a problem with the development workflow (too vague)
RCA-011: Bug (not descriptive)
RCA-012: /dev command issue when context files missing from greenfield projects (too long)
```

### Title Quality Criteria

**Good titles:**
- ✅ 3-6 words (brief)
- ✅ Identifies component and issue
- ✅ Scannable (can find in directory listing)
- ✅ Specific enough to be unique

**Poor titles:**
- ❌ Too vague ("Problem", "Issue", "Bug")
- ❌ Too long (>8 words)
- ❌ Too generic (could apply to many issues)
- ❌ Includes solution (title describes problem, not fix)

### Title Patterns by Issue Type

**Validation Missing:**
- `{Component} {Validation Type} Missing`
- Example: "Context File Validation Missing"

**Logic Error:**
- `{Component} {Incorrect Behavior}`
- Example: "Autonomous Deferrals Without Approval"

**Workflow Violation:**
- `{Workflow Phase} {Problem}`
- Example: "Skill Execution Incomplete Workflow"

**Constraint Bypass:**
- `{Constraint Type} Bypass`
- Example: "Quality Gate Bypass"

**Pattern Violation:**
- `{Pattern} Violation in {Component}`
- Example: "Lean Orchestration Violation in QA Command"

---

## Issue Description Clarity

### Issue Description Requirements

**Must include:**
1. **What happened** - Specific behavior observed
2. **When happened** - Date/time or trigger (command invoked, story executed)
3. **Where happened** - Component (skill/command/subagent)
4. **Expected behavior** - What should have occurred
5. **Actual behavior** - What actually occurred
6. **Impact** - Consequences of the issue

### Issue Description Template

```markdown
## Issue Description

When {trigger event}, the {component} {incorrect behavior}. This resulted in {negative consequence}, violating the {principle/constraint}.

**What happened:** {Specific behavior}
**When:** {Date or trigger}
**Where:** {Component}
**Expected:** {What should have happened}
**Actual:** {What actually happened}
**Impact:** {Consequences}
```

### Clarity Guidelines

**Be specific:**
```
❌ WRONG: "The development workflow had issues"
✅ CORRECT: "When /dev STORY-042 was executed, the development workflow did not validate context files existed before starting TDD"
```

**Be factual:**
```
❌ WRONG: "Claude probably forgot to check context files"
✅ CORRECT: "Phase 0 of devforgeai-development skill does not include a context file validation step"
```

**Be outcome-focused:**
```
❌ WRONG: "There was a bug in the code"
✅ CORRECT: "TDD implementation proceeded without architectural constraints, resulting in QA failure"
```

### Example Issue Descriptions

**Example 1: Validation Missing**
```markdown
When `/dev STORY-042` was executed, the development workflow did not validate that all 6 context files existed before starting the TDD cycle. This resulted in TDD implementation without architectural constraints, violating the spec-driven development principle.

**What happened:** TDD started without checking for context files
**When:** /dev STORY-042 invocation
**Where:** devforgeai-development skill, Phase 0
**Expected:** Phase 0 validates context files, halts if missing
**Actual:** Phase 0 skipped context validation, proceeded to TDD
**Impact:** Implementation proceeded without constraints, caused QA failure
```

**Example 2: Autonomous Operation**
```markdown
When `/dev STORY-008.1` was invoked, the development workflow accepted 3 pre-existing deferrals from the story template without challenging them or requiring user approval. This allowed autonomous deferrals, violating the "attempt first, defer only if blocked" principle and RCA-006 resolution.

**What happened:** Pre-existing deferrals accepted without challenge
**When:** /dev STORY-008.1 execution, Phase 5 (DoD validation)
**Where:** devforgeai-development skill, DoD validation logic
**Expected:** All deferrals challenged, user approval required
**Actual:** Pre-justified deferrals skipped, no user approval
**Impact:** Story marked "Dev Complete" with untested deferrals
```

---

## 5 Whys Formatting

### Formatting Standards

**Each "Why" must include:**
1. Question (bold)
2. Answer (clear, evidence-based)
3. Evidence reference (optional but recommended)

**Format Pattern:**
```markdown
1. **Why did {event} happen?**
   - {Answer with evidence reference}

2. **Why did {answer 1} occur?**
   - {Deeper answer with evidence}

...

5. **Why did {answer 4} occur?**
   - **ROOT CAUSE:** {Fundamental underlying issue}
```

### Question Formatting

**Bold the question:**
```markdown
✅ CORRECT: **Why did this happen?**
❌ WRONG: Why did this happen?
```

**Use past tense:**
```markdown
✅ CORRECT: Why did validation fail?
❌ WRONG: Why does validation fail?
```

**Reference previous answer:**
```markdown
1. Why did QA fail? → Coverage too low
2. **Why was coverage too low?** (references "coverage too low" from #1)
```

### Answer Formatting

**Bullet format:**
```markdown
1. **Why did this happen?**
   - {Answer here}
```

**Evidence inline (optional):**
```markdown
1. **Why did this happen?**
   - {Answer} (Evidence: SKILL.md:150-180 shows...)
```

**ROOT CAUSE emphasis:**
```markdown
5. **Why did {answer 4} occur?**
   - **ROOT CAUSE:** {Fundamental issue}
```

### Example 5 Whys Section

```markdown
## 5 Whys Analysis

**Issue:** /dev command accepted pre-existing deferrals without user approval

1. **Why did /dev accept deferrals without approval?**
   - Pre-existing deferrals in story template were accepted without challenge

2. **Why were pre-existing deferrals accepted without challenge?**
   - Development skill Phase 5 (DoD validation) was designed to skip items that already had justifications (Evidence: SKILL.md:450-480)

3. **Why did Phase 5 skip items with justifications?**
   - Workflow assumed justifications meant user had already approved in a previous QA retry iteration

4. **Why was that assumption made?**
   - Workflow designed for QA retry scenario where user approves deferrals, then /dev re-runs. Template pre-population scenario not considered.

5. **Why wasn't template pre-population scenario considered?**
   - **ROOT CAUSE:** Story template allows pre-populating deferrals before implementation, bypassing the approval requirement that Phase 5 was designed to enforce
```

---

## Evidence Section Organization

### Organization Principles

**Group evidence by type:**
1. Files Examined (primary evidence)
2. Context Files Validation (constraint compliance)
3. Workflow State Analysis (state machine validation)

**Order within each group:**
- Most significant first
- Related files together
- Chronological if showing progression

### Files Examined Format

**Template per file:**
```markdown
**{File Path}**
- **Lines examined:** {line range}
- **Finding:** {discovery summary}
- **Excerpt:**
  ```
  {relevant code/text with line numbers}
  ```
- **Significance:** {why this matters for RCA}
```

**Excerpt inclusion criteria:**
- Include if CRITICAL or HIGH significance
- Include if directly supports recommendation
- Omit if LOW significance or redundant

**Excerpt formatting:**
- Use code fence with language hint (```markdown, ```python, etc.)
- Include line numbers if critical
- Keep to 10-30 lines (balance detail vs readability)

### Context Files Validation Format

**Checklist template:**
```markdown
**Files checked:**
- [x] tech-stack.md - {PASS|FAIL} - {note}
- [x] source-tree.md - {PASS|FAIL} - {note}
- [x] dependencies.md - {PASS|FAIL} - {note}
- [x] coding-standards.md - {PASS|FAIL} - {note}
- [x] architecture-constraints.md - {PASS|FAIL} - {note}
- [x] anti-patterns.md - {PASS|FAIL} - {note}

**Violations found:** {list each violation OR "None"}
```

**Violation detail template:**
```markdown
1. **{Context file} violation:** {Brief description}
   - File: {violating file}:{line}
   - Code: `{problematic code snippet}`
   - Constraint: "{constraint text from context file}"
```

---

## Recommendation Prioritization

### Section Organization

**Four priority sections (always in this order):**
1. CRITICAL Priority (Implement Immediately)
2. HIGH Priority (Implement This Sprint)
3. MEDIUM Priority (Next Sprint)
4. LOW Priority (Backlog)

**Each section:**
- May have 0-N recommendations
- If 0 recommendations, include section with "None"
- List recommendations in logical order (dependency order if dependencies exist)

### Recommendation Ordering Within Priority

**Primary order:** By dependency
- Rec 1: Foundation (no dependencies)
- Rec 2: Depends on Rec 1
- Rec 3: Depends on Rec 1
- Rec 4: Depends on Rec 2 & 3

**Secondary order:** By effort (within same dependency level)
- Quick fixes first (encourages progress)
- Complex changes last

### Example Prioritization

```markdown
## Recommendations (Evidence-Based)

### CRITICAL Priority (Implement Immediately)

**Recommendation 1: Add Context File Validation to Phase 0**
{Full recommendation}

### HIGH Priority (Implement This Sprint)

**Recommendation 2: Update Pre-Flight Validation Reference**
{Full recommendation}
(Depends on: Rec 1)

**Recommendation 3: Add Test Case for Missing Context Files**
{Full recommendation}
(Depends on: Rec 1)

### MEDIUM Priority (Next Sprint)

None

### LOW Priority (Backlog)

**Recommendation 4: Add Context File Example to Documentation**
{Full recommendation}
```

---

## Implementation Checklist Generation

### Checklist Purpose

**Auto-generate action items from recommendations:**
- Provides quick scan of all work
- Ensures nothing forgotten
- Tracks progress during implementation

### Checklist Structure

**General items (always include):**
```markdown
- [ ] Review all recommendations
- [ ] Prioritize by impact/effort
- [ ] Create story for CRITICAL items (if substantial work)
```

**Recommendation-derived items:**
```markdown
- [ ] Implement REC-1: {Brief title}
- [ ] Implement REC-2: {Brief title}
- [ ] Test REC-1 and REC-2
```

**Documentation items (conditional):**
```markdown
- [ ] Update CLAUDE.md (if framework principle change)
- [ ] Update protocol (if process change)
- [ ] Update memory files (if command/skill/subagent change)
```

**Testing items:**
```markdown
- [ ] Add regression tests for issue
- [ ] Verify fix prevents recurrence
```

**Completion items:**
```markdown
- [ ] Mark RCA as RESOLVED
- [ ] Document lessons learned
- [ ] Commit changes
```

### Checklist Generation Algorithm

```
FOR each recommendation:
  IF priority == CRITICAL:
    Add: "Implement REC-{N}: {title}"
    Add: "Test REC-{N}"

FOR each recommendation:
  IF affects CLAUDE.md OR protocol OR memory:
    Add: "Update {file}"

IF recommendations modify skill/command/subagent:
  Add: "Update memory files"

IF testing recommendations exist:
  Add: "Add regression tests"

Add: "Mark RCA as RESOLVED"
Add: "Commit changes"
```

---

## Prevention Strategy Formulation

### Prevention Strategy Components

**Short-term (Immediate):**
- Tactical fixes from CRITICAL recommendations
- Immediate actions to prevent recurrence TODAY
- Usually: Implement the fix, add validation, update documentation

**Long-term (Framework Enhancement):**
- Strategic improvements from HIGH/MEDIUM recommendations
- Systematic changes to prevent class of issues
- Usually: Pattern updates, protocol enhancements, architecture improvements

**Monitoring:**
- How to detect if issue recurs
- What to watch for
- When to audit

### Short-Term Strategy

**Focus:** Immediate fixes

**Template:**
```markdown
**Short-term (Immediate):**
- Implement REC-1: {Primary fix}
- Add validation: {Specific check}
- Update: {Specific file with fix}
- Test: {Verification approach}
```

**Example:**
```markdown
**Short-term (Immediate):**
- Implement REC-1: Add context file validation to Phase 0
- Add validation: Check for 6 context files before TDD
- Update: devforgeai-development SKILL.md Phase 0
- Test: Run /dev without context files, verify halt with error
```

### Long-Term Strategy

**Focus:** Systematic improvements

**Template:**
```markdown
**Long-term (Framework Enhancement):**
- Pattern: {Systematic change}
- Process: {Workflow improvement}
- Architecture: {Structural change}
- Education: {Documentation improvement}
```

**Example:**
```markdown
**Long-term (Framework Enhancement):**
- Pattern: All skills that depend on context files add pre-flight validation
- Process: Create context-file-validator subagent for reusable validation
- Architecture: Define "context-dependent" skill trait with validation requirement
- Education: Update framework onboarding to explain context file criticality
```

### Monitoring Strategy

**Focus:** Detection and prevention

**Template:**
```markdown
**Monitoring:**
- Watch for: {Specific error pattern}
- Track: {Metric or frequency}
- Audit: {Periodic check}
- Alert: {When to escalate}
```

**Example:**
```markdown
**Monitoring:**
- Watch for: "Context files missing" errors in /dev workflow
- Track: How often users skip /create-context before /dev
- Audit: Monthly review of greenfield project onboarding success rate
- Alert: If >3 occurrences per month, investigate onboarding UX
```

---

## Related RCAs Linking

### Pattern Recognition

**Automatic linking criteria:**
- Similar root cause
- Same component affected
- Same workflow phase
- Related recommendations

**Manual linking criteria:**
- Different manifestation of same issue
- Follow-up to previous RCA
- Part of larger pattern

### Linking Format

```markdown
## Related RCAs

- **RCA-006:** Autonomous Deferrals Prevention (similar validation bypass pattern)
- **RCA-003:** Empty Git Repository (similar pre-flight validation issue)
- **RCA-009:** Skill Execution Incomplete Workflow (same skill affected)
```

**Include:**
- RCA number
- Title
- Relationship description (why related)

---

## Document Quality Checklist

Before finalizing RCA document, verify:

**Structure:**
- [ ] All 8 required sections present
- [ ] Sections in correct order
- [ ] Header metadata complete (date, component, severity)
- [ ] Title follows conventions (3-6 words)

**Content:**
- [ ] Issue description clear and specific
- [ ] All 5 Whys answered with evidence
- [ ] Evidence section comprehensive (3+ files)
- [ ] Recommendations prioritized correctly
- [ ] All recommendations have exact implementation
- [ ] Implementation checklist derived from recommendations
- [ ] Prevention strategy has short-term and long-term

**Quality:**
- [ ] No aspirational content (all evidence-based)
- [ ] No vague recommendations ("improve validation")
- [ ] All file paths are absolute and correct
- [ ] All code/text is copy-paste ready
- [ ] Testing procedures are clear
- [ ] Effort estimates are realistic

**Completeness:**
- [ ] All placeholders replaced (no {PLACEHOLDER} remaining)
- [ ] Related RCAs linked (if any)
- [ ] RCA number is correct (sequential)
- [ ] File naming matches RCA number

---

## RCA Document Examples

### Minimal Complete RCA (Small Issue)

```markdown
# RCA-010: Missing Error Message

**Date:** 2025-11-16
**Reported By:** User
**Affected Component:** devforgeai-qa skill
**Severity:** LOW

---

## Issue Description

When QA validation fails due to missing story file, the error message does not include the story ID. This causes minor user confusion.

---

## 5 Whys Analysis

**Issue:** Error message missing story ID

1. **Why was story ID missing?**
   - Error template doesn't include {STORY_ID} placeholder

2. **Why doesn't template include it?**
   - Template was copied from generic error, not customized

3. **Why wasn't it customized?**
   - No review of error messages during implementation

4. **Why no review?**
   - Error message review not in implementation checklist

5. **Why not in checklist?**
   - **ROOT CAUSE:** No error message quality standard in coding-standards.md

---

## Evidence Collected

**.claude/skills/devforgeai-qa/SKILL.md**
- **Lines:** 500-520
- **Finding:** Error template: "Story file not found"
- **Excerpt:** `Display: "Story file not found"`
- **Significance:** Missing {STORY_ID} in message

---

## Recommendations (Evidence-Based)

### CRITICAL Priority
None

### HIGH Priority
None

### MEDIUM Priority
None

### LOW Priority

**Recommendation 1: Add Story ID to Error Message**
- **File:** `.claude/skills/devforgeai-qa/SKILL.md`
- **Section:** Phase 0, error handling
- **Change:** Modify error template

**Modify from:**
```
Display: "Story file not found"
```

**Modify to:**
```
Display: "Story file not found: {STORY_ID}"
```

**Rationale:** Improves error clarity
**Testing:** Trigger error, verify ID included
**Effort:** 5 minutes

---

## Implementation Checklist

- [ ] Update error message template
- [ ] Test error display

---

## Prevention Strategy

**Short-term:** Update error message
**Long-term:** Add error message standard to coding-standards.md
**Monitoring:** Review error messages during code review

---

## Related RCAs

None
```

### Comprehensive RCA (Complex Issue)

**See:** `devforgeai/RCA/RCA-006-autonomous-deferrals.md` for full example with:
- Timeline of events (chronological incident)
- Contributing factors (multiple causes)
- Multi-phase recommendations (Phase 1 CRITICAL, Phase 2 HIGH)
- Detailed testing strategy
- Framework-wide impact analysis

---

## Reference

**RCA Document Examples:**
- `devforgeai/RCA/RCA-006-autonomous-deferrals.md` - Multi-phase, complex
- `devforgeai/RCA/RCA-007-multi-file-story-creation.md` - 3-phase solution
- `devforgeai/RCA/RCA-008-autonomous-git-stashing.md` - User approval pattern
- `devforgeai/RCA/RCA-009-skill-execution-incomplete-workflow.md` - Skill refactoring

**Related DevForgeAI Documentation:**
- `.claude/skills/devforgeai-rca/assets/rca-document-template.md` - Base template
- `.claude/skills/devforgeai-rca/references/recommendation-framework.md` - Recommendation details
- `CLAUDE.md` - Framework principles

---

**End of RCA Writing Guide**

**Total: ~600 lines**
