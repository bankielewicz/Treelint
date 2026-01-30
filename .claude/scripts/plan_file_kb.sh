#!/bin/bash
# Plan File Knowledge Base Functions
# STORY-222: Extract Plan File Knowledge Base for Decision Archive
#
# Provides 4 functions for plan file parsing and decision archive building:
# - extract_yaml_frontmatter: Parse YAML frontmatter from plan files
# - extract_story_ids: Extract STORY-NNN patterns with context
# - build_decision_archive: Build bidirectional story<->plan mapping
# - query_archive: Query archive for related plan files

set -euo pipefail

# =============================================================================
# Helper Functions
# =============================================================================

# Validate that a file exists before processing
# Usage: validate_file_exists "$file_path"
# Returns: 0 if exists, 1 if not (also outputs JSON error)
validate_file_exists() {
    local file="$1"
    if [[ ! -f "$file" ]]; then
        echo '{"error": "File not found"}'
        return 1
    fi
    return 0
}

# Escape special characters for JSON output
# Usage: json_escape "$string"
# Handles: backslash, quotes, newlines, carriage returns, tabs
json_escape() {
    local str="$1"
    str="${str//\\/\\\\}"
    str="${str//\"/\\\"}"
    str="${str//$'\n'/\\n}"
    str="${str//$'\r'/\\r}"
    str="${str//$'\t'/\\t}"
    echo "$str"
}

# Build a JSON array from newline-separated lines
# Usage: json_array_from_lines < input_lines
# Example: echo -e "STORY-001\nSTORY-002" | json_array_from_lines
json_array_from_lines() {
    awk 'BEGIN{printf "["} NR>1{printf ", "} {printf "\"%s\"", $0} END{printf "]"}'
}

# Parse a specific YAML field from content
# Usage: parse_yaml_field "$yaml_content" "status"
# Returns: field value without quotes
parse_yaml_field() {
    local content="$1"
    local field_name="$2"
    echo "$content" | grep -E "^${field_name}:" | sed "s/^${field_name}:[[:space:]]*//" | tr -d '"' | tr -d "'" || echo ""
}

# =============================================================================
# Function 1: extract_yaml_frontmatter
# AC#1: Parse YAML frontmatter from plan files
# SM-010: Parse YAML frontmatter from plan files (Critical)
# =============================================================================
extract_yaml_frontmatter() {
    local plan_file="$1"

    validate_file_exists "$plan_file" || return 1

    # Check if file starts with ---
    local first_line
    first_line=$(head -n1 "$plan_file" 2>/dev/null || echo "")

    if [[ "$first_line" != "---" ]]; then
        # No YAML frontmatter - return empty/default
        echo '{"status": "", "created": "", "author": "", "related_stories": []}'
        return 0
    fi

    # Extract content between first and second ---
    local yaml_content
    yaml_content=$(awk 'BEGIN{found=0} /^---$/{found++; if(found==2) exit; next} found==1{print}' "$plan_file" 2>/dev/null || echo "")

    if [[ -z "$yaml_content" ]]; then
        echo '{"status": "", "created": "", "author": "", "related_stories": []}'
        return 0
    fi

    # Parse individual fields from YAML using helper
    local status
    status=$(parse_yaml_field "$yaml_content" "status")

    local created
    created=$(parse_yaml_field "$yaml_content" "created")

    local author
    author=$(parse_yaml_field "$yaml_content" "author")

    # Extract related_stories array
    local related_stories
    related_stories=$(parse_related_stories "$yaml_content")

    # Output JSON
    printf '{"status": "%s", "created": "%s", "author": "%s", "related_stories": %s}\n' \
        "$status" "$created" "$author" "$related_stories"
}

# Helper to parse related_stories field (handles inline and multiline formats)
parse_related_stories() {
    local yaml_content="$1"
    local related_stories_field
    related_stories_field=$(echo "$yaml_content" | grep -E "^related_stories:" || echo "")

    if [[ -z "$related_stories_field" ]]; then
        echo "[]"
        return 0
    fi

    # Try to extract STORY-XXX patterns from the field
    local stories_array
    stories_array=$(echo "$yaml_content" | grep -E "^related_stories:" | \
        sed 's/^related_stories:[[:space:]]*//' | \
        grep -oE 'STORY-[0-9]+' | \
        json_array_from_lines)

    # If no patterns found, return empty array
    if [[ -z "$stories_array" || "$stories_array" == "[]" ]]; then
        echo "[]"
    else
        echo "$stories_array"
    fi
}

