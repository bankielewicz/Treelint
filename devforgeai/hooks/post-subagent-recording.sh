#!/bin/bash
################################################################################
# STORY-151: Post-Subagent Recording Hook
#
# Purpose: Records subagent invocations to phase state file for audit trail
# Event: post_tool_call (triggered after Task tool completes)
#
# Environment Variables:
#   CLAUDE_TOOL_NAME   - Tool that was invoked (expected: "Task")
#   CLAUDE_TOOL_INPUT  - JSON input containing subagent_type
#   DEVFORGEAI_STORY_ID - Story ID if set by /dev command (optional)
#
# Exit Codes:
#   0 - Always (non-blocking design)
#
# Dependencies:
#   - jq (JSON parsing)
#   - devforgeai-validate CLI (STORY-149)
#
# Performance Target: < 50ms
#
################################################################################

set -euo pipefail

# Configuration
readonly LOG_FILE="${DEVFORGEAI_LOG_DIR:-devforgeai/logs}/subagent-recordings.log"
readonly WORKFLOWS_DIR="${DEVFORGEAI_WORKFLOWS_DIR:-devforgeai/workflows}"
readonly CONFIG_FILE="${DEVFORGEAI_CONFIG_DIR:-devforgeai/config}/workflow-subagents.yaml"
readonly PROJECT_ROOT="${PROJECT_ROOT:-.}"

# Constants for recording results
readonly RESULT_RECORDED="recorded"
readonly RESULT_SKIPPED="skipped"
readonly RESULT_ERROR="error"
readonly UNKNOWN_VALUE="unknown"
readonly TOOL_NAME_TASK="Task"

# Non-blocking: catch all errors and exit 0
trap 'log_entry "" "" "" "$RESULT_ERROR" "Hook error: $BASH_COMMAND failed"; exit 0' ERR

################################################################################
# Utility Functions
################################################################################

# Get ISO-8601 timestamp
get_timestamp() {
    date -u +"%Y-%m-%dT%H:%M:%SZ"
}

# Build JSON log entry as a single line
build_log_json() {
    local story_id="$1"
    local subagent_name="$2"
    local phase_id="$3"
    local result="$4"
    local reason="$5"

    if command -v jq &> /dev/null; then
        jq -nc \
            --arg ts "$(get_timestamp)" \
            --arg sid "$story_id" \
            --arg sa "$subagent_name" \
            --arg pid "$phase_id" \
            --arg res "$result" \
            --arg rsn "$reason" \
            '{timestamp: $ts, story_id: $sid, subagent_name: $sa, phase_id: $pid, result: $res, reason: $rsn}'
    else
        # Fallback: manual JSON construction
        printf '{"timestamp":"%s","story_id":"%s","subagent_name":"%s","phase_id":"%s","result":"%s","reason":"%s"}' \
            "$(get_timestamp)" "$story_id" "$subagent_name" "$phase_id" "$result" "$reason"
    fi
}

# Log recording decision to JSON Lines file
log_entry() {
    local story_id="${1:-$UNKNOWN_VALUE}"
    local subagent_name="${2:-$UNKNOWN_VALUE}"
    local phase_id="${3:-$UNKNOWN_VALUE}"
    local result="${4:-$UNKNOWN_VALUE}"
    local reason="${5:-No reason provided}"

    # Ensure log directory exists
    mkdir -p "$(dirname "$LOG_FILE")"

    # Create and append JSON log entry (single line)
    local log_line
    log_line=$(build_log_json "$story_id" "$subagent_name" "$phase_id" "$result" "$reason")
    echo "$log_line" >> "$LOG_FILE"
}

# Check if jq is installed (required for JSON parsing)
check_jq() {
    if ! command -v jq &> /dev/null; then
        log_entry "" "" "" "error" "jq not installed, skipping recording"
        exit 0
    fi
}

# Validate JSON is well-formed
is_valid_json() {
    local json_str="$1"
    [[ -n "$json_str" ]] && echo "$json_str" | jq empty 2>/dev/null
}

################################################################################
# Story Context Extraction (AC#3)
################################################################################

# Extract story ID from DEVFORGEAI_STORY_ID environment variable (Priority 1)
extract_story_id_from_env() {
    if [[ -n "${DEVFORGEAI_STORY_ID:-}" ]]; then
        echo "$DEVFORGEAI_STORY_ID"
        return 0
    fi
    return 1
}

# Extract story ID from most recent state file (Priority 2)
extract_story_id_from_state_file() {
    if [[ ! -d "$WORKFLOWS_DIR" ]]; then
        return 1
    fi

    local latest_file
    latest_file=$(find "$WORKFLOWS_DIR" -name "*-phase-state.json" -type f -printf '%T@ %p\n' 2>/dev/null | sort -n | tail -1 | cut -d' ' -f2- || true)

    if [[ -n "$latest_file" ]]; then
        basename "$latest_file" | grep -oE "STORY-[0-9]+" || true
    fi
}

