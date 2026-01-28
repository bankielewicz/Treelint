# Claude Code Terminal - Troubleshooting Guide

**Source:** Official docs from code.claude.com (updated 2025-12-09)

**Purpose:** Comprehensive troubleshooting guide covering installation, authentication, performance, integrations, and common issues.

---

## Table of Contents

1. [Installation Issues](#installation-issues)
2. [Authentication & Permissions](#authentication--permissions)
3. [Performance & Stability](#performance--stability)
4. [IDE Integration](#ide-integration)
5. [Tool & Feature Issues](#tool--feature-issues)
6. [MCP Server Issues](#mcp-server-issues)
7. [Hooks & Subagents](#hooks--subagents)
8. [Configuration Issues](#configuration-issues)
9. [Diagnostic Commands](#diagnostic-commands)
10. [Quick Troubleshooting Matrix](#quick-troubleshooting-matrix)

---

## Installation Issues

### Windows WSL Problems

#### Problem: "Node not found" on Windows WSL

**Symptoms:**
- `claude` command not found after installation
- `node -v` shows different version in WSL vs Windows
- NPM global packages not accessible

**Root Cause:**
Running `npm install -g @anthropic-ai/claude-code` in Windows installs to Windows PATH, but WSL uses a separate PATH.

**Solution 1: Install natively in WSL (Recommended)**

```bash
# Inside WSL terminal
# 1. Install Node.js in WSL (if not present)
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# 2. Install Claude Code in WSL
npm install -g @anthropic-ai/claude-code

# 3. Verify installation
claude --version
```

**Solution 2: Use NVM for both environments**

```bash
# Inside WSL
# 1. Install NVM in WSL
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc

# 2. Install Node.js
nvm install 20
nvm use 20

# 3. Install Claude Code
npm install -g @anthropic-ai/claude-code

# 4. Verify
which claude  # Should show path in ~/.nvm/
```

**Solution 3: Add Windows Node to WSL PATH (Not Recommended)**

```bash
# Add to ~/.bashrc (use absolute Windows path)
export PATH="$PATH:/mnt/c/Program Files/nodejs"

# Reload
source ~/.bashrc
```

**Verification:**

```bash
# Check Node location
which node

# Check NPM global prefix
npm config get prefix

# Check Claude installation
which claude
claude --version
```

---

#### Problem: NVM conflicts between Windows and WSL

**Symptoms:**
- Different Node versions in Windows vs WSL
- `claude` works in one environment but not the other
- PATH conflicts

**Solution:**

```bash
# Option 1: Use separate NVM installations
# Windows: Use nvm-windows (https://github.com/coreybutler/nvm-windows)
# WSL: Use nvm (https://github.com/nvm-sh/nvm)

# Option 2: Standardize on WSL
# Uninstall Node from Windows
# Use only WSL for Claude Code

# Option 3: Use specific versions
# In WSL
nvm install 20
nvm alias default 20
nvm use default

# Verify
node -v  # Should show v20.x.x
npm -v
claude --version
```

---

#### Problem: Permission denied installing globally on Linux/macOS

**Symptoms:**
```bash
npm ERR! code EACCES
npm ERR! syscall access
npm ERR! path /usr/local/lib/node_modules
```

**Solution 1: Use NVM (Recommended)**

```bash
# Install NVM
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc  # or ~/.zshrc

# Install Node
nvm install 20
nvm use 20

# Install Claude Code (no sudo needed)
npm install -g @anthropic-ai/claude-code
```

**Solution 2: Change NPM prefix**

```bash
# Create local directory for global packages
mkdir ~/.npm-global

# Configure NPM
npm config set prefix '~/.npm-global'

# Add to PATH in ~/.bashrc or ~/.zshrc
export PATH=~/.npm-global/bin:$PATH

# Reload
source ~/.bashrc

# Install (no sudo)
npm install -g @anthropic-ai/claude-code
```

**Solution 3: Fix permissions (Not Recommended)**

```bash
sudo chown -R $(whoami) $(npm config get prefix)/{lib/node_modules,bin,share}
npm install -g @anthropic-ai/claude-code
```

---

#### Problem: Outdated version won't update

**Symptoms:**
- `claude update` fails
- `npm update -g @anthropic-ai/claude-code` does nothing
- Version number doesn't change

**Solution:**

```bash
# Uninstall completely
npm uninstall -g @anthropic-ai/claude-code

# Clear npm cache
npm cache clean --force

# Reinstall
npm install -g @anthropic-ai/claude-code

# Verify
claude --version
```

**For NVM users:**

```bash
# Check current Node version
nvm current

# Update Node if needed
nvm install 20
nvm use 20

# Reinstall Claude Code
npm uninstall -g @anthropic-ai/claude-code
npm install -g @anthropic-ai/claude-code
```

---

#### Problem: Installation succeeds but `claude` command not found

**Symptoms:**
- `npm install -g` completes successfully
- `claude` command not found
- `which claude` returns nothing

**Solution:**

```bash
# Find NPM global bin directory
npm config get prefix

# Check if bin directory is in PATH
echo $PATH

# Add NPM bin to PATH if missing (add to ~/.bashrc or ~/.zshrc)
export PATH="$(npm config get prefix)/bin:$PATH"

# Reload shell
source ~/.bashrc  # or source ~/.zshrc

# Verify
which claude
claude --version
```

**For macOS with Homebrew Node:**

```bash
# Check Homebrew Node installation
brew list node

# Reinstall if needed
brew reinstall node

# Install Claude Code
npm install -g @anthropic-ai/claude-code

# Verify
claude --version
```

---

### Migration Issues

#### Problem: Settings lost after reinstall

**Symptoms:**
- Previous configuration not loaded
- API keys missing
- Custom commands gone

**Solution:**

```bash
# Settings should persist at:
# ~/.claude/settings.json (user-level)
# <project>/.claude/settings.json (project-level)

# Verify settings files exist
ls -la ~/.claude/
ls -la .claude/

# If missing, restore from backup or recreate

# Test configuration
claude --verbose
# Check "Loaded settings from:" output
```

---

## Authentication & Permissions

### Repeated Permission Prompts

#### Problem: Claude repeatedly asks for permission even when allowed

**Symptoms:**
- Same tool/command asked multiple times
- Permissions not persisting across turns
- "Allow always" not working

**Cause:**
Pattern in `settings.json` doesn't match exact tool usage.

**Solution:**

```json
// Instead of specific patterns
{
  "permissions": {
    "allow": [
      "Bash(git status)",
      "Bash(git diff main)"
    ]
  }
}

// Use wildcards to allow all variants
{
  "permissions": {
    "allow": [
      "Bash(git *)"  // Allows all git commands
    ]
  }
}
```

**Common patterns:**

```json
{
  "permissions": {
    "allow": [
      "Read(*)",                    // All file reads
      "Write(src/**/*)",            // All writes in src/
      "Edit(*.{ts,tsx,js,jsx})",    // Edit JS/TS files
      "Bash(npm *)",                // All npm commands
      "Bash(git *)",                // All git commands
      "Bash(node *)"                // All node commands
    ],
    "deny": [
      "Bash(rm -rf *)",             // Prevent dangerous deletes
      "Bash(sudo *)",               // Prevent sudo
      "Read(.env*)"                 // Protect secrets
    ]
  }
}
```

**Debug permissions:**

```bash
# Start with verbose logging
claude --verbose

# Check loaded permissions
/status

# Test permission
# Claude will show permission check in verbose output
```

---

#### Problem: Permission mode not persisting

**Symptoms:**
- Permission mode resets to default each session
- `--permission-mode` flag doesn't stick
- Mode from settings.json ignored

**Solution:**

```json
// In settings.json (project or user-level)
{
  "permissions": {
    "defaultMode": "acceptEdits"  // or "readOnly", "acceptAll"
  }
}
```

**Start session with explicit mode:**

```bash
claude --permission-mode acceptEdits
```

**Check effective mode:**

```bash
claude --verbose
/status
# Look for "Permission mode: [mode]"
```

---

#### Problem: "Bypassing permissions disabled" error

**Symptoms:**
```
Error: Cannot use --dangerously-skip-permissions
Bypassing permissions is disabled
```

**Cause:**
Enterprise policy or settings.json disables bypass.

**Solution:**

Check `settings.json`:
```json
{
  "permissions": {
    "disableBypassPermissionsMode": "disable"  // Remove or change to "allow"
  }
}
```

**For enterprise environments:**
Contact your administrator - this may be intentionally restricted.

---

#### Problem: API key not recognized

**Symptoms:**
```
Error: Invalid API key
Authentication failed
```

**Solution 1: Set via environment variable**

```bash
# Add to ~/.bashrc or ~/.zshrc
export ANTHROPIC_API_KEY="sk-ant-api03-..."

# Reload
source ~/.bashrc

# Verify
echo $ANTHROPIC_API_KEY

# Test
claude "test query"
```

**Solution 2: Set in settings.json**

```json
// ~/.claude/settings.json or .claude/settings.local.json
{
  "env": {
    "ANTHROPIC_API_KEY": "sk-ant-api03-..."
  }
}
```

**Solution 3: Use third-party provider**

```json
{
  "providers": {
    "anthropic": {
      "type": "aws-bedrock",
      "region": "us-east-1"
    }
  }
}
```

**Verify API key:**

```bash
# Check environment
env | grep ANTHROPIC

# Test authentication
claude -p "test" --verbose
# Should not show authentication errors
```

---

### Permissions Configuration Issues

#### Problem: File-specific permissions not working

**Symptoms:**
- Specific file patterns don't match
- Glob patterns not working as expected
- Files outside allowed paths blocked

**Solution:**

```json
{
  "permissions": {
    "allow": [
      // Exact file
      "Read(package.json)",

      // Single directory
      "Read(src/*)",

      // Recursive (any depth)
      "Read(src/**/*)",

      // Multiple extensions
      "Edit(*.{ts,tsx,js,jsx})",

      // Exclude pattern
      "Read(*)",
      "deny": ["Read(*.test.ts)"]  // Except test files
    ],
    "additionalDirectories": [
      "../shared-lib",  // Access parent directory
      "/absolute/path"  // Absolute path
    ]
  }
}
```

**Test patterns:**

```bash
# Start Claude with verbose logging
claude --verbose

# Try to read a file
# Verbose output shows permission check:
# "Checking permission: Read(src/app.ts)"
# "Pattern 'Read(src/***)' matches: true"
```

**Pattern matching rules:**
- `*` matches any characters except `/`
- `**` matches any characters including `/`
- Bash patterns use prefix matching: `Bash(git *)` matches `git status`, `git commit`, etc.
- Tool patterns are exact: `Read(src/*)` does NOT match `src/subdir/file.ts` (use `src/**/*`)

---

## Performance & Stability

### High Memory Usage

#### Problem: Claude Code consuming excessive memory

**Symptoms:**
- RAM usage >2GB
- System slowdown during sessions
- Out of memory errors

**Solution 1: Limit conversation length**

```bash
# Clear conversation periodically
/clear

# Start new session instead of continuing very long ones
# Avoid /continue for sessions with >100 turns
```

**Solution 2: Adjust Node.js memory**

```bash
# Increase Node.js heap (if hitting limits)
export NODE_OPTIONS="--max-old-space-size=4096"

# Add to ~/.bashrc for persistence
echo 'export NODE_OPTIONS="--max-old-space-size=4096"' >> ~/.bashrc
```

**Solution 3: Use checkpoints strategically**

```bash
# Clear after checkpointing important state
/rewind save "Before major refactor"
/clear

# Resume from checkpoint if needed
/rewind
```

**Monitor memory:**

```bash
# macOS
top -pid $(pgrep -f "node.*claude")

# Linux
ps aux | grep claude
```

---

### Slow Response Times

#### Problem: Claude Code taking very long to respond

**Symptoms:**
- Responses take >60 seconds
- "Thinking..." spinner indefinitely
- Timeouts

**Possible Causes & Solutions:**

**1. Large file operations:**

```bash
# Avoid reading entire large files
# Instead of:
Read(logs/debug.log)  # 50MB file

# Use:
Bash(tail -n 1000 logs/debug.log)  # Last 1000 lines only
Grep(pattern="ERROR", path="logs/debug.log")  # Search specific pattern
```

**2. Network latency:**

```bash
# Test network connection
ping api.anthropic.com

# Check API status
curl -I https://api.anthropic.com/v1/messages

# Use verbose mode to see request/response timing
claude --verbose
```

**3. Too many tool calls per turn:**

```bash
# Limit scope in prompts
# Instead of: "Review all files in src/"
# Use: "Review src/auth/login.ts"

# Set max turns limit
claude --max-turns 10
```

**4. MCP server timeouts:**

```json
// Add timeout to slow MCP servers
{
  "mcpServers": {
    "slow-server": {
      "command": "...",
      "timeout": 30000  // 30 seconds
    }
  }
}
```

---

### Unresponsive Commands

#### Problem: Interactive commands don't respond

**Symptoms:**
- Slash commands ignored
- No output after input
- Terminal appears frozen

**Solution 1: Check if process is running**

```bash
# Press Ctrl+C to cancel current operation

# Verify Claude is running
ps aux | grep claude

# Kill if frozen
pkill -f "node.*claude"

# Restart
claude
```

**Solution 2: Reset terminal state**

```bash
# If terminal is corrupted
reset

# Or exit and restart
exit
claude
```

**Solution 3: Clear broken session**

```bash
# Start fresh session
claude
/clear

# Or start completely new
rm ~/.claude/.lastSession
claude
```

---

### WSL2 Performance Issues

#### Problem: Slow file operations in WSL2

**Symptoms:**
- File reads/writes take several seconds
- Grep operations slow
- Cross-platform path issues

**Solution 1: Work in WSL filesystem**

```bash
# BAD: Working in Windows filesystem from WSL
cd /mnt/c/Users/username/project  # Slow

# GOOD: Working in WSL filesystem
cd ~/projects/project  # Fast

# Move project to WSL
cp -r /mnt/c/Users/username/project ~/projects/
cd ~/projects/project
```

**Solution 2: Use WSL2 optimizations**

```bash
# In Windows PowerShell (as Administrator)
# Increase WSL2 memory
# Create/edit: C:\Users\<username>\.wslconfig

[wsl2]
memory=4GB
processors=4
```

**Solution 3: Disable Windows Defender for WSL paths**

```powershell
# Windows PowerShell (as Administrator)
Add-MpPreference -ExclusionPath "\\wsl$\Ubuntu\home\<username>\projects"
```

---

### Search Tool Performance

#### Problem: Grep/Glob operations slow on large codebases

**Symptoms:**
- `Grep` takes >30 seconds
- `Glob` hangs
- "Searching..." never completes

**Solution 1: Use focused searches**

```bash
# Instead of searching entire codebase
Grep(pattern="function", path=".")  # Slow on large repos

# Search specific directories
Grep(pattern="function", path="src/auth")  # Much faster

# Use file type filters
Grep(pattern="TODO", type="ts")  # Only TypeScript files
```

**Solution 2: Optimize gitignore**

```bash
# Add large directories to .gitignore
node_modules/
dist/
build/
coverage/
.next/
.cache/

# Claude respects .gitignore by default
```

**Solution 3: Exclude patterns**

```json
// In settings.json
{
  "permissions": {
    "deny": [
      "Read(node_modules/**)",
      "Read(dist/**)",
      "Read(*.log)",
      "Grep(node_modules/**)"
    ]
  }
}
```

**Solution 4: Use ripgrep directly for very large searches**

```bash
# For one-off large searches
Bash(rg "pattern" --type ts)  # Faster than Grep on huge repos
```

---

## IDE Integration

### JetBrains IDEs on WSL2

#### Problem: Cannot open JetBrains IDE from Claude Code in WSL2

**Symptoms:**
- `Open in IDE` fails
- "Cannot find IDE" error
- IDE opens in Windows, not WSL project

**Solution 1: Use JetBrains Gateway**

```bash
# Install JetBrains Gateway on Windows
# Connect to WSL2 from Gateway
# Open project in remote IDE

# Then from Claude in WSL:
# IDE opening will work through Gateway
```

**Solution 2: Use WSL native toolbox**

```bash
# Install JetBrains Toolbox in WSL
cd ~
wget https://download.jetbrains.com/toolbox/jetbrains-toolbox-*.tar.gz
tar -xzf jetbrains-toolbox-*.tar.gz
./jetbrains-toolbox

# Install IDE through Toolbox
# Set up PATH
export PATH="$HOME/.local/share/JetBrains/Toolbox/scripts:$PATH"

# Add to ~/.bashrc
echo 'export PATH="$HOME/.local/share/JetBrains/Toolbox/scripts:$PATH"' >> ~/.bashrc
```

**Solution 3: Configure IDE path manually**

```json
// In settings.json
{
  "ide": {
    "command": "/mnt/c/Program Files/JetBrains/IntelliJ IDEA/bin/idea64.exe",
    "args": ["$FILE_PATH"]
  }
}
```

---

### VS Code Integration Issues

#### Problem: VS Code extension not appearing

**Symptoms:**
- Extension installed but not in sidebar
- "Claude" icon missing
- Extension shows as disabled

**Solution:**

```bash
# 1. Verify extension installed
code --list-extensions | grep anthropic

# 2. If not found, install
code --install-extension anthropic.claude-code

# 3. Reload VS Code
# Ctrl+Shift+P (or Cmd+Shift+P) → "Reload Window"

# 4. Check for conflicts
# Disable other AI extensions temporarily
```

**Check extension logs:**

```
View → Output → Select "Claude Code" from dropdown
```

---

#### Problem: ESC key conflicts

**Symptoms:**
- ESC closes Claude input unexpectedly
- Cannot exit insert mode properly
- Vim keybindings interfere

**Solution 1: Remap ESC in VS Code**

```json
// settings.json
{
  "keyboard.dispatch": "keyCode",
  "vim.handleKeys": {
    "<Esc>": false
  }
}
```

**Solution 2: Use alternative exit keys**

```bash
# In Claude input
Ctrl+C  # Alternative to ESC
Ctrl+D  # Submit instead of Ctrl+Enter
```

**Solution 3: Disable Vim plugin temporarily**

```bash
# VS Code Command Palette
"Extensions: Disable (Workspace)" → Vim
```

---

### Terminal Integration Problems

#### Problem: Terminal multiplexer (tmux/screen) issues

**Symptoms:**
- Colors broken
- Input lag
- Rendering artifacts

**Solution:**

```bash
# For tmux - add to ~/.tmux.conf
set -g default-terminal "screen-256color"
set -ga terminal-overrides ",xterm-256color:Tc"

# Reload tmux config
tmux source-file ~/.tmux.conf

# For screen - add to ~/.screenrc
term screen-256color

# Test colors
claude
# Should see proper syntax highlighting
```

---

## Tool & Feature Issues

### File Operations

#### Problem: "File not found" despite file existing

**Symptoms:**
- `Read(file.ts)` fails
- File clearly exists with `ls`
- Absolute paths don't work

**Solution:**

```bash
# Use relative paths from working directory
Read(./src/app.ts)       # Explicit relative
Read(src/app.ts)         # Implicit relative

# Check current working directory
pwd

# Verify file exists
ls -la src/app.ts

# Use Glob to verify path
Glob(pattern="src/app.ts")
```

---

#### Problem: Edit tool changes not persisting

**Symptoms:**
- `Edit` completes successfully
- File content unchanged
- No error message

**Possible Causes & Solutions:**

**1. File opened in editor:**

```bash
# Close file in editor
# Or use Edit with force flag (if supported)
```

**2. Permissions:**

```bash
# Check file permissions
ls -l src/app.ts

# Fix if read-only
chmod u+w src/app.ts
```

**3. Git conflicts:**

```bash
# Check for merge conflicts
git status

# Resolve conflicts first
git mergetool
```

**4. File watcher interference:**

```bash
# Temporarily stop file watchers
# npm, webpack, nodemon, etc.
pkill -f "nodemon"

# Make edit
Edit(...)

# Restart watcher
```

---

#### Problem: Write tool creates file with wrong permissions

**Symptoms:**
- New files not executable when they should be
- Wrong owner/group
- Cannot edit created files

**Solution:**

```bash
# After Write, fix permissions
Write(script.sh, content="#!/bin/bash\necho 'test'")
Bash(chmod +x script.sh)

# Or use Bash to create with correct permissions
Bash(cat > script.sh << 'EOF'
#!/bin/bash
echo 'test'
EOF
chmod +x script.sh)
```

---

### Bash Tool Issues

#### Problem: Bash commands fail silently

**Symptoms:**
- Command appears to run
- No output
- No error
- Expected side effects don't happen

**Solution:**

```bash
# Use verbose mode to see full output
claude --verbose

# Check exit codes explicitly
Bash(command || echo "Failed with code $?")

# Redirect stderr to stdout
Bash(command 2>&1)

# Use set -e for strict error handling
Bash(set -e; command1; command2)
```

---

#### Problem: PATH not set correctly for Bash commands

**Symptoms:**
- `command not found` for installed tools
- Tools work in shell but not in Claude Bash()

**Solution:**

```json
// In settings.json
{
  "env": {
    "PATH": "${PATH}:/custom/bin:/another/path"
  }
}
```

**Or export in Bash commands:**

```bash
Bash(export PATH=$PATH:/custom/bin && mycommand)
```

---

### Checkpointing Issues

#### Problem: Checkpoint restore fails

**Symptoms:**
```
Error: Cannot restore checkpoint
Checkpoint corrupted
```

**Solution 1: List available checkpoints**

```bash
claude
/rewind
# Shows list of valid checkpoints
# Choose working checkpoint
```

**Solution 2: Clear corrupted checkpoints**

```bash
# Checkpoint data stored in:
# ~/.claude/checkpoints/<session-id>/

# Remove corrupted session
rm -rf ~/.claude/checkpoints/<session-id>/

# Start fresh
claude
```

**Solution 3: Use manual Git checkpoint instead**

```bash
# Instead of Claude checkpoints
git add -A
git commit -m "Checkpoint: before refactor"
git tag checkpoint-1

# Restore manually if needed
git reset --hard checkpoint-1
```

---

#### Problem: Checkpoints consuming too much disk space

**Symptoms:**
- `~/.claude/checkpoints/` very large
- Disk space warnings
- Old sessions never cleaned

**Solution:**

```bash
# Check checkpoint disk usage
du -sh ~/.claude/checkpoints/

# Clean old sessions (older than 30 days)
find ~/.claude/checkpoints/ -type d -mtime +30 -exec rm -rf {} +

# Or clean all
rm -rf ~/.claude/checkpoints/*

# Configure retention (if supported)
# settings.json
{
  "checkpoints": {
    "maxAge": 7,  // days
    "maxCount": 10
  }
}
```

---

### Model Selection Issues

#### Problem: Model flag ignored

**Symptoms:**
- `--model opus` doesn't use Opus
- `--model sonnet[1m]` fails
- Default model always used

**Solution:**

```bash
# Use correct model names
claude --model opus           # NOT "opus-3"
claude --model sonnet         # Latest Sonnet
claude --model haiku          # Latest Haiku
claude --model sonnet[1m]     # Extended context

# Check model in session
/status
# Should show: "Model: claude-opus-..."

# Change model mid-session
/model opus
```

**Verify model configuration:**

```json
// settings.json
{
  "model": "sonnet"  // Default model
}
```

---

#### Problem: Extended context model not working

**Symptoms:**
- `sonnet[1m]` not recognized
- Still using standard context
- Truncation happening too early

**Solution:**

```bash
# Ensure correct syntax
claude --model sonnet[1m]

# Or in settings.json
{
  "model": "sonnet[1m]"
}

# Verify in session
/status
# Should show: "Context window: 1000000 tokens"

# Check if available in your plan
# Extended context requires specific plan tier
```

---

## MCP Server Issues

### MCP Server Won't Start

#### Problem: MCP server fails to initialize

**Symptoms:**
```
Error: MCP server <name> failed to start
Connection timeout
Server process exited
```

**Solution 1: Check configuration**

```json
// settings.json
{
  "mcpServers": {
    "github": {
      "command": "npx",  // Verify command exists
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}"  // Verify env var set
      }
    }
  }
}
```

**Solution 2: Test command manually**

```bash
# Run MCP server command directly
npx -y @modelcontextprotocol/server-github

# Check for errors
# Verify dependencies installed
```

**Solution 3: Check environment variables**

```bash
# Verify token is set
echo $GITHUB_TOKEN

# Set if missing
export GITHUB_TOKEN="ghp_..."

# Add to ~/.bashrc for persistence
echo 'export GITHUB_TOKEN="ghp_..."' >> ~/.bashrc
```

**Solution 4: Enable debug logging**

```bash
# Start with verbose mode
claude --verbose

# MCP connection logs will show details
# Look for:
# "Starting MCP server: github"
# "MCP server github: connected"
```

---

### MCP Tools Not Available

#### Problem: MCP server running but tools not accessible

**Symptoms:**
- Server shows as "connected"
- Claude doesn't see tools
- `/mcp` shows server but no tools listed

**Solution 1: Check tool permissions**

```json
{
  "mcpServers": {
    "github": { ... }
  },
  "permissions": {
    "allow": [
      "mcp_github_*"  // Allow all tools from github server
    ]
  }
}
```

**Solution 2: Restart session**

```bash
# MCP tools loaded at session start
# Restart to load new configuration
exit
claude

# Verify tools loaded
/mcp
# Should show tools from server
```

**Solution 3: Check server capabilities**

```bash
# In Claude session with --verbose
# Look for MCP capability negotiation logs
# Verify server advertises tools
```

---

### MCP Connection Timeout

#### Problem: MCP server connects intermittently

**Symptoms:**
- Sometimes works, sometimes doesn't
- "Connection timeout" errors
- Server appears slow

**Solution:**

```json
{
  "mcpServers": {
    "slow-server": {
      "command": "...",
      "timeout": 60000,  // Increase timeout (milliseconds)
      "retries": 3       // Retry failed connections
    }
  }
}
```

---

## Hooks & Subagents

### Hooks Not Executing

#### Problem: PostToolUse hook doesn't run

**Symptoms:**
- Edit completes but no auto-format
- Hook configured correctly
- No errors shown

**Solution 1: Check matcher pattern**

```json
{
  "hooks": {
    "PostToolUse": [
      {
        // Wrong - too specific
        "matcher": "Edit(src/app.ts)",

        // Right - use wildcards
        "matcher": "Edit(*.ts)",

        "hooks": [
          {
            "type": "command",
            "command": "npx prettier --write \"$FILE_PATH\""
          }
        ]
      }
    ]
  }
}
```

**Solution 2: Verify hook command**

```bash
# Test hook command manually
npx prettier --write "src/app.ts"

# Check command exists
which prettier
```

**Solution 3: Check hook output**

```bash
# Run with verbose logging
claude --verbose

# After Edit, check logs for:
# "Running hook: PostToolUse"
# "Hook command output: ..."
```

**Solution 4: Ensure proper escaping**

```json
{
  "hooks": [{
    "command": "npx prettier --write \"$FILE_PATH\""  // Quotes around $FILE_PATH
  }]
}
```

---

### PreToolUse Hook Blocking

#### Problem: PreToolUse hook prevents tool execution

**Symptoms:**
- Tool never runs
- Hook exits with non-zero
- No error message to user

**Solution:**

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Edit(*)",
        "hooks": [
          {
            "type": "command",
            // Must exit 0 to allow tool
            "command": "echo 'Pre-check passed' && exit 0"

            // Exit 1 to block:
            // "command": "some-validation || exit 1"
          }
        ]
      }
    ]
  }
}
```

**Debug blocking hooks:**

```bash
# Verbose mode shows hook exit codes
claude --verbose

# Look for:
# "PreToolUse hook exited with code: 1"
# "Tool execution blocked by hook"
```

---

### Subagent Not Invoked

#### Problem: Subagent defined but never called

**Symptoms:**
- Subagent configured in settings.json
- Situation matches description
- Claude doesn't invoke it

**Possible Causes & Solutions:**

**1. Description not clear enough:**

```json
{
  "agents": {
    "reviewer": {
      // Vague
      "description": "Reviews code",

      // Better - explicit trigger
      "description": "Expert code reviewer. Use PROACTIVELY after any Edit or Write to review changes for bugs and style issues.",

      "prompt": "You are a code reviewer...",
      "model": "sonnet"
    }
  }
}
```

**2. Tool access too restrictive:**

```json
{
  "agents": {
    "reviewer": {
      "description": "...",
      "prompt": "...",
      "tools": ["Read", "Grep", "Glob"]  // Add necessary tools
    }
  }
}
```

**3. Invoke explicitly:**

```bash
# User can invoke manually
> @reviewer Please review the changes I just made
```

**4. Check verbose logs:**

```bash
claude --verbose
# Shows subagent invocation decisions
# "Considering subagent: reviewer"
# "Invoking subagent: reviewer"
```

---

## Configuration Issues

### Settings Not Loading

#### Problem: Changes to settings.json have no effect

**Symptoms:**
- Edit settings.json
- Restart Claude
- Old settings still active

**Solution:**

```bash
# 1. Verify JSON syntax
# Use JSON validator
cat ~/.claude/settings.json | jq .

# 2. Check file location
# User-level: ~/.claude/settings.json
# Project-level: <project>/.claude/settings.json
# Project-local: <project>/.claude/settings.local.json

# 3. Verify settings hierarchy
# Higher priority overrides lower
claude --verbose
# Shows: "Loaded settings from: ..."

# 4. Check for syntax errors
# Invalid JSON silently ignored
# Validate:
node -e "JSON.parse(require('fs').readFileSync('.claude/settings.json', 'utf8'))"
```

---

#### Problem: Environment variables not available

**Symptoms:**
- `${VAR}` in settings.json not expanded
- MCP servers fail due to missing env vars
- Tools don't see environment

**Solution:**

```bash
# 1. Export variables BEFORE starting Claude
export GITHUB_TOKEN="ghp_..."
export ANTHROPIC_API_KEY="sk-ant-..."
claude

# 2. Add to shell profile
# ~/.bashrc or ~/.zshrc
export GITHUB_TOKEN="ghp_..."
export ANTHROPIC_API_KEY="sk-ant-..."
source ~/.bashrc

# 3. Set in settings.json
{
  "env": {
    "GITHUB_TOKEN": "ghp_...",  // Literal value
    "API_KEY": "${EXTERNAL_API_KEY}"  // Reference system var
  }
}

# 4. Verify variables available
claude --verbose
# Shows environment passed to tools
```

---

#### Problem: Settings.local.json ignored

**Symptoms:**
- Create `.claude/settings.local.json`
- Values not applied
- Overrides don't work

**Cause:**
Local settings only loaded from project directory, not global.

**Solution:**

```bash
# Local settings MUST be in project
cd /path/to/project
ls .claude/settings.local.json  # Must exist here

# NOT in user directory
# ~/.claude/settings.local.json  # Wrong location

# Use user settings for personal global config
~/.claude/settings.json

# Use project local for personal project config
<project>/.claude/settings.local.json

# Verify loaded
claude --verbose
# Should show: "Loaded settings from: ...settings.local.json"
```

---

### Custom Output Styles Not Working

#### Problem: Custom output style not applied

**Symptoms:**
- Create `.claude/output-styles/custom.md`
- `/output-style custom` fails
- "Style not found" error

**Solution:**

```bash
# 1. Verify file location
# Project-level: .claude/output-styles/custom.md
# User-level: ~/.claude/output-styles/custom.md

# 2. Check file format
# Must have frontmatter:
---
description: My custom style
---

Custom instructions here...

# 3. Restart session
exit
claude

# 4. List available styles
/output-style
# Should show custom style

# 5. Apply style
/output-style custom
```

---

### Custom Slash Commands Not Found

#### Problem: Custom command doesn't appear

**Symptoms:**
- Create `.claude/commands/review.md`
- `/review` not recognized
- Not in `/help`

**Solution:**

```bash
# 1. Verify filename convention
# Must be: <command-name>.md
# .claude/commands/review.md → /review

# 2. Check file format
# Must have description:
---
description: Review code for issues
---

Please review the following code...

# 3. Restart terminal (important!)
# Commands loaded at terminal launch
exit
# Restart terminal completely
claude

# 4. Verify command loaded
/help
# Should show custom command

# 5. Test command
/review
```

---

## Diagnostic Commands

### /doctor Command

```bash
# Run diagnostics
claude
/doctor

# Output shows:
# - Installation status
# - Node version
# - Configuration loaded
# - API connectivity
# - MCP server status
# - Permission issues
```

**What /doctor checks:**
- Claude Code version
- Node.js version (requires 18+)
- Settings file syntax
- API key validity
- MCP servers connectivity
- File permissions
- PATH configuration

---

### Verbose Logging

```bash
# Start with verbose output
claude --verbose

# Shows detailed logs:
# - Settings loading
# - Model selection
# - Tool permission checks
# - MCP connections
# - Hook execution
# - API request/response timing
```

**Interpreting verbose logs:**

```
[DEBUG] Loaded settings from: /home/user/.claude/settings.json
[DEBUG] Model: claude-model: opus-4-5-20251001
[DEBUG] Permission check: Read(src/app.ts) → allowed
[DEBUG] Tool execution: Read(src/app.ts) → success (234ms)
[DEBUG] MCP server github: connected
[DEBUG] Hook PostToolUse: Edit(*.ts) → running prettier
[DEBUG] Hook exit code: 0
```

---

### Status Command

```bash
# Check current session status
claude
/status

# Shows:
# - Model in use
# - Context window usage
# - Permission mode
# - Working directories
# - MCP servers connected
# - Active subagents
```

---

### Context Command

```bash
# Check token usage
/context

# Output:
# Current: 45k/1000k tokens (4.5%)
```

---

### Debug Flags

```bash
# Maximum debugging
claude --verbose --debug

# Debug specific components
DEBUG=claude:* claude

# Debug MCP only
DEBUG=claude:mcp claude

# Debug tools only
DEBUG=claude:tools claude
```

---

### Log Files

```bash
# Session logs (if enabled)
# Location varies by platform:

# macOS
~/Library/Logs/Claude Code/

# Linux
~/.config/claude-code/logs/

# Windows
%APPDATA%\Claude Code\logs\

# View recent logs
tail -f ~/.config/claude-code/logs/main.log
```

---

## Quick Troubleshooting Matrix

### Installation

| Problem | Quick Fix | Full Solution |
|---------|-----------|---------------|
| `claude` not found | `npm install -g @anthropic-ai/claude-code` | Check PATH, use NVM |
| Permission denied | Use NVM | Change npm prefix |
| WSL Node.js mismatch | Install in WSL | Native WSL Node.js |
| Can't update | `npm uninstall -g @anthropic-ai/claude-code && npm install -g @anthropic-ai/claude-code` | Clear npm cache |

### Authentication

| Problem | Quick Fix | Full Solution |
|---------|-----------|---------------|
| Invalid API key | Set `ANTHROPIC_API_KEY` env var | Add to ~/.bashrc |
| Repeated permissions | Add wildcard patterns | Review permissions config |
| Permission mode resets | Set `defaultMode` in settings.json | Configure per-project |
| Bypass disabled | Remove `disableBypassPermissionsMode` | Check enterprise policies |

### Performance

| Problem | Quick Fix | Full Solution |
|---------|-----------|---------------|
| High memory | `/clear` to reset | Limit conversation length |
| Slow responses | Narrow scope | Optimize file operations |
| Unresponsive | Ctrl+C to cancel | Kill and restart |
| WSL2 slow | Work in WSL filesystem | Move project to WSL |
| Slow search | Use focused paths | Optimize .gitignore |

### Tools

| Problem | Quick Fix | Full Solution |
|---------|-----------|---------------|
| File not found | Use relative paths | Check working directory |
| Edit not persisting | Close file in editor | Check permissions |
| Bash fails silently | Use `--verbose` | Add error handling |
| PATH issues | Set in settings.json | Configure env vars |

### MCP

| Problem | Quick Fix | Full Solution |
|---------|-----------|---------------|
| Server won't start | Test command manually | Check env vars |
| Tools not available | Allow in permissions | Restart session |
| Connection timeout | Increase timeout | Check server logs |

### Configuration

| Problem | Quick Fix | Full Solution |
|---------|-----------|---------------|
| Settings not loading | Validate JSON syntax | Check hierarchy |
| Env vars missing | Export before Claude | Add to shell profile |
| Hooks not running | Check matcher pattern | Test command manually |
| Subagent ignored | Make description explicit | Add tool access |

### IDE Integration

| Problem | Quick Fix | Full Solution |
|---------|-----------|---------------|
| JetBrains on WSL | Use JetBrains Gateway | Configure IDE path |
| VS Code missing | Reinstall extension | Check extension logs |
| ESC conflicts | Remap in settings | Disable Vim plugin |
| Terminal colors | Set `TERM=screen-256color` | Configure multiplexer |

---

## Getting Further Help

### Official Resources

- **Documentation**: https://docs.claude.com/en/docs/claude-code/
- **GitHub Issues**: https://github.com/anthropics/claude-code/issues
- **Status Page**: https://status.anthropic.com/

### Community Support

- **Discord**: (Check official documentation for link)
- **GitHub Discussions**: https://github.com/anthropics/claude-code/discussions

### Reporting Bugs

When reporting issues, include:

1. **Version information**:
   ```bash
   claude --version
   node --version
   npm --version
   ```

2. **Operating system**:
   ```bash
   uname -a  # Linux/macOS
   # Or on Windows: ver
   ```

3. **Configuration** (redact secrets):
   ```bash
   cat ~/.claude/settings.json
   ```

4. **Verbose logs**:
   ```bash
   claude --verbose
   # Copy relevant error output
   ```

5. **Minimal reproduction**:
   - Steps to reproduce
   - Expected behavior
   - Actual behavior

---

## Best Practices to Avoid Issues

### 1. Use Version Control

```bash
# Checkpoint before major changes
git add -A
git commit -m "Checkpoint before refactor"

# Use Claude checkpoints too
/rewind save "Before refactor"
```

### 2. Start Fresh for Complex Tasks

```bash
# Don't continue very long sessions
# Start new for complex tasks
claude  # New session
# Not: claude -c  # After 100+ turns
```

### 3. Be Specific in Prompts

```bash
# Vague - may cause issues
> "Fix the app"

# Specific - works better
> "Fix the authentication error in src/auth/login.ts"
```

### 4. Use Appropriate Permissions

```json
{
  "permissions": {
    // Start restrictive
    "allow": ["Read(*)", "Grep(*)", "Glob(*)"],

    // Expand as needed
    "allow": ["Read(*)", "Edit(src/**/*.ts)"],

    // Not: Wide open
    "allow": ["*"]  // Only if you trust completely
  }
}
```

### 5. Monitor Resource Usage

```bash
# Check before long sessions
/context  # Token usage
top       # Memory usage

# Clear if getting high
/clear
```

### 6. Test Configuration Changes

```bash
# After editing settings.json
# 1. Validate JSON
cat settings.json | jq .

# 2. Test with verbose
claude --verbose

# 3. Check status
/status
```

### 7. Keep Software Updated

```bash
# Update Claude Code regularly
claude update

# Update Node.js
nvm install 20
nvm use 20

# Update npm
npm install -g npm@latest
```

---

## Conclusion

This troubleshooting guide covers the most common issues encountered with Claude Code Terminal. For issues not covered here:

1. Search official documentation
2. Check GitHub issues for similar problems
3. Run diagnostics (`/doctor`, `--verbose`)
4. Report bugs with full diagnostic information

**Remember:** Most issues stem from:
- Installation/PATH problems
- Permission configurations
- Resource constraints
- Configuration syntax errors

Start with the Quick Troubleshooting Matrix, use diagnostic commands, and escalate to official support if needed.

---

**Last Updated:** 2025-11-06
**Version:** 1.0
**Sources:**
- Official Claude Code documentation (https://code.claude.com/docs)
- DevForgeAI Terminal documentation (devforgeai/specs/Terminal/)
- Community-reported issues and solutions
