# Research Report Template

**Purpose:** Standard structure for all internet-sleuth research reports ensuring consistency, completeness, and framework compliance.

**Usage:** All research reports must follow this template structure (9 required sections + YAML frontmatter).

---

## YAML Frontmatter Schema

**All fields required unless marked optional:**

```yaml
---
# Identifiers
research_id: RESEARCH-NNN         # Gap-aware ID (RESEARCH-001, RESEARCH-002, ...)
epic_id: EPIC-NNN | null          # Epic this research supports (null if not epic-specific)
story_id: STORY-NNN | null        # Story this research supports (null if not story-specific)

# Workflow Context
workflow_state: Backlog | Architecture | Ready for Dev | In Development | Dev Complete | QA In Progress | QA Approved | QA Failed | Releasing | Released
research_mode: discovery | investigation | competitive-analysis | repository-archaeology | market-intelligence

# Metadata
timestamp: YYYY-MM-DDTHH:MM:SSZ   # ISO 8601 format
quality_gate_status: PASS | WARN | FAIL | BLOCKED
version: "2.0"                    # Template version

# Optional Fields
author: string | null              # Researcher name (null if agent-generated)
tags: [string] | null              # Keywords (e.g., ["authentication", "oauth2", "aws"])
---
```

**Field Descriptions:**

| Field | Type | Required | Description | Example |
|-------|------|----------|-------------|---------|
| `research_id` | string | ✅ | Gap-aware ID | `RESEARCH-001` |
| `epic_id` | string\|null | ✅ | Epic reference | `EPIC-007` or `null` |
| `story_id` | string\|null | ✅ | Story reference | `STORY-042` or `null` |
| `workflow_state` | enum | ✅ | Current development phase | `Architecture` |
| `research_mode` | enum | ✅ | Research type | `discovery` |
| `timestamp` | datetime | ✅ | Research completion time | `2025-11-17T15:30:22Z` |
| `quality_gate_status` | enum | ✅ | Framework compliance result | `PASS` |
| `version` | string | ✅ | Template version | `2.0` |
| `author` | string\|null | ❌ | Researcher (null for agent) | `null` |
| `tags` | array\|null | ❌ | Keywords | `["oauth2", "aws"]` |

**Validation Rules:**
- `research_id` must match pattern `RESEARCH-[0-9]{3}` (e.g., RESEARCH-001, RESEARCH-042)
- `epic_id` must exist in `devforgeai/specs/Epics/` if not null
- `story_id` must exist in `devforgeai/specs/Stories/` if not null
- `workflow_state` must be one of 11 valid DevForgeAI states
- `research_mode` must be one of 5 valid modes
- `timestamp` must be valid ISO 8601 datetime
- `quality_gate_status` must be PASS | WARN | FAIL | BLOCKED
- `version` must be "2.0" (current template version)

---

## Report Sections (9 Required)

### 1. Executive Summary

**Purpose:** 2-3 sentence high-level summary of research findings and recommendations.

**Format:**
```markdown
## Executive Summary

[2-3 sentences summarizing: (1) what was researched, (2) key finding/recommendation, (3) critical insight or risk]

**Examples:**

**Discovery Mode:**
"Analyzed OAuth 2.0 feasibility for multi-tenant SaaS platform (feasibility score: 8.2/10). Top recommendation: AWS Cognito (aligns with tech-stack.md, cost-effective at scale, strong AWS integration). Critical risk: Complex initial setup (mitigate with 1-week integration sprint and Terraform IaC)."

**Repository Archaeology:**
"Analyzed 5 high-quality repositories (avg quality: 7.8/10) implementing OAuth 2.0 in Python/FastAPI. Top recommendation: Repository Pattern with dependency injection (9.2/10, production-proven across 3.2K+ deployments). Critical pitfall avoided: N+1 query anti-pattern (documented in 3 repositories with fixes)."

**Competitive Analysis:**
"Analyzed 5 major competitors in SaaS authentication market (Auth0, AWS Cognito, Firebase, Supertokens, Keycloak). Market opportunity: 'AWS-native auth with Auth0 features at Cognito prices' positioning gap. Key risk: Auth0 price cuts (mitigate with faster feature velocity and AWS ecosystem integration)."
```

**Content Requirements:**
- [ ] Max 3 sentences (concise, executive-friendly)
- [ ] Includes primary recommendation
- [ ] Mentions feasibility score OR quality score (if applicable)
- [ ] Highlights critical risk with mitigation

---

### 2. Research Scope

**Purpose:** Define research questions, boundaries, and assumptions.

