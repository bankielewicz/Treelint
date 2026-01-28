#!/bin/bash
#
# Requirements Traceability Matrix Parser
# STORY-083: Parse epic/story files and generate traceability matrix
#
# Usage:
#   ./parse-requirements.sh [command] [options]
#
# Commands:
#   --extract-epic-id FILE       Extract epic_id from file
#   --extract-epic-title FILE    Extract title from epic file
#   --extract-features FILE      Extract features section
#   --count-features FILE        Count features in epic
#   --extract-stories-table FILE Extract Stories table
#   --list-epic-stories FILE     List story IDs from epic
#   --extract-story-id-from-filename FILENAME  Extract STORY-NNN
#   --extract-story-epic FILE    Extract epic: field from story
#   --extract-story-title FILE   Extract title from story
#   --validate-epic-reference REF Validate epic reference format
#   --validate-story-id ID       Validate story ID format
#   --validate-epic-exists ID    Check if epic file exists
#   --parse-all-epics            Parse all epics, output JSON
#   --parse-all-stories          Parse all stories, output JSON
#   --detect-orphans             Detect orphaned stories
#   --generate-matrix            Generate full requirements matrix
#   --run-validation             Run validation checks
#   --incremental-update         Update matrix for changed files only
#

set -euo pipefail

# Constants
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_FILE="${SCRIPT_DIR}/config.json"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

# Load configuration
if [ -f "$CONFIG_FILE" ]; then
    EPIC_ID_PATTERN=$(jq -r '.epic_id_pattern' "$CONFIG_FILE")
    STORY_ID_PATTERN=$(jq -r '.story_id_pattern' "$CONFIG_FILE")
    EPICS_DIR="${PROJECT_ROOT}/$(jq -r '.epics_dir' "$CONFIG_FILE")"
    STORIES_DIR="${PROJECT_ROOT}/$(jq -r '.stories_dir' "$CONFIG_FILE")"
    OUTPUT_FILE="${PROJECT_ROOT}/$(jq -r '.output_file' "$CONFIG_FILE")"
else
    # Defaults if config not found
    EPIC_ID_PATTERN="^EPIC-[0-9]{3}$"
    STORY_ID_PATTERN="^STORY-[0-9]{3}$"
    EPICS_DIR="${PROJECT_ROOT}/devforgeai/specs/Epics"
    STORIES_DIR="${PROJECT_ROOT}/devforgeai/specs/Stories"
    OUTPUT_FILE="${PROJECT_ROOT}/devforgeai/traceability/requirements-matrix.json"
fi

#############################################################################
# UTILITY FUNCTIONS
#############################################################################

# Validate file path is within allowed directories (security)
validate_path() {
    local file="$1"

    # Check file exists
    if [ ! -f "$file" ]; then
        return 1
    fi

    local real_path
    real_path=$(realpath "$file" 2>/dev/null) || return 1

    # Must be within project root
    if [[ "$real_path" != "${PROJECT_ROOT}"* ]]; then
        echo "ERROR: Path traversal detected: $file" >&2
        return 1
    fi

    return 0
}

# Check if file is empty
is_file_empty() {
    local file="$1"
    [ ! -s "$file" ]
}

#############################################################################
# PARSING FUNCTIONS
#############################################################################

# Extract YAML frontmatter from file
# Returns: YAML content between --- delimiters
# Handles both LF and CRLF line endings
extract_frontmatter() {
    local file="$1"

    if ! validate_path "$file"; then
        return 1
    fi

    if is_file_empty "$file"; then
        return 1
    fi

    # Check if file has frontmatter delimiters (handle CRLF)
    local first_line
    first_line=$(head -1 "$file" | tr -d '\r')
    if [ "$first_line" != "---" ]; then
        return 1
    fi

    # Extract content between first two --- lines, handling CRLF
    # First strip CR, then extract frontmatter
    tr -d '\r' < "$file" | sed -n '2,/^---$/p' | sed '$d' 2>/dev/null
}

# Extract specific field from frontmatter
# Args: file, field_name
# Returns: field value
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

# Extract epic_id from file (handles both epic_id: and id: formats)
extract_epic_id() {
    local file="$1"

    if ! validate_path "$file"; then
        return 1
    fi

    if is_file_empty "$file"; then
        return 1
    fi

    local epic_id

    # Try epic_id: field first
    epic_id=$(extract_frontmatter_field "$file" "epic_id" 2>/dev/null) || true

    # Fallback to id: field
    if [ -z "$epic_id" ]; then
        epic_id=$(extract_frontmatter_field "$file" "id" 2>/dev/null) || true
    fi

    # Validate format
    if [ -z "$epic_id" ]; then
        return 1
    fi

    if [[ "$epic_id" =~ ^EPIC-[0-9]{3}$ ]]; then
        echo "$epic_id"
        return 0
    else
        return 1
    fi
}

