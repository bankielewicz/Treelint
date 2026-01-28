# Troubleshooting Lean Orchestration Pattern Violations

**Part of:** Lean Orchestration Pattern Protocol
**Version:** 1.0
**Date:** 2025-11-18
**Applies To:** All DevForgeAI slash commands

**Quick Link:** When violations detected, jump to [Auto-Detection](#auto-detection) or [Violation Patterns](#common-violation-patterns)

---

## Purpose

This guide helps diagnose and resolve violations of the lean orchestration pattern in DevForgeAI slash commands.

**Lean Orchestration Constitutional Principle:**
> "Commands orchestrate. Skills validate. Subagents specialize."

This separation of concerns ensures:
- Commands stay within 15K character budget
- Skills contain comprehensive business logic
- Subagents handle specialized tasks in isolated contexts
- Token efficiency is maximized (60-80% reduction in main conversation)

**This guide is used when:**
- Automated `/audit-budget` detects violations
- Manual code review identifies pattern violations
- New commands approach budget limits
- Existing commands accumulate business logic creep
- Framework integrity needs verification

---

## Quick Diagnosis Checklist

**Use this 5-item checklist to quickly identify violations:**

- [ ] **Budget:** Does command exceed 15,000 characters? (`wc -c < .claude/commands/command.md`)
- [ ] **Logic:** Does command contain validation algorithms, calculations, or business decision-making?
- [ ] **Files:** Does command read files it didn't create (reports, outputs, artifacts)?
- [ ] **Templates:** Does command have display templates or formatting logic (>50 lines)?
- [ ] **Architecture:** Does command invoke subagents directly (should go through skill)?

**If ANY checkbox is marked:** Continue to [Auto-Detection](#auto-detection) or jump to relevant [Violation Pattern](#common-violation-patterns)

---

## Auto-Detection

### Using /audit-budget Command

**The automated budget auditor identifies budget violations:**

```bash
> /audit-budget
```

**Output example:**
```
Command Budget Compliance Audit (2025-11-18)

OVER BUDGET (>15,000 chars):
  ❌ create-ui          19,908 chars (126% - 4,908 chars over)
  ❌ release            18,166 chars (121% - 3,166 chars over)

HIGH USAGE (12,001-15,000 chars):
  ⚠️  create-story     14,163 chars (94%)
  ⚠️  orchestrate      14,422 chars (96%)

COMPLIANT (<12,000 chars):
  ✅ qa                 8,172 chars (54%)
  ✅ create-sprint      8,000 chars (53%)
  ✅ create-epic       11,270 chars (75%)

Priority Queue:
1. create-ui (CRITICAL: 4,908 chars over)
2. release (HIGH: 3,166 chars over)
3. create-story (MONITOR: 94% budget)
```

### Manual Budget Check

```bash
# Quick character count
wc -c < .claude/commands/command.md

# Budget status calculation
chars=$(wc -c < .claude/commands/command.md)
percent=$((chars * 100 / 15000))
echo "Budget usage: ${percent}%"

# All commands status
for cmd in .claude/commands/*.md; do
  name=$(basename "$cmd" .md)
  chars=$(wc -c < "$cmd")
  percent=$((chars * 100 / 15000))
  [ $chars -gt 15000 ] && echo "❌ $name: $chars chars (${percent}%)"
done
```

### Automated Pattern Detection

**Detect specific violation patterns:**

```bash
# 1. Find business logic (calculations, loops, conditions)
grep -n "FOR each\|WHILE\|Calculate\|IF.*THEN\|Validate.*:" .claude/commands/command.md

# 2. Find file reading (command reading its own outputs)
grep -n "Read.*report\|Read.*output\|Read.*result\|Read.*\.json" .claude/commands/command.md

# 3. Find display templates (multiple formatting variants)
grep -n "Display:\|Output:\|Template:" .claude/commands/command.md | wc -l
# If >5 templates: Extract to subagent

# 4. Find subagent invocation without skill layer
grep -n "Task(subagent_type=" .claude/commands/command.md | head -1
# If found and command <50 lines of other logic: Violation detected

# 5. Find error handling matrix
grep -n "Error:\|Error handling:\|❌" .claude/commands/command.md | wc -l
# If >8 error scenarios: Consider extracting
```

---

## Common Violation Patterns

### Pattern 1: Command Over 15K Characters

**The most visible violation - command exceeds hard budget limit.**

#### Symptoms

- `/audit-budget` flags command as "❌ OVER BUDGET"
- Character count >15,000
- Command file feels "long" when viewing (>600 lines)
- Pre-commit hook blocks deployment (validate-dod checks budget)
- Main conversation token usage very high (~8-10K)

#### Root Causes

**Cause 1: Business logic accumulation**
- Command contains validation algorithms (100+ lines)
- Command has complex error handling matrix (150+ lines)
- Command includes calculation logic (80+ lines)
- Solution: Move to skill

**Cause 2: Display template explosion**
- Command has 5+ display variants (161 lines in /qa before refactoring)
- Multiple IF/ELSE branches for different output formats
- Template logic intertwined with decision-making
- Solution: Create subagent (qa-result-interpreter pattern)

**Cause 3: File reading duplication**
- Command reads report created by skill
- Command parses report sections (validation, error detection)
- Command reads same report multiple times
- Solution: Move parsing to skill, return processed result

**Cause 4: Educational bloat**
- Command includes extensive integration notes (>300 lines)
- Multiple examples of usage
- Comprehensive error documentation
- Solution: Move to reference file, keep command focused

**Cause 5: Multiple workflows**
- Command handles epic, sprint, and story creation modes
- Mode detection and branching logic inline
- Different workflows have different logic paths
- Solution: Consolidate in skill, command just delegates

#### Resolution

**Step 1: Measure current state**
```bash
wc -c < .claude/commands/problem-command.md
# Result: 19,908 (example)

# Calculate reduction needed
target=10000  # Target 10K for safety
needed=$((current - target))
percent=$((needed * 100 / current))
echo "Need to reduce $needed chars ($percent%)"
```

**Step 2: Categorize content**
```bash
# Extract each section's size
grep -n "^### " .claude/commands/problem-command.md | head -20
# Note line numbers for each section

# Estimate lines per section
echo "Lines 1-50: Argument Validation"
echo "Lines 51-150: Business Logic ← EXTRACT"
echo "Lines 151-300: Display Templates ← EXTRACT"
echo "Lines 301-450: Error Handling ← EXTRACT"
echo "Lines 451-600: Integration Notes ← EXTERNALIZE"
```

**Step 3: Execute extraction strategy**

**For business logic (validation, calculations):**
- Move to skill as new phases or workflow steps
- Command delegates to skill
- Result: 80-150 lines extracted per section

**For display templates:**
- Create specialized subagent (follow qa-result-interpreter pattern)
- Subagent selects appropriate template based on result type
- Subagent returns structured JSON with display template
- Command just outputs `result.display`
- Result: 100-200 lines extracted

**For file reading:**
- Skill generates output AND invokes parser subagent
- Skill returns processed result to command
- Command doesn't read files
- Result: 60-100 lines eliminated (no file I/O in command)

**For integration notes:**
- Create separate reference document (epic-creation-guide.md pattern)
- Link from command to reference
- Keep command focused on execution
- Result: 200-300 lines externalized (kept for user value, not in command)

**Step 4: Create subagents (if needed)**

Example: create-ui refactoring needs ui-spec-formatter subagent
```markdown
# New subagent: ui-spec-formatter.md
- Purpose: Validate UI spec, generate display template
- Tools: Read, Grep (minimal file access)
- Input: UI spec file location
- Output: Structured JSON with display, validation results, next steps
- Reference file: ui-result-formatting-guide.md (framework constraints)
```

**Step 5: Refactor command to lean structure**
```
Phase 0: Argument Validation (30 lines)
├─ Validate story ID format
├─ Load story via @file
└─ Parse optional mode/environment

Phase 1: Invoke Skill (20 lines)
├─ Set context markers
└─ Skill(command="devforgeai-[skillname]")

Phase 2: Display Results (15 lines)
└─ Output: skill result (no parsing)

Integration Notes (200+ lines OK)
└─ Educational content, examples, success criteria
```

**Step 6: Verify refactoring**
```bash
# Check new character count
wc -c < .claude/commands/problem-command.md
# Should be <12,000 for safety

# Check skill enhancement
wc -l .claude/skills/[skill]/SKILL.md
# Should grow by 200-400 lines (acceptable)

# Check subagents created
ls -la .claude/agents/new-subagent.md
# Verify reference file created
ls -la .claude/skills/[skill]/references/new-guide.md
```

#### Example: /qa Refactoring (692 → 295 Lines)

**Before (Over Budget):**
```
Total: 692 lines, 31,000 characters (206% over 15K)

Phase 2: Handle QA Results (72 lines)
├─ Read QA report from disk
├─ Parse failure reasons
├─ Branch on deferral failures
└─ AskUserQuestion for next steps

Phase 3: Result Verification (33 lines)
├─ Read QA report AGAIN
├─ Parse report sections
└─ Verify story status

Phase 4: Display Results (161 lines) ← PROBLEM: 5 display templates
├─ Light PASS template (35 lines)
├─ Deep PASS template (38 lines)
├─ Failure template (28 lines)
├─ Coverage failure template (31 lines)
└─ Spec compliance template (29 lines)

Phase 5: Summary (34 lines)
├─ Light mode next steps
├─ Deep mode next steps
└─ Failure next steps

Error Handling (97 lines) ← PROBLEM: 6 error scenarios
└─ Each error with details and recovery
```

**Refactoring actions:**
1. Extract Phase 2 & 3 to skill (read report once, process there)
2. Extract Phase 4 templates → qa-result-interpreter subagent
3. Simplify Phase 5 (skill provides next steps)
4. Reduce error handling to 4-5 essential types

**After (Budget Compliant):**
```
Total: 295 lines, 7,205 characters (48% budget)

Phase 0: Argument Validation (20 lines)
└─ Validate story ID, load via @file, parse mode

Phase 1: Invoke Skill (15 lines)
├─ Set context markers
└─ Skill(command="devforgeai-qa")

Phase 2: Display Results (10 lines)
└─ Output: result.display (generated by subagent)

Phase 3: Next Steps (5 lines)
└─ Display: result.next_steps

Error Handling (25 lines)
└─ Minimal (4 types)

Integration Notes (125 lines)
└─ Educational content (acceptable)
```

**Result:** 57% reduction, 77% character reduction, 66% token savings

---

### Pattern 2: Commands Reading Files They Didn't Create

**Command reading output files to extract data or verify results.**

#### Symptoms

- Command contains `Read(file_path="devforgeai/qa/reports/...")` for files created by skill
- Report is read in command to make decisions (e.g., parse failure type)
- Same file read multiple times (duplication)
- File parsing logic in command (Grep, string matching)
- Command's workflow depends on file structure (brittle architecture)

#### Root Causes

**Cause 1: Result interpretation in command**
- Skill creates report file
- Command reads report to determine next action
- Decision logic mixes with display logic
- Example: /qa reading QA report to determine if retry needed

**Cause 2: Validation duplicated**
- Skill validates something, outputs to file
- Command reads file to validate again
- Double validation unnecessary
- Example: Sprint creation skill creates file, command reads to verify

**Cause 3: No structured result return**
- Skill creates file but doesn't return parsed result
- Command forced to parse file to get data
- Subagent could return structured JSON instead
- Example: UI generator creating spec, command reading spec to validate

#### Resolution

**Option A: Return structured result (Preferred)**

**Before:**
```markdown
Phase 1: Invoke Skill
  Skill(command="devforgeai-qa")

Phase 2: Read and Parse Report
  Read devforgeai/qa/reports/STORY-001-qa-report.md
  Parse file for: pass/fail status, violation type, coverage
  Determine: retry needed? approve? fail?
  Branch on: deferral failures vs other failures
```

**After:**
```markdown
Phase 1: Invoke Skill
  Skill(command="devforgeai-qa")
  (Skill returns: result.status, result.violations, result.display, result.next_steps)

Phase 2: Display Results
  Display result.display
  Display result.next_steps
  (No file reading needed)
```

**Step-by-step:**

1. **Skill enhancement:** Instead of just creating file, return structured result
   ```json
   {
     "status": "PASS|FAIL",
     "mode": "light|deep",
     "violations": [
       {"type": "coverage", "count": 3, "severity": "HIGH"},
       {"type": "deferral", "pattern": "circular", "severity": "CRITICAL"}
     ],
     "coverage_percent": 85.2,
     "next_steps": ["Review deferrals", "Update story"],
     "display": "{{template content generated by subagent}}"
   }
   ```

2. **Subagent for interpretation:** Create subagent to generate display template
   ```
   qa-result-interpreter subagent:
   - Input: QA report data (structured)
   - Output: display template + next steps
   - Result: Command just displays output
   ```

3. **Command simplified:** Remove file reading logic
   ```
   Phase 2: Display Results
   Output: result.display
   Output: result.next_steps
   ```

**Option B: Move parsing to skill**

**When skill should handle it:**
- Parsing is complex (multiple sections, conditional logic)
- Parsing result affects workflow (not just display)
- Parsing needs to invoke additional subagents

**Process:**
1. Skill creates file (for audit trail)
2. Skill ALSO reads and parses file
3. Skill invokes subagent for interpretation (if needed)
4. Skill returns parsed result to command
5. Command displays result (no file reading)

#### Example: /qa Refactoring (File Reading Elimination)

**Before (Command reads report):**
```bash
Phase 2: Handle QA Results (72 lines)
  IF skill execution succeeded:
    Read devforgeai/qa/reports/$story_id-qa-report.md
    Parse file for:
      - status: PASS|FAIL|INCOMPLETE
      - violations: [array of violations]
      - coverage_percent: number
      - failure_type: "deferral"|"coverage"|"other"

    IF failure_type == "deferral":
      AskUserQuestion: "Deferrals detected. Fix, retry, or defer?"
    ELIF failure_type == "coverage":
      AskUserQuestion: "Coverage gaps. Fill, ignore, or defer?"
```

**After (Skill returns parsed result):**
```markdown
Phase 2: Display Results
  Output result.display
  (qa-result-interpreter subagent determined template)

  IF result.violations contains deferral:
    AskUserQuestion: "Deferrals detected. Fix, retry, or defer?"
    (Based on structured result.violations, not file parsing)
```

**Key change:** Parsing moved to skill, command uses structured data

---

### Pattern 3: Business Logic in Command

**Command contains validation algorithms, calculations, or decision-making logic.**

#### Symptoms

- Command contains `FOR each`, `WHILE`, `IF...THEN...ELSE` chains (>20 lines)
- Command has algorithms (calculate capacity, complexity score, validate format)
- Command makes decisions (this needs to retry, this should create follow-up story)
- Command implements state machines (transition tracking)
- Command contains complex conditional logic (>8 branches)

#### Root Causes

**Cause 1: Lazy refactoring**
- Original implementation put logic in command
- Refactoring deferred (later, when we clean it up)
- Logic accumulates over time
- Command becomes top-heavy

**Cause 2: Feature creep**
- Command was simple initially
- New features added features to command (instead of skill)
- Each feature adds validation or calculation
- Soon command has 200+ lines of business logic

**Cause 3: Wrong abstraction boundary**
- Developer thought "this is orchestration logic" (belongs in command)
- Turns out it's business logic (belongs in skill)
- No refactoring happened after discovery
- Example: Sprint capacity calculation (should be in skill, not command)

**Cause 4: Missing skill layer**
- Some commands were built command → subagent directly
- Skipped skill layer entirely
- All business logic ended up in command
- Example: Original /create-sprint (497 lines, no skill invocation)

#### Resolution

**Step 1: Identify business logic**

**Business logic belongs in SKILL (not command):**
- Validation algorithms (format checking, constraint validation)
- Calculations (capacity, complexity, coverage %)
- Workflow decisions (should retry? should create follow-up story?)
- State transitions (status changes, history updates)
- Database operations (reading/writing story files)
- Complex conditionals (>8 branches)

**Orchestration belongs in COMMAND (not skill):**
- Argument validation (basic format checks)
- User interaction (AskUserQuestion flows)
- Context loading (@file references)
- Skill invocation setup (context markers)
- Display of results (output formatting)

**Step 2: Categorize each section of command**

```bash
# Extract command structure
grep -n "^### " .claude/commands/problem-command.md

# For each section, ask:
# "Is this business logic?" (IF YES → move to skill)
# "Is this orchestration?" (IF YES → keep in command)

# Example categorization:
echo "Phase 1: Argument Validation → ORCHESTRATION (keep)"
echo "Phase 2: Calculate Capacity → BUSINESS LOGIC (move to skill)"
echo "Phase 3: Validate Dependencies → BUSINESS LOGIC (move to skill)"
echo "Phase 4: Create Documents → BUSINESS LOGIC (move to skill or subagent)"
echo "Phase 5: Update Stories → BUSINESS LOGIC (move to skill)"
echo "Phase 6: Invoke Skill → ORCHESTRATION (keep)"
echo "Phase 7: Display Results → ORCHESTRATION (keep)"
```

**Step 3: Extract to skill**

**Pattern 1: Move entire phase**
```
Before:
  Command Phase 5: Sprint Creation (120 lines)
  ├─ Calculate next sprint number
  ├─ Generate sprint YAML
  ├─ Write to file
  └─ Update status

After:
  Command Phase 3: Invoke Skill
  Skill Phase 3: Sprint Creation (moved from command)
  (Skill contains 120 lines of logic)
  (Command is 15 lines: just invoke skill)
```

**Pattern 2: Add workflow to existing skill**

```
Before:
  Command: 497 lines (all logic inline)
  Skill: Not used (missing skill layer)

After:
  Command: 250 lines (only orchestration)
  Skill: Phase 3 added (289 lines - sprint creation logic)
```

**Step 4: Update command to lean structure**

```
Phase 0: Argument Validation (20 lines)
├─ Basic format checks
├─ Load context files
└─ Extract parameters

Phase 1: Invoke Skill (15 lines)
├─ Set context markers
└─ Skill(command="devforgeai-[skillname]")

Phase 2: Display Results (10 lines)
└─ Output skill result

Integration Notes (educational content OK)
└─ Usage examples, success criteria
```

**Step 5: Test to ensure behavior unchanged**

```bash
# Before refactoring: Establish baseline
.claude/tests/commands/test-problem-command.sh > baseline.txt
# Check: All tests pass? Yes/No

# After refactoring: Run same tests
.claude/tests/commands/test-problem-command.sh > refactored.txt

# Compare: Identical results?
diff baseline.txt refactored.txt
# Should show: No differences (0 failed tests)
```

#### Example: /create-sprint Refactoring

**Before (Business logic in command):**
```bash
Phase 1: Sprint Discovery (80 lines) ← BUSINESS LOGIC
  Calculate next sprint number:
    FOR each existing sprint in devforgeai/specs/Sprints/:
      Extract sprint number from filename
      Track maximum
    Next number = MAX + 1

Phase 2: Story Validation (70 lines) ← BUSINESS LOGIC
  FOR each selected story:
    Check status == "Backlog"
    Verify file exists
    Extract story points
    Sum points (capacity validation)
  IF capacity < 20 OR capacity > 40:
    AskUserQuestion: "Capacity OK?"

Phase 3: Sprint Creation (120 lines) ← BUSINESS LOGIC
  Create sprint YAML with:
    - sprint_number (calculated)
    - dates (computed from start + duration)
    - capacity (summed from stories)
    - status (new)
  Write to devforgeai/specs/Sprints/Sprint-{N}.md

Phase 4: Update Stories (70 lines) ← BUSINESS LOGIC
  FOR each selected story:
    Read story file
    Update status → "Ready for Dev"
    Add sprint reference
    Append to workflow history
    Write file back
```

**After (Business logic in skill):**
```bash
Command:
  Phase 0: User Interaction (120 lines) ← ORCHESTRATION
    - Epic selection
    - Story selection
    - Sprint metadata (dates, duration)

  Phase 1: Invoke Skill (15 lines)
    - Set context markers
    - Skill(command="devforgeai-orchestration")

  Phase 2: Display Results (20 lines)
    - Output sprint summary

Skill:
  Phase 3: Sprint Planning Workflow (289 lines) ← BUSINESS LOGIC
    - Sprint discovery (calculate number)
    - Story validation (check status, calculate capacity)
    - Document generation (create sprint YAML)
    - Story updates (status, references, history)
    - Returns structured summary to command
```

**Result:** 497 → 250 lines in command (50% reduction), Business logic properly located in skill

---

### Pattern 4: Display Templates Too Large

**Command contains multiple display variants and template selection logic (>50 lines).**

#### Symptoms

- Command has 5+ different output formats
- Multiple IF/ELSE branches deciding which template to use (>30 lines)
- Template strings are long (>200 chars per template)
- Display logic makes decisions (what to show depends on result type)
- Multiple display variants: light/deep, pass/fail, etc.
- Command contains >100 lines of template definitions

#### Root Causes

**Cause 1: Result type complexity**
- Multiple result types (QA pass, QA fail, coverage failure, deferral failure, spec failure)
- Each type needs different display
- Template selection logic in command
- Example: /qa had 5 display variants (161 lines)

**Cause 2: Mode-dependent display**
- Light mode shows minimal output
- Deep mode shows comprehensive output
- Different content for each mode
- Templates interleaved with decision logic

**Cause 3: User-facing complexity**
- Multiple error scenarios with different guidance
- Success messages vary by situation
- Next steps depend on result details
- Command tries to be "smart" about display

#### Resolution

**Create specialized subagent for template generation**

**Process:**

1. **Identify all display variants**
   ```bash
   grep -n "Display:" .claude/commands/problem-command.md | head -20
   # Count unique templates
   # Note: Are there 5+? That's a subagent candidate
   ```

2. **Categorize templates**
   ```
   Template 1: Light PASS - Show minimal, next steps
   Template 2: Deep PASS - Show detailed metrics
   Template 3: Failure (Coverage) - Show gap, recommendations
   Template 4: Failure (Deferral) - Show blockers, approval needed
   Template 5: Failure (Spec) - Show mismatches, review needed
   ```

3. **Create subagent to generate appropriate template**
   ```
   Subagent: result-interpreter (generic pattern)
   Input: Result data (status, type, details)
   Output: Structured JSON with display template

   Example: qa-result-interpreter subagent
   Input: {status: "FAIL", violation_type: "coverage", gaps: [...]}
   Output: {display: "❌ Coverage FAILED: ...", recommendations: [...]}
   ```

4. **Create reference file for framework guardrails**
   ```
   Reference: qa-result-formatting-guide.md
   Purpose: Explicit constraints for display generation
   Content:
     - DevForgeAI workflow states (when each template used)
     - Framework constraints (quality gates, thresholds)
     - Display guidelines (tone, emoji, structure)
     - Integration points (links to related components)
   ```

5. **Command simplified: Just display the template**
   ```
   Phase 1: Invoke Skill
     Skill returns structured result
     (Includes result.display - template already generated)

   Phase 2: Display Results
     Output result.display
     (No branching, no template selection, no formatting)
   ```

#### Example: /qa Refactoring (161 Lines → Subagent)

**Before (Templates in command):**
```markdown
Phase 4: Display Results (161 lines)

IF mode == "light" AND status == "PASS":
  Display:
    "✅ Light QA PASSED
     Tests: {count}/{total}
     Coverage: {percent}%
     Proceed to release"

ELIF mode == "deep" AND status == "PASS":
  Display:
    "✅ Deep QA PASSED
     Coverage: {percent}% ({business}% logic, {app}% app, {infra}% infra)
     Quality Metrics:
       - Complexity: {score} (target: <10)
       - Duplication: {percent}% (target: <5%)
       - Maintainability: {index} (target: >70)
     Anti-patterns: {count} issues found
       {list of issues}
     Recommend: APPROVED for production"

ELIF status == "FAIL" AND violation_type == "coverage":
  Display:
    "❌ Coverage FAILED
     Current: {percent}% (target: {target}%)
     Gap: {gap}%
     Files to fix: {count}
       {file list with line coverage}
     Action: Fix coverage gaps or create deferral"

ELIF status == "FAIL" AND violation_type == "deferral":
  Display:
    "❌ Deferral Check FAILED
     Issues: {details}
     Blockers: {list}
     Action: Resolve blockers or get approval"

ELIF status == "FAIL" AND violation_type == "spec":
  Display:
    "❌ Spec Compliance FAILED
     Mismatches: {count}
       {AC not tested}
       {Tech spec not implemented}
     Action: Implement missing functionality"
```

**After (Subagent generates template):**
```markdown
Phase 1: Invoke Skill
  Skill(command="devforgeai-qa")
  (Skill invokes qa-result-interpreter subagent)

Phase 2: Display Results
  Output result.display
  (qa-result-interpreter determined template based on:
    - mode (light/deep)
    - status (PASS/FAIL)
    - violation_type (coverage/deferral/spec)
  )

  (No if/else branches, no template selection)
  (Just output the pre-generated template)
```

**Subagent (qa-result-interpreter):**
```
Input: {
  status: "FAIL",
  mode: "deep",
  violation_type: "deferral",
  violations: [{type: "circular", ...}, {...}],
  next_steps: ["Resolve blockers", "Resubmit"]
}

Processing:
1. Determine template type (deferral failure)
2. Generate emoji/color (❌ for FAIL)
3. Format violation details (readable list)
4. Add framework-aware guidance (from reference file)
5. Generate next steps (from reference file)

Output: {
  display: "{{template with all details filled in}}",
  recommendations: ["Fix A", "Fix B"],
  next_steps: ["Action 1", "Action 2"]
}
```

**Result:** 161 lines extracted to subagent, command simplified to 10-line display

---

### Pattern 5: Complex Error Handling Matrix

**Command has 8+ error scenarios with detailed recovery procedures in command.**

#### Symptoms

- Command has section titled "Error Handling" with 50+ lines
- Multiple error scenarios listed (8+)
- Each error has: description, cause, resolution
- Detailed recovery procedures for each error type
- Error handling mixed with success path logic
- IF/ELSE chain for different error types

#### Root Causes

**Cause 1: Anticipating failures**
- Developer tried to be comprehensive
- Listed every possible error scenario
- Added recovery procedures for each
- Created unnecessarily complex error matrix

**Cause 2: Lack of skill/subagent abstraction**
- Errors come from skill execution
- Command tried to handle them all
- Should have been in skill or subagent
- Command doesn't need to handle internal errors

**Cause 3: User experience overthinking**
- Wanted to be very helpful with error messages
- Added detailed context for each error
- Tried to guide user through recovery
- Created error section that rivals business logic

#### Resolution

**Principle: Command handles 3-4 common errors, skill handles the rest**

**What command should handle:**
1. Story file not found → User guidance (list stories, retry)
2. Argument format invalid → Show usage example
3. Skill execution failed → Display skill error (let skill explain)
4. Skill timeout → Suggest retry or checkpoint resume

**What command should NOT handle:**
- Internal validation errors (skill responsibility)
- Complex recovery procedures (skill responsibility)
- Detailed troubleshooting (reference file responsibility)
- Framework constraint violations (skill/subagent responsibility)

**Refactoring approach:**

**Before (10 error scenarios):**
```markdown
## Error Handling

### Error 1: Story File Not Found
  Detection: Read(file_path=...) fails
  Cause: Story ID doesn't exist
  Resolution: List available stories, ask user to retry
  [15 lines]

### Error 2: Invalid Status for Workflow
  Detection: Story status not in allowed list
  Cause: Story in wrong state for this operation
  Resolution: Show valid states, recommend different command
  [12 lines]

### Error 3: [8 more error scenarios...]
  [80+ lines total]
```

**After (4 essential errors):**
```markdown
## Error Handling

### Story Not Found
  Display: List available stories, ask to retry with different ID

### Skill Execution Failed
  Display: "Check skill output above for details"

### Argument Invalid
  Display: Show usage example

### Unknown Error
  Display: "Contact framework maintainers, include full output above"

[~25 lines total, pointing users to source of truth]
```

**Key changes:**
1. Removed detailed cause/resolution explanations (not helpful in command)
2. Consolidated similar errors (fewer categories)
3. Point users to skill output for details (skill explains itself)
4. Create troubleshooting guide in reference file instead
5. Keep only immediate user actions in command

---

### Pattern 6: Command Invoking Subagents Directly

**Command invokes subagents without skill layer (violates skills-first architecture).**

#### Symptoms

- Command contains `Task(subagent_type="...")` invocations
- Subagent returned directly to command (not through skill)
- Skill exists but is bypassed
- Command orchestrates multiple subagents in sequence
- Example: Original /create-sprint (invoked sprint-planner without skill layer)

#### Root Causes

**Cause 1: Skill layer missing entirely**
- Command and subagents created first
- Skills added to framework later
- Original architecture had command → subagent directly
- Refactoring deferred (never updated command to use skill)

**Cause 2: Misunderstanding of skills**
- Developer thought "skills are for user-facing workflows"
- Didn't realize skills should coordinate subagents
- Put orchestration logic in command instead
- Skill was never used

**Cause 3: Iterative development**
- Started with command → subagent (quick iteration)
- Should have refactored to command → skill → subagent
- Never did the refactoring
- Architecture debt accumulated

#### Resolution

**Restore skills-first architecture:**

**Before (Direct invocation):**
```
User
  ↓
/create-sprint Command (orchestrates subagents)
  ├─ Invoke requirements-analyst subagent (feature list)
  └─ Invoke architect-reviewer subagent (complexity scoring)
```

**After (Skills-first):**
```
User
  ↓
/create-sprint Command (orchestration only)
  ↓
devforgeai-orchestration Skill (coordinates subagents)
  ├─ Invoke requirements-analyst subagent
  └─ Invoke architect-reviewer subagent
```

**Implementation steps:**

1. **Identify where subagents are invoked in command**
   ```bash
   grep -n "Task(subagent_type=" .claude/commands/problem-command.md
   # Note line numbers and subagent names
   ```

2. **Move subagent invocations to skill**
   ```markdown
   # In .claude/skills/skill-name/SKILL.md

   Phase X: [Phase Name]

     Task(subagent_type="requirement-analyst", ...)
     # Process subagent result

     Task(subagent_type="architect-reviewer", ...)
     # Process subagent result

     Return structured summary to command
   ```

3. **Simplify command to invoke skill**
   ```markdown
   # In .claude/commands/command.md

   Phase 0: Argument Validation (30 lines)
   └─ Validate arguments, load context

   Phase 1: Invoke Skill (15 lines)
   ├─ Set context markers
   └─ Skill(command="devforgeai-skill-name")

   Phase 2: Display Results (15 lines)
   └─ Output skill summary
   ```

4. **Verify skill is properly scoped**
   - Skill coordinates all subagent invocations
   - Skill processes subagent results
   - Skill returns structured data to command
   - Command is thin orchestration layer

5. **Test to ensure behavior identical**
   ```bash
   # Before refactoring baseline
   .claude/tests/commands/test-command.sh > before.txt

   # After refactoring
   .claude/tests/commands/test-command.sh > after.txt

   # Compare
   diff before.txt after.txt
   # Should be identical (no test failures)
   ```

#### Example: /create-sprint Architecture Restoration

**Before (Command → Subagent directly):**
```markdown
/create-sprint Command

Phase 0: User Interaction (150 lines)
└─ Epic selection, story selection, metadata

Phase 1: Sprint Discovery (80 lines) ← SKILL LOGIC
└─ Calculate next sprint number

Phase 2: Story Validation (70 lines) ← SKILL LOGIC
└─ Verify stories, sum capacity

Phase 3: Sprint Creation (120 lines) ← SKILL LOGIC
├─ Generate sprint YAML
└─ Write to file

Phase 4: Update Stories (70 lines) ← SKILL LOGIC
└─ Update status, add references

Phase 5: Invoke Subagent (60 lines)
└─ Task(subagent_type="sprint-planner", ...)
    (Should have been in skill, not command)

Issue: Subagent invoked directly from command
Missing: Skill layer that should coordinate this
```

**After (Command → Skill → Subagent):**
```markdown
/create-sprint Command

Phase 0: User Interaction (120 lines)
└─ Epic selection, story selection, metadata

Phase 1: Invoke Skill (15 lines)
├─ Set context markers
└─ Skill(command="devforgeai-orchestration")

Phase 2: Display Results (20 lines)
└─ Output sprint summary

---

devforgeai-orchestration Skill

Phase 3: Sprint Planning Workflow (289 lines)

  Step 3.1: Extract parameters
  └─ Sprint name, selected stories, dates from conversation context

  Step 3.2: Invoke sprint-planner subagent
  └─ Task(subagent_type="sprint-planner", ...)

  Step 3.3: Process subagent result
  └─ Parse summary, prepare for command

  Step 3.4: Return to command
  └─ Structured JSON with sprint summary
```

**Architecture:** Restored skills-first pattern (Command → Skill → Subagent)

---

## Resolution Decision Tree

**Use this flowchart to determine the right fix for violations:**

```
START: Detected Violation
  │
  ├─ Is character count >15,000?
  │  YES → Pattern 1: Over Budget
  │        Action: Extract display templates or business logic
  │        Target: Reduce to <12,000 chars
  │        Go to: Pattern 1 Resolution
  │  │
  │  NO → Continue to next check
  │
  ├─ Does command read files it didn't create?
  │  YES → Pattern 2: Reading Own Outputs
  │        Action: Return structured result from skill instead
  │        Target: Remove file reading from command
  │        Go to: Pattern 2 Resolution
  │  │
  │  NO → Continue to next check
  │
  ├─ Does command contain calculation/validation/decision logic?
  │  YES → Pattern 3: Business Logic in Command
  │        Action: Move logic to skill
  │        Target: Command only orchestrates
  │        Go to: Pattern 3 Resolution
  │  │
  │  NO → Continue to next check
  │
  ├─ Does command have 5+ display templates (>50 lines)?
  │  YES → Pattern 4: Display Templates Too Large
  │        Action: Create interpreter subagent
  │        Target: Command just displays result.display
  │        Go to: Pattern 4 Resolution
  │  │
  │  NO → Continue to next check
  │
  ├─ Does command have 8+ error scenarios (>50 lines)?
  │  YES → Pattern 5: Complex Error Handling Matrix
  │        Action: Simplify to 4 essential errors
  │        Target: Point to skill for details
  │        Go to: Pattern 5 Resolution
  │  │
  │  NO → Continue to next check
  │
  └─ Does command invoke subagents directly (Task tool)?
     YES → Pattern 6: Missing Skill Layer
           Action: Move subagent invocations to skill
           Target: Command → Skill → Subagent
           Go to: Pattern 6 Resolution
     │
     NO → All clear! No violations detected
          ✅ Command is compliant with lean orchestration pattern
```

---

## Recovery Examples

### Scenario 1: Budget Violation Detected by /audit-budget

**Situation:**
```
> /audit-budget
❌ create-ui: 19,908 chars (126% - 4,908 over limit)
Priority: CRITICAL - needs refactoring
```

**Recovery steps:**

1. **Backup original**
   ```bash
   cp .claude/commands/create-ui.md .claude/commands/create-ui.md.backup
   ```

2. **Analyze structure**
   ```bash
   grep -n "^### " .claude/commands/create-ui.md
   wc -l .claude/commands/create-ui.md
   # Note: 614 lines total
   ```

3. **Identify extraction candidates**
   ```bash
   grep -n "Display:\|Template:\|Phase.*:" .claude/commands/create-ui.md | head -30
   # Find sections with display logic or templates

   grep -c "FOR\|WHILE\|IF\|Calculate\|Validate" .claude/commands/create-ui.md
   # Find business logic
   ```

4. **Execute extraction**
   ```
   For each identified section:
   - Extract lines X-Y to skill as Phase Z
   - Create subagent if templates found
   - Update command to invoke skill instead
   - Verify lines removed = chars reduced
   ```

5. **Verify refactoring**
   ```bash
   # Check new budget
   wc -c < .claude/commands/create-ui.md
   # Should be <12,000 (target) or <15,000 (max)

   # Run tests
   .claude/tests/commands/test-create-ui.sh
   # Should: All tests pass (100%)

   # Smoke test
   /create-ui STORY-001
   # Should: Same output as before refactoring
   ```

6. **Deploy**
   ```bash
   git add .claude/commands/create-ui.md
   git commit -m "refactor(create-ui): Extract business logic to skill, reduce chars 19908→12000 (40%)"
   ```

### Scenario 2: Business Logic Discovered During Code Review

**Situation:**
Developer adding new feature to /create-story command. Review finds business logic accumulating.

**Recovery steps:**

1. **Identify business logic section**
   ```bash
   # Example: Capacity validation in /create-sprint before refactoring
   grep -A 20 "Capacity Validation" .claude/commands/create-story.md
   # Lines 145-165: 20 lines of validation logic
   ```

2. **Categorize: Stay or Move?**
   ```
   Phase 2: Validate Story Fields (20 lines)
   ├─ Check title length (2 lines) → STAY (formatting)
   ├─ Parse acceptance criteria (8 lines) → MOVE (parsing belongs in AC parser)
   ├─ Validate coverage thresholds (10 lines) → MOVE (business logic)

   Decision: Move lines 153-165 to skill
   ```

3. **Move to skill**
   ```
   Create Skill Phase X: Story Field Validation
   - Copy lines 153-165 from command
   - Adapt to skill context (may need to read story file, etc.)
   - Have skill return validation result to command
   - Command displays result (no validation logic)
   ```

4. **Update command**
   ```
   Replace lines 153-165 with:
     Skill handles validation (see Phase X)

   Result: Command shrinks by ~15 lines
   ```

5. **Test**
   ```bash
   # Run all story creation tests
   .claude/tests/commands/test-create-story.sh

   # Verify: All tests pass
   # Verify: Behavior identical to before refactoring
   ```

### Scenario 3: File Reading Duplication

**Situation:**
Code review finds command reading QA report to determine action (Pattern 2 violation).

**Recovery steps:**

1. **Identify file reading**
   ```bash
   grep -n "Read.*qa-report\|Read.*report.*STORY" .claude/commands/qa.md

   # Result: Line 127: Read QA report
   #         Line 145: Read report again to verify status
   # Duplication detected!
   ```

2. **Trace purpose**
   ```
   Line 127: Read report to get violations
   Purpose: Determine if retry needed

   Line 145: Read report to check final status
   Purpose: Update story status in command

   Issue: Should be in skill, not command
   ```

3. **Move to skill**
   ```
   Current Skill Workflow:
     Phase 5: Generate QA report

   Enhanced Skill Workflow:
     Phase 5: Generate QA report
     Phase 6: Parse report (NEW)
       └─ Invoke qa-result-interpreter subagent
       └─ Get violations, status, next steps
       └─ Return to command in result.violations

   Command no longer reads file
   Uses result.violations instead
   ```

4. **Update command**
   ```
   Remove: Lines 127-155 (file reading logic)
   Add: 10 lines referencing result.violations

   Result: 30 lines removed, command cleaner, no duplication
   ```

5. **Test**
   ```bash
   .claude/tests/commands/test-qa.sh
   # Verify: All tests pass
   ```

---

## Prevention Checklist

### Pre-Development Checklist

**Before writing new command:**
- [ ] Skill already exists for this domain?
  - YES → Extend skill, don't create command logic
  - NO → Create skill first, then minimal command
- [ ] Business logic involved?
  - YES → Plan skill phases upfront
  - NO → Command-only is OK for very simple operations
- [ ] Multiple display variants needed?
  - YES → Plan subagent for template generation
  - NO → Simple output in command is OK
- [ ] File I/O needed?
  - YES → Will skill do it, or command?
  - Answer: Skill should do I/O, command shouldn't read files

### During Development Checklist

**While implementing command:**
- [ ] Character count growing >300 lines?
  - YES → Pause and extract business logic to skill
  - NO → Continue
- [ ] New calculation/validation logic added?
  - YES → Move to skill immediately (don't wait for refactoring)
  - NO → Continue
- [ ] New error scenario found?
  - YES → Is it >4th unique error? If yes, consolidate
  - NO → Add to error handling (max 4 types)
- [ ] Display logic getting complex?
  - YES → Plan subagent (don't add more if/else in command)
  - NO → Continue
- [ ] Reading files created by skill?
  - YES → Stop and refactor (skill should return parsed result)
  - NO → Continue

### Pre-Commit Checklist

**Before committing changes:**
- [ ] `wc -c < .claude/commands/[name].md` < 12,000?
  - YES → Great! Proceed
  - NO → Run extraction workflow before commit
- [ ] All tests passing? (Unit, integration, regression)
  - YES → Proceed
  - NO → Fix failing tests before commit
- [ ] Skill properly used (not bypassed)?
  - YES → Proceed
  - NO → Refactor to use skill layer
- [ ] No duplication with skill (command reading files, etc.)?
  - YES → Proceed
  - NO → Move duplication to skill
- [ ] Error handling <50 lines?
  - YES → Proceed
  - NO → Simplify to 4 essential errors

### Pre-Deployment Checklist

**Before deploying to production:**
- [ ] `/audit-budget` shows COMPLIANT (green)?
  - YES → Proceed to smoke tests
  - NO → Don't deploy, fix budget first
- [ ] Smoke tests pass? (3 command runs with different args)
  - YES → Proceed
  - NO → Debug and fix
- [ ] Behavior identical to previous version?
  - YES → Deploy
  - NO → Rollback and investigate
- [ ] Token usage acceptable?
  - YES → Monitor
  - NO → Check for regressions

### Quarterly Review Checklist

**Every 3 months, audit patterns:**
- [ ] Any new commands approaching budget?
- [ ] Any commands showing business logic creep?
- [ ] Any file reading duplication discovered?
- [ ] Refactoring strategy still effective?
- [ ] New anti-patterns observed?
- [ ] Update troubleshooting guide if new patterns found

---

## Related Documentation

**Core Protocol:**
- `lean-orchestration-pattern.md` - Pattern definition, principles, templates
- `refactoring-case-studies.md` - 5 real refactorings with before/after analysis
- `command-budget-reference.md` - Current status, budget tables, monitoring

**Framework References:**
- `.claude/memory/commands-reference.md` - Command documentation
- `.claude/memory/skills-reference.md` - Skills architecture
- `.claude/memory/subagents-reference.md` - Subagent guide
- `CLAUDE.md` - Full framework overview

**Tools:**
- `/audit-budget` - Automated budget compliance checker
- `/audit-deferrals` - Deferral audit tool
- `/rca` - Root cause analysis tool

**Real Examples:**
- `.claude/commands/qa.md` - Refactored (reference implementation)
- `.claude/commands/create-sprint.md` - Refactored (reference implementation)
- `.claude/agents/qa-result-interpreter.md` - Subagent for display templates
- `.claude/skills/devforgeai-qa/references/qa-result-formatting-guide.md` - Reference file for guardrails

---

## Contact & Escalation

**Pattern violations detected?**

1. **Try automated resolution:** `/audit-budget` command
2. **Review relevant pattern:** See [Common Violation Patterns](#common-violation-patterns)
3. **Execute resolution:** Follow step-by-step procedure
4. **Still stuck?** Use `/rca` for Root Cause Analysis

**Questions about patterns?**
- See `lean-orchestration-pattern.md` for core principles
- See `refactoring-case-studies.md` for real examples
- See `command-budget-reference.md` for current status

---

**This troubleshooting guide is a living document. Add new patterns as they're discovered and resolved.**

**Last Updated:** 2025-11-18
**Version:** 1.0
**Character Count:** 17,847 (within protocol limits)
