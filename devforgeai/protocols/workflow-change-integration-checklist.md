# Workflow Change Integration Checklist

**Purpose:** Ensure all touchpoints updated when adding/modifying DevForgeAI workflow phases

**Created:** 2025-11-18 (RCA-010 Recommendation 2)
**Version:** 1.0
**Applies To:** All DevForgeAI skills with multi-phase workflows

---

## When to Use This Checklist

**Use when:**
- Adding new phase to skill workflow
- Modifying existing phase logic
- Implementing RCA recommendations that change workflow
- Enhancing validation checkpoints
- Splitting phases (e.g., Phase 4 → Phase 4a + Phase 4b)
- Merging phases (e.g., Phase 2 + Phase 3 → single Phase 2)

**Don't use when:**
- Fixing typos in documentation (no workflow change)
- Updating examples or clarifications (no structural change)
- Adding comments or notes (no execution change)

---

## The 8 Touchpoints

When adding/modifying a workflow phase, update ALL 8 touchpoints to prevent integration gaps.

### Touchpoint 1: SKILL.md Workflow Overview

**What to update:** The TDD Workflow section listing all phases

**Location:** `.claude/skills/{skill}/SKILL.md`
**Section:** "TDD Workflow (N Phases)" or "Workflow Overview"

**What to check:**
- [ ] Phase listed in correct sequential order
- [ ] Phase has brief description
- [ ] Reference file name included
- [ ] Purpose statement included
- [ ] MANDATORY marker added (if required phase)

**Example:**
```markdown
### Phase 4.5-5 Bridge: DoD Update Workflow ✓ MANDATORY (NEW - RCA-009)
Update DoD format for git commit → Validate format → Prepare for Phase 5
**Reference:** `dod-update-workflow.md`
**Purpose:** Ensure DoD items formatted correctly
```

---

### Touchpoint 2: TodoWrite Execution Tracker

**What to update:** The TodoWrite tracker array in SKILL.md

**Location:** `.claude/skills/{skill}/SKILL.md`
**Section:** "Workflow Execution Checklist"

**What to check:**
- [ ] New phase added to todos array
- [ ] Positioned in correct sequence
- [ ] content, status, activeForm all specified
- [ ] Total phase count comment updated (if present)

**Example:**
```javascript
TodoWrite(
  todos=[
    ...
    {content: "Execute Phase 4.5: Deferral Challenge (...)", status: "pending", activeForm: "..."},
    {content: "Execute Phase 4.5-5 Bridge: Update DoD Checkboxes (...)", status: "pending", activeForm: "..."}, // NEW
    {content: "Execute Phase 5: Git Workflow (...)", status: "pending", activeForm: "..."},
    ...
  ]
)
```

**CRITICAL:** This is the enforcement mechanism. Without tracker entry, phase can be skipped.

---

### Touchpoint 3: Complete Workflow Execution Map

**What to update:** The visual workflow diagram showing all mandatory steps

**Location:** `.claude/skills/{skill}/SKILL.md`
**Section:** "Complete Workflow Execution Map"

**What to check:**
- [ ] Phase listed in visual diagram
- [ ] Arrows show correct flow
- [ ] MANDATORY markers present
- [ ] Common skip points noted (if applicable)

**Example:**
```markdown
Phase 4.5: Deferral Challenge (phase-4.5-deferral-challenge.md)
  ├─ Detect deferrals ✓ MANDATORY
  └─ User approval ✓ MANDATORY IF deferrals exist
  ↓
Phase 4.5-5 Bridge: DoD Update (dod-update-workflow.md ← NEW)  // ADD THIS
  ├─ Mark DoD items [x] ✓ MANDATORY
  ├─ Add items to Implementation Notes ✓ MANDATORY
  └─ Validate format: devforgeai validate-dod ✓ MANDATORY
  ↓
Phase 5: Git Workflow (git-workflow-conventions.md)
```

---

### Touchpoint 4: Subagent Coordination Section

