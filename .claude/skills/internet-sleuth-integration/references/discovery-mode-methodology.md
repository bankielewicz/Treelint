# Discovery Mode Methodology - Internet-Sleuth

**Purpose:** High-level exploration and feasibility assessment for new ideas, technologies, or market opportunities.

**When to Use:** devforgeai-ideation Phase 5 (Feasibility & Constraints Analysis), early epic planning, technology evaluation pre-architecture

**Loaded:** Conditionally (when research_mode = "discovery")

---

## Discovery Mode Overview

**Scope:** Broad exploration (breadth over depth)
**Duration:** 3-5 minutes (p95)
**Output:** Feasibility score (0-10), high-level alternatives, go/no-go recommendation

**Research Questions Answered:**
- "Is this idea technically feasible?"
- "What are the main alternatives?"
- "What are high-level risks?"
- "Should we proceed to architecture phase?"

**Not Answered (Use investigation mode instead):**
- "How exactly should we implement this?"
- "What are all possible edge cases?"
- "What's the optimal database schema?"

---

## Discovery Workflow (6 Steps)

### Step 1: Define Research Scope

**Goal:** Clarify what to discover and boundaries.

**Inputs:**
- Epic description or business idea
- Workflow state (usually "Backlog" or early "Architecture")
- Current context files (if brownfield project)

**Actions:**
1. **Extract research questions from epic/idea:**
   - "Build SaaS authentication system" → "Is OAuth 2.0 feasible? Alternatives?"
   - "Add AI recommendations" → "What ML frameworks work with Python? Cloud vs on-prem?"

2. **Set exploration boundaries:**
   - Technology scope: Backend only? Full-stack? Mobile?
   - Market scope: B2B SaaS? Consumer apps? Enterprise?
   - Budget constraints: Open-source only? Paid services allowed?

3. **Document assumptions:**
   - "Assuming Python backend (from tech-stack.md)"
   - "Assuming cloud deployment (AWS/Azure/GCP)"
   - "Assuming <$500/month operational cost"

**Outputs:**
- Research question list (3-5 primary questions)
- Scope boundaries (in-scope vs out-of-scope)
- Assumptions document

**Example:**
```markdown
## Research Scope: User Authentication System

**Primary Questions:**
1. Is OAuth 2.0 feasible for multi-tenant SaaS?
2. What are standard authentication flows for web + mobile?
3. Which providers (Auth0, Firebase, Cognito) best fit our needs?

**Boundaries:**
- In-scope: OAuth 2.0, OIDC, SAML
- Out-of-scope: Custom crypto implementation, blockchain auth
- Technology: Python backend, React frontend (per tech-stack.md)

**Assumptions:**
- Cloud deployment (AWS preferred)
- <1000 users initially, scaling to 10K within 6 months
- Budget: <$200/month for auth service
```

---

### Step 2: Broad Research (Perplexity API)

**Goal:** Gather high-level information from multiple sources.

**Perplexity Query Structure:**
```
"What are the pros and cons of [TECHNOLOGY] for [USE CASE]?
Include: performance benchmarks, community support, learning curve, cost.
Cite official documentation and recent surveys (2023+)."
```

**Example Queries:**

**Query 1: Technology Evaluation**
```
"What are the pros and cons of Auth0 vs AWS Cognito vs Firebase Authentication
for a Python/React SaaS application with multi-tenancy?
Include: pricing for <1000 users, integration complexity, OAuth 2.0 support,
customization options. Cite official docs and comparison articles from 2024."
```

**Query 2: Market Landscape**
```
"What percentage of SaaS companies use OAuth 2.0 vs SAML vs custom auth in 2024?
Include adoption trends, security considerations, developer preferences.
Cite industry surveys like Stack Overflow Survey 2024, State of Auth 2024."
```

**Query 3: Implementation Patterns**
```
"What are common architecture patterns for implementing OAuth 2.0 in
Python (FastAPI/Django) with React frontend? Include session management,
token refresh, security best practices. Cite implementation guides from 2023-2024."
```

**Actions:**
1. Execute 3-5 Perplexity queries (parallel if possible)
2. Collect responses with source URLs
3. Extract key facts, statistics, trade-offs
4. Note source quality scores (official docs = 10, blogs = 5, etc.)

**Retry Logic:**
- Max 3 retries per query
- Exponential backoff: 1s, 2s, 4s
- Cache partial results on rate limit (429)

**Outputs:**
- Query responses (markdown format)
- Source URL list with quality scores
- Key findings summary (bullet points)

---

### Step 3: Alternatives Identification

**Goal:** Identify 3-5 viable alternatives for comparison.

**Criteria for Inclusion:**
- Active development (commits within last 6 months)
- Sufficient community (≥500 GitHub stars OR corporate backing)
- Compatible with constraints (tech-stack.md, architecture-constraints.md)
- Documented integration (official guides available)

