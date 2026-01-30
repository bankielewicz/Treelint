# Deferral Decision Tree

Quick reference for determining if a deferral is valid. Used by QA skill Step 2.5 and deferral-validator subagent.

**Purpose:** Provide clear, unambiguous criteria for deferral validation

**Created:** RCA-007 Recommendation 7

---

## Step 1: Is there a deferral reason?

**Check:** DoD item marked `[ ]` has text after hyphen explaining why incomplete

**Example Formats:**
```markdown
- [ ] Item description - Deferred to STORY-XXX: {reason}
- [ ] Item description - Blocked by {external}: {ETA}
- [ ] Item description - Out of scope: ADR-XXX
```

### NO reason provided → ❌ **INVALID**

**Violation:**
- **Severity:** HIGH
- **Type:** "Missing deferral justification"
- **Message:** "Deferred DoD item has no explanation"
- **Remediation:** "Add reason in format: 'Deferred to STORY-XXX: {reason}' OR 'Blocked by {external}: {ETA}' OR 'Out of scope: ADR-XXX'"

### YES reason provided → Continue to Step 2

---

## Step 2: What type of deferral is it?

### Type A: External Blocker

**Pattern:** `"Blocked by {external_system}: {specific_reason with ETA}"`

**Valid Examples:**
- ✅ "Blocked by Payment API v2 (available 2025-12-01)"
- ✅ "Blocked by tree-sitter 0.21 release (planned Q2 2025)"
- ✅ "Blocked by third-party OAuth service (ETA: January 2026)"
- ✅ "Blocked by DBA approval process (ETA: 2 weeks)"

**Invalid Examples:**
- ❌ "Blocked by our authentication module" (internal, not external)
- ❌ "Blocked by Payment API" (no ETA)
- ❌ "Blocked" (no system specified)

**Validation Checklist:**
- [ ] Blocker is EXTERNAL (not "our code", "our API", "our module", "internal")
- [ ] ETA or resolution date provided (concrete: date, sprint, "when X available")
- [ ] Specific external system named (not vague)

**Valid with all checks?** → ✅ **VALID (No ADR required)**

**Invalid?** → ❌ **INVALID**
- **Severity:** HIGH
- **Message:** "Invalid external blocker (missing ETA or blocker is internal)"
- **Remediation:** "Provide concrete ETA OR confirm blocker is external OR complete work now"

