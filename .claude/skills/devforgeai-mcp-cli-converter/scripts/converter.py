#!/usr/bin/env python3
"""
MCP-to-CLI Converter
Converts MCP servers into standalone CLI utilities with auto-generated skills.
"""

import json
import argparse
import sys
from pathlib import Path
from typing import Dict, Any, List, Optional
import subprocess

class MCPAnalyzer:
    """Analyzes MCP server code/schema to detect patterns."""
    
    def __init__(self, mcp_source: str, mcp_lang: str = "python"):
        self.source = mcp_source
        self.lang = mcp_lang
        self.tools: List[Dict[str, Any]] = []
        self.mcp_type = ""
        self.detected_pattern = ""
        
    def analyze(self) -> Dict[str, Any]:
        """Run analysis and return structured info about the MCP."""
        
        # Load MCP definition
        mcp_def = self._load_mcp_definition()
        if not mcp_def:
            return {"error": "Could not load MCP definition"}
        
        self.tools = mcp_def.get("tools", [])
        
        # Detect pattern
        pattern = self._detect_pattern()
        self.detected_pattern = pattern
        
        # Analyze state requirements
        state_analysis = self._analyze_state_management()
        
        # Build recommendations
        recommendations = self._build_recommendations(pattern, state_analysis)
        
        return {
            "mcp_type": self.mcp_type,
            "detected_pattern": pattern,
            "confidence": self._confidence_score(pattern),
            "tools": self.tools,
            "state_management": state_analysis,
            "conversion_recommendations": recommendations,
            "language": self.lang
        }
    
    def _load_mcp_definition(self) -> Optional[Dict[str, Any]]:
        """Load MCP from schema file, package, or source code."""
        
        # Try loading from schema file
        if self.source.endswith(".json"):
            try:
                with open(self.source) as f:
                    return json.load(f)
            except Exception as e:
                print(f"Error loading schema: {e}", file=sys.stderr)
                return None
        
        # Try loading from installed package (npm: prefix)
        if self.source.startswith("npm:"):
            return self._load_from_npm_package(self.source)
        
        # Try loading from local source code
        if Path(self.source).is_dir():
            return self._extract_from_source_code(self.source)
        
        return None
    
    def _extract_from_source_code(self, source_dir: str) -> Optional[Dict[str, Any]]:
        """Extract MCP tools from source code by analyzing tool definitions."""
        
        source_path = Path(source_dir)
        
        # Look for common patterns
        tools = []
        
        if self.lang == "python":
            # Parse Python tool definitions
            for py_file in source_path.rglob("*.py"):
                tools.extend(self._extract_python_tools(py_file))
        
        elif self.lang == "typescript" or self.lang == "javascript":
            # Parse TypeScript/JS tool definitions
            for ts_file in source_path.rglob("*.ts"):
                tools.extend(self._extract_ts_tools(ts_file))
        
        if not tools:
            # Fallback: try to find index/main files with tool registration
            return None
        
        return {"tools": tools}
    
    def _extract_python_tools(self, py_file: Path) -> List[Dict[str, Any]]:
        """Extract @mcp.tool() decorated functions from Python files."""
        tools = []
        try:
            with open(py_file) as f:
                content = f.read()
                
            # Simple regex-based extraction of @mcp.tool() definitions
            import re
            
            # Find decorated functions
            pattern = r'@.*\.tool\(\s*\)?\s*(?:async\s+)?def\s+(\w+)\s*\((.*?)\).*?(?:return|""")'
            
            for match in re.finditer(pattern, content, re.DOTALL):
                tool_name = match.group(1)
                params_str = match.group(2)
                
                # Extract parameter names
                param_names = [p.strip().split(":")[0] for p in params_str.split(",") if p.strip()]
                
                tools.append({
                    "name": tool_name,
                    "inputs": {name: "string" for name in param_names},  # Simplified
                    "outputs": "any",
                    "async": "async def" in match.group(0)
                })
        except Exception:
            pass
        
        return tools
    
    def _extract_ts_tools(self, ts_file: Path) -> List[Dict[str, Any]]:
        """Extract tool definitions from TypeScript files."""
        # Similar logic for TypeScript
        return []
    
    def _load_from_npm_package(self, package_spec: str) -> Optional[Dict[str, Any]]:
        """Load from npm package (e.g., 'npm:mcp-puppeteer@latest')."""
        # Implementation would fetch package and extract schema
        return None
    
    def _detect_pattern(self) -> str:
        """Detect which pattern best fits this MCP."""
        
        if not self.tools:
            return "unknown"
        
        # Heuristics for pattern detection
        
        # API Wrapper: stateless, no side-effects mentioned
        stateless_tools = [t for t in self.tools if "side_effects" not in t or not t.get("side_effects")]
        if len(stateless_tools) == len(self.tools) and len(self.tools) <= 5:
            return "api-wrapper"
        
        # State-Based: tools reference session/connection/context
        state_keywords = ["session", "connection", "context", "state", "browser", "page", "transaction"]
        tool_names = [t["name"] for t in self.tools]
        tool_names_str = " ".join(tool_names)
        
        if any(keyword in tool_names_str.lower() for keyword in state_keywords):
            return "state-based"
        
        # Look for common stateful patterns
        if any("navigate" in t["name"] or "click" in t["name"] for t in self.tools):
            return "state-based"
        
        # Default to API wrapper if unsure
        return "api-wrapper"
    
    def _analyze_state_management(self) -> Dict[str, Any]:
        """Analyze if/how state is managed."""
        
        stateful_tools = [t for t in self.tools if t.get("side_effects")]
        
        return {
            "stateful": len(stateful_tools) > 0,
            "session_required": self.detected_pattern == "state-based",
            "concurrent_sessions": True,  # Most can handle this
            "state_keywords": self._find_state_keywords()
        }
    
    def _find_state_keywords(self) -> List[str]:
        """Find state-related keywords in tool names/descriptions."""
        keywords = []
        state_words = ["session", "connection", "context", "browser", "page"]
        
        for tool in self.tools:
            tool_name = tool.get("name", "").lower()
            for word in state_words:
                if word in tool_name:
                    keywords.append(word)
        
        return list(set(keywords))
    
    def _build_recommendations(self, pattern: str, state_analysis: Dict) -> List[str]:
        """Generate recommendations for conversion."""
        
        recommendations = []
        
        if pattern == "state-based":
            recommendations.append("Use ephemeral session model")
            recommendations.append("Queue operations within session")
            recommendations.extend([
                "Implement session timeout (recommend 1 hour)",
                "Add session cleanup handlers"
            ])
        
        elif pattern == "api-wrapper":
            recommendations.append("Direct 1:1 tool → CLI command mapping")
            recommendations.append("Handle HTTP error codes → exit codes")
            recommendations.append("Normalize JSON responses")
        
        if any("binary" in t.get("outputs", "") for t in self.tools):
            recommendations.append("Stream binary outputs as base64")
        
        return recommendations
    
    def _confidence_score(self, pattern: str) -> float:
        """Return confidence in pattern detection (0-1)."""
        # Simplified scoring
        if pattern == "unknown":
            return 0.3
        elif pattern == "api-wrapper" and len(self.tools) <= 10:
            return 0.95
        elif pattern == "state-based":
            return 0.85
        return 0.7


