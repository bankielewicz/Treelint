#!/bin/bash
# DevForgeAI Claude Launcher
# Usage: ./start-devforgeai.sh [claude args...]
#
# Appends DevForgeAI core directives to Claude's system prompt

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SYSTEM_PROMPT_FILE="$SCRIPT_DIR/../system-prompt-core.md"

if [ ! -f "$SYSTEM_PROMPT_FILE" ]; then
    echo "Error: system-prompt-core.md not found at $SYSTEM_PROMPT_FILE"
    exit 1
fi

# Launch Claude with appended system prompt
claude --append-system-prompt "$(cat "$SYSTEM_PROMPT_FILE")" "$@"