**Comparison Matrix:**

| Alternative | Pros | Cons | Community | Cost | Compatibility |
|-------------|------|------|-----------|------|---------------|
| Auth0 | Full-featured, easy integration | Expensive at scale | Large (Okta-backed) | $23/mo (1K users) | ✅ Python/React |
| AWS Cognito | AWS integration, scalable | Complex setup | Large (AWS) | $0.0055/MAU | ✅ Python/React |
| Firebase Auth | Simple, fast setup | Google lock-in | Large (Google) | $0.006/MAU | ✅ Python/React |
| Supertokens | Open-source, self-hosted | Maintenance overhead | Medium (2K stars) | Free (self-host) | ✅ Python/React |
| Keycloak | Enterprise-grade, free | Heavy, complex | Large (RedHat) | Free (self-host) | ⚠️ Java-based |

**Actions:**
1. Filter alternatives by constraints
2. Score each alternative (0-10) on:
   - Technical fit (8/10)
   - Cost efficiency (7/10)
   - Developer experience (9/10)
   - Scalability (8/10)
3. Calculate composite score: (sum of scores) / 4
4. Rank alternatives by composite score

**Outputs:**
- Comparison matrix (table format)
- Composite scores (ranked list)
- Top 3 recommendations with rationale

---

### Step 4: Feasibility Assessment

**Goal:** Determine technical feasibility score (0-10) and go/no-go recommendation.

**Feasibility Dimensions:**

**1. Technical Feasibility (0-10)**
- Does technology exist and work? (yes = 8+, experimental = 5-7, theoretical = 0-4)
- Are integrations available? (official = 10, community = 7, none = 3)
- Is documentation complete? (comprehensive = 10, partial = 6, sparse = 3)

**2. Team Capability (0-10)**
- Does team have required skills? (yes = 10, learning curve <1 week = 7, >1 month = 4)
- Are training resources available? (official courses = 10, blogs only = 6, none = 2)

**3. Risk Assessment (0-10, inverted: 10 = low risk)**
- Vendor lock-in risk (none = 10, moderate = 6, high = 2)
- Breaking changes frequency (rare = 10, occasional = 6, frequent = 2)
- Community sustainability (corporate-backed = 10, active OSS = 7, abandoned = 0)

**4. Cost Feasibility (0-10)**
- Within budget? (yes = 10, marginal = 6, exceeds = 2)
- Total cost of ownership <3 years? (affordable = 10, manageable = 6, expensive = 2)

**Composite Feasibility Score:**
```
score = (technical * 0.4) + (capability * 0.2) + (risk * 0.2) + (cost * 0.2)
```

**Example Calculation:**
```
Auth0 Feasibility:
- Technical: 9/10 (mature, well-documented)
- Capability: 8/10 (team knows OAuth, Auth0 easy)
- Risk: 6/10 (vendor lock-in, but stable)
- Cost: 7/10 (within budget for 1K users, scales up)

Score = (9 * 0.4) + (8 * 0.2) + (6 * 0.2) + (7 * 0.2)
      = 3.6 + 1.6 + 1.2 + 1.4
      = 7.8/10
```

**Go/No-Go Thresholds:**
- **9-10:** GO (high confidence, low risk)
- **7-8.9:** GO with caution (identify mitigation strategies)
- **5-6.9:** CONDITIONAL (requires deeper investigation or risk acceptance)
- **0-4.9:** NO-GO (too risky, insufficient feasibility)

**Outputs:**
- Feasibility score (0-10)
- Dimension breakdown (technical, capability, risk, cost)
- Go/no-go recommendation
- Mitigation strategies (if GO with caution)

---

### Step 5: Framework Compliance Validation

**Goal:** Validate research findings against all 6 DevForgeAI context files.

**Validation Workflow:**
1. **Load context files:**
   - Read tech-stack.md, source-tree.md, dependencies.md, coding-standards.md, architecture-constraints.md, anti-patterns.md

2. **Check recommendations against constraints:**
   - Recommended tech in tech-stack.md? (if no → CRITICAL violation)
   - File structure matches source-tree.md? (if no → MEDIUM violation)
   - Dependencies approved in dependencies.md? (if no → HIGH violation)
   - Patterns match coding-standards.md? (if no → LOW violation)
   - Architecture respects architecture-constraints.md? (if no → HIGH violation)
   - No anti-patterns from anti-patterns.md? (if yes → CRITICAL violation)

3. **Invoke context-validator subagent:**
```python
Task(
  subagent_type="context-validator",
  description="Validate research recommendations",
  prompt=f"""
  Validate the following research recommendations against all 6 context files:

  Recommended Technologies:
  - Auth0 (authentication service)
  - Python FastAPI (backend framework)
  - React (frontend framework)

  Check for conflicts with:
  - tech-stack.md (locked technologies)
  - dependencies.md (approved packages)
  - architecture-constraints.md (layer boundaries)

  Return: Structured violation report with severity (CRITICAL/HIGH/MEDIUM/LOW)
  """
)
```

