# DevForgeAI Integration Guide

How `devforgeai-mcp-cli-converter` skill integrates into the DevForgeAI framework ecosystem.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    DevForgeAI Framework                     │
│                                                             │
│  /spec → /analyze → /plan → /build → /test → /validate    │
│                                                             │
│  Personas: Analyst, Architect, PM, Dev, QA                │
│                                                             │
└─────────────────┬───────────────────────────────────────────┘
                  │
                  ├─→ /convert-mcp-server [MCP]
                  │   │
                  │   └─→ Runs devforgeai-mcp-cli-converter skill
                  │       ├─ Analyzes MCP structure
                  │       ├─ Generates CLI wrapper
                  │       ├─ Generates skill docs
                  │       └─ Returns both as outputs
                  │
                  ├─→ Dev persona gets:
                  │   ├─ CLI tool ready to use
                  │   ├─ Skill documentation
                  │   └─ Tests/examples
                  │
                  └─→ QA persona validates:
                      ├─ CLI executes correctly
                      ├─ Skill matches CLI interface
                      └─ Tests pass
```

## Typical Workflow

### 1. Developer Discovers MCP

```
Dev: "I want to use the puppeteer-mcp for browser automation"
```

### 2. Trigger Conversion

```
/convert-mcp-server puppeteer-mcp \
  --source npm:mcp-puppeteer@latest
```

### 3. Analyzer Runs

The `MCPAnalyzer` from converter.py:
- Fetches npm package or local source
- Extracts tool definitions
- Detects state-based pattern
- Outputs analysis JSON

### 4. CLI Generated

`CLIGenerator` creates:
```
puppeteer-cli/
├── cli.py                    # Main entry point
├── adapters/
│   └── state_based_adapter.py  # Session management
├── utils/
│   ├── error_handler.py
│   └── output_formatter.py
├── requirements.txt
└── tests/
    └── test_cli.py
```

### 5. Skill Generated

`SkillGenerator` creates:
```
puppeteer-cli/skill/
├── SKILL.md                   # Claude knows how to use it
├── references/
│   ├── cli_reference.md      # Command documentation
│   └── usage_examples.md     # Example commands
├── scripts/
│   └── setup.sh              # Installation script
└── assets/
    └── error_codes.md        # Error reference
```

### 6. Dev Tests CLI

```bash
# Install
pip install -r puppeteer-cli/requirements.txt

# Create session
SESSION=$(python puppeteer-cli/cli.py session create --name "test")

# Use it
python puppeteer-cli/cli.py navigate --session $SESSION --url "https://example.com"
python puppeteer-cli/cli.py screenshot --session $SESSION --output /tmp/page.png

# Cleanup
python puppeteer-cli/cli.py session destroy --session $SESSION
```

### 7. QA Validates

QA persona has skill documentation and validates:
- CLI commands match skill interface ✓
- Error codes are correct ✓
- Session management works ✓
- Examples run successfully ✓

### 8. Framework Integration

```python
# In DevForgeAI state
available_skills = {
    "puppeteer-cli": {...},      # Auto-registered
    "weather-cli": {...},         # Previous conversions
    # ... etc
}

# Dev can now use in future conversations
/build "Create a web scraper"
# → Dev persona sees puppeteer-cli skill in context
```

---

## Slash Command Integration

### `/convert-mcp-server`

**Purpose**: Convert an MCP to CLI + skill

**Usage**:
```
/convert-mcp-server <mcp-name-or-package>
  [--source npm:package | --source ./local/path | --source ./schema.json]
  [--pattern api-wrapper|state-based|custom]
  [--adapter-script ./custom.py]
  [--force]
```

**Parameters**:

| Param | Type | Default | Notes |
|-------|------|---------|-------|
| mcp-name | string | required | Name or npm package spec |
| --source | string | required | Where to find the MCP |
| --pattern | enum | auto | Force pattern (skips detection) |
| --adapter-script | path | - | Custom adapter for 'custom' pattern |
| --force | flag | false | Skip confirmation, force conversion |

**Returns**:

```json
{
  "status": "success",
  "cli_generated": true,
  "skill_generated": true,
  "pattern": "state-based",
  "confidence": 0.95,
  "outputs": {
    "cli_dir": "./puppeteer-cli",
    "skill_dir": "./puppeteer-cli/skill",
    "test_command": "python puppeteer-cli/cli.py --help"
  },
  "next_steps": [
    "Run: pip install -r puppeteer-cli/requirements.txt",
    "Test: python puppeteer-cli/cli.py session create",
    "Review: cat puppeteer-cli/skill/SKILL.md"
  ]
}
```

---

## CLI Output Formats

Generated CLIs normalize to standard formats for Claude consumption.

### JSON (Default)

```bash
$ puppeteer-cli navigate --url "https://example.com" --format json
{
  "status": "success",
  "command": "navigate",
  "session": "abc123",
  "data": {
    "url": "https://example.com",
    "loaded": true
  }
}
```

### Text (Human-readable)

```bash
$ puppeteer-cli screenshot --session abc123 --format text
/tmp/screenshot-abc123.png
```

### Base64 (Binary data)

```bash
$ puppeteer-cli screenshot --session abc123 --format base64
iVBORw0KGgoAAAANSUhEUgAAAAEAAAA... [base64 encoded PNG]
```

Claude Code can:
- Parse JSON for structured data
- Read text output as file paths or status messages
- Decode base64 to write binary files or process images

---

## Skill Auto-Registration

After conversion, the skill is available in DevForgeAI context.

### Manual Registration

```bash
# Copy generated skill to DevForgeAI skills directory
cp -r puppeteer-cli/skill ~/.devforge/skills/puppeteer-cli.skill
```

### Auto-Discovery

Framework can scan output directory and register:
```python
# In DevForgeAI init
for skill_dir in outputs.glob("*/skill"):
    register_skill(skill_dir)
