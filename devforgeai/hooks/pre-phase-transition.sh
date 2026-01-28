#!/bin/bash
################################################################################
# STORY-150: Pre-Phase-Transition Hook
#
# Purpose: Validates phase completion before allowing phase transitions
# Event: pre_tool_call (triggered before Task tool invocation)
#
# Environment Variables:
#   CLAUDE_TOOL_NAME  - Tool being invoked (expected: "Task")
#   CLAUDE_TOOL_INPUT - JSON input containing subagent_type and prompt
#
# Exit Codes:
#   0 - Validation passed, allow transition
#   1 - Validation failed, block transition
#
# Dependencies:
#   - jq (JSON parsing)
#   - devforgeai-validate CLI (phase state management)
#
# Performance Target: < 100ms
#
################################################################################

set -euo pipefail

# Configuration
readonly LOG_FILE="${DEVFORGEAI_LOG_DIR:-devforgeai/logs}/phase-enforcement.log"
readonly WORKFLOWS_DIR="${DEVFORGEAI_WORKFLOWS_DIR:-devforgeai/workflows}"
readonly PROJECT_ROOT="${PROJECT_ROOT:-.}"

# Constants
readonly VALID_SUBAGENTS="git-validator|tech-stack-detector|test-automator|backend-architect|frontend-developer|context-validator|refactoring-specialist|code-reviewer|integration-tester|deferral-validator|dev-result-interpreter"

# Error handling - fail-closed behavior
trap 'log_decision "" "" "blocked" "Hook error: $BASH_COMMAND failed"; exit 1' ERR

################################################################################
# Utility Functions
################################################################################

# Get ISO-8601 timestamp
get_timestamp() {
    date -u +"%Y-%m-%dT%H:%M:%SZ"
}

# Log validation decision to JSON Lines file
log_decision() {
    local story_id="${1:-unknown}"
    local target_phase="${2:-unknown}"
    local decision="${3:-unknown}"
    local reason="${4:-No reason provided}"

    # Ensure log directory exists
    mkdir -p "$(dirname "$LOG_FILE")"

    # Create JSON log entry (single line)
    local log_entry
    log_entry=$(jq -nc \
        --arg ts "$(get_timestamp)" \
        --arg sid "$story_id" \
        --arg tp "$target_phase" \
        --arg dec "$decision" \
        --arg rsn "$reason" \
        '{timestamp: $ts, story_id: $sid, target_phase: $tp, decision: $dec, reason: $rsn}')

    # Append to log file
    echo "$log_entry" >> "$LOG_FILE"
}

# Output structured error message
output_error() {
    local phase_incomplete="${1:-unknown}"
    local expected_subagents="${2:-[]}"
    local invoked_subagents="${3:-[]}"
    local remediation="${4:-Complete the previous phase before proceeding}"

    # Create structured error JSON
    jq -nc \
        --arg phase "$phase_incomplete" \
        --argjson expected "$expected_subagents" \
        --argjson invoked "$invoked_subagents" \
        --arg remedy "$remediation" \
        '{
            error: "Phase transition blocked",
            phase_incomplete: $phase,
            subagents: {expected: $expected, invoked: $invoked},
            remediation: $remedy
        }'
}

# Check if jq is installed
check_jq() {
    if ! command -v jq &> /dev/null; then
        echo "ERROR: jq is required for JSON parsing" >&2
        echo "Install jq:" >&2
        echo "  - Ubuntu/Debian: sudo apt-get install jq" >&2
        echo "  - macOS: brew install jq" >&2
        echo "  - Windows: choco install jq" >&2
        exit 1
    fi
}

# Validate JSON is well-formed
is_valid_json() {
    local json_str="$1"
    [[ -n "$json_str" ]] && echo "$json_str" | jq empty 2>/dev/null
}

# Extract story ID from prompt text
extract_story_id() {
    local prompt="$1"

    # Try to extract STORY-XXX pattern from prompt
    local story_id
    story_id=$(echo "$prompt" | grep -oE 'STORY-[0-9]+' | head -1 || true)

    if [[ -z "$story_id" ]]; then
        echo ""
        return 1
    fi

    echo "$story_id"
}

