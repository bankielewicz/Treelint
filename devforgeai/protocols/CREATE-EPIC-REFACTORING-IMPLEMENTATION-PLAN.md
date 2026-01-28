# /create-epic Refactoring: Implementation Plan

**Date:** 2025-11-05
**Status:** ✅ READY FOR IMPLEMENTATION
**Priority:** 🟡 MEDIUM
**Pattern:** Lean Orchestration (proven: /dev, /qa, /ideate, /create-story)

---

## Executive Summary

Refactor `/create-epic` from **526 lines (14,309 chars - 95% budget)** to **~250 lines (~8K chars - 53% budget)** by extracting business logic to `devforgeai-orchestration` skill, which already has epic creation infrastructure.

**Strategy:** Enhance orchestration skill's existing `--create-epic` mode, create lean command following proven pattern.

**Subagents needed:** ❌ NONE - Use existing requirements-analyst and architect-reviewer

**Reference files created:** ✅ 3 new files (2,550 lines total) via agent-generator

---

## Problem Statement

### Current Architecture Violations

The `/create-epic` command violates lean orchestration principles established in `devforgeai/protocols/lean-orchestration-pattern.md`:

**Command responsibilities (what it SHOULD do):**
- ✅ Parse arguments → Currently does (lines 13-15)
- ❌ Load context via @file → Currently MISSING
- ❌ Set context markers → Currently MISSING
- ❌ Invoke skill → Currently MISSING
- ❌ Display results → Currently MISSING

**Command violations (what it should NOT do):**
- ✅ Business logic → Currently has 350+ lines (Phases 1-7) **VIOLATES**
- ✅ Subagent invocation → Currently invokes requirements-analyst, technical-assessor **VIOLATES**
- ✅ Template generation → Currently generates epic document (lines 186-284) **VIOLATES**
- ✅ Complex decision-making → Technology conflicts (lines 171-179) **VIOLATES**
- ✅ Error recovery → Fallback strategies (lines 387-440) **VIOLATES**

**Character budget impact:**
```
Current: 14,309 characters (95% of 15,000 limit)
Status: ⚠️ HIGH USAGE - approaching hard limit
Risk: Next feature addition causes overflow
```

**Token efficiency impact:**
```
Current: ~10,000 tokens in main conversation
After refactoring: ~2,000 tokens
Savings: 80% reduction (8,000 tokens freed)
```

### Root Cause

**Historical context:** `/create-epic` command predates lean orchestration pattern:
- Created before `/dev` refactoring (Nov 2025)
- Created before lean-orchestration-pattern.md protocol
- Implemented business logic directly (no skill layer)
- Never refactored when pattern established

**Evidence:** `devforgeai-orchestration` skill declares epic creation capability but doesn't implement it:
- Line 50: `Skill(command="devforgeai-orchestration --create-epic --title='User Management'")`
- Lines 234-244: Epic Creation section (placeholder only)
- References epic-management.md (EXISTS), epic-template.md (EXISTS)
- **Conclusion:** Infrastructure exists, implementation missing

---

## Solution Architecture

### Target State

```
User: /create-epic User Authentication System
  ↓
/create-epic Command (Lean - 250 lines, 8K chars)
  ├─ Phase 0: Validate epic name argument
  ├─ Phase 1: Set context markers (**Epic name:** ...)
  ├─ Phase 2: Invoke Skill(command="devforgeai-orchestration")
  └─ Phase 3: Display results from skill
  ↓
devforgeai-orchestration Skill (Enhanced - add 300 lines)
  ├─ Detect "create-epic" mode from context
  ├─ Phase 1: Epic Discovery (ID generation, duplicate check)
  ├─ Phase 2: Context Gathering (goal, timeline, stakeholders, success criteria)
  ├─ Phase 3: Feature Decomposition
  │  ├─ Load feature-decomposition-patterns.md (NEW - 850 lines)
  │  └─ Invoke requirements-analyst subagent
  ├─ Phase 4: Technical Assessment
  │  ├─ Load technical-assessment-guide.md (NEW - 900 lines)
  │  └─ Invoke architect-reviewer subagent
  ├─ Phase 5: Epic File Creation (using epic-template.md)
  ├─ Phase 6: Optional Requirements Spec
  └─ Phase 7: Validation
     ├─ Load epic-validation-checklist.md (NEW - 800 lines)
     └─ Self-heal correctable issues, HALT on critical failures
  ↓
Subagents (Isolated Contexts)
  ├─ requirements-analyst (~30K tokens) - Feature decomposition
  └─ architect-reviewer (~40K tokens) - Technical complexity
```

**Total tokens in main conversation:** ~2,000 (command overhead only)
**Total tokens in isolated contexts:** ~70,000 (skill + subagents)
**Efficiency gain:** 80% reduction in main conversation tokens

---

## Implementation Steps

### ✅ STEP 1: Create Missing Reference Files (COMPLETE)

**Status:** ✅ COMPLETE (via agent-generator)

**Files created:**
1. ✅ `feature-decomposition-patterns.md` (850 lines, 26K)
2. ✅ `technical-assessment-guide.md` (900 lines, 28K)
3. ✅ `epic-validation-checklist.md` (800 lines, 21K)

**Location:** `.claude/skills/devforgeai-orchestration/references/`

**Validation:**
```bash
ls -lh .claude/skills/devforgeai-orchestration/references/*.md
# All 3 files present with expected sizes
```

---

### STEP 2: Enhance devforgeai-orchestration Skill

**File:** `.claude/skills/devforgeai-orchestration/SKILL.md`

**Current state:**
- Epic creation declared (line 50) but not implemented
- Placeholder section (lines 234-244)
- References exist: epic-management.md, epic-template.md

**Enhancement required:**

#### 2.1: Add Epic Creation Mode Detection

**Location:** After "When to Use This Skill" section (after line 54)

**Add:**
```markdown
## Mode Detection

This skill operates in multiple modes based on conversation context:

**Epic Creation Mode:**
- Triggered by: `**Command:** create-epic` marker in conversation
- Entry point: Phase 4 Epic Creation
- Subagents: requirements-analyst, architect-reviewer
- Output: Epic document + optional requirements spec

**Sprint Planning Mode:**
- Triggered by: `**Command:** create-sprint` marker
- Entry point: Phase 4 Sprint Planning
- Subagents: requirements-analyst
- Output: Sprint plan + story associations

**Story Management Mode:**
- Triggered by: `**Story ID:** STORY-NNN` marker
- Entry point: Phase 1-3 (status management)
- Subagents: Various (based on workflow state)
- Output: Story status updates, workflow history

**Default Mode (Story Orchestration):**
- Triggered by: No specific mode marker
- Entry point: Phase 1 (story validation)
- Behavior: Standard story lifecycle management
```

#### 2.2: Implement Epic Creation Workflow

**Location:** Replace placeholder at lines 234-244

**Add complete implementation:**

