---
name: devforgeai-story-creation
description: Create user stories with acceptance criteria, technical specifications, and UI specifications. Use when transforming feature descriptions into structured stories, generating stories from epic features, or creating follow-up stories for deferred work. Supports CRUD, authentication, workflow, and reporting story types with complete technical and UI specifications.
model: claude-opus-4-5-20251101
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - Task
  - AskUserQuestion
  - TodoWrite
---

# DevForgeAI Story Creation Skill

Generate complete user stories with acceptance criteria, technical specifications, and UI specifications through an 8-phase workflow.

---

## ⚠️ EXECUTION MODEL: This Skill Expands Inline

**After invocation, YOU (Claude) execute these instructions phase by phase.**

**When you invoke this skill:**
1. This SKILL.md content is now in your conversation
2. You execute each phase sequentially
3. You display results as you work through phases
4. You complete with success/failure report

**Do NOT:**
- ❌ Wait passively for skill to "return results"
- ❌ Assume skill is executing elsewhere
- ❌ Stop workflow after invocation

**Proceed to "Purpose" section below and begin execution.**

---

## Purpose

This skill transforms feature descriptions into comprehensive, implementation-ready user stories. Each generated story includes:

- **User Story:** As a/I want/So that format
- **Acceptance Criteria:** 3+ testable Given/When/Then scenarios
- **Technical Specification:** API contracts, data models, business rules
- **UI Specification:** Components, mockups, accessibility (if applicable)
- **Non-Functional Requirements:** Performance, security, scalability targets
- **Definition of Done:** Implementation, quality, testing, documentation checklists

### Core Philosophy

**"Spec-Driven Stories Enable Zero-Debt Development"**
- Comprehensive specifications prevent ambiguity during implementation
- Testable acceptance criteria enable TDD Red phase
- Technical specifications define API contracts before coding
- UI specifications provide mockups before frontend work

**"Ask, Don't Assume"**
- Use AskUserQuestion for ALL ambiguities
- Never infer technical details from incomplete descriptions
- Validate assumptions explicitly with users

**"Quality at Creation, Not Later"**
- Self-validation in Phase 7 ensures quality before completion
- Measurable NFRs (not vague like "fast" or "scalable")
- Testable acceptance criteria (can verify pass/fail)
- Complete specifications (no TBD or TODO placeholders)

---

## Story Template Versions

**Current Version:** 2.1 (as of 2025-01-21)

### Version History

**v2.1 (2025-01-21) - AC Header Clarity Enhancement (RCA-012)**
- **Change:** Removed checkbox syntax from AC headers
  - Before: `### 1. [ ] Criterion Title`
  - After: `### AC#1: Criterion Title`
