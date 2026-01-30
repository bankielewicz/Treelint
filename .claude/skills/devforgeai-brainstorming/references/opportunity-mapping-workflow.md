---
id: opportunity-mapping-workflow
title: Phase 3 - Opportunity Mapping Workflow
version: "1.0"
created: 2025-12-21
status: Published
phase: 3
estimated_duration: "8-12 minutes"
question_count: "5-10"
---

# Phase 3: Opportunity Mapping

Explore WHAT COULD BE through blue-sky thinking, market research, and solution ideation.

## Overview

| Attribute | Value |
|-----------|-------|
| **Purpose** | Identify potential solutions and opportunities |
| **Duration** | 8-12 minutes |
| **Questions** | 5-10 |
| **Output** | Opportunity canvas with solutions, research, and vision |
| **Optional** | Market research via internet-sleuth subagent |

---

## Step 3.1: Research Option

**Purpose:** Decide whether to include market research

**Tool Invocations:**
```
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

**Decision Logic:**
```
IF response == "Yes - Research":
  session.research_enabled = true
  GOTO Step 3.2 (Conduct Research)
ELSE:
  session.research_enabled = false
  session.market_research = null
  SKIP Step 3.2, GOTO Step 3.3
```

---

## Step 3.2: Conduct Market Research (Optional)

**Purpose:** Gather external data on solutions and trends

**Trigger:** User selected research option

**Subagent Invocation:**
```
Display: "Researching market solutions and competitor approaches..."

Task(
  subagent_type="internet-sleuth",
  prompt="Research market trends and competitor approaches for:
          Problem: {session.problem_statement}
          Industry: {inferred from stakeholders}

          Find:
          1. How competitors solve this problem
          2. Available technology solutions
          3. Market trends in this space
          4. Best practices and case studies

          Return structured findings."
)
```

**Error Handling:**
```
TRY:
  research_result = await subagent_task
  session.market_research = research_result
EXCEPT TimeoutError:
  Display: "Research taking too long. Skipping to continue."
  session.market_research = null
  session.research_enabled = false
EXCEPT NetworkError:
  Display: "Unable to access research. Continuing with internal knowledge."
  session.market_research = null
  session.research_enabled = false
```

**Output:**
```yaml
session.market_research:
  competitors:
    - name: "Competitor A"
      approach: "Uses AI-powered automation"
      pros: ["Fast", "Scalable"]
      cons: ["Expensive", "Complex setup"]
  trends:
    - "Low-code automation platforms"
    - "Integration-as-a-service"
  technologies:
    - "Zapier, Make.com for integration"
    - "UiPath for RPA"
  case_studies:
    - "Company X reduced processing time by 80%"
```

---

## Step 3.3: Blue-Sky Visioning

**Purpose:** Imagine the ideal future state without constraints

**Tool Invocations:**
```
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

**If User Needs Help:**
```
IF response == "Help me brainstorm":
  AskUserQuestion:
    questions:
      - question: "In an ideal world, how fast would this process be?"
        header: "Ideal Speed"
        multiSelect: false
        options:
          - label: "Instant"
            description: "Happens immediately"
          - label: "Minutes"
            description: "Done in a few minutes"
          - label: "Same day"
            description: "Completed within hours"

  AskUserQuestion:
    questions:
      - question: "In an ideal world, who would do this work?"
        header: "Ideal Actor"
        multiSelect: false
        options:
          - label: "Fully automated"
            description: "No human intervention"
          - label: "Minimal human oversight"
            description: "Humans review exceptions"
          - label: "Assisted by technology"
            description: "Humans do it faster with help"
```

**Output:**
- `session.ideal_state` - Description of ideal future state

---

## Step 3.4: Success Vision

**Purpose:** Define what success looks like

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "What would change in your organization if this problem was completely solved?"
      header: "Vision"
      multiSelect: false
      options:
        - label: "Let me describe"
          description: "I'll paint the picture"

# Follow-up for specific outcomes
AskUserQuestion:
  questions:
    - question: "How would you measure success for this initiative?"
      header: "Success Metrics"
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
        - label: "Customer satisfaction"
          description: "Improve NPS or retention"
```

**Output:**
```yaml
session.success_vision:
  description: "Orders processed in under 5 minutes with zero errors"
  metrics:
    - "Time saved"
    - "Errors reduced"
```

---

## Step 3.5: Technology Opportunities

**Purpose:** Explore technology solutions

**Tool Invocations:**
```
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
        - label: "Mobile/Web apps"
          description: "New user interfaces"
        - label: "Data analytics"
          description: "Better insights and reporting"
```

**Follow-Up:**
```
IF len(selected_technologies) > 0:
  FOR each technology:
    AskUserQuestion:
      questions:
        - question: "Why do you think {technology} might help?"
          header: "Rationale"
          multiSelect: false
          options:
            - label: "Let me explain"
              description: "I have specific reasons"
            - label: "Just heard about it"
              description: "Worth exploring"
