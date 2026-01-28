---
id: brainstorm-handoff-workflow
title: Brainstorm to Ideation Handoff Workflow
version: "1.0"
created: 2025-12-21
status: Published
---

# Brainstorm Handoff Workflow

Detailed workflow for detecting and integrating brainstorm session data into ideation.

## Overview

| Attribute | Value |
|-----------|-------|
| **Purpose** | Pre-populate ideation with brainstorm discoveries |
| **Trigger** | /ideate command Phase 0 |
| **Input** | BRAINSTORM-{NNN}.brainstorm.md |
| **Effect** | Skip/shorten discovery, pre-populate constraints |

---

## Section 1: Brainstorm Detection

### 1.1 When to Check

Check for brainstorms at the START of ideation, before any user questions.

```
FUNCTION check_for_brainstorms():
  # Search for brainstorm documents
  brainstorms = Glob(pattern="devforgeai/specs/brainstorms/BRAINSTORM-*.brainstorm.md")

  IF len(brainstorms) == 0:
    RETURN null  # No brainstorms, proceed normally

  # Sort by creation date (newest first)
  brainstorms = sort_by_date_desc(brainstorms)

  RETURN brainstorms
```

### 1.2 Display Available Brainstorms

```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Existing Brainstorm(s) Detected
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

FOR each brainstorm in brainstorms:
  frontmatter = read_frontmatter(brainstorm)
  Display:
  "• {frontmatter.id}: {frontmatter.title}
    Created: {frontmatter.created}
    Confidence: {frontmatter.confidence_level}
    Problem: {frontmatter.problem_statement[:50]}..."
```

### 1.3 User Selection

```
AskUserQuestion:
  questions:
    - question: "Would you like to use an existing brainstorm as input for ideation?"
      header: "Brainstorm"
      multiSelect: false
      options:
        - label: "Yes - use most recent (Recommended)"
          description: "Pre-populate ideation with {most_recent.id}"
        - label: "Yes - let me choose"
          description: "Select which brainstorm to use"
        - label: "No - start fresh"
          description: "Begin new ideation discovery"
```

**Handle "let me choose":**
```
IF response == "Yes - let me choose":
  options = []
  FOR each brainstorm in brainstorms:
    options.append({
      label: brainstorm.id,
      description: brainstorm.title
    })

  AskUserQuestion:
    questions:
      - question: "Which brainstorm would you like to use?"
        header: "Select"
        multiSelect: false
        options: options
```

---

## Section 2: Data Extraction

### 2.1 Read Brainstorm Document

```
FUNCTION extract_brainstorm_data(brainstorm_path):
  content = Read(file_path=brainstorm_path)

  # Parse YAML frontmatter
  frontmatter = parse_yaml_frontmatter(content)

  # Extract key fields
  brainstorm_context = {
    # Identity
    brainstorm_id: frontmatter.id,
    title: frontmatter.title,
    confidence_level: frontmatter.confidence_level,

    # Core outputs
    problem_statement: frontmatter.problem_statement,
    target_outcome: frontmatter.target_outcome,
    recommended_approach: frontmatter.recommended_approach,

    # Stakeholders
    primary_stakeholder: frontmatter.primary_stakeholder,
    user_personas: frontmatter.user_personas,

    # Constraints
    budget_range: frontmatter.budget_range,
    timeline: frontmatter.timeline,
    hard_constraints: frontmatter.hard_constraints,

    # Hypotheses
    critical_assumptions: frontmatter.critical_assumptions,

    # Priorities
    must_have_capabilities: frontmatter.must_have_capabilities,
    nice_to_have: frontmatter.nice_to_have
  }

  RETURN brainstorm_context
```

### 2.2 Validate Extracted Data

```
FUNCTION validate_brainstorm_context(context):
  required_fields = [
    "problem_statement",
    "user_personas",
    "hard_constraints",
    "must_have_capabilities"
  ]

  missing = []
  FOR field in required_fields:
    IF context[field] is null OR context[field] is empty:
      missing.append(field)

  IF len(missing) > 0:
    Display:
    "⚠ Brainstorm missing some fields: {missing}
     These will be asked during ideation."

    context.incomplete_fields = missing

  RETURN context
```

---

## Section 3: Ideation Pre-Population

### 3.1 Set Session Context

```
FUNCTION apply_brainstorm_to_ideation(context):
  session = new IdeationSession()

  # Pre-populate from brainstorm
  session.brainstorm_input = context
  session.skip_discovery = context.confidence_level in ["HIGH", "MEDIUM"]

  # Map brainstorm fields to ideation session
  session.problem_statement = context.problem_statement
  session.business_goals = [context.target_outcome]
  session.user_personas = context.user_personas
  session.constraints = context.hard_constraints
  session.must_have_requirements = context.must_have_capabilities
  session.assumptions = context.critical_assumptions

  RETURN session
```

### 3.2 Display Pre-Population Summary

```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Continuing from Brainstorm: {context.brainstorm_id}
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Pre-populated from brainstorm:
  ✓ Problem: {context.problem_statement[:60]}...
  ✓ Users: {len(context.user_personas)} persona(s)
  ✓ Constraints: {len(context.hard_constraints)} identified
  ✓ Must-haves: {len(context.must_have_capabilities)} capabilities

Confidence: {context.confidence_level}

{IF confidence_level == 'HIGH':}
  → Skipping Phase 1 discovery (already complete)
  → Starting at Phase 2: Requirements Elicitation
{ELSE:}
  → Shortened Phase 1 discovery
  → Will ask for missing details only
"
```

