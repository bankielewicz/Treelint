#!/bin/bash
#
# Epic Metadata Parser
# STORY-084: Parse epic markdown files to extract metadata and features
#
# Usage:
#   ./epic-parser.sh [command] [options]
#
# Commands:
#   --parse-frontmatter FILE    Parse all frontmatter fields, return JSON
#   --extract-features FILE     Extract features section, return JSON
#   --parse-epic FILE           Full parse (frontmatter + features), return JSON
#   --parse-all                 Parse all epics, return JSON array
#   --validate FILE             Validate epic against schema
#   --help                      Show this help
#
# Output: JSON objects conforming to models/epic.json schema

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
CONFIG_FILE="${SCRIPT_DIR}/config.json"
MODELS_DIR="${SCRIPT_DIR}/models"

# Load configuration
if [ -f "$CONFIG_FILE" ]; then
    EPIC_ID_PATTERN=$(jq -r '.epic_id_pattern' "$CONFIG_FILE")
    EPICS_DIR="${PROJECT_ROOT}/$(jq -r '.epics_dir' "$CONFIG_FILE")"
else
    EPIC_ID_PATTERN="^EPIC-[0-9]{3}$"
    EPICS_DIR="${PROJECT_ROOT}/devforgeai/specs/Epics"
fi

# Note: Utility functions are defined locally to avoid sourcing issues
# parse-requirements.sh is a standalone script with its own main()

#############################################################################
# UTILITY FUNCTIONS
#############################################################################

# Validate file path is within allowed directories (security)
validate_path() {
    local file="$1"

    if [ ! -f "$file" ]; then
        return 1
    fi

    local real_path
    real_path=$(realpath "$file" 2>/dev/null) || return 1

    # Must be within project root
    if [[ "$real_path" != "${PROJECT_ROOT}"* ]]; then
        echo '{"error_type":"VALIDATION_ERROR","file_path":"'"$file"'","error_message":"Path traversal detected"}' >&2
        return 1
    fi

    return 0
}

# Check if file is empty
is_file_empty() {
    local file="$1"
    [ ! -s "$file" ]
}

# Extract YAML frontmatter from file
extract_frontmatter() {
    local file="$1"

    if ! validate_path "$file"; then
        return 1
    fi

    if is_file_empty "$file"; then
        return 1
    fi

    # Check if file has frontmatter delimiters (handle CRLF and BOM)
    local first_line
    first_line=$(head -1 "$file" | tr -d '\r' | sed 's/^\xEF\xBB\xBF//')
    if [ "$first_line" != "---" ]; then
        return 1
    fi

    # Extract content between first two --- lines, handling CRLF and BOM
    # First strip BOM and CR, then extract frontmatter
    sed 's/^\xEF\xBB\xBF//' "$file" | tr -d '\r' | sed -n '2,/^---$/p' | sed '$d' 2>/dev/null
}

# Extract specific field from frontmatter
extract_frontmatter_field() {
    local file="$1"
    local field="$2"

    local frontmatter
    frontmatter=$(extract_frontmatter "$file") || return 1

    if [ -z "$frontmatter" ]; then
        return 1
    fi

    # Extract field value (handle both quoted and unquoted values)
    local value
    value=$(echo "$frontmatter" | grep "^${field}:" | head -1 | sed "s/^${field}:[[:space:]]*//" | sed 's/^"//' | sed 's/"$//' | tr -d '\r')

    if [ -z "$value" ]; then
        return 1
    fi

    echo "$value"
}

# Extract array field from frontmatter (like tags)
extract_array_field() {
    local file="$1"
    local field="$2"

    local frontmatter
    frontmatter=$(extract_frontmatter "$file") || return 1

    # Check if field exists in array format (indented with -)
    local array_values
    array_values=$(echo "$frontmatter" | awk "/^${field}:/,/^[a-z_]+:/" | grep "^[[:space:]]*-" | sed 's/^[[:space:]]*-[[:space:]]*//' | tr -d '\r')

    if [ -n "$array_values" ]; then
        # Convert to JSON array
        echo "$array_values" | jq -R . | jq -s .
    else
        echo "[]"
    fi
}

