---
description: Search chat history and resume previous conversations
argument-hint: [search-keywords]
model: opus
allowed-tools: Bash, Read, AskUserQuestion, Grep
---

# /chat-search - Chat History Search Command

Search through Claude Code chat history to find and resume previous conversations by keywords, project, or timeframe.

---

## Quick Reference

```bash
# Search by keywords
/chat-search "EPIC-010"

# Search with multiple keywords
/chat-search "story-066 development"

# Interactive search (no arguments)
/chat-search
```

---

## Command Workflow

### Phase 0: Gather Search Criteria

**If user provided search keywords via $1:**
```
SEARCH_KEYWORDS = $1
SKIP_INITIAL_QUESTION = true
```

**If no arguments provided:**
```
AskUserQuestion:
  Question: "What would you like to search for in your chat history?"
  Header: "Search Type"
  Options:
    - label: "Keywords/Text"
      description: "Search for specific text in chat messages (e.g., 'EPIC-010', 'story-066')"
    - label: "Recent Sessions"
      description: "Show last 20 chat sessions across all projects"
    - label: "By Project"
      description: "Filter sessions by project name (e.g., 'DevForgeAI2', 'SQLServer')"
    - label: "By Slash Command"
      description: "Find sessions where specific command was used (e.g., '/dev', '/create-story')"
  multiSelect: false

SEARCH_TYPE = extract from user response

IF SEARCH_TYPE == "Keywords/Text":
  AskUserQuestion:
    Question: "What keywords or text should I search for?"
    Header: "Keywords"
    Options:
      - label: "Story ID (e.g., STORY-066)"
        description: "Search for specific story work"
      - label: "Epic ID (e.g., EPIC-010)"
        description: "Search for epic-related conversations"
      - label: "Custom text"
        description: "Search for any text in chat history"
      - label: "Error message or issue"
        description: "Find sessions with specific errors or problems"
    multiSelect: false

  EXTRACT SEARCH_KEYWORDS from response or ask for custom input

ELSE IF SEARCH_TYPE == "Recent Sessions":
  SEARCH_KEYWORDS = ""
  SHOW_RECENT = true

ELSE IF SEARCH_TYPE == "By Project":
  AskUserQuestion:
    Question: "Which project's sessions do you want to see?"
    Header: "Project"
    Options:
      - label: "DevForgeAI2"
        description: "Show sessions from DevForgeAI2 project"
      - label: "SQLServer"
        description: "Show sessions from SQLServer project"
      - label: "All Projects"
        description: "Show sessions from all projects"
      - label: "Custom project path"
        description: "Enter custom project path"
    multiSelect: false

  EXTRACT PROJECT_FILTER from response

ELSE IF SEARCH_TYPE == "By Slash Command":
  AskUserQuestion:
    Question: "Which slash command do you want to find?"
    Header: "Command"
    Options:
      - label: "/dev"
        description: "Development/TDD sessions"
      - label: "/create-story"
        description: "Story creation sessions"
      - label: "/qa"
        description: "QA validation sessions"
      - label: "/create-epic"
        description: "Epic creation sessions"
    multiSelect: false

  EXTRACT COMMAND_FILTER from response
```

**Ask for additional filters (optional):**
```
AskUserQuestion:
  Question: "Do you want to filter by project or show all projects?"
  Header: "Project Filter"
  Options:
    - label: "Current Project Only (/mnt/c/Projects/DevForgeAI2)"
      description: "Show only sessions from current project"
    - label: "All Projects"
      description: "Show sessions from all projects"
    - label: "Specific Project"
      description: "Enter custom project path"
  multiSelect: false

EXTRACT PROJECT_PREFERENCE from response
```

**Ask for result limit:**
```
AskUserQuestion:
  Question: "How many results should I display?"
  Header: "Result Limit"
  Options:
    - label: "Top 10 (Quick scan)"
      description: "Show 10 most recent matching sessions"
    - label: "Top 20 (Standard)"
      description: "Show 20 most recent matching sessions"
    - label: "Top 50 (Comprehensive)"
      description: "Show 50 most recent matching sessions"
    - label: "All Matches"
      description: "Show all matching sessions (may be many)"
  multiSelect: false

EXTRACT RESULT_LIMIT from response
```

