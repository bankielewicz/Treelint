# 5 Whys Methodology for DevForgeAI RCA

**Purpose:** Guide for conducting systematic root cause analysis using the 5 Whys technique

**Target Audience:** devforgeai-rca skill (Claude executing RCA workflow)

---

## Table of Contents

1. [Introduction to 5 Whys](#introduction-to-5-whys)
2. [When to Use](#when-to-use)
3. [How to Ask Effective "Why" Questions](#how-to-ask-effective-why-questions)
4. [Identifying Root Causes vs Symptoms](#identifying-root-causes-vs-symptoms)
5. [Validation Techniques](#validation-techniques)
6. [Common Pitfalls](#common-pitfalls)
7. [DevForgeAI-Specific Patterns](#devforgeai-specific-patterns)

---

## Introduction to 5 Whys

### What is the 5 Whys Technique?

The 5 Whys is a systematic questioning method used to explore cause-and-effect relationships underlying a particular problem. By asking "why" five times in succession, you drill down from symptoms to the root cause.

**Core Principle:** The fifth "why" typically reveals a process, system, or framework issue that, when fixed, prevents recurrence.

### Historical Context

Developed by Sakichi Toyoda (Toyota Industries) in the 1930s, adopted by Toyota Production System, and now standard in:
- Lean manufacturing
- Six Sigma
- Software engineering (post-mortems)
- DevOps incident analysis

### Why 5 Whys Works

**Psychological:** Forces systematic thinking, prevents jumping to conclusions
**Structural:** Each answer becomes the subject of the next question, creating a chain
**Practical:** Simple enough to use without special training or tools

**Example Chain:**
```
Problem: Build failed in CI/CD
Why 1? → Test suite timeout
Why 2? → Test database slow
Why 3? → Database not cleaned between tests
Why 4? → Teardown function not called
Why 5? → Test framework hook misconfigured
ROOT CAUSE: Framework upgrade changed hook API, config not updated
```

---

## When to Use

### DevForgeAI Framework Breakdowns

Use 5 Whys when:
- **Process failures:** Skill/command didn't follow intended workflow
- **Workflow violations:** Quality gate bypassed, workflow state incorrect
- **Constraint violations:** Context files ignored, anti-patterns introduced
- **Unexpected behavior:** Framework components behaving incorrectly
- **User reports:** "This didn't work as expected"

### Indicators for RCA

**Strong indicators (always use 5 Whys):**
- CRITICAL or HIGH severity issues
- Recurrent problems (happened before)
- Framework architecture violations
- Silent failures (no error, wrong result)

**Moderate indicators (consider 5 Whys):**
- MEDIUM severity issues
- User confusion about framework behavior
- Documentation gaps causing issues

**Weak indicators (may not need full 5 Whys):**
- LOW severity issues
- One-time environmental issues
- User error (not framework problem)

### When NOT to Use

**Skip 5 Whys if:**
- Cause is immediately obvious and isolated
- External dependency failure (not framework issue)
- User explicitly made wrong choice
- No framework improvement possible

**Example: Skip RCA**
```
Issue: "Command failed because I typed /deev instead of /dev"
Analysis: Typo, not framework breakdown
Action: No RCA needed, suggest /help for command list
```

---

## How to Ask Effective "Why" Questions

### Question Formation Patterns

**Pattern 1: Direct Causation**
```
Why did {event} happen?
→ Because {immediate cause}
```

**Pattern 2: Absence of Control**
```
Why wasn't {expected behavior} present?
→ Because {missing mechanism}
```

**Pattern 3: Process Breakdown**
```
Why didn't {process step} occur?
→ Because {process gap}
```

### Question Quality Criteria

**Good "Why" questions:**
- ✅ Specific: "Why did Phase 0 skip context validation?"
- ✅ Neutral: "Why was the check missing?" (not "Why did you forget the check?")
- ✅ Process-focused: "Why wasn't this caught in testing?"
- ✅ Evidence-based: "Why does the code at line 150 not include this?"

**Poor "Why" questions:**
- ❌ Vague: "Why didn't it work?"
- ❌ Blame-oriented: "Why did Claude mess this up?"
- ❌ Multi-part: "Why did X and also Y happen?"
- ❌ Assumptive: "Why did you intentionally skip validation?"

### Progressive Depth

**Layer 1 (Surface):** Immediate observable cause
- "Why did /dev fail?"
- "→ Context files missing"

**Layer 2 (First Depth):** Why that cause occurred
- "Why were context files missing?"
- "→ Validation step not in Phase 0"

**Layer 3 (Second Depth):** Why the mechanism failed
- "Why was validation not in Phase 0?"
- "→ Assumed context always exists"

**Layer 4 (Third Depth):** Why the assumption was made
- "Why was that assumption made?"
- "→ No test case for greenfield projects"

**Layer 5 (Root Cause):** System/process/framework gap
- "Why wasn't greenfield scenario tested?"
- "→ ROOT CAUSE: Test suite lacks scenario coverage for edge cases"

---

## Identifying Root Causes vs Symptoms

### Symptoms vs Root Causes

**Symptoms:** Observable problems (effects)
**Root Causes:** Underlying issues (causes)

**Example:**
```
Symptom: QA validation failed
Symptom: Coverage report stale
Symptom: loader.rs not in report
Root Cause: Development workflow doesn't generate coverage reports
```

### Root Cause Characteristics

**A true root cause:**
1. **Actionable:** Can be fixed with specific changes
2. **Preventive:** Fixing it prevents recurrence
3. **Systemic:** Affects framework/process, not one-time
4. **Within control:** Framework can address it
5. **Evidence-based:** Supported by file examination

### Test: Is This the Root Cause?

**Ask these validation questions:**

**Q1: Would fixing this prevent the problem from happening again?**
- If NO → Not root cause, keep asking "why"
- If YES → Likely root cause

**Q2: Does this explain all observed symptoms?**
- If NO → Might be a contributing factor, not sole root
- If YES → Strong candidate for root cause

**Q3: Is this within the framework's control to change?**
- If NO → External dependency, not root cause
- If YES → Root cause

**Q4: Is this based on evidence, not assumption?**
- If NO → Speculation, need more evidence
- If YES → Validated root cause

**Example Validation:**
```
Proposed Root Cause: "Phase 0 doesn't validate context files"

Q1: Would adding validation prevent future failures?
→ YES (no TDD without constraints)

Q2: Does this explain all symptoms?
→ YES (explains why TDD started without constraints)

Q3: Can framework add this validation?
→ YES (add to SKILL.md Phase 0)

Q4: Is this evidence-based?
→ YES (SKILL.md examined, no validation step found)

CONCLUSION: ✅ This is the root cause
```

### Multiple Root Causes

**Some issues have multiple contributing root causes:**

**Example:**
```
Issue: Autonomous deferrals accepted without approval

Root Cause 1: Pre-justified deferrals in story template
Root Cause 2: No deferral challenge checkpoint in /dev
Root Cause 3: No validation that deferrals were user-approved

All three must be fixed to prevent recurrence.
```

**How to handle:**
- Identify PRIMARY root cause (most direct)
- List CONTRIBUTING root causes
- Provide recommendations for each

---

## Validation Techniques

### Counterfactual Testing

**Technique:** "If we had fixed X, would the problem have been prevented?"

**Example:**
```
Root Cause: No context file validation in Phase 0

Counterfactual: If validation existed, would /dev have failed?
→ YES, validation would halt workflow, display clear error
→ VALIDATES root cause
```

### Evidence Cross-Reference

**Technique:** Find evidence in files that confirms the cause

**Example:**
```
Why #3: Validation step missing from Phase 0

Evidence Check:
1. Read .claude/skills/devforgeai-development/SKILL.md
2. Search for "context" in Phase 0
3. Find: No step mentioning context file validation
4. Cross-reference: devforgeai-architecture creates context files
5. Conclusion: Gap confirmed, validation missing

✅ Evidence validates this "why" answer
```

### Scenario Replay

**Technique:** Walk through the scenario with proposed root cause fixed

**Example:**
```
Scenario: User runs /dev without context files

WITHOUT FIX:
1. /dev invoked
2. Phase 0 runs (Git, tech detection)
3. TDD starts
4. Implementation without constraints
5. QA fails

WITH FIX (context validation added):
1. /dev invoked
2. Phase 0 runs (Git, tech, CONTEXT validation)
3. Context validation fails (6 files not found)
4. Workflow HALTS with clear error
5. User runs /create-context
6. User re-runs /dev
7. TDD starts with constraints

✅ Fix prevents the issue
```

---

## Common Pitfalls

### Pitfall 1: Stopping Too Early

**Problem:** Accepting a symptom as root cause

**Example:**
```
❌ WRONG:
Why did QA fail? → Coverage too low
ROOT CAUSE: Coverage too low

This is a symptom, not root cause!

✅ CORRECT:
Why did QA fail? → Coverage too low
Why was coverage low? → No coverage report generated
Why wasn't report generated? → /dev didn't create it
Why didn't /dev create it? → DoD validation accepted pre-existing deferrals
Why were deferrals accepted? → No challenge checkpoint
ROOT CAUSE: No deferral challenge checkpoint
```

### Pitfall 2: Branching "Whys"

**Problem:** Asking multiple "whys" simultaneously

**Example:**
```
❌ WRONG:
Why did this happen?
→ Because A and B and C

Now you have 3 branches to follow.

✅ CORRECT:
Why did this happen?
→ Because A (primary cause)

Follow A through 5 layers, then circle back to B and C as contributing factors.
```

### Pitfall 3: Blame-Oriented Questioning

**Problem:** Focusing on "who" instead of "what/why"

**Example:**
```
❌ WRONG:
Why did Claude skip validation?
→ Implies intent, assigns blame

✅ CORRECT:
Why was validation not performed?
→ Focuses on process gap, not blame
```

### Pitfall 4: Assuming Instead of Evidence

**Problem:** Guessing answers without examining files

**Example:**
```
❌ WRONG:
Why was check missing? → Probably oversight

✅ CORRECT:
Why was check missing? → Read SKILL.md Phase 0 (lines 50-150), no context validation step found. Check git history: validation was never added.
```

### Pitfall 5: Accepting "Human Error" as Root Cause

**Problem:** Stopping at "someone made a mistake"

**Example:**
```
❌ WRONG:
Why did bug occur? → Developer forgot to validate
ROOT CAUSE: Human error

✅ CORRECT:
Why did bug occur? → Developer forgot to validate
Why was validation forgotten? → No checklist enforced
Why no checklist? → No pre-commit hook
Why no hook? → Framework doesn't include hooks by default
ROOT CAUSE: Framework missing pre-commit validation hooks
```

---

## DevForgeAI-Specific Patterns

### Pattern 1: Lean Orchestration Violations

**Breakdown:** Command contains business logic (should be in skill)

**5 Whys Example:**
```
Why did command have business logic?
→ Logic not extracted during refactoring

Why wasn't it extracted?
→ Refactoring checklist incomplete

Why was checklist incomplete?
→ lean-orchestration-pattern.md didn't specify this logic type

Why didn't protocol specify it?
→ New pattern not documented yet

Why not documented?
→ ROOT CAUSE: Protocol update process missing for new patterns
```

**Evidence to Collect:**
- Command .md file (find business logic)
- Skill SKILL.md (check if logic should be there)
- lean-orchestration-pattern.md (check protocol coverage)

### Pattern 2: Context File Constraint Violations

**Breakdown:** Code violates tech-stack.md or anti-patterns.md

**5 Whys Example:**
```
Why did code violate tech-stack.md?
→ context-validator didn't catch it

Why didn't validator catch it?
→ context-validator not invoked in workflow

Why not invoked?
→ Skill Phase X doesn't include validation step

Why no validation step?
→ Skill assumed validation happens in quality gate

Why that assumption?
→ ROOT CAUSE: Unclear responsibility (when does validation happen?)
```

**Evidence to Collect:**
- Skill SKILL.md (check validation steps)
- Context file (show violated constraint)
- context-validator subagent (check if it could detect this)

### Pattern 3: Quality Gate Bypass

**Breakdown:** Story progressed without meeting gate criteria

**5 Whys Example:**
```
Why did story progress past gate?
→ Gate validation returned PASS incorrectly

Why did validation pass?
→ Validation checked wrong metric

Why wrong metric?
→ Metric definition changed, validation not updated

Why not updated?
→ No dependency tracking between metrics and validators

Why no tracking?
→ ROOT CAUSE: Validation logic not linked to metric definitions
```

**Evidence to Collect:**
- Quality gate definition (from CLAUDE.md or protocols)
- Validation code (from skill)
- Story file (show actual state vs expected)

### Pattern 4: Progressive Disclosure Failure

**Breakdown:** Skill loaded all reference files at once (not progressively)

**5 Whys Example:**
```
Why did skill load all references?
→ SKILL.md says "Read all references"

Why does SKILL.md say that?
→ Instructions not refactored for progressive disclosure

Why not refactored?
→ Progressive disclosure pattern not applied to this skill

Why not applied?
→ Skill created before pattern established

Why before pattern?
→ ROOT CAUSE: Older skills need retroactive refactoring for progressive disclosure
```

**Evidence to Collect:**
- SKILL.md (check if references loaded conditionally)
- Recent skill refactorings (show progressive disclosure pattern)
- Token usage (show inefficiency from loading everything)

### Pattern 5: Subagent Framework Silo

**Breakdown:** Subagent operated without framework awareness

**5 Whys Example:**
```
Why did subagent violate framework constraints?
→ Subagent didn't check context files

Why didn't it check?
→ Subagent system prompt doesn't mention context files

Why doesn't prompt mention them?
→ Subagent created before framework-aware pattern

Why before pattern?
→ Subagent created during Phase 2 (before enhancement)

Why not updated?
→ ROOT CAUSE: No systematic review process to update old subagents with new patterns
```

**Evidence to Collect:**
- Subagent .md file (check system prompt)
- agent-generator subagent (show framework-aware pattern)
- When subagent was created (git log)

---

## Example: Complete 5 Whys (RCA-006)

**Issue:** Autonomous deferrals accepted without user approval

**Why #1:** Why did /dev accept deferrals without approval?
**Answer:** Pre-existing deferrals in story template were accepted without challenge
**Evidence:** STORY-008.1.story.md lines 45-60 show 3 deferrals with justifications already present before /dev invoked

**Why #2:** Why were pre-existing deferrals accepted without challenge?
**Answer:** Development skill Phase 5 (DoD validation) was designed to skip items that already had justifications
**Evidence:** devforgeai-development SKILL.md Phase 5 logic: "IF justification exists, skip validation"

**Why #3:** Why did Phase 5 skip justified items?
**Answer:** Workflow assumed justifications meant user had already approved in a previous iteration
**Evidence:** SKILL.md comments: "// Deferrals with justifications are from previous QA retry, already validated"

**Why #4:** Why was that assumption made?
**Answer:** Workflow designed for QA retry scenario, didn't account for pre-justified deferrals in template
**Evidence:** QA retry workflow adds justifications after user approves. Template pre-population not considered.

**Why #5:** Why didn't workflow account for template pre-justification?
**Answer:** **ROOT CAUSE:** Story template allows pre-populating deferrals before implementation, bypassing approval requirement

**Validation:**
- Q1: Would preventing template pre-justification stop autonomous deferrals? → YES
- Q2: Does this explain all symptoms (3 deferrals accepted)? → YES
- Q3: Can framework fix this? → YES (add Phase 4.5 challenge checkpoint)
- Q4: Evidence-based? → YES (story file, SKILL.md examined)

**Conclusion:** ✅ Validated root cause, recommend Phase 4.5 Deferral Challenge Checkpoint

---

## Reference

**For complete RCA examples, see:**
- `devforgeai/RCA/RCA-006-autonomous-deferrals.md` (deferral validation)
- `devforgeai/RCA/RCA-007-multi-file-story-creation.md` (subagent file creation)
- `devforgeai/RCA/RCA-008-autonomous-git-stashing.md` (git workflow)
- `devforgeai/RCA/RCA-009-skill-execution-incomplete-workflow.md` (skill execution)

**Related DevForgeAI Documentation:**
- `CLAUDE.md` - Framework overview
- `devforgeai/protocols/lean-orchestration-pattern.md` - Command/skill/subagent architecture
- `.claude/memory/skills-reference.md` - Skill patterns
- `.claude/memory/subagents-reference.md` - Subagent patterns

---

**End of 5 Whys Methodology Guide**

**Total: ~800 lines**
