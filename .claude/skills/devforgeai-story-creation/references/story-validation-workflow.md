# Phase 7: Story Self-Validation Workflow

Execute quality validation checks and self-healing procedures.

## Overview

This phase validates the generated story against quality standards and automatically corrects issues when possible.

---

## Prerequisites

**Load validation checklists:**
```
Read(file_path=".claude/skills/devforgeai-story-creation/references/validation-checklists.md")
```

This reference provides comprehensive validation logic for story quality assurance.

---

## Step 7.1: Validate YAML Frontmatter

**Objective:** Ensure frontmatter complete and valid

```
Read created story file (first 15 lines for frontmatter)

Required fields:
- [ ] id: Matches filename
- [ ] title: Present and descriptive (5-80 chars)
- [ ] epic: Present (EPIC-XXX or null)
- [ ] sprint: Present (SPRINT-XXX or "Backlog")
- [ ] status: Set to "Backlog"
- [ ] priority: One of Critical/High/Medium/Low
- [ ] points: Fibonacci number (1/2/3/5/8/13)
- [ ] created: Today's date (YYYY-MM-DD format)
- [ ] updated: Same as created
- [ ] assigned_to: null (unassigned initially)

If validation fails:
    # Self-healing: Regenerate frontmatter
    # Edit story file to fix
    # Retry validation
```

---

## Step 7.2: Validate User Story Format

**Objective:** Verify user story follows As a/I want/So that format

```
Read user story section

Validation:
- [ ] Starts with "As a"
- [ ] Contains "I want"
- [ ] Contains "So that"
- [ ] Role is specific (not generic "user")
- [ ] Action is clear and unambiguous
- [ ] Benefit articulates business value

If validation fails:
    # Extract role, action, benefit from subagent output
    # Regenerate user story section
    # Edit story file
    # Retry validation
```

---

## Step 7.3: Validate Acceptance Criteria

**Objective:** Ensure AC meet quality standards

```
Read acceptance criteria section

Validation:
- [ ] Minimum 3 acceptance criteria
- [ ] Each has unique title/number (AC1, AC2, AC3, ...)
- [ ] Each follows Given/When/Then structure
- [ ] At least 1 happy path scenario
- [ ] At least 1 error/edge case scenario
- [ ] All criteria are testable (can write automated test)
- [ ] No ambiguous language ("should", "might", "probably")
- [ ] No vague terms ("fast", "easy", "intuitive" without definition)

If validation fails:
    # Identify which criteria are invalid
    # Regenerate specific criteria
    # Use acceptance-criteria-patterns.md for templates
    # Edit story file
    # Retry validation
```

---

## Step 7.4: Validate Technical Specification

**Objective:** Verify technical spec completeness

```
Validation:

If API contracts present:
- [ ] HTTP method specified (GET/POST/PUT/DELETE/PATCH)
- [ ] Endpoint path follows RESTful conventions
- [ ] Request schema includes all required fields
- [ ] Success response (200/201) schema defined
- [ ] Error responses (400/401/403/404/500) documented
- [ ] Authentication requirements specified

Data models:
- [ ] At least 1 entity defined
- [ ] Each entity has attributes with types
- [ ] Constraints specified (Required, Unique, Length, Format)
- [ ] Relationships documented (if applicable)
- [ ] Primary key identified

Business rules:
- [ ] At least 1 rule documented (if business logic exists)
- [ ] Rules are specific (not generic)
- [ ] Validation logic clear

Dependencies:
- [ ] All external dependencies identified
- [ ] Integration methods specified
- [ ] Fallback behavior defined

If validation fails:
    # Identify gaps
    # Regenerate missing sections
    # Use technical-specification-guide.md for templates
    # Edit story file
    # Retry validation
```

---

## Step 7.5: Validate Non-Functional Requirements

**Objective:** Ensure NFRs are measurable

```
Validation:

- [ ] Performance targets quantified (e.g., "<500ms response time")
- [ ] Security requirements specific (e.g., "OAuth2 with JWT", not "secure")
- [ ] Usability requirements clear (e.g., "Max 3 clicks to checkout")
- [ ] Scalability targets measurable (e.g., "Support 10k concurrent users")
- [ ] No vague terms without metrics

If validation fails:
    # Identify vague NFRs
    # Use AskUserQuestion to quantify
    # Edit story file
    # Retry validation
```

---

