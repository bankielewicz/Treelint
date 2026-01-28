#!/bin/bash
#
# STORY-085: Gap Detection Engine
# Detects coverage gaps between epics and stories using multiple validation strategies
#
# Usage: gap-detector.sh [epic-id]
#        gap-detector.sh              # Analyze all epics
#        gap-detector.sh EPIC-015     # Analyze specific epic
#
# Environment Variables (for testing):
#   GAP_STORIES_DIR - Override stories directory
#   GAP_EPICS_DIR   - Override epics directory
#   GAP_OUTPUT_DIR  - Override output directory
#

# Only enable strict mode when running directly (not when sourced)
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    # Using set -u (nounset) and set -o pipefail but NOT set -e
    # set -e causes issues with loops that use grep which may return 1
    set -uo pipefail
fi

# ============================================================================
# CONFIGURATION - Directories can be overridden for testing
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="${GAP_REPO_ROOT:-$(cd "$SCRIPT_DIR/../.." && pwd)}"
STORIES_DIR="${GAP_STORIES_DIR:-$REPO_ROOT/devforgeai/specs/Stories}"
EPICS_DIR="${GAP_EPICS_DIR:-$REPO_ROOT/devforgeai/specs/Epics}"
OUTPUT_DIR="${GAP_OUTPUT_DIR:-$REPO_ROOT/devforgeai/traceability}"

# ============================================================================
# HELPER FUNCTIONS
# ============================================================================