```markdown
### Epic Creation (NEW IMPLEMENTATION)

**Entry point:** Detect `**Command:** create-epic` and `**Epic name:** {name}` in conversation

**Prerequisites:**
- Epic name provided in conversation context
- devforgeai/specs/Epics/ directory (create if missing)

**Workflow (7 Phases):**

#### Phase 1: Epic Discovery

**Objective:** Generate unique epic ID, check for duplicates

```
# Find existing epics
epic_files = Glob(pattern="devforgeai/specs/Epics/*.epic.md")

# Determine next epic ID
if no epics exist:
  next_id = "EPIC-001"
else:
  Read highest epic file
  Extract ID from YAML frontmatter
  Increment: EPIC-002, EPIC-003, etc.

# Check for duplicate names
Grep(
  pattern="title: .*{epic_name}",
  path="devforgeai/specs/Epics/",
  output_mode="files_with_matches",
  -i=true
)

# If duplicate found:
AskUserQuestion:
  Question: "Epic with name '{epic_name}' already exists. How to proceed?"
  Options:
    - Create new epic with same name (will use {next_id})
    - Edit existing epic instead
    - Cancel and provide different name
```

**Output:** Unique epic ID, duplicate resolution

**Reference:** Load epic-management.md (Section: Epic ID Generation)

---

#### Phase 2: Epic Context Gathering

**Objective:** Collect business context, goals, stakeholders, success criteria

**Step 2.1: Epic Goal**
```
AskUserQuestion:
  Question: "What is the primary goal of this epic?"
  Header: "Epic Goal"
  Options:
    - "New feature development"
    - "Technical improvement/refactoring"
    - "Bug fix/technical debt resolution"
    - "Infrastructure/DevOps enhancement"
    - "Migration/modernization"
    - "Performance optimization"
```

**Step 2.2: Timeline and Priority**
```
AskUserQuestion:
  Question: "What is the expected timeline and priority?"
  Header: "Epic Planning"
  Options (multiSelect):
    Timeline:
      - "1-2 sprints"
      - "3-4 sprints"
      - "5+ sprints"
      - "Unknown"
    Priority:
      - "Critical"
      - "High"
      - "Medium"
      - "Low"
    Business Value:
      - "High"
      - "Medium"
      - "Low"
```

**Step 2.3: Stakeholders**
```
AskUserQuestion:
  Question: "Who are the key stakeholders?"
  Header: "Stakeholders"
  Free-form: "List stakeholders (comma-separated): Product owner, Tech lead, QA lead, etc."
```

**Step 2.4: Success Criteria**
```
AskUserQuestion:
  Question: "What defines success for this epic?"
  Header: "Success Criteria"
  Free-form: "Describe 3+ measurable outcomes (e.g., 'Reduce login time by 50%', 'Support 10K concurrent users')"
```

**Output:** Complete epic context (goal, timeline, priority, business value, stakeholders, success criteria)

**Reference:** Load epic-management.md (Section: Epic Context Definition)

---

#### Phase 3: Feature Decomposition

**Objective:** Break epic into 3-8 independently valuable features

**Load reference:**
```
Read(file_path=".claude/skills/devforgeai-orchestration/references/feature-decomposition-patterns.md")
# 850 lines of decomposition patterns by epic type
# Progressive disclosure: Load only when Phase 3 executes
```

**Invoke subagent:**
```
Task(
  subagent_type="requirements-analyst",
  description="Decompose epic into features",
  prompt="Decompose the epic '{epic_name}' into high-level features.

Epic Context:
- Goal: {epic_goal from Phase 2}
- Timeline: {timeline}
- Priority: {priority}
- Business Value: {business_value}
- Success Criteria: {success_criteria}

Use feature decomposition patterns from reference file to:
1. Identify epic type (CRUD, Auth, API, Reporting, Workflow, E-commerce)
2. Apply appropriate decomposition pattern
3. Generate 3-8 features that:
   - Represent significant functional units
   - Can be implemented independently (or with minimal dependencies)
   - Deliver incremental value
   - Are testable and demonstrable

For each feature provide:
- Feature name (clear, action-oriented, user-focused)
- Description (1-2 sentences describing user capability)
- Estimated complexity (Simple, Moderate, Complex)
- Dependencies (which features must be done first)
- User value (what benefit does this provide)

Reference framework constraints:
- tech-stack.md (if exists): Technology choices
- architecture-constraints.md (if exists): Layer boundaries

Format as structured markdown ready for epic document."
)
```

**Review features:**
```
Display feature list to user

AskUserQuestion:
  Question: "Review generated features. Accept or modify?"
  Header: "Feature Review"
  Options:
    - "Accept all features (proceed to next phase)"
    - "Remove specific features (I'll specify which)"
    - "Add additional features (I'll describe)"
    - "Modify feature descriptions (I'll provide changes)"
```

**If modifications requested:**
- Iterate with requirements-analyst
- Re-present for confirmation
- Loop until approved

**Output:** 3-8 approved features with metadata

**Reference:** feature-decomposition-patterns.md (850 lines - loaded in this phase)

---

#### Phase 4: Technical Assessment

**Objective:** Assess complexity, identify risks, validate against context files

**Check context files:**
```
context_files = Glob(pattern="devforgeai/context/*.md")

if context_files.length == 6:
  greenfield = false
  Read(file_path="devforgeai/context/tech-stack.md")
  Read(file_path="devforgeai/context/architecture-constraints.md")
  Read(file_path="devforgeai/context/dependencies.md")
  Read(file_path="devforgeai/context/anti-patterns.md")
else:
  greenfield = true
  # Skip context validation (no constraints yet)
```

**Load reference:**
```
Read(file_path=".claude/skills/devforgeai-orchestration/references/technical-assessment-guide.md")
# 900 lines of complexity scoring rubric, risk patterns
# Progressive disclosure: Load only when Phase 4 executes
```

**Invoke subagent:**
```
Task(
  subagent_type="architect-reviewer",
  description="Assess epic technical complexity",
  prompt="Assess technical complexity for epic '{epic_name}' with features:

{Feature list from Phase 3}

Use technical assessment guide to analyze:
1. Technology Stack Impact - New technologies? Learning curve?
2. Architecture Changes - New layers, services, infrastructure?
3. Integration Complexity - External systems, APIs, third-party services?
4. Data Modeling Complexity - Schema changes, migrations, entities?
5. Testing Complexity - New test types, infrastructure, automation?
6. Security Considerations - Auth, authz, data protection, OWASP?
7. Performance Requirements - Scalability, latency, throughput?

Score complexity (0-10 scale) using rubric:
- 0-2: Trivial
- 3-4: Low
- 5-6: Moderate
- 7-8: High
- 9-10: Very High (may require epic split)

{IF context files exist}
CRITICAL: Validate against framework constraints:
- tech-stack.md: Are technologies approved? Flag conflicts.
- architecture-constraints.md: Do changes violate layer rules? Block violations.
- dependencies.md: Are integrations approved? Flag unapproved.
- anti-patterns.md: Do features introduce forbidden patterns? Block violations.

{IF greenfield mode}
Note: Context files not present - operating in greenfield mode.
Recommend creating context files before implementation.

Provide:
- Overall complexity score (0-10)
- Technology stack list (with approval status if context exists)
- Architecture impact description
- Key technical risks (2+ with mitigation strategies)
- Prerequisites (skills, tools, infrastructure)
- [IF CONFLICTS] Technology conflict flags for resolution"
)
```

**Validate against context files:**
```
if architect_reviewer_output contains "REQUIRES ADR" or "⚠️ Conflicts":
  AskUserQuestion:
    Question: "Technical assessment found conflicts with tech-stack.md. How to proceed?"
    Options:
      - "Update tech-stack.md to allow new technology (requires ADR)"
      - "Adjust epic scope to use existing approved technology"
      - "Mark as technical debt to resolve during architecture phase"
