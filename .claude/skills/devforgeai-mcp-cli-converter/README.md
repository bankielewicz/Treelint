# MCP-CLI Converter Skill

**Convert any MCP server into a standalone CLI utility + auto-generated skill.**

A hybrid framework that handles 80% of MCP patterns automatically while allowing custom extensions for advanced cases.

## What This Does

Takes an MCP server (Puppeteer, Filesystem, Weather API, etc.) and converts it to:

1. **Standalone CLI** - Executable without MCP server infrastructure
2. **Auto-generated Skill** - Documentation Claude understands
3. **Ready for DevForgeAI** - Integrates immediately into your framework

## Quick Start

### Example: Convert Puppeteer MCP

```bash
# Analyze the MCP
python scripts/converter.py analyze npm:mcp-puppeteer@latest

# Convert to CLI + skill
python scripts/converter.py convert puppeteer \
  --source npm:mcp-puppeteer@latest \
  --output-dir ./puppeteer-cli

# Now you have:
# - ./puppeteer-cli/cli.py         (the CLI)
# - ./puppeteer-cli/skill/SKILL.md (the skill)
```

### Use the Generated CLI

```bash
# Install dependencies
pip install -r ./puppeteer-cli/requirements.txt

# Create browser session
SESSION=$(python ./puppeteer-cli/cli.py session create --name "scraper")

# Use it
python ./puppeteer-cli/cli.py navigate --session $SESSION --url "https://example.com"
python ./puppeteer-cli/cli.py screenshot --session $SESSION --format base64

# Clean up
python ./puppeteer-cli/cli.py session destroy --session $SESSION
```

## The Three Patterns

### Pattern 1: API Wrapper (Stateless)

**Use for**: Weather APIs, Translation services, data providers

```
Input: MCP with stateless tools
Output: CLI with 1:1 command mapping
Example: weather-cli forecast --location Seattle --days 5
```

### Pattern 2: State-Based (Stateful)

**Use for**: Browser automation, database clients, interactive tools

```
Input: MCP with session/connection concepts
Output: CLI with session lifecycle (create, use, destroy)
Example: puppeteer-cli session create → puppeteer-cli navigate --session abc123
```

### Pattern 3: Custom (Hybrid/Advanced)

**Use for**: Non-standard patterns, complex orchestration

```
Input: MCP + your custom adapter script
Output: CLI with your orchestration logic
Example: Your domain-specific workflow
```

## File Structure

```
devforgeai-mcp-cli-converter-skill/
├── SKILL.md                         # Main skill documentation
├── scripts/
│   └── converter.py                 # Main converter engine
│       └── Classes:
│           ├── MCPAnalyzer          # Analyzes MCP structure
│           ├── CLIGenerator         # Generates CLI code
│           ├── SkillGenerator       # Generates skill docs
│
├── references/
│   ├── pattern_detection.md         # How patterns are detected
│   ├── adapter_api.md               # How to write adapters
│   └── devforge_integration.md      # Integration with DevForgeAI
```

## How It Works

### Step 1: Analysis

`MCPAnalyzer` inspects your MCP:
- Extracts tool definitions
- Detects patterns (API wrapper, state-based, custom)
- Returns structured analysis JSON
- Calculates confidence score

### Step 2: CLI Generation

`CLIGenerator` creates:
- Main CLI entry point (`cli.py`)
- Pattern-specific adapter (API wrapper, state-based, or custom)
- Utility modules (error handling, output formatting)
- Requirements and tests

### Step 3: Skill Generation

`SkillGenerator` creates:
- SKILL.md with Claude-friendly documentation
- Reference guides (CLI commands, examples)
- Setup script for installation
- Error code documentation

### Step 4: Integration

Both outputs are ready to:
- Run immediately: `python cli.py <command>`
- Register as skill: Copy `skill/` to DevForgeAI skills directory
- Integrate with framework: `/convert-mcp-server` slash command

## Core Components

### MCPAnalyzer

```python
analyzer = MCPAnalyzer("npm:mcp-puppeteer@latest", "python")
analysis = analyzer.analyze()
# Returns:
# {
#   "mcp_type": "puppeteer-mcp",
#   "detected_pattern": "state-based",
#   "confidence": 0.95,
#   "tools": [...],
#   "state_management": {...},
#   "conversion_recommendations": [...]
# }
```

**Detects**:
- MCP type and language
- Pattern (API wrapper, state-based, custom)
- State requirements
- Tool structure
- Conversion recommendations

### CLIGenerator

```python
cli_gen = CLIGenerator(analysis, "./output", "state-based")
cli_gen.generate()
# Generates:
# - cli.py (main entry point)
# - adapters/ (pattern-specific code)
# - utils/ (utilities)
# - requirements.txt
# - tests/
```

**Generates**:
- Argparse-based CLI with subcommands
- Pattern-specific adapter
- Error handling
- Output formatting
- Test templates

### SkillGenerator

```python
skill_gen = SkillGenerator(analysis, "./output")
skill_gen.generate()
# Generates:
# - SKILL.md (Claude skill documentation)
# - references/ (CLI documentation)
# - scripts/ (setup)
# - assets/ (error codes)
```

**Generates**:
- SKILL.md with triggers and usage
- Command reference documentation
- Usage examples
- Error code reference
- Setup instructions

## Usage Examples

### CLI: Analyze MCP

```bash
python scripts/converter.py analyze ./path/to/mcp-server \
  --lang python \
  --output analysis.json

cat analysis.json
```

### CLI: Convert with Auto-Detection

```bash
python scripts/converter.py convert weather \
  --source npm:mcp-openweather@latest \
  --output-dir ./weather-cli
```

