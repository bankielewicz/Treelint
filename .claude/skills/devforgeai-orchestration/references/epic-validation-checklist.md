# Epic Validation Checklist

Self-validation procedures and quality gates for epic completion, with automatic self-healing where possible.

---

## Overview

This checklist validates epics before they are finalized and marked complete. It ensures all required sections are present, content is coherent, scoping is appropriate, and framework constraints are respected.

**Key Principle:** Validate thoroughly but allow self-healing for correctable issues. Only HALT on critical violations (circular dependencies, framework violations, over-scoped).

---

## Validation Phase (Phase 7)

Epic validation occurs at the final phase, before marking epic as "Ready for Sprint Planning".

**Validation Logic:**
```
1. Check YAML frontmatter (all required fields present)
2. Check content sections (all required sections have content)
3. Check feature scoping (3-8 features, no circular dependencies)
4. Check coherence (features align with epic goal)
5. Check feasibility (complexity score reasonable)
6. Check framework alignment (tech-stack, architecture, anti-patterns)
7. Generate validation report with self-healing actions
```

---

## 1. YAML Frontmatter Validation

### Required Fields

```yaml
---
id: EPIC-NNN
title: [Epic Title]
status: Planning
priority: [Critical/High/Medium/Low]
business_value: [High/Medium/Low]
timeline: [1-2 sprints / 3-4 sprints / 5+ sprints]
created: YYYY-MM-DD
updated: YYYY-MM-DD
owner: [Product Owner Name]
tech_lead: [Tech Lead Name]
---
```

### Validation Checks

| Field | Required | Format Check | Self-Heal |
|-------|----------|-------------|-----------|
| `id` | YES | EPIC-NNN | Generate if missing |
| `title` | YES | 10-100 chars | HALT if <10 chars (too vague) |
| `status` | YES | Planning / In Progress / Complete | Default to Planning |
| `priority` | YES | Critical/High/Medium/Low | HALT if invalid |
| `business_value` | YES | High/Medium/Low | HALT if invalid |
| `timeline` | YES | 1-2 sprints / 3-4 sprints / 5+ sprints | HALT if invalid |
| `created` | YES | YYYY-MM-DD format | Auto-populate with today |
| `updated` | YES | YYYY-MM-DD format | Auto-populate with today |
| `owner` | YES | Name | HALT if missing (requires user input) |
| `tech_lead` | YES | Name | HALT if missing (requires user input) |

### Self-Healing Procedures

**Procedure: Missing ID**
```
IF id field missing:
  Generate: EPIC-{auto_increment_number}
  Log: "Self-healed: Generated EPIC ID"
  Continue validation
```

**Procedure: Missing Date Fields**
```
IF created field missing:
  Set: created = today's date (YYYY-MM-DD)
  Log: "Self-healed: Set created date to {date}"

IF updated field missing:
  Set: updated = today's date (YYYY-MM-DD)
  Log: "Self-healed: Set updated date to {date}"
```

**Procedure: Invalid Priority/Status**
```
IF priority not in [Critical, High, Medium, Low]:
  HALT - Require valid priority
  Message: "Priority must be one of: Critical, High, Medium, Low"

IF status not in [Planning, In Progress, Complete]:
  HALT - Invalid status value
  Message: "Status must be one of: Planning, In Progress, Complete"
```

**Procedure: Short/Vague Title**
```
IF title length < 10 characters:
  HALT - Title too vague
  Message: "Epic title should be 10+ characters. Examples: 'User Authentication with SSO', 'Checkout Optimization'"

IF title > 100 characters:
  HALT - Title too long
  Message: "Epic title should be <100 characters. Be concise."
```

---

## 2. Content Section Validation

### Required Sections

```markdown
✅ Business Goal (50+ words)
✅ Success Metrics (3+ measurable outcomes)
✅ Scope: In Scope / Out of Scope
✅ Features (3-8 features listed)
✅ Technical Assessment (complexity score)
✅ Dependencies (internal + external)
✅ Risks & Mitigation (2+ risks)
✅ Stakeholders (3+ stakeholders)
✅ Timeline (duration estimate)
✅ Status History (creation entry)
```

### Validation Checks

