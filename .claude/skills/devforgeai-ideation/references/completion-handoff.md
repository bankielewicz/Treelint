# Phase 6.5-6.6: Completion Summary & Next Action Determination

Present ideation results and determine appropriate next steps based on project type and existing context.

## Overview

Phase 6.5-6.6 completes the ideation workflow by presenting a comprehensive summary to the user and determining the appropriate next action (architecture creation for greenfield, orchestration for brownfield).

**Duration:** 2-5 minutes
**Output:** Completion summary with artifacts, complexity assessment, recommendations, and next action

---

## Step 6.5: Present Completion Summary

### Load Output Templates

```
Read(file_path=".claude/skills/devforgeai-ideation/references/output-templates.md")
```

This reference provides standardized templates for:
- Completion summary structure
- Technology recommendations by architecture tier (Tier 1-4)
- Next steps templates for greenfield/brownfield transitions

For output templates, see: [output-templates.md](output-templates.md) (Completion Summary & Next Steps templates)
For technology recommendations, see: [complexity-assessment-matrix.md](complexity-assessment-matrix.md) (Technology Recommendations by Tier)

### Generate Completion Summary

Use the **Ideation Completion Template** from output-templates.md:

```markdown
## ✅ Ideation Complete

### Generated Artifacts

**Epic Documents:** {count} created
- EPIC-001: {Epic Title} ({feature_count} features, {story_points} points)
- EPIC-002: {Epic Title} ({feature_count} features, {story_points} points)
- EPIC-003: {Epic Title} ({feature_count} features, {story_points} points)

📁 **Location:** `devforgeai/specs/Epics/`

**Requirements Specification:** {created|not created}
{if created:}
- Functional Requirements: {count}
- Non-Functional Requirements: {count}
- Data Models: {count} entities
- Integration Points: {count} external systems

📁 **Location:** `devforgeai/specs/requirements/{project-name}-requirements.md`

---

### Complexity Assessment

**Total Score:** {score}/60
**Architecture Tier:** {Tier 1: Simple | Tier 2: Moderate | Tier 3: Complex | Tier 4: Enterprise}

**Score Breakdown:**
- Functional Complexity: {score}/20 ({Low|Medium|High})
- Technical Complexity: {score}/20 ({Low|Medium|High})
- Team/Organizational Complexity: {score}/10 ({Low|Medium|High})
- Non-Functional Complexity: {score}/10 ({Low|Medium|High})

**Rationale:** {Brief explanation of score}

---

### Recommended Technology Stack

**Based on {tier} architecture tier:**

{Load technology recommendations from output-templates.md for this tier}

**Example for Tier 2 (Moderate Application):**

**Backend:**
- Language: Node.js, Python, C#, Java
- Framework: NestJS, ASP.NET Core, Spring Boot, Django
- Architecture: Modular Monolith or Layered Architecture

**Frontend:**
- Framework: React, Vue, Angular
- State Management: Zustand, Redux Toolkit, Vuex
- Styling: Tailwind CSS, Material-UI, Bootstrap

**Database:**
- Primary: PostgreSQL, MySQL
- Caching: Redis
- Search: Elasticsearch (if needed)

**Infrastructure:**
- Deployment: AWS ECS, Azure App Service, Google Cloud Run
- CI/CD: GitHub Actions, GitLab CI, Azure DevOps
- Monitoring: Datadog, New Relic, CloudWatch

{Complete recommendations from output-templates.md}

---

### Implementation Roadmap

**Estimated Timeline:**
- Total Effort: {total_story_points} story points
- Estimated Sprints: {sprint_count} sprints (~{weeks} weeks)
- Target Completion: {estimated_date}

**Epic Sequence:**
1. Sprint 1-2: {Epic 1 name} (Priority: P0)
2. Sprint 3-4: {Epic 2 name} (Priority: P0)
3. Sprint 5-7: {Epic 3 name} (Priority: P1)

---

### Risk Summary

**CRITICAL Risks:** {count}
{List CRITICAL risks with mitigations}

**HIGH Risks:** {count}
{List HIGH risks with mitigations}

**Recommendation:** {Proceed|Proceed with mitigations|Adjust scope}

---

### Next Steps

{Generated based on Step 6.6 decision - see below}
```

**Present formatted summary to user** - This is the primary output users see from ideation skill.

---

## Step 6.6: Determine Next Action

### Check for Existing Context Files