**ADR Required?** NO (external blockers don't need ADRs)

---

### Type B: Story Split (Technical Dependency)

**Pattern:** `"Deferred to STORY-XXX: {justification}"`

**Valid Examples:**
- ✅ "Deferred to STORY-125: Requires parser integration from STORY-124"
- ✅ "Deferred to STORY-006: Will be implemented in main.rs integration story"
- ✅ "Deferred to STORY-089: Performance optimization epic handles this"

**Invalid Examples:**
- ❌ "Deferred to STORY-125" (no justification)
- ❌ "Deferred to STORY-999" (story doesn't exist)
- ❌ "Deferred to STORY-XXX" where STORY-XXX also defers it (chain)

**Validation Checklist:**
- [ ] Referenced story (STORY-XXX) EXISTS (Glob finds file)
- [ ] Referenced story INCLUDES deferred work (keywords in AC or spec)
- [ ] **NO deferral chain** (STORY-XXX doesn't also defer this work) ⭐ RCA-007 CRITICAL
- [ ] Work within original scope (if descoped, ADR required - see Type C)

**Chain Detection (CRITICAL):**

```
Read STORY-XXX Implementation Notes > DoD Status

IF STORY-XXX has incomplete items:
    FOR each item:
        IF item reason contains "Deferred to STORY-YYY":
            IF work description matches current deferred item:
                ❌ MULTI-LEVEL CHAIN DETECTED
                Chain: current_story → STORY-XXX → STORY-YYY
                Severity: CRITICAL
                Evidence: RCA-007 - STORY-004 → STORY-005 → STORY-006 (exit codes lost)
```

**Valid with all checks?** → ✅ **VALID (No ADR if within scope)**

**Valid but chain detected?** → ❌ **INVALID (CRITICAL)**
- **Severity:** CRITICAL
- **Type:** "Multi-level deferral chain detected"
- **Message:** "Work deferred multiple times: {chain}"
- **Rationale:** "Deferral chains >1 level are PROHIBITED (increases risk of lost work)"
- **Remediation:** "STORY-XXX must implement work OR create ADR justifying 3+ story span OR complete in current story"

**Valid but missing from target?** → ❌ **INVALID (HIGH)**
- **Severity:** HIGH
- **Type:** "Referenced story doesn't include deferred work"
- **Message:** "STORY-XXX acceptance criteria doesn't mention '{item}'"
- **Remediation:** "Add work to STORY-XXX OR complete in current story"

**ADR Required?**
- **NO** if work within original scope (just splitting for size)
- **YES** if work was in original spec but being descoped (scope change)

---

### Type C: Scope Change (Descoping Original Work)

**Pattern:** `"Out of scope: {reason}"` OR `"Deferred to STORY-XXX"` where work was in original spec

**How to Detect Scope Change:**
1. Read story acceptance criteria (original requirements)
2. Check if deferred item appears in original AC or technical spec
3. If YES → work was in scope, deferring it = scope change

**Examples:**
- ✅ "Out of scope: ADR-007 documents exit code removal from STORY-004"
- ✅ "Deferred to STORY-XXX: ADR-008 explains multi-story approach"
- ❌ "Out of scope" (no ADR reference)
- ❌ "Deferred to STORY-XXX" where item was in original spec (needs ADR)

**Validation Checklist:**
- [ ] ADR exists justifying scope change ← MANDATORY
- [ ] ADR references this specific story
- [ ] ADR documents this specific deferred item
- [ ] Product owner approved descope (documented in ADR)

**Valid with ADR?** → ✅ **VALID (ADR required and exists)**

**Invalid without ADR?** → ❌ **INVALID**
- **Severity:** MEDIUM
- **Type:** "Scope change without ADR"
- **Message:** "DoD item was in original scope, deferring requires ADR documentation"
- **Remediation:** "Create ADR-XXX documenting why '{item}' descoped from {story_id}"

**ADR Required?** YES (always for scope changes)

**Reference:** See devforgeai-architecture skill references (`adr-policy.md`) for complete ADR policy and deferral requirements

---

### Type D: Version Deferral

**Pattern:** `"Deferred to v2.0"` OR `"Deferred to vX.Y"`

**Examples:**
- ✅ "Deferred to v2.0: Performance benchmarks (tech-stack.md Out of Scope section)"
- ❌ "Deferred to v2.0" (no tech-stack.md reference)

**Validation Checklist:**
- [ ] References tech-stack.md "Out of Scope" section
- [ ] Version number specified (v1.1, v2.0, etc.)

**Valid?** → ✅ **VALID (No ADR if documented in tech-stack.md)**

**Invalid?** → ❌ **INVALID**
- **Severity:** MEDIUM
- **Remediation:** "Add to tech-stack.md Out of Scope section OR change to story reference"

---

## Step 3: Run Automated Validation

**ALWAYS invoke deferral-validator subagent:**

```
Task(
    subagent_type="deferral-validator",
    description="Validate deferrals",
    prompt="Validate all deferred DoD items. Detect:
            - Circular chains (A→B→A)
            - Multi-level chains (A→B→C) ← RCA-007
            - Invalid story references
            - Missing ADRs for scope changes
            - External blocker verification
            Return JSON validation report."
)
```

**NO manual override allowed**

**Rationale (RCA-007 Evidence):**
- Manual validation missed STORY-004 → STORY-005 → STORY-006 chain
- Manual validation assumed "reason exists" = "reason valid"
- Manual validation didn't check if STORY-006 included work
- Manual validation didn't require ADR for scope change

**Automated validation prevents:**
- Human error in chain detection
- Inconsistent validation standards
- Shortcuts that bypass quality gates
- Technical debt accumulation

---

## Violation Severity Guide

### CRITICAL Violations (Block QA Approval Immediately)

- **Circular deferral chains:** A→B→A (work loops back, never completed)
- **Multi-level deferral chains:** A→B→C (work deferred >1 time, risk of loss)
- **Work lost in chain:** Referenced story doesn't exist or doesn't include work

**Impact:** Technical debt untracked, work lost, quality gate bypassed

**Action:** HALT QA approval, fix violations before proceeding

### HIGH Violations (Block QA Approval)

- **Invalid story reference:** STORY-XXX doesn't exist
- **Referenced story missing work:** STORY-XXX exists but doesn't include deferred item
- **Unjustified deferral:** Vague reason ("will add later", "not enough time")
- **Unnecessary deferral:** Work feasible now (pattern in spec, <50 lines, no blockers)

**Impact:** Deferred work likely to be forgotten, poor scope management

**Action:** Fix violations, create proper justifications, or complete work

### MEDIUM Violations (Document but Don't Block)

- **Scope change without ADR:** DoD item was in scope, deferring requires ADR
- **External blocker missing ETA:** Blocker documented but no resolution date
- **Deferral reason format:** Doesn't match required patterns

**Impact:** Documentation gaps, audit trail incomplete

**Action:** Create ADRs, add ETAs, improve deferral reason clarity

### LOW Violations (Advisory)

- **Style issues:** Formatting inconsistencies
- **Minor clarity issues:** Deferral reason could be clearer

**Impact:** Minimal

**Action:** Improve for consistency (optional)

---

## Quick Reference Table

| Deferral Type | Valid Pattern | ADR Required? | Chain Allowed? | Max Hops |
|---------------|---------------|---------------|----------------|----------|
| **External Blocker** | "Blocked by {external}: {ETA}" | **No** | N/A | N/A |
| **Story Split (in scope)** | "Deferred to STORY-XXX: {reason}" | **No** | **NO** | 1 (A→B only) |
| **Story Split (multi-hop)** | A→B→C | **YES** | **NO** | 1 without ADR |
| **Scope Change** | "Out of scope: ADR-XXX" | **YES** | N/A | N/A |
| **Architectural Impact** | APIs, data structures | **YES** | N/A | N/A |
| **Version Deferral** | "Deferred to v2.0 (tech-stack.md)" | **No** | N/A | N/A |

---

## Usage

**By QA Skill:**
- Load this file during Step 2.5 (deferral validation)
- Reference decision tree to understand validation logic
- Use to explain violations to user

**By deferral-validator Subagent:**
- Use decision tree logic in validation workflow (Substeps 1-6)
- Apply severity guidelines consistently
- Generate remediation based on violation type

**By Developers:**
- Reference when writing deferral justifications
- Understand what makes a deferral valid
- Know when ADR is required

**By Product Owners:**
- Understand deferral policy
- Review ADRs for scope changes
- Approve/reject deferrals based on business impact

---

## Examples

### Example 1: Valid External Blocker (No ADR)

**Deferral:**
```markdown
- [ ] OAuth integration - Blocked by Google OAuth API approval (ETA: 2025-11-15)
```

**Validation:**
- Blocker: Google OAuth API approval (EXTERNAL ✅)
- ETA: 2025-11-15 (CONCRETE ✅)
- Result: ✅ VALID (no ADR needed)

---

### Example 2: Invalid Multi-Level Chain (ADR Required)

**STORY-004:**
```markdown
- [ ] Exit code handling - Deferred to STORY-005: Error framework story
```

**STORY-005:**
```markdown
- [ ] main.rs error integration - Deferred to STORY-006: main.rs integration
```

**STORY-006:**
- Exit codes NOT mentioned in acceptance criteria

**Validation:**
- Chain detected: STORY-004 → STORY-005 → STORY-006 (MULTI-LEVEL ❌)
- Work lost: Not in STORY-006 scope (CRITICAL ❌)
- ADR missing: No justification for 3-story span (MEDIUM ❌)
- Result: ❌ INVALID (CRITICAL violation)

**Remediation Options:**
1. Implement in STORY-006 (update STORY-006 scope)
2. Create ADR-XXX justifying why work spans 3 stories
3. Create new STORY-007 explicitly for exit code handling

---

### Example 3: Valid Story Split (No Chain)

**STORY-010:**
```markdown
- [ ] Advanced search filters - Deferred to STORY-015: Search optimization epic
```

**STORY-015:**
```markdown
## Acceptance Criteria
- Given user wants advanced filtering
- When user applies multiple filters
- Then results are filtered correctly
```

**Validation:**
- Story exists: STORY-015 found (EXISTS ✅)
- Includes work: "advanced filtering" in AC (INCLUDES ✅)
- Chain check: STORY-015 has no incomplete items deferring this work (NO CHAIN ✅)
- Scope: Filtering was in STORY-010 original spec (IN SCOPE ✅, but story split for size)
- Result: ✅ VALID (no ADR needed - valid story split)

---

### Example 4: Scope Change Requiring ADR

**STORY-020:**
```markdown
- [ ] Real-time notifications - Out of scope: ADR-015 descoped for v1.0
```

**ADR-015:**
```markdown
# ADR-015: Descope Real-Time Notifications from v1.0

## Context
STORY-020 originally included real-time notifications via WebSockets.

## Decision
Defer to v2.0 - implement polling-based approach in v1.0.

## Rationale
- WebSocket infrastructure not available in v1.0
- Polling sufficient for MVP
- Reduces complexity

## Consequences
- Users see updates with 30s delay (polling interval)
- v2.0 will migrate to WebSockets
```

**Validation:**
- Pattern: "Out of scope: ADR-015" (SCOPE CHANGE ✅)
- ADR exists: Glob finds ADR-015 (EXISTS ✅)
- ADR documents item: "real-time notifications" mentioned (DOCUMENTED ✅)
- Result: ✅ VALID (ADR exists and documents scope change)

---

## Step 3: Run Automated Validation (MANDATORY)

**ALWAYS invoke deferral-validator subagent:**

No manual override. No shortcuts. No exceptions.

**Why?**
- **RCA-007:** Manual validation missed multi-level chain
- **Evidence:** STORY-004 → STORY-005 → STORY-006 approved despite broken chain
- **Lesson:** Automated validation catches what manual review misses

**What Subagent Detects:**
1. Format validation (reason matches required patterns)
2. Technical blocker verification (external vs internal)
3. Implementation feasibility (could be done now?)
4. ADR requirement (scope change detection)
5. Circular deferral chains (A→B→A)
6. **Multi-level deferral chains (A→B→C)** ← RCA-007
7. Story reference validation (exists, includes work)

---

## Violation Severity Guide

### CRITICAL (Block QA Immediately)

**Violations:**
- Circular deferral chains (A→B→A)
- **Multi-level deferral chains (A→B→C)** ← RCA-007
- Work lost in broken chain

**Why CRITICAL:**
- Work guaranteed to be lost or forgotten
- Creates infinite loops or broken chains
- No owner for deferred work

**Examples:**
- STORY-004 → STORY-005 → STORY-004 (circular)
- STORY-004 → STORY-005 → STORY-006 where STORY-006 doesn't include work (multi-level, lost)

**Action:** HALT QA approval, cannot proceed

---

### HIGH (Block QA Until Fixed)

**Violations:**
- Invalid story reference (STORY-XXX doesn't exist)
- Referenced story missing work (STORY-XXX exists but incomplete scope)
- Unjustified deferral (vague: "will add later", "not enough time")
- Unnecessary deferral (feasible now: pattern in spec, <50 lines, no blockers)
- Internal blocker claimed as external ("our API" isn't external)

**Why HIGH:**
- Deferred work likely to be forgotten
- Poor scope management
- Technical justification missing

**Action:** Fix violations before QA approval

---

### MEDIUM (Document but Can Approve)

**Violations:**
- Scope change without ADR (item was in spec, deferring without documentation)
- External blocker missing ETA (blocker exists but no timeline)
- Deferral reason format (doesn't match patterns clearly)

**Why MEDIUM:**
- Documentation gaps
- Audit trail incomplete
- Can be resolved post-approval if needed

**Action:** Create ADRs, add ETAs, document properly (can proceed with documentation plan)

---

## Process Flow Diagram

```
DoD Item Incomplete?
    ↓ YES
Has deferral reason?
    ↓ YES
Match Type A (External Blocker)?
    ↓ NO
Match Type B (Story Split)?
    ↓ YES
Referenced story exists?
    ↓ YES
Referenced story includes work?
    ↓ YES
Check for deferral chain...
    ↓
Referenced story ALSO defers this work?
    ↓ YES
    → ❌ CRITICAL: Multi-level chain detected (A→B→C prohibited)
    ↓ NO
    → ✅ VALID: Single-hop deferral (A→B allowed)
```

---

## Integration

**Referenced By:**
- QA skill (Phase 3, Step 2.5) - Loads for deferral validation context
- deferral-validator subagent - Uses decision tree logic in substeps
- `/audit-deferrals` command - Applies same criteria to all stories
- Developers - Reference when creating deferrals

**Maintained By:**
- Updated when deferral policy changes
- Reviewed during RCA analysis
- Versioned in git with framework

---

## Deferral Prevention (Developer Workflow)

**Purpose:** Educate developers and AI agents on proper deferral protocol to prevent violations BEFORE they reach QA.

**Added:** RCA-006 Recommendation 5 (Prevention Guide)

---

### During Development (/dev Command)

**MANDATORY Protocol - Before ANY DoD item is marked [ ] (deferred):**

<prevention_protocol>
  <step_1>Attempt implementation FIRST - default is to complete, not defer</step_1>
  <step_2>If blocked, use AskUserQuestion to determine reason</step_2>
  <step_3>Document user-approved justification - NEVER defer without user input</step_3>
  <step_4>Automated validation - Script and subagent verify all [ ] items have proper format</step_4>
</prevention_protocol>

#### Step 1: Attempt Implementation First

**Default assumption:** All DoD items CAN be completed

**Before deferring, try:**
- Research solution approaches (5-10 minutes investigation)
- Check if dependencies are actually available
- Estimate actual effort (may be smaller than assumed)
- Ask user: "This seems hard, but should I try implementing it?"

**Only defer if:**
- ✅ External blocker confirmed (dependency not available)
- ✅ Scope change approved (user agrees to remove from scope)
- ✅ User explicitly requests deferral

❌ **Never defer because:**
- "Seems complicated" (investigate first)
- "Don't know how" (research or ask user)
- "Might take time" (estimate first, then ask user)

---

#### Step 2: Use AskUserQuestion for Blocked Work

**If implementation blocked, determine reason via user interaction:**

```
AskUserQuestion:
    Question: "Cannot complete '{item}'. What's blocking this work?"
    Header: "Blocker Analysis"
    Options:
        - "Try different approach (AI suggests alternatives)"
        - "External blocker (describe dependency with ETA)"
        - "Out of scope (requires architecture decision/ADR)"
        - "Actually feasible (AI will implement now)"
    multiSelect: false
```

**Based on user selection:**
- **Try different approach:** AI researches alternatives, implements if feasible
- **External blocker:** Document with ETA, proceed with deferral
- **Out of scope:** Create ADR, document scope change
- **Actually feasible:** Return to implementation (no deferral)

**NEVER make autonomous decision** about why work is blocked

---

#### Step 3: Document User-Approved Justification

**After user interaction, format deferral correctly:**

**Valid Formats:**
```markdown
- [ ] Item description
    Deferred to STORY-XXX: [specific reason user provided]

- [ ] Item description
    Blocked by: [external system] (ETA: [date/condition])

- [ ] Item description
    Out of scope: ADR-XXX documents scope change decision
```

**Include:**
- ✅ Specific story reference (STORY-XXX, not "future story")
- ✅ Brief reason (why deferred, not just "deferred")
- ✅ ETA for external blockers (when unblocked)
- ✅ ADR reference for scope changes

---

#### Step 4: Automated Validation Layers

**Three validation layers execute automatically:**

**Layer 1: Format Check** (.claude/scripts/validate_deferrals.py)
- Speed: <100ms
- Checks: Basic justification format present
- Result: Warnings for format issues (non-blocking)

**Layer 2: Interactive Checkpoint** (.claude/skills/devforgeai-development/references/dod-validation-checkpoint.md)
- Requires: AskUserQuestion for EVERY unjustified item
- Creates: Follow-up stories, ADRs as user directs
- Result: BLOCKS commit until user approves all deferrals

**Layer 3: AI Validation** (deferral-validator subagent)
- Validates: Feasibility, circular chains, references exist
- Checks: Could work be done now? Are references valid?
- Result: BLOCKS commit on CRITICAL/HIGH violations

**All layers must pass before git commit allowed**

---

### Red Flags (QA Will Fail)

**Invalid Deferral Reasons:**

❌ **Vague or Generic**
- "Will do this later" (no story reference)
- "Not important right now" (scope change without ADR)
- "Too complex" (no blocker or reason)
- "Deferred" (no details at all)
- "Future work" (no story reference)

❌ **Missing Story Reference**
- "Deferred to performance optimization story" (no STORY-ID)
- "Moving to follow-up" (which follow-up?)
- "Deferred for later implementation" (when? where?)

❌ **Missing ADR for Scope Change**
- "Removing from scope" (no ADR documenting decision)
- "Not needed anymore" (scope change without architecture review)
- "Out of scope" (no ADR reference)

❌ **Internal Blockers Treated as External**
- "Blocked by our authentication module" (internal - YOU can fix this)
- "Blocked by our API design" (internal decision - not external dependency)
- "Waiting for our team to decide" (internal - not external blocker)

❌ **No ETA for External Blockers**
- "Blocked by Payment API v2" (when is it available?)
- "Blocked by third-party service" (what's the ETA?)

❌ **Circular Deferrals**
- STORY-001 defers to STORY-002
- STORY-002 defers back to STORY-001
- **CRITICAL violation** - work will never complete

---

### Green Flags (Valid Deferrals)

✅ **Story Split with Reason**
```markdown
- [ ] Performance benchmarking
    Deferred to STORY-007: Criterion benchmark framework requires dedicated performance optimization story with baseline measurements
```

✅ **External Blocker with ETA**
```markdown
- [ ] OAuth integration
    Blocked by: Third-party OAuth API v2 (provider ETA: January 2026, confirmed via email)
```

✅ **Scope Change with ADR**
```markdown
- [ ] Advanced caching
    Out of scope: ADR-003 documents decision to defer caching to v2.0 (simplicity over performance in v1.0)
```

✅ **User-Approved via Checkpoint**
```markdown
- [ ] Multi-language support
    Deferred to STORY-012: User approved split during development checkpoint (Phase 2.5b, 2025-11-04)
```

---

### Best Practices

**1. Default to Completing Work**
- Try to implement before deferring
- Estimate effort before assuming it's too large
- Research solutions before claiming "blocked"

**2. Create Follow-Up Stories Immediately**
- Don't defer to "future story" - create STORY-XXX now
- Use requirements-analyst subagent to generate story
- Link stories with prerequisite_stories field

**3. Document External Blockers with ETAs**
- Specify exact dependency (API name, version, system)
- Include ETA or resolution condition
- Verify blocker is truly external (not internal decision)

**4. Create ADRs for Scope Changes**
- Use architect-reviewer subagent to generate ADR
- Document WHY scope changed (not just THAT it changed)
- Reference ADR in deferral justification

**5. Leverage Three-Layer Validation**
- Layer 1 catches format errors instantly (<100ms)
- Layer 2 ensures user approval (no autonomous deferrals)
- Layer 3 validates feasibility (AI prevents unnecessary deferrals)

---

### Tools and Integration

**Prevention Tools (RCA-006 Implementation):**

1. **Python Format Validator**
   - File: `.claude/scripts/validate_deferrals.py`
   - Usage: Invoked automatically in /dev Phase 2.5a
   - Purpose: Fast format feedback (<100ms)

2. **Interactive Checkpoint**
   - File: `.claude/skills/devforgeai-development/references/dod-validation-checkpoint.md`
   - Usage: Invoked automatically in /dev Phase 5 Step 1b (via progressive loading)
   - Purpose: User approval gate (ZERO autonomous deferrals)

3. **deferral-validator Subagent**
   - File: `.claude/agents/deferral-validator.md`
   - Usage: Invoked by skill Phase 5 Step 1.5
   - Purpose: Comprehensive AI analysis

4. **Story Size Detection** (NEW - RCA-006 Rec 4)
   - Location: devforgeai-development skill Phase 5 Step 1b
   - Trigger: >3 deferrals
   - Action: Suggests story splitting to reduce debt

**All tools work together to prevent deferral violations**

---

### Common Mistakes to Avoid

**Mistake 1: Deferring Without User Interaction**
```
❌ BAD: AI autonomously defers work
"This seems complex, deferring to future story"

✅ GOOD: AI asks user first
AskUserQuestion: "Item seems complex. Should I defer or try implementing?"
```

**Mistake 2: Vague Story References**
```
❌ BAD: "Deferred to performance story"
✅ GOOD: "Deferred to STORY-007: Performance optimization epic"
```

**Mistake 3: Internal Blockers as External**
```
❌ BAD: "Blocked by our API design decision"
✅ GOOD: "Out of scope: ADR-005 documents decision to simplify API in v1.0"
```

**Mistake 4: No Reason Provided**
```
❌ BAD:
- [ ] Cross-platform testing
    (no explanation)

✅ GOOD:
- [ ] Cross-platform testing
    Blocked by: CI/CD matrix not configured until STORY-001 (ETA: Week 1)
```

**Mistake 5: Deferral Chains**
```
❌ BAD:
STORY-001 → defers to STORY-002
STORY-002 → defers to STORY-003
STORY-003 → defers to STORY-004
(Work fragmented across 4 stories - will likely never complete)

✅ GOOD:
STORY-001 → AI detects >3 deferrals
          → Suggests split
          → User approves
          → Creates focused STORY-002 (grouped work)
(Work concentrated in 2 stories - clear ownership)
```

---

## Validation Checklist (For QA Skill)

Before approving story with deferrals:

- [ ] All deferrals have reasons
- [ ] All reasons match valid patterns (A, B, C, or D)
- [ ] External blockers have ETAs
- [ ] Story references exist and include work
- [ ] **No multi-level chains detected (A→B→C)** ← RCA-007
- [ ] No circular chains detected (A→B→A)
- [ ] Scope changes have ADR approval
- [ ] **deferral-validator subagent invoked** (MANDATORY)

**If ANY checklist item fails:** QA Status = FAILED

---

**Created:** 2025-11-03 (RCA-007 Recommendation 7)
**Purpose:** Clear, unambiguous deferral validation criteria
**Usage:** QA skill, deferral-validator subagent, developers
**Version:** 1.0
