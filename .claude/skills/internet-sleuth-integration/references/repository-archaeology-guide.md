# Repository Archaeology Guide - Internet-Sleuth

**Purpose:** Mine GitHub repositories for real-world implementation patterns, code examples, and architectural decisions.

**When to Use:** devforgeai-architecture Phase 2 (Technology Selection), implementation research during "Ready for Dev" / "In Development" states, pattern validation

**Loaded:** Conditionally (when research_mode = "repository-archaeology")

---

## Repository Archaeology Overview

**Scope:** Deep code analysis and pattern extraction (depth over breadth)
**Duration:** 8-10 minutes (p95)
**Output:** Implementation patterns with code examples, quality-scored repositories, architectural insights

**Research Questions Answered:**
- "How do production systems implement [pattern]?"
- "What are common pitfalls in [technology] implementations?"
- "What architectural patterns work well at scale?"
- "What are real-world code examples for [feature]?"

**Not Answered (Use discovery mode instead):**
- "Is [technology] feasible?"
- "What are all alternatives to [technology]?"

---

## Archaeology Workflow (8 Steps)

### Step 1: Define Search Strategy

**Goal:** Formulate precise GitHub search queries to find relevant repositories.

**GitHub Search Syntax:**
```
[keyword] language:[lang] stars:>[stars] pushed:>[date] archived:false
```

**Search Dimensions:**

**1. Technology Keyword**
- Core technology: "fastapi authentication"
- Pattern: "repository pattern python"
- Integration: "stripe payment integration"

**2. Language Filter**
- language:python
- language:typescript
- language:go

**3. Quality Filters**
- stars:>100 (minimum community validation)
- stars:>500 (well-established)
- stars:>1000 (production-proven)

**4. Recency Filter**
- pushed:>2023-01-01 (actively maintained)
- pushed:>2024-01-01 (cutting-edge)
- created:>2022-01-01 (not legacy)

**5. Archive Status**
- archived:false (exclude abandoned projects)

**Example Search Queries:**

**Query 1: OAuth 2.0 + FastAPI**
```
"fastapi oauth2" language:python stars:>100 pushed:>2023-01-01 archived:false
```

**Query 2: Repository Pattern + Domain-Driven Design**
```
"repository pattern" "domain driven design" language:python stars:>200 pushed:>2023-01-01
```

**Query 3: Multi-Tenancy SaaS**
```
"multi tenant" "saas" language:python stars:>50 pushed:>2024-01-01 archived:false
```

**Advanced Filters:**
- `in:readme` - Search only README files (high-level docs)
- `in:file` - Search code files (implementation details)
- `path:/` - Search specific directories (e.g., `path:src/auth`)
- `filename:` - Search specific filenames (e.g., `filename:auth.py`)

**Actions:**
1. Formulate 3-5 search queries based on research questions
2. Execute searches via GitHub API or gh CLI
3. Filter results by quality criteria
4. Select 5-10 repositories for deep analysis

**Outputs:**
- Search query list (with rationale)
- Repository candidate list (5-10 repos)
- Initial quality assessment (stars, last push date, contributor count)

---

### Step 2: Repository Quality Scoring

**Goal:** Assess repository quality before deep code analysis.

**Quality Rubric (0-10 Scale):**

**1. Community Health (0-3 points)**
- ≥1000 stars: 3 points
- 500-999 stars: 2 points
- 100-499 stars: 1 point
- <100 stars: 0 points

**2. Maintenance Activity (0-2 points)**
- Commits in last 30 days: 2 points
- Commits in last 90 days: 1 point
- No commits in 90+ days: 0 points

**3. Documentation Quality (0-2 points)**
- Comprehensive README + docs/: 2 points
- README only: 1 point
- Minimal/no docs: 0 points

**4. Test Coverage (0-2 points)**
- Tests directory + CI badges: 2 points
- Tests directory (no CI): 1 point
- No tests: 0 points

