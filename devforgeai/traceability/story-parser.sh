#!/bin/bash
#
# Story Metadata Parser
# STORY-084: Parse story markdown files to extract metadata
#
# Usage:
#   ./story-parser.sh [command] [options]
#
# Commands:
#   --parse-frontmatter FILE    Parse all frontmatter fields, return JSON
#   --parse-story FILE          Full parse with validation, return JSON
#   --parse-all                 Parse all stories, return JSON array
#   --validate FILE             Validate story against schema
#   --help                      Show this help
#
# Output: JSON objects conforming to models/story.json schema

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
CONFIG_FILE="${SCRIPT_DIR}/config.json"

# Load configuration
if [ -f "$CONFIG_FILE" ]; then
    STORY_ID_PATTERN=$(jq -r '.story_id_pattern' "$CONFIG_FILE")
    STORIES_DIR="${PROJECT_ROOT}/$(jq -r '.stories_dir' "$CONFIG_FILE")"
else
    STORY_ID_PATTERN="^STORY-[0-9]{3}$"
    STORIES_DIR="${PROJECT_ROOT}/devforgeai/specs/Stories"
fi

# Valid Fibonacci points
VALID_POINTS="1 2 3 5 8 13 21"

# Valid story statuses
VALID_STATUSES="Backlog|Ready for Dev|In Development|Dev Complete|QA In Progress|QA Approved|QA Failed|Releasing|Released"

#############################################################################
# UTILITY FUNCTIONS
#############################################################################

validate_path() {
    local file="$1"

    if [ ! -f "$file" ]; then
        return 1
    fi

    local real_path
    real_path=$(realpath "$file" 2>/dev/null) || return 1

    if [[ "$real_path" != "${PROJECT_ROOT}"* ]]; then
        echo '{"error_type":"VALIDATION_ERROR","file_path":"'"$file"'","error_message":"Path traversal detected"}' >&2
        return 1
    fi

    return 0
}

is_file_empty() {
    local file="$1"
    [ ! -s "$file" ]
}

extract_frontmatter() {
    local file="$1"

    if ! validate_path "$file"; then
        return 1
    fi

    if is_file_empty "$file"; then
        return 1
    fi

    # Check for frontmatter (handle CRLF and BOM)
    local first_line
    first_line=$(head -1 "$file" | tr -d '\r' | sed 's/^\xEF\xBB\xBF//')
    if [ "$first_line" != "---" ]; then
        return 1
    fi

    sed 's/^\xEF\xBB\xBF//' "$file" | tr -d '\r' | sed -n '2,/^---$/p' | sed '$d' 2>/dev/null
}

extract_frontmatter_field() {
    local file="$1"
    local field="$2"

    local frontmatter
    frontmatter=$(extract_frontmatter "$file") || return 1

    if [ -z "$frontmatter" ]; then
        return 1
    fi

    local value
    value=$(echo "$frontmatter" | grep "^${field}:" | head -1 | sed "s/^${field}:[[:space:]]*//" | sed 's/^"//' | sed 's/"$//' | tr -d '\r')

    if [ -z "$value" ]; then
        return 1
    fi

    echo "$value"
}

# Extract story ID from filename (fallback)
extract_id_from_filename() {
    local file="$1"
    local filename
    filename=$(basename "$file")

    if [[ "$filename" =~ ^(STORY-[0-9]{3}) ]]; then
        echo "${BASH_REMATCH[1]}"
        return 0
    fi
    return 1
}

# Validate points is Fibonacci
is_fibonacci_points() {
    local points="$1"
    [[ " $VALID_POINTS " =~ " $points " ]]
}

# Validate status enum
is_valid_status() {
    local status="$1"
    echo "$status" | grep -qE "^($VALID_STATUSES)$"
}

# Coerce numeric epic to EPIC-NNN format (Edge case #8)
coerce_epic_reference() {
    local epic_ref="$1"

    # Handle None/null
    if [ "$epic_ref" = "None" ] || [ "$epic_ref" = "null" ] || [ -z "$epic_ref" ]; then
        echo "None"
        return 0
    fi

    # Check if already valid format
    if [[ "$epic_ref" =~ ^EPIC-[0-9]{3}$ ]]; then
        echo "$epic_ref"
        return 0
    fi

    # Check if numeric (Edge case #8: epic: 15)
    if [[ "$epic_ref" =~ ^[0-9]+$ ]]; then
        printf "EPIC-%03d" "$epic_ref"
        return 0
    fi

    # Invalid format
    echo "$epic_ref"
    return 1
}

