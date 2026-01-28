# Claude Code Terminal - Quick Reference Cheat Sheet

**Purpose:** Fast lookup for common commands, keyboard shortcuts, and configuration patterns.

---

## Installation & Setup

```bash
# Install Claude Code
npm install -g @anthropic-ai/claude-code

# Update to latest version
claude update

# Check installation health
/doctor

# Start interactive session
claude

# Quick query (non-interactive)
claude -p "Review this code"
```

---

## Memory & Rules Quick Reference (2025)

### Memory Commands
| Command | Description |
|---------|-------------|
| `/memory` | View loaded memory files |
| `/init` | Bootstrap CLAUDE.md for codebase |
| `#` prefix | Fast memory addition (start input with `#`) |

### Memory Hierarchy (highest to lowest precedence)
| Level | Location | Purpose |
|-------|----------|---------|
| Enterprise | System dirs | Organization-wide |
| Project | `./CLAUDE.md` | Team-shared |
| Rules | `./.claude/rules/*.md` | Modular rules |
| User | `~/.claude/CLAUDE.md` | Personal global |
| Local | `./CLAUDE.local.md` | Personal project |

### Rules Directory Structure
```
.claude/rules/
├── code-style.md     # Coding standards
├── testing.md        # Test conventions
├── security.md       # Security rules
└── frontend/
    └── react.md      # Framework-specific
```

### Conditional Rules (YAML frontmatter)
```yaml
---
paths: src/api/**/*.ts
---
# Rules apply only to matched files
```

### Import Syntax in CLAUDE.md
```markdown
@README            # Import from project root
@docs/setup.md     # Relative path
@~/.claude/my.md   # Home directory
```

---

## Essential Slash Commands (30+ Built-in)

### Session Management
| Command | Description |
|---------|-------------|
| `/help` | Show all available commands |
| `/clear` | Clear conversation history |
| `/compact [instructions]` | Compact conversation (95% threshold or manual) |
| `/rewind` | Restore previous code/conversation state (December 2025) |
| `/rename [name]` | Name current session for later reference (December 2025) |
| `/resume <n>` | Resume a named session (December 2025) |
| `/stats` | View usage statistics and streak (December 2025) |
| `/cost` | Show token usage and cost statistics |
| `/context` | Show context window usage |

### Configuration
| Command | Description |
|---------|-------------|
| `/config` | Open Settings interface |
| `/status` | Show version, model, account, connectivity |
| `/model <name>` | Change AI model (sonnet, opus, haiku) |
| `/permissions` | Manage tool permissions |
| `/output-style [style]` | Change output style (default, explanatory, learning) |

### Feature Management
| Command | Description |
|---------|-------------|
| `/agents` | Manage custom subagents |
| `/mcp` | Manage MCP server connections |
| `/hooks` | Configure event hooks |
| `/plugin` | Manage plugins and marketplaces |
| `/vim` | Enable Vim editor mode |
| `/bug` | Report issues directly within Claude Code |
| `/terminal-setup` | Configure terminal shortcuts (Shift+Enter) |
| `/memory` | Edit CLAUDE.md memory files |
| `/init` | Initialize project with CLAUDE.md |

### Development Tools
| Command | Description |
|---------|-------------|
| `/review` | Request code review |
| `/pr_comments` | View pull request comments |
| `/vim` | Enter vim mode for editing |
| `/terminal-setup` | Install Shift+Enter for newlines (iTerm2/VSCode) |

### Account Management
| Command | Description |
|---------|-------------|
| `/login` | Switch Anthropic accounts |
| `/logout` | Sign out |
| `/usage` | Show plan limits and rate limits |
| `/bug` | Report bugs to Anthropic |

---

## Keyboard Shortcuts

### General Controls
| Shortcut | Action | Platform |
|----------|--------|----------|
| `Ctrl+C` | Cancel current generation | All |
| `Ctrl+D` | Exit Claude Code | All |
| `Ctrl+L` | Clear terminal screen (preserve history) | All |
| `Ctrl+O` | Toggle verbose output | All |
| `Ctrl+R` | Reverse search command history | All |
| `Ctrl+B` | Run command in background | All |
| `Up/Down` | Navigate command history | All |

### Input Modes
| Shortcut | Action | Platform |
|----------|--------|----------|
| `\ + Enter` | Insert newline (always works) | All |
| `Shift+Enter` | Insert newline (after /terminal-setup) | iTerm2, VSCode |
| `Option+Enter` | Insert newline | macOS Terminal.app |
| `Ctrl+J` | Line feed (multiline input) | All |

