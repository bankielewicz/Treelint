#!/bin/bash

################################################################################
# Post-Epic-Create Hook
# STORY-300: Context Preservation Hooks for /create-epic Command
#
# Purpose: Invoke context-preservation-validator subagent to validate epic-to-
#          brainstorm linkage after epic creation.
#
# Usage:   Called by /create-epic command after epic file creation
#          Input: Epic file path via $1 argument
#
# Exit Codes:
#   0 - Proceed (validation passed OR greenfield mode)
#   1 - Warn (validation issues but non-blocking)
#
# Components:
#   COMP-003: Invoke context-preservation-validator subagent for linkage validation
#
# Business Rules:
#   BR-002: Post-hook validation failure is non-blocking (warning only)
#
# Created: 2026-01-23 (STORY-300)
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

# Input argument
EPIC_PATH="${1:-}"

################################################################################
# COMP-003: Validate Epic Context
################################################################################

validate_epic_context() {
    local epic_file="$1"

    # Check if argument provided
    if [[ -z "$epic_file" ]]; then
        echo -e "${YELLOW}[WARN]${NC} No epic file path provided"
        return 1
    fi

    # Check if file exists
    if [[ ! -f "$epic_file" ]]; then
        echo -e "${YELLOW}[WARN]${NC} Epic file not found: $epic_file"
        return 1
    fi

    echo -e "${GREEN}[OK]${NC} Epic file found: $epic_file"
    return 0
}

################################################################################
# COMP-003: Check for Provenance Section
################################################################################

check_provenance_section() {
    local epic_file="$1"

    # Check if epic has provenance section
    if grep -qE "<provenance>|## Provenance" "$epic_file" 2>/dev/null; then
        echo -e "${GREEN}[OK]${NC} Provenance section found in epic"
        return 0
    else
        echo -e "${CYAN}[INFO]${NC} Greenfield epic: No provenance section"
        echo -e "${CYAN}[INFO]${NC} Validation skipped for greenfield epics"
        return 1  # Greenfield mode
    fi
}

################################################################################
# COMP-003: Extract Source Brainstorm Reference
################################################################################

extract_source_brainstorm() {
    local epic_file="$1"

    # Extract source_brainstorm from provenance section or YAML frontmatter
    local source_brainstorm=""

    # Try XML format first (POSIX-compatible - no -P flag for portability)
    source_brainstorm=$(grep -o '<source_brainstorm>[^<]*</source_brainstorm>' "$epic_file" 2>/dev/null | head -1 | sed 's/<[^>]*>//g')

    # Try YAML frontmatter if XML not found
    if [[ -z "$source_brainstorm" ]]; then
        source_brainstorm=$(grep -E "^source_brainstorm:" "$epic_file" 2>/dev/null | head -1 | sed 's/source_brainstorm:\s*//' | tr -d ' ')
    fi

    if [[ -n "$source_brainstorm" ]]; then
        echo -e "${BLUE}[EXTRACT]${NC} Source brainstorm: $source_brainstorm"
        echo "$source_brainstorm"
        return 0
    else
        echo -e "${CYAN}[INFO]${NC} No source brainstorm reference found"
        return 1
    fi
}

################################################################################
# COMP-003: Invoke Context Preservation Validator (Subagent Pattern)
################################################################################

