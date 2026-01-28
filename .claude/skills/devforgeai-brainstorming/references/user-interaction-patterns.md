---
id: user-interaction-patterns
title: User Interaction Patterns for Brainstorming Sessions
version: "1.0"
created: 2025-12-21
status: Published
audience: devforgeai-brainstorming skill (Internal Use)
---

# User Interaction Patterns

**Purpose:** Question templates and interaction patterns for the 7-phase brainstorming workflow. Provides copy-paste ready AskUserQuestion templates for each phase.

**Quick Links:**
- [Section 1: Question Design Principles](#section-1-question-design-principles)
- [Section 2: Phase-Specific Templates](#section-2-phase-specific-templates)
- [Section 3: Follow-Up Patterns](#section-3-follow-up-patterns)
- [Section 4: Session Management Questions](#section-4-session-management-questions)

---

## Section 1: Question Design Principles

### 1.1 Core Principles

1. **Open-Ended First, Narrow Later**
   - Start with broad questions to understand context
   - Use options to guide when user seems uncertain
   - Follow up with specific questions to clarify

2. **Progressive Disclosure**
   - Don't ask all questions upfront
   - Let previous answers inform subsequent questions
   - Skip irrelevant questions based on context

3. **Respect User Time**
   - Target 30-50 questions per session
   - Offer shortcuts when possible
   - Allow "I don't know" responses

4. **Capture Both Facts and Feelings**
   - Ask what IS (current state)
   - Ask what SHOULD BE (desired state)
   - Ask what HURTS (pain points)
   - Ask what MATTERS (priorities)

### 1.2 Question Types

| Type | Use Case | Example |
|------|----------|---------|
| **Discovery** | Open exploration | "What business problem are you trying to solve?" |
| **Clarification** | Narrow ambiguity | "When you say 'slow', what does that mean in minutes/hours?" |
| **Validation** | Confirm understanding | "So the main goal is reducing order processing time, correct?" |
| **Prioritization** | Rank importance | "Which of these is most critical to solve first?" |
| **Impact** | Quantify effects | "How much does this cost the business per month?" |
| **Stakeholder** | Identify people | "Who else is affected by this problem?" |

### 1.3 Question Formatting Rules

```yaml
# Standard AskUserQuestion format for brainstorming
AskUserQuestion:
  questions:
    - question: "{Clear, specific question ending with ?}"
      header: "{2-12 char label}"
      multiSelect: {true|false}
      options:
        - label: "{Short option text}"
          description: "{Clarifying detail}"
```

**Header Guidelines:**
- Max 12 characters
- Use nouns or short phrases
- Examples: "Topic", "Budget", "Timeline", "Priority", "Stakeholder"

**Option Guidelines:**
- 2-4 options per question
- Include escape hatch ("Other", "Not sure", "Skip")
- Put recommended option first with "(Recommended)" suffix

---

## Section 2: Phase-Specific Templates

### 2.1 Phase 0: Session Initialization

#### INIT-001: Topic Discovery (if not provided)

```yaml
AskUserQuestion:
  questions:
    - question: "What business problem or opportunity would you like to explore?"
      header: "Topic"
      multiSelect: false
      options:
        - label: "I have a specific problem"
          description: "Something isn't working well and needs fixing"
        - label: "I see an opportunity"
          description: "Something could be better or added"
        - label: "I'm not sure yet"
          description: "Help me discover it through questions"
```

#### INIT-002: Topic Description

```yaml
AskUserQuestion:
  questions:
    - question: "Describe the problem or opportunity in a few sentences:"
      header: "Description"
      multiSelect: false
      options:
        - label: "Let me describe it"
          description: "I'll type out the details"
        - label: "Ask me questions"
          description: "Guide me with specific questions"
```

---

### 2.2 Phase 1: Stakeholder Discovery

#### STAKE-001: Primary Decision Maker

```yaml
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

#### STAKE-002: End Users

```yaml
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

#### STAKE-003: Affected Parties

```yaml
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

#### STAKE-004: Stakeholder Goals

```yaml
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
```

#### STAKE-005: Stakeholder Concerns

```yaml
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
```

---

### 2.3 Phase 2: Problem Exploration

#### PROB-001: Current State

```yaml
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

#### PROB-002: Process Duration

```yaml
AskUserQuestion:
  questions:
    - question: "How long does the current process take?"
      header: "Duration"
      multiSelect: false
      options:
        - label: "Minutes"
          description: "Under 30 minutes per instance"
        - label: "Hours"
          description: "Takes several hours"
        - label: "Days"
          description: "Takes one or more days"
        - label: "Weeks or longer"
          description: "Extended timeline"
```

#### PROB-003: Error Rate

```yaml
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

#### PROB-004: 5 Whys - Level 1

```yaml
AskUserQuestion:
  questions:
    - question: "Why does [PROBLEM] happen?"
      header: "Why 1"
      multiSelect: false
      options:
        - label: "Let me explain"
          description: "I'll describe the cause"
        - label: "I don't know"
          description: "The root cause is unclear"
```

#### PROB-005: Pain Point Severity

```yaml
AskUserQuestion:
  questions:
    - question: "How severe is [PAIN_POINT] for your business?"
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
```

#### PROB-006: Business Impact

```yaml
AskUserQuestion:
  questions:
    - question: "What type of business impact does [PAIN_POINT] cause?"
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
```

#### PROB-007: Previous Attempts

```yaml
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

---

### 2.4 Phase 3: Opportunity Mapping

#### OPP-001: Research Option

```yaml
AskUserQuestion:
  questions:
    - question: "Would you like to include market research in this brainstorm?"
      header: "Research"
      multiSelect: false
      options:
        - label: "Yes - Research competitors & trends (Recommended)"
          description: "I'll search for market data and competitor approaches"
        - label: "Skip - Use internal knowledge only"
          description: "Faster session, relies on what you know"
```

#### OPP-002: Ideal Future State

```yaml
AskUserQuestion:
  questions:
    - question: "If you had unlimited resources, what would the ideal solution look like?"
      header: "Ideal State"
      multiSelect: false
      options:
        - label: "Let me describe it"
          description: "I have a vision to share"
        - label: "Help me brainstorm"
          description: "Guide me with questions"
```

#### OPP-003: Technology Ideas

```yaml
AskUserQuestion:
  questions:
    - question: "Are there any technologies you've heard about that might help?"
      header: "Tech Ideas"
      multiSelect: true
      options:
        - label: "AI/ML automation"
          description: "Artificial intelligence solutions"
        - label: "Process automation (RPA)"
          description: "Robotic process automation"
        - label: "Cloud migration"
          description: "Moving to cloud infrastructure"
        - label: "Integration/APIs"
          description: "Connecting existing systems"
```

#### OPP-004: Success Definition

```yaml
AskUserQuestion:
  questions:
    - question: "How would you measure success for this initiative?"
      header: "Success"
      multiSelect: true
      options:
        - label: "Time saved"
          description: "Reduce hours/days on tasks"
        - label: "Money saved"
          description: "Reduce operational costs"
        - label: "Revenue increased"
          description: "Generate more income"
        - label: "Errors reduced"
          description: "Improve accuracy/quality"
```

---

### 2.5 Phase 4: Constraint Discovery

#### CONST-001: Budget Range

```yaml
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
```

#### CONST-002: Timeline

```yaml
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
```

#### CONST-003: Deadline Flexibility

```yaml
AskUserQuestion:
  questions:
    - question: "Is the timeline deadline negotiable?"
      header: "Flexibility"
      multiSelect: false
      options:
        - label: "Fixed - cannot change"
          description: "Hard deadline (regulatory, contractual)"
        - label: "Somewhat flexible"
          description: "Can adjust with justification"
        - label: "Very flexible"
          description: "No hard deadline"
```

#### CONST-004: Technical Constraints

```yaml
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
```

#### CONST-005: Organizational Constraints

```yaml
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
```

---

### 2.6 Phase 5: Hypothesis Formation

#### HYPO-001: Critical Assumptions

```yaml
AskUserQuestion:
  questions:
    - question: "Which of these assumptions are critical to validate?"
      header: "Critical"
      multiSelect: true
      options:
        - label: "[Assumption 1 from session]"
          description: "High risk if wrong"
        - label: "[Assumption 2 from session]"
          description: "Moderate risk if wrong"
        - label: "[Assumption 3 from session]"
          description: "Lower risk if wrong"
```

#### HYPO-002: Risk Assessment

```yaml
AskUserQuestion:
  questions:
    - question: "What happens if [ASSUMPTION] turns out to be wrong?"
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
          description: "Assumption isn't critical"
```

---

### 2.7 Phase 6: Prioritization

#### PRIO-001: MoSCoW Classification

```yaml
AskUserQuestion:
  questions:
    - question: "How would you classify [CAPABILITY]?"
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
```

#### PRIO-002: Impact Assessment

```yaml
AskUserQuestion:
  questions:
    - question: "What is the expected impact of [OPPORTUNITY]?"
      header: "Impact"
      multiSelect: false
      options:
        - label: "High impact"
          description: "Significant business value"
        - label: "Medium impact"
          description: "Moderate business value"
        - label: "Low impact"
          description: "Minor business value"
```

#### PRIO-003: Effort Assessment

```yaml
AskUserQuestion:
  questions:
    - question: "How much effort do you estimate [OPPORTUNITY] would require?"
      header: "Effort"
      multiSelect: false
      options:
        - label: "Low effort"
          description: "Days to a week"
        - label: "Medium effort"
          description: "Weeks to a month"
        - label: "High effort"
          description: "Months of work"
```

---

### 2.8 Phase 7: Handoff Synthesis

#### SYNTH-001: Validation Check

```yaml
AskUserQuestion:
  questions:
    - question: "Does this summary accurately capture what we discussed?"
      header: "Validation"
      multiSelect: false
      options:
        - label: "Yes, looks accurate"
          description: "Proceed with generating document"
        - label: "Needs minor corrections"
          description: "Small adjustments needed"
        - label: "Missing something important"
          description: "Key information was missed"
```

#### SYNTH-002: Next Steps Preference

```yaml
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
        - label: "Schedule follow-up"
          description: "Continue this brainstorm later"
```

---

## Section 3: Follow-Up Patterns

### 3.1 When User Says "I Don't Know"

**Pattern:** Offer research or skip with note

```yaml
AskUserQuestion:
  questions:
    - question: "Would you like me to research this, or should we skip it for now?"
      header: "Unknown"
      multiSelect: false
      options:
        - label: "Research it"
          description: "I'll look for information"
        - label: "Skip for now"
          description: "Mark as TBD and continue"
        - label: "Ask someone else"
          description: "I know who might know"
```

### 3.2 When User Gives Vague Answer

**Pattern:** Ask for specifics

```yaml
AskUserQuestion:
  questions:
    - question: "Can you be more specific about [VAGUE_TERM]? What does that mean in measurable terms?"
      header: "Clarify"
      multiSelect: false
      options:
        - label: "Let me explain"
          description: "I'll provide more detail"
        - label: "Give me examples"
          description: "Help me think of specifics"
```

### 3.3 When User Contradicts Earlier Answer

**Pattern:** Acknowledge and clarify

```yaml
AskUserQuestion:
  questions:
    - question: "Earlier you mentioned [PREVIOUS_ANSWER], but now you said [NEW_ANSWER]. Which is more accurate?"
      header: "Clarify"
      multiSelect: false
      options:
        - label: "The first answer"
          description: "Earlier was correct"
        - label: "The new answer"
          description: "I changed my mind / realized"
        - label: "Both are true"
          description: "The situation is complex"
```

### 3.4 When User Wants to Go Back

**Pattern:** Offer correction options

```yaml
AskUserQuestion:
  questions:
    - question: "What would you like to correct or revisit?"
      header: "Revise"
      multiSelect: true
      options:
        - label: "Stakeholder information"
          description: "Who's involved"
        - label: "Problem definition"
          description: "What the issue is"
        - label: "Constraints"
          description: "Budget, timeline, technical limits"
        - label: "Priorities"
          description: "What matters most"
```

---

## Section 4: Session Management Questions

### 4.1 Context Window Warning

```yaml
AskUserQuestion:
  questions:
    - question: "Context window is approximately [PERCENT]% full. Would you like to:"
      header: "Session"
      multiSelect: false
      options:
        - label: "Continue in this session"
          description: "Proceed to next phase"
        - label: "Save and continue later"
          description: "Create checkpoint and exit"
```

### 4.2 Session Interruption

```yaml
AskUserQuestion:
  questions:
    - question: "Would you like to save your progress and continue later?"
      header: "Pause"
      multiSelect: false
      options:
        - label: "Yes, save checkpoint"
          description: "I'll resume later with /brainstorm --resume"
        - label: "No, continue now"
          description: "I want to finish this session"
```

### 4.3 Phase Completion

```yaml
AskUserQuestion:
  questions:
    - question: "Phase [N] complete. Ready to proceed to [NEXT_PHASE]?"
      header: "Continue"
      multiSelect: false
      options:
        - label: "Yes, continue"
          description: "Proceed to next phase"
        - label: "Wait, I have more"
          description: "I want to add more to this phase"
        - label: "Save checkpoint"
          description: "Pause and continue later"
```

---

## Section 5: Template Selection Guide

### 5.1 Quick Reference by Phase

| Phase | Primary Templates | When to Use |
|-------|-------------------|-------------|
| 0: Init | INIT-001, INIT-002 | Session start |
| 1: Stakeholder | STAKE-001 to STAKE-005 | Identifying people |
| 2: Problem | PROB-001 to PROB-007 | Understanding issue |
| 3: Opportunity | OPP-001 to OPP-004 | Exploring solutions |
| 4: Constraint | CONST-001 to CONST-005 | Finding limits |
| 5: Hypothesis | HYPO-001, HYPO-002 | Testing assumptions |
| 6: Priority | PRIO-001 to PRIO-003 | Ranking importance |
| 7: Synthesis | SYNTH-001, SYNTH-002 | Finishing up |

### 5.2 Template Count per Phase

| Phase | Templates | Estimated Questions |
|-------|-----------|---------------------|
| Phase 0 | 2 | 1-2 |
| Phase 1 | 5 | 5-10 |
| Phase 2 | 7 | 8-15 |
| Phase 3 | 4 | 5-10 |
| Phase 4 | 5 | 5-8 |
| Phase 5 | 2 | 3-6 |
| Phase 6 | 3 | 3-5 |
| Phase 7 | 2 | 0-2 |
| **Total** | **30** | **30-58** |

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
