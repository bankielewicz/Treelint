#!/bin/bash

################################################################################
# Pre-Epic-Create Hook
# STORY-300: Context Preservation Hooks for /create-epic Command
#
# Purpose: Validate brainstorm file existence and extract context for provenance
#          population before epic creation.
#
# Usage:   Called by /create-epic command before epic file creation
#          Input: Brainstorm file path via $1 argument
#
# Exit Codes:
#   0 - Proceed (brainstorm validated OR greenfield mode)
#   1 - Warn (brainstorm issues but non-blocking)
#   2 - Halt (critical error - malformed input, etc.)
#
# Components:
#   COMP-001: Validate brainstorm file exists and is readable
#   COMP-002: Populate provenance section template with extracted context
#
# Business Rules:
#   BR-001: Pre-hook failure does NOT block epic creation (warning only)
#
# Created: 2026-01-23 (STORY-300)
################################################################################

set -o pipefail

# Configuration
PROJECT_ROOT="${PROJECT_ROOT:-/mnt/c/Projects/DevForgeAI2}"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Input argument
BRAINSTORM_PATH="${1:-}"

# Extracted context storage
PROBLEM_STATEMENT=""
STAKEHOLDER_GOALS=""
HYPOTHESES=""
BRAINSTORM_ID=""

################################################################################
# Utility: XML Character Escaping (CR-003 fix)
################################################################################

xml_escape() {
    # Escape special XML characters: & < > " '
    # Order matters: & must be escaped first to avoid double-escaping
    local input="$1"
    echo "$input" | sed 's/&/\&amp;/g; s/</\&lt;/g; s/>/\&gt;/g; s/"/\&quot;/g; s/'"'"'/\&apos;/g'
}

################################################################################
# COMP-001: Validate Brainstorm File Exists
################################################################################

validate_brainstorm_exists() {
    local brainstorm_file="$1"

    # Check if argument provided
    if [[ -z "$brainstorm_file" ]]; then
        echo -e "${CYAN}[INFO]${NC} Greenfield mode: No brainstorm file provided"
        echo -e "${CYAN}[INFO]${NC} Epic will be created without provenance section"
        echo -e "${CYAN}[INFO]${NC} Continue without brainstorm context"
        return 1  # Return 1 for greenfield mode (no file provided)
    fi

    # Check if file exists
    if [[ ! -f "$brainstorm_file" ]]; then
        echo -e "${YELLOW}[WARN]${NC} Brainstorm file not found: $brainstorm_file"
        echo -e "${CYAN}[INFO]${NC} Greenfield mode: Proceeding without brainstorm"
        return 1  # Return 1 for greenfield mode (file not found)
    fi

    # Check if file is readable (actual error - different from greenfield)
    if [[ ! -r "$brainstorm_file" ]]; then
        echo -e "${RED}[ERROR]${NC} Brainstorm file not readable: $brainstorm_file"
        echo -e "${RED}[ERROR]${NC} Permission denied or file access error"
        return 2  # Return 2 for actual error (file exists but not readable)
    fi

    echo -e "${GREEN}[OK]${NC} Brainstorm file validated: $brainstorm_file"
    return 0
}

################################################################################
# COMP-001: Extract Brainstorm Context
################################################################################

