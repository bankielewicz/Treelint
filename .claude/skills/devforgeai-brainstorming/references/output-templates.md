---
id: output-templates
title: Output Document Templates for Brainstorming Sessions
version: "1.0"
created: 2025-12-21
status: Published
---

# Output Document Templates

Templates for brainstorm output documents and display formats.

## Overview

| Template | Purpose | Format |
|----------|---------|--------|
| Brainstorm Document | Main output file | Markdown with YAML frontmatter |
| Session Displays | Phase transition messages | ASCII box format |
| Next Steps | End-of-session guidance | Structured list |
| Checkpoint Display | Session save confirmation | ASCII box format |

---

## Section 1: Brainstorm Document Template

### 1.1 Complete Document Structure

```markdown
---
# DevForgeAI Brainstorm Session
id: BRAINSTORM-{NNN}
title: "{Descriptive Title}"
status: Complete
created: {YYYY-MM-DD}
facilitator: DevForgeAI
session_duration: "{duration} minutes"
question_count: {total questions asked}

# Core Outputs (AI-Consumable for /ideate)
problem_statement: "{One sentence problem statement}"
target_outcome: "{Desired future state}"
recommended_approach: "{High-level solution direction}"
confidence_level: "{High|Medium|Low}"

# Stakeholder Summary
primary_stakeholder: "{Main decision maker}"
user_personas:
  - "{Persona 1: Role - Primary goal}"
  - "{Persona 2: Role - Primary goal}"

# Constraint Summary
budget_range: "{$X - $Y}"
timeline: "{Target delivery date}"
hard_constraints:
  - "{Constraint 1}"
  - "{Constraint 2}"

# Hypothesis Summary
critical_assumptions:
  - "{Assumption 1}"
  - "{Assumption 2}"

# Prioritization Summary
must_have_capabilities:
  - "{Capability 1}"
  - "{Capability 2}"
nice_to_have:
  - "{Capability 3}"
---

# {Brainstorm Title}

## Key Files for Context (Optional)

{If brainstorm references framework-specific files:}

| Component | File Path | Purpose |
|-----------|-----------|---------|
| {Component 1} | `{full/path/to/file.md}` | {What it does} |
| {Component 2} | `{full/path/to/file.md}` | {What it does} |

---

## Glossary (Optional)

{If brainstorm uses framework-specific terminology:}

| Term | Definition |
|------|------------|
| {Term 1} | {Clear definition for someone without context} |
| {Term 2} | {Clear definition for someone without context} |

> **Note:** Include "Key Files for Context" when the brainstorm references:
> - Files in `.claude/` directory (skills, agents, commands)
> - Files in `devforgeai/` directory (specs, context, workflows)
> - Configuration files (hooks.yaml, settings.json, etc.)
>
> Include "Glossary" when the brainstorm uses:
> - Phase numbers (Phase 01, Phase 09, etc.)
> - DevForgeAI-specific terms (subagent, skill, context file, quality gate, etc.)
> - Workflow terminology (TDD, DoD, AC, preflight, etc.)

---

## Executive Summary

{2-3 paragraph summary of brainstorming outcomes suitable for executive presentation.
Include the core problem, key stakeholders, recommended approach, and expected outcomes.}

---

## 1. Stakeholder Analysis

### 1.1 Stakeholder Map

| Category | Stakeholder | Role | Influence |
|----------|-------------|------|-----------|
| Primary | {Name/Role} | {Decision authority} | HIGH |
| Secondary | {Name/Role} | {User/Beneficiary} | MEDIUM |
| Tertiary | {Name/Role} | {Affected party} | LOW |

### 1.2 Stakeholder Goals & Concerns

**Primary Stakeholders:**

| Stakeholder | Goals | Concerns |
|-------------|-------|----------|
| {Name} | {Goal 1}, {Goal 2} | {Concern 1}, {Concern 2} |

**Secondary Stakeholders:**

| Stakeholder | Goals | Concerns |
|-------------|-------|----------|
| {Name} | {Goal 1} | {Concern 1} |

### 1.3 Identified Conflicts

| Stakeholders | Conflict | Resolution Approach |
|--------------|----------|---------------------|
| {A} vs {B} | {Nature of conflict} | {Proposed resolution} |

---

## 2. Problem Analysis

### 2.1 Problem Statement

> {Clear, one-sentence problem statement}
>
> Example: "Operations team spends 45 minutes per order on manual data entry,
> limiting throughput to 80 orders/day and causing 8% error rate."

### 2.2 Root Cause Analysis (5 Whys)

| Level | Question | Answer |
|-------|----------|--------|
| 1 | Why does {problem} happen? | {Answer 1} |
| 2 | Why does {Answer 1} happen? | {Answer 2} |
| 3 | Why does {Answer 2} happen? | {Answer 3} |
| 4 | Why does {Answer 3} happen? | {Answer 4} |
| 5 | Why does {Answer 4} happen? | {Root Cause} |

**Root Cause:** {Final root cause statement}

### 2.3 Current State Assessment

**Process Type:** {Manual | Semi-Automated | Automated but Broken | None}

| Metric | Current Value | Target Value |
|--------|---------------|--------------|
| Process time | {X minutes} | {Y minutes} |
| Error rate | {X%} | {Y%} |
| Throughput | {X units/day} | {Y units/day} |

**Bottlenecks:**
1. {Bottleneck 1}
2. {Bottleneck 2}

### 2.4 Pain Point Inventory

| Pain Point | Business Impact | Severity | Est. Cost |
|------------|-----------------|----------|-----------|
| {Pain 1} | {Impact} | CRITICAL | {$/month} |
| {Pain 2} | {Impact} | HIGH | {$/month} |
| {Pain 3} | {Impact} | MEDIUM | {$/month} |

### 2.5 Failed Solution History

| Previous Attempt | What Happened | Lessons Learned |
|------------------|---------------|-----------------|
| {Solution tried} | {Why it failed} | {What to avoid} |

---

## 3. Opportunity Canvas

### 3.1 Blue-Sky Vision

**Ideal Future State:**
{Description of what success looks like if problem is fully solved}

**Key Outcomes:**
- {Outcome 1}
- {Outcome 2}
- {Outcome 3}

### 3.2 Market Research

{If research was conducted:}

**Competitor Approaches:**
| Competitor | Approach | Pros | Cons |
|------------|----------|------|------|
| {Name} | {How they solve it} | {Benefits} | {Drawbacks} |

**Technology Trends:**
- {Trend 1}: {Relevance}
- {Trend 2}: {Relevance}

{If research was skipped:}
*Market research was not conducted during this session.*

### 3.3 Identified Opportunities

| ID | Opportunity | Description | Potential Impact |
|----|-------------|-------------|------------------|
| OPP-1 | {Name} | {What could be done} | {Expected benefit} |
| OPP-2 | {Name} | {What could be done} | {Expected benefit} |

### 3.4 Adjacent Opportunities

| Related Problem | Connection | Synergy |
|-----------------|------------|---------|
| {Problem} | {How related} | {Benefit of solving together} |

---

## 4. Constraint Matrix

### 4.1 Budget Constraints

| Aspect | Constraint | Flexibility |
|--------|------------|-------------|
| Initial investment | {$X - $Y} | {Fixed | Negotiable} |
| Ongoing costs | {$/month} | {Fixed | Negotiable} |
| ROI expectation | {X months} | {Requirement | Target} |

### 4.2 Timeline Constraints

| Milestone | Date | Flexibility |
|-----------|------|-------------|
| Decision deadline | {Date} | {Fixed | Negotiable} |
| Project start | {Date} | {Fixed | Negotiable} |
| MVP delivery | {Date} | {Fixed | Negotiable} |
| Full rollout | {Date} | {Fixed | Negotiable} |

### 4.3 Resource Constraints

| Resource | Available | Gap |
|----------|-----------|-----|
| Team size | {X people} | {None | Need Y more} |
| Skills | {List} | {Missing skills} |
| Tools | {List} | {Needed tools} |

### 4.4 Technical Constraints

- [ ] Must integrate with: {System 1}, {System 2}
- [ ] Must use: {Required technology}
- [ ] Must comply with: {Security/compliance standard}
- [ ] Must deploy: {On-premise | Cloud | Hybrid}

### 4.5 Organizational Constraints

- [ ] Requires approval from: {Role/Person}
- [ ] Union/labor considerations: {Yes/No - details}
- [ ] Change resistance expected: {High | Medium | Low}
- [ ] Regulatory requirements: {List}

---

## 5. Hypothesis Register

| ID | Hypothesis | Success Criteria | Validation | Risk |
|----|------------|------------------|------------|------|
| H1 | IF {condition}, THEN {outcome} | {Measurable} | {Method} | {Level} |
| H2 | IF {condition}, THEN {outcome} | {Measurable} | {Method} | {Level} |

### Validation Priority

**Must Validate First:**
1. {H1}: {Reason - blocking}

**Can Validate During Development:**
- {H2}

---

## 6. Prioritized Opportunities

### 6.1 MoSCoW Classification

**Must Have (Critical):**
- {Capability 1}
- {Capability 2}

**Should Have (Important):**
- {Capability 3}

**Could Have (Nice to Have):**
- {Capability 4}

**Won't Have (Out of Scope):**
- {Capability 5}

### 6.2 Impact-Effort Matrix

```
                    HIGH EFFORT
                         |
    Major Projects       |       Avoid
    {list}               |       {list}
                         |
