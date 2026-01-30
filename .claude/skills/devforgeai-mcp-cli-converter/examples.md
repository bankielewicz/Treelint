# MCP-CLI Converter: Concrete Examples

Real-world examples of MCPs being converted to CLIs with auto-generated skills.

## Example 1: Puppeteer MCP → State-Based CLI

### The MCP (Input)

```python
# mcp-puppeteer has tools like:
@mcp.tool()
async def navigate(url: str) -> dict:
    """Navigate browser to URL."""
    return {"url": url, "loaded": True}

@mcp.tool()
async def screenshot(selector: str = None) -> bytes:
    """Take screenshot of page or element."""
    return await browser.screenshot()

@mcp.tool()
async def click(selector: str) -> dict:
    """Click element."""
    return {"clicked": selector}

@mcp.tool()
async def extract_text(selector: str) -> str:
    """Extract text content."""
    return await page.text_content(selector)
```

### Analysis Output

```json
{
  "mcp_type": "puppeteer-mcp",
  "detected_pattern": "state-based",
  "confidence": 0.98,
  "tools": [
    {
      "name": "navigate",
      "inputs": {"url": "string"},
      "outputs": "dict",
      "async": true
    },
    {
      "name": "screenshot",
      "inputs": {"selector": "string|null"},
      "outputs": "bytes",
      "async": true,
      "side_effects": ["browser_state"]
    }
  ],
  "state_management": {
    "stateful": true,
    "session_required": true,
    "state_keywords": ["browser", "page", "navigate", "click"]
  },
  "conversion_recommendations": [
    "Use ephemeral session model",
    "Queue operations within session",
    "Stream binary outputs as base64"
  ]
}
```

### Generated CLI

```bash
# Create session with browser instance
SESSION=$(python cli.py session create --name "scrape-job")

# Navigate
python cli.py navigate --session $SESSION --url "https://example.com"

# Take screenshot (returns base64)
python cli.py screenshot --session $SESSION --selector "body" --format base64 > page.png.b64

# Extract text
python cli.py extract-text --session $SESSION --selector "div.content" --format json
# Output: {"status": "success", "data": "extracted content"}

# Click button and wait for navigation
python cli.py click --session $SESSION --selector "button.submit"

# Clean up
python cli.py session destroy --session $SESSION
```

### Generated Skill (SKILL.md excerpt)

```markdown
---
name: puppeteer-cli
description: Browser automation CLI for web scraping and testing. Use for: (1) Navigating to websites, (2) Taking screenshots of pages, (3) Extracting text content, (4) Clicking elements and interacting with pages. Run commands with: puppeteer-cli <command> --session <SESSION_ID> [options]
---

# Puppeteer Browser Automation CLI

Pattern: STATE-BASED

## Session Management

Create a browser session before running commands:

```bash
SESSION=$(puppeteer-cli session create --name "my-job")
puppeteer-cli navigate --session $SESSION --url "https://example.com"
puppeteer-cli session destroy --session $SESSION
```

## Available Commands

- `navigate --session ID --url URL` - Navigate to webpage
- `screenshot --session ID [--selector SELECTOR] --format json|base64` - Capture page/element
- `click --session ID --selector SELECTOR` - Click element
- `extract-text --session ID --selector SELECTOR --format json` - Get text content

## Session Lifecycle

Sessions maintain browser state and context:
- Create: `session create [--name STRING]` → SESSION_ID
- Use: Pass `--session SESSION_ID` to any command
- Destroy: `session destroy --session SESSION_ID` (cleanup)
- List: `session list` → JSON of active sessions

Sessions timeout after 1 hour of inactivity.
```

---

## Example 2: OpenWeather API → API Wrapper CLI

### The MCP (Input)

```typescript
// mcp-openweather has stateless tools:
@tool()
async function getForecast(
  location: string,
  days: number = 5,
  units: string = "metric"
): Promise<WeatherForecast> {
  // Calls OpenWeather API
  return weatherData;
}

@tool()
async function getCurrentWeather(
  location: string,
  units: string = "metric"
): Promise<CurrentWeather> {
  return currentWeatherData;
}

@tool()
async function searchCities(query: string): Promise<City[]> {
  return citiesMatchingQuery;
}
```

### Analysis Output

```json
{
  "mcp_type": "openweather-mcp",
  "detected_pattern": "api-wrapper",
  "confidence": 0.97,
  "tools": [
    {
      "name": "getForecast",
      "inputs": {
        "location": "string",
        "days": "integer",
        "units": "string"
      },
      "outputs": "WeatherForecast",
      "async": true
    }
  ],
  "state_management": {
    "stateful": false,
    "session_required": false,
    "concurrent_sessions": false
  },
  "conversion_recommendations": [
    "Direct 1:1 tool → CLI command mapping",
    "Handle HTTP errors → exit codes",
    "Normalize JSON responses"
  ]
}
```

### Generated CLI

