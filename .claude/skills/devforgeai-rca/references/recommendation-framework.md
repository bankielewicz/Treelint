# Recommendation Framework for DevForgeAI RCA

**Purpose:** Guide for generating actionable, evidence-based recommendations from root cause analysis

**Target Audience:** devforgeai-rca skill (Claude executing RCA Phase 4)

---

## Table of Contents

1. [Priority Criteria](#priority-criteria)
2. [Evidence-Based Recommendation Structure](#evidence-based-recommendation-structure)
3. [Implementation Detail Requirements](#implementation-detail-requirements)
4. [Rationale Writing Guidelines](#rationale-writing-guidelines)
5. [Testing Procedure Specifications](#testing-procedure-specifications)
6. [Effort Estimation Methodology](#effort-estimation-methodology)
7. [Impact Analysis Framework](#impact-analysis-framework)

---

## Priority Criteria

### CRITICAL Priority (Implement Immediately)

**Criteria for CRITICAL:**
- **Severity:** Framework broken, blocking work
- **Impact:** Prevents normal development workflow
- **Scope:** Affects all users or all stories
- **Risk:** Causes data loss, silent failures, or quality gate bypass
- **Urgency:** Must fix before next framework usage

**Examples:**
- Quality gate bypass (allows broken code to proceed)
- Silent data loss (files deleted without warning)
- Constraint validation disabled (spec-driven development broken)
- Infinite loops (workflow never completes)
- Autonomous operations violating user control

**Recommendation Pattern:**
```markdown
**Recommendation 1: {Fix Framework-Breaking Issue}**
**Priority:** CRITICAL

**Problem:** Framework cannot function correctly without this fix

**Solution:** Add missing validation/fix broken logic

**File:** Exact path to skill/command/subagent
**Section:** Specific phase/step
**Change:** Add validation checkpoint / Fix logic bug

**Effort:** <4 hours
**Impact:** Unblocks framework, prevents data loss
```

### HIGH Priority (Implement This Sprint)

**Criteria for HIGH:**
- **Severity:** Significant impact on workflow
- **Impact:** Affects specific workflows or scenarios
- **Scope:** Multiple stories or users affected
- **Risk:** Quality degradation, technical debt accumulation
- **Urgency:** Fix within current sprint (1-2 weeks)

**Examples:**
- Missing validation allowing violations
- Documentation gaps causing confusion
- Performance degradation (>2x slower)
- Error messages unclear or missing
- Reference files missing framework constraints

**Recommendation Pattern:**
```markdown
**Recommendation 2: {Improve Workflow Quality}**
**Priority:** HIGH

**Problem:** Workflow functions but quality/usability degraded

**Solution:** Add missing check/documentation/reference

**File:** Exact path
**Section:** Specific location
**Change:** Add validation / Update documentation

**Effort:** 1-4 hours
**Impact:** Improves quality, prevents future issues
```

### MEDIUM Priority (Next Sprint)

**Criteria for MEDIUM:**
- **Severity:** Quality improvement opportunity
- **Impact:** Affects edge cases or specific scenarios
- **Scope:** Limited to specific component
- **Risk:** Minor technical debt, usability friction
- **Urgency:** Fix in next sprint (2-4 weeks)

**Examples:**
- Missing edge case handling
- Documentation could be clearer
- Minor refactoring opportunity
- Test coverage gaps (non-critical paths)
- Optimization potential

**Recommendation Pattern:**
```markdown
**Recommendation 3: {Enhance Component}**
**Priority:** MEDIUM

**Problem:** Component works but could be better

**Solution:** Refactor / Add documentation / Improve tests

**File:** Exact path
**Section:** Specific location
**Change:** Refactor code / Add section

**Effort:** 2-8 hours
**Impact:** Better quality, reduced friction
```

### LOW Priority (Backlog)

**Criteria for LOW:**
- **Severity:** Nice-to-have improvement
- **Impact:** Minimal or cosmetic
- **Scope:** Very narrow (single file/function)
- **Risk:** No risk if not fixed
- **Urgency:** Opportunistic (fix when convenient)

**Examples:**
- Cosmetic improvements (formatting)
- Verbose logging could be reduced
- Comment clarity
- Minor code duplication
- Documentation typos

**Recommendation Pattern:**
```markdown
**Recommendation 4: {Minor Improvement}**
**Priority:** LOW

**Problem:** Minor quality or clarity issue

**Solution:** Small fix, no urgency

**File:** Exact path
**Section:** Specific location
**Change:** Update text / Refactor

**Effort:** <1 hour
**Impact:** Minor quality improvement
```

---

## Evidence-Based Recommendation Structure

### Constitutional Principle

**All recommendations MUST be evidence-based:**
- ❌ "We should probably add validation" (aspirational)
- ✅ "Evidence shows Phase 0 lacks validation (line 150). Add validation checkpoint." (evidence-based)

**Evidence Requirements:**
1. **Proof of problem:** File shows missing/incorrect behavior
2. **Proof of impact:** Demonstrates negative consequence
3. **Proof of solution:** Shows similar pattern working elsewhere OR proves solution feasibility

### Recommendation Anatomy

**Complete recommendation has 7 components:**

1. **Title:** Brief, descriptive (5-10 words)
2. **Problem Addressed:** Which root cause/factor this fixes
3. **Proposed Solution:** What to implement (1-2 sentences)
4. **Implementation Details:** Exact file/section/code
5. **Rationale:** Why this works (evidence-based)
6. **Testing:** How to verify fix
7. **Effort/Impact:** Time estimate and benefit

**Missing ANY component = Incomplete recommendation**

### Quality Checklist

Before finalizing recommendation, verify:
- [ ] Title is clear and specific
- [ ] Problem links to root cause from 5 Whys
- [ ] Solution is concrete (not vague)
- [ ] File path is absolute and correct
- [ ] Section/phase is specific (not "somewhere in the file")
- [ ] Code/text is copy-paste ready (exact)
- [ ] Rationale references evidence from Phase 3
- [ ] Testing has 3+ verification steps
- [ ] Effort estimate is realistic
- [ ] Impact describes benefit

---

## Implementation Detail Requirements

### Exact File Paths

**ALWAYS provide:**
- Absolute path from repo root
- No ambiguity (not "the skill file" but `.claude/skills/devforgeai-development/SKILL.md`)
- Correct file extension

**Examples:**
```markdown
✅ CORRECT:
**File:** `.claude/skills/devforgeai-development/SKILL.md`
**File:** `.claude/commands/dev.md`
**File:** `devforgeai/specs/context/tech-stack.md`

❌ WRONG:
**File:** "the development skill"
**File:** "SKILL.md"
**File:** "dev command"
```

### Specific Sections

**ALWAYS provide:**
- Phase number (if skill)
- Step number (if within phase)
- Line range (if known)
- Descriptive section name

**Examples:**
```markdown
✅ CORRECT:
**Section:** Phase 0, after Step 7 (Tech Stack Detection)
**Section:** Lines 150-180 (Phase 0: Pre-Flight Validation)
**Section:** Phase 4: Recommendation Generation, Step 3 (Prioritization)

❌ WRONG:
**Section:** "Early in the file"
**Section:** "Validation section"
**Section:** "Around line 150"
```

### Copy-Paste Ready Code/Text

**ALWAYS provide exact text to add:**

**For new code/text:**
```markdown
**Exact text to add:**
```markdown
**Step 8: Context File Validation**

Validate all 6 context files exist:

```
Glob(pattern="devforgeai/specs/context/*.md")

IF result.length < 6:
  HALT workflow
  Display error message with missing files
  EXIT
```
```
```

**For modifications:**
```markdown
**Modify from:**
```markdown
for item in definition_of_done:
    if item.has_justification():
        continue  # Skip validated items
```

**Modify to:**
```markdown
for item in definition_of_done:
    # NEVER skip items with justifications
    # All deferrals require user approval (RCA-006)
    validate_item_or_challenge_deferral(item)
```
```

**For deletions:**
```markdown
**Delete:**
```markdown
Lines 200-250: Remove autonomous validation skip logic
```
```

### Implementation Change Types

**Add:** New section/step/validation
**Modify:** Change existing logic/text
**Delete:** Remove problematic code
**Refactor:** Restructure without behavior change
**Extract:** Move to different file (skill → subagent)

---

## Rationale Writing Guidelines

### Rationale Components

**Complete rationale answers:**
1. **Why this solution?** (Why not alternatives?)
2. **How does it prevent recurrence?** (Mechanism)
3. **What evidence supports this?** (References)
4. **What are trade-offs?** (If any)

### Rationale Quality Criteria

**Strong rationale:**
- ✅ References specific evidence from Phase 3
- ✅ Explains prevention mechanism
- ✅ Considers alternatives (if any)
- ✅ Acknowledges trade-offs
- ✅ Cites DevForgeAI patterns/protocols

**Weak rationale:**
- ❌ Generic (could apply to anything)
- ❌ No evidence references
- ❌ Doesn't explain prevention
- ❌ Ignores trade-offs

**Example - Strong Rationale:**
```markdown
### Rationale

Adding context file validation to Phase 0 prevents TDD from starting without architectural constraints (spec-driven principle).

**Evidence supporting this approach:**
- Evidence shows Phase 0 has pre-flight checks (Git, tech stack)
- Evidence shows context files are critical (6 immutable constraints)
- Evidence shows validation is fast (Glob operation, <100ms)
- Pattern from RCA-003: Similar pre-flight validation for Git repository

**Prevention mechanism:**
Validation occurs BEFORE any code written (Phase 0), ensuring 100% compliance. Early detection provides clear user guidance (run /create-context).

**Trade-offs:**
- Adds 15-20 lines to Phase 0
- Adds ~200ms to /dev startup
- Blocks greenfield /dev usage (requires /create-context first)

Trade-offs acceptable because:
- Spec-driven development requires constraints
- Early failure better than late QA failure
- Clear error message guides user to fix
```

### Rationale Formula

```
{Prevention explanation}

**Evidence supporting this approach:**
- {Evidence reference 1}
- {Evidence reference 2}
- {Pattern from existing RCA/protocol}

**Prevention mechanism:**
{How exactly this stops recurrence}

**Trade-offs:**
- {Trade-off 1}
- {Trade-off 2}

{Why trade-offs acceptable}
```

---

## Testing Procedure Specifications

### Testing Requirements

**All recommendations MUST include:**
- 3+ verification steps
- Expected outcome
- Success criteria
- Failure indicators

### Testing Procedure Format

```markdown
### Testing

**How to verify fix:**
1. {Step 1 - Setup/precondition}
2. {Step 2 - Execute action}
3. {Step 3 - Verify result}
4. {Step 4 - Additional verification if needed}

**Expected outcome:**
{Clear statement of what should happen}

**Success criteria:**
- {Criterion 1}
- {Criterion 2}

**Failure indicators:**
- {What indicates fix didn't work}
```

### Testing Categories

**Unit Testing (Component-Level):**
- Test the changed component in isolation
- Verify fix works as intended
- No side effects introduced

**Integration Testing (Workflow-Level):**
- Test complete workflow with fix
- Verify fix doesn't break other components
- Workflow progresses correctly

**Regression Testing (Framework-Level):**
- Run existing test suite
- All previous tests still pass
- No new failures introduced

### Example Testing Procedures

**Example 1: Validation Addition**
```markdown
### Testing

**How to verify fix:**
1. Create test project without context files (rm -rf devforgeai/specs/context/)
2. Create test story (STORY-TEST-001.story.md)
3. Run: /dev STORY-TEST-001
4. Expected: Workflow halts at Phase 0 with error message
5. Verify error message lists missing context files
6. Run: /create-context test-project
7. Run: /dev STORY-TEST-001
8. Expected: Workflow proceeds to TDD normally

**Expected outcome:**
Context file validation blocks /dev when files missing, provides clear error with resolution steps

**Success criteria:**
- [ ] /dev halts when context files missing
- [ ] Error message displays missing file names
- [ ] Error message guides user to /create-context
- [ ] /dev succeeds after context files created

**Failure indicators:**
- /dev proceeds without context files (validation not working)
- No error message displayed
- Workflow crashes instead of halting gracefully
```

**Example 2: Logic Fix**
```markdown
### Testing

**How to verify fix:**
1. Create story with pre-existing deferred items
2. Add justifications to deferrals (simulate template)
3. Run: /dev {STORY-ID}
4. Expected: Phase 4.5 challenges ALL deferrals, requests user approval
5. Verify: AskUserQuestion triggered for each deferral
6. Select: "Defer (blocker exists)"
7. Verify: Timestamp added to approval
8. Check: Story file has approval markers

**Expected outcome:**
All deferrals (pre-existing + new) require user approval, zero autonomous deferrals

**Success criteria:**
- [ ] Pre-existing deferrals challenged (not skipped)
- [ ] User approval requested for each deferral
- [ ] Timestamp recorded in story file
- [ ] deferral-validator subagent invoked

**Failure indicators:**
- Pre-existing deferrals accepted without challenge
- No AskUserQuestion for deferrals
- No timestamps in story file
```

---

## Effort Estimation Methodology

### Estimation Formula

**Base Effort = Change Complexity + Testing Time + Documentation Time**

**Change Complexity:**
- Add simple validation (1 step): 15-30 min
- Add complex validation (multi-step): 30-60 min
- Modify logic (simple): 30-60 min
- Modify logic (complex): 1-2 hours
- Refactor (small): 1-2 hours
- Refactor (large): 2-4 hours
- Create new subagent: 2-3 hours
- Create new reference file: 1-2 hours

**Testing Time:**
- Unit tests: 15-30 min
- Integration tests: 30-60 min
- Regression tests: 15-30 min
- Total testing: ~1 hour typical

**Documentation Time:**
- Update CLAUDE.md: 15 min
- Update protocol: 15-30 min
- Update reference file: 30-60 min
- Total documentation: ~30-60 min

### Complexity Factors

**Low Complexity:**
- Single file change
- Add new section/step
- No dependencies
- Clear insertion point
- No logic changes

**Medium Complexity:**
- 2-3 file changes
- Modify existing logic
- Some dependencies
- Requires testing multiple scenarios
- Minor refactoring

**High Complexity:**
- 5+ file changes
- Complex logic modification
- Many dependencies
- Requires new subagent/reference
- Major refactoring

### Effort Categories

**Quick (<1 hour):**
- Add simple validation step
- Update documentation
- Fix typos/clarity
- Add comments

**Standard (1-4 hours):**
- Add validation checkpoint
- Update skill phase
- Create reference file
- Moderate refactoring

**Substantial (4-8 hours):**
- Create new subagent
- Major skill refactoring
- New command creation
- Complex logic changes

**Major (>8 hours):**
- Framework-wide changes
- Multiple skill updates
- New workflow phases
- Architecture changes

---

## Impact Analysis Framework

### Impact Dimensions

**1. Benefit (What Improves)**

**Framework Quality:**
- Prevents recurrence of issue
- Improves reliability
- Reduces false positives/negatives
- Better error messages

**User Experience:**
- Clearer guidance
- Faster workflow
- Fewer errors
- Better documentation

**Code Quality:**
- Better architecture
- Reduced technical debt
- Improved maintainability
- Better test coverage

**2. Risk (What Could Go Wrong)**

**Breaking Changes:**
- Could break existing workflows
- Could require user re-training
- Could invalidate existing stories

**Performance:**
- Could slow down workflows
- Could increase token usage
- Could increase complexity

**Maintenance:**
- Could add maintenance burden
- Could create new edge cases
- Could require ongoing updates

**3. Scope (What's Affected)**

**Component-Level:**
- Single skill/command/subagent
- Single workflow phase
- Single reference file

**Workflow-Level:**
- Multiple skills
- Complete workflow (dev → qa → release)
- User-facing commands

**Framework-Level:**
- All skills
- Core protocols
- Breaking changes to API

### Impact Assessment Template

```markdown
### Impact

**Benefit:**
- {Primary benefit}
- {Secondary benefit}
- {User-facing improvement}

**Risk:**
- {Primary risk}
- {Mitigation strategy for risk}

**Scope:**
- Files affected: {count}
- Workflows affected: {list}
- Users affected: {all/some/specific scenario}
```

---

## Complete Recommendation Example

```markdown
**Recommendation 1: Add Context File Validation to Phase 0**
**Priority:** CRITICAL

### Problem Addressed

ROOT CAUSE: No pre-flight validation in development skill enforces context file existence before TDD begins (from 5 Whys Analysis, Why #5)

### Proposed Solution

Add context file validation step to devforgeai-development skill Phase 0 (Pre-Flight Validation), after Step 7 (Tech Stack Detection). Validation checks for 6 required context files before allowing TDD to proceed.

### Implementation Details

**File:** `.claude/skills/devforgeai-development/SKILL.md`
**Section:** Phase 0: Pre-Flight Validation, after Step 7
**Change Type:** Add

**Exact text to add:**
```markdown
**Step 8: Context File Validation**

Validate all 6 context files exist before proceeding:

```
Glob(pattern="devforgeai/specs/context/*.md")

EXPECTED_FILES = [
  "tech-stack.md",
  "source-tree.md",
  "dependencies.md",
  "coding-standards.md",
  "architecture-constraints.md",
  "anti-patterns.md"
]

IF result.length < 6:
  MISSING = find_missing_files(EXPECTED_FILES, result)

  HALT workflow

  Display:
  "❌ Context files missing. Cannot proceed with TDD.

  Missing files:
  {list MISSING files}

  Context files are required for spec-driven development.
  They define architectural constraints that TDD must follow.

  Run: /create-context {project-name}

  Then retry: /dev {STORY-ID}"

  EXIT workflow
```
```

### Rationale

Adding context file validation to Phase 0 prevents TDD from starting without architectural constraints, enforcing the spec-driven development principle.

**Evidence supporting this approach:**
- Evidence from Phase 3 shows Phase 0 has pre-flight checks (Git validation at Step 1, tech detection at Step 7)
- Evidence from devforgeai/specs/context/ shows 6 immutable constraint files exist and are critical
- Evidence from RCA-003 shows similar pre-flight validation for Git repository worked effectively
- Pattern from devforgeai-architecture: Context files created before development

**Prevention mechanism:**
Validation occurs in Phase 0 (before TDD Red phase), ensuring 100% of TDD cycles operate within constraints. Early detection (before any code written) provides immediate, actionable user guidance (/create-context command).

**Trade-offs:**
- Adds 25-30 lines to Phase 0
- Adds ~200-300ms to /dev startup (Glob operation)
- Blocks greenfield /dev usage without /create-context first
- Requires users to run /create-context before first /dev

**Trade-offs acceptable because:**
- Spec-driven development cannot function without constraints
- Early failure (Phase 0) better than late failure (QA validation)
- Clear error message guides user to exact fix
- 200ms overhead negligible (<1% of typical /dev execution time)

### Testing

**How to verify fix:**
1. Create test directory: `mkdir test-project && cd test-project && git init`
2. Create story file without context: `devforgeai/specs/Stories/STORY-TEST-001.story.md`
3. Run: `/dev STORY-TEST-001`
4. Expected: Workflow halts at Phase 0 Step 8
5. Verify error message displays:
   - "Context files missing" heading
   - List of 6 missing files
   - "/create-context" command suggestion
6. Run: `/create-context test-project`
7. Run: `/dev STORY-TEST-001`
8. Expected: Workflow proceeds to Phase 1 (Red phase) normally

**Expected outcome:**
Context file validation blocks /dev when files missing, provides clear error with resolution steps (/create-context), allows /dev to proceed after files created

**Success criteria:**
- [ ] /dev halts at Phase 0 when context files missing
- [ ] Error message displays all 6 missing file names
- [ ] Error message includes /create-context suggestion
- [ ] Error message includes project context explanation
- [ ] /dev succeeds after /create-context run
- [ ] No false positives (doesn't block when files exist)

**Failure indicators:**
- /dev proceeds to TDD without context files (validation not working)
- No error message displayed when files missing
- Error message missing file list or guidance
- Workflow crashes instead of graceful halt
- False positive (blocks even when files exist)

### Effort Estimate

**Time:** 45 minutes
**Breakdown:**
- Add validation step to SKILL.md: 15 min
- Add error message formatting: 10 min
- Test with missing files: 10 min
- Test with files present: 5 min
- Regression test (existing /dev workflows): 5 min

**Complexity:** Low
- Single file change
- Add new step (no logic modification)
- Clear insertion point (after Step 7)
- No dependencies on other changes

**Dependencies:** None (standalone improvement)

### Impact

**Benefit:**
- Prevents TDD without architectural constraints (100% compliance)
- Early failure detection (Phase 0 vs QA validation)
- Clear user guidance (actionable error message)
- Enforces spec-driven development principle
- Reduces QA failures from missing constraints

**Risk:**
- Could frustrate users on greenfield projects (blocked without context)
- Mitigation: Error message clearly explains /create-context requirement

**Scope:**
- Files affected: 1 (devforgeai-development SKILL.md)
- Workflows affected: /dev command, /orchestrate command (uses /dev)
- Users affected: All users (anyone running /dev)
- Scenarios: Greenfield projects, projects without /create-context run
```

---

## Recommendation Prioritization Matrix

### Decision Tree

```
Does fix prevent CRITICAL framework failure?
├─ YES → CRITICAL priority
└─ NO → Does fix prevent quality degradation?
          ├─ YES → HIGH priority
          └─ NO → Does fix improve UX or quality?
                    ├─ YES → MEDIUM priority
                    └─ NO → LOW priority (backlog)
```

### Priority Assignment Checklist

**For each recommendation:**

**If ANY of these are TRUE → CRITICAL:**
- [ ] Framework broken without fix
- [ ] Data loss possible
- [ ] Quality gate bypass
- [ ] Silent failure (no error, wrong result)
- [ ] Blocks all users

**If ANY of these are TRUE → HIGH:**
- [ ] Workflow degraded significantly
- [ ] Quality violation possible
- [ ] Technical debt accumulates
- [ ] Affects multiple stories/users
- [ ] Recurrent issue

**If ANY of these are TRUE → MEDIUM:**
- [ ] Edge case unhandled
- [ ] Documentation gap
- [ ] Minor refactoring opportunity
- [ ] Test coverage gap (non-critical path)
- [ ] Usability improvement

**All others → LOW**

---

## Reference

**Related DevForgeAI Documentation:**
- `devforgeai/protocols/lean-orchestration-pattern.md` - Command/skill patterns
- `.claude/memory/skills-reference.md` - Skill patterns
- `.claude/memory/subagents-reference.md` - Subagent patterns
- `CLAUDE.md` - Framework overview, quality gates, workflow states

**RCA Recommendation Examples:**
- `devforgeai/RCA/RCA-006-autonomous-deferrals.md` - Phase 1 & 2 recommendations
- `devforgeai/RCA/RCA-007-multi-file-story-creation.md` - Multi-phase recommendations
- `devforgeai/RCA/RCA-008-autonomous-git-stashing.md` - Git workflow recommendations

---

**End of Recommendation Framework**

**Total: ~900 lines**
