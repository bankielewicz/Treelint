# Context File Validation Reference

**Purpose:** Centralized validation logic for story content against 6 constitutional context files. This reference is the Single Source of Truth (SSOT) for context validation, reused by:
- Phase 3.6 (during tech spec generation)
- Phase 5.1 (before file write)
- Phase 7.7 (final validation)
- `/validate-stories` command (post-hoc validation)

---

## Overview

Validates story content against 6 constitutional context files to prevent technical debt at specification stage.

**Context Files (Read-Only Reference):**

| File | Path | Validates |
|------|------|-----------|
| tech-stack.md | `devforgeai/specs/context/tech-stack.md` | Technology choices |
| source-tree.md | `devforgeai/specs/context/source-tree.md` | File paths and directories |
| dependencies.md | `devforgeai/specs/context/dependencies.md` | Package dependencies |
| coding-standards.md | `devforgeai/specs/context/coding-standards.md` | Coverage thresholds, patterns |
| architecture-constraints.md | `devforgeai/specs/context/architecture-constraints.md` | Layer boundaries |
| anti-patterns.md | `devforgeai/specs/context/anti-patterns.md` | Forbidden patterns |

---

## Greenfield Mode

**Check for context files before validation:**

```
context_dir = "devforgeai/specs/context/"
context_files = Glob(pattern=f"{context_dir}*.md")

if len(context_files) == 0:
    # Greenfield mode: no context files exist
    Display: """
    ℹ️ Greenfield Mode: Context validation skipped

    No context files found in devforgeai/specs/context/
    Story creation will proceed without framework validation.

    Recommendation: Run /create-context to enable validation
    """
    SKIP validation functions
    RETURN { greenfield: true, violations: [] }
```

**If context files exist:** Proceed with validation functions below.

---

## Validation Functions

### 1. validate_technologies(tech_spec_content)

**Purpose:** Validate technologies in technical specification against tech-stack.md

**Severity:** HIGH

**Input:** Technical specification text from story

**Process:**
```
1. Read tech-stack.md:
   tech_stack = Read(file_path="devforgeai/specs/context/tech-stack.md")

2. Extract LOCKED technologies from tech-stack.md:
   - Parse sections for "LOCKED" markers
   - Build approved_technologies list
   - Note PROHIBITED technologies

3. Extract technology mentions from tech_spec_content:
   - Scan for framework names (React, Vue, Angular, etc.)
   - Scan for language names (Python, TypeScript, C#, etc.)
   - Scan for database names (PostgreSQL, MongoDB, Redis, etc.)
   - Scan for library mentions

4. Compare:
   FOR each technology in tech_spec:
     IF technology in PROHIBITED list:
       violations.append({
         type: "PROHIBITED_TECHNOLOGY",
         technology: technology,
         severity: "CRITICAL",
         source: "tech-stack.md",
         remediation: "Remove or replace with approved alternative"
       })
     ELIF technology NOT in approved_technologies AND not generic:
       violations.append({
         type: "UNAPPROVED_TECHNOLOGY",
         technology: technology,
         severity: "HIGH",
         source: "tech-stack.md",
         remediation: "Add to tech-stack.md (requires ADR) or remove"
       })

5. Return violations list
```

---

### 2. validate_file_paths(tech_spec_content)

**Purpose:** Validate file paths in technical specification against source-tree.md

**Severity:** HIGH

**Input:** Technical specification text from story

**Process:**
```
1. Read source-tree.md:
   source_tree = Read(file_path="devforgeai/specs/context/source-tree.md")

2. Extract allowed directory patterns from source-tree.md:
   - Parse directory structure tree
   - Build allowed_paths list (e.g., "devforgeai/specs/Stories/", ".claude/skills/")
   - Note FORBIDDEN paths

3. Extract file paths from tech_spec_content:
   - Scan for patterns like "src/...", "devforgeai/...", etc.
   - Scan for file_path fields in YAML
   - Extract proposed implementation locations

4. Compare:
   FOR each path in proposed_paths:
     IF path matches FORBIDDEN pattern:
       violations.append({
         type: "FORBIDDEN_PATH",
         path: path,
         severity: "CRITICAL",
         source: "source-tree.md",
         remediation: f"Use approved path from source-tree.md"
       })
     ELIF path NOT in allowed_paths structure:
       # Find closest match
       suggested_path = find_closest_allowed_path(path)
       violations.append({
         type: "INVALID_PATH",
         path: path,
         suggested: suggested_path,
         severity: "HIGH",
         source: "source-tree.md",
         remediation: f"Change to: {suggested_path}"
       })

5. Return violations list
```