| Section | Min Length | Content Check | Self-Heal |
|---------|-----------|--------------|-----------|
| Business Goal | 50 words | Clear problem statement + value | HALT if <50 words |
| Success Metrics | 3+ metrics | SMART format (Specific, Measurable, Achievable, Relevant, Time-bound) | HALT if <3 or vague |
| Features | 3-8 features | List of features with descriptions | See Feature Scoping below |
| Technical Assessment | Present | Complexity score + architecture impact | Re-run architect-reviewer if missing |
| Dependencies | List | Dependency name, owner, status | HALT if circular dependencies |
| Risks | 2+ risks | Risk, probability, impact, mitigation | Add default risks if <2 |
| Stakeholders | 3+ | Name, role, responsibility | Add default if <3 |
| Timeline | Present | Duration in sprints + key milestones | Calculate from feature estimates |

### Self-Healing Procedures

**Procedure: Missing Business Goal**
```
IF Business Goal section missing OR <50 words:
  HALT - Require user input
  Message: "Epic needs clear business goal. What problem does it solve?"

Cannot auto-heal: Business goal requires user/product team expertise
```

**Procedure: Missing/Vague Success Metrics**
```
IF Success Metrics <3:
  HALT - Need at least 3 measurable outcomes
  Message: "Success metrics should be SMART: Specific, Measurable, Achievable, Relevant, Time-bound"
  Suggestion: "Examples: 'Reduce checkout time by 30%', 'Increase conversion rate by 15%'"

IF metric is vague ("improve", "enhance", "optimize"):
  HALT - Metrics must be measurable
  Message: "Metrics should include baseline → target. E.g., '5 min → 3.5 min (30% reduction)'"
```

**Procedure: Missing Technical Assessment**
```
IF Technical Assessment section missing:
  Action:
    1. Log: "Technical assessment missing - re-invoking architect-reviewer subagent"
    2. Invoke: architect-reviewer subagent with epic details
    3. Insert: Assessment results into epic
    4. Continue validation
```

**Procedure: Missing Dependencies**
```
IF Dependencies section missing:
  Log: "Self-healed: Created empty Dependencies section"
  Note: "No dependencies found - epic can start independently"

IF Dependencies exist but marked as "None":
  Validate: Is this realistic? (Most epics have prerequisites)
  If realistic: Accept as-is
  If suspicious: Flag for review
```

**Procedure: Missing Risks**
```
IF Risks section missing OR <2 risks:
  Action: Add default risk categories relevant to epic

  Default risks to add (if not present):
  - Scope creep (High probability, High impact)
  - Timeline slippage (Medium probability, Medium impact)
  - Dependency delays (Medium probability, Medium impact)
  - Technical complexity underestimated (Medium probability, High impact)

  Log: "Self-healed: Added {N} default risks based on epic type"
```

**Procedure: Missing Stakeholders**
```
IF Stakeholders <3:
  Action: Add default stakeholder roles

  Default stakeholders to add:
  - Product Owner
  - Tech Lead
  - QA Lead

  Log: "Self-healed: Added {N} default stakeholders"
```

---

## 3. Feature Scoping Validation

### Feature Count Validation

```
OPTIMAL RANGE: 3-8 features per epic

IF feature_count < 3:
  ⚠️ WARNING: Epic under-scoped
  Message: "Epic has {N} features. Consider combining with another epic or adding more features."
  Status: Allow creation but mark as "Planning (Under-Scoped)"
  Suggestion: "Features too small? Could they be combined?"

IF feature_count > 8:
  ⚠️ WARNING: Epic over-scoped
  Message: "Epic has {N} features. Consider splitting into multiple epics."
  Status: Allow creation but mark as "Planning (Over-Scoped)"
  Suggestion: "Which features could be deferred to Epic 2?"

IF feature_count = 3-8:
  ✅ OPTIMAL SCOPING
  Continue validation
```

### Feature Content Validation

For each feature in epic, validate:

| Aspect | Check | Action |
|--------|-------|--------|
| Name | Present, user-focused | HALT if technical name (e.g., "API endpoints") |
| Description | 1-2 sentences | HALT if missing |
| Complexity | Simple/Moderate/Complex | HALT if missing |
| Dependencies | Listed or "None" | HALT if circular dependencies |
| User Value | Clear benefit statement | HALT if "technical implementation" |
| Story Count | 3-6 stories estimated | HALT if <1 or >10 |

### Self-Healing: Dependency Validation

