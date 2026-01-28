# Epic Creation Guide

Educational reference for understanding epic creation in DevForgeAI framework.

---

## Pattern Precedent

The `/create-epic` command follows the lean orchestration pattern proven across multiple command refactorings:

**Successful refactorings:**
- /qa: 692 → 295 lines (57% reduction, 74% character reduction)
- /dev: 860 → 513 lines (40% reduction, 58% character reduction)
- /ideate: 463 → 410 lines (11% reduction, 24% character reduction)
- /create-story: 857 → 500 lines (42% reduction, 38% character reduction)
- **/create-epic: 526 → ~250 lines (52% reduction, 44% character reduction)**

**Average:** 40% line reduction, 50% character reduction, 60-80% token savings

---

## Lean Orchestration Principle

> "Commands orchestrate. Skills validate. Subagents specialize."

**Architecture layers:**
```
User Input
  ↓
/create-epic Command (Lean Orchestration)
  ↓
devforgeai-orchestration Skill (Business Logic)
  ↓
Subagents (Specialized Tasks)
  ├─ requirements-analyst (feature decomposition)
  └─ architect-reviewer (technical assessment)
```

**Separation of concerns:**
- **Command:** Argument validation, context markers, skill invocation, result display
- **Skill:** Epic discovery, context gathering, feature decomposition, technical assessment, file creation, validation
- **Subagents:** Domain expertise (requirements analysis, architecture review)

---

## Epic Lifecycle

### Epic States

**Planning → In Progress → Paused → Completed → Archived**

**Status transitions:**
- **Created:** Status = "Planning"
- **First story started:** Status = "In Progress"
- **Work paused:** Status = "Paused" (external blocker, reprioritization)
- **All stories complete:** Status = "Completed"
- **No longer relevant:** Status = "Archived"

**Status managed by:** devforgeai-orchestration skill during story lifecycle events

---

### Epic vs Feature vs Story

**Epic** (Business initiative - multiple sprints)
- Represents major business capability or goal
- Contains 3-8 features
- Spans multiple sprints (1-10+ sprints typical)
- Has measurable business outcomes
- Example: "User Authentication System", "Payment Processing Overhaul"

**Feature** (Functional unit - 1-2 sprints)
- Represents significant user-facing capability
- Can be implemented independently (or minimal dependencies)
- Delivers incremental value
- Testable and demonstrable
- Example: "User Registration with Email Verification", "Password Reset Flow"

**Story** (Atomic work unit - 1-5 days)
- Smallest deliverable increment
- Completed within single sprint
- Has acceptance criteria (Given/When/Then)
- Fully testable in isolation
- Example: "As a user, I want to register with email, so I can create an account"

**Hierarchy:**
```
EPIC-001: User Authentication System
├─ Feature 1: User Registration with Email Verification
│  ├─ STORY-001: Registration form with validation
│  ├─ STORY-002: Email verification workflow
│  └─ STORY-003: Registration confirmation page
├─ Feature 2: Password Reset Flow
│  ├─ STORY-004: Forgot password request
│  ├─ STORY-005: Password reset email
│  └─ STORY-006: New password submission
└─ Feature 3: Multi-Factor Authentication
   ├─ STORY-007: MFA setup during registration
   ├─ STORY-008: MFA challenge on login
   └─ STORY-009: MFA backup codes
```

---

## When to Create Epics

### ✅ Create Epic When:

**Starting new major features:**
- Multiple related capabilities needed
- Work spans 2+ sprints
- Clear business goal with measurable outcomes
- Example: Building complete authentication system

**Planning large initiatives:**
- Significant architecture changes
- New technology introduction
- Multiple team members involved
- Example: Migrating from monolith to microservices

**Organizing related work:**
- Multiple features share common goal
- Work has logical grouping
- Stakeholder coordination needed
- Example: Customer self-service portal (registration, profile, orders, support)

**Before sprint planning:**
- Have requirements but need feature breakdown
- Want to estimate total effort
- Need to communicate scope to stakeholders
- Example: After /ideate requirements discovery

**After requirements discovery:**
- devforgeai-ideation skill completed
- Requirements documented
- Ready to plan implementation
- Example: Ideation generated requirements spec → Create epic

---

### ❌ Do NOT Create Epic When:

**For single user stories:**
- Work fits in one story (<5 days)
- No related features needed
- **Use instead:** /create-story directly
- Example: Add validation to existing form

**For bug fixes:**
- Fixing existing functionality
- No new features added
- **Use instead:** /create-story with type "Bug Fix"
- Example: Fix login error on mobile

