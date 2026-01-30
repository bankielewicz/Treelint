---
name: devforgeai-mcp-cli-converter
description: Convert any MCP (Model Context Protocol) server into a CLI utility and auto-generate a complementary skill. Use when you have an MCP server (Puppeteer, filesystem, weather, etc.) and need to: (1) Create a standalone CLI wrapper that Claude Code can execute, (2) Auto-generate a skill so Claude understands how to use the CLI, (3) Bridge MCP async patterns into sync CLI patterns, or (4) Rapidly prototype tool integration without MCP server overhead.
license: Complete terms in LICENSE.txt
model: claude-model: opus-4-5-20251001
---

# MCP-to-CLI Converter

Convert any MCP server into a locally-executable CLI utility with auto-generated skill documentation. This hybrid framework handles 80% of MCP patterns automatically while allowing custom extensions for outliers.

## Quick Start: Three Conversion Patterns

### Pattern 1: API Wrapper MCPs (Easiest)

**Examples**: Weather APIs, Translation services, data providers
**Characteristics**: Stateless, single tool calls, structured JSON responses
**Conversion**: Direct 1:1 mapping of tools → CLI commands

```bash
devforgeai-mcp-cli-converter convert weather-mcp \
  --pattern api-wrapper \
  --output-dir ./weather-cli
```

Result:
- `weather-cli get-forecast --location "San Francisco" --days 5`
- Skill auto-generated with clear input/output documentation

### Pattern 2: State-Based Operations (Medium)

**Examples**: Puppeteer, database, file systems
**Characteristics**: Maintain state, sequential operations, context-dependent
**Conversion**: Session management + command queueing

```bash
devforgeai-mcp-cli-converter convert puppeteer-mcp \
  --pattern state-based \
  --session-model ephemeral \
  --output-dir ./puppeteer-cli
```

Result:
- `puppeteer-cli session create` → session ID
- `puppeteer-cli navigate --session <id> --url "https://example.com"`
- `puppeteer-cli screenshot --session <id>` → base64 to stdout
- Skill documents session lifecycle

### Pattern 3: Custom Extensions (Advanced)

**When patterns don't fit**: File custom adapter code
**Conversion**: Framework + your logic

```bash
devforgeai-mcp-cli-converter convert custom-mcp \
  --pattern custom \
  --adapter-script ./my_adapter.py \
  --output-dir ./custom-cli
```

## Understanding Your MCP

Before converting, the framework analyzes your MCP server to determine the best pattern:

### Automatic Detection

Run analysis to see what the framework discovers:

```bash
devforgeai-mcp-cli-converter analyze <path-to-mcp-server>
```

This generates `mcp_analysis.json`:

```json
{
  "mcp_type": "puppeteer-mcp",
  "detected_pattern": "state-based",
  "confidence": 0.95,
  "tools": [
    {
      "name": "navigate",
      "inputs": {"url": "string"},
      "outputs": "void",
      "side_effects": ["browser_state"],
      "async": true
    }
  ],
  "state_management": {
    "stateful": true,
    "session_required": true,
    "concurrent_sessions": true
  },
  "conversion_recommendations": [
    "Use ephemeral session model",
    "Queue operations within session",
    "Stream binary outputs as base64"
  ]
}
```

### Manual Override

If detection is wrong, explicitly specify:

```bash
devforgeai-mcp-cli-converter convert my-mcp \
  --force-pattern api-wrapper \
  --config ./converter.yaml
```

## Conversion Workflow

### Step 1: Provide MCP Source

Three input methods:

**A) From published MCP package:**
```bash
devforgeai-mcp-cli-converter convert puppeteer \
  --source npm:mcp-puppeteer@latest \
  --output-dir ./puppeteer-cli
```

**B) From local source code:**
```bash
devforgeai-mcp-cli-converter convert my-service \
  --source ./path/to/mcp-server \
  --lang python \
  --output-dir ./my-service-cli
```

**C) From schema/spec file:**
```bash
devforgeai-mcp-cli-converter convert my-api \
  --source ./mcp-schema.json \
  --output-dir ./my-api-cli
```

### Step 2: Framework Generates CLI Wrapper

The converter produces:

```
output-dir/
├── cli.py (or cli.ts for Node)
├── adapters/
│   ├── base_adapter.py
│   └── <detected-pattern>_adapter.py
├── utils/
│   ├── error_handler.py
│   ├── output_formatter.py
│   └── session_manager.py
├── requirements.txt
├── README.md
└── tests/
    └── test_cli.py
```

### Step 3: Framework Generates Skill

The converter produces:

```
output-dir/
├── skill/
│   ├── SKILL.md
│   ├── references/
│   │   ├── cli_reference.md
│   │   └── usage_examples.md
│   ├── scripts/
│   │   └── setup.sh (installs CLI)
│   └── assets/
│       └── error_codes.md
```

### Step 4: Package & Distribute

Bundle both together:

```bash
devforgeai-mcp-cli-converter package \
  --cli-dir ./puppeteer-cli \
  --skill-dir ./puppeteer-cli/skill \
  --output ./puppeteer-bundle.zip
```

## Pattern Deep Dives

### Pattern 1: API Wrapper

**Use when**: Stateless service calls, external APIs, data providers

**Generated CLI structure:**
```bash
tool-cli <command> [options] → JSON/structured output
```

**Example:**
```bash
weather-cli forecast --location "Seattle" --days 5 --format json
```

**Skill section:**
```markdown
## Weather Forecast CLI

Run `weather-cli forecast --location <city> --days <n> --format json` 
to get forecast data. Returns JSON with temperature, conditions, etc.
```

