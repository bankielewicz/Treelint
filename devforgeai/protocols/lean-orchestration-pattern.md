# Lean Orchestration Pattern Protocol

**Version:** 1.2
**Date:** 2025-11-06
**Status:** Active Protocol
**Applies To:** All DevForgeAI slash commands

**📚 Related Documentation:**
- **Case Studies:** See `refactoring-case-studies.md` for detailed refactoring examples (/qa, /dev, /sprint, /epic, /orchestrate)
- **Budget Reference:** See `command-budget-reference.md` for budget tables, monitoring, and appendices

---

## Purpose

This protocol defines the **lean orchestration pattern** for slash commands in the DevForgeAI framework, ensuring commands remain within character budget constraints while delegating business logic to skills and subagents.

**Problem Solved:**
- Commands becoming "top-heavy" with business logic (692 lines, 31K characters)
- Character budget violations (15K limit exceeded)
- Duplication between commands and skills
- Mixed concerns (validation + interpretation + display in one file)
- Token inefficiency (commands consuming 8K+ tokens in main conversation)

**Pattern Origin:**
- Established in `/dev` refactoring (860 → 513 lines, 40% reduction)
- Proven in `/qa` refactoring (692 → 295 lines, 57% reduction)
- Based on Claude Skills architectural principles (progressive disclosure, context isolation)

---

## Constitutional Principle

**Commands orchestrate. Skills validate. Subagents specialize.**

This separation of concerns ensures:
- ✅ Commands stay within 15K character budget
- ✅ Skills contain comprehensive business logic
- ✅ Subagents handle specialized tasks in isolated contexts
- ✅ Token efficiency (66-80% reduction in main conversation)
- ✅ Maintainability (single source of truth per concern)

---

## The Pattern

### Command Responsibilities (ONLY)

**What commands SHOULD do:**
1. **Parse arguments** - Extract and validate user input ($1, $2, etc.)
2. **Load context** - Load story/epic files via @file reference
3. **Set markers** - Provide explicit context statements for skills
4. **Invoke skill** - Single Skill(command="...") call
5. **Display results** - Output what skill returns (no parsing, no formatting)

**What commands should NOT do:**
- ❌ Business logic (validation algorithms, report generation)
- ❌ Complex parsing (reading reports, extracting sections)
- ❌ Template generation (creating display variants)
- ❌ Decision-making (branching on failure types)
- ❌ Error recovery (determining remediation strategies)

**Total target:** 150-300 lines, 6K-12K characters

---

### Skill Responsibilities

**What skills SHOULD do:**
1. **Extract parameters** - Parse story ID, mode, environment from conversation context
2. **Execute workflow** - Multi-phase validation, implementation, or deployment
3. **Invoke subagents** - Delegate specialized tasks to domain experts
4. **Generate outputs** - Reports, code, documentation
5. **Return results** - Structured summaries for command to display
6. **Communicate errors** - Clear error messages with recovery steps

**What skills should NOT do:**
- ❌ User interaction (AskUserQuestion belongs in commands for UX decisions)
- ❌ Argument parsing (commands handle user input)

**Total target:** 1,000-2,000 lines (comprehensive implementation logic)

---

### Subagent Responsibilities

**What subagents SHOULD do:**
1. **Specialized tasks** - Domain-specific work (testing, security, interpretation)
2. **Isolated context** - Heavy token work separate from main conversation
3. **Return structured data** - JSON or well-defined format for reliable parsing
4. **Reference guardrails** - Consult framework reference files for constraints

**What subagents should NOT do:**
- ❌ Make autonomous decisions (follow framework constraints)
- ❌ Operate in silos (must be framework-aware)
- ❌ Violate context files (tech-stack, anti-patterns, etc.)

**Total target:** 200-500 lines (focused, specialized logic)

---

## Character Budget Management

### Budget Limits

