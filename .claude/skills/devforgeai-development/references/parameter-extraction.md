# Parameter Extraction from Conversation Context

**Purpose:** How the development skill extracts the story ID and other parameters from conversation context.

**Applies to:** devforgeai-development skill (and pattern reusable for other skills)

---

## Background

Skills CANNOT accept command-line parameters. Instead, they extract parameters from:
1. Loaded file content (YAML frontmatter)
2. Explicit context markers in conversation
3. Natural language in user messages

---

## CRITICAL: Extracting Parameters from Conversation Context

**IMPORTANT:** Skills CANNOT accept runtime parameters. All information must be extracted from conversation context.

### How Slash Commands Pass "Parameters" to Skills

When a slash command invokes this skill, it:
1. Loads story file via @file reference: `@devforgeai/specs/Stories/STORY-XXX.story.md`
2. States context explicitly: "Story ID: STORY-XXX"
3. Invokes skill WITHOUT arguments: `Skill(command="devforgeai-development")`

**You must extract story ID from the conversation.**

### Story ID Extraction

The slash command loads the story file via @file reference, making story content available in conversation.

**Extract story ID from conversation:**

**Method 1: Read YAML frontmatter**
```
Look for YAML frontmatter in conversation:
  ---
  id: STORY-XXX
  title: ...
  status: ...
  ---

Extract: id field = Story ID
```

**Method 2: Search for file reference**
```
Search conversation for pattern:
  "devforgeai/specs/Stories/STORY-XXX"

Extract STORY-XXX from file path
```

**Method 3: Search for explicit statement**
```
Search conversation for:
  "Story ID: STORY-XXX"
  "Story: STORY-XXX"

Extract STORY-XXX
```

**Method 4: Grep loaded content**
```
If methods 1-3 fail:
  Grep conversation for "STORY-[0-9]+" pattern
  Use first match found
```

### Resume Mode Detection (NEW - RCA-013)

**After story ID extraction, check if this is a resumption:**

**Method 1: Search for resume context marker**
```
Search conversation for:
  "**Resume from Phase:** {N}"
  "**Resume Mode:** manual" OR "**Resume Mode:** auto"

IF found:
  resume_mode = true
  resume_from_phase = N  # Extract phase number
  resume_type = "manual" or "auto"  # Extract type

  Display: ""
  Display: "✓ RESUME MODE DETECTED"
  Display: "  Resume from: Phase {N}"
  Display: "  Resume type: {resume_type}"
  Display: "  Skipping phases 0-{N-1}..."
  Display: ""
ELSE:
  resume_mode = false
  resume_from_phase = 0  # Start from beginning
```

**Method 2: Infer from story status**
```
IF resume_mode == false:
  Read story YAML frontmatter: status field

  IF status == "In Development" AND conversation contains "second run" OR "re-running":
    # User is re-running /dev on incomplete story
    # Could auto-detect resumption point, but explicit is better
    Display: ""
    Display: "⚠️ Story status 'In Development' suggests previous incomplete run"
    Display: "   Use /resume-dev for explicit phase control"
    Display: "   Continuing with full workflow from Phase 01..."
    Display: ""
    resume_mode = false  # Full workflow, but user aware of alternative
```

**Skip completed phases if resume_mode = true:**
```
IF resume_mode == true:
  FOR phase_num = 0 TO resume_from_phase - 1:
    Display: "⊘ Phase {phase_num}: Skipped (resume mode)"
    # Mark in TodoWrite as "skipped"
    todos[phase_num].status = "skipped"

  Display: ""
  Display: "════════════════════════════════════════════════════════════"
  Display: "STARTING EXECUTION FROM PHASE {resume_from_phase}"
  Display: "════════════════════════════════════════════════════════════"
  Display: ""

  GOTO Phase {resume_from_phase}
```

**IMPORTANT: Partial Phase 01 Execution in Resume Mode**

When resume_mode = true, Phase 01 is marked "skipped" but **CRITICAL validations must still execute**:

**These Phase 01 steps are MANDATORY even in resume mode:**
- Phase 01.4: Validate Context Files Exist (devforgeai-validate validate-context)
- Phase 01.6: Validate Spec vs Context Files (conflict detection)
- Phase 01.7: Detect and Validate Technology Stack (tech-stack-detector)

**These Phase 01 steps are SKIPPED in resume mode:**
- Phase 01.1: Validate Git Repository Status (git-validator)
- Phase 01.1.5: User Consent for Git Operations
- Phase 01.1.6: Stash Warning
- Phase 01.2: Adapt TDD Workflow (git vs file-based)
- Phase 01.3: File-Based Tracking Setup

**Rationale:**
- Context files: Required for ALL phases (implementation, quality, integration all need constraints)
- Tech stack: Prevents technology drift between runs (ensures compatibility)
- Git validation: Not needed for resumption (story already has git state, continuing work)

**Implementation:**
The /resume-dev command executes Steps d., f., g. in its Phase 01 (Essential Pre-Flight Checks) BEFORE invoking the skill. This ensures context is validated even though skill's Phase 01 is skipped.

### Validation Before Proceeding

Before starting TDD workflow, verify:
- [ ] Story ID extracted successfully
- [ ] Story content available in conversation (via @file load)
- [ ] Acceptance criteria accessible from story content
- [ ] Technical specification present
- [ ] Resume mode detected (if applicable)
- [ ] Resume phase number valid (0-7 range)

**If extraction fails:**
```
HALT with error:
"Cannot extract story ID from conversation context.

Expected to find:
  - YAML frontmatter with 'id: STORY-XXX' field
  - OR file reference like 'devforgeai/specs/Stories/STORY-XXX.story.md'
  - OR explicit statement like 'Story ID: STORY-XXX'

Please ensure story is loaded via slash command or provide story ID explicitly."
```

**If resume phase invalid:**
```
HALT with error:
"Invalid resume phase: {resume_from_phase}

Valid phases: 0-7
  0 = Pre-Flight
  1 = Red (Test Generation)
  2 = Green (Implementation)
  3 = Refactor (Quality)
  4 = Integration
  5 = Git Workflow
  6 = Feedback Hook
  7 = Result Interpretation

Use: /resume-dev {STORY_ID} {valid-phase-number}"
```
