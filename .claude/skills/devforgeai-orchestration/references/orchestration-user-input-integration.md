---
id: orchestration-user-input-integration
title: Orchestration Skill - User Input Guidance Integration
version: "1.0"
created: 2025-01-21
updated: 2025-01-21
status: Published
audience: DevForgeAI Development Team
parent_document: user-input-guidance.md
skill: devforgeai-orchestration
---

# Orchestration Skill - User Input Guidance Integration Reference

**Purpose:** Document how devforgeai-orchestration skill integrates user-input-guidance.md with conditional loading logic for epic creation and sprint planning modes.

**Context:** This skill coordinates the complete spec-driven development lifecycle. It operates in two main interactive modes: (1) Epic Creation (Phase 4A) and (2) Sprint Planning (Phase 3). Both modes benefit from guided user input patterns.

---

## Section 1: Mode Detection and Conditional Loading

### 1.1 Orchestration Modes

This skill operates in 5 modes (from SKILL.md), but guidance loading applies to 2:

**Mode 1: Epic Creation (Phase 4A) - LOAD GUIDANCE**
- **Detection:** Context marker `**Command:** create-epic` present
- **Action:** Load guidance in Phase 4A Step 2 (Context Gathering)
- **Questions:** Epic goal (open-ended), timeline (bounded), priority (classification), success criteria (min count)
- **Token Cost:** ~1,000 tokens

**Mode 2: Sprint Planning (Phase 3) - LOAD GUIDANCE**
- **Detection:** Context marker `**Command:** create-sprint` present
- **Action:** Load guidance in Phase 3 Step 1 (Sprint Planning)
- **Questions:** Epic selection (bounded + explicit none), story selection (multi-select with capacity)
- **Token Cost:** ~1,000 tokens

**Other Modes - SKIP GUIDANCE (not applicable):**
- Story Management: No user interaction needed (automated workflow)
- Audit Deferrals: No interactive guidance needed (audit mode)
- Checkpoint Detection: No guidance needed

### 1.2 Mode-Based Guidance Loading Logic

**Epic Creation Mode:**

```python
def phase_4a_epic_creation_context_gathering():
    """Phase 4A Step 2: Context Gathering with guidance"""

    # Check if epic creation mode
    if context_marker_exists("**Command:** create-epic"):
        mode = "epic_creation"
        skip_guidance = False
        log_info("Epic creation mode detected. Loading user-input-guidance.md for Phase 4A...")
        guidance_content = Read(file_path=".claude/skills/devforgeai-orchestration/references/user-input-guidance.md")

        # Step 0: Load guidance (epic patterns)
        # Patterns available for epic questions

        # Step 1: Ask epic goal (Open-Ended Discovery)
        # Step 2: Ask timeline (Bounded Choice - 4 options)
        # Step 3: Ask priority (Explicit Classification - 4 levels)
        # Step 4: Ask success criteria (Open-Ended with Min Count - 3+ required)
        # etc.

    else:
        mode = "other"
        # Not epic mode - skip guidance for this mode
```

**Sprint Planning Mode:**

```python
def phase_3_sprint_planning():
    """Phase 3 Step 1: Sprint Planning with guidance"""

    # Check if sprint planning mode
    if context_marker_exists("**Command:** create-sprint"):
        mode = "sprint_planning"
        skip_guidance = False
        log_info("Sprint planning mode detected. Loading user-input-guidance.md for Phase 3...")
        guidance_content = Read(file_path=".claude/skills/devforgeai-orchestration/references/user-input-guidance.md")

        # Step 0: Load guidance (sprint patterns)
        # Patterns available for sprint questions

        # Step 1: Ask epic selection (Bounded + Explicit None)
        # Step 2: Ask story selection (Multi-Select with Capacity Guidance)
        # etc.

    else:
        mode = "other"
        # Not sprint mode - skip guidance for this mode
```

---

## Section 2: Epic Mode Pattern Mapping

### 2.1 Phase 4A Questions-to-Patterns (Epic Mode)

