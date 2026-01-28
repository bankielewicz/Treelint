#!/bin/bash

################################################################################
# Pre-Story-Create Hook
# STORY-302: Context Preservation Hooks for /create-story Command
#
# Purpose: Validate epic file existence and extract context for provenance
#          population before story creation.
#
# Usage:   Called by /create-story command before story file creation
#          Input: Epic file path via $1 argument (optional for greenfield mode)
#
# Exit Codes:
#   0 - Proceed (epic validated OR greenfield mode)
#   1 - Warn (epic issues but non-blocking per BR-001)
#
# Components:
#   COMP-001: Validate epic file exists when epic_id is provided
#   COMP-002: Traverse epic -> brainstorm chain when source_brainstorm exists
#   COMP-003: Prepare provenance data structure for story template population
#   COMP-005: Handle single-story mode (greenfield) when no epic provided
#
# Business Rules:
#   BR-001: Pre-hook failure does NOT block story creation (warning only)
#   BR-004: Chain traversal depth limited to 5 hops
#
# Created: 2026-01-23 (STORY-302)
################################################################################

set -o pipefail

# Configuration
PROJECT_ROOT="${PROJECT_ROOT:-/mnt/c/Projects/DevForgeAI2}"
MAX_CHAIN_DEPTH=5  # BR-004: Chain depth limit

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Input argument
EPIC_PATH="${1:-}"

# Extracted context storage
EPIC_ID=""
BUSINESS_GOAL=""
SUCCESS_METRICS=""
FEATURE_DESCRIPTION=""
STAKEHOLDER_GOALS=""
HYPOTHESES=""
PROBLEM_STATEMENT=""
DECISION_RATIONALE=""
SOURCE_BRAINSTORM=""
BRAINSTORM_PATH=""

################################################################################
# Utility: XML Character Escaping
################################################################################

xml_escape() {
    local input="$1"
    echo "$input" | sed 's/&/\&amp;/g; s/</\&lt;/g; s/>/\&gt;/g; s/"/\&quot;/g; s/'"'"'/\&apos;/g'
}

################################################################################
# COMP-001: Validate Epic File Exists
################################################################################

validate_epic_exists() {
    local epic_file="$1"

    # Check if argument provided
    if [[ -z "$epic_file" ]]; then
        echo -e "${CYAN}[INFO]${NC} Single-story mode: No epic file provided"
        echo -e "${CYAN}[INFO]${NC} Story will be created without provenance section"
        echo -e "${CYAN}[INFO]${NC} Continue without epic context"
        return 1  # Return 1 for greenfield mode
    fi

    # Check if file exists
    if [[ ! -f "$epic_file" ]]; then
        echo -e "${YELLOW}[WARN]${NC} Epic file not found: $epic_file"
        echo -e "${CYAN}[INFO]${NC} Single-story mode: Proceeding without epic"
        return 1  # Return 1 for greenfield mode
    fi

    # Check if file is readable
    if [[ ! -r "$epic_file" ]]; then
        echo -e "${RED}[ERROR]${NC} Epic file not readable: $epic_file"
        echo -e "${YELLOW}[WARN]${NC} Permission denied - proceeding without epic context"
        return 1  # Non-blocking per BR-001
    fi

    echo -e "${GREEN}[OK]${NC} Epic file validated: $epic_file"
    return 0
}

################################################################################
# COMP-001: Extract Epic Context
################################################################################