```
context_files = Glob(pattern="devforgeai/specs/context/*.md")

if len(context_files) == 6:
    # Brownfield project with existing DevForgeAI context
    project_mode = "brownfield"
    existing_context = True
else:
    # Greenfield project, need to create context
    project_mode = "greenfield"
    existing_context = False
```

### Greenfield Path (No Context Files)

**Present next action options:**

```
AskUserQuestion(
  questions=[{
    question: "Ideation phase complete. How would you like to proceed?",
    header: "Next action",
    options: [
      {
        label: "Create context files",
        description: "Run /create-context to define tech stack and project structure (6 context files)"
      },
      {
        label: "Review requirements first",
        description: "I want to review/edit generated requirements before proceeding to architecture"
      }
    ],
    multiSelect: false
  }]
)
```

**Based on user response:**

**Option: "Create context files"**

```
Report: """
Next step: Run `/create-context {project-name}`

The architecture skill will:
1. Reference requirements spec and complexity tier
2. Ask technology preference questions
3. Generate 6 context files:
   - tech-stack.md (locked technologies)
   - source-tree.md (project structure)
   - dependencies.md (approved packages)
   - coding-standards.md (code patterns)
   - architecture-constraints.md (layer boundaries)
   - anti-patterns.md (forbidden patterns)
4. Create ADRs for technology decisions
5. Validate requirements against constraints

After context creation:
1. Run `/create-missing-stories` to generate stories from epic features
2. Run `/create-sprint 1` to begin sprint planning
"""
```

**Option: "Review requirements first"**

```
Report: """
📂 Review these files:
- **Epics:** `devforgeai/specs/Epics/EPIC-*.epic.md`
- **Requirements:** `devforgeai/specs/requirements/{project-name}-requirements.md`

You can:
- Manually edit files (Use Read/Edit tools or text editor)
- Ask me to make specific changes ("Update EPIC-001 to add Feature X")
- Run `/create-context {project-name}` when ready to proceed to architecture

Tip: Epic documents feed directly into sprint planning, so ensure they're complete.
"""
```

### Brownfield Path (Context Files Exist)

**Validate requirements against existing constraints:**

```
# Load existing context
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/source-tree.md")
Read(file_path="devforgeai/specs/context/architecture-constraints.md")

# Check for conflicts (should have been resolved in Phase 5)
# If new conflicts found, use AskUserQuestion to resolve
conflicts = validate_against_context_files(requirements)

if len(conflicts) > 0:
    # Present conflicts and resolution options
    for conflict in conflicts:
        AskUserQuestion(
            question: f"Requirement '{conflict.requirement}' conflicts with {conflict.context_file}. Resolve?",
            header: "Constraint conflict",
            options: [
                "Update context file (requires ADR)",
                "Modify requirement to comply",
                "Defer to future release"
            ]
        )

    # Document resolutions
    # Update requirements spec with resolved conflicts
```

**Present next action options:**

```
AskUserQuestion(
  questions=[{
    question: "Ideation complete. Context files detected. How would you like to proceed?",
    header: "Next action",
    options: [
      {
        label: "Proceed to sprint planning",
        description: "Create Sprint 1 with /create-sprint (existing context will be used)"
      },
      {
        label: "Update context files",
        description: "Run /create-context to update existing constraints (creates ADRs for changes)"
      },
      {
        label: "Review requirements first",
        description: "I want to review/edit generated requirements before proceeding"
      }
    ],
    multiSelect: false
  }]
)
```

**Based on user response:**

**Option: "Proceed to sprint planning"**

```
Report: """
✅ Requirements validated against existing constraints

Next steps:
1. Run `/create-missing-stories` to generate stories from epic features
2. Run `/create-sprint 1` to begin sprint planning

The orchestration skill will:
1. Use existing context files (tech-stack, architecture-constraints, etc.)
2. Reference epic documents from ideation
3. Create Sprint 1 plan with goals and capacity
4. Select stories from epic features for Sprint 1
5. Update story status to "Ready for Dev"

After sprint planning, run `/dev {STORY-ID}` to implement first story.
"""
```

**Option: "Update context files"**

```
Report: """
Next step: Run `/create-context {project-name}`

The architecture skill will:
1. Load existing context files
2. Review new requirements for conflicts
3. Ask technology preference questions for new capabilities
4. Update context files as needed
5. Create ADRs documenting changes (preserves decision history)
6. Validate updated constraints

This is recommended when:
- New requirements need different technologies
- New architecture patterns needed
- Compliance requirements changed
"""
```

**Option: "Review requirements first"**

