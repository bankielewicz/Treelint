# Adapter API Reference

The core extensibility point for the MCP-CLI Converter. When patterns don't fit standard categories, implement a custom adapter.

## Base Adapter Interface

All adapters inherit from (or implement) this interface:

```python
from typing import Dict, Any, Optional
from abc import ABC, abstractmethod


class BaseAdapter(ABC):
    """Base interface for all adapters."""
    
    @abstractmethod
    def execute(self, command: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """
        Execute a command.
        
        Args:
            command: Tool/command name (e.g., "navigate", "click")
            args: Parsed command-line arguments as dict
                  Always includes 'format' key: json|text|base64
        
        Returns:
            Dict with structure:
            {
                "status": "success|error",
                "command": <command>,
                "data": <result>,  # Optional
                "error": <error_msg>,  # If status is error
                "exit_code": <int>  # Optional, defaults to 0 or 1
            }
        
        Raises:
            Exception: Any unhandled exception becomes exit code 1
        """
        pass
```

## Pattern-Specific Adapters

### 1. APIWrapperAdapter

For stateless, API-like MCPs.

```python
class APIWrapperAdapter(BaseAdapter):
    def __init__(self):
        # Initialize any API clients, config, etc.
        pass
    
    def execute(self, command: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """
        Stateless execution. Each call is independent.
        
        Example flow:
        1. Extract format preference from args
        2. Remove 'format' from args (keep only tool params)
        3. Call underlying tool
        4. Return result wrapped in standard response dict
        """
        output_format = args.pop('format', 'json')
        
        try:
            # Call the actual tool
            result = self._call_tool(command, args)
            
            return {
                "status": "success",
                "command": command,
                "data": result
            }
        except Exception as e:
            return {
                "status": "error",
                "command": command,
                "error": str(e),
                "exit_code": 1
            }
    
    def _call_tool(self, tool_name: str, params: Dict[str, Any]) -> Any:
        """
        Call the underlying MCP tool.
        
        Args:
            tool_name: Name of the tool to call
            params: Tool parameters (already filtered)
        
        Returns:
            Tool result (will be JSON-serializable)
        
        Raise exceptions for errors; they're caught by execute()
        """
        raise NotImplementedError(f"Implement _call_tool for: {tool_name}")
```

**When to implement**:
- External API calls (weather, translation, data lookup)
- Simple data transformation
- Single-tool-call workflows

**Implementation checklist**:
- [ ] Override `_call_tool()`
- [ ] Handle API errors → Python exceptions
- [ ] Return JSON-serializable data
- [ ] Map HTTP errors to semantic exceptions

---

### 2. StateBasedAdapter

For stateful operations requiring session management.

