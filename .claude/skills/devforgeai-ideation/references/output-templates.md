# Ideation Output Templates

This reference provides standardized templates for presenting ideation results to users. Load this file when preparing completion summaries in Phase 6 of the ideation workflow.

---

## Completion Summary Template

Use this template when reporting ideation completion to users:

```markdown
## ✅ Ideation Complete

### Generated Artifacts

**Epic Documents:**
- EPIC-001: {Epic Title} ({Feature Count} features, {Story Points} points)
- EPIC-002: {Epic Title} ({Feature Count} features, {Story Points} points)
[... list all epics ...]
- Location: `devforgeai/specs/Epics/`

**Requirements Specification:**
- Location: `devforgeai/specs/requirements/{project-name}-requirements.md`
- Functional Requirements: {count}
- Non-Functional Requirements: {count}
- Data Models: {count} entities
- Integration Points: {count} external systems

### Complexity Assessment

**Total Score:** {score}/60
**Architecture Tier:** {Simple|Standard|Advanced|Enterprise}

**Score Breakdown:**
- Functional Complexity: {score}/20
- Technical Complexity: {score}/20
- Team/Organizational Complexity: {score}/10
- Non-Functional Complexity: {score}/10

### Recommended Technology Stack

Based on {tier} architecture tier:

**Backend:** {recommendations}
**Frontend:** {recommendations}
**Database:** {recommendations}
**Infrastructure:** {recommendations}

### Next Steps

1. **Review** generated epics and requirements specification
2. **Validate** complexity assessment and architecture tier
3. **Proceed** to architecture phase (`/create-context`) to define:
   - Technology stack decisions (tech-stack.md)
   - Project structure (source-tree.md)
   - Approved dependencies (dependencies.md)
   - Coding standards (coding-standards.md)
   - Architecture constraints (architecture-constraints.md)
   - Anti-patterns to avoid (anti-patterns.md)
```

---

## Technology Recommendations by Tier

Technology recommendations vary by complexity tier. The authoritative source for detailed recommendations is the complexity assessment matrix.

**Brief Summary:**
- **Tier 1 (Simple):** Lightweight frameworks (Express, FastAPI), single database, serverless/VPS hosting
- **Tier 2 (Moderate):** Full-featured frameworks (NestJS, Django), read replicas, cloud platforms
- **Tier 3 (Complex):** Microservices, polyglot persistence, Kubernetes, service mesh
- **Tier 4 (Enterprise):** Distributed systems, event-driven architecture, multi-region, zero-trust security

For full details, see: [complexity-assessment-matrix.md](complexity-assessment-matrix.md) (Technology Recommendations by Tier)

---

## Next Steps Template

Use this template when transitioning users to the architecture phase:

```markdown
## 🎯 Ready for Architecture Phase

Ideation is complete with structured requirements and complexity assessment. The next step is defining architectural constraints.

### What /create-context Does

The architecture skill will generate **6 context files** that serve as immutable constraints for all development:

1. **tech-stack.md** - Locked technology choices (prevents library substitution)
2. **source-tree.md** - Project structure rules (prevents chaos)
3. **dependencies.md** - Approved packages (prevents bloat)
4. **coding-standards.md** - Code patterns (enforces consistency)
5. **architecture-constraints.md** - Layer boundaries (prevents violations)
6. **anti-patterns.md** - Forbidden patterns (prevents technical debt)

### Input from Ideation

The architecture skill will reference:
- ✅ Requirements specification (functional/non-functional requirements)
- ✅ Complexity assessment (architecture tier: {tier})
- ✅ Technology recommendations (based on {score}/60 complexity)
- ✅ Epic documents (feature scope and priorities)

### Expected Interaction

The architecture skill will:
1. **Validate** requirements and complexity tier
2. **Ask** technology preference questions (via AskUserQuestion)
3. **Recommend** technologies based on complexity tier
4. **Resolve** any conflicts between user preferences and requirements
5. **Generate** all 6 context files
6. **Create** initial ADR (Architecture Decision Record)

### Command to Run

```bash
/create-context {project-name}
```

Example:
```bash
/create-context task-management-saas
```

This will create context files in `devforgeai/specs/context/` and transition you to orchestration phase (sprint planning).
```

---

## Brownfield Project Template

Use this template when dealing with existing codebases:

