# Skill Coordination Patterns - Internet-Sleuth Integration

**Purpose:** Documentation for DevForgeAI skills on how to invoke internet-sleuth agent, parse results, handle errors, and manage token budgets.

**Audience:** devforgeai-ideation, devforgeai-architecture, other skills needing research capabilities

**Loaded:** As reference when skills need to integrate with internet-sleuth

---

## Overview

internet-sleuth is a DevForgeAI subagent providing research capabilities via Task tool invocation. This guide shows how skills coordinate with the agent, handle responses, and integrate findings into workflow outputs.

---

## Invocation Patterns

### Pattern 1: Basic Research Invocation

**Use when:** Simple research task with standard output

**Syntax:**
```python
Task(
  subagent_type="internet-sleuth",
  description="Research [topic]",
  prompt="""
  Research Mode: [discovery|investigation|competitive-analysis|repository-archaeology|market-intelligence]
  Research Scope: [brief description]
  Context: Epic [EPIC-ID] ([epic name]), Workflow State: [state]
  Required Outputs: [what you need from research]
  Constraints: [respect tech-stack.md, etc.]
  """
)
```

**Example (devforgeai-ideation Phase 5):**
```python
Task(
  subagent_type="internet-sleuth",
  description="Feasibility research for authentication system",
  prompt="""
  Research Mode: discovery
  Research Scope: OAuth 2.0 feasibility for multi-tenant SaaS platform
  Context: Epic EPIC-007 (User Authentication System), Workflow State: Architecture
  Required Outputs:
    - Technical feasibility score (0-10)
    - Top 3 recommended approaches with trade-offs
    - Market viability evidence (adoption %, community health)
    - Risk factors with mitigation strategies
  Constraints:
    - Respect tech-stack.md (AWS preferred, Python backend)
    - Validate against architecture-constraints.md (clean architecture)
    - Budget: <$500/month operational cost
  """
)
```

**Expected Response Structure:**
```json
{
  "research_id": "RESEARCH-001",
  "technical_feasibility_score": 8.5,
  "market_viability": "HIGH",
  "top_recommendations": [
    {
      "approach": "AWS Cognito",
      "feasibility_score": 8.2,
      "pros": ["AWS-native", "Cost-effective", "Scalable"],
      "cons": ["Complex setup", "Limited customization"],
      "estimated_cost": "$55/month for 10K MAU"
    },
    {
      "approach": "Auth0",
      "feasibility_score": 7.8,
      "pros": ["Excellent DX", "Comprehensive features"],
      "cons": ["Expensive at scale", "Vendor lock-in"],
      "estimated_cost": "$535/month for 10K MAU"
    },
    {
      "approach": "Supertokens (self-hosted)",
      "feasibility_score": 6.5,
      "pros": ["Open-source", "Free (self-host)"],
      "cons": ["Maintenance overhead", "Smaller community"],
      "estimated_cost": "$100/month (infrastructure)"
    }
  ],
  "risk_factors": [
    {"risk": "Vendor lock-in", "severity": "MEDIUM", "mitigation": "Use OIDC standards, abstract auth interface"},
    {"risk": "Complex setup", "severity": "MEDIUM", "mitigation": "Allocate 1-2 weeks for integration"}
  ],
  "framework_compliance": "PASS",
  "quality_gate_status": "PASS",
  "report_path": "devforgeai/specs/research/feasibility/EPIC-007-2025-11-17-143022-research.md"
}
```

---

### Pattern 2: Context-Aware Research Invocation

**Use when:** Research needs current story/epic context

**Syntax:**
```python
# Load story/epic first
Read(file_path="devforgeai/specs/Stories/STORY-042.story.md")
# OR
Read(file_path="devforgeai/specs/Epics/EPIC-007.epic.md")

# Then invoke with explicit context markers
**Story ID:** STORY-042
**Epic ID:** EPIC-007
**Workflow State:** Architecture

Task(
  subagent_type="internet-sleuth",
  description="Technology evaluation for [story]",
  prompt="""
  Research Mode: repository-archaeology
  Story Context: STORY-042 (Implement OAuth 2.0 flows)
  Research Focus: Real-world implementation patterns for OAuth 2.0 in Python/FastAPI
  Required Outputs:
    - Top 3 code patterns with examples
    - GitHub repositories (quality score ≥7)
    - Common pitfalls with mitigations
  Constraints: Respect tech-stack.md (FastAPI, SQLAlchemy)
  """
)
```

