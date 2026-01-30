# Phase 5: Story File Creation

Construct complete story document from collected information and write to disk.

## Overview

This phase assembles all information gathered from Phases 1-4 into a complete story document following DevForgeAI structure standards.

---

## Step 5.0: Output Directory Validation

**Objective:** Validate story output directory against source-tree.md before file write

**Reference:** `.claude/skills/devforgeai-story-creation/references/context-validation.md`

**Trigger:** Before any Write tool invocation

**Workflow:**

```
1. Check for source-tree.md:
   source_tree_exists = file_exists("devforgeai/specs/context/source-tree.md")

   IF source_tree_exists:
     source_tree = Read(file_path="devforgeai/specs/context/source-tree.md")

     # Extract canonical story directory from source-tree.md
     # Look for pattern: "Stories/" or "specs/Stories/"
     story_dir_pattern = extract_story_directory(source_tree)

     IF story_dir_pattern found:
       OUTPUT_DIR = story_dir_pattern  # e.g., "devforgeai/specs/Stories/"
     ELSE:
       OUTPUT_DIR = "devforgeai/specs/Stories/"  # Framework default
```

```
2. If source-tree.md not found (greenfield):
   OUTPUT_DIR = "devforgeai/specs/Stories/"  # Framework standard default
   Display: "ℹ️ Using default story directory: devforgeai/specs/Stories/"
```

```
3. Validate directory exists or create:
   Bash(command="mkdir -p {OUTPUT_DIR}")
```

```
4. CRITICAL validation - prevent wrong directory:
   FORBIDDEN_PATHS = [
     "devforgeai/stories/",      # Wrong: lowercase, no specs/
     ".ai_docs/stories/",        # Wrong: old structure
     "stories/",                 # Wrong: root level
     ".claude/stories/"          # Wrong: in .claude/
   ]

   FOR each forbidden in FORBIDDEN_PATHS:
     IF OUTPUT_DIR matches forbidden:
       HALT: """
       ❌ CRITICAL: Invalid story directory detected

       Attempted: {OUTPUT_DIR}
       Required:  devforgeai/specs/Stories/

       Stories MUST be in devforgeai/specs/Stories/ per source-tree.md
       """
       OUTPUT_DIR = "devforgeai/specs/Stories/"  # Force correct path
```

```
5. Set OUTPUT_DIR for Step 5.4 (Write Story File):
   story_output_path = f"{OUTPUT_DIR}{story_id}-{slug}.story.md"

   Display: f"✓ Story will be written to: {story_output_path}"
```

**Output:** Validated OUTPUT_DIR for use in Step 5.4

---

## Step 5.1: Load Story Template

**Objective:** Load base template structure

**Read template from assets:**
```
template = Read(file_path=".claude/skills/devforgeai-story-creation/assets/templates/story-template.md")
```

**Load structure guide:**
```
Read(file_path=".claude/skills/devforgeai-story-creation/references/story-structure-guide.md")
```

This reference provides:
- YAML frontmatter field descriptions
- Required vs optional sections
- Section ordering requirements
- Markdown formatting standards

---

## Step 5.2: Construct YAML Frontmatter

**Objective:** Build frontmatter with all metadata from Phase 1

**Build frontmatter with all metadata:**

```yaml
---
id: {story_id}                     # STORY-001, STORY-002, etc.
title: {brief_title}               # 5-10 words, descriptive
epic: {epic_id or null}            # EPIC-001 or null
sprint: {sprint_id or "Backlog"}   # SPRINT-001 or "Backlog"
status: Backlog                    # Always "Backlog" initially
priority: {Critical|High|Medium|Low}
points: {1|2|3|5|8|13}
depends_on: {depends_on_array}     # Array of STORY-IDs or [] (from Step 1.6)
created: {YYYY-MM-DD}              # Today's date
updated: {YYYY-MM-DD}              # Same as created initially
assigned_to: null                  # Not assigned yet
format_version: "2.2"              # Template version (STORY-090)
tags: []                           # Empty initially, can add later
---
```

**Generate brief title from feature description:**
```
# Example: "Add user registration with email verification"
# Title: "User registration with email verification"

# Keep under 80 characters
# Remove filler words ("add", "implement", "create")
# Use title case
```

---

## Step 5.3: Build Markdown Sections

**Objective:** Assemble all sections from previous phases

**Section 1: User Story**
```markdown
## User Story

As a {role},
I want {action},
So that {benefit}.
```

**Section 2: Acceptance Criteria**
```markdown
## Acceptance Criteria

### AC1: {Criterion title}
**Given** {context/precondition}
**When** {action/trigger}
**Then** {expected outcome}

### AC2: {Criterion title}
**Given** {context/precondition}
**When** {action/trigger}
**Then** {expected outcome}

[... all acceptance criteria from Phase 2 ...]
```