### CLI: Force Pattern

```bash
python scripts/converter.py convert custom-tool \
  --source ./my-mcp \
  --pattern state-based \
  --output-dir ./custom-cli
```

### CLI: Custom Adapter

```bash
python scripts/converter.py convert special-mcp \
  --pattern custom \
  --adapter-script ./my_adapter.py \
  --source ./special-mcp \
  --output-dir ./special-cli
```

## Integration with DevForgeAI

### As a DevForgeAI Command

```
User: "I want to use the puppeteer MCP"

/convert-mcp-server puppeteer-mcp \
  --source npm:mcp-puppeteer@latest

Framework:
1. Runs converter.py convert
2. Generates CLI and skill
3. Registers skill
4. Dev persona now has access to puppeteer-cli
5. Dev can use in next /build or /test command
```

### Skill Registration

After conversion, skill is available:

```
/list-skills → includes "puppeteer-cli"

/build "Scrape website"
→ Dev persona sees puppeteer-cli in context
→ Can use: puppeteer-cli navigate, screenshot, extract-text
```

## Pattern Selection Guide

| MCP Type | Pattern | Why |
|----------|---------|-----|
| REST API | API Wrapper | Stateless, 1:1 mapping |
| Puppeteer | State-Based | Needs session management |
| Weather API | API Wrapper | Simple data lookup |
| Database | State-Based | Connection state |
| File System | State-Based | Working directory |
| Custom | Custom | Your logic |

## Common Use Cases

### 1. Puppeteer Browser Automation

```bash
python scripts/converter.py convert puppeteer \
  --source npm:mcp-puppeteer@latest
# Generates: puppeteer-cli with session management
```

### 2. External API Wrapper

```bash
python scripts/converter.py convert openweather \
  --source npm:mcp-openweather@latest
# Generates: openweather-cli with direct command mapping
```

### 3. Database Client

```bash
python scripts/converter.py convert postgres \
  --source ./postgres-mcp \
  --pattern state-based
# Generates: postgres-cli with connection pooling
```

### 4. Complex Orchestration

```bash
python scripts/converter.py convert myservice \
  --pattern custom \
  --adapter-script ./complex_adapter.py
# Generates: myservice-cli with your orchestration logic
```

## Error Handling

Generated CLIs use standard Unix exit codes:

```
0 → Success
1 → General error
2 → Invalid arguments
3 → Timeout
4 → Resource unavailable
5 → Authentication failed
6 → Rate limited
```

Skill documents these and provides recovery guidance.

## Testing Generated CLI

```bash
# Install
pip install -r puppeteer-cli/requirements.txt

# Test help
python puppeteer-cli/cli.py --help

# Run generated tests
python -m pytest puppeteer-cli/tests/

# Manual test (for state-based)
SESSION=$(python puppeteer-cli/cli.py session create)
python puppeteer-cli/cli.py <command> --session $SESSION --format json
python puppeteer-cli/cli.py session destroy --session $SESSION
```

## For DevForgeAI Integration

### Register the Skill

```bash
# After conversion
cp -r puppeteer-cli/skill ~/.devforge/skills/puppeteer-cli
```

### Use in Specs

```yaml
spec:
  name: "Web Scraper"
  requires_tools:
    - puppeteer-cli
  dev_workflow: |
    Use puppeteer-cli to navigate, screenshot, and extract text
```

### Dev Persona Access

```
Dev persona context includes:
- puppeteer-cli skill
- Known commands: session create, navigate, screenshot, click, extract-text
- Session lifecycle: create → use → destroy
- Error codes: 0-5 mapped to recovery steps
```

## Troubleshooting

### "Could not auto-detect pattern"

```bash
# Override pattern explicitly
python scripts/converter.py convert myapp \
  --source ./myapp \
  --pattern custom \
  --adapter-script ./adapter.py
```

### "Generated CLI errors"

1. Check adapter implementation
2. Ensure MCP dependencies are installed
3. Run with `--debug` flag
4. Review adapter code

### "Skill doesn't match CLI"

1. Regenerate skill: `python scripts/converter.py regenerate-skill <cli-dir>`
2. Or manually edit: `nano <cli-dir>/skill/SKILL.md`
3. Verify adapter signature matches skill documentation

## References

- **SKILL.md** - Main skill documentation (triggers, patterns, workflows)
- **references/pattern_detection.md** - How patterns are detected and classified
- **references/adapter_api.md** - Complete adapter interface for custom implementations
- **references/devforge_integration.md** - How this integrates with DevForgeAI framework

## Architecture

The converter is built around three core classes:

```
MCPAnalyzer (→ analysis.json)
    ↓
CLIGenerator (→ cli/)
    ↓
SkillGenerator (→ skill/)
    ↓
[DevForgeAI Framework]
```

Each stage is independent and can be run separately:

```bash
# Just analyze
python converter.py analyze <mcp>

# Analyze + generate CLI only
python converter.py convert <name> --source <mcp> --output-dir <dir>
```

## Next Steps

1. Try converting an existing MCP (Puppeteer, OpenWeather, Filesystem)
2. Review generated CLI code
3. Test generated skill documentation
4. Customize adapters for your specific needs
5. Integrate with DevForgeAI `/convert-mcp-server` command
6. Build skill registry for commonly-converted MCPs

## Design Principles

1. **80% automatic** - Standard patterns handled by framework
2. **20% customizable** - Custom adapters for edge cases
3. **Zero dependencies** - Generated CLIs are standalone
4. **Claude-aware** - Generated skills Claude understands
5. **DevForgeAI-native** - Integrates seamlessly with framework

---

**Built for DevForgeAI** - A spec-driven development framework optimized for Claude Code terminal.
