# Skill Generation Rules for MCP-CLI Converter

**Purpose:** Rules and patterns for auto-generating Claude Code skills from MCP server analysis.

**Context:** When the MCP-CLI converter generates a CLI wrapper from an MCP server, it also generates a complementary skill (SKILL.md) so Claude Code understands how to use the CLI effectively. This document defines the rules for skill generation.

---

## Overview

The converter automatically generates skills following Anthropic's official skill specification and DevForgeAI best practices. The generated skill serves as an "onboarding guide" that transforms Claude from a general-purpose agent into a specialist equipped with knowledge of the specific CLI tool.

---

## Skill Anatomy for Generated CLIs

Every generated skill consists of:

```
<cli-name>-cli/
├── skill/
│   ├── SKILL.md (required) - Entry point with YAML frontmatter + markdown
│   ├── references/ (optional)
│   │   ├── cli_reference.md - Complete command reference
│   │   └── usage_examples.md - Pattern-specific examples
│   ├── scripts/ (optional)
│   │   └── setup.sh - Installation script
│   └── assets/ (optional)
│       └── error_codes.md - Error code reference
```

---

## SKILL.md Structure

### Required YAML Frontmatter

```yaml
---
name: <cli-name>-cli
description: <One-sentence description of what the CLI does>. Use for: (1) <Primary use case>, (2) <Secondary use case>, (3) <Tertiary use case>. <Pattern-specific usage instruction>.
---
```

**Frontmatter Rules:**

1. **name**: Must match CLI directory name, use kebab-case, end with `-cli`
2. **description**:
   - Max 1024 characters
   - Third-person voice: "This CLI..." or "Use for..." (NOT "You can use...")
   - Enumerate 3-5 primary use cases with numbered list: "(1) ..., (2) ..., (3) ..."
   - Include pattern-specific guidance (session management, format flags, etc.)
   - Be specific about WHEN to use the skill

**Example:**
```yaml
---
name: puppeteer-cli
description: Browser automation CLI for web scraping and testing. Use for: (1) Navigating to websites, (2) Taking screenshots of pages, (3) Extracting text content, (4) Clicking elements and interacting with pages. Requires session management - create session before running commands.
---
```

---

### Markdown Body Structure

Generated SKILL.md follows this structure:

```markdown
# <CLI Name> CLI

Pattern: <API-WRAPPER | STATE-BASED | CUSTOM>

## [Pattern-Specific Section]

[Session Management OR Direct Usage section based on pattern]

## Available Commands

[Command reference with syntax and examples]

## [Pattern-Specific Features]

[Error handling, output formats, session lifecycle, etc.]

## Common Patterns

[Usage patterns and workflows]

## Troubleshooting

[Common errors and solutions]
```

---

## Pattern-Specific Generation Rules

### Pattern 1: API Wrapper (Stateless)

**Detection Criteria:**
- No state management detected
- Independent tool calls
- No session/connection requirements
- Typically external API wrappers

**SKILL.md Template:**

```markdown
# <Name> CLI

Pattern: API-WRAPPER (Stateless)

## Direct Usage

Each command is independent - no session needed. Run commands directly:

```bash
<cli-name> <command> [options] --format json
```

## Available Commands

- `<command-1> --param VALUE` - Description
- `<command-2> --param VALUE` - Description
- `<command-3> --param VALUE` - Description

## Output Format

All commands return JSON by default:

```json
{
  "status": "success",
  "data": {
    // Command-specific data
  }
}
```

Use `--format text` for plain text output.

Exit codes: 0 = success, non-zero = error.

## Examples

```bash
# Example 1
<cli-name> <command> --param "value"

# Example 2
<cli-name> <command> --param "value" --format json
```

## Error Handling

Common errors:
- **Exit 2**: Invalid arguments
- **Exit 3**: Timeout
- **Exit 5**: Authentication failed
- **Exit 6**: Rate limited

See `assets/error_codes.md` for complete reference.
```