**5. Production Indicators (0-1 point)**
- Docker/Kubernetes configs: +0.5 points
- CI/CD pipelines: +0.5 points
- (Both present: 1 point total)

**Total Score:** 0-10 points

**Quality Thresholds:**
- **9-10:** Exemplary (production-ready, well-maintained)
- **7-8:** High quality (good reference for patterns)
- **5-6:** Acceptable (use with caution, validate patterns)
- **0-4:** Low quality (skip or note limitations)

**Example Scoring:**

**Repository: fastapi-users/fastapi-users**
- Stars: 3.2K → 3 points (Community Health)
- Last commit: 2 days ago → 2 points (Maintenance)
- Docs: README + docs/ + examples/ → 2 points (Documentation)
- Tests: /tests/ + GitHub Actions CI → 2 points (Test Coverage)
- Production: Dockerfile + .github/workflows/ → 1 point (Production Indicators)
- **Total:** 10/10 (Exemplary)

**Repository: small-fastapi-example/auth**
- Stars: 45 → 0 points (Community Health)
- Last commit: 8 months ago → 0 points (Maintenance)
- Docs: Basic README only → 1 point (Documentation)
- Tests: None → 0 points (Test Coverage)
- Production: None → 0 points (Production Indicators)
- **Total:** 1/10 (Low quality - skip)

**Actions:**
1. Score each repository candidate (5-10 repos)
2. Filter: Keep repos with score ≥5
3. Prioritize: Analyze highest-scoring repos first
4. Document: Record scores in research report

**Outputs:**
- Quality scores for each repository (table format)
- Filtered repository list (score ≥5)
- Prioritized analysis queue (highest score first)

---

### Step 3: Code Pattern Extraction

**Goal:** Extract implementation patterns and code examples from selected repositories.

**Pattern Types:**

**1. Architectural Patterns**
- Layered architecture (domain / application / infrastructure)
- Repository pattern (data access abstraction)
- Dependency injection (service registration)
- Event-driven architecture (domain events, message queues)

**2. Design Patterns**
- Factory pattern (object creation)
- Strategy pattern (algorithm selection)
- Observer pattern (event handling)
- Decorator pattern (cross-cutting concerns)

**3. Integration Patterns**
- API client patterns (authentication, retry logic, error handling)
- Database patterns (connection pooling, transaction management)
- Cache patterns (cache-aside, write-through, read-through)
- Message queue patterns (pub/sub, request/reply)

**4. Security Patterns**
- Authentication flows (OAuth 2.0, JWT, session management)
- Authorization patterns (RBAC, ABAC, policy-based)
- Input validation (sanitization, whitelisting)
- Secrets management (environment variables, vaults)

**Extraction Workflow:**

**Step 3.1: Clone repository (optional, for deep analysis)**
```bash
gh repo clone [owner]/[repo] /tmp/research/[repo]
cd /tmp/research/[repo]
```

**Step 3.2: Identify key files**
- Entry points: `main.py`, `app.py`, `__init__.py`
- Core logic: `src/domain/`, `src/services/`, `src/repositories/`
- Configuration: `config.py`, `.env.example`, `settings.py`
- Tests: `tests/`, `test_*.py`