**For minor improvements:**
- Small enhancements to existing features
- No significant architecture impact
- **Use instead:** /create-story with type "Enhancement"
- Example: Add sorting to table

**During active sprint:**
- Mid-sprint work planning
- Disrupts current sprint
- **Wait for:** Sprint retrospective or planning session
- Example: New urgent requirement during sprint week 1

---

## Epic Best Practices

### Keep Epics Focused

**Single business goal:**
- Epic has one clear objective
- Success criteria align with goal
- Features all contribute to same goal
- Avoid: Mixing unrelated features in one epic

**Example - Good Focus:**
```
EPIC: User Authentication System
Goal: Enable secure user access to platform
Features: Registration, Login, Password Reset, MFA, Session Management
✅ All features support authentication goal
```

**Example - Poor Focus:**
```
EPIC: User Features
Goal: Improve user experience (vague!)
Features: Registration, Profile, Search, Notifications, Settings
❌ Features support different goals (auth, personalization, discovery, communication)
```

---

### Limit to 3-8 Features

**Optimal range:** 3-8 features per epic

**Why 3 minimum:**
- Less than 3: Too small, just create stories directly
- Single feature: Not an epic, it's already a feature
- 2 features: Consider if truly an epic or just related stories

**Why 8 maximum:**
- More than 8: Epic too large, hard to manage
- 10+ features: Risk of scope creep, timeline uncertainty
- Over-scoped: Split into multiple epics

**Feature count remediation:**
- **Under-scoped (<3 features):** Combine with another epic or expand scope
- **Over-scoped (>8 features):** Split into multiple epics or defer some features

**Skill handles:** Validation Phase 7 detects and flags scoping issues

---

### Define Measurable Success Criteria

**SMART criteria required:**
- **Specific:** Clear what to achieve
- **Measurable:** Quantifiable outcome
- **Achievable:** Realistic given constraints
- **Relevant:** Aligns with business goals
- **Time-bound:** When to achieve

**Examples - Good Success Criteria:**
```
✅ "Reduce login time from 3 seconds to <1 second"
   - Specific: Login time
   - Measurable: <1 second
   - Achievable: Technical feasibility validated
   - Relevant: User experience improvement
   - Time-bound: By epic completion

✅ "Support 10,000 concurrent users with <100ms latency"
   - Specific: Concurrent users + latency
   - Measurable: 10K users, <100ms
   - Achievable: Architecture can scale
   - Relevant: Growth target
   - Time-bound: By production release

✅ "Achieve 99.9% uptime for authentication service"
   - Specific: Uptime for auth service
   - Measurable: 99.9%
   - Achievable: Redundancy and monitoring in place
   - Relevant: Reliability requirement
   - Time-bound: Ongoing after release
```

**Examples - Poor Success Criteria:**
```
❌ "Improve user experience"
   - Not measurable (how much improvement?)
   - Not specific (which aspect of UX?)

❌ "Fast performance"
   - Not measurable (how fast is "fast"?)
   - Not specific (performance of what?)

❌ "Users like the feature"
   - Not measurable (how many users? what metric?)
   - Subjective (no clear pass/fail)
```

**Skill requires:** 3+ success criteria (validates in Phase 2, flags in Phase 7 if <3)

---

### Identify Stakeholders Early

**Why stakeholders matter:**
- Clarifies ownership (who decides, who approves)
- Ensures communication (who needs updates)
- Defines responsibilities (who does what)
- Prevents gaps (all roles covered)

**Minimum stakeholders (default):**
- Product Owner: Requirements, prioritization
- Tech Lead: Architecture, implementation
- QA Lead: Testing strategy

**Additional stakeholders (common):**
- UX Designer: User experience, interface design
- Security Lead: Security requirements, compliance
- DevOps Lead: Infrastructure, deployment
- Business Analyst: Requirements refinement
- End Users/Customers: Validation, feedback

**Skill handles:** Validation Phase 7 ensures ≥3 stakeholders (self-heals if missing)

---

### Document Technical Risks Upfront

**Why risk identification matters:**
- Prevents surprises during implementation
- Enables proactive mitigation
- Informs timeline estimates
- Supports resource planning

**Risk categories:**
- **Technology risks:** Unproven tech, learning curve, vendor lock-in
- **Integration risks:** Third-party API instability, data format changes
- **Data risks:** Data loss during migration, privacy violations
- **Team risks:** Skill gaps, knowledge silos, capacity constraints

