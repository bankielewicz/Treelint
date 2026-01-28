#!/bin/bash

################################################################################
# Post-Story-Create Hook
# STORY-302: Context Preservation Hooks for /create-story Command
#
# Purpose: Invoke context-preservation-validator subagent to validate story-to-
#          epic-to-brainstorm linkage after story creation.
#
# Usage:   Called by /create-story command after story file creation
#          Input: Story file path(s) via positional arguments
#          Batch: Multiple story paths for batch mode validation
#
# Exit Codes:
#   0 - Proceed (validation passed OR greenfield mode)
#   1 - Warn (validation issues but non-blocking per BR-002)
#
# Components:
#   COMP-004: Invoke context-preservation-validator subagent with story file path
#   COMP-007: Support batch mode deferral - collect story IDs, single validation call
#
# Business Rules:
#   BR-002: Post-hook validation failure is non-blocking (warning only)
#
# Created: 2026-01-23 (STORY-302)
################################################################################

set -o pipefail

# Configuration
PROJECT_ROOT="${PROJECT_ROOT:-/mnt/c/Projects/DevForgeAI2}"
VALIDATOR_AGENT=".claude/agents/context-preservation-validator.md"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Input arguments (supports multiple for batch mode)
STORY_PATHS=("$@")
BATCH_MODE="${BATCH_MODE:-false}"

# Collected story IDs for batch validation
STORY_IDS=()

################################################################################
# COMP-007: Detect Batch Mode
################################################################################