**What to update:** The subagent invocation sequences documenting which subagents run in which phases

**Location:** `.claude/skills/{skill}/SKILL.md`
**Section:** "Subagent Coordination"

**What to check:**
- [ ] Phase added to coordination list
- [ ] Subagents involved specified (or "No subagents" if direct operations)
- [ ] Invocation pattern documented (sequential/parallel)
- [ ] Token costs noted
- [ ] MANDATORY markers included

**Example:**
```markdown
### Phase 4.5-5 Bridge: DoD Update Workflow

**No subagents** - Direct file operations to update DoD format

**CRITICAL:** Execute dod-update-workflow.md AFTER Phase 4.5, BEFORE Phase 5

**Operations:**
- Edit story file to mark DoD checkboxes [x]
- Add completion notes to Implementation Notes
- Validate format with devforgeai validate-dod

**Token cost:** ~1,500 tokens (loaded on-demand)
```

---

### Touchpoint 5: Reference File Creation

**What to update:** Create phase-specific reference file if workflow complexity requires it

**Location:** `.claude/skills/{skill}/references/{phase-name}.md`
**Section:** N/A (new file)

**What to check:**
- [ ] Reference file created (if needed)
- [ ] Added to references/ directory
- [ ] Documented in skill's reference file list
- [ ] Included in progressive loading notes
- [ ] Token cost estimated

**Decision criteria:**
- **Create reference file if:** Phase has >50 lines of instructions, complex logic, or multiple steps
- **Keep inline if:** Phase is simple (<20 lines), single operation, or rarely executed

**Example:**
```markdown
# File: .claude/skills/devforgeai-development/references/dod-update-workflow.md

# DoD Update Workflow (Phase 4.5-5 Bridge)

**Purpose:** Update Definition of Done items after validation

**Execution:** After Phase 4.5, BEFORE Phase 5

[4-step workflow with detailed instructions]
```

---

### Touchpoint 6: Command Documentation

**What to update:** Command notes if phase affects user-visible behavior

**Location:** `.claude/commands/{command}.md`
**Section:** "Note" or "Success Criteria" or "What the skill does"

**What to check:**
- [ ] Command notes updated (if user-facing impact)
- [ ] Success criteria includes new phase
- [ ] Workflow documentation reflects change
- [ ] No duplication with skill (keep lean)

**Example:**
```markdown
**Note:** The `devforgeai-development` skill handles all implementation logic including:
- ...
- Phase 4.5: Deferral Challenge (validate deferred items)
- Phase 4.5-5 Bridge: DoD Update (mark checkboxes) ← ADD IF USER-FACING
- Phase 5: Git Workflow (commit changes)
- ...
```

**When to skip:** If phase is internal implementation detail (not user-visible), command documentation doesn't need update.

---

### Touchpoint 7: Memory Reference Files

**What to update:** Framework reference guides in `.claude/memory/`

**Location:** `.claude/memory/*.md`
**Files:** `skills-reference.md`, `commands-reference.md`, relevant guides

**What to check:**
- [ ] skills-reference.md updated (if major workflow change)
- [ ] commands-reference.md updated (if user-facing)
- [ ] Relevant guides updated (qa-automation.md, token-efficiency.md, etc.)

**Decision criteria:**
- **Update if:** Major workflow change (new quality gate, new validation, user-facing behavior change)
- **Skip if:** Internal implementation detail, minor refactoring

**Example in skills-reference.md:**
```markdown
**Phase 4.5-5 Bridge (NEW - RCA-009, RCA-010):**
- Purpose: Update DoD checkboxes and validate format
- When: After deferral validation, before git commit
- Enforcement: TodoWrite tracker (RCA-010 fix)
```

---

### Touchpoint 8: Protocol Documentation

**What to update:** Framework protocols in `devforgeai/protocols/`

**Location:** `devforgeai/protocols/*.md`
**Files:** `lean-orchestration-pattern.md`, case studies, budget references