**Each risk requires:**
- Clear description (what could go wrong)
- Mitigation strategy (how to prevent or minimize)
- Owner (who monitors and responds)

**Skill requires:** 2+ risks with mitigations (validates in Phase 4, self-heals in Phase 7 if <2)

---

## Framework Integration

### Context Files

**Greenfield mode (no context files):**
- Epic created without constraint validation
- Skill notes: "Operating in greenfield mode"
- Recommendation: Create context files before implementation
- Next step: /create-context {project-name}

**Brownfield mode (context files exist):**
- Skill validates against all 6 context files
- Technologies checked against tech-stack.md
- Architecture checked against architecture-constraints.md
- Integrations checked against dependencies.md
- Patterns checked against anti-patterns.md
- HALTS on violations (cannot proceed)

**Context file validation happens in:** Skill Phase 4 (Technical Assessment)

---

### Epic → Sprint → Story Workflow

**Complete workflow:**

1. **Create Epic** ← You are here
   ```
   /create-epic User Authentication System
   ```

2. **Create Context Files** (if greenfield)
   ```
   /create-context my-project
   ```

3. **Create Sprint**
   ```
   /create-sprint Sprint-1
   # During planning: Break epic features into stories
   ```

4. **Create Stories** (during sprint planning)
   ```
   /create-story User registration form with validation
   /create-story Email verification workflow
   /create-story Password reset request
   ```

5. **Implement Stories**
   ```
   /dev STORY-001
   /dev STORY-002
   /dev STORY-003
   ```

6. **Track Epic Progress**
   - Orchestration skill updates epic as stories complete
   - Epic status: Planning → In Progress → Completed

---

## Reference Files

For detailed epic creation procedures, see:

**Skill documentation:**
- `.claude/skills/devforgeai-orchestration/SKILL.md` (Phase 4A: Epic Creation Workflow)

**Reference files (loaded by skill):**
- `epic-management.md` - Epic planning, ID generation, duplicate handling
- `feature-decomposition-patterns.md` - Epic → feature breakdown by type (CRUD, Auth, API, etc.)
- `technical-assessment-guide.md` - Complexity scoring rubric, risk assessment
- `epic-validation-checklist.md` - Validation procedures, self-healing logic

**Framework protocols:**
- `devforgeai/protocols/lean-orchestration-pattern.md` - Command architecture pattern
- `devforgeai/protocols/CREATE-EPIC-REFACTORING-IMPLEMENTATION-PLAN.md` - Refactoring details

**DevForgeAI guides:**
- `.claude/memory/skills-reference.md` - Skills overview
- `.claude/memory/commands-reference.md` - Commands overview
- `CLAUDE.md` - Framework overview

---

## Common Questions

### Q: Can I create an epic before context files?

**A:** Yes, epics can be created in greenfield mode (before /create-context). The skill will:
- Skip context file validation
- Note: "Operating in greenfield mode"
- Recommend creating context files before implementation
- Proceed with epic creation without constraint enforcement

### Q: What if my epic has 10+ features?

**A:** The skill will flag this as over-scoped during validation (Phase 7):
- Warning: "Over-scoped: {count} features (recommend 3-8)"
- Epic status updated: "Planning (Over-Scoped)"
- Epic created with warning note
- Recommendation: Consider splitting into multiple epics

### Q: What if features have circular dependencies?

**A:** The skill will detect this during validation (Phase 7):
- Critical failure: "Circular dependencies detected: {chains}"
- Epic creation BLOCKED
- Display dependency graph
- Require manual resolution (redesign features to break cycle)

### Q: What if proposed technology conflicts with tech-stack.md?

**A:** The skill handles this during technical assessment (Phase 4):
- architect-reviewer subagent detects conflict
- Flags: "REQUIRES ADR" in technical assessment
- Skill presents AskUserQuestion with 3 options:
  1. Update tech-stack.md (requires ADR creation)
  2. Adjust epic scope (use existing approved technology)
  3. Mark as technical debt (defer decision)
- Epic created with ADR requirement noted in summary

### Q: Can I edit an existing epic?

**A:** Yes, the skill supports update mode:
- During Phase 1 (Epic Discovery), if duplicate name found
- AskUserQuestion option: "Edit existing epic"
- Skill reads existing epic, merges changes
- Preserves existing data, updates modified fields

### Q: Do I need requirements specification?

**A:** Optional - the skill asks during Phase 6:
- AskUserQuestion: "Create detailed requirements spec?"
- Options: Yes (comprehensive), No (epic sufficient), Later (during sprint planning)
- If yes: requirements-analyst generates full requirements doc
- If no: Epic document serves as requirements

