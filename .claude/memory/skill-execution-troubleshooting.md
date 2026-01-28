# Skill Execution Troubleshooting Guide

Quick reference for diagnosing and recovering from skill execution errors.

---

## Symptom: I Invoked a Skill But Stopped Working

**Problem:** After `Skill(command="...")`, you stopped and waited instead of executing.

**Root Cause:** Confusion between Skill tool (inline expansion) and Task tool (isolated execution).

---

## Correct Behavior After Skill Invocation

### Step 1: Recognize Skill Content Has Expanded

When you see:
```
<command-message>The "devforgeai-development" skill is running</command-message>
```

**This means:**
- ✅ Skill's SKILL.md file content is now in your conversation
- ✅ You have access to the skill's workflow instructions
- ✅ **You must now execute those instructions**

**This does NOT mean:**
- ❌ Skill is executing elsewhere
- ❌ Wait for skill to return results
- ❌ Skill is running in background

### Step 2: Read the Skill's Phase 0 Instructions

Immediately after skill invocation:
1. The skill's SKILL.md content has been loaded into conversation
2. Locate the skill's Phase 0 section
3. Read the first step of Phase 0

### Step 3: Execute Phase 0

**Action required:** Run the Phase 0 instructions now

Example from devforgeai-development skill:
```
Phase 0: Pre-Flight Validation
Step 1: Validate Git status
  → Invoke git-validator subagent
  → Process subagent result
  → Set CAN_COMMIT flag
```

**You execute this:**
```
Task(
  subagent_type="git-validator",
  description="Check Git availability",
  prompt="Validate Git repository status..."
)
```

### Step 4: Display Phase 0 Results

**Show user what you did:**
```
✓ Git validation complete
✓ Repository status: Clean working tree
✓ CAN_COMMIT: true
```

### Step 5: Continue to Phase 1

**Read Phase 1 instructions from skill**
**Execute Phase 1**
**Display Phase 1 results**

### Step 6: Continue Through All Phases

**Sequential execution:**
- Phase 1 → Execute → Display results
- Phase 2 → Execute → Display results
- Phase 3 → Execute → Display results
- ...
- Final phase → Execute → Display completion report

---

## What to Do If You Catch Yourself Waiting

### If You Notice the Error Immediately

**Recognition:**
- "I just invoked a skill and said 'I'll wait'"
- "I stopped after seeing 'skill is running'"
- "I'm not executing the skill's instructions"

**Correct Course:**
1. **Acknowledge:** "I incorrectly stopped after skill invocation"
2. **Explain:** "Skills expand inline - I need to execute the instructions"
3. **Resume:** "Let me read the skill's Phase 0 now"
4. **Execute:** Begin Phase 0 execution
5. **Continue:** Complete all phases

### If User Points Out the Error

**User says:**
- "Why did you stop?"
- "You should be executing the skill, not waiting"
- "The skill doesn't run in the background"

**Your response:**
1. **Apologize:** "I apologize - I incorrectly treated the skill as a background process"
2. **Correct understanding:** "Skills expand inline and I must execute their instructions"
3. **Resume immediately:** "Let me execute the skill's workflow now, starting from Phase 0"
4. **Complete workflow:** Execute all phases to completion

---

## Quick Check: Am I Executing Correctly?

**Ask yourself after skill invocation:**

- ✅ Am I reading the skill's phase instructions?
- ✅ Am I executing those instructions?
- ✅ Am I displaying results as I work?
- ✅ Am I progressing through phases sequentially?

**If answer is "no" to any:** You've stopped incorrectly. Resume execution.

---

## Mental Model Correction

### ❌ WRONG Mental Model

```
Skill invocation → Wait for result → Display result
```

**This is the Task tool (subagents) pattern, NOT the Skill tool pattern.**

### ✅ CORRECT Mental Model

```
Skill invocation → Load instructions → Execute instructions → Display your work
```

**Skills expand inline. You do the work.**

---

## Comparison: Skills vs Subagents

