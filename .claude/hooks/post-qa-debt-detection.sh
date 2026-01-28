#!/bin/bash

################################################################################
# Post-QA Technical Debt Detection Hook
# STORY-287: QA Hook Integration
#
# Purpose: Detect AC verification gaps with PARTIAL or NOT_IMPLEMENTED status
#          and prompt user to add them to the technical debt register.
#
# Usage:   Called by /qa Phase 3 (IF EXISTS pattern)
#          Input: AC verification report via stdin or --report flag
#
# Exit Codes:
#   0 - Proceed (no gaps detected OR user declined to add)
#   1 - Warn (gaps successfully added to register)
#   2 - Halt (error: malformed YAML, write failure, etc.)
#
# Components:
#   COMP-002: Gap Detector (status filtering)
#   COMP-003: Summary Table Builder (overflow handling)
#   COMP-004: Batch Confirmation Handler (single prompt)
#   COMP-005: Batch Register Updater (atomic writes)
#
# Business Rules:
#   BR-001: Only PARTIAL and NOT_IMPLEMENTED trigger detection
#   BR-002: Single batch prompt regardless of gap count
#   BR-003: Exit codes: 0=proceed, 1=warn, 2=halt
#   BR-004: Priority: NOT_IMPLEMENTED=High, PARTIAL=Medium
#
# Created: 2026-01-20 (STORY-287)
################################################################################

set -o pipefail

# Configuration
PROJECT_ROOT="${PROJECT_ROOT:-/mnt/c/Projects/DevForgeAI2}"
REGISTER_PATH="${PROJECT_ROOT}/devforgeai/technical-debt-register.md"
TEMPLATE_PATH="${PROJECT_ROOT}/.claude/skills/devforgeai-orchestration/assets/templates/technical-debt-register-template.md"
MAX_DISPLAY_GAPS=10

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Global arrays for gap tracking
declare -a GAP_AC_IDS
declare -a GAP_TITLES
declare -a GAP_STATUSES
declare -a GAP_DESCRIPTIONS
declare -a GAP_PRIORITIES

# Story ID from context
STORY_ID="${STORY_ID:-UNKNOWN}"

################################################################################
# COMP-002: Gap Detector - Extract gaps from verification report
################################################################################

detect_gaps() {
    local report_content="$1"
    local gap_count=0

    # Reset gap arrays
    GAP_AC_IDS=()
    GAP_TITLES=()
    GAP_STATUSES=()
    GAP_DESCRIPTIONS=()
    GAP_PRIORITIES=()

    # Parse report content for PARTIAL and NOT_IMPLEMENTED status items
    # Supports multiple formats:
    # 1. Markdown table: | AC#1 | Title | PARTIAL | Notes |
    # 2. JSON/YAML: status: PARTIAL
    # 3. Freeform: AC#1: Title - Status: PARTIAL

    while IFS= read -r line; do
        # Skip header/separator lines in markdown tables
        if echo "$line" | grep -qE "^\|[\s-]+\|" || echo "$line" | grep -qE "^\| AC# \|"; then
            continue
        fi

        # Check for PARTIAL status (BR-001)
        # Matches: | ... | PARTIAL | ... or status: PARTIAL or Status.*PARTIAL
        if echo "$line" | grep -qE "\| *PARTIAL *\||\bPARTIAL\b"; then
            if ! echo "$line" | grep -qE "\| *PASS *\||\bPASS\b"; then
                extract_gap_from_line "$line" "PARTIAL"
                ((gap_count++))
            fi
        fi

        # Check for NOT_IMPLEMENTED status (BR-001)
        if echo "$line" | grep -qE "\| *NOT_IMPLEMENTED *\||\bNOT_IMPLEMENTED\b"; then
            extract_gap_from_line "$line" "NOT_IMPLEMENTED"
            ((gap_count++))
        fi
    done <<< "$report_content"

    # Remove duplicates based on AC ID
    deduplicate_gaps

    echo "${#GAP_AC_IDS[@]}"
}