| Phase | Step | Question | Pattern Name | Pattern # | Template | Options/Bounds | Rationale |
|-------|------|----------|--------------|-----------|----------|----------------|-----------|
| **4A** | **2** | "Epic goal?" | Open-Ended Discovery | 1 | FUN-001 | N/A (free text) | Unique per epic; unbounded |
| **4A** | **2** | "Timeline?" | Bounded Choice | 2 | CONST-002 | 4 options: 1 sprint / 2-3 sprints / 4-6 sprints / 6+ sprints | Standard durations |
| **4A** | **2** | "Priority level?" | Explicit Classification | 3 | CONST-002 | 4 levels: Critical / High / Medium / Low | Well-defined levels |
| **4A** | **2** | "Success criteria?" | Open-Ended with Min Count | (custom) | (custom) | Minimum 3 required | Multiple measurable outcomes |

### 2.2 Epic Mode Pattern Applications

**Pattern 1: Open-Ended Discovery (Epic Goal)**
- **Use for:** Epic goal/objective question
- **Application:** Ask "What is the primary goal of this epic?" with NO preset options
- **Reasoning:** Unique per epic; any business goal valid
- **User Input:** Free-text goal description
- **Result:** Document goal in epic file

**Pattern 2: Bounded Choice (Epic Timeline)**
- **Use for:** Estimated duration for epic
- **Application:** Present 4 standard sprint range options
- **Reasoning:** Standard sprint ranges for planning
- **User Input:** Select one timeline range
- **Result:** Document timeline estimate in epic

**Pattern 3: Explicit Classification (Epic Priority)**
- **Use for:** Business priority level
- **Application:** Present 4 priority levels with descriptions
- **Reasoning:** Standard priority levels; clear definitions prevent ambiguity
- **User Input:** Select one priority level
- **Result:** Document priority in epic (influences story selection)

**Pattern (Custom): Open-Ended with Minimum Count (Success Criteria)**
- **Use for:** Epic success criteria (requires ≥3)
- **Application:** Ask for success criteria with minimum count guidance
- **Reasoning:** Multiple measurable outcomes needed for validation
- **User Input:** Free-text criteria (≥3 items required)
- **Result:** Document success criteria in epic (tested in QA phase)

### 2.3 Epic Mode Examples

**Example: Epic Goal Question (Open-Ended)**

```
AskUserQuestion(
    question: "What is the primary goal of this epic?",
    header: "Epic Goal",
    description: "Define the business outcome this epic aims to achieve. Be specific about the problem being solved and the value delivered. Example: 'Enable users to manage their team members and assign tasks with real-time notifications'",
    multiSelect: false
)
```

**Example: Epic Timeline (Bounded Choice - 4 Options)**

```
AskUserQuestion(
    question: "What's the estimated timeline for this epic?",
    header: "Epic Timeline",
    description: "Estimate how many sprints this epic will take. Consider team size, complexity, and dependencies.",
    options: [
        {label: "1 sprint", description: "Small epic, can complete in single 2-week sprint (10-15 story points)"},
        {label: "2-3 sprints", description: "Medium epic, 4-6 weeks (20-40 story points)"},
        {label: "4-6 sprints", description: "Large epic, 2-3 months (50-100 story points)"},
        {label: "6+ sprints", description: "Major initiative, 3+ months (100+ story points)"}
    ],
    multiSelect: false
)
```

**Example: Epic Priority (Explicit Classification - 4 Levels)**

```
AskUserQuestion(
    question: "What's the business priority for this epic?",
    header: "Epic Priority",
    description: "Priority influences scheduling and resource allocation. Higher priority epics are scheduled earlier.",
    options: [
        {label: "Critical", description: "Blocks other work, revenue impact, user-blocking issue. Schedule immediately."},
        {label: "High", description: "Important business value, scheduled in next 1-2 sprints"},
        {label: "Medium", description: "Valuable feature, scheduled in next 3-4 sprints"},
        {label: "Low", description: "Nice-to-have, backlog item, no current scheduling pressure"}
    ],
    multiSelect: false
)
```

**Example: Success Criteria (Open-Ended with Min Count)**

```
AskUserQuestion(
    question: "What are the success criteria for this epic? (minimum 3 required)",
    header: "Success Criteria",
    description: "Define 3+ measurable outcomes that indicate this epic is complete. Examples: 'Users can invite team members', 'Real-time notifications working', 'Dashboard loads in <2s'",
    multiSelect: false
)

# User response must include ≥3 criteria
# Validate in Phase 4A Step 2: If <3, ask again
```