**Step 3.3: Extract code snippets**
```python
# Example: Repository Pattern

# File: src/repositories/user_repository.py
from abc import ABC, abstractmethod
from typing import List, Optional
from src.domain.user import User

class UserRepository(ABC):
    """Abstract repository for user persistence."""

    @abstractmethod
    async def get_by_id(self, user_id: int) -> Optional[User]:
        """Retrieve user by ID."""
        pass

    @abstractmethod
    async def get_by_email(self, email: str) -> Optional[User]:
        """Retrieve user by email."""
        pass

    @abstractmethod
    async def save(self, user: User) -> User:
        """Persist user."""
        pass

# File: src/infrastructure/sqlalchemy_user_repository.py
from sqlalchemy.ext.asyncio import AsyncSession
from src.repositories.user_repository import UserRepository
from src.domain.user import User
from src.infrastructure.models import UserModel

class SQLAlchemyUserRepository(UserRepository):
    """SQLAlchemy implementation of UserRepository."""

    def __init__(self, session: AsyncSession):
        self.session = session

    async def get_by_id(self, user_id: int) -> Optional[User]:
        result = await self.session.execute(
            select(UserModel).where(UserModel.id == user_id)
        )
        model = result.scalar_one_or_none()
        return User.from_orm(model) if model else None

    async def save(self, user: User) -> User:
        model = UserModel(**user.dict())
        self.session.add(model)
        await self.session.commit()
        await self.session.refresh(model)
        return User.from_orm(model)
```

**Step 3.4: Annotate patterns**
```markdown
## Pattern: Repository Pattern with Dependency Injection

**Source:** [fastapi-ddd-example](https://github.com/example/fastapi-ddd-example)
**Quality Score:** 8/10
**File:** `src/repositories/user_repository.py` (Lines 1-20)

**Pattern Description:**
Abstract base class (`UserRepository`) defines data access interface.
Concrete implementation (`SQLAlchemyUserRepository`) provides SQLAlchemy-specific logic.
Dependency injection via constructor (`__init__(session: AsyncSession)`).

**Benefits:**
- Testability: Mock repository in unit tests
- Flexibility: Swap SQLAlchemy for PostgreSQL raw queries without changing domain logic
- Clean separation: Domain layer doesn't depend on infrastructure

**Drawbacks:**
- Boilerplate overhead: Abstract class + concrete class for each entity
- Complexity: May be overkill for simple CRUD apps

**When to Use:**
- Domain-Driven Design projects
- Complex business logic requiring testability
- Multiple data sources (SQL + NoSQL)
- Long-term maintainability prioritized

**When NOT to Use:**
- Simple CRUD apps with <5 entities
- Prototypes or MVPs (use direct ORM)
- Team unfamiliar with DDD patterns
```

**Actions:**
1. Clone or browse repository online
2. Identify 3-5 key patterns per repository
3. Extract code snippets with file paths and line numbers
4. Annotate patterns with benefits, drawbacks, when to use
5. Document in research report

**Outputs:**
- Code snippets (3-5 per repository)
- Pattern annotations (benefits, drawbacks, applicability)
- Implementation notes (gotchas, edge cases)

---

### Step 4: Architectural Insights

**Goal:** Identify high-level architectural decisions and trade-offs.

**Insight Categories:**

**1. Technology Stack Choices**
- **Example:** "Uses FastAPI + SQLAlchemy async + Alembic migrations"
- **Rationale:** "FastAPI provides async support, SQLAlchemy ORM reduces boilerplate, Alembic handles schema versioning"
- **Trade-off:** "Complexity vs productivity (ORM abstracts SQL but adds magic)"

**2. Project Structure**
- **Example:** "Follows hexagonal architecture (ports & adapters)"
- **Directory layout:**
  ```
  src/
  ├── domain/        # Business logic (pure Python, no dependencies)
  ├── application/   # Use cases (orchestration)
  ├── infrastructure/# External integrations (DB, APIs, queues)
  └── presentation/  # Controllers, routers (FastAPI)
  ```
- **Rationale:** "Isolates business logic from frameworks (testability, flexibility)"
- **Trade-off:** "More directories vs easier testing and technology swaps"

**3. Design Decisions**
- **Example:** "Uses event sourcing for audit trail"
- **Implementation:** "All state changes stored as immutable events"
- **Rationale:** "Complete audit history, supports event replay for debugging"
- **Trade-off:** "Storage overhead vs audit capabilities"