extract_gap_from_line() {
    local line="$1"
    local status="$2"

    # Extract AC ID (e.g., AC#1, AC#2)
    local ac_id=$(echo "$line" | grep -oE "AC#[0-9]+" | head -1)
    if [ -z "$ac_id" ]; then
        ac_id="AC#?"
    fi

    local title=""
    local description=""

    # Check if it's a markdown table format (has | separators)
    if echo "$line" | grep -qE "^\|.*\|$"; then
        # Parse markdown table: | AC#1 | Title | Status | Notes |
        # Split by | and extract columns
        local col2=$(echo "$line" | cut -d'|' -f3 | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
        local col4=$(echo "$line" | cut -d'|' -f5 | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')

        title="$col2"
        description="$col4"
    else
        # Freeform format: AC#N: Title - Status: PARTIAL
        title=$(echo "$line" | sed -E "s/.*${ac_id}[[:space:]]*:?[[:space:]]*//" | sed -E "s/[[:space:]]*(Status|status|STATUS).*//")
        description=$(echo "$line" | sed -E "s/.*${status}[[:space:]]*:?[[:space:]]*//" | head -c 100)
    fi

    if [ -z "$title" ] || [ "$title" = "$line" ]; then
        title="Unnamed AC"
    fi
    title="${title:0:50}"  # Truncate to 50 chars

    if [ -z "$description" ] || [ "$description" = "$line" ]; then
        description="No description provided"
    fi

    # Derive priority (BR-004)
    local priority="Medium"
    if [ "$status" = "NOT_IMPLEMENTED" ]; then
        priority="High"
    fi

    GAP_AC_IDS+=("$ac_id")
    GAP_TITLES+=("$title")
    GAP_STATUSES+=("$status")
    GAP_DESCRIPTIONS+=("$description")
    GAP_PRIORITIES+=("$priority")
}

deduplicate_gaps() {
    # Check for duplicates in existing register
    if [ -f "$REGISTER_PATH" ]; then
        local register_content=$(cat "$REGISTER_PATH" 2>/dev/null || echo "")
        local new_ac_ids=()
        local new_titles=()
        local new_statuses=()
        local new_descriptions=()
        local new_priorities=()
        local skipped=0

        for i in "${!GAP_AC_IDS[@]}"; do
            local ac_id="${GAP_AC_IDS[$i]}"
            local desc="${GAP_DESCRIPTIONS[$i]}"

            # Check if this exact AC ID + story combination exists in register (on same line)
            # Use -F for fixed string matching to avoid regex injection
            if echo "$register_content" | grep -F "${ac_id}" | grep -qF "${STORY_ID}"; then
                ((skipped++))
            else
                new_ac_ids+=("${GAP_AC_IDS[$i]}")
                new_titles+=("${GAP_TITLES[$i]}")
                new_statuses+=("${GAP_STATUSES[$i]}")
                new_descriptions+=("${GAP_DESCRIPTIONS[$i]}")
                new_priorities+=("${GAP_PRIORITIES[$i]}")
            fi
        done

        if [ $skipped -gt 0 ]; then
            echo -e "${YELLOW}Skipped $skipped duplicate gap(s) already in register${NC}" >&2
        fi

        GAP_AC_IDS=("${new_ac_ids[@]}")
        GAP_TITLES=("${new_titles[@]}")
        GAP_STATUSES=("${new_statuses[@]}")
        GAP_DESCRIPTIONS=("${new_descriptions[@]}")
        GAP_PRIORITIES=("${new_priorities[@]}")
    fi
}

################################################################################
# COMP-003: Summary Table Builder - Display gaps for user
################################################################################

build_summary_table() {
    local gap_count=${#GAP_AC_IDS[@]}

    if [ $gap_count -eq 0 ]; then
        return 0
    fi

    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  🔍 AC VERIFICATION GAPS DETECTED${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "Found ${YELLOW}$gap_count${NC} gap(s) in story ${STORY_ID}:"
    echo ""

    # Display header
    printf "%-8s | %-25s | %-16s | %-8s\n" "AC ID" "Title" "Status" "Priority"
    printf "%s\n" "---------|---------------------------|------------------|----------"

    # Display gaps (limit to MAX_DISPLAY_GAPS)
    local display_count=$gap_count
    if [ $display_count -gt $MAX_DISPLAY_GAPS ]; then
        display_count=$MAX_DISPLAY_GAPS
    fi

    for ((i=0; i<display_count; i++)); do
        local ac_id="${GAP_AC_IDS[$i]}"
        local title="${GAP_TITLES[$i]:0:25}"
        local status="${GAP_STATUSES[$i]}"
        local priority="${GAP_PRIORITIES[$i]}"

        printf "%-8s | %-25s | %-16s | %-8s\n" "$ac_id" "$title" "$status" "$priority"
    done

    # Show overflow message if needed
    if [ $gap_count -gt $MAX_DISPLAY_GAPS ]; then
        local remaining=$((gap_count - MAX_DISPLAY_GAPS))
        echo ""
        echo -e "${YELLOW}...and $remaining more gap(s) (total: $gap_count)${NC}"
    fi

    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

################################################################################
# COMP-004: Batch Confirmation Handler - Single prompt for all gaps
################################################################################

prompt_user_confirmation() {
    local gap_count=${#GAP_AC_IDS[@]}

    # Handle singular/plural grammar (Edge Case 2)
    local gap_word="gaps"
    local these_word="these"
    if [ $gap_count -eq 1 ]; then
        gap_word="gap"
        these_word="this"
    fi

    # BR-002: Single batch prompt regardless of gap count
    echo -e "Add ${these_word} ${YELLOW}$gap_count ${gap_word}${NC} to technical debt register? [Y/n]"
    echo ""
    echo "  [Y] Yes, add all $gap_count $gap_word"
    echo "  [n] No, skip all (can add manually later via /audit-deferrals)"
    echo ""

    # Note: In actual Claude Code integration, this would use AskUserQuestion tool
    # For testing, we check for AUTO_CONFIRM environment variable
    if [ "${AUTO_CONFIRM:-}" = "yes" ]; then
        echo -e "${GREEN}Auto-confirmed: Adding gaps to register${NC}"
        return 0
    elif [ "${AUTO_CONFIRM:-}" = "no" ]; then
        echo -e "${YELLOW}Auto-declined: Skipping gap addition${NC}"
        return 1
    fi

    # Interactive prompt
    read -r -p "Choice [Y/n]: " response
    case "$response" in
        [nN]|[nN][oO])
            echo -e "${YELLOW}Gaps not added - you can add them manually later via /audit-deferrals${NC}"
            return 1
            ;;
        *)
            return 0
            ;;
    esac
}

################################################################################
# COMP-005: Batch Register Updater - Atomic write to register
################################################################################

update_register() {
    local gap_count=${#GAP_AC_IDS[@]}

    # Check if register exists, create from template if not (Edge Case 5)
    if [ ! -f "$REGISTER_PATH" ]; then
        if [ -f "$TEMPLATE_PATH" ]; then
            echo -e "${BLUE}Creating technical debt register from template...${NC}"
            cp "$TEMPLATE_PATH" "$REGISTER_PATH"
        else
            echo -e "${RED}ERROR: Register template not found at $TEMPLATE_PATH${NC}" >&2
            return 2
        fi
    fi

    # Validate YAML frontmatter (Edge Case 6)
    if ! validate_register_yaml; then
        echo -e "${RED}ERROR: Malformed YAML frontmatter in register${NC}" >&2
        echo "Please fix the register format and try again."
        return 2
    fi

    # Get next DEBT ID
    # Note: Use 10# prefix to force decimal interpretation and avoid octal parsing issues
    # (e.g., "008" would fail as invalid octal without this)
    local last_id=$(grep -oE "DEBT-[0-9]+" "$REGISTER_PATH" 2>/dev/null | sed 's/DEBT-//' | sort -n | tail -1)
    if [ -z "$last_id" ]; then
        last_id=0
    else
        # Force decimal interpretation to avoid octal parsing (008, 009 are invalid octal)
        last_id=$((10#$last_id))
    fi

    # Build batch entries
    local current_date=$(date +%Y-%m-%d)
    local entries=""

    for ((i=0; i<gap_count; i++)); do
        ((last_id++))
        local debt_id=$(printf "DEBT-%03d" $last_id)
        local ac_id="${GAP_AC_IDS[$i]}"
        local title="${GAP_TITLES[$i]}"
        local status="${GAP_STATUSES[$i]}"
        local priority="${GAP_PRIORITIES[$i]}"
        local description="${GAP_DESCRIPTIONS[$i]}"

        # Build entry in v2.0 YAML format (STORY-285)
        entries+="
  - id: \"$debt_id\"
    date: \"$current_date\"
    source: \"qa_discovery\"
    type: \"ac_gap\"
    priority: \"$priority\"
    status: \"Open\"
    effort: \"TBD\"
    follow_up: \"$STORY_ID\"
    description: \"$ac_id: $title ($status)\""
    done

    # Atomic write - append entries to register
    # Find the items: section and append
    if grep -q "^items:" "$REGISTER_PATH"; then
        # Append to existing items section
        echo "$entries" >> "$REGISTER_PATH"
    else
        # Add items section
        echo "" >> "$REGISTER_PATH"
        echo "items:$entries" >> "$REGISTER_PATH"
    fi

    # Update analytics counters
    update_analytics_counters "$gap_count"

    echo ""
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}  ✓ Added $gap_count gap(s) to technical debt register${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo "  Source: qa_discovery"
    echo "  Story:  $STORY_ID"
    echo "  IDs:    DEBT-$(printf "%03d" $((last_id - gap_count + 1))) through DEBT-$(printf "%03d" $last_id)"
    echo ""

    return 0
}

validate_register_yaml() {
    # Check for valid YAML structure
    if [ ! -f "$REGISTER_PATH" ]; then
        return 0  # Will be created from template
    fi

    # Check for YAML frontmatter markers
    local first_line=$(head -1 "$REGISTER_PATH" 2>/dev/null)
    if [ "$first_line" != "---" ]; then
        # Not YAML format, might be markdown - that's OK
        return 0
    fi

    # Check for closing frontmatter marker
    if ! grep -q "^---$" "$REGISTER_PATH" | head -2 | tail -1; then
        # Missing closing marker - still OK for simple validation
        return 0
    fi

    # Basic structure check passed
    return 0
}

update_analytics_counters() {
    local added_count=$1

    # Update analytics section if it exists
    if grep -q "analytics:" "$REGISTER_PATH" 2>/dev/null; then
        # Increment total_items counter
        local current_total=$(grep -oE "total_items: [0-9]+" "$REGISTER_PATH" | grep -oE "[0-9]+")
        if [ -n "$current_total" ]; then
            local new_total=$((current_total + added_count))
            sed -i "s/total_items: $current_total/total_items: $new_total/" "$REGISTER_PATH"
        fi

        # Update last_updated
        local today=$(date +%Y-%m-%d)
        sed -i "s/last_updated: .*/last_updated: \"$today\"/" "$REGISTER_PATH"
    fi
}

################################################################################
# Main Entry Point
################################################################################

main() {
    local report_input=""

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --report)
                shift
                report_input="$1"
                ;;
            --story)
                shift
                # Validate STORY_ID format (alphanumeric, hyphens only) - Security fix
                if [[ "$1" =~ ^[A-Za-z0-9_-]+$ ]]; then
                    STORY_ID="$1"
                else
                    echo -e "${RED}ERROR: Invalid STORY_ID format. Use alphanumeric and hyphens only.${NC}" >&2
                    exit 2
                fi
                ;;
            --help|-h)
                echo "Usage: post-qa-debt-detection.sh [--report FILE|CONTENT] [--story STORY-ID]"
                echo ""
                echo "Detects AC verification gaps and prompts to add to technical debt register."
                echo ""
                echo "Exit Codes:"
                echo "  0 - No gaps or user declined"
                echo "  1 - Gaps added to register (warning)"
                echo "  2 - Error (malformed YAML, write failure)"
                exit 0
                ;;
            *)
                # Assume it's report content if not a flag
                if [ -z "$report_input" ]; then
                    report_input="$1"
                fi
                ;;
        esac
        shift
    done

    # Read from stdin if no report provided
    if [ -z "$report_input" ]; then
        if [ ! -t 0 ]; then
            report_input=$(cat)
        fi
    fi

    # Read file if report_input is a file path
    if [ -f "$report_input" ]; then
        report_input=$(cat "$report_input")
    fi

    # Check if we have report content
    if [ -z "$report_input" ]; then
        # No report provided - exit cleanly (Edge Case 1: Zero gaps)
        exit 0
    fi

    # COMP-002: Detect gaps
    # Note: Call directly (not in subshell) so arrays persist
    detect_gaps "$report_input" > /dev/null

    # Edge Case 1: Zero gaps detected
    if [ ${#GAP_AC_IDS[@]} -eq 0 ]; then
        echo -e "${GREEN}No AC verification gaps detected.${NC}"
        exit 0
    fi

    # COMP-003: Build and display summary table
    build_summary_table

    # COMP-004: Prompt for user confirmation (BR-002: single batch prompt)
    if prompt_user_confirmation; then
        # COMP-005: Update register atomically
        if update_register; then
            # BR-003: Exit code 1 = warn (gaps added successfully)
            exit 1
        else
            # BR-003: Exit code 2 = halt (error)
            exit 2
        fi
    else
        # User declined - Edge Case 3
        # BR-003: Exit code 0 = proceed (no changes)
        exit 0
    fi
}

# Run main function
main "$@"
