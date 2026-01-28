# User Interaction Patterns

AskUserQuestion templates and patterns used throughout orchestration workflows.

## Purpose

**Standardized user interaction** for common decision points:
- Sprint capacity conflicts
- Story blocker resolution
- QA exception requests
- Epic feature review
- Technical assessment clarifications
- Deferral recovery options

**Pattern characteristics:**
- Clear, concise questions (single question, not compound)
- Short headers (max 12 characters for chip display)
- 2-4 options with descriptions
- multiSelect used when appropriate (non-exclusive choices)

---

## Pattern 1: Story Priority Conflict

**Scenario:** Sprint capacity insufficient for all candidate stories

**When used:** Sprint planning, capacity validation detects overflow

```
Sprint capacity: 20 points
Stories ready: 3 (totaling 25 points)

AskUserQuestion:
  question: "Sprint has 3 stories (25 points) but capacity for 20 points. Which stories to include?"
  header: "Sprint capacity"
  options:
    - label: "STORY-001: User registration (5 points, High priority)"
      description: "Include user registration - enables user account creation"
    - label: "STORY-002: Order history (8 points, Medium priority)"
      description: "Include order history - users can view past orders"
    - label: "STORY-003: Admin dashboard (12 points, Low priority)"
      description: "Include admin dashboard - analytics and reporting"
  multiSelect: true
```

**User selects:** Multiple stories (checkbox multi-select)

**Example response:** `["STORY-001", "STORY-002"]` (13 points total, under 20 limit)

**Skill action:** Create sprint with selected stories only, defer others to backlog

---

## Pattern 2: Blocked Story Resolution

**Scenario:** Story blocked on external dependency

**When used:** Story status validation detects blocker

```
Story blocked on external dependency (email service API not ready)

AskUserQuestion:
  question: "STORY-001 blocked waiting for email API (ETA: 7 days, beyond sprint). How to proceed?"
  header: "Story blocker"
  options:
    - label: "Wait for dependency (keep blocked)"
      description: "Keep story blocked, plan to complete when API ready (may slip sprint)"
    - label: "Create mock/stub to unblock"
      description: "Build temporary mock email service, replace when API available"
    - label: "De-prioritize, start different story"
      description: "Move to future sprint, start different story this sprint"
    - label: "Escalate to tech lead"
      description: "External blocker requires leadership intervention"
  multiSelect: false
```

**User selects:** Single option (radio button)

**Skill action:** Based on selection, update story status, create mock if needed, or escalate

---

## Pattern 3: QA Threshold Exception Request

**Scenario:** Coverage below threshold but edge case justifiable

**When used:** QA validation detects coverage gap in non-critical code

```
Infrastructure coverage: 68% (threshold: 80%)
Uncovered code: Logging utilities (non-critical)
Business logic: 97% (exceeds 95%)

AskUserQuestion:
  question: "Infrastructure coverage 68% < 80%. Uncovered: non-critical utilities. Business logic: 97%. Accept exception?"
  header: "Coverage exception"
  options:
    - label: "Fail QA - Require tests for all infrastructure"
      description: "Enforce 80% threshold strictly, no exceptions (recommended)"
    - label: "Pass with waiver - Document reason"
      description: "Accept 68%, create ADR explaining why utilities don't need tests"
    - label: "Require public API tests only - Skip internals"
      description: "Test public interfaces (80%), exempt internal utilities (adjust threshold)"
  multiSelect: false
```

**User selects:** Single option

**Skill action:** Based on selection, fail QA, create ADR waiver, or adjust threshold

---

## Pattern 4: Epic Feature Review

**Scenario:** Requirements analyst generated features, need user approval

**When used:** Epic creation Phase 3 (feature decomposition)

```
Epic: User Authentication System
Features generated: 8 features

AskUserQuestion:
  question: "Requirements analyst generated 8 features for epic. Review and approve?"
  header: "Feature review"
  options:
    - label: "Approve all 8 features"
      description: "Features look good, proceed with epic creation"
    - label: "Edit feature list"
      description: "Remove, add, or modify features before proceeding"
    - label: "Regenerate with different focus"
      description: "Ask requirements analyst to decompose differently"
    - label: "Cancel epic creation"
      description: "Stop, I'll create epic manually"
  multiSelect: false
```

**User selects:** Single option

**Skill action:** Proceed with features, edit loop, regenerate, or halt

---

## Pattern 5: Technical Complexity Warning

**Scenario:** Epic complexity score >8 (high complexity)

**When used:** Epic creation Phase 4 (technical assessment)

```
Epic: Migrate Monolith to Microservices
Complexity score: 9/10 (Very high)

AskUserQuestion:
  question: "Epic complexity 9/10 (very high). This may require multiple epics. Proceed anyway?"
  header: "Complexity"
  options:
    - label: "Proceed with single epic"
      description: "Accept high complexity, careful planning required"
    - label: "Split into multiple epics"
      description: "Recommend splitting into 2-3 epics for manageability"
    - label: "Reduce scope to simplify"
      description: "Remove some features to lower complexity"
    - label: "Cancel and redesign"
      description: "Rethink approach before creating epic"
  multiSelect: false
```