detect_batch_mode() {
    local arg_count=${#STORY_PATHS[@]}

    # Batch mode if BATCH_MODE env is set or multiple arguments
    if [[ "$BATCH_MODE" == "true" ]] || [[ $arg_count -gt 1 ]]; then
        echo -e "${CYAN}[INFO]${NC} Batch mode detected ($arg_count stories)"
        return 0  # Batch mode
    else
        echo -e "${CYAN}[INFO]${NC} Single story mode"
        return 1  # Single mode
    fi
}

################################################################################
# COMP-004: Validate Story Context
################################################################################

validate_story_context() {
    local story_file="$1"

    # Check if argument provided
    if [[ -z "$story_file" ]]; then
        echo -e "${YELLOW}[WARN]${NC} No story file path provided"
        return 1
    fi

    # Check if file exists
    if [[ ! -f "$story_file" ]]; then
        echo -e "${YELLOW}[WARN]${NC} Story file not found: $story_file"
        return 1
    fi

    echo -e "${GREEN}[OK]${NC} Story file found: $story_file"
    return 0
}

################################################################################
# COMP-004: Check for Provenance Section
################################################################################

check_provenance_section() {
    local story_file="$1"

    # Check if story has provenance section
    if grep -qE "<provenance>|## Provenance" "$story_file" 2>/dev/null; then
        echo -e "${GREEN}[OK]${NC} Provenance section found in story"
        return 0
    else
        echo -e "${CYAN}[INFO]${NC} Greenfield story: No provenance section"
        echo -e "${CYAN}[INFO]${NC} Validation skipped for greenfield stories"
        return 1  # Greenfield mode
    fi
}

################################################################################
# COMP-004: Extract Epic Reference
################################################################################

# Security: Validate identifier pattern to prevent path traversal
validate_identifier() {
    local identifier="$1"
    # Allow only EPIC-NNN format or alphanumeric with hyphen/underscore
    if [[ "$identifier" =~ ^(EPIC-[0-9]+|[A-Za-z0-9_-]+)$ ]]; then
        return 0
    else
        echo -e "${YELLOW}[WARN]${NC} Invalid identifier format: $identifier" >&2
        return 1
    fi
}

extract_epic_reference() {
    local story_file="$1"
    local epic_ref=""

    # Extract epic from YAML frontmatter
    epic_ref=$(grep -E "^epic:" "$story_file" 2>/dev/null | head -1 | sed 's/epic:\s*//' | tr -d ' ' | tr -d '"')

    # Try source_epic from provenance section
    if [[ -z "$epic_ref" ]]; then
        epic_ref=$(grep -o '<source_epic>[^<]*</source_epic>' "$story_file" 2>/dev/null | head -1 | sed 's/<[^>]*>//g')
    fi

    if [[ -n "$epic_ref" ]]; then
        # Security: Validate identifier before using in file paths
        if ! validate_identifier "$epic_ref"; then
            echo -e "${CYAN}[INFO]${NC} No epic reference found - greenfield story"
            return 1
        fi
        echo -e "${BLUE}[EXTRACT]${NC} Epic reference: $epic_ref"
        echo "$epic_ref"
        return 0
    else
        echo -e "${CYAN}[INFO]${NC} No epic reference found - greenfield story"
        return 1
    fi
}

################################################################################
# COMP-004: Invoke Context Preservation Validator
################################################################################

invoke_validator_subagent() {
    local story_file="$1"
    local epic_ref="$2"

    echo ""
    echo -e "${CYAN}[VALIDATOR]${NC} Invoking context-preservation-validator subagent..."
    echo ""

    # Output Task invocation pattern for Claude to execute
    echo -e "${BLUE}[RECOMMENDATION]${NC} Run validation with:"
    echo ""
    echo "  Task("
    echo "    subagent_type=\"context-preservation-validator\","
    echo "    description=\"Validate story-to-epic-to-brainstorm chain\","
    echo "    prompt=\"Validate story-to-epic-to-brainstorm chain for $story_file\""
    echo "  )"
    echo ""

    # Perform basic validation in shell
    echo -e "${CYAN}[VALIDATION]${NC} Basic linkage check:"

    # Check if epic file exists
    if [[ -n "$epic_ref" ]]; then
        local epic_path=$(find "${PROJECT_ROOT}/devforgeai/specs/Epics" -name "${epic_ref}*.epic.md" 2>/dev/null | head -1)
        if [[ -f "$epic_path" ]]; then
            echo -e "  ${GREEN}✓${NC} Epic file exists: $epic_path"

            # Check for source_brainstorm in epic
            local source_brainstorm=$(grep -E "^source_brainstorm:" "$epic_path" 2>/dev/null | head -1)
            if [[ -n "$source_brainstorm" ]]; then
                echo -e "  ${GREEN}✓${NC} Epic has brainstorm reference"
                echo -e "  ${GREEN}✓${NC} chain_status: intact"
                echo "intact"
                return 0
            else
                echo -e "  ${YELLOW}⚠${NC} Epic has no brainstorm reference"
                echo -e "  ${YELLOW}⚠${NC} chain_status: partial"
                echo "partial"
                return 0
            fi
        else
            echo -e "  ${YELLOW}⚠${NC} Epic file not found: $epic_ref"
            echo -e "  ${YELLOW}⚠${NC} chain_status: broken"
            echo "broken"
            return 0
        fi
    else
        echo -e "  ${CYAN}ℹ${NC} No epic reference - greenfield story"
        echo -e "  ${CYAN}ℹ${NC} chain_status: greenfield"
        echo "greenfield"
        return 0
    fi
}

################################################################################
# COMP-004: Display Validation Result
################################################################################

display_validation_result() {
    local story_file="$1"
    local chain_status="$2"

    echo ""
    echo -e "${CYAN}=========================================${NC}"
    echo -e "${CYAN}  Context Validation Result${NC}"
    echo -e "${CYAN}=========================================${NC}"
    echo ""

    case "$chain_status" in
        "intact")
            echo -e "${GREEN}[RESULT]${NC} Story-to-epic-to-brainstorm linkage: INTACT"
            echo -e "${GREEN}[RESULT]${NC} Context preservation: COMPLETE"
            ;;
        "partial")
            echo -e "${YELLOW}[RESULT]${NC} Story-to-epic linkage: VALID"
            echo -e "${YELLOW}[RESULT]${NC} Epic-to-brainstorm linkage: MISSING"
            echo -e "${YELLOW}[RESULT]${NC} chain_status: PARTIAL"
            echo -e "${CYAN}[INFO]${NC} Recommendation: Add source_brainstorm to epic"
            ;;
        "broken")
            echo -e "${YELLOW}[RESULT]${NC} Story-to-epic linkage: BROKEN"
            echo -e "${YELLOW}[RESULT]${NC} chain_status: BROKEN"
            echo -e "${CYAN}[INFO]${NC} Recommendation: Verify epic file exists"
            ;;
        "greenfield")
            echo -e "${CYAN}[RESULT]${NC} Story type: GREENFIELD"
            echo -e "${CYAN}[RESULT]${NC} No provenance linkage required"
            echo -e "${CYAN}[RESULT]${NC} chain_status: N/A (greenfield mode)"
            echo -e "${CYAN}[INFO]${NC} Recommendation: Consider linking story to epic later"
            ;;
        *)
            echo -e "${YELLOW}[RESULT]${NC} chain_status: UNKNOWN"
            ;;
    esac

    echo ""
    echo -e "${CYAN}[INFO]${NC} Post-hook validation is non-blocking (BR-002)"
    echo -e "${CYAN}[INFO]${NC} Story creation continues regardless of validation result"

    return 0
}

################################################################################
# COMP-007: Batch Mode Validation
################################################################################