| Limit | Threshold | Action |
|-------|-----------|--------|
| **Hard Limit** | 15,000 chars | Must refactor if exceeded |
| **Warning** | 12,000 chars | Monitor, may need refactoring |
| **Target** | 6,000-10,000 chars | Optimal range |
| **Minimum** | 1,000 chars | Avoid over-optimization |

### Budget Calculation

```bash
# Check command budget
wc -c < .claude/commands/your-command.md

# Budget status
chars=$(wc -c < .claude/commands/your-command.md)
if [ $chars -gt 15000 ]; then
  echo "❌ OVER BUDGET: $chars chars (limit: 15,000)"
elif [ $chars -gt 12000 ]; then
  echo "⚠️ HIGH: $chars chars (approaching limit)"
else
  echo "✅ COMPLIANT: $chars chars"
fi
```

**For current command status, see:** `command-budget-reference.md`

---

## Refactoring Decision Matrix

### When to Refactor

Use this decision tree to determine if a command needs refactoring:

```
Is command >15,000 characters?
    ├─ YES → MUST refactor (budget violation)
    │
    └─ NO → Is command >12,000 characters?
              ├─ YES → SHOULD refactor (approaching limit)
              │
              └─ NO → Is command >10,000 characters?
                        ├─ YES → CONSIDER refactoring (optimization opportunity)
                        │
                        └─ NO → COMPLIANT (no action needed)
```

### Refactoring Priority

**See `command-budget-reference.md` for current command status and priority queue.**

---

## Refactoring Methodology

### Step 1: Identify Top-Heavy Logic

**Common patterns in top-heavy commands:**
- Complex argument validation (>30 lines)
- Multiple display templates (>100 lines)
- Error handling matrix (>50 lines)
- Business logic (validation, parsing, decision-making)
- Deferral/failure handling (branching logic)
- Report parsing and interpretation
- Result formatting and template generation

**For detailed examples, see:** `refactoring-case-studies.md` (Before/After comparisons)

### Step 2: Categorize Logic

For each section of logic, ask:

**Q1: Is this argument parsing?**
- YES → Keep minimal version in command (~20 lines)
- Example: Story ID format validation, mode parsing

**Q2: Is this business logic (validation, analysis, decisions)?**
- YES → Move to skill
- Example: Coverage calculations, anti-pattern detection, deferral validation

**Q3: Is this template generation or formatting?**
- YES → Create specialized subagent
- Example: Display templates, result interpretation, remediation guidance

**Q4: Is this error handling?**
- YES → Skill communicates errors, command displays them (~20 lines)
- Example: Minimal error display, refer to skill output for details

**Q5: Is this documentation or integration notes?**
- YES → Can keep (educational value) but make concise
- Example: Quick reference, integration patterns, success criteria

### Step 3: Determine Extraction Strategy