4. **Categorize violations by severity:**
   - CRITICAL: Contradicts locked tech-stack.md
   - HIGH: Violates architecture-constraints.md
   - MEDIUM: Conflicts with coding-standards.md
   - LOW: Informational (style preferences)

5. **HALT on CRITICAL violations:**
   - Invoke AskUserQuestion with 3 options:
     - Update tech-stack.md + create ADR (requires user approval)
     - Adjust research scope (use existing approved tech)
     - Document as technical debt (defer decision)

**Outputs:**
- Compliance validation report (table format)
- Violation list by severity
- User decision (if CRITICAL violations exist)
- Updated research recommendations (if adjusted)

**Example Compliance Report:**
```markdown
## Framework Compliance Validation

**Validation Date:** 2025-11-17 15:22:11
**Context Files Checked:** 6/6 ✅

| Context File | Status | Violations | Details |
|--------------|--------|------------|---------|
| tech-stack.md | ❌ CRITICAL | 1 | Recommends Auth0, but tech-stack.md locks "AWS Cognito" |
| source-tree.md | ✅ PASS | 0 | — |
| dependencies.md | ✅ PASS | 0 | — |
| coding-standards.md | ✅ PASS | 0 | — |
| architecture-constraints.md | ✅ PASS | 0 | — |
| anti-patterns.md | ✅ PASS | 0 | — |

**Quality Gate Status:** ❌ BLOCKED (1 CRITICAL violation)
**Action Required:** User decision on tech-stack.md conflict
```

---

### Step 6: Report Generation

**Goal:** Create standardized research report with all findings.

**Report Sections (9 Required):**
1. Executive Summary (3-5 sentences)
2. Research Scope (questions, boundaries, assumptions)
3. Methodology Used ("Discovery Mode" + Perplexity queries)
4. Findings (alternatives, feasibility scores, evidence)
5. Framework Compliance Check (validation results)
6. Workflow State (current phase: Backlog/Architecture)
7. Recommendations (top 3 alternatives with rationale)
8. Risk Assessment (risks by severity with mitigation)
9. ADR Readiness (if applicable: evidence for ADR creation)

**YAML Frontmatter Schema:**
```yaml
---
research_id: RESEARCH-001
epic_id: EPIC-007
story_id: null  # (if story-specific)
workflow_state: Architecture
research_mode: discovery
timestamp: 2025-11-17T15:30:22Z
quality_gate_status: PASS  # PASS | WARN | FAIL | BLOCKED
version: 2.0
---
```

**Report Template:**
```markdown
---
research_id: RESEARCH-001
epic_id: EPIC-007
workflow_state: Architecture
research_mode: discovery
timestamp: 2025-11-17T15:30:22Z
quality_gate_status: PASS
version: 2.0
---

# Research Report: User Authentication System Feasibility

## Executive Summary

OAuth 2.0 authentication is highly feasible for our multi-tenant SaaS platform (feasibility score: 7.8/10). Top recommendation: AWS Cognito (aligns with tech-stack.md, cost-effective at scale, strong AWS integration). Alternative: Auth0 (superior DX, but requires tech-stack.md update + ADR). Risk mitigation required: vendor lock-in (AWS), complex initial setup.

## Research Scope

**Primary Questions:**
1. Is OAuth 2.0 feasible for multi-tenant SaaS?
2. Which provider (Auth0, Cognito, Firebase) best fits constraints?
3. What are implementation patterns for Python + React?

**Boundaries:**
- In-scope: OAuth 2.0, OIDC providers
- Out-of-scope: SAML, custom crypto
- Technology: Python/React (per tech-stack.md)

**Assumptions:**
- AWS deployment
- <1000 initial users → 10K within 6 months
- Budget: <$200/month

## Methodology Used

**Research Mode:** Discovery (broad exploration)
**Duration:** 4 minutes 38 seconds
**Perplexity Queries:** 3
- Query 1: Auth0 vs Cognito vs Firebase comparison (5 sources)
- Query 2: OAuth 2.0 adoption trends 2024 (4 sources)
- Query 3: Python/React OAuth patterns (6 sources)

**Source Quality:**
- Official docs: 8 sources (quality score: 10/10)
- Industry surveys: 3 sources (quality score: 7/10)
- Implementation guides: 4 sources (quality score: 6/10)

## Findings

[... detailed findings with comparison matrix, feasibility scores ...]

## Framework Compliance Check

[... compliance validation table ...]

## Workflow State

**Current State:** Architecture
**Research Focus:** Technology evaluation and pattern selection
**Staleness Check:** N/A (newly generated)

## Recommendations

**1. AWS Cognito (Recommended)**
- Feasibility: 8.2/10
- Aligns with tech-stack.md (AWS preferred)
- Cost-effective: $0.0055/MAU
- Strong AWS ecosystem integration
- Mitigation: Complex initial setup (allocate 1 week for integration)

**2. Auth0 (Alternative - Requires ADR)**
- Feasibility: 7.8/10
- Superior developer experience
- Requires tech-stack.md update (not currently approved)
- Cost: $23/mo base + $23/1K users (higher than budget)
- Trade-off: Ease of use vs cost + vendor lock-in

**3. Supertokens (Open-Source Option)**
- Feasibility: 6.5/10
- Free (self-hosted)
- Requires maintenance overhead
- Less mature (smaller community)
- Consider if budget absolutely constrained

## Risk Assessment

| Risk | Severity | Mitigation |
|------|----------|------------|
| Vendor lock-in (AWS) | MEDIUM | Abstract auth behind interface, use OIDC standards |
| Complex Cognito setup | MEDIUM | Allocate 1 week, use Terraform for IaC |
| Scaling costs | LOW | Monitor MAU, optimize token refresh policies |

## ADR Readiness

**ADR Required:** Yes (if selecting Auth0 over Cognito)
**ADR Title:** ADR-XXX: Authentication Provider Selection (Auth0 vs AWS Cognito)
**Evidence Collected:** ✅ (comparison matrix, cost analysis, integration complexity)
**Next Steps:** Create ADR documenting decision rationale if Auth0 chosen

---

**Report Generated:** 2025-11-17 15:35:42
**Report Location:** devforgeai/specs/research/feasibility/EPIC-007-2025-11-17-153542-research.md
```