HIGH IMPACT -------------|------------- LOW IMPACT
                         |
    Quick Wins           |       Fill-ins
    {list}               |       {list}
                         |
                    LOW EFFORT
```

### 6.3 Recommended Sequence

1. **First:** {Item} - {Rationale}
2. **Second:** {Item} - {Rationale}
3. **Third:** {Item} - {Rationale}

---

## 7. Handoff to Ideation

### 7.1 Summary for /ideate

This brainstorm session has produced inputs for ideation:

**Problem to Solve:**
> {problem_statement from frontmatter}

**Primary Users:**
{user_personas from frontmatter}

**Success Looks Like:**
> {target_outcome from frontmatter}

**Key Constraints:**
{hard_constraints from frontmatter}

**Must-Have Capabilities:**
{must_have_capabilities from frontmatter}

### 7.2 Recommended Next Steps

1. **Review this document** - Verify accuracy with stakeholders
2. **Run /ideate** - Transform into formal requirements
   - This brainstorm will be auto-detected
   - Core inputs will be pre-populated
3. **After ideation** - Run /create-context for architecture

### 7.3 Open Questions for Ideation

| Question | Context | Who Can Answer |
|----------|---------|----------------|
| {Question} | {Why it matters} | {Stakeholder} |

---

## Appendix A: Session Metadata

- **Brainstorm ID:** BRAINSTORM-{NNN}
- **Created:** {YYYY-MM-DD}
- **Duration:** {X} minutes
- **Questions Asked:** {X}
- **Phases Completed:** 7/7
- **Facilitator:** DevForgeAI
- **Confidence Level:** {High|Medium|Low}
- **Research Conducted:** {Yes|No}

---

## Appendix B: Raw Session Data

<details>
<summary>Click to expand raw session responses</summary>

### Phase 1 Responses
{Raw Q&A from stakeholder discovery}

### Phase 2 Responses
{Raw Q&A from problem exploration}

### Phase 3 Responses
{Raw Q&A from opportunity mapping}

### Phase 4 Responses
{Raw Q&A from constraint discovery}

### Phase 5 Responses
{Raw Q&A from hypothesis formation}

### Phase 6 Responses
{Raw Q&A from prioritization}

</details>
```

