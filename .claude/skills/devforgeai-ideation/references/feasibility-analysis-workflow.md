# Phase 5: Feasibility & Constraints Analysis Workflow

Evaluate technical, business, and resource constraints to identify risks before implementation.

## TodoWrite - Phase Start

**At phase start, update todo list:**
```
TodoWrite([
  {"content": "Phase 5: Feasibility & Constraints Analysis", "status": "in_progress", "activeForm": "Analyzing constraints"}
])
```

## Overview

Phase 5 assesses whether requirements are technically and practically achievable given constraints. This phase identifies risks early, proposes mitigations, and validates that the solution is feasible within timeline and budget.

**Duration:** 10-15 minutes
**Questions:** Constraint validation questions
**Output:** Feasibility assessment, risk register with mitigations, constraint documentation

---

## Step 5.1: Technical Feasibility

### Load Feasibility Framework

```
Read(file_path=".claude/skills/devforgeai-ideation/references/feasibility-analysis-framework.md")
```

This reference provides comprehensive checklists for:
- Technical feasibility (can we build this?)
- Business feasibility (should we build this?)
- Resource feasibility (do we have capacity?)
- Risk assessment templates
- MVP scoping techniques

### Assess Technical Risks

```
Question: "Are there any technical constraints or concerns?"
Options:
  - "Must integrate with legacy systems"
  - "Must support offline functionality"
  - "Must work on low-bandwidth networks"
  - "Requires real-time data synchronization"
  - "Requires complex algorithms/ML"
  - "No major technical constraints"
multiSelect: true
```

**For each constraint selected:**

Probe deeper with follow-up questions:

```
Question: "Regarding {constraint}, what are the specific requirements?"
# Free-text or options based on constraint type
```

**Document each constraint:**
- **Nature:** What is the constraint?
- **Impact:** How does it affect architecture?
- **Mitigation:** How can we address it?
- **Risk Level:** Low/Medium/High

**Example:**

```markdown
### Technical Constraint: Legacy System Integration

**Nature:** Must integrate with legacy SOAP API (no REST endpoint)
**Impact:** Requires SOAP client library, XML parsing, potentially slower performance
**Mitigation:**
- Use proven SOAP client (e.g., node-soap for Node.js)
- Cache responses to reduce SOAP call frequency
- Create facade/adapter layer for future migration
**Risk Level:** Medium
```

---

## Step 5.1.5: Research-Based Feasibility Validation (NEW - STORY-036)

**Purpose:** Invoke internet-sleuth agent for evidence-based feasibility assessment when idea is new/unproven or technology unfamiliar.

**When to invoke research:**
```python
# Determine if research needed
research_needed = (
    idea_is_new_or_unproven()  # No existing implementations to reference
    OR technology_unfamiliar()  # Team has no experience with proposed tech
    OR market_validation_needed()  # Uncertain about market viability
    OR complex_integrations()  # Multiple third-party systems involved
)

if research_needed:
    invoke_internet_sleuth()
else:
    skip_research()  # Proceed with standard feasibility assessment
```

**Research Invocation:**
```python
Task(
  subagent_type="internet-sleuth",
  description="Feasibility research for {epic_name}",
  prompt=f"""
  Research Mode: discovery
  Research Scope: {business_idea_summary}
  Context: Epic {epic_id} ({epic_title}), Workflow State: Backlog
  Required Outputs:
    - Technical feasibility score (0-10)
    - Market viability (HIGH/MEDIUM/LOW with evidence)
    - Top 3 approaches/technologies with pros/cons
    - Risk factors with severity + mitigation strategies

  Constraints:
    - Respect tech-stack.md (if brownfield mode)
    - Budget: {budget_constraint}
    - Team expertise: {team_skills}
    - Timeline: {target_timeline}
  """
)
```