**Description Pattern:**
```
<Service/API name> CLI for <domain>. Use for: (1) <Primary action>, (2) <Secondary action>, (3) <Tertiary action>. Each command is independent - no session needed.
```

---

### Pattern 2: State-Based (Stateful)

**Detection Criteria:**
- Session management detected
- State keywords: "connect", "session", "transaction", "browser", "page"
- Sequential operations
- Context-dependent commands

**SKILL.md Template:**

```markdown
# <Name> CLI

Pattern: STATE-BASED

## Session Management

Sessions maintain state across commands. Create a session before running commands:

```bash
SESSION=$(<cli-name> session create --name "job-name")
<cli-name> <command> --session $SESSION [options]
<cli-name> session destroy --session $SESSION
```

## Session Commands

- `session create [--name STRING]` - Create session → SESSION_ID
- `session destroy --session SESSION_ID` - Close session
- `session list` - List active sessions → JSON array

## Available Commands

All commands require `--session SESSION_ID`:

- `<command-1> --session ID --param VALUE` - Description
- `<command-2> --session ID --param VALUE` - Description
- `<command-3> --session ID --param VALUE` - Description

## Session Lifecycle

Sessions maintain <state-type> and context:

1. **Create**: `session create` → SESSION_ID
2. **Use**: Pass `--session SESSION_ID` to commands
3. **Destroy**: `session destroy --session SESSION_ID` (cleanup)

Sessions timeout after 1 hour of inactivity.

## Examples

```bash
# Create session
SESSION=$(<cli-name> session create)

# Run commands within session
<cli-name> <command-1> --session $SESSION --param "value"
<cli-name> <command-2> --session $SESSION

# Clean up
<cli-name> session destroy --session $SESSION
```

## Output Formats

- `--format json` - Structured JSON (default)
- `--format text` - Plain text
- `--format base64` - Binary data (images, files)

## Troubleshooting

**Session not found:**
- Session expired (1 hour timeout)
- Session destroyed
- Run `session list` to see active sessions

**State conflicts:**
- Ensure operations run in sequence
- Don't run parallel commands in same session
```

**Description Pattern:**
```
<Service name> CLI for <domain>. Use for: (1) <Primary action>, (2) <Secondary action>, (3) <Tertiary action>. Requires session management - create session before running commands.
```

---

### Pattern 3: Custom (Complex)

**Detection Criteria:**
- Pattern detection confidence < 0.8
- Custom adapter provided
- Hybrid stateless/stateful operations
- Complex orchestration logic

**SKILL.md Template:**

```markdown
# <Name> CLI

Pattern: CUSTOM

This CLI implements custom orchestration with <specific features>.

## Usage Overview

<High-level description of how to use the CLI>

## Commands

- `<command-1> [--param VALUE]` - Description
- `<command-2> [--param VALUE]` - Description
- `<command-3> [--param VALUE]` - Description

## Special Features

<Document any custom features like caching, streaming, etc.>

## Examples

```bash
# Example workflow
<cli-name> <command-1> --param "value"
<cli-name> <command-2> --param "value"
```

## Notes

<Any important notes about behavior, limitations, or best practices>
```

**Description Pattern:**
```
<Service name> CLI with <custom features>. Use for: (1) <Primary action>, (2) <Secondary action>, (3) <Tertiary action>. <Pattern-specific guidance>.
```

---

## Writing Style Rules

### Imperative/Infinitive Form (Required)

**Use verb-first instructions, NOT second person:**

✅ **Correct:**
- "Create a session before running commands"
- "To execute commands, use the following syntax"
- "Run `session destroy` when done"
- "Pass `--session ID` to all commands"

❌ **Incorrect:**
- "You should create a session"
- "You can execute commands"
- "You need to run `session destroy`"
- "You must pass `--session ID`"

### Progressive Disclosure

**Three-level loading system:**

1. **Metadata (name + description)** - Always in context (~100 words)
2. **SKILL.md body** - When skill triggers (<300 words recommended)
3. **Reference files** - As needed by Claude (unlimited)

