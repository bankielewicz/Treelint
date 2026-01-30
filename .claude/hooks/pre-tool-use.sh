#!/bin/bash
# .claude/hooks/pre-tool-use.sh - DevForgeAI validation hook

# Logging setup
PROJECT_ROOT="${CLAUDE_PROJECT_DIR:-/mnt/c/Projects/DevForgeAI2}"
LOG_FILE="$PROJECT_ROOT/devforgeai/logs/pre-tool-use.log"
UNKNOWN_COMMANDS_LOG="$PROJECT_ROOT/devforgeai/logs/hook-unknown-commands.log"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Create log directory if needed
mkdir -p "$PROJECT_ROOT/devforgeai/logs" 2>/dev/null

# Log function
log() {
  echo "[$TIMESTAMP] $1" >> "$LOG_FILE"
}

# Log unknown commands that require manual approval
log_unknown_command() {
  local cmd="$1"
  echo "[$TIMESTAMP] UNKNOWN COMMAND REQUIRING APPROVAL: $cmd" >> "$UNKNOWN_COMMANDS_LOG"
}

log "========== HOOK INVOKED =========="

# Read tool input
TOOL_INPUT=$(cat)
log "Raw input length: ${#TOOL_INPUT} chars"
log "Input preview: ${TOOL_INPUT:0:200}..."

# Extract command
COMMAND=$(echo "$TOOL_INPUT" | jq -r '.tool_input.command // empty' 2>/dev/null)
EXTRACT_STATUS=$?

log "jq extraction exit code: $EXTRACT_STATUS"
log "Extracted command: '$COMMAND'"

if [ -z "$COMMAND" ]; then
  log "WARNING: Command is empty after extraction"
  log "Full input: $TOOL_INPUT"
fi

# Auto-approve safe DevForgeAI patterns
SAFE_PATTERNS=(
  "npm run test"
  "npm run build"
  "npm run lint"  
  "dotnet test"
  "dotnet build"
  "git status"
  "git diff"
  "git add"
  "git commit"
  "git log"
  "wc -"
  "bash tests/"
  "bash .claude/scripts/"
  "bash devforgeai/"
  "echo "
  "cat tests/"
  "cat devforgeai/"
  "cat >"
  "cat <<"
  "cat << 'EOF'"
  "cp"
  "grep -E"
  "head -"
  "tail -"
  "mkdir -p"
  "chmod +x"
  "dos2unix"
  "sed -i"
  "python3 -m json.tool"
  "python3 <<"
  "python -m pytest"
  "python3 -m pytest"
  "python3 << 'EOF'"
  "python3 << 'EOF'
   import re"
  "pytest"
  "wc -l"
  "ls -la"
  "ls -lh"
  "ls -1"
  "cat src/"
  "cat installer/"
  "find installer"
  "find /mnt/c/Projects/DevForgeAI2/installer"
  "find /mnt/c/Projects/DevForgeAI2/src"
  "find /mnt/c/Projects/DevForgeAI2/tests"
  "grep -r"
  "python3 -m py_compile"
  "sort -"
  # Common command composition (RCA-015 - reduces 90% of approval friction)
  "cd "                           # Directory changes (always safe)
  "python3 -c "                   # Inline Python (safe, no file modification)
  "python3 << 'EOF'"              # HERE-documents (safe, read-only analysis)
  "python << 'EOF'"               # Python 2 HERE-docs
  "devforgeai "                   # Framework's own CLI (always safe)
  "git rev-parse"                 # Git introspection (read-only)
  "git branch"                    # Branch info (read-only)
  "git --version"                 # Git version check
  "git rev-list"                  # Commit history (read-only)
  "which "                        # Command location (safe)
  "command -v"                    # Command detection (safe)
  "type "                         # Command type (safe)
  "stat "                         # File stats (read-only)
  "file "                         # File type detection (read-only)
  "basename "                     # Path manipulation (safe)
)

# CRITICAL: Define BLOCKED_PATTERNS before SAFE_PATTERNS loop uses it (STORY-195 fix)
BLOCKED_PATTERNS=(
  "rm -rf"
  "sudo"
  "git push"
  "npm publish"
  "curl"
  "wget"
)

log "Checking against ${#SAFE_PATTERNS[@]} safe patterns..."