# Extract story_id from filename
# Args: filename (not full path)
# Returns: STORY-NNN
extract_story_id_from_filename() {
    local filename="$1"

    # Pattern: STORY-NNN-slug.story.md
    local story_id
    story_id=$(echo "$filename" | grep -oE 'STORY-[0-9]{3}' | head -1)

    if [ -n "$story_id" ]; then
        echo "$story_id"
        return 0
    else
        return 1
    fi
}

# Extract epic reference from story file
# Returns: EPIC-NNN, None, or empty (for missing)
extract_story_epic_ref() {
    local file="$1"

    if ! validate_path "$file"; then
        return 2
    fi

    if is_file_empty "$file"; then
        return 2
    fi

    local epic_ref
    epic_ref=$(extract_frontmatter_field "$file" "epic" 2>/dev/null) || true

    if [ -z "$epic_ref" ]; then
        # Missing epic: field - return code 2
        return 2
    fi

    echo "$epic_ref"
    return 0
}

# Extract features section from epic file
extract_features_section() {
    local file="$1"

    if ! validate_path "$file"; then
        return 1
    fi

    if is_file_empty "$file"; then
        return 1
    fi

    # Extract content between ## Features and next ## header
    # Handle CRLF by stripping CR first
    tr -d '\r' < "$file" | awk '/^## Features/,/^## [^F]/' | sed '1d' | sed '$d' 2>/dev/null
}

# Count features in epic (### Feature N headers)
count_features() {
    local file="$1"

    if ! validate_path "$file"; then
        echo "0"
        return 1
    fi

    if is_file_empty "$file"; then
        echo "0"
        return 1
    fi

    # Handle CRLF by stripping CR first
    local count
    count=$(tr -d '\r' < "$file" | grep -c "^### Feature" 2>/dev/null) || count=0
    echo "$count"
}

# Extract Stories table from epic
extract_stories_table() {
    local file="$1"

    if ! validate_path "$file"; then
        return 1
    fi

    if is_file_empty "$file"; then
        return 1
    fi

    # Extract content from ## Stories to end of file (Stories is typically last section)
    # Handle CRLF - use awk ,0 to match until EOF
    tr -d '\r' < "$file" | awk '/^## Stories$/,0' | tail -n +2 2>/dev/null
}

# List story IDs from epic's Stories table
list_epic_stories() {
    local file="$1"

    local table_content
    table_content=$(extract_stories_table "$file") || return 1

    if [ -z "$table_content" ]; then
        return 1
    fi

    # Extract STORY-NNN patterns from table
    echo "$table_content" | grep -oE 'STORY-[0-9]{3}' | sort -u
}

#############################################################################
# VALIDATION FUNCTIONS
#############################################################################

# Validate epic reference format
validate_epic_reference() {
    local ref="$1"

    if [ "$ref" = "None" ]; then
        echo "standalone"
        return 0
    elif [[ "$ref" =~ ^EPIC-[0-9]{3}$ ]]; then
        echo "valid"
        return 0
    elif [[ "$ref" =~ ^STORY- ]]; then
        echo "invalid"  # References story instead of epic
        return 1
    else
        echo "invalid"
        return 1
    fi
}

# Validate story ID format
validate_story_id() {
    local id="$1"

    if [[ "$id" =~ ^STORY-[0-9]{3}$ ]]; then
        return 0
    else
        return 1
    fi
}

# Check if epic exists
validate_epic_exists() {
    local epic_id="$1"

    local epic_files
    epic_files=$(find "$EPICS_DIR" -name "${epic_id}-*.epic.md" -type f 2>/dev/null | head -1)

    if [ -n "$epic_files" ]; then
        echo "exists"
        return 0
    else
        echo "not_found"
        return 1
    fi
}

#############################################################################
# BATCH PROCESSING FUNCTIONS
#############################################################################

