# Slash Command Argument Validation Pattern

**Purpose:** Standard defensive validation for DevForgeAI slash commands
**Version:** 1.0
**Last Updated:** 2025-11-02
**Status:** Production Ready

---

## Overview

This pattern provides defensive argument validation for slash commands to handle:
- Malformed story IDs
- Unknown flags (--mode=, --env=, etc.)
- Typos and user errors
- Ambiguous input
- Missing required arguments

## Core Principle: Ask, Don't Assume

**Philosophy:** When user input is unclear or malformed, ASK for clarification rather than failing silently or making assumptions.

**Benefits:**
- ✅ Graceful error handling (no cryptic failures)
- ✅ User education (shows correct syntax in options)
- ✅ Self-documenting (users learn through interaction)
- ✅ Defensive programming (validates before executing)
- ✅ Framework alignment ("Ask, Don't Assume" principle)

---

## Standard Phase 0: Argument Validation

Add this as the first phase (Phase 0) to all slash commands that accept story IDs or other structured arguments.

### Step 1: Extract Story ID

```markdown
**Extract story ID:**
```
STORY_ID = $1
```
```

**Expected format:** `STORY-NNN` where NNN is numeric (e.g., STORY-001, STORY-042, STORY-123)

**Regex pattern:** `^STORY-[0-9]+$`

---

### Step 2: Validate Story ID Format

```markdown
**Validate story ID format:**
```
IF $1 is empty:
  AskUserQuestion:
  Question: "No story ID provided. Which story should I process?"
  Header: "Story ID Required"
  Options:
    - "List stories in [appropriate status]"
    - "Show correct command syntax"
    - "Cancel command"
  multiSelect: false

IF $1 does NOT match pattern "STORY-[0-9]+":
  # Malformed format - could be typo, wrong case, missing dash, etc.

  AskUserQuestion:
  Question: "Story ID '$1' doesn't match format STORY-NNN. What story should I process?"
  Header: "Story ID Format"
  Options:
    - "Try to extract STORY-NNN from: $1"
    - "List all available stories"
    - "Show correct format examples"
  multiSelect: false

  Extract STORY_ID based on user response
```
```

**Common malformed inputs:**
- `story-001` (lowercase s) → Extract to `STORY-001`
- `STORY001` (missing dash) → Extract to `STORY-001`
- `001` (missing prefix) → Extract to `STORY-001`
- `my-feature` (wrong format) → Ask user which story they meant

---

### Step 3: Validate Story File Exists

```markdown
**Validate story file exists:**
```
Glob(pattern="devforgeai/specs/Stories/${STORY_ID}*.story.md")

IF no matches found:
  # Story doesn't exist - user provided non-existent ID

  AskUserQuestion:
  Question: "Story ${STORY_ID} not found. What should I do?"
  Header: "Story Not Found"
  Options:
    - "List all available stories"
    - "Create ${STORY_ID} (run /create-story first)"
    - "I meant a different story (let me re-enter)"
    - "Cancel command"
  multiSelect: false

  Handle based on user selection:
  - If "List all": Glob("devforgeai/specs/Stories/*.md"), display list, ask which one
  - If "Create": Exit with message to run /create-story
  - If "Different story": Ask for correct story ID
  - If "Cancel": Exit gracefully

IF multiple matches found:
  # Example: STORY-001-feature-a.story.md, STORY-001-feature-b.story.md
  # Unlikely but handle gracefully

  AskUserQuestion:
  Question: "Multiple files match ${STORY_ID}. Which one?"
  Header: "Story Selection"
  Options:
    [List each matched filename as option]
  multiSelect: false

  STORY_FILE = user selection

ELSE:
  # Exactly 1 match - ideal case
  ✓ Story file found
  STORY_FILE = matched file path
```
```

---

### Step 4: Parse Optional Arguments

For commands with optional arguments (mode, environment, etc.), validate the second argument.

#### Pattern A: Mode Argument (for /qa command)