# =============================================================================
# Function 2: extract_story_ids
# AC#2: Extract STORY-NNN patterns with surrounding context
# SM-011: Extract STORY-NNN patterns with regex (High)
# =============================================================================
extract_story_ids() {
    local plan_file="$1"

    validate_file_exists "$plan_file" || return 1

    # Extract all STORY-NNN patterns (3+ digits)
    local story_ids
    story_ids=$(grep -oE 'STORY-[0-9]{3,}' "$plan_file" 2>/dev/null | sort -u || echo "")

    if [[ -z "$story_ids" ]]; then
        echo '{"story_ids": [], "contexts": {}}'
        return 0
    fi

    # Build JSON array of story IDs using helper
    local ids_json
    ids_json=$(echo "$story_ids" | json_array_from_lines)

    # Build contexts object with surrounding text for each story ID
    local contexts_json="{"
    local is_first_id_entry=true

    while IFS= read -r story_id; do
        if [[ -z "$story_id" ]]; then continue; fi

        # Get line with context (extract the line containing the story ID)
        local context
        context=$(grep -m1 "$story_id" "$plan_file" 2>/dev/null | head -c 200 | tr '\n' ' ' | tr '"' "'" || echo "")

        if [[ "$is_first_id_entry" == "true" ]]; then
            is_first_id_entry=false
        else
            contexts_json+=", "
        fi
        contexts_json+="\"$story_id\": \"$context\""
    done <<< "$story_ids"

    contexts_json+="}"

    printf '{"story_ids": %s, "contexts": %s}\n' "$ids_json" "$contexts_json"
}

# =============================================================================
# Function 3: build_decision_archive
# AC#3: Build bidirectional story<->decision mapping
# SM-012: Build story→decision bidirectional mapping (High)
# NFR-010: Index 350+ plan files within 10 seconds
# =============================================================================
# Process a single plan file and update mappings
process_plan_file() {
    local plan_file="$1"
    local -n story_to_plans_ref=$2
    local -n plan_to_stories_ref=$3
    local -n plan_metadata_ref=$4

    local plan_name
    plan_name=$(basename "$plan_file")

    # Extract frontmatter
    local frontmatter
    frontmatter=$(extract_yaml_frontmatter "$plan_file")
    plan_metadata_ref["$plan_name"]="$frontmatter"

    # Extract story IDs
    local story_ids
    story_ids=$(grep -oE 'STORY-[0-9]+' "$plan_file" 2>/dev/null | sort -u || echo "")

    # Build plan_to_stories mapping using helper
    if [[ -n "$story_ids" ]]; then
        local stories_array
        stories_array=$(echo "$story_ids" | json_array_from_lines)
        plan_to_stories_ref["$plan_name"]="$stories_array"

        # Build story_to_plans mapping (bidirectional)
        while IFS= read -r story_id; do
            if [[ -n "$story_id" ]]; then
                if [[ -n "${story_to_plans_ref[$story_id]:-}" ]]; then
                    story_to_plans_ref["$story_id"]+=", \"$plan_name\""
                else
                    story_to_plans_ref["$story_id"]="\"$plan_name\""
                fi
            fi
        done <<< "$story_ids"
    else
        plan_to_stories_ref["$plan_name"]="[]"
    fi
}

# Build JSON object from associative array
build_json_object_from_array() {
    local -n array_ref=$1
    local json_str="{"
    local is_first=true

    for key in "${!array_ref[@]}"; do
        if [[ "$is_first" == "true" ]]; then
            is_first=false
        else
            json_str+=", "
        fi
        json_str+="\"$key\": ${array_ref[$key]}"
    done
    json_str+="}"
    echo "$json_str"
}