#############################################################################
# STORY FRONTMATTER PARSING (AC#3)
#############################################################################

parse_story_frontmatter() {
    local file="$1"

    if ! validate_path "$file"; then
        echo '{"error_type":"VALIDATION_ERROR","file_path":"'"$file"'","error_message":"File not found or path traversal"}'
        return 1
    fi

    # Check for frontmatter
    local first_line
    first_line=$(head -1 "$file" | tr -d '\r' | sed 's/^\xEF\xBB\xBF//')
    if [ "$first_line" != "---" ]; then
        # Try fallback extraction
        local fallback_id
        fallback_id=$(extract_id_from_filename "$file") || fallback_id=""

        if [ -n "$fallback_id" ]; then
            echo '{"error_type":"MISSING_FRONTMATTER","file_path":"'"$file"'","error_message":"No YAML frontmatter found","needs_remediation":true,"extracted_id":"'"$fallback_id"'","recovery_suggestion":"Add YAML frontmatter with id: '"$fallback_id"'"}'
        else
            echo '{"error_type":"MISSING_FRONTMATTER","file_path":"'"$file"'","error_message":"No YAML frontmatter found","needs_remediation":true}'
        fi
        return 1
    fi

    # Extract ID (with filename fallback)
    local story_id
    story_id=$(extract_frontmatter_field "$file" "id" 2>/dev/null) || \
        story_id=$(extract_id_from_filename "$file" 2>/dev/null) || story_id=""

    if [ -z "$story_id" ]; then
        echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"id","error_message":"Missing id field and could not extract from filename"}'
        return 1
    fi

    # Validate story_id format
    if [[ ! "$story_id" =~ ^STORY-[0-9]{3}$ ]]; then
        echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"id","value":"'"$story_id"'","error_message":"Invalid story_id format, expected STORY-NNN"}'
        return 1
    fi

    # Extract title
    local title
    title=$(extract_frontmatter_field "$file" "title" 2>/dev/null) || title=""
    if [ -z "$title" ]; then
        echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"title","error_message":"Missing title field"}'
        return 1
    fi

    # Validate title length
    if [ ${#title} -gt 200 ]; then
        echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"title","error_message":"title exceeds 200 characters"}'
        return 1
    fi

    # Extract epic reference with coercion
    local epic_raw epic_ref epic_reference confidence
    epic_raw=$(extract_frontmatter_field "$file" "epic" 2>/dev/null) || epic_raw=""

    if [ -z "$epic_raw" ]; then
        # Check for implicit reference in file content
        local content
        content=$(cat "$file" | tr -d '\r')
        if echo "$content" | grep -qE "Related to EPIC-[0-9]{3}|EPIC-[0-9]{3}"; then
            epic_ref=$(echo "$content" | grep -oE "EPIC-[0-9]{3}" | head -1)
            epic_reference="implicit"
            confidence="low"
        else
            epic_ref="None"
            epic_reference="none"
            confidence="high"
        fi
    else
        epic_ref=$(coerce_epic_reference "$epic_raw") || {
            echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"epic","value":"'"$epic_raw"'","error_message":"Invalid epic reference format"}'
            return 1
        }
        epic_reference="explicit"
        confidence="high"
    fi

    # Extract sprint
    local sprint
    sprint=$(extract_frontmatter_field "$file" "sprint" 2>/dev/null) || sprint="Backlog"

    # Extract status
    local status
    status=$(extract_frontmatter_field "$file" "status" 2>/dev/null) || status="Backlog"

    # Validate status
    if ! is_valid_status "$status"; then
        echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"status","value":"'"$status"'","error_message":"invalid status, expected one of: Backlog, Ready for Dev, In Development, Dev Complete, QA In Progress, QA Approved, QA Failed, Releasing, Released"}'
        return 1
    fi

    # Extract points
    local points
    points=$(extract_frontmatter_field "$file" "points" 2>/dev/null) || points="0"

    # Validate points is Fibonacci (if not 0)
    if [ "$points" != "0" ] && ! is_fibonacci_points "$points"; then
        echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"points","value":"'"$points"'","error_message":"points must be Fibonacci (1, 2, 3, 5, 8, 13, 21)"}'
        return 1
    fi

    # Extract priority
    local priority
    priority=$(extract_frontmatter_field "$file" "priority" 2>/dev/null) || priority="Medium"

    # Extract other fields
    local assigned_to created completed updated format_version
    assigned_to=$(extract_frontmatter_field "$file" "assigned_to" 2>/dev/null) || assigned_to="Unassigned"
    created=$(extract_frontmatter_field "$file" "created" 2>/dev/null) || created=""
    completed=$(extract_frontmatter_field "$file" "completed" 2>/dev/null) || completed=""
    updated=$(extract_frontmatter_field "$file" "updated" 2>/dev/null) || updated=""
    format_version=$(extract_frontmatter_field "$file" "format_version" 2>/dev/null) || format_version="1.0"

    # Get relative file path
    local rel_path
    rel_path=$(realpath --relative-to="$PROJECT_ROOT" "$file" 2>/dev/null) || rel_path="$file"

    # Build JSON output
    jq -n \
        --arg id "$story_id" \
        --arg title "$title" \
        --arg epic "$epic_ref" \
        --arg sprint "$sprint" \
        --arg status "$status" \
        --argjson points "$points" \
        --arg priority "$priority" \
        --arg assigned "$assigned_to" \
        --arg created "$created" \
        --arg completed "$completed" \
        --arg updated "$updated" \
        --arg format_version "$format_version" \
        --arg file_path "$rel_path" \
        --arg epic_reference "$epic_reference" \
        --arg confidence "$confidence" \
        '{
            id: $id,
            title: $title,
            epic: (if $epic == "None" then null else $epic end),
            sprint: $sprint,
            status: $status,
            points: $points,
            priority: $priority,
            assigned_to: $assigned,
            created: (if $created == "" then null else $created end),
            completed: (if $completed == "" then null else $completed end),
            updated: (if $updated == "" then null else $updated end),
            format_version: $format_version,
            file_path: $file_path,
            epic_reference: $epic_reference,
            confidence: $confidence
        }'
}