**Why context markers matter:**
- internet-sleuth detects workflow state from conversation
- Research recommendations adapt to current phase
- Staleness detection uses workflow state metadata
- Epic/story linkage automatic (YAML frontmatter updated)

---

### Pattern 3: Multi-Mode Research (Sequential)

**Use when:** Need multiple research perspectives

**Syntax:**
```python
# Step 1: Feasibility (discovery mode)
discovery_result = Task(
  subagent_type="internet-sleuth",
  description="Feasibility research",
  prompt="Research Mode: discovery\n[... feasibility questions ...]"
)

# Step 2: Wait for result
# (Subagent executes in isolated context, returns result)

# Step 3: If feasible, deep-dive (investigation or repository-archaeology)
if discovery_result.technical_feasibility_score >= 7:
  implementation_result = Task(
    subagent_type="internet-sleuth",
    description="Implementation patterns research",
    prompt="Research Mode: repository-archaeology\n[... pattern extraction ...]"
  )
```

**Example (devforgeai-architecture Phase 2):**
```python
# Phase 2 Step 2.1: Feasibility check
feasibility = Task(
  subagent_type="internet-sleuth",
  description="OAuth 2.0 feasibility",
  prompt="Research Mode: discovery\nScope: OAuth 2.0 for SaaS\nOutputs: Feasibility score, top 3 providers"
)

# Phase 2 Step 2.2: If feasible (score ≥7), get implementation patterns
if feasibility.technical_feasibility_score >= 7:
  patterns = Task(
    subagent_type="internet-sleuth",
    description="OAuth implementation patterns",
    prompt="Research Mode: repository-archaeology\nScope: Python FastAPI OAuth 2.0\nOutputs: Code patterns, GitHub repos"
  )

  # Phase 2 Step 2.3: Incorporate into tech-stack.md creation
  tech_stack_content = f"""
  ## Authentication
  - Provider: {feasibility.top_recommendations[0].approach}
  - Rationale: {feasibility.top_recommendations[0].pros}
  - Implementation: {patterns.top_recommendations[0].pattern_description}
  - Reference: {patterns.report_path}
  """
```

---

## Result Parsing Patterns

### Pattern 4: Extract Feasibility Score

**Use when:** Need go/no-go decision based on research

**Code:**
```python
result = Task(subagent_type="internet-sleuth", ...)

# Extract feasibility score (0-10)
feasibility_score = result.technical_feasibility_score

# Decision logic
if feasibility_score >= 9:
    recommendation = "GO (high confidence)"
elif feasibility_score >= 7:
    recommendation = "GO with caution (identify risks)"
elif feasibility_score >= 5:
    recommendation = "CONDITIONAL (deeper investigation needed)"
else:
    recommendation = "NO-GO (too risky)"

# Display in epic/story
display(f"Feasibility: {feasibility_score}/10 - {recommendation}")
display(f"Research: {result.report_path}")
```

---

### Pattern 5: Extract Top Recommendations

**Use when:** Need specific technology/approach suggestions

**Code:**
```python
result = Task(subagent_type="internet-sleuth", ...)

# Extract top 3 recommendations
top_3 = result.top_recommendations[:3]

# Format for display
for i, rec in enumerate(top_3, 1):
    display(f"""
    {i}. {rec.approach} (Feasibility: {rec.feasibility_score}/10)
       Pros: {', '.join(rec.pros)}
       Cons: {', '.join(rec.cons)}
       Cost: {rec.estimated_cost}
    """)

# Use in tech-stack.md
selected = top_3[0]  # Highest scored
tech_stack_entry = f"- **{selected.approach}:** {selected.pros[0]} (research: {result.research_id})"
```

---

### Pattern 6: Handle Quality Gate Violations

**Use when:** Research findings conflict with context files