**Keep SKILL.md lean:**
- Overview and workflow guidance in SKILL.md
- Detailed command reference in `references/cli_reference.md`
- Pattern-specific examples in `references/usage_examples.md`
- Error codes in `assets/error_codes.md`

### No Duplication

**Information lives in ONE place:**
- Quick reference in SKILL.md
- Detailed reference in `references/cli_reference.md`
- Don't repeat command syntax in both locations

**Example of proper separation:**

**SKILL.md:**
```markdown
## Available Commands

- `navigate --session ID --url URL` - Navigate to webpage
- `screenshot --session ID [--selector SELECTOR]` - Capture page/element
```

**references/cli_reference.md:**
```markdown
### navigate

**Syntax:**
```bash
<cli-name> navigate --session SESSION_ID --url URL [--wait-for SELECTOR]
```

**Parameters:**
- `--session SESSION_ID` (required) - Session identifier
- `--url URL` (required) - Target URL
- `--wait-for SELECTOR` (optional) - CSS selector to wait for

**Returns:**
```json
{"status": "success", "url": "https://...", "loaded": true}
```

**Errors:**
- Exit 3: Timeout waiting for page load
- Exit 4: Invalid URL
```

---

## Auto-Generation Logic

### Step 1: Analyze MCP Server

Extract from `mcp_analysis.json`:
- Detected pattern (api-wrapper, state-based, custom)
- Tool list with inputs/outputs
- State management requirements
- Async/sync characteristics

### Step 2: Generate SKILL.md Frontmatter

```python
def generate_frontmatter(analysis):
    """Generate YAML frontmatter from analysis."""

    name = f"{analysis['mcp_type']}-cli"

    # Extract primary use cases from tools
    use_cases = []
    for tool in analysis['tools'][:3]:  # Top 3 tools
        use_cases.append(format_use_case(tool))

    # Pattern-specific guidance
    if analysis['detected_pattern'] == 'state-based':
        pattern_note = "Requires session management - create session before running commands."
    elif analysis['detected_pattern'] == 'api-wrapper':
        pattern_note = "Each command is independent - no session needed."
    else:
        pattern_note = "See documentation for usage patterns."

    description = f"{analysis['service_name']} CLI for {analysis['domain']}. "
    description += f"Use for: {', '.join([f'({i+1}) {uc}' for i, uc in enumerate(use_cases)])}. "
    description += pattern_note

    return {
        'name': name,
        'description': description[:1024]  # Enforce max length
    }
```

### Step 3: Generate Body Sections

**Session Management Section (if state-based):**
```python
if pattern == 'state-based':
    skill_md += """
## Session Management

Sessions maintain state across commands. Create a session before running commands:

```bash
SESSION=$({cli_name} session create --name "job-name")
{cli_name} <command> --session $SESSION [options]
{cli_name} session destroy --session $SESSION
```
"""
```

**Available Commands Section:**
```python
def generate_commands_section(tools):
    """Generate command reference from tools."""

    commands = []
    for tool in tools:
        cmd_name = tool['name'].replace('_', '-')

        # Build parameter list
        params = []
        if 'session_required' in tool:
            params.append('--session ID')
        for param, type in tool['inputs'].items():
            params.append(f'--{param} {type.upper()}')

        syntax = f"{cmd_name} {' '.join(params)}"
        description = tool.get('description', 'No description')

        commands.append(f"- `{syntax}` - {description}")

    return '\n'.join(commands)
```

### Step 4: Generate Reference Files

**references/cli_reference.md:**
- Complete command reference
- Parameter details
- Return value schemas
- Error conditions

**references/usage_examples.md:**
- Pattern-specific workflows
- Real-world usage scenarios
- Multi-command sequences

**assets/error_codes.md:**
- Exit code reference
- Error messages
- Recovery strategies

---

## Output Format Documentation

### JSON Format (Default)

