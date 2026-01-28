# Phase 6: Artifact Generation (Steps 6.1-6.3)

Generate epic documents and optional requirements specifications from discovered requirements.

## Overview

Phase 6 transforms all discovered requirements into structured documents that feed downstream DevForgeAI skills. This phase creates the tangible outputs from ideation: epic documents, requirements specifications, and handoff materials.

**Duration:** 10-20 minutes
**Output:** 1-3 epic documents, optional requirements spec, transition to architecture

---

## Load Constitutional Epic Template

**CRITICAL: Before generating any epic, load the canonical template:**

```
Read(file_path=".claude/skills/devforgeai-orchestration/assets/templates/epic-template.md")
```

This ensures all generated epics contain the complete constitutional structure.

**IMPORTANT - Source of Truth Warning:**

> **DO NOT** create epics manually or use any inline template. Always load the constitutional template via the Read() instruction above. The canonical template contains all 12 required sections; abbreviated versions are incomplete and non-compliant.

### Section Compliance Checklist

Every generated epic MUST contain all 12 constitutional sections. Verify against this checklist:

| Section | Required | Purpose |
|---------|----------|---------|
| YAML Frontmatter | ✓ | Epic metadata (id, title, status, dates, points, owner, team) |
| Business Goal | ✓ | Problem statement and value proposition |
| Success Metrics | ✓ | Measurable outcomes with targets and measurement plan |
| Scope | ✓ | In-scope features and explicit out-of-scope exclusions |
| Target Sprints | ✓ | Sprint breakdown with goals, points, and deliverables |
| User Stories | ✓ | High-level stories to decompose into detailed stories |
| Technical Considerations | ✓ | Architecture, technology decisions, security, performance |
| Dependencies | ✓ | Internal and external dependencies with status tracking |
| Risks & Mitigation | ✓ | Risk register with probability, impact, and mitigation |
| Stakeholders | ✓ | Primary stakeholders and communication plan |
| Timeline | ✓ | Key milestones and epic timeline visualization |
| Progress Tracking | ✓ | Sprint summary table and burndown metrics |

**Validation:** After generating an epic, verify all 12 sections are present. Missing sections = non-compliant epic.

### Cross-Session Context Requirements

When another Claude session resumes epic generation, it needs:

- **Brainstorm document:** The source `BRAINSTORM-NNN.brainstorm.md` with problem statement and discovery data
- **Complexity score:** Phase 3 assessment results (score out of 60, architecture tier 1-4)
- **Epic decomposition:** Phase 4 feature breakdown and epic boundaries
- **Feasibility assessment:** Phase 5 risk analysis and go/no-go recommendation
- **Project context files:** If brownfield, existing `devforgeai/specs/context/*.md` constraints

**Recovery pattern:** Load brainstorm → Read complexity assessment → Load epic template → Generate compliant epic

---

## Step 6.1: Generate Epic Document(s)

### Epic Document Structure

Create epic documents in `devforgeai/specs/Epics/EPIC-NNN-[name].epic.md` following the DevForgeAI epic template.

**CRITICAL: Track epic creation with TodoWrite**

```
At start of epic generation, create todos for each epic:

TodoWrite([
  {"content": "Create EPIC-001: {name}", "status": "pending", "activeForm": "Creating EPIC-001"},
  {"content": "Create EPIC-002: {name}", "status": "pending", "activeForm": "Creating EPIC-002"},
  {"content": "Create EPIC-003: {name}", "status": "pending", "activeForm": "Creating EPIC-003"}
])

Mark each epic as in_progress before creating, completed after file written.
```

### Verify Epic Creation

**CRITICAL verification gate:**