| When to Wait | Skills (Skill tool) | Subagents (Task tool) |
|--------------|--------------------|-----------------------|
| **After invocation** | ❌ NEVER - You execute | ✅ YES - Agent executes |
| **Who produces output** | You | Agent |
| **Where execution happens** | Current conversation | Isolated context |
| **What you see** | Skill's SKILL.md content | Agent's structured result |

---

## Common Scenarios

### Scenario 1: /dev Command

**Command executes:**
```
Skill(command="devforgeai-development")
```

**System message appears:**
```
<command-message>The "devforgeai-development" skill is running</command-message>
```

**✅ Correct action:**
1. Read devforgeai-development skill's Phase 0
2. Execute Phase 0 Step 1 (git-validator subagent)
3. Execute Phase 0 Step 2 (adapt workflow)
4. Continue through all Phase 0 steps
5. Execute Phase 1 (Red phase)
6. Continue through all phases
7. Display completion report

**❌ Incorrect action:**
```
"The skill is running. I'll wait for it to complete."
[Stops and waits]
```

### Scenario 2: /qa Command

**Command executes:**
```
Skill(command="devforgeai-qa")
```

**✅ Correct action:**
1. Read devforgeai-qa skill's Phase 0
2. Execute validation mode detection
3. Execute story loading
4. Execute context validation
5. Continue through all phases (Light or Deep)
6. Invoke qa-result-interpreter subagent (Phase 5)
7. Display results from subagent

**❌ Incorrect action:**
```
"QA validation is running. I'll monitor progress."
[Stops and waits]
```

### Scenario 3: /orchestrate Command

**Command executes:**
```
Skill(command="devforgeai-orchestration")
```

**✅ Correct action:**
1. Read orchestration skill's Phase 0
2. Execute checkpoint detection
3. Invoke devforgeai-development skill (if needed)
4. Invoke devforgeai-qa skill (if needed)
5. Handle QA retry loop (if QA fails)
6. Invoke devforgeai-release skill (staging, production)
7. Display orchestration completion report

**❌ Incorrect action:**
```
"Orchestration workflow is running. I'll wait for all phases."
[Stops and waits]
```

---

## Emergency Recovery

**If you've already stopped and are reading this:**

### Step 1: Acknowledge the Error
```
"I apologize - I incorrectly stopped after skill invocation. Skills expand inline in the conversation, and I should have immediately begun executing the skill's workflow instructions."
```

### Step 2: Explain the Correct Model
```
"When I invoke a skill, the skill's SKILL.md content becomes part of my context. I am responsible for reading and executing each phase of the skill's workflow, not waiting for an external process to complete."
```

### Step 3: Resume Execution
```
"Let me now execute the skill's workflow, starting from Phase 0."
```

### Step 4: Execute All Phases
- Read Phase 0 instructions
- Execute Phase 0
- Display Phase 0 results
- Continue through all phases sequentially
- Display final completion report

### Step 5: Learn from the Error
```
"I will remember this for future skill invocations: Skills are inline prompt expansions that I execute, not background processes that I wait for."
```

---

## Prevention Checklist

**Before invoking any skill, remind yourself:**
- [ ] Skill invocation loads SKILL.md content into conversation
- [ ] I will read the skill's instructions
- [ ] I will execute those instructions phase by phase
- [ ] I will display results as I work
- [ ] I will NOT wait passively

**After skill invocation message appears:**
- [ ] I see "skill is running" message
- [ ] This means skill content is loaded
- [ ] I now read Phase 0 instructions
- [ ] I execute Phase 0 immediately
- [ ] I continue through all phases

---

## Testing Your Understanding

**Question 1:** After `Skill(command="devforgeai-development")`, what should you do?

**✅ Correct answer:** Read the devforgeai-development skill's Phase 0 instructions and begin executing them immediately.

**❌ Incorrect answer:** Wait for the skill to complete and monitor progress.

---

**Question 2:** What does the system message "The 'devforgeai-qa' skill is running" mean?

**✅ Correct answer:** The skill's SKILL.md content has been loaded into my conversation, and I must now execute its instructions.