```

**Output:**
- `session.technology_ideas[]` - Technology options with rationale

---

## Step 3.6: Adjacent Opportunities

**Purpose:** Find related problems that could be solved together

**Tool Invocations:**
```
AskUserQuestion:
  questions:
    - question: "Are there related problems that could be solved at the same time?"
      header: "Related"
      multiSelect: false
      options:
        - label: "Yes, several"
          description: "Multiple related issues"
        - label: "Maybe one or two"
          description: "A few possibilities"
        - label: "No, this is isolated"
          description: "Problem is independent"
        - label: "Not sure"
          description: "Haven't thought about it"
```

**Follow-Up if Adjacent:**
```
IF response in ["Yes, several", "Maybe one or two"]:
  # Capture related problems
  FOR i in [1..3]:
    related_problem = user_input
    IF related_problem is empty:
      BREAK

    AskUserQuestion:
      questions:
        - question: "How is '{related_problem}' connected to the main problem?"
          header: "Connection"
          multiSelect: false
          options:
            - label: "Same root cause"
              description: "Both stem from same issue"
            - label: "Same stakeholders"
              description: "Affects same people"
            - label: "Same system"
              description: "Same technology involved"
            - label: "Same process"
              description: "Part of same workflow"

    Store in session.adjacent_opportunities[]
```

**Output:**
```yaml
session.adjacent_opportunities:
  - problem: "Reporting takes too long"
    connection: "Same system"
    synergy: "Solving integration helps both"
```

---

## Step 3.7: Compile Opportunities

**Purpose:** List all identified opportunities

**Logic:**
```
opportunities = []

# From user vision
opportunities.append({
  source: "user_vision",
  description: session.ideal_state,
  type: "solution"
})

# From technology ideas
FOR tech in session.technology_ideas:
  opportunities.append({
    source: "technology",
    description: f"Implement {tech.name}",
    rationale: tech.rationale
  })

# From market research (if available)
IF session.market_research:
  FOR competitor in session.market_research.competitors:
    opportunities.append({
      source: "competitor",
      description: f"Adopt {competitor.approach}",
      pros: competitor.pros,
      cons: competitor.cons
    })

# From adjacent problems
FOR adjacent in session.adjacent_opportunities:
  opportunities.append({
    source: "adjacent",
    description: f"Also solve: {adjacent.problem}",
    synergy: adjacent.synergy
  })

session.opportunities = opportunities
```

**Display:**
```
Display:
"I've identified {len(opportunities)} potential opportunities:

1. {opportunity_1.description}
2. {opportunity_2.description}
...

These will be prioritized in Phase 6."
```

---

## Step 3.8: Context Window Check

**Purpose:** Offer checkpoint (this is often the heaviest phase)

**Trigger:** After completing opportunity mapping

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
            description: "Proceed to Phase 4 (Constraint Discovery)"
          - label: "Save and continue later"
            description: "Create checkpoint and exit"

  IF response == "Save and continue later":
    Generate checkpoint
    Display resume instructions
    EXIT skill
```

---

## Phase 3 Summary Output

At end of Phase 3, session should contain:

```yaml
opportunity_canvas:
  research_conducted: true
  market_research:
    competitors: [...]
    trends: [...]
    technologies: [...]

  ideal_state: "Orders processed in under 5 minutes automatically"

  success_vision:
    description: "Zero-touch order processing"
    metrics: ["Time saved", "Errors reduced"]

  technology_ideas:
    - name: "Integration/APIs"
      rationale: "Connect existing systems"
    - name: "AI/ML automation"
      rationale: "Automate data extraction"

  adjacent_opportunities:
    - problem: "Reporting delays"
      connection: "Same system"
      synergy: "One integration solves both"

  opportunities:
    - source: "user_vision"
      description: "Automated order processing"
    - source: "technology"
      description: "Implement API integration layer"
    - source: "competitor"
      description: "Adopt cloud-based automation like Competitor X"
    - source: "adjacent"
      description: "Also automate reporting"
```

---

## Transition to Phase 4

**Display:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 3 Complete: Opportunity Mapping
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Research: {conducted/skipped}
{IF conducted: Competitors analyzed: {count}}

Opportunities Identified: {count}
  - {opp_1}
  - {opp_2}
  - {opp_3}

Adjacent Problems: {count}

Proceeding to Phase 4: Constraint Discovery...
```

---

## Common Issues and Recovery

| Issue | Symptom | Recovery |
|-------|---------|----------|
| Research times out | Subagent takes too long | Skip research, continue |
| No technology ideas | User unfamiliar with options | Provide common suggestions |
| Ideal state too vague | "Make it better" | Ask for specific metrics |
| Too many opportunities | 20+ identified | Group similar ones, prioritize later |

---

## Success Criteria

- [ ] Research decision made (conducted or skipped)
- [ ] Ideal state described
- [ ] Success metrics identified
- [ ] At least 3 opportunities identified
- [ ] Adjacent problems explored
- [ ] Context window check completed

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
