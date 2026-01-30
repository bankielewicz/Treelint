---
id: hypothesis-formation-workflow
title: Phase 5 - Hypothesis Formation Workflow
version: "1.0"
created: 2025-12-21
status: Published
phase: 5
estimated_duration: "5-8 minutes"
question_count: "3-6"
---

# Phase 5: Hypothesis Formation

Create TESTABLE assumptions that need validation before committing to a solution.

## Overview

| Attribute | Value |
|-----------|-------|
| **Purpose** | Identify and prioritize assumptions that could derail the project |
| **Duration** | 5-8 minutes |
| **Questions** | 3-6 |
| **Output** | Hypothesis register with validation methods and risk levels |

---

## Step 5.1: Extract Implicit Assumptions

**Purpose:** Identify assumptions made during earlier phases

**Analysis Logic:**
```
# Review all previous phases for implicit assumptions
assumptions = []

# From Problem Statement
IF session.problem_statement contains claims without evidence:
  assumptions.append({
    source: "problem_statement",
    assumption: extract_claim(session.problem_statement),
    type: "problem_validity"
  })

# From Root Cause Analysis
FOR why in session.root_causes:
  IF why is causal_claim:
    assumptions.append({
      source: "root_cause",
      assumption: why,
      type: "causation"
    })

# From Opportunity Mapping
FOR opportunity in session.opportunities:
  assumptions.append({
    source: "opportunity",
    assumption: f"{opportunity.description} will solve the problem",
    type: "solution_effectiveness"
  })

# From Constraints
FOR constraint in session.constraints:
  IF constraint.flexibility != "Fixed":
    assumptions.append({
      source: "constraint",
      assumption: f"{constraint} is negotiable",
      type: "constraint_flexibility"
    })
```

**Display:**
```
Display:
"Based on our discussion, I've identified these implicit assumptions:

1. {assumption_1}
2. {assumption_2}
3. {assumption_3}
...

Let's validate which ones are critical to test."
```

---

## Step 5.2: Prioritize Assumptions

**Purpose:** Identify which assumptions are most critical

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "Which of these assumptions are most critical to validate before proceeding?"
      header: "Critical"
      multiSelect: true
      options:
        - label: "{assumption_1}"
          description: "From: {source}"
        - label: "{assumption_2}"
          description: "From: {source}"
        - label: "{assumption_3}"
          description: "From: {source}"
        - label: "None are critical"
          description: "All assumptions are safe"
```

**Mark Selected as Critical:**
```
FOR each selected assumption:
  assumption.priority = "critical"
  assumption.validation_required = true
```

---

## Step 5.3: Formulate Hypotheses

**Purpose:** Convert assumptions into testable hypotheses

**Logic:**
```
FOR each critical_assumption:
  hypothesis = {
    id: f"H{index}",
    statement: formulate_if_then(critical_assumption),
    source: critical_assumption.source,
    type: critical_assumption.type
  }

  # Formulate as IF-THEN statement
  # Example: "If we implement API integration, then order processing time will reduce by 80%"
```

**User Refinement:**
```
FOR each hypothesis:
  Display:
    "Hypothesis {id}:
     IF {condition}, THEN {expected_outcome}

     Based on assumption: {original_assumption}"

  AskUserQuestion:
    questions:
      - question: "Is this hypothesis correctly stated?"
        header: "Validate"
        multiSelect: false
        options:
          - label: "Yes, correct"
            description: "Proceed with this hypothesis"
          - label: "Needs adjustment"
            description: "I'll refine it"
```

**Output:**
```yaml
session.hypotheses:
  - id: "H1"
    statement: "IF we implement API integration, THEN order processing time will reduce by 80%"
    condition: "API integration implemented"
    expected_outcome: "80% reduction in processing time"
    source: "opportunity"
```

---

## Step 5.4: Define Success Criteria

**Purpose:** Make hypotheses measurable

**Tool Invocations:**
```
FOR each hypothesis:
  AskUserQuestion:
    questions:
      - question: "How would we know if '{hypothesis.statement}' is true? What's the success metric?"
        header: "Metric"
        multiSelect: false
        options:
          - label: "Let me specify"
            description: "I'll define the metric"
          - label: "Suggest options"
            description: "Help me think of metrics"

  IF response == "Suggest options":
    # Provide relevant metric options based on hypothesis type
    IF hypothesis.type == "solution_effectiveness":
      options = ["Time reduction %", "Error rate reduction %", "Cost savings $"]
    ELSE IF hypothesis.type == "causation":
      options = ["Before/after comparison", "A/B test results", "Correlation analysis"]

  # Capture success criteria
  hypothesis.success_criteria = user_input