**4. Scalability Patterns**
- **Example:** "Implements CQRS (Command Query Responsibility Segregation)"
- **Architecture:** "Separate read/write models, denormalized read views"
- **Rationale:** "Optimize reads independently from writes, scales to high traffic"
- **Trade-off:** "Complexity and eventual consistency vs read performance"

**5. Testing Strategies**
- **Example:** "100% test coverage with test pyramid (70/20/10)"
- **Breakdown:** "70% unit tests (domain), 20% integration tests (DB), 10% E2E tests (API)"
- **Rationale:** "Fast feedback loop, catches regressions early"
- **Trade-off:** "Test maintenance cost vs confidence in changes"

**Actions:**
1. Review project structure and README
2. Identify architectural patterns (hexagonal, clean architecture, etc.)
3. Document technology choices and rationale
4. Analyze test strategy and coverage
5. Note scalability considerations

**Outputs:**
- Architectural insights summary (bullet points)
- Technology trade-offs (complexity vs benefits)
- Applicability assessment (when this architecture fits)

---

### Step 5: Common Pitfalls & Anti-Patterns

**Goal:** Identify mistakes to avoid based on repository issues, commit history, and community discussions.

**Pitfall Sources:**

**1. GitHub Issues**
- Search: `is:issue label:bug` (bug reports)
- Search: `is:issue label:enhancement closed:>2023-01-01` (feature requests indicating missing functionality)
- Example finding: "OAuth token refresh race condition (#234) - fixed by adding mutex lock"

**2. Pull Request Discussions**
- Search: `is:pr is:merged` (merged PRs with code review comments)
- Example finding: "Refactored repository pattern to avoid N+1 queries (PR #456)"

**3. Commit Messages**
- Search: `git log --grep="fix\|bug\|issue"` (bug fix commits)
- Example finding: "Fix: Async context manager leak in database sessions (commit abc123)"

**4. Documentation FAQs**
- Review `docs/faq.md`, `CONTRIBUTING.md`, `docs/gotchas.md`
- Example finding: "Common mistake: Forgetting to close database connections in async contexts"

**Common Pitfall Categories:**

**1. Performance Issues**
- N+1 queries (lazy loading without preloading)
- Missing database indexes (slow queries at scale)
- Synchronous blocking in async code (defeats async benefits)
- Memory leaks (unclosed connections, circular references)

**2. Security Vulnerabilities**
- Hardcoded secrets (API keys in code)
- SQL injection (string concatenation instead of parameterized queries)
- Missing authentication/authorization checks
- Insecure password hashing (MD5, SHA1 instead of bcrypt/argon2)

**3. Concurrency Problems**
- Race conditions (shared state without locking)
- Deadlocks (circular wait on locks)
- Thread-safety violations (mutable shared state)

**4. Testing Gaps**
- Missing edge case tests (null handling, boundary conditions)
- Flaky tests (timing-dependent, non-deterministic)
- Brittle tests (tight coupling to implementation details)

**Actions:**
1. Review GitHub issues (50 most recent)
2. Analyze merged PRs (20 most impactful)
3. Search commit history for bug fixes
4. Extract common pitfalls and anti-patterns
5. Document with code examples and fixes

**Outputs:**
- Pitfall list (5-10 common mistakes)
- Anti-pattern examples (code snippets showing wrong approach)
- Mitigation strategies (how to avoid each pitfall)

**Example Pitfall Documentation:**
```markdown
## Pitfall: N+1 Queries with Lazy Loading

**Problem:**
```python
# BAD: Lazy loading causes N+1 queries
users = await session.execute(select(User))
for user in users:
    # Triggers separate query for each user's posts
    posts = user.posts  # Lazy load!
```

**Impact:** 1 query for users + N queries for posts = N+1 total queries
**Manifestation:** Slow API responses at scale (>100 users = >100 queries)

**Solution:**
```python
# GOOD: Eager loading with single query
users = await session.execute(
    select(User).options(selectinload(User.posts))
)
for user in users:
    posts = user.posts  # Already loaded, no extra query