**Special Case - Story Output Directory:**
```
# CRITICAL: Stories MUST go in devforgeai/specs/Stories/
IF story_output_path != "devforgeai/specs/Stories/":
    violations.append({
      type: "WRONG_STORY_DIRECTORY",
      path: story_output_path,
      correct: "devforgeai/specs/Stories/",
      severity: "CRITICAL",
      remediation: "Stories MUST be in devforgeai/specs/Stories/"
    })
```

---

### 3. validate_dependencies(dependencies_section)

**Purpose:** Validate package dependencies against dependencies.md

**Severity:** HIGH

**Input:** Dependencies section from story technical specification

**Process:**
```
1. Read dependencies.md:
   deps = Read(file_path="devforgeai/specs/context/dependencies.md")

2. Extract approved packages from dependencies.md:
   - Parse LOCKED packages with versions
   - Note FORBIDDEN alternatives

3. Extract proposed packages from dependencies_section:
   - Scan for package names (npm, pip, nuget patterns)
   - Extract version constraints if present

4. Compare:
   FOR each package in proposed_packages:
     IF package in FORBIDDEN list:
       approved_alternative = get_approved_alternative(package)
       violations.append({
         type: "FORBIDDEN_DEPENDENCY",
         package: package,
         alternative: approved_alternative,
         severity: "HIGH",
         source: "dependencies.md",
         remediation: f"Use {approved_alternative} instead"
       })
     ELIF package NOT in approved_packages:
       violations.append({
         type: "UNAPPROVED_DEPENDENCY",
         package: package,
         severity: "HIGH",
         source: "dependencies.md",
         remediation: "Add to dependencies.md (requires ADR) or remove"
       })

5. Return violations list
```

---

### 4. validate_coverage_thresholds(dod_content, file_paths)

**Purpose:** Validate test coverage thresholds match architectural layer

**Severity:** MEDIUM

**Input:** Definition of Done content, file paths from technical specification

**Process:**
```
1. Read coding-standards.md:
   standards = Read(file_path="devforgeai/specs/context/coding-standards.md")

2. Extract coverage thresholds by layer:
   thresholds = {
     "business_logic": 95,  # Core domain, services
     "application": 85,      # Controllers, handlers
     "infrastructure": 80    # Repositories, external integrations
   }

3. Determine layer from file_paths:
   FOR each path in file_paths:
     layer = classify_layer(path)
     # src/domain/, src/services/ → business_logic
     # src/controllers/, src/handlers/ → application
     # src/repositories/, src/integrations/ → infrastructure

4. Extract coverage mentioned in DoD:
   - Scan for patterns like "95% coverage", "85%", etc.
   - Extract stated threshold

5. Compare:
   IF stated_threshold != thresholds[layer]:
     violations.append({
       type: "INCORRECT_COVERAGE_THRESHOLD",
       stated: stated_threshold,
       correct: thresholds[layer],
       layer: layer,
       severity: "MEDIUM",
       source: "coding-standards.md",
       remediation: f"Update coverage to {thresholds[layer]}% for {layer} layer"
     })

6. Return violations list
```

---

### 5. validate_architecture(tech_spec_content)

**Purpose:** Validate technical specification against architecture constraints

**Severity:** HIGH

**Input:** Technical specification text from story

**Process:**
```
1. Read architecture-constraints.md:
   arch = Read(file_path="devforgeai/specs/context/architecture-constraints.md")

2. Extract layer dependency rules:
   - Parse dependency matrix (which layers can call which)
   - Extract FORBIDDEN cross-layer dependencies

3. Analyze tech_spec for layer violations:
   - Check if proposed design has controllers calling repositories directly
   - Check if infrastructure depends on domain
   - Check for circular dependencies

4. Compare:
   FOR each proposed_dependency in tech_spec:
     IF violates_layer_rules(proposed_dependency):
       violations.append({
         type: "LAYER_VIOLATION",
         from_layer: source_layer,
         to_layer: target_layer,
         severity: "HIGH",
         source: "architecture-constraints.md",
         remediation: "Insert application layer between {source} and {target}"
       })

5. Return violations list
```

---

### 6. validate_anti_patterns(tech_spec_content)

**Purpose:** Detect forbidden patterns in technical specification

**Severity:** CRITICAL

**Input:** Technical specification text from story