### Special Functions
| Shortcut | Action | Platform |
|----------|--------|----------|
| `Tab` | Accept prompt suggestion (December 2025) | All |
| `Shift+Tab` or `Alt+M` | Cycle permission modes (Auto-Accept/Plan/Normal) | All |
| `Esc Esc` | Rewind conversation/code (checkpoint restore) | All |
| `Ctrl+V` (Mac/Linux) or `Alt+V` (Win) | Paste images | All |
| `Alt+P` | Quick model switching | Windows/Linux |
| `Option+P` | Quick model switching | macOS |
| `?` | Show available shortcuts for your terminal | All |

### Vim Mode (after `/vim`)
| Shortcut | Action |
|----------|--------|
| `Esc` | Enter NORMAL mode |
| `i` | Insert mode (before cursor) |
| `a` | Insert mode (after cursor) |
| `o` | Insert mode (new line below) |
| `h/j/k/l` | Navigate left/down/up/right |
| `w/e/b` | Word navigation |
| `0/$` | Line start/end |
| `gg/G` | File start/end |
| `x` | Delete character |
| `dd` | Delete line |
| `cc` | Change line |
| `.` | Repeat last action |

---

## Input Prefixes

| Prefix | Function | Example |
|--------|----------|---------|
| `/` | Execute slash command | `/review` |
| `#` | Add to CLAUDE.md memory | `#Use 2-space indents` |
| `!` | Direct bash execution (no Claude interpretation) | `!git status` |
| `@` | File path autocomplete/reference | `@src/app.ts` |

---

## CLI Flags (Most Useful)

### Session Control
```bash
claude                              # Interactive mode
claude -p "query"                   # Print mode (non-interactive)
claude -c                           # Continue last session
claude --resume <session-id>        # Resume specific session
claude --verbose                    # Enable verbose logging
```

### Model & Behavior
```bash
claude --model opus                 # Use Opus model
claude --model sonnet[1m]           # Extended context
claude --permission-mode plan       # Start in plan mode
claude --permission-mode acceptEdits # Auto-accept file edits
```

### Tool & Permission Control
```bash
claude --allowedTools "Bash,Read,Edit"          # Pre-authorize tools
claude --disallowedTools "WebFetch,WebSearch"   # Block tools
claude --dangerously-skip-permissions           # Skip all prompts (use with caution)
```

### Output & Input Formats
```bash
claude -p "query" --output-format json          # JSON output
claude -p "query" --output-format stream-json   # Streaming JSON
claude -p --input-format stream-json            # Accept JSON input
claude -p --include-partial-messages "query"    # Include streaming events
```

### Advanced Configuration
```bash
claude --add-dir ../lib ../apps                 # Additional working directories
claude --append-system-prompt "Custom rules"    # Add to system prompt (with -p)
claude --system-prompt-file prompt.txt          # Replace system prompt (with -p)
claude --max-turns 5                            # Limit agentic iterations
claude --mcp-config ./mcp.json                  # Load MCP servers from file
```

### Subagents (Dynamic Definition)
```bash
claude --agents '{
  "reviewer": {
    "description": "Code reviewer",
    "prompt": "You are a senior code reviewer...",
    "tools": ["Read", "Grep", "Glob"],
    "model": "sonnet"
  }
}'
```

### New CLI Flags (2025)
```bash
claude --fork-session                 # Create new session ID when resuming
claude --session-id <uuid>            # Use specific session ID
claude --fallback-model sonnet        # Auto-fallback when primary overloaded
claude --json-schema '{...}'          # Get validated JSON output
claude --strict-mcp-config            # Only use --mcp-config MCP servers
claude --debug "api,mcp"              # Debug mode with categories
claude --ide                          # Auto-connect to IDE
claude --settings ./settings.json     # Load settings file
```

---

## Model Aliases

| Alias | Full Name | Use Case | Cost |
|-------|-----------|----------|------|
| `default` | (account-dependent) | Recommended starting point | Varies |
| `sonnet` | claude-model: opus-4-5-20251001 | Daily coding tasks | Medium |
| `opus` | claude-opus-4-1-20250805 | Complex reasoning, architecture | High |
| `haiku` | claude-3-5-haiku-20241022 | Simple tasks, fast responses | Low |
| `sonnet[1m]` | (extended context) | Large codebase analysis | High |
| `opusplan` | Opus (plan) + Sonnet (execute) | Hybrid planning + execution | High |

**Switch models during session:** `/model <alias>`

---

## Permission Patterns

### Allow/Ask/Deny Rules

