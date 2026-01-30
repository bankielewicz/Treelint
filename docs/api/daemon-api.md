# Treelint Daemon API Reference

The Treelint daemon provides instant symbol search via IPC communication.

## Overview

The daemon maintains an always-fresh symbol index in memory, enabling sub-5ms search latency instead of the 20-60 second on-demand indexing time.

### Platform Support

| Platform | IPC Transport | Socket Path |
|----------|---------------|-------------|
| Unix/macOS | Unix domain socket | `.treelint/daemon.sock` |
| Windows | Named pipe | `\\.\pipe\treelint-daemon` |

### Protocol

The daemon uses **NDJSON** (Newline-Delimited JSON) protocol:

- **Request**: Single line of JSON terminated by newline
- **Response**: Single line of JSON terminated by newline

## Request Format

```json
{
  "id": "unique-request-id",
  "method": "search|status|index",
  "params": { ... }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Unique request identifier for correlation |
| `method` | string | Yes | Method to invoke: `search`, `status`, `index` |
| `params` | object | Yes | Method-specific parameters (can be empty `{}`) |

## Response Format

### Success Response

```json
{
  "id": "request-id",
  "result": { ... },
  "error": null
}
```

### Error Response

```json
{
  "id": "request-id",
  "result": null,
  "error": {
    "code": "E001",
    "message": "Human-readable error message"
  }
}
```

## Methods

### `search` - Symbol Search (STORY-012)

Search for symbols in the indexed codebase. The daemon now queries the **actual IndexStorage** using QueryFilters, returning real symbol matches from the database.

**Request:**

```json
{
  "id": "req-001",
  "method": "search",
  "params": {
    "symbol": "validateUser",
    "type": "function",
    "case_insensitive": false,
    "regex": false
  }
}
```

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `symbol` | string | Yes* | Symbol name to search for (uses pattern matching) |
| `type` | string | Yes* | Filter by type: `function`, `class`, `method`, `variable`, `constant`, `import`, `export` |
| `case_insensitive` | boolean | No | Case-insensitive search (default: false) |
| `regex` | boolean | No | Treat symbol as regex pattern (default: false) |

*At least one of `symbol` or `type` is required.

**Response:**

```json
{
  "id": "req-001",
  "result": {
    "symbols": [
      {
        "name": "validateUser",
        "type": "function",
        "file": "/path/to/auth/validator.py",
        "line_start": 10,
        "line_end": 45,
        "signature": "def validateUser(email: str, password: str) -> bool",
        "body": "def validateUser(email: str, password: str) -> bool:\n    ..."
      }
    ],
    "total": 1
  },
  "error": null
}
```

**Search Behavior (STORY-012):**

- Queries **actual IndexStorage** using `QueryFilters.with_name_pattern()`
- Returns real matching symbols from `.treelint/index.db`
- Each symbol includes: name, type, file, lines, signature, body
- Empty `symbols` array only returned if no matches found
- `total` reflects actual match count

**Performance (STORY-012):**

- Search latency: < 50ms (p95) for queries returning < 100 results
- Same output format as CLI search command

### `status` - Daemon Status

Get current daemon status and statistics.

**Request:**

```json
{
  "id": "req-002",
  "method": "status",
  "params": {}
}
```

**Response:**

```json
{
  "id": "req-002",
  "result": {
    "status": "ready",
    "indexed_files": 150,
    "indexed_symbols": 2500,
    "last_index_time": "2026-01-29T12:30:45Z",
    "uptime_seconds": 3600,
    "pid": 12345,
    "socket_path": "/project/.treelint/daemon.sock"
  },
  "error": null
}
```

**Status Values:**

| Status | Description |
|--------|-------------|
| `starting` | Daemon is initializing |
| `ready` | Daemon is ready to accept requests |
| `indexing` | Daemon is currently indexing files |
| `stopping` | Daemon is shutting down |

### Extended Status (with File Watcher)

When the file watcher is active, the status response includes additional fields:

```json
{
  "id": "req-002",
  "result": {
    "status": "ready",
    "indexed_files": 150,
    "indexed_symbols": 2500,
    "last_index_time": "2026-01-29T12:30:45Z",
    "uptime_seconds": 3600,
    "pid": 12345,
    "socket_path": "/project/.treelint/daemon.sock",
    "watcher": {
      "enabled": true,
      "watching_paths": 150,
      "events_processed": 1234,
      "errors_count": 0,
      "last_event": "2026-01-30T10:15:30Z"
    }
  },
  "error": null
}
```

**Watcher Status Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `enabled` | boolean | Whether file watching is active |
| `watching_paths` | number | Number of files/directories being monitored |
| `events_processed` | number | Total file change events processed |
| `errors_count` | number | Number of watcher errors encountered |
| `last_event` | string | ISO 8601 timestamp of last processed event |

### `index` - Trigger Re-indexing (STORY-012)

Trigger a full or incremental re-index operation. The daemon now performs **actual indexing** using SymbolExtractor to parse source files and store symbols in IndexStorage.

**Request (Incremental):**

```json
{
  "id": "req-003",
  "method": "index",
  "params": {}
}
```

**Request (Force Full Rebuild):**

```json
{
  "id": "req-003",
  "method": "index",
  "params": {
    "force": true
  }
}
```

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `force` | boolean | No | If `true`, clears existing index and re-parses all files (default: false) |

**Response:**

```json
{
  "id": "req-003",
  "result": {
    "status": "completed",
    "files_indexed": 150,
    "symbols_found": 2500,
    "project_root": "/path/to/project"
  },
  "error": null
}
```

**Index Behavior (STORY-012):**

| Mode | Behavior |
|------|----------|
| Incremental (default) | Uses hash-based change detection, only re-parses modified files |
| Force (`force: true`) | Clears all index entries, re-parses all source files |

**What Happens:**

1. Daemon discovers source files in project directory
2. SymbolExtractor parses each file using tree-sitter
3. Extracted symbols stored in IndexStorage (`.treelint/index.db`)
4. Response includes **actual** `files_indexed` and `symbols_found` counts

**Performance (STORY-012):**

- < 1 second per 100 files for incremental index
- Full re-index scales linearly with codebase size

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| E001 | Index Not Ready | Index is still being built. Wait and retry. |
| E002 | Invalid Method | Unknown method name. Check spelling. |
| E003 | Invalid Params | Missing or invalid parameters. Check required fields. |

## Example Usage

### Python Client

```python
import socket
import json