```

**Impact:** 2 queries total (1 for users, 1 for all posts)
**Source:** Issue #234 (fastapi-ddd-example)
**Fix:** [PR #456](https://github.com/example/repo/pull/456)
```

---

### Step 6: Framework Compliance Validation

**Goal:** Validate repository patterns against DevForgeAI context files.

**Validation Questions:**
- Do extracted patterns align with architecture-constraints.md?
- Are technologies compatible with tech-stack.md?
- Do patterns follow coding-standards.md conventions?
- Are any anti-patterns.md violations present?

**Validation Workflow:**
1. **Load DevForgeAI context files:**
   - Read tech-stack.md (locked technologies)
   - Read architecture-constraints.md (layer boundaries)
   - Read coding-standards.md (naming conventions, code structure)
   - Read anti-patterns.md (forbidden patterns)

2. **Check extracted patterns:**
   - Repository pattern: Aligns with architecture-constraints.md? (domain → infrastructure)
   - Technology (SQLAlchemy): Approved in tech-stack.md?
   - Naming (PascalCase classes): Follows coding-standards.md?
   - Anti-pattern (God Object): Present in any examples? (flag if found)

3. **Invoke context-validator subagent:**
```python
Task(
  subagent_type="context-validator",
  description="Validate repository patterns",
  prompt="""
  Validate the following extracted patterns against context files:

  Patterns:
  - Repository pattern (abstract base + concrete implementation)
  - Dependency injection via constructor
  - Async SQLAlchemy with session management

  Check:
  - architecture-constraints.md: Domain layer purity (no infrastructure deps)
  - coding-standards.md: Naming conventions (PascalCase, snake_case)
  - anti-patterns.md: No God Objects, no SQL concatenation

  Return: Violation report with severity
  """
)
```

4. **Categorize compliance issues:**
   - CRITICAL: Pattern violates locked architecture-constraints.md
   - HIGH: Pattern conflicts with coding-standards.md
   - MEDIUM: Pattern not explicitly forbidden but not recommended
   - LOW: Minor style deviation

5. **Document compliance in research report:**
```markdown
## Framework Compliance Validation

**Patterns Extracted:** 5
**Context Files Checked:** 6/6 ✅

| Pattern | Status | Notes |
|---------|--------|-------|
| Repository Pattern | ✅ COMPLIANT | Aligns with architecture-constraints.md (domain/infrastructure separation) |
| Async SQLAlchemy | ✅ COMPLIANT | Approved in tech-stack.md |
| Event Sourcing | ⚠️ NOT IN STACK | Not listed in tech-stack.md, requires ADR if adopted |
| God Object (User class) | ❌ VIOLATION | Violates anti-patterns.md (class has 15 methods, limit is 10) |

**Quality Gate Status:** ⚠️ WARN (1 anti-pattern found, 1 unapproved technology)
**Recommendation:** Use repository pattern ✅, avoid event sourcing without ADR, refactor User class
```

**Outputs:**
- Compliance validation table
- Violation summary
- Recommended patterns (compliant) vs patterns to avoid (violations)

---

### Step 7: Pattern Synthesis

**Goal:** Synthesize findings into actionable recommendations with evidence.

**Synthesis Workflow:**

**Step 7.1: Rank patterns by quality + compliance**
```
Quality Score = (Repository Score * 0.4) + (Compliance Score * 0.3) + (Community Validation * 0.3)

Example:
Repository Pattern (fastapi-ddd-example):
  - Repository Score: 8/10 (quality scoring)
  - Compliance Score: 10/10 (fully compliant)
  - Community: 3.2K stars (high validation)
  - **Total:** (8*0.4) + (10*0.3) + (10*0.3) = 3.2 + 3 + 3 = 9.2/10
```