---

## Section 3: Sprint Mode Pattern Mapping

### 3.1 Phase 3 Questions-to-Patterns (Sprint Mode)

| Phase | Step | Question | Pattern Name | Pattern # | Template | Options/Bounds | Rationale |
|-------|------|----------|--------------|-----------|----------|----------------|-----------|
| **3** | **1** | "Select epic?" | Bounded + Explicit None | (custom) | CONST-001 | N+1 options (epics + "None") | Optional epic linkage |
| **3** | **1** | "Select stories?" | Bounded Multi-Select + Capacity | (custom) | CONST-002 | N items (all Backlog stories) | Multiple with capacity guidance |

### 3.2 Sprint Mode Pattern Applications

**Pattern (Custom): Bounded Choice + Explicit None (Epic Selection)**
- **Use for:** Optional epic linkage in sprint
- **Application:** List all epics + explicit "None - Standalone Sprint" option
- **Reasoning:** Sprint may or may not belong to epic
- **User Input:** Select 0 or 1 epic (single-select)
- **Result:** Link sprint to epic (or mark standalone)

**Pattern (Custom): Bounded Multi-Select with Capacity Guidance (Story Selection)**
- **Use for:** Selecting stories for sprint with capacity awareness
- **Application:** Multi-select from all Backlog stories, display running total, show capacity warnings
- **Reasoning:** Capacity planning (20-40 points recommended), but flexible
- **User Input:** Select multiple stories (multi-select), running total displayed
- **Guidance:** Warn if <20 or >40 points (recommendations, not enforcement)
- **Result:** Sprint created with selected stories

### 3.3 Sprint Mode Examples

**Example: Epic Selection (Bounded + Explicit None)**

```
# First, list all existing epics + explicit None option:
epics = [
    "EPIC-001: User Authentication",
    "EPIC-002: Dashboard Features",
    "EPIC-003: Reporting System"
]

AskUserQuestion(
    question: "Which epic does this sprint belong to? (optional)",
    header: "Sprint Epic Linkage",
    description: "Sprints can belong to an epic or be standalone. Select the epic this sprint contributes to, or select 'None' for independent work.",
    options: [
        {label: "EPIC-001: User Authentication", description: "Estimated remaining: 2-3 sprints"},
        {label: "EPIC-002: Dashboard Features", description: "Estimated remaining: 4-5 sprints"},
        {label: "EPIC-003: Reporting System", description: "Estimated remaining: 1-2 sprints"},
        {label: "None - Standalone Sprint", description: "This sprint is not part of any epic (maintenance, bugfixes, etc.)"}
    ],
    multiSelect: false
)
```

**Example: Story Selection (Multi-Select with Capacity Guidance)**

```
# Display all Backlog stories with capacity feedback:

AskUserQuestion(
    question: "Which stories will you include in this sprint?",
    header: "Sprint Story Selection",
    description: "Select stories to add to the sprint. Recommended capacity: 20-40 story points. As you select, running total displays with capacity warnings.",
    options: [
        {label: "STORY-051: User login form", description: "5 points | Frontend feature | Ready"},
        {label: "STORY-052: Database schema", description: "8 points | Infrastructure | Ready"},
        {label: "STORY-053: Email notifications", description: "13 points | Backend feature | Blocked (needs STORY-052)"},
        {label: "STORY-054: Analytics dashboard", description: "21 points | Full-stack | Ready"},
        {label: "STORY-055: Mobile responsive", description: "13 points | Frontend | Ready"},
        # ... more stories
    ],
    multiSelect: true
)

# DURING selection, show:
# "Selected: STORY-051 (5 pts), STORY-052 (8 pts) | Total: 13 pts"
# "Selected: STORY-051 (5 pts), STORY-052 (8 pts), STORY-054 (21 pts) | Total: 34 pts ✓ Optimal"
# "Selected: STORY-051 (5 pts), STORY-052 (8 pts), STORY-054 (21 pts), STORY-055 (13 pts) | Total: 47 pts ⚠️ Over capacity"
```