## Step 7.6: Validation Success Criteria

**Objective:** Confirm all validations passed before proceeding

**Before proceeding to Phase 8:**

```
All validations must pass:
- ✅ YAML frontmatter complete and valid
- ✅ User story follows format
- ✅ 3+ testable acceptance criteria
- ✅ Technical specification complete (if applicable)
- ✅ UI specification complete (if applicable)
- ✅ NFRs measurable (not vague)
- ✅ Edge cases documented
- ✅ Definition of Done present
- ✅ File exists on disk
- ✅ AC-TechSpec traceability validated (Step 7.6.6)

If any CRITICAL failures (missing sections, invalid frontmatter):
    # Self-healing: Regenerate and retry (max 2 attempts)
    # If still failing: Report to user with specific issues

If all validations pass:
    ✅ Proceed to Step 7.6.5 (Citation Compliance Validation)
```

---

## Step 7.6.5: Citation Compliance Validation

### Citation Compliance Validation

**Objective:** Validate that all technology and architecture claims in the story follow the Read-Quote-Cite-Verify protocol, ensuring modification claims are grounded in verified evidence.

**Reference:** `.claude/rules/core/citation-requirements.md`

**Purpose:** This validation ensures that technical specifications making claims about existing code or files (e.g., "modifies Component X", "updates File Y") have proper citation evidence. Stories without grounded citations risk implementing changes to non-existent components or incorrect file locations.

**Defense in Depth:**
- **Layer 1 (Phase 3):** Evidence-Verification Gate proactively gathers evidence
- **Layer 2 (Phase 7):** Citation Compliance Validation (this step) validates evidence exists before story creation

```
1. Check if story contains modification claims:

   modification_indicators = [
     "modifies", "updates", "changes", "extends", "enhances",
     "adds to", "removes from", "refactors", "integrates with",
     "violation", "replace", "remove"
   ]

   story_content = Read(story_file_path)

   has_modification_claims = any(
     indicator in story_content.lower()
     for indicator in modification_indicators
   )

   IF NOT has_modification_claims:
     Display: "ℹ️ Documentation-only story: Citation compliance validation skipped"
     Log: "Citation compliance validation skipped - no modification claims"
     SKIP to Step 7.7
     RETURN { documentation_only: true, citation_compliant: true }
```

```
2. Validate citation structure for modification claims:

   violations = []

   FOR each component in tech_spec.components:

     # **Item 1:** Check for verified_violations when claims exist
     IF component.description contains ["violation", "replace", "remove", "refactor"]:
       IF component does NOT have verified_violations:
         violations.append({
           "item": 1,
           "type": "MISSING_VERIFIED_VIOLATIONS",
           "component": component.name,
           "reason": "Modification claim without verified_violations section",
           "evidence": f"Component '{component.name}' claims modification but lacks grounding"
         })

     # **Item 2:** Check line number format is array of integers
     IF component.verified_violations exists:
       IF lines field exists AND is NOT array of integers:
         violations.append({
           "item": 2,
           "type": "INVALID_LINE_FORMAT",
           "component": component.name,
           "reason": "Line numbers must be array of integers [N, M, O], not generic ranges",
           "evidence": f"Found: {type(component.lines).__name__}, expected: list of positive integers"
         })
       IF lines contains generic values like "around", "approx", "~":
         violations.append({
           "item": 2,
           "type": "GENERIC_LINE_NUMBERS",
           "component": component.name,
           "reason": "Generic line numbers not allowed",
           "evidence": f"Line reference contains vague terms"
         })

     # **Item 3:** Check for generic descriptions
     IF component.description matches /[Rr]emove.*commands?$/ without specific count and lines:
       violations.append({
         "item": 3,
         "type": "GENERIC_DESCRIPTION",
         "component": component.name,
         "reason": "Description uses vague terms without specific counts or line numbers",
         "evidence": f"Generic language detected: '{component.description}' - must include specific count and line numbers"
       })

     # **Item 4:** Check file paths exist (use cached results from Phase 3)
     IF verified_violations.file NOT in validated_files_cache:
       violations.append({
         "item": 4,
         "type": "UNVALIDATED_FILE_PATH",
         "component": component.name,
         "reason": "File path not validated in Phase 3 evidence collection",
         "evidence": f"Path '{verified_violations.file}' not found in phase3_validated_files cache"
       })

     # **Item 5:** Check for placeholders
     IF verified_violations contains ["TBD", "TODO", "PLACEHOLDER", "N/A"]:
       violations.append({
         "item": 5,
         "type": "PLACEHOLDER_VALUE",
         "component": component.name,
         "reason": "Placeholder values detected - all values must be concrete",
         "evidence": f"Found placeholder in verified_violations section"
       })
```