---

## Troubleshooting

### Issue: "Cannot determine orchestration mode"

**Cause:** Context markers not set correctly

**Solution:** Ensure markers present:
```
**Epic name:** {your epic name}
**Command:** create-epic
```

**Why this happens:** Skill operates in multiple modes (epic, sprint, story). Without markers, skill cannot determine which workflow to execute.

---

### Issue: "Epic validation failed - circular dependencies"

**Cause:** Features depend on each other in circular pattern (A → B → C → A)

**Solution:**
- Redesign features to break dependency cycle
- Option 1: Merge dependent features into one larger feature
- Option 2: Introduce intermediary feature that both depend on
- Option 3: Remove dependencies (make features more independent)

**Prevention:** During feature review (Phase 3), examine dependencies carefully

---

### Issue: "Architecture constraint violation detected"

**Cause:** Proposed epic design violates architecture-constraints.md rules

**Solution:**
- Review architecture-constraints.md
- Understand violated constraint
- Redesign features to respect constraints
- OR: If constraint is outdated, create ADR to update it

**Example:**
```
Violation: "Domain layer cannot depend on Infrastructure"
Proposed: Feature requires direct database access in business logic
Solution: Use repository pattern (Infrastructure → Domain via interface)
```

---

### Issue: "Technical complexity score >10"

**Cause:** Epic is extremely complex (new tech stack, major redesign, distributed systems)

**Solution:**
- Skill HALTs on complexity >10
- Split epic into multiple smaller epics
- Each epic should have complexity ≤8 (optimal)
- Plan epic sequence (Epic 1 → Epic 2 → Epic 3)

**Example:**
```
Original: "Migrate Monolith to Microservices" (complexity 12)
Split into:
  - Epic 1: "Service Extraction - User Service" (complexity 6)
  - Epic 2: "Service Extraction - Order Service" (complexity 6)
  - Epic 3: "Event-Driven Integration Layer" (complexity 7)
```

---

## Advanced Topics

### Epic Sequencing

**Dependent epics:**
- Epic 2 requires Epic 1 completion
- Document in Epic 2's "Dependencies" section
- Plan sequentially in roadmap

**Parallel epics:**
- Independent epics
- Can execute simultaneously
- Coordinate via sprint planning

**Epic dependencies managed by:** Product Owner during epic prioritization

---

### Epic Metrics

**Tracked automatically:**
- Feature count
- Complexity score (0-10)
- Risk count
- Prerequisite count
- Timeline estimate (sprints)
- Story count (as stories created)
- Story completion percentage
- Actual vs estimated timeline

**Epic metrics location:** Epic file Status History section

---

### Epic Retrospectives