# RCA-015 REC-02: Quote-aware base command extraction
# Strips pipes and redirects OUTSIDE quotes to enable safe command + pipe/redirect auto-approval
extract_base_command() {
  local cmd="$1"
  local result=""
  local in_quote=false
  local quote_char=""

  for (( i=0; i<${#cmd}; i++ )); do
    char="${cmd:$i:1}"

    # Handle quote state changes
    if [ "$in_quote" = false ] && { [ "$char" = "'" ] || [ "$char" = '"' ]; }; then
      in_quote=true
      quote_char="$char"
      result="${result}${char}"
    elif [ "$char" = "$quote_char" ] && [ "$in_quote" = true ]; then
      in_quote=false
      quote_char=""
      result="${result}${char}"

    # Outside quotes: stop at pipe or redirect
    elif [ "$in_quote" = false ]; then
      case "$char" in
        "|")
          break
          ;;
        ">")
          break
          ;;
        "2")
          if [ "${cmd:$i:4}" = "2>&1" ] || [ "${cmd:$i:2}" = "2>" ]; then
            break
          else
            result="${result}${char}"
          fi
          ;;
        *)
          result="${result}${char}"
          ;;
      esac

    # Inside quotes: keep everything (including | and >)
    else
      result="${result}${char}"
    fi
  done

  echo "$result"
}

# Enhanced pattern matching with pipe/redirect support
for pattern in "${SAFE_PATTERNS[@]}"; do
  # Extract base command (quote-aware)
  BASE_CMD=$(extract_base_command "$COMMAND")

  # Match base command against safe pattern
  if [[ "$BASE_CMD" == "$pattern"* ]]; then
    log "✓ MATCHED safe pattern: '$pattern'"

    # Log extraction if pipes/redirects were stripped
    if [[ "$COMMAND" != "$BASE_CMD" ]]; then
      log "  Full command: $COMMAND"
      log "  Base extracted: $BASE_CMD"
    fi

    # SAFETY CHECK 1: Verify full command doesn't contain blocked patterns
    # Catches: git status | rm -rf /tmp (safe base, dangerous pipe)
    for blocked in "${BLOCKED_PATTERNS[@]}"; do
      if [[ "$COMMAND" =~ ${blocked} ]]; then
        log "✗ Base safe BUT full command contains blocked pattern: '$blocked'"
        log "Decision: BLOCK (exit 2)"
        log "Sending error to Claude: Command contains dangerous operation"
        log "=========================================="
        echo '{"decision": "block", "reason": "Command contains dangerous operation: '"$blocked"'"}' >&2
        exit 2
      fi
    done

    # SAFETY CHECK 2: Block redirects to system directories
    # Catches: echo test > /etc/passwd (even though echo is safe)
    if [[ "$COMMAND" =~ \>[[:space:]]*/etc/ ]] || \
       [[ "$COMMAND" =~ \>[[:space:]]*/usr/ ]] || \
       [[ "$COMMAND" =~ \>[[:space:]]*/sys/ ]] || \
       [[ "$COMMAND" =~ \>[[:space:]]*/boot/ ]] || \
       [[ "$COMMAND" =~ \>[[:space:]]*/root/ ]]; then
      log "✗ Redirect to system directory detected"
      log "Decision: BLOCK (exit 2)"
      log "Sending error to Claude: Redirect to protected system directory"
      log "=========================================="
      echo '{"decision": "block", "reason": "Redirect to protected system directory"}' >&2
      exit 2
    fi

    # Safe base + no blocked patterns + no system redirects = auto-approve
    log "Decision: AUTO-APPROVE (exit 0)"
    log "=========================================="
    exit 0  # Auto-approve
  fi
done

log "No safe pattern matched"

# STORY-197: Near-miss detection for pattern improvement
NEAR_MISSES=()
for pattern in "${SAFE_PATTERNS[@]}"; do
    if [[ "$COMMAND" == *"$pattern"* ]]; then
        NEAR_MISSES+=("$pattern")
    fi
done

# Log near-misses if any found
if [[ ${#NEAR_MISSES[@]} -gt 0 ]]; then
    log "NEAR-MISS DETECTED"
    log "Command starts with: ${COMMAND:0:20}"
    for nm in "${NEAR_MISSES[@]}"; do
        log "  Near-miss pattern: $nm"
    done
    log "RECOMMENDATION: Command contains safe pattern but doesn't start with it - consider adding pattern"
fi

# Block anti-patterns (BLOCKED_PATTERNS defined at top for use in safety checks)
log "Checking against ${#BLOCKED_PATTERNS[@]} blocked patterns..."

for pattern in "${BLOCKED_PATTERNS[@]}"; do
  if [[ "$COMMAND" =~ ${pattern} ]]; then
    log "✗ MATCHED blocked pattern: '$pattern'"
    log "Decision: BLOCK (exit 2)"
    log "Sending error to Claude: Dangerous operation: $COMMAND"
    log "=========================================="
    echo '{"decision": "block", "reason": "Dangerous operation: '"$COMMAND"'"}' >&2
    exit 2
  fi
done

log "No blocked pattern matched"

# For all others, ask user for approval
log "Decision: ASK USER (exit 1)"
log "Command requires manual approval: $COMMAND"
log_unknown_command "$COMMAND"
log "=========================================="
exit 1