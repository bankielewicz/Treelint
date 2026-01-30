---
id: constraint-discovery-workflow
title: Phase 4 - Constraint Discovery Workflow
version: "1.0"
created: 2025-12-21
status: Published
phase: 4
estimated_duration: "5-8 minutes"
question_count: "5-8"
---

# Phase 4: Constraint Discovery

Understand WHAT LIMITS the solution space through budget, timeline, resource, and organizational constraints.

## Overview

| Attribute | Value |
|-----------|-------|
| **Purpose** | Identify all constraints that limit solution options |
| **Duration** | 5-8 minutes |
| **Questions** | 5-8 |
| **Output** | Constraint matrix with budget, timeline, resources, technical, organizational limits |

---

## Step 4.1: Budget Constraints

**Purpose:** Understand financial limits

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "What budget range is available for this initiative?"
      header: "Budget"
      multiSelect: false
      options:
        - label: "< $10K"
          description: "Small project / proof of concept"
        - label: "$10K - $50K"
          description: "Modest investment"
        - label: "$50K - $200K"
          description: "Significant project"
        - label: "$200K - $1M"
          description: "Major initiative"
        - label: "> $1M"
          description: "Enterprise transformation"
        - label: "Not defined yet"
          description: "Budget TBD"
```

**Follow-Up:**
```
IF response != "Not defined yet":
  AskUserQuestion:
    questions:
      - question: "Is this budget flexible or fixed?"
        header: "Flexibility"
        multiSelect: false
        options:
          - label: "Fixed - cannot exceed"
            description: "Hard budget limit"
          - label: "Somewhat flexible"
            description: "Can adjust with justification"
          - label: "Very flexible"
            description: "Budget can grow if value proven"

  # Ongoing costs
  AskUserQuestion:
    questions:
      - question: "Is there budget for ongoing operational costs?"
        header: "Ongoing"
        multiSelect: false
        options:
          - label: "Yes - included in budget"
            description: "Operating costs considered"
          - label: "Separate budget"
            description: "OpEx is different from CapEx"
          - label: "Not considered yet"
            description: "Need to discuss"
```

**Output:**
```yaml
session.constraints.budget:
  range: "$50K - $200K"
  flexibility: "Somewhat flexible"
  ongoing_costs: "Separate budget"
```

---

## Step 4.2: Timeline Constraints

**Purpose:** Understand time limits

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "When does this need to be done?"
      header: "Timeline"
      multiSelect: false
      options:
        - label: "ASAP (< 1 month)"
          description: "Urgent need"
        - label: "This quarter"
          description: "Within 3 months"
        - label: "This half"
          description: "Within 6 months"
        - label: "This year"
          description: "Within 12 months"
        - label: "No hard deadline"
          description: "Flexible timing"
```

**Follow-Up if Urgent:**
```
IF response in ["ASAP", "This quarter"]:
  AskUserQuestion:
    questions:
      - question: "Is this deadline negotiable?"
        header: "Fixed?"
        multiSelect: false
        options:
          - label: "Fixed - regulatory/contractual"
            description: "Cannot miss this date"
          - label: "Fixed - business event"
            description: "Tied to launch/event"
          - label: "Preferred but flexible"
            description: "Can adjust if needed"

  AskUserQuestion:
    questions:
      - question: "What happens if the deadline is missed?"
        header: "Consequence"
        multiSelect: false
        options:
          - label: "Significant penalties"
            description: "Financial or legal impact"
          - label: "Missed opportunity"
            description: "Business impact but no penalty"
          - label: "Minor inconvenience"
            description: "Not critical"
```

**Output:**
```yaml
session.constraints.timeline:
  target: "This quarter"
  flexibility: "Fixed - business event"
  consequence: "Missed opportunity"
```

---

## Step 4.3: Resource Constraints

**Purpose:** Understand team and skill availability

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "What team or resources are available for this initiative?"
      header: "Resources"
      multiSelect: false
      options:
        - label: "Dedicated team available"
          description: "Have people assigned"
        - label: "Shared resources"
          description: "People split across projects"
        - label: "Need to hire/contract"
          description: "No current resources"
        - label: "Not sure"
          description: "Resource plan unclear"
```

**Follow-Up:**
```
IF response == "Dedicated team available" OR response == "Shared resources":
  AskUserQuestion:
    questions:
      - question: "Approximately how many people?"
        header: "Team Size"
        multiSelect: false
        options:
          - label: "1-2 people"
            description: "Small team"
          - label: "3-5 people"
            description: "Medium team"
          - label: "6-10 people"
            description: "Large team"
          - label: "10+ people"
            description: "Enterprise scale"

# Skill gaps
AskUserQuestion:
  questions:
    - question: "Are there any skill gaps that would need to be filled?"
      header: "Skill Gaps"
      multiSelect: true
      options:
        - label: "Technical skills"
          description: "Programming, architecture, etc."
        - label: "Domain expertise"
          description: "Industry/business knowledge"
        - label: "Project management"
          description: "Planning and execution"
        - label: "No significant gaps"
          description: "Team is capable"
```

**Output:**
```yaml
session.constraints.resources:
  availability: "Shared resources"
  team_size: "3-5 people"
  skill_gaps: ["Technical skills"]