**What to check:**
- [ ] lean-orchestration-pattern.md updated (if architectural change)
- [ ] refactoring-case-studies.md updated (if refactoring pattern change)
- [ ] command-budget-reference.md updated (if affects budget)

**Decision criteria:**
- **Update if:** Pattern-level change (affects multiple skills/commands), architectural principle change
- **Skip if:** Skill-specific implementation detail

**Example:**
```markdown
# In lean-orchestration-pattern.md:

**Skill Responsibilities:**
...
- Execute workflow phases including validation bridges (e.g., Phase 4.5-5 Bridge for DoD format)
```

---

### Touchpoint 9: Testing

**What to update:** Add tests for new phase execution

**Location:** `tests/unit/`, `tests/integration/`
**Files:** Skill-specific test files

**What to check:**
- [ ] Unit tests verify phase executes
- [ ] Unit tests verify sequence enforcement (phase X before phase Y)
- [ ] Integration tests include new phase in full workflow
- [ ] Regression tests verify existing phases unchanged

**Example:**
```python
def test_phase_4_5_5_bridge_executes_between_4_5_and_5():
    """Test: Phase 4.5-5 Bridge executes in correct sequence"""
    # Arrange: Mock story with DoD items
    # Act: Execute workflow
    # Assert: Phase 4.5-5 Bridge marked completed AFTER 4.5, BEFORE 5
```

---

## Validation Checklist

After updating all touchpoints, verify consistency:

### Phase Count Consistency

Count phases in each location - **all should match:**

- [ ] SKILL.md Workflow Overview lists **N phases**
- [ ] TodoWrite tracker has **N items**
- [ ] Complete Workflow Execution Map shows **N phases**
- [ ] Subagent Coordination documents **N phases**
- [ ] Reference files directory has **N phase files** (or fewer if phases share files)

**Self-Check Command:**
```bash
# Count phases in workflow overview
grep -c "^### Phase" .claude/skills/devforgeai-development/SKILL.md

# Count TodoWrite tracker items
grep -c '"Execute Phase' .claude/skills/devforgeai-development/SKILL.md

# Should match!
```

### Mandatory Marker Consistency

- [ ] All MANDATORY phases have ✓ MANDATORY marker in overview
- [ ] All MANDATORY phases in execution map have ✓ MANDATORY marker
- [ ] All MANDATORY steps in reference files have [MANDATORY] marker

### Reference File Consistency

- [ ] All referenced files exist: `ls .claude/skills/{skill}/references/{phase}.md`
- [ ] All reference files have corresponding phase in overview
- [ ] No orphaned reference files (files not mentioned in any phase)

### Documentation Consistency

- [ ] Command documentation reflects workflow (if user-facing)
- [ ] Memory reference files updated (if major change)
- [ ] Protocol documents updated (if pattern change)

---

## Example: Applying Checklist to Phase 4.5-5 Bridge (RCA-010)

**Phase Being Added:** Phase 4.5-5 Bridge (DoD Update Workflow)
**Context:** RCA-009 created the phase, now RCA-010 enforces complete integration

### Touchpoint Audit (Before RCA-010)

| Touchpoint | Status | Evidence |
|------------|--------|----------|
| 1. SKILL.md Workflow Overview | ✅ DONE | Line 158-162 |
| 2. TodoWrite Tracker | ❌ MISSING | Lines 68-69 (no bridge item) |
| 3. Complete Workflow Map | ✅ DONE | Lines 208-213 |
| 4. Subagent Coordination | ✅ DONE | Lines 349-354 |
| 5. Reference File | ✅ DONE | dod-update-workflow.md exists |
| 6. Command Documentation | ⚠️ PARTIAL | Mentioned but not detailed |
| 7. Memory References | ❌ MISSING | Not in skills-reference.md |
| 8. Protocol Documentation | ❌ MISSING | Not in lean-orchestration-pattern.md |
| 9. Testing | ❌ MISSING | No unit tests for bridge |

**Completeness: 3.5/9 touchpoints (39%)**