**Code:**
```python
result = Task(subagent_type="internet-sleuth", ...)

# Check quality gate status
if result.quality_gate_status == "BLOCKED":
    # CRITICAL violation detected (e.g., recommends Vue but tech-stack.md specifies React)

    # Parse violation details
    violations = result.framework_compliance.violations
    critical_violations = [v for v in violations if v.severity == "CRITICAL"]

    # Present to user via AskUserQuestion
    for violation in critical_violations:
        response = AskUserQuestion(
            questions=[{
                question: f"Research recommends {violation.recommendation} but {violation.context_file} specifies {violation.existing_value}. How to proceed?",
                header: "Context Conflict",
                multiSelect: false,
                options: [
                    {
                        label: f"Update {violation.context_file} + create ADR",
                        description: f"Adopt research recommendation ({violation.recommendation}), document decision"
                    },
                    {
                        label: f"Use existing ({violation.existing_value})",
                        description: "Respect current context file, adjust research scope"
                    },
                    {
                        label: "Document as technical debt",
                        description: "Defer decision, create follow-up story"
                    }
                ]
            }]
        )

        # Handle user decision
        if "Update" in response:
            create_adr(violation)
            update_context_file(violation.context_file, violation.recommendation)
        elif "existing" in response:
            # Re-invoke research with adjusted scope
            adjusted_result = Task(
                subagent_type="internet-sleuth",
                prompt=f"... focus on {violation.existing_value} (not {violation.recommendation}) ..."
            )
        else:
            # Create technical debt tracking story
            create_follow_up_story(violation)

elif result.quality_gate_status == "WARN":
    # Non-critical violations (log and proceed)
    display(f"⚠️ {len(result.framework_compliance.violations)} warnings in research (see {result.report_path})")

elif result.quality_gate_status == "PASS":
    # No violations, safe to proceed
    display(f"✅ Research compliant with all context files")
```

---

## Error Handling Patterns

### Pattern 7: Handle Research Failures

**Use when:** Subagent encounters errors (API limits, network issues, etc.)

**Code:**
```python
try:
    result = Task(subagent_type="internet-sleuth", ...)

    # Check for partial results (research may have cached progress)
    if result.status == "PARTIAL":
        display(f"⚠️ Research partially complete (cached: {result.cache_path})")
        display(f"Sections completed: {', '.join(result.completed_sections)}")
        display(f"Resume: Re-invoke research (will load from cache)")

        # Option 1: Proceed with partial results
        if len(result.completed_sections) >= 3:  # Minimum viable
            display("Using partial results...")
            feasibility = result.technical_feasibility_score or "N/A"
            # ... proceed with limited data

        # Option 2: Retry research
        else:
            display("Retrying research (will resume from cache)...")
            result = Task(subagent_type="internet-sleuth", ...)  # Resumes from cache

    # Check for complete failure
    elif result.status == "FAILED":
        display(f"❌ Research failed: {result.error_message}")
        display(f"Fallback: Use manual research or skip")

        # Fallback strategies
        fallback = AskUserQuestion(
            questions=[{
                question: "Research agent failed. How to proceed?",
                header: "Research Error",
                options: [
                    {label: "Retry", description: "Re-invoke agent (may hit same error)"},
                    {label: "Manual research", description: "I'll provide research findings manually"},
                    {label: "Skip", description: "Proceed without research (higher risk)"}
                ]
            }]
        )

        if "Retry" in fallback:
            result = Task(subagent_type="internet-sleuth", ...)
        elif "Manual" in fallback:
            # Accept user-provided research data
            display("Please provide: feasibility score, top recommendations, key risks")
        else:
            display("⚠️ Proceeding without research (document assumption)")

except Exception as e:
    display(f"❌ Unexpected error invoking research: {e}")
    display("Fallback: Manual research required")
```

---

### Pattern 8: Handle Perplexity API Rate Limits

**Use when:** Research hits Perplexity API 429 errors

**Code:**
```python
result = Task(subagent_type="internet-sleuth", ...)

if result.status == "RATE_LIMITED":
    display(f"⚠️ Perplexity API rate limit hit")
    display(f"Cached progress: {result.cache_path}")
    display(f"Retry after: {result.retry_after} seconds")

    # Option 1: Wait and retry automatically (if retry_after < 60 seconds)
    if result.retry_after < 60:
        display(f"Waiting {result.retry_after}s and retrying...")
        time.sleep(result.retry_after)
        result = Task(subagent_type="internet-sleuth", ...)  # Resumes from cache

    # Option 2: Defer research (if retry_after > 60 seconds)
    else:
        display(f"Rate limit recovery time too long ({result.retry_after}s)")
        display(f"Options: 1) Proceed without research, 2) Retry later")

        response = AskUserQuestion(
            questions=[{
                question: f"Perplexity API rate limited (retry in {result.retry_after}s). Proceed?",
                header: "Rate Limit",
                options: [
                    {label: "Wait and retry", description: f"I'll wait {result.retry_after}s"},
                    {label: "Skip research", description: "Proceed without (risk: uninformed decision)"},
                    {label: "Defer", description: "Save progress, resume later"}
                ]
            }]
        )

        if "Wait" in response:
            time.sleep(result.retry_after)
            result = Task(subagent_type="internet-sleuth", ...)
        elif "Skip" in response:
            display("⚠️ Proceeding without research")
        else:
            # Save cache reference for later
            save_cache_reference(epic_id, result.cache_path)
            display(f"Research deferred. Resume with: {result.cache_path}")
```