**Framework handles**: HTTP error conversion, timeout management, output normalization

---

### Pattern 2: State-Based Operations

**Use when**: Browser automation, database transactions, file processing workflows

**Session lifecycle:**
```bash
# Create session
SESSION=$(puppeteer-cli session create --name "scrape-job")

# Perform operations within session
puppeteer-cli navigate --session $SESSION --url "https://example.com"
puppeteer-cli screenshot --session $SESSION --output /tmp/page.png
puppeteer-cli click --session $SESSION --selector "button.next"

# Clean up
puppeteer-cli session destroy --session $SESSION
```

**Skill section:**
```markdown
## Puppeteer Browser Automation

Create a session with `puppeteer-cli session create`, then run commands 
scoped to that session. Sessions maintain browser state between commands.
Always destroy sessions when done.

### Session Commands
- `session create [--name STRING]` → SESSION_ID
- `session destroy --session SESSION_ID` → confirmation
- `session list` → JSON array of active sessions

### Browser Commands
- `navigate --session SESSION_ID --url URL`
- `screenshot --session SESSION_ID [--output PATH]`
- `click --session SESSION_ID --selector SELECTOR`
- etc.
```

**Framework handles**: Session lifecycle management, operation queueing, state persistence, cleanup on timeout

---

### Pattern 3: Custom Extensions

**Use when**: MCP doesn't fit standard patterns, complex orchestration needed

**Adapter interface:**
```python
class CustomAdapter:
    def __init__(self, mcp_config):
        # Initialize MCP client
        pass
    
    def parse_tool_call(self, tool_name: str, params: dict) -> CLICommand:
        # Map MCP tool → CLI command structure
        pass
    
    def execute(self, command: CLICommand) -> CLIResponse:
        # Custom execution logic
        pass
    
    def format_output(self, response) -> str:
        # Format for stdout/Claude consumption
        pass
```

**Workflow:**
1. Framework generates boilerplate
2. You implement 4 methods above
3. Framework wraps your logic into CLI
4. Skill auto-generates from your adapter

**Minimal example:**
```python
class MyAdapter(CustomAdapter):
    def parse_tool_call(self, tool_name, params):
        # Your logic here
        if tool_name == "special_operation":
            return CLICommand(
                cmd="myservice",
                args=["do-thing", "--param", params["value"]]
            )
    
    def format_output(self, response):
        return json.dumps(response, indent=2)
```

## Common Patterns: When to Use Which

| MCP Type | Pattern | Why | Example |
|----------|---------|-----|---------|
| REST API wrapper | API Wrapper | Stateless, 1:1 mapping | OpenWeather, Stripe, Slack |
| Browser automation | State-Based | Session state crucial | Puppeteer, Playwright |
| Database client | State-Based | Transactions, connections | MySQL, PostgreSQL |
| File operations | State-Based or Custom | Working directory context | LocalFS, S3 |
| Custom orchestration | Custom | Complex multi-step logic | Your proprietary tool |
| Hybrid (API + state) | Custom | Mix of concerns | Advanced search engines |

## Output Format Standardization

All generated CLIs normalize output to one of:

### JSON (recommended for structured data)
```bash
cli-tool command --format json
# stdout: {"status": "success", "data": {...}}
```

### Text (recommended for human-readable, single values)
```bash
cli-tool get-status
# stdout: "active" or "error: connection failed"
```

### Base64 (recommended for binary: images, files)
```bash
cli-tool screenshot --session abc123
# stdout: iVBORw0KGgoAAAANSUhEUgAAAAEAAAA...
```

**Skill documents which format to expect for each command.**

## Error Handling Strategy

Generated CLIs use Unix exit codes:

```
0   = Success
1   = General error
2   = Invalid arguments/usage
3   = Timeout
4   = Resource unavailable
5   = Authentication failed
6   = Rate limited
```

**stderr** contains human-readable error message with recovery hints.

**Skill documents common errors** and Claude knows how to handle them.

## Integration with DevForgeAI

When used within DevForgeAI framework:

```
/convert-mcp-server puppeteer-mcp
  ↓
Dev persona gets generated CLI + skill
  ↓
Dev persona tests CLI in terminal
  ↓
QA persona uses auto-generated skill to validate
  ↓
Framework registers skill for future conversations
```

## Next Steps

- **See**: `references/pattern_detection.md` - How framework detects MCP patterns
- **See**: `references/adapter_api.md` - Full adapter interface specification
- **See**: `references/skill_generation_rules.md` - Rules for auto-generated skills
- **See**: `scripts/converter.py` - Main converter engine
- **See**: `examples/` - Complete conversion examples (weather, puppeteer, filesystem)

## Troubleshooting

### "Could not auto-detect pattern"
The framework couldn't confidently classify your MCP. Either:
1. Use `--force-pattern` to override
2. Use `--pattern custom` and provide adapter script
3. Check `mcp_analysis.json` for what it detected

### "CLI works but skill is confusing"
Skill generation got the interface wrong:
1. Edit generated `skill/SKILL.md` directly
2. Run `devforgeai-mcp-cli-converter regenerate-skill <cli-dir>` to try again
3. Or copy patterns from `examples/`

### "State-based operations losing context"
Session timeout or cleanup issue:
1. Check `--session-timeout` setting (default 1 hour)
2. Verify `session destroy` is being called
3. Run `puppeteer-cli session list` to see orphaned sessions
4. Look at adapter logs: `--debug` flag