#############################################################################
# EPIC FRONTMATTER PARSING (AC#1)
#############################################################################

parse_epic_frontmatter() {
    local file="$1"

    if ! validate_path "$file"; then
        echo '{"error_type":"VALIDATION_ERROR","file_path":"'"$file"'","error_message":"File not found or path traversal"}'
        return 1
    fi

    # Check for frontmatter
    local first_line
    first_line=$(head -1 "$file" | tr -d '\r' | sed 's/^\xEF\xBB\xBF//')
    if [ "$first_line" != "---" ]; then
        echo '{"error_type":"MISSING_FRONTMATTER","file_path":"'"$file"'","error_message":"No YAML frontmatter found","needs_remediation":true}'
        return 1
    fi

    # Extract all fields
    local epic_id title status priority created complexity sprints

    epic_id=$(extract_frontmatter_field "$file" "epic_id" 2>/dev/null) || \
        epic_id=$(extract_frontmatter_field "$file" "id" 2>/dev/null) || epic_id=""

    # Validate epic_id format
    if [ -z "$epic_id" ]; then
        echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"epic_id","error_message":"Missing epic_id field"}'
        return 1
    fi

    if [[ ! "$epic_id" =~ ^EPIC-[0-9]{3}$ ]]; then
        echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"epic_id","value":"'"$epic_id"'","error_message":"Invalid epic_id format, expected EPIC-NNN"}'
        return 1
    fi

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

    status=$(extract_frontmatter_field "$file" "status" 2>/dev/null) || status="Planning"

    # Validate status enum
    if [[ ! "$status" =~ ^(Planning|In\ Progress|Complete|On\ Hold)$ ]]; then
        echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"status","value":"'"$status"'","error_message":"invalid status, expected Planning|In Progress|Complete|On Hold"}'
        return 1
    fi

    priority=$(extract_frontmatter_field "$file" "priority" 2>/dev/null) || priority="Medium"
    created=$(extract_frontmatter_field "$file" "created" 2>/dev/null) || created=""

    # Validate date format
    if [ -n "$created" ] && [[ ! "$created" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
        echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"created","value":"'"$created"'","error_message":"invalid date format, expected YYYY-MM-DD"}'
        return 1
    fi

    complexity=$(extract_frontmatter_field "$file" "complexity" 2>/dev/null) || complexity="0"
    sprints=$(extract_frontmatter_field "$file" "estimated_sprints" 2>/dev/null) || sprints="0"

    # Validate complexity is positive
    if [ "$complexity" != "0" ] && [ "$complexity" -lt 1 ] 2>/dev/null; then
        echo '{"error_type":"INVALID_FIELD","file_path":"'"$file"'","field":"complexity","value":"'"$complexity"'","error_message":"complexity must be positive"}'
        return 1
    fi

    # Extract tags array
    local tags
    tags=$(extract_array_field "$file" "tags" 2>/dev/null) || tags="[]"

    # Get relative file path
    local rel_path
    rel_path=$(realpath --relative-to="$PROJECT_ROOT" "$file" 2>/dev/null) || rel_path="$file"

    # Build JSON output
    jq -n \
        --arg epic_id "$epic_id" \
        --arg title "$title" \
        --arg status "$status" \
        --arg priority "$priority" \
        --arg created "$created" \
        --argjson complexity "${complexity:-0}" \
        --argjson sprints "${sprints:-0}" \
        --argjson tags "$tags" \
        --arg file_path "$rel_path" \
        '{
            epic_id: $epic_id,
            title: $title,
            status: $status,
            priority: $priority,
            created: $created,
            complexity: $complexity,
            estimated_sprints: $sprints,
            tags: $tags,
            file_path: $file_path
        }'
}

#############################################################################
# FEATURES SECTION EXTRACTION (AC#2)
#############################################################################