**Capacity Guidance Messages:**
- **<20 points:** "⚠️ Low capacity: X pts (recommended: 20-40 pts). Consider adding more stories."
- **20-40 points:** "✓ Optimal capacity: X pts. Good sprint load for typical team."
- **>40 points:** "⚠️ Over capacity: X pts (recommended: 20-40 pts). Consider reducing stories."
- **Enforcement:** Guidance is advice, not blocking. User can proceed with any total.

---

## Section 4: Reference Deployment

### 4.1 File Locations

**Master File:** `src/claude/skills/devforgeai-ideation/references/user-input-guidance.md`

**Orchestration Deployment:**
- Location: `src/claude/skills/devforgeai-orchestration/references/user-input-guidance.md`
- Deployment: Copy from master using:
  ```bash
  cp src/claude/skills/devforgeai-ideation/references/user-input-guidance.md \
     src/claude/skills/devforgeai-orchestration/references/user-input-guidance.md
  ```

**Operational Folder:**
- Also copied to: `.claude/skills/devforgeai-orchestration/references/user-input-guidance.md`

### 4.2 Checksum Validation

**Verify deployment integrity:**
```bash
# All 3 orchestration deployments should have identical SHA256
sha256sum src/claude/skills/devforgeai-orchestration/references/user-input-guidance.md \
          .claude/skills/devforgeai-orchestration/references/user-input-guidance.md

# If hashes match: ✅ Deployment successful
# If hashes differ: ❌ Files out of sync, redeploy
```

---

## Section 5: Testing Strategy

### 5.1 Epic Mode Unit Tests

**Test 1: Epic mode loads guidance**
```python
def test_epic_mode_loads_guidance():
    """Verify guidance loads for epic creation"""
    # Setup: Context marker "**Command:** create-epic"
    # Execute: Phase 4A Step 2
    # Assert: Read called with guidance file path
    # Assert: Log contains "Epic creation mode"
```

**Test 2: Epic goal uses Open-Ended Discovery**
```python
def test_epic_goal_open_ended():
    """Verify epic goal question has no preset options"""
    # Setup: Epic mode, guidance loaded
    # Execute: Goal question generation
    # Assert: Question is open-ended (no options)
    # Assert: Accepts free-text input
```

**Test 3: Epic timeline uses Bounded Choice (4 options)**
```python
def test_epic_timeline_4_options():
    """Verify timeline has exactly 4 sprint range options"""
    # Setup: Epic mode, guidance loaded
    # Execute: Timeline question
    # Assert: Exactly 4 options: 1 / 2-3 / 4-6 / 6+ sprints
```

**Test 4: Epic priority uses Classification (4 levels)**
```python
def test_epic_priority_4_levels():
    """Verify priority has 4 levels"""
    # Setup: Epic mode, guidance loaded
    # Execute: Priority question
    # Assert: Exactly 4 options: Critical / High / Medium / Low
```

**Test 5: Epic success criteria minimum 3 validation**
```python
def test_epic_success_criteria_min_3():
    """Verify success criteria requires ≥3 items"""
    # Setup: Epic mode, guidance loaded
    # Execute: Success criteria question
    # If user provides <3: Ask again with error message
    # If user provides ≥3: Accept and proceed
    # Assert: Validation enforced
```

### 5.2 Sprint Mode Unit Tests

**Test 6: Sprint mode loads guidance**
```python
def test_sprint_mode_loads_guidance():
    """Verify guidance loads for sprint planning"""
    # Setup: Context marker "**Command:** create-sprint"
    # Execute: Phase 3 Step 1
    # Assert: Read called with guidance file path
    # Assert: Log contains "Sprint planning mode"
```

**Test 7: Epic selection (Bounded + None)**
```python
def test_epic_selection_bounded_plus_none():
    """Verify epic selection includes 'None' option"""
    # Setup: Sprint mode, guidance loaded
    # Execute: Epic selection question
    # Assert: All existing epics listed
    # Assert: Explicit "None - Standalone Sprint" option present
    # Assert: User can select one epic or None
```

**Test 8: Story selection (Multi-Select)**
```python
def test_story_selection_multi_select():
    """Verify story selection supports multi-select"""
    # Setup: Sprint mode, guidance loaded
    # Execute: Story selection question
    # Assert: Multi-select enabled (multiple stories)
    # Assert: Running total displayed
    # Assert: Capacity warnings shown
```