# Build JSON array of plan names from story_to_plans mapping
build_story_to_plans_json() {
    local -n array_ref=$1
    local json_str="{"
    local is_first=true

    for story_id in "${!array_ref[@]}"; do
        if [[ "$is_first" == "true" ]]; then
            is_first=false
        else
            json_str+=", "
        fi
        json_str+="\"$story_id\": [${array_ref[$story_id]}]"
    done
    json_str+="}"
    echo "$json_str"
}

build_decision_archive() {
    local plans_dir="$1"
    local archive_dir="$2"

    if [[ ! -d "$plans_dir" ]]; then
        echo '{"error": "Plans directory not found"}'
        return 1
    fi

    # Create archive directory if it doesn't exist
    mkdir -p "$archive_dir"

    # Initialize data structures
    declare -A story_to_plans
    declare -A plan_to_stories
    declare -A plan_metadata

    # Process all plan files
    local plan_count=0
    while IFS= read -r -d '' plan_file; do
        process_plan_file "$plan_file" story_to_plans plan_to_stories plan_metadata
        ((plan_count++))
    done < <(find "$plans_dir" -name "*.md" -type f -print0 2>/dev/null)

    # Build final JSON using helpers
    local story_to_plans_json
    story_to_plans_json=$(build_story_to_plans_json story_to_plans)

    local plan_to_stories_json
    plan_to_stories_json=$(build_json_object_from_array plan_to_stories)

    local plan_metadata_json
    plan_metadata_json=$(build_json_object_from_array plan_metadata)

    local archive_json
    archive_json="{\"story_to_plans\": $story_to_plans_json, \"plan_to_stories\": $plan_to_stories_json, \"metadata\": $plan_metadata_json}"

    # Write archive to file
    echo "$archive_json" > "$archive_dir/decision_archive.json"

    # Return summary
    printf '{"status": "success", "plan_count": %d, "archive_path": "%s/decision_archive.json"}\n' \
        "$plan_count" "$archive_dir"
}

# =============================================================================
# Function 4: query_archive
# AC#4: Query archive for related plan files
# =============================================================================
# Extract plans for a story from archive content
extract_plans_from_archive() {
    local archive_content="$1"
    local story_id="$2"

    # Try to extract story_to_plans entry
    local plans_array
    plans_array=$(echo "$archive_content" | grep -oE "\"$story_id\": *\[[^]]*\]" | \
        sed 's/.*\[\([^]]*\)\].*/[\1]/' | head -1 || echo "[]")

    # If not found, try alternate pattern
    if [[ -z "$plans_array" || "$plans_array" == "[]" ]]; then
        local story_entry
        story_entry=$(echo "$archive_content" | grep -o "\"$story_id\": \[[^]]*\]" || echo "")
        if [[ -n "$story_entry" ]]; then
            plans_array=$(echo "$story_entry" | sed 's/.*\[\(.*\)\]/[\1]/')
        fi
    fi

    echo "$plans_array"
}

query_archive() {
    local archive_dir="$1"
    local story_id="$2"

    local archive_file="$archive_dir/decision_archive.json"

    if [[ ! -f "$archive_file" ]]; then
        echo '{"error": "Archive not found", "story_id": "'"$story_id"'", "plans": []}'
        return 1
    fi

    # Read archive
    local archive_content
    archive_content=$(cat "$archive_file")

    # Extract plans using helper
    local plans_array
    plans_array=$(extract_plans_from_archive "$archive_content" "$story_id")

    if [[ -z "$plans_array" || "$plans_array" == "[]" ]]; then
        echo '{"story_id": "'"$story_id"'", "plans": [], "count": 0}'
        return 0
    fi

    # Count plans
    local count
    count=$(echo "$plans_array" | grep -oE '"[^"]*\.md"' | wc -l || echo "0")

    # Build result with plan details
    local result='{"story_id": "'"$story_id"'", "plans": '"$plans_array"', "count": '"$count"'}'

    echo "$result"
}