```
Report: """
📂 Review these files:
- **Epics:** `devforgeai/specs/Epics/EPIC-*.epic.md`
- **Requirements:** `devforgeai/specs/requirements/{project-name}-requirements.md`
- **Existing Context:** `devforgeai/specs/context/*.md`

After review, proceed with:
- `/create-sprint 1` (if context files are current)
- `/create-context {project-name}` (if context needs updates)
"""
```

---

## Transition Logic Summary

**Decision Tree:**

```
Ideation Complete
  ↓
Check context files exist?
  ├─ NO (Greenfield)
  │   ↓
  │   User choice: Create context | Review first
  │   ↓
  │   If "Create context":
  │       → /create-context (architecture skill)
  │       → /create-missing-stories (story generation)
  │       → /create-sprint (orchestration skill)
  │   If "Review first":
  │       → User reviews files
  │       → Then /create-context when ready
  │
  └─ YES (Brownfield)
      ↓
      Validate against context
      ↓
      User choice: Sprint planning | Update context | Review first
      ↓
      If "Sprint planning":
          → /create-sprint (orchestration skill)
      If "Update context":
          → /create-context (architecture skill with ADRs)
          → Then /create-sprint
      If "Review first":
          → User reviews files
          → Then /create-sprint or /create-context when ready
```

---

## Output from Steps 6.5-6.6

**Completion summary presented to user** (formatted markdown)

**Next action determined** - One of:
- `/create-context {project-name}` (greenfield or context update)
- `/create-sprint 1` (brownfield with valid context)
- User review (pause for manual editing)

**Ideation skill completes** - Handoff successful

---

## Step 6.7: Display Final Summary

**Purpose:** Present compact, actionable summary following QA skill pattern (Step 4.3).

**Display Template:**

```
╔════════════════════════════════════════════════════════╗
║               IDEATION COMPLETE                        ║
╠════════════════════════════════════════════════════════╣
║ Project: {project_name}                                ║
║ Mode: {Greenfield|Brownfield}                          ║
║ Complexity: {score}/60 (Tier {tier})                   ║
╠════════════════════════════════════════════════════════╣
║ Generated Artifacts:                                   ║
║   Epics: {count} in devforgeai/specs/Epics/            ║
║   Features: {total_features} across all epics          ║
║   Requirements Spec: {Yes|No}                          ║
╠════════════════════════════════════════════════════════╣
║ Next Steps (in order):                                 ║
║   1. /create-context {project}                         ║
║   2. /create-missing-stories                           ║
║   3. /create-sprint 1                                  ║
║   4. /dev {STORY-ID}                                   ║
╚════════════════════════════════════════════════════════╝
```

**Brownfield Variant (context files exist):**

```
╔════════════════════════════════════════════════════════╗
║               IDEATION COMPLETE                        ║
╠════════════════════════════════════════════════════════╣
║ Project: {project_name}                                ║
║ Mode: Brownfield (context files exist)                 ║
║ Complexity: {score}/60 (Tier {tier})                   ║
╠════════════════════════════════════════════════════════╣
║ Generated Artifacts:                                   ║
║   Epics: {count} in devforgeai/specs/Epics/            ║
║   Features: {total_features} across all epics          ║
║   Requirements Spec: {Yes|No}                          ║
╠════════════════════════════════════════════════════════╣
║ Next Steps (in order):                                 ║
║   1. /create-missing-stories                           ║
║   2. /create-sprint 1                                  ║
║   3. /dev {STORY-ID}                                   ║
╚════════════════════════════════════════════════════════╝
```

**Implementation:**
```python
def display_final_summary(session):
    mode = "Greenfield" if not session.context_files_exist else "Brownfield"

    Display: """
    ╔════════════════════════════════════════════════════════╗
    ║               IDEATION COMPLETE                        ║
    ╠════════════════════════════════════════════════════════╣
    ║ Project: {session.project_name}                        ║
    ║ Mode: {mode}                                           ║
    ║ Complexity: {session.complexity_score}/60 (Tier {session.tier}) ║
    ╠════════════════════════════════════════════════════════╣
    ...
    """
```

**Key Difference from QA Skill:**
- QA has pass/fail outcome variants
- Ideation has greenfield/brownfield path variants
- Both use numbered next steps for clarity

---

## Common Issues and Recovery

### Issue: User Unsure About Next Action

**Symptom:** User selects "Review requirements first" but uncertain what to review

**Recovery:**

```
Provide specific review guidance:

"Review these aspects:

1. **Epic Priority:** Are epics prioritized correctly for business value?
   - Check: `devforgeai/specs/Epics/EPIC-*.epic.md` frontmatter `priority` field

2. **Feature Completeness:** Are all required features documented?
   - Check: Each epic's Features section

3. **Success Metrics:** Are success criteria measurable?
   - Check: Each epic's Business Goal section

4. **Complexity Score:** Does complexity tier match expectations?
   - Check: Requirements spec Complexity Assessment section

5. **Risks:** Are all risks identified with mitigations?
   - Check: Requirements spec Risk Register

After review, run `/create-context {project-name}` to proceed to architecture phase."
```

### Issue: Context File Conflict Not Resolved in Phase 5

**Symptom:** Brownfield validation finds conflict that wasn't handled in feasibility phase

**Recovery:**

```
# This shouldn't happen (Phase 5 should catch all conflicts)
# But if detected in Step 6.6:

For each new conflict:
    AskUserQuestion(
        question: "Conflict detected: {conflict description}. How to resolve?",
        header: "Late conflict",
        options: [
            "Update context file (creates ADR requirement)",
            "Modify requirement to comply",
            "Defer requirement to future release"
        ]
    )

    Apply resolution
    Document in requirements spec
    Continue validation
```

### Issue: User Wants to Modify Epics After Generation

**Symptom:** User says "Change EPIC-001 to include Feature X"

**Recovery:**

```
# Use Edit tool to modify epic document
Read(file_path="devforgeai/specs/Epics/EPIC-001-{name}.epic.md")

# Find Features section
# Add new feature to list
Edit(
    file_path="devforgeai/specs/Epics/EPIC-001-{name}.epic.md",
    old_string="{existing features section}",
    new_string="{existing features + new feature}"
)

# Re-run validation to ensure consistency
# Update requirements spec if needed
```

---

## References Used in Steps 6.5-6.6

**Primary:**
- **output-templates.md** (619 lines) - Completion summary templates, technology recommendations by tier

**Related:**
- **validation-checklists.md** - Handoff readiness criteria
- **domain-specific-patterns.md** - Domain-specific guidance for next steps

**On Error:**
- **error-handling.md** - Handoff failure recovery

---

## Success Criteria

Completion handoff successful when:
- [ ] Completion summary presented to user
- [ ] All artifacts listed with locations
- [ ] Complexity assessment communicated
- [ ] Technology recommendations provided
- [ ] Risk summary presented
- [ ] Next action determined (greenfield vs brownfield path)
- [ ] User understands what to do next
- [ ] Ideation workflow complete

**Token Budget:** ~3,000-6,000 tokens (load templates, generate summary, determine next action)

---

## Final Output Example

**What user sees:**

```
## ✅ Ideation Complete

### Generated Artifacts

**Epic Documents:** 3 created
- EPIC-001: User Management (5 features, 25 points)
- EPIC-002: Product Catalog (6 features, 30 points)
- EPIC-003: Shopping & Checkout (7 features, 35 points)

📁 Location: `devforgeai/specs/Epics/`

**Requirements Specification:**
- Functional Requirements: 32
- Non-Functional Requirements: 18
- Data Models: 12 entities
- Integration Points: 5 external systems

📁 Location: `devforgeai/specs/requirements/ecommerce-platform-requirements.md`

---

### Complexity Assessment

**Total Score:** 28/60
**Architecture Tier:** Tier 2: Moderate Application

**Score Breakdown:**
- Functional Complexity: 12/20 (Medium)
- Technical Complexity: 10/20 (Medium)
- Team/Organizational Complexity: 4/10 (Low)
- Non-Functional Complexity: 2/10 (Low)

**Rationale:** Standard e-commerce platform with 3-5 user roles, 10-15 entities, moderate integrations, standard performance requirements. Suitable for modular monolith architecture.

---

### Recommended Technology Stack

**Backend:**
- Language: Node.js 20+ or C# .NET 8
- Framework: NestJS (Node) or ASP.NET Core (C#)
- Architecture: Modular Monolith with layered architecture

**Frontend:**
- Framework: React 18+ or Next.js 14+
- State Management: Zustand or Redux Toolkit
- UI Components: shadcn/ui or Material-UI
- Styling: Tailwind CSS

**Database:**
- Primary: PostgreSQL 15+
- Caching: Redis 7+
- Search: Elasticsearch (optional for product search)

**Infrastructure:**
- Deployment: AWS ECS or Azure App Service
- CDN: CloudFront or Azure CDN
- CI/CD: GitHub Actions
- Monitoring: Datadog or Application Insights

---

### Risk Summary

**CRITICAL Risks:** 0
**HIGH Risks:** 2
- Payment gateway integration complexity (Mitigation: Use Stripe with proven SDK)
- Scalability to 10k users (Mitigation: Design for horizontal scaling from start)

**Recommendation:** Proceed with mitigations in place

---

### Next Steps

**Recommended action: Create context files**

Run `/create-context ecommerce-platform`

The architecture skill will:
1. Reference requirements spec and complexity tier
2. Ask technology preference questions (confirm React vs Vue, PostgreSQL vs MySQL, etc.)
3. Generate 6 context files defining architectural boundaries
4. Create ADRs documenting technology decisions
5. Validate requirements against constraints

After context creation:
1. Run `/create-missing-stories` to generate stories from epic features
2. Run `/create-sprint 1` to select stories into sprint capacity

---

**Ideation complete.** Requirements discovery, complexity assessment, and epic decomposition finished. Ready for architecture phase.
```

---

## Common Issues and Recovery

### Issue: User Confused About Next Action

**Symptom:** User asks "What should I do now?"

**Recovery:**

```
Clarify based on project mode:

Greenfield: "Run `/create-context {project-name}` next. This creates the 6 architectural constraint files that prevent technical debt."

Brownfield: "Run `/create-sprint 1` next. This uses existing context files to plan Sprint 1 and generate stories from your epics."

Always: "You can review/edit epic documents in `devforgeai/specs/Epics/` before proceeding."
```

### Issue: User Wants Different Technology Than Recommended

**Symptom:** User says "I want to use technology X instead of recommended Y"

**Recovery:**

```
Acknowledge: "Technology recommendations are based on complexity tier, not mandatory."

Explain: "During `/create-context`, you'll be asked to choose technologies. You can select {preferred technology} at that time."

Note: "The architecture skill will validate your choices against requirements (performance, scalability, etc.) and create ADRs documenting decisions."

Proceed: Continue to next action (user makes final technology decisions in architecture phase)
```

### Issue: Completion Summary Too Long (>500 lines)

**Symptom:** Summary is overwhelming with too much detail

**Recovery:**

```
# Create executive summary version (short)
Executive Summary (~50 lines):
- Epics created: {count}
- Complexity: {tier}
- Recommended tech: {brief}
- Next action: /create-context

# Link to full details
"See complete details in:"
- Epic documents: `devforgeai/specs/Epics/`
- Requirements spec: `devforgeai/specs/requirements/`
```

---

## Integration with DevForgeAI Framework

### Flows to Next Skills

**Greenfield:**
- Ideation → **devforgeai-architecture** (create context) → devforgeai-orchestration (sprint planning)

**Brownfield:**
- Ideation → **devforgeai-orchestration** (sprint planning, uses existing context)
- OR: Ideation → devforgeai-architecture (update context) → devforgeai-orchestration

### What Happens Next

**Architecture skill receives:**
- Requirements specification
- Complexity tier and score
- Technology recommendations (as suggestions)
- Constraints and assumptions

**Architecture skill produces:**
- 6 context files (immutable constraints)
- ADRs documenting technology decisions
- Validation that requirements are feasible within constraints

**Orchestration skill receives:**
- Epic documents
- Context files (from architecture)
- Requirements spec

**Orchestration skill produces:**
- Sprint 1 plan
- Stories generated from epic features
- Story workflow setup

---

## Success Criteria

Steps 6.5-6.6 complete when:
- [ ] Completion summary presented to user
- [ ] Next action determined (greenfield vs brownfield)
- [ ] User knows exactly what to do next
- [ ] All questions answered
- [ ] Ideation skill exits gracefully
- [ ] Handoff to architecture or orchestration ready

**Token Budget:** ~3,000-6,000 tokens (load templates, generate summary, user interaction)

---

**Phase 6 Complete** - Ideation skill has successfully transformed business idea into structured requirements ready for architecture and development phases.

---

## References Used

**Primary:**
- [output-templates.md](output-templates.md) - Summary templates (cross-references matrix for tech details)

**Supporting:**
- [validation-checklists.md](validation-checklists.md) - Handoff readiness validation
- [complexity-assessment-matrix.md](complexity-assessment-matrix.md) - Technology recommendations per tier (authoritative source)

---

**Ideation Workflow Complete** ✅
