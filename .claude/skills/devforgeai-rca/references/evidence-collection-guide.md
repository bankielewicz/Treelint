# Evidence Collection Guide for DevForgeAI RCA

**Purpose:** Guide for systematically collecting and organizing evidence during root cause analysis

**Target Audience:** devforgeai-rca skill (Claude executing RCA Phase 3)

---

## Table of Contents

1. [What to Examine](#what-to-examine)
2. [How to Read Files Systematically](#how-to-read-files-systematically)
3. [What Excerpts to Capture](#what-excerpts-to-capture)
4. [Determining Significance](#determining-significance)
5. [Evidence Organization](#evidence-organization)
6. [Context File Validation](#context-file-validation)
7. [Sufficiency Criteria](#sufficiency-criteria)

---

## What to Examine

### File Categories by Breakdown Type

**Skill Breakdown:**
- Primary: `.claude/skills/{skill}/SKILL.md`
- Secondary: `.claude/skills/{skill}/references/*.md`
- Tertiary: Subagents invoked by skill (`.claude/agents/*.md`)
- Context: Story files if skill operates on stories

**Command Breakdown:**
- Primary: `.claude/commands/{command}.md`
- Secondary: Skill invoked by command
- Tertiary: lean-orchestration-pattern.md (if pattern violation)
- Context: Example command usage from memory files

**Subagent Breakdown:**
- Primary: `.claude/agents/{subagent}.md`
- Secondary: Reference files for subagent (if exists)
- Tertiary: Skills that invoke this subagent
- Context: Framework integration points

**Context File Violation:**
- Primary: Violated context file (`devforgeai/specs/context/{file}.md`)
- Secondary: Code that violates constraint
- Tertiary: Validation code that should have caught it
- Context: ADRs related to constraint

**Workflow State Error:**
- Primary: Story file (`devforgeai/specs/Stories/{STORY-ID}.story.md`)
- Secondary: Skill that manages workflow state
- Tertiary: Workflow state documentation
- Context: orchestration skill

### Priority Matrix

**MUST examine (always):**
1. Component directly involved in breakdown (skill/command/subagent)
2. Files that component reads or modifies
3. Documentation that defines expected behavior

**SHOULD examine (usually):**
4. Related components (if skill invokes subagent, examine both)
5. Context files (if constraint-related issue)
6. Story files (if story-specific issue)
7. Recent RCAs for similar issues

**MAY examine (if needed):**
8. Git history (when was problematic code added?)
9. Test files (are there tests for this scenario?)
10. Protocol documents (is there guidance for this?)

### Examination Checklist

For each file category, check:
- [ ] Does this file exist?
- [ ] When was it last modified? (git log)
- [ ] What sections are relevant to the issue?
- [ ] Are there TODO/FIXME comments related?
- [ ] Does it reference other files?
- [ ] Are there recent changes (git diff)?

---

## How to Read Files Systematically

### Reading Strategy

**Step 1: Locate the File**
```
Glob(pattern=".claude/skills/devforgeai-{skill}/*.md")
# OR
Read(file_path=".claude/commands/{command}.md")
```

**Step 2: Identify Relevant Sections**
- For skills: Read YAML frontmatter, then relevant phase
- For commands: Read YAML frontmatter, then relevant phase
- For subagents: Read system prompt, then tool access
- For context files: Read constraint being violated

**Step 3: Extract Line Numbers**
- Use cat -n format (Read tool provides this)
- Note: Line numbers help with precise evidence

**Step 4: Capture Context**
- Read 10-20 lines before/after relevant section
- Understand surrounding logic/flow

### Pattern: Workflow Phase Examination

**Goal:** Find where workflow should have done X but didn't

**Example: Context file validation missing**

```
1. Read devforgeai-development SKILL.md
2. Find Phase 0 section (pre-flight validation)
3. Scan Phase 0 for "context" keyword
4. Find: No mention of context file validation
5. Capture: Lines 50-200 (entire Phase 0)
6. Note: Missing step between tech detection and TDD start
```

**Evidence to Record:**
```
File: .claude/skills/devforgeai-development/SKILL.md
Lines: 50-200 (Phase 0: Pre-Flight Validation)
Finding: No context file validation step
Excerpt: [Phase 0 section with missing step highlighted]
Significance: Explains why TDD started without constraints
```

### Pattern: Constraint Violation Examination

**Goal:** Find what constraint was violated and why

**Example: Tech stack constraint violated**

```
1. Read devforgeai/specs/context/tech-stack.md
2. Find constraint: "Frontend: React 18.2+"
3. Read implementation code
4. Find: Code uses Vue.js
5. Read validation code (context-validator subagent)
6. Find: Validator not invoked in workflow
7. Capture: All 3 files with line numbers
```

**Evidence to Record:**
```
File 1: devforgeai/specs/context/tech-stack.md
Lines: 15-20
Finding: Constraint specifies React
Excerpt: "## Frontend\n- Framework: React 18.2+"

File 2: src/components/App.vue
Lines: 1-5
Finding: Implementation uses Vue
Excerpt: "<template>\n  <div id=\"app\">..."

File 3: .claude/agents/context-validator.md
Lines: 100-150
Finding: Validator can detect this, but not invoked
Excerpt: "Check package.json for framework..."
```

### Pattern: Workflow State Examination

**Goal:** Find expected state vs actual state

**Example: Story progressed without QA approval**

```
1. Read story file YAML frontmatter
2. Find: status: "Releasing"
3. Read workflow states documentation
4. Find: Expected previous state = "QA Approved"
5. Read story Workflow History section
6. Find: No "QA Approved" transition
7. Read orchestration skill release phase
8. Find: No validation of previous state
```

**Evidence to Record:**
```
File 1: devforgeai/specs/Stories/STORY-042.story.md (YAML)
Lines: 1-15
Finding: Status = Releasing without QA Approved
Excerpt: "status: Releasing..."

File 2: Workflow History section
Lines: 500-550
Finding: No QA transition recorded
Excerpt: [History section]

File 3: devforgeai-orchestration SKILL.md
Lines: 800-850
Finding: Release phase doesn't validate previous state
Excerpt: [Release phase section]
```

---

## What Excerpts to Capture

### Excerpt Quality Criteria

**Good excerpts:**
- ✅ Self-contained (readable without full file context)
- ✅ Precisely relevant (directly shows issue)
- ✅ Appropriate length (10-30 lines typically)
- ✅ Include line numbers
- ✅ Show surrounding context (not just problematic line)

**Poor excerpts:**
- ❌ Too short (single line out of context)
- ❌ Too long (entire 500-line file)
- ❌ Missing context (can't understand what it's doing)
- ❌ No line numbers (can't verify)

### Excerpt Length Guidelines

**For missing code:**
- Capture 20-30 lines showing WHERE code should be
- Highlight gap with comment

**Example:**
```markdown
Lines 150-180: Phase 0 validation steps

```
Step 6: Validate story file exists
Step 7: Detect technology stack

[MISSING: Context file validation step should be here]

Step 8: Begin TDD Red phase
```
```

**For incorrect code:**
- Capture problematic code + 10 lines before/after
- Show what's wrong

**Example:**
```markdown
Lines 200-230: DoD validation logic

```python
for item in definition_of_done:
    if item.has_justification():
        # BUG: Skips validation if justification exists
        # Allows pre-justified deferrals to bypass approval
        continue
    validate_item_completed(item)
```
```

**For missing constraint:**
- Capture constraint definition
- Capture code that violates it
- Capture validator that should catch it

---

## Determining Significance

### Significance Formula

**For each piece of evidence, answer:**

**Q1: How does this relate to the root cause?**
- Demonstrates the gap/bug/violation directly
- Shows why the gap exists
- Proves the root cause hypothesis

**Q2: Would the issue have been prevented if this were different?**
- If YES → Highly significant
- If MAYBE → Moderately significant
- If NO → Low significance, may omit

**Q3: Does this support a recommendation?**
- Evidence that will inform the fix
- Shows exact location to modify
- Demonstrates impact of change

### Significance Categories

**CRITICAL Significance:**
- Directly demonstrates root cause
- Shows exact code/text to fix
- No issue without this evidence

**HIGH Significance:**
- Strongly supports root cause
- Shows contributing factor
- Helpful for understanding impact

**MEDIUM Significance:**
- Provides context
- Supports secondary hypothesis
- Useful but not essential

**LOW Significance:**
- Background information
- Related but not directly causal
- Could omit without losing clarity

**Example Significance Assessment:**
```
Evidence: SKILL.md Phase 0 doesn't mention context validation
Significance: CRITICAL
Rationale:
  - Q1: Directly shows missing validation step
  - Q2: If validation existed, issue prevented
  - Q3: Supports Recommendation 1 (add validation)
  - Impact: Without this evidence, can't prove root cause
```

---

## Evidence Organization

### Evidence Document Structure

**Section 1: Files Examined**

For each file:
```markdown
**{Full File Path}**
- **Lines examined:** {range}
- **Finding:** {what was discovered}
- **Excerpt:**
  ```
  {relevant code/text}
  ```
- **Significance:** {why this matters}
```

**Section 2: Context Files Validation**

```markdown
**Files checked:**
- [ ] tech-stack.md - {PASS/FAIL} - {note}
- [ ] source-tree.md - {PASS/FAIL} - {note}
- [ ] dependencies.md - {PASS/FAIL} - {note}
- [ ] coding-standards.md - {PASS/FAIL} - {note}
- [ ] architecture-constraints.md - {PASS/FAIL} - {note}
- [ ] anti-patterns.md - {PASS/FAIL} - {note}

**Violations found:** {list or "None"}
```

**Section 3: Workflow State Analysis**

```markdown
**Expected state:** {what should have happened}
**Actual state:** {what actually happened}
**Discrepancy:** {gap between expected and actual}
```

### Ordering Evidence

**Primary Evidence First:**
1. Component directly involved (skill/command/subagent)
2. Files showing missing/incorrect behavior
3. Files showing expected behavior (for comparison)

**Supporting Evidence Second:**
4. Related components
5. Context files
6. Documentation/protocols

**Background Evidence Last:**
7. Git history
8. Test files
9. Related RCAs

---

## Context File Validation

### Validation Procedure

**Step 1: Check Existence**
```
Glob(pattern="devforgeai/specs/context/*.md")

Expected: 6 files
- tech-stack.md
- source-tree.md
- dependencies.md
- coding-standards.md
- architecture-constraints.md
- anti-patterns.md
```

**Step 2: Validate Constraint Compliance**

For each context file:
1. Read constraint relevant to issue
2. Check if implementation follows constraint
3. Mark: PASS or FAIL
4. If FAIL: Record exact violation

**Example:**
```
tech-stack.md:
  Constraint: "Testing: pytest"
  Implementation: Uses pytest ✅
  Status: PASS

anti-patterns.md:
  Constraint: "No God Objects (>500 lines)"
  Implementation: Class has 650 lines ❌
  Status: FAIL
  Violation: UserService class exceeds 500 line limit
```

**Step 3: Cross-Reference Violations**

If violation found:
1. Was validation supposed to catch this?
2. Why didn't validation run?
3. Is validator broken or not invoked?

### Validation Output Template

```markdown
## Context Files Validation

**Files checked:**
- [x] tech-stack.md - PASS - Uses approved technologies
- [x] source-tree.md - PASS - Files in correct locations
- [x] dependencies.md - FAIL - Unapproved package: lodash
- [x] coding-standards.md - PASS - Follows naming conventions
- [x] architecture-constraints.md - FAIL - Domain → Infrastructure dependency
- [x] anti-patterns.md - PASS - No forbidden patterns

**Violations found:**
1. **dependencies.md violation:** Unapproved package 'lodash' in package.json
   - File: package.json:25
   - Constraint: dependencies.md only lists approved packages
   - Violation: lodash not in approved list

2. **architecture-constraints.md violation:** Domain layer importing Infrastructure
   - File: src/domain/User.ts:5
   - Import: `import { Database } from '../infrastructure/db'`
   - Constraint: "Domain must not depend on Infrastructure"
```

---

## Sufficiency Criteria

### When to Stop Collecting Evidence

**Sufficient evidence when:**
- ✅ All 5 Whys have supporting evidence
- ✅ Root cause clearly demonstrated
- ✅ All files mentioned in 5 Whys examined
- ✅ At least 3 recommendations supported by evidence
- ✅ Can answer: "Where exactly do we fix this?"

**Insufficient evidence if:**
- ❌ Any "why" lacks file evidence
- ❌ Root cause is assumption, not proven
- ❌ Recommendations vague (no exact file paths)
- ❌ Missing critical component examination
- ❌ Can't explain discrepancy (expected vs actual)

### Evidence Checklist

Before moving to Phase 4 (Recommendations), verify:

**For Root Cause:**
- [ ] Evidence proves root cause exists
- [ ] Evidence shows why root cause occurred
- [ ] Evidence demonstrates impact of root cause
- [ ] Evidence supports counterfactual (if fixed, issue prevented)

**For Recommendations:**
- [ ] Evidence shows exact file to modify
- [ ] Evidence shows exact section/phase/step
- [ ] Evidence shows surrounding code/context
- [ ] Evidence demonstrates fix will work

**For Completeness:**
- [ ] All components examined (skill/command/subagent)
- [ ] All context files validated (if relevant)
- [ ] All referenced files read
- [ ] All claims evidence-backed (no assumptions)

### Quality Threshold

**Minimum evidence required:**
- 3-5 files examined (primary components)
- 6-10 file excerpts captured
- All 6 context files validated (PASS/FAIL status)
- Workflow state analysis (if workflow issue)
- 10-20 lines per excerpt (with context)

**Optimal evidence:**
- 5-10 files examined
- 15-25 file excerpts captured
- Context files + related ADRs examined
- Git history checked (when bug introduced)
- Test coverage checked (missing test case?)

---

## Evidence Collection Workflow

### Phase 3 Integration

**Step 1: Load This Guide**
```
Read(file_path=".claude/skills/devforgeai-rca/references/evidence-collection-guide.md")
```

**Step 2: Load Template**
```
Read(file_path=".claude/skills/devforgeai-rca/assets/evidence-section-template.md")
```

**Step 3: Determine What to Examine**
- Use "What to Examine" section above
- Based on affected component type
- Based on 5 Whys answers

**Step 4: Read Files Systematically**
- Use "How to Read Files" section
- Capture line numbers
- Extract relevant excerpts

**Step 5: Determine Significance**
- Use "Significance Formula"
- Categorize: CRITICAL/HIGH/MEDIUM/LOW
- Omit LOW significance evidence

**Step 6: Organize Evidence**
- Use template structure
- Primary evidence first
- Group by file type

**Step 7: Validate Context Files**
- Check all 6 context files
- Mark PASS/FAIL
- List violations

**Step 8: Check Sufficiency**
- Use sufficiency criteria
- Verify checklist complete
- If insufficient, collect more

**Step 9: Populate Template**
- Fill evidence-section-template.md
- Insert into RCA document
- Ensure all placeholders replaced

---

## Example: Complete Evidence Section

**Issue:** /dev command started TDD without validating context files

**Evidence Collected:**

```markdown
## Evidence Collected

**Files Examined:**

**.claude/skills/devforgeai-development/SKILL.md**
- **Lines examined:** 50-200 (Phase 0: Pre-Flight Validation)
- **Finding:** Phase 0 has Git validation and tech detection, but no context file check
- **Excerpt:**
  ```markdown
  ## Phase 0: Pre-Flight Validation

  **Step 1: Git Validation** (invoke git-validator subagent)
  ...

  **Step 7: Tech Stack Detection** (invoke tech-stack-detector subagent)
  ...

  [MISSING: Context file validation step should be here]

  **Begin TDD Cycle:**
  Proceed to Phase 1 (Red phase)...
  ```
- **Significance:** CRITICAL - Directly demonstrates missing validation step, supports Recommendation 1

**devforgeai/specs/context/tech-stack.md**
- **Lines examined:** 1-50 (all constraints)
- **Finding:** File exists with 15 technology constraints
- **Excerpt:**
  ```markdown
  ## Frontend
  - Framework: React 18.2+
  - State Management: Zustand
  ...

  ## Backend
  - Language: Python 3.10+
  - Framework: FastAPI
  ...
  ```
- **Significance:** HIGH - Shows constraints exist that /dev should enforce

**.claude/agents/context-validator.md**
- **Lines examined:** 50-150 (validation logic)
- **Finding:** context-validator can check for 6 files, but not invoked
- **Excerpt:**
  ```markdown
  ## Validation Logic

  Step 1: Glob for devforgeai/specs/context/*.md
  Step 2: Expect 6 files
  Step 3: If <6 found, return violations
  ```
- **Significance:** MEDIUM - Shows validation capability exists, just not used

**Context Files Validation:**

**Files checked:**
- [ ] tech-stack.md - NOT CHECKED (validation missing)
- [ ] source-tree.md - NOT CHECKED (validation missing)
- [ ] dependencies.md - NOT CHECKED (validation missing)
- [ ] coding-standards.md - NOT CHECKED (validation missing)
- [ ] architecture-constraints.md - NOT CHECKED (validation missing)
- [ ] anti-patterns.md - NOT CHECKED (validation missing)

**Violations found:** None (check never performed, cannot detect violations)

**Workflow State Analysis:**

**Expected state:** /dev validates context files exist before TDD
**Actual state:** /dev proceeds to TDD without context validation
**Discrepancy:** Missing validation step in Phase 0 allows TDD to start without architectural constraints, violating spec-driven development principle
```

---

## Reference

**Related DevForgeAI Documentation:**
- `.claude/memory/qa-automation.md` - Validation patterns
- `devforgeai/protocols/lean-orchestration-pattern.md` - File reading patterns
- `CLAUDE.md` - Context file descriptions

**RCA Examples:**
- `devforgeai/RCA/RCA-006-autonomous-deferrals.md` - Evidence section example
- `devforgeai/RCA/RCA-008-autonomous-git-stashing.md` - Workflow state evidence
- `devforgeai/RCA/RCA-009-skill-execution-incomplete-workflow.md` - Missing phase evidence

---

**End of Evidence Collection Guide**

**Total: ~700 lines**