**Step 7.2: Filter by applicability**
- Project type: SaaS / API / Library / CLI
- Scale: Startup / SMB / Enterprise
- Team size: Solo / Small (2-5) / Medium (6-15) / Large (15+)
- Maturity: Prototype / MVP / Production / Legacy

**Example Filtering:**
- Repository Pattern: ✅ Applicable (SaaS, Production, Medium team)
- Event Sourcing: ❌ Not applicable (too complex for MVP, small team)
- CQRS: ⚠️ Conditionally applicable (only if >10K users, read-heavy)

**Step 7.3: Generate top 3 recommendations**
```markdown
## Top 3 Implementation Patterns (Ranked)

### 1. Repository Pattern with Dependency Injection ⭐ (9.2/10)

**Source:** [fastapi-ddd-example](https://github.com/cosmic-python/code) (8/10 quality, 3.2K stars)

**Pattern:**
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

    async def get_by_id(self, user_id: int) -> Optional[User]:
        result = await self.session.execute(
            select(UserModel).where(UserModel.id == user_id)
        )
        return User.from_orm(result.scalar_one_or_none())
```

**Benefits:**
- ✅ Testability (mock repositories in tests)
- ✅ Framework compliance (aligns with architecture-constraints.md)
- ✅ Flexibility (swap data sources without changing business logic)
- ✅ Production-proven (3.2K stars, 500+ production deployments)

**Applicability:**
- ✅ Multi-entity domains (>5 entities)
- ✅ Long-term projects (>6 months)
- ✅ Team size ≥3 developers
- ❌ Simple CRUD apps (<3 entities)

**Implementation Time:** 2-3 days (3 entities)
**Recommendation:** **ADOPT** (high confidence)

### 2. Hexagonal Architecture (Ports & Adapters) (8.5/10)

[... detailed pattern 2 ...]

### 3. Test Pyramid Strategy (70/20/10) (8.1/10)

[... detailed pattern 3 ...]
```

**Step 7.4: Link to ADR preparation**
If patterns require architecture decisions:
```markdown
## ADR Readiness

**ADR Required:** Yes (Repository Pattern adoption)
**ADR Title:** ADR-XXX: Adopt Repository Pattern for Data Access
**Evidence Collected:** ✅
  - Code examples from 3 high-quality repositories (quality scores: 8, 7.5, 9)
  - Benefits/trade-offs documented
  - Applicability assessment complete
  - Anti-pattern avoided (God Object refactored)

**Next Steps:**
1. Create ADR documenting decision rationale
2. Create reference implementation in `/examples/`
3. Update coding-standards.md with repository pattern guidelines
```

**Outputs:**
- Top 3 pattern recommendations (ranked by quality score)
- Applicability matrix (when to use each pattern)
- Implementation estimates (time, complexity)
- ADR readiness checklist

---

### Step 8: Report Generation

**Goal:** Create comprehensive research report with all archaeology findings.

**Report Sections (9 Required):**
1. **Executive Summary** (2-3 sentences summarizing top findings)
2. **Research Scope** (search queries, repository selection criteria)
3. **Methodology Used** ("Repository Archaeology" + GitHub search strategy)
4. **Findings** (quality scores, code patterns, architectural insights)
5. **Framework Compliance Check** (validation against 6 context files)
6. **Workflow State** (current development phase: Architecture/In Development)
7. **Recommendations** (top 3 patterns with code examples)
8. **Risk Assessment** (pitfalls, anti-patterns, mitigation)
9. **ADR Readiness** (evidence for architecture decisions)

**Report Template:**
```markdown
---
research_id: RESEARCH-002
epic_id: EPIC-007
story_id: STORY-042
workflow_state: Ready for Dev
research_mode: repository-archaeology
timestamp: 2025-11-17T16:45:33Z
quality_gate_status: PASS
version: 2.0
---

# Research Report: Authentication Implementation Patterns

## Executive Summary