# =============================================================================
# Helper: Get archive statistics
# =============================================================================
get_archive_stats() {
    local archive_dir="$1"
    local archive_file="$archive_dir/decision_archive.json"

    if [[ ! -f "$archive_file" ]]; then
        echo '{"error": "Archive not found"}'
        return 1
    fi

    local story_count
    story_count=$(grep -oE '"STORY-[0-9]+":' "$archive_file" | sort -u | wc -l || echo "0")

    local plan_count
    plan_count=$(grep -oE '"[^"]+\.md":' "$archive_file" | sort -u | wc -l || echo "0")

    printf '{"story_count": %d, "plan_count": %d}\n' "$story_count" "$plan_count"
}

# =============================================================================
# STORY-232 Helper: Extract markdown section by header name
# Usage: extract_markdown_section "$file" "Section Name"
# Extracts content from "## Section Name" to next "##" or EOF
# =============================================================================
extract_markdown_section() {
    local file="$1"
    local section_name="$2"

    awk -v section="$section_name" '
        $0 ~ ("^## " section "$") { capture=1; next }
        /^## / && capture { capture=0 }
        capture { print }
    ' "$file" 2>/dev/null | sed '/^$/d' | tr '\n' ' ' | sed 's/  */ /g' | sed 's/^ //;s/ $//' || echo ""
}

# =============================================================================
# STORY-232 Helper: Extract JSON field value from JSON string
# Usage: extract_json_field "$json_string" "field_name"
# =============================================================================
extract_json_field() {
    local json_string="$1"
    local field_name="$2"

    echo "$json_string" | grep -oE "\"$field_name\": *\"[^\"]*\"" | \
        sed "s/\"$field_name\": *\"//" | sed 's/"$//' || echo ""
}

# =============================================================================
# STORY-232 Helper: Add keyword-to-plan mapping
# Usage: add_keyword_mapping keyword_array_name "$keyword" "$plan_name"
# =============================================================================
add_keyword_mapping() {
    local -n keyword_map_ref=$1
    local keyword="$2"
    local plan_name="$3"

    if [[ -z "$keyword" ]]; then return; fi

    if [[ -n "${keyword_map_ref[$keyword]:-}" ]]; then
        # Check if plan already in list
        if ! echo "${keyword_map_ref[$keyword]}" | grep -q "\"$plan_name\""; then
            keyword_map_ref["$keyword"]="${keyword_map_ref[$keyword]}, \"$plan_name\""
        fi
    else
        keyword_map_ref["$keyword"]="\"$plan_name\""
    fi
}

# =============================================================================
# STORY-232: Function 5: extract_decision_sections
# AC#2: Extract ## Decision and ## Technical Approach sections from plan file
# Usage: extract_decision_sections "$plan_file"
# Returns: JSON with "decision" and "technical_approach" keys
# =============================================================================
extract_decision_sections() {
    local plan_file="${1:-}"

    # Validate argument provided
    if [[ -z "$plan_file" ]]; then
        echo '{"error": "plan_file argument required"}'
        return 1
    fi

    # Validate path doesn't contain traversal
    if [[ "$plan_file" == *".."* ]]; then
        echo '{"error": "Path traversal not allowed"}'
        return 1
    fi

    validate_file_exists "$plan_file" || return 1

    # Extract sections using helper
    local decision_content
    decision_content=$(extract_markdown_section "$plan_file" "Decision")

    local technical_approach_content
    technical_approach_content=$(extract_markdown_section "$plan_file" "Technical Approach")

    # Escape content for JSON
    local escaped_decision
    escaped_decision=$(json_escape "$decision_content")

    local escaped_technical
    escaped_technical=$(json_escape "$technical_approach_content")

    # Return JSON
    printf '{"decision": "%s", "technical_approach": "%s"}\n' "$escaped_decision" "$escaped_technical"
}