```

**Output:**
- `hypothesis.success_criteria` - Measurable success metric

---

## Step 5.5: Identify Validation Methods

**Purpose:** Plan how to test each hypothesis

**Tool Invocations:**
```
FOR each hypothesis:
  AskUserQuestion:
    questions:
      - question: "How could we validate '{hypothesis.statement}'?"
        header: "Validation"
        multiSelect: false
        options:
          - label: "Proof of concept"
            description: "Build a small test"
          - label: "User research"
            description: "Talk to users/stakeholders"
          - label: "Data analysis"
            description: "Analyze existing data"
          - label: "Expert consultation"
            description: "Ask domain experts"
          - label: "Market research"
            description: "Research similar solutions"

  hypothesis.validation_method = response
```

**Output:**
- `hypothesis.validation_method` - How to test the hypothesis

---

## Step 5.6: Assess Risk Level

**Purpose:** Understand consequences if hypothesis is wrong

**Tool Invocations:**
```
FOR each hypothesis:
  AskUserQuestion:
    questions:
      - question: "What happens if '{hypothesis.statement}' turns out to be wrong?"
        header: "Risk Level"
        multiSelect: false
        options:
          - label: "Project fails"
            description: "Critical dependency"
          - label: "Major rework needed"
            description: "Significant impact but recoverable"
          - label: "Minor adjustments"
            description: "Can adapt without major changes"
          - label: "No real impact"
            description: "Hypothesis isn't critical"

  hypothesis.risk_level = response

  IF response == "Project fails":
    hypothesis.blocking = true
    hypothesis.validate_first = true
```

**Output:**
- `hypothesis.risk_level` - Severity if wrong
- `hypothesis.blocking` - Whether it blocks the project

---

## Step 5.7: Context Window Check

**Purpose:** Offer checkpoint before synthesis

**Trigger:** After completing hypothesis formation

**Logic:**
```
IF estimated_context_usage > 70%:
  AskUserQuestion:
    questions:
      - question: "Context window is approximately {PERCENT}% full. Would you like to:"
        header: "Session"
        multiSelect: false
        options:
          - label: "Continue in this session"
            description: "Proceed to Phase 6 (Prioritization)"
          - label: "Save and continue later"
            description: "Create checkpoint and exit"
```

---

## Phase 5 Summary Output

At end of Phase 5, session should contain:

```yaml
hypothesis_register:
  hypotheses:
    - id: "H1"
      statement: "IF we implement API integration, THEN order processing time will reduce by 80%"
      condition: "API integration implemented"
      expected_outcome: "80% reduction in processing time"
      source: "opportunity"
      success_criteria: "Processing time < 5 minutes (currently 45 min)"
      validation_method: "Proof of concept"
      risk_level: "Project fails"
      blocking: true
      validate_first: true

    - id: "H2"
      statement: "IF we provide training, THEN user adoption will exceed 80%"
      condition: "Training provided"
      expected_outcome: ">80% adoption rate"
      source: "constraint_resistance"
      success_criteria: "80% of users actively using system after 30 days"
      validation_method: "User research"
      risk_level: "Major rework needed"
      blocking: false

  validation_priority:
    - "H1" # Validate first - blocking
    - "H2" # Validate second

  assumptions_validated: 2
  assumptions_remaining: 0
```

---

## Transition to Phase 6

**Display:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 5 Complete: Hypothesis Formation
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Hypotheses Formed: {count}
  - H1: {statement} (Risk: {risk_level})
  - H2: {statement} (Risk: {risk_level})

Blocking Hypotheses: {count}
  Must validate before proceeding: H1

Proceeding to Phase 6: Prioritization...
```

---

## Common Issues and Recovery

| Issue | Symptom | Recovery |
|-------|---------|----------|
| No assumptions found | Analysis finds nothing | Ask directly about risky beliefs |
| All assumptions "safe" | User says none critical | Probe with "what if wrong" scenarios |
| Can't formulate IF-THEN | Assumption is vague | Ask for specific expected outcome |
| No validation method | User unsure how to test | Suggest appropriate methods |

---

## Success Criteria

- [ ] Implicit assumptions extracted (3+ minimum)
- [ ] Critical assumptions identified
- [ ] Hypotheses formulated as IF-THEN statements
- [ ] Success criteria defined for each hypothesis
- [ ] Validation methods identified
- [ ] Risk levels assessed
- [ ] Blocking hypotheses flagged for priority validation

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
