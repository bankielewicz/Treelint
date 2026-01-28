#!/usr/bin/env bash

################################################################################
# STORY-086: Coverage Reporting System
# Generates coverage reports in terminal, markdown, and JSON formats
# with historical tracking of epic-to-story coverage metrics
################################################################################

set -eo pipefail

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EPICS_DIR="${EPICS_DIR:-devforgeai/specs/Epics}"
STORIES_DIR="${STORIES_DIR:-devforgeai/specs/Stories}"
REPORTS_DIR="${REPORTS_DIR:-${SCRIPT_DIR}/reports}"
HISTORY_DIR="${HISTORY_DIR:-${SCRIPT_DIR}/history}"
HISTORY_FILE="${HISTORY_DIR}/coverage-history.json"
ENABLE_HISTORY="${ENABLE_HISTORY:-false}"
OUTPUT_FORMAT="${OUTPUT_FORMAT:-terminal}"

# Color codes (ANSI)
GREEN='\033[32m'
YELLOW='\033[33m'
RED='\033[31m'
RESET='\033[0m'

# Global arrays for coverage data
declare -A epic_ids
declare -A epic_titles
declare -A epic_priorities
declare -A epic_features
declare -A epic_linked_features
declare -A epic_features_list  # Store features as pipe-delimited string
declare -A story_epics

################################################################################
# Helper Functions
################################################################################

# Log error message
error() {
    echo "ERROR: $*" >&2
    exit 1
}

# Log warning message
warn() {
    echo "WARNING: $*" >&2
}

# Validate epic ID format
is_valid_epic_id() {
    local id="$1"
    [[ "$id" =~ ^EPIC-[0-9]{3}$ ]]
}

# Validate story ID format
is_valid_story_id() {
    local id="$1"
    [[ "$id" =~ ^STORY-[0-9]{3}$ ]]
}

# Get color code for percentage
get_color_for_percentage() {
    local percentage="$1"
    local bc_result

    # Check for 100% (green)
    bc_result=$(echo "$percentage == 100.0" | bc -l 2>/dev/null || echo "0")
    if [[ "$bc_result" -eq 1 ]]; then
        echo "$GREEN"
        return
    fi

    # Check for >= 50% (yellow)
    bc_result=$(echo "$percentage >= 50" | bc -l 2>/dev/null || echo "0")
    if [[ "$bc_result" -eq 1 ]]; then
        echo "$YELLOW"
        return
    fi

    # < 50% (red)
    echo "$RED"
}

# Parse YAML frontmatter value
parse_yaml_value() {
    local file="$1"
    local key="$2"
    grep "^${key}:" "$file" 2>/dev/null | sed "s/^${key}:\s*//" | tr -d '\r' | head -1 || echo ""
}