**User selects:** Single option

**Skill action:** Proceed, split, reduce scope, or halt for redesign

---

## Pattern 6: Circular Deferral Detection

**Scenario:** Technical debt analyzer detects circular deferrals

**When used:** Sprint planning, debt analysis (Phase 4.5)

```
Circular deferral chain detected:
STORY-042 defers to STORY-043
STORY-043 defers to STORY-044
STORY-044 defers to STORY-042

AskUserQuestion:
  question: "Circular deferral detected (STORY-042 → STORY-043 → STORY-044 → STORY-042). How to resolve?"
  header: "Circular defer"
  options:
    - label: "Complete STORY-042 fully (break circle)"
      description: "Finish all work in STORY-042, don't defer anything"
    - label: "Merge 3 stories into one larger story"
      description: "Combine STORY-042, 043, 044 into single comprehensive story"
    - label: "Redesign feature decomposition"
      description: "Rethink how features are split to eliminate dependency circle"
    - label: "Mark as technical debt, escalate"
      description: "Document circular issue, escalate to tech lead for resolution"
  multiSelect: false
```

**User selects:** Single option

**Skill action:** Schedule story for completion, merge stories, redesign, or escalate

---

## Pattern 7: QA Retry Decision (Deferral Failures)

**Scenario:** QA failed due to deferral violations

**When used:** Phase 3.5 (QA retry workflow), failure type = deferral

```
QA failed: Deferred DoD items without tracking (attempt 1/3)

AskUserQuestion:
  question: "QA failed due to deferred DoD items (attempt 1/3). How should we proceed?"
  header: "Deferral fail"
  options:
    - label: "Return to dev, fix deferrals, retry QA"
      description: "Go back to development to complete or properly justify deferred items, then automatically retry QA validation"
    - label: "Create follow-up stories for deferred items"
      description: "Generate tracking stories for deferred work, document deferrals as justified (story remains QA Failed)"
    - label: "Manual resolution - I'll fix it"
      description: "Stop orchestration, I'll handle deferral issues manually via /dev command"
  multiSelect: false
```

**User selects:** Single option

**Skill action:** Retry with dev fix, create tracking stories, or pause for manual fix

---

## Pattern 8: QA Retry Decision (Coverage Failures)

**Scenario:** QA failed due to coverage below thresholds

**When used:** Phase 3.5 (QA retry workflow), failure type = coverage

```
QA failed: Coverage below thresholds (attempt 1/3)
Business logic: 88% (threshold 95%)

AskUserQuestion:
  question: "QA failed: Coverage below thresholds (attempt 1/3). Fix and retry?"
  header: "Coverage fail"
  options:
    - label: "Return to dev, add missing tests"
      description: "Go back to development to write tests for uncovered code, then retry QA"
    - label: "Request coverage exception"
      description: "Document justification for below-threshold coverage (requires approval)"
    - label: "Manual fix - I'll handle it"
      description: "Stop orchestration, I'll write tests manually"
  multiSelect: false
```

**User selects:** Single option

**Skill action:** Retry with tests added, request exception (ADR), or pause

---

## Pattern 9: QA Retry Decision (Anti-Pattern Violations)

**Scenario:** QA failed due to anti-pattern violations (CRITICAL/HIGH)

**When used:** Phase 3.5 (QA retry workflow), failure type = anti-pattern

```
QA failed: Anti-pattern violations detected (attempt 1/3)
CRITICAL: 2 (SQL injection, hardcoded secret)

AskUserQuestion:
  question: "QA found anti-pattern violations (attempt 1/3). Fix and retry?"
  header: "Anti-pattern"
  options:
    - label: "Return to dev, refactor code"
      description: "Go back to development to fix anti-pattern violations (SQL injection, layer violations, etc.)"
    - label: "Create ADR, request exception"
      description: "Document architectural decision for exception approval"
    - label: "Manual fix - I'll handle it"
      description: "Stop orchestration, I'll fix violations manually"
  multiSelect: false
```

**User selects:** Single option

**Skill action:** Retry with refactoring, request exception, or pause

---

## Pattern 10: Sprint Story Selection

**Scenario:** Multiple stories available for sprint planning

**When used:** Sprint creation, story selection from backlog

```
Backlog stories: 12 stories available
Sprint capacity: 40 points recommended

AskUserQuestion:
  question: "Select stories for Sprint-3 (target: 20-40 points):"
  header: "Story select"
  options:
    - label: "STORY-001: User registration (5 pts, High)"
      description: "High priority - enables user accounts"
    - label: "STORY-002: Profile editing (3 pts, High)"
      description: "High priority - users can update info"
    - label: "STORY-003: Password reset (5 pts, Medium)"
      description: "Medium priority - password recovery flow"
    - label: "STORY-004: Social login (8 pts, Low)"
      description: "Low priority - OAuth integration"
  multiSelect: true
```

**User selects:** Multiple stories (up to capacity)