```json
{
  "permissions": {
    "allow": [
      "Read(*)",
      "Grep(*)",
      "Glob(*)",
      "Bash(git status:*)",
      "Bash(npm test:*)"
    ],
    "ask": [
      "Bash(git push:*)",
      "Write(*)"
    ],
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)",
      "Bash(rm -rf:*)",
      "Bash(curl:*)"
    ]
  }
}
```

### Permission Modes

| Mode | Behavior | Use Case |
|------|----------|----------|
| `default` | Ask for most operations | Safe exploration |
| `plan` | Read-only, propose changes | Review before action |
| `acceptEdits` | Auto-accept file edits | Active development |
| `bypassPermissions` | Skip all prompts | Automation (use cautiously) |

**Switch modes:** `Shift+Tab` or `/permissions`

---

## MCP Quick Commands

### Add Servers
```bash
# HTTP server (remote)
claude mcp add --transport http <name> <url>

# stdio server (local)
claude mcp add --transport stdio <name> -- <command>

# With environment variables
claude mcp add --transport stdio airtable \
  --env AIRTABLE_API_KEY=YOUR_KEY \
  -- npx -y airtable-mcp-server

# With scope (local/project/user)
claude mcp add --transport http stripe --scope project https://mcp.stripe.com
```

### Manage Servers
```bash
claude mcp list                  # List all configured servers
claude mcp get <server-name>     # Get server details
claude mcp remove <server-name>  # Remove a server
/mcp                             # Authenticate, view status (within Claude Code)
```

### Use MCP Tools
```
> Use @github:issue://123 to implement this feature
> /mcp__github__list_prs
> /mcp__jira__create_issue "Bug title" high
```

---

## Plugin Quick Commands

```bash
# Interactive management
/plugin

# Add marketplace
/plugin marketplace add owner/repo

# Install/manage plugins
/plugin install plugin-name@marketplace-name
/plugin enable plugin-name@marketplace-name
/plugin disable plugin-name@marketplace-name
/plugin uninstall plugin-name@marketplace-name
```

---

## Settings Hierarchy (Precedence Order)

1. **Enterprise Managed** (highest) - `/Library/Application Support/ClaudeCode/` (macOS), `/etc/claude-code/` (Linux), `C:\ProgramData\ClaudeCode\` (Windows)
2. **Command-line flags** - `--model`, `--allowedTools`, etc.
3. **Local project** - `.claude/settings.local.json` (git-ignored)
4. **Shared project** - `.claude/settings.json` (version controlled)
5. **User settings** (lowest) - `~/.claude/settings.json`

---

## File Locations Reference

### User-Level
| Type | Location | Scope |
|------|----------|-------|
| Settings | `~/.claude/settings.json` | All projects |
| Commands | `~/.claude/commands/` | All projects |
| Subagents | `~/.claude/agents/` | All projects |
| Skills | `~/.claude/skills/` | All projects |
| Output Styles | `~/.claude/output-styles/` | All projects |
| Memory | `~/.claude/CLAUDE.md` | All projects |

### Project-Level
| Type | Location | Scope | Git |
|------|----------|-------|-----|
| Settings (shared) | `.claude/settings.json` | Team | ✅ Commit |
| Settings (local) | `.claude/settings.local.json` | Personal | ❌ .gitignore |
| Commands | `.claude/commands/` | Team | ✅ Commit |
| Subagents | `.claude/agents/` | Team | ✅ Commit |
| Skills | `.claude/skills/` | Team | ✅ Commit |
| Memory | `CLAUDE.md` or `.claude/CLAUDE.md` | Team | ✅ Commit |
| MCP Config | `.mcp.json` | Team | ✅ Commit |

---

## Environment Variables (Most Useful)

### Authentication
| Variable | Purpose |
|----------|---------|
| `ANTHROPIC_API_KEY` | API authentication |
| `ANTHROPIC_AUTH_TOKEN` | Bearer token authorization |

### Model Selection
| Variable | Purpose |
|----------|---------|
| `ANTHROPIC_MODEL` | Default model |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | Sonnet alias mapping |
| `ANTHROPIC_DEFAULT_OPUS_MODEL` | Opus alias mapping |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL` | Haiku alias mapping |
| `CLAUDE_CODE_SUBAGENT_MODEL` | Subagent default model |

### Cloud Providers
| Variable | Purpose |
|----------|---------|
| `CLAUDE_CODE_USE_BEDROCK` | Enable AWS Bedrock |
| `CLAUDE_CODE_USE_VERTEX` | Enable Google Vertex AI |
| `ANTHROPIC_VERTEX_PROJECT_ID` | GCP project for Vertex |
| `CLOUD_ML_REGION` | Vertex AI region |