```json
{
  "status": "success" | "error",
  "command": "<command-name>",
  "data": {
    // Command-specific response
  },
  "error": "Error message" // Only if status=error
}
```

### Text Format

Plain text output for single values or human-readable results.

### Base64 Format

For binary data (images, PDFs, files):
- Outputs base64-encoded string to stdout
- Use for screenshot, file download, etc.

---

## Error Handling Documentation

### Standard Exit Codes

```markdown
## Error Handling

Exit codes follow Unix conventions:

- **0** - Success
- **1** - General error
- **2** - Invalid arguments/usage
- **3** - Timeout
- **4** - Resource unavailable
- **5** - Authentication failed
- **6** - Rate limited
- **7** - Permission denied

**stderr** contains human-readable error messages with recovery hints.

See `assets/error_codes.md` for complete reference.
```

---

## Session Management Documentation

### For State-Based CLIs

```markdown
## Session Lifecycle

Sessions maintain <state-type> and context:

1. **Create**: `session create [--name STRING]` → SESSION_ID
   - Returns unique identifier
   - Optional name for tracking

2. **Use**: Pass `--session SESSION_ID` to all commands
   - Session maintains state between commands
   - Operations execute in sequence

3. **Destroy**: `session destroy --session SESSION_ID`
   - Cleanup resources
   - Required to prevent resource leaks

**Session Management:**
- `session create [--name STRING]` - Create new session
- `session list` - List active sessions
- `session destroy --session ID` - Close session
- `session info --session ID` - Get session details

**Timeouts:**
- Sessions expire after 1 hour of inactivity
- Destroyed sessions cannot be recovered
- Run `session list` to see active sessions
```

---

## Quality Checklist for Generated Skills

Before finalizing generated skill, verify:

- [ ] **Frontmatter complete**: name, description (< 1024 chars)
- [ ] **Description follows pattern**: "(1) ..., (2) ..., (3) ..."
- [ ] **Imperative language**: No "you should", "you can"
- [ ] **Pattern documented**: API-WRAPPER, STATE-BASED, or CUSTOM
- [ ] **Commands listed**: All MCP tools mapped to CLI commands
- [ ] **Session management**: Documented if state-based
- [ ] **Output formats**: JSON, text, base64 documented
- [ ] **Error handling**: Exit codes documented
- [ ] **Examples provided**: Real-world usage scenarios
- [ ] **Progressive disclosure**: Detailed content in references/
- [ ] **No duplication**: Content lives in one place
- [ ] **File structure**: SKILL.md, references/, scripts/, assets/

---

## Integration with Converter Workflow

```
1. MCP Analysis
   ├─ Detect pattern (api-wrapper, state-based, custom)
   ├─ Extract tools, inputs, outputs
   └─ Determine state management needs

2. Skill Generation
   ├─ Generate SKILL.md frontmatter
   ├─ Generate body sections (pattern-specific)
   ├─ Generate command reference
   └─ Generate examples

3. Reference Generation
   ├─ references/cli_reference.md (detailed commands)
   ├─ references/usage_examples.md (workflows)
   ├─ scripts/setup.sh (installation)
   └─ assets/error_codes.md (error reference)

4. Validation
   ├─ Verify YAML frontmatter
   ├─ Check description length
   ├─ Validate imperative language
   └─ Ensure no duplication
```

---

## Examples by Pattern

### API Wrapper Example (OpenWeather)

**MCP Tools:**
- `getForecast(location, days, units)`
- `getCurrentWeather(location, units)`
- `searchCities(query)`