extract_brainstorm_context() {
    local brainstorm_file="$1"

    # Extract brainstorm ID from filename or YAML frontmatter
    BRAINSTORM_ID=$(grep -E "^id:" "$brainstorm_file" 2>/dev/null | head -1 | sed 's/id:\s*//' | tr -d ' ')
    if [[ -z "$BRAINSTORM_ID" ]]; then
        BRAINSTORM_ID=$(basename "$brainstorm_file" | sed 's/\.brainstorm\.md$//' | sed 's/\.md$//')
    fi
    echo -e "${BLUE}[EXTRACT]${NC} Brainstorm ID: $BRAINSTORM_ID"

    # Extract problem statement (look for ## Problem Statement section)
    PROBLEM_STATEMENT=$(awk '/^## Problem Statement/,/^##[^#]/' "$brainstorm_file" 2>/dev/null | grep -v "^##" | head -5 | tr '\n' ' ' | sed 's/  */ /g' | xargs)
    if [[ -n "$PROBLEM_STATEMENT" ]]; then
        echo -e "${BLUE}[EXTRACT]${NC} Problem statement found"
    fi

    # Extract stakeholder goals (look for ## Stakeholder Goals or ## Stakeholder Analysis)
    STAKEHOLDER_GOALS=$(awk '/^## Stakeholder (Goals|Analysis)/,/^##[^#]/' "$brainstorm_file" 2>/dev/null | grep -E "^-|^\*" | head -5 | tr '\n' '|' | sed 's/|$//')
    if [[ -n "$STAKEHOLDER_GOALS" ]]; then
        echo -e "${BLUE}[EXTRACT]${NC} Stakeholder goals found"
    fi

    # Extract hypotheses (look for ## Hypotheses section)
    HYPOTHESES=$(awk '/^## Hypothes/,/^##[^#]/' "$brainstorm_file" 2>/dev/null | grep -E "^[0-9]|^-|^\*" | head -5 | tr '\n' '|' | sed 's/|$//')
    if [[ -n "$HYPOTHESES" ]]; then
        echo -e "${BLUE}[EXTRACT]${NC} Hypotheses found"
    fi

    return 0
}

################################################################################
# COMP-002: Generate Provenance Data
################################################################################

generate_provenance_data() {
    # Generate provenance XML structure for epic template
    # Apply XML escaping to all user content to prevent XML corruption (CR-003)
    echo ""
    echo -e "${CYAN}[PROVENANCE]${NC} Generated provenance data:"
    echo ""

    # Escape all user-provided content before XML output
    local escaped_brainstorm_id
    local escaped_problem_statement
    local escaped_stakeholder_goals
    local escaped_hypotheses

    escaped_brainstorm_id=$(xml_escape "$BRAINSTORM_ID")
    escaped_problem_statement=$(xml_escape "$PROBLEM_STATEMENT")
    escaped_stakeholder_goals=$(xml_escape "$STAKEHOLDER_GOALS")
    escaped_hypotheses=$(xml_escape "$HYPOTHESES")

    cat << EOF
<provenance>
  <source_brainstorm>$escaped_brainstorm_id</source_brainstorm>
  <stakeholder_goals>
$(echo "$escaped_stakeholder_goals" | tr '|' '\n' | sed 's/^/    /')
  </stakeholder_goals>
  <business_rationale>$escaped_problem_statement</business_rationale>
  <hypotheses>
$(echo "$escaped_hypotheses" | tr '|' '\n' | sed 's/^/    /')
  </hypotheses>
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
    echo -e "${CYAN}  Pre-Epic-Create Hook (STORY-300)${NC}"
    echo -e "${CYAN}=========================================${NC}"
    echo ""

    # COMP-001: Validate brainstorm file exists
    validate_brainstorm_exists "$BRAINSTORM_PATH"
    local validation_result=$?

    if [[ $validation_result -eq 1 ]]; then
        # Greenfield mode - proceed without provenance (non-blocking per BR-001)
        echo ""
        echo -e "${GREEN}[RESULT]${NC} Greenfield mode - continuing without provenance"
        echo -e "${CYAN}[INFO]${NC} To add provenance later, run /create-epic with --brainstorm flag"
        exit 0  # Proceed (non-blocking)
    elif [[ $validation_result -eq 2 ]]; then
        # Actual error (permission issue) - warn but continue (non-blocking per BR-001)
        echo ""
        echo -e "${YELLOW}[WARN]${NC} Brainstorm file access error - continuing without provenance"
        echo -e "${CYAN}[INFO]${NC} Check file permissions: $BRAINSTORM_PATH"
        exit 0  # Proceed (non-blocking per BR-001, but error was logged)
    fi

    # COMP-001: Extract brainstorm context
    extract_brainstorm_context "$BRAINSTORM_PATH"

    # COMP-002: Generate provenance data
    generate_provenance_data

    echo ""
    echo -e "${GREEN}[RESULT]${NC} Pre-hook completed successfully"
    echo -e "${CYAN}[INFO]${NC} Provenance data ready for epic template"
    exit 0  # Proceed
}

# Run main
main "$@"
