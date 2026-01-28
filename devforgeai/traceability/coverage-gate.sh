#!/bin/bash

##############################################################################
# Coverage Gate - STORY-089 AC#2
# Purpose: Coverage quality gate for /orchestrate workflow
#
# Validates coverage during sprint planning phase:
# - Pass (green) if coverage >= 80%
# - Warn (yellow) if coverage 70-80%
# - Block (red) if coverage < 70%
#
# Exit codes:
# - 0: PASS (coverage >= 80%)
# - 1: WARN (coverage 70-80%)
# - 2: BLOCK (coverage < 70%)
# - 3: Internal error
##############################################################################

set -o pipefail

# Script info
SCRIPT_NAME="coverage-gate.sh"
VERSION="1.0.0"

# Paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR}/../.."
GAP_DETECTOR="${SCRIPT_DIR}/gap-detector.sh"
REPORT_GENERATOR="${PROJECT_ROOT}/devforgeai/epic-coverage/generate-report.sh"

# Default directories
DEFAULT_EPIC_DIR="${PROJECT_ROOT}/devforgeai/specs/Epics"
DEFAULT_STORY_DIR="${PROJECT_ROOT}/devforgeai/specs/Stories"

# Default thresholds
PASS_THRESHOLD=80
WARN_THRESHOLD=70

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

log_pass() {
    echo -e "${GREEN}✓ PASS:${NC} $1"
}

log_block() {
    echo -e "${RED}✗ BLOCK:${NC} $1"
}

usage() {
    cat << EOF
Usage: $SCRIPT_NAME [OPTIONS]

Coverage quality gate for /orchestrate workflow sprint planning phase.

Options:
    --coverage <percent>       Test with specific coverage percentage
    --epic-dir <dir>          Directory containing epic files
    --story-dir <dir>         Directory containing story files
    --config <file>           Custom thresholds configuration file
    --phase <phase>           Workflow phase (default: sprint-planning)
    --color                   Force color output
    --verbose                 Show detailed output
    --help                    Show this help message
    --version                 Show version

Exit Codes:
    0    PASS (coverage >= 80%)
    1    WARN (coverage 70-80%)
    2    BLOCK (coverage < 70%)
    3    Internal error

Thresholds:
    Default: pass=80%, warn=70%
    Custom thresholds via --config JSON file

Examples:
    $SCRIPT_NAME --coverage 75                    # Test with 75% coverage
    $SCRIPT_NAME --epic-dir devforgeai/specs/Epics        # Validate real files
    $SCRIPT_NAME --config thresholds.json         # Use custom thresholds
EOF
}

##############################################################################
# Threshold Management
##############################################################################

load_thresholds() {
    local config_file="$1"

    if [[ -f "$config_file" ]]; then
        local pass warn
        pass=$(jq -r '.pass // .coverage_gates.pass // 80' "$config_file" 2>/dev/null)
        warn=$(jq -r '.warn // .coverage_gates.warn // 70' "$config_file" 2>/dev/null)

        if [[ "$pass" =~ ^[0-9]+$ ]]; then
            PASS_THRESHOLD=$pass
        fi
        if [[ "$warn" =~ ^[0-9]+$ ]]; then
            WARN_THRESHOLD=$warn
        fi
    fi
}

##############################################################################
# Coverage Calculation
##############################################################################