**Generated SKILL.md:**
```markdown
---
name: openweather-cli
description: Weather data CLI using OpenWeather API. Use for: (1) Getting weather forecasts, (2) Checking current conditions, (3) Searching locations. Each command is independent - no session needed.
---

# OpenWeather CLI

Pattern: API-WRAPPER (Stateless)

## Direct Usage

Each command is independent - no session needed:

```bash
openweather-cli get-forecast --location "Seattle" --days 5
openweather-cli get-current-weather --location "Seattle"
```

## Available Commands

- `get-forecast --location CITY [--days N] [--units metric|imperial]` - Get weather forecast
- `get-current-weather --location CITY [--units metric|imperial]` - Get current conditions
- `search-cities --query SEARCH_STRING` - Search for cities

## Output Format

All commands return JSON:

```json
{
  "status": "success",
  "data": {
    // Weather data
  }
}
```

Exit code 0 = success, non-zero = error.
```

---

### State-Based Example (Puppeteer)

**MCP Tools:**
- `navigate(url)`
- `screenshot(selector)`
- `click(selector)`
- `extractText(selector)`

**Generated SKILL.md:**
```markdown
---
name: puppeteer-cli
description: Browser automation CLI for web scraping and testing. Use for: (1) Navigating to websites, (2) Taking screenshots of pages, (3) Extracting text content, (4) Clicking elements and interacting with pages. Requires session management - create session before running commands.
---

# Puppeteer Browser Automation CLI

Pattern: STATE-BASED

## Session Management

Create a browser session before running commands:

```bash
SESSION=$(puppeteer-cli session create --name "scrape-job")
puppeteer-cli navigate --session $SESSION --url "https://example.com"
puppeteer-cli session destroy --session $SESSION
```

## Session Commands

- `session create [--name STRING]` - Create session → SESSION_ID
- `session destroy --session SESSION_ID` - Close session
- `session list` - List active sessions

## Available Commands

All commands require `--session SESSION_ID`:

- `navigate --session ID --url URL` - Navigate to webpage
- `screenshot --session ID [--selector SELECTOR]` - Capture page/element
- `click --session ID --selector SELECTOR` - Click element
- `extract-text --session ID --selector SELECTOR` - Get text content

## Session Lifecycle

Sessions maintain browser state and context:

1. **Create**: `session create` → SESSION_ID
2. **Use**: Pass `--session SESSION_ID` to commands
3. **Destroy**: `session destroy --session SESSION_ID` (cleanup)

Sessions timeout after 1 hour of inactivity.
```

---

## Common Pitfalls to Avoid

### ❌ Second-Person Language

**Wrong:**
```markdown
You should create a session before running commands. You can then execute commands by passing the session ID.
```

**Correct:**
```markdown
Create a session before running commands. Execute commands by passing the session ID.
```

---

### ❌ Duplicating Content

**Wrong:**
```markdown
## Available Commands

- `navigate --session ID --url URL` - Navigate to webpage
  - Parameters: session (required), url (required)
  - Returns: JSON with navigation result
  - Errors: Exit 3 on timeout

[Same content repeated in references/cli_reference.md]
```

**Correct:**
```markdown
## Available Commands

- `navigate --session ID --url URL` - Navigate to webpage

See `references/cli_reference.md` for detailed parameter reference.
```

---

### ❌ Missing Pattern-Specific Guidance

**Wrong:**
```markdown
description: Browser automation CLI. Use for navigating, screenshots, and text extraction.
```

**Correct:**
```markdown
description: Browser automation CLI for web scraping and testing. Use for: (1) Navigating to websites, (2) Taking screenshots of pages, (3) Extracting text content. Requires session management - create session before running commands.
```

---

## Summary

Auto-generated skills follow these principles:

1. **Progressive disclosure** - SKILL.md is lean entry point, details in references/
2. **Imperative language** - Verb-first instructions, no second person
3. **Pattern-specific** - Template varies by API-WRAPPER, STATE-BASED, or CUSTOM
4. **Complete but concise** - All commands documented, detailed syntax in references
5. **Discoverable** - Clear use cases in description, examples provided
6. **Error-friendly** - Exit codes documented, troubleshooting included
7. **No duplication** - Each piece of information lives in one place
8. **Quality validated** - Frontmatter, structure, and language verified

The goal: Transform Claude from general-purpose agent to specialist who knows exactly how to use the generated CLI.