```

**Output:** Technical assessment with complexity score, risks, prerequisites, conflict resolution

**Reference:** technical-assessment-guide.md (900 lines - loaded in this phase)

---

#### Phase 5: Epic File Creation

**Objective:** Generate epic document using template

**Load template:**
```
Read(file_path=".claude/skills/devforgeai-orchestration/assets/templates/epic-template.md")
# 265 lines - epic document structure
```

**Populate template with gathered data:**

```markdown
---
id: {EPIC-ID from Phase 1}
title: {epic_name}
status: Planning
priority: {priority from Phase 2}
business_value: {business_value from Phase 2}
timeline: {timeline from Phase 2}
created: {current_date YYYY-MM-DD}
updated: {current_date YYYY-MM-DD}
stakeholders:
  - {stakeholder_1 from Phase 2}
  - {stakeholder_2}
  - ...
---

# {EPIC-ID}: {epic_name}

## Overview

{Business context from Phase 2 - epic goal}

## Business Value

{Business value description from Phase 2}

## Success Criteria

{Success criteria from Phase 2}

### Measurable Outcomes

- [ ] {Outcome 1}
- [ ] {Outcome 2}
- [ ] {Outcome 3}

## Features

{Features from Phase 3 - formatted per template}

### Feature 1: {Feature Name}
**Description:** {description}
**Complexity:** {Simple/Moderate/Complex}
**Dependencies:** {dependencies}
**User Value:** {user value}
**Estimated Story Points:** TBD during sprint planning

[... repeat for all features ...]

## Technical Assessment

{Technical assessment from Phase 4}

**Overall Complexity:** {score}/10

**Technology Stack:**
- {Tech 1}: {Purpose} {[APPROVED] or [REQUIRES ADR]}

**Architecture Impact:**
{Description}

**Key Technical Risks:**
1. {Risk 1} - Mitigation: {approach}
2. {Risk 2} - Mitigation: {approach}

**Prerequisites:**
- {Prerequisite 1}
- {Prerequisite 2}

## Timeline Estimate

**Target Duration:** {timeline from Phase 2}
**Estimated Sprints:** {calculated from features}
**Dependencies:** {External dependencies}

## Stakeholders

| Role | Name | Responsibility |
|------|------|----------------|
{Stakeholder table from Phase 2}

## Related Documentation

- Requirements Spec: `devforgeai/specs/requirements/{EPIC-ID}-requirements.md` (to be created)
- ADRs: (to be created during architecture phase)
- User Stories: (to be created during sprint planning)

## Status History

- {YYYY-MM-DD}: Created - Epic defined and scoped
```

**Write file:**
```
Write(
  file_path="devforgeai/specs/Epics/{EPIC-ID}.epic.md",
  content={populated_template}
)
```

**Output:** Epic file created at devforgeai/specs/Epics/{EPIC-ID}.epic.md

**Reference:** epic-template.md (265 lines - loaded in this phase)

---

#### Phase 6: Requirements Specification (Optional)

**Objective:** Create detailed requirements document if requested

**Ask user:**
```
AskUserQuestion:
  Question: "Would you like to create a detailed requirements specification for this epic?"
  Header: "Requirements Documentation"
  Options:
    - "Yes - Create full requirements spec (functional, non-functional, data models)"
    - "No - Epic document is sufficient for now"
    - "Later - I'll create requirements during sprint planning"
```

**If "Yes" selected:**

```
Task(
  subagent_type="requirements-analyst",
  description="Create requirements specification",
  prompt="Create detailed requirements specification for epic '{epic_name}'.

Based on features:
{Feature list from Phase 3}

Generate comprehensive requirements document including:
1. Functional Requirements - Detailed use cases, user flows
2. Non-Functional Requirements - Performance, security, scalability
3. Data Models - Entities, attributes, relationships
4. API Contracts - Endpoints, request/response formats (if applicable)
5. Business Rules - Validation, workflows, constraints
6. Integration Requirements - External systems, APIs, services

Reference framework constraints:
- tech-stack.md (if exists)
- architecture-constraints.md (if exists)
- dependencies.md (if exists)

Format as structured markdown suitable for `devforgeai/specs/requirements/`"
)

Write(
  file_path="devforgeai/specs/requirements/{EPIC-ID}-requirements.md",
  content={requirements_spec}
)

# Update epic with requirements link
Edit(
  file_path="devforgeai/specs/Epics/{EPIC-ID}.epic.md",
  old_string="- Requirements Spec: `devforgeai/specs/requirements/{EPIC-ID}-requirements.md` (to be created)",
  new_string="- Requirements Spec: `devforgeai/specs/requirements/{EPIC-ID}-requirements.md`"
)
```

**Output:** Requirements specification document (if requested)

---

#### Phase 7: Epic Validation and Self-Healing

**Objective:** Validate epic quality, self-heal correctable issues

**Load reference:**
```
Read(file_path=".claude/skills/devforgeai-orchestration/references/epic-validation-checklist.md")
# 800 lines of validation procedures, self-healing logic
# Progressive disclosure: Load only when Phase 7 executes
```

**Execute validation:**
```
1. YAML Frontmatter Validation
   - Check all required fields present
   - Self-heal: Auto-generate missing IDs, dates
   - HALT: If epic name invalid

2. Content Section Validation
   - Check all sections have content (>50 words)
   - Self-heal: Re-populate from Phase 2 data if missing
   - HALT: If business goal missing and cannot infer

3. Feature Scoping Validation
   - Check feature count: 3-8 optimal
   - Check for circular dependencies
   - Self-heal: Flag over/under-scoped
   - HALT: If circular dependencies detected

4. Coherence Validation
   - Features align with epic goal
   - Success criteria match features
   - Cannot self-heal: Requires manual review

5. Feasibility Validation
   - Complexity score ≤8 (if >8, over-scoped)
   - Timeline realistic (features × complexity)
   - Self-heal: Flag high complexity
   - HALT: If complexity >10

6. Framework Integration Validation
   - If context files exist:
     - Technologies in tech-stack.md: HALT if conflicts
     - Architecture constraints respected: HALT if violations
     - No anti-patterns present: HALT if found
   - If context files don't exist:
     - Note greenfield mode
     - Recommend context creation before implementation