extract_epic_context() {
    local epic_file="$1"

    # Extract epic ID from filename or YAML frontmatter
    EPIC_ID=$(grep -E "^id:" "$epic_file" 2>/dev/null | head -1 | sed 's/id:\s*//' | tr -d ' ')
    if [[ -z "$EPIC_ID" ]]; then
        EPIC_ID=$(basename "$epic_file" | sed 's/\.epic\.md$//' | sed 's/\.md$//' | grep -oE 'EPIC-[0-9]+')
    fi
    echo -e "${BLUE}[EXTRACT]${NC} Epic ID: $EPIC_ID"

    # Extract business_goal (look for business_goal in YAML or ## Business Goal section)
    BUSINESS_GOAL=$(grep -E "^business_goal:" "$epic_file" 2>/dev/null | head -1 | sed 's/business_goal:\s*//' | sed 's/^"//' | sed 's/"$//')
    if [[ -z "$BUSINESS_GOAL" ]]; then
        BUSINESS_GOAL=$(awk '/^## Business Goal/,/^##[^#]/' "$epic_file" 2>/dev/null | grep -v "^##" | head -3 | tr '\n' ' ' | xargs)
    fi
    if [[ -n "$BUSINESS_GOAL" ]]; then
        echo -e "${BLUE}[EXTRACT]${NC} business_goal found"
    fi

    # Extract success_metrics
    SUCCESS_METRICS=$(awk '/^## Success Metrics/,/^##[^#]/' "$epic_file" 2>/dev/null | grep -E "^-|^\*" | head -5 | tr '\n' '|' | sed 's/|$//')
    if [[ -n "$SUCCESS_METRICS" ]]; then
        echo -e "${BLUE}[EXTRACT]${NC} success_metrics found"
    fi

    # Extract feature_description (from ## Features or first feature)
    FEATURE_DESCRIPTION=$(awk '/^## Features/,/^##[^#]/' "$epic_file" 2>/dev/null | grep -E "^[0-9]|^-" | head -3 | tr '\n' '|' | sed 's/|$//')
    if [[ -n "$FEATURE_DESCRIPTION" ]]; then
        echo -e "${BLUE}[EXTRACT]${NC} feature_description found"
    fi

    # Extract decision_rationale (from ## Rationale or decision section)
    DECISION_RATIONALE=$(awk '/^## (Rationale|Decision)/,/^##[^#]/' "$epic_file" 2>/dev/null | grep -v "^##" | head -3 | tr '\n' ' ' | xargs)
    if [[ -n "$DECISION_RATIONALE" ]]; then
        echo -e "${BLUE}[EXTRACT]${NC} decision_rationale found"
    fi

    return 0
}

################################################################################
# COMP-002: Traverse Epic -> Brainstorm Chain
################################################################################

# Security: Validate identifier pattern to prevent path traversal
validate_identifier() {
    local identifier="$1"
    # Allow only alphanumeric, hyphen, underscore (no path separators)
    if [[ "$identifier" =~ ^[A-Za-z0-9_-]+$ ]]; then
        return 0
    else
        echo -e "${YELLOW}[WARN]${NC} Invalid identifier format: $identifier"
        return 1
    fi
}

extract_source_brainstorm() {
    local epic_file="$1"

    # Extract source_brainstorm from YAML frontmatter
    SOURCE_BRAINSTORM=$(grep -E "^source_brainstorm:" "$epic_file" 2>/dev/null | head -1 | sed 's/source_brainstorm:\s*//' | tr -d ' ' | tr -d '"')

    if [[ -n "$SOURCE_BRAINSTORM" ]]; then
        # Security: Validate identifier before using in file paths
        if ! validate_identifier "$SOURCE_BRAINSTORM"; then
            echo -e "${YELLOW}[WARN]${NC} Skipping invalid source_brainstorm reference"
            return 1
        fi
        echo -e "${BLUE}[EXTRACT]${NC} source_brainstorm: $SOURCE_BRAINSTORM"
        return 0
    else
        echo -e "${CYAN}[INFO]${NC} No source_brainstorm field in epic"
        return 1
    fi
}

traverse_chain() {
    local epic_file="$1"
    local depth=0

    # Extract source_brainstorm reference
    if ! extract_source_brainstorm "$epic_file"; then
        echo -e "${CYAN}[INFO]${NC} Epic-only context (no brainstorm chain)"
        return 0
    fi

    # Construct brainstorm file path
    BRAINSTORM_PATH="${PROJECT_ROOT}/devforgeai/specs/brainstorms/${SOURCE_BRAINSTORM}.brainstorm.md"

    # Check for alternate path patterns
    if [[ ! -f "$BRAINSTORM_PATH" ]]; then
        BRAINSTORM_PATH=$(find "${PROJECT_ROOT}/devforgeai/specs/brainstorms" -name "${SOURCE_BRAINSTORM}*.md" 2>/dev/null | head -1)
    fi

    if [[ -f "$BRAINSTORM_PATH" ]]; then
        echo -e "${GREEN}[OK]${NC} Brainstorm file found: $BRAINSTORM_PATH"
        extract_brainstorm_context "$BRAINSTORM_PATH"
    else
        echo -e "${YELLOW}[WARN]${NC} Brainstorm file not found: ${SOURCE_BRAINSTORM}"
        echo -e "${CYAN}[INFO]${NC} Broken chain at epic -> brainstorm link"
    fi

    # BR-004: Check chain depth
    ((depth++))
    if [[ $depth -ge $MAX_CHAIN_DEPTH ]]; then
        echo -e "${YELLOW}[WARN]${NC} Chain depth exceeded (limit: $MAX_CHAIN_DEPTH)"
        return 0
    fi

    return 0
}

################################################################################
# COMP-002: Extract Brainstorm Context
################################################################################