def daemon_request(method: str, params: dict) -> dict:
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect('.treelint/daemon.sock')

    request = {
        "id": "1",
        "method": method,
        "params": params
    }

    sock.sendall(json.dumps(request).encode() + b'\n')
    response = sock.recv(65536).decode()
    sock.close()

    return json.loads(response)

# Search for functions
result = daemon_request("search", {"symbol": "main", "type": "function"})
print(result)

# Get status
status = daemon_request("status", {})
print(f"Indexed {status['result']['indexed_symbols']} symbols")
```

### Rust Client

```rust
use std::os::unix::net::UnixStream;
use std::io::{BufRead, BufReader, Write};
use serde_json::{json, Value};

fn daemon_request(method: &str, params: Value) -> Value {
    let stream = UnixStream::connect(".treelint/daemon.sock").unwrap();
    let mut writer = &stream;
    let mut reader = BufReader::new(&stream);

    let request = json!({
        "id": "1",
        "method": method,
        "params": params
    });

    writeln!(writer, "{}", request).unwrap();
    writer.flush().unwrap();

    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    serde_json::from_str(&response).unwrap()
}

fn main() {
    // Search for functions
    let result = daemon_request("search", json!({"symbol": "main", "type": "function"}));
    println!("{}", serde_json::to_string_pretty(&result).unwrap());

    // Get status
    let status = daemon_request("status", json!({}));
    println!("Status: {}", status["result"]["status"]);
}
```

### Bash/Netcat

```bash
# Connect and send search request
echo '{"id":"1","method":"search","params":{"symbol":"main","type":"function"}}' | nc -U .treelint/daemon.sock