**After epic completion:**
- Review actual vs estimated timeline
- Analyze risks (which occurred, which didn't)
- Document lessons learned
- Update epic Status History with retrospective notes

**Feeds into:** Future epic planning (better estimates, risk identification)

---

## Related Commands

**Epic creation workflow:**
```
/ideate → /create-epic → /create-context → /create-sprint → /create-story → /dev
```

**Command sequence:**
1. `/ideate` - Requirements discovery, epic decomposition
2. `/create-epic` - Epic document creation ← You are here
3. `/create-context` - Architectural constraints (if greenfield)
4. `/create-sprint` - Sprint planning with epic features
5. `/create-story` - Break features into stories
6. `/dev` - Implement stories

**Each command:** Follows lean orchestration pattern, delegates to skills

---

## Token Efficiency

**Why lean orchestration matters:**

**Before refactoring (/create-epic with business logic):**
- Command: 526 lines, 14,309 chars
- Main conversation: ~10,000 tokens
- All logic in command (no isolation)
- Result: 10K tokens consumed in main conversation

**After refactoring (lean orchestration):**
- Command: ~250 lines, ~8,000 chars
- Main conversation: ~2,000 tokens
- Logic in skill: ~125,000 tokens (isolated context)
- Result: 2K tokens in main, 125K isolated (98% efficiency)

**Main conversation budget:** 1,000,000 tokens
**Efficiency gain:** 8,000 tokens freed per epic creation (4x more epics possible in single session)

---

## See Also

**Lean orchestration pattern:**
- `devforgeai/protocols/lean-orchestration-pattern.md` - Pattern definition and methodology

**Other refactored commands:**
- `/qa` - Quality assurance (reference implementation - 48% budget)
- `/dev` - Development workflow (reference implementation - 84% budget)
- `/ideate` - Requirements discovery (78% budget)
- `/create-story` - Story creation (95% budget - still needs optimization)

**Framework documentation:**
- `CLAUDE.md` - Complete framework overview
- `.claude/memory/skills-reference.md` - Skills usage guide
- `.claude/memory/commands-reference.md` - Commands reference

---

## Implementation Architecture Details

### Lean Orchestration Pattern Compliance

**Command responsibilities (ONLY):**
- ✅ Parse arguments (epic name validation)
- ✅ Load context (N/A - no file to load for epic creation)
- ✅ Set markers (`**Epic name:**`, `**Command:** create-epic`)
- ✅ Invoke skill (`Skill(command="devforgeai-orchestration")`)
- ✅ Display results (skill summary output)

**Command does NOT:**
- ❌ Business logic (removed - now in skill)
- ❌ Subagent invocation (removed - delegated to skill)
- ❌ Template generation (removed - skill handles)
- ❌ Complex decision-making (removed - skill handles)
- ❌ Error recovery (removed - skill handles)

**Pattern compliance:** ✅ 5/5 responsibilities met, 0/5 violations

---

### Comparison to Reference Implementation (/qa)

**Similar structure:**
- Phase 0: Argument validation (create-epic: epic name, /qa: story ID + mode)
- Phase 1: Context markers (both use explicit `**Parameter:**` format)
- Phase 2: Single skill invocation (both delegate to devforgeai-* skill)
- Phase 3: Display results (both show skill output directly)
- Phase 4: Next steps (both provide context-aware guidance)

**Differences:**
- /qa loads story file via @file (create-epic doesn't - no pre-existing file)
- /qa has mode parameter (deep/light), create-epic doesn't (single mode)
- Both are ~250 lines, ~8K characters (similar size)

---

### Skills-First Architecture Restored

**Before refactoring:**
```
User → /create-epic command → subagents (NO SKILL LAYER!)
```

**After refactoring:**
```
User → /create-epic command → devforgeai-orchestration skill → subagents
```

**Benefits:**
- ✅ Proper layer separation (command → skill → subagents)
- ✅ Business logic in skill (isolated context)
- ✅ Single source of truth (skill contains workflow)
- ✅ Token efficiency (80% reduction in main conversation)
- ✅ Framework compliance (lean orchestration pattern)

---

## Framework Context Integration

**Context file handling:**

The skill (not command) handles context file validation:
- Detects greenfield mode (no devforgeai/specs/context/*.md)
- Detects brownfield mode (6 context files exist)
- Validates technologies against tech-stack.md (brownfield only)
- Validates architecture against architecture-constraints.md (brownfield only)
- HALTS on framework violations
- Recommends context creation in greenfield mode

**Command responsibility:** None (skill handles all context logic)

---

## Progressive Disclosure

**Reference files loaded by skill (not command):**

Total: 3,311 lines of reference content available

Loaded progressively during skill execution:
1. epic-management.md (496 lines - Phases 1-2)
2. feature-decomposition-patterns.md (850 lines - Phase 3)
3. technical-assessment-guide.md (900 lines - Phase 4)
4. epic-template.md (265 lines - Phase 5)
5. epic-validation-checklist.md (800 lines - Phase 7)

**Main conversation impact:** Near zero (all in skill's isolated context)

**Token efficiency:**
- Skill execution: ~125,000-146,000 tokens (isolated)
- Main conversation: ~2,000 tokens (command overhead)
- Efficiency: 98% of work in isolated context

---

## Quality Gates

The skill enforces 6 quality gates during epic creation:

1. **Duplicate Prevention** - Grep search for existing epic names, user decision if found
2. **Context File Validation** - If brownfield, validates against all 6 context files (tech-stack, architecture-constraints, dependencies, anti-patterns, coding-standards, source-tree)
3. **Feature Scoping** - 3-8 features optimal, warns if under/over-scoped
4. **Success Criteria** - 3+ measurable outcomes required (SMART criteria)
5. **Technical Feasibility** - Complexity ≤8 optimal, warns if >8, HALTS if >10
6. **Self-Validation** - 9 validation checks, self-heals correctable issues (missing IDs, dates, defaults), HALTS on critical failures (circular dependencies, framework violations)

**Command responsibility:** None (skill enforces all gates)

**Gate enforcement location:** Skill Phase 7 (Epic Validation and Self-Healing)

---

**This guide provides educational context for epic creation. For execution instructions, see the /create-epic command and devforgeai-orchestration skill.**
