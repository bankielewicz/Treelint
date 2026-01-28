# Architecture Decision Records (ADRs)

## Overview

ADRs document significant architectural decisions with context, rationale, and consequences. They provide an audit trail of technical choices and ensure future developers understand why decisions were made.

---

## When ADRs Are Required

### Technology Decisions
- Framework selection (React, Vue, Angular)
- ORM selection (Entity Framework, Dapper, Prisma)
- State management patterns
- Database selection
- Testing framework choices

### Architectural Patterns
- Clean Architecture vs N-Tier
- CQRS implementation
- Domain-Driven Design patterns
- Layer boundary definitions
- Service communication patterns

### Deferrals Requiring ADR Approval ⭐ NEW (RCA-007)

An ADR MUST be created when deferring Definition of Done items IF any of these conditions apply:

#### 1. **Scope Change (Descoping Original Work)**

**Condition:** DoD item was part of original story scope but is being descoped

**Example:**
- STORY-004 JSON output includes exit codes (in technical specification)
- Exit codes being removed from this story's scope
- **ADR Required:** Justify why original scope is being changed

**Template:**
```markdown
ADR-XXX: Descoping Exit Code Handling from STORY-004

## Context
Exit codes were in original specification (lines 270-300) but cannot be implemented in STORY-004.

## Decision
Descope exit code handling from STORY-004 to STORY-006 (main.rs integration).

## Rationale
- Technical reason: [blocker, dependency]
- Business reason: [scope clarity, separation of concerns]

## Consequences
- STORY-004 incomplete (1 DoD item deferred)
- STORY-006 scope includes exit code integration
- Creates dependency: STORY-004 → STORY-006

## Alternatives
- Implement now: [why rejected]
- Create separate story: [why rejected]
```

**When NOT Required:**
- DoD item was never in original scope (out of scope from start)
- Requirement changed BEFORE implementation (documented in story updates)

#### 2. **Architectural Impact**

**Condition:** Deferred work affects system architecture, API contracts, or data structures

**Examples:**
- API endpoints (contract changes)
- Data structures (schema changes)
- Error handling patterns (framework-level decisions)
- Authentication/authorization (security architecture)

**ADR Required:** Document architectural implications of deferral

**Rationale:** Architectural decisions need review and approval, can't be deferred silently

#### 3. **Multi-Story Deferral (Deferral Chain)**

**Condition:** Work is deferred more than once (creates chain A→B→C)

**Example:**
- STORY-004 defers exit codes to STORY-005
- STORY-005 defers exit codes to STORY-006
- **ADR Required:** Document why work spans 3+ stories

**Rationale:**
- Multi-level deferrals create risk of lost work (RCA-007 evidence)
- Each hop increases probability work is forgotten
- Chains >1 level indicate systemic issue (scope creep, poor planning, or requirements change)

**ADR Must Explain:**
- Why work couldn't be completed in original story
- Why work couldn't be completed in first deferral target
- What guarantees work will be completed in final target
- Whether this indicates epic scope issue

#### 4. **Cross-Epic Deferral**

**Condition:** Work is deferred to a story in a different epic

**Example:**
- STORY-004 (EPIC-001: Core CLI) defers work to STORY-125 (EPIC-002: Performance)

**ADR Required:** Justify why work moves across epic boundaries

**Rationale:**
- Epic scope changes affect roadmap planning
- Cross-epic dependencies create coordination complexity
- May indicate mis-categorization of original story

---

## Deferrals NOT Requiring ADR

Valid without ADR if ALL requirements met:

### 1. **External Blocker**

**Pattern:** "Blocked by {external_system}: {specific_reason with ETA}"

**Examples:**
- ✅ "Blocked by Payment API v2 (available 2025-12-01)"
- ✅ "Blocked by tree-sitter 0.21 release (planned Q2 2025)"
- ✅ "Blocked by third-party OAuth service (ETA: January 2026)"

**Requirements:**
- [ ] Blocker is EXTERNAL (not internal code, not internal decision)
- [ ] ETA or resolution date provided (concrete timeline)
- [ ] Specific external system named (not vague)

**No ADR Required:** External blockers are legitimate technical dependencies

**Validation:** deferral-validator checks blocker is external (not "our code", "our API", "our module")

### 2. **Technical Dependency (Story Split - Single Hop Only)**

**Pattern:** "Deferred to STORY-XXX: {justification}"

**Examples:**
- ✅ "Deferred to STORY-125: Requires parser integration from STORY-124"
- ✅ "Deferred to STORY-006: Will be implemented in main.rs integration story"