7. Generate Validation Report
   - Passed checks, warnings (self-healed), failures (HALT)
   - Self-healing actions taken
   - Manual interventions required
```

**Output:** Validation report, self-healed epic, or HALT with errors

**Reference:** epic-validation-checklist.md (800 lines - loaded in this phase)

---

#### Phase 8: Completion Summary

**Objective:** Return structured summary for command to display

**Generate summary:**
```markdown
{
  "status": "success",
  "epic_id": "{EPIC-ID}",
  "epic_name": "{epic_name}",
  "priority": "{priority}",
  "business_value": "{business_value}",
  "timeline": "{timeline}",
  "feature_count": {count},
  "features": [
    {"name": "{feature_1}", "complexity": "{Simple/Moderate/Complex}"},
    {"name": "{feature_2}", "complexity": "{...}"},
    ...
  ],
  "technical_assessment": {
    "complexity_score": {score},
    "risk_count": {count},
    "prerequisite_count": {count}
  },
  "files_created": [
    "devforgeai/specs/Epics/{EPIC-ID}.epic.md",
    "devforgeai/specs/requirements/{EPIC-ID}-requirements.md" (if created)
  ],
  "next_steps": [
    "Review epic document: devforgeai/specs/Epics/{EPIC-ID}.epic.md",
    "Create sprint: /create-sprint {sprint-number}",
    "Break features into stories during sprint planning",
    "Implement stories: /dev {STORY-ID}"
  ],
  "validation": {
    "passed": {count},
    "warnings": {count},
    "self_healed": [{actions}]
  }
}
```

**Return to command:** Structured JSON summary for display

---

**Reference files used progressively:**
- epic-management.md (Phase 1, 2 - 496 lines)
- feature-decomposition-patterns.md (Phase 3 - 850 lines)
- technical-assessment-guide.md (Phase 4 - 900 lines)
- epic-template.md (Phase 5 - 265 lines)
- epic-validation-checklist.md (Phase 7 - 800 lines)

**Total reference content:** 3,311 lines (loaded progressively, isolated context)

**Subagents invoked:**
- requirements-analyst (Phase 3, optional Phase 6)
- architect-reviewer (Phase 4)

**Estimated skill addition:** ~300 lines of workflow orchestration
**Total skill size after enhancement:** ~900 lines (all in isolated context)
```

#### 2.3: Update Reference Section

**Location:** Section "Reference Files" (around line 577)

**Add new reference files:**
```markdown
### Project Management
- **`./references/epic-management.md`** - Epic planning, decomposition, estimation (496 lines)
- **`./references/feature-decomposition-patterns.md`** - Epic → feature breakdown patterns by type (850 lines) **NEW**
- **`./references/technical-assessment-guide.md`** - Complexity scoring, risk assessment (900 lines) **NEW**
- **`./references/epic-validation-checklist.md`** - Epic quality validation, self-healing (800 lines) **NEW**
- **`./references/sprint-planning.md`** - Sprint capacity, story selection, tracking (620 lines)
```

---

### STEP 3: Refactor /create-epic Command

**File:** `.claude/commands/create-epic.md`

**Current:** 526 lines, 14,309 characters (95% of budget)
**Target:** ~250 lines, ~8,000 characters (53% of budget)

**New command structure:**

```markdown
---
description: Create epic with feature breakdown
argument-hint: [epic-name]
model: opus
allowed-tools: AskUserQuestion, Skill
---

# Create Epic Command

Creates a new epic by delegating to devforgeai-orchestration skill.

## Arguments

- `epic-name` (required): Name of the epic (e.g., "User Authentication System")

## Workflow

### Phase 0: Argument Validation

**Validate epic name:**

```bash
epic_name="$1"

if [ -z "$epic_name" ]; then
  echo "❌ Error: Epic name required"
  echo ""
  echo "Usage: /create-epic [epic-name]"
  echo "Example: /create-epic User Authentication System"
  echo ""
  HALT
fi
```

**Validate epic name format:**
- Minimum 10 characters
- Maximum 100 characters
- Allowed: Alphanumeric, spaces, hyphens, underscores

**If invalid:**
```
❌ Invalid epic name: {reason}

Epic names must:
  - Be 10-100 characters
  - Use alphanumeric characters, spaces, hyphens, underscores only
  - Be descriptive and action-oriented

Examples:
  ✅ "User Authentication System"
  ✅ "Payment Processing Overhaul"
  ✅ "Real-time Analytics Dashboard"
  ❌ "Auth" (too short)
  ❌ "Epic@123!" (special characters)

Please provide a valid epic name.
```

---

### Phase 1: Set Context Markers

**Provide explicit context for orchestration skill:**

```
**Epic name:** $epic_name
**Command:** create-epic
**Mode:** interactive
```

---

### Phase 2: Invoke Orchestration Skill

**Delegate to skill:**

```
Skill(command="devforgeai-orchestration")
```

The orchestration skill will:
1. Detect "create-epic" mode from context markers
2. Execute 7-phase epic creation workflow
3. Invoke subagents (requirements-analyst, architect-reviewer)
4. Load reference files progressively (epic-management, feature-decomposition-patterns, technical-assessment-guide, epic-validation-checklist, epic-template)
5. Generate epic document and optional requirements spec
6. Self-validate epic quality
7. Return structured completion summary

---

### Phase 3: Display Results

**Display skill output directly:**

[Skill returns structured summary - display without modification]

```
✅ Epic Created Successfully

Epic Details:
  📋 ID: {EPIC-ID}
  🎯 Title: {epic_name}
  🏆 Priority: {priority}
  📊 Business Value: {business_value}
  📅 Timeline: {timeline}

Features: {count} features identified
  ✨ {Feature 1 name} - {complexity}
  ✨ {Feature 2 name} - {complexity}
  ✨ {Feature 3 name} - {complexity}
  [... list all features from skill output ...]

Technical Assessment:
  🔧 Complexity Score: {score}/10
  ⚠️ Key Risks: {count} identified
  📦 Prerequisites: {count}

Files Created:
  📁 devforgeai/specs/Epics/{EPIC-ID}.epic.md
  {If requirements spec created}
  📁 devforgeai/specs/requirements/{EPIC-ID}-requirements.md

{If validation warnings}
⚠️ Validation Warnings:
  - {Warning 1} (self-healed)
  - {Warning 2} (self-healed)
