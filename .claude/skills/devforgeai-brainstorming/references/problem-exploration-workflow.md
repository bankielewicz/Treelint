---
id: problem-exploration-workflow
title: Phase 2 - Problem Exploration Workflow
version: "1.0"
created: 2025-12-21
status: Published
phase: 2
estimated_duration: "10-15 minutes"
question_count: "8-15"
---

# Phase 2: Problem Exploration

Deep dive into WHAT the actual problem is using root cause analysis and current state assessment.

## Overview

| Attribute | Value |
|-----------|-------|
| **Purpose** | Understand the problem deeply through 5 Whys and current state analysis |
| **Duration** | 10-15 minutes |
| **Questions** | 8-15 |
| **Output** | Problem statement, root causes, pain points, current state |
| **Techniques** | 5 Whys, Current State Mapping, Pain Point Inventory |

---

## Step 2.1: Current State Assessment

**Purpose:** Understand how things are done today

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "How is this task or process currently being done?"
      header: "Current State"
      multiSelect: false
      options:
        - label: "Manual process"
          description: "Done by humans with little automation"
        - label: "Semi-automated"
          description: "Mix of manual and automated steps"
        - label: "Automated but broken"
          description: "System exists but has issues"
        - label: "No process exists"
          description: "Currently not being done at all"
```

**Follow-Up Logic:**
```
IF response == "Manual process" OR response == "Semi-automated":
  session.current_state.type = response
  GOTO Step 2.1.1 (Process Details)

ELSE IF response == "Automated but broken":
  session.current_state.type = "automated_broken"
  GOTO Step 2.1.2 (System Issues)

ELSE IF response == "No process exists":
  session.current_state.type = "none"
  SKIP process details, GOTO Step 2.2
```

**Output:**
- `session.current_state.type` - Process type classification

---

## Step 2.1.1: Process Details (Manual/Semi-Automated)

**Purpose:** Capture metrics about current process

**Tool Invocations:**
```
# Process duration
AskUserQuestion:
  questions:
    - question: "How long does the current process take per instance?"
      header: "Duration"
      multiSelect: false
      options:
        - label: "Minutes (< 30 min)"
          description: "Quick task"
        - label: "Hours (30 min - 4 hrs)"
          description: "Significant time investment"
        - label: "Days (4+ hrs)"
          description: "Multi-day process"
        - label: "Weeks or longer"
          description: "Extended timeline"

# Volume
AskUserQuestion:
  questions:
    - question: "How many times per day/week is this process executed?"
      header: "Volume"
      multiSelect: false
      options:
        - label: "Occasionally (< 10/week)"
          description: "Infrequent task"
        - label: "Regularly (10-50/week)"
          description: "Common task"
        - label: "Frequently (50-200/week)"
          description: "High-volume task"
        - label: "Constantly (200+/week)"
          description: "Critical path"

# Error rate
AskUserQuestion:
  questions:
    - question: "How often does the current process fail or produce errors?"
      header: "Error Rate"
      multiSelect: false
      options:
        - label: "Rarely (< 5%)"
          description: "Errors are uncommon"
        - label: "Sometimes (5-15%)"
          description: "Noticeable error rate"
        - label: "Often (15-30%)"
          description: "Frequent problems"
        - label: "Very often (> 30%)"
          description: "Major reliability issues"
```

**Output:**
```yaml
session.current_state:
  duration: "{selected}"
  volume: "{selected}"
  error_rate: "{selected}"
```

---

## Step 2.1.2: System Issues (Automated but Broken)

**Purpose:** Understand what's wrong with existing system

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "What are the main issues with the current system?"
      header: "Issues"
      multiSelect: true
      options:
        - label: "Reliability problems"
          description: "System crashes or fails"
        - label: "Performance issues"
          description: "Too slow"
        - label: "Missing features"
          description: "Doesn't do what we need"
        - label: "Hard to use"
          description: "Poor user experience"
```

**Output:**
- `session.current_state.system_issues[]` - List of current system problems

---

## Step 2.2: 5 Whys Root Cause Analysis

**Purpose:** Dig to the root cause of the problem

**Introduction:**
```
Display:
"Now let's explore WHY this problem exists using the 5 Whys technique.
 We'll ask 'why' repeatedly to find the root cause."
```