```markdown
## 🏗️ Brownfield Project Detected

Existing codebase found. Ideation complete with requirements for new features/modernization.

### Existing Context Files

**Status:** Context files found in `devforgeai/specs/context/`

**Files:**
- ✅ tech-stack.md (existing technology choices)
- ✅ source-tree.md (current project structure)
- ✅ dependencies.md (approved packages)
- ✅ coding-standards.md (existing standards)
- ✅ architecture-constraints.md (current constraints)
- ✅ anti-patterns.md (known anti-patterns)

### Requirements Validation

The ideation skill has validated new requirements against existing constraints:

**Conflicts Detected:** {count}
- {List any technology conflicts}
- {List any architecture constraint violations}
- {List any anti-pattern concerns}

**Resolution Required:**
- Use AskUserQuestion to resolve each conflict
- Update context files if strategic change needed
- Create ADR documenting any context file changes

### Next Steps

**Option 1: Proceed with Orchestration**
If no conflicts or all resolved:
```bash
/create-sprint {sprint-number}
```

**Option 2: Update Context Files**
If strategic technology changes needed:
```bash
/create-context {project-name}
```
Note: This will update existing context files and create ADRs for changes.

**Option 3: Manual Review**
Review requirements and context files manually:
- Requirements: `devforgeai/specs/requirements/`
- Context: `devforgeai/specs/context/`
- Epics: `devforgeai/specs/Epics/`
```

---

## Epic Summary Template

Use this template when summarizing individual epics:

```markdown
### Epic: {EPIC-ID} - {Epic Title}

**Business Value:** {1-sentence value proposition}

**Features:** {count} features
- {Feature 1 name} (Priority: {High|Medium|Low})
- {Feature 2 name} (Priority: {High|Medium|Low})
[... list all features ...]

**Estimated Complexity:** {story points} points

**Success Metrics:**
- {Metric 1}: {Target}
- {Metric 2}: {Target}
[... list all metrics ...]

**Dependencies:**
- {Dependency 1}
- {Dependency 2}

**Risks:**
- {Risk 1} (Likelihood: {Low|Medium|High}, Impact: {Low|Medium|High})
- {Risk 2} (Likelihood: {Low|Medium|High}, Impact: {Low|Medium|High})

**Timeline:** {estimated weeks}
```

---

## Complexity Assessment Explanation Template

Use this template when explaining complexity scores to users:

```markdown
### Understanding Your Complexity Score

**Total Score:** {score}/60
**Tier:** {Simple|Standard|Advanced|Enterprise}

#### Score Breakdown

**Functional Complexity: {score}/20**
- User Roles: {count} roles ({Low|Medium|High} = {points} points)
- Core Entities: {count} entities ({Low|Medium|High} = {points} points)
- Integrations: {count} integrations ({Low|Medium|High} = {points} points)
- Workflow Complexity: {Linear|Branching|State Machines} = {points} points

**Technical Complexity: {score}/20**
- Data Volume: {volume} ({Low|Medium|High} = {points} points)
- Concurrency: {users} concurrent users ({Low|Medium|High} = {points} points)
- Real-time Requirements: {None|Polling|WebSockets} = {points} points

**Team/Organizational Complexity: {score}/10**
- Team Size: {count} developers ({Low|Medium|High} = {points} points)
- Team Distribution: {Co-located|Remote|Multi-timezone} = {points} points

**Non-Functional Complexity: {score}/10**
- Performance Requirements: {Moderate|Standard|High} = {points} points
- Compliance Requirements: {None|Standard|Strict} = {points} points

#### What This Means

**{Tier} Architecture** is recommended for complexity level {score}/60.

**Typical characteristics of {tier} applications:**
{Insert tier-specific characteristics from tier definitions above}

**Technology recommendations:**
{Insert tier-specific tech stack recommendations}

**Development timeline estimate:**
MVP: {weeks} weeks
Full feature set: {weeks} weeks

**Team size recommendation:**
{count} developers for optimal velocity
```

---

## Reference Usage Instructions

**When to Load This File:**
- Phase 6: Requirements Documentation & Handoff
- Specifically Phase 6.3: Transition to Architecture Skill
- When preparing completion summary for user

**How to Use Templates:**
1. Load this reference file: `Read(file_path=".claude/skills/devforgeai-ideation/references/output-templates.md")`
2. Select appropriate template based on context (greenfield vs brownfield, tier, etc.)
3. Fill in placeholders with actual values from ideation process
4. Present formatted output to user

**Progressive Disclosure:**
- Load only when needed (Phase 6)
- Templates provide structure without consuming tokens during Phases 1-5
- Reference file can be updated without changing skill logic