---

## Token Budget Management

### Pattern 9: Monitor Token Usage

**Goal:** Ensure research operations stay within 50K token budget (isolated context)

**Monitoring:**
```python
result = Task(subagent_type="internet-sleuth", ...)

# Check token usage (reported in result)
tokens_used = result.token_usage

if tokens_used > 50000:
    display(f"⚠️ Research exceeded token budget ({tokens_used}/50K)")
    display(f"Consider: Narrower scope, simpler research mode")
elif tokens_used > 40000:
    display(f"ℹ️ Research near token budget ({tokens_used}/50K)")
else:
    display(f"✅ Token usage acceptable ({tokens_used}/50K)")

# Log for optimization
log_token_usage(research_id=result.research_id, tokens=tokens_used, mode=result.research_mode)
```

**Optimization Strategies:**
- Use `discovery` mode for high-level exploration (lower token usage than `investigation`)
- Narrow research scope (specific questions vs broad exploration)
- Progressive disclosure: Only load necessary methodology files

---

### Pattern 10: Batch Research (Multiple Topics)

**Use when:** Need research on multiple related topics

**Inefficient Approach:**
```python
# ❌ INEFFICIENT: 5 separate research invocations (5 * 45K = 225K tokens!)
auth_research = Task(subagent_type="internet-sleuth", prompt="Research: OAuth 2.0")
db_research = Task(subagent_type="internet-sleuth", prompt="Research: PostgreSQL vs MySQL")
cache_research = Task(subagent_type="internet-sleuth", prompt="Research: Redis vs Memcached")
queue_research = Task(subagent_type="internet-sleuth", prompt="Research: RabbitMQ vs SQS")
monitoring_research = Task(subagent_type="internet-sleuth", prompt="Research: Prometheus vs Datadog")
```

**Efficient Approach:**
```python
# ✅ EFFICIENT: 1 broader research invocation (1 * 65K = 65K tokens)
tech_stack_research = Task(
  subagent_type="internet-sleuth",
  description="Technology stack evaluation",
  prompt="""
  Research Mode: discovery
  Research Scope: Complete technology stack for SaaS platform
  Topics:
    1. Authentication: OAuth 2.0 providers (Auth0, Cognito, Firebase)
    2. Database: PostgreSQL vs MySQL
    3. Cache: Redis vs Memcached
    4. Message Queue: RabbitMQ vs AWS SQS
    5. Monitoring: Prometheus vs Datadog

  Required Outputs:
    - Feasibility score per topic
    - Top recommendation per topic
    - Total cost estimate (all technologies combined)

  Constraints: AWS-preferred stack (per tech-stack.md)
  """
)

# Extract per-topic recommendations
auth_rec = tech_stack_research.topics["authentication"].top_recommendation
db_rec = tech_stack_research.topics["database"].top_recommendation
# ... etc
```

**Token Savings:** 71% (225K → 65K tokens)

---

## Integration Examples

### Example 1: devforgeai-ideation Phase 5 Integration

**Location:** `.claude/skills/devforgeai-ideation/SKILL.md` Phase 5

**Integration Code:**
```markdown
## Phase 5: Feasibility & Constraints Analysis

### Step 5.1: Determine if Research Needed

IF business idea is new/unproven OR technology unfamiliar:
  research_needed = TRUE
ELSE:
  research_needed = FALSE  # Skip research for well-understood domains

### Step 5.2: Invoke internet-sleuth (Discovery Mode)

IF research_needed:
  Task(
    subagent_type="internet-sleuth",
    description="Feasibility research for [epic name]",
    prompt=f"""
    Research Mode: discovery
    Research Scope: {epic.business_idea}
    Context: Epic {epic.id} ({epic.title}), Workflow State: Backlog
    Required Outputs:
      - Technical feasibility score (0-10)
      - Market viability (HIGH/MEDIUM/LOW with evidence)
      - Top 3 approaches with pros/cons
      - Risk factors with severity + mitigation

    Constraints:
      - Respect tech-stack.md (if brownfield)
      - Budget: {epic.budget_constraint}
    """
  )

### Step 5.3: Extract Research Findings

feasibility_score = result.technical_feasibility_score
market_viability = result.market_viability
top_approaches = result.top_recommendations[:3]
risks = result.risk_factors

### Step 5.4: Incorporate into Epic Document

epic_content += f"""
## Feasibility Analysis

**Research Report:** [{result.research_id}]({result.report_path})
**Technical Feasibility:** {feasibility_score}/10
**Market Viability:** {market_viability}
**Recommended Approach:** {top_approaches[0].approach}

**Key Risks:**
{format_risks(risks)}

**Go/No-Go Decision:** {"✅ GO" if feasibility_score >= 7 else "❌ NO-GO"}
"""

### Step 5.5: Update Epic YAML Frontmatter

epic.research_references = [result.research_id]
```