```markdown
**Parse mode argument:**
```
IF $2 provided:
  # User provided second argument - could be mode or flag

  IF $2 in ["deep", "light"]:
    # Correct usage - no flag syntax
    MODE = $2
    ✓ Valid mode

  ELSE IF $2 starts with "--mode=":
    # User used flag syntax (educate them)
    EXTRACTED_MODE = substring after "--mode="

    IF EXTRACTED_MODE in ["deep", "light"]:
      MODE = EXTRACTED_MODE

      Note to user: "Flag syntax (--mode=) not needed. Use: /qa STORY-001 deep"
      ✓ Mode extracted, user educated

    ELSE:
      # Flag with invalid value: --mode=invalid
      AskUserQuestion:
      Question: "Unknown mode in flag: $2. Which validation mode?"
      Header: "QA Mode"
      Options:
        - "deep (comprehensive validation ~2 min)"
        - "light (quick checks ~30 sec)"
      multiSelect: false

      MODE = user selection

  ELSE IF $2 starts with "--":
    # Unknown flag syntax: --unknown or --env=staging (wrong flag for command)
    AskUserQuestion:
    Question: "Unknown flag: $2. Which validation mode did you want?"
    Header: "QA Mode"
    Options:
      - "deep (comprehensive validation)"
      - "light (quick checks)"
      - "Show correct /qa syntax"
    multiSelect: false

    MODE = user selection
    Note to user: "Flags not needed. Use: /qa STORY-001 deep"

  ELSE:
    # Unknown value (not a recognized mode, not a flag)
    # Example: /qa STORY-001 comprehensive

    AskUserQuestion:
    Question: "Unknown mode: $2. Which validation mode?"
    Header: "QA Mode"
    Options:
      - "deep (comprehensive validation)"
      - "light (quick checks)"
    multiSelect: false

    MODE = user selection

ELSE:
  # No mode provided - use intelligent default

  # Read story status from YAML frontmatter (already loaded via @file)
  IF story status == "Dev Complete":
    MODE = "deep"  # Full validation before QA approval
    Note: "Defaulting to deep validation (story is Dev Complete)"

  ELSE IF story status == "In Development":
    MODE = "light"  # Quick validation during development
    Note: "Defaulting to light validation (story in development)"

  ELSE:
    # Unclear - ask user
    AskUserQuestion:
    Question: "No mode specified. Which validation?"
    Header: "QA Mode"
    Options:
      - "deep (comprehensive - for Dev Complete stories)"
      - "light (quick - for In Development stories)"
    multiSelect: false

    MODE = user selection
```
```

#### Pattern B: Environment Argument (for /release command)

```markdown
**Parse environment argument:**
```
IF $2 provided:
  # User provided environment argument

  IF $2 in ["staging", "production", "prod", "stage"]:
    # Correct usage - normalize variants
    IF $2 in ["prod", "production"]:
      ENVIRONMENT = "production"
    ELSE IF $2 in ["stage", "staging"]:
      ENVIRONMENT = "staging"
    ✓ Valid environment

  ELSE IF $2 starts with "--env=":
    # User used flag syntax (educate them)
    EXTRACTED_ENV = substring after "--env="

    IF EXTRACTED_ENV in ["staging", "production"]:
      ENVIRONMENT = EXTRACTED_ENV

      Note to user: "Flag syntax (--env=) not needed. Use: /release STORY-001 production"
      ✓ Environment extracted, user educated

    ELSE:
      # Flag with invalid value: --env=invalid
      AskUserQuestion:
      Question: "Unknown environment in flag: $2. Where should I deploy?"
      Header: "Deployment Target"
      Options:
        - "staging (test environment)"
        - "production (live environment)"
      multiSelect: false

      ENVIRONMENT = user selection

  ELSE IF $2 starts with "--":
    # Unknown flag: --unknown or --mode=deep (wrong flag for command)
    AskUserQuestion:
    Question: "Unknown flag: $2. Where should I deploy?"
    Header: "Deployment Target"
    Options:
      - "staging (test environment first - recommended)"
      - "production (skip staging - risky!)"
    multiSelect: false

    ENVIRONMENT = user selection
    Note to user: "Flags not needed. Use: /release STORY-001 production"

  ELSE:
    # Unknown value: /release STORY-001 live
    AskUserQuestion:
    Question: "Unknown environment: $2. Where should I deploy?"
    Header: "Deployment Target"
    Options:
      - "staging (safe choice for testing)"
      - "production (requires QA approval)"
    multiSelect: false

    ENVIRONMENT = user selection