```python
from uuid import uuid4
from datetime import datetime, timedelta
from typing import Dict, Any


class StateBasedAdapter(BaseAdapter):
    def __init__(self, session_timeout_hours: int = 1):
        self.session_timeout = timedelta(hours=session_timeout_hours)
        self.sessions: Dict[str, SessionState] = {}
    
    def execute(self, command: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """
        Stateful execution with session management.
        
        Special handling:
        - If command == "session", delegate to _handle_session_command()
        - Otherwise, require --session argument
        - Load session state from manager
        - Execute tool within session context
        - Return result with session ID
        """
        
        # Handle special session commands
        if command == "session":
            return self._handle_session_command(args)
        
        # Regular tool: require session
        session_id = args.get("session")
        if not session_id:
            return {
                "status": "error",
                "error": "Tool commands require --session argument",
                "exit_code": 2
            }
        
        # Get and validate session
        try:
            session = self._get_session(session_id)
        except ValueError as e:
            return {
                "status": "error",
                "error": str(e),
                "exit_code": 4
            }
        
        # Execute tool in session context
        try:
            result = self._call_tool_in_session(command, args, session)
            
            # Update session state if tool modified it
            self.sessions[session_id]["state"] = session["state"]
            
            return {
                "status": "success",
                "command": command,
                "session": session_id,
                "data": result
            }
        
        except Exception as e:
            return {
                "status": "error",
                "command": command,
                "session": session_id,
                "error": str(e),
                "exit_code": 1
            }
    
    def _handle_session_command(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """
        Handle session lifecycle: create, destroy, list.
        
        Subcommands:
        - create [--name NAME]: Create new session → return session_id
        - destroy --session SESSION_ID: Destroy session
        - list: List all active sessions
        """
        
        subcommand = args.get("subcommand", "create")
        
        if subcommand == "create":
            session_id = self._create_session(args.get("name", ""))
            return {
                "status": "success",
                "session_id": session_id
            }
        
        elif subcommand == "destroy":
            self._destroy_session(args["session"])
            return {"status": "success"}
        
        elif subcommand == "list":
            sessions_info = [
                {
                    "id": sid,
                    "name": s.get("name", ""),
                    "created": s["created"].isoformat(),
                    "age_seconds": (datetime.now() - s["created"]).total_seconds()
                }
                for sid, s in self.sessions.items()
            ]
            return {
                "status": "success",
                "sessions": sessions_info
            }
        
        return {
            "status": "error",
            "error": f"Unknown session command: {subcommand}",
            "exit_code": 2
        }
    
    def _create_session(self, name: str = "") -> str:
        """Create new session, return ID."""
        session_id = str(uuid4())[:8]
        self.sessions[session_id] = {
            "created": datetime.now(),
            "name": name,
            "state": {}
        }
        return session_id
    
    def _get_session(self, session_id: str) -> Dict[str, Any]:
        """Get session, raise ValueError if not found or expired."""
        if session_id not in self.sessions:
            raise ValueError(f"Session not found: {session_id}")
        
        session = self.sessions[session_id]
        age = datetime.now() - session["created"]
        
        if age > self.session_timeout:
            del self.sessions[session_id]
            raise ValueError(f"Session expired: {session_id}")
        
        return session
    
    def _destroy_session(self, session_id: str):
        """Destroy session."""
        if session_id in self.sessions:
            del self.sessions[session_id]
    
    def _call_tool_in_session(
        self, 
        tool_name: str, 
        params: Dict[str, Any],
        session: Dict[str, Any]
    ) -> Any:
        """
        Call tool with session context.
        
        Args:
            tool_name: Tool to execute
            params: Tool parameters
            session: Session dict with "state" key containing session state
        
        Returns:
            Tool result
        
        The session["state"] dict persists between tool calls,
        so tools can read/write shared state.
        """
        raise NotImplementedError(f"Implement _call_tool_in_session for: {tool_name}")
```

**When to implement**:
- Browser automation (Puppeteer, Playwright)
- Database clients
- File operations with working directory
- Interactive workflows

**Implementation checklist**:
- [ ] Override `_call_tool_in_session()`
- [ ] Use session["state"] for persistence
- [ ] Clean up resources in _destroy_session() if needed
- [ ] Handle timeout gracefully
- [ ] Manage concurrent sessions safely

---

### 3. CustomAdapter

For patterns that don't fit standard categories.

```python
class CustomAdapter(BaseAdapter):
    """
    Custom adapter for non-standard MCPs.
    Implement execute() with your full logic.
    """
    
    def __init__(self, config: Optional[Dict[str, Any]] = None):
        self.config = config or {}
    
    def execute(self, command: str, args: Dict[str, Any]) -> Dict[str, Any]:
        """
        Fully custom execution logic.
        
        You have complete control:
        - Can maintain state across calls
        - Can implement hybrid patterns
        - Can do complex orchestration
        - Can stream results
        
        Must return standard response format:
        {
            "status": "success|error",
            "command": <str>,
            "data": <any>,  # Optional
            "error": <str>,  # If error
            "exit_code": <int>  # Optional
        }
        """
        
        try:
            # Your orchestration logic
            result = self._execute_custom(command, args)
            
            return {
                "status": "success",
                "command": command,
                "data": result
            }
        
        except Exception as e:
            return {
                "status": "error",
                "command": command,
                "error": str(e),
                "exit_code": 1
            }
    
    def _execute_custom(self, command: str, args: Dict[str, Any]) -> Any:
        """Override with your implementation."""
        raise NotImplementedError()
```