### RCA-010 Implementation (Fixing Gaps)

| Touchpoint | Action | REC # |
|------------|--------|-------|
| 1. SKILL.md Workflow Overview | ✅ Add MANDATORY marker | REC-3 |
| 2. TodoWrite Tracker | ✅ Add Phase 4.5-5 Bridge item | REC-1 |
| 3. Complete Workflow Map | ✅ Already complete | N/A |
| 4. Subagent Coordination | ✅ Already complete | N/A |
| 5. Reference File | ✅ Already exists | N/A |
| 6. Command Documentation | ⚠️ Skip (internal detail) | N/A |
| 7. Memory References | ⚠️ Skip (minor addition) | N/A |
| 8. Protocol Documentation | ✅ Add to this checklist doc | REC-2 |
| 9. Testing | ✅ Add unit tests | REC-4 |

**After RCA-010: 7/9 touchpoints complete (78%)** - sufficient for internal phase

---

## Common Integration Patterns

### Pattern 1: Adding Validation Phase (Like Phase 4.5-5 Bridge)

**Characteristics:**
- Validates data format or state
- No subagents (direct file operations)
- MANDATORY execution
- Short execution time (<1 minute)

**Required Touchpoints:** 1, 2, 3, 4, 5, 9 (6 of 9)
**Optional Touchpoints:** 6, 7, 8 (if major impact)

**Example:** Phase 4.5-5 Bridge (DoD Update)

---

### Pattern 2: Adding Subagent-Powered Phase (Like Phase 2)

**Characteristics:**
- Invokes one or more subagents
- Complex logic in reference file
- MANDATORY execution
- Medium execution time (5-15 minutes)

**Required Touchpoints:** ALL 9 touchpoints
**Why:** Subagent involvement = major workflow change, needs full documentation

**Example:** Phase 2 (Implementation with backend-architect + context-validator)

---

### Pattern 3: Adding Optional Step to Existing Phase

**Characteristics:**
- Enhancement within existing phase
- Conditional execution (not always runs)
- Not a separate phase (just a new step)

**Required Touchpoints:** 4, 5 (reference file + subagent coordination)
**Optional Touchpoints:** 1, 2, 3 (update phase description, not new phase)

**Example:** Adding "Check test coverage" as Step 4 in Phase 3

---

### Pattern 4: Splitting Existing Phase

**Characteristics:**
- Break large phase into two smaller phases
- Change phase numbering (e.g., old Phase 5 becomes Phase 6)
- Affects all downstream phases

**Required Touchpoints:** ALL 9 touchpoints + update all downstream phase numbers
**Complexity:** HIGH (many file updates)

**Example:** Split Phase 2 → Phase 2a (Implementation) + Phase 2b (Validation)

**Additional steps:**
- [ ] Update all phase numbers after split point
- [ ] Update all cross-references to renamed phases
- [ ] Update all commit messages/documentation mentioning phases

---

## Integration Checklist Template

**Copy this checklist for each workflow change:**

```markdown
## Integration Checklist for [Phase Name]

**Phase Number:** {N}
**Skill:** {skill-name}
**Change Type:** Add | Modify | Split | Merge | Delete
**RCA:** {RCA-NNN} (if applicable)

### Pre-Integration

- [ ] Backup all files to be modified
- [ ] Document current phase count: ____ phases
- [ ] Identify integration pattern (validation/subagent/optional/split)

### Touchpoint Updates

- [ ] 1. SKILL.md Workflow Overview updated
- [ ] 2. TodoWrite Tracker updated
- [ ] 3. Complete Workflow Execution Map updated
- [ ] 4. Subagent Coordination updated
- [ ] 5. Reference File created/updated
- [ ] 6. Command Documentation updated (if user-facing)
- [ ] 7. Memory Reference Files updated (if major change)
- [ ] 8. Protocol Documentation updated (if pattern change)
- [ ] 9. Testing added (unit + integration)

### Post-Integration Validation

- [ ] Phase count consistent across all locations
- [ ] MANDATORY markers consistent
- [ ] Reference files all exist
- [ ] Tests pass (unit + integration)
- [ ] No orphaned documentation

### Final Steps

- [ ] Create implementation commit
- [ ] Update RCA status (if applicable)
- [ ] Document in change log
```