```
# Count planned epics (from Phase 4 decomposition)
planned_epics = {count from Phase 4}

# Count created epic files
created_epic_files = Glob(pattern="devforgeai/specs/Epics/EPIC-*.epic.md")
created_count = len(created_epic_files)

# Verification gate
if created_count < planned_epics:
    # HALT - Incomplete work detected
    missing_count = planned_epics - created_count

    ERROR: Only {created_count}/{planned_epics} epics created

    Missing epics: Review Phase 4 decomposition and create remaining epic documents

    DO NOT PROCEED to Step 6.2 until all epics are created and verified.

else:
    # All epics created, safe to proceed
    ✓ All {planned_epics} epics created and verified
    → Proceed to Step 6.2
```

**Why this gate is critical:**
- Prevents incomplete artifact generation
- Ensures all planned work documented
- Catches TodoWrite tracking errors
- Verifies file system writes succeeded

---

## Step 6.2: Generate Requirements Specification (Optional)

### Determine if Requirements Spec Needed

```
AskUserQuestion(
    question: "Create detailed requirements specification document?",
    header: "Requirements spec",
    options: [
        {
            label: "Yes - comprehensive documentation",
            description: "Generate full requirements spec with all details (~500-2000 lines)"
        },
        {
            label: "No - epic documents sufficient",
            description: "Epic documents contain enough detail for architecture phase"
        },
        {
            label: "Later - during sprint planning",
            description: "Generate requirements spec when creating Sprint 1"
        }
    ]
)
```

**If "Yes":**

### Requirements Spec Structure

Create in `devforgeai/specs/requirements/{project-name}-requirements.md`:

```markdown
# {Project Name} - Requirements Specification

**Version:** 1.0
**Date:** {YYYY-MM-DD}
**Status:** Draft
**Author:** DevForgeAI Ideation
**Complexity Score:** {X}/60 (Tier {N})

---

## 1. Project Overview

### 1.1 Project Context
**Type:** {Greenfield|Brownfield|Modernization|Problem-solving}
**Domain:** {E-commerce|SaaS|Fintech|Healthcare|etc.}
**Timeline:** {target timeline}
**Team:** {team size and distribution}

### 1.2 Problem Statement
{Clear articulation from Phase 1}

### 1.3 Solution Overview
{High-level solution description}

### 1.4 Success Criteria
{Measurable business outcomes from Phase 1}

---

## 2. User Roles & Personas

### 2.1 Primary Users
{From Phase 1.3}

### 2.2 User Personas
**Persona 1: {Name}**
- **Role:** {role}
- **Goals:** {goals}
- **Needs:** {needs}
- **Pain Points:** {current problems}

[... all personas ...]

---

## 3. Functional Requirements

### 3.1 User Stories
{All user stories from Phase 2.1}

### 3.2 Feature Requirements
{Organized by epic and feature from Phase 4}

---

## 4. Data Requirements

### 4.1 Data Model
{Complete entity-relationship documentation from Phase 2.2}

### 4.2 Data Constraints
- Validation rules
- Business rules
- Data integrity constraints

---

## 5. Integration Requirements

### 5.1 External Services
{All integrations from Phase 2.3}

### 5.2 API Contracts
- Endpoints
- Request/response formats
- Authentication methods
- Error handling

---

## 6. Non-Functional Requirements

### 6.1 Performance
{Complete NFRs from Phase 2.4}

### 6.2 Security
{Security requirements}

### 6.3 Scalability
{Scalability targets}

### 6.4 Availability
{Uptime and monitoring requirements}

---

## 7. Complexity Assessment

{Complete complexity report from Phase 3}

**Total Score:** {X}/60
**Architecture Tier:** {Tier 1-4}
**Recommended Technologies:** {tech stack}

---

## 8. Feasibility Analysis

{Complete feasibility assessment from Phase 5}

### 8.1 Technical Feasibility
{Assessment and risks}

### 8.2 Business Feasibility
{Budget and timeline analysis}

### 8.3 Resource Feasibility
{Team capacity assessment}

### 8.4 Risk Register
{All risks with mitigations}

---

## 9. Constraints & Assumptions

### 9.1 Technical Constraints
{From Phase 5}

### 9.2 Business Constraints
{Budget, timeline, regulatory}

### 9.3 Assumptions (Require Validation)
{All assumptions flagged during discovery}

---

## 10. Epic Breakdown

### Epic Roadmap
{Implementation roadmap from Phase 4}

### Epic Summaries
**EPIC-001:** {Name} - {Business goal} ({features count} features, {points} points)
[... all epics ...]

---

## 11. Next Steps

1. **Architecture Phase:** Run `/create-context {project-name}`
2. **Sprint Planning:** Run `/create-sprint 1`
3. **Story Creation:** Break features into stories

## Appendices

### A. Glossary
{Domain-specific terms}

### B. References
{External documentation, standards, APIs}

### C. Open Questions
{Items requiring future clarification}
```