**Requirements:**
- [ ] Referenced story (STORY-XXX) EXISTS
- [ ] Referenced story INCLUDES deferred work (in acceptance criteria or technical spec)
- [ ] **NO deferral chain** (STORY-XXX doesn't also defer this work) ← RCA-007 CRITICAL
- [ ] Work is within original scope (if descoped, ADR required)

**No ADR Required IF:**
- Work within original scope (story split for size, not scope change)
- Single hop only (A→B allowed, A→B→C prohibited without ADR)

**Validation:** deferral-validator checks story exists, includes work, and detects chains

### 3. **Version Deferral**

**Pattern:** Deferred to future version (v1.1, v2.0, etc.)

**Example:**
- ✅ "Deferred to v2.0: Performance benchmarks (documented in tech-stack.md)"

**Requirements:**
- [ ] Already covered by tech-stack.md "Out of Scope" section
- [ ] Must reference tech-stack.md section

**No ADR Required:** Already documented in context files

---

## ADR Numbering Convention

ADRs are numbered sequentially starting from ADR-001:
- `ADR-001-database-selection.md`
- `ADR-002-orm-selection.md`
- `ADR-003-state-management.md`
- ...

**Next ADR Number:** Check existing ADRs in this directory, use next sequential number

---

## ADR Template

**Location:** `.claude/skills/devforgeai-architecture/assets/adr-examples/`

**Required Sections:**
1. **Context** - Why this decision is needed, what changed
2. **Decision** - What was decided
3. **Rationale** - Why this choice was made
4. **Consequences** - Positive and negative impacts
5. **Alternatives Considered** - What was rejected and why
6. **Related Documents** - Links to stories, specs, other ADRs

**Example Template:**
See `ADR-EXAMPLE-006-scope-descope.md` for deferral-specific ADR template

---

## Deferral-Specific ADR Guidelines

### When Documenting Scope Changes

**Must Answer:**
1. What was in original scope? (reference story acceptance criteria)
2. Why can't it be completed now? (technical blocker, dependency, discovery)
3. Where is work going? (STORY-XXX, future version, cancelled)
4. When will it be completed? (sprint, release, never)
5. Who approved this change? (product owner, architect, team lead)

**Must Include:**
- Reference to original story (STORY-XXX)
- Quote from original acceptance criteria or technical spec
- Technical justification (not just "ran out of time")
- Impact assessment (what doesn't work without this)
- Follow-up plan (specific story ID or version number)

### When Documenting Multi-Level Deferrals

**If work deferred >1 time (A→B→C), ADR must explain:**
1. Why couldn't original story (A) complete it?
2. Why couldn't first deferral target (B) complete it?
3. What's different about final target (C) that enables completion?
4. Is this a symptom of poor planning or genuine technical evolution?

**Red Flags:**
- Vague reasons at each hop ("better fit", "makes more sense")
- No technical justification (just organizational preference)
- Epic scope creep (work keeps moving forward)

---

## Usage

### By Developers
- Reference when deciding if ADR needed for deferral
- Use template when creating deferral-related ADRs
- Check existing ADRs before creating duplicate

### By QA Skill
- Validate ADR references in deferrals
- Check ADR documents the specific deferred work
- Verify ADR created within reasonable timeframe

### By deferral-validator Subagent
- Check ADR exists when scope change detected
- Validate ADR references in substep 5
- Return MEDIUM violation if ADR missing for scope change

---

## ADR Review Process

**When Created:**
1. Developer creates ADR (during deferral decision or later)
2. Architect reviews ADR (validates technical justification)
3. Product owner approves (validates business impact)
4. ADR stored in `devforgeai/specs/adrs/`

**Annual Review:**
- Review all ADRs for relevance
- Mark superseded ADRs
- Update with lessons learned
- Archive obsolete decisions

---

## Quick Reference

### Deferral Type → ADR Required?

| Deferral Type | Pattern | ADR Required? | Reason |
|---------------|---------|---------------|--------|
| **External Blocker** | "Blocked by {external}: {ETA}" | **No** | Legitimate technical dependency |
| **Story Split (Single Hop)** | "Deferred to STORY-XXX: {reason}" | **No** (if in scope) | Valid story decomposition |
| **Story Split (Multi-Hop)** | A→B→C | **YES** | High risk, needs justification |
| **Scope Change** | "Out of scope" OR defer what was in spec | **YES** | Changing agreed scope |
| **Architectural Impact** | API, data structures, patterns | **YES** | Architecture decisions need review |
| **Cross-Epic Deferral** | Different epic | **YES** | Epic scope implications |
| **Version Deferral** | "Deferred to v2.0" | **No** (if in tech-stack.md) | Already documented |

---

## Examples

### Example 1: Scope Change ADR (Required)

**Scenario:** STORY-004 JSON output originally included exit codes, now being descoped

**ADR Required:** YES - In original spec, being removed = scope change

**ADR-007: Descoping Exit Code Handling from STORY-004**
- Context: Exit codes in original spec (lines 270-300)
- Decision: Move to STORY-006
- Rationale: Better separation (main.rs integration separate concern)
- Consequence: STORY-004 incomplete, dependency on STORY-006

### Example 2: External Blocker (No ADR)

**Scenario:** Database migration blocked by DBA approval process

**ADR Required:** NO - External blocker with ETA

**Deferral:** "Blocked by DBA approval process (ETA: 2025-11-15)"

### Example 3: Multi-Level Chain (ADR Required)

**Scenario:** STORY-004 → STORY-005 → STORY-006 (exit code handling)

**ADR Required:** YES - Work deferred twice creates broken chain

**ADR-008: Multi-Story Exit Code Implementation Plan**
- Context: Exit codes deferred twice, work at risk of being lost
- Decision: Explicitly assign to STORY-006 with clear scope
- Rationale: Prevents work from disappearing in chain
- Consequence: STORY-006 must include exit codes (not negotiable)

---

**Created:** 2025-11-03
**Source:** RCA-007 Recommendation 2
**Version:** 1.0
