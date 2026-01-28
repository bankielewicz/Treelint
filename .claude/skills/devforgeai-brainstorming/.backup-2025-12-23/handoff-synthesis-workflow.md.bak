---
id: handoff-synthesis-workflow
title: Phase 7 - Handoff Synthesis Workflow
version: "1.0"
created: 2025-12-21
status: Published
phase: 7
estimated_duration: "3-5 minutes"
question_count: "0-2"
---

# Phase 7: Handoff Synthesis

Generate the AI-consumable brainstorm document for handoff to `/ideate`.

## Overview

| Attribute | Value |
|-----------|-------|
| **Purpose** | Compile all session data into structured output document |
| **Duration** | 3-5 minutes |
| **Questions** | 0-2 (validation only) |
| **Output** | BRAINSTORM-{NNN}.brainstorm.md with YAML frontmatter |
| **Next Step** | /ideate command (auto-detects brainstorm) |

---

## Step 7.1: Compile Session Data

**Purpose:** Gather all outputs from Phases 1-6

**Logic:**
```
# Collect from all phases
compiled_data = {
  # Phase 1: Stakeholders
  stakeholder_map: session.stakeholder_map,

  # Phase 2: Problem
  problem_statement: session.problem_statement,
  root_causes: session.root_causes,
  current_state: session.current_state,
  pain_points: session.pain_points,
  failed_solutions: session.failed_solutions,

  # Phase 3: Opportunities
  research_conducted: session.research_enabled,
  market_research: session.market_research,
  ideal_state: session.ideal_state,
  success_vision: session.success_vision,
  technology_ideas: session.technology_ideas,
  opportunities: session.opportunities,
  adjacent_opportunities: session.adjacent_opportunities,

  # Phase 4: Constraints
  constraints: session.constraints,

  # Phase 5: Hypotheses
  hypotheses: session.hypotheses,

  # Phase 6: Prioritization
  moscow: session.moscow,
  impact_effort: session.impact_effort,
  recommended_sequence: session.recommended_sequence
}
```

---

## Step 7.2: Generate Confidence Level

**Purpose:** Assess overall confidence in brainstorm outputs

**Logic:**
```
# Calculate confidence based on completeness
confidence_factors = []

# Factor 1: Stakeholder completeness
IF len(stakeholder_map.primary) >= 1 AND len(stakeholder_map.secondary) >= 1:
  confidence_factors.append("stakeholders_complete")

# Factor 2: Root cause depth
IF root_cause_level >= 3:
  confidence_factors.append("root_cause_explored")

# Factor 3: Constraint clarity
IF constraints.budget.range != "Not defined yet" AND constraints.timeline.target != null:
  confidence_factors.append("constraints_clear")

# Factor 4: Hypothesis validation plan
IF len([h for h in hypotheses if h.validation_method]) >= 1:
  confidence_factors.append("hypotheses_testable")

# Factor 5: Priority alignment
IF len(moscow.must_have) >= 1 AND len(moscow.must_have) <= 5:
  confidence_factors.append("priorities_focused")

# Calculate confidence level
IF len(confidence_factors) >= 4:
  confidence_level = "HIGH"
ELSE IF len(confidence_factors) >= 2:
  confidence_level = "MEDIUM"
ELSE:
  confidence_level = "LOW"

session.confidence_level = confidence_level
session.confidence_factors = confidence_factors
```

---

## Step 7.3: Generate Short Name

**Purpose:** Create URL-friendly identifier for file name

**Logic:**
```
# Extract key words from topic
topic_words = session.topic.lower().split()

# Remove common words
stop_words = ["the", "a", "an", "and", "or", "for", "to", "of", "in", "on"]
key_words = [w for w in topic_words if w not in stop_words]

# Take first 3 key words
short_name = "-".join(key_words[:3])

# Sanitize for filename
short_name = sanitize_filename(short_name)

session.short_name = short_name
```

---