```
3. Handle Citation Compliance Violation Detected:

   IF len(violations) > 0:
     HALT workflow

     Display: f"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Citation Compliance Violation Detected
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

❌ CRITICAL: Story fails Citation Compliance validation

Violations found: {len(violations)}
     """

     FOR violation in violations:
       Display: f"""
**Violation:** Item {violation['item']} - {violation['type']}

**Component:** {violation['component']}

**Reason:** {violation['reason']}

**Evidence:** {violation['evidence']}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
       """

     # Provide fix instructions
     Display: """
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Fix Required
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

| Violation | Fix Instruction |
|-----------|-----------------|
| Item 1 (Missing verified_violations) | Add verified_violations section to component. Read target file and gather line numbers. |
| Item 2 (Invalid line format) | Convert lines to array of integers: lines: [469, 598, 599] |
| Item 3 (Generic description) | Add specific count and line numbers: 'Remove 3 Bash mkdir commands (lines 469, 598, 599)' |
| Item 4 (Invalid file path) | Verify file path exists. Check for typos or outdated references. |
| Item 5 (Placeholder values) | Replace TBD/TODO with actual values from target file verification. |

**Reference:** .claude/rules/core/citation-requirements.md (Read-Quote-Cite-Verify protocol)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Story file NOT created. Address violation and retry.
     """

     # Re-run validation after fixes (max 2 retries)
     GOTO Step 7.6.5 (max 2 retry attempts)
```

```
4. If validation passes:
   Display: f"""
   ✅ Citation Compliance Validation PASSED

   Components validated: {len(tech_spec.components)}
   Modification claims verified: {count_modification_claims}
   All citations properly grounded per Read-Quote-Cite-Verify protocol.
   """

   PROCEED to Step 7.7 (Context File Compliance)
```

**Validation Checklist:**

| Item | Check | Severity |
|------|-------|----------|
| **Item 1** | All components with modification claims have `verified_violations` section | CRITICAL |
| **Item 2** | Line numbers in format `lines: [N, M, O]` (array of integers) | HIGH |
| **Item 3** | No generic descriptions - must have specific count and line numbers | HIGH |
| **Item 4** | File paths validated (from Phase 3 cache) | CRITICAL |
| **Item 5** | No placeholder values (TBD, TODO, PLACEHOLDER) | CRITICAL |

**Exit Criteria:** All 5 validation items pass for every component with modification claims

---

## Step 7.6.6: AC-TechSpec Traceability Validation

**Objective:** Verify bidirectional traceability between Acceptance Criteria and Technical Specification components. Ensures every AC has at least one implementing component, and every `implements_ac` reference points to a valid AC.

**Reference:** STORY-284, EPIC-046 (AC Compliance Verification System)

**Purpose:** As part of Phase 7 Self-Validation, this traceability check ensures complete coverage between what we promise (Acceptance Criteria) and what we build (Technical Specification). Orphaned ACs indicate untested requirements. Invalid AC references indicate specification errors that would cause verification failures.

**Business Rules:**
- **BR-001:** Orphaned AC (AC without any `implements_ac` reference) = WARNING
  - Allows incremental adoption of traceability
  - Does not block story creation
- **BR-002:** Invalid AC reference (implements_ac points to non-existent AC) = ERROR
  - Data integrity issue
  - Blocks story creation until fixed

---

### Traceability Validation Workflow

```
1. Check if traceability validation is applicable:

   # Read story file content
   story_content = Read(story_file_path)

   # Check for Acceptance Criteria section
   has_ac_section = "## Acceptance Criteria" in story_content

   # Check for Technical Specification with implements_ac fields
   has_techspec = "technical_specification:" in story_content
   has_implements_ac = "implements_ac:" in story_content

   IF NOT has_ac_section OR NOT has_techspec:
     Display: "ℹ️ Traceability validation skipped: Missing AC or Technical Specification section"
     SKIP to Step 7.7
     RETURN { skipped: true, reason: "missing_sections" }

   IF NOT has_implements_ac:
     Display: "⚠️ Traceability validation: No implements_ac fields found in Technical Specification"
     Display: "   Consider adding implements_ac to components for full traceability"
     SKIP to Step 7.7 (non-blocking warning)
     RETURN { skipped: true, reason: "no_implements_ac_fields" }
```