**Parse Research Results:**
```python
research_result = internet_sleuth_result

# Extract feasibility score
technical_feasibility_score = research_result.technical_feasibility_score  # 0-10
market_viability = research_result.market_viability  # HIGH/MEDIUM/LOW

# Extract recommendations
top_approaches = research_result.top_recommendations[:3]

# Extract risks (merge with manually identified risks in Step 5.3)
research_risks = research_result.risk_factors

# Display research summary
display(f"""
Research Completed:
  ✓ Research ID: {research_result.research_id}
  ✓ Technical Feasibility: {technical_feasibility_score}/10
  ✓ Market Viability: {market_viability}
  ✓ Top Recommendation: {top_approaches[0].approach}
  ✓ Report: {research_result.report_path}
""")
```

**Incorporate into Epic Document:**
```markdown
## Feasibility Analysis

**Research Report:** [{research_result.research_id}]({research_result.report_path})
**Technical Feasibility:** {technical_feasibility_score}/10
**Market Viability:** {market_viability}
**Recommended Approach:** {top_approaches[0].approach}

**Key Findings:**
- {top_approaches[0].pros[0]}
- {top_approaches[0].pros[1]}

**Key Risks:**
- {research_risks[0].risk}: {research_risks[0].severity} (Mitigation: {research_risks[0].mitigation})
- {research_risks[1].risk}: {research_risks[1].severity} (Mitigation: {research_risks[1].mitigation})

**Go/No-Go Decision:** {"✅ GO" if technical_feasibility_score >= 7 else "❌ NO-GO"}
```

**Update Epic YAML Frontmatter:**
```python
# Add research reference to epic frontmatter
epic_frontmatter["research_references"] = [research_result.research_id]
```

**Handle Quality Gate Violations:**
```python
if research_result.quality_gate_status == "BLOCKED":
    # CRITICAL violation (e.g., research recommends Vue but tech-stack.md specifies React)
    display("❌ Research recommendations conflict with context files (CRITICAL)")
    display(f"   See {research_result.report_path} for violation details")
    display("   User decision required (already handled by internet-sleuth agent)")
    # Agent already triggered AskUserQuestion, decision recorded in report

elif research_result.quality_gate_status == "FAIL" or research_result.quality_gate_status == "WARN":
    # Non-critical violations
    display(f"⚠️ Research has {len(research_result.framework_compliance.violations)} warnings")
    display(f"   See Framework Compliance section in {research_result.report_path}")
    # Proceed with warnings logged
```

**Benefits of Research Integration:**
- ✅ Evidence-based feasibility scores (not subjective estimates)
- ✅ Real-world technology comparisons (GitHub repository analysis)
- ✅ Market validation (adoption trends, community health metrics)
- ✅ Risk identification from production implementations (common pitfalls documented)
- ✅ Framework compliance validation (research respects tech-stack.md constraints)
- ✅ ADR-ready evidence (if technology selection required)

**Fallback if research unavailable:**
```python
if research_failed or research_skipped:
    # Proceed with manual feasibility assessment
    display("⚠️ Proceeding without research (manual assessment)")

    # Ask feasibility questions manually
    technical_feasibility = AskUserQuestion(
        questions=[{
            question: "What is the technical feasibility?",
            header: "Feasibility",
            options: [
                {label: "High (proven tech)", description: "Technology mature, well-documented"},
                {label: "Medium (some unknowns)", description: "Technology stable, some learning curve"},
                {label: "Low (experimental)", description: "Technology new, significant risk"}
            ]
        }]
    )

    # Map to 0-10 score
    score_map = {"High": 8, "Medium": 5, "Low": 3}
    technical_feasibility_score = score_map[technical_feasibility]
```

---

## Step 5.2: Business Constraints

### Budget & Resources

```
Question: "What are the budget and resource constraints?"
Options:
  - "Limited budget - minimize cloud/licensing costs"
  - "Limited team - simple, maintainable architecture"
  - "Time-constrained - MVP in {X weeks/months}"
  - "No major resource constraints"
multiSelect: true
```

**Document:**
- Budget limitations (cloud costs, licensing, third-party services)
- Team capacity (FTE count, skill levels, availability)
- Timeline pressure (hard deadlines, market windows)

### Timeline Constraints

```
Question: "What is the target timeline?"
Options:
  - "Urgent - MVP in 4-6 weeks"
  - "Standard - MVP in 2-3 months"
  - "Flexible - 4-6 months or longer"
```

**Validate timeline against scope:**

