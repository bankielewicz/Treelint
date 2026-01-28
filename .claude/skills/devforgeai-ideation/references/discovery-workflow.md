# Phase 1: Discovery & Problem Understanding

Initial problem exploration through strategic questioning to establish foundational understanding.

## TodoWrite - Phase Start

**At phase start, update todo list:**
```
TodoWrite([
  {"content": "Phase 1: Discovery & Problem Understanding", "status": "in_progress", "activeForm": "Discovering problem space"}
])
```

## Overview

Phase 1 establishes foundational understanding of the business problem, users, and desired outcomes before diving into detailed requirements. This phase prevents wasted effort by ensuring alignment on problem definition and project type.

**Duration:** 5-15 minutes
**Questions:** 5-10 strategic questions
**Output:** Problem statement, user personas, business goals, project type classification

---

## Step 1.1: Project Context Discovery

Use AskUserQuestion to establish foundation:

```
Question: "What type of project is this?"
Header: "Project type"
Options:
  - "Greenfield - New project/product from scratch"
  - "Brownfield - Adding features to existing system"
  - "Modernization - Replacing/upgrading legacy system"
  - "Problem-solving - Fixing issues in current system"
```

**Purpose:** Determines workflow path (greenfield vs brownfield) and discovery approach.

**Downstream Impact:**
- **Greenfield:** Full discovery, no existing constraints
- **Brownfield:** Analyze existing system first, respect constraints
- **Modernization:** Understand current state, plan migration path
- **Problem-solving:** Root cause analysis, solution validation

---

## Step 1.2: Existing System Analysis (Brownfield/Modernization Only)

**Skip this step if Greenfield or Problem-solving.**

### Discover Codebase Structure

```
Glob(pattern="**/*.sln")          # .NET solutions
Glob(pattern="**/package.json")   # Node.js projects
Glob(pattern="**/requirements.txt") # Python projects
Glob(pattern="**/pom.xml")        # Java/Maven projects
Glob(pattern="**/Cargo.toml")     # Rust projects
Glob(pattern="**/go.mod")         # Go modules
```

**Document:** Primary language, framework, project structure

### Check for Existing DevForgeAI Context

```
context_files = Glob(pattern="devforgeai/specs/context/*.md")

if len(context_files) == 6:
    # Existing DevForgeAI project
    Read(file_path="devforgeai/specs/context/tech-stack.md")
    Read(file_path="devforgeai/specs/context/source-tree.md")
    Read(file_path="devforgeai/specs/context/architecture-constraints.md")

    existing_context = True
    # Use existing constraints for validation in Phase 5
else:
    existing_context = False
    # Greenfield within existing codebase
```

### Analyze Current Architecture

```
# Detect architecture patterns
Grep(pattern="class.*Controller", output_mode="files_with_matches")
Grep(pattern="interface I.*Repository", output_mode="files_with_matches")
Grep(pattern="class.*Service", output_mode="files_with_matches")

# Detect database usage
Grep(pattern="DbContext", output_mode="files_with_matches")  # Entity Framework
Grep(pattern="mongoose", output_mode="files_with_matches")   # MongoDB
Grep(pattern="Sequelize", output_mode="files_with_matches")  # Sequelize ORM
```

**Document:**
- Technology stack (languages, frameworks, databases)
- Architecture patterns (MVC, clean architecture, layered)
- Current pain points (performance, maintainability, scalability)
- Integration points (APIs, third-party services)

---

## Step 1.3: Problem Space Exploration

Use AskUserQuestion for open-ended discovery:

```
Question: "What business problem are you trying to solve?"
# Free-text response expected
```

**Follow-up probing:**

```
Question: "Who are the primary users or beneficiaries?"
Options:
  - "End customers/consumers"
  - "Internal employees"
  - "Business partners/vendors"
  - "Administrators/operators"
multiSelect: true
```

**Goals and success metrics:**

```
Question: "What is the primary goal or success metric?"
Options:
  - "Increase revenue/conversions"
  - "Reduce costs/inefficiency"
  - "Improve user experience"
  - "Enable new capabilities"
  - "Compliance/regulatory requirement"
multiSelect: true
```