```bash
# Get forecast (no session needed, stateless)
python cli.py get-forecast \
  --location "Seattle" \
  --days 5 \
  --units metric \
  --format json

# Returns:
{
  "status": "success",
  "command": "get-forecast",
  "data": {
    "location": "Seattle, WA",
    "forecast": [
      {"date": "2025-11-09", "temp": 48, "condition": "cloudy"},
      ...
    ]
  }
}

# Get current weather
python cli.py get-current-weather \
  --location "Seattle" \
  --format json

# Search cities
python cli.py search-cities \
  --query "San" \
  --format json
# Returns: {"status": "success", "data": [{"name": "San Francisco"}, ...]}
```

### Generated Skill (SKILL.md excerpt)

```markdown
---
name: openweather-cli
description: Weather data CLI using OpenWeather API. Use for: (1) Getting weather forecasts, (2) Checking current conditions, (3) Searching locations. Each command is independent - no session needed.
---

# OpenWeather CLI

Pattern: API-WRAPPER (Stateless)

## Available Commands

- `get-forecast --location CITY [--days N] [--units metric|imperial]`
- `get-current-weather --location CITY [--units metric|imperial]`
- `search-cities --query SEARCH_STRING`

## Examples

```bash
# 5-day forecast for Seattle
openweather-cli get-forecast --location "Seattle" --days 5

# Current weather
openweather-cli get-current-weather --location "Seattle"

# Find cities named "San"
openweather-cli search-cities --query "San"
```

## Output Format

All commands return JSON (default) or text:

```json
{
  "status": "success",
  "data": {
    // Command-specific data
  }
}
```

Exit code 0 = success, non-zero = error.
```

---

## Example 3: PostgreSQL MCP → State-Based CLI

### The MCP (Input)

```python
# mcp-postgres has connection state:
@mcp.tool()
async def connect(host: str, port: int, username: str, password: str, database: str) -> dict:
    """Connect to PostgreSQL."""
    return {"connected": True, "host": host}

@mcp.tool()
async def execute(sql: str) -> list:
    """Execute SQL query (requires active connection)."""
    return query_results

@mcp.tool()
async def begin_transaction() -> dict:
    """Start transaction (requires connection)."""
    return {"transaction_started": True}

@mcp.tool()
async def commit() -> dict:
    """Commit transaction."""
    return {"committed": True}

@mcp.tool()
async def rollback() -> dict:
    """Rollback transaction."""
    return {"rolled_back": True}
```

### Analysis Output

```json
{
  "mcp_type": "postgres-mcp",
  "detected_pattern": "state-based",
  "confidence": 0.99,
  "tools": [
    {"name": "connect", "inputs": {...}, "outputs": "dict"},
    {"name": "execute", "inputs": {"sql": "string"}, "outputs": "list"},
    {"name": "begin_transaction", "inputs": {}, "outputs": "dict"},
    {"name": "commit", "inputs": {}, "outputs": "dict"},
    {"name": "rollback", "inputs": {}, "outputs": "dict"}
  ],
  "state_management": {
    "stateful": true,
    "session_required": true,
    "state_keywords": ["connect", "transaction", "commit", "rollback"]
  },
  "conversion_recommendations": [
    "Use session-based connection pooling",
    "Maintain transaction state across operations",
    "Handle connection cleanup",
    "Support multiple concurrent connections"
  ]
}
```

### Generated CLI

```bash
# Create session/connection
CONN=$(python cli.py session create --name "data-load")

# Connect to database
python cli.py connect \
  --session $CONN \
  --host localhost \
  --port 5432 \
  --username dbuser \
  --password secret \
  --database mydb

# Start transaction
python cli.py begin-transaction --session $CONN

# Execute query
python cli.py execute \
  --session $CONN \
  --sql "INSERT INTO users (name, email) VALUES ('John', 'john@example.com')" \
  --format json

# Execute another query
python cli.py execute \
  --session $CONN \
  --sql "SELECT * FROM users" \
  --format json

# Commit
python cli.py commit --session $CONN

# Or rollback
python cli.py rollback --session $CONN

# Clean up connection
python cli.py session destroy --session $CONN
```

### Generated Skill (SKILL.md excerpt)

```markdown
---
name: postgres-cli
description: PostgreSQL database client CLI. Use for: (1) Connecting to databases, (2) Executing SQL queries, (3) Running transactions. Maintain session for database connection.
---

# PostgreSQL CLI

Pattern: STATE-BASED

## Connection Management

Sessions represent database connections:

```bash
CONN=$(postgres-cli session create --name "load-data")
postgres-cli connect --session $CONN --host localhost --port 5432 --username user --password pass --database db
```

## Transactions

Execute queries within transactions:

```bash
postgres-cli begin-transaction --session $CONN
postgres-cli execute --session $CONN --sql "INSERT INTO ..."
postgres-cli execute --session $CONN --sql "UPDATE ..."
postgres-cli commit --session $CONN
```

## Available Commands

- `session create` - Create connection session
- `session destroy` - Close connection
- `connect --session ID --host HOST --port PORT --username USER --password PASS --database DB`
- `execute --session ID --sql SQL_STRING`
- `begin-transaction --session ID`
- `commit --session ID`
- `rollback --session ID`
```