```
# Calculate estimated effort from Phase 4
total_story_points = sum(epic.estimated_points for epic in epics)

# Estimate sprint count (assuming 20-30 points per 2-week sprint)
estimated_sprints = total_story_points / 25

# Convert to weeks
estimated_weeks = estimated_sprints * 2

# Compare to target timeline
if estimated_weeks > target_timeline_weeks:
    # Timeline at risk
    use AskUserQuestion to discuss:
    - Reduce scope (defer features)
    - Increase team capacity
    - Extend timeline
```

---

## Step 5.3: Risk Assessment

### Identify Risks Across Categories

**Technical Risks:**
- Unproven technology
- Complex integrations
- Performance unknowns
- Scalability challenges
- Data migration complexity

**Business Risks:**
- Market timing
- Competitive pressure
- Regulatory changes
- User adoption uncertainty

**Team Risks:**
- Skill gaps (learning curve)
- Key person dependencies
- Capacity constraints
- Distributed team coordination

**For each risk, document:**

```markdown
### Risk: {Risk Name}

**Category:** Technical|Business|Team
**Description:** {What could go wrong}
**Probability:** Low|Medium|High (10-30% | 30-70% | 70%+)
**Impact:** Low|Medium|High (Minor | Significant | Critical)
**Mitigation Strategy:** {How to prevent or minimize}
**Contingency Plan:** {What to do if risk occurs}
**Owner:** {Who monitors and responds to this risk}
```

### Prioritize Risks by Severity

**Severity = Probability × Impact**

| Probability | Impact | Severity | Action |
|-------------|--------|----------|--------|
| High | High | **CRITICAL** | Must mitigate before starting |
| High | Medium | **HIGH** | Mitigate in early sprints |
| Medium | High | **HIGH** | Mitigate in early sprints |
| Medium | Medium | **MEDIUM** | Monitor and prepare contingency |
| Low | High | **MEDIUM** | Monitor and prepare contingency |
| Low | Low | **LOW** | Document only |

**Critical risks block progression:**
- Must have mitigation strategy before Phase 6
- May require architecture changes
- May require scope reduction
- Use AskUserQuestion to validate mitigation

---

## Step 5.4: Constraint Validation (Brownfield Projects)

**If existing DevForgeAI context files exist (from Phase 1.2):**

### Validate Against Existing Constraints

```
# Load existing constraints
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
Read(file_path="devforgeai/specs/context/dependencies.md")
Read(file_path="devforgeai/specs/context/anti-patterns.md")

# Check for conflicts
conflicts = []

# Technology conflicts
if new_requirement.technology not in existing_tech_stack:
    conflicts.append("Technology conflict: {new} not in approved stack")

# Architecture pattern conflicts
if new_requirement.violates_architecture_constraints:
    conflicts.append("Architecture conflict: {constraint violated}")

# Dependency conflicts
if new_requirement.dependency not in approved_dependencies:
    conflicts.append("Dependency conflict: {new dependency} not approved")

# Anti-pattern violations
if new_requirement.implements_anti_pattern:
    conflicts.append("Anti-pattern: {pattern name}")
```

### Resolve Conflicts

**If conflicts detected:**

```
For each conflict:
    AskUserQuestion(
        question: "New requirement conflicts with existing constraint. How to resolve?",
        header: "Conflict resolution",
        options: [
            "Update constraint (requires ADR)",
            "Modify requirement to fit constraint",
            "Mark as future scope (defer)"
        ]
    )

    If "Update constraint":
        # Document in ADR requirements for architecture phase
        # Note: ADR will be created by devforgeai-architecture skill
        document_adr_requirement(conflict, resolution)

    If "Modify requirement":
        # Update requirements spec
        modify_requirement_to_comply(requirement, constraint)

    If "Mark as future scope":
        # Move to future scope list
        defer_requirement(requirement)
```

---

## Output from Phase 5

**Feasibility Assessment Report:**

