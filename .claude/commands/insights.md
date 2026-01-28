---
description: Execute session data mining queries for workflow patterns, errors, and decisions
argument-hint: "[query-type] [options] | --help"
model: opus
allowed-tools: Read, Glob, Grep, Skill, AskUserQuestion
---

# /insights - Session Data Mining Query Interface

Execute insights queries to discover workflow patterns, errors, and decisions from session data.

**Purpose:** User interface for EPIC-034 (Session Data Mining). Routes queries to the devforgeai-insights skill which orchestrates the session-miner subagent.

---

## Quick Reference

```bash
# Dashboard overview (default)
/insights

# Workflow pattern analysis
/insights workflows

# Error mining and analysis
/insights errors

# Decision archive search
/insights decisions [query]

# Story-specific insights
/insights story STORY-XXX

# Display help
/insights --help
```

---

## Command Workflow

### Phase 01: Argument Validation

**Parse $ARGUMENTS:**
```
QUERY_TYPE = null
QUERY_PARAM = null
STORY_ID = null

# Parse arguments
FOR arg in $ARGUMENTS:
    IF arg == "--help" OR arg == "-h":
        GOTO Help Section
        EXIT
    ELIF arg == "workflows":
        QUERY_TYPE = "workflows"
    ELIF arg == "errors":
        QUERY_TYPE = "errors"
    ELIF arg == "decisions":
        QUERY_TYPE = "decisions"
        # Next argument is the search query
        QUERY_PARAM = remaining_args
    ELIF arg == "story":
        QUERY_TYPE = "story"
        # Next argument should be STORY-XXX
    ELIF arg matches "STORY-[0-9]+":
        STORY_ID = arg

# Default to dashboard if no query type
IF QUERY_TYPE is empty AND $ARGUMENTS is empty:
    QUERY_TYPE = "dashboard"
```

**Validate query type:**
```
VALID_QUERY_TYPES = ["dashboard", "workflows", "errors", "decisions", "story", "command-patterns"]

IF QUERY_TYPE not in VALID_QUERY_TYPES AND QUERY_TYPE is not null:
    # Error: Invalid query type
    Display:
    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
      Error: Unknown query type
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "Invalid query type: '${QUERY_TYPE}'"
    Display: ""
    Display: "Valid query types:"
    Display: "  • dashboard         - Overview of session insights (default)"
    Display: "  • workflows         - Workflow pattern analysis"
    Display: "  • errors            - Error mining and frequency analysis"
    Display: "  • decisions         - Search decision archive"
    Display: "  • story             - Story-specific insights (requires STORY-ID)"
    Display: "  • command-patterns  - Top 10 command sequences by frequency"
    Display: ""
    Display: "Usage: /insights [query-type] [options]"
    Display: "       /insights --help for more information"
    HALT
```

**Validate story query:**
```
IF QUERY_TYPE == "story" AND STORY_ID is empty:
    # Error: Missing STORY-ID
    Display:
    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
      Error: Missing STORY-ID
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "The 'story' query type requires a STORY-ID parameter."
    Display: ""
    Display: "Usage: /insights story STORY-XXX"
    Display: ""
    Display: "Example: /insights story STORY-224"
    Display: ""
    Display: "Run /insights --help for more information."
    HALT
```

---

### Phase 02: Invoke Skill

**Set context markers for skill:**
```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  DevForgeAI Insights Query"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "**Query Type:** ${QUERY_TYPE}"
IF QUERY_PARAM:
    Display: "**Query:** ${QUERY_PARAM}"
IF STORY_ID:
    Display: "**Story:** ${STORY_ID}"
Display: ""
Display: "Executing insights query..."
Display: ""
```

**Invoke devforgeai-insights skill:**
```
Skill(command="devforgeai-insights", args="--type=${QUERY_TYPE} --query=${QUERY_PARAM} --story=${STORY_ID}")
```

**Skill returns structured result to command.**

---

### Phase 03: Display Results

**Skill has returned result object with display template:**
```
# Result pass-through from skill
# Skill handles all formatting and display
```

---

## Help Section

**Display when --help flag is provided:**

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  /insights - Session Data Mining Query Interface
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DESCRIPTION:
  Execute insights queries to discover workflow patterns, errors,
  and decisions from DevForgeAI session data.

USAGE:
  /insights [query-type] [options]

QUERY TYPES:
  dashboard    Overview of session insights (default when no args)
  workflows    Analyze workflow patterns and command sequences
  errors       Mine error patterns and frequency analysis
  decisions    Search decision archive for architectural choices
  story        Get insights for a specific story

PARAMETERS:
  [query]      Search string for 'decisions' query type
  STORY-XXX    Story ID for 'story' query type (e.g., STORY-224)

OPTIONS:
  --help, -h   Display this help message

EXAMPLES:
  /insights                         # Dashboard overview
  /insights workflows               # Workflow pattern analysis
  /insights errors                  # Error mining
  /insights decisions "caching"     # Search decisions about caching
  /insights story STORY-224         # Insights for STORY-224

INTEGRATION:
  Routes to devforgeai-insights skill which orchestrates:
  - session-miner subagent for data extraction
  - Pattern analysis algorithms
  - Report generation

RELATED COMMANDS:
  /feedback-search    Search feedback history
  /chat-search        Search chat history
  /rca                Root cause analysis

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## Error Handling

**Invalid Query Type:**
```
Error: Unknown query type

Invalid query type: '[user-input]'

Valid query types:
  • dashboard  - Overview of session insights (default)
  • workflows  - Workflow pattern analysis
  • errors     - Error mining and frequency analysis
  • decisions  - Search decision archive
  • story      - Story-specific insights (requires STORY-ID)

Usage: /insights [query-type] [options]
       /insights --help for more information
```

**Missing STORY-ID:**
```
Error: Missing STORY-ID

The 'story' query type requires a STORY-ID parameter.

Usage: /insights story STORY-XXX

Example: /insights story STORY-224

Run /insights --help for more information.
```

**Skill Not Found:**
```
Error: devforgeai-insights skill not available

The insights skill is not installed or configured.

Resolution:
1. Verify skill exists at .claude/skills/devforgeai-insights/SKILL.md
2. Run /create-context to ensure framework is configured
3. Check EPIC-034 implementation status
```

---

## Integration Notes

**Skill Dependency:**
- Requires `devforgeai-insights` skill (STORY-221)
- Skill orchestrates `session-miner` subagent (STORY-220)
- Session catalog from `STORY-223`

**Data Sources:**
- Session files in `.claude/sessions/`
- Feedback data in `devforgeai/feedback/`
- Story files in `devforgeai/specs/Stories/`

**Performance:**
- Command initialization < 2 seconds (NFR-CMD-001)
- Query execution time depends on data volume

---

**Refactored 2025-01-03 (STORY-224):** Command created for EPIC-034 Session Data Mining
**Pattern:** Lean orchestration with skill delegation