extract_features_section() {
    local file="$1"

    if ! validate_path "$file"; then
        echo '{"features":[],"features_count":0,"error":"File not found"}'
        return 1
    fi

    local features_json="[]"
    local features_count=0
    local has_empty_features=false

    # Extract content between ## Features and next ## header
    local features_content
    features_content=$(tr -d '\r' < "$file" | sed 's/^\xEF\xBB\xBF//' | awk '/^## Features/,/^## [^F]/' | sed '1d' | sed '$d' 2>/dev/null) || features_content=""

    if [ -z "$features_content" ]; then
        has_empty_features=true
        echo '{"features":[],"features_count":0,"has_empty_features":true}'
        return 0
    fi

    # Process features line by line
    local current_feature_id=""
    local current_title=""
    local current_description=""
    local current_dependencies=""
    local current_points=""
    local current_linked_story=""
    local in_description=false

    while IFS= read -r line; do
        # Check for feature header (### Feature N: Title or ### Feature N.N: Title)
        if [[ "$line" =~ ^###\ Feature\ ([0-9]+\.?[0-9]*):\ (.+)$ ]]; then
            # Save previous feature if exists
            if [ -n "$current_feature_id" ]; then
                features_json=$(echo "$features_json" | jq \
                    --arg id "$current_feature_id" \
                    --arg title "$current_title" \
                    --arg desc "$current_description" \
                    --arg deps "$current_dependencies" \
                    --arg points "$current_points" \
                    --arg linked "$current_linked_story" \
                    '. + [{
                        feature_id: $id,
                        title: $title,
                        description: ($desc | if . == "" then null else . end),
                        dependencies: ($deps | if . == "" then null else . end),
                        estimated_points: (if $points == "" then null else ($points | tonumber) end),
                        linked_story: ($linked | if . == "" then null else . end)
                    }]')
                features_count=$((features_count + 1))
            fi

            current_feature_id="${BASH_REMATCH[1]}"
            current_title="${BASH_REMATCH[2]}"
            current_description=""
            current_dependencies=""
            current_points=""
            current_linked_story=""
            in_description=true

            # Check for linked story in title (STORY-XXX)
            if [[ "$current_title" =~ \(STORY-([0-9]{3})\) ]]; then
                current_linked_story="STORY-${BASH_REMATCH[1]}"
                current_title="${current_title% (STORY-*)}"
            fi

        elif [[ "$line" =~ ^\*\*Description:\*\*\ (.+)$ ]]; then
            current_description="${BASH_REMATCH[1]}"
            in_description=false

        elif [[ "$line" =~ ^\*\*Dependencies:\*\*\ (.+)$ ]]; then
            current_dependencies="${BASH_REMATCH[1]}"
            in_description=false

        elif [[ "$line" =~ ^\*\*Estimated\ Points:\*\*\ ([0-9]+)$ ]]; then
            current_points="${BASH_REMATCH[1]}"
            in_description=false

        elif [[ "$line" =~ ^---$ ]]; then
            in_description=false

        elif [ "$in_description" = true ] && [ -n "$line" ] && [[ ! "$line" =~ ^\*\* ]]; then
            # Accumulate description text
            if [ -n "$current_description" ]; then
                current_description="${current_description} ${line}"
            else
                current_description="$line"
            fi
        fi
    done <<< "$features_content"

    # Save last feature
    if [ -n "$current_feature_id" ]; then
        features_json=$(echo "$features_json" | jq \
            --arg id "$current_feature_id" \
            --arg title "$current_title" \
            --arg desc "$current_description" \
            --arg deps "$current_dependencies" \
            --arg points "$current_points" \
            --arg linked "$current_linked_story" \
            '. + [{
                feature_id: $id,
                title: $title,
                description: ($desc | if . == "" then null else . end),
                dependencies: ($deps | if . == "" then null else . end),
                estimated_points: (if $points == "" then null else ($points | tonumber) end),
                linked_story: ($linked | if . == "" then null else . end)
            }]')
        features_count=$((features_count + 1))
    fi

    # Build final output
    jq -n \
        --argjson features "$features_json" \
        --argjson count "$features_count" \
        --argjson empty "$has_empty_features" \
        '{
            features: $features,
            features_count: $count,
            has_empty_features: $empty
        }'
}