```
2. Extract all AC IDs from Acceptance Criteria section:

   # Pattern: ### AC#N: or ### AC#N (with or without colon/title)
   ac_pattern = r'###\s+AC#(\d+)'

   # Find all AC IDs in the Acceptance Criteria section
   ac_section_start = story_content.find("## Acceptance Criteria")
   ac_section_end = story_content.find("## Technical Specification")

   IF ac_section_end == -1:
     ac_section_end = len(story_content)

   ac_section = story_content[ac_section_start:ac_section_end]

   # Extract AC IDs (e.g., ["AC#1", "AC#2", "AC#3", "AC#4"])
   defined_ac_ids = set()
   FOR match in regex.findall(ac_pattern, ac_section):
     defined_ac_ids.add(f"AC#{match}")

   Display: f"Found {len(defined_ac_ids)} AC definitions: {sorted(defined_ac_ids)}"
```

```
3. Extract all implements_ac references from Technical Specification:

   # Pattern: implements_ac: ["AC#1", "AC#2"] or implements_ac: ["AC#1"]
   implements_pattern = r'implements_ac:\s*\[([^\]]+)\]'

   # Find all implements_ac arrays in the Technical Specification
   techspec_start = story_content.find("## Technical Specification")
   techspec_section = story_content[techspec_start:]

   # Extract referenced AC IDs
   referenced_ac_ids = set()
   FOR match in regex.findall(implements_pattern, techspec_section):
     # Parse array contents: "AC#1", "AC#2" -> ["AC#1", "AC#2"]
     refs = [ref.strip().strip('"').strip("'") for ref in match.split(",")]
     FOR ref in refs:
       referenced_ac_ids.add(ref)

   Display: f"Found {len(referenced_ac_ids)} AC references in implements_ac: {sorted(referenced_ac_ids)}"
```

```
4. Perform set comparison for orphan detection:

   # Find orphaned ACs (ACs with no implementing component)
   orphaned_acs = defined_ac_ids - referenced_ac_ids

   # Find invalid references (implements_ac pointing to non-existent AC)
   invalid_references = referenced_ac_ids - defined_ac_ids

   # Prepare validation results
   traceability_result = {
     "defined_acs": sorted(defined_ac_ids),
     "referenced_acs": sorted(referenced_ac_ids),
     "orphaned_acs": sorted(orphaned_acs),
     "invalid_references": sorted(invalid_references),
     "coverage_percent": (len(referenced_ac_ids & defined_ac_ids) / len(defined_ac_ids) * 100) if defined_ac_ids else 100,
     "warnings": [],
     "errors": []
   }
```

```
5. Flag orphaned entities per business rules:

   # BR-001: Orphaned ACs are WARNING (non-blocking)
   IF len(orphaned_acs) > 0:
     FOR ac_id in sorted(orphaned_acs):
       traceability_result["warnings"].append({
         "type": "ORPHANED_AC",
         "severity": "WARNING",
         "ac_id": ac_id,
         "message": f"AC '{ac_id}' has no implementing component in Technical Specification",
         "recommendation": f"Add 'implements_ac: [\"{ac_id}\"]' to a relevant component"
       })

   # BR-002: Invalid references are ERROR (blocking)
   IF len(invalid_references) > 0:
     FOR ref in sorted(invalid_references):
       traceability_result["errors"].append({
         "type": "INVALID_AC_REFERENCE",
         "severity": "ERROR",
         "reference": ref,
         "message": f"implements_ac references '{ref}' which does not exist in Acceptance Criteria",
         "recommendation": f"Fix reference to valid AC ID or add missing AC definition"
       })
```

```
6. Handle validation results:

   has_errors = len(traceability_result["errors"]) > 0
   has_warnings = len(traceability_result["warnings"]) > 0

   IF has_errors:
     HALT workflow

     Display: f"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  AC-TechSpec Traceability Validation FAILED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

❌ CRITICAL: Invalid AC references detected (BR-002)

Errors found: {len(traceability_result["errors"])}
     """

     FOR error in traceability_result["errors"]:
       Display: f"""
**Error:** {error["type"]}

**Reference:** {error["reference"]}

**Message:** {error["message"]}

**Fix:** {error["recommendation"]}
       """

     Display: """
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Fix Required
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Options:
1. Fix the implements_ac reference to point to a valid AC ID
2. Add the missing AC definition to Acceptance Criteria section

Story file NOT created. Address errors and retry.
     """

     # Re-run validation after fixes (max 2 retries)
     GOTO Step 7.6.6 (max 2 retry attempts)
```