**File Size:** Typically 500-2000 lines depending on project complexity

---

## Step 6.3: Transition to Architecture Skill

### Check for Existing Context Files

```
context_files = Glob(pattern="devforgeai/specs/context/*.md")

if len(context_files) == 6:
    existing_context = True
    # Brownfield project with existing constraints
else:
    existing_context = False
    # Greenfield project, need to create context
```

### Greenfield Path (No Context Files)

**Report to user:**

```
✅ Requirements documentation complete

Generated Documents:
- {N} Epic documents in devforgeai/specs/Epics/
- Requirements specification in devforgeai/specs/requirements/

Next Steps:
1. Run `/create-context [project-name]` to create context files
2. After context creation, run `/create-sprint 1` to begin sprint planning
```

**Recommended Next Action (display-only, no auto-invocation):**

Run `/create-context [project-name]`

The architecture skill will:
1. Reference requirements spec and complexity tier
2. Ask technology preference questions
3. Generate 6 context files (tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns)
4. Create ADRs for technology decisions
5. Validate requirements against constraints

**NOTE:** Per W3 compliance (STORY-135), the ideation skill does NOT auto-invoke
the architecture skill. The user manually runs `/create-context` when ready.

### Brownfield Path (Context Files Exist)

**Validate requirements against constraints:**

```
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/source-tree.md")
Read(file_path="devforgeai/specs/context/architecture-constraints.md")

# Check for conflicts (should have been resolved in Phase 5)
# If new conflicts found, use AskUserQuestion to resolve
```

**Report to user:**

```
✅ Requirements documentation complete

Context files exist. Requirements validated against constraints.

Generated Documents:
- {N} Epic documents in devforgeai/specs/Epics/
- Requirements specification in devforgeai/specs/requirements/

Ready to proceed with:
1. devforgeai-orchestration (create sprints/stories)
2. devforgeai-development (implement features)

Recommended: Run `/create-sprint 1` to begin sprint planning
```

**Do NOT invoke architecture skill** - context already exists

---

### Step 6.3.5: Validate Source Tree Compliance

**Purpose:** Detect epic proposals that reference directories not in source-tree.md constitutional file, preventing unconstitutional directory creation.

**Trigger:** After epic generation (Step 6.1), before transition (Step 6.3)

**Workflow:**