**When to implement**:
- Hybrid patterns (API + state)
- Complex orchestration
- Streaming operations
- Event-driven workflows
- Proprietary patterns

---

## Common Implementation Patterns

### Pattern: Wrapper Around External CLI

```python
class ExternalCLIAdapter(StateBasedAdapter):
    """Wrap an external CLI tool."""
    
    def _call_tool_in_session(self, tool_name, params, session):
        import subprocess
        import json
        
        # Build command line
        cmd = ["external-cli", tool_name]
        for k, v in params.items():
            if k not in ["session", "format"]:
                cmd.extend([f"--{k}", str(v)])
        
        # Execute
        result = subprocess.run(cmd, capture_output=True, text=True)
        
        if result.returncode != 0:
            raise RuntimeError(f"Command failed: {result.stderr}")
        
        # Parse output
        return json.loads(result.stdout)
```

### Pattern: Database Client

```python
class DatabaseAdapter(StateBasedAdapter):
    """Connect to database, maintain connections in sessions."""
    
    def _call_tool_in_session(self, tool_name, params, session):
        # Get or create connection in session state
        conn = session["state"].get("connection")
        
        if not conn:
            conn = self._create_connection(params)
            session["state"]["connection"] = conn
        
        # Execute query
        if tool_name == "query":
            return self._execute_query(conn, params["sql"])
        
        elif tool_name == "execute":
            return self._execute_statement(conn, params["sql"])
        
        # ... etc
    
    def _destroy_session(self, session_id):
        session = self.sessions.get(session_id)
        if session and "connection" in session["state"]:
            session["state"]["connection"].close()
        
        super()._destroy_session(session_id)
```

### Pattern: Event-Driven Streaming

```python
class StreamingAdapter(CustomAdapter):
    """Stream results as events."""
    
    def execute(self, command: str, args: Dict[str, Any]) -> Dict[str, Any]:
        output_format = args.get("format", "json")
        
        if output_format == "streaming":
            # Special handling for streaming
            return self._stream_results(command, args)
        
        # Regular response
        result = self._get_result(command, args)
        return {
            "status": "success",
            "command": command,
            "data": result
        }
    
    def _stream_results(self, command, args):
        """Stream results line-by-line."""
        # Implementation depends on your streaming format
        pass
```

---

## Error Handling Best Practices

### Mapping Exceptions to Exit Codes

```python
EXIT_CODES = {
    ValueError: 2,           # Bad input
    TimeoutError: 3,         # Timeout
    ConnectionError: 4,      # Can't reach resource
    PermissionError: 5,      # Auth failed
    RuntimeError: 1,         # Generic error
}

def execute(self, command, args):
    try:
        result = self._do_work(command, args)
        return {"status": "success", "data": result}
    
    except Exception as e:
        exit_code = EXIT_CODES.get(type(e), 1)
        return {
            "status": "error",
            "error": str(e),
            "exit_code": exit_code
        }
```

### Actionable Error Messages

Instead of:
```
"error": "Connection failed"
```

Use:
```
"error": "Connection to localhost:5432 failed. Is PostgreSQL running? Try: brew start postgresql"
```

---

## Testing Your Adapter

```python
def test_adapter():
    adapter = MyAdapter()
    
    # Test basic command
    result = adapter.execute("test_command", {"param": "value", "format": "json"})
    
    assert result["status"] == "success"
    assert "data" in result
    
    # Test error handling
    result = adapter.execute("invalid", {})
    assert result["status"] == "error"
    assert result["exit_code"] in [1, 2, 3, 4, 5]
```

---

## Registering Your Adapter

In the converter config:

```yaml
# converter.yaml
custom_adapter: ./my_adapter.py
adapter_class: MyAdapter
```

Or CLI:
```bash
devforgeai-mcp-cli-converter convert my-service \
  --pattern custom \
  --adapter-script ./my_adapter.py
```

The framework will:
1. Import your module
2. Instantiate your adapter class
3. Call execute() for each command
4. Handle response formatting and exit codes