```

### In Subsequent Specs

```
Spec: Build a web scraper for product prices

Dev persona context:
- puppeteer-cli skill available
- Can use: puppeteer-cli navigate, screenshot, extract-text
- Session model understood
```

---

## Error Handling & Recovery

### CLI Error → Claude Recovery

Generated CLIs use standard Unix exit codes. Claude knows:

```
Exit 0   → Success, parse response
Exit 1   → General error, check stderr
Exit 2   → Bad arguments, review command syntax
Exit 3   → Timeout, retry or increase timeout
Exit 4   → Resource unavailable, check connectivity
Exit 5   → Auth failed, check credentials
```

Skill documents these and provides recovery steps.

### Example Error Flow

```bash
# Command fails
$ puppeteer-cli navigate --session expired123 --url "https://example.com"
# Exit code: 4

# stderr:
{
  "status": "error",
  "error": "Session not found: expired123",
  "exit_code": 4
}

# Claude skill tells Dev:
# "Session not found. Create new: puppeteer-cli session create"
```

---

## Pattern-Specific Workflows

### API Wrapper Pattern

Workflow for stateless API services:

```
1. /convert-mcp-server weather-mcp \
     --source npm:mcp-weather@latest

2. Converter detects: API wrapper pattern (stateless)

3. Generated CLI:
   weather-cli forecast --location "Seattle" --days 5

4. Dev uses immediately:
   python cli.py forecast --location "Seattle" --days 5 --format json

5. Skill documents:
   Each command is independent, no session needed
```

### State-Based Pattern

Workflow for interactive/browser automation:

```
1. /convert-mcp-server puppeteer-mcp \
     --source npm:mcp-puppeteer@latest

2. Converter detects: State-based pattern (sessions)

3. Generated CLI includes session management:
   puppeteer-cli session create/destroy/list
   puppeteer-cli <command> --session <ID>

4. Dev uses in sequence:
   SESSION=$(cli session create)
   cli navigate --session $SESSION --url "..."
   cli screenshot --session $SESSION
   cli session destroy --session $SESSION

5. Skill documents:
   Session lifecycle, concurrent sessions, cleanup
```

### Custom Pattern

Workflow for non-standard MCPs:

```
1. /convert-mcp-server my-custom-mcp \
     --pattern custom \
     --adapter-script ./my_adapter.py \
     --source ./my-mcp

2. Framework uses custom adapter code

3. Dev and QA validate that adapter works correctly

4. Skill auto-generated from adapter interface

5. Framework registers for future use
```

---

## Integration Hooks for DevForgeAI

### Hook: After Conversion

```python
# DevForgeAI can hook into post-conversion workflow

def on_mcp_converted(conversion_result):
    """Called after successful MCP conversion."""
    
    cli_path = conversion_result["cli_dir"]
    skill_path = conversion_result["skill_dir"]
    
    # Option 1: Auto-run tests
    test_cli(cli_path)
    
    # Option 2: Register skill
    register_skill(skill_path)
    
    # Option 3: Notify Dev persona
    dev_persona.register_tool(
        name=f"cli_{conversion_result['mcp_type']}",
        path=cli_path,
        skill_path=skill_path
    )
```

### Hook: In Specs

```
Spec YAML:
  requires_tools:
    - puppeteer-cli
    - postgres-cli

Framework:
  1. Checks if CLIs exist
  2. If not, auto-converts MCPs
  3. Dev persona gets access immediately
```

---

## Troubleshooting Integration

### "Skill generated but Dev can't find it"

Check:
1. Skill registered? `framework list-skills | grep puppeteer`
2. Correct path? `stat puppeteer-cli/skill/SKILL.md`
3. Valid YAML? `cat puppeteer-cli/skill/SKILL.md | head -5`

### "CLI works but Skill docs are wrong"

Regenerate:
```bash
devforgeai-mcp-cli-converter regenerate-skill ./puppeteer-cli
```

Or manually edit:
```bash
nano puppeteer-cli/skill/SKILL.md
```

### "Pattern detected wrong"

Override:
```bash
/convert-mcp-server my-mcp \
  --force-pattern custom \
  --adapter-script ./correct_adapter.py
```

---

## Next Steps

1. **Build converter skill** - Implement full MCPAnalyzer, CLIGenerator, SkillGenerator
2. **Add pattern templates** - Refine templates for api-wrapper, state-based
3. **Integration tests** - Verify CLI generation, skill registration
4. **Example conversions** - Convert Puppeteer, Filesystem, Weather MCPs as examples
5. **DevForgeAI integration** - Add `/convert-mcp-server` command to framework