---

## Self-Check Validation

**Before marking integration complete, run these checks:**

### Check 1: Count Phases Everywhere

```bash
# SKILL.md Workflow Overview
overview_count=$(grep -c "^### Phase" .claude/skills/devforgeai-development/SKILL.md)

# TodoWrite Tracker
tracker_count=$(grep -c '"Execute Phase' .claude/skills/devforgeai-development/SKILL.md)

# Complete Workflow Map
map_count=$(grep -c '^Phase [0-9]' .claude/skills/devforgeai-development/SKILL.md)

echo "Overview: $overview_count, Tracker: $tracker_count, Map: $map_count"

# All three should be equal!
if [ "$overview_count" = "$tracker_count" ] && [ "$tracker_count" = "$map_count" ]; then
  echo "✅ Phase counts consistent"
else
  echo "❌ MISMATCH DETECTED - Integration incomplete!"
fi
```

### Check 2: Verify Reference Files Exist

```bash
# Extract reference file names from SKILL.md
grep '**Reference:**' .claude/skills/devforgeai-development/SKILL.md | \
  sed 's/.*`\(.*\)`.*/\1/' | \
  while read ref; do
    if [ -f ".claude/skills/devforgeai-development/references/$ref" ]; then
      echo "✅ $ref exists"
    else
      echo "❌ $ref MISSING"
    fi
  done
```

### Check 3: Verify TodoWrite Sequence

```python
# Python script to validate phase sequence
import re

with open('.claude/skills/devforgeai-development/SKILL.md') as f:
    content = f.read()

# Extract phase numbers from TodoWrite
tracker = re.search(r'TodoWrite\((.*?)\)', content, re.DOTALL)
phases = re.findall(r'Execute Phase ([\d.]+(?:-[\d.]+)?)', tracker.group(1))

# Convert to sortable
def sort_key(p):
    return [float(x) for x in p.replace('-', '.').split('.')]

sorted_phases = sorted(phases, key=sort_key)

if phases == sorted_phases:
    print("✅ Phases in correct order:", phases)
else:
    print("❌ Phase order incorrect!")
    print("  Expected:", sorted_phases)
    print("  Actual:", phases)
```

---

## Troubleshooting Integration Issues

### Issue: "Phase count doesn't match"

**Symptoms:**
- Overview shows N phases
- TodoWrite tracker has M items (N ≠ M)

**Diagnosis:**
```bash
# Find the discrepancy
grep "^### Phase" .claude/skills/devforgeai-development/SKILL.md | nl
grep '"Execute Phase' .claude/skills/devforgeai-development/SKILL.md | nl
# Compare line-by-line
```

**Fix:**
- Add missing phase to TodoWrite tracker (Touchpoint 2)
- OR remove extra phase from overview (if not needed)
- Re-run Self-Check 1

---

### Issue: "Reference file mentioned but doesn't exist"

**Symptoms:**
- SKILL.md says: `**Reference:** phase-new-feature.md`
- File doesn't exist in references/

**Diagnosis:**
```bash
ls .claude/skills/devforgeai-development/references/phase-new-feature.md
# File not found
```

**Fix:**
- Create missing reference file (Touchpoint 5)
- OR remove reference mention if file not needed
- Re-run Self-Check 2

---

### Issue: "Tests fail after workflow change"

**Symptoms:**
- Integration tests expect 8 phases
- Workflow now has 9 phases
- Tests fail with "expected 8, got 9"

**Diagnosis:**
- Tests hardcoded old phase count

**Fix:**
- Update test expectations to match new phase count
- Make tests dynamic (query SKILL.md for phase count)
- Add test for new phase (Touchpoint 9)