```
FOR each feature:
  IF feature depends on feature X:
    IF feature X depends on current feature:
      🛑 CIRCULAR DEPENDENCY DETECTED
      HALT - Cannot proceed
      Message: "Circular dependency: Feature {A} → {B} → {A}"
      Action: Require manual resolution

    IF feature X doesn't exist:
      🛑 INVALID DEPENDENCY
      HALT - Cannot proceed
      Message: "Feature depends on {X}, but {X} not found in epic"
      Action: Either add feature or remove dependency

  IF dependency is valid:
    ✅ Dependency recorded
    Continue validation

VALIDATE: Dependencies form valid DAG (Directed Acyclic Graph)
```

---

## 4. Coherence Validation

### Does Epic Cohere (Features Align with Goal)?

```
Epic Goal: "Improve checkout experience to increase conversion"

Feature 1: "Guest Checkout"
- Aligns with goal? YES (reduces barriers to checkout)
- Check: ✅ PASS

Feature 2: "User Profile Management"
- Aligns with goal? NO (improves account management, not checkout flow)
- Check: 🛑 FAIL
- Action: Either remove feature or clarify goal
```

### Coherence Check Procedure

```
FOR each feature:
  IF feature directly supports epic goal:
    ✅ Feature included

  IF feature tangentially related but necessary:
    ⚠️ Flag as "infrastructure feature" (e.g., API authentication for checkout)
    Allow inclusion

  IF feature unrelated to epic goal:
    🛑 Feature doesn't belong in this epic
    HALT - Require removal or goal clarification
    Message: "Feature {X} doesn't align with epic goal: {goal}"
```

---

## 5. Feasibility Validation

### Complexity Check

```
IF complexity score ≤ 6:
  ✅ FEASIBLE - Can be single epic

IF complexity score 7-8:
  ⚠️ HIGH COMPLEXITY - Possible as single epic but challenging
  Recommendation: "Consider 2-phase approach (MVP + extended)"

IF complexity score 9-10:
  🛑 TOO COMPLEX - Likely over-scoped
  HALT - Cannot be single epic
  Message: "Complexity score {N}/10 suggests multiple epics needed"
  Action: "Consider splitting into 2-3 smaller epics"
```

### Duration Check

```
ESTIMATE: {total_points} points / {avg_velocity} points per sprint

IF duration = 1-2 sprints:
  ✅ OPTIMAL - Well-scoped epic

IF duration = 3-4 sprints:
  ✅ ACCEPTABLE - Still cohesive

IF duration > 4 sprints:
  ⚠️ LONG DURATION - Consider scope reduction
  Message: "Epic spans {N} sprints. Is everything in scope?"

IF duration > 6 sprints:
  🛑 OVER-SCOPED - Split into multiple epics
  HALT - Require scope reduction
  Message: "Epic spans {N} sprints. This should be 2-3 separate epics."
```

---

## 6. Framework Integration Validation

### Context File Alignment

**IF context files exist in `devforgeai/specs/context/`:**

```
Read all 6 files:
- tech-stack.md
- architecture-constraints.md
- dependencies.md
- coding-standards.md
- anti-patterns.md
- source-tree.md
```

#### 6.1 Technology Validation (vs tech-stack.md)

```
FOR each technology in epic's technical assessment:
  IF technology in tech-stack.md:
    ✅ APPROVED - No action needed

  IF technology NOT in tech-stack.md:
    ⚠️ NOT APPROVED - Requires ADR
    Flag: "ADR Needed: Justify adoption of {technology}"
    Continue: Can still create epic, but flag for review

  IF 3+ new technologies:
    ⚠️ SIGNIFICANT DECISION - Likely requires governance
    Flag: "Multiple new technologies - recommend architecture review"
```

#### 6.2 Architecture Validation (vs architecture-constraints.md)

```
FOR each architectural change in epic:
  Check: Does it violate layer boundaries?

  IF violates layer boundaries:
    Example: "Domain layer directly imports Infrastructure"
    🛑 BLOCKED - Cannot proceed
    Message: "Proposed architecture violates {constraint}"
    Action: "Require architectural redesign"

  IF respects all boundaries:
    ✅ COMPLIANT - Continue validation
```

#### 6.3 Dependency Validation (vs dependencies.md)

```
FOR each external integration (payment gateway, API, etc.):
  Check: Is integration in dependencies.md?

  IF in dependencies.md:
    ✅ APPROVED - Continue

  IF NOT in dependencies.md:
    ⚠️ NOT APPROVED - Requires review/approval
    Flag: "External integration {X} not in approved list"
```