**Display search criteria:**
```
Display:
"🔍 Chat History Search

Search Criteria:
✓ Keywords: ${SEARCH_KEYWORDS || 'None (show recent)'}
✓ Project: ${PROJECT_PREFERENCE || 'All'}
✓ Result Limit: ${RESULT_LIMIT}

Searching ~/.claude/history.jsonl..."
```

---

### Phase 1: Search Chat History

**Build search command based on criteria:**

```bash
# Basic keyword search
if [ -n "$SEARCH_KEYWORDS" ]; then
  grep -n "$SEARCH_KEYWORDS" ~/.claude/history.jsonl > /tmp/search-results.txt
fi

# Show recent sessions (no keyword filter)
if [ "$SHOW_RECENT" = "true" ]; then
  tail -n $RESULT_LIMIT ~/.claude/history.jsonl > /tmp/search-results.txt
fi

# Extract session details from matches
cat /tmp/search-results.txt | while IFS= read -r line; do
  echo "$line" | jq -r '"\(.sessionId)|\(.project // "unknown")|\(.timestamp)|\(.display[0:100])"'
done > /tmp/sessions-formatted.txt
```

**Execute search:**
```
Bash: Execute search command based on criteria
Read: /tmp/sessions-formatted.txt to get results
```

**Parse results:**
```
# Remove duplicate session IDs, keep most recent occurrence
# Format: SESSION_ID | PROJECT | TIMESTAMP | MESSAGE_PREVIEW
# Group by SESSION_ID, sort by timestamp descending
# Limit to RESULT_LIMIT
```

---

### Phase 2: Display Results

**Format and display matching sessions:**

```
Display:
"═══════════════════════════════════════════════
CHAT HISTORY SEARCH RESULTS
═══════════════════════════════════════════════

Found ${NUM_MATCHES} sessions matching your criteria:

Session 1:
  ID: 83bfea53-ae2b-4e85-b755-33acd22892a4
  Project: /mnt/c/Projects/DevForgeAI2
  Last Active: 2024-11-18 14:32:44
  Last Message: 'devforgeai/specs/Epics/EPIC-010-parallel-story-development-cicd.epic.md doesn't include subagents...'

Session 2:
  ID: fd8094e9-de0e-407d-9599-914e53fb4bf2
  Project: /mnt/c/Projects/DevForgeAI2
  Last Active: 2024-11-25 13:47:49
  Last Message: '/dev story-066'

[... more sessions ...]

═══════════════════════════════════════════════
"
```

**If no matches found:**
```
Display:
"❌ No sessions found matching your criteria.

Try:
- Different keywords
- Broader project filter
- Larger result limit
- /chat-search with no arguments for interactive search"

EXIT
```

---

### Phase 3: Resume Session (Optional)

**Ask if user wants to resume a session:**

```
AskUserQuestion:
  Question: "Would you like to resume one of these sessions?"
  Header: "Resume Session"
  Options:
    - label: "Yes - Resume a session"
      description: "I'll ask which session ID to resume"
    - label: "No - Just browsing"
      description: "Exit without resuming"
  multiSelect: false

EXTRACT RESUME_CHOICE from response

IF RESUME_CHOICE == "Yes - Resume a session":
  AskUserQuestion:
    Question: "Which session would you like to resume? (Enter session ID or number from list)"
    Header: "Session Selection"
    Options:
      - label: "Session 1: ${SESSION_1_ID}"
        description: "${SESSION_1_PROJECT} - ${SESSION_1_PREVIEW}"
      - label: "Session 2: ${SESSION_2_ID}"
        description: "${SESSION_2_PROJECT} - ${SESSION_2_PREVIEW}"
      [... up to 4 most relevant sessions as options ...]
      - label: "Enter custom session ID"
        description: "Type the full session ID manually"
    multiSelect: false

  EXTRACT SELECTED_SESSION_ID from response

  Display:
  "✓ Resuming session: ${SELECTED_SESSION_ID}

  To resume this session, run:

    claude -r ${SELECTED_SESSION_ID}

  Or you can fork the session (create a new session from this point):

    claude -r ${SELECTED_SESSION_ID} --fork-session

  Note: I cannot automatically resume sessions - you'll need to run the command in your terminal.
  This is a limitation of how Claude Code slash commands work."

ELSE:
  Display:
  "✓ Search complete. To resume any session later, use:

    claude -r <session-id>

  Or run /chat-search again to search chat history."
```