**❌ Incorrect answer:** The skill is executing in the background, and I should wait for it to return results.

---

**Question 3:** When should you wait after invoking a skill?

**✅ Correct answer:** NEVER. Skills expand inline, and I execute their instructions immediately.

**❌ Incorrect answer:** Always wait for the skill to complete before continuing.

---

**Question 4:** What's the difference between Skill tool and Task tool?

**✅ Correct answer:**
- Skill tool: Inline expansion, I execute instructions
- Task tool: Isolated context, separate agent executes, I wait for result

**❌ Incorrect answer:** Both tools launch background processes that return results.

---

## Related Documentation

**Core reference:**
- `CLAUDE.md` - Section "CRITICAL: How Skills Work"
- `.claude/memory/skills-reference.md` - Skills overview with execution model

**Skills documentation:**
- `.claude/skills/devforgeai-development/SKILL.md` - Development workflow
- `.claude/skills/devforgeai-qa/SKILL.md` - QA validation workflow
- `.claude/skills/devforgeai-orchestration/SKILL.md` - Orchestration workflow

**Commands documentation:**
- `.claude/commands/dev.md` - /dev command (invokes devforgeai-development skill)
- `.claude/commands/qa.md` - /qa command (invokes devforgeai-qa skill)
- `.claude/commands/orchestrate.md` - /orchestrate command (invokes orchestration skill)

---

## Remember

**Skills are inline prompt expansions.**
- You invoke them
- They load into your conversation
- You execute their instructions
- You produce the output

**You NEVER wait for skills to "complete" or "return results."**

**If you find yourself waiting after skill invocation, STOP and resume execution immediately.**

---

## Pattern: Progressive Disclosure Phase Skipping

**Pattern ID:** RCA-009 / RCA-011 / RCA-016
**Severity:** HIGH
**Frequency:** Recurring across skills with progressive disclosure
**First Documented:** 2025-11-14
**Most Recent:** 2025-12-01

---

### What Is This Pattern?

**Problem:** You execute a skill's workflow but skip phases by not loading reference files.

**Example:**
```
Phase 0.9: AC-DoD Traceability ✅ Executed
Phase 1: Test Coverage ✅ Executed
Phase 2: Anti-Pattern Detection ❌ SKIPPED (jumped to Phase 5)
Phase 3: Spec Compliance ❌ SKIPPED
Phase 4: Code Quality ❌ SKIPPED
Phase 5: Report Generation ✅ Executed (but with incomplete data)
Phase 6: Feedback Hooks ❌ SKIPPED
Phase 7: Story Update ❌ SKIPPED
```

**Root Cause:** Phase summaries in SKILL.md appear self-explanatory, but the actual workflow steps are in reference files. You see "Phase 2: Anti-Pattern Detection" and think "I know what that means" without loading the 6-step workflow from the reference file.

---

### Symptoms (How to Detect)

**Watch for these warning signs:**

1. **Workflow completes too quickly**
   - Deep QA should take ~10-12 minutes, not 3 minutes
   - TDD cycle should take ~30 minutes, not 5 minutes
   - If you're "done" suspiciously fast, you likely skipped phases

2. **Reference files not loaded**
   - SKILL.md says "**Ref:** `references/workflow-name.md`"
   - But you didn't `Read(file_path="...")` that file
   - You executed based on phase title alone

3. **Subagents not invoked**
   - SKILL.md says "**Subagent:** agent-name (MANDATORY)"
   - But you didn't `Task(subagent_type="agent-name", ...)`
   - You made assumptions instead of following documented workflow