## Step 7.4: Generate YAML Frontmatter

**Purpose:** Create AI-consumable metadata header

**Template:**
```yaml
---
# DevForgeAI Brainstorm Session
id: {brainstorm_id}
title: "{topic}"
status: Complete
created: {date}
facilitator: DevForgeAI
session_duration: "{duration} minutes"
question_count: {total_questions}

# Core Outputs (AI-Consumable for /ideate)
problem_statement: "{problem_statement}"
target_outcome: "{ideal_state}"
recommended_approach: "{first_must_have_description}"
confidence_level: "{confidence_level}"

# Stakeholder Summary
primary_stakeholder: "{primary_stakeholder_name}"
user_personas:
  - "{persona_1}: {role} - {goal}"
  - "{persona_2}: {role} - {goal}"

# Constraint Summary
budget_range: "{budget_range}"
timeline: "{timeline_target}"
hard_constraints:
  - "{constraint_1}"
  - "{constraint_2}"

# Hypothesis Summary
critical_assumptions:
  - "{hypothesis_1}"
  - "{hypothesis_2}"

# Prioritization Summary
must_have_capabilities:
  - "{must_have_1}"
  - "{must_have_2}"
nice_to_have:
  - "{should_have_1}"
---
```

---

## Step 7.5: Generate Document Body

**Purpose:** Create human-readable markdown sections

**Template Structure:**
```markdown
# {Title}

## Executive Summary
{2-3 paragraph summary}

---

## 1. Stakeholder Analysis
### 1.1 Stakeholder Map
{table}
### 1.2 Goals & Concerns
{details}
### 1.3 Conflicts
{if any}

---

## 2. Problem Analysis
### 2.1 Problem Statement
{blockquote}
### 2.2 Root Cause Analysis
{5 whys table}
### 2.3 Current State
{metrics}
### 2.4 Pain Points
{table with severity}
### 2.5 Failed Solutions
{if any}

---

## 3. Opportunity Canvas
### 3.1 Blue-Sky Vision
{ideal state}
### 3.2 Market Research
{if conducted}
### 3.3 Opportunities
{table}
### 3.4 Adjacent Opportunities
{if any}

---

## 4. Constraint Matrix
### 4.1 Budget
### 4.2 Timeline
### 4.3 Resources
### 4.4 Technical
### 4.5 Organizational

---

## 5. Hypothesis Register
{table with validation methods}

---

## 6. Prioritized Opportunities
### 6.1 MoSCoW Classification
### 6.2 Impact-Effort Matrix
### 6.3 Recommended Sequence

---

## 7. Handoff to Ideation
### 7.1 Summary for /ideate
### 7.2 Recommended Next Steps
### 7.3 Open Questions

---

## Appendix A: Session Metadata
## Appendix B: Raw Session Data
```

---

## Step 7.6: Write Output File

**Purpose:** Save brainstorm document to file system

**Tool Invocations:**
```
output_path = f"devforgeai/specs/brainstorms/{brainstorm_id}-{short_name}.brainstorm.md"

Write(
  file_path=output_path,
  content=frontmatter + document_body
)
```

**Error Handling:**
```
TRY:
  Write(file_path=output_path, content=content)
  session.output_file = output_path
EXCEPT PermissionError:
  # Try alternative location
  alt_path = f"./brainstorms/{brainstorm_id}-{short_name}.brainstorm.md"
  Write(file_path=alt_path, content=content)
  session.output_file = alt_path
EXCEPT:
  # Display content for manual save
  Display: "Could not write file. Please copy the following:"
  Display: content
```

---

## Step 7.7: Delete Checkpoint (if exists)

**Purpose:** Clean up checkpoint file after successful completion

**Logic:**
```
IF session.checkpoint_file exists:
  # Checkpoint no longer needed
  Delete(file_path=session.checkpoint_file)
  Display: "Checkpoint removed (session complete)"
```

---

## Step 7.8: Validate with User