**Format:**
```markdown
## Research Scope

**Primary Questions:**
1. [Question 1]
2. [Question 2]
3. [Question 3]
...

**Boundaries:**
- **In-scope:** [What was included in research]
- **Out-of-scope:** [What was explicitly excluded]
- **Technology constraints:** [From tech-stack.md or explicit]

**Assumptions:**
- [Assumption 1]
- [Assumption 2]
...

**Example:**

## Research Scope

**Primary Questions:**
1. Is OAuth 2.0 feasible for multi-tenant SaaS platform?
2. Which provider (Auth0, Cognito, Firebase) best fits our constraints?
3. What are implementation patterns for Python + React?

**Boundaries:**
- **In-scope:** OAuth 2.0, OIDC providers
- **Out-of-scope:** SAML, custom cryptography implementation
- **Technology constraints:** Python backend, React frontend (per tech-stack.md)

**Assumptions:**
- AWS deployment (AWS preferred in context files)
- <1000 initial users, scaling to 10K within 6 months
- Budget: <$200/month for authentication service
```

**Content Requirements:**
- [ ] 3-5 primary research questions
- [ ] Clear in-scope vs out-of-scope boundaries
- [ ] Technology constraints documented (from context files or explicit)
- [ ] Assumptions listed (budget, scale, timeline, etc.)

---

### 3. Methodology Used

**Purpose:** Document research approach, sources, and duration.

**Format:**
```markdown
## Methodology Used

**Research Mode:** [discovery | investigation | competitive-analysis | repository-archaeology | market-intelligence]
**Duration:** [X minutes Y seconds]
**Tools:** [Perplexity API | GitHub API | WebSearch | WebFetch | etc.]

**Data Sources:**
- [Source 1 with quality score]
- [Source 2 with quality score]
...

**Methodology Steps:**
1. [Step 1 description]
2. [Step 2 description]
...

**Example (Discovery Mode):**

## Methodology Used

**Research Mode:** Discovery (broad exploration)
**Duration:** 4 minutes 38 seconds
**Tools:** Perplexity API, WebSearch, WebFetch

**Data Sources:**
- Official documentation (8 sources, quality: 10/10)
- Industry surveys (3 sources, quality: 7/10)
- Implementation guides (4 sources, quality: 6/10)

**Methodology Steps:**
1. Formulated 3 Perplexity queries (OAuth comparison, adoption trends, implementation patterns)
2. Executed queries in parallel (collected 15 sources)
3. Analyzed alternatives (created comparison matrix for 5 providers)
4. Calculated feasibility scores (4 dimensions: technical, capability, risk, cost)
5. Validated against 6 context files (tech-stack.md, architecture-constraints.md, etc.)
6. Generated recommendations (ranked by composite score)
```

**Content Requirements:**
- [ ] Research mode specified
- [ ] Duration documented (minutes:seconds)
- [ ] Data sources listed with quality scores (official: 10, surveys: 7, blogs: 5, etc.)
- [ ] Methodology steps enumerated (3-8 steps typical)

---

### 4. Findings

**Purpose:** Present detailed research findings (comparison matrix, code examples, SWOT, etc.).

**Format:** Varies by research mode

**Discovery Mode Findings:**
```markdown
## Findings

### Alternatives Comparison

| Alternative | Feasibility | Pros | Cons | Cost (10K MAU) |
|-------------|-------------|------|------|----------------|
| AWS Cognito | 8.2/10 | AWS-native, scalable | Complex setup | $55/mo |
| Auth0 | 7.8/10 | Excellent DX | Expensive | $535/mo |
| Firebase | 7.5/10 | Easy setup | Google lock-in | $60/mo |

### Feasibility Assessment

**Dimension Breakdown:**
- Technical Feasibility: 9/10 (mature tech, well-documented)
- Team Capability: 8/10 (OAuth familiarity, learning curve <1 week)
- Risk Assessment: 7/10 (vendor lock-in medium, community stable)
- Cost Feasibility: 8/10 (within budget for 10K MAU)

**Composite Score:** (9*0.4) + (8*0.2) + (7*0.2) + (8*0.2) = **8.2/10**
```

**Repository Archaeology Findings:**
```markdown
## Findings

### Repository Quality Scores

| Repository | Stars | Quality Score | Last Commit | Notes |
|------------|-------|---------------|-------------|-------|
| fastapi-users | 4.5K | 10/10 | 2 days ago | Exemplary (production-ready) |
| cosmic-python | 2.1K | 9/10 | 1 week ago | High quality (DDD focus) |
| fastapi-ddd | 850 | 7/10 | 3 months ago | Acceptable reference |

### Code Patterns Extracted

**Pattern 1: Repository Pattern with Dependency Injection**

**Source:** fastapi-ddd-example (quality: 8/10)

```python
# Abstract interface
class UserRepository(ABC):
    @abstractmethod
    async def get_by_id(self, user_id: int) -> Optional[User]:
        pass