ELSE:
  # No environment specified - safe default
  ENVIRONMENT = "staging"
  Note to user: "Defaulting to staging. Use '/release STORY-001 production' for production deployment."
```
```

---

### Step 5: Display Validation Summary

```markdown
**Validation summary:**
```
✓ Story ID: ${STORY_ID}
✓ Story file: ${STORY_FILE}
[If mode applicable]
✓ Validation mode: ${MODE}
[If environment applicable]
✓ Environment: ${ENVIRONMENT}
[If other args applicable]
✓ [Other validated arguments]

✓ Proceeding with command execution...
```
```

---

## Implementation Examples

### Example 1: /dev Command

```markdown
### Phase 0: Argument Validation

**Extract story ID:**
STORY_ID = $1

**Validate format:**
IF $1 does NOT match "STORY-[0-9]+":
  AskUserQuestion:
  Question: "Story ID '$1' doesn't match format STORY-NNN. What story should I develop?"
  Header: "Story ID"
  Options:
    - "List stories in Ready for Dev status"
    - "List stories in Backlog status"
    - "Show correct /dev syntax"
  multiSelect: false

**Validate file exists:**
Glob(pattern="devforgeai/specs/Stories/${STORY_ID}*.story.md")

IF no matches:
  AskUserQuestion:
  Question: "Story ${STORY_ID} not found. What should I do?"
  Header: "Story Not Found"
  Options:
    - "List all available stories"
    - "Create ${STORY_ID} (run /create-story first)"
    - "Cancel command"
  multiSelect: false

**Validation summary:**
✓ Story ID: ${STORY_ID}
✓ Story file: ${STORY_FILE}
✓ Proceeding with development...
```

---

### Example 2: /qa Command (with mode argument)

```markdown
### Phase 0: Argument Validation

**Extract arguments:**
STORY_ID = $1
MODE_ARG = $2 (optional)

**Validate story ID:**
[Same as Example 1]

**Parse mode argument:**
IF $2 provided:
  IF $2 in ["deep", "light"]:
    MODE = $2
  ELSE IF $2 starts with "--mode=":
    EXTRACTED = substring after "--mode="
    IF EXTRACTED in ["deep", "light"]:
      MODE = EXTRACTED
      Note: "Use: /qa STORY-001 deep (no -- needed)"
    ELSE:
      AskUserQuestion: [Select deep or light]
  ELSE:
    AskUserQuestion: [Select deep or light]
ELSE:
  # Default based on story status
  IF story status == "Dev Complete":
    MODE = "deep"
  ELSE:
    MODE = "light"

**Validation summary:**
✓ Story ID: ${STORY_ID}
✓ Validation mode: ${MODE}
✓ Proceeding with QA validation...
```

---

### Example 3: /release Command (with environment argument)

```markdown
### Phase 0: Argument Validation

**Extract arguments:**
STORY_ID = $1
ENV_ARG = $2 (optional)

**Validate story ID:**
[Same as Example 1]

**Parse environment argument:**
IF $2 provided:
  IF $2 in ["staging", "production", "prod", "stage"]:
    ENVIRONMENT = normalize($2)  # prod→production, stage→staging
  ELSE IF $2 starts with "--env=":
    EXTRACTED = substring after "--env="
    IF EXTRACTED in ["staging", "production"]:
      ENVIRONMENT = EXTRACTED
      Note: "Use: /release STORY-001 production (no -- needed)"
    ELSE:
      AskUserQuestion: [Select staging or production]
  ELSE:
    AskUserQuestion: [Select staging or production]
ELSE:
  ENVIRONMENT = "staging"  # Safe default
  Note: "Defaulting to staging. Add 'production' for prod deployment."

**Validation summary:**
✓ Story ID: ${STORY_ID}
✓ Environment: ${ENVIRONMENT}
✓ Proceeding with deployment...
```

---

## Testing the Pattern

### Test Cases

**Test Case 1: Correct Usage (No Interaction)**
```bash
> /qa STORY-001 deep

Expected:
  ✓ Story ID: STORY-001 (valid format)
  ✓ Story file found: devforgeai/specs/Stories/STORY-001.story.md
  ✓ Mode: deep (valid)
  ✓ Validation complete

  Proceeding with deep QA validation...
```

**Result:** ✅ No AskUserQuestion triggered - smooth execution

---

