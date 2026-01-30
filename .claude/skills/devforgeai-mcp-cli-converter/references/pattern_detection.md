# Pattern Detection Guide

The converter automatically detects which pattern best fits your MCP server. Understanding these patterns helps you guide the conversion when auto-detection needs assistance.

## Pattern 1: API Wrapper (Stateless)

**When to use**: REST APIs, external data services, stateless tool calls

### Characteristics

- Tools don't reference session, connection, or context
- Each tool call is independent
- No side effects that persist between calls
- Results are typically JSON data
- Fewer than 10-15 tools (usually 3-8)

### Detection Signals

The converter looks for:
- Tool names without state keywords (session, page, connection, etc.)
- No "side_effects" annotations in tools
- Clear input/output contracts
- API-like naming conventions (get_*, list_*, fetch_*, etc.)

### Example MCPs

- OpenWeather API wrapper
- Stripe payment service
- Translation services
- Data lookup services
- Calculation services

### Generated CLI Pattern

```bash
weather-cli forecast --location "Seattle" --days 5 --format json
# Returns: {"status": "success", "data": {...}}

stripe-cli create-charge --amount 1000 --currency usd
# Returns: {"status": "success", "data": {"charge_id": "ch_..."}}
```

### Skill Documents

- Direct 1:1 command mapping
- Input/output specifications
- Error code translation
- Rate limiting guidance (if applicable)

---

## Pattern 2: State-Based (Stateful)

**When to use**: Browser automation, database clients, interactive tools, file processing

### Characteristics

- Tools reference session, connection, browser, page, or context
- Operations must occur in sequence within a session
- State persists between calls
- Tools build on each other's effects
- Multiple concurrent sessions may be needed

### Detection Signals

The converter looks for:
- Tool names containing: navigate, click, query, connect, open, page, session, transaction
- Explicit "session" parameters in tool definitions
- Dependencies between tools
- Side effects that modify application state
- Async operations

### Example MCPs

- Puppeteer (browser automation)
- MySQL/PostgreSQL clients
- Filesystem operations
- DynamoDB operations
- LLM context management

### Generated CLI Pattern

```bash
# Create session
SESSION=$(puppeteer-cli session create --name "scraper-job")

# Run operations in sequence
puppeteer-cli navigate --session $SESSION --url "https://example.com"
puppeteer-cli screenshot --session $SESSION --output /tmp/page.png
puppeteer-cli click --session $SESSION --selector "button.submit"
puppeteer-cli extract-text --session $SESSION --selector "div.results" --format json

# Clean up
puppeteer-cli session destroy --session $SESSION
```

### Skill Documents

- Session lifecycle (create, list, destroy)
- Session scoping for commands
- Timeout and cleanup procedures
- State management best practices
- Concurrency patterns

---

## Pattern 3: Custom (Hybrid/Complex)

**When to use**: Non-standard patterns, hybrid concerns, complex orchestration

### Characteristics

- Mix of API-wrapper and state-based patterns
- Complex orchestration logic
- Proprietary workflows
- Advanced error recovery
- Special handling needed

### When Auto-Detection Fails

You might see:
```
"detected_pattern": "unknown",
"confidence": 0.3,
"conversion_recommendations": [
  "Consider using --pattern custom",
  "Provide custom adapter script"
]
```

### Implementation Path

For custom patterns, provide your own adapter:

```bash
devforgeai-mcp-cli-converter convert my-service \
  --pattern custom \
  --adapter-script ./my_adapter.py \
  --output-dir ./my-service-cli
```

Your adapter implements:

```python
class CustomAdapter:
    def execute(self, command: str, args: dict) -> dict:
        """Your orchestration logic here"""
        pass
    
    def format_output(self, response) -> str:
        """Your output formatting"""
        pass
```

---

## Confidence Scoring

The converter reports confidence in its pattern detection:

| Confidence | Meaning | Action |
|-----------|---------|--------|
| 0.95+ | Very confident | Trust auto-detection |
| 0.85-0.94 | Confident | Review but likely correct |
| 0.70-0.84 | Moderate | Consider reviewing/overriding |
| 0.50-0.69 | Uncertain | Review carefully |
| < 0.50 | Low confidence | Use `--pattern custom` or manual override |

---

## Edge Cases

### API Wrapper with State

Some services maintain state but present as stateless (e.g., OAuth with bearer tokens).

**Indicator**: Tokens or credentials passed with each call

**Recommendation**: Use `api-wrapper` pattern; handle tokens as command parameters

### State-Based with Query Language

Some stateful tools have complex query interfaces (e.g., GraphQL clients)

**Indicator**: Complex structured query parameters

**Recommendation**: Use `state-based` if session management is primary concern; otherwise `custom`

### Streaming/Subscription Pattern

MCPs that emit events or stream data continuously

**Indicator**: Tools with output type "stream" or event keywords

**Recommendation**: Likely needs `custom` pattern for proper stream handling

---

## Override Guide

If auto-detection is wrong, override with:

```bash
# Force a specific pattern
devforgeai-mcp-cli-converter convert my-mcp \
  --force-pattern state-based \
  --source ./my-mcp

# With explicit configuration
devforgeai-mcp-cli-converter convert my-mcp \
  --force-pattern custom \
  --source ./my-mcp \
  --adapter-script ./adapter.py
```

After overriding, review generated CLI and skill to ensure patterns match your expectations.