```
7. Handle warnings (non-blocking per BR-001):

   ELIF has_warnings:
     Display: f"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  AC-TechSpec Traceability Validation PASSED with Warnings
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️ WARNING: Orphaned ACs detected (BR-001)

Orphaned ACs: {len(traceability_result["warnings"])}
Coverage: {traceability_result["coverage_percent"]:.0f}% of ACs have implementing components
     """

     FOR warning in traceability_result["warnings"]:
       Display: f"""
**Warning:** {warning["type"]}
**AC:** {warning["ac_id"]}
**Message:** {warning["message"]}
**Recommendation:** {warning["recommendation"]}
       """

     Display: """
Note: Orphaned ACs are non-blocking to allow incremental adoption.
Consider adding traceability for complete coverage.

Proceeding to Step 7.7 with warnings noted.
     """

     # Embed warning note in story file
     Edit story file to add:
     """
     <!-- Traceability Warnings
     Orphaned ACs (no implementing component):
     {format_orphaned_acs(orphaned_acs)}
     -->
     """

     PROCEED to Step 7.7
```

```
8. If validation passes completely:

   ELSE:
     Display: f"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  AC-TechSpec Traceability Validation PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Full bidirectional traceability verified

ACs defined: {len(defined_ac_ids)}
ACs with implementing components: {len(referenced_ac_ids & defined_ac_ids)}
Coverage: 100%

All acceptance criteria have implementing components.
All implements_ac references point to valid ACs.

Proceeding to Step 7.7 (Context File Compliance).
     """

     PROCEED to Step 7.7
```

---

### Traceability Validation Checklist

| Check | Description | Severity |
|-------|-------------|----------|
| **AC Extraction** | All AC#N headers extracted from Acceptance Criteria section | INFO |
| **Reference Extraction** | All implements_ac arrays parsed from Technical Specification | INFO |
| **Coverage Validation** | Every AC has at least one implementing component | WARNING (BR-001) |
| **Reference Validation** | Every implements_ac reference points to existing AC | ERROR (BR-002) |

**Exit Criteria:** No ERROR-level violations (invalid references). WARNING-level violations (orphaned ACs) are noted but do not block.

---

## Step 7.7: Context File Compliance Validation

**Objective:** Final validation that story adheres to all constitutional context files

**Reference:** `.claude/skills/devforgeai-story-creation/references/context-validation.md`

**Purpose:** This is the comprehensive final gate that validates the complete story against all 6 context files before story creation completes.

**Workflow:**

```
1. Load context files (if exist):
   context_dir = "devforgeai/specs/context/"
   context_files = Glob(pattern=f"{context_dir}*.md")

   IF len(context_files) == 0:
     Display: "ℹ️ Greenfield mode: context compliance validation skipped"
     SKIP to Phase 8
     RETURN { greenfield: true, compliant: true }
```

```
2. Load all available context files in PARALLEL:
   Read(file_path="devforgeai/specs/context/tech-stack.md")
   Read(file_path="devforgeai/specs/context/source-tree.md")
   Read(file_path="devforgeai/specs/context/dependencies.md")
   Read(file_path="devforgeai/specs/context/coding-standards.md")
   Read(file_path="devforgeai/specs/context/architecture-constraints.md")
   Read(file_path="devforgeai/specs/context/anti-patterns.md")
```

```
3. For each context file that exists, call validation functions:

   violations = []

   # From context-validation.md:
   IF tech_stack_exists:
     violations.extend(validate_technologies(tech_spec_content))

   IF source_tree_exists:
     violations.extend(validate_file_paths(tech_spec_content))

   IF dependencies_exists:
     violations.extend(validate_dependencies(dependencies_section))

   IF coding_standards_exists:
     violations.extend(validate_coverage_thresholds(dod_content, file_paths))

   IF architecture_exists:
     violations.extend(validate_architecture(tech_spec_content))

   IF anti_patterns_exists:
     violations.extend(validate_anti_patterns(tech_spec_content))
```

