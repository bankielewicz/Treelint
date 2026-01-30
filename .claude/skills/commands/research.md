# Research Command

Invoke the devforgeai-research skill to capture and persist research findings.

---

## Usage

```
/research [topic]           - Start new research session
/research --resume ID       - Resume existing research
/research --search query    - Search research documents
/research --list            - List all research documents
/research --category type   - Filter by category
```

---

## Examples

```bash
# Start new research
/research "AWS Kiro Competitive Analysis"

# Resume existing
/research --resume RESEARCH-001

# Search
/research --search "token efficiency"

# List all
/research --list

# Filter by category
/research --list --category competitive
```

---

## Categories

| Category | Description |
|----------|-------------|
| `competitive` | Competitor analysis, features, pricing |
| `technology` | Library/framework evaluation |
| `market` | Trends, statistics, developer needs |
| `integration` | External service/API integration |
| `architecture` | Design patterns, best practices |

---

## Research Storage

All research is persisted in: `devforgeai/specs/research/`

Research index: `devforgeai/specs/research/research-index.md`

---

## Workflow

When you run `/research "Topic"`, the skill will:

1. Generate a unique RESEARCH-ID
2. Ask interactive questions about the topic
3. Execute research (web searches, codebase analysis)
4. Synthesize findings into themes
5. Generate recommendations
6. Write structured research document
7. Update research index
8. Optionally link to epics/stories/ADRs

---

## Research Execution

The `/research` command delegates research execution to the `internet-sleuth` subagent, which provides:

- **Repository Archaeology** - Clone and analyze GitHub repos for code patterns
- **Context Validation** - Validates recommendations against all 6 context files
- **Progressive Methodology** - Loads mode-specific research guides (65% token savings)
- **Workflow Awareness** - Adapts research focus to current development phase

### Category → Mode Mapping

| Category | internet-sleuth Mode | Research Focus |
|----------|---------------------|----------------|
| competitive | competitive-analysis | Features, pricing, positioning |
| technology | repository-archaeology | Code patterns, GitHub analysis |
| market | market-intelligence | Statistics, trends, pain points |
| integration | investigation | APIs, SDKs, integration patterns |
| architecture | discovery | Design patterns, best practices |

### Fallback Behavior

If internet-sleuth is unavailable, the skill falls back to direct web search using
`references/search-strategies.md` methodology.

---

## Invocation

This command invokes:
```
Skill(command="devforgeai-research", args="{user_args}")
```
