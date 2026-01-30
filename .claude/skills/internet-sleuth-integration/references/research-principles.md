# Research Principles - Internet-Sleuth Foundation

**Purpose:** Core research methodology and framework integration principles shared across all internet-sleuth research modes (discovery, investigation, competitive-analysis, repository-archaeology, market-intelligence).

**Loaded:** Always (base reference for all research operations)

---

## Core Principles

### 1. Evidence-Based Research Only

**Principle:** All research findings must be backed by verifiable evidence with citations.

**Requirements:**
- Every claim must have ≥1 source URL (HTTPS only, HTTP flagged)
- Primary sources preferred over secondary (official docs > blog posts)
- Evidence quality scoring: Official docs (10/10), Academic papers (9/10), Community surveys (7/10), Blog posts (5/10), Social media (3/10)

**Anti-Patterns:**
- ❌ Speculation without evidence
- ❌ Assumptions presented as facts
- ❌ Outdated sources (>2 years for technology research)
- ❌ HTTP URLs (security risk)
- ❌ Broken/404 links

**Validation:**
```python
def validate_evidence(claim, sources):
    if len(sources) == 0:
        return "INVALID: No sources provided"

    for source in sources:
        if not source.startswith("https://"):
            flag_as_insecure(source)

        if is_404(source):
            return f"INVALID: Broken link {source}"

        if source_age(source) > 2 years and is_technology_research:
            flag_as_outdated(source)

    return "VALID"
```

---

### 2. Framework-Aware Behavior

**Principle:** All research must respect DevForgeAI context files and framework constraints. Never operate autonomously.

**Context Files to Reference (All 6 Required):**
1. **tech-stack.md** - Locked technologies (cannot recommend alternatives without ADR)
2. **source-tree.md** - Project structure conventions
3. **dependencies.md** - Approved packages (cannot add unapproved deps)
4. **coding-standards.md** - Code patterns and naming conventions
5. **architecture-constraints.md** - Layer boundaries (domain/application/infrastructure)
6. **anti-patterns.md** - Forbidden patterns (must not recommend)

**Validation Workflow:**
```
1. Research generates recommendation (e.g., "Use PostgreSQL")
2. Invoke context-validator subagent
3. Check tech-stack.md: "MySQL" specified (CONFLICT!)
4. Categorize violation: CRITICAL (contradicts locked tech)
5. Trigger AskUserQuestion:
   - Option A: Update tech-stack.md + create ADR (user approval)
   - Option B: Adjust research scope to MySQL (use existing tech)
   - Option C: Document as technical debt (defer decision)
6. Never autonomously override context files
```

**Framework Integration Points:**
- **Invoked by:** devforgeai-ideation (Phase 5), devforgeai-architecture (Phase 2)
- **Invokes:** context-validator (quality gate validation), requirements-analyst (optional), architect-reviewer (optional)
- **Updates:** Research reports in devforgeai/specs/research/, epic/story YAML frontmatter

**Success Criteria:**
- [ ] All 6 context files validated before returning research report
- [ ] CRITICAL violations trigger AskUserQuestion (user approval required)
- [ ] Research report includes "Framework Compliance" section
- [ ] No autonomous context file modifications

---

### 3. Workflow State Awareness

**Principle:** Research recommendations must adapt to current development phase.

**Workflow States (11 Total):**
```
Backlog → Architecture → Ready for Dev → In Development →
Dev Complete → QA In Progress → QA Approved | QA Failed →
Releasing → Released
```

**Research Focus by State:**

| Workflow State | Research Focus | Example Questions |
|----------------|----------------|-------------------|
| **Backlog** | Feasibility, market viability | "Is this idea technically feasible?" |
| **Architecture** | Technology evaluation, pattern selection | "Which database best fits constraints?" |
| **Ready for Dev** | Implementation patterns, best practices | "How do others implement this pattern?" |
| **In Development** | Debugging, performance optimization | "What are common pitfalls with this library?" |
| **QA In Progress** | Testing strategies, edge cases | "What edge cases have others encountered?" |
| **Released** | Post-release monitoring, user feedback | "How do users typically use this feature?" |

**Stale Research Detection:**
- **Trigger:** Report >30 days old OR 2+ workflow states behind current story/epic state
- **Example:** Report from "Backlog" (2025-10-01), current state "In Development" (2025-11-17, 47 days, 2 states ahead)
- **Action:** Flag as "STALE - Re-research Recommended" in report header

