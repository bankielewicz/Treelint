#!/bin/bash

##############################################################################
# Error Handler - STORY-089 AC#3-4
# Purpose: Handle malformed epic files and detect orphaned stories
#
# AC#3: Malformed Epic File Error Handling
# - Clear error message identifying the specific issue
# - Line number where error was detected
# - Fix suggestion with example of correct format
# - Non-blocking continuation for other valid epic files
#
# AC#4: Orphaned Story Detection and Reporting
# - Identify stories with invalid epic_id references
# - Report as warnings (not blocking)
# - Provide suggested actions
#
# Exit codes:
# - 0: No issues found
# - 1: Warnings only (orphaned stories, non-blocking)
# - 2: Blocking errors
# - 3: Internal error
##############################################################################

set -o pipefail

# Script info
SCRIPT_NAME="error-handler.sh"
VERSION="1.0.0"

# Paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR}/../.."

# Default directories
DEFAULT_EPIC_DIR="${PROJECT_ROOT}/devforgeai/specs/Epics"
DEFAULT_STORY_DIR="${PROJECT_ROOT}/devforgeai/specs/Stories"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

##############################################################################
# Helper Functions
##############################################################################

log_error() {
    echo -e "${RED}ERROR:${NC} $1" >&2
}

log_warn() {
    echo -e "${YELLOW}WARNING:${NC} $1"
}

log_info() {
    echo -e "${BLUE}INFO:${NC} $1"
}

usage() {
    cat << EOF
Usage: $SCRIPT_NAME [OPTIONS] <directory>

Handle malformed epic files and detect orphaned stories.

Options:
    --validate <file>           Validate single file with error reporting
    --validate-dir <dir>        Validate all files in directory
    --detect-orphans <dir>      Detect orphaned stories
    --suggest-similar           Suggest similar epic names for typos
    --full-validation <dir>     Run all validations
    --validate-and-continue <dir>  Validate but continue on errors
    --verbose                   Show detailed output
    --help                      Show this help message
    --version                   Show version

Exit Codes:
    0    No issues found
    1    Warnings only (non-blocking)
    2    Blocking errors found
    3    Internal error
EOF
}

##############################################################################
# YAML Validation with Line Numbers
##############################################################################

validate_yaml_with_lines() {
    local file="$1"
    local errors=()

    # Check file exists
    if [[ ! -f "$file" ]]; then
        echo "File not found: $file"
        return 2
    fi

    # Read file with line numbers
    local line_num=0
    local in_frontmatter=false
    local frontmatter_start=0
    local frontmatter_end=0
    local has_error=false

    while IFS= read -r line || [[ -n "$line" ]]; do
        line_num=$((line_num + 1))
        line=$(echo "$line" | tr -d '\r')

        # Track frontmatter
        if [[ "$line" == "---" ]]; then
            if [[ "$in_frontmatter" == "false" ]]; then
                in_frontmatter=true
                frontmatter_start=$line_num
            else
                in_frontmatter=false
                frontmatter_end=$line_num
            fi
            continue
        fi

        # Check for YAML errors in frontmatter
        if [[ "$in_frontmatter" == "true" ]]; then
            # Check for invalid indentation after a key
            if [[ "$line" =~ ^[[:space:]]+ ]] && [[ "$line" =~ ^[[:space:]]+[a-zA-Z_]+: ]]; then
                # This could be a nested key - check if previous line had a list or map
                if [[ ! "$line" =~ ^[[:space:]]+-[[:space:]] ]]; then
                    echo "YAML error at line $line_num: invalid indentation"
                    echo "  Suggestion: Ensure consistent indentation. Nested values should use proper YAML syntax."
                    has_error=true
                fi
            fi

            # Check for tabs (YAML doesn't allow tabs for indentation)
            if [[ "$line" == *$'\t'* ]]; then
                echo "YAML error at line $line_num: tabs not allowed in YAML"
                echo "  Suggestion: Use spaces for indentation."
                has_error=true
            fi
        fi

    done < "$file"

    # Check for missing frontmatter
    if [[ $frontmatter_start -eq 0 ]]; then
        echo "Missing frontmatter: No opening '---' delimiter found"
        echo "  Suggestion: Add YAML frontmatter at the start of the file:"
        echo "  ---"
        echo "  epic_id: EPIC-001"
        echo "  title: Your Epic Title"
        echo "  status: Planning"
        echo "  priority: High"
        echo "  ---"
        has_error=true
    fi

    # Check for truncated frontmatter (no closing delimiter)
    if [[ $frontmatter_start -gt 0 ]] && [[ $frontmatter_end -eq 0 ]]; then
        echo "Missing closing frontmatter delimiter at line $frontmatter_start"
        echo "  Suggestion: Add closing '---' after frontmatter fields."
        has_error=true
    fi

    if [[ "$has_error" == "true" ]]; then
        return 1
    fi

    return 0
}