# Extract features from epic markdown
parse_epic_features() {
    local file="$1"
    local in_features=0
    local features=()

    while IFS= read -r line; do
        line=$(echo "$line" | tr -d '\r')

        # Detect Features section
        if [[ "$line" =~ ^##\ Features ]]; then
            in_features=1
            continue
        fi

        # Stop if we hit another major section (## but not ### or ## Features)
        if [[ "$in_features" == "1" ]] && [[ "$line" =~ ^##\  ]] && ! [[ "$line" =~ ^###\  ]] && ! [[ "$line" =~ ^##\ Features ]]; then
            break
        fi

        # Extract feature lines - match explicit story references
        # Supports: bullets, headers, any format with (STORY-XXX), (No story), or (Pending)
        if [[ "$in_features" == "1" ]]; then
            if [[ "$line" =~ \(STORY-[0-9]+\) ]] || \
               [[ "$line" =~ \(No\ story\) ]] || \
               [[ "$line" =~ \(Pending\) ]]; then
                # Line has explicit story reference - it's a feature
                features+=("$line")
            elif [[ "$line" =~ ^###\ Feature\ [0-9]+\.[0-9]+: ]]; then
                # Legacy format: "### Feature X.X: Name" without story ref in header
                local feature_name
                feature_name=$(echo "$line" | sed 's/^### Feature [0-9]*\.[0-9]*: //')
                features+=("- $feature_name")
            fi
        fi
    done < "$file"

    printf '%s\n' "${features[@]}"
}

################################################################################
# Phase 1: Parse Epics
################################################################################

parse_epics() {
    local epic_count=0

    if [[ ! -d "$EPICS_DIR" ]]; then
        return 0
    fi

    for epic_file in "$EPICS_DIR"/EPIC-*.epic.md; do
        [[ ! -f "$epic_file" ]] && continue

        local epic_id
        epic_id=$(parse_yaml_value "$epic_file" "id")
        epic_id=$(echo "$epic_id" | tr -d '[:space:]')

        if ! is_valid_epic_id "$epic_id"; then
            warn "Skipping epic file with invalid ID: $epic_file"
            continue
        fi

        local title
        title=$(parse_yaml_value "$epic_file" "title")

        local priority
        priority=$(parse_yaml_value "$epic_file" "priority")
        priority="${priority:=Medium}"

        # Parse features
        local features_list
        features_list=$(parse_epic_features "$epic_file")
        local feature_count=0
        local -a features_array=()

        while IFS= read -r feature_line; do
            [[ -z "$feature_line" ]] && continue
            # Extract feature name (everything before parentheses)
            local feature_name
            feature_name=$(echo "$feature_line" | sed 's/^-\s*//; s/\s*([^)]*).*$//')
            features_array+=("$feature_name")
            ((feature_count++)) || true
        done <<< "$features_list"

        # Store epic metadata
        epic_ids["$epic_id"]=1
        epic_titles["$epic_id"]="$title"
        epic_priorities["$epic_id"]="$priority"
        epic_features["$epic_id"]="$feature_count"
        epic_linked_features["$epic_id"]="0"

        # Store features as pipe-delimited string (simpler than eval)
        local features_string=""
        for feat in "${features_array[@]}"; do
            if [[ -n "$features_string" ]]; then
                features_string+="|$feat"
            else
                features_string="$feat"
            fi
        done
        epic_features_list["$epic_id"]="$features_string"

        ((epic_count++)) || true
    done
}

################################################################################
# Phase 2: Parse Stories
################################################################################

parse_stories() {
    if [[ ! -d "$STORIES_DIR" ]]; then
        return 0
    fi

    for story_file in "$STORIES_DIR"/STORY-*.md; do
        [[ ! -f "$story_file" ]] && continue

        local story_id
        story_id=$(parse_yaml_value "$story_file" "id")
        story_id=$(echo "$story_id" | tr -d '[:space:]')

        if ! is_valid_story_id "$story_id"; then
            continue
        fi

        local epic_id
        epic_id=$(parse_yaml_value "$story_file" "epic")
        epic_id=$(echo "$epic_id" | tr -d '[:space:]')

        [[ -z "$epic_id" ]] || [[ "$epic_id" == "null" ]] && continue

        if ! is_valid_epic_id "$epic_id"; then
            continue
        fi

        # Store story-to-epic mapping
        story_epics["$story_id"]="$epic_id"
    done
}

################################################################################
# Phase 3: Calculate Statistics
################################################################################

calculate_statistics() {
    local total_epics=0
    local total_features=0
    local total_linked=0
    local overall_percentage="0.0"

    # Count total epics and features
    for epic_id in "${!epic_ids[@]}"; do
        local feature_count="${epic_features[$epic_id]}"
        if [[ $feature_count -gt 0 ]]; then
            ((total_epics++)) || true
            ((total_features += feature_count)) || true
        fi
    done

    # Count linked features by matching story epic claims with feature names
    for story_id in "${!story_epics[@]}"; do
        local story_epic="${story_epics[$story_id]}"

        if [[ -v epic_ids["$story_epic"] ]]; then
            # Check if epic has features using new associative array
            if [[ -n "${epic_features_list[$story_epic]:-}" ]]; then
                local current_linked="${epic_linked_features[$story_epic]:-0}"
                local max_features="${epic_features[$story_epic]:-0}"
                # Only count if we haven't already reached feature count for this epic
                if [[ "$current_linked" -lt "$max_features" ]]; then
                    ((total_linked++)) || true
                    epic_linked_features["$story_epic"]=$((current_linked + 1))
                fi
            fi
        fi
    done

    # Cap total_linked at total_features (can't have more coverage than features)
    if [[ "$total_linked" -gt "$total_features" ]]; then
        total_linked="$total_features"
    fi

    # Calculate overall coverage percentage
    if [[ "$total_features" -gt 0 ]]; then
        overall_percentage=$(echo "scale=1; ($total_linked * 100) / $total_features" | bc -l 2>/dev/null || echo "0.0")
    fi

    # Return as JSON
    cat <<EOF
{
  "total_epics": $total_epics,
  "total_features": $total_features,
  "total_linked": $total_linked,
  "missing_count": $((total_features - total_linked)),
  "overall_coverage_percent": $overall_percentage
}
EOF
}

################################################################################
# Phase 4: Generate Terminal Output (AC#1)
################################################################################

generate_terminal_output() {
    local stats_json
    stats_json=$(calculate_statistics)

    local total_epics
    total_epics=$(echo "$stats_json" | jq -r '.total_epics')
    local overall_percent
    overall_percent=$(echo "$stats_json" | jq -r '.overall_coverage_percent')

    if [[ "$total_epics" -eq 0 ]]; then
        echo "No epics found"
        return 0
    fi

    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Epic Coverage Report"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    # Per-epic breakdown
    for epic_id in $(printf '%s\n' "${!epic_ids[@]}" | sort); do
        local feature_count="${epic_features[$epic_id]}"

        # Skip empty epics
        if [[ "$feature_count" -eq 0 ]]; then
            continue
        fi

        # Count stories linked to this epic
        local linked_count=0
        for story_id in "${!story_epics[@]}"; do
            if [[ "${story_epics[$story_id]}" == "$epic_id" ]]; then
                ((linked_count++)) || true
            fi
        done

        # Cap linked_count at feature_count (can't have more coverage than features)
        if [[ "$linked_count" -gt "$feature_count" ]]; then
            linked_count="$feature_count"
        fi

        local percentage=0.0
        if [[ "$feature_count" -gt 0 ]]; then
            percentage=$(echo "scale=1; ($linked_count * 100) / $feature_count" | bc -l 2>/dev/null || echo "0.0")
        fi

        local title="${epic_titles[$epic_id]}"
        local color
        color=$(get_color_for_percentage "$percentage")

        echo -ne "${color}"
        printf "  %-10s  %-40s  %6.1f%%\n" "$epic_id" "$title" "$percentage"
        echo -ne "${RESET}"
    done

    echo ""

    # Summary line
    local summary_color
    summary_color=$(get_color_for_percentage "$overall_percent")
    echo -ne "${summary_color}"
    echo "  Overall Coverage: $overall_percent%"
    echo -ne "${RESET}"
    echo ""
}

################################################################################
# Phase 5: Generate Markdown Report (AC#2)
################################################################################

generate_markdown_report() {
    local timestamp
    timestamp=$(date -u +"%Y-%m-%d-%H-%M-%S")

    local report_file="${REPORTS_DIR}/${timestamp}.md"

    # Create reports directory if needed
    mkdir -p "$REPORTS_DIR"

    local stats_json
    stats_json=$(calculate_statistics)

    local total_epics
    total_epics=$(echo "$stats_json" | jq -r '.total_epics')
    local total_features
    total_features=$(echo "$stats_json" | jq -r '.total_features')
    local overall_percent
    overall_percent=$(echo "$stats_json" | jq -r '.overall_coverage_percent')

    {
        echo "# Coverage Report"
        echo ""
        echo "**Generated:** $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
        echo ""
        echo "## Summary Statistics"
        echo ""
        echo "| Metric | Value |"
        echo "|--------|-------|"
        echo "| Total Epics | $total_epics |"
        echo "| Total Features | $total_features |"
        echo "| Overall Coverage | $overall_percent% |"
        echo "| Missing Stories | $((total_features - $(echo "$stats_json" | jq -r '.total_linked'))) |"
        echo ""
        echo "## Per-Epic Breakdown"
        echo ""
        echo "| Epic ID | Title | Features | Coverage |"
        echo "|---------|-------|----------|----------|"

        for epic_id in $(printf '%s\n' "${!epic_ids[@]}" | sort); do
            local feature_count="${epic_features[$epic_id]}"

            if [[ "$feature_count" -eq 0 ]]; then
                continue
            fi

            # Count stories linked to this epic
            local linked_count=0
            for story_id in "${!story_epics[@]}"; do
                if [[ "${story_epics[$story_id]}" == "$epic_id" ]]; then
                    ((linked_count++)) || true
                fi
            done

            # Cap linked_count at feature_count (can't have more coverage than features)
            if [[ "$linked_count" -gt "$feature_count" ]]; then
                linked_count="$feature_count"
            fi

            local percentage=0.0
            if [[ "$feature_count" -gt 0 ]]; then
                percentage=$(echo "scale=1; ($linked_count * 100) / $feature_count" | bc -l 2>/dev/null || echo "0.0")
            fi

            local title="${epic_titles[$epic_id]}"
            echo "| $epic_id | $title | $feature_count | $percentage% |"
        done

        echo ""
        echo "## Actionable Next Steps"
        echo ""

        local actions
        actions=$(generate_actionable_steps)

        if [[ -z "$actions" ]]; then
            echo "All epics at 100% coverage! 🎉"
        else
            # Format each line as markdown list item
            echo "$actions" | while read -r line; do
                echo "- \`$line\`"
            done
        fi
    } > "$report_file"

    echo "$report_file"
}

################################################################################
# Phase 6: Generate JSON Export (AC#3)
################################################################################

generate_json_export() {
    local stats_json
    stats_json=$(calculate_statistics)

    local total_epics
    total_epics=$(echo "$stats_json" | jq -r '.total_epics')
    local total_features
    total_features=$(echo "$stats_json" | jq -r '.total_features')
    local overall_percent
    overall_percent=$(echo "$stats_json" | jq -r '.overall_coverage_percent')
    local missing_count
    missing_count=$(echo "$stats_json" | jq -r '.missing_count')

    # Build epics array
    local epics_json="["
    local first=1

    for epic_id in $(printf '%s\n' "${!epic_ids[@]}" | sort); do
        local feature_count="${epic_features[$epic_id]}"

        if [[ "$feature_count" -eq 0 ]]; then
            continue
        fi

        # Count stories linked to this epic
        local linked_count=0
        for story_id in "${!story_epics[@]}"; do
            if [[ "${story_epics[$story_id]}" == "$epic_id" ]]; then
                ((linked_count++)) || true
            fi
        done

        # Cap linked_count at feature_count (can't have more coverage than features)
        if [[ "$linked_count" -gt "$feature_count" ]]; then
            linked_count="$feature_count"
        fi

        local percentage=0.0
        if [[ "$feature_count" -gt 0 ]]; then
            percentage=$(echo "scale=1; ($linked_count * 100) / $feature_count" | bc -l 2>/dev/null || echo "0.0")
        fi

        local title="${epic_titles[$epic_id]}"

        # Build missing features array from pipe-delimited string
        # Features beyond the linked_count are considered "missing"
        local features_str="${epic_features_list[$epic_id]:-}"
        local missing_features="["
        local missing_first=1

        if [[ -n "$features_str" ]]; then
            # Split pipe-delimited string into array
            IFS='|' read -ra features <<< "$features_str"
            local feature_index=0
            for feature in "${features[@]}"; do
                # Features at index >= linked_count are missing
                # (first N features are covered by N stories)
                if [[ "$feature_index" -ge "$linked_count" ]]; then
                    if [[ "$missing_first" -eq 0 ]]; then
                        missing_features+=","
                    fi
                    # Escape double quotes in feature name for valid JSON
                    local escaped_feature="${feature//\"/\\\"}"
                    missing_features+="\"$escaped_feature\""
                    missing_first=0
                fi
                ((feature_index++)) || true
            done
        fi

        missing_features+="]"

        if [[ "$first" -eq 0 ]]; then
            epics_json+=","
        fi

        epics_json+="{\"epic_id\":\"$epic_id\",\"title\":\"$title\",\"completion_percent\":$percentage,\"missing_features\":$missing_features}"
        first=0
    done

    epics_json+="]"

    # Build actionable steps array
    local actions_output
    actions_output=$(generate_actionable_steps)
    local actions_json
    if [[ -n "$actions_output" ]]; then
        actions_json=$(echo "$actions_output" | jq -Rs 'split("\n") | map(select(length > 0))')
    else
        actions_json="[]"
    fi

    # Build final JSON
    local generated_at
    generated_at=$(date -u '+%Y-%m-%dT%H:%M:%SZ')

    cat <<EOF
{
  "summary": {
    "total_epics": $total_epics,
    "total_features": $total_features,
    "overall_coverage_percent": $overall_percent,
    "missing_stories_count": $missing_count
  },
  "epics": $epics_json,
  "actionable_next_steps": $actions_json,
  "generated_at": "$generated_at",
  "format_version": "1.0"
}
EOF
}

################################################################################
# Phase 7: Generate Actionable Next Steps (AC#6)
################################################################################

generate_actionable_steps() {
    local -a actions=()

    # Extract all missing features across all epics
    for epic_id in $(printf '%s\n' "${!epic_ids[@]}" | sort); do
        local features_str="${epic_features_list[$epic_id]:-}"
        local feature_count="${epic_features[$epic_id]:-0}"
        if [[ -n "$features_str" ]]; then
            # Count stories linked to this epic
            local linked_count=0
            for story_id in "${!story_epics[@]}"; do
                if [[ "${story_epics[$story_id]}" == "$epic_id" ]]; then
                    ((linked_count++)) || true
                fi
            done

            # Cap linked_count at feature_count (can't have more coverage than features)
            if [[ "$linked_count" -gt "$feature_count" ]]; then
                linked_count="$feature_count"
            fi

            # Split pipe-delimited string into array
            IFS='|' read -ra features <<< "$features_str"
            local feature_index=0

            for feature in "${features[@]}"; do
                # Features at index >= linked_count are missing
                if [[ "$feature_index" -ge "$linked_count" ]]; then
                    local priority="${epic_priorities[$epic_id]}"
                    actions+=("$priority|$epic_id|$feature")
                fi
                ((feature_index++)) || true
            done
        fi
    done

    # Sort by priority
    IFS=$'\n' sorted_actions=($(sort -t'|' -k1,1 -k2,2 <<<"${actions[*]}"))
    unset IFS

    # Limit to 10 and format
    local count=0
    for action_line in "${sorted_actions[@]}"; do
        if [[ "$count" -ge 10 ]]; then
            break
        fi

        IFS='|' read -r priority epic_id feature <<< "$action_line"
        echo "/create-story --epic=$epic_id \"$feature\""

        count=$((count + 1))
    done
}

################################################################################
# Phase 8: Persist History (AC#7)
################################################################################

persist_history() {
    if [[ "$ENABLE_HISTORY" != "true" ]]; then
        return 0
    fi

    mkdir -p "$HISTORY_DIR"

    local stats_json
    stats_json=$(calculate_statistics)

    local total_epics
    total_epics=$(echo "$stats_json" | jq -r '.total_epics')
    local total_features
    total_features=$(echo "$stats_json" | jq -r '.total_features')
    local overall_percent
    overall_percent=$(echo "$stats_json" | jq -r '.overall_coverage_percent')
    local missing_count
    missing_count=$(echo "$stats_json" | jq -r '.missing_count')

    local timestamp
    timestamp=$(date -u '+%Y-%m-%dT%H:%M:%SZ')

    local new_entry="{\"timestamp\":\"$timestamp\",\"overall_coverage_percent\":$overall_percent,\"total_epics\":$total_epics,\"total_features\":$total_features,\"missing_count\":$missing_count}"

    local temp_file="${HISTORY_DIR}/.history.tmp"

    if [[ -f "$HISTORY_FILE" ]]; then
        # AC#7.10: Prevent duplicate entries for same timestamp
        if jq --arg ts "$timestamp" 'any(.[]; .timestamp == $ts)' "$HISTORY_FILE" 2>/dev/null | grep -q "true"; then
            # Idempotent: entry already exists, skip
            return 0
        fi

        # Append to existing history
        jq ". += [$new_entry]" "$HISTORY_FILE" > "$temp_file" 2>/dev/null || {
            echo "[$new_entry]" > "$temp_file"
        }
    else
        # Create new history file
        echo "[$new_entry]" > "$temp_file"
    fi

    # Atomic write
    mv "$temp_file" "$HISTORY_FILE"
}

################################################################################
# Main Execution
################################################################################

main() {
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --format=*)
                OUTPUT_FORMAT="${1#--format=}"
                ;;
            --epics-dir=*)
                EPICS_DIR="${1#--epics-dir=}"
                ;;
            --stories-dir=*)
                STORIES_DIR="${1#--stories-dir=}"
                ;;
            --reports-dir=*)
                REPORTS_DIR="${1#--reports-dir=}"
                ;;
            --history-dir=*)
                HISTORY_DIR="${1#--history-dir=}"
                HISTORY_FILE="${HISTORY_DIR}/coverage-history.json"
                ;;
            --enable-history)
                ENABLE_HISTORY="true"
                ;;
            --help|-h)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --format=FORMAT           Output format (terminal, markdown, json) [default: terminal]"
                echo "  --epics-dir=PATH          Path to epics directory [default: devforgeai/specs/Epics]"
                echo "  --stories-dir=PATH        Path to stories directory [default: devforgeai/specs/Stories]"
                echo "  --reports-dir=PATH        Path to reports directory [default: devforgeai/epic-coverage/reports]"
                echo "  --history-dir=PATH        Path to history directory [default: devforgeai/epic-coverage/history]"
                echo "  --enable-history          Enable history file persistence"
                echo "  --help                    Show this help message"
                exit 0
                ;;
            *)
                warn "Unknown option: $1"
                ;;
        esac
        shift
    done

    # Parse data
    parse_epics
    parse_stories

    # Generate output
    case "$OUTPUT_FORMAT" in
        terminal)
            generate_terminal_output
            ;;
        markdown)
            local report_file
            report_file=$(generate_markdown_report)
            echo "Report generated: $report_file"
            ;;
        json)
            generate_json_export
            ;;
        *)
            error "Unknown output format: $OUTPUT_FORMAT"
            ;;
    esac

    # Persist history if enabled
    persist_history
}

main "$@"
