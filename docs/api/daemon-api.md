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

### `search` - Symbol Search

Search for symbols in the indexed codebase.

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
| `symbol` | string | Yes* | Symbol name to search for |
| `type` | string | Yes* | Filter by type: `function`, `class`, `method`, `variable`, `constant`, `import`, `export` |
| `case_insensitive` | boolean | No | Case-insensitive search (default: false) |
| `regex` | boolean | No | Treat symbol as regex pattern (default: false) |

*At least one of `symbol` or `type` is required.

**Response:**

```json
{
  "id": "req-001",
  "result": [
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
  "error": null
}
```

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

### `index` - Trigger Re-indexing

Trigger a full or incremental re-index operation.

**Request:**

```json
{
  "id": "req-003",
  "method": "index",
  "params": {}
}
```

**Response:**

```json
{
  "id": "req-003",
  "result": {
    "status": "completed",
    "files_indexed": 150,
    "symbols_found": 2500
  },
  "error": null
}
```

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

## Module Structure

```
src/daemon/
├── mod.rs           # Module exports
├── server.rs        # DaemonServer implementation (1204 lines)
└── protocol.rs      # NDJSON protocol types (117 lines)
```

## Related Documentation

- [STORY-007](devforgeai/specs/Stories/archive/STORY-007-daemon-core-ipc.story.md) - Original story specification
- [QA Report](devforgeai/qa/reports/STORY-007-qa-report.md) - Quality validation results
- [CLI Reference](docs/api/cli-reference.md) - Command-line interface

---

*Generated for Treelint v0.7.0 - STORY-007 Daemon Core Architecture*