#### 6.4 Anti-Pattern Check (vs anti-patterns.md)

```
Check epic features for forbidden patterns:
- God Objects (classes >500 lines)
- Direct instantiation (violates DI)
- SQL concatenation (SQL injection risk)
- Hardcoded secrets
- Weak cryptography (MD5, SHA1)
- Copy-paste code

FOR each forbidden pattern found:
  🛑 BLOCKED - Cannot proceed
  Message: "Feature contains forbidden anti-pattern: {pattern}"
  Action: "Require architectural redesign"
```

**IF context files DON'T exist:**

```
Note in validation report:
- "Operating in greenfield mode - no context files to validate"
- Recommendation: "Create context files (via /create-context) before starting development"
- Epic can proceed but note constraint absence
```

---

## 7. Generate Validation Report

### Report Template

```markdown
# Epic Validation Report

**Epic**: {EPIC-ID} - {Title}
**Validation Date**: {YYYY-MM-DD}
**Validated By**: {Subagent/System}

---

## Validation Summary

**Overall Status**: ✅ PASSED / ⚠️ PASSED WITH WARNINGS / 🛑 FAILED

**Checks Passed**: {count}/{total}
**Self-Healing Actions**: {count}
**Manual Intervention Required**: {count}

---

## Detailed Results

### 1. YAML Frontmatter ✅

✅ id: EPIC-XXX
✅ title: [Title] (XX characters)
✅ status: Planning
✅ priority: {Priority}
✅ business_value: {Value}
✅ timeline: {Timeline}
✅ created: {Date}
✅ updated: {Date}
✅ owner: {Name}
✅ tech_lead: {Name}

**Self-Healing Actions Taken:**
- None

---

### 2. Content Sections ✅

✅ Business Goal (XXX words)
✅ Success Metrics (N metrics)
✅ Scope: In Scope / Out of Scope
✅ Features (N features)
✅ Technical Assessment (Complexity: X/10)
✅ Dependencies (M dependencies)
✅ Risks & Mitigation (N risks)
✅ Stakeholders (M stakeholders)
✅ Timeline (X sprints)
✅ Status History (Present)

**Self-Healing Actions Taken:**
- Added {N} default risks
- Added {M} default stakeholders

---

### 3. Feature Scoping ✅

Feature Count: N (range 3-8: ✅ OPTIMAL)

**Features:**
- Feature 1: [Name] (Complexity: {Level}, Story Count: {N})
- Feature 2: [Name] (Complexity: {Level}, Story Count: {N})
...

**Dependency Validation:**
✅ No circular dependencies
✅ Dependencies form valid DAG

---

### 4. Coherence ✅

**Epic Goal**: "..."

**Feature Alignment:**
✅ All features align with epic goal

---

### 5. Feasibility ✅

**Complexity Score**: X/10 ({Level})
- Justification: [Why this score]

**Duration**: N sprints (Optimal: 2-4 sprints)
- Total Points: XXX
- Avg Velocity: XX pts/sprint

---

### 6. Framework Alignment ✅

**Context Files Validated**:
✅ tech-stack.md (All technologies approved)
✅ architecture-constraints.md (No violations)
✅ dependencies.md (All integrations approved)
✅ anti-patterns.md (No forbidden patterns found)

[IF ISSUES]
**⚠️ Context File Issues:**
- [Issue]: [Resolution required before implementation]

---

## Self-Healing Actions Taken

The following issues were automatically corrected:

1. ✅ [Action 1]
2. ✅ [Action 2]

**Unchanged by Human Review:**
These self-healed issues should be reviewed to ensure accuracy.

---

## Approval Status

**Can Proceed to Sprint Planning**: ✅ YES / 🛑 NO

[IF NO]
**Blockers** (Must be resolved before proceeding):
1. [Blocker 1]: [Resolution steps]
2. [Blocker 2]: [Resolution steps]

---

## Recommendations

[If complexity high, suggest 2-phase approach]
[If over-scoped, suggest feature reduction]
[If under-scoped, suggest combining with another epic]

---

**Report Generated**: {Timestamp}
**Validator Version**: 1.0
```

---

## Validation Decision Tree