---

## Integration Effort Estimates

**By Pattern:**

| Pattern | Touchpoints | Effort | Example |
|---------|-------------|--------|---------|
| Validation Phase | 6 of 9 | 30-60 min | Phase 4.5-5 Bridge |
| Subagent Phase | 9 of 9 | 2-4 hours | Phase 2 Implementation |
| Optional Step | 2 of 9 | 15-30 min | Step 4 in existing phase |
| Phase Split | 9 of 9 + renumber | 4-6 hours | Split Phase 2 → 2a + 2b |

**Buffer:** Add 25% buffer for testing and validation

---

## Examples from DevForgeAI History

### Example 1: Phase 4.5 Addition (RCA-006)

**What:** Added Deferral Challenge Checkpoint
**Touchpoints Updated:** 7/9 (missing tests, partial memory updates)
**Outcome:** Worked but had edge cases (RCA-009 found gaps)
**Lesson:** 7/9 sufficient for MVP, 9/9 needed for production quality

---

### Example 2: Phase 4.5-5 Bridge Addition (RCA-009)

**What:** Added DoD Update Workflow
**Touchpoints Updated:** 3/9 (SKILL.md overview, subagent coordination, reference file)
**Outcome:** Documented but not enforced (RCA-010 found gap)
**Lesson:** TodoWrite tracker (Touchpoint 2) is CRITICAL - without it, phase can be skipped

---

### Example 3: Light QA Addition to Phase 3 (RCA-009 Rec 3)

**What:** Promoted Light QA to explicit Step 5 in refactor phase
**Touchpoints Updated:** 5/9 (reference file, SKILL.md summary, subagent coordination, workflow map, validation)
**Outcome:** Working well, no gaps reported
**Lesson:** 5/9 sufficient when phase already existed (just making explicit)

---

## Quick Reference: Minimum Required Touchpoints

**For ANY workflow change:**
- ✅ MUST update: Touchpoint 1 (SKILL.md overview)
- ✅ MUST update: Touchpoint 2 (TodoWrite tracker) ← CRITICAL FOR ENFORCEMENT
- ✅ MUST update: Touchpoint 3 (Workflow map)

**For new MANDATORY phase:**
- ✅ Also update: Touchpoint 4 (Subagent coordination)
- ✅ Also update: Touchpoint 5 (Reference file)
- ✅ Also update: Touchpoint 9 (Testing)

**For major workflow changes:**
- ✅ Update ALL 9 touchpoints (comprehensive integration)

**Rule of thumb:** If unsure, update more touchpoints rather than fewer. Documentation gaps are harder to find later.

---

## Success Criteria

Integration is complete when:

- [ ] All applicable touchpoints updated (minimum 3/9, ideally 6/9 or 9/9)
- [ ] Self-check validation passes (phase counts match)
- [ ] Reference files all exist
- [ ] Tests pass (if tests updated)
- [ ] No orphaned documentation
- [ ] Git commit includes all changes atomically
- [ ] RCA (if applicable) references this checklist in implementation

---

## References

**Created From:**
- RCA-006: Autonomous Deferrals (Phase 4.5 integration lessons)
- RCA-009: Incomplete Skill Workflow Execution (Phase 4.5-5 Bridge integration lessons)
- RCA-010: DoD Checkboxes Not Validated (TodoWrite tracker gap identified)

**Related Protocols:**
- `devforgeai/protocols/lean-orchestration-pattern.md` - Command refactoring checklist
- `.claude/memory/skills-reference.md` - Skill documentation patterns

**Usage:**
- Reference this checklist in all future RCA implementations
- Apply when adding/modifying workflow phases
- Use as code review checklist for skill changes

---

**Version History:**
- v1.0 (2025-11-18): Initial creation (RCA-010 REC-2)
  - 8 core touchpoints defined
  - 4 integration patterns documented
  - 3 examples from RCA history
  - Self-check validation procedures
  - Troubleshooting guide