# Extract target phase from subagent type or prompt
extract_target_phase() {
    local subagent_type="$1"
    local prompt="$2"

    # Try to extract phase number from prompt (e.g., "Phase 02", "phase=02", "phase 02")
    local phase
    phase=$(echo "$prompt" | grep -oEi 'phase[=: ]*([0-9]{1,2})' | grep -oE '[0-9]+' | head -1 || true)

    if [[ -n "$phase" ]]; then
        # Pad to 2 digits
        printf "%02d" "$phase"
        return 0
    fi

    # Map subagent types to typical phases
    case "$subagent_type" in
        git-validator|tech-stack-detector)
            echo "01"
            ;;
        test-automator)
            echo "02"
            ;;
        backend-architect|frontend-developer|context-validator)
            echo "03"
            ;;
        refactoring-specialist|code-reviewer)
            echo "04"
            ;;
        integration-tester)
            echo "05"
            ;;
        deferral-validator)
            echo "06"
            ;;
        dev-result-interpreter)
            echo "10"
            ;;
        *)
            echo ""
            return 1
            ;;
    esac
}

# Get previous phase number
get_previous_phase() {
    local current_phase="$1"
    local phase_num

    # Remove leading zeros and get integer
    phase_num=$((10#$current_phase))

    if [[ $phase_num -le 1 ]]; then
        echo ""
        return 1
    fi

    # Return previous phase padded to 2 digits
    printf "%02d" $((phase_num - 1))
}

# Check if state file exists and is valid
is_valid_state_file() {
    local state_file="$1"

    if [[ ! -f "$state_file" ]]; then
        return 1
    fi

    is_valid_json "$(cat "$state_file")"
}

# Get phase status from state file
get_phase_status() {
    local state_file="$1"
    local phase="$2"

    jq -r ".phases.\"$phase\".status // \"pending\"" "$state_file" 2>/dev/null || echo "pending"
}

# Check if phase checkpoint was passed
is_checkpoint_passed() {
    local state_file="$1"
    local phase="$2"

    jq -r ".phases.\"$phase\".checkpoint_passed // false" "$state_file" 2>/dev/null || echo "false"
}

# Read state file and check if phase is complete
check_phase_complete() {
    local state_file="$1"
    local phase="$2"

    if ! is_valid_state_file "$state_file"; then
        return 1
    fi

    local status checkpoint_passed
    status=$(get_phase_status "$state_file" "$phase")
    checkpoint_passed=$(is_checkpoint_passed "$state_file" "$phase")

    # Allow completed phases with passed checkpoints
    if [[ "$status" == "completed" && "$checkpoint_passed" == "true" ]]; then
        return 0
    fi

    # Also allow "skipped" status (BR-003: skipped phases don't block)
    if [[ "$status" == "skipped" ]]; then
        return 0
    fi

    return 1
}

# Get expected subagents for a phase
get_expected_subagents() {
    local phase="$1"

    case "$phase" in
        01) echo '["git-validator", "tech-stack-detector"]' ;;
        02) echo '["test-automator"]' ;;
        03) echo '["backend-architect", "frontend-developer", "context-validator"]' ;;
        04) echo '["refactoring-specialist", "code-reviewer"]' ;;
        05) echo '["integration-tester"]' ;;
        06) echo '["deferral-validator"]' ;;
        07) echo '[]' ;;
        08) echo '[]' ;;
        09) echo '[]' ;;
        10) echo '["dev-result-interpreter"]' ;;
        *) echo '[]' ;;
    esac
}