### Behavior & Performance
| Variable | Purpose |
|----------|---------|
| `DISABLE_TELEMETRY` | Disable analytics |
| `DISABLE_ERROR_REPORTING` | Disable Sentry |
| `MAX_THINKING_TOKENS` | Enable extended thinking |
| `BASH_DEFAULT_TIMEOUT_MS` | Bash command timeout |
| `MAX_MCP_OUTPUT_TOKENS` | MCP tool output limit (default: 25,000) |
| `MCP_TIMEOUT` | MCP server startup timeout (ms) |

### SlashCommand Tool
| Variable | Purpose |
|----------|---------|
| `SLASH_COMMAND_TOOL_CHAR_BUDGET` | Character budget limit (default: 15,000) |

---

## Common Configuration Patterns

### Basic settings.json
```json
{
  "model": "sonnet",
  "permissions": {
    "allow": ["Read(*)", "Grep(*)", "Glob(*)"],
    "ask": ["Edit(*)", "Write(*)"],
    "deny": ["Read(./.env)", "Bash(rm -rf:*)"]
  }
}
```

### Team-shared project settings
```json
{
  "model": "sonnet",
  "permissions": {
    "allow": ["*"],
    "deny": ["Bash(curl:*)", "Read(./.env)"]
  },
  "env": {
    "NODE_ENV": "development"
  },
  "enabledPlugins": {
    "team-tools@company": true
  },
  "extraKnownMarketplaces": {
    "company": {
      "source": "github",
      "repo": "company/claude-plugins"
    }
  }
}
```

### Enterprise managed policies
```json
{
  "permissions": {
    "allow": ["Read(*)", "Grep(*)", "Glob(*)"],
    "deny": ["WebFetch(*)", "WebSearch(*)"]
  },
  "allowedMcpServers": ["github", "jira", "sentry"],
  "deniedMcpServers": ["*"],
  "model": "sonnet"
}
```

---

## CRITICAL: Token Efficiency - Native Tools vs Bash

**ALWAYS use native tools for file operations** (40-73% token savings):

| Operation | ❌ Bash (Inefficient) | ✅ Native (Efficient) | Savings |
|-----------|---------------------|---------------------|---------|
| Read file | `cat file.py` | `Read(file_path="file.py")` | 40% |
| Search code | `grep -r "pattern"` | `Grep(pattern="pattern")` | 60% |
| Find files | `find . -name "*.ts"` | `Glob(pattern="**/*.ts")` | 73% |
| Edit file | `sed -i 's/old/new/'` | `Edit(old_string="old", new_string="new")` | 75% |
| Create file | `echo "content" > file` | `Write(file_path="file", content="content")` | 77% |

**Use Bash ONLY for:**
- Git operations: `git status`, `git commit`, `git push`
- Package managers: `npm install`, `pip install`, `cargo build`
- Test runners: `pytest`, `npm test`, `cargo test`
- Build tools: `make`, `cmake`, `gradle build`
- Containers: `docker build`, `kubectl apply`

**Official Claude Code system prompt mandates native tools for file operations.**

---

## Subagent Configuration Template

```markdown
---
name: my-subagent
description: Brief description of when to use this subagent. Include keywords and use cases.
tools: Read, Grep, Glob, Bash  # Optional - omit to inherit all
model: opus  # Optional - sonnet, opus, haiku, or inherit
---

# Subagent System Prompt

You are a specialized [role] expert in [domain].

When invoked:
1. [Step 1]
2. [Step 2]
3. [Step 3]

Key responsibilities:
- [Responsibility 1]
- [Responsibility 2]

Best practices:
- [Practice 1]
- [Practice 2]
```

**Save as:** `.claude/agents/my-subagent.md` (project) or `~/.claude/agents/my-subagent.md` (user)

---

## Skill Configuration Template

```yaml
---
name: my-skill-name
description: What this skill does and when Claude should use it. Include specific keywords and scenarios.
allowed-tools: Read, Grep, Glob  # Optional - restrict tool access
---

# My Skill Name

## Purpose
[Brief description]

## Instructions
1. [Step 1]
2. [Step 2]
3. [Step 3]

## Examples
[Concrete usage examples]

## Best Practices
[Domain-specific guidance]
```

**Save as:** `.claude/skills/my-skill/SKILL.md` (project) or `~/.claude/skills/my-skill/SKILL.md` (user)

---

## Slash Command Template

```markdown
---
description: Brief description for /help output
argument-hint: [arg1] [arg2]
model: opus
allowed-tools: Bash(git:*), Read, Edit
---

# Command Title

[Instructions for Claude to follow]

Use arguments: $1, $2, or $ARGUMENTS

Reference files: @path/to/file

Execute bash: !`git status`
```