**Test 9: Capacity guidance (running total)**
```python
def test_capacity_guidance_running_total():
    """Verify running total and capacity warnings"""
    # Setup: User selecting stories
    # User selects STORY-051 (5 pts)
    # Assert: Display "Selected: STORY-051 (5 pts) | Total: 5 pts"
    # User selects STORY-054 (21 pts)
    # Assert: Display "Selected: ... | Total: 26 pts ✓ Optimal"
    # User selects STORY-055 (13 pts)
    # Assert: Display "Selected: ... | Total: 39 pts ⚠️ Over capacity"
```

**Test 10: Capacity enforcement (none, guidance only)**
```python
def test_capacity_enforcement_guidance_only():
    """Verify capacity is guidance, not enforcement"""
    # Setup: Sprint with 55 points (well over 40)
    # Execute: Sprint creation with capacity warning
    # Assert: Sprint created successfully (no blocking)
    # Assert: Warning displayed (guidance)
    # Assert: User can proceed (no enforcement)
```

### 5.3 Integration Tests

**Test 11: Epic creation full flow**
```python
def test_orchestration_epic_full_flow():
    """Full epic workflow: detect mode → load guidance → apply patterns"""
    # Setup: User runs /create-epic
    # Execute: Phase 4A complete
    # Assert: Mode detected as epic
    # Assert: Guidance loaded
    # Assert: Goal question asked (Open-Ended)
    # Assert: Timeline options presented (4 choices)
    # Assert: Priority levels presented (4 levels)
    # Assert: Success criteria collected (min 3)
    # Assert: Epic file created with guided responses
```

**Test 12: Sprint planning full flow**
```python
def test_orchestration_sprint_full_flow():
    """Full sprint workflow: detect mode → load guidance → apply patterns"""
    # Setup: User runs /create-sprint
    # Execute: Phase 3 complete
    # Assert: Mode detected as sprint
    # Assert: Guidance loaded
    # Assert: Epic selection asked (with None option)
    # Assert: Story selection asked (multi-select, capacity warnings)
    # Assert: Running total displayed as user selects
    # Assert: Sprint file created with capacity note
```

**Test 13: Backward compatibility - story management unchanged**
```python
def test_backward_compat_story_management():
    """Verify story management mode unchanged (no guidance)"""
    # Setup: Existing story in development
    # Execute: Orchestration for story management
    # Assert: No guidance loaded
    # Assert: Same behavior as pre-STORY-057
    # Assert: Workflow progresses automatically (no interactive questions)
```

### 5.4 Regression Tests

**15 existing orchestration tests:**
- Sprint planning tests (5)
- Epic creation tests (5)
- Story management tests (3)
- Workflow state transition tests (2)

All must pass with guidance integration.

---

## Section 6: Token Budget Analysis

### 6.1 Epic Mode Token Cost

```
Step 0: Mode detection + guidance loading
  - Context marker check: ~30 tokens
  - Log messages: ~20 tokens
  - Read file (600 lines): ~500-700 tokens
  - Pattern lookups: ~50-100 tokens
  - Subtotal: ~600-850 tokens

Steps 1-4: Pattern application (epic questions)
  - Epic goal question: ~50-100 tokens
  - Timeline options: ~50 tokens
  - Priority levels: ~50 tokens
  - Success criteria: ~50 tokens
  - Subtotal: ~200-250 tokens

TOTAL epic mode: 800-1,100 tokens (~1,000 avg)
```

### 6.2 Sprint Mode Token Cost

```
Step 0: Mode detection + guidance loading
  - Context marker check: ~30 tokens
  - Log messages: ~20 tokens
  - Read file (600 lines): ~500-700 tokens
  - Pattern lookups: ~50-100 tokens
  - Subtotal: ~600-850 tokens

Steps 1-2: Pattern application (sprint questions)
  - Epic selection options: ~75-100 tokens
  - Story multi-select: ~50-75 tokens
  - Capacity guidance: ~50-100 tokens
  - Subtotal: ~175-275 tokens

TOTAL sprint mode: 775-1,125 tokens (~1,000 avg)
```

### 6.3 Other Modes Token Cost

```
Story Management, Audit, etc.: 0 tokens (no guidance loaded)
```

---

## Section 7: Skill Integration Checklist

### 7.1 SKILL.md Modifications