**Section 3: Technical Specification**
```markdown
## Technical Specification

### API Contracts

{Include API contracts from Phase 3.2 if applicable}

### Data Models

{Include data models from Phase 3.3}

### Business Rules

{Include business rules from Phase 3.4}

### Dependencies

{Include dependencies from Phase 3.5}
```

**Section 4: UI Specification (if applicable)**
```markdown
## UI Specification

### Components

{Include component documentation from Phase 4.2}

### Layout Mockup

```
{Include ASCII mockup from Phase 4.3}
```

### Component Interfaces

{Include TypeScript/C# interfaces from Phase 4.4}

### User Interactions

{Include interaction flows from Phase 4.5}

### Accessibility

{Include accessibility requirements from Phase 4.6}
```

**Section 5: Non-Functional Requirements**
```markdown
## Non-Functional Requirements

### Performance
{Performance targets from Phase 2}

### Security
{Security requirements from Phase 2}

### Usability
{Usability requirements from Phase 2}

### Scalability
{Scalability targets from Phase 2}
```

**Section 6: Edge Cases & Error Handling**
```markdown
## Edge Cases & Error Handling

{Include edge cases from Phase 2}

Example format:
1. **Case:** User closes browser during form submission
   **Expected:** Transaction completes or rolls back, no partial state

2. **Case:** Duplicate email registration attempt
   **Expected:** Error message "Email already registered", suggest login
```

**Step 5.3.5: Generate AC Verification Checklist Section [NEW - RCA-011]**

**Purpose:** Break down acceptance criteria into granular, testable sub-items mapped to TDD phases

**See:** `devforgeai/specs/enhancements/AC-CHECKLIST-TEMPLATE-DESIGN.md` for complete generation logic

**Generate checklist by analyzing ACs:**
```
ac_verification_checklist_section = "## Acceptance Criteria Verification Checklist\n\n"
ac_verification_checklist_section += "**Purpose:** Real-time progress tracking during TDD implementation. Check off items as each sub-task completes.\n\n"
ac_verification_checklist_section += "**Usage:** The devforgeai-development skill updates this checklist at the end of each TDD phase (Phases 1-5), providing granular visibility into AC completion progress.\n\n"
ac_verification_checklist_section += "**Tracking Mechanisms:**\n"
ac_verification_checklist_section += "- **TodoWrite:** Phase-level tracking (AI monitors workflow position)\n"
ac_verification_checklist_section += "- **AC Checklist:** AC sub-item tracking (user sees granular progress) ← YOU ARE HERE\n"
ac_verification_checklist_section += "- **Definition of Done:** Official completion record (quality gate validation)\n\n"

FOR each AC in acceptance_criteria:
  ac_verification_checklist_section += f"### AC#{ac.number}: {ac.title}\n\n"

  # Break AC into testable sub-items
  sub_items = generate_sub_items_from_ac(ac)

  FOR each sub_item in sub_items:
    # Infer phase mapping
    phase = infer_phase_from_item(sub_item)

    # Infer evidence location
    evidence = infer_evidence_location(sub_item, story_type)

    ac_verification_checklist_section += f"- [ ] {sub_item} - **Phase:** {phase} - **Evidence:** {evidence}\n"

  ac_verification_checklist_section += "\n"

ac_verification_checklist_section += "---\n\n"
ac_verification_checklist_section += "**Checklist Progress:** 0/{total_items} items complete (0%)\n\n"
```

**Helper function: generate_sub_items_from_ac(ac):**
```
IF ac contains Given/When/Then:
  Extract testable assertions from Then clause
  Example: "Then user receives 201 Created" → "201 Created response validated"

IF ac contains metrics (≤, ≥, <, >):
  Extract metric as sub-item
  Example: "Character count ≤15,000" → "Character count ≤15,000"

IF ac contains "all", "every", "each":
  Create sub-item for each instance
  Example: "All 6 scenarios pass" → 6 sub-items (one per scenario)

IF ac mentions implementation:
  Create implementation sub-item
  Example: "Business logic extracted" → "Business logic in correct location"
```