# =============================================================================
# STORY-232: Function 6: build_searchable_index
# AC#1: Build searchable index with frontmatter (story ID, status, created date)
# AC#3: Create searchable_index.json with full-text content
# Usage: build_searchable_index "$plans_dir" "$index_dir"
# Returns: JSON with "status", "plan_count", "index_path" keys
# =============================================================================
build_searchable_index() {
    local plans_dir="${1:-}"
    local index_dir="${2:-}"

    # Validate arguments provided
    if [[ -z "$plans_dir" ]] || [[ -z "$index_dir" ]]; then
        echo '{"error": "plans_dir and index_dir arguments required"}'
        return 1
    fi

    # Validate paths don't contain traversal
    if [[ "$plans_dir" == *".."* ]] || [[ "$index_dir" == *".."* ]]; then
        echo '{"error": "Path traversal not allowed"}'
        return 1
    fi

    if [[ ! -d "$plans_dir" ]]; then
        echo '{"error": "Plans directory not found"}'
        return 1
    fi

    # Create index directory if it doesn't exist
    mkdir -p "$index_dir"

    # Start building JSON
    local plans_json="{"
    local keywords_json="{"
    local is_first_plan=true
    local plan_count=0

    # Associative array for keyword tracking
    declare -A keyword_to_plans

    # Process all plan files - OPTIMIZED: Single file read per plan
    while IFS= read -r -d '' plan_file; do
        local plan_name
        plan_name=$(basename "$plan_file")

        # Extract story ID from filename (STORY-NNN pattern) - no subshell
        local story_id=""
        [[ "$plan_name" =~ STORY-([0-9]+) ]] && story_id="STORY-${BASH_REMATCH[1]}"

        # OPTIMIZATION: Read file content ONCE into memory
        local file_content
        file_content=$(cat "$plan_file" 2>/dev/null) || file_content=""

        # Extract frontmatter from cached content
        local status="" created=""
        if [[ "$file_content" == "---"* ]]; then
            local yaml_block
            yaml_block=$(echo "$file_content" | awk 'BEGIN{f=0} /^---$/{f++; if(f==2) exit; next} f==1{print}')
            status=$(echo "$yaml_block" | grep -E "^status:" | sed 's/^status:[[:space:]]*//' | tr -d '"' | tr -d "'" || echo "")
            created=$(echo "$yaml_block" | grep -E "^created:" | sed 's/^created:[[:space:]]*//' | tr -d '"' | tr -d "'" || echo "")
        fi

        # Extract decision sections from cached content
        local decision="" technical_approach=""
        decision=$(echo "$file_content" | awk '/^## Decision$/{capture=1; next} /^## /{capture=0} capture{print}' | tr '\n' ' ' | sed 's/  */ /g;s/^ //;s/ $//' || echo "")
        technical_approach=$(echo "$file_content" | awk '/^## Technical Approach$/{capture=1; next} /^## /{capture=0} capture{print}' | tr '\n' ' ' | sed 's/  */ /g;s/^ //;s/ $//' || echo "")

        # Build full text from cached content
        local full_text
        full_text=$(echo "$file_content" | tr '\n' ' ' | sed 's/  */ /g' | head -c 500 || echo "")
        local escaped_full_text escaped_decision escaped_technical
        escaped_full_text=$(json_escape "$full_text")
        escaped_decision=$(json_escape "$decision")
        escaped_technical=$(json_escape "$technical_approach")

        # Build keywords from cached content - use word list directly, skip common words
        local content_words
        content_words=$(echo "$file_content" | tr '[:upper:]' '[:lower:]' | grep -oE '[a-z]{4,}' | \
            grep -vE '^(that|this|with|from|have|been|were|will|would|could|should|their|there|which|about)$' | \
            sort -u | head -50 || echo "")

        # Track keywords for this plan using helper
        while IFS= read -r word; do
            [[ -n "$word" ]] && add_keyword_mapping keyword_to_plans "$word" "$plan_name"
        done <<< "$content_words"

        # Add to plans JSON
        if [[ "$is_first_plan" == "true" ]]; then
            is_first_plan=false
        else
            plans_json+=", "
        fi

        plans_json+="\"$plan_name\": {\"story_id\": \"$story_id\", \"status\": \"$status\", \"created\": \"$created\", \"decision\": \"$escaped_decision\", \"technical_approach\": \"$escaped_technical\", \"full_text\": \"$escaped_full_text\"}"

        ((plan_count++))
    done < <(find "$plans_dir" -name "*.md" -type f -print0 2>/dev/null)

    plans_json+="}"

    # Build keywords JSON
    local is_first_keyword=true
    for keyword in "${!keyword_to_plans[@]}"; do
        if [[ "$is_first_keyword" == "true" ]]; then
            is_first_keyword=false
        else
            keywords_json+=", "
        fi
        keywords_json+="\"$keyword\": [${keyword_to_plans[$keyword]}]"
    done
    keywords_json+="}"

    # Combine into final index
    local index_json="{\"plans\": $plans_json, \"keywords\": $keywords_json}"

    # Write to file
    echo "$index_json" > "$index_dir/searchable_index.json"

    # Return success
    printf '{"status": "success", "plan_count": %d, "index_path": "%s/searchable_index.json"}\n' \
        "$plan_count" "$index_dir"
}