class CLIGenerator:
    """Generates CLI wrapper from MCP definition."""
    
    def __init__(self, analysis: Dict[str, Any], output_dir: str, pattern: str):
        self.analysis = analysis
        self.output_dir = Path(output_dir)
        self.pattern = pattern
        self.output_dir.mkdir(parents=True, exist_ok=True)
    
    def generate(self) -> bool:
        """Generate CLI structure."""
        
        try:
            # Create directory structure
            self._create_structure()
            
            # Generate main CLI script
            self._generate_cli_main()
            
            # Generate adapters
            self._generate_adapter()
            
            # Generate utilities
            self._generate_utils()
            
            # Generate requirements.txt
            self._generate_requirements()
            
            # Generate README
            self._generate_readme()
            
            # Generate tests
            self._generate_tests()
            
            print(f"✓ CLI generated in {self.output_dir}")
            return True
            
        except Exception as e:
            print(f"Error generating CLI: {e}", file=sys.stderr)
            return False
    
    def _create_structure(self):
        """Create directory structure."""
        dirs = [
            self.output_dir / "adapters",
            self.output_dir / "utils",
            self.output_dir / "tests",
            self.output_dir / "skill" / "references",
            self.output_dir / "skill" / "scripts",
            self.output_dir / "skill" / "assets"
        ]
        
        for d in dirs:
            d.mkdir(parents=True, exist_ok=True)
    
    def _generate_cli_main(self):
        """Generate main CLI entry point."""
        
        template = '''#!/usr/bin/env python3
"""
Auto-generated CLI from MCP server conversion.
Generated pattern: {pattern}
Tools: {tool_count}
"""

import argparse
import json
import sys
from pathlib import Path

from adapters import {adapter_class}
from utils import ErrorHandler, OutputFormatter


def main():
    parser = argparse.ArgumentParser(description="MCP CLI Interface")
    
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
{command_definitions}
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return 1
    
    try:
        adapter = {adapter_class}()
        result = adapter.execute(args.command, vars(args))
        
        formatter = OutputFormatter(args.format if hasattr(args, 'format') else 'json')
        output = formatter.format(result)
        print(output)
        
        return 0
        
    except Exception as e:
        ErrorHandler.handle(e)
        return 1


if __name__ == "__main__":
    sys.exit(main())
'''
        
        adapter_class = {
            "api-wrapper": "APIWrapperAdapter",
            "state-based": "StateBasedAdapter",
            "custom": "CustomAdapter"
        }.get(self.pattern, "BaseAdapter")
        
        command_defs = self._generate_command_definitions()
        
        cli_content = template.format(
            pattern=self.pattern,
            tool_count=len(self.analysis.get("tools", [])),
            adapter_class=adapter_class,
            command_definitions=command_defs
        )
        
        with open(self.output_dir / "cli.py", "w") as f:
            f.write(cli_content)
    
    def _generate_command_definitions(self) -> str:
        """Generate argparse subparsers for each tool."""
        
        defs = ""
        
        for tool in self.analysis.get("tools", []):
            tool_name = tool["name"]
            inputs = tool.get("inputs", {})
            
            def_line = f"    {tool_name}_parser = subparsers.add_parser('{tool_name}')\n"
            
            for param_name in inputs.keys():
                def_line += f"    {tool_name}_parser.add_argument('--{param_name}', required=True)\n"
            
            def_line += f"    {tool_name}_parser.add_argument('--format', default='json', choices=['json', 'text', 'base64'])\n\n"
            
            defs += def_line
        
        return defs
    
    def _generate_adapter(self):
        """Generate pattern-specific adapter."""
        
        if self.pattern == "api-wrapper":
            self._generate_api_wrapper_adapter()
        elif self.pattern == "state-based":
            self._generate_state_based_adapter()
        else:
            self._generate_custom_adapter_template()
    
    def _generate_api_wrapper_adapter(self):
        """Generate API wrapper adapter template."""
        
        template = '''"""API Wrapper Adapter - stateless tool calls"""

class APIWrapperAdapter:
    def __init__(self):
        pass
    
    def execute(self, command: str, args: dict):
        """Execute command by calling tool."""
        
        # Remove format from args
        output_format = args.pop('format', 'json')
        
        # Remove None values
        args = {k: v for k, v in args.items() if v is not None}
        
        # Call actual MCP tool (stub)
        result = self._call_tool(command, args)
        
        return {
            "status": "success",
            "command": command,
            "data": result
        }
    
    def _call_tool(self, tool_name: str, params: dict):
        """Call the underlying MCP tool. Override this with actual logic."""
        raise NotImplementedError(f"Implement tool call for: {tool_name}")
'''
        
        with open(self.output_dir / "adapters" / "api_wrapper_adapter.py", "w") as f:
            f.write(template)
    
    def _generate_state_based_adapter(self):
        """Generate state-based adapter template."""
        
        template = '''"""State-Based Adapter - maintains session state"""

from typing import Dict, Any
import uuid
from datetime import datetime, timedelta


class SessionManager:
    def __init__(self, timeout_hours: int = 1):
        self.sessions: Dict[str, Dict[str, Any]] = {}
        self.timeout = timedelta(hours=timeout_hours)
    
    def create(self, name: str = "") -> str:
        """Create new session, return session ID."""
        session_id = str(uuid.uuid4())[:8]
        self.sessions[session_id] = {
            "created": datetime.now(),
            "name": name,
            "state": {}
        }
        return session_id
    
    def get(self, session_id: str) -> Dict[str, Any]:
        """Get session, raise if not found or expired."""
        if session_id not in self.sessions:
            raise ValueError(f"Session not found: {session_id}")
        
        session = self.sessions[session_id]
        if datetime.now() - session["created"] > self.timeout:
            del self.sessions[session_id]
            raise ValueError(f"Session expired: {session_id}")
        
        return session
    
    def destroy(self, session_id: str):
        """Destroy session."""
        if session_id in self.sessions:
            del self.sessions[session_id]
    
    def list_sessions(self):
        """List all active sessions."""
        return [
            {
                "id": sid,
                "name": s.get("name", ""),
                "created": s["created"].isoformat(),
                "age_seconds": (datetime.now() - s["created"]).total_seconds()
            }
            for sid, s in self.sessions.items()
        ]


class StateBasedAdapter:
    def __init__(self):
        self.session_manager = SessionManager()
    
    def execute(self, command: str, args: dict):
        """Execute command with session management."""
        
        if command == "session":
            return self._handle_session_command(args)
        
        # Regular tool call
        session_id = args.get("session")
        if not session_id:
            raise ValueError("--session required for tool commands")
        
        session = self.session_manager.get(session_id)
        
        # Execute tool (stub)
        result = self._call_tool(command, args, session)
        
        return {
            "status": "success",
            "command": command,
            "session": session_id,
            "data": result
        }
    
    def _handle_session_command(self, args: dict):
        """Handle session lifecycle commands."""
        
        subcommand = args.get("subcommand")
        
        if subcommand == "create":
            session_id = self.session_manager.create(args.get("name", ""))
            return {
                "status": "success",
                "session_id": session_id
            }
        
        elif subcommand == "destroy":
            self.session_manager.destroy(args["session"])
            return {"status": "success"}
        
        elif subcommand == "list":
            return {
                "status": "success",
                "sessions": self.session_manager.list_sessions()
            }
        
        raise ValueError(f"Unknown session command: {subcommand}")
    
    def _call_tool(self, tool_name: str, params: dict, session: dict):
        """Call tool within session context. Override with actual logic."""
        raise NotImplementedError(f"Implement tool call for: {tool_name}")
'''
        
        with open(self.output_dir / "adapters" / "state_based_adapter.py", "w") as f:
            f.write(template)
    
    def _generate_custom_adapter_template(self):
        """Generate template for custom adapter implementation."""
        
        template = '''"""Custom Adapter Template - implement for your MCP"""

from typing import Dict, Any


class CustomAdapter:
    """Override this class to implement your MCP-specific logic."""
    
    def __init__(self):
        pass
    
    def execute(self, command: str, args: dict) -> Dict[str, Any]:
        """
        Execute command.
        
        Args:
            command: Tool name
            args: Command arguments (dict of all parsed args)
        
        Returns:
            Result dict with status and data
        """
        raise NotImplementedError("Implement execute() for your MCP")
'''
        
        with open(self.output_dir / "adapters" / "custom_adapter.py", "w") as f:
            f.write(template)
    
    def _generate_utils(self):
        """Generate utility modules."""
        
        error_handler = '''"""Error handling utilities"""

import sys
import json


class ErrorHandler:
    @staticmethod
    def handle(error: Exception):
        """Handle error and write to stderr."""
        
        error_map = {
            ValueError: 2,
            TimeoutError: 3,
            ConnectionError: 4,
            PermissionError: 5,
            RuntimeError: 1
        }
        
        error_type = type(error)
        exit_code = error_map.get(error_type, 1)
        
        error_obj = {
            "status": "error",
            "error": str(error),
            "type": error_type.__name__,
            "exit_code": exit_code
        }
        
        print(json.dumps(error_obj, indent=2), file=sys.stderr)
'''
        
        output_formatter = '''"""Output formatting utilities"""

import json
import base64


class OutputFormatter:
    def __init__(self, format: str = 'json'):
        self.format = format
    
    def format(self, data) -> str:
        """Format output based on format specification."""
        
        if self.format == 'json':
            return json.dumps(data, indent=2)
        
        elif self.format == 'text':
            if isinstance(data, dict) and 'data' in data:
                return str(data['data'])
            return str(data)
        
        elif self.format == 'base64':
            if isinstance(data, bytes):
                return base64.b64encode(data).decode('utf-8')
            return base64.b64encode(str(data).encode()).decode('utf-8')
        
        return json.dumps(data)
'''
        
        with open(self.output_dir / "utils" / "error_handler.py", "w") as f:
            f.write(error_handler)
        
        with open(self.output_dir / "utils" / "output_formatter.py", "w") as f:
            f.write(output_formatter)
        
        with open(self.output_dir / "utils" / "__init__.py", "w") as f:
            f.write("from .error_handler import ErrorHandler\nfrom .output_formatter import OutputFormatter\n")
    
    def _generate_requirements(self):
        """Generate requirements.txt."""
        
        reqs = ["# Generated CLI dependencies\n"]
        
        if self.pattern == "state-based":
            reqs.append("# State management\n")
        
        # Add MCP SDK if available
        if self.analysis.get("language") == "python":
            reqs.append("mcp>=0.1.0\n")
        
        with open(self.output_dir / "requirements.txt", "w") as f:
            f.writelines(reqs)
    
    def _generate_readme(self):
        """Generate README.md."""
        
        readme = f'''# {self.analysis.get('mcp_type', 'MCP')} CLI

Auto-generated CLI from MCP server.

## Installation

```bash
pip install -r requirements.txt
chmod +x cli.py
```

## Usage

This CLI was generated for the **{self.pattern}** pattern.

```bash
./cli.py <command> [options]
```

See generated skill documentation for full reference.

## Pattern: {self.pattern.upper()}

This MCP was converted using the **{self.pattern}** pattern.

### Tools

'''
        
        for tool in self.analysis.get("tools", []):
            readme += f"\n- `{tool['name']}`"
        
        readme += "\n"
        
        with open(self.output_dir / "README.md", "w") as f:
            f.write(readme)
    
    def _generate_tests(self):
        """Generate test template."""
        
        test_template = '''"""
Generated test template for CLI.
Add actual tests here.
"""

import unittest
import subprocess
import json


class TestGeneratedCLI(unittest.TestCase):
    """Test cases for generated CLI."""
    
    def test_cli_help(self):
        """Test that CLI shows help."""
        result = subprocess.run(["python", "cli.py", "--help"], capture_output=True)
        self.assertEqual(result.returncode, 0)
    
    def test_commands_exist(self):
        """Test that expected commands are registered."""
        # Add specific command tests here
        pass


if __name__ == "__main__":
    unittest.main()
'''
        
        with open(self.output_dir / "tests" / "test_cli.py", "w") as f:
            f.write(test_template)