**Implementation:**
```python
def detect_staleness(report_date, report_state, current_date, current_state):
    age_days = (current_date - report_date).days
    state_distance = workflow_states.index(current_state) - workflow_states.index(report_state)

    if age_days > 30 or state_distance >= 2:
        return {
            "status": "STALE",
            "reason": f"Age: {age_days} days, State distance: {state_distance} states",
            "recommendation": "Re-research with current workflow focus"
        }
    return {"status": "CURRENT"}
```

---

### 4. Quality Gate Integration

**Principle:** All research reports must pass quality validation before delivery.

**Quality Gate Workflow:**
```
1. Research completed → Generate report draft
2. Validate report structure (9 required sections)
3. Invoke context-validator subagent
4. Check recommendations vs 6 context files
5. Categorize violations by severity:
   - CRITICAL: Contradicts tech-stack.md (locked tech)
   - HIGH: Violates architecture-constraints.md
   - MEDIUM: Conflicts with coding-standards.md
   - LOW: Informational (no blocking issue)
6. Add "Framework Compliance" section to report
7. HALT on CRITICAL violations → AskUserQuestion
8. Proceed on HIGH/MEDIUM/LOW (log violations)
```

**Violation Severity Matrix:**

| Violation Type | Severity | Blocker? | Action |
|----------------|----------|----------|--------|
| Recommends tech not in tech-stack.md | CRITICAL | Yes | AskUserQuestion (update or adjust) |
| Violates layer boundaries | HIGH | No | Log warning, include in compliance section |
| Naming convention mismatch | MEDIUM | No | Log info, suggest alignment |
| Minor style preference | LOW | No | Informational note |

**Compliance Section Template:**
```markdown
## Framework Compliance Check

**Validation Date:** 2025-11-17 14:35:22
**Context Files Checked:** 6/6 ✅

| Context File | Status | Violations |
|--------------|--------|------------|
| tech-stack.md | ✅ PASS | 0 |
| source-tree.md | ✅ PASS | 0 |
| dependencies.md | ⚠️ WARN | 1 MEDIUM (suggests package not in approved list) |
| coding-standards.md | ✅ PASS | 0 |
| architecture-constraints.md | ✅ PASS | 0 |
| anti-patterns.md | ✅ PASS | 0 |

**Quality Gate Status:** PASS (1 MEDIUM violation logged)
**Recommendation:** Proceed with research findings, address MEDIUM violation during architecture phase.
```

---

### 5. Progressive Disclosure Pattern

**Principle:** Load only necessary methodology reference files to prevent token bloat.

**Research Modes (5 Total):**
1. **discovery** - High-level exploration, broad scope
2. **investigation** - Deep-dive analysis, narrow scope
3. **competitive-analysis** - Market landscape, SWOT analysis
4. **repository-archaeology** - GitHub mining, implementation patterns
5. **market-intelligence** - User trends, adoption metrics

**Progressive Loading Strategy:**
```
Base (Always): research-principles.md (~300 lines)
Mode-Specific (Conditional):
  - discovery → discovery-mode-methodology.md (~400 lines)
  - investigation → investigation-mode-methodology.md (~400 lines)
  - competitive-analysis → competitive-analysis-patterns.md (~500 lines)
  - repository-archaeology → repository-archaeology-guide.md (~600 lines)
  - market-intelligence → market-intelligence-guide.md (~450 lines)

Total per operation: 700-900 lines (not all 2,500+ lines)
```

**Token Efficiency:**
- **Without progressive disclosure:** 2,500 lines loaded per operation (~20K tokens)
- **With progressive disclosure:** 700-900 lines loaded per operation (~7K tokens)
- **Savings:** 65% token reduction per research operation

**Implementation:**
```python
def load_methodology(research_mode):
    # Always load base
    base = read_file("research-principles.md")  # 300 lines

    # Conditionally load mode-specific
    mode_files = {
        "discovery": "discovery-mode-methodology.md",
        "investigation": "investigation-mode-methodology.md",
        "competitive-analysis": "competitive-analysis-patterns.md",
        "repository-archaeology": "repository-archaeology-guide.md",
        "market-intelligence": "market-intelligence-guide.md"
    }

    if research_mode in mode_files:
        mode_specific = read_file(mode_files[research_mode])
        return base + mode_specific  # ~700-900 lines total

    return base  # fallback to base only
```

---

## DevForgeAI Framework Integration

### Skill Coordination

**internet-sleuth is invoked by:**
- devforgeai-ideation (Phase 5: Feasibility Analysis)
- devforgeai-architecture (Phase 2: Technology Selection)
- devforgeai-orchestration (optional: research checkpoints)

**internet-sleuth invokes:**
- context-validator (quality gate validation)
- requirements-analyst (optional: requirement synthesis)
- architect-reviewer (optional: architecture pattern evaluation)

