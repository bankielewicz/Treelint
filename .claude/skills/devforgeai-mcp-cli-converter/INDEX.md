# MCP-CLI Converter Skill - Complete Package

**A hybrid framework that converts any MCP server into a standalone CLI utility + auto-generated skill for DevForgeAI.**

## Package Contents

```
devforgeai-mcp-cli-converter-skill/
├── README.md                          # Quick start & overview (START HERE)
├── SKILL.md                           # Main skill documentation for Claude
├── examples.md                        # 5 concrete examples (Puppeteer, Weather, PostgreSQL, etc.)
│
├── scripts/
│   └── converter.py                   # Main converter engine (~750 lines)
│       ├── MCPAnalyzer               # Analyzes MCP structure & detects patterns
│       ├── CLIGenerator              # Generates standalone CLI code
│       └── SkillGenerator            # Generates skill documentation
│
└── references/
    ├── pattern_detection.md          # How pattern detection works, when to override
    ├── adapter_api.md                # Complete API for custom adapters
    └── devforge_integration.md       # How this integrates with DevForgeAI
```

**Total: ~2,500 lines of code, documentation, and examples**

## Key Features

### 1. Three Conversion Patterns

| Pattern | Use Case | Example |
|---------|----------|---------|
| **API Wrapper** | Stateless APIs | Weather, Translation, Payment |
| **State-Based** | Stateful operations | Browser, Database, Files |
| **Custom** | Advanced/Hybrid | Your domain-specific logic |

### 2. Automatic Detection

```bash
python scripts/converter.py analyze <mcp>
# ↓
# Detects pattern + confidence score
# Provides recommendations
```

### 3. One Command Conversion

```bash
python scripts/converter.py convert puppeteer \
  --source npm:mcp-puppeteer@latest
# ↓
# Generates:
# - CLI wrapper (cli.py)
# - Skill documentation (SKILL.md)
# - Tests & utilities
```

### 4. DevForgeAI Integration

Generated CLIs immediately work in DevForgeAI:
- Accessible via `/convert-mcp-server` command
- Skill auto-registered
- Dev persona has instant access
- QA persona can validate

## Quick Start

### 1. Analyze an MCP

```bash
python scripts/converter.py analyze npm:mcp-puppeteer@latest
# Output: mcp_analysis.json with detected pattern, tools, recommendations
```

### 2. Convert to CLI + Skill

```bash
python scripts/converter.py convert puppeteer \
  --source npm:mcp-puppeteer@latest \
  --output-dir ./puppeteer-cli
```

### 3. Use Generated CLI

```bash
cd puppeteer-cli
pip install -r requirements.txt
python cli.py session create
python cli.py navigate --session abc123 --url "https://example.com"
python cli.py screenshot --session abc123
```

### 4. Use in DevForgeAI

```
/convert-mcp-server puppeteer-mcp
# ↓
# Dev persona now has puppeteer-cli available
# /build "Scrape website" → Can use puppeteer-cli commands
```

## Understanding the Architecture

```
INPUT: MCP Server (Puppeteer, Weather API, etc.)
  ↓
MCPAnalyzer
  ├─ Extracts tool definitions
  ├─ Detects pattern (API wrapper, state-based, custom)
  ├─ Analyzes state requirements
  └─ Generates analysis.json
  ↓
CLIGenerator (pattern-specific)
  ├─ Creates main CLI (cli.py)
  ├─ Generates adapter (api_wrapper, state_based, or custom)
  ├─ Creates utilities (error handling, output formatting)
  └─ Generates tests
  ↓
SkillGenerator
  ├─ Creates SKILL.md (Claude documentation)
  ├─ Generates references (command docs, examples)
  ├─ Creates setup scripts
  └─ Generates error code reference
  ↓
OUTPUT:
  ├─ ./puppeteer-cli/cli.py (standalone executable)
  ├─ ./puppeteer-cli/skill/SKILL.md (Claude-friendly)
  └─ Both ready to use immediately
```

## The Three Patterns Explained

### Pattern 1: API Wrapper (80% stateless MCPs)

**Detection**: No session/connection keywords, stateless tools
**Generated CLI**: Direct 1:1 command mapping
**Example**: `weather-cli forecast --location Seattle --days 5`

```bash
# Simple, no state management needed
python cli.py command --param value --format json
```

### Pattern 2: State-Based (15% stateful MCPs)

**Detection**: Session/connection/state keywords present
**Generated CLI**: Session lifecycle (create → use → destroy)
**Example**: `puppeteer-cli session create → navigate → screenshot → destroy`

```bash
# State-aware execution
SESSION=$(python cli.py session create)
python cli.py command --session $SESSION --param value
python cli.py session destroy --session $SESSION
```

### Pattern 3: Custom (5% advanced/hybrid)