# Concrete implementation
class SQLAlchemyUserRepository(UserRepository):
    def __init__(self, session: AsyncSession):
        self.session = session
    ...
```

**Benefits:** Testability, flexibility, framework compliance
**Applicability:** Multi-entity domains (>5 entities), long-term projects
```

**Competitive Analysis Findings:**
```markdown
## Findings

### Feature Comparison Matrix

| Feature | Auth0 | AWS Cognito | Firebase | Supertokens |
|---------|-------|-------------|----------|-------------|
| OAuth 2.0 | ✅ Full | ✅ Full | ✅ Full | ✅ Full |
| SSO (SAML) | ✅ Enterprise | ✅ Yes | ❌ No | ✅ Yes |
| Self-hosted | ❌ No | ❌ No | ❌ No | ✅ Yes |

### SWOT Analysis

**Strengths:**
- S1: AWS Integration (native AWS ecosystem fit)
- S2: Cost Efficiency (lower than Auth0)
...

**Weaknesses:**
- W1: Feature Parity (missing some Auth0 advanced features)
...

**Opportunities:**
- O1: Open-Source Adoption (growing self-hosted preference)
...

**Threats:**
- T1: Auth0 Innovation (Okta backing, large R&D budget)
...
```

**Content Requirements:**
- [ ] Findings specific to research mode (comparison matrix, code examples, SWOT, etc.)
- [ ] Evidence-based (all claims have sources)
- [ ] Structured format (tables, code blocks, headings)
- [ ] Quality scores included (repositories, sources, etc.)

---

### 5. Framework Compliance Check

**Purpose:** Validate research findings against all 6 DevForgeAI context files.

**Format:**
```markdown
## Framework Compliance Check

**Validation Date:** YYYY-MM-DD HH:MM:SS
**Context Files Checked:** 6/6 ✅

| Context File | Status | Violations | Details |
|--------------|--------|------------|---------|
| tech-stack.md | ✅ PASS | 0 | Recommended tech (AWS Cognito) aligns with AWS preference |
| source-tree.md | ✅ PASS | 0 | — |
| dependencies.md | ⚠️ WARN | 1 MEDIUM | Suggests `authlib` package (not in approved list) |
| coding-standards.md | ✅ PASS | 0 | — |
| architecture-constraints.md | ✅ PASS | 0 | Repository pattern respects layer boundaries |
| anti-patterns.md | ✅ PASS | 0 | No God Objects or forbidden patterns detected |

**Violations Detail:**

**MEDIUM (dependencies.md):**
- **Issue:** Research recommends `authlib` Python package for OAuth 2.0
- **Context:** dependencies.md does not list `authlib` in approved packages
- **Resolution:** Add `authlib` to dependencies.md with rationale OR use existing approved package (`oauthlib`)
- **User Action Required:** No (proceed with warning, address during architecture phase)

**Quality Gate Status:** ⚠️ WARN (1 MEDIUM violation - non-blocking)
**Recommendation:** Proceed with research findings, resolve MEDIUM violation during implementation
```

**Content Requirements:**
- [ ] Validation timestamp
- [ ] 6/6 context files checked (all must be validated)
- [ ] Status per file (✅ PASS, ⚠️ WARN, ❌ FAIL, 🚫 BLOCKED)
- [ ] Violation count and severity (CRITICAL, HIGH, MEDIUM, LOW)
- [ ] Detailed violation explanations (issue, context, resolution, user action)
- [ ] Quality gate status (PASS, WARN, FAIL, BLOCKED)

**Quality Gate Status Definitions:**
- **PASS:** No violations, fully compliant
- **WARN:** LOW or MEDIUM violations (non-blocking, log and proceed)
- **FAIL:** HIGH violations (blocking, requires resolution before proceeding)
- **BLOCKED:** CRITICAL violations (requires user decision via AskUserQuestion)

---

### 6. Workflow State

**Purpose:** Document current development phase and research focus alignment.

