# ADR-006: [Story ID] - Scope Change - [Brief Title]

**Status:** Accepted
**Date:** [YYYY-MM-DD]
**Deciders:** [Team/Person]
**Relates to:** [Story ID]

---

## Context

**Original Scope:**
Story [STORY-ID] included the following Definition of Done item:
- "[ ] [DoD item description]"

This item was part of the original story acceptance criteria and technical specification.

**Implementation Reality:**
During development, it became apparent that:
- [Describe what changed during implementation]
- [Why the DoD item can't/shouldn't be completed in this story]
- [What technical or business factors emerged]

**Current Situation:**
- Story has been implemented except for [DoD item]
- [DoD item] needs to be deferred to [future story/epic/release]
- This constitutes a scope change requiring documentation

---

## Decision

**Descope [DoD item] from [STORY-ID]**

The following work will be moved to [STORY-XXX or future epic/release]:
- [Specific work being deferred]
- [Related acceptance criteria being deferred]
- [Any dependent functionality]

---

## Rationale

### Why Scope is Changing

**Option 1: Technical Blocker**
- Blocker: [External dependency not available]
- Example: Payment API v2 required but not available until [date]
- Impact: Cannot implement [feature] without [dependency]
- Resolution: Defer until [dependency] available

**Option 2: Requirements Changed**
- Original requirement: [What was originally specified]
- Changed requirement: [What changed and why]
- Business justification: [Why the change is necessary]
- Impact: [DoD item] no longer needed OR moved to different story

**Option 3: Story Split for Focused Implementation**
- Original story scope: Too large ([X] points)
- Splitting rationale: Better to have [core feature] in [STORY-ID], [extended feature] in [STORY-XXX]
- Example: Core API in STORY-004, advanced error handling in STORY-005
- Benefit: Focused implementation, clearer scope boundaries

**Option 4: Architectural Decision**
- Better separation of concerns: [Explain]
- Layer boundary consideration: [Detail]
- Future extensibility: [Reasoning]

### Why Not Implement in Current Story

**Considered Alternative 1: Complete work now**
- Effort estimate: [X hours/days]
- Complexity: [Technical complexity factors]
- Risk: [Risk of expanding scope]
- Decision: Rejected because [reason]

**Considered Alternative 2: Minimal implementation**
- Could implement basic version in [Y hours]
- Would sacrifice [quality/feature completeness]
- Decision: Rejected because [prefer complete implementation in dedicated story]

**Selected Approach:** Defer to dedicated story
- Allows focused implementation
- Maintains story scope discipline
- Ensures proper planning for [DoD item]

---

## Consequences

### Positive

- ✅ Current story completes with clear, focused scope
- ✅ [DoD item] gets proper attention in dedicated story
- ✅ Better separation of concerns
- ✅ Prevents scope creep and rushed implementation

### Negative

- ❌ Original DoD incomplete (story delivered with known gap)
- ❌ Creates dependency: [STORY-ID] → [STORY-XXX]
- ❌ Additional story needed (impacts sprint capacity)
- ❌ Gap exists until [STORY-XXX] completes

### Mitigation

- Create [STORY-XXX] immediately (track deferred work)
- Set dependency: [STORY-XXX] prerequisite: [STORY-ID]
- Set priority: [High/Medium/Low based on importance]
- Target sprint: [Sprint-X]
- Document in technical debt register

---

## Alternatives Considered

### Alternative 1: Implement Now (Expand Current Story)

**Description:**
Complete [DoD item] within [STORY-ID] despite scope change.

**Pros:**
- No incomplete DoD
- No dependency created
- Work completed sooner

**Cons:**
- Scope creep ([original_points] → [new_points])
- Rushed implementation quality risk
- Violates scope discipline
- May delay current story completion

**Decision:** **Rejected** - Prefer controlled scope and quality

---

### Alternative 2: Remove from DoD Entirely

**Description:**
Eliminate [DoD item] from requirements completely (not just defer).

**Pros:**
- Story completes fully within original scope
- No future work created
- Simplest path

**Cons:**
- Loses required functionality
- Gap in system capabilities
- User stories incomplete

**Decision:** **Rejected** - Work is still needed, just in different story

---

### Alternative 3: Create Separate Story (Selected)

**Description:**
Defer [DoD item] to [STORY-XXX] with proper tracking and planning.

**Pros:**
- Maintains scope discipline
- Allows proper estimation and planning for [DoD item]
- Creates clear dependency chain
- Enables focused implementation

**Cons:**
- Additional story overhead
- Current story incomplete
- Requires coordination between stories

**Decision:** **ACCEPTED** - Best balance of quality, scope management, and delivery

---

## Implementation

### Immediate Actions

1. **Update Story [STORY-ID]:**
   - Mark DoD item as deferred: `[ ] {item} - Out of scope: ADR-006 descoped to STORY-XXX`
   - Reference this ADR in Implementation Notes
   - Update story status to "Dev Complete" (with documented deferral)

2. **Create Follow-up Story [STORY-XXX]:**
   - Title: [Descriptive title for deferred work]
   - Epic: [Same as STORY-ID or dedicated epic]
   - Dependency: Prerequisite: [STORY-ID]
   - Status: Backlog
   - Target: Sprint-[X]

3. **Update Technical Debt Register:**
   - Log deferred work: "[DoD item] from [STORY-ID]: Out of scope per ADR-006"
   - Follow-up: STORY-XXX
   - Priority: [High/Medium/Low]
   - Status: Open

### Validation

Before approving this scope change:
- [ ] Business stakeholder aware of deferral
- [ ] Follow-up story [STORY-XXX] created
- [ ] Technical debt register updated
- [ ] Impact on release timeline assessed
- [ ] Dependencies documented

---

## Review

**Next Review:** After [STORY-XXX] completes
**Review Criteria:**
- Did deferral decision prove correct?
- Was follow-up story properly scoped?
- Any learnings for future scope management?

**Supersedes:** None (new decision)
**Superseded by:** None
**Related ADRs:** [List related ADRs if any]

---

## Example Usage

**Scenario:** STORY-004 defers exit code handling to STORY-005

```markdown
# ADR-007: STORY-004 - Descope Exit Code Handling

## Context
STORY-004 (JSON Output) included DoD item:
"Exit code 0 for success, 2 for error"

Technical specification included main.rs error handling pattern.
Implementation feasible (15 lines, no external dependencies).

## Decision
Descope exit code handling from STORY-004 to STORY-005 (Error Handling Framework).

## Rationale
STORY-005 will create TreeLintError types and integrate with main.rs error flow.
Exit codes are part of error flow, better fits STORY-005 scope for cleaner separation.

User approved deferral via AskUserQuestion.

## Consequences
- STORY-004 completes with 1 deferred DoD item
- STORY-005 scope includes exit code integration
- Creates dependency: STORY-004 → STORY-005

## Alternatives Considered
- **Implement now:** Rejected - prefer cleaner scope separation
- **Create STORY-006:** Rejected - too small, fits in STORY-005
```

---

## Notes

**Created by:** devforgeai-architecture skill (architect-reviewer subagent)
**Invoked by:** devforgeai-development skill when user selects "Scope change" for incomplete DoD item

**Template Usage:**
1. Replace all [placeholders] with actual values
2. Select one rationale section (Technical Blocker, Requirements Changed, Story Split, or Architectural)
3. Remove unused sections
4. Provide concrete evidence and reasoning
5. Document all alternatives considered