```
4. Generate compliance report:

   context_compliance = {
     "validated_at": datetime.now().isoformat(),
     "context_files_checked": count_files_checked,
     "violations": {
       "CRITICAL": [v for v in violations if v.severity == "CRITICAL"],
       "HIGH": [v for v in violations if v.severity == "HIGH"],
       "MEDIUM": [v for v in violations if v.severity == "MEDIUM"],
       "LOW": [v for v in violations if v.severity == "LOW"]
     },
     "total_violations": len(violations),
     "status": "COMPLIANT" if len(critical + high) == 0 else "FAILED"
   }
```

```
5. Handle violations by severity:

   IF CRITICAL or HIGH violations found:
     HALT workflow

     Display: f"""
     ❌ Context Compliance Validation FAILED

     CRITICAL Issues: {len(critical)}
     HIGH Issues: {len(high)}

     {format_violations(critical + high)}

     Story cannot be completed until violations are resolved.
     """

     FOR each violation in (critical + high):
       AskUserQuestion:
         Question: f"How to resolve: {violation.type} - {violation.description}?"
         Header: "Fix needed"
         Options:
           - "Fix in story"
             Description: "I'll provide the correct value"
           - "Update context file"
             Description: "Requires ADR - constraint should change"
           - "Defer to manual review"
             Description: "Flag for later, proceed with warning"

       Apply resolution based on user choice

     # Re-run validation after fixes
     GOTO Step 7.7 (max 2 retry attempts)
```

```
6. If only MEDIUM or LOW violations:
   Display: f"""
   ⚠️ Context Compliance Validation PASSED with warnings

   MEDIUM Issues: {len(medium)}
   LOW Issues: {len(low)}

   {format_violations(medium + low)}

   Proceeding to Phase 8 with warnings noted.
   """

   # Embed warning note in story file
   Edit story file to add:
   """
   <!-- Context Validation Warnings
   {format_violations(medium + low)}
   -->
   """
```

```
7. If no violations:
   Display: f"""
   ✅ Context Compliance Validation PASSED

   Context files checked: {count}/6
   Violations found: 0
   Status: COMPLIANT

   Story is fully compliant with all constitutional context files.
   """
```

**Validation Summary Table:**

| Context File | Validation Checks |
|--------------|-------------------|
| tech-stack.md | All technologies in tech spec are LOCKED or approved |
| source-tree.md | File paths in tech spec match allowed directories |
| dependencies.md | All packages in Dependencies section are approved |
| coding-standards.md | Coverage thresholds match layer (95%/85%/80%) |
| architecture-constraints.md | No cross-layer violations in design |
| anti-patterns.md | No forbidden patterns in technical spec |

**Exit Criteria:** All CRITICAL and HIGH violations resolved

**Output:**

```yaml
context_compliance:
  validated_at: "2025-12-23T14:30:00Z"
  context_files_checked: 6
  violations_found: 0
  status: "COMPLIANT"
```

---

## Reference Files Used

**Phase 7 references:**
- `validation-checklists.md` (1,038 lines) - Comprehensive quality checks
- `story-structure-guide.md` (662 lines) - Frontmatter and section requirements
- `acceptance-criteria-patterns.md` (1,259 lines) - AC format validation
- `technical-specification-guide.md` (1,269 lines) - Tech spec completeness

---

## Output

**Phase 7 produces:**
- ✅ Story validated against all quality standards
- ✅ Auto-corrected issues (if self-healing applied)
- ✅ Confirmed ready for implementation

---

## Error Handling

**Error 1: CRITICAL validation failure (max retries exceeded)**
- **Detection:** Self-healing attempted 2 times, still failing
- **Recovery:** Report to user with specific validation failures, request manual intervention

**Error 2: Missing required sections**
- **Detection:** Sections expected from previous phases not found in story file
- **Recovery:** Re-execute relevant phase (2, 3, or 4), regenerate section, retry validation

**Error 3: Vague NFRs cannot be quantified automatically**
- **Detection:** NFR contains "fast", "scalable" without metrics, cannot infer numbers
- **Recovery:** Use AskUserQuestion to get specific targets from user

See `error-handling.md` for comprehensive error recovery procedures.

---

## Next Phase

**After Phase 7 completes →** Phase 8: Completion Report

Load `completion-report.md` for Phase 8 workflow.