##############################################################################
# Directory Validation with Continuation
##############################################################################

validate_directory_with_continuation() {
    local dir="$1"
    local valid_count=0
    local invalid_count=0
    local errors_found=()

    if [[ ! -d "$dir" ]]; then
        log_error "Directory not found: $dir"
        return 2
    fi

    # Find all markdown files
    local files
    files=$(find "$dir" -name "*.md" 2>/dev/null)

    if [[ -z "$files" ]]; then
        echo "No files found in: $dir"
        return 0
    fi

    echo ""
    echo "Validating files in: $dir"
    echo "========================="
    echo ""

    while IFS= read -r file; do
        if [[ -z "$file" ]]; then
            continue
        fi

        local validation_output
        validation_output=$(validate_yaml_with_lines "$file" 2>&1)
        local exit_code=$?

        if [[ $exit_code -eq 0 ]]; then
            valid_count=$((valid_count + 1))
            echo -e "${GREEN}✓${NC} $(basename "$file")"
        else
            invalid_count=$((invalid_count + 1))
            errors_found+=("$file")
            echo -e "${RED}✗${NC} $(basename "$file")"
            echo "$validation_output" | sed 's/^/    /'
            echo ""
        fi

    done <<< "$files"

    echo ""
    echo "Validation Summary:"
    echo "  Files validated successfully: $valid_count"
    echo "  Files with errors: $invalid_count"

    if [[ $invalid_count -gt 0 ]]; then
        return 1
    fi

    return 0
}

##############################################################################
# Orphaned Story Detection
##############################################################################

get_all_epic_ids() {
    local epic_dir="${1:-$DEFAULT_EPIC_DIR}"
    local ids=()

    if [[ ! -d "$epic_dir" ]]; then
        return
    fi

    while IFS= read -r epic_file; do
        if [[ -z "$epic_file" ]]; then
            continue
        fi

        local epic_id
        epic_id=$(tr -d '\r' < "$epic_file" | grep -E "^(epic_id|id):" | head -1 | sed 's/^[^:]*:[[:space:]]*//' | tr -d '"' | tr -d "'")

        if [[ -n "$epic_id" ]]; then
            ids+=("$epic_id")
        fi
    done < <(find "$epic_dir" -name "*.epic.md" -o -name "*.md" 2>/dev/null)

    printf '%s\n' "${ids[@]}"
}

detect_orphaned_stories() {
    local dir="${1:-$DEFAULT_STORY_DIR}"
    local suggest_similar="${2:-false}"
    local orphans=()
    local valid_epic_ids

    # Get all valid epic IDs
    valid_epic_ids=$(get_all_epic_ids "$DEFAULT_EPIC_DIR")

    if [[ ! -d "$dir" ]]; then
        log_error "Directory not found: $dir"
        return 2
    fi

    echo ""
    echo "Orphaned Story Detection"
    echo "========================"
    echo ""

    local orphan_count=0
    local total_count=0

    while IFS= read -r story_file; do
        if [[ -z "$story_file" ]]; then
            continue
        fi

        total_count=$((total_count + 1))

        # Extract story ID
        local story_id
        story_id=$(tr -d '\r' < "$story_file" | grep -E "^id:" | head -1 | sed 's/^id:[[:space:]]*//' | tr -d '"')

        # Extract epic reference
        local epic_ref
        epic_ref=$(tr -d '\r' < "$story_file" | grep -E "^epic:" | head -1 | sed 's/^epic:[[:space:]]*//' | tr -d '"')

        if [[ -z "$epic_ref" ]]; then
            continue  # No epic reference, skip
        fi

        # Check if epic exists
        local is_valid=false
        while IFS= read -r valid_id; do
            if [[ "$epic_ref" == "$valid_id" ]]; then
                is_valid=true
                break
            fi
        done <<< "$valid_epic_ids"

        if [[ "$is_valid" == "false" ]]; then
            orphan_count=$((orphan_count + 1))
            echo -e "${YELLOW}⚠ WARNING:${NC} Orphaned story detected"
            echo "  Story ID: $story_id"
            echo "  Invalid epic reference: $epic_ref"
            echo "  File: $(basename "$story_file")"

            # Suggest similar epic if requested
            if [[ "$suggest_similar" == "true" ]]; then
                local similar
                similar=$(find_similar_epic "$epic_ref" "$valid_epic_ids")
                if [[ -n "$similar" ]]; then
                    echo "  Did you mean: $similar"
                fi
            fi

            echo "  Suggested action: Update epic reference or create EPIC file"
            echo ""
        fi

    done < <(find "$dir" -name "*.story.md" 2>/dev/null)

    echo ""
    echo "Summary:"
    echo "  Total stories scanned: $total_count"
    echo "  Orphaned stories: $orphan_count"

    if [[ $orphan_count -gt 0 ]]; then
        echo ""
        echo "Suggested actions:"
        echo "  1. Review orphaned stories and correct epic references"
        echo "  2. Create missing epic files if needed"
        echo "  3. Run /validate-epic-coverage for detailed analysis"
        return 1
    fi

    return 0
}