#############################################################################
# FULL EPIC PARSING
#############################################################################

parse_epic() {
    local file="$1"

    local frontmatter
    frontmatter=$(parse_epic_frontmatter "$file") || {
        echo "$frontmatter"
        return 1
    }

    local features
    features=$(extract_features_section "$file") || features='{"features":[],"features_count":0}'

    # Merge frontmatter with features
    echo "$frontmatter" | jq \
        --argjson features "$(echo "$features" | jq '.features')" \
        --argjson features_count "$(echo "$features" | jq '.features_count')" \
        '. + {features: $features, features_count: $features_count}'
}

#############################################################################
# BATCH PARSING
#############################################################################

parse_all_epics() {
    local epics_json='{"epics":{},"errors":[]}'

    for epic_file in "$EPICS_DIR"/*.epic.md; do
        [ -f "$epic_file" ] || continue

        local result
        result=$(parse_epic "$epic_file" 2>/dev/null) || {
            # Capture error
            epics_json=$(echo "$epics_json" | jq \
                --arg file "$epic_file" \
                '.errors += [{"file_path": $file, "error": "Parse failed"}]')
            continue
        }

        # Check if result is an error
        if echo "$result" | jq -e '.error_type' >/dev/null 2>&1; then
            epics_json=$(echo "$epics_json" | jq \
                --argjson error "$result" \
                '.errors += [$error]')
        else
            local epic_id
            epic_id=$(echo "$result" | jq -r '.epic_id')
            epics_json=$(echo "$epics_json" | jq \
                --arg id "$epic_id" \
                --argjson data "$result" \
                '.epics[$id] = $data')
        fi
    done

    echo "$epics_json" | jq -S '.'
}

#############################################################################
# MAIN COMMAND HANDLER
#############################################################################

show_help() {
    cat << EOF
Epic Metadata Parser - STORY-084

Usage: $0 [command] [options]

Commands:
  --parse-frontmatter FILE    Parse all frontmatter fields, return JSON
  --extract-features FILE     Extract features section, return JSON
  --parse-epic FILE           Full parse (frontmatter + features), return JSON
  --parse-all                 Parse all epics, return JSON array
  --validate FILE             Validate epic against schema
  --help                      Show this help

Examples:
  $0 --parse-frontmatter /path/to/EPIC-015.epic.md
  $0 --extract-features /path/to/EPIC-015.epic.md
  $0 --parse-all

Output: JSON objects conforming to models/epic.json schema
EOF
}

# Handle --source-only flag for sourcing utility functions
if [[ "${1:-}" == "--source-only" ]]; then
    return 0 2>/dev/null || exit 0
fi

case "${1:-}" in
    --parse-frontmatter)
        [ -n "${2:-}" ] || { echo '{"error":"Missing file argument"}'; exit 1; }
        parse_epic_frontmatter "$2"
        ;;
    --extract-features)
        [ -n "${2:-}" ] || { echo '{"error":"Missing file argument"}'; exit 1; }
        extract_features_section "$2"
        ;;
    --parse-epic)
        [ -n "${2:-}" ] || { echo '{"error":"Missing file argument"}'; exit 1; }
        parse_epic "$2"
        ;;
    --parse-all)
        parse_all_epics
        ;;
    --validate)
        [ -n "${2:-}" ] || { echo '{"error":"Missing file argument"}'; exit 1; }
        result=$(parse_epic "$2") && echo '{"valid":true}' || echo "$result"
        ;;
    --help|-h)
        show_help
        ;;
    *)
        echo '{"error":"Unknown command. Use --help for usage."}'
        exit 1
        ;;
esac