# Normalize epic ID format (e.g., epic-7, EPIC-07, EPIC 007 → EPIC-007)
# BR-004: Epic ID normalization: uppercase, hyphen separator, 3-digit padding
normalize_epic_id() {
    local epic_id="$1"

    # Handle null/empty
    if [[ -z "$epic_id" ]] || [[ "$epic_id" == "null" ]] || [[ "$epic_id" == "None" ]]; then
        return 1
    fi

    # Convert to uppercase and replace spaces with hyphens
    epic_id="${epic_id^^}"
    epic_id="${epic_id// /-}"

    # Extract just the number if it has EPIC prefix
    if [[ $epic_id =~ EPIC-?([0-9]+) ]]; then
        local num="${BASH_REMATCH[1]}"
        # Remove leading zeros and re-pad to 3 digits
        num=$((10#$num))
        num=$(printf "%03d" "$num")
        echo "EPIC-$num"
        return 0
    else
        return 1
    fi
}

# Extract epic ID from story frontmatter
# Pattern: ^epic:\s*EPIC-\d{3}
extract_epic_from_story() {
    local story_file="$1"

    if [[ ! -f "$story_file" ]]; then
        return 1
    fi

    # Extract epic field from YAML frontmatter (handle Windows line endings)
    local epic_line
    epic_line=$(grep "^epic:" "$story_file" 2>/dev/null | head -1 | tr -d '\r' | sed 's/^epic:[[:space:]]*//' | tr -d '"' | xargs)

    # BR-005: Stories with epic: None or epic: null excluded
    if [[ -z "$epic_line" ]] || [[ "$epic_line" == "null" ]] || [[ "$epic_line" == "None" ]] || [[ "$epic_line" == '""' ]]; then
        return 1
    fi

    # Normalize and return
    normalize_epic_id "$epic_line"
}

# ============================================================================
# STRATEGY 1: Extract epic field from stories (AC#1)
# ============================================================================

# Build story-to-epic mapping from all story files
# Returns: Associative array name passed by reference
strategy1_extract_epics() {
    # Fix: Separate nameref declaration from assignment (Bash nameref pattern)
    local -n _story_epic_map
    _story_epic_map=$1
    local stories_dir="${2:-$STORIES_DIR}"

    local count=0

    # Enable nullglob for safe glob handling
    local saved_nullglob=$(shopt -p nullglob 2>/dev/null || echo "shopt -u nullglob")
    shopt -s nullglob

    local files=("$stories_dir"/STORY-*.story.md)

    # Check if glob matched anything
    if [[ ${#files[@]} -eq 0 ]]; then
        eval "$saved_nullglob"
        echo "0"
        return 0
    fi

    for story_file in "${files[@]}"; do
        [[ -f "$story_file" ]] || continue

        local story_id basename_file
        basename_file=$(basename "$story_file")

        # Extract STORY-NNN from filename
        if [[ "$basename_file" =~ STORY-([0-9]+) ]]; then
            story_id="STORY-${BASH_REMATCH[1]}"
        else
            continue
        fi

        # Skip if already processed (avoid duplicates)
        if [[ -v _story_epic_map[$story_id] ]]; then
            continue
        fi

        # Try to extract epic
        local epic
        if epic=$(extract_epic_from_story "$story_file"); then
            _story_epic_map["$story_id"]="$epic"
            ((count++))
        fi
    done

    eval "$saved_nullglob"
    echo "$count"
}

# ============================================================================
# STRATEGY 2: Parse epic Stories tables (AC#2)
# ============================================================================

# Helper: Check if line is Stories section header
is_stories_header() {
    local line="$1"
    [[ "$line" =~ ^##[[:space:]]+Stories ]]
}

# Helper: Check if line is next section header (ends Stories section)
is_next_section() {
    local line="$1"
    [[ "$line" =~ ^##[[:space:]][^#] ]]
}

# Helper: Check if line is table separator
is_table_separator() {
    local line="$1"
    [[ "$line" =~ \|[[:space:]]*[-]+[[:space:]]*\| ]]
}

# Helper: Check if line is table header
is_table_header() {
    local line="$1"
    [[ "$line" =~ Story.*ID.*Feature.*# ]] || [[ "$line" =~ Story[[:space:]]*ID[[:space:]]*\|[[:space:]]*Feature ]]
}

# Helper: Check if line is table data row
is_table_data_row() {
    local line="$1"
    [[ "$line" =~ ^\|.*\| ]]
}

# Helper: Parse single table row and extract story data
# Returns: feature_num|title|points|status (or empty if invalid)
parse_table_row() {
    local line="$1"

    # Split by pipe and extract fields
    local IFS='|'
    local -a fields
    read -ra fields <<< "$line"

    # Need at least 6 elements (empty + 5 columns + trailing)
    if [[ ${#fields[@]} -lt 6 ]]; then
        echo "WARNING: Malformed row (insufficient columns): $line" >&2
        return 1
    fi

    # Trim whitespace from fields (fields[0] is empty before first |)
    local story_id feature_num title points status
    story_id=$(echo "${fields[1]}" | xargs)
    feature_num=$(echo "${fields[2]}" | xargs)
    title=$(echo "${fields[3]}" | xargs)
    points=$(echo "${fields[4]}" | xargs)
    status=$(echo "${fields[5]}" | xargs)

    # Skip if story ID is empty
    if [[ -z "$story_id" ]]; then
        return 1
    fi

    # Normalize story ID format
    if [[ "$story_id" =~ STORY-([0-9]+) ]]; then
        story_id="STORY-${BASH_REMATCH[1]}"
    fi

    # Return normalized: story_id|feature_num|title|points|status
    echo "$story_id|$feature_num|$title|$points|$status"
}

# Parse the Stories table from an epic file
# Returns: Associative array with story details
parse_epic_stories_table() {
    local epic_file="$1"
    # Fix: Separate nameref declaration from assignment
    local -n _results
    _results=$2

    if [[ ! -f "$epic_file" ]]; then
        return 1
    fi

    local in_stories_section=0
    local table_started=0

    while IFS= read -r line || [[ -n "$line" ]]; do
        # Strip Windows carriage returns
        line="${line//$'\r'/}"

        # Check for ## Stories header
        if is_stories_header "$line"; then
            in_stories_section=1
            continue
        fi

        # Stop if we hit another ## header after Stories section
        if [[ "$in_stories_section" == 1 ]] && is_next_section "$line"; then
            break
        fi

        if [[ "$in_stories_section" == 0 ]]; then
            continue
        fi

        # Skip empty lines
        [[ -z "${line// /}" ]] && continue

        # Check for pipe-delimited table row
        if is_table_data_row "$line"; then
            # Skip separator row
            if is_table_separator "$line"; then
                continue
            fi

            # Skip header row
            if is_table_header "$line"; then
                table_started=1
                continue
            fi

            # Parse data row
            if [[ "$table_started" == 1 ]]; then
                local row_data
                row_data=$(parse_table_row "$line") || continue

                # Extract story_id from row_data
                local story_id="${row_data%%|*}"
                local feature_data="${row_data#*|}"  # Remove story_id prefix

                # Store result
                _results["$story_id"]="$feature_data"
            fi
        fi
    done < "$epic_file"

    # Don't echo count - let caller check ${#array[@]}
    return 0
}

# Build epic-to-stories mapping from all epic files
strategy2_parse_tables() {
    # Fix: Separate nameref declaration from assignment
    local -n _epic_stories_map
    _epic_stories_map=$1
    local epics_dir="${2:-$EPICS_DIR}"

    local total_entries=0
    for epic_file in "$epics_dir"/EPIC-*.epic.md "$epics_dir"/EPIC-*.md; do
        if [[ ! -f "$epic_file" ]]; then
            continue
        fi

        local epic_id basename_file
        basename_file=$(basename "$epic_file")

        # Extract EPIC-NNN from filename
        if [[ "$basename_file" =~ EPIC-([0-9]+) ]]; then
            epic_id=$(normalize_epic_id "EPIC-${BASH_REMATCH[1]}")
        else
            continue
        fi

        declare -A table_results=()
        # Call without command substitution to avoid subshell issues
        parse_epic_stories_table "$epic_file" table_results 2>/dev/null

        # Merge results into output map
        for story_id in "${!table_results[@]}"; do
            # Store as: epic_id:story_id -> feature_data
            _epic_stories_map["$epic_id:$story_id"]="${table_results[$story_id]}"
            ((total_entries++))
        done

        unset table_results
    done

    echo "$total_entries"
}

# ============================================================================
# STRATEGY 3: Cross-validate bidirectional consistency (AC#3)
# ============================================================================

# Helper: Calculate consistency score from match counts
calculate_consistency_score() {
    local matched="$1"
    local total="$2"

    # Calculate consistency score (BR-003)
    if [[ $total -gt 0 ]]; then
        local score
        score=$(echo "scale=1; ($matched * 100) / $total" | bc 2>/dev/null)

        # Ensure score has decimal point (bc might return "100" instead of "100.0")
        if [[ ! "$score" =~ \. ]]; then
            score="${score}.0"
        fi

        echo "$score"
    else
        echo "100.0"
    fi
}

strategy3_cross_validate() {
    # Fix: Separate nameref declaration from assignment
    local -n _story_epic_map
    _story_epic_map=$1
    local -n _epic_stories_map
    _epic_stories_map=$2
    local -n _mismatches
    _mismatches=$3

    local match_count=0
    local total_relationships=0

    # Check stories pointing to epics (story→epic direction)
    for story_id in "${!_story_epic_map[@]}"; do
        local claimed_epic="${_story_epic_map[$story_id]}"
        ((total_relationships++))

        # Look for corresponding epic→story link
        local found=0
        local key="$claimed_epic:$story_id"
        if [[ -v _epic_stories_map[$key] ]]; then
            found=1
            ((match_count++))
        fi

        if [[ $found -eq 0 ]]; then
            # Story claims epic but not in epic's table
            _mismatches["$story_id"]="NOT_IN_EPIC_TABLE:$claimed_epic"
        fi
    done

    # Check epics pointing to stories (epic→story direction)
    for epic_story_key in "${!_epic_stories_map[@]}"; do
        local epic_id="${epic_story_key%%:*}"
        local story_id="${epic_story_key#*:}"

        # Check if story exists and points back to this epic
        if [[ -v _story_epic_map[$story_id] ]]; then
            local story_claimed_epic="${_story_epic_map[$story_id]}"
            if [[ "$story_claimed_epic" != "$epic_id" ]]; then
                # Story points to different epic
                _mismatches["$story_id"]="BIDIRECTIONAL_MISMATCH:claims $story_claimed_epic but in $epic_id table"
            fi
        else
            # Epic table references story but story doesn't have epic field
            _mismatches["$story_id"]="MISSING_EPIC_FIELD:referenced in $epic_id"
        fi
    done

    # Return consistency score using helper
    calculate_consistency_score "$match_count" "$total_relationships"
}

# ============================================================================
# AC#4: Calculate completion percentage
# ============================================================================

# BR-001: Completion % = (matched_stories / total_features) * 100 rounded to 1 decimal
calculate_completion() {
    local total_features="$1"
    local matched_stories="$2"

    # Handle division by zero - return 0.0 not error
    if [[ $total_features -eq 0 ]]; then
        echo "0.0"
        return 0
    fi

    # Formula: (matched / total) * 100, rounded to 1 decimal
    local result
    result=$(echo "scale=1; ($matched_stories * 100) / $total_features" | bc 2>/dev/null || echo "0.0")
    echo "$result"
}

# ============================================================================
# AC#5: Find missing features
# ============================================================================

# Helper: Find story file with given ID
find_story_file() {
    local story_id="$1"
    local stories_dir="$2"

    # Try different filename patterns
    for pattern in "${story_id}.story.md" "${story_id}-*.story.md"; do
        for sf in "$stories_dir"/$pattern; do
            if [[ -f "$sf" ]]; then
                echo "$sf"
                return 0
            fi
        done
    done

    return 1
}

find_missing_features() {
    local epic_id="$1"
    # Fix: Separate nameref declaration from assignment
    local -n _epic_stories_map
    _epic_stories_map=$2
    local -n _story_epic_map
    _story_epic_map=$3
    local -n _missing_features
    _missing_features=$4
    local stories_dir="${5:-$STORIES_DIR}"

    for epic_story_key in "${!_epic_stories_map[@]}"; do
        local key_epic="${epic_story_key%%:*}"
        local story_id="${epic_story_key#*:}"

        # Only process for the specified epic
        if [[ "$key_epic" != "$epic_id" ]]; then
            continue
        fi

        local feature_data="${_epic_stories_map[$epic_story_key]}"
        local feature_num="${feature_data%%|*}"

        # Check if story file exists
        local story_file
        if story_file=$(find_story_file "$story_id" "$stories_dir"); then
            # Check if story has epic field pointing to this epic
            local story_epic
            if story_epic=$(extract_epic_from_story "$story_file" 2>/dev/null); then
                if [[ "$story_epic" != "$epic_id" ]]; then
                    _missing_features["$story_id"]="WRONG_EPIC:$feature_num:claims $story_epic"
                fi
            else
                _missing_features["$story_id"]="MISSING_EPIC_FIELD:$feature_num"
            fi
        else
            _missing_features["$story_id"]="MISSING_FILE:$feature_num"
        fi
    done
}

# ============================================================================
# AC#6: Find orphaned stories
# ============================================================================

# Helper: Find epic file with given ID
find_epic_file() {
    local epic_id="$1"
    local epics_dir="$2"

    # Try both naming patterns
    for ef in "$epics_dir"/${epic_id}*.epic.md "$epics_dir"/${epic_id}*.md; do
        if [[ -f "$ef" ]]; then
            echo "$ef"
            return 0
        fi
    done

    return 1
}

# BR-002: Orphan detection requires BOTH epic file existence AND Stories table entry check
find_orphaned_stories() {
    # Fix: Separate nameref declaration from assignment
    local -n _story_epic_map
    _story_epic_map=$1
    local -n _epic_stories_map
    _epic_stories_map=$2
    local -n _orphans
    _orphans=$3
    local epics_dir="${4:-$EPICS_DIR}"

    for story_id in "${!_story_epic_map[@]}"; do
        local claimed_epic="${_story_epic_map[$story_id]}"

        # Check if epic file exists
        if ! find_epic_file "$claimed_epic" "$epics_dir" > /dev/null 2>&1; then
            _orphans["$story_id"]="EPIC_NOT_FOUND:$claimed_epic"
            continue
        fi

        # Check if story is in epic's Stories table
        local key="$claimed_epic:$story_id"
        if [[ ! -v _epic_stories_map[$key] ]]; then
            _orphans["$story_id"]="NOT_IN_EPIC_TABLE:$claimed_epic"
        fi
    done
}

# ============================================================================
# AC#7: Generate consolidated report
# ============================================================================

# Helper: Build mismatches JSON array
build_mismatches_json() {
    local -n _mismatches_ref
    _mismatches_ref=$1

    local json="["
    local first=1
    for story_id in "${!_mismatches_ref[@]}"; do
        [[ $first -eq 0 ]] && json+=","
        first=0
        local reason="${_mismatches_ref[$story_id]}"
        json+="{\"story_id\": \"$story_id\", \"reason\": \"$reason\"}"
    done
    json+="]"
    echo "$json"
}

# Helper: Build orphaned stories JSON array
build_orphans_json() {
    local -n _orphans_ref
    _orphans_ref=$1

    local json="["
    local first=1
    for story_id in "${!_orphans_ref[@]}"; do
        [[ $first -eq 0 ]] && json+=","
        first=0
        local reason="${_orphans_ref[$story_id]}"
        local reason_code="${reason%%:*}"
        local claimed_epic="${reason#*:}"
        json+="{\"story_id\": \"$story_id\", \"reason\": \"$reason_code\", \"claimed_epic\": \"$claimed_epic\"}"
    done
    json+="]"
    echo "$json"
}

# Helper: Build recommendations JSON array
build_recommendations_json() {
    local -n _orphans_ref
    _orphans_ref=$1

    local json="["
    local first=1
    for story_id in "${!_orphans_ref[@]}"; do
        [[ $first -eq 0 ]] && json+=","
        first=0
        local reason="${_orphans_ref[$story_id]}"
        local reason_code="${reason%%:*}"
        local claimed_epic="${reason#*:}"
        if [[ "$reason_code" == "NOT_IN_EPIC_TABLE" ]]; then
            json+="\"Add $story_id to $claimed_epic Stories table\""
        elif [[ "$reason_code" == "EPIC_NOT_FOUND" ]]; then
            json+="\"Create epic file for $claimed_epic or update $story_id epic field\""
        fi
    done
    json+="]"
    echo "$json"
}

# Helper: Write JSON report to file
write_json_report() {
    local json="$1"
    local report_file="$2"

    # Format with jq if available, otherwise write raw
    if command -v jq &>/dev/null; then
        echo "$json" | jq '.' > "$report_file" 2>/dev/null || echo "$json" > "$report_file"
    else
        echo "$json" > "$report_file"
    fi
}

generate_report() {
    # Fix: Separate nameref declaration from assignment
    local -n _story_epic_map
    _story_epic_map=$1
    local -n _epic_stories_map
    _epic_stories_map=$2
    local -n _mismatches
    _mismatches=$3
    local -n _orphans
    _orphans=$4
    local consistency_score="$5"
    local output_dir="${6:-$OUTPUT_DIR}"

    local report_file="$output_dir/gap-detection-report.json"
    mkdir -p "$output_dir"

    # Build JSON components
    local mismatches_json
    mismatches_json=$(build_mismatches_json _mismatches)

    local orphans_json
    orphans_json=$(build_orphans_json _orphans)

    local recommendations_json
    recommendations_json=$(build_recommendations_json _orphans)

    # Build complete JSON report
    local json="{"
    json+="\"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\","
    json+="\"consistency_score\": $consistency_score,"
    json+="\"story_count\": ${#_story_epic_map[@]},"
    json+="\"epic_entry_count\": ${#_epic_stories_map[@]},"
    json+="\"mismatch_count\": ${#_mismatches[@]},"
    json+="\"orphan_count\": ${#_orphans[@]},"
    json+="\"mismatches\": $mismatches_json,"
    json+="\"orphaned_stories\": $orphans_json,"
    json+="\"recommendations\": $recommendations_json"
    json+="}"

    # Write report to file
    write_json_report "$json" "$report_file"

    echo "$report_file"
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

main() {
    local target_epic="${1:-}"

    declare -A story_epic_map=()
    declare -A epic_stories_map=()
    declare -A mismatches=()
    declare -A orphans=()

    echo "=== STORY-085: Gap Detection Engine ===" >&2
    echo "Analyzing repository for epic coverage gaps..." >&2
    echo "" >&2

    # Strategy 1: Extract epic fields from stories
    echo "[1/3] Running Strategy 1: Extracting epic fields from stories..." >&2
    strategy1_extract_epics story_epic_map > /dev/null
    local story_count=${#story_epic_map[@]}
    echo "  Found $story_count stories with epic links" >&2

    # Strategy 2: Parse epic Stories tables
    echo "[2/3] Running Strategy 2: Parsing epic Stories tables..." >&2
    strategy2_parse_tables epic_stories_map > /dev/null
    local entry_count=${#epic_stories_map[@]}
    echo "  Found $entry_count story entries in epic tables" >&2

    # Strategy 3: Cross-validate bidirectional links
    echo "[3/3] Running Strategy 3: Cross-validating bidirectional links..." >&2
    local consistency_score
    consistency_score=$(strategy3_cross_validate story_epic_map epic_stories_map mismatches)
    echo "  Consistency score: $consistency_score%" >&2

    # Find orphaned stories
    find_orphaned_stories story_epic_map epic_stories_map orphans
    echo "  Found ${#orphans[@]} orphaned stories" >&2

    # Generate consolidated report
    echo "" >&2
    local report_file
    report_file=$(generate_report story_epic_map epic_stories_map mismatches orphans "$consistency_score")
    echo "Report generated: $report_file" >&2

    # Output summary to stdout
    echo "{"
    echo "  \"consistency_score\": $consistency_score,"
    echo "  \"story_count\": ${#story_epic_map[@]},"
    echo "  \"epic_entry_count\": ${#epic_stories_map[@]},"
    echo "  \"orphan_count\": ${#orphans[@]},"
    echo "  \"report_file\": \"$report_file\""
    echo "}"

    return 0
}

# ============================================================================
# STORY-089: Confidence Scoring Integration (AC#5)
# ============================================================================

# Wrapper function for confidence scoring operations
# Delegates to confidence-scorer.sh module
run_with_confidence() {
    local args=("$@")
    local confidence_scorer="${SCRIPT_DIR}/confidence-scorer.sh"

    if [[ ! -f "$confidence_scorer" ]]; then
        echo "ERROR: confidence-scorer.sh not found" >&2
        return 3
    fi

    # Forward to confidence scorer
    bash "$confidence_scorer" "${args[@]}"
}

# Calculate coverage with confidence scoring
# BR-003: Low-confidence matches (60-75%) excluded from coverage
calculate_coverage_with_confidence() {
    local story_dir="${1:-$STORIES_DIR}"
    local epic_file="${2:-}"
    local confidence_scorer="${SCRIPT_DIR}/confidence-scorer.sh"

    if [[ ! -f "$confidence_scorer" ]]; then
        echo "ERROR: confidence-scorer.sh not found" >&2
        return 3
    fi

    if [[ -n "$epic_file" ]]; then
        bash "$confidence_scorer" --calculate-coverage --stories "$story_dir" --epic "$epic_file"
    else
        bash "$confidence_scorer" --calculate-coverage --stories "$story_dir" --epic-dir "$EPICS_DIR"
    fi
}

# Extended main with confidence scoring support
main_with_confidence() {
    local with_confidence=false
    local format=""
    local story=""
    local epic=""
    local calculate_cov=false
    local confidence_threshold=""
    local remaining_args=()

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --with-confidence)
                with_confidence=true
                shift
                ;;
            --format)
                format="$2"
                shift 2
                ;;
            --story)
                story="$2"
                shift 2
                ;;
            --stories)
                STORIES_DIR="$2"
                shift 2
                ;;
            --epic)
                epic="$2"
                shift 2
                ;;
            --epic-dir)
                EPICS_DIR="$2"
                shift 2
                ;;
            --calculate-coverage)
                calculate_cov=true
                shift
                ;;
            --confidence-threshold)
                confidence_threshold="$2"
                shift 2
                ;;
            --test-br-003)
                # Delegate BR-003 test to confidence scorer
                run_with_confidence --test-br-003
                return $?
                ;;
            *)
                remaining_args+=("$1")
                shift
                ;;
        esac
    done

    # If confidence scoring requested, delegate to confidence scorer
    if [[ "$with_confidence" == "true" ]]; then
        local args=()

        [[ -n "$story" ]] && args+=(--story "$story")
        [[ -n "$STORIES_DIR" ]] && args+=(--stories "$STORIES_DIR")
        [[ -n "$epic" ]] && args+=(--epic "$epic")
        [[ -n "$EPICS_DIR" ]] && args+=(--epic-dir "$EPICS_DIR")
        [[ -n "$format" ]] && args+=(--format "$format")
        [[ -n "$confidence_threshold" ]] && args+=(--confidence-threshold "$confidence_threshold")
        [[ "$calculate_cov" == "true" ]] && args+=(--calculate-coverage)

        run_with_confidence "${args[@]}"
        return $?
    fi

    # Calculate coverage mode
    if [[ "$calculate_cov" == "true" ]]; then
        local confidence_scorer="${SCRIPT_DIR}/confidence-scorer.sh"
        if [[ -f "$confidence_scorer" ]]; then
            # Use confidence scorer for coverage calculation
            local args=(--calculate-coverage)
            [[ -n "$story" ]] && args+=(--story "$story")
            [[ -n "$STORIES_DIR" ]] && args+=(--stories "$STORIES_DIR")
            [[ -n "$epic" ]] && args+=(--epic "$epic")
            [[ -n "$EPICS_DIR" ]] && args+=(--epic-dir "$EPICS_DIR")
            bash "$confidence_scorer" "${args[@]}"
            return $?
        else
            calculate_coverage_with_confidence "$STORIES_DIR" "$epic"
            return $?
        fi
    fi

    # Default: run standard gap detection
    main "${remaining_args[@]}"
}

# Run main if script is executed directly (not sourced)
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    # Check if any confidence-related flags are present
    if [[ "$*" == *"--with-confidence"* ]] || [[ "$*" == *"--test-br-003"* ]] || [[ "$*" == *"--calculate-coverage"* ]] || [[ "$*" == *"--confidence-threshold"* ]]; then
        main_with_confidence "$@"
    else
        main "$@"
    fi
fi