# Get status
echo '{"id":"2","method":"status","params":{}}' | nc -U .treelint/daemon.sock
```

## Lifecycle

### Startup

1. Daemon creates `.treelint/` directory if needed
2. Checks for existing socket/pipe (fails if another daemon running)
3. Cleans up stale socket if previous daemon crashed
4. Creates socket with user-only permissions (0600 on Unix)
5. Enters event loop, ready to accept connections

### Graceful Shutdown

1. Stops accepting new connections
2. Waits up to 5 seconds for active requests to complete
3. Removes socket/pipe file
4. Exits with code 0

### Signal Handling

| Signal | Behavior |
|--------|----------|
| SIGTERM | Graceful shutdown |
| SIGINT | Graceful shutdown |

## Performance

| Metric | Target | Typical |
|--------|--------|---------|
| Search latency (p95) | <5ms | ~2ms |
| Status latency (p95) | <1ms | ~0.5ms |
| Idle CPU usage | <0.1% | ~0% |
| Memory (base) | <50MB | ~30MB |
| Concurrent connections | 10+ | Tested with 10 |

## Security

- **Unix socket permissions**: 0600 (user-only read/write)
- **Windows named pipe**: DACL restricts to current user
- **No network exposure**: IPC only, localhost communication
- **Input validation**: All JSON fields validated before processing

## File Watcher (STORY-008)

The daemon includes a file watcher that automatically updates the index when source files change.

### Features

| Feature | Description |
|---------|-------------|
| **Cross-platform** | Uses notify crate (inotify on Linux, FSEvents on macOS, ReadDirectoryChangesW on Windows) |
| **Debouncing** | 100ms window prevents redundant re-indexing during rapid saves (IDE auto-save) |
| **Hash-based detection** | SHA-256 comparison skips unchanged files (e.g., `touch` command) |
| **Gitignore support** | Respects project `.gitignore` patterns |
| **Extension filtering** | Only monitors supported file types (`.py`, `.ts`, `.tsx`, `.rs`, `.md`) |
| **Incremental updates** | Only re-indexes changed files, not entire codebase |

### Event Types

| Event | Description | Index Action |
|-------|-------------|--------------|
| `Create` | New file created | Parse and add symbols |
| `Modify` | File content changed | Remove old symbols, add new |
| `Delete` | File deleted | Remove symbols for file |
| `Rename` | File renamed | Treat as delete + create |

### Performance

| Metric | Target | Typical |
|--------|--------|---------|
| Event detection latency | <500ms | ~100ms |
| File change → index update | <1s | ~200ms |
| Debounce window | 100ms | 100ms (fixed) |
| Idle CPU usage (watching) | <1% | ~0.1% |

### Error Recovery

The watcher handles errors gracefully without crashing:

| Error Type | Behavior |
|------------|----------|
| Permission denied | Logged, continue watching other files |
| Too many watches (inotify) | Logged warning, suggest increasing limit |
| Transient I/O error | Logged, retry on next event |

### Watcher API Types

**`WatcherEventKind`**

```rust
pub enum WatcherEventKind {
    Create,  // New file created
    Modify,  // File content changed
    Delete,  // File deleted
    Rename,  // File renamed (detected as delete + create)
}
```

**`WatcherStatus`**

```rust
pub struct WatcherStatus {
    pub enabled: bool,
    pub watching_paths: usize,
    pub events_processed: u64,
    pub errors_count: u32,
    pub last_event: Option<String>,
}
```

**`IndexStats`**

```rust
pub struct IndexStats {
    pub symbols_added: usize,
    pub symbols_removed: usize,
    pub parse_time_ms: u64,
}
```

## Module Structure

```
src/daemon/
├── mod.rs           # Module exports
├── server.rs        # DaemonServer implementation (1204 lines)
├── protocol.rs      # NDJSON protocol types (117 lines)
└── watcher.rs       # File watcher implementation (1098 lines) [STORY-008]
```

## Daemon-Index Integration (STORY-012)

STORY-012 completed the daemon by wiring it to the actual IndexStorage and SymbolExtractor components:

### What Changed

| Before STORY-012 | After STORY-012 |
|------------------|-----------------|
| Search returned stub/empty results | Search queries actual IndexStorage |
| Index returned hardcoded counts | Index triggers real SymbolExtractor parsing |
| Status returned stub values | Status returns accurate file/symbol counts |
| No force rebuild option | Force flag clears and rebuilds entire index |

### Key Implementation Details

- **Search Handler**: Uses `QueryFilters.with_name_pattern()` and `ctx.storage.lock()` to query IndexStorage
- **Index Handler**: Creates `SymbolExtractor::new()` and calls `extract_from_file()` for each source file
- **Status Handler**: Returns `ctx.indexed_files.load()` and `ctx.indexed_symbols.load()` from atomic counters
- **Force Index**: Parses `force` param and calls `storage.clear_all()` before re-indexing

### Error Codes (Updated)

| Code | Meaning | When Returned |
|------|---------|---------------|
| E001 | Index Not Ready | Storage not initialized at daemon start |
| E002 | Invalid Method | Unknown method name |
| E003 | Invalid Params | Missing required `symbol` or `type` parameter |

## Related Documentation

- [STORY-007](devforgeai/specs/Stories/archive/STORY-007-daemon-core-ipc.story.md) - Daemon Core Architecture
- [STORY-008](devforgeai/specs/Stories/archive/STORY-008-file-watcher-incremental-index.story.md) - File Watcher & Incremental Indexing
- [STORY-012](devforgeai/specs/Stories/archive/STORY-012-daemon-index-integration.story.md) - Daemon-Index Integration
- [QA Report STORY-007](devforgeai/qa/reports/STORY-007-qa-report.md) - Daemon quality validation
- [QA Report STORY-008](devforgeai/qa/reports/STORY-008-qa-report.md) - File watcher quality validation
- [QA Report STORY-012](devforgeai/qa/reports/STORY-012-qa-report.md) - Daemon-index integration quality validation
- [CLI Reference](docs/api/cli-reference.md) - Command-line interface
- [Library Reference](docs/api/library-reference.md) - Rust library API

---

*Generated for Treelint v0.12.0 - STORY-007 Daemon Core, STORY-008 File Watcher, STORY-012 Daemon-Index Integration*