#############################################################################
# BATCH PARSING
#############################################################################

parse_all_stories() {
    local stories_json='{"stories":{},"errors":[]}'

    for story_file in "$STORIES_DIR"/*.story.md; do
        [ -f "$story_file" ] || continue

        local result
        result=$(parse_story_frontmatter "$story_file" 2>/dev/null) || {
            stories_json=$(echo "$stories_json" | jq \
                --arg file "$story_file" \
                '.errors += [{"file_path": $file, "error": "Parse failed"}]')
            continue
        }

        # Check if result is an error
        if echo "$result" | jq -e '.error_type' >/dev/null 2>&1; then
            stories_json=$(echo "$stories_json" | jq \
                --argjson error "$result" \
                '.errors += [$error]')
        else
            local story_id
            story_id=$(echo "$result" | jq -r '.id')
            stories_json=$(echo "$stories_json" | jq \
                --arg id "$story_id" \
                --argjson data "$result" \
                '.stories[$id] = $data')
        fi
    done

    echo "$stories_json" | jq -S '.'
}

#############################################################################
# MAIN COMMAND HANDLER
#############################################################################

show_help() {
    cat << EOF
Story Metadata Parser - STORY-084

Usage: $0 [command] [options]

Commands:
  --parse-frontmatter FILE    Parse all frontmatter fields, return JSON
  --parse-story FILE          Full parse with validation, return JSON
  --parse-all                 Parse all stories, return JSON array
  --validate FILE             Validate story against schema
  --help                      Show this help

Examples:
  $0 --parse-frontmatter /path/to/STORY-084.story.md
  $0 --parse-all

Output: JSON objects conforming to models/story.json schema
EOF
}

case "${1:-}" in
    --parse-frontmatter|--parse-story)
        [ -n "${2:-}" ] || { echo '{"error":"Missing file argument"}'; exit 1; }
        parse_story_frontmatter "$2"
        ;;
    --parse-all)
        parse_all_stories
        ;;
    --validate)
        [ -n "${2:-}" ] || { echo '{"error":"Missing file argument"}'; exit 1; }
        result=$(parse_story_frontmatter "$2") && echo '{"valid":true}' || echo "$result"
        ;;
    --help|-h)
        show_help
        ;;
    *)
        echo '{"error":"Unknown command. Use --help for usage."}'
        exit 1
        ;;
esac