##############################################################################
# Similar Epic Suggestion
##############################################################################

find_similar_epic() {
    local typo="$1"
    local valid_ids="$2"

    # Simple Levenshtein-like matching: check for common typos
    # EPCI -> EPIC, etc.

    # Normalize the typo
    local normalized
    normalized=$(echo "$typo" | tr '[:lower:]' '[:upper:]')

    # Check for common patterns
    if [[ "$normalized" =~ ^EPC[I]?- ]]; then
        # Likely meant EPIC-
        local num
        num=$(echo "$normalized" | grep -oE "[0-9]+" | head -1)
        echo "EPIC-$num"
        return
    fi

    # Check for similar IDs
    while IFS= read -r valid_id; do
        if [[ -z "$valid_id" ]]; then
            continue
        fi

        # Compare length and first few chars
        local typo_prefix
        local valid_prefix
        typo_prefix="${typo:0:4}"
        valid_prefix="${valid_id:0:4}"

        if [[ "${typo_prefix^^}" == "${valid_prefix^^}" ]]; then
            echo "$valid_id"
            return
        fi
    done <<< "$valid_ids"
}

##############################################################################
# Full Validation
##############################################################################

full_validation() {
    local dir="$1"
    local has_errors=false
    local has_warnings=false

    echo ""
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║            Full Validation Report                            ║"
    echo "╚══════════════════════════════════════════════════════════════╝"

    # Validate epic files
    echo ""
    echo "Phase 1: Epic File Validation"
    echo "─────────────────────────────"

    local epic_dir="${dir}/Epics"
    if [[ -d "$epic_dir" ]]; then
        validate_directory_with_continuation "$epic_dir"
        [[ $? -ne 0 ]] && has_errors=true
    else
        validate_directory_with_continuation "$dir"
        [[ $? -ne 0 ]] && has_errors=true
    fi

    # Detect orphaned stories
    echo ""
    echo "Phase 2: Orphaned Story Detection"
    echo "──────────────────────────────────"

    local story_dir="${dir}/Stories"
    if [[ -d "$story_dir" ]]; then
        detect_orphaned_stories "$story_dir" true
        [[ $? -ne 0 ]] && has_warnings=true
    else
        detect_orphaned_stories "$dir" true
        [[ $? -ne 0 ]] && has_warnings=true
    fi

    # Summary
    echo ""
    echo "═══════════════════════════════════════════════════════════════"
    echo "Summary:"

    if [[ "$has_errors" == "true" ]]; then
        echo -e "  ${RED}✗${NC} Errors found (blocking)"
        return 2
    elif [[ "$has_warnings" == "true" ]]; then
        echo -e "  ${YELLOW}⚠${NC} Warnings found (non-blocking)"
        return 1
    else
        echo -e "  ${GREEN}✓${NC} All validations passed"
        return 0
    fi
}

##############################################################################
# Main Entry Point
##############################################################################

main() {
    local mode=""
    local target=""
    local suggest_similar=false
    local verbose=false

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --validate)
                mode="validate"
                target="$2"
                shift 2
                ;;
            --validate-dir)
                mode="validate-dir"
                target="$2"
                shift 2
                ;;
            --detect-orphans)
                mode="orphans"
                target="$2"
                shift 2
                ;;
            --suggest-similar)
                suggest_similar=true
                shift
                ;;
            --full-validation)
                mode="full"
                target="$2"
                shift 2
                ;;
            --validate-and-continue)
                mode="validate-continue"
                target="$2"
                shift 2
                ;;
            --verbose)
                verbose=true
                shift
                ;;
            --help)
                usage
                exit 0
                ;;
            --version)
                echo "$SCRIPT_NAME version $VERSION"
                exit 0
                ;;
            *)
                if [[ -z "$target" ]]; then
                    target="$1"
                fi
                shift
                ;;
        esac
    done

    if [[ -z "$target" ]] && [[ "$mode" != "" ]]; then
        target="$DEFAULT_STORY_DIR"
    fi

    case "$mode" in
        validate)
            validate_yaml_with_lines "$target"
            exit $?
            ;;
        validate-dir)
            validate_directory_with_continuation "$target"
            exit $?
            ;;
        orphans)
            detect_orphaned_stories "$target" "$suggest_similar"
            exit $?
            ;;
        full)
            full_validation "$target"
            exit $?
            ;;
        validate-continue)
            validate_directory_with_continuation "$target"
            # Always exit 0 for continue mode unless fatal error
            local result=$?
            [[ $result -eq 3 ]] && exit 3
            exit 0
            ;;
        *)
            if [[ -n "$target" ]]; then
                full_validation "$target"
                exit $?
            else
                usage
                exit 3
            fi
            ;;
    esac
}

# Run if not sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