**Detection**: Doesn't fit patterns 1 or 2
**Implementation**: You provide adapter code
**Example**: Complex orchestration, caching, streaming

```python
class CustomAdapter:
    def execute(self, command, args):
        # Your orchestration logic here
        pass
```

## Files & Their Purpose

### Main Files

| File | Purpose | Size |
|------|---------|------|
| **README.md** | Quick start & overview | 550 lines |
| **SKILL.md** | Main skill for Claude (triggers, patterns, workflows) | 350 lines |
| **scripts/converter.py** | Main converter engine (3 classes, ~750 lines) | 750 lines |

### Reference Documentation

| File | Purpose | Read When |
|------|---------|-----------|
| **references/pattern_detection.md** | How patterns detected, confidence scoring | Auto-detect wrong? |
| **references/adapter_api.md** | Complete adapter interface for custom patterns | Building custom adapter |
| **references/devforge_integration.md** | DevForgeAI integration details | Integrating with framework |
| **examples.md** | 5 concrete conversion examples | Want to see it work |

## Core Components

### MCPAnalyzer

**What it does**: Inspects MCP to understand its structure

```python
analyzer = MCPAnalyzer("npm:mcp-puppeteer@latest", "python")
analysis = analyzer.analyze()
# Returns: {
#   "mcp_type": "puppeteer-mcp",
#   "detected_pattern": "state-based",
#   "confidence": 0.98,
#   "tools": [...],
#   "state_management": {...},
#   "recommendations": [...]
# }
```

**Key Methods**:
- `analyze()` - Full analysis
- `_detect_pattern()` - Determines which pattern to use
- `_analyze_state_management()` - Checks for state requirements
- `_confidence_score()` - Confidence in pattern (0-1)

### CLIGenerator

**What it does**: Creates standalone CLI code from analysis

```python
gen = CLIGenerator(analysis, "./output", "state-based")
gen.generate()
# Creates: cli.py, adapters/, utils/, requirements.txt, tests/
```

**Generated Outputs**:
- `cli.py` - Main CLI with argparse
- `adapters/api_wrapper_adapter.py` - API wrapper pattern
- `adapters/state_based_adapter.py` - State-based pattern
- `utils/error_handler.py` - Error handling
- `utils/output_formatter.py` - JSON/text/base64 formatting

### SkillGenerator

**What it does**: Creates Claude-friendly skill documentation

```python
gen = SkillGenerator(analysis, "./output")
gen.generate()
# Creates: SKILL.md, references/, scripts/, assets/
```

**Generated Outputs**:
- `SKILL.md` - Main skill documentation (triggers, patterns)
- `references/cli_reference.md` - Command reference
- `references/usage_examples.md` - Usage examples
- `scripts/setup.sh` - Installation script
- `assets/error_codes.md` - Error code reference

## Usage Patterns

### Pattern A: Simple Conversion

```bash
python converter.py convert weather \
  --source npm:mcp-openweather@latest \
  --output-dir ./weather-cli
```

Auto-detects pattern, generates both CLI and skill.

### Pattern B: Force Pattern

```bash
python converter.py convert myservice \
  --source ./my-mcp \
  --pattern state-based \
  --output-dir ./myservice-cli
```

Override auto-detection when you know what pattern to use.

### Pattern C: Custom Adapter

```bash
python converter.py convert special \
  --source ./special-mcp \
  --pattern custom \
  --adapter-script ./my_adapter.py \
  --output-dir ./special-cli
```

Provide your own orchestration logic for advanced cases.

### Pattern D: Analyze Only

```bash
python converter.py analyze ./my-mcp \
  --lang python \
  --output analysis.json
```

Just analyze without generating (useful for planning).

## Integration with DevForgeAI

### How It Works

```
User: "I need browser automation"
  ↓
/convert-mcp-server puppeteer-mcp
  ↓
Framework:
  1. Runs converter.py analyze
  2. Runs converter.py generate
  3. Creates CLI + skill
  4. Registers skill
  ↓
Dev Persona:
  - Has puppeteer-cli available
  - Skill in context
  - Can use immediately
  ↓
/build "Scrape website"
  ↓
Dev uses:
  - puppeteer-cli session create
  - puppeteer-cli navigate
  - puppeteer-cli screenshot
```

### Skill Registration

After conversion:

```bash
# Skill auto-available in DevForgeAI
/list-skills | grep puppeteer
# → puppeteer-cli (available)

# Dev persona sees it in next conversation
/build "Something with browser automation"
# → Dev can use puppeteer-cli commands
```

## Common Conversions

### Convert Puppeteer (State-Based)

```bash
python converter.py convert puppeteer \
  --source npm:mcp-puppeteer@latest
```

Result: `puppeteer-cli` with session management

### Convert OpenWeather (API Wrapper)

```bash
python converter.py convert openweather \
  --source npm:mcp-openweather@latest
```