**Skill action:** Create sprint with selected stories, update status to Ready for Dev

---

## Pattern 11: Missing Reference Resolution

**Scenario:** Deferral references story/ADR that doesn't exist

**When used:** Phase 4.5 (deferred tracking), validation detects missing reference

```
Deferral: "Integration tests - Tracked in STORY-043"
Issue: STORY-043 doesn't exist

AskUserQuestion:
  question: "Deferral references STORY-043 but story doesn't exist. Create it?"
  header: "Missing story"
  options:
    - label: "Yes - create tracking story now"
      description: "Generate STORY-043 to track this deferred work"
    - label: "No - I'll create it manually"
      description: "Skip for now, I'll create STORY-043 later"
    - label: "Fix reference (I meant different story)"
      description: "Update deferral reason to correct story ID"
  multiSelect: false
```

**User selects:** Single option

**Skill action:** Create story with story-creation skill, skip, or prompt for correction

---

## Pattern 12: Debt Reduction Sprint Recommendation

**Scenario:** Technical debt analyzer finds >10 open deferred items

**When used:** Sprint planning, after debt analysis

```
Technical debt: 15 open items
Oldest: 94 days old

AskUserQuestion:
  question: "Technical debt is high (15 open items). Schedule debt reduction sprint?"
  header: "Debt sprint"
  options:
    - label: "Yes - create debt reduction sprint"
      description: "Dedicate next sprint to resolving technical debt"
    - label: "No - continue with planned work"
      description: "Acknowledge debt, will address later"
    - label: "Review debt items first"
      description: "Show me the debt register before deciding"
  multiSelect: false
```

**User selects:** Single option

**Skill action:** Create debt-focused sprint, continue normally, or show debt register

---

## Best Practices

### Writing Effective Questions

**Do:**
- ✅ Ask single, focused question (not compound)
- ✅ Provide context in question text
- ✅ Use short headers (max 12 chars for chip display)
- ✅ Write clear option labels (1-5 words)
- ✅ Include descriptions explaining consequences
- ✅ Use multiSelect for non-exclusive choices

**Don't:**
- ❌ Ask multiple questions in one (use separate AskUserQuestion calls)
- ❌ Use vague headers ("Question", "Choose")
- ❌ Write option labels as full sentences
- ❌ Omit descriptions (users need context)
- ❌ Use multiSelect for exclusive choices (creates confusion)

---

### Header Examples

**Good headers:**
```
"Sprint capacity" ✅
"Story blocker" ✅
"Coverage fail" ✅
"Anti-pattern" ✅
"Debt sprint" ✅
"Missing story" ✅
```

**Bad headers:**
```
"Question about sprint capacity overflow" ❌ (too long)
"What should we do?" ❌ (vague)
"Choose" ❌ (not descriptive)
"User decision required" ❌ (obvious, not helpful)
```

---

### Option Label Examples

**Good labels:**
```
"Approve all 8 features" ✅
"Return to dev, fix deferrals, retry QA" ✅
"Create follow-up stories for deferred items" ✅
"Wait for dependency (keep blocked)" ✅
```

**Bad labels:**
```
"Option 1" ❌ (not descriptive)
"Yes" ❌ (unclear what yes means)
"I think we should probably go ahead and approve these features" ❌ (too long)
```

---

### Description Examples

**Good descriptions:**
```
"Go back to development to complete or properly justify deferred items, then automatically retry QA validation" ✅
"Generate tracking stories for deferred work, document deferrals as justified (story remains QA Failed)" ✅
"High priority - enables user account creation" ✅
```

**Bad descriptions:**
```
"This option does something" ❌ (vague)
"" ❌ (empty - no description)
"Approve" ❌ (duplicates label without adding context)
```

---

## Integration Patterns

### Epic Creation Workflow

**Uses Pattern 4:** Feature review and approval

**Context:** Phase 3 (Feature Decomposition)

**Frequency:** Once per epic (single approval decision)

---

### Sprint Planning Workflow

**Uses Patterns 1, 10:** Capacity conflicts, story selection

**Context:** Phase 3 (Sprint Planning)

**Frequency:** Once per sprint planning session

---

### QA Retry Workflow

**Uses Patterns 7, 8, 9:** Deferral, coverage, anti-pattern recovery

**Context:** Phase 3.5 (QA Failure Recovery)

**Frequency:** Each QA failure (up to 3 attempts)

---

### Deferral Tracking Workflow

**Uses Patterns 11, 12:** Missing references, debt reduction

**Context:** Phase 4.5 (Deferred Work Tracking)

**Frequency:** Each missing reference detected, sprint planning

---

## Related Files

- **mode-detection.md** - Determines which workflow (and which patterns) apply
- **qa-retry-workflow.md** - Phase 3.5, uses Patterns 7-9 extensively
- **deferred-tracking.md** - Phase 4.5, uses Patterns 11-12
- **sprint-planning-guide.md** - Uses Patterns 1, 10 for capacity and selection
- **epic-management.md** - Uses Pattern 4 for feature approval
- **troubleshooting.md** - Documents common issues with user interaction