---

## Example 4: Custom MCP → Custom Adapter

### The Problem

Some MCPs don't fit standard patterns. Example: A tool that needs:
- State management like state-based
- But also has complex caching logic
- Plus event streaming

### The Solution

Create custom adapter:

```python
# my_adapter.py
from typing import Dict, Any

class MyServiceAdapter:
    """Custom adapter with caching and streaming."""
    
    def __init__(self):
        self.connections = {}
        self.cache = {}
    
    def execute(self, command: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """Execute with custom orchestration."""
        
        if command == "search":
            return self._search_with_cache(args)
        
        elif command == "stream-results":
            return self._stream_results(args)
        
        elif command == "cache-status":
            return {"status": "success", "cache": self.cache}
        
        return {"status": "error", "error": f"Unknown command: {command}"}
    
    def _search_with_cache(self, args):
        """Search with caching layer."""
        query = args["query"]
        
        # Check cache
        if query in self.cache:
            return {
                "status": "success",
                "source": "cache",
                "data": self.cache[query]
            }
        
        # Cache miss: perform search
        results = self._perform_search(query)
        self.cache[query] = results
        
        return {
            "status": "success",
            "source": "live",
            "data": results
        }
    
    def _stream_results(self, args):
        """Stream results line by line."""
        # Custom streaming logic
        pass
```

### Usage

```bash
# Convert with custom adapter
python converter.py convert myservice \
  --source ./myservice-mcp \
  --pattern custom \
  --adapter-script ./my_adapter.py

# Generated CLI knows to use custom adapter
python cli.py search --query "data" --format json
# {"status": "success", "source": "cache", "data": [...]}

python cli.py stream-results --query "data" --format streaming
# Streams results line by line

python cli.py cache-status
# Shows current cache contents
```

### Generated Skill

```markdown
---
name: myservice-cli
description: MyService custom CLI with intelligent caching and streaming. Use for: (1) Cached searches, (2) Streaming results, (3) Cache management.
---

# MyService CLI

Pattern: CUSTOM

This CLI implements custom orchestration with caching and streaming support.

## Commands

- `search --query STRING` - Search with caching
- `stream-results --query STRING` - Stream results
- `cache-status` - View cache

## Example

```bash
# First search: hits live data
myservice-cli search --query "python" → {"source": "live"}

# Second search: returns cached
myservice-cli search --query "python" → {"source": "cache"}

# Stream results
myservice-cli stream-results --query "python"
```
```

---

## Example 5: Conversion Workflow in DevForgeAI

### Initial Request

```
User: "I want to build a web scraper using Puppeteer"

Dev Persona: "I'll need puppeteer available as a CLI tool"
```

### Conversion Request

```
/convert-mcp-server puppeteer \
  --source npm:mcp-puppeteer@latest
```

### Framework Response

```
✓ Analysis complete: state-based pattern detected (0.98 confidence)
✓ CLI generated: ./puppeteer-cli/cli.py
✓ Skill generated: ./puppeteer-cli/skill/SKILL.md
✓ Tests generated: ./puppeteer-cli/tests/test_cli.py

Next steps:
1. Install: pip install -r puppeteer-cli/requirements.txt
2. Test: python puppeteer-cli/cli.py --help
3. Verify: python -m pytest puppeteer-cli/tests/

Ready to use:
- CLI: python puppeteer-cli/cli.py <command>
- Skill: puppeteer-cli (auto-registered)
```

### Dev Uses It

```
/build "Scrape product prices from website"

Dev Persona now has access to:
- puppeteer-cli skill (knows all commands)
- Session management (create/destroy/list)
- Commands: navigate, screenshot, click, extract-text
- Error handling and recovery

Writes:
SESSION=$(python puppeteer-cli/cli.py session create)
python puppeteer-cli/cli.py navigate --session $SESSION --url "$URL"
python puppeteer-cli/cli.py extract-text --session $SESSION --selector "$SELECTOR" --format json
python puppeteer-cli/cli.py session destroy --session $SESSION
```

### QA Validates

```
/test "Verify scraper works"

QA Persona reviews:
✓ CLI matches skill documentation
✓ Commands execute successfully
✓ Session management works
✓ Error codes properly mapped
✓ Output formats correct (JSON, text, base64)
```

---

## Summary

These examples show:

1. **Pattern 1 (API Wrapper)**: OpenWeather - stateless, direct mapping
2. **Pattern 2 (State-Based)**: Puppeteer & PostgreSQL - sessions, state management
3. **Pattern 3 (Custom)**: MyService - advanced orchestration, caching, streaming
4. **Integration**: How DevForgeAI uses conversions in workflows

Each generates:
- Runnable CLI immediately usable
- Skill documentation Claude understands
- Proper error handling and exit codes
- Ready to integrate into DevForgeAI

The framework handles 80% automatically. Custom adapters handle the 20% of complex cases.
