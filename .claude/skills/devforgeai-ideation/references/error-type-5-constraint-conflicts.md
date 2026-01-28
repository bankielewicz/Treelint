# Error Type 5: Brownfield Constraint Conflicts

Handling conflicts between new requirements and existing context file constraints.

---

## Error Detection

**Symptom:** New requirements conflict with existing context files (brownfield projects)

**Detected during:** Phase 5 (Feasibility Analysis) or Phase 6.6 (Handoff)

**Examples:**
- New requirement needs React, but tech-stack.md specifies Vue
- New feature requires microservices, but architecture-constraints.md enforces monolith
- New dependency not in approved dependencies.md
- New pattern violates anti-patterns.md

**Detection logic:**

```
if existing_context_files:
    # Load existing constraints
    Read(file_path="devforgeai/specs/context/tech-stack.md")
    Read(file_path="devforgeai/specs/context/architecture-constraints.md")
    Read(file_path="devforgeai/specs/context/dependencies.md")
    Read(file_path="devforgeai/specs/context/anti-patterns.md")

    # Compare with new requirements
    conflicts = []

    if new_requirement.technology not in existing_tech_stack:
        conflicts.append({
            "type": "technology_conflict",
            "new": new_requirement.technology,
            "existing": existing_tech_stack,
            "severity": "HIGH"
        })

    if new_requirement.violates_architecture_constraints:
        conflicts.append({
            "type": "architecture_conflict",
            "constraint": constraint_violated,
            "severity": "CRITICAL"
        })

    if len(conflicts) > 0:
        trigger brownfield_conflict_recovery(conflicts)
```

---

## Recovery Procedures

### Step 1: Detect Conflicts

```
# Technology conflicts
tech_conflicts = Grep(
    pattern=new_technology_name,
    path="devforgeai/specs/context/tech-stack.md",
    output_mode="count"
)

if tech_conflicts == 0:
    # Technology not in approved stack
    conflict detected: New requirement uses {new_tech}, approved stack has {existing_tech}

# Architecture pattern conflicts
constraint_violations = validate_against_constraints(new_requirement)

if len(constraint_violations) > 0:
    # Architecture constraints violated
    conflict detected: Requirement violates {constraint_name}

# Dependency conflicts
dependency_conflicts = check_dependencies(new_requirement)

if len(dependency_conflicts) > 0:
    # Unapproved dependencies needed
    conflict detected: Requirement needs {new_dependency}, not in approved list
```

### Step 2: Use AskUserQuestion to Resolve Each Conflict

```
For each conflict:
    AskUserQuestion(
        question: "New requirement '{requirement}' conflicts with existing constraint '{constraint}'. How to resolve?",
        header: "Conflict resolution",
        options: [
            {
                label: "Update constraint",
                description: "Modify existing constraint to accommodate new requirement (creates ADR)"
            },
            {
                label: "Modify requirement",
                description: "Adjust new requirement to fit existing constraints"
            },
            {
                label: "Mark as future scope",
                description: "Defer this requirement to future release"
            }
        ],
        multiSelect: false
    )
```

### Step 3: Document Resolution in ADR Requirements

```
If user chooses "Update constraint":
    # Document ADR requirement for architecture phase
    adr_requirement = {
        "number": get_next_adr_number(),
        "title": f"{constraint_type}-change-for-{requirement_name}",
        "context": f"New requirement from ideation conflicts with existing {context_file}",
        "decision": f"Update {context_file} to allow {new_value}",
        "rationale": user_provided_rationale,
        "consequences": "Impact on existing codebase and future development"
    }

    # Add to requirements spec
    Add section: "## Architecture Decision Requirements"
    Document: "ADR required for {context_file} change - will be created in architecture phase"

If user chooses "Modify requirement":
    # Update requirement to comply with constraint
    original_requirement = requirement
    modified_requirement = modify_to_comply(requirement, constraint)

    # Update in requirements spec
    Edit(
        file_path=requirements_spec,
        old_string=original_requirement,
        new_string=f"{modified_requirement}\n*(Modified to comply with {context_file})*"
    )

    ✓ Requirement modified to fit constraints

If user chooses "Mark as future scope":
    # Move to future scope section
    Move requirement from "In Scope" to "Future Scope"

    # Document reason
    Add note: "Deferred due to conflict with {context_file}"

    ✓ Requirement deferred to future release
```

### Step 4: Update Requirements Spec with Resolved Conflicts

```
# Add conflict resolution section
Add to requirements spec:

**Constraint Conflicts Resolved:**
1. {Requirement}: {Resolution} (See ADR requirement / Modified / Deferred)
2. [... all conflicts ...]

# Ensure all conflicts documented for architecture phase
```

---

## Example Scenarios

### Scenario 1: Technology Stack Conflict

**Error:** New feature needs Redis, but tech-stack.md only allows PostgreSQL

**Recovery:**
1. Detect technology not in approved stack
2. Ask user: Update stack, modify requirement, or defer
3. Document resolution (ADR if stack updated)

### Scenario 2: Architecture Pattern Conflict

**Error:** Feature requires microservices, but monolith architecture is enforced

**Recovery:**
1. Detect architecture constraint violation
2. Present options to user
3. If updating constraint, document ADR requirement

### Scenario 3: Anti-Pattern Violation

**Error:** Proposed design uses God Object pattern (forbidden)

**Recovery:**
1. Detect anti-pattern violation
2. Explain why pattern is forbidden
3. Suggest compliant alternative design

---

## Max Recovery Attempts

**Not applicable** - Each conflict requires explicit user decision (not auto-correctable)

**Process:** Detect all conflicts → Present to user → Resolve each → Document resolutions

---

## Related Patterns

- See context files in `devforgeai/specs/context/` for constraint definitions
- See [error-handling-index.md](error-handling-index.md) for error type decision tree
- ADR creation handled by devforgeai-architecture skill

---

## Phase Context

This error occurs during **Phase 5: Feasibility Analysis** (brownfield projects only) when validating that new requirements don't conflict with existing architectural constraints.

It may also occur during **Phase 6.6: Handoff** when preparing the project for architecture phase.

Recovery requires explicit user decision via AskUserQuestion - no automatic resolution possible.

---

**Token Budget:** ~1,000-2,000 tokens per conflict resolution