---

## Section 2: Session Display Templates

### 2.1 Session Start

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  DevForgeAI Brainstorming Session
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Session ID: {brainstorm_id}
Mode: {New | Resume}

This session will guide you through 7 phases of discovery:
  1. Stakeholder Discovery - Who is involved?
  2. Problem Exploration - What's the actual problem?
  3. Opportunity Mapping - What's possible?
  4. Constraint Discovery - What limits us?
  5. Hypothesis Formation - What do we assume?
  6. Prioritization - What matters most?
  7. Handoff Synthesis - Package for ideation

Estimated duration: 20-45 minutes
Questions: 30-50 interactive prompts

Let's begin...
```

### 2.2 Phase Transition

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase {N} Complete: {Phase Name}
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

{Phase-specific summary}
  - {Key output 1}
  - {Key output 2}

Proceeding to Phase {N+1}: {Next Phase Name}...
```

### 2.3 Session Complete

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Brainstorm Session Complete
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Document: {output_path}

Summary:
  - Stakeholders: {count} identified ({primary_count} primary)
  - Problem: {problem_statement_short}
  - Opportunities: {count} candidates
  - Constraints: {constraint_summary}
  - Hypotheses: {count} to validate

Confidence: {confidence_level}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Next Steps
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Review brainstorm document for accuracy
2. Run /ideate to transform into formal requirements
   - The brainstorm will be automatically detected
   - Key inputs will be pre-populated
3. After ideation: /create-context for architecture

Recommended command:
  /ideate