- **Rationale:** AC headers are definitions (what to test), not trackers (what's complete)
  - Three-layer tracking system handles progress:
    - TodoWrite: AI phase-level monitoring
    - AC Verification Checklist: Granular sub-item tracking (20-50 items)
    - Definition of Done: Official completion record (30-40 items)
  - AC headers with checkbox syntax created false expectation of marking
- **Impact:** Eliminates systematic user confusion about unchecked AC headers in completed stories
- **Evidence:** RCA-012 sampling showed 80% of stories left AC headers unchecked despite 100% DoD completion
- **References:** `devforgeai/RCA/RCA-012/` (complete analysis and remediation plan)

**v2.0 (2025-10-30) - Structured Tech Spec (RCA-006 Phase 2)**
- **Change:** Added machine-readable `technical_specification` YAML block
  - Component types: Service, Worker, Configuration, API, Repository, DataModel, Logging
  - Embedded test requirements for each component (`test_requirement` field)
  - Deterministic parsing for automated test generation
- **Impact:** Test generation accuracy improved from 85% to 95%+
- **References:** `devforgeai/specs/STRUCTURED-FORMAT-SPECIFICATION.md`
    Read(file_path="devforgeai/specs/STRUCTURED-FORMAT-SPECIFICATION.md")

**v1.0 (Initial) - Original Template**
- **Features:** User story format, AC headers with checkboxes, freeform tech spec, Definition of Done section
- **Status:** Legacy format (still supported for backward compatibility)

### Migration Paths

**v1.0 → v2.0:** Gradual migration (on story update)
**v2.0 → v2.1:** Optional automated migration
- **Script:** `.claude/skills/devforgeai-story-creation/scripts/migrate-ac-headers.sh`
- **Usage:** `bash migrate-ac-headers.sh <story-file>`
- **Documentation:** `devforgeai/RCA/RCA-012/MIGRATION-SCRIPT.md`
- **Safety:** Creates `.v2.0-backup` before changes, provides restore instructions

**Backward Compatibility:** All versions (v1.0, v2.0, v2.1) supported by framework. Old stories continue to work without migration. Migration is optional (for visual consistency only).

**Template Location:** `assets/templates/story-template.md`
    Read(file_path=".claude/skills/devforgeai-story-creation/assets/templates/story-template.md")

**See Also:** Template changelog in `story-template.md` header (lines 1-58) for complete version history

---

## When to Use This Skill

### ✅ Trigger Scenarios

- User runs `/create-story [feature-description]` command
- devforgeai-orchestration decomposes epic features into stories
- devforgeai-development creates tracking stories for deferred DoD items
- Sprint planning requires story generation
- Manual invocation: `Skill(command="devforgeai-story-creation")`

### ❌ When NOT to Use

- Epic creation (use devforgeai-orchestration epic mode instead)
- Sprint planning (use devforgeai-orchestration sprint mode instead)
- Story already exists (use Edit tool to modify existing story)

---

## Batch Mode Support (NEW - Enhancement)

**Batch mode triggered when:**
- Context marker `**Batch Mode:** true` present in conversation

**Batch mode behavior:**
- **Phase 1 modified:** Skip interactive questions, extract metadata from context markers
- **Phases 2-7:** Execute normally (requirements, tech spec, UI spec, file creation, linking, validation)
- **Phase 8 modified:** Skip next action AskUserQuestion, return immediately to batch loop

**Required context markers for batch mode:**
```
**Story ID:** STORY-009
**Epic ID:** EPIC-001
**Feature Number:** 1.1
**Feature Name:** User Registration Form
**Feature Description:** Implement user registration form with email validation...
**Priority:** High
**Points:** 5
**Type:** feature
**Sprint:** Sprint-1
**Batch Mode:** true
**Batch Index:** 0
```

**When batch mode detected:**
1. Extract all metadata from conversation context
2. Validate all required markers present (Story ID, Epic ID, Feature Description, Priority, Points, Type, Sprint)
3. Skip Phase 1 interactive questions (epic/sprint/priority/points/type selection)
4. Use provided values instead of asking user
5. Execute Phases 2-7 normally (full story generation)
6. Skip Phase 8 next action question (batch loop handles this)
7. Return control to command for next feature in batch

**Fallback:** If required markers missing, switch to interactive mode and ask questions

**See `references/story-discovery.md` for batch mode detection and metadata extraction logic.**
    Read(file_path=".claude/skills/devforgeai-story-creation/references/story-discovery.md.md")

---

## Story Creation Workflow (8 Phases)

**⚠️ EXECUTION STARTS HERE - You are now executing the skill's workflow.**

Each phase loads its reference file on-demand for detailed implementation.

### Phase 0: Epic Input Validation (STORY-301)

**Purpose:** Validate epic document against schema before story decomposition

**Schema Validation:**
```
IF epic_document is provided:
  # Schema validation for epic input
  Read(file_path="src/claude/skills/devforgeai-orchestration/references/skill-output-schemas.yaml")

  # Validate epic input against schema
  validation_result = validate_epic_schema(epic_document)

  IF validation_result.status == "FAILED":
    HALT workflow
    Display: "❌ Schema validation failed for epic document"
    Display: validation_result.errors
    Display: validation_result.recommended_action
    RETURN

  IF validation_result.status == "WARN":
    Display: "⚠️ Schema validation passed with warnings (legacy document)"
    # Proceed with degraded context preservation

  IF validation_result.status == "PASSED":
    Display: "✓ Schema validation passed for epic"
```

---

### Phase 1: Story Discovery
**Purpose:** Generate story ID, discover epic/sprint context, collect metadata (with user-input guidance patterns)
**Reference:** `references/story-discovery.md` (306 lines)
**Guidance Integration:** `references/user-input-integration-guide.md` (1,247 lines)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/story-discovery.md")
    Read(file_path=".claude/skills/devforgeai-story-creation/references/user-input-integration-guide.md")

**Steps:**

#### Step 0.5: Load User Input Guidance Patterns

**Purpose:** Load question enhancement patterns before metadata collection to improve question quality

**Execution:**
```
guidance_path = "src/claude/skills/devforgeai-ideation/references/user-input-guidance.md"
    Read(file_path=".claude/skills/devforgeai-ideation/references/user-input-guidance.md")

TRY:
    guidance_content = Read(file_path=guidance_path)
    token_count = estimate_tokens(guidance_content)  # ~4 chars = 1 token

    IF token_count > 1000:
        # Selective loading: Extract only 4 critical patterns
        Log: "Guidance file large ({token_count} tokens), applying selective loading"
        critical_patterns = ["Explicit Classification", "Bounded Choice", "Fibonacci Bounded Choice", "Open-Ended Discovery"]
        patterns = extract_specific_patterns(guidance_content, critical_patterns)
        token_count_selective = estimate_tokens(patterns)

        IF token_count_selective <= 1000:
            GUIDANCE_AVAILABLE = true
            Log: "Selective loading successful: {len(patterns)} critical patterns loaded ({token_count_selective} tokens)"
        ELSE:
            GUIDANCE_AVAILABLE = false
            Log: "Selective loading failed (still {token_count_selective} tokens), using baseline logic"
    ELSE:
        # Normal loading: Extract all patterns
        patterns = extract_patterns(guidance_content)  # Parse markdown headings
        GUIDANCE_AVAILABLE = true
        Log: "Loaded user-input-guidance.md ({len(patterns)} patterns, {token_count} tokens)"

CATCH FileNotFoundError:
    GUIDANCE_AVAILABLE = false
    Log: "user-input-guidance.md not found, proceeding with baseline logic"
CATCH Exception as e:
    GUIDANCE_AVAILABLE = false
    Log: "Failed to load guidance: {e}, proceeding with baseline"

# For implementation details, see references/user-input-integration-guide.md
```

**Performance Targets:**
- Execution time < 2 seconds (p95)
- Token overhead ≤ 1,000 tokens
- Phase 1 total increase ≤ 5% vs baseline

**Graceful Degradation:**
- If guidance unavailable → Log warning, continue with baseline logic
- If parse fails → Log warning, continue with baseline logic
- If guidance large → Apply selective loading (extract 4 critical patterns only)

**Batch Mode Caching:**
- Story 1: Load and cache guidance
- Stories 2-9: Reuse cached guidance (no re-read, no token overhead)

---

#### Steps 1-2: Feature Capture & ID Generation

See references/story-discovery.md for complete implementation.

#### Step 3: Discover Epic Context (Enhanced with Guidance Patterns)

**IF GUIDANCE_AVAILABLE:**
```
pattern = lookup_pattern("explicit classification bounded choice")
epic_files = Glob(pattern="devforgeai/specs/Epics/*.epic.md")
epic_options = []

FOR each epic_file in epic_files:
    epic_content = Read(file_path=epic_file)
    epic_id = extract_yaml_field(epic_content, "id")
    epic_status = extract_yaml_field(epic_content, "status")
    epic_complexity = extract_yaml_field(epic_content, "complexity")
    epic_options.append({
        label: epic_id,
        description: "Status: {epic_status}, Complexity: {epic_complexity}"
    })

epic_options.append({
    label: "None - standalone story",
    description: "Story not associated with any epic"
})

AskUserQuestion(
    question="Which epic does this story belong to? (Epic linkage enables feature tracking and traceability)",
    header="Epic Association",
    options=epic_options
)
```
**ELSE (baseline logic):**
```
AskUserQuestion with simple epic list (original implementation)
```

#### Step 4: Discover Sprint Context (Enhanced with Guidance Patterns)

**IF GUIDANCE_AVAILABLE:**
```
pattern = lookup_pattern("bounded choice")
sprint_files = Glob(pattern="devforgeai/specs/Sprints/*.md")
sprint_options = [{
    label: "Backlog",
    description: "Not assigned to any sprint"
}]

FOR each sprint_file in sprint_files:
    sprint_content = Read(file_path=sprint_file)
    sprint_id = extract_sprint_number(sprint_content)
    sprint_dates = extract_sprint_dates(sprint_content)
    sprint_capacity = calculate_capacity(sprint_content)
    sprint_options.append({
        label: "Sprint-{sprint_id}",
        description: "{sprint_dates}, {sprint_capacity.used}/{sprint_capacity.total} points used"
    })

# Sort by start date (chronological)
sprint_options = sort_by_date(sprint_options)

AskUserQuestion(
    question="Which sprint should this story be assigned to?",
    header:"Sprint Assignment",
    options=sprint_options
)
```
**ELSE (baseline logic):**
```
AskUserQuestion with simple sprint list (original implementation)
```

#### Step 5: Collect Story Metadata (Enhanced with Guidance Patterns)

**Priority Selection (Explicit Classification):**
```
IF GUIDANCE_AVAILABLE:
    pattern = lookup_pattern("explicit classification")
    AskUserQuestion(
        question:"What is the story priority?",
        header="Priority",
        options=[
            {label:"Critical", description:"Blocking other work, must be done immediately"},
            {label:"High", description:"Important for upcoming release, schedule soon"},
            {label:"Medium", description:"Desirable feature, normal priority"},
            {label:"Low", description:"Nice to have, can be scheduled later"}
        ]
    )
ELSE:
    AskUserQuestion with simple priority list
```

**Story Type Selection (STORY-126):**

**Reference:** `references/story-type-classification.md` for full enum, phase skip matrix, and workflow diagrams.
    Read(file_path=".claude/skills/devforgeai-story-creation/references/story-type-classification.md")

```
AskUserQuestion(
    question:"What type of story is this?",
    header:"Story Type",
    options=[
        {label:"feature", description:"Full TDD workflow (default)"},
        {label:"documentation", description:"Skip integration testing"},
        {label:"bugfix", description:"Skip refactoring phase"},
        {label:"refactor", description:"Skip test generation"}
    ]
)

# Default to "feature" if user skips or selects "Other"
story_type = user_response OR "feature"
Display:"Story type: {story_type}"
```

**Story Points (Fibonacci Bounded Choice):**
```
IF GUIDANCE_AVAILABLE:
    pattern = lookup_pattern("fibonacci bounded choice")
    AskUserQuestion(
        question:"Estimate story complexity:",
        header:"Story Points",
        options=[
            {label:"1", description:"Trivial - Few hours, minimal complexity"},
            {label:"2", description:"Simple - Half day, straightforward implementation"},
            {label:"3", description:"Standard - 1 day, moderate complexity"},
            {label:"5", description:"Complex - 2-3 days, multiple components"},
            {label:"8", description:"Very complex - 3-5 days, significant work"},
            {label:"13", description:"Extremely complex - Consider splitting story"}
        ]
    )

    IF user_selects("13"):
        Display:"⚠️  13-point stories are difficult to estimate and complete"
        Display:"Consider splitting into smaller stories for better predictability"
ELSE:
    AskUserQuestion with simple points list
```

**Output:** Story ID, epic/sprint links, priority, points, type (with enhanced user input quality via patterns)

### Phase 2: Requirements Analysis
**Purpose:** Generate user story and acceptance criteria
**Reference:** `references/requirements-analysis.md` (201 lines)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/requirements-analysis.md")
**Subagent:** requirements-analyst
**Steps:** Invoke subagent, validate output, refine if incomplete
**Output:** User story, 3+ AC (Given/When/Then format), edge cases, NFRs

### Phase 3: Technical Specification
**Purpose:** Define API contracts, data models, business rules
**Reference:** `references/technical-specification-creation.md` (303 lines)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/technical-specification-creation.md")
**Subagent:** api-designer (conditional - if API endpoints detected)
**Steps:** Detect API needs, generate contracts, define data models, document rules, identify dependencies
**Output:** API contracts, data models, business rules, dependencies

**Format Version:** 2.0 (Structured YAML) - **Default for all stories created after 2025-11-07**

**Critical:** All new stories MUST use v2.0 structured YAML format in Technical Specification section. This enables:
- 95%+ parsing accuracy (vs 85% with v1.0 freeform text)
- Automated validation in Phase 3 (implementation-validator requires v2.0)
- Deterministic coverage gap detection in Phase 1 Step 4

**v2.0 Format Overview:**
Technical Specification section contains YAML code block with:
- `components`: Array of Service, Worker, Configuration, Logging, Repository, API, DataModel
- `business_rules`: Array of domain rules with test_requirement
- `non_functional_requirements`: Array of NFRs with measurable metrics

**Complete schema reference:**
`devforgeai/specs/STRUCTURED-FORMAT-SPECIFICATION.md`
    Read(file_path="devforgeai/specs/STRUCTURED-FORMAT-SPECIFICATION.md")

**The reference file `technical-specification-creation.md` contains:**
- Complete v2.0 generation instructions
- Component type selection guide
- Test requirement format
- api-designer integration for API components

**Load and follow technical-specification-creation.md for Phase 3 execution.**

### Phase 4: UI Specification
**Purpose:** Document UI components, mockups, accessibility
**Reference:** `references/ui-specification-creation.md` (312 lines)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/ui-specification-creation.md")
**Steps:** Detect UI needs, document components, create ASCII mockup, define interfaces, specify interactions, WCAG AA compliance
**Output:** Component list, layout mockup, interfaces, interaction flows, accessibility requirements

### Phase 5: Story File Creation
**Purpose:** Assemble complete story document
**Reference:** `references/story-file-creation.md` (323 lines)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/story-file-creation.md")
**Template:** `assets/templates/story-template.md` (609 lines)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/story-template.md")
**Steps:** Load template, construct frontmatter, build sections, populate provenance from brainstorm/epic chain (if available), write to disk, verify
**Output:** Complete .story.md file in devforgeai/specs/Stories/ with provenance traceability

### Phase 6: Epic/Sprint Linking
**Purpose:** Update parent documents with story references
**Reference:** `references/epic-sprint-linking.md` (140 lines)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/epic-sprint-linking.md")
**Steps:** Update epic file, update sprint file, verify linking
**Output:** Epic/sprint files updated, links verified

### Phase 7: Self-Validation
**Purpose:** Quality checks and self-healing
**Reference:** `references/story-validation-workflow.md` (233 lines)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/story-validation-workflow.md")
**Checklist:** `references/validation-checklists.md` (1,038 lines)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/validation-checklists.md")
**Steps:** Validate frontmatter, user story, AC, tech spec, NFRs
**Output:** Validated story, auto-corrected issues

### Phase 8: Completion Report
**Purpose:** Generate summary and guide next actions
**Reference:** `references/completion-report.md` (160 lines)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/completion-report.md")
**Steps:** Generate completion summary, determine next action (AskUserQuestion)
**Output:** Structured completion summary, next step recommendations

**See individual phase reference files for complete implementation details.**

---

## Subagent Coordination

This skill delegates specialized tasks to subagents:

- **requirements-analyst** (Phase 2) - User story and AC generation
- **api-designer** (Phase 3, conditional) - API contract design when endpoints detected

**Subagent invocation details in:**
- `references/requirements-analysis.md` (requirements-analyst coordination)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/requirements-analysis.md")
- `references/technical-specification-creation.md` (api-designer coordination)
    Read(file_path=".claude/skills/devforgeai-story-creation/references/technical-specification-creation.md")

---

## Integration Points

**Invoked by:**
- `/create-story` command (user-initiated)
- devforgeai-orchestration skill (epic/sprint decomposition)
- devforgeai-development skill (deferred work tracking)

**Provides output to:**
- devforgeai-ui-generator (AC → UI requirements)
- devforgeai-development (AC → test generation)
- devforgeai-qa (AC → validation targets)

**See `references/integration-guide.md` for complete integration patterns.**
    Read(file_path=".claude/skills/devforgeai-story-creation/references/integration-guide.md")

---

## Success Criteria

Complete story generated with:
- [ ] Valid story ID (STORY-NNN format)
- [ ] User story (As a/I want/So that)
- [ ] 3+ acceptance criteria (Given/When/Then)
- [ ] Technical specification (complete)
- [ ] UI specification (if applicable)
- [ ] Non-functional requirements (measurable)
- [ ] Edge cases documented
- [ ] Definition of Done (checkboxes)
- [ ] File written to devforgeai/specs/Stories/
- [ ] Epic/sprint updated (if applicable)
- [ ] Self-validation passed
- [ ] Token usage <90K (isolated context)

---

## Reference Files

Load these on-demand during workflow execution:

### Phase Workflows (10 files)
- **story-discovery.md** (306 lines) - Phase 1: ID generation, context discovery
- **requirements-analysis.md** (201 lines) - Phase 2: User story and AC
- **technical-specification-creation.md** (303 lines) - Phase 3: APIs, models, rules
- **ui-specification-creation.md** (312 lines) - Phase 4: Components, mockups
- **story-file-creation.md** (323 lines) - Phase 5: Document assembly
- **epic-sprint-linking.md** (140 lines) - Phase 6: Parent doc updates
- **story-validation-workflow.md** (233 lines) - Phase 7: Quality checks
- **completion-report.md** (160 lines) - Phase 8: Summary generation
- **error-handling.md** (385 lines) - Error recovery procedures
- **integration-guide.md** (359 lines) - Skill integration patterns

### Supporting Guides (6 files - existing)
- **acceptance-criteria-patterns.md** (1,259 lines) - Given/When/Then templates by domain
- **story-examples.md** (1,905 lines) - 4 complete story examples (CRUD, auth, workflow, reporting)
- **story-structure-guide.md** (662 lines) - YAML frontmatter, section formatting
- **technical-specification-guide.md** (1,269 lines) - API contract patterns, data modeling
- **ui-specification-guide.md** (1,344 lines) - Component design, ASCII mockups, accessibility
- **validation-checklists.md** (1,038 lines) - Quality validation procedures

### Assets
- **assets/templates/story-template.md** (609 lines) - Base story template (YAML + markdown)

**Total:** 16 reference files + 1 template asset

---

## Best Practices

**Top 5 practices for story creation:**

1. **Provide clear feature description** - Minimum 10 words, specific WHO/WHAT
2. **Associate with epic when possible** - Enables traceability and feature tracking
3. **Ensure AC are testable** - All criteria must be verifiable (Given/When/Then)
4. **Include UI specs for frontend work** - Mockups prevent implementation ambiguity
5. **Trust self-validation** - Phase 7 auto-corrects common issues, high quality output

**See phase-specific reference files for detailed best practices.**