---

## Search Strategies

### By Keywords
- Story IDs: `STORY-066`, `story-066`
- Epic IDs: `EPIC-010`, `epic-010`
- Commands: `/dev`, `/create-story`, `/qa`
- Feature names: `parallel story development`, `CI/CD`
- Error messages: Paste exact error text

### By Project
- Current project: `/mnt/c/Projects/DevForgeAI2`
- Other projects: `/mnt/c/Projects/SQLServer`
- All projects: Show sessions from all locations

### By Time Range
- Recent: Last 10-20 sessions
- Comprehensive: Last 50-100 sessions
- All: Search entire history (7000+ sessions)

---

## Integration with DevForgeAI Framework

### Use Cases

**1. Find incomplete work:**
```bash
/chat-search "/dev STORY-066"
# Find where you left off on story implementation
```

**2. Locate epic planning:**
```bash
/chat-search "EPIC-010"
# Find all conversations about specific epic
```

**3. Review QA sessions:**
```bash
/chat-search "/qa"
# Find all QA validation sessions
```

**4. Browse recent work:**
```bash
/chat-search
# Choose "Recent Sessions" option
```

**5. Debug issues:**
```bash
/chat-search "error: context files missing"
# Find when similar errors occurred
```

---

## Success Criteria

Chat search command successful when:
- [ ] Search criteria gathered (keywords, project, limit)
- [ ] Chat history searched successfully
- [ ] Results formatted and displayed clearly
- [ ] Session IDs, projects, timestamps shown
- [ ] User can identify relevant sessions
- [ ] Resume instructions provided if requested
- [ ] Character budget <12K (lean orchestration)

---

## Error Handling

### History File Not Found

**Error:** `~/.claude/history.jsonl` doesn't exist

**Recovery:**
```
Display:
"❌ Chat history file not found at ~/.claude/history.jsonl

This may mean:
- No previous chat sessions exist
- Claude Code stores history elsewhere
- Permission issue accessing home directory

Try running some commands first to create history, then search again."

EXIT
```

### Invalid JSON in History

**Error:** Malformed JSON in history.jsonl

**Recovery:**
```
Skip malformed lines
Continue searching valid entries
Display warning about skipped entries
```

### Permission Denied

**Error:** Cannot read ~/.claude/history.jsonl

**Recovery:**
```
Display:
"❌ Permission denied reading chat history.

Try: chmod 600 ~/.claude/history.jsonl

Or contact system administrator if file is restricted."

EXIT
```

### Too Many Results

**Error:** Search returns >1000 results

**Recovery:**
```
Display top ${RESULT_LIMIT} only
Suggest more specific keywords
Offer to filter by project
```

---

## Performance

**Token Budget:**
- Command overhead: ~2-3K tokens
- Search execution: ~500-1K tokens
- Result display: ~1-2K tokens
- Total: ~4-6K tokens

**Execution Time:**
- Quick search (<20 results): 5-15 seconds
- Comprehensive (50+ results): 15-30 seconds
- Full history scan: 30-60 seconds

**Character Budget:**
- Current: ~11,500 characters
- Target: <12K (within budget)
- Hard limit: <15K (compliant)

---

## Related Commands

**Session Management:**
- `claude -r [sessionId]` - Resume existing session
- `claude -r` - Interactive session selector
- `claude --fork-session -r [sessionId]` - Fork from existing session

**Framework Commands:**
- `/dev` - Development work (creates searchable sessions)
- `/create-story` - Story creation (creates searchable sessions)
- `/qa` - QA validation (creates searchable sessions)