calculate_coverage() {
    local epic_dir="$1"
    local story_dir="$2"

    # If gap detector exists, use it
    if [[ -f "$GAP_DETECTOR" ]]; then
        local result
        result=$("$GAP_DETECTOR" 2>&1)
        local exit_code=$?

        if [[ $exit_code -eq 0 ]]; then
            # Try to extract coverage from gap detector output
            local coverage
            coverage=$(echo "$result" | grep -oE "[0-9]+\.?[0-9]*%" | head -1 | tr -d '%')
            if [[ -n "$coverage" ]]; then
                echo "$coverage"
                return 0
            fi
        fi
    fi

    # Fallback: calculate manually
    local total_features=0
    local covered_features=0

    # Count features from all epics
    if [[ -d "$epic_dir" ]]; then
        while IFS= read -r epic_file; do
            local feature_count
            feature_count=$(tr -d '\r' < "$epic_file" | grep -cE "^###[[:space:]]+Feature[[:space:]]+[0-9]+:" 2>/dev/null || echo "0")
            total_features=$((total_features + feature_count))
        done < <(find "$epic_dir" -name "*.epic.md" -o -name "*.md" 2>/dev/null)
    fi

    # Count stories as covered features (simplified)
    if [[ -d "$story_dir" ]]; then
        covered_features=$(find "$story_dir" -name "*.story.md" 2>/dev/null | wc -l)
    fi

    # Calculate percentage
    if [[ $total_features -eq 0 ]]; then
        echo "0"
    else
        local percentage
        percentage=$(echo "scale=1; ($covered_features * 100) / $total_features" | bc 2>/dev/null || echo "0")
        echo "$percentage"
    fi
}

##############################################################################
# Gate Evaluation
##############################################################################

evaluate_gate() {
    local coverage="$1"

    # Handle decimal values
    local int_coverage
    int_coverage=$(echo "$coverage" | cut -d'.' -f1)

    if [[ -z "$int_coverage" ]] || [[ "$int_coverage" == "" ]]; then
        int_coverage=0
    fi

    # Compare against thresholds
    if (( $(echo "$coverage >= $PASS_THRESHOLD" | bc -l 2>/dev/null || echo "0") )); then
        echo "PASS"
        return 0
    elif (( $(echo "$coverage >= $WARN_THRESHOLD" | bc -l 2>/dev/null || echo "0") )); then
        echo "WARN"
        return 1
    else
        echo "BLOCK"
        return 2
    fi
}

##############################################################################
# Per-Epic Breakdown
##############################################################################