collect_story_ids() {
    for story_path in "${STORY_PATHS[@]}"; do
        if [[ -f "$story_path" ]]; then
            local story_id=$(grep -E "^id:" "$story_path" 2>/dev/null | head -1 | sed 's/id:\s*//' | tr -d ' ')
            if [[ -z "$story_id" ]]; then
                story_id=$(basename "$story_path" | sed 's/\.story\.md$//' | grep -oE 'STORY-[0-9]+')
            fi
            if [[ -n "$story_id" ]]; then
                STORY_IDS+=("$story_id")
            fi
        fi
    done
    echo -e "${BLUE}[BATCH]${NC} Collected ${#STORY_IDS[@]} story IDs for validation"
}

invoke_batch_validator() {
    echo ""
    echo -e "${CYAN}[BATCH VALIDATION]${NC} Deferred validation for batch..."
    echo ""

    # Output single Task invocation for all stories
    local story_list=$(printf ", " "${STORY_IDS[@]}")
    story_list="${story_list%, }"  # Remove trailing comma

    echo -e "${BLUE}[RECOMMENDATION]${NC} Run aggregate batch validation with:"
    echo ""
    echo "  Task("
    echo "    subagent_type=\"context-preservation-validator\","
    echo "    description=\"Batch validate ${#STORY_IDS[@]} stories\","
    echo "    prompt=\"Validate story-to-epic-to-brainstorm chain for batch: ${STORY_IDS[*]}\""
    echo "  )"
    echo ""

    # Aggregate chain_status
    local intact_count=0
    local partial_count=0
    local broken_count=0
    local greenfield_count=0

    for story_path in "${STORY_PATHS[@]}"; do
        if [[ -f "$story_path" ]]; then
            local epic_ref=$(grep -E "^epic:" "$story_path" 2>/dev/null | head -1 | sed 's/epic:\s*//' | tr -d ' ')
            if [[ -n "$epic_ref" ]]; then
                local epic_path=$(find "${PROJECT_ROOT}/devforgeai/specs/Epics" -name "${epic_ref}*.epic.md" 2>/dev/null | head -1)
                if [[ -f "$epic_path" ]]; then
                    if grep -qE "^source_brainstorm:" "$epic_path" 2>/dev/null; then
                        ((intact_count++))
                    else
                        ((partial_count++))
                    fi
                else
                    ((broken_count++))
                fi
            else
                ((greenfield_count++))
            fi
        fi
    done

    echo -e "${CYAN}[BATCH RESULT]${NC} Aggregate chain_status:"
    echo -e "  ${GREEN}Intact${NC}: $intact_count"
    echo -e "  ${YELLOW}Partial${NC}: $partial_count"
    echo -e "  ${RED}Broken${NC}: $broken_count"
    echo -e "  ${CYAN}Greenfield${NC}: $greenfield_count"
    echo ""

    return 0
}

################################################################################
# Main Execution
################################################################################

main() {
    echo ""
    echo -e "${CYAN}=========================================${NC}"
    echo -e "${CYAN}  Post-Story-Create Hook (STORY-302)${NC}"
    echo -e "${CYAN}=========================================${NC}"
    echo ""

    # Check if any arguments provided
    if [[ ${#STORY_PATHS[@]} -eq 0 ]]; then
        echo -e "${YELLOW}[WARN]${NC} No story file paths provided"
        echo -e "${CYAN}[INFO]${NC} Usage: post-story-create.sh <story-file> [story-file2 ...]"
        exit 0  # Non-blocking per BR-002
    fi

    # COMP-007: Detect batch mode
    if detect_batch_mode; then
        # Batch mode - defer validation
        collect_story_ids
        invoke_batch_validator
        echo -e "${GREEN}[RESULT]${NC} Batch validation completed (deferred until all stories created)"
        exit 0  # Proceed
    fi

    # Single story mode
    local story_path="${STORY_PATHS[0]}"

    # COMP-004: Validate story file
    if ! validate_story_context "$story_path"; then
        echo -e "${YELLOW}[WARN]${NC} Cannot validate - story file issue"
        exit 0  # Non-blocking per BR-002
    fi

    # Check for provenance section
    if ! check_provenance_section "$story_path"; then
        # Greenfield mode
        display_validation_result "$story_path" "greenfield"
        exit 0  # Non-blocking
    fi

    # Extract epic reference
    epic_ref=$(extract_epic_reference "$story_path")

    # Invoke validator subagent pattern
    chain_status=$(invoke_validator_subagent "$story_path" "$epic_ref")

    # Display validation result
    display_validation_result "$story_path" "$chain_status"

    echo ""
    echo -e "${GREEN}[RESULT]${NC} Post-hook completed successfully"
    exit 0  # Proceed (non-blocking per BR-002)
}

# Run main
main "$@"