**Invocation Pattern:**
```
Task(
  subagent_type="internet-sleuth",
  description="Research React ecosystem feasibility",
  prompt="""
  Research Mode: discovery
  Research Scope: React ecosystem for SaaS platform
  Context: Epic EPIC-007 (User Authentication System), Workflow State: Architecture
  Required Outputs: Technical feasibility score (0-10), framework alternatives, community health
  Constraints: Respect tech-stack.md (may specify existing framework), no autonomous tech changes
  """
)
```

**Result Structure:**
```json
{
  "research_id": "RESEARCH-001",
  "technical_feasibility_score": 8.5,
  "market_viability": "HIGH",
  "competitive_landscape": {
    "alternatives": ["Vue.js", "Angular", "Svelte"],
    "react_advantages": ["Largest ecosystem", "Corporate backing", "Job market"],
    "react_disadvantages": ["Learning curve", "Boilerplate overhead"]
  },
  "risk_factors": [
    "Frequent breaking changes in minor versions",
    "Dependency on third-party libraries for state management"
  ],
  "framework_compliance": "PASS",
  "workflow_state": "Architecture",
  "report_path": "devforgeai/specs/research/feasibility/EPIC-007-2025-11-17-143022-research.md"
}
```

---

### Token Budget Management

**Target:** <50K tokens per research operation (isolated context)

**Budget Breakdown:**
- Research principles: ~2.5K tokens (always loaded)
- Mode-specific methodology: ~5K tokens (conditional)
- Research execution (Perplexity API calls): ~30K tokens
- Report generation: ~5K tokens
- Quality gate validation: ~3K tokens
- **Total:** ~45K tokens (within budget)

**Exceeding Budget:**
- If research operation exceeds 50K → flag for optimization
- Consider: Splitting into multiple focused research tasks
- Example: "React ecosystem" → "React core feasibility" + "React ecosystem libraries"

---

## Evidence Standards

### URL Validation

**Requirements:**
- HTTPS only (HTTP URLs flagged as INSECURE)
- No broken/404 links (validation required)
- No paywalled content (prefer open-access sources)
- Archive.org snapshots acceptable for historical research

**Example Validation:**
```python
def validate_url(url):
    if not url.startswith("https://"):
        return {"status": "WARN", "message": "HTTP URL (insecure)"}

    response = requests.head(url, timeout=5)
    if response.status_code == 404:
        return {"status": "ERROR", "message": "Broken link (404)"}

    if "paywall" in response.headers.get("Content-Type", ""):
        return {"status": "WARN", "message": "Paywalled content"}

    return {"status": "VALID"}
```

---

### Source Quality Scoring

**Scoring Rubric (0-10):**

| Source Type | Score | Criteria |
|-------------|-------|----------|
| Official documentation | 10 | react.dev, python.org, etc. |
| Academic papers | 9 | Peer-reviewed, published in journals |
| Technical books | 8 | Published by reputable publishers (O'Reilly, Manning) |
| Community surveys | 7 | Stack Overflow Survey, State of JS |
| High-quality blogs | 6 | Martin Fowler, Kent Beck, industry experts |
| Standard blogs | 5 | Individual developer blogs (no peer review) |
| Forums/discussions | 4 | Reddit, HackerNews, GitHub Discussions |
| Social media | 3 | Twitter/X, LinkedIn posts |
| Unsourced claims | 1 | No verification possible |

**Usage:**
- Prioritize high-scoring sources (≥7) for critical decisions
- Use low-scoring sources (≤5) for trend indicators only
- Always cite source scores in research reports

---

## Success Criteria

Research operation succeeds when:
- [ ] Evidence-based (all claims have sources with quality scores)
- [ ] Framework-aware (validated against 6 context files)
- [ ] Workflow-adapted (recommendations match current development phase)
- [ ] Quality-gated (compliance section present, violations categorized)
- [ ] Progressively-loaded (only necessary methodology files loaded)
- [ ] Token-efficient (<50K per operation)
- [ ] Report-complete (9 required sections present)
- [ ] YAML-valid (frontmatter schema correct)

---

## Related Documentation

- `discovery-mode-methodology.md` - Discovery research workflow
- `investigation-mode-methodology.md` - Deep-dive investigation workflow
- `competitive-analysis-patterns.md` - Market analysis patterns
- `repository-archaeology-guide.md` - GitHub mining strategies
- `market-intelligence-guide.md` - User trend analysis
- `skill-coordination-patterns.md` - Task invocation examples
- `research-report-template.md` - Standard report structure

---

**Created:** 2025-11-17
**Version:** 1.0
**Lines:** 295 (target: 300)
**Purpose:** Shared foundation for all internet-sleuth research operations