---

## Section 4: Phase 1 Behavior Change

### 4.1 Skip or Shorten Discovery

**If brainstorm context present with HIGH confidence:**
```
FUNCTION handle_phase_1_with_brainstorm(session):
  IF session.brainstorm_input AND session.brainstorm_input.confidence_level == "HIGH":
    # Skip Phase 1 entirely
    Display: "Phase 1 (Discovery) - Skipped (from brainstorm)"
    GOTO Phase 2

  ELSE IF session.brainstorm_input:
    # Shortened Phase 1 - only ask for missing fields
    FOR field in session.brainstorm_input.incomplete_fields:
      ask_question_for(field)

    Display: "Phase 1 (Discovery) - Validated with additions"
    GOTO Phase 2

  ELSE:
    # No brainstorm - full discovery
    execute_full_phase_1()
```

### 4.2 Validate Brainstorm Data

```
FUNCTION validate_brainstorm_with_user(session):
  Display:
  "From your brainstorm, the problem is:
   \"{session.problem_statement}\"

   Is this still accurate?"

  AskUserQuestion:
    questions:
      - question: "Is the problem statement still accurate?"
        header: "Validate"
        multiSelect: false
        options:
          - label: "Yes, proceed"
            description: "Use brainstorm problem statement"
          - label: "Update it"
            description: "I want to refine the problem"

  IF response == "Update it":
    AskUserQuestion:
      questions:
        - question: "What's the updated problem statement?"
          header: "Problem"
          multiSelect: false
          options:
            - label: "Let me describe"
              description: "I'll provide an update"

    session.problem_statement = user_input
```

---

## Section 5: Conflict Resolution

### 5.1 Brainstorm vs User Input Conflicts

**If user provides different answer than brainstorm:**
```
FUNCTION resolve_conflict(field, brainstorm_value, user_value):
  IF brainstorm_value != user_value:
    AskUserQuestion:
      questions:
        - question: "Your brainstorm said '{brainstorm_value}' but you now said '{user_value}'. Which is correct?"
          header: "Clarify"
          multiSelect: false
          options:
            - label: "Use new answer"
              description: "I've updated my thinking"
            - label: "Use brainstorm"
              description: "Original brainstorm is correct"
            - label: "Both are valid"
              description: "Context changed"

    IF response == "Use new answer":
      RETURN user_value
    ELSE IF response == "Use brainstorm":
      RETURN brainstorm_value
    ELSE:
      # Note both, let user clarify
      RETURN {original: brainstorm_value, updated: user_value}
```

### 5.2 Handle Outdated Brainstorms

```
FUNCTION check_brainstorm_freshness(context):
  brainstorm_date = parse_date(context.created)
  days_old = (today - brainstorm_date).days

  IF days_old > 30:
    AskUserQuestion:
      questions:
        - question: "This brainstorm is {days_old} days old. Has anything changed?"
          header: "Freshness"
          multiSelect: false
          options:
            - label: "Still accurate"
              description: "Proceed with brainstorm data"
            - label: "Some things changed"
              description: "I'll note the updates"
            - label: "Start fresh"
              description: "Too much has changed"

    IF response == "Start fresh":
      session.brainstorm_input = null
      session.skip_discovery = false
```

---

## Section 6: Integration Points

### 6.1 Where Brainstorm Data Flows

| Brainstorm Field | Ideation Phase | Usage |
|------------------|----------------|-------|
| `problem_statement` | Phase 1 | Pre-populate, validate |
| `user_personas` | Phase 1 | Pre-populate, skip persona questions |
| `target_outcome` | Phase 1 | Business goals |
| `hard_constraints` | Phase 5 | Feasibility constraints |
| `must_have_capabilities` | Phase 2 | Seed requirements |
| `critical_assumptions` | Phase 5 | Risk assessment |
| `budget_range` | Phase 5 | Feasibility |
| `timeline` | Phase 5 | Feasibility |
| `recommended_approach` | Phase 4 | Epic structure hint |

### 6.2 Phase Modifications

| Phase | Without Brainstorm | With Brainstorm |
|-------|-------------------|-----------------|
| Phase 1 | 5-10 questions | 0-3 questions |
| Phase 2 | 15-25 questions | 10-20 questions |
| Phase 3 | Standard | Standard |
| Phase 4 | Standard | Hint from approach |
| Phase 5 | Discover constraints | Validate constraints |
| Phase 6 | Standard | Standard |

---

## Common Issues and Recovery

| Issue | Symptom | Recovery |
|-------|---------|----------|
| Brainstorm file corrupted | YAML parse error | Offer fresh ideation |
| Missing critical fields | Null values | Ask during Phase 1 |
| Outdated brainstorm | Old date | Prompt for updates |
| Conflicting data | User contradicts | Resolution dialog |

---

## Success Criteria

- [ ] Brainstorms detected at ideation start
- [ ] User can choose which brainstorm to use
- [ ] Data extracted correctly from frontmatter
- [ ] Phase 1 shortened when brainstorm present
- [ ] Conflicts resolved through user dialog
- [ ] Pre-population summary displayed

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