**Test Case 2: Flag Syntax (Educational Interaction)**
```bash
> /qa STORY-001 --mode=deep

Expected:
  ✓ Story ID: STORY-001 (valid)
  ⚠️ Flag syntax detected: --mode=deep

  Note: Flag syntax (--mode=) not needed. Use: /qa STORY-001 deep

  ✓ Mode: deep (extracted)
  ✓ Validation complete

  Proceeding with deep QA validation...
```

**Result:** ✅ Works but educates user - improves future usage

---

**Test Case 3: Malformed Story ID (Recovery Interaction)**
```bash
> /qa story-001 deep

Expected:
  ⚠️ Story ID 'story-001' doesn't match format STORY-NNN

  What story should I validate?
  □ Try to extract STORY-NNN from: story-001
  □ List stories in Dev Complete status
  □ Show correct /qa syntax

  [User selects: Try to extract]

  ✓ Story ID: STORY-001 (extracted)
  ✓ Story file: STORY-001.story.md
  ✓ Mode: deep
  ✓ Validation complete

  Proceeding with deep QA validation...
```

**Result:** ✅ Recovers from typo gracefully

---

**Test Case 4: Story Not Found (Helpful Interaction)**
```bash
> /dev STORY-999

Expected:
  ✓ Story ID: STORY-999 (valid format)
  ✗ Story file not found

  Story STORY-999 not found. What should I do?
  □ List all available stories
  □ Create STORY-999 (run /create-story first)
  □ Cancel command

  [User selects: List all available stories]

  Available stories:
  1. STORY-001: Setup Cargo Workspace
  2. STORY-002: CLI Argument Parsing
  3. STORY-003: Tree-sitter Integration

  Which story?
  □ STORY-001
  □ STORY-002
  □ STORY-003

  [User selects: STORY-001]

  ✓ Story ID: STORY-001
  ✓ Story file: STORY-001-setup-cargo-workspace.story.md
  ✓ Validation complete

  Proceeding with development...
```

**Result:** ✅ User discovers correct story ID through interaction

---

**Test Case 5: Unknown Flag (Educational Interaction)**
```bash
> /release STORY-001 --unknown-flag

Expected:
  ✓ Story ID: STORY-001 (valid)
  ⚠️ Unknown flag: --unknown-flag

  Where should I deploy?
  □ staging (test environment first - recommended)
  □ production (skip staging - risky!)

  Note: Flags not needed. Use: /release STORY-001 production

  [User selects: staging]

  ✓ Environment: staging
  ✓ Validation complete

  Proceeding with deployment...
```

**Result:** ✅ Handles gracefully, educates on syntax

---

## Integration with Skills

After Phase 0 validation completes, the command has:
- Validated STORY_ID (format checked, file exists)
- Validated MODE or ENVIRONMENT (if applicable)
- Story content loaded via @file reference

**Skill invocation pattern:**

```markdown
### Phase N: Invoke [Skill Name] Skill

**Context for skill:**
- Story content loaded via @file reference above
- Story ID: ${STORY_ID}
[If mode applicable]
- Validation mode: ${MODE}
[If environment applicable]
- Environment: ${ENVIRONMENT}

**Invoke skill (no arguments):**
```
Skill(command="skill-name")
```

**Note:** Skill will extract story ID from conversation context (YAML frontmatter in loaded story file) and [mode/environment] from explicit statement above.
```

**Key points:**
1. Story already loaded in conversation (via @file)
2. Mode/environment explicitly stated
3. Skill invoked WITHOUT arguments
4. Skill reads context from conversation

---

## Benefits of This Pattern

### 1. Graceful Error Handling ⭐⭐⭐⭐⭐

**Before (Silent Failure):**
```
> /qa story-001
Error: Story file not found: devforgeai/specs/Stories/story-001.story.md
```
User frustrated: "What's wrong? I have a story!"

**After (Helpful Interaction):**
```
> /qa story-001
Story ID 'story-001' doesn't match format STORY-NNN.
What story should I validate?
□ Try to extract STORY-NNN from: story-001
□ List stories in Dev Complete status
```
User helped: "Oh, it should be uppercase! STORY-001"

---

### 2. User Education Through Options

**Learning by doing:**
- User sees correct format in options
- Note messages show correct syntax
- Options demonstrate proper usage
- No need to read documentation