```

---

## Step 4.4: Technical Constraints

**Purpose:** Understand technology limits

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "Are there technical constraints we need to work within?"
      header: "Tech Limits"
      multiSelect: true
      options:
        - label: "Must integrate with existing systems"
          description: "Legacy system requirements"
        - label: "Must use specific technology"
          description: "Technology mandates"
        - label: "Must meet security standards"
          description: "Security/compliance requirements"
        - label: "Must work on-premise"
          description: "No cloud allowed"
        - label: "No major constraints"
          description: "Greenfield opportunity"
```

**Follow-Up for Integrations:**
```
IF "Must integrate with existing systems" selected:
  AskUserQuestion:
    questions:
      - question: "Which systems must be integrated?"
        header: "Systems"
        multiSelect: false
        options:
          - label: "Let me list them"
            description: "I'll name the systems"

  # Capture system names
  Store in session.constraints.technical.systems[]
```

**Follow-Up for Security:**
```
IF "Must meet security standards" selected:
  AskUserQuestion:
    questions:
      - question: "Which security or compliance standards apply?"
        header: "Standards"
        multiSelect: true
        options:
          - label: "SOC 2"
            description: "Security trust principles"
          - label: "HIPAA"
            description: "Healthcare data"
          - label: "PCI-DSS"
            description: "Payment card data"
          - label: "GDPR"
            description: "EU data protection"
          - label: "Internal security policy"
            description: "Company-specific"
```

**Output:**
```yaml
session.constraints.technical:
  requirements:
    - "Must integrate with existing systems"
    - "Must meet security standards"
  systems: ["SAP", "Salesforce", "Legacy ERP"]
  security_standards: ["SOC 2", "Internal security policy"]
  deployment: "cloud_allowed"
```

---

## Step 4.5: Organizational Constraints

**Purpose:** Understand political and cultural limits

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "Are there organizational or political constraints?"
      header: "Org Limits"
      multiSelect: true
      options:
        - label: "Requires executive approval"
          description: "C-level sign-off needed"
        - label: "Union considerations"
          description: "Labor agreements to consider"
        - label: "Change resistance expected"
          description: "Cultural barriers"
        - label: "Regulatory requirements"
          description: "Industry regulations"
        - label: "None significant"
          description: "Organization is supportive"
```

**Follow-Up for Change Resistance:**
```
IF "Change resistance expected" selected:
  AskUserQuestion:
    questions:
      - question: "Where is the resistance likely to come from?"
        header: "Source"
        multiSelect: true
        options:
          - label: "End users"
            description: "People who use current system"
          - label: "IT department"
            description: "Technical teams"
          - label: "Management"
            description: "Middle management"
          - label: "Executive leadership"
            description: "Senior leaders"

  AskUserQuestion:
    questions:
      - question: "What's driving the resistance?"
        header: "Cause"
        multiSelect: true
        options:
          - label: "Fear of job loss"
            description: "Automation concerns"
          - label: "Comfort with current system"
            description: "Change fatigue"
          - label: "Previous bad experiences"
            description: "Past failures"
          - label: "Unclear benefits"
            description: "Don't see the value"
```

**Output:**
```yaml
session.constraints.organizational:
  requirements:
    - "Requires executive approval"
    - "Change resistance expected"
  resistance_sources: ["End users", "IT department"]
  resistance_causes: ["Comfort with current system", "Previous bad experiences"]
```

---

## Phase 4 Summary Output

At end of Phase 4, session should contain:

```yaml
constraint_matrix:
  budget:
    range: "$50K - $200K"
    flexibility: "Somewhat flexible"
    ongoing_costs: "Separate budget"

  timeline:
    target: "This quarter"
    flexibility: "Fixed - business event"
    consequence: "Missed opportunity"

  resources:
    availability: "Shared resources"
    team_size: "3-5 people"
    skill_gaps: ["Technical skills"]

  technical:
    requirements: ["Must integrate", "Must meet security"]
    systems: ["SAP", "Salesforce"]
    security_standards: ["SOC 2"]

  organizational:
    requirements: ["Executive approval", "Change resistance"]
    resistance_sources: ["End users"]
    resistance_causes: ["Previous bad experiences"]
```

---

## Transition to Phase 5

**Display:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 4 Complete: Constraint Discovery
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Budget: {range} ({flexibility})
Timeline: {target} ({flexibility})
Resources: {team_size} ({availability})
Technical: {count} constraints
Organizational: {count} constraints

Proceeding to Phase 5: Hypothesis Formation...
```

---

## Common Issues and Recovery

| Issue | Symptom | Recovery |
|-------|---------|----------|
| Budget unknown | "Not defined yet" | Note as TBD, emphasize importance |
| All constraints "flexible" | No real limits | Probe for hidden constraints |
| Too many constraints | 20+ items | Group by category, prioritize blockers |
| Conflicting constraints | Budget vs timeline | Highlight tradeoff for prioritization |

---

## Success Criteria

- [ ] Budget range identified (even if TBD)
- [ ] Timeline identified
- [ ] Resource availability assessed
- [ ] Technical constraints documented
- [ ] Organizational constraints documented

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
