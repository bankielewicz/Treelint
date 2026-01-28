---
id: prioritization-workflow
title: Phase 6 - Prioritization Workflow
version: "1.0"
created: 2025-12-21
status: Published
phase: 6
estimated_duration: "5-8 minutes"
question_count: "3-5"
---

# Phase 6: Prioritization

Rank opportunities and solutions using MoSCoW classification and Impact-Effort analysis.

## Overview

| Attribute | Value |
|-----------|-------|
| **Purpose** | Prioritize opportunities to focus effort on highest-value items |
| **Duration** | 5-8 minutes |
| **Questions** | 3-5 |
| **Output** | MoSCoW classification, Impact-Effort matrix, recommended sequence |
| **Techniques** | MoSCoW, Impact-Effort Matrix |

---

## Step 6.1: Compile Items for Prioritization

**Purpose:** Gather all items that need prioritization

**Logic:**
```
items_to_prioritize = []

# Opportunities from Phase 3
FOR opportunity in session.opportunities:
  items_to_prioritize.append({
    id: f"OPP-{index}",
    description: opportunity.description,
    source: opportunity.source,
    type: "opportunity"
  })

# Must-have capabilities from constraints
FOR capability in session.success_vision.metrics:
  items_to_prioritize.append({
    id: f"CAP-{index}",
    description: capability,
    source: "success_vision",
    type: "capability"
  })

# Adjacent problems
FOR adjacent in session.adjacent_opportunities:
  items_to_prioritize.append({
    id: f"ADJ-{index}",
    description: adjacent.problem,
    source: "adjacent",
    type: "adjacent"
  })

session.prioritization_items = items_to_prioritize
```

**Display:**
```
Display:
"We have {len(items)} items to prioritize:

Opportunities:
  - OPP-1: {description}
  - OPP-2: {description}

Capabilities:
  - CAP-1: {description}

Adjacent Problems:
  - ADJ-1: {description}

Let's classify each one."
```

---

## Step 6.2: MoSCoW Classification

**Purpose:** Classify each item as Must/Should/Could/Won't Have

**Tool Invocations:**
```
FOR each item in session.prioritization_items:
  AskUserQuestion:
    questions:
      - question: "How would you classify '{item.description}'?"
        header: "Priority"
        multiSelect: false
        options:
          - label: "Must Have"
            description: "Critical - project fails without it"
          - label: "Should Have"
            description: "Important but can work around"
          - label: "Could Have"
            description: "Nice to have if time/budget allows"
          - label: "Won't Have (this time)"
            description: "Explicitly out of scope"

  item.moscow = response
```

**Categorize Results:**
```
session.moscow = {
  must_have: [items where moscow == "Must Have"],
  should_have: [items where moscow == "Should Have"],
  could_have: [items where moscow == "Could Have"],
  wont_have: [items where moscow == "Won't Have"]
}
```

**Validation Check:**
```
IF len(must_have) == 0:
  Display: "No Must Haves identified. Are you sure nothing is critical?"
  AskUserQuestion:
    questions:
      - question: "Is there truly nothing that's absolutely required?"
        header: "Confirm"
        multiSelect: false
        options:
          - label: "Correct - all flexible"
            description: "Everything is negotiable"
          - label: "Let me reconsider"
            description: "I'll upgrade some items"

IF len(must_have) > 5:
  Display: "Many Must Haves identified. This may exceed constraints."
  AskUserQuestion:
    questions:
      - question: "Can we reduce the Must Haves to focus on the most critical?"
        header: "Reduce"
        multiSelect: false
        options:
          - label: "Yes, let me reconsider"
            description: "I'll downgrade some items"
          - label: "All are truly critical"
            description: "Keep as is"
```

**Output:**
```yaml
session.moscow:
  must_have:
    - id: "OPP-1"
      description: "API integration for order processing"
    - id: "CAP-1"
      description: "Reduce processing time"
  should_have:
    - id: "OPP-2"
      description: "Automated error detection"
  could_have:
    - id: "ADJ-1"
      description: "Also automate reporting"
  wont_have:
    - id: "OPP-3"
      description: "Mobile app for order entry"
```

---

## Step 6.3: Impact-Effort Matrix

**Purpose:** Plot items on impact vs effort for sequencing

**Introduction:**
```
Display:
"Now let's assess impact and effort for each Must Have and Should Have item.

This will help determine the optimal sequence.

     HIGH IMPACT
          │
  Major   │   Quick
 Projects │   Wins
──────────┼──────────
  Avoid   │  Fill-ins
          │
     LOW IMPACT
   HIGH EFFORT   LOW EFFORT
"
```

**Tool Invocations:**
```
FOR each item in [must_have + should_have]:
  # Impact assessment
  AskUserQuestion:
    questions:
      - question: "What is the expected impact of '{item.description}'?"
        header: "Impact"
        multiSelect: false
        options:
          - label: "High impact"
            description: "Significant business value"
          - label: "Medium impact"
            description: "Moderate business value"
          - label: "Low impact"
            description: "Minor business value"

  item.impact = response

  # Effort assessment
  AskUserQuestion:
    questions:
      - question: "How much effort do you estimate '{item.description}' would require?"
        header: "Effort"
        multiSelect: false
        options:
          - label: "Low effort"
            description: "Days to a week"
          - label: "Medium effort"
            description: "Weeks to a month"
          - label: "High effort"
            description: "Months of work"

  item.effort = response
```

**Categorize by Quadrant:**
```
session.impact_effort = {
  quick_wins: [items where impact == "High" AND effort == "Low"],
  major_projects: [items where impact == "High" AND effort == "High"],
  fill_ins: [items where impact == "Low" AND effort == "Low"],
  avoid: [items where impact == "Low" AND effort == "High"]
}
```