**Tool Invocations:**
```
# Why Level 1
current_problem = session.topic
AskUserQuestion:
  questions:
    - question: "Why does {current_problem} happen?"
      header: "Why 1"
      multiSelect: false
      options:
        - label: "Let me explain"
          description: "I'll describe the cause"
        - label: "I don't know"
          description: "The cause is unclear"

IF response == "Let me explain":
  # User provides free-form explanation
  Store in session.root_causes[0]
  current_problem = session.root_causes[0]
ELSE:
  # Mark as unknown, still try to continue
  session.root_causes[0] = "Unknown - needs investigation"
  SKIP remaining whys OR ask from different angle
```

**Loop for Whys 2-5:**
```
FOR level in [2, 3, 4, 5]:
  IF current_problem == "Unknown" OR user_says_stop:
    BREAK

  AskUserQuestion:
    questions:
      - question: "Why does {current_problem} happen?"
        header: "Why {level}"
        multiSelect: false
        options:
          - label: "Let me explain"
            description: "I'll describe the cause"
          - label: "That's the root cause"
            description: "We've found the fundamental issue"
          - label: "I don't know"
            description: "Unsure of deeper cause"

  IF response == "Let me explain":
    Store explanation in session.root_causes[level-1]
    current_problem = session.root_causes[level-1]
  ELSE IF response == "That's the root cause":
    Mark session.root_cause_found = true
    BREAK
  ELSE:
    session.root_causes[level-1] = "Unknown"
    BREAK
```

**Output:**
```yaml
session.root_causes:
  why_1: "Manual data entry takes 45 minutes"
  why_2: "Data comes from 5 different sources"
  why_3: "No integration between systems"
  why_4: "Systems purchased separately over time"
  why_5: "No central IT strategy" # Root cause
root_cause_level: 5
root_cause_found: true
```

---

## Step 2.3: Pain Point Inventory

**Purpose:** Catalog the specific pains caused by this problem

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "What are the top 3-5 pain points caused by this problem?"
      header: "Pain Points"
      multiSelect: false
      options:
        - label: "Let me list them"
          description: "I'll describe each pain point"
```

**For Each Pain Point:**
```
FOR i in [1..5]:
  # Get pain point description
  pain_point = user_input

  IF pain_point is empty OR user_says_done:
    BREAK

  # Get impact type
  AskUserQuestion:
    questions:
      - question: "What type of business impact does '{pain_point}' cause?"
        header: "Impact Type"
        multiSelect: true
        options:
          - label: "Revenue loss"
            description: "Directly affects income"
          - label: "Cost increase"
            description: "Increases operational costs"
          - label: "Customer impact"
            description: "Affects customer satisfaction/retention"
          - label: "Employee impact"
            description: "Affects productivity or morale"
          - label: "Risk/Compliance"
            description: "Creates legal or regulatory exposure"

  # Get severity
  AskUserQuestion:
    questions:
      - question: "How severe is '{pain_point}' for your business?"
        header: "Severity"
        multiSelect: false
        options:
          - label: "Critical"
            description: "Business cannot function properly"
          - label: "High"
            description: "Significant negative impact"
          - label: "Medium"
            description: "Noticeable but manageable"
          - label: "Low"
            description: "Minor inconvenience"

  Store in session.pain_points[]
```

**Output:**
```yaml
session.pain_points:
  - description: "45 minutes per order processing"
    impact_types: ["Cost increase", "Employee impact"]
    severity: "High"
  - description: "8% order error rate"
    impact_types: ["Customer impact", "Revenue loss"]
    severity: "Critical"
```

---

## Step 2.4: Impact Quantification

**Purpose:** Put numbers to the pain

**Tool Invocations:**
```
FOR each pain_point in session.pain_points WHERE severity in ["Critical", "High"]:
  AskUserQuestion:
    questions:
      - question: "Can you estimate the cost of '{pain_point}' per month?"
        header: "Cost"
        multiSelect: false
        options:
          - label: "< $1,000/month"
            description: "Minor cost"
          - label: "$1,000 - $10,000/month"
            description: "Moderate cost"
          - label: "$10,000 - $100,000/month"
            description: "Significant cost"
          - label: "> $100,000/month"
            description: "Major cost"
          - label: "Can't estimate"
            description: "Unknown or hard to quantify"

  Store in pain_point.estimated_cost
```

**Output:**
- `pain_point.estimated_cost` - Monthly cost estimate per pain point

---

## Step 2.5: Failed Solution History

**Purpose:** Learn from past attempts

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "Have you tried solving this problem before?"
      header: "History"
      multiSelect: false
      options:
        - label: "Yes - it failed"
          description: "We tried but it didn't work"
        - label: "Yes - partially worked"
          description: "Some success but not complete"
        - label: "No - first attempt"
          description: "Haven't tried before"
        - label: "Not sure"
          description: "Before my time or don't know"
```