# Get invoked subagents from state file
get_invoked_subagents() {
    local state_file="$1"
    local phase="$2"

    if [[ ! -f "$state_file" ]]; then
        echo '[]'
        return
    fi

    jq -c ".phases.\"$phase\".subagents // []" "$state_file" 2>/dev/null || echo '[]'
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

    # Only process Task tool calls
    if [[ "$tool_name" != "Task" ]]; then
        # Not a Task call, allow silently
        exit 0
    fi

    # Validate tool input is valid JSON
    if ! is_valid_json "$tool_input"; then
        # Invalid input, allow (don't block non-DevForgeAI tasks)
        exit 0
    fi

    # Extract subagent type
    local subagent_type
    subagent_type=$(echo "$tool_input" | jq -r '.subagent_type // ""' 2>/dev/null || echo "")

    # Check if this is a phase-related subagent
    if [[ -z "$subagent_type" ]]; then
        exit 0  # Not a subagent call, allow
    fi

    # Filter: Only validate known phase-related subagents
    if ! [[ "$subagent_type" =~ ^($VALID_SUBAGENTS)$ ]]; then
        # Not a phase-related subagent, allow
        exit 0
    fi

    # Extract prompt for story ID and phase detection
    local prompt
    prompt=$(echo "$tool_input" | jq -r '.prompt // ""' 2>/dev/null || echo "")

    # Extract story ID
    local story_id
    story_id=$(extract_story_id "$prompt")

    if [[ -z "$story_id" ]]; then
        # Cannot determine story ID, allow (might not be DevForgeAI workflow)
        exit 0
    fi

    # Extract target phase
    local target_phase
    target_phase=$(extract_target_phase "$subagent_type" "$prompt")

    if [[ -z "$target_phase" ]]; then
        # Cannot determine phase, allow
        log_decision "$story_id" "unknown" "allowed" "Could not determine target phase"
        exit 0
    fi

    # BR-001: Phase 01 always passes (no prior phase to check)
    if [[ "$target_phase" == "01" ]]; then
        log_decision "$story_id" "$target_phase" "allowed" "Phase 01 always allowed (no prior phase)"
        exit 0
    fi

    # Get state file path
    local state_file="${WORKFLOWS_DIR}/${story_id}-phase-state.json"

    # AC#5: Handle missing state file - auto-initialize
    if [[ ! -f "$state_file" ]]; then
        # Try to initialize state file
        if command -v python3 &> /dev/null; then
            if python3 -m src.claude.scripts.devforgeai_cli.cli phase-init "$story_id" --project-root="$PROJECT_ROOT" 2>/dev/null; then
                log_decision "$story_id" "$target_phase" "allowed" "State file auto-initialized"
                exit 0
            fi
        fi

        # If init fails but we're trying to start, allow
        log_decision "$story_id" "$target_phase" "allowed" "State file missing, allowing fresh start"
        exit 0
    fi

    # Check for corrupted state file
    if ! is_valid_state_file "$state_file"; then
        local error_msg
        error_msg=$(output_error "unknown" '[]' '[]' "State file corrupted. Delete ${state_file} and restart workflow.")
        echo "$error_msg" >&2
        log_decision "$story_id" "$target_phase" "blocked" "State file corrupted"
        exit 1
    fi

    # Get previous phase
    local prev_phase
    prev_phase=$(get_previous_phase "$target_phase")

    if [[ -z "$prev_phase" ]]; then
        # No previous phase (shouldn't happen for non-phase-01)
        log_decision "$story_id" "$target_phase" "allowed" "No previous phase to check"
        exit 0
    fi

    # AC#2: Check if previous phase is complete
    if check_phase_complete "$state_file" "$prev_phase"; then
        # Previous phase complete, allow transition
        log_decision "$story_id" "$target_phase" "allowed" "Previous phase $prev_phase completed successfully"
        exit 0
    fi

    # Previous phase not complete - block transition
    local expected_subagents invoked_subagents
    expected_subagents=$(get_expected_subagents "$prev_phase")
    invoked_subagents=$(get_invoked_subagents "$state_file" "$prev_phase")

    # AC#3: Output structured error message
    local error_msg
    error_msg=$(output_error \
        "$prev_phase" \
        "$expected_subagents" \
        "$invoked_subagents" \
        "Complete phase $prev_phase before proceeding to phase $target_phase")

    echo "$error_msg" >&2

    # Log blocked decision
    log_decision "$story_id" "$target_phase" "blocked" "Previous phase $prev_phase not completed"

    exit 1
}

# Run main function
main "$@"
