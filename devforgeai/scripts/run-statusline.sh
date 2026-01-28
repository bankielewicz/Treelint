#!/bin/bash
# Universal Python Launcher for Statusline
# Detects platform and calls appropriate Python command

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STATUSLINE_SCRIPT="$SCRIPT_DIR/statusline.py"

# Try commands in order of preference
if command -v python3 >/dev/null 2>&1; then
    # WSL/Linux/macOS standard
    exec python3 "$STATUSLINE_SCRIPT"
elif command -v py >/dev/null 2>&1; then
    # Windows Python Launcher
    exec py -3 "$STATUSLINE_SCRIPT"
elif command -v python >/dev/null 2>&1; then
    # Windows/Git Bash fallback
    exec python "$STATUSLINE_SCRIPT"
else
    echo "Error: No Python 3 installation found" >&2
    echo "Please install Python 3 and ensure it's in your PATH" >&2
    exit 1
fi