```

---

### Phase 4: Next Steps Guidance

**Provide actionable next steps:**

```
Next Steps:
  1. Review epic document: devforgeai/specs/Epics/{EPIC-ID}.epic.md
  2. {If context files don't exist}
     ⚠️ Create architectural context: /create-context {project-name}
  3. Create sprint: /create-sprint {sprint-number}
  4. Break features into stories during sprint planning
  5. Implement stories: /dev {STORY-ID}

{If technology conflicts flagged}
⚠️ Action Required:
  - Technology conflicts detected (see epic technical assessment)
  - Create ADR before implementation: devforgeai/adrs/ADR-NNN-{decision}.md
  - Update tech-stack.md with approved technologies
```

---

## Error Handling

### Error: Invalid Epic Name

**Condition:** Epic name empty, too short/long, invalid characters

**Action:** Display validation error in Phase 0, HALT

### Error: Skill Invocation Failed

**Condition:** devforgeai-orchestration skill returns error

**Action:**
```
❌ Epic creation failed: {skill_error_message}

The orchestration skill encountered an issue:
  {Error details from skill}

Suggested actions:
  {Recovery steps from skill}

Do NOT attempt to create epic manually - the skill will handle recovery.
```

**No fallback logic** - Let skill handle all error recovery

### Error: Directory Structure Missing

**Condition:** devforgeai/specs/Epics/ directory doesn't exist

**Action:** Skill creates directory automatically, command does not handle

---

## Success Criteria

- [x] Epic name validated in Phase 0
- [x] Context markers set in Phase 1
- [x] Skill invoked in Phase 2
- [x] Results displayed in Phase 3
- [x] Next steps provided in Phase 4
- [x] Character budget < 8,000 (target: 53% of 15K limit)
- [x] Token usage < 2,000 in main conversation
- [x] Zero business logic in command
- [x] Single skill invocation only

---

## Token Efficiency

**Estimated token breakdown:**

| Phase | Description | Tokens |
|-------|-------------|--------|
| Phase 0 | Argument validation | ~200 |
| Phase 1 | Context markers | ~100 |
| Phase 2 | Skill invocation | ~500 |
| Phase 3 | Display results | ~1,000 |
| Phase 4 | Next steps guidance | ~200 |
| **TOTAL** | **Command overhead** | **~2,000** |

**Skill execution (isolated context):**
| Component | Tokens |
|-----------|--------|
| Orchestration skill workflow | ~30,000 |
| requirements-analyst subagent | ~30,000 |
| architect-reviewer subagent | ~40,000 |
| Reference files (progressive) | ~25,000 |
| **TOTAL (isolated)** | **~125,000** |

**Main conversation impact:** ~2,000 tokens (98% reduction from current ~10,000)

---

## Integration Points

**Command invokes:**
- devforgeai-orchestration skill (--create-epic mode)

**Skill invokes:**
- requirements-analyst subagent (Phase 3, optional Phase 6)
- architect-reviewer subagent (Phase 4)

**Reference files loaded (progressive disclosure):**
- epic-management.md (Phase 1, 2)
- feature-decomposition-patterns.md (Phase 3)
- technical-assessment-guide.md (Phase 4)
- epic-template.md (Phase 5)
- epic-validation-checklist.md (Phase 7)

**Prerequisites:**
- None (can create epics before context files exist)

**Enables:**
- /create-sprint command (requires epics)
- Epic → Sprint → Story workflow
- devforgeai-orchestration skill (epic management)
```

**Implementation:**
1. Backup current command: `cp create-epic.md create-epic.md.backup`
2. Replace with new lean command
3. Test with example epic

---

### STEP 4: Test Refactored Implementation

**Test scenarios:**

#### Scenario 1: Greenfield Epic (No Context Files)

```bash
# Setup
rm -rf devforgeai/context/

# Execute
> /create-epic User Authentication System

# Expected
- Skill detects greenfield mode
- Epic created without tech-stack validation
- Warning: "Create context files before implementation"
- Success without blocking
```

#### Scenario 2: Brownfield Epic (Context Files Exist)

```bash
# Setup
# Ensure devforgeai/context/*.md exist

# Execute
> /create-epic Real-time Analytics Dashboard

# Expected
- Skill reads tech-stack.md
- Validates proposed technologies
- Flags conflicts if new tech proposed
- AskUserQuestion for conflict resolution
```

#### Scenario 3: Duplicate Epic Name

```bash
# Setup
# Create epic: /create-epic User Management
# Try again with same name

# Execute
> /create-epic User Management

# Expected
- Skill detects duplicate via Grep
- AskUserQuestion with options:
  - Create new with same name (EPIC-002)
  - Edit existing EPIC-001
  - Cancel and rename
```

#### Scenario 4: Optional Requirements Spec

```bash
# Execute
> /create-epic Payment Processing System

# During workflow
AskUserQuestion: "Create detailed requirements spec?"
# Select "Yes"

# Expected
- Epic created: EPIC-NNN.epic.md
- Requirements created: EPIC-NNN-requirements.md
- Epic links to requirements doc
```

#### Scenario 5: Character Budget Compliance

```bash
# Check
wc -c < .claude/commands/create-epic.md

# Expected
< 8,000 characters (53% of 15K limit)
✅ COMPLIANT
```

**Acceptance criteria:**
- All 5 scenarios pass
- Character budget < 8,000
- Token usage < 2,000 in main conversation
- Epic creation workflow identical to current user experience

---

### STEP 5: Update Documentation

#### 5.1: Update CLAUDE.md

**File:** `/mnt/c/Projects/DevForgeAI2/CLAUDE.md`

**Changes:**

**Line 486-490 (Component Summary):**
```markdown
- **Skills:** 7 (3 enhanced with deferral validation)
- **Subagents:** 19 (no change)
- **Commands:** 11 (3 refactored: /dev, /qa, /create-epic) ← UPDATE
- **Context Files:** 6 (immutable constraints)
- **Quality Gates:** 4 (Gate 3 enhanced with deferral validation)
```

**Add to Quick Reference (line 167):**
```markdown
- **Lean Orchestration:** @devforgeai/protocols/lean-orchestration-pattern.md
- **Epic Refactoring:** @devforgeai/protocols/create-epic-refactoring-plan.md ← ADD
```

---

#### 5.2: Update .claude/memory/commands-reference.md

**File:** `/mnt/c/Projects/DevForgeAI2/.claude/memory/commands-reference.md`

**Replace /create-epic section (lines 123-142):**

```markdown
### /create-epic [epic-name]

**Purpose:** Create epic with feature breakdown

**Invokes:** `devforgeai-orchestration` skill (--create-epic mode)

**Workflow:**
1. Argument validation (epic name format)
2. Set context markers (epic name, command mode)
3. Invoke orchestration skill
4. Display results from skill

**The skill handles all implementation:**
- **Phase 1-2:** Epic Discovery & Context Gathering (goal, timeline, stakeholders)
- **Phase 3-4:** Feature Decomposition & Technical Assessment (subagents)
- **Phase 5-6:** Epic File Creation & Optional Requirements Spec
- **Phase 7-8:** Validation (self-healing) & Completion Summary

**Example:**
```
> /create-epic User Authentication System
```

**Output:**
- Epic file in `devforgeai/specs/Epics/{EPIC-ID}.epic.md`
- Feature list with descriptions (3-8 features)
- Technical assessment (complexity, risks, prerequisites)
- Optional requirements specification

**Architecture (Post-Refactoring 2025-11-05):**

**Command (250 lines - Lean Orchestration):**
- Argument parsing and validation
- Context markers for skill
- Skill invocation
- Results display

**Skill (devforgeai-orchestration - Epic Creation Mode):**
- Phase 1-7: Complete epic creation workflow
- Progressive reference loading (5 files, 3,311 lines)
- Subagent delegation (requirements-analyst, architect-reviewer)
- Self-validation and self-healing

**Token Efficiency:**
- Command: ~2,000 tokens (down from ~10,000)
- Skill: ~125,000 tokens (isolated context)
- **Savings: 80% reduction in main conversation tokens**
- **Character budget: ~8,000 chars (53% of limit - down from 95%)**
```

---

#### 5.3: Update .claude/memory/skills-reference.md

**File:** `/mnt/c/Projects/DevForgeAI2/.claude/memory/skills-reference.md`

**Update orchestration skill section (after line 49):**

```markdown
**Key Features:**
- Tracks deferred work (Phase 4.5)
- Invokes technical-debt-analyzer during sprint planning
- Validates deferral tracking
- Creates follow-up stories for missing references
- **Epic creation** (Phase 4 Epic Creation) - 7-phase workflow with progressive reference loading **NEW**
- **Sprint planning** (Phase 4 Sprint Planning) - Capacity calculation, story selection
```

**Add epic creation details:**

```markdown
### devforgeai-orchestration (ENHANCED)

**Use when:**
- Starting new epics or sprints
- Creating stories from requirements
- Managing story workflow progression
- Enforcing quality gates
- Tracking deferred work (RCA-006)
- Analyzing technical debt (RCA-006)
- **Creating epics with feature decomposition** (NEW - 2025-11-05)

**Invocation (Epic Creation Mode):**
```
# Set context markers
**Epic name:** User Authentication System
**Command:** create-epic

Skill(command="devforgeai-orchestration")
```

**Epic Creation Workflow (7 Phases):**
1. Epic Discovery - Generate ID, check duplicates
2. Context Gathering - Goal, timeline, stakeholders, success criteria (via AskUserQuestion)
3. Feature Decomposition - requirements-analyst subagent, 3-8 features
4. Technical Assessment - architect-reviewer subagent, complexity scoring, risk identification
5. Epic File Creation - Using epic-template.md (265 lines)
6. Requirements Spec - Optional detailed requirements document
7. Validation & Self-Healing - epic-validation-checklist.md (800 lines)

**Reference Files (Progressive Loading):**
- epic-management.md (496 lines - Phase 1, 2)
- feature-decomposition-patterns.md (850 lines - Phase 3) **NEW**
- technical-assessment-guide.md (900 lines - Phase 4) **NEW**
- epic-validation-checklist.md (800 lines - Phase 7) **NEW**
- epic-template.md (265 lines - Phase 5)

**Subagents Used:**
- requirements-analyst (Phase 3 feature decomposition, optional Phase 6 requirements spec)
- architect-reviewer (Phase 4 technical assessment)

**Total Reference Content:** 3,311 lines (loaded progressively in isolated context)
```

---

#### 5.4: Update lean-orchestration-pattern.md

**File:** `/mnt/c/Projects/DevForgeAI2/devforgeai/protocols/lean-orchestration-pattern.md`

**Update command status table (lines 126-147):**

```markdown
| Command | Lines | Characters | Status | Priority |
|---------|-------|------------|--------|----------|
| qa | 295 | 7,205 | ✅ Compliant | ✅ Reference |
| **create-epic** | **~250** | **~8,000** | **✅ Compliant** | **✅ Reference** | ← UPDATE
| dev | 513 | 12,630 | ⚠️ High | ✅ Monitor |
| ideate | 410 | 11,717 | ⚠️ High | ✅ Monitor |
| **create-story** | 857 | **23,006** | ❌ **OVER** | 🔴 **URGENT** |
| **create-ui** | 614 | **18,908** | ❌ **OVER** | 🔴 **HIGH** |
| **release** | 655 | **18,166** | ❌ **OVER** | 🔴 **HIGH** |
| **orchestrate** | 599 | **15,012** | ❌ **OVER** | 🟡 **MEDIUM** |
| audit-deferrals | 452 | 13,088 | ⚠️ High | 🟡 Watch |
| create-context | 420 | 12,631 | ⚠️ High | 🟡 Watch |
| create-sprint | 496 | 12,602 | ⚠️ High | 🟡 Watch |

**Summary:**
- **4 commands OVER BUDGET** (down from 5) - Require immediate refactoring
- **5 commands HIGH USAGE** (up from 5) - Monitor and consider optimization
- **2 commands COMPLIANT** (up from 1) - Reference implementations (/qa, /create-epic) ← UPDATE
```

**Add create-epic as reference implementation:**

```markdown
### Reference Implementation: /create-epic (NEW)

**Command structure:**
- Phase 0: Argument validation (epic name)
- Phase 1: Context markers (**Epic name:**, **Command:** create-epic)
- Phase 2: Skill invocation (devforgeai-orchestration)
- Phase 3: Display results (skill summary output)
- Phase 4: Next steps guidance

**Metrics:**
- Lines: ~250 (down from 526)
- Characters: ~8,000 (down from 14,309)
- Budget usage: 53% (down from 95%)
- Token usage: ~2,000 (down from ~10,000)
- Reduction: 52% lines, 44% characters, 80% tokens

**Pattern compliance:**
- ✅ Zero business logic in command
- ✅ Single skill invocation
- ✅ Context isolation (125K tokens in skill/subagents)
- ✅ Framework-aware (references context files)
- ✅ Self-validating (epic-validation-checklist.md)

**Lessons learned:**
1. Extend existing skills (orchestration) rather than create new ones when overlap exists
2. Create reference files for subagent guidance (3 new files, 2,550 lines)
3. Progressive disclosure essential (load references only when phase needs)
4. Self-healing improves UX (validation-checklist enables auto-correction)
5. Framework integration prevents silos (validate against context files)
```

---

### STEP 6: Verification

**Post-refactoring checks:**

```bash
# 1. Character budget compliance
wc -c < .claude/commands/create-epic.md
# Expected: ~8,000 (53% of 15K limit)

# 2. Line count
wc -l < .claude/commands/create-epic.md
# Expected: ~250

# 3. Skill references exist
ls .claude/skills/devforgeai-orchestration/references/ | grep -E "(epic|feature|technical|validation)"
# Expected: 4 files (epic-management, feature-decomposition-patterns, technical-assessment-guide, epic-validation-checklist)

# 4. Pattern compliance check
grep -c "Skill(command=" .claude/commands/create-epic.md
# Expected: 1 (single skill invocation)

grep -c "Task(subagent_type=" .claude/commands/create-epic.md
# Expected: 0 (no direct subagent invocations - delegated to skill)

# 5. Documentation updated
grep -c "create-epic" .claude/memory/commands-reference.md
# Expected: Multiple (documentation present)
```

---

## Success Metrics

### Command Metrics

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Lines | 526 | ~250 | 52% reduction |
| Characters | 14,309 | ~8,000 | 44% reduction |
| Budget usage | 95% | 53% | 42 percentage points |
| Token usage (main) | ~10,000 | ~2,000 | 80% reduction |
| Business logic | 350+ lines | 0 lines | 100% removed |
| Skill invocations | 0 | 1 | ✅ Pattern compliant |
| Subagent invocations | 2 direct | 0 direct | ✅ Delegated to skill |

### Skill Metrics

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| Epic creation workflow | Placeholder | 7 phases | ✅ Implemented |
| Reference files | 2 | 5 | +3 (2,550 lines) |
| Lines (epic mode) | ~50 (placeholder) | ~300 | +250 lines |
| Context isolation | N/A | ✅ Yes | Token efficient |
| Self-validation | ❌ None | ✅ Yes | Quality improved |

### Framework Metrics

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Commands over budget | 5 | 4 | 1 fewer |
| Commands at warning | 5 | 6 | 1 more (moved from over) |
| Commands compliant | 1 | 2 | 1 more (/qa, /create-epic) |
| Lean pattern compliance | 4/11 (36%) | 5/11 (45%) | +9 percentage points |
| Reference implementations | 1 (/qa) | 2 (/qa, /create-epic) | Double |

---

## Risk Mitigation

### Risk 1: Epic Creation Mode Detection

**Risk:** Skill must reliably detect "create-epic" from context markers

**Mitigation:**
- Use explicit marker: `**Command:** create-epic`
- Proven pattern: /dev uses `**Story ID:**`, /qa uses `**Validation mode:**`
- Test detection in greenfield and brownfield scenarios

**Fallback:** If detection fails, skill can check for `**Epic name:**` marker as secondary signal

---

### Risk 2: Subagent Context Completeness

**Risk:** requirements-analyst and architect-reviewer need comprehensive epic context

**Mitigation:**
- Build detailed prompts with all Phase 2 gathered data
- Reference framework context files in prompts
- Load reference files before subagent invocation
- Proven pattern: /create-story successfully does this (Phase 2, 3)

**Validation:** Test subagent outputs have all required fields

---

### Risk 3: Reference File Loading Order

**Risk:** 5 reference files (3,311 lines) - loading wrong order wastes tokens

**Mitigation:**
- Progressive disclosure: Load only when phase needs
- Phase 3 → feature-decomposition-patterns.md
- Phase 4 → technical-assessment-guide.md
- Phase 7 → epic-validation-checklist.md
- Proven pattern: devforgeai-story-creation loads 6 reference files (7,477 lines)

**Monitoring:** Track token usage per phase, optimize if exceeds targets

---

### Risk 4: User Experience Change

**Risk:** Refactored command might feel different to users

**Mitigation:**
- Output format remains identical (same success report structure)
- Same AskUserQuestion flows (goal, timeline, stakeholders, features)
- Same files created (epic.md, requirements.md)
- Only difference: Faster execution, lower token usage (invisible to user)

**Testing:** Run both old and new commands, compare user-facing output

---

### Risk 5: Testing Coverage

**Risk:** New code paths need comprehensive testing

**Mitigation:**
- Test 5 scenarios (greenfield, brownfield, duplicate, requirements, budget)
- Validate each phase independently
- Test error conditions (invalid name, skill failure, validation HALT)

**Timeline:** ~2 hours of testing (acceptable)

---

## Timeline Estimate

### Phase 1: Reference Files Creation
**Duration:** ✅ COMPLETE (1-2 hours actual)
**Status:** 3 files created via agent-generator (2,550 lines total)

### Phase 2: Skill Enhancement
**Duration:** 2-3 hours
**Tasks:**
- Add epic creation workflow to orchestration skill (~300 lines)
- Implement 7 phases with reference loading
- Add mode detection logic
- Update skill documentation

### Phase 3: Command Refactoring
**Duration:** 1 hour
**Tasks:**
- Create lean command (~250 lines)
- Remove all business logic
- Add context markers and skill invocation
- Backup current command

### Phase 4: Testing
**Duration:** 2 hours
**Tasks:**
- Test 5 scenarios
- Validate character budget compliance
- Measure token usage
- Compare user experience

### Phase 5: Documentation
**Duration:** 1 hour
**Tasks:**
- Update CLAUDE.md
- Update commands-reference.md
- Update skills-reference.md
- Update lean-orchestration-pattern.md

**Total estimated time:** 6-8 hours (1 reference file phase complete)

---

## Rollout Strategy

### Phase 1: Preparation (COMPLETE ✅)
- ✅ Create refactoring plan
- ✅ Create 3 reference files via agent-generator
- ✅ Validate reference files exist

### Phase 2: Skill Enhancement (NEXT)
- ⏳ Enhance devforgeai-orchestration skill with epic creation
- ⏳ Test skill in isolation (without command)
- ⏳ Validate reference file loading

### Phase 3: Command Refactoring
- ⏳ Backup current command (create-epic.md.backup)
- ⏳ Create lean command following /qa pattern
- ⏳ Test command + skill integration

### Phase 4: Validation
- ⏳ Run 5 test scenarios
- ⏳ Measure metrics (lines, characters, tokens)
- ⏳ Validate user experience unchanged

### Phase 5: Documentation
- ⏳ Update 4 documentation files
- ⏳ Update protocol with new reference implementation

### Phase 6: Deployment
- ⏳ Replace old command with new command
- ⏳ Archive backup for reference
- ⏳ Mark refactoring complete

---

## Acceptance Criteria

- [x] **Reference files created** (3 files, 2,550 lines) ✅ COMPLETE
- [ ] **Skill enhanced** with epic creation (7 phases, ~300 lines)
- [ ] **Command refactored** to lean pattern (~250 lines, ~8K chars)
- [ ] **Character budget compliant** (<8,000 chars, 53% of limit)
- [ ] **Token efficiency achieved** (<2,000 tokens in main conversation)
- [ ] **All 5 test scenarios pass** (greenfield, brownfield, duplicate, requirements, budget)
- [ ] **Documentation updated** (4 files: CLAUDE.md, commands-reference.md, skills-reference.md, lean-orchestration-pattern.md)
- [ ] **User experience preserved** (same output format, same UX flows)
- [ ] **Pattern compliance validated** (zero business logic in command, single skill invocation)

---

## Post-Refactoring State

### Framework Command Budget Status (Projected)

**After /create-epic refactoring:**

| Status | Count | Commands |
|--------|-------|----------|
| ❌ **OVER BUDGET** | 4 | create-story (153%), create-ui (126%), release (121%), orchestrate (100%) |
| ⚠️ **HIGH USAGE** | 6 | create-epic (53% - **MOVED**), create-context (84%), create-sprint (84%), dev (84%), audit-deferrals (87%) |
| ✅ **COMPLIANT** | 2 | qa (48%), audit-budget (66%), create-epic (53% - **ADDED**) |

**Progress toward full compliance:**
- Compliant: 1 → 2 commands (+100% increase)
- Over budget: 5 → 4 commands (-20% decrease)
- Lean pattern adoption: 36% → 45% (+9 percentage points)

**Reference implementations:** /qa (48%), /create-epic (53% - **NEW**)

### Next Refactoring Targets (Priority Order)

**🔴 URGENT:**
1. /create-story (23,006 chars - 153% - **CRITICAL**)
2. /create-ui (18,908 chars - 126%)

**🟡 HIGH:**
3. /release (18,166 chars - 121%)
4. /orchestrate (15,012 chars - 100%)

**Pattern:** Apply same lean orchestration pattern proven in /dev, /qa, /ideate, /create-story, /create-epic

---

## Conclusion

This implementation plan provides a complete roadmap for refactoring `/create-epic` to follow the lean orchestration pattern. The refactoring:

1. **Restores skills-first architecture** - Business logic moves to devforgeai-orchestration skill
2. **Achieves budget compliance** - 95% → 53% (42 percentage point improvement)
3. **Improves token efficiency** - 80% reduction in main conversation tokens
4. **Leverages existing infrastructure** - Orchestration skill already has epic capabilities
5. **Adds framework-aware guidance** - 3 new reference files (2,550 lines) for subagent integration
6. **Uses existing subagents** - requirements-analyst, architect-reviewer (no new subagents needed)
7. **Enables self-validation** - epic-validation-checklist.md for quality assurance
8. **Maintains user experience** - Same epic creation flow, just more efficient

**Status:** ✅ READY FOR IMPLEMENTATION
**Risk:** 🟢 LOW (pattern proven in 4 previous refactorings)
**Estimated effort:** 6-8 hours (Phase 1 complete - 1-2 hours done)

**Recommendation:** PROCEED with implementation following this plan.

---

## Appendix A: Files Created

### Reference Files (via agent-generator)

1. **feature-decomposition-patterns.md**
   - Location: `.claude/skills/devforgeai-orchestration/references/`
   - Size: 850 lines, 26K
   - Purpose: Guide requirements-analyst for epic → feature breakdown
   - Patterns: CRUD, Auth, API, Reporting, Workflow, E-commerce
   - Framework integration: References tech-stack.md, architecture-constraints.md

2. **technical-assessment-guide.md**
   - Location: `.claude/skills/devforgeai-orchestration/references/`
   - Size: 900 lines, 28K
   - Purpose: Guide architect-reviewer for complexity scoring
   - Rubric: 0-10 scale with detailed criteria
   - Framework integration: Validates against all 6 context files

3. **epic-validation-checklist.md**
   - Location: `.claude/skills/devforgeai-orchestration/references/`
   - Size: 800 lines, 21K
   - Purpose: Self-validation before epic completion
   - Self-healing: Auto-corrects missing IDs, dates, default values
   - Framework integration: Enforces context file compliance

**Total new content:** 2,550 lines, 75K
**Progressive loading:** Only loaded when workflow phase needs
**Context isolation:** All in skill/subagent contexts (minimal main conversation impact)

### Planning Documents

4. **create-epic-refactoring-plan.md**
   - Location: `devforgeai/protocols/`
   - Size: Analysis and planning document
   - Purpose: Initial refactoring analysis

5. **CREATE-EPIC-REFACTORING-IMPLEMENTATION-PLAN.md**
   - Location: `devforgeai/protocols/`
   - Size: Complete implementation guide
   - Purpose: Step-by-step refactoring instructions

---

## Appendix B: Subagent Analysis

### No New Subagents Needed

**Analysis conclusion:** Existing subagents are sufficient:

**requirements-analyst (473 lines - EXISTS)**
- ✅ Handles epic feature decomposition (Phase 3)
- ✅ Handles requirements specification (Phase 6 optional)
- ✅ Framework-aware (references context files)
- ✅ No enhancement needed

**architect-reviewer (528 lines - EXISTS)**
- ✅ Handles technical complexity assessment (Phase 4)
- ✅ Validates against tech-stack.md, architecture-constraints.md
- ✅ Identifies risks and prerequisites
- ✅ No enhancement needed (replaces non-existent technical-assessor)

**technical-assessor (DOES NOT EXIST)**
- ❌ Referenced in current /create-epic command (lines 137-158)
- ✅ Replaced by architect-reviewer subagent
- Action: Update command to use architect-reviewer

### Why No New Subagents

**Question:** Do we need epic-result-interpreter (like qa-result-interpreter)?

**Answer:** ❌ NO

**Reasoning:**
- Epic output is simple structured data (YAML frontmatter + features list)
- No complex report parsing required (unlike QA reports)
- Command can display skill summary directly
- No benefit from additional interpretation layer

**Pattern difference:**
- QA reports: Complex (violations, coverage, anti-patterns) → interpreter needed
- Epic output: Simple (ID, features, assessment) → direct display sufficient

---

## Appendix C: Reference to Proven Patterns

### Pattern 1: /qa Command (Reference Implementation)

**Before refactoring:** 692 lines, 31K chars (206% budget)
**After refactoring:** 295 lines, 7K chars (48% budget)
**Reduction:** 57% lines, 77% characters

**Pattern applied:**
1. Argument validation (story ID, mode)
2. Context markers (**Story ID:**, **Validation mode:**)
3. Skill invocation (devforgeai-qa)
4. Display results (from qa-result-interpreter subagent)

**Result:** ✅ Character budget compliant, 74% token reduction

---

### Pattern 2: /dev Command (Reference Implementation)

**Before refactoring:** 860 lines, ~30K chars (200% budget)
**After refactoring:** 513 lines, 12.6K chars (84% budget)
**Reduction:** 40% lines, 58% characters

**Pattern applied:**
1. Argument validation (story ID)
2. Story file loading via @file
3. Context markers (**Story ID:**)
4. Skill invocation (devforgeai-development)
5. Results reporting

**Result:** ⚠️ High usage but within budget, 67% token reduction

---

### Pattern 3: /ideate Command (Reference Implementation)

**Before refactoring:** 463 lines, 15.3K chars (102% budget)
**After refactoring:** 410 lines, 11.7K chars (78% budget)
**Reduction:** 11% lines, 24% characters

**Pattern applied:**
1. Argument validation (business idea)
2. Skill invocation (devforgeai-ideation)
3. Artifact verification
4. Brief completion confirmation
5. Next steps guidance (defers to skill)

**Result:** ⚠️ High usage but within budget, 24% token reduction

---

### Pattern 4: /create-story Command (Reference Implementation)

**Before refactoring:** 857 lines, 23K chars (153% budget) - ❌ STILL OVER
**After refactoring:** 500 lines, 14.2K chars (95% budget) - ⚠️ HIGH
**Reduction:** 42% lines, 38% characters

**Pattern applied:**
1. Argument validation (feature description)
2. Skill invocation (devforgeai-story-creation)
3. Story file verification
4. Brief confirmation
5. Next steps guidance

**Result:** ⚠️ Still high usage, needs further optimization, but 38% token reduction achieved

---

**Conclusion:** Lean orchestration pattern proven across 4 commands with 11-80% token reductions. Same pattern applies to /create-epic with high confidence of success.