generate_epic_breakdown() {
    local epic_dir="$1"
    local story_dir="$2"

    echo ""
    echo "Epic Coverage Breakdown:"
    echo "========================"
    echo ""

    if [[ ! -d "$epic_dir" ]]; then
        echo "  No epics directory found"
        return
    fi

    local epic_files
    epic_files=$(find "$epic_dir" -name "*.epic.md" 2>/dev/null)

    if [[ -z "$epic_files" ]]; then
        echo "  No epic files found"
        return
    fi

    printf "  %-15s %-40s %10s %8s\n" "Epic ID" "Title" "Features" "Coverage"
    printf "  %-15s %-40s %10s %8s\n" "-------" "-----" "--------" "--------"

    while IFS= read -r epic_file; do
        if [[ -z "$epic_file" ]]; then
            continue
        fi

        # Extract epic ID
        local epic_id
        epic_id=$(tr -d '\r' < "$epic_file" | grep -E "^(epic_id|id):" | head -1 | sed 's/^[^:]*:[[:space:]]*//' | tr -d '"')

        # Extract title
        local title
        title=$(tr -d '\r' < "$epic_file" | grep -E "^title:" | head -1 | sed 's/^title:[[:space:]]*//' | tr -d '"' | cut -c1-40)

        # Count features
        local feature_count
        feature_count=$(tr -d '\r' < "$epic_file" | grep -cE "^###[[:space:]]+Feature[[:space:]]+[0-9]+:" 2>/dev/null || echo "0")

        # Count stories for this epic
        local story_count=0
        if [[ -d "$story_dir" ]]; then
            story_count=$(grep -l "epic:[[:space:]]*${epic_id}" "$story_dir"/*.story.md 2>/dev/null | wc -l)
        fi

        # Calculate epic coverage
        local epic_coverage
        if [[ $feature_count -eq 0 ]]; then
            epic_coverage="N/A"
        else
            epic_coverage=$(echo "scale=0; ($story_count * 100) / $feature_count" | bc 2>/dev/null || echo "0")
            epic_coverage="${epic_coverage}%"
        fi

        # Color code
        local status_icon
        local cov_num
        cov_num=$(echo "$epic_coverage" | tr -d '%')
        if [[ "$epic_coverage" == "N/A" ]] || [[ "$cov_num" -ge 80 ]]; then
            status_icon="${GREEN}✓${NC}"
        elif [[ "$cov_num" -ge 70 ]]; then
            status_icon="${YELLOW}⚠${NC}"
        else
            status_icon="${RED}✗${NC}"
        fi

        printf "  %s %-13s %-40s %10s %8s\n" "$status_icon" "$epic_id" "$title" "$feature_count" "$epic_coverage"

    done <<< "$epic_files"

    echo ""
}

##############################################################################
# Remediation Steps
##############################################################################

show_remediation() {
    local coverage="$1"

    echo ""
    echo "Remediation Steps:"
    echo "=================="
    echo ""
    echo "  Current coverage: ${coverage}%"
    echo "  Required for pass: ${PASS_THRESHOLD}%"
    echo ""
    echo "  Suggested actions:"
    echo "    1. Run /validate-epic-coverage to identify gaps"
    echo "    2. Run /create-missing-stories EPIC-ID to create stories for gaps"
    echo "    3. Review orphaned stories and reassign to correct epics"
    echo "    4. Consider splitting large epics into smaller ones"
    echo ""
}

##############################################################################
# Main Entry Point
##############################################################################

main() {
    local coverage=""
    local epic_dir="$DEFAULT_EPIC_DIR"
    local story_dir="$DEFAULT_STORY_DIR"
    local config_file=""
    local phase="sprint-planning"
    local use_color=true
    local verbose=false

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --coverage)
                coverage="$2"
                shift 2
                ;;
            --epic-dir)
                epic_dir="$2"
                shift 2
                ;;
            --story-dir)
                story_dir="$2"
                shift 2
                ;;
            --config)
                config_file="$2"
                shift 2
                ;;
            --phase)
                phase="$2"
                shift 2
                ;;
            --color)
                use_color=true
                shift
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
                shift
                ;;
        esac
    done

    # Load custom thresholds if provided
    if [[ -n "$config_file" ]]; then
        load_thresholds "$config_file"
    fi

    # Display phase info
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Coverage Quality Gate - Sprint Planning Phase"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    # Calculate or use provided coverage
    if [[ -z "$coverage" ]]; then
        coverage=$(calculate_coverage "$epic_dir" "$story_dir")
    fi

    if [[ -z "$coverage" ]] || [[ "$coverage" == "" ]]; then
        coverage=0
    fi

    # Show breakdown
    if [[ "$verbose" == "true" ]] || [[ -z "$2" ]]; then
        generate_epic_breakdown "$epic_dir" "$story_dir"
    fi

    # Evaluate gate
    local status
    status=$(evaluate_gate "$coverage")
    local exit_code=$?

    echo ""
    echo "Gate Result:"
    echo "============"
    echo ""

    case "$status" in
        PASS)
            log_pass "Coverage ${coverage}% meets threshold (>= ${PASS_THRESHOLD}%)"
            echo ""
            echo "Workflow may proceed."
            ;;
        WARN)
            log_warn "Coverage ${coverage}% is below optimal (70-80%)"
            echo ""
            echo "Workflow may proceed with warnings."
            echo "Consider addressing coverage gaps before next sprint."
            ;;
        BLOCK)
            log_block "Coverage ${coverage}% is below minimum (< ${WARN_THRESHOLD}%)"
            echo ""
            echo "Workflow BLOCKED. Coverage must be improved before proceeding."
            show_remediation "$coverage"
            ;;
    esac

    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    return $exit_code
}

# Run if not sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