class SkillGenerator:
    """Generates skill documentation from MCP analysis."""
    
    def __init__(self, analysis: Dict[str, Any], cli_dir: str):
        self.analysis = analysis
        self.skill_dir = Path(cli_dir) / "skill"
        self.skill_dir.mkdir(parents=True, exist_ok=True)
    
    def generate(self) -> bool:
        """Generate skill files."""
        
        try:
            self._generate_skill_md()
            self._generate_references()
            self._generate_setup_script()
            self._generate_error_codes()
            
            print(f"✓ Skill generated in {self.skill_dir}")
            return True
            
        except Exception as e:
            print(f"Error generating skill: {e}", file=sys.stderr)
            return False
    
    def _generate_skill_md(self):
        """Generate main SKILL.md."""
        
        mcp_type = self.analysis.get("mcp_type", "MCP")
        pattern = self.analysis.get("detected_pattern", "unknown")
        
        tools_section = "\n".join([
            f"- `{tool['name']}` - {tool.get('description', 'Tool call')}"
            for tool in self.analysis.get("tools", [])
        ])
        
        skill_md = f'''---
name: {mcp_type}-cli
description: CLI interface for {mcp_type}. Use this skill when Claude Code needs to execute {mcp_type} operations. Run commands with `{mcp_type}-cli <command> [options]` to interact with the {mcp_type} service.
license: Auto-generated
---

# {mcp_type} CLI Skill

Auto-generated skill for {mcp_type} CLI interface.

## Pattern: {pattern.upper()}

This CLI was generated using the **{pattern}** pattern.

## Available Commands

{tools_section}

## Usage

### Basic Command Format

```bash
{mcp_type}-cli <command> [options] --format [json|text|base64]
```

### Output Format

- `--format json`: Structured JSON response (default)
- `--format text`: Human-readable text
- `--format base64`: Binary output as base64

## Error Handling

Exit codes:
- `0`: Success
- `1`: General error
- `2`: Invalid arguments
- `3`: Timeout
- `4`: Resource unavailable
- `5`: Authentication failed

## Full Reference

See `references/cli_reference.md` for complete command reference.
See `references/usage_examples.md` for examples.
'''
        
        with open(self.skill_dir / "SKILL.md", "w") as f:
            f.write(skill_md)
    
    def _generate_references(self):
        """Generate reference documentation."""
        
        cli_ref = "# CLI Command Reference\n\n"
        
        for tool in self.analysis.get("tools", []):
            tool_name = tool["name"]
            inputs = tool.get("inputs", {})
            
            cli_ref += f"## {tool_name}\n\n"
            cli_ref += f"```bash\n{self.analysis.get('mcp_type')}-cli {tool_name}"
            
            for param in inputs.keys():
                cli_ref += f" --{param} <value>"
            
            cli_ref += "\n```\n\n"
        
        with open(self.skill_dir / "references" / "cli_reference.md", "w") as f:
            f.write(cli_ref)
        
        # Usage examples
        examples = f'''# Usage Examples

## Pattern: {self.analysis.get('detected_pattern')}

### Example 1: Basic call
```bash
{self.analysis.get('mcp_type')}-cli {self.analysis.get('tools', [{}])[0].get('name', 'command')} --param value
```

### Example 2: With output format
```bash
{self.analysis.get('mcp_type')}-cli {self.analysis.get('tools', [{}])[0].get('name', 'command')} --param value --format json
```
'''
        
        with open(self.skill_dir / "references" / "usage_examples.md", "w") as f:
            f.write(examples)
    
    def _generate_setup_script(self):
        """Generate setup script."""
        
        setup = f'''#!/bin/bash
# Setup script for {self.analysis.get('mcp_type')} CLI

set -e

echo "Installing {self.analysis.get('mcp_type')} CLI..."

# Install dependencies
pip install -r ../requirements.txt

# Make CLI executable
chmod +x ../cli.py

echo "✓ {self.analysis.get('mcp_type')} CLI installed"
echo "Use: {self.analysis.get('mcp_type')}-cli <command> [options]"
'''
        
        with open(self.skill_dir / "scripts" / "setup.sh", "w") as f:
            f.write(setup)
    
    def _generate_error_codes(self):
        """Generate error code reference."""
        
        error_codes = '''# Error Codes

| Code | Meaning | Recovery |
|------|---------|----------|
| 0 | Success | N/A |
| 1 | General error | Check stderr for details |
| 2 | Invalid arguments | Review command syntax |
| 3 | Timeout | Retry or increase timeout |
| 4 | Resource unavailable | Check connectivity |
| 5 | Authentication failed | Check credentials |

## Common Errors

### "Session not found"
Session has expired or was destroyed. Create a new session with `session create`.

### "Connection refused"
Service is not running or not accessible. Verify connectivity.

### "Invalid arguments"
Review command syntax with `--help` flag.
'''
        
        with open(self.skill_dir / "assets" / "error_codes.md", "w") as f:
            f.write(error_codes)