```
# Step 1: Read source-tree.md for valid directory patterns
source_tree = Read(file_path="devforgeai/specs/context/source-tree.md")

# Step 2: Extract proposed paths from epic content
# Pattern matches .claude/ and devforgeai/ directory references
proposed_paths = Grep(pattern="\\.(claude|devforgeai)/[a-zA-Z0-9_/-]+", path="devforgeai/specs/Epics/EPIC-{NNN}-*.epic.md")

# Step 3: Validate each proposed path against source-tree.md
violations = []
FOR each path in proposed_paths:
    IF path not found in source_tree:
        violations.append(path)
        Display: "WARNING: Proposed directory not in source-tree.md: {path}"

# Step 4: Add ADR requirement if violations found
IF len(violations) > 0:
    # Inject ADR prerequisite into epic
    adr_requirement = """
### ADR Required: Source Tree Update

**Proposed directories not in source-tree.md:**
{list of violations}

**Action Required:**
Before implementing stories that create these directories:
1. Create ADR documenting the need for new directory structure
2. Update source-tree.md with approved directories
3. Get architectural approval for constitutional change

**Reference:** source-tree.md is a constitutional file. Changes require ADR.
"""

    Display:
    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    "⚠️ WARNING: Source-tree.md compliance issues detected"
    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    FOR violation in violations:
        Display: "  • {violation} - not in source-tree.md"
    Display:
    "Epic updated with ADR requirement in Prerequisites section."
    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    # Add to epic Prerequisites section
    Edit(
        file_path="devforgeai/specs/Epics/EPIC-{NNN}.epic.md",
        old_string="## Prerequisites",
        new_string="## Prerequisites\n\n" + adr_requirement
    )

ELSE:
    Display: "✓ All proposed directories comply with source-tree.md"
```

**Rationale:** Prevents constitutional violations by detecting non-compliant directory proposals before they propagate to stories and implementation.

**Reference:** RCA-031 (Ideation Epic Missing Constitutional Sections), source-tree.md

---

## Common Issues and Recovery

### Issue: Epic Document Write Failed

**Symptom:** Write() tool fails with permission error or directory missing

**Recovery:**

```
# Ensure directory exists using Write/.gitkeep pattern (Constitutional C1 compliant)
Write(file_path="devforgeai/specs/Epics/.gitkeep", content="")

# Retry write
Write(file_path="devforgeai/specs/Epics/EPIC-001.epic.md", content=epic_content)

# If still fails after retry
if write_failed_after_retry:
    Report: """
    ERROR: Could not create epic document automatically

    Manual creation steps:
    1. Create file: devforgeai/specs/Epics/EPIC-001-{name}.epic.md
    2. Copy this content: {epic_content}
    3. Save file
    4. Continue to next epic or run /create-context
    """
```

### Issue: TodoWrite Tracking Mismatch

**Symptom:** TodoWrite shows 2 epics created but Glob finds 3 epic files

**Recovery:**

```
# Trust file system as source of truth
actual_epic_files = Glob(pattern="devforgeai/specs/Epics/EPIC-*.epic.md")

# Verify each file has valid YAML frontmatter
for epic_file in actual_epic_files:
    Read(file_path=epic_file, limit=20)  # Read frontmatter only
    # Validate has required fields

# Update TodoWrite to match reality
# Proceed if all epic files valid
```

### Issue: Requirements Spec Too Large (>3000 lines)

**Symptom:** Generated requirements spec is massive and difficult to navigate

**Recovery:**

```
# Split into multiple files
1. Create: {project}-requirements-overview.md (executive summary, ~200 lines)
2. Create: {project}-functional-requirements.md (user stories, features, ~800 lines)
3. Create: {project}-technical-requirements.md (data, integrations, NFRs, ~600 lines)
4. Create: {project}-complexity-feasibility.md (assessment, risks, ~400 lines)

# Link from overview document
# Easier to navigate and reference specific sections
```

---

## Output from Steps 6.1-6.3

**Files Created:**

1. **Epic Documents** (1-3 files)
   - Location: `devforgeai/specs/Epics/EPIC-{NNN}-{name}.epic.md`
   - Size: 150-400 lines per epic
   - Format: YAML frontmatter + markdown sections

2. **Requirements Specification** (1 file, optional)
   - Location: `devforgeai/specs/requirements/{project}-requirements.md`
   - Size: 500-2000 lines
   - Format: Markdown with numbered sections

**Verification:**

```
# Verify all planned epics created
epic_files = Glob(pattern="devforgeai/specs/Epics/EPIC-*.epic.md")
assert len(epic_files) == planned_epic_count

# Verify requirements spec (if generated)
if requirements_spec_requested:
    req_files = Glob(pattern="devforgeai/specs/requirements/*.md")
    assert len(req_files) >= 1

✓ Artifact generation complete
→ Proceed to Phase 6.4 (Self-Validation)
```