**Helper function: infer_phase_from_item(sub_item):**
```
IF sub_item contains "test" or "coverage":
  RETURN 1  # Red phase (test generation)

ELIF sub_item contains "implement" or "create" or "endpoint" or "code":
  RETURN 2  # Green phase (implementation)

ELIF sub_item contains "refactor" or "quality" or "complexity" or "pattern":
  RETURN 3  # Refactor phase (code quality)

ELIF sub_item contains "integration" or "scenario" or "performance" or "coverage threshold":
  RETURN 4  # Integration phase

ELIF sub_item contains "deferral" or "approval":
  RETURN 4.5  # Deferral challenge

ELIF sub_item contains "commit" or "status" or "backward":
  RETURN 5  # Git workflow

ELSE:
  RETURN 2  # Default to implementation phase
```

**Helper function: infer_evidence_location(sub_item, story_type):**
```
IF sub_item contains "test":
  IF story_type == "CRUD":
    RETURN "tests/integration/test_{entity}_crud.py"
  ELIF story_type == "Refactoring":
    RETURN "tests/unit/test_{component}_refactoring.py"
  ELSE:
    RETURN "tests/ (test files)"

ELIF sub_item contains "character count" or "line count":
  RETURN "wc -c/-l < {file_path}"

ELIF sub_item contains "endpoint":
  RETURN "src/controllers/{entity}.controller.{ext}"

ELIF sub_item contains "commit":
  RETURN "git log -1"

ELSE:
  RETURN "{implementation_location}"
```

---

**Step 5.3.6: Populate Provenance Section [NEW - STORY-294]**

**Purpose:** Extract lineage data from brainstorm/epic chain for traceability

**Trigger:** When epic has `brainstorm_id` field

**Workflow:**
```
provenance_section = ""

# Step 1: Check for brainstorm source
IF epic_id exists:
    epic_content = Read(file_path=f"devforgeai/specs/Epics/{epic_id}.epic.md")
    brainstorm_id = extract_yaml_field(epic_content, "brainstorm_id")

    IF brainstorm_id:
        brainstorm_path = f"devforgeai/specs/brainstorms/{brainstorm_id}.brainstorm.md"
        brainstorm_content = Read(file_path=brainstorm_path)

        # Step 2: Extract provenance elements
        origin = extract_origin(brainstorm_path, brainstorm_content, feature_description)
        stakeholder = extract_stakeholder(brainstorm_content)
        hypothesis = extract_hypothesis(brainstorm_content)
        decision = extract_decision(epic_content, feature_number)

        # Step 3: Build XML section
        provenance_section = build_provenance_xml(origin, decision, stakeholder, hypothesis)

# If no brainstorm source, leave empty (optional section)
```

**Helper: extract_origin(path, content, feature_desc):**
```
# Find verbatim quote matching feature
lines = find_matching_lines(content, feature_desc)
RETURN {
    document: path,
    quote: lines.text,
    line_reference: f"lines {lines.start}-{lines.end}"
}
```

**Helper: extract_stakeholder(brainstorm_content):**
```
# Parse ## Stakeholder Analysis section
stakeholder_section = extract_section(brainstorm_content, "Stakeholder Analysis")
RETURN {
    role: extract_field(stakeholder_section, "role"),
    goal: extract_field(stakeholder_section, "goal"),
    quote: extract_verbatim(stakeholder_section)
}
```

**Helper: extract_hypothesis(brainstorm_content):**
```
# Parse ## Hypotheses section
hypothesis_section = extract_section(brainstorm_content, "Hypotheses")
RETURN {
    id: extract_field(hypothesis_section, "id"),
    validation: extract_field(hypothesis_section, "validation"),
    success_criteria: extract_field(hypothesis_section, "success_criteria")
}
```

**Helper: extract_decision(epic_content, feature_number):**
```
# Parse feature section from epic for decision rationale
feature_section = extract_section(epic_content, f"Feature {feature_number}")
RETURN {
    selected: extract_field(feature_section, "selected_approach"),
    rejected: extract_field(feature_section, "rejected_alternatives"),
    trade_off: extract_field(feature_section, "trade_offs")
}
```

**Helper: build_provenance_xml(origin, decision, stakeholder, hypothesis):**
```
# Build XML provenance section from extracted elements
xml = "<provenance>\n"

IF origin:
    xml += f"  <origin>\n"
    xml += f"    <document>{origin.document}</document>\n"
    xml += f"    <quote>{origin.quote}</quote>\n"
    xml += f"    <line_reference>{origin.line_reference}</line_reference>\n"
    xml += f"  </origin>\n"

IF decision:
    xml += f"  <decision>\n"
    xml += f"    <selected>{decision.selected}</selected>\n"
    xml += f"    <rejected reason=\"{decision.rejected}\">rejected alternative</rejected>\n"
    xml += f"    <trade_off>{decision.trade_off}</trade_off>\n"
    xml += f"  </decision>\n"

IF stakeholder:
    xml += f"  <stakeholder>\n"
    xml += f"    <role>{stakeholder.role}</role>\n"
    xml += f"    <goal>{stakeholder.goal}</goal>\n"
    xml += f"    <quote>{stakeholder.quote}</quote>\n"
    xml += f"  </stakeholder>\n"

IF hypothesis:
    xml += f"  <hypothesis id=\"{hypothesis.id}\">\n"
    xml += f"    <validation>{hypothesis.validation}</validation>\n"
    xml += f"    <success_criteria>{hypothesis.success_criteria}</success_criteria>\n"
    xml += f"  </hypothesis>\n"

xml += "</provenance>"
RETURN xml
```

