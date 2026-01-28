---
id: stakeholder-discovery-workflow
title: Phase 1 - Stakeholder Discovery Workflow
version: "1.0"
created: 2025-12-21
status: Published
phase: 1
estimated_duration: "5-10 minutes"
question_count: "5-10"
---

# Phase 1: Stakeholder Discovery

Identify WHO is involved in the problem space and WHAT they want from a solution.

## Overview

| Attribute | Value |
|-----------|-------|
| **Purpose** | Identify all stakeholders and their goals/concerns |
| **Duration** | 5-10 minutes |
| **Questions** | 5-10 |
| **Output** | Stakeholder map with goals, concerns, and conflicts |
| **Subagent** | stakeholder-analyst (optional enhancement) |

---

## Step 1.1: Get Initial Topic

**Purpose:** Establish what we're brainstorming about

**Trigger:** Topic not provided in command arguments

**Tool Invocations:**
```
IF topic is null:
  AskUserQuestion:
    questions:
      - question: "What business problem or opportunity would you like to explore?"
        header: "Topic"
        multiSelect: false
        options:
          - label: "I have a specific problem"
            description: "Something isn't working well"
          - label: "I see an opportunity"
            description: "Something could be better"
          - label: "I'm not sure yet"
            description: "Help me discover it"
```

**Decision Logic:**
```
IF response == "specific problem":
  Ask for problem description
  Set session.topic_type = "problem"
ELSE IF response == "opportunity":
  Ask for opportunity description
  Set session.topic_type = "opportunity"
ELSE:
  Set session.topic_type = "discovery"
  Skip detailed description, proceed to stakeholder questions
```

**Output:**
- `session.topic` - The brainstorm topic
- `session.topic_type` - "problem", "opportunity", or "discovery"

---

## Step 1.2: Identify Primary Decision Maker

**Purpose:** Find who has authority to approve/reject solutions

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "Who has the authority to approve or reject solutions for this problem?"
      header: "Decision Maker"
      multiSelect: false
      options:
        - label: "I am the decision maker"
          description: "I have budget and approval authority"
        - label: "Someone else"
          description: "I'll need to get approval from others"
        - label: "Multiple people"
          description: "It's a committee or shared decision"
        - label: "Not sure"
          description: "Decision authority is unclear"
```

**Follow-Up Logic:**
```
IF response == "Someone else" OR response == "Multiple people":
  AskUserQuestion:
    questions:
      - question: "Who are the key decision makers?"
        header: "Who"
        multiSelect: false
        options:
          - label: "Let me list them"
            description: "I'll name the people/roles"

  # Capture names/roles
  Store in session.stakeholders.primary[]
```

**Output:**
- `session.stakeholders.primary[]` - List of primary stakeholders with decision authority

---

## Step 1.3: Identify End Users

**Purpose:** Find who will use the solution daily

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "Who will actually use the solution on a daily basis?"
      header: "End Users"
      multiSelect: true
      options:
        - label: "Internal employees"
          description: "Staff within the organization"
        - label: "External customers"
          description: "Paying customers or clients"
        - label: "Partners/vendors"
          description: "Third-party business relationships"
        - label: "The general public"
          description: "Anyone can access it"
```

**Follow-Up Logic:**
```
FOR each selected user_type:
  AskUserQuestion:
    questions:
      - question: "How many [USER_TYPE] would use this solution?"
        header: "Count"
        multiSelect: false
        options:
          - label: "1-10"
            description: "Small team"
          - label: "10-100"
            description: "Department level"
          - label: "100-1000"
            description: "Company-wide"
          - label: "1000+"
            description: "Large scale"
```

**Output:**
- `session.stakeholders.secondary[]` - List of end user groups with counts

---

## Step 1.4: Identify Affected Parties

**Purpose:** Find who else will be impacted by the change

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "Who else will be impacted by this change, even if they don't use the solution directly?"
      header: "Affected"
      multiSelect: true
      options:
        - label: "IT/Operations team"
          description: "Those who maintain systems"
        - label: "Finance/Accounting"
          description: "Those who handle budgets and reporting"
        - label: "Compliance/Legal"
          description: "Those who ensure regulatory compliance"
        - label: "Customer Support"
          description: "Those who help users with issues"
```

**Output:**
- `session.stakeholders.tertiary[]` - List of affected parties

---

## Step 1.5: Map Stakeholder Goals

**Purpose:** Understand what each stakeholder wants

**Tool Invocations:**
```
FOR each stakeholder in session.stakeholders.primary:
  AskUserQuestion:
    questions:
      - question: "What does [STAKEHOLDER] want to achieve from this initiative?"
        header: "Goals"
        multiSelect: true
        options:
          - label: "Reduce costs"
            description: "Save money on operations"
          - label: "Increase revenue"
            description: "Generate more income"
          - label: "Improve efficiency"
            description: "Do more with less"
          - label: "Reduce risk"
            description: "Prevent problems or losses"

  Store response in stakeholder.goals[]