**Outputs:**
- Complete research report (markdown file)
- YAML frontmatter with metadata
- Report saved to devforgeai/specs/research/feasibility/

---

## Integration with devforgeai-ideation

**Invocation Point:** Phase 5 (Feasibility & Constraints Analysis)

**Ideation Workflow:**
```
Phase 5 Step 5.1: Determine if feasibility research needed
  - If business idea is new/unproven → YES
  - If technology well-understood → NO (skip research)

Phase 5 Step 5.2: Invoke internet-sleuth (discovery mode)
  Task(
    subagent_type="internet-sleuth",
    description="Feasibility research for [epic]",
    prompt="Research Mode: discovery\nEpic: [epic description]\nQuestions: [primary questions]"
  )

Phase 5 Step 5.3: Receive research report
  - Extract technical_feasibility_score (0-10)
  - Extract go/no-go recommendation
  - Extract top 3 alternatives with trade-offs

Phase 5 Step 5.4: Incorporate into ideation output
  - Add feasibility section to epic document
  - Link research report in epic YAML frontmatter
  - Use recommendations in Phase 6 (Architecture Transition)
```

**Example Integration:**
```markdown
# Epic: User Authentication System

## Feasibility Analysis

**Research Report:** [RESEARCH-001](../research/feasibility/EPIC-007-2025-11-17-research.md)
**Technical Feasibility:** 8.2/10 (high confidence)
**Recommended Approach:** OAuth 2.0 with AWS Cognito
**Key Risks:** Vendor lock-in (MEDIUM), complex setup (MEDIUM)
**Go/No-Go:** ✅ GO with caution (mitigation strategies documented)

### Research Findings Summary
- OAuth 2.0 is industry standard (83% of SaaS companies use it - Stack Overflow Survey 2024)
- AWS Cognito provides best cost/feature balance for our constraints
- Implementation pattern well-documented (Python FastAPI + React integration guides available)
```

---

## Success Criteria

Discovery mode research succeeds when:
- [ ] Research scope clearly defined (3-5 questions, boundaries, assumptions)
- [ ] Broad research completed (3-5 Perplexity queries with quality sources)
- [ ] Alternatives identified (3-5 viable options with comparison matrix)
- [ ] Feasibility score calculated (0-10 with dimension breakdown)
- [ ] Framework compliance validated (6 context files checked, violations categorized)
- [ ] Report generated (9 required sections, YAML frontmatter correct)
- [ ] Duration <5 minutes (p95 threshold)
- [ ] Token usage <50K (within budget)

---

## Related Documentation

- `research-principles.md` - Core research methodology (always loaded)
- `investigation-mode-methodology.md` - Deep-dive research (narrower scope)
- `skill-coordination-patterns.md` - Task invocation examples
- `research-report-template.md` - Standard report structure
- `.claude/skills/devforgeai-ideation/SKILL.md` - Ideation integration

---

**Created:** 2025-11-17
**Version:** 1.0
**Lines:** 415 (target: 400 ✓)
**Purpose:** High-level exploration and feasibility assessment workflow