**Output:** Populated `<provenance>` XML section or empty string if no brainstorm source

---

**Section 7: Definition of Done**
```markdown
## Definition of Done

### Implementation
- [ ] All acceptance criteria implemented
- [ ] Unit tests written and passing (95% coverage for business logic)
- [ ] Integration tests written and passing
- [ ] API endpoints implemented (if applicable)
- [ ] UI components implemented (if applicable)
- [ ] Error handling implemented for all edge cases
- [ ] Logging added for debugging

### Code Quality
- [ ] Code follows coding-standards.md
- [ ] No violations of architecture-constraints.md
- [ ] No anti-patterns from anti-patterns.md
- [ ] Cyclomatic complexity <10 per method
- [ ] Code review completed

### Testing
- [ ] All acceptance criteria have automated tests
- [ ] Edge cases have tests
- [ ] Test coverage meets thresholds (95%/85%/80%)
- [ ] All tests passing (100% pass rate)

### Documentation
- [ ] Code comments for complex logic
- [ ] API documentation generated (Swagger/OpenAPI)
- [ ] README updated if needed

### Security
- [ ] Input validation implemented
- [ ] Authentication/authorization implemented (if applicable)
- [ ] No hardcoded secrets or credentials
- [ ] Security scan passed (no CRITICAL or HIGH vulnerabilities)
```

**Section 8: Workflow History**
```markdown
## Workflow History

- **{timestamp}** - Story created, status: Backlog
```

---

## Step 5.4: Write Story File

**Objective:** Write complete story document to disk

**Construct complete file:**
```
story_content = f"""---
{frontmatter}
---

{user_story_section}

{acceptance_criteria_section}

{technical_specification_section}

{ui_specification_section or ''}

{nfr_section}

{edge_cases_section}

{ac_verification_checklist_section}

{definition_of_done_section}

{workflow_history_section}
"""
```

**Note:** `ac_verification_checklist_section` is generated in Step 5.3.5 (NEW - RCA-011)

**Write to disk:**
```
# Ensure directory exists
Bash(mkdir -p devforgeai/specs/Stories/)

# Generate filename slug
slug = slugify(title)  # "user-registration" from "User Registration"

# Write file
Write(
  file_path=f"devforgeai/specs/Stories/{story_id}-{slug}.story.md",
  content=story_content
)
```

**Verify file creation:**
```
# Read back to confirm
created_file = Read(file_path=f"devforgeai/specs/Stories/{story_id}-{slug}.story.md", limit=30)

# Verify frontmatter parses correctly
if not created_file.startswith("---"):
    ERROR: Story file creation failed or corrupted
```

**Update TodoWrite:**
```
TodoWrite: Mark story creation as completed
```

---

## Reference Files Used

**Phase 5 references:**
- `assets/templates/story-template.md` (609 lines) - Base template structure
- `story-structure-guide.md` (662 lines) - YAML frontmatter, section formatting

---

## Output

**Phase 5 produces:**
- ✅ Complete story file created at `devforgeai/specs/Stories/{STORY-ID}-{slug}.story.md`
- ✅ All sections populated (user story, AC, tech spec, UI spec, NFRs, edge cases, DoD, history)
- ✅ YAML frontmatter valid
- ✅ File verified on disk

---

## Error Handling

**Error 1: File write failed**
- **Detection:** Write tool returns error or file doesn't exist after write
- **Recovery:** Check directory permissions, verify path, retry write

**Error 2: YAML frontmatter invalid**
- **Detection:** Read-back shows corrupted frontmatter (missing ---, invalid fields)
- **Recovery:** Reconstruct frontmatter, validate all fields, re-write

**Error 3: Missing required sections**
- **Detection:** Sections from Phases 2-4 not included in final document
- **Recovery:** Verify phase outputs exist, re-assemble content

See `error-handling.md` for comprehensive error recovery procedures.

---

## Next Phase

**After Phase 5 completes →** Phase 6: Epic/Sprint Linking

Load `epic-sprint-linking.md` for Phase 6 workflow.