**Document collected:**
- **Problem statement:** Clear articulation of problem
- **User personas:** Who will use this (roles, needs)
- **Business goals:** Measurable outcomes (revenue, efficiency, UX)
- **Success metrics:** How to measure success (KPIs)

---

## Step 1.4: Scope Boundary Definition

```
Question: "What is the initial scope for the MVP or first release?"
Options:
  - "Core feature only (single user flow)"
  - "Core + 2-3 secondary features"
  - "Full feature set (comprehensive solution)"
  - "Not sure - need help defining MVP"
```

**If "Not sure":**
- Use requirements from next phases to propose MVP scope
- Revisit in Phase 4 after epic decomposition
- Apply 80/20 rule (20% of features deliver 80% of value)

**Document:**
- **In Scope:** Features included in MVP/first release
- **Out of Scope:** Features explicitly excluded
- **Future Scope:** Features deferred to later releases

**Scope management:**
- Keep MVP focused (3-5 core features maximum)
- Ensure MVP delivers complete user value (not half-features)
- Plan roadmap for future releases

---

## Output from Phase 1

**Discovery Document (internal):**
```markdown
# Discovery Summary

## Project Type
{Greenfield|Brownfield|Modernization|Problem-solving}

## Problem Statement
{Clear articulation of business problem}

## User Personas
{Primary users, roles, needs}

## Business Goals
{Measurable goals and success metrics}

## Scope Boundaries
- In Scope: {MVP features}
- Out of Scope: {Excluded features}
- Future Scope: {Deferred features}

## Existing System Analysis (Brownfield only)
- Technology: {Current stack}
- Architecture: {Current patterns}
- Pain Points: {Issues to address}
- Constraints: {DevForgeAI context files if exist}
```

**Transition:** Proceed to Phase 2 (Requirements Elicitation)

---

## Common Issues and Recovery

### Issue: User Cannot Define MVP Scope

**Symptom:** User selects "Not sure - need help defining MVP"

**Recovery:**
1. Defer scope definition to Phase 4 (after epic decomposition)
2. Complete requirements discovery (Phases 2-3)
3. In Phase 4, prioritize features by value/effort ratio
4. Propose MVP = highest value + lowest effort features
5. Use AskUserQuestion to validate proposed MVP

### Issue: Brownfield System Too Complex to Analyze

**Symptom:** Codebase has 10,000+ files, unclear structure

**Recovery:**
1. Ask user for architecture documentation
2. Focus on pain points, not complete analysis
3. Use Grep to find specific patterns only
4. Document current state as "Complex existing system (full analysis deferred)"
5. Proceed with requirements discovery for new features

### Issue: Conflicting Business Goals

**Symptom:** User selects "Increase revenue" AND "Reduce costs" with no priority

**Recovery:**
```
AskUserQuestion(
    question: "Both goals are important. Which should be prioritized if there's a trade-off?",
    header: "Goal priority",
    options: [
        "Revenue growth (even if costs increase)",
        "Cost reduction (even if revenue temporarily flat)",
        "Balance both (may take longer)"
    ]
)
```

---

## Integration Points

**Calls from:**
- /ideate command (user-initiated)
- devforgeai-orchestration (when creating new epics without requirements)

**Flows to:**
- Phase 2: Requirements Elicitation (always)
- Error handling: Incomplete answers (as needed)

**References used:**
- None in Phase 1 (pure discovery)

---

## Success Criteria

Phase 1 complete when:
- [ ] Project type determined (greenfield/brownfield/modernization/problem-solving)
- [ ] Problem statement clearly articulated
- [ ] User personas identified (1-5 personas)
- [ ] Business goals documented (1-3 measurable goals)
- [ ] Success metrics defined (quantifiable)
- [ ] Scope boundaries established (in/out/future)
- [ ] Existing system analyzed (if brownfield)
- [ ] No critical ambiguities remain

**Token Budget:** ~2,000-4,000 tokens (5-10 AskUserQuestion interactions)

---

## TodoWrite - Phase Completion

**At phase end, mark as completed:**
```
TodoWrite([
  {"content": "Phase 1: Discovery & Problem Understanding", "status": "completed", "activeForm": "Discovering problem space"}
])
```

---

**Next Phase:** Phase 2 (Requirements Elicitation)