**Example:**
```
Note: Flag syntax (--mode=) not needed. Use: /qa STORY-001 deep
```

User learns: "Oh, I don't need the -- prefix!"

---

### 3. Framework Philosophy Alignment

**"Ask, Don't Assume":**
- ✅ Applied to argument parsing
- ✅ Validates user intent interactively
- ✅ No silent failures or assumptions

**Evidence-Based:**
- ✅ User confirms what they meant
- ✅ No guessing at intent

**Defensive Programming:**
- ✅ Validates before executing
- ✅ Catches errors early
- ✅ Prevents downstream failures

---

### 4. Backwards Compatible

**Correct usage:** No interaction needed
```
> /qa STORY-001 deep
[No questions, executes immediately]
```

**Incorrect usage:** Asks for clarification
```
> /qa STORY-001 --mode=deep
[Asks mode, educates, continues]
```

**Degrades gracefully:**
- Expert users: Fast (no questions)
- New users: Helpful (guides through errors)
- Typos: Caught and corrected

---

### 5. Self-Documenting

**Users don't need to memorize syntax:**
- Options show available values
- Notes show correct format
- Examples demonstrate usage
- Error messages are educational

**Example:**
```
Show correct /qa syntax:
  /qa [STORY-ID] [mode]

  Examples:
    /qa STORY-001           # Defaults to deep
    /qa STORY-001 deep      # Explicit deep
    /qa STORY-001 light     # Explicit light

  Mode values: 'deep' or 'light' (no -- prefix)
```

---

## When to Use This Pattern

### Use Phase 0 Validation When:
- ✅ Command accepts story IDs or other structured arguments
- ✅ Command has optional arguments (mode, environment, etc.)
- ✅ User input could be malformed or ambiguous
- ✅ Command invokes Skills (parameter passing context is critical)
- ✅ Command is user-facing (needs good UX)

### Skip Phase 0 Validation When:
- ❌ Command takes free-form text only (e.g., commit messages)
- ❌ Command has no arguments
- ❌ Command is internal/developer-only (not user-facing)
- ❌ Validation would add no value

---

## Checklist for Implementation

When adding Phase 0 validation to a slash command:

- [ ] Extract story ID using $1 (not $ARGUMENTS)
- [ ] Validate story ID format (regex: ^STORY-[0-9]+$)
- [ ] Validate story file exists (use Glob)
- [ ] Parse optional arguments ($2, $3) with flag handling
- [ ] Use AskUserQuestion for all ambiguous input
- [ ] Provide educational notes for incorrect syntax
- [ ] Display validation summary before proceeding
- [ ] Handle all error cases (missing ID, file not found, invalid mode/env)
- [ ] Test with correct usage (no questions)
- [ ] Test with incorrect usage (questions, recovery)

---

## Related Documentation

**Official Sources:**
- `.ai_docs/Terminal/slash-commands.md` - Argument handling
- `.ai_docs/claude-skills.md` - Skills cannot accept parameters

**DevForgeAI Patterns:**
- `CLAUDE.md` - "Ask, Don't Assume" principle
- `devforgeai/context/anti-patterns.md` - Copy-paste warnings
- `.claude/memory/skills-reference.md` - Skill invocation patterns

**RCA Context:**
- `devforgeai/specs/enhancements/RCA-005-skill-parameter-passing.md` - Root cause analysis
- `devforgeai/specs/enhancements/RCA-005-command-audit.md` - Audit findings
- `devforgeai/specs/enhancements/RCA-005-slash-command-parameter-fix-plan.md` - Implementation plan

---

## Pattern Evolution

**Version 1.0 (2025-11-02):**
- Initial pattern created from RCA-005 findings
- Incorporates user suggestion for AskUserQuestion on unknown flags
- Applied to 5 DevForgeAI commands (dev, qa, release, orchestrate, create-ui)
- Tested with multiple scenarios

**Future Enhancements:**
- Auto-correction for common typos (story-001 → STORY-001)
- Fuzzy matching for story IDs (ST001 → STORY-001)
- Recent story suggestions (show last 5 accessed)
- Sprint context awareness (show current sprint stories)

---

**This pattern transforms error-prone commands into user-friendly, self-teaching workflows that align perfectly with DevForgeAI's "Ask, Don't Assume" philosophy.**