**Save as:** `.claude/commands/my-command.md` (project) or `~/.claude/commands/my-command.md` (user)

---

## Hook Configuration Template

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "script.sh",
            "timeout": 60
          }
        ]
      }
    ]
  }
}
```

**Add to:** `~/.claude/settings.json` or `.claude/settings.json`

---

## Common Workflows

### Code Review Workflow
```bash
# 1. Start session
claude

# 2. Review changes
> Review my recent changes for security issues

# 3. Or use slash command
> /review

# 4. Or create custom subagent
/agents
# Create "code-reviewer" with Read, Grep, Glob tools
```

### Feature Implementation
```bash
# 1. Start in plan mode
claude --permission-mode plan

# 2. Plan the feature
> I need to add user authentication. Let's plan this step-by-step.

# 3. Switch to execution mode
Shift+Tab  # Switch to acceptEdits mode

# 4. Implement with checkpoints
> Implement the authentication module
Esc Esc  # Rewind if needed
```

### CI/CD Automation
```yaml
# GitHub Actions workflow
name: Claude Code Review
on:
  pull_request:
    types: [opened, synchronize]
jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "/review"
```

### Bug Fixing with MCP
```bash
# 1. Add Sentry MCP
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp

# 2. Authenticate
/mcp

# 3. Debug production errors
> What are the most common errors in the last 24 hours?
> Show me the stack trace for error ID abc123
> Fix the issue causing error abc123
```

---

## Troubleshooting Quick Lookup

| Problem | Solution |
|---------|----------|
| Repeated permission prompts | `/permissions` → Configure tool allowlist |
| Authentication fails | `/logout` → Restart → Login again |
| Search tool slow | Install system ripgrep, set `USE_BUILTIN_RIPGREP=0` |
| WSL node not found | `npm config set os linux`, install Node via apt/nvm |
| Commands not appearing | Restart Claude Code after creating command files |
| Subagent not invoked | Check description specificity, verify file location |
| Skill not activating | Verify YAML syntax, check description keywords |
| MCP server won't connect | Check URL, verify authentication, check `/mcp` status |
| Hook not executing | Verify matcher pattern, check script permissions |
| Out of tokens | `/compact` or `/clear`, use lighter model |

---

## Diagnostic Commands

```bash
# Health check
/doctor

# Verbose mode (see all details)
claude --verbose

# Debug mode (see loading errors, hook execution)
claude --debug

# Check status
/status

# View usage
/usage  # Subscription plans only
/cost   # Show token usage and costs

# Context window usage
/context
```

---

## Best Practices Checklist

### General
- [ ] Use native tools (Read, Edit, Write, Glob, Grep) for file operations
- [ ] Reserve Bash for git, npm, pytest, docker only
- [ ] Start with plan mode for complex features
- [ ] Use checkpoints before major refactorings
- [ ] Configure permissions to avoid repetitive prompts

### Security
- [ ] Use deny rules for sensitive files (`.env`, `.git/`)
- [ ] Review all slash commands and hooks before trusting
- [ ] Never commit API keys to settings.json
- [ ] Use environment variables for secrets
- [ ] Enable sandboxing for additional isolation

### Performance
- [ ] Use Haiku for simple tasks
- [ ] Use Sonnet for daily development
- [ ] Reserve Opus for complex reasoning
- [ ] Use `/compact` when approaching context limit
- [ ] Configure appropriate timeouts for long operations

### Team Collaboration
- [ ] Commit project settings, commands, subagents, skills to git
- [ ] Use `.gitignore` for `.claude/settings.local.json`
- [ ] Document custom commands and subagents in README
- [ ] Use plugins for distribution
- [ ] Configure team marketplaces in settings.json

---

## Resource Links

### Official Documentation
- Main docs: https://code.claude.com/docs/
- API docs: https://docs.anthropic.com/
- MCP spec: https://modelcontextprotocol.io/

### Tools & Extensions
- VS Code extension: Search "Claude Code" in marketplace
- MCP servers: https://github.com/modelcontextprotocol/servers
- Example workflows: https://github.com/anthropics/claude-code-action

### Support
- Report bugs: `/bug` command or https://github.com/anthropics/claude-code/issues
- Security vulnerabilities: HackerOne program
- Account issues: Claude.ai support

---

**Quick Tip:** Use `/help` to discover all available commands in your current setup!

---

**Document Version:** 2.0 (2025-12-20)
**Claude Code Version:** 2.0.74