**Format:**
```markdown
## Workflow State

**Current State:** [Backlog | Architecture | Ready for Dev | In Development | Dev Complete | QA In Progress | QA Approved | QA Failed | Releasing | Released]
**Research Focus:** [Focus appropriate for current state]
**Staleness Check:** [CURRENT | STALE]

**Example:**

## Workflow State

**Current State:** Architecture
**Research Focus:** Technology evaluation and pattern selection (aligns with Architecture phase goals)
**Staleness Check:** CURRENT (research completed 2025-11-17, workflow state unchanged)

**Staleness Criteria:**
- **STALE if:** Report >30 days old OR 2+ workflow states behind current story/epic state
- **Example:** Report from Backlog (2025-10-01), current state In Development (2 states ahead, 47 days old) → STALE
```

**Content Requirements:**
- [ ] Current workflow state documented
- [ ] Research focus matches state (Architecture → technology evaluation, In Development → implementation patterns, etc.)
- [ ] Staleness check performed (CURRENT or STALE with reason)

---

### 7. Recommendations

**Purpose:** Provide actionable recommendations ranked by priority.

**Format:**
```markdown
## Recommendations

### 1. [Top Recommendation] ⭐ (Score: X.X/10)

**Approach:** [Technology/pattern/strategy name]
**Feasibility:** X.X/10
**Evidence:** [Sources with quality scores]

**Benefits:**
- ✅ [Benefit 1]
- ✅ [Benefit 2]
...

**Drawbacks:**
- ❌ [Drawback 1]
- ❌ [Drawback 2]
...

**Applicability:**
- ✅ [When to use]
- ❌ [When NOT to use]

**Implementation:**
- **Effort:** [Time estimate]
- **Complexity:** [Low | Medium | High]
- **Prerequisites:** [Dependencies, skills, resources]

**Example:**

## Recommendations

### 1. AWS Cognito (Recommended) ⭐ (Feasibility: 8.2/10)

**Approach:** AWS Cognito User Pools for authentication
**Evidence:** Official AWS docs (10/10), 3 implementation guides (8/10 avg)

**Benefits:**
- ✅ AWS-native (aligns with tech-stack.md preference)
- ✅ Cost-effective ($55/mo for 10K MAU vs Auth0 $535/mo)
- ✅ Scalable (handles millions of users)
- ✅ Compliance (SOC 2, HIPAA, GDPR certified)

**Drawbacks:**
- ❌ Complex initial setup (learning curve 1-2 weeks)
- ❌ Limited customization (UI customization restricted)
- ❌ AWS vendor lock-in (migration challenging)

**Applicability:**
- ✅ AWS-first teams (already on AWS infrastructure)
- ✅ Budget-conscious (premium features, AWS pricing)
- ❌ Simple prototypes (overkill for MVP)

**Implementation:**
- **Effort:** 1-2 weeks (integration + testing)
- **Complexity:** Medium (AWS-specific knowledge required)
- **Prerequisites:** AWS account, Terraform or CDK (IaC recommended)

### 2. Auth0 (Alternative) (Feasibility: 7.8/10)

[... similar structure ...]

### 3. Supertokens (Open-Source) (Feasibility: 6.5/10)

[... similar structure ...]
```

**Content Requirements:**
- [ ] Top 3 recommendations ranked (scores, rationale)
- [ ] Benefits and drawbacks for each
- [ ] Applicability criteria (when to use, when NOT to use)
- [ ] Implementation details (effort, complexity, prerequisites)

---

### 8. Risk Assessment

**Purpose:** Identify risks with severity levels and mitigation strategies.

**Format:**
```markdown
## Risk Assessment

| Risk | Severity | Probability | Impact | Mitigation |
|------|----------|-------------|--------|------------|
| [Risk 1] | CRITICAL\|HIGH\|MEDIUM\|LOW | HIGH\|MEDIUM\|LOW | [Impact description] | [Mitigation strategy] |
| [Risk 2] | ... | ... | ... | ... |

**Severity Definitions:**
- **CRITICAL:** Project failure or major security breach
- **HIGH:** Significant delay (>2 weeks) or cost overrun (>50%)
- **MEDIUM:** Minor delay (<2 weeks) or moderate cost increase (<50%)
- **LOW:** Minimal impact, manageable with standard practices

**Example:**

## Risk Assessment

| Risk | Severity | Probability | Impact | Mitigation |
|------|----------|-------------|--------|------------|
| Vendor lock-in (AWS Cognito) | MEDIUM | HIGH | Migration to alternative costly | Abstract auth behind interface layer, use OIDC standards |
| Complex Cognito setup | MEDIUM | MEDIUM | 1-2 week delay in integration | Allocate dedicated sprint, use Terraform IaC, follow official guides |
| Security breach | CRITICAL | LOW | Reputational damage, data loss | Hire security firm for audit, implement bug bounty, regular penetration testing |
| Auth0 price cuts | MEDIUM | LOW | Competitive pressure | Differentiate on AWS integration, faster feature velocity |
| AWS Cognito DX improvements | LOW | MEDIUM | Our value-add reduced | Build on top of Cognito (not compete), add abstraction layer |

**Risk Matrix:**

```
         Impact
         ↑
    HIGH │   🔴 Security Breach
         │   (CRITICAL, LOW prob)
         │
  MEDIUM │   🟠 Vendor Lock-in        🟠 Complex Setup
         │   (MEDIUM, HIGH prob)      (MEDIUM, MEDIUM prob)
         │
     LOW │                             🟡 DX Improvements
         │                             (LOW, MEDIUM prob)
         │
         └────────────────────────────────────→ Probability
                  LOW          MEDIUM         HIGH