- [ ] **Phase 4A Step 2 Head:** Add note "Step 0: Conditional guidance loading for epic mode"
- [ ] **Phase 4A Step 2 (NEW):** Add ~10 lines of epic mode detection + guidance loading
- [ ] **Phase 3 Step 1 Head:** Add note "Step 0: Conditional guidance loading for sprint mode"
- [ ] **Phase 3 Step 1 (NEW):** Add ~10 lines of sprint mode detection + guidance loading
- [ ] **Pattern References:** Add guidance pattern references to questions (20 lines)
- [ ] **Total additions:** ~40 lines to SKILL.md (distributed: 10 + 10 + 20)

### 7.2 Reference File Deployment

- [ ] **Master:** user-input-guidance.md exists in ideation/references/
- [ ] **Orchestration Copy:** user-input-guidance.md copied to orchestration/references/
- [ ] **Operational:** user-input-guidance.md synced to .claude/skills/devforgeai-orchestration/
- [ ] **Checksums:** All 3 copies have identical SHA256 hashes

### 7.3 Testing

- [ ] **Epic Mode Tests:** 5 tests (mode detection, goal, timeline, priority, success criteria)
- [ ] **Sprint Mode Tests:** 5 tests (mode detection, epic selection, story selection, capacity, enforcement)
- [ ] **Integration Tests:** 2 tests (epic full flow, sprint full flow)
- [ ] **Regression Tests:** 15 existing orchestration tests all passing
- [ ] **Total:** 27 orchestration-specific tests (5 + 5 + 2 + 15)

### 7.4 Documentation

- [ ] **This Reference File:** Created and reviewed (~300 lines for dual modes)
- [ ] **Epic Pattern Mapping Table:** Completed (Section 2.1)
- [ ] **Sprint Pattern Mapping Table:** Completed (Section 3.1)
- [ ] **Examples:** 6 before/after examples (3 epic + 3 sprint)
- [ ] **Deployment Process:** Step-by-step instructions provided

---

## Section 8: Success Validation

**Epic Mode Validation:**
✅ Context marker `**Command:** create-epic` detected
✅ guidance_content = Read(...) executes
✅ Log: "Epic creation mode detected"
✅ Phase 4A Step 2 questions use guidance patterns:
  - Goal: Open-Ended Discovery (free text)
  - Timeline: Bounded Choice (4 options)
  - Priority: Explicit Classification (4 levels)
  - Success Criteria: Min Count validated (≥3)
✅ Epic file created with guided responses

**Sprint Mode Validation:**
✅ Context marker `**Command:** create-sprint` detected
✅ guidance_content = Read(...) executes
✅ Log: "Sprint planning mode detected"
✅ Phase 3 Step 1 questions use guidance patterns:
  - Epic selection: Bounded + Explicit None (all epics + None)
  - Story selection: Multi-Select (all stories, running total)
  - Capacity guidance: Warnings for <20 or >40 points
✅ Sprint file created with capacity note

**Backward Compatibility Validation:**
✅ All 15 existing orchestration tests pass
✅ Story management mode behavior unchanged (no guidance)
✅ No breaking changes to SKILL interface
✅ Other modes (audit, checkpoint) unaffected
✅ Guidance is non-blocking (if file missing, continues)

---

## Section 9: Consistency Across All 3 Skills

**Same Guidance File:**
- Architecture uses: `.claude/skills/devforgeai-architecture/references/user-input-guidance.md`
- UI-Generator uses: `.claude/skills/devforgeai-ui-generator/references/user-input-guidance.md`
- Orchestration uses: `.claude/skills/devforgeai-orchestration/references/user-input-guidance.md`
- All 3 are IDENTICAL copies (checksum validated)

**Pattern Name Consistency:**
- Open-Ended Discovery (all 3 use)
- Bounded Choice (all 3 use)
- Explicit Classification (all 3 use)
- Closed Confirmation (architecture only)
- Custom patterns for multi-select with capacity (orchestration only)

**Fallback Behavior:**
- If guidance file missing: Log warning, use baseline questions
- If guidance file corrupted: Log error, graceful degradation
- If pattern not found: Log info, use fallback logic
- NO workflow halting (all non-blocking)

---

**Version 1.0** | **Status: Published** | **Created: 2025-01-21**