# Extract story ID from tool input via grep (Priority 3)
extract_story_id_from_input() {
    local tool_input="${CLAUDE_TOOL_INPUT:-}"
    if [[ -n "$tool_input" ]]; then
        echo "$tool_input" | grep -oE "STORY-[0-9]+" | head -1 || true
    fi
}

# Extract story ID from multiple sources in priority order
extract_story_id() {
    local story_id

    # Priority 1: Environment variable
    story_id=$(extract_story_id_from_env) && { echo "$story_id"; return 0; }

    # Priority 2: Most recent state file
    story_id=$(extract_story_id_from_state_file) && [[ -n "$story_id" ]] && { echo "$story_id"; return 0; }

    # Priority 3: Tool input
    story_id=$(extract_story_id_from_input) && [[ -n "$story_id" ]] && { echo "$story_id"; return 0; }

    # No story context found
    return 1
}

# Get current phase from state file
get_current_phase() {
    local story_id="$1"
    local state_file="${WORKFLOWS_DIR}/${story_id}-phase-state.json"

    if [[ ! -f "$state_file" ]]; then
        echo ""
        return 1
    fi

    jq -r '.current_phase // ""' "$state_file" 2>/dev/null || echo ""
}

################################################################################
# Subagent Filtering (AC#4)
################################################################################

# Check if subagent is a workflow subagent
is_workflow_subagent() {
    local subagent="$1"

    if [[ ! -f "$CONFIG_FILE" ]]; then
        # Config missing - assume workflow subagent to be safe
        return 0
    fi

    # Extract workflow_subagents section only (before excluded_subagents:)
    local workflow_section
    workflow_section=$(sed -n '/^workflow_subagents:/,/^excluded_subagents:/p' "$CONFIG_FILE" 2>/dev/null || true)

    # Check if subagent is in workflow section (use -F for fixed string matching)
    if echo "$workflow_section" | grep -qF -- "- $subagent"; then
        return 0
    fi

    return 1
}

################################################################################
# Validation and Recording Helper Functions
################################################################################

# Validate tool call is a Task with valid JSON input
validate_tool_call() {
    local tool_name="$1"
    local tool_input="$2"

    if [[ "$tool_name" != "$TOOL_NAME_TASK" ]]; then
        return 1
    fi

    is_valid_json "$tool_input" || return 1
    return 0
}

# Extract and validate subagent type from tool input
extract_subagent_type() {
    local tool_input="$1"
    echo "$tool_input" | jq -r '.subagent_type // ""' 2>/dev/null || echo ""
}

# Check if state file exists for story
state_file_exists() {
    local story_id="$1"
    local state_file="${WORKFLOWS_DIR}/${story_id}-phase-state.json"
    [[ -f "$state_file" ]]
}

# Record subagent invocation via CLI
record_subagent_to_state() {
    local story_id="$1"
    local phase_id="$2"
    local subagent_type="$3"

    if ! command -v python3 &> /dev/null; then
        log_entry "$story_id" "$subagent_type" "$phase_id" "$RESULT_ERROR" "python3 not available"
        return 1
    fi

    if python3 -m src.claude.scripts.devforgeai_cli.cli record-subagent "$story_id" "$phase_id" "$subagent_type" --project-root="$PROJECT_ROOT" 2>/dev/null; then
        log_entry "$story_id" "$subagent_type" "$phase_id" "$RESULT_RECORDED" "Workflow subagent recorded to phase state"
        return 0
    else
        log_entry "$story_id" "$subagent_type" "$phase_id" "$RESULT_ERROR" "devforgeai-validate CLI failed"
        return 1
    fi
}

################################################################################
# Main Hook Logic
################################################################################

main() {
    # Check dependencies
    check_jq

    # Get environment variables from Claude Code hook
    local tool_name="${CLAUDE_TOOL_NAME:-}"
    local tool_input="${CLAUDE_TOOL_INPUT:-}"

    # Validate tool call is Task with valid JSON
    if ! validate_tool_call "$tool_name" "$tool_input"; then
        exit 0
    fi

    # Extract and validate subagent type
    local subagent_type
    subagent_type=$(extract_subagent_type "$tool_input")
    if [[ -z "$subagent_type" ]]; then
        exit 0
    fi

    # AC#4: Skip non-workflow subagents
    if ! is_workflow_subagent "$subagent_type"; then
        log_entry "" "$subagent_type" "" "$RESULT_SKIPPED" "Non-workflow subagent skipped"
        exit 0
    fi

    # AC#3: Extract story context
    local story_id
    story_id=$(extract_story_id) || {
        log_entry "" "$subagent_type" "" "$RESULT_SKIPPED" "No story context detected, skipping recording"
        exit 0
    }

    # AC#5: Check if state file exists
    if ! state_file_exists "$story_id"; then
        log_entry "$story_id" "$subagent_type" "" "$RESULT_SKIPPED" "No state file found for $story_id, skipping recording"
        exit 0
    fi

    # Get current phase (default to "unknown" if not found)
    local phase_id
    phase_id=$(get_current_phase "$story_id") || phase_id="unknown"

    # AC#2: Record subagent invocation
    record_subagent_to_state "$story_id" "$phase_id" "$subagent_type"
    exit 0
}

# Run main function
main "$@"