```

**Optional Subagent Enhancement:**
```
# For deeper stakeholder analysis, invoke subagent
Task(
  subagent_type="stakeholder-analyst",
  prompt="Analyze stakeholders for: {session.topic}.
          Primary: {session.stakeholders.primary}
          Secondary: {session.stakeholders.secondary}
          Identify goals, concerns, and potential conflicts."
)
```

**Output:**
- `stakeholder.goals[]` - Goals for each stakeholder

---

## Step 1.6: Map Stakeholder Concerns

**Purpose:** Understand fears and blockers for each stakeholder

**Tool Invocations:**
```
FOR each stakeholder in session.stakeholders.primary:
  AskUserQuestion:
    questions:
      - question: "What concerns or fears does [STAKEHOLDER] have about this initiative?"
        header: "Concerns"
        multiSelect: true
        options:
          - label: "Budget overruns"
            description: "It might cost too much"
          - label: "Timeline delays"
            description: "It might take too long"
          - label: "Disruption"
            description: "It might disrupt current operations"
          - label: "Adoption issues"
            description: "People might not use it"

  Store response in stakeholder.concerns[]
```

**Output:**
- `stakeholder.concerns[]` - Concerns for each stakeholder

---

## Step 1.7: Identify Conflicts

**Purpose:** Find competing interests between stakeholders

**Analysis Logic:**
```
conflicts = []

FOR each pair (stakeholder_a, stakeholder_b):
  IF stakeholder_a.goals CONFLICTS_WITH stakeholder_b.goals:
    conflict = {
      stakeholders: [stakeholder_a.name, stakeholder_b.name],
      nature: describe_conflict(stakeholder_a.goals, stakeholder_b.goals),
      resolution: null  # Will be addressed in prioritization
    }
    conflicts.append(conflict)
```

**Conflict Detection Rules:**
- "Reduce costs" vs "Increase features" → Resource conflict
- "ASAP timeline" vs "High quality" → Speed vs quality
- "Minimal disruption" vs "Complete overhaul" → Scope conflict

**User Confirmation:**
```
IF len(conflicts) > 0:
  Display: "I've identified potential conflicts:"
  FOR each conflict:
    Display: "- [STAKEHOLDER_A] wants [GOAL_A] but [STAKEHOLDER_B] wants [GOAL_B]"

  AskUserQuestion:
    questions:
      - question: "Are these conflicts accurate? Any others?"
        header: "Validate"
        multiSelect: false
        options:
          - label: "Yes, accurate"
            description: "These conflicts are real"
          - label: "Some corrections needed"
            description: "I'll clarify"
          - label: "No conflicts"
            description: "They're actually aligned"
```

**Output:**
- `session.conflicts[]` - List of identified conflicts

---

## Step 1.8: Context Window Check

**Purpose:** Offer checkpoint before proceeding

**Trigger:** After completing all stakeholder discovery

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
            description: "Proceed to Phase 2 (Problem Exploration)"
          - label: "Save and continue later"
            description: "Create checkpoint and exit"

  IF response == "Save and continue later":
    Generate checkpoint
    Display resume instructions
    EXIT skill
```

---

## Phase 1 Summary Output

At end of Phase 1, session should contain:

```yaml
stakeholder_map:
  primary:
    - name: "{Role/Title}"
      type: "decision_maker"
      goals: ["Goal 1", "Goal 2"]
      concerns: ["Concern 1", "Concern 2"]
      influence: "HIGH"

  secondary:
    - name: "{User Group}"
      type: "end_user"
      count: "10-100"
      goals: ["Goal 1"]
      concerns: ["Concern 1"]
      influence: "MEDIUM"

  tertiary:
    - name: "{Department}"
      type: "affected_party"
      goals: ["Goal 1"]
      concerns: ["Concern 1"]
      influence: "LOW"

  conflicts:
    - stakeholders: ["A", "B"]
      nature: "Resource allocation"
      resolution: null  # TBD in prioritization
```

---

## Transition to Phase 2

**Display:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 1 Complete: Stakeholder Discovery
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Stakeholders Identified:
  Primary: {count} decision maker(s)
  Secondary: {count} user group(s)
  Tertiary: {count} affected parties

Conflicts: {count} potential conflicts identified

Proceeding to Phase 2: Problem Exploration...
```

---

## Common Issues and Recovery

| Issue | Symptom | Recovery |
|-------|---------|----------|
| User can't identify decision maker | "Not sure" selected | Ask about budget/approval process to infer |
| Too many stakeholders | 10+ stakeholders listed | Group by role, prioritize top 3-5 |
| No concerns identified | All stakeholders say "no concerns" | Probe with specific scenarios |
| Conflicting goals unclear | Conflict detection misses obvious ones | Ask explicitly about competing priorities |

---

## Success Criteria

- [ ] At least 1 primary stakeholder identified
- [ ] At least 1 secondary stakeholder (end user) identified
- [ ] Goals captured for primary stakeholders
- [ ] Concerns captured for primary stakeholders
- [ ] Conflicts identified (if any exist)
- [ ] Context window check completed

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