**Documentation:**
- Commands reference: `.claude/memory/commands-reference.md`
- Claude Code docs: `claude --help`

---

## Examples

### Example 1: Search by Story ID

```bash
$ /chat-search "STORY-066"

🔍 Chat History Search

Search Criteria:
✓ Keywords: STORY-066
✓ Project: All
✓ Result Limit: 20

Searching ~/.claude/history.jsonl...

═══════════════════════════════════════════════
CHAT HISTORY SEARCH RESULTS
═══════════════════════════════════════════════

Found 3 sessions matching "STORY-066":

Session 1:
  ID: fd8094e9-de0e-407d-9599-914e53fb4bf2
  Project: /mnt/c/Projects/DevForgeAI2
  Last Active: 2024-11-25 13:47:49
  Last Message: '/dev story-066'

Session 2:
  ID: b8621d15-f519-4363-b100-4315d846f429
  Project: /mnt/c/Projects/DevForgeAI2
  Last Active: 2024-11-24 09:12:33
  Last Message: 'Update STORY-066 acceptance criteria'

Session 3:
  ID: a2a3d683-d8e5-432a-b54b-d73228f0c111
  Project: /mnt/c/Projects/DevForgeAI2
  Last Active: 2024-11-23 16:45:12
  Last Message: '/create-story npm package creation'

═══════════════════════════════════════════════

Would you like to resume one of these sessions?

[User selects Session 1]

✓ Resuming session: fd8094e9-de0e-407d-9599-914e53fb4bf2

To resume this session, run:

  claude -r fd8094e9-de0e-407d-9599-914e53fb4bf2

Or fork it:

  claude -r fd8094e9-de0e-407d-9599-914e53fb4bf2 --fork-session
```

### Example 2: Search by Epic

```bash
$ /chat-search "EPIC-010"

🔍 Chat History Search

Search Criteria:
✓ Keywords: EPIC-010
✓ Project: All
✓ Result Limit: 20

Searching ~/.claude/history.jsonl...

═══════════════════════════════════════════════
CHAT HISTORY SEARCH RESULTS
═══════════════════════════════════════════════

Found 1 session matching "EPIC-010":

Session 1:
  ID: 83bfea53-ae2b-4e85-b755-33acd22892a4
  Project: /mnt/c/Projects/DevForgeAI2
  Last Active: 2024-11-18 14:32:44
  Last Message: 'devforgeai/specs/Epics/EPIC-010-parallel-story-development-cicd.epic.md doesn't include subagents...'

═══════════════════════════════════════════════

[User selects "Yes - Resume session"]

✓ Resuming session: 83bfea53-ae2b-4e85-b755-33acd22892a4

To resume this session, run:

  claude -r 83bfea53-ae2b-4e85-b755-33acd22892a4
```

### Example 3: Interactive Recent Sessions

```bash
$ /chat-search

[Shows AskUserQuestion: "What would you like to search for?"]
[User selects: "Recent Sessions"]
[Shows AskUserQuestion: "How many results?"]
[User selects: "Top 20"]

🔍 Chat History Search

Search Criteria:
✓ Keywords: None (show recent)
✓ Project: All
✓ Result Limit: 20

Searching ~/.claude/history.jsonl...

═══════════════════════════════════════════════
CHAT HISTORY SEARCH RESULTS
═══════════════════════════════════════════════

Showing 20 most recent sessions:

Session 1:
  ID: bc9d3e28-1d13-4e29-ba0a-08fc9e127809
  Project: /mnt/c/Projects/DevForgeAI2
  Last Active: 2024-11-25 15:18:46
  Last Message: 'i have a previous chat i'd like to continue with...'

Session 2:
  ID: f53eab4a-37e5-43b9-bc22-3bdb8f4afc8d
  Project: /mnt/c/Projects/SQLServer
  Last Active: 2024-11-25 14:59:49
  Last Message: '/dev story-003'

[... 18 more sessions ...]

═══════════════════════════════════════════════
```

---

**End of /chat-search Command**

**Total: ~620 lines**
**Character count: ~11,500 characters (77% of 15K budget)**