4. **Results lack expected detail**
   - Anti-pattern scan returns no violations (but you didn't read actual code files)
   - Coverage analysis has no layer breakdown (skipped classification step)
   - Spec compliance has no traceability matrix (skipped generation step)

5. **User questions completeness**
   - "Did you skip any phases?"
   - "Did you follow 100%?"
   - "Why didn't you invoke the subagent?"
   - If user has to ask, you likely skipped something

---

### Detection Checklist

**Run this checklist if you suspect phase skipping:**

- [ ] **Reference files:** Did I `Read()` every reference file mentioned in SKILL.md phases?
- [ ] **Subagent invocations:** Did I `Task()` every MANDATORY subagent listed?
- [ ] **Phase count:** Did I execute ALL phases (not just some)?
- [ ] **Execution time:** Is elapsed time reasonable for workflow complexity?
- [ ] **Result detail:** Do results have expected granularity (layer breakdown, violation counts, traceability matrix)?
- [ ] **Checkpoint markers:** Did I see and respond to every "⚠️ CHECKPOINT" in SKILL.md?
- [ ] **Completion checklists:** Did I verify every checkbox before proceeding to next phase?

**If ANY checkbox is unchecked:** You likely skipped phases. See Recovery Procedure below.

---

### Recovery Procedure

**If you catch yourself (or user points out) skipping phases:**

#### Step 1: Acknowledge the Gap

```
"You're right - I executed phases [X, Y] but skipped phases [A, B, C, D].
I proceeded based on phase titles without loading the reference files
that contain the actual workflow steps."
```

#### Step 2: Identify What Was Skipped

List specifically:
- Which phases were skipped
- Which reference files were not loaded
- Which subagents were not invoked

Example:
```
Skipped phases:
- Phase 2: Anti-Pattern Detection (did not load anti-pattern-detection-workflow.md)
- Phase 3: Spec Compliance (did not load spec-compliance-workflow.md, did not invoke deferral-validator)
- Phase 4: Code Quality (did not load code-quality-workflow.md)
- Phase 6: Feedback Hooks (did not load feedback-hooks-workflow.md)
- Phase 7: Story Update (did not load story-update-workflow.md)
```

#### Step 3: Load Missing Reference Files

```
Read(file_path=".claude/skills/devforgeai-qa/references/anti-pattern-detection-workflow.md")
Read(file_path=".claude/skills/devforgeai-qa/references/spec-compliance-workflow.md")
Read(file_path=".claude/skills/devforgeai-qa/references/code-quality-workflow.md")
Read(file_path=".claude/skills/devforgeai-qa/references/feedback-hooks-workflow.md")
Read(file_path=".claude/skills/devforgeai-qa/references/story-update-workflow.md")
```

#### Step 4: Execute Skipped Phases

For each skipped phase:
1. Read the reference file's workflow steps
2. Execute EACH step in order
3. Invoke any MANDATORY subagents
4. Verify completion checklist before proceeding
5. Display phase completion to user

#### Step 5: Update Results

Integrate newly collected data into final results:
- Add violation counts from anti-pattern scan
- Add compliance status from spec validation
- Add quality metrics from code analysis
- Update overall QA result based on all phases

#### Step 6: Verify Completeness

Display completion checklist showing ALL phases executed:
```
✓ Phase 0.9: AC-DoD Traceability (100% score)
✓ Phase 1: Test Coverage (85% overall)
✓ Phase 2: Anti-Pattern Detection (0 CRITICAL, 2 MEDIUM)
✓ Phase 3: Spec Compliance (8/8 AC validated)
✓ Phase 4: Code Quality (MI: 78, Complexity: avg 4.2)
✓ Phase 5: Report Generated
✓ Phase 6: Feedback Hooks (triggered)
✓ Phase 7: Story Updated to QA Approved
```

---

### Prevention Strategies

**Before executing any skill phase:**

#### Strategy 1: Check for CHECKPOINT Markers

Look for `⚠️ CHECKPOINT` in SKILL.md. These mark phases requiring reference loading.

Example:
```markdown
### Phase 2: Anti-Pattern Detection

**⚠️ CHECKPOINT: You MUST load the reference file and execute ALL steps before proceeding**

**Step 2.0: Load Workflow Reference (REQUIRED)**
Read(file_path=".claude/skills/devforgeai-qa/references/anti-pattern-detection-workflow.md")
```

**If you see a CHECKPOINT:** You MUST load the reference file. No exceptions.

#### Strategy 2: Verify Reference Loading

Before claiming a phase complete, ask yourself:
- Did I `Read()` the reference file for this phase?
- Did I see the workflow steps from that file?
- Did I execute EACH step, not just the phase title?

#### Strategy 3: Use Phase Completion Checklists

Each phase should have a completion checklist. Verify EVERY checkbox before proceeding.

Example:
```markdown
**Phase 2 Completion Checklist:**
- [ ] Loaded anti-pattern-detection-workflow.md (Step 2.0)
- [ ] Step 1: Loaded ALL 6 context files
- [ ] Step 2: Invoked anti-pattern-scanner subagent
- [ ] Step 3: Parsed JSON response
- [ ] Step 4: Updated blocks_qa state
- [ ] Step 5: Displayed violations summary
- [ ] Step 6: Stored violations for report
```

**IF any checkbox is unchecked:** HALT and complete missing steps.

#### Strategy 4: Display Loading Confirmation

When you load a reference file, explicitly confirm:
```
Loading reference: anti-pattern-detection-workflow.md
Found 6 workflow steps. Executing steps 1-6...
```

This creates a visible trail showing reference files were actually loaded.

#### Strategy 5: Track Execution Time

Be suspicious if workflow completes very quickly:
- Deep QA: Expect ~10-12 minutes
- TDD Development: Expect ~30 minutes
- Light QA: Expect ~3-5 minutes

If done much faster, verify you didn't skip phases.

---

### Affected Skills

**Skills with progressive disclosure that may exhibit this pattern:**

| Skill | Reference Files | Risk Level |
|-------|-----------------|------------|
| devforgeai-development | 8 reference files | HIGH (RCA-009, RCA-011) |
| devforgeai-qa | 19 reference files | HIGH (RCA-016) |
| devforgeai-orchestration | 4 reference files | MEDIUM |
| devforgeai-release | 5 reference files | MEDIUM |
| devforgeai-architecture | 6 reference files | MEDIUM |
| devforgeai-ideation | 16 reference files | MEDIUM |
| devforgeai-story-creation | 6 reference files | MEDIUM |
| devforgeai-documentation | 7 reference files | LOW |

**High-risk indicators:**
- More reference files = more phases to potentially skip
- MANDATORY subagents = more invocations to potentially miss
- Complex workflows = more opportunities for shortcuts

---

### Related RCAs

**RCA-009: Incomplete Skill Workflow Execution (2025-11-14)**
- Skill: devforgeai-development
- Symptom: Skipped Tech Spec Coverage step in Phase 2
- Root cause: Progressive disclosure - reference file not loaded
- Status: Partially resolved

**RCA-011: Mandatory TDD Phase Skipping (2025-11-19)**
- Skill: devforgeai-development
- Symptom: Skipped mandatory TDD phases
- Root cause: Same as RCA-009 - progressive disclosure ambiguity
- Status: Recommendations pending

**RCA-016: QA Skill Phase Skipping During Deep Validation (2025-12-01)**
- Skill: devforgeai-qa
- Symptom: Skipped 5 phases (2, 3, 4, 6, 7), jumped from Phase 1 to Phase 5
- Root cause: Same pattern - reference files not loaded
- Status: REC-1 COMPLETE, REC-2 COMPLETE, REC-3 COMPLETE

**Pattern frequency:** 3 incidents in 3 weeks (HIGH recurrence rate)

---

### Success Criteria

After implementing prevention strategies, verify:

- ✅ All reference files loaded (visible `Read()` calls in conversation)
- ✅ All workflow steps executed (not just phase titles)
- ✅ All MANDATORY subagents invoked
- ✅ Completion checklists verified before phase transitions
- ✅ Expected execution time achieved
- ✅ User does NOT need to question completeness

---

### Quick Reference Card

```
┌─────────────────────────────────────────────────────────────┐
│  PROGRESSIVE DISCLOSURE PHASE SKIPPING - QUICK CHECK        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  BEFORE each phase:                                         │
│  □ Look for ⚠️ CHECKPOINT marker                            │
│  □ Read() the reference file if indicated                   │
│  □ Note the workflow step count (e.g., "6 steps")           │
│                                                             │
│  DURING phase execution:                                    │
│  □ Execute EACH step from reference file                    │
│  □ Invoke MANDATORY subagents with Task()                   │
│  □ Display step completion as you work                      │
│                                                             │
│  AFTER each phase:                                          │
│  □ Verify completion checklist (ALL boxes checked)          │
│  □ Display "Phase X Complete" summary                       │
│  □ Only THEN proceed to next phase                          │
│                                                             │
│  WARNING SIGNS:                                             │
│  ⚠ Workflow completing too fast                            │
│  ⚠ No Read() calls for reference files                     │
│  ⚠ No Task() calls for MANDATORY subagents                 │
│  ⚠ Results lacking expected detail                         │
│  ⚠ User asking "did you skip phases?"                      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

### Difference from "Waiting After Invoke" Pattern

**Two distinct patterns exist. Don't confuse them:**

| Aspect | Waiting After Invoke | Phase Skipping |
|--------|---------------------|----------------|
| **When it occurs** | Immediately after `Skill()` | During skill execution |
| **Symptom** | Stop and wait passively | Execute but skip phases |
| **What you do wrong** | Nothing (waiting) | Partial execution |
| **Reference files** | N/A (never started) | Not loaded |
| **Recovery** | Start executing Phase 0 | Load refs, execute missing phases |
| **Related RCAs** | RCA-009 (original) | RCA-009, RCA-011, RCA-016 |

**Both patterns can occur in same session:**
1. First, you might wait after skill invocation (Pattern 1)
2. After resuming, you might skip phases (Pattern 2)

**Check for BOTH patterns when troubleshooting skill execution issues.**

---

## Pattern: WSL File Corruption (ENOENT / Inaccessible Files)

**Pattern ID:** Observed 2026-01-13
**Severity:** BLOCKING
**Environment:** WSL (Windows Subsystem for Linux)

---

### What Is This Pattern?

**Problem:** File becomes corrupted/inaccessible after Claude Code edit operations.

**Symptoms:**
- `ls -la` shows file with `?????????` permissions (e.g., `-????????? ? ? ? ? ? filename.md`)
- Read tool returns: `ENOENT: no such file or directory, statx '/path/to/file.md'`
- File appears in directory listing but cannot be accessed
- Occurs after Edit/Write operations, especially during unit test runs

**Root Cause:** WSL filesystem caching issue, potentially caused by rapid file operations or character encoding conflicts between Windows and Linux.

---

### Recovery Procedure

#### Step 1: Create Backup Copy (if possible)

If file is readable before corruption detected:
```bash
cp file.md file-backup.txt
```

#### Step 2: Install dos2unix (if not installed)

```bash
sudo apt-get update && sudo apt-get install dos2unix
```

#### Step 3: Convert Line Endings

```bash
dos2unix file-backup.txt
```

#### Step 4: Remove Corrupted File

```bash
rm -f file.md
```

#### Step 5: Restore from Backup

```bash
mv file-backup.txt file.md
```

OR

```bash
cp file-backup.txt file.md
```

#### Step 6: Verify File Accessible

```bash
ls -la file.md  # Should show normal permissions
head -10 file.md  # Should display content
```

---

### Prevention Strategies

1. **After large edits:** Wait a moment before running tests or further operations
2. **Copy before editing:** `cp original.md original.backup.md`
3. **Use Write over Edit for complete rewrites:** Write creates fresh file vs. in-place edit
4. **Monitor for corruption signs:** Watch `ls -la` output during workflows

---

### Quick Fix Command Sequence

```bash
# If file shows ????????? in ls:
dos2unix backup-copy.txt
rm -f corrupted-file.md
mv backup-copy.txt corrupted-file.md
```

---

### Related

- Claude Code Terminal update (2026-01-xx) introduced potential WSL filesystem interaction changes
- May be related to rapid file operations during TDD test execution