def main():
    parser = argparse.ArgumentParser(
        description="Convert MCP servers to CLI utilities with auto-generated skills"
    )
    
    subparsers = parser.add_subparsers(dest="action", help="Action to perform")
    
    # Analyze action
    analyze_parser = subparsers.add_parser("analyze", help="Analyze MCP server")
    analyze_parser.add_argument("source", help="Path/package spec for MCP server")
    analyze_parser.add_argument("--lang", default="python", choices=["python", "typescript", "javascript"])
    analyze_parser.add_argument("--output", help="Output analysis to JSON file")
    
    # Convert action
    convert_parser = subparsers.add_parser("convert", help="Convert MCP to CLI + skill")
    convert_parser.add_argument("name", help="Name for the converted CLI")
    convert_parser.add_argument("--source", required=True, help="Path/package spec for MCP server")
    convert_parser.add_argument("--pattern", choices=["api-wrapper", "state-based", "custom"], 
                                help="Force pattern (auto-detect if not specified)")
    convert_parser.add_argument("--lang", default="python")
    convert_parser.add_argument("--output-dir", default=".", help="Output directory")
    convert_parser.add_argument("--adapter-script", help="Custom adapter script for 'custom' pattern")
    
    args = parser.parse_args()
    
    if args.action == "analyze":
        analyzer = MCPAnalyzer(args.source, args.lang)
        analysis = analyzer.analyze()
        
        if args.output:
            with open(args.output, "w") as f:
                json.dump(analysis, f, indent=2)
            print(f"Analysis written to {args.output}")
        else:
            print(json.dumps(analysis, indent=2))
        
        return 0
    
    elif args.action == "convert":
        # Step 1: Analyze
        analyzer = MCPAnalyzer(args.source, args.lang)
        analysis = analyzer.analyze()
        
        if "error" in analysis:
            print(f"Error analyzing MCP: {analysis['error']}", file=sys.stderr)
            return 1
        
        # Step 2: Determine pattern
        pattern = args.pattern or analysis.get("detected_pattern", "api-wrapper")
        
        print(f"✓ Detected pattern: {pattern} (confidence: {analysis.get('confidence', 0):.0%})")
        
        # Step 3: Generate CLI
        cli_gen = CLIGenerator(analysis, args.output_dir, pattern)
        if not cli_gen.generate():
            return 1
        
        # Step 4: Generate Skill
        skill_gen = SkillGenerator(analysis, args.output_dir)
        if not skill_gen.generate():
            return 1
        
        print(f"\n✓ Conversion complete!")
        print(f"  CLI: {args.output_dir}/cli.py")
        print(f"  Skill: {args.output_dir}/skill/SKILL.md")
        
        return 0
    
    else:
        parser.print_help()
        return 1


if __name__ == "__main__":
    sys.exit(main())