**Process:**
```
1. Read anti-patterns.md:
   patterns = Read(file_path="devforgeai/specs/context/anti-patterns.md")

2. Extract forbidden patterns:
   anti_patterns = [
     "God Object",           # Classes >500 lines or >20 methods
     "SQL concatenation",    # String-based SQL queries
     "Hardcoded secrets",    # API keys, passwords in code
     "Bash for file ops",    # Using cat/echo instead of Read/Write
     "Monolithic skills",    # Single skill doing everything
     "Direct instantiation"  # Not using dependency injection
   ]

3. Scan tech_spec_content for pattern matches:
   FOR each pattern in anti_patterns:
     IF pattern_detected(tech_spec_content, pattern):
       violations.append({
         type: "ANTI_PATTERN_DETECTED",
         pattern: pattern,
         severity: "CRITICAL",
         source: "anti-patterns.md",
         remediation: get_remediation_for_pattern(pattern)
       })

4. Return violations list
```

---

## Resolution Protocol

**For each violation found:**

```
1. Categorize by severity:
   CRITICAL > HIGH > MEDIUM > LOW

2. If CRITICAL or HIGH violations exist:
   HALT workflow

   Use AskUserQuestion:
   Question: "Context validation found {count} violation(s). How to proceed?"
   Header: "Validation"
   Options:
     - "Fix in story"
       Description: "I'll provide the correct value"
     - "Update context file"
       Description: "Requires ADR - the constraint should change"
     - "Defer to manual review"
       Description: "Flag for later review, proceed with warning"

   IF user selects "Fix in story":
     AskUserQuestion for correct value
     Apply fix to story content
     Re-validate

   IF user selects "Update context file":
     HALT: "Create ADR first, then update context file, then retry"

   IF user selects "Defer to manual review":
     Add warning note to story
     Log deferral
     Continue with warning

3. If only MEDIUM or LOW violations:
   Display warnings
   Embed note in story: "⚠️ {count} validation warnings - see below"
   Continue workflow
```

---

## Validation Report Format

**Generate report after all validation functions run:**

```markdown
## Context Validation Report

**Story:** {story_id}
**Validated At:** {timestamp}
**Context Files Checked:** {count}/6

### Summary

| Severity | Count | Blocking |
|----------|-------|----------|
| CRITICAL | {n}   | Yes      |
| HIGH     | {n}   | Yes      |
| MEDIUM   | {n}   | No       |
| LOW      | {n}   | No       |

**Status:** {COMPLIANT | FAILED | WARNINGS}

### Violations

{for each violation}
#### {severity}: {type}
- **Location:** {where in story}
- **Issue:** {description}
- **Context File:** {source} (line {line_number})
- **Remediation:** {how to fix}
{/for}

### Compliance Score

{percentage}% compliant ({passed}/{total} checks)
```

---

## Integration Points

### Phase 3.6: Technology Validation

**Trigger:** After technical specification generated
**Functions called:** `validate_technologies()`, `validate_dependencies()`, `validate_architecture()`
**On violation:** HALT + AskUserQuestion

### Phase 5.1: Directory Validation

**Trigger:** Before story file write
**Functions called:** `validate_file_paths()` (specifically story output directory)
**On violation:** Auto-correct to `devforgeai/specs/Stories/`

### Phase 7.7: Comprehensive Validation

**Trigger:** Final validation before completion
**Functions called:** All 6 validation functions
**On violation:** HALT + AskUserQuestion for CRITICAL/HIGH, warn for MEDIUM/LOW

### /validate-stories Command

**Trigger:** User invokes command on existing stories
**Functions called:** All 6 validation functions
**On violation:** Report + AskUserQuestion for fixes

---

## Error Handling

**Context file missing:**
```
IF Read() fails for context file:
  Log: "Warning: {file} not found, skipping {validation_type}"
  SKIP that validation function
  Continue with remaining validations
```

**Malformed context file:**
```
IF parsing fails:
  Log: "Warning: {file} has invalid format, skipping validation"
  SKIP that validation function
  Continue with remaining validations
```

**All context files missing:**
```
IF no context files exist:
  Greenfield mode (documented above)
  SKIP all validation
  Return success with greenfield flag
```

---

## Performance Considerations

**Load context files in parallel:**
```
# Parallel reads for efficiency
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/source-tree.md")
Read(file_path="devforgeai/specs/context/dependencies.md")
Read(file_path="devforgeai/specs/context/coding-standards.md")
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
Read(file_path="devforgeai/specs/context/anti-patterns.md")
```

**Cache context files within skill execution:**
- Load once at start of validation
- Reuse across all validation functions
- Don't reload for each function call