```
```

**Content Requirements:**
- [ ] 5-10 risks identified
- [ ] Severity, probability, impact documented
- [ ] Mitigation strategy for each risk
- [ ] Risk matrix visualization (optional but recommended)

---

### 9. ADR Readiness

**Purpose:** Document if Architecture Decision Record (ADR) is required and evidence collected.

**Format:**
```markdown
## ADR Readiness

**ADR Required:** [Yes | No | Conditional]
**ADR Title:** ADR-XXX: [Decision title]
**Evidence Collected:** ✅ | ⚠️ | ❌

**Evidence Summary:**
- [Evidence type 1]: ✅ Complete
- [Evidence type 2]: ✅ Complete
...

**Next Steps:**
1. [Step 1]
2. [Step 2]
...

**Example (ADR Required):**

## ADR Readiness

**ADR Required:** Yes (technology selection decision)
**ADR Title:** ADR-XXX: Adopt AWS Cognito for Authentication
**Evidence Collected:** ✅ Complete

**Evidence Summary:**
- Comparison matrix: ✅ (5 alternatives evaluated)
- Cost analysis: ✅ (TCO calculated for 3 scenarios)
- Implementation patterns: ✅ (repository archaeology completed)
- Risk assessment: ✅ (5 risks identified with mitigation)
- Framework compliance: ✅ (validated against 6 context files)

**Next Steps:**
1. Create ADR in `devforgeai/specs/adrs/ADR-XXX-adopt-cognito.md`
2. Document decision context (why this decision now)
3. Record decision (AWS Cognito selected)
4. Document alternatives considered (Auth0, Firebase, Supertokens)
5. Note consequences (vendor lock-in, complex setup, cost efficiency)
6. Update tech-stack.md with Cognito entry
7. Link research report in ADR (research_id: RESEARCH-001)

**Example (No ADR Required):**

## ADR Readiness

**ADR Required:** No (informational research only, no decision made)
**Evidence Collected:** N/A

**Rationale:** This research was exploratory (feasibility assessment for epic planning). No technology selection decision made yet. ADR will be required during architecture phase when specific technology is chosen.
```

**Content Requirements:**
- [ ] ADR requirement status (Yes, No, Conditional with criteria)
- [ ] ADR title (if required)
- [ ] Evidence collection status (✅ Complete, ⚠️ Partial, ❌ Incomplete)
- [ ] Evidence summary (what evidence exists for ADR creation)
- [ ] Next steps (how to create ADR, update context files, etc.)

---

## Template Footer

**All reports must include:**

```markdown
---

**Report Generated:** YYYY-MM-DD HH:MM:SS
**Report Location:** devforgeai/specs/research/[feasibility|examples|shared]/[filename].md
**Research ID:** RESEARCH-NNN
**Version:** 2.0 (template version)
```

---

## Validation Checklist

Before finalizing report, validate:

- [ ] YAML frontmatter complete (all required fields present)
- [ ] research_id follows pattern `RESEARCH-[0-9]{3}`
- [ ] epic_id/story_id exist in devforgeai/specs/ (if not null)
- [ ] All 9 sections present (Executive Summary → ADR Readiness)
- [ ] Executive Summary ≤3 sentences
- [ ] Research Scope has 3-5 questions
- [ ] Findings appropriate for research mode (comparison matrix, code examples, SWOT, etc.)
- [ ] Framework Compliance validates all 6 context files
- [ ] Workflow State documents current phase
- [ ] Recommendations ranked (top 3 with scores)
- [ ] Risk Assessment has 5-10 risks with mitigation
- [ ] ADR Readiness status clear (Yes/No/Conditional)
- [ ] Report footer included (timestamp, location, version)

---

**Created:** 2025-11-17
**Template Version:** 2.0
**Purpose:** Standard structure for all internet-sleuth research reports