```markdown
## Feasibility Assessment

### Technical Feasibility: {FEASIBLE|AT RISK|NOT FEASIBLE}

**Assessment:**
{Summary of technical feasibility}

**Technical Risks:**
1. {Risk 1}: {Probability}/{Impact} - {Mitigation}
2. {Risk 2}: {Probability}/{Impact} - {Mitigation}

**Technical Constraints:**
1. {Constraint 1}: {Mitigation strategy}
2. {Constraint 2}: {Mitigation strategy}

### Business Feasibility: {FEASIBLE|AT RISK|NOT FEASIBLE}

**Budget Analysis:**
- Estimated cloud costs: ${X}/month
- Licensing costs: ${Y}/month
- Total development cost: ${Z}
- Within budget: {YES|NO}

**Timeline Analysis:**
- Estimated effort: {N} story points ({M} sprints)
- Target timeline: {T} weeks
- Timeline feasible: {YES|NO|AT RISK}

### Resource Feasibility: {FEASIBLE|AT RISK|NOT FEASIBLE}

**Team Capacity:**
- Available developers: {N} FTE
- Required developers: {M} FTE
- Skill gaps: {skills needed}
- Training required: {estimate}

### Risk Register

| Risk | Category | Probability | Impact | Severity | Mitigation |
|------|----------|-------------|--------|----------|------------|
| {Risk 1} | Technical | High | High | CRITICAL | {Strategy} |
| {Risk 2} | Business | Medium | High | HIGH | {Strategy} |
| {Risk 3} | Team | Low | Medium | MEDIUM | {Strategy} |

### Overall Feasibility: {FEASIBLE|AT RISK|NOT FEASIBLE}

**Recommendation:** {Proceed|Proceed with mitigations|Adjust scope|Not recommended}
```

**Transition:** Proceed to Phase 6 (Requirements Documentation & Handoff)

---

## Common Issues and Recovery

### Issue: NOT FEASIBLE Assessment

**Symptom:** Technical/business/resource feasibility = NOT FEASIBLE

**Recovery:**
1. Identify which dimension is not feasible
2. Propose scope reductions or timeline extensions
3. Use AskUserQuestion to explore alternatives
4. Options:
   - Reduce scope to feasible subset
   - Extend timeline to match capacity
   - Increase team capacity (hiring, outsourcing)
   - Defer to future when constraints change

### Issue: CRITICAL Risks Without Mitigation

**Symptom:** High-probability, high-impact risks with no clear mitigation

**Recovery:**
1. Research mitigation strategies (WebFetch if needed)
2. Propose multiple mitigation options
3. Use AskUserQuestion for user input
4. If no viable mitigation: Flag as blocker, recommend deferral

### Issue: Brownfield Conflicts Unresolvable

**Symptom:** New requirements fundamentally incompatible with existing constraints

**Recovery:**
1. Present conflict clearly to user
2. Options:
   - Major refactoring of existing system (creates separate epic)
   - Build as separate system (microservice approach)
   - Defer until existing system redesigned
3. Document as architectural decision for architecture phase

---

## References Used in Phase 5

**Primary:**
- **feasibility-analysis-framework.md** (587 lines) - Complete feasibility checklists and risk templates

**Related:**
- **complexity-assessment-matrix.md** - Complexity score informs feasibility
- **domain-specific-patterns.md** - Known feasibility patterns per domain

**On Error:**
- **error-handling.md** - Recovery for constraint conflict resolution

---

## Success Criteria

Phase 5 complete when:
- [ ] Technical feasibility assessed (feasible/at risk/not feasible)
- [ ] Business feasibility assessed (budget, timeline)
- [ ] Resource feasibility assessed (team capacity, skills)
- [ ] All risks identified and categorized
- [ ] CRITICAL risks have mitigation strategies
- [ ] Brownfield conflicts resolved (if applicable)
- [ ] Overall feasibility determined
- [ ] Recommendation documented (proceed/adjust/defer)

**Token Budget:** ~3,000-6,000 tokens (load framework, assess, document)

---

## TodoWrite - Phase Completion

**At phase end, mark as completed:**
```
TodoWrite([
  {"content": "Phase 5: Feasibility & Constraints Analysis", "status": "completed", "activeForm": "Analyzing constraints"}
])
```

---

**Next Phase:** Phase 6 (Requirements Documentation & Handoff)
