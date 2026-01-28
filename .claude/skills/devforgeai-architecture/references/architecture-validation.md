# Phase 5: Architecture Validation

Validate that technical specifications respect all context file constraints before proceeding to development.

## Overview

Phase 5 ensures consistency between technical specifications and the 6 immutable context files created in Phase 2. This prevents conflicts and ensures specifications are implementable within established constraints.

**Purpose:** Catch conflicts early, before development begins.

---

## Validation Workflow

### Step 1: Load All Context Files

```
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/source-tree.md")
Read(file_path="devforgeai/specs/context/dependencies.md")
Read(file_path="devforgeai/specs/context/coding-standards.md")
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
Read(file_path="devforgeai/specs/context/anti-patterns.md")
```

---

### Step 2: Validate Technical Spec Compliance

#### 2.1 Tech Stack Compliance

**Check:** Does spec require any technologies not in tech-stack.md?

```
Example violation:
- Spec requires: Redis for caching
- tech-stack.md specifies: No caching layer defined
- Action: CONFLICT DETECTED
```

**Resolution:**
```
Use AskUserQuestion:

Question: "Spec requires Redis for caching, but tech-stack.md doesn't specify a caching layer. How should we proceed?"
Header: "Tech conflict"
Options:
  - "Add Redis to tech-stack.md (create ADR)"
  - "Remove caching from spec (not needed)"
  - "Use alternative caching (specify which)"
Description: "This will update tech-stack.md and require ADR documentation"
multiSelect: false
```

#### 2.2 Dependency Compliance

**Check:** Does spec require any packages not in dependencies.md?

```
Example violation:
- Spec requires: NewtonSoft.Json for serialization
- dependencies.md specifies: System.Text.Json only
- Action: CONFLICT DETECTED
```

**Resolution:**
```
Use AskUserQuestion:

Question: "Spec requires NewtonSoft.Json, but dependencies.md specifies System.Text.Json. Which is correct?"
Header: "Dependency conflict"
Options:
  - "Use System.Text.Json (follow existing standard)"
  - "Add NewtonSoft.Json (update dependencies.md + ADR)"
  - "Reassess requirement (maybe System.Text.Json is sufficient)"
multiSelect: false
```

#### 2.3 Structure Compliance

**Check:** Does spec define file locations that violate source-tree.md?

```
Example violation:
- Spec places: API controllers in src/Controllers/
- source-tree.md specifies: Controllers must be in src/API/Controllers/
- Action: CONFLICT DETECTED
```

**Resolution:**
- Update spec to match source-tree.md (context files are immutable)
- If source-tree.md is genuinely wrong, update it + create ADR

#### 2.4 Architecture Constraint Compliance

**Check:** Does spec violate layer boundaries?

```
Example violation:
- Spec shows: Domain entity with database annotations
- architecture-constraints.md: Domain must be infrastructure-agnostic
- Action: CONSTRAINT VIOLATION
```

**Resolution:**
- Redesign to respect layer boundaries
- Domain entities stay pure, Infrastructure handles persistence mapping

#### 2.5 Anti-Pattern Detection

**Check:** Does spec include any forbidden patterns?

```
Example violation:
- Spec includes: Direct SQL concatenation
- anti-patterns.md forbids: SQL injection vulnerabilities
- Action: ANTI-PATTERN DETECTED
```

**Resolution:**
- Redesign using parameterized queries
- Update spec to use safe patterns

---

### Step 3: Validation Report

Generate validation summary:

```markdown
## Validation Results

### Tech Stack Compliance: ✅ PASS
- All technologies in spec are approved in tech-stack.md
- No conflicts detected

### Dependency Compliance: ⚠️ WARNING
- Spec requires NewtonSoft.Json (not in dependencies.md)
- Resolution: AskUserQuestion completed, System.Text.Json approved

### Structure Compliance: ✅ PASS
- All file locations match source-tree.md
- No placement conflicts

### Architecture Compliance: ✅ PASS
- No layer boundary violations
- Repository pattern followed
- DTO pattern followed

### Anti-Pattern Detection: ✅ PASS
- No forbidden patterns found
- All queries parameterized
- No hardcoded secrets

**Overall Status:** ✅ APPROVED (1 warning resolved)
```

---

### Step 4: Conflict Resolution

If conflicts detected:

1. **Identify conflict type:**
   - Technology not approved
   - Dependency not listed
   - Structure doesn't match
   - Architecture violation
   - Anti-pattern present

2. **Use AskUserQuestion:**
   - Present conflict clearly
   - Offer 2-3 resolution options
   - Explain consequences of each

3. **Update artifacts:**
   - If context file wrong: Update context file + create ADR
   - If spec wrong: Update spec to respect constraints

4. **Re-validate** after changes

---

## Success Criteria

Phase 5 succeeds when:

- [ ] All 6 context files loaded and reviewed
- [ ] Spec validated against each context file
- [ ] All conflicts resolved (via AskUserQuestion)
- [ ] No technology not in tech-stack.md
- [ ] No package not in dependencies.md
- [ ] All file locations match source-tree.md
- [ ] No layer boundary violations
- [ ] No anti-patterns present
- [ ] Validation report generated

**Output:** Validated technical specification ready for implementation.

**Next Phase:** Ready for devforgeai-orchestration (story planning) or devforgeai-development (implementation).