# =============================================================================
# STORY-232: Function 7: search_index
# AC#3: Search index for keyword matches in decision, technical approach, full text
# Usage: search_index "$index_dir" "$keyword"
# Returns: JSON with "query", "matches", "count" keys
# =============================================================================
search_index() {
    local index_dir="${1:-}"
    local keyword="${2:-}"

    # Validate index_dir provided
    if [[ -z "$index_dir" ]]; then
        echo '{"error": "index_dir argument required", "query": "", "matches": [], "count": 0}'
        return 1
    fi

    # Validate path doesn't contain traversal
    if [[ "$index_dir" == *".."* ]]; then
        echo '{"error": "Path traversal not allowed", "query": "", "matches": [], "count": 0}'
        return 1
    fi

    local index_file="$index_dir/searchable_index.json"

    if [[ ! -f "$index_file" ]]; then
        echo '{"error": "Index not found", "query": "'"$keyword"'", "matches": [], "count": 0}'
        return 1
    fi

    # Handle empty keyword
    if [[ -z "$keyword" ]]; then
        echo '{"error": "Empty query", "query": "", "matches": [], "count": 0}'
        return 1
    fi

    local index_content
    index_content=$(cat "$index_file")

    # Convert keyword to lowercase for case-insensitive search
    local keyword_lower
    keyword_lower=$(echo "$keyword" | tr '[:upper:]' '[:lower:]')

    # Sanitize regex metacharacters to prevent command injection
    local sanitized_keyword
    sanitized_keyword=$(printf '%s\n' "$keyword_lower" | sed 's/[.[\*^$()+?{|\\]/\\&/g')

    # Collect matching plan files
    declare -A matches_set

    # Method 1: Check keyword index (pre-built keywords section)
    # Look for "keyword": ["plan1.md", "plan2.md"] pattern
    local keyword_matches
    keyword_matches=$(echo "$index_content" | grep -oE "\"$sanitized_keyword\": *\[[^\]]*\]" | \
        grep -oE '"STORY-[^"]+\.md"' | tr -d '"' || echo "")

    while IFS= read -r match; do
        if [[ -n "$match" ]]; then
            matches_set["$match"]=1
        fi
    done <<< "$keyword_matches"

    # Method 2: Search in full text content (case insensitive grep through entire file)
    # Find plan names that contain the keyword in their content
    local plan_names
    plan_names=$(echo "$index_content" | grep -oE '"STORY-[^"]+\.md":' | tr -d '":' || echo "")

    while IFS= read -r plan_name; do
        if [[ -z "$plan_name" ]]; then continue; fi

        # Use awk to extract the block for this plan and search for keyword
        local found
        found=$(echo "$index_content" | awk -v plan="$plan_name" -v kw="$keyword_lower" '
            BEGIN { IGNORECASE=1; in_plan=0; content="" }
            $0 ~ ("\"" plan "\":") { in_plan=1 }
            in_plan { content = content $0 }
            in_plan && /\}[,]?$/ && !/\{/ {
                if (content ~ kw) print plan
                in_plan=0; content=""
            }
        ' || echo "")

        if [[ -n "$found" ]]; then
            matches_set["$plan_name"]=1
        fi
    done <<< "$plan_names"

    # Build matches array
    local matches_json="["
    local is_first=true
    local count=0

    for match in "${!matches_set[@]}"; do
        if [[ "$is_first" == "true" ]]; then
            is_first=false
        else
            matches_json+=", "
        fi
        matches_json+="\"$match\""
        ((count++))
    done
    matches_json+="]"

    # Return result
    printf '{"query": "%s", "matches": %s, "count": %d}\n' "$keyword" "$matches_json" "$count"
}