```
START: Epic Validation
│
├─ YAML Frontmatter Valid?
│  ├─ YES → Continue
│  └─ NO → Self-heal if possible, else HALT
│
├─ Content Sections Complete?
│  ├─ YES → Continue
│  └─ NO → Self-heal if possible, else HALT
│
├─ Feature Scoping (3-8 features)?
│  ├─ YES (3-8) → Continue
│  ├─ <3 → ⚠️ WARNING, allow with note
│  └─ >8 → ⚠️ WARNING, allow with note
│
├─ No Circular Dependencies?
│  ├─ YES → Continue
│  └─ NO → 🛑 HALT, require redesign
│
├─ Complexity Reasonable (≤8)?
│  ├─ YES → Continue
│  └─ NO (9-10) → 🛑 HALT, suggest split
│
├─ Framework Aligned?
│  ├─ YES (tech-stack, architecture, anti-patterns) → Continue
│  ├─ Needs ADR (new tech) → ⚠️ FLAG, allow with ADR requirement
│  └─ Violates Constraints → 🛑 HALT, require redesign
│
└─ ✅ PASSED VALIDATION → Ready for Sprint Planning
```

---

## When to HALT (Critical Failures)

**Do NOT allow epic to proceed if:**

1. **Circular Dependencies Detected**
   - Features form a cycle (A → B → A)
   - Cannot be resolved by simple reordering
   - Requires redesign

2. **Architecture Constraints Violated**
   - Layer boundaries broken
   - Dependency direction wrong
   - Architectural pattern violated

3. **Forbidden Patterns Present**
   - God Objects (>500 lines per entity)
   - Hardcoded secrets in design
   - SQL injection vulnerabilities
   - Weak cryptography specified

4. **Complexity >10 Score**
   - Suggests over-scoped initiative
   - Recommend splitting into multiple epics
   - Cannot fit in single epic execution

5. **Ambiguous Requirements**
   - Business goal missing or vague
   - Success metrics not SMART
   - Features unclear or overlapping

**Action for HALT:**
```
1. Generate validation report with detailed blocker description
2. Do NOT create epic file
3. Return report to user with resolution steps
4. Request user action before retrying validation
```

---

## Example Validation Scenarios

### Scenario 1: Minor Self-Healing

```
Epic: "User Authentication"

Validation Results:
- YAML: Missing created/updated dates
- Content: All sections present
- Features: 4 features (optimal)
- Dependencies: None (valid)
- Coherence: All features align
- Complexity: 5/10 (Moderate - acceptable)
- Framework: All validated ✅

Self-Healing Actions:
- Auto-set created: 2025-11-05
- Auto-set updated: 2025-11-05
- Auto-add 2 default risks

Result: ✅ PASSED - Ready for Sprint Planning
```

---

### Scenario 2: Over-Scoped Warning

```
Epic: "Complete E-Commerce Platform"

Validation Results:
- YAML: Valid
- Content: All sections present
- Features: 12 features (⚠️ > 8, over-scoped)
- Complexity: 8/10 (High)
- Duration: 6 sprints (⚠️ > 4)

Result: ⚠️ PASSED WITH WARNINGS
Message: "Epic is over-scoped. Recommend splitting into 2-3 epics"
Status: Mark epic as "Planning (Over-Scoped)"
Suggestion:
  - Epic 1: Product Catalog + Shopping Cart (3 features)
  - Epic 2: Checkout + Payment (3 features)
  - Epic 3: Order Management (3 features, deferred)
```

---

### Scenario 3: Critical Blocker

```
Epic: "Payment Processing"

Validation Results:
- YAML: Valid
- Content: Incomplete (no business goal)
- Features: 3 features
- Complexity: 7/10

Self-Healing Attempt:
- Business Goal missing → Cannot auto-heal (requires user/product expertise)

Result: 🛑 FAILED
Blocker: "Missing business goal. What business problem does this epic solve?"
Action Required: User must provide business goal before epic can be created
```

---

## Self-Validation Before Finalization

**Before marking epic as complete:**

- [ ] YAML frontmatter has all required fields
- [ ] All content sections have substantive content (not placeholders)
- [ ] Feature count is 3-8 (or flagged if outside range)
- [ ] No circular dependencies in feature prerequisites
- [ ] All features align with epic goal (coherence check)
- [ ] Complexity score ≤8 (or flagged as over-scoped)
- [ ] All technologies in tech-stack.md or flagged for ADR
- [ ] No architecture constraint violations
- [ ] No forbidden anti-patterns present
- [ ] Epic can be explained in 2-3 sentences

---

**Last Updated:** 2025-11-05
**Version:** 1.0
**Framework:** DevForgeAI Orchestration