**Output:**
```yaml
session.impact_effort:
  quick_wins:
    - id: "OPP-1"
      description: "API integration"
      impact: "High"
      effort: "Low"
  major_projects:
    - id: "CAP-1"
      description: "Full automation"
      impact: "High"
      effort: "High"
  fill_ins:
    - id: "OPP-2"
      description: "Error detection"
      impact: "Medium"
      effort: "Low"
  avoid: []
```

---

## Step 6.4: Generate Recommended Sequence

**Purpose:** Create an ordered list of what to tackle first

**Logic:**
```
# Build sequence based on prioritization rules
sequence = []
order = 1

# 1. Quick Wins that are Must Have (do first)
FOR item in quick_wins WHERE item.id in must_have:
  sequence.append({
    order: order,
    item: item,
    rationale: "Quick Win + Must Have = Do first"
  })
  order += 1

# 2. Quick Wins that are Should Have
FOR item in quick_wins WHERE item.id in should_have:
  sequence.append({
    order: order,
    item: item,
    rationale: "Quick Win + Should Have = Do early"
  })
  order += 1

# 3. Major Projects that are Must Have
FOR item in major_projects WHERE item.id in must_have:
  sequence.append({
    order: order,
    item: item,
    rationale: "Major Project + Must Have = Plan carefully"
  })
  order += 1

# 4. Major Projects that are Should Have
FOR item in major_projects WHERE item.id in should_have:
  sequence.append({
    order: order,
    item: item,
    rationale: "Major Project + Should Have = If time allows"
  })
  order += 1

# 5. Fill-ins (Could Have)
FOR item in fill_ins:
  sequence.append({
    order: order,
    item: item,
    rationale: "Fill-in = When bandwidth available"
  })
  order += 1

session.recommended_sequence = sequence
```

**Display:**
```
Display:
"Based on MoSCoW and Impact-Effort analysis, here's the recommended sequence:

1. {item_1} - Quick Win + Must Have
2. {item_2} - Quick Win + Should Have
3. {item_3} - Major Project + Must Have
...

Items to avoid or defer:
- {avoid_item} (Low impact, high effort)"
```

---

## Step 6.5: Validate Sequence

**Purpose:** Get user confirmation on recommended order

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "Does this sequence look right? Any adjustments needed?"
      header: "Validate"
      multiSelect: false
      options:
        - label: "Yes, looks good"
          description: "Proceed with this sequence"
        - label: "Adjust order"
          description: "I'd like to change the priority"
        - label: "Add dependencies"
          description: "Some items depend on others"
```

**Handle Adjustments:**
```
IF response == "Adjust order":
  AskUserQuestion:
    questions:
      - question: "What would you like to move?"
        header: "Move"
        multiSelect: false
        options:
          - label: "Move {item_1} higher"
            description: "Should be done earlier"
          - label: "Move {item_2} lower"
            description: "Can wait"
          - label: "Other changes"
            description: "Let me explain"

IF response == "Add dependencies":
  AskUserQuestion:
    questions:
      - question: "Which item depends on another?"
        header: "Dependency"
        multiSelect: false
        options:
          - label: "{item_2} depends on {item_1}"
            description: "Must complete {item_1} first"
          # ... other dependency options
```

---

## Phase 6 Summary Output

At end of Phase 6, session should contain:

```yaml
prioritization:
  moscow:
    must_have:
      - id: "OPP-1"
        description: "API integration for order processing"
      - id: "CAP-1"
        description: "Reduce processing time"
    should_have:
      - id: "OPP-2"
        description: "Automated error detection"
    could_have:
      - id: "ADJ-1"
        description: "Also automate reporting"
    wont_have:
      - id: "OPP-3"
        description: "Mobile app"

  impact_effort:
    quick_wins: ["OPP-1"]
    major_projects: ["CAP-1"]
    fill_ins: ["OPP-2"]
    avoid: []

  recommended_sequence:
    - order: 1
      id: "OPP-1"
      description: "API integration"
      rationale: "Quick Win + Must Have"
    - order: 2
      id: "CAP-1"
      description: "Full automation"
      rationale: "Major Project + Must Have"
    - order: 3
      id: "OPP-2"
      description: "Error detection"
      rationale: "Fill-in + Should Have"

  dependencies: []
```

---

## Transition to Phase 7

**Display:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 6 Complete: Prioritization
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

MoSCoW Classification:
  Must Have: {count}
  Should Have: {count}
  Could Have: {count}
  Won't Have: {count}

Impact-Effort Analysis:
  Quick Wins: {count}
  Major Projects: {count}
  Fill-ins: {count}
  Avoid: {count}

Recommended Start:
  1. {first_item}

Proceeding to Phase 7: Handoff Synthesis...
```

---

## Common Issues and Recovery

| Issue | Symptom | Recovery |
|-------|---------|----------|
| Everything is Must Have | 10+ Must Haves | Force rank top 3-5 |
| Everything is Quick Win | Underestimating effort | Probe for hidden complexity |
| Nothing is Must Have | All items optional | Ask about project success criteria |
| Circular dependencies | A depends on B depends on A | Identify minimum viable scope |

---

## Success Criteria

- [ ] All opportunities classified (MoSCoW)
- [ ] Must Haves identified (1-5 items)
- [ ] Impact-Effort assessed for Must/Should Haves
- [ ] Recommended sequence generated
- [ ] User validated sequence
- [ ] Dependencies noted (if any)

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