Analyzed 5 high-quality repositories (avg quality: 7.8/10) implementing OAuth 2.0 authentication in Python. Top recommendation: Repository Pattern with dependency injection (9.2/10, production-proven across 3.2K+ deployments). Key insight: Hexagonal architecture provides best testability vs complexity trade-off for our SaaS context. Critical pitfall avoided: N+1 query anti-pattern (documented in 3 repositories).

## Research Scope

**Search Queries:**
1. `"fastapi oauth2" language:python stars:>100 pushed:>2023-01-01` (12 results)
2. `"repository pattern" "dependency injection" language:python stars:>200` (8 results)
3. `"hexagonal architecture" python stars:>50 pushed:>2024-01-01` (5 results)

**Selection Criteria:**
- Quality score ≥5/10
- Active maintenance (commits within 90 days)
- Test coverage indicators (tests/ directory + CI)

**Repositories Analyzed:** 5
- fastapi-ddd-example (8/10 quality, 3.2K stars)
- fastapi-users/fastapi-users (10/10 quality, 4.5K stars)
- cosmic-python/code (9/10 quality, 2.1K stars)
- hex-architecture-python (7/10 quality, 850 stars)
- minimal-fastapi-auth (5/10 quality, 120 stars)

## Methodology Used

**Research Mode:** Repository Archaeology (GitHub code mining)
**Duration:** 9 minutes 12 seconds
**Tools:** GitHub API, gh CLI, git clone for deep analysis

**Steps:**
1. Formulated 3 search queries
2. Quality-scored 12 candidate repositories
3. Selected top 5 (score ≥5)
4. Extracted 15 code patterns
5. Identified 8 architectural insights
6. Documented 5 common pitfalls
7. Validated against 6 context files
8. Synthesized top 3 recommendations

## Findings

[... detailed code patterns, quality scores, architectural insights ...]

## Framework Compliance Check

[... validation table against 6 context files ...]

## Workflow State

**Current State:** Ready for Dev
**Research Focus:** Implementation patterns and code examples
**Staleness Check:** N/A (newly generated)

## Recommendations

[... top 3 patterns with code examples, ranked by quality score ...]

## Risk Assessment

[... 5 common pitfalls with mitigation strategies ...]

## ADR Readiness

**ADR Required:** Yes (Repository Pattern + Hexagonal Architecture)
**Evidence:** ✅ Code examples, quality scores, trade-off analysis complete

---

**Report Generated:** 2025-11-17 16:50:42
**Report Location:** devforgeai/specs/research/shared/RESEARCH-002-auth-implementation-patterns.md
```

**Outputs:**
- Complete research report (markdown file)
- Code snippets embedded with file paths
- Repository quality scores table
- Report saved to devforgeai/specs/research/shared/

---

## Success Criteria

Repository archaeology succeeds when:
- [ ] Search strategy defined (3-5 precise GitHub queries)
- [ ] Repository quality scored (5-10 repos, score ≥5 selected)
- [ ] Code patterns extracted (3-5 patterns per repo, annotated)
- [ ] Architectural insights documented (5+ high-level decisions)
- [ ] Pitfalls identified (5-10 common mistakes with fixes)
- [ ] Framework compliance validated (patterns checked against 6 context files)
- [ ] Pattern synthesis complete (top 3 ranked recommendations)
- [ ] Report generated (9 sections, YAML frontmatter, code examples)
- [ ] Duration <10 minutes (p95 threshold)
- [ ] Token usage <50K (within budget)

---

## Related Documentation

- `research-principles.md` - Core research methodology (always loaded)
- `discovery-mode-methodology.md` - Feasibility research (broader scope)
- `skill-coordination-patterns.md` - Task invocation examples
- `research-report-template.md` - Standard report structure
- `.claude/skills/devforgeai-architecture/SKILL.md` - Architecture integration

---

**Created:** 2025-11-17
**Version:** 1.0
**Lines:** 605 (target: 600 ✓)
**Purpose:** GitHub code mining and implementation pattern extraction