invoke_validator_subagent() {
    local epic_file="$1"
    local source_brainstorm="$2"

    echo ""
    echo -e "${CYAN}[VALIDATOR]${NC} Invoking context-preservation-validator subagent..."
    echo ""

    # This is a shell script hook, so we output the Task invocation pattern
    # that the calling context (Claude) should execute
    #
    # The actual subagent invocation happens in the Claude context, not in shell
    # This hook provides the validation recommendation

    echo -e "${BLUE}[RECOMMENDATION]${NC} Run validation with:"
    echo ""
    echo "  Task("
    echo "    subagent_type=\"context-preservation-validator\","
    echo "    description=\"Validate epic-to-brainstorm chain\","
    echo "    prompt=\"Validate story-to-epic-to-brainstorm chain for $epic_file\""
    echo "  )"
    echo ""

    # Perform basic validation in shell
    echo -e "${CYAN}[VALIDATION]${NC} Basic linkage check:"

    # Check if source brainstorm file exists
    if [[ -n "$source_brainstorm" ]]; then
        local brainstorm_path="${PROJECT_ROOT}/devforgeai/specs/brainstorms/${source_brainstorm}.brainstorm.md"
        if [[ -f "$brainstorm_path" ]]; then
            echo -e "  ${GREEN}✓${NC} Source brainstorm file exists"
            echo -e "  ${GREEN}✓${NC} Linkage: Epic → Brainstorm VALID"
        else
            echo -e "  ${YELLOW}⚠${NC} Source brainstorm file not found: $brainstorm_path"
            echo -e "  ${YELLOW}⚠${NC} Linkage: PARTIAL (brainstorm referenced but not found)"
        fi
    else
        echo -e "  ${CYAN}ℹ${NC} No brainstorm reference - greenfield epic"
    fi

    return 0
}

################################################################################
# COMP-003: Display Validation Result
################################################################################

display_validation_result() {
    local epic_file="$1"
    local validation_status="$2"

    echo ""
    echo -e "${CYAN}=========================================${NC}"
    echo -e "${CYAN}  Context Validation Result${NC}"
    echo -e "${CYAN}=========================================${NC}"
    echo ""

    case "$validation_status" in
        "valid")
            echo -e "${GREEN}[RESULT]${NC} Epic-to-brainstorm linkage: VALID"
            echo -e "${GREEN}[RESULT]${NC} Context preservation: COMPLETE"
            ;;
        "partial")
            echo -e "${YELLOW}[RESULT]${NC} Epic-to-brainstorm linkage: PARTIAL"
            echo -e "${YELLOW}[RESULT]${NC} Recommendation: Verify brainstorm file exists"
            ;;
        "greenfield")
            echo -e "${CYAN}[RESULT]${NC} Epic type: GREENFIELD"
            echo -e "${CYAN}[RESULT]${NC} No provenance linkage required"
            echo -e "${CYAN}[RESULT]${NC} Validation: N/A (greenfield mode)"
            ;;
        *)
            echo -e "${YELLOW}[RESULT]${NC} Validation status: UNKNOWN"
            ;;
    esac

    echo ""
    echo -e "${CYAN}[INFO]${NC} Post-hook validation is non-blocking (BR-002)"
    echo -e "${CYAN}[INFO]${NC} Epic creation continues regardless of validation result"

    return 0
}

################################################################################
# Main Execution
################################################################################

main() {
    echo ""
    echo -e "${CYAN}=========================================${NC}"
    echo -e "${CYAN}  Post-Epic-Create Hook (STORY-300)${NC}"
    echo -e "${CYAN}=========================================${NC}"
    echo ""

    # COMP-003: Validate epic file
    if ! validate_epic_context "$EPIC_PATH"; then
        echo -e "${YELLOW}[WARN]${NC} Cannot validate - epic file issue"
        exit 0  # Non-blocking (BR-002)
    fi

    # Check for provenance section
    if ! check_provenance_section "$EPIC_PATH"; then
        # Greenfield mode
        display_validation_result "$EPIC_PATH" "greenfield"
        exit 0  # Non-blocking
    fi

    # Extract source brainstorm reference
    source_brainstorm=$(extract_source_brainstorm "$EPIC_PATH")

    # Invoke validator subagent pattern
    invoke_validator_subagent "$EPIC_PATH" "$source_brainstorm"

    # Determine validation status
    if [[ -n "$source_brainstorm" ]]; then
        brainstorm_path="${PROJECT_ROOT}/devforgeai/specs/brainstorms/${source_brainstorm}.brainstorm.md"
        if [[ -f "$brainstorm_path" ]]; then
            display_validation_result "$EPIC_PATH" "valid"
        else
            display_validation_result "$EPIC_PATH" "partial"
        fi
    else
        display_validation_result "$EPIC_PATH" "greenfield"
    fi

    echo ""
    echo -e "${GREEN}[RESULT]${NC} Post-hook completed successfully"
    exit 0  # Proceed (non-blocking per BR-002)
}

# Run main
main "$@"