# Parse all epic files and return JSON
parse_all_epics() {
    local epics_json="{}"

    if [ ! -d "$EPICS_DIR" ]; then
        echo '{"epics": {}}'
        return 0
    fi

    for epic_file in "$EPICS_DIR"/*.epic.md; do
        [ -f "$epic_file" ] || continue

        local epic_id title features_count file_path linked_stories

        epic_id=$(extract_epic_id "$epic_file" 2>/dev/null) || continue

        title=$(extract_frontmatter_field "$epic_file" "title" 2>/dev/null) || title="Unknown"
        features_count=$(count_features "$epic_file" 2>/dev/null) || features_count=0
        file_path=$(realpath --relative-to="$PROJECT_ROOT" "$epic_file" 2>/dev/null) || file_path="$epic_file"

        # Extract linked stories from Stories table
        linked_stories=$(list_epic_stories "$epic_file" 2>/dev/null | jq -R . | jq -s . 2>/dev/null) || linked_stories="[]"

        epics_json=$(echo "$epics_json" | jq --arg id "$epic_id" \
            --arg title "$title" \
            --argjson features "$features_count" \
            --arg path "$file_path" \
            --argjson stories "$linked_stories" \
            '.[$id] = {title: $title, features_count: $features, file_path: $path, linked_stories: $stories}')
    done

    echo "$epics_json" | jq '{epics: .}'
}

# Parse all story files and return JSON
parse_all_stories() {
    local stories_json="{}"

    if [ ! -d "$STORIES_DIR" ]; then
        echo '{"stories": {}}'
        return 0
    fi

    for story_file in "$STORIES_DIR"/*.story.md; do
        [ -f "$story_file" ] || continue

        local filename story_id title epic_ref status file_path

        filename=$(basename "$story_file")
        story_id=$(extract_story_id_from_filename "$filename" 2>/dev/null) || continue

        title=$(extract_frontmatter_field "$story_file" "title" 2>/dev/null) || title="Unknown"
        epic_ref=$(extract_story_epic_ref "$story_file" 2>/dev/null) || epic_ref=""
        status=$(extract_frontmatter_field "$story_file" "status" 2>/dev/null) || status="Unknown"
        file_path=$(realpath --relative-to="$PROJECT_ROOT" "$story_file" 2>/dev/null) || file_path="$story_file"

        stories_json=$(echo "$stories_json" | jq --arg id "$story_id" \
            --arg title "$title" \
            --arg epic "$epic_ref" \
            --arg status "$status" \
            --arg path "$file_path" \
            '.[$id] = {title: $title, epic_ref: $epic, status: $status, file_path: $path}')
    done

    echo "$stories_json" | jq '{stories: .}'
}

# Detect orphaned stories
detect_orphans() {
    local stories_json
    stories_json=$(parse_all_stories | jq '.stories')

    local intentionally_standalone="[]"
    local broken_references="[]"
    local missing_metadata="[]"

    for story_id in $(echo "$stories_json" | jq -r 'keys[]' 2>/dev/null); do
        local epic_ref
        epic_ref=$(echo "$stories_json" | jq -r --arg id "$story_id" '.[$id].epic_ref')

        if [ -z "$epic_ref" ] || [ "$epic_ref" = "null" ] || [ "$epic_ref" = "" ]; then
            # Missing epic: field
            missing_metadata=$(echo "$missing_metadata" | jq --arg id "$story_id" '. + [$id]')
        elif [ "$epic_ref" = "None" ]; then
            # Intentionally standalone
            intentionally_standalone=$(echo "$intentionally_standalone" | jq --arg id "$story_id" '. + [$id]')
        else
            # Check if epic exists
            if ! validate_epic_exists "$epic_ref" >/dev/null 2>&1; then
                broken_references=$(echo "$broken_references" | jq --arg id "$story_id" '. + [$id]')
            fi
        fi
    done

    local standalone_count broken_count missing_count total
    standalone_count=$(echo "$intentionally_standalone" | jq 'length')
    broken_count=$(echo "$broken_references" | jq 'length')
    missing_count=$(echo "$missing_metadata" | jq 'length')
    total=$((standalone_count + broken_count + missing_count))

    jq -n --argjson standalone "$intentionally_standalone" \
          --argjson broken "$broken_references" \
          --argjson missing "$missing_metadata" \
          --argjson total "$total" \
          '{intentionally_standalone: $standalone, broken_references: $broken, missing_metadata: $missing, summary: {total_orphans: $total}}'
}

# Generate full requirements matrix
generate_matrix() {
    local epics_json stories_json orphans_json
    epics_json=$(parse_all_epics | jq '.epics')
    stories_json=$(parse_all_stories | jq '.stories')
    orphans_json=$(detect_orphans)

    local timestamp
    timestamp=$(date -Iseconds 2>/dev/null || date "+%Y-%m-%dT%H:%M:%S")

    local matrix
    matrix=$(jq -n \
        --argjson epics "$epics_json" \
        --argjson stories "$stories_json" \
        --argjson validation "$orphans_json" \
        --arg version "1.0.0" \
        --arg timestamp "$timestamp" \
        '{
            version: $version,
            generated_at: $timestamp,
            epics: $epics,
            stories: $stories,
            validation: $validation
        }')

    # Ensure output directory exists
    mkdir -p "$(dirname "$OUTPUT_FILE")"

    # Write with consistent formatting (for idempotency)
    echo "$matrix" | jq -S '.' > "$OUTPUT_FILE"

    echo "$matrix"
}

# Check for duplicate story IDs
check_duplicate_stories() {
    if [ ! -d "$STORIES_DIR" ]; then
        return 0
    fi

    # Find duplicate STORY-NNN prefixes
    local duplicates
    duplicates=$(ls "$STORIES_DIR"/*.story.md 2>/dev/null | xargs -I{} basename {} | \
        grep -oE 'STORY-[0-9]{3}' | sort | uniq -d)

    if [ -n "$duplicates" ]; then
        echo "$duplicates"
        return 1
    fi
    return 0
}

# Check bidirectional reference
check_bidirectional() {
    local story_id="$1"
    local epic_id="$2"

    local epic_file
    epic_file=$(find "$EPICS_DIR" -name "${epic_id}-*.epic.md" 2>/dev/null | head -1)

    if [ -z "$epic_file" ] || [ ! -f "$epic_file" ]; then
        echo "epic_not_found"
        return 1
    fi

    if grep -q "$story_id" "$epic_file" 2>/dev/null; then
        echo "bidirectional"
        return 0
    else
        echo "unidirectional"
        return 0
    fi
}

#############################################################################
# MAIN ENTRY POINT
#############################################################################

main() {
    local command="${1:-}"
    shift || true

    case "$command" in
        --extract-epic-id)
            extract_epic_id "$1"
            ;;
        --extract-epic-title)
            extract_frontmatter_field "$1" "title"
            ;;
        --extract-features)
            extract_features_section "$1"
            ;;
        --count-features)
            count_features "$1"
            ;;
        --extract-stories-table)
            extract_stories_table "$1"
            ;;
        --list-epic-stories)
            list_epic_stories "$1"
            ;;
        --extract-story-id-from-filename)
            extract_story_id_from_filename "$1"
            ;;
        --extract-story-epic)
            extract_story_epic_ref "$1"
            ;;
        --extract-story-title)
            extract_frontmatter_field "$1" "title"
            ;;
        --validate-epic-reference)
            validate_epic_reference "$1"
            ;;
        --validate-story-id)
            validate_story_id "$1"
            ;;
        --validate-epic-exists)
            validate_epic_exists "$1"
            ;;
        --parse-all-epics)
            parse_all_epics
            ;;
        --parse-all-stories)
            parse_all_stories
            ;;
        --detect-orphans)
            detect_orphans
            ;;
        --generate-matrix)
            generate_matrix >/dev/null
            echo "Matrix generated: $OUTPUT_FILE"
            ;;
        --run-validation)
            generate_matrix
            ;;
        --incremental-update)
            # For MVP: just regenerate (optimization in refactor)
            generate_matrix >/dev/null
            ;;
        --check-duplicate-stories)
            check_duplicate_stories
            ;;
        --check-bidirectional)
            check_bidirectional "$1" "$2"
            ;;
        *)
            echo "Usage: $0 <command> [options]"
            echo ""
            echo "Commands:"
            echo "  --extract-epic-id FILE           Extract epic_id from file"
            echo "  --extract-epic-title FILE        Extract title from epic file"
            echo "  --extract-features FILE          Extract features section"
            echo "  --count-features FILE            Count features in epic"
            echo "  --extract-stories-table FILE     Extract Stories table"
            echo "  --list-epic-stories FILE         List story IDs from epic"
            echo "  --extract-story-id-from-filename FILE Extract STORY-NNN"
            echo "  --extract-story-epic FILE        Extract epic: field from story"
            echo "  --parse-all-epics                Parse all epics"
            echo "  --parse-all-stories              Parse all stories"
            echo "  --detect-orphans                 Detect orphaned stories"
            echo "  --generate-matrix                Generate requirements matrix"
            echo "  --run-validation                 Run validation checks"
            exit 1
            ;;
    esac
}

main "$@"