**Purpose:** Confirm output is accurate

**Tool Invocations:**
```
Display:
"Here's a summary of the brainstorm:

Problem: {problem_statement}
Stakeholders: {count}
Opportunities: {count}
Must-Haves: {count}
Confidence: {confidence_level}

Document saved to: {output_path}"

AskUserQuestion:
  questions:
    - question: "Does this summary accurately capture what we discussed?"
      header: "Validation"
      multiSelect: false
      options:
        - label: "Yes, looks accurate"
          description: "Proceed with handoff"
        - label: "Needs minor corrections"
          description: "I'll make small edits later"
        - label: "Missing something important"
          description: "Key information was missed"
```

**Handle Corrections:**
```
IF response == "Missing something important":
  AskUserQuestion:
    questions:
      - question: "What's missing?"
        header: "Missing"
        multiSelect: false
        options:
          - label: "Let me describe"
            description: "I'll explain what's missing"

  # Add to document appendix or update relevant section
  # May need to re-generate document
```

---

## Step 7.9: Display Completion Summary

**Purpose:** Provide clear next steps

**Display Template:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Brainstorm Session Complete
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Document: {output_path}

Summary:
  - Stakeholders: {stakeholder_count} identified ({primary_count} primary)
  - Problem: {problem_statement_short}
  - Opportunities: {opportunity_count} candidates
  - Constraints: {constraint_summary}
  - Hypotheses: {hypothesis_count} to validate

Confidence: {confidence_level}
  {confidence_factor_1} ✓
  {confidence_factor_2} ✓
  ...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Next Steps
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Review brainstorm document for accuracy
2. Run /ideate to transform into formal requirements
   - The brainstorm will be automatically detected
   - Key inputs will be pre-populated
3. After ideation: /create-context for architecture

Recommended command:
  /ideate
```

---

## Step 7.10: Next Steps Prompt

**Purpose:** Guide user to next action

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "What would you like to do next?"
      header: "Next Step"
      multiSelect: false
      options:
        - label: "Proceed to /ideate"
          description: "Transform into formal requirements"
        - label: "Review document first"
          description: "I want to review before continuing"
        - label: "Share with stakeholders"
          description: "I need to get input from others"
        - label: "Done for now"
          description: "I'll continue later"
```

**Handle Response:**
```
IF response == "Proceed to /ideate":
  Display: "Run: /ideate"
  Display: "The brainstorm will be auto-detected and pre-populate key fields."

IF response == "Review document first":
  Display: f"Document location: {output_path}"
  Display: "After review, run: /ideate"

IF response == "Share with stakeholders":
  Display: f"Document to share: {output_path}"
  Display: "After stakeholder review, run: /ideate"

IF response == "Done for now":
  Display: "Your brainstorm is saved."
  Display: f"Location: {output_path}"
  Display: "When ready: /ideate"
```

---

## Phase 7 Output

**Primary Output:**
`devforgeai/specs/brainstorms/BRAINSTORM-{NNN}-{short-name}.brainstorm.md`

**File Structure:**
```
devforgeai/specs/brainstorms/
├── BRAINSTORM-001-order-automation.brainstorm.md
├── BRAINSTORM-002-customer-onboarding.brainstorm.md
└── .gitkeep
```

---

## Common Issues and Recovery

| Issue | Symptom | Recovery |
|-------|---------|----------|
| File write fails | Permission denied | Try alternative path, display content |
| Missing data | Section is empty | Mark as TBD, note in open questions |
| Low confidence | Few factors met | Highlight gaps for follow-up |
| User says incorrect | Missing info | Add to appendix, offer re-run of specific phase |

---

## Success Criteria

- [ ] All session data compiled
- [ ] Confidence level calculated
- [ ] YAML frontmatter generated
- [ ] Document body generated with all sections
- [ ] File written successfully
- [ ] Checkpoint cleaned up (if existed)
- [ ] User validated output
- [ ] Next steps displayed

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