**Option A: Move to Skill** (Best for business logic)
- ✅ Use when: Validation, analysis, workflow logic
- ✅ Benefits: Single source of truth, comprehensive implementation
- ❌ Drawback: Skill grows (but that's acceptable)

**Option B: Create Subagent** (Best for specialized tasks)
- ✅ Use when: Parsing/interpretation, template generation, specialized analysis
- ✅ Benefits: Isolated context, token efficiency, testable
- ❌ Drawback: More files to maintain

**Option C: Delete** (Best for duplication)
- ✅ Use when: Duplicate guidance, redundant documentation
- ✅ Benefits: Reduced bloat, clearer focus
- ❌ Drawback: May lose useful context (review carefully)

### Step 4: Create Refactoring Plan

**Template:**
```markdown
# [Command] Refactoring Plan

## Current State
- Lines: [XXX]
- Characters: [XXX]
- Budget status: [OVER/HIGH/COMPLIANT]

## Issues Identified
1. [Phase/Section]: [Lines] - [Issue description]
2. [Phase/Section]: [Lines] - [Issue description]

## Extraction Strategy
1. [Section] → [Skill/Subagent/Delete] - [Rationale]
2. [Section] → [Skill/Subagent/Delete] - [Rationale]

## Target State
- Lines: [~XXX] (XX% reduction)
- Characters: [~XXX] (within budget)

## New Artifacts
- Subagent: [name] (if needed)
- Reference file: [name] (if subagent created)
- Skill updates: [changes needed]

## Testing
- Unit tests: [count]
- Integration tests: [count]
- Regression tests: [count]
```

### Step 5: Execute Refactoring

**Follow this sequence:**

1. **Create backup** (preserve original)
   ```bash
   cp .claude/commands/command.md .claude/commands/command.md.backup
   ```

2. **Create new artifacts** (if needed)
   - Subagent: `.claude/agents/new-subagent.md`
   - Reference: `.claude/skills/skill-name/references/guide.md`

3. **Refactor command**
   - Keep: Argument validation, context loading, skill invocation
   - Remove: Business logic, templates, complex error handling
   - Simplify: Error messages, integration notes

4. **Update skill** (if moving logic there)
   - Add: Phases/steps for logic from command
   - Add: Subagent invocations (if created subagent)
   - Add: Return structured results

5. **Test comprehensively**
   - Run all test cases (unit, integration, regression)
   - Verify token budgets
   - Confirm behavior unchanged

6. **Update references**
   - `.claude/memory/subagents-reference.md` (if created subagent)
   - `.claude/memory/commands-reference.md` (note refactoring)
   - `CLAUDE.md` (if pattern changes)

7. **Deploy**
   - Git commit with descriptive message
   - Restart terminal
   - Smoke tests (3 command runs)

**For complete refactoring examples, see:** `refactoring-case-studies.md`

---

## Implementation Checklist

### Pre-Refactoring

- [ ] Analyze current command structure (count lines per phase)
- [ ] Check character budget: `wc -c < .claude/commands/command.md`
- [ ] Identify top-heavy sections (>50 lines of non-orchestration logic)
- [ ] Determine extraction strategy (skill vs subagent vs delete)
- [ ] Review skill to see what's already there (avoid duplication)

### During Refactoring

- [ ] Create backup: `cp command.md command.md.backup`
- [ ] Create new subagent (if needed)
- [ ] Create reference file for subagent (framework guardrails)
- [ ] Refactor command to lean structure (150-300 lines)
- [ ] Update skill with new phases/subagent invocations
- [ ] Test argument validation (various inputs)
- [ ] Test skill invocation (context extraction works)
- [ ] Test subagent (if created) - unit tests
- [ ] Integration test (full workflow)
- [ ] Regression test (behavior unchanged)
- [ ] Verify character budget: `wc -c < command.md` (should be <12K)

### Post-Refactoring

- [ ] Update `.claude/memory/commands-reference.md`
- [ ] Update `.claude/memory/subagents-reference.md` (if created subagent)
- [ ] Document refactoring in `devforgeai/specs/enhancements/`
- [ ] Git commit with descriptive message
- [ ] Restart terminal
- [ ] Smoke tests (3 command runs)
- [ ] Monitor for 1 week (token usage, errors)

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Command with Business Logic

**Problem:**
```markdown
# Command contains validation algorithms
Phase 2: Validate Coverage
  FOR each file:
    Calculate coverage percentage
    Compare to threshold
    IF below threshold:
      Generate gap report
```

**Solution:**
```markdown
# Command delegates to skill
Phase 1: Invoke Skill
  Skill(command="devforgeai-qa")

# Skill contains validation logic
```

### Anti-Pattern 2: Reading Files Command Already Generated

**Problem:**
```markdown
Phase 1: Skill generates QA report
Phase 2: Command reads QA report from disk
Phase 3: Command parses report sections
```

**Solution:**
```markdown
Phase 1: Skill generates report AND invokes subagent to parse it
Phase 2: Skill returns parsed result to command
Phase 3: Command displays result (no file reading)
```

### Anti-Pattern 3: Multiple Display Templates in Command

**Problem:**
```markdown
# Command has 5 display templates (161 lines)
IF mode == "light" AND status == "PASS":
  Display: "✅ Light QA PASSED..."
IF mode == "deep" AND status == "PASS":
  Display: "✅ Deep QA PASSED... [full metrics]"
IF mode == "deep" AND status == "FAIL" AND violation_type == "coverage":
  Display: "❌ Coverage Failed... [specific template]"
...
```

**Solution:**
```markdown
# Subagent generates appropriate template
Phase 1: Skill invokes qa-result-interpreter
Phase 2: Subagent determines template based on mode/result/violations
Phase 3: Subagent returns template in result.display
Phase 4: Command outputs result.display (no logic)
```

### Anti-Pattern 4: Complex Error Handling Matrix

**Problem:**
```markdown
# 97 lines of error scenarios
Error: Story not found (10 lines)
Error: Invalid status (12 lines)
Error: Skill failed (19 lines)
Error: Missing context (18 lines)
Error: Report missing (17 lines)
Error: Invalid mode (11 lines)
Error: [10 more scenarios...]
```

**Solution:**
```markdown
# Minimal error handling (25 lines)
Error: Story ID invalid
  → "Usage: /command STORY-001"

Error: Story file not found
  → List available stories

Error: Skill execution failed
  → Display skill error output

All other errors: Skill communicates them
```

### Anti-Pattern 5: Argument Parsing with All Edge Cases

**Problem:**
```markdown
# 99 lines of argument validation
IF $1 empty:
  AskUserQuestion...
ELIF $1 invalid format:
  AskUserQuestion...
ELIF story file not found:
  AskUserQuestion...
ELIF $2 is flag:
  Parse flag...
  IF flag invalid:
    AskUserQuestion...
  ELSE:
    Extract value...
ELIF $2 is value:
  Validate value...
...
```

**Solution:**
```markdown
# 20 lines of essential validation
IF $1 empty OR $1 doesn't match "STORY-[0-9]+":
  AskUserQuestion: "What story ID?"

Glob: devforgeai/specs/Stories/$1*.story.md
IF not found:
  AskUserQuestion: "Story not found. Cancel or list stories?"

MODE = $2 or infer from story status
```

---

## Template: Lean Command Structure

Use this template for all refactored commands:

```markdown
---
description: [Brief command description]
argument-hint: [arg1] [arg2]
model: opus
allowed-tools: Read, Skill, AskUserQuestion, [others as needed]
---

# /command-name - [Title]

[One-paragraph description]

---

## Quick Reference

```bash
# Common usage examples (3-5 examples)
/command ARG1 ARG2
/command ARG1       # With default
```

---

## Command Workflow

### Phase 0: Argument Validation and Context Loading

**Validate required arguments:**
```
IF $1 invalid:
  AskUserQuestion with clear options
  Extract from response OR exit
```

**Load context:**
```
@.ai_docs/[Type]/$1.[type].md
```

**Parse optional arguments:**
```
ARG2 = $2 or default_value
```

**Validation summary:**
```
✓ [Primary arg]: ${VALUE}
✓ [Secondary arg]: ${VALUE}
✓ Proceeding...
```

---

### Phase 1: Invoke Skill

**Set context markers:**
```
**[Primary Param]:** ${VALUE}
**[Secondary Param]:** ${VALUE}
```

**Invoke skill:**
```
Skill(command="devforgeai-[skillname]")
```

**What skill does:**
[Brief description of skill workflow]

---

### Phase 2: Display Results

**Receive and output:**
```
# Skill returns structured result
output result.display

# OR skill returns summary text
output skill_result
```

---

### Phase 3: Provide Next Steps

**Display recommendations:**
```
output result.next_steps

# OR
Based on result:
- [Action 1]
- [Action 2]
```

---

## Error Handling

### [Error Type 1]
```
Error: [Description]
Action: [What to do]
```

### [Error Type 2]
```
Error: [Description]
Action: [What to do]
```

[Keep minimal - 3-5 error types max]

---

## Success Criteria

- [ ] [Criterion 1]
- [ ] [Criterion 2]
- [ ] [Criterion 3]
- [ ] Token usage [target]
- [ ] Character count <12K

---

## Integration

**Invoked by:** [Who calls this command]
**Invokes:** [What skills/subagents]
**Updates:** [What files change]

---

## Performance

**Token Budget:**
- Command overhead: [~XXK]
- Skill execution: [~XXK] (isolated)
- Total main conversation: [~XXK]

**Execution Time:**
- Typical: [X-Y minutes]
- Complex: [Y-Z minutes]
```

**Target:** 150-300 lines, 6K-12K characters

---

## Subagent Creation Guidelines

### When to Create a Subagent

Create a specialized subagent when:
- ✅ Task is computationally expensive (parsing, analysis)
- ✅ Task has specialized domain knowledge (security, testing)
- ✅ Task benefits from isolated context (heavy token usage)
- ✅ Task is reusable across multiple skills
- ✅ Task needs framework guardrails (interpretation, formatting)

Don't create a subagent when:
- ❌ Logic is simple (<30 lines)
- ❌ Task is command-specific (not reusable)
- ❌ Logic already exists in skill
- ❌ No framework constraints needed

### Subagent Template

```markdown
---
name: [subagent-name]
description: [When to invoke this subagent]. Use [specific scenarios].
model: opus|sonnet|inherit
tools: Read, Grep, Glob, [others - minimal set]
---

# [Subagent Title]

[One-paragraph purpose statement]

## Purpose

This subagent:
1. [Primary responsibility]
2. [Secondary responsibility]
3. [Tertiary responsibility]

## When Invoked

**Proactively triggered:**
- [Automatic invocation scenario 1]
- [Automatic invocation scenario 2]

**Explicit invocation:**
```
Task(
  subagent_type="[name]",
  description="[Brief task]",
  prompt="[Detailed instructions with context]"
)
```

**Not invoked:**
- [When NOT to use]

## Workflow

### Step 1: [First Step]

[Instructions for step 1]

### Step 2: [Second Step]

[Instructions for step 2]

### Step N: Return Structured Result

```json
{
  "status": "...",
  "data": {...},
  "recommendations": [...]
}
```

---

## Framework Integration

**Invoked by:** [Which skills invoke this]
**Requires:** [What context/files needed]
**References:** [What reference files to consult]
**Returns:** [What gets sent back]

---

## Framework Constraints

[List key DevForgeAI rules this subagent must follow]

**Load reference file for complete constraints:**
```
Read(file_path=".claude/skills/[skill]/references/[guide].md")
```

---

## Success Criteria

- [ ] [Criterion 1]
- [ ] [Criterion 2]
- [ ] Token usage <[target]
- [ ] Returns structured output
- [ ] Framework-aware (not siloed)

---

## Error Handling

### [Error Scenario 1]
**Detection:** [How to detect]
**Response:** [What to return]
**Recovery:** [How caller should handle]

---

## Testing Checklist

- [ ] Unit test: [Test scenario 1]
- [ ] Unit test: [Test scenario 2]
- [ ] Integration: [Full workflow test]
- [ ] Regression: [Behavior unchanged]
- [ ] Performance: Token usage <[target]
```

**Target:** 200-500 lines, framework-aware, minimal tool access

---

## Reference File Template

### When to Create a Reference File

**Create reference file when subagent needs:**
- ✅ Framework context (workflow states, quality gates)
- ✅ Immutable constraints (coverage thresholds, violation rules)
- ✅ Decision boundaries (what's valid, what's invalid)
- ✅ Display guidelines (templates, tone, structure)
- ✅ Integration patterns (how to work with other components)

**Don't create reference file when:**
- ❌ Subagent has minimal logic (<100 lines)
- ❌ No framework-specific constraints
- ❌ Task is generic (not DevForgeAI-specific)

### Reference File Structure

```markdown
# [Topic] Guide

**Purpose:** Framework-aware guardrails for [subagent-name]

[One-paragraph description of why this reference exists]

---

## DevForgeAI Context

### [Framework Concept 1]

[Explain concept relevant to subagent's work]

**Example:**
```
[Concrete example from framework]
```

### [Framework Concept 2]

[Another relevant concept]

---

## Framework Constraints

### 1. [Constraint Category 1] (Strict, Immutable)

[Define the constraint]

**Rules:**
- [Rule 1]
- [Rule 2]

**Display guidance:**
- [How to present this]
- [What NOT to say]

**Example:**
```
[Concrete example]
```

### 2. [Constraint Category 2] (Deterministic)

[Define the constraint]

---

## [Subagent-Specific] Guidelines

### [Guideline Topic 1]

[Specific instructions for subagent task]

**Template:**
```
[Example template or structure]
```

### [Guideline Topic 2]

[More instructions]

---

## Framework Integration Points

### Context Files to Reference

- **tech-stack.md** - [When to check]
- **anti-patterns.md** - [When to check]
- [Other context files as relevant]

### Related Skills/Subagents

- **[Related component 1]** - [When to invoke]
- **[Related component 2]** - [When to coordinate]

---

## Error Scenarios

### [Error Type 1]

**Detection:** [How subagent detects]
**Response:** [What to return]
**Guidance:** [How caller should handle]

---

## Testing Checklist

- [ ] Subagent respects constraint 1
- [ ] Subagent respects constraint 2
- [ ] Output format matches guidelines
- [ ] Framework-aware behavior verified
- [ ] Integration with [skill] tested
```

**Target:** 200-400 lines, framework-specific, explicit constraints

---

## Pattern Consistency Analysis

### Overview

This section analyzes all 5 completed command refactorings to identify consistent patterns and successful techniques. For detailed Before/After code comparisons, see `refactoring-case-studies.md`.

**Refactorings analyzed:**
- Case Study 1: /dev refactoring (860 → 513 lines, 40% reduction)
- Case Study 2: /qa refactoring (692 → 295 lines, 57% reduction)
- Case Study 3: /create-sprint refactoring (497 → 250 lines, 50% reduction)
- Case Study 4: /create-epic refactoring (526 → 392 lines, 25% reduction)
- Case Study 5: /orchestrate refactoring (599 → 527 lines, 12% reduction)

### Metrics Across All Refactorings

Comprehensive comparison of all 5 completed command refactorings:

| Command | Before Lines | After Lines | Reduction | Before Chars | After Chars | Char Reduction | Token Savings |
|---------|--------------|-------------|-----------|--------------|-------------|----------------|---------------|
| /dev | 860 | 513 | 40% | 38,000 | 12,630 | 67% | 67% |
| /qa | 692 | 295 | 57% | 31,000 | 7,205 | 77% | 66% |
| /create-sprint | 497 | 250 | 50% | 12,525 | 8,000 | 36% | 58% |
| /create-epic | 526 | 392 | 25% | 14,309 | 11,270 | 21% | 80% |
| /orchestrate | 599 | 527 | 12% | 15,012 | 14,422 | 4% | 37% |
| **AVERAGE** | **635** | **395** | **37%** | **22,169** | **10,705** | **41%** | **62%** |

### Success Patterns

**All 5 refactorings achieved:**
- ✅ Budget compliance (<15K chars)
- ✅ Token efficiency improvement (37-80% savings)
- ✅ 100% backward compatibility (behavior unchanged)
- ✅ Clearer architecture (skills-first restored where violated)
- ✅ Framework-aware subagents (no silos)

**Variation in reduction percentages:**
- High reduction (57%): Display-heavy commands (/qa with 161-line templates)
- Moderate reduction (40-50%): Logic-heavy commands (/dev, /create-sprint)
- Low reduction (12-25%): Already had skill invocation, just moved phases (/orchestrate, /create-epic)

**Key insight:** Success = compliance, not maximum reduction. Commands at 96% budget (orchestrate) are compliant and successful.

### Common Refactoring Techniques

**Technique 1: Extract Display Templates to Subagent**
- **Used in:** /qa refactoring
- **Before:** 161 lines of templates in command
- **After:** qa-result-interpreter subagent generates templates
- **Savings:** 161 lines extracted, command displays result.template
- **When to use:** Command has >50 lines of display templates with multiple variants

**Technique 2: Extract Pre-Flight Validation to Subagent**
- **Used in:** /dev refactoring
- **Before:** Git detection (84 lines), tech detection (93 lines) in command
- **After:** git-validator subagent, tech-stack-detector subagent
- **Savings:** 177 lines extracted to 2 subagents
- **When to use:** Command has >50 lines of validation logic that's reusable across commands

**Technique 3: Extract Workflow Phases to Skill**
- **Used in:** /create-sprint, /create-epic, /orchestrate refactorings
- **Before:** All phases in command
- **After:** Phases in skill, command delegates
- **Savings:** 134-410 lines per command moved to skill
- **When to use:** Command has detailed phase logic, skill layer exists but underutilized

**Technique 4: Preserve User Interaction in Command**
- **Used in:** All 5 refactorings
- **Pattern:** AskUserQuestion stays in commands, not skills
- **Rationale:** Commands handle UX decisions, skills handle business logic
- **Examples:** /create-sprint (11 AskUserQuestion instances), /create-epic (4 instances), /qa (1 instance)

**Technique 5: Two-Pass Trimming**
- **Used in:** /create-epic refactoring
- **First pass:** Extract business logic (526 → 498 lines, 5% reduction, still 104% budget)
- **Second pass:** Externalize educational content (498 → 392 lines, 21% more, 75% budget)
- **When to use:** First pass doesn't achieve compliance, command has substantial educational content

### Extraction Strategy Patterns

**Subagent Creation Decisions:**

| Command | Subagent Created? | Rationale |
|---------|-------------------|-----------|
| /dev | ✅ YES (2) | git-validator, tech-stack-detector (pre-flight checks) |
| /qa | ✅ YES (1) | qa-result-interpreter (display template generation) |
| /create-sprint | ✅ YES (1) | sprint-planner (document generation) |
| /create-epic | ❌ NO | Existing subagents sufficient (requirements-analyst, architect-reviewer) |
| /orchestrate | ❌ NO | Coordination logic stays in skill |

**Pattern:** Create subagent when task is specialized and reusable. Use existing subagents when suitable.

**Skill Enhancement Decisions:**

| Command | Skill Enhanced? | Lines Added | Rationale |
|---------|-----------------|-------------|-----------|
| /dev | ❌ NO | 0 | Skill already comprehensive |
| /qa | ❌ NO | 0 | Skill already comprehensive |
| /create-sprint | ✅ YES | +289 | Added Phase 3 to orchestration skill |
| /create-epic | ✅ YES | +1,424 | Added Phase 4A (8-phase epic workflow) |
| /orchestrate | ✅ YES | +898 | Added Phases 0, 3.5, 6 + 3 skill integrations |

**Pattern:** Enhance skill when command has business logic that belongs in skill layer.

---

## Best Practices

### Do's ✅

1. **Start with analysis** - Understand current structure before refactoring
2. **Create backup** - Always preserve original command
3. **Test comprehensively** - 30+ test cases (unit, integration, regression)
4. **Document decisions** - Create analysis document explaining "why"
5. **Measure tokens** - Verify efficiency improvements
6. **Use proven pattern** - Follow /qa and /dev examples
7. **Create guardrails** - Reference files for subagents
8. **Keep it lean** - Target 150-300 lines for commands

### Don'ts ❌

1. **Don't skip analysis** - Refactoring without understanding causes issues
2. **Don't skip backups** - Always preserve original
3. **Don't skip testing** - Behavior changes must be caught before deployment
4. **Don't over-optimize** - Commands at 80-90% budget may not need refactoring yet
5. **Don't create silos** - Subagents must be framework-aware
6. **Don't duplicate** - If logic exists in skill, don't put it in command
7. **Don't guess** - Use AskUserQuestion if uncertain about extraction strategy
8. **Don't rush** - Refactoring takes 4-6 hours for good reason

---

## Quick Reference

### Command Checklist

Before deploying a new or updated command:
- [ ] Lines: 150-300 (target)
- [ ] Characters: <12K (target) or <15K (max)
- [ ] Phases: 3-5 (lean workflow)
- [ ] Business logic: None (delegated to skill)
- [ ] Display templates: None (generated by skill/subagent)
- [ ] Error handling: Minimal (3-5 types)
- [ ] Documentation: Concise (integration notes only)

### Refactoring Triggers

Refactor when:
- [ ] Command >15K characters (MUST)
- [ ] Command >12K characters (SHOULD)
- [ ] Command has business logic (validation, calculations)
- [ ] Command has display templates (>50 lines)
- [ ] Command reads files it didn't create (duplication)
- [ ] Command has complex error matrix (>50 lines)

### Pattern Application

1. Analyze: Count lines per phase, identify logic
2. Extract: Move business logic to skill, templates to subagent
3. Test: 30+ cases (unit, integration, regression)
4. Deploy: Backup, refactor, verify, monitor
5. Document: Analysis, summary, checklist

---

## Related Documentation

**Core Principles:**
- `.ai_docs/claude-skills.md` - Skills architecture and progressive disclosure
- `.ai_docs/Terminal/slash-commands-best-practices.md` - Command design patterns
- `.ai_docs/Terminal/sub-agents.md` - Subagent architecture and context isolation

**Implementation Examples:**
- `refactoring-case-studies.md` - 5 complete case studies with Before/After analysis
- `.claude/commands/qa.md` - Reference implementation (refactored)
- `.claude/commands/create-sprint.md` - Reference implementation (refactored)
- `.claude/commands/create-epic.md` - Reference implementation (refactored)

**Budget Monitoring:**
- `command-budget-reference.md` - Current status tables, monitoring procedures, appendices
- `/audit-budget` command - Automated budget compliance audit

**Refactoring Documentation:**
- `devforgeai/specs/enhancements/QA-COMMAND-REFACTORING-ANALYSIS.md` - Deep analysis
- `devforgeai/specs/enhancements/CREATE-SPRINT-REFACTORING-SUMMARY.md` - Implementation summary
- `devforgeai/specs/enhancements/ORCHESTRATE-COMPLETE-2025-11-06.md` - Orchestrate refactoring

---

## Version History

**v1.0 (2025-11-05):**
- Initial protocol document
- Based on /dev and /qa refactorings
- Establishes lean orchestration as standard pattern
- Documents character budget management
- Provides refactoring methodology
- Includes 5-command priority queue

**v1.1 (2025-11-05):**
- Added Case Study 3: /create-sprint refactoring
- Updated command status tables (5 refactored, 4 over-budget remaining)
- Updated priority queue (5 → 4 commands)
- Added sprint-planner subagent documentation
- Updated Pattern Maturity section (3 refactorings complete)

**v1.2 (2025-11-06):**
- Added Case Study 4: /create-epic refactoring (526 → 392 lines, 25% reduction)
- Enhanced devforgeai-orchestration skill with epic creation workflow
- Created 3 new reference files (feature-decomposition-patterns, technical-assessment-guide, epic-validation-checklist)
- Updated command status tables (6 refactored, 4 over-budget remaining, 3 compliant)
- Updated Pattern Maturity section (4 refactorings complete, 10 lessons learned)
- Added create-epic as reference implementation
- **Split into 3 files (49K → 15K + 17K + 17K) for startup performance**

**v1.3 (Future):**
- Add automated budget monitoring enhancements
- Command scaffolding templates
- Refactoring runbook automation

---

**This protocol is a living document. Update as new patterns emerge and refactorings are completed.**

**Character count:** ~15,153 characters (31% of original, well under 40K limit)