---

## Integration with Phase 4 Decomposition

**Phase 4 provides:**
- Epic list (1-3 epics)
- Feature breakdown (3-8 features per epic)
- High-level story outlines (2-5 stories per feature)
- Epic dependencies
- Priority assignments

**Phase 6.1 transforms into:**
- Structured epic documents with YAML frontmatter
- Complete feature descriptions
- Business goals and success metrics
- Complexity and architecture recommendations
- Risk register

**Key difference:** Phase 4 = planning data, Phase 6.1 = formal documents

---

## Directory Structure Requirements

**Expected structure after artifact generation:**

```
devforgeai/specs/
├── Epics/
│   ├── EPIC-001-user-management.epic.md
│   ├── EPIC-002-product-catalog.epic.md
│   └── EPIC-003-shopping-checkout.epic.md

devforgeai/
├── specs/
│   └── requirements/
│       └── {project-name}-requirements.md
```

**Validation:**

```
# Ensure directories exist using Write/.gitkeep pattern (Constitutional C1 compliant)
Write(file_path="devforgeai/specs/Epics/.gitkeep", content="")
Write(file_path="devforgeai/specs/requirements/.gitkeep", content="")

# Verify creation
Glob(pattern="devforgeai/specs/Epics/")  # Should exist
Glob(pattern="devforgeai/specs/requirements/")  # Should exist
```

---

## Epic Numbering Convention

**Sequential numbering starting from 001:**

```
# Find highest existing epic number
existing_epics = Glob(pattern="devforgeai/specs/Epics/EPIC-*.epic.md")

if len(existing_epics) == 0:
    next_epic_number = 1
else:
    # Extract numbers from filenames
    epic_numbers = []
    for epic_file in existing_epics:
        # Parse EPIC-NNN from filename
        number = extract_epic_number(epic_file)
        epic_numbers.append(number)

    next_epic_number = max(epic_numbers) + 1

# Format as 3-digit: EPIC-001, EPIC-002, etc.
epic_id = f"EPIC-{next_epic_number:03d}"
```

**Prevents:**
- Epic ID collisions
- Numbering gaps (if epics deleted)
- Confusion in brownfield projects

---

## Epic Status Field

**Valid statuses:**
- **Planning:** Epic created, not started (default for new epics)
- **In Progress:** At least one story started
- **Paused:** Work temporarily stopped (external blocker, reprioritization)
- **Completed:** All features delivered
- **Archived:** No longer relevant

**Status managed by:**
- Ideation skill: Sets "Planning" on creation
- Orchestration skill: Updates during story lifecycle
- Manual: User can edit status in epic file

---

## Success Criteria for Steps 6.1-6.3

Artifact generation complete when:
- [ ] All planned epics created (verified via Glob)
- [ ] All epic files have valid YAML frontmatter
- [ ] All epics have features section (3-8 features)
- [ ] All epics have business goals and success metrics
- [ ] Requirements spec created (if requested)
- [ ] No write errors occurred
- [ ] TodoWrite tracking matches file system reality
- [ ] Ready for validation (Phase 6.4)

**Token Budget:** ~5,000-12,000 tokens (generate 1-3 epics, optional requirements spec)

---

## References Used in Steps 6.1-6.3

**Phase Data Sources:**
- Phase 1: Problem statement, users, goals
- Phase 2: Complete requirements list
- Phase 3: Complexity score and tier
- Phase 4: Epic/feature decomposition
- Phase 5: Feasibility assessment and risks

**Reference Files:**
- **domain-specific-patterns.md** - Epic template patterns by domain
- **output-templates.md** - Epic document templates (loaded in 6.5, not 6.1)

**On Error:**
- **error-handling.md** - Artifact generation failure recovery

---

**Next Step:** Phase 6.4 (Self-Validation) - Load self-validation-workflow.md