```

### 2.4 Checkpoint Created

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Brainstorm Session Checkpointed
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Progress: {percent}% complete (Phases 1-{last} of 7)
Checkpoint: {checkpoint_path}

Completed:
  ✓ Phase 1: Stakeholder Discovery
  ✓ Phase 2: Problem Exploration
  ✓ Phase 3: Opportunity Mapping

Remaining:
  ○ Phase 4: Constraint Discovery (in progress)
  ○ Phase 5: Hypothesis Formation
  ○ Phase 6: Prioritization
  ○ Phase 7: Handoff Synthesis

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Resume Instructions
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

In your next Claude session, run:
  /brainstorm --resume {brainstorm_id}

Your progress has been saved. The session will continue
from Phase 4 (Constraint Discovery).
```

### 2.5 Session Resumed

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Session Restored
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Brainstorm: {brainstorm_id}
Progress: {percent}% complete

Completed Phases:
  ✓ Phase 1: Stakeholder Discovery
  ✓ Phase 2: Problem Exploration
  ✓ Phase 3: Opportunity Mapping

Resuming: Phase 4 (Constraint Discovery)
```

---

## Section 3: YAML Frontmatter Values

### 3.1 Required Fields

| Field | Type | Example |
|-------|------|---------|
| `id` | string | "BRAINSTORM-001" |
| `title` | string | "Order Processing Automation" |
| `status` | enum | "Complete" |
| `created` | date | "2025-12-21" |
| `problem_statement` | string | "Operations team spends..." |
| `confidence_level` | enum | "High", "Medium", "Low" |

### 3.2 Stakeholder Fields

| Field | Type | Example |
|-------|------|---------|
| `primary_stakeholder` | string | "VP Operations" |
| `user_personas` | list | ["Clerk: Data entry - Speed", ...] |

### 3.3 Constraint Fields

| Field | Type | Example |
|-------|------|---------|
| `budget_range` | string | "$50K - $200K" |
| `timeline` | string | "Q2 2025" |
| `hard_constraints` | list | ["Must integrate with SAP", ...] |

### 3.4 Priority Fields

| Field | Type | Example |
|-------|------|---------|
| `must_have_capabilities` | list | ["API integration", ...] |
| `nice_to_have` | list | ["Mobile app", ...] |
| `critical_assumptions` | list | ["Users will adopt", ...] |

---

## Section 4: Confidence Level Calculation

### 4.1 Factors

| Factor | Criteria | Weight |
|--------|----------|--------|
| Stakeholders | 1+ primary, 1+ secondary | 20% |
| Root Cause | Depth >= 3 levels | 20% |
| Constraints | Budget + timeline defined | 20% |
| Hypotheses | At least 1 testable | 20% |
| Priorities | 1-5 must-haves | 20% |

### 4.2 Level Thresholds

| Level | Criteria |
|-------|----------|
| HIGH | 4+ factors met |
| MEDIUM | 2-3 factors met |
| LOW | 0-1 factors met |

---

## Section 5: Artifact Generation Display Templates

### 5.1 Artifact Generation Prompt

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Project Initialization
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Your brainstorm document has been created successfully.

Would you like to generate initial project files?

These files help future Claude sessions understand your project:

  README.md  - Project overview for humans/GitHub
  CLAUDE.md  - AI context differentiating project vs framework
  .gitignore - Standard ignore patterns for version control
```

### 5.2 Artifact Created Confirmation

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Project Files Created
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✓ README.md created at project root
  - Problem statement from brainstorm
  - MVP features from MoSCoW priorities
  - Post-MVP roadmap

✓ CLAUDE.md created at project root
  - Project vs Framework distinction
  - DevForgeAI workflow reference
  - Context files guidance

✓ .gitignore created at project root
  - Standard development ignores
  - DevForgeAI artifact patterns

These files are initial drafts. You can customize them
after running /ideate and /create-context.
```

### 5.3 Conflict Resolution Display

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  File Conflict Detected
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

{filename} already exists at project root.

Options:
  1. Overwrite - Replace with brainstorm-generated version
  2. Create {filename}-brainstorm - Keep both versions
  3. Skip - Keep existing file unchanged

Note: Existing file contents will be preserved if you
choose option 2 or 3.
```

### 5.4 Skip Confirmation

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Artifact Generation Skipped
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

You chose to skip project file generation.

You can manually create these files later using:
  - The brainstorm document as reference
  - Templates at: .claude/skills/devforgeai-brainstorming/assets/templates/

Proceeding to session validation...
```

### 5.5 Partial Generation Display

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Project Files Created (Partial)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

{#each created_files}
✓ {this.name} created at project root
{/each}

{#each skipped_files}
○ {this.name} skipped ({this.reason})
{/each}

Proceeding to session validation...
```

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
