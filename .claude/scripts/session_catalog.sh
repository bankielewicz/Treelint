#!/bin/bash
#
# Session Catalog Functions for STORY-223
# Provides session file cataloging, dependency graph building, and session chain tracking
#
# Usage: source .claude/scripts/session_catalog.sh
#
# Functions:
#   catalog_session_files(directory)    - Map plans to stories to artifacts (AC#1)
#   build_dependency_graph(directory)   - Build file dependency graph (AC#2)
#   track_session_chains(directory)     - Track session continuity chains (AC#3)
#
# Refactored: Extracted helper functions, reduced complexity, improved performance
#

# ============================================================================
# Helper Functions (DRY - extracted repeated patterns)
# ============================================================================

# Find files matching a pattern and store in array variable
# Usage: find_files_into_array "/path" "*.md" result_array
find_files_into_array() {
    local directory="$1"
    local pattern="$2"
    local -n _result_array="$3"
    _result_array=()
    while IFS= read -r -d '' file; do
        _result_array+=("$file")
    done < <(find "$directory" -name "$pattern" -type f -print0 2>/dev/null)
}

# Classify file type based on path
# Usage: file_type=$(classify_file_type "/path/to/file")
classify_file_type() {
    local file="$1"
    case "$file" in
        */plans/*) echo "plan" ;;
        */sessions/*) echo "session" ;;
        */artifacts/*) echo "artifact" ;;
        */Stories/*|*/stories/*) echo "story" ;;
        *) echo "other" ;;
    esac
}

# Build JSON array from bash array
# Usage: json=$(build_json_array "${items[@]}")
build_json_array() {
    local items=("$@")
    local result=""
    for item in "${items[@]}"; do
        # Escape special JSON characters
        local escaped="${item//\\/\\\\}"
        escaped="${escaped//\"/\\\"}"
        [[ -n "$result" ]] && result+=","
        result+="\"$escaped\""
    done
    echo "[$result]"
}

# ============================================================================
# AC#1: Catalog Session Files
# Maps plan files -> story references -> associated artifacts
# ============================================================================

# Extract story references from plan file YAML frontmatter
# Usage: extract_story_refs "/path/to/plan.md" result_array
_extract_story_refs() {
    local plan_file="$1"
    local -n _refs="$2"
    _refs=()
    while IFS= read -r line; do
        if [[ "$line" =~ ^[[:space:]]*-[[:space:]]*(STORY-[0-9]+) ]]; then
            _refs+=("${BASH_REMATCH[1]}")
        fi
    done < <(sed -n '/^related_stories:/,/^[^[:space:]-]/p' "$plan_file" 2>/dev/null)
}

# Process plan files and populate mappings
# Uses associative arrays for O(1) lookups, then converts to JSON at end
_process_plan_files() {
    local directory="$1"
    local -n _p2s_map="$2"
    local -n _s2p_map="$3"
    local -n _files_json="$4"

    local plan_files=()
    while IFS= read -r -d '' file; do
        plan_files+=("$file")
    done < <(find "$directory/plans" -name "*.md" -type f -print0 2>/dev/null)

    for plan_file in "${plan_files[@]}"; do
        local plan_id
        plan_id=$(basename "$plan_file" .md)

        local story_refs=()
        _extract_story_refs "$plan_file" story_refs

        if [[ ${#story_refs[@]} -gt 0 ]]; then
            # Store as comma-separated for later conversion
            _p2s_map["$plan_id"]="${story_refs[*]}"

            # Build reverse mapping
            for story in "${story_refs[@]}"; do
                if [[ -n "${_s2p_map[$story]:-}" ]]; then
                    _s2p_map["$story"]+=" $plan_id"
                else
                    _s2p_map["$story"]="$plan_id"
                fi
            done
        fi

        # Collect file entry
        _files_json+=("{\"path\":\"$plan_file\",\"type\":\"plan\"}")
    done
}

# Process artifact directories and populate mappings
_process_artifacts() {
    local directory="$1"
    local -n _s2a_map="$2"
    local -n _files_json="$3"

    local artifact_dirs=()
    while IFS= read -r -d '' dir; do
        artifact_dirs+=("$dir")
    done < <(find "$directory/artifacts" -mindepth 1 -maxdepth 1 -type d -print0 2>/dev/null)

    for artifact_dir in "${artifact_dirs[@]}"; do
        local story_id
        story_id=$(basename "$artifact_dir")
        local artifact_names=()

        while IFS= read -r -d '' file; do
            local filename
            filename=$(basename "$file")
            artifact_names+=("$filename")
            _files_json+=("{\"path\":\"$file\",\"type\":\"artifact\"}")
        done < <(find "$artifact_dir" -type f -print0 2>/dev/null)

        if [[ ${#artifact_names[@]} -gt 0 ]]; then
            _s2a_map["$story_id"]="${artifact_names[*]}"
        fi
    done
}

# Convert associative array to JSON object with array values
# Usage: _assoc_to_json_object assoc_array -> {"key":["val1","val2"]}
_assoc_to_json_object() {
    local -n _assoc="$1"
    local result="{"
    local first=true

    for key in "${!_assoc[@]}"; do
        [[ "$first" == "true" ]] || result+=","
        first=false

        # Convert space-separated values to JSON array
        local vals="${_assoc[$key]}"
        local arr_json=""
        for val in $vals; do
            [[ -n "$arr_json" ]] && arr_json+=","
            arr_json+="\"$val\""
        done
        result+="\"$key\":[$arr_json]"
    done

    echo "$result}"
}

catalog_session_files() {
    local directory="${1:-.}"

    # Use associative arrays for O(1) lookups during processing
    declare -A plan_to_story_map
    declare -A story_to_artifacts_map
    declare -A story_to_plans_map
    local files_entries=()

    # Process all plan files
    _process_plan_files "$directory" plan_to_story_map story_to_plans_map files_entries

    # Process all artifacts
    _process_artifacts "$directory" story_to_artifacts_map files_entries

    # Process session files
    while IFS= read -r -d '' file; do
        files_entries+=("{\"path\":\"$file\",\"type\":\"session\"}")
    done < <(find "$directory/sessions" -name "*.json" -type f -print0 2>/dev/null)

    # Convert to JSON (single jq call for final assembly)
    local p2s_json s2a_json s2p_json files_json
    p2s_json=$(_assoc_to_json_object plan_to_story_map)
    s2a_json=$(_assoc_to_json_object story_to_artifacts_map)
    s2p_json=$(_assoc_to_json_object story_to_plans_map)

    # Build files array
    if [[ ${#files_entries[@]} -eq 0 ]]; then
        files_json="[]"
    else
        files_json="[$(IFS=,; echo "${files_entries[*]}")]"
    fi

    # Single jq call to assemble final output (performance optimization)
    jq -n -c \
        --argjson plan_to_story "$p2s_json" \
        --argjson story_to_artifacts "$s2a_json" \
        --argjson story_to_plans "$s2p_json" \
        --argjson files "$files_json" \
        '{
            "plan_to_story_map": $plan_to_story,
            "story_to_artifacts_map": $story_to_artifacts,
            "story_to_plans_map": $story_to_plans,
            "files": $files
        }'
}

# ============================================================================
# AC#2: Build Dependency Graph
# Analyzes file references and builds dependency graph
# Performance: O(n*m) where n=files, m=average file size for grep
# Optimized: Uses associative arrays for O(1) lookups in cycle detection
# ============================================================================

# Classify dependency type based on source and target paths
# Usage: dep_type=$(_classify_dependency_type "/source/path" "/target/path")
_classify_dependency_type() {
    local source_path="$1"
    local target_path="$2"

    if [[ "$source_path" == */plans/* && "$target_path" == */Stories/* ]]; then
        echo "plan_to_story"
    elif [[ "$source_path" == */sessions/* && "$target_path" == */sessions/* ]]; then
        echo "session_chain"
    else
        echo "reference"
    fi
}

# Build nodes map from file list (batch operation)
# Usage: nodes_json=$(_build_nodes_map "${files[@]}")
_build_nodes_map() {
    local files=("$@")
    local result="{"
    local first=true

    for file in "${files[@]}"; do
        local file_id file_type
        file_id=$(basename "$file")
        file_type=$(classify_file_type "$file")

        [[ "$first" == "true" ]] || result+=","
        first=false
        result+="\"$file_id\":{\"path\":\"$file\",\"type\":\"$file_type\"}"
    done

    echo "$result}"
}

# Find file references using optimized grep (single pass per file)
# Populates associative array with source|target pairs
_find_file_references() {
    local -n _dep_entries="$1"
    shift
    local files=("$@")

    # Build lookup sets for O(1) target matching
    declare -A file_ids
    declare -A file_basenames
    declare -A file_paths
    for file in "${files[@]}"; do
        local fid fbase
        fid=$(basename "$file")
        fbase="${fid%.*}"
        file_ids["$fid"]=1
        file_basenames["$fbase"]="$fid"
        file_paths["$fid"]="$file"
    done

    # Process each source file
    for source_file in "${files[@]}"; do
        local source_id
        source_id=$(basename "$source_file")

        # Single grep to get all potential references
        local file_content
        file_content=$(cat "$source_file" 2>/dev/null) || continue

        # Check each potential target
        for target_id in "${!file_ids[@]}"; do
            [[ "$source_id" == "$target_id" ]] && continue

            local target_basename="${target_id%.*}"

            # Check if file content references target
            if [[ "$file_content" == *"$target_id"* ]] || [[ "$file_content" == *"$target_basename"* ]]; then
                local target_file="${file_paths[$target_id]}"
                local dep_type
                dep_type=$(_classify_dependency_type "$source_file" "$target_file")
                _dep_entries+=("{\"source\":\"$source_id\",\"target\":\"$target_id\",\"type\":\"$dep_type\"}")
            fi
        done
    done
}

# Detect circular dependencies using associative array for O(n) performance
# Usage: circular_json=$(_detect_circular_deps "${dep_entries[@]}")
_detect_circular_deps() {
    local dep_entries=("$@")

    # Build edge set for O(1) reverse lookup
    declare -A edge_set
    declare -A seen_cycles
    local circular_entries=()

    for entry in "${dep_entries[@]}"; do
        # Extract source and target from JSON entry
        local source target
        source=$(echo "$entry" | grep -o '"source":"[^"]*"' | cut -d'"' -f4)
        target=$(echo "$entry" | grep -o '"target":"[^"]*"' | cut -d'"' -f4)
        edge_set["$source|$target"]=1
    done

    # Check for reverse edges (cycles)
    for edge in "${!edge_set[@]}"; do
        IFS='|' read -r source target <<< "$edge"
        local reverse_edge="$target|$source"

        if [[ -n "${edge_set[$reverse_edge]:-}" ]]; then
            # Normalize cycle representation (alphabetical order)
            local cycle_key
            if [[ "$source" < "$target" ]]; then
                cycle_key="$source|$target"
            else
                cycle_key="$target|$source"
            fi

            # Only add if not already seen
            if [[ -z "${seen_cycles[$cycle_key]:-}" ]]; then
                seen_cycles["$cycle_key"]=1
                if [[ "$source" < "$target" ]]; then
                    circular_entries+=("[\"$source\",\"$target\"]")
                else
                    circular_entries+=("[\"$target\",\"$source\"]")
                fi
            fi
        fi
    done

    # Build JSON array
    if [[ ${#circular_entries[@]} -eq 0 ]]; then
        echo "[]"
    else
        echo "[$(IFS=,; echo "${circular_entries[*]}")]"
    fi
}

build_dependency_graph() {
    local directory="${1:-.}"

    # Find all relevant files
    local all_files=()
    while IFS= read -r -d '' file; do
        all_files+=("$file")
    done < <(find "$directory" \( -name "*.md" -o -name "*.json" -o -name "*.sh" \) -type f -print0 2>/dev/null)

    # Build nodes map (batch operation - no jq calls in loop)
    local nodes_json
    nodes_json=$(_build_nodes_map "${all_files[@]}")

    # Find all dependencies
    local dep_entries=()
    _find_file_references dep_entries "${all_files[@]}"

    # Detect circular dependencies (O(n) with hash lookup)
    local circular_json
    circular_json=$(_detect_circular_deps "${dep_entries[@]}")

    # Build dependencies JSON array
    local deps_json
    if [[ ${#dep_entries[@]} -eq 0 ]]; then
        deps_json="[]"
    else
        deps_json="[$(IFS=,; echo "${dep_entries[*]}")]"
    fi

    # Single jq call for final assembly
    jq -n -c \
        --argjson deps "$deps_json" \
        --argjson nodes "$nodes_json" \
        --argjson circular "$circular_json" \
        '{
            "dependencies": $deps,
            "nodes": $nodes,
            "circular_dependencies": $circular
        }'
}

# ============================================================================
# AC#3: Track Session Chains
# Identifies parent-child relationships via parentUuid
# Optimized: Reduced jq calls, uses pure bash for chain building
# ============================================================================

# Extract uuid and parentUuid from session file (single jq call)
# Usage: read uuid parent_uuid < <(_extract_session_uuids "$file")
_extract_session_uuids() {
    local file="$1"
    jq -r '[.uuid // "", .parentUuid // ""] | @tsv' "$file" 2>/dev/null
}

# Build session chain using BFS (pure bash, no jq in loop)
# Usage: chain_json=$(_build_session_chain "$root_uuid" children_map visited_set)
_build_session_chain() {
    local root_uuid="$1"
    local -n _children="$2"
    local -n _visited="$3"

    local depth=0
    local node_list=("$root_uuid")
    local queue=("$root_uuid")
    _visited["$root_uuid"]=1

    while [[ ${#queue[@]} -gt 0 ]]; do
        local current="${queue[0]}"
        queue=("${queue[@]:1}")

        local children="${_children[$current]:-}"
        for child in $children; do
            if [[ -z "${_visited[$child]:-}" ]]; then
                _visited["$child"]=1
                queue+=("$child")
                node_list+=("$child")
                ((depth++))
            fi
        done
    done

    # Build JSON without jq
    local nodes_json
    nodes_json=$(build_json_array "${node_list[@]}")
    echo "{\"root\":\"$root_uuid\",\"nodes\":$nodes_json,\"depth\":$depth}"
}

track_session_chains() {
    local session_directory="${1:-.}"

    # Find all session files
    local session_files=()
    while IFS= read -r -d '' file; do
        session_files+=("$file")
    done < <(find "$session_directory" -name "*.json" -type f -print0 2>/dev/null)

    # Build session maps: uuid -> file, uuid -> parent, parent -> children
    declare -A session_map
    declare -A parent_map
    declare -A children_map

    for file in "${session_files[@]}"; do
        local uuid parent_uuid
        read -r uuid parent_uuid < <(_extract_session_uuids "$file")

        [[ -z "$uuid" ]] && continue

        session_map["$uuid"]="$file"

        if [[ -n "$parent_uuid" ]]; then
            parent_map["$uuid"]="$parent_uuid"
            children_map["$parent_uuid"]+="$uuid "
        fi
    done

    # Categorize sessions: roots (no parent) and orphans (missing parent)
    local root_uuids=()
    local orphan_uuids=()

    for uuid in "${!session_map[@]}"; do
        local parent="${parent_map[$uuid]:-}"

        if [[ -z "$parent" ]]; then
            root_uuids+=("$uuid")
        elif [[ -z "${session_map[$parent]:-}" ]]; then
            orphan_uuids+=("$uuid")
        fi
    done

    # Build chains from each root (pure bash)
    local chain_entries=()
    for root in "${root_uuids[@]}"; do
        declare -A visited
        local chain_json
        chain_json=$(_build_session_chain "$root" children_map visited)
        chain_entries+=("$chain_json")
        unset visited
    done

    # Build output JSON arrays without intermediate jq calls
    local roots_json orphans_json chains_json
    roots_json=$(build_json_array "${root_uuids[@]}")
    orphans_json=$(build_json_array "${orphan_uuids[@]}")

    if [[ ${#chain_entries[@]} -eq 0 ]]; then
        chains_json="[]"
    else
        chains_json="[$(IFS=,; echo "${chain_entries[*]}")]"
    fi

    # Single jq call for final assembly
    jq -n -c \
        --argjson chains "$chains_json" \
        --argjson roots "$roots_json" \
        --argjson orphans "$orphans_json" \
        '{
            "session_chains": $chains,
            "root_sessions": $roots,
            "orphan_sessions": $orphans
        }'
}

# ============================================================================
# Export functions
# ============================================================================

# Primary API functions
export -f catalog_session_files
export -f build_dependency_graph
export -f track_session_chains

# Helper functions (also exported for testing/reuse)
export -f find_files_into_array
export -f classify_file_type
export -f build_json_array

# Internal helper functions (exported for subshell compatibility)
export -f _extract_story_refs
export -f _process_plan_files
export -f _process_artifacts
export -f _assoc_to_json_object
export -f _classify_dependency_type
export -f _build_nodes_map
export -f _find_file_references
export -f _detect_circular_deps
export -f _extract_session_uuids
export -f _build_session_chain