extract_brainstorm_context() {
    local brainstorm_file="$1"

    # Extract stakeholder_goals
    STAKEHOLDER_GOALS=$(awk '/^## Stakeholder (Goals|Analysis)/,/^##[^#]/' "$brainstorm_file" 2>/dev/null | grep -E "^-|^\*" | head -5 | tr '\n' '|' | sed 's/|$//')
    if [[ -n "$STAKEHOLDER_GOALS" ]]; then
        echo -e "${BLUE}[EXTRACT]${NC} stakeholder_goals found"
    fi

    # Extract hypotheses
    HYPOTHESES=$(awk '/^## Hypothes/,/^##[^#]/' "$brainstorm_file" 2>/dev/null | grep -E "^[0-9]|^-|^\*" | head -5 | tr '\n' '|' | sed 's/|$//')
    if [[ -n "$HYPOTHESES" ]]; then
        echo -e "${BLUE}[EXTRACT]${NC} hypotheses found"
    fi

    # Extract problem_statement
    PROBLEM_STATEMENT=$(awk '/^## Problem Statement/,/^##[^#]/' "$brainstorm_file" 2>/dev/null | grep -v "^##" | head -5 | tr '\n' ' ' | sed 's/  */ /g' | xargs)
    if [[ -n "$PROBLEM_STATEMENT" ]]; then
        echo -e "${BLUE}[EXTRACT]${NC} problem_statement found"
    fi

    return 0
}

################################################################################
# COMP-003: Generate Provenance Data
################################################################################

generate_provenance_data() {
    echo ""
    echo -e "${CYAN}[PROVENANCE]${NC} Generated provenance data:"
    echo ""

    # Escape all user-provided content
    local escaped_epic_id=$(xml_escape "$EPIC_ID")
    local escaped_brainstorm=$(xml_escape "$SOURCE_BRAINSTORM")
    local escaped_business_goal=$(xml_escape "$BUSINESS_GOAL")
    local escaped_success_metrics=$(xml_escape "$SUCCESS_METRICS")
    local escaped_feature_description=$(xml_escape "$FEATURE_DESCRIPTION")
    local escaped_stakeholder_goals=$(xml_escape "$STAKEHOLDER_GOALS")
    local escaped_hypotheses=$(xml_escape "$HYPOTHESES")
    local escaped_problem_statement=$(xml_escape "$PROBLEM_STATEMENT")
    local escaped_decision_rationale=$(xml_escape "$DECISION_RATIONALE")

    cat << EOF
<provenance>
  <source_epic>$escaped_epic_id</source_epic>
  <source_brainstorm>${escaped_brainstorm:-null}</source_brainstorm>
  <business_rationale>$escaped_business_goal</business_rationale>
  <success_metrics>
$(echo "$escaped_success_metrics" | tr '|' '\n' | sed 's/^/    /')
  </success_metrics>
  <feature_description>
$(echo "$escaped_feature_description" | tr '|' '\n' | sed 's/^/    /')
  </feature_description>
  <stakeholder_goals>
$(echo "$escaped_stakeholder_goals" | tr '|' '\n' | sed 's/^/    /')
  </stakeholder_goals>
  <hypotheses>
$(echo "$escaped_hypotheses" | tr '|' '\n' | sed 's/^/    /')
  </hypotheses>
  <problem_statement>$escaped_problem_statement</problem_statement>
  <decision_rationale>$escaped_decision_rationale</decision_rationale>
</provenance>
EOF

    echo ""
    return 0
}

################################################################################
# Main Execution
################################################################################

main() {
    echo ""
    echo -e "${CYAN}=========================================${NC}"
    echo -e "${CYAN}  Pre-Story-Create Hook (STORY-302)${NC}"
    echo -e "${CYAN}=========================================${NC}"
    echo ""

    # COMP-001: Validate epic file exists
    validate_epic_exists "$EPIC_PATH"
    local validation_result=$?

    if [[ $validation_result -eq 1 ]]; then
        # COMP-005: Greenfield/single-story mode
        echo ""
        echo -e "${GREEN}[RESULT]${NC} Single-story mode - continuing without provenance"
        echo -e "${CYAN}[INFO]${NC} To add provenance later, run /create-story with --epic flag"
        exit 0  # Proceed (non-blocking per BR-001)
    fi

    # COMP-001: Extract epic context
    extract_epic_context "$EPIC_PATH"

    # COMP-002: Traverse epic -> brainstorm chain
    traverse_chain "$EPIC_PATH"

    # COMP-003: Generate provenance data
    generate_provenance_data

    echo ""
    echo -e "${GREEN}[RESULT]${NC} Pre-hook completed successfully"
    echo -e "${CYAN}[INFO]${NC} Provenance data ready for story template"
    exit 0  # Proceed
}

# Run main
main "$@"