**Follow-Up if Previous Attempts:**
```
IF response in ["Yes - it failed", "Yes - partially worked"]:
  AskUserQuestion:
    questions:
      - question: "What was tried before?"
        header: "Previous"
        multiSelect: false
        options:
          - label: "Let me describe"
            description: "I'll explain what we tried"

  # Capture description
  Store in session.failed_solutions[].what

  AskUserQuestion:
    questions:
      - question: "Why didn't it work?"
        header: "Why Failed"
        multiSelect: true
        options:
          - label: "Too expensive"
            description: "Cost exceeded budget"
          - label: "Too complex"
            description: "Solution was too complicated"
          - label: "Poor adoption"
            description: "People didn't use it"
          - label: "Didn't solve the problem"
            description: "Wrong solution for the issue"
          - label: "Other factors"
            description: "External circumstances"

  Store in session.failed_solutions[].why
```

**Output:**
```yaml
session.failed_solutions:
  - what: "Implemented RPA bots"
    why: ["Too complex", "Poor adoption"]
    lessons: "Need simpler solution with better training"
```

---

## Step 2.6: Generate Problem Statement

**Purpose:** Synthesize a clear problem statement

**Logic:**
```
# Build problem statement from collected data
primary_stakeholder = session.stakeholders.primary[0].name
primary_pain = session.pain_points[0].description
root_cause = session.root_causes[-1]  # Deepest root cause
impact = session.pain_points[0].impact_types[0]

problem_statement = f"{primary_stakeholder} experiences {primary_pain} because {root_cause}, resulting in {impact}."
```

**Validation:**
```
Display:
"Based on our discussion, here's the problem statement:

  \"{problem_statement}\"

"

AskUserQuestion:
  questions:
    - question: "Does this accurately capture the problem?"
      header: "Validate"
      multiSelect: false
      options:
        - label: "Yes, that's accurate"
          description: "Proceed with this statement"
        - label: "Needs adjustment"
          description: "I'll refine it"
        - label: "Let me rewrite it"
          description: "I have a better version"
```

**Output:**
- `session.problem_statement` - Validated problem statement

---

## Phase 2 Summary Output

At end of Phase 2, session should contain:

```yaml
problem_analysis:
  problem_statement: "Operations team spends 45 minutes per order on manual data entry because no integration exists between 5 source systems, resulting in reduced throughput and 8% error rate."

  current_state:
    type: "manual"
    duration: "Hours"
    volume: "Frequently (50-200/week)"
    error_rate: "Often (15-30%)"

  root_causes:
    why_1: "Manual data entry takes 45 minutes per order"
    why_2: "Data must be copied from 5 different systems"
    why_3: "No integration exists between systems"
    why_4: "Systems were purchased separately over 10 years"
    why_5: "No central IT strategy or data architecture"
    root_cause_found: true
    root_cause_level: 5

  pain_points:
    - description: "45 minutes per order"
      impact_types: ["Cost increase", "Employee impact"]
      severity: "High"
      estimated_cost: "$10,000 - $100,000/month"
    - description: "8% error rate"
      impact_types: ["Customer impact"]
      severity: "Critical"
      estimated_cost: "$1,000 - $10,000/month"

  failed_solutions:
    - what: "RPA bots for data transfer"
      why: ["Too complex", "Poor adoption"]
      lessons: "Need simpler solution"
```

---

## Transition to Phase 3

**Display:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 2 Complete: Problem Exploration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Problem Statement:
  "{problem_statement}"

Root Cause (Level {root_cause_level}):
  {root_cause}

Pain Points: {count} identified
  - {pain_1} (Severity: {severity})
  - {pain_2} (Severity: {severity})

Previous Attempts: {count}

Proceeding to Phase 3: Opportunity Mapping...
```

---

## Common Issues and Recovery

| Issue | Symptom | Recovery |
|-------|---------|----------|
| Can't get past Why 1 | User says "I don't know" immediately | Ask from different angle: "What do people blame?" |
| Too many pain points | 10+ pain points listed | Focus on top 5 by severity |
| No failed solutions | User says "first attempt" | Still valuable - note greenfield opportunity |
| Problem statement too vague | Generic statement | Use specific numbers from pain point inventory |

---

## Success Criteria

- [ ] Current state documented (type, duration, volume, error rate)
- [ ] At least 3 levels of 5 Whys completed
- [ ] 2+ pain points identified with severity
- [ ] Problem statement generated and validated
- [ ] Failed solutions documented (if any)

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