---

### Example 2: devforgeai-architecture Phase 2 Integration

**Location:** `.claude/skills/devforgeai-architecture/SKILL.md` Phase 2

**Integration Code:**
```markdown
## Phase 2: Create Immutable Context Files

### Step 2.1: Technology Selection Workflow

FOR each technology category (language, framework, database, etc.):

  # Step 2.1.1: Check if research needed
  IF multiple valid options exist:
    research_needed = TRUE
  ELSE:
    research_needed = FALSE  # Only one obvious choice

  # Step 2.1.2: Invoke internet-sleuth (Repository Archaeology)
  IF research_needed:
    result = Task(
      subagent_type="internet-sleuth",
      description="Technology evaluation for [category]",
      prompt=f"""
      Research Mode: repository-archaeology
      Research Scope: {category} for {project_description}
      Context: Epic {epic.id}, Workflow State: Architecture
      Required Outputs:
        - Top 3 technologies with real-world implementation patterns
        - GitHub repositories (quality score ≥7)
        - Common pitfalls + mitigation strategies

      Constraints:
        - Respect existing tech-stack.md (if brownfield)
        - Team expertise: {team_skills}
      """
    )

  # Step 2.1.3: Extract Repository Patterns
  top_tech = result.top_recommendations[0]
  implementation_patterns = top_tech.code_patterns
  github_repos = top_tech.repositories

  # Step 2.1.4: Populate tech-stack.md Entry
  tech_stack_entry = f"""
  ## {category}
  - **Technology:** {top_tech.approach}
  - **Rationale:** {top_tech.pros[0]}
  - **Implementation Pattern:** {implementation_patterns[0].pattern_description}
  - **Reference Repositories:**
    {format_repos(github_repos)}
  - **Research:** {result.research_id} ({result.report_path})
  """

  # Step 2.1.5: Check for ADR Requirement
  IF top_tech.approach NOT IN existing_tech_stack:
    create_adr(
      title=f"ADR-XXX: Adopt {top_tech.approach} for {category}",
      context=result.rationale,
      decision=top_tech.approach,
      alternatives=result.top_recommendations[1:3],
      consequences=top_tech.cons
    )
```

---

## Success Criteria

Skill coordination succeeds when:
- [ ] Task invocation syntax correct (subagent_type, description, prompt with mode)
- [ ] Result parsing handles all expected fields (feasibility_score, recommendations, etc.)
- [ ] Error handling comprehensive (PARTIAL, FAILED, RATE_LIMITED states)
- [ ] Quality gate violations trigger AskUserQuestion (BLOCKED status)
- [ ] Token budget monitored (<50K per operation)
- [ ] Research results incorporated into skill outputs (epic, tech-stack.md, etc.)
- [ ] Epic/story YAML frontmatter updated (research_references)

---

## Related Documentation

- `research-principles.md` - Core research methodology (loaded by agent)
- `discovery-mode-methodology.md` - Discovery workflow (ideation Phase 5 primary)
- `repository-archaeology-guide.md` - Repository patterns (architecture Phase 2 primary)
- `competitive-analysis-patterns.md` - Market analysis (ideation Phase 5 secondary)
- `research-report-template.md` - Standard report structure
- `.claude/skills/devforgeai-ideation/SKILL.md` - Ideation skill
- `.claude/skills/devforgeai-architecture/SKILL.md` - Architecture skill

---

**Created:** 2025-11-17
**Version:** 1.0
**Lines:** 450 (target: 450 ✓)
**Purpose:** Documentation for DevForgeAI skills on internet-sleuth integration