Result: `openweather-cli` with direct commands

### Convert PostgreSQL (State-Based)

```bash
python converter.py convert postgres \
  --source ./postgres-mcp \
  --pattern state-based
```

Result: `postgres-cli` with connection management

### Convert Custom Service (Custom)

```bash
python converter.py convert myservice \
  --source ./myservice-mcp \
  --pattern custom \
  --adapter-script ./adapter.py
```

Result: `myservice-cli` with custom logic

## Error Handling

Generated CLIs use standard Unix exit codes:

```
0 → Success
1 → General error
2 → Invalid arguments
3 → Timeout
4 → Resource unavailable
5 → Authentication failed
```

Skill documents these and provides recovery steps.

## Troubleshooting

### "Auto-detection failed"

**Problem**: `"detected_pattern": "unknown"`

**Solution**: 
```bash
# Override pattern
python converter.py convert myapp \
  --source ./myapp \
  --force-pattern state-based
```

### "Generated CLI doesn't work"

**Problem**: Adapter not executing MCP correctly

**Solution**:
1. Check adapter code
2. Verify MCP dependencies installed
3. Review error output
4. Look at adapter_api.md for examples

### "Skill doesn't match CLI"

**Problem**: Generated skill docs are inaccurate

**Solution**:
1. Regenerate: `python converter.py regenerate-skill <cli-dir>`
2. Or edit: `nano <cli-dir>/skill/SKILL.md`
3. Verify against actual CLI output

## Next Steps

1. **Read README.md** - 10 minute overview
2. **Review examples.md** - See how real MCPs are converted
3. **Look at scripts/converter.py** - Understand architecture
4. **Try a conversion** - `python converter.py convert weather --source npm:...`
5. **Integrate with DevForgeAI** - Add `/convert-mcp-server` command
6. **Create custom adapter** - When patterns don't fit

## Key Design Decisions

1. **80% automatic** - Standard patterns handled by framework
2. **20% customizable** - Custom adapters for edge cases
3. **Zero dependencies** - Generated CLIs are standalone (no MCP server needed)
4. **Claude-aware** - Generated skills Claude understands natively
5. **DevForgeAI-native** - Direct integration with framework
6. **Incremental** - Each stage can run independently

## Architecture Philosophy

The converter follows **progressive revelation**:

```
Step 1: Analysis (what is this MCP?)
  → Output: Structured analysis JSON
  → Can stop here if just evaluating

Step 2: CLI Generation (how to call it?)
  → Output: Standalone CLI ready to use
  → Can test/use here independently

Step 3: Skill Generation (how does Claude use it?)
  → Output: SKILL.md that Claude understands
  → Skill + CLI form complete package
```

Each step is independent. You can:
- Analyze without generating
- Generate CLI without skill
- Generate skill separately
- Re-run any step

## Files at a Glance

| File | Lines | Purpose |
|------|-------|---------|
| README.md | 550 | Quick start |
| SKILL.md | 350 | Claude skill documentation |
| scripts/converter.py | 750 | Main converter engine |
| examples.md | 400 | 5 concrete examples |
| references/pattern_detection.md | 300 | Pattern guide |
| references/adapter_api.md | 550 | Custom adapter API |
| references/devforge_integration.md | 350 | DevForgeAI integration |
| **TOTAL** | **3,250** | **Complete framework** |

## Getting Started

### 1. Understand the Problem

MCPs require running an MCP server. This converts them to:
- **Standalone CLIs** - Can be called directly
- **Skill documentation** - Claude knows how to use them

### 2. Review the Examples

Read `examples.md` to see:
- Puppeteer → state-based CLI
- OpenWeather → API wrapper CLI
- PostgreSQL → state-based CLI
- Custom → custom adapter CLI

### 3. Try a Conversion

```bash
python scripts/converter.py convert weather \
  --source npm:mcp-openweather@latest \
  --output-dir ./weather-cli
```

### 4. Review Generated Files

```bash
cat weather-cli/SKILL.md
python weather-cli/cli.py --help
cat weather-cli/skill/SKILL.md
```

### 5. Integrate with DevForgeAI

Add `/convert-mcp-server` command to your framework:

```yaml
commands:
  /convert-mcp-server:
    handler: converter.py convert
    description: Convert MCP to CLI + skill
```

## Support & Documentation

- **Quick questions**: See README.md
- **How patterns work**: See references/pattern_detection.md
- **Building custom adapters**: See references/adapter_api.md
- **DevForgeAI integration**: See references/devforge_integration.md
- **Concrete examples**: See examples.md
- **Deep dive**: Review scripts/converter.py

## License

Auto-generated framework for DevForgeAI.

---

**Built for DevForgeAI** - A spec-driven development framework optimized for Claude Code terminal.

Ready to convert your first MCP? Start with `README.md`!
