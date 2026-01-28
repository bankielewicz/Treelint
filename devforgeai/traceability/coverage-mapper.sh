#!/bin/bash
#
# Coverage Mapping Service
# STORY-084: Generate bidirectional epic-story indexes and coverage statistics
#
# Usage:
#   ./coverage-mapper.sh [command] [options]
#
# Commands:
#   --generate-coverage         Generate full coverage report
#   --validate-linkage STORY    Validate story's epic linkage
#   --stories-for-epic EPIC     Get stories linked to epic
#   --epic-for-story STORY      Get epic for story
#   --detect-orphans            Detect orphaned stories
#   --detect-uncovered          Detect uncovered features
#   --help                      Show this help
#
# Output: JSON objects with coverage statistics

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
CONFIG_FILE="${SCRIPT_DIR}/config.json"
EPIC_PARSER="${SCRIPT_DIR}/epic-parser.sh"
STORY_PARSER="${SCRIPT_DIR}/story-parser.sh"

# Load configuration
if [ -f "$CONFIG_FILE" ]; then
    EPICS_DIR="${PROJECT_ROOT}/$(jq -r '.epics_dir' "$CONFIG_FILE")"
    STORIES_DIR="${PROJECT_ROOT}/$(jq -r '.stories_dir' "$CONFIG_FILE")"
else
    EPICS_DIR="${PROJECT_ROOT}/devforgeai/specs/Epics"
    STORIES_DIR="${PROJECT_ROOT}/devforgeai/specs/Stories"
fi

#############################################################################
# LINKAGE VALIDATION (AC#6)
#############################################################################

validate_epic_linkage() {
    local story_id="$1"

    # Find story file
    local story_file
    story_file=$(find "$STORIES_DIR" -name "${story_id}*.story.md" 2>/dev/null | head -1)

    if [ -z "$story_file" ] || [ ! -f "$story_file" ]; then
        echo '{"is_valid":false,"error_message":"Story file not found for '"$story_id"'"}'
        return 1
    fi

    # Parse story to get epic reference
    local story_data
    story_data=$("$STORY_PARSER" --parse-frontmatter "$story_file" 2>/dev/null) || {
        echo '{"is_valid":false,"error_message":"Failed to parse story '"$story_id"'"}'
        return 1
    }

    local epic_ref
    epic_ref=$(echo "$story_data" | jq -r '.epic // "null"')

    if [ "$epic_ref" = "null" ] || [ -z "$epic_ref" ]; then
        echo '{"is_valid":true,"referenced_epic":null,"epic_exists":false,"note":"Story is standalone (no epic reference)"}'
        return 0
    fi

    # Check if epic file exists
    local epic_file
    epic_file=$(find "$EPICS_DIR" -name "${epic_ref}*.epic.md" 2>/dev/null | head -1)

    if [ -z "$epic_file" ] || [ ! -f "$epic_file" ]; then
        # Build list of available epics
        local available_epics
        available_epics=$(find "$EPICS_DIR" -name "EPIC-*.epic.md" 2>/dev/null | \
            xargs -I{} basename {} | \
            grep -oE "EPIC-[0-9]{3}" | \
            sort -u | \
            tr '\n' ',' | sed 's/,$//')

        echo '{"is_valid":false,"referenced_epic":"'"$epic_ref"'","epic_exists":false,"error_message":"Story '"$story_id"' references non-existent epic '"$epic_ref"'. Available epics: ['"$available_epics"']"}'
        return 1
    fi

    # Parse epic to get title
    local epic_data epic_title
    epic_data=$("$EPIC_PARSER" --parse-frontmatter "$epic_file" 2>/dev/null) || epic_data='{}'
    epic_title=$(echo "$epic_data" | jq -r '.title // "Unknown"')

    echo '{"is_valid":true,"referenced_epic":"'"$epic_ref"'","epic_exists":true,"epic_title":"'"$epic_title"'"}'
}

#############################################################################
# INDEX BUILDING
#############################################################################

build_story_to_epic_index() {
    local index='{}'

    for story_file in "$STORIES_DIR"/*.story.md; do
        [ -f "$story_file" ] || continue

        local story_data
        story_data=$("$STORY_PARSER" --parse-frontmatter "$story_file" 2>/dev/null) || continue

        # Skip if error
        if echo "$story_data" | jq -e '.error_type' >/dev/null 2>&1; then
            continue
        fi

        local story_id epic_ref
        story_id=$(echo "$story_data" | jq -r '.id')
        epic_ref=$(echo "$story_data" | jq -r '.epic // "null"')

        if [ "$epic_ref" != "null" ] && [ -n "$epic_ref" ]; then
            index=$(echo "$index" | jq --arg id "$story_id" --arg epic "$epic_ref" '.[$id] = $epic')
        fi
    done

    echo "$index"
}

build_epic_to_stories_index() {
    local index='{}'

    for story_file in "$STORIES_DIR"/*.story.md; do
        [ -f "$story_file" ] || continue

        local story_data
        story_data=$("$STORY_PARSER" --parse-frontmatter "$story_file" 2>/dev/null) || continue

        # Skip if error
        if echo "$story_data" | jq -e '.error_type' >/dev/null 2>&1; then
            continue
        fi

        local story_id epic_ref
        story_id=$(echo "$story_data" | jq -r '.id')
        epic_ref=$(echo "$story_data" | jq -r '.epic // "null"')

        if [ "$epic_ref" != "null" ] && [ -n "$epic_ref" ]; then
            # Initialize epic array if not exists
            if ! echo "$index" | jq -e ".\"$epic_ref\"" >/dev/null 2>&1; then
                index=$(echo "$index" | jq --arg epic "$epic_ref" '.[$epic] = []')
            fi
            index=$(echo "$index" | jq --arg epic "$epic_ref" --arg id "$story_id" '.[$epic] += [$id]')
        fi
    done

    echo "$index"
}

#############################################################################
# ORPHAN DETECTION (AC#5)
#############################################################################

detect_orphaned_stories() {
    local intentionally_standalone='[]'
    local broken_references='[]'
    local missing_metadata='[]'

    for story_file in "$STORIES_DIR"/*.story.md; do
        [ -f "$story_file" ] || continue

        local story_data
        story_data=$("$STORY_PARSER" --parse-frontmatter "$story_file" 2>/dev/null) || {
            missing_metadata=$(echo "$missing_metadata" | jq --arg file "$story_file" '. += [$file]')
            continue
        }

        # Check for parse errors
        if echo "$story_data" | jq -e '.error_type' >/dev/null 2>&1; then
            local error_type
            error_type=$(echo "$story_data" | jq -r '.error_type')
            if [ "$error_type" = "MISSING_FRONTMATTER" ]; then
                local story_id
                story_id=$(echo "$story_data" | jq -r '.extracted_id // empty')
                missing_metadata=$(echo "$missing_metadata" | jq --arg id "$story_id" '. += [$id]')
            fi
            continue
        fi

        local story_id epic_ref
        story_id=$(echo "$story_data" | jq -r '.id')
        epic_ref=$(echo "$story_data" | jq -r '.epic // "null"')

        if [ "$epic_ref" = "null" ] || [ -z "$epic_ref" ]; then
            # Intentionally standalone
            intentionally_standalone=$(echo "$intentionally_standalone" | jq --arg id "$story_id" '. += [$id]')
        else
            # Check if epic exists
            local epic_file
            epic_file=$(find "$EPICS_DIR" -name "${epic_ref}*.epic.md" 2>/dev/null | head -1)

            if [ -z "$epic_file" ] || [ ! -f "$epic_file" ]; then
                broken_references=$(echo "$broken_references" | jq --arg id "$story_id" '. += [$id]')
            fi
        fi
    done

    local total_orphans
    total_orphans=$(echo "$intentionally_standalone $broken_references $missing_metadata" | jq -s 'map(length) | add')

    jq -n \
        --argjson standalone "$intentionally_standalone" \
        --argjson broken "$broken_references" \
        --argjson missing "$missing_metadata" \
        --argjson total "$total_orphans" \
        '{
            intentionally_standalone: $standalone,
            broken_references: $broken,
            missing_metadata: $missing,
            summary: {
                total_orphans: $total
            }
        }'
}

#############################################################################
# UNCOVERED FEATURES DETECTION
#############################################################################

detect_uncovered_features() {
    local uncovered='[]'
    local epic_to_stories
    epic_to_stories=$(build_epic_to_stories_index)

    for epic_file in "$EPICS_DIR"/*.epic.md; do
        [ -f "$epic_file" ] || continue

        local epic_data
        epic_data=$("$EPIC_PARSER" --parse-epic "$epic_file" 2>/dev/null) || continue

        # Skip if error
        if echo "$epic_data" | jq -e '.error_type' >/dev/null 2>&1; then
            continue
        fi

        local epic_id features_count linked_stories_count
        epic_id=$(echo "$epic_data" | jq -r '.epic_id')
        features_count=$(echo "$epic_data" | jq '.features_count')
        linked_stories_count=$(echo "$epic_to_stories" | jq --arg id "$epic_id" '.[$id] // [] | length')

        # Check each feature for linked story
        local features
        features=$(echo "$epic_data" | jq '.features')

        echo "$features" | jq -c '.[]' | while read -r feature; do
            local feature_id linked_story
            feature_id=$(echo "$feature" | jq -r '.feature_id')
            linked_story=$(echo "$feature" | jq -r '.linked_story // "null"')

            if [ "$linked_story" = "null" ]; then
                # Feature has no linked story - check if any story covers it
                # For now, mark as potentially uncovered
                echo '{"epic_id":"'"$epic_id"'","feature_id":"'"$feature_id"'"}'
            fi
        done
    done | jq -s '.'
}

#############################################################################
# COVERAGE REPORT GENERATION (AC#7)
#############################################################################

generate_coverage_report() {
    local epic_to_stories story_to_epic orphans epic_coverage

    # Build indexes
    story_to_epic=$(build_story_to_epic_index)
    epic_to_stories=$(build_epic_to_stories_index)

    # Detect orphans
    orphans=$(detect_orphaned_stories)

    # Calculate per-epic coverage
    epic_coverage='{}'
    local total_features=0
    local total_covered=0

    for epic_file in "$EPICS_DIR"/*.epic.md; do
        [ -f "$epic_file" ] || continue

        local epic_data
        epic_data=$("$EPIC_PARSER" --parse-epic "$epic_file" 2>/dev/null) || continue

        # Skip if error
        if echo "$epic_data" | jq -e '.error_type' >/dev/null 2>&1; then
            continue
        fi

        local epic_id features_count linked_stories coverage_pct
        epic_id=$(echo "$epic_data" | jq -r '.epic_id')
        features_count=$(echo "$epic_data" | jq '.features_count')
        linked_stories=$(echo "$epic_to_stories" | jq --arg id "$epic_id" '.[$id] // [] | length')

        # Calculate coverage percentage
        if [ "$features_count" -gt 0 ]; then
            # Simple coverage: stories linked / features
            # Cap at 100%
            coverage_pct=$(( (linked_stories * 100) / features_count ))
            [ "$coverage_pct" -gt 100 ] && coverage_pct=100
        else
            coverage_pct=0
        fi

        total_features=$((total_features + features_count))
        total_covered=$((total_covered + (linked_stories < features_count ? linked_stories : features_count)))

        epic_coverage=$(echo "$epic_coverage" | jq \
            --arg id "$epic_id" \
            --argjson total "$features_count" \
            --argjson covered "$linked_stories" \
            --argjson pct "$coverage_pct" \
            '.[$id] = {total_features: $total, covered_features: $covered, coverage_percentage: $pct}')
    done

    # Calculate aggregate
    local aggregate_pct=0
    if [ "$total_features" -gt 0 ]; then
        aggregate_pct=$((total_covered * 100 / total_features))
    fi

    # Build final report
    jq -n \
        --argjson story_to_epic "$story_to_epic" \
        --argjson epic_to_stories "$epic_to_stories" \
        --argjson orphaned_stories "$(echo "$orphans" | jq '[.intentionally_standalone[], .broken_references[], .missing_metadata[]]')" \
        --argjson validation "$orphans" \
        --argjson epic_coverage "$epic_coverage" \
        --argjson total_features "$total_features" \
        --argjson total_covered "$total_covered" \
        --argjson aggregate_pct "$aggregate_pct" \
        '{
            story_to_epic: $story_to_epic,
            epic_to_stories: $epic_to_stories,
            orphaned_stories: $orphaned_stories,
            uncovered_features: [],
            epic_coverage: $epic_coverage,
            aggregate: {
                total_features: $total_features,
                total_covered: $total_covered,
                total_coverage_percentage: $aggregate_pct
            },
            validation: $validation
        }' | jq -S '.'
}

#############################################################################
# QUERY FUNCTIONS
#############################################################################

stories_for_epic() {
    local epic_id="$1"
    local index
    index=$(build_epic_to_stories_index)
    echo "$index" | jq --arg id "$epic_id" '.[$id] // []'
}

epic_for_story() {
    local story_id="$1"
    local index
    index=$(build_story_to_epic_index)
    echo "$index" | jq -r --arg id "$story_id" '.[$id] // "null"'
}

#############################################################################
# MAIN COMMAND HANDLER
#############################################################################

show_help() {
    cat << EOF
Coverage Mapping Service - STORY-084

Usage: $0 [command] [options]

Commands:
  --generate-coverage         Generate full coverage report
  --validate-linkage STORY    Validate story's epic linkage
  --stories-for-epic EPIC     Get stories linked to epic
  --epic-for-story STORY      Get epic for story
  --detect-orphans            Detect orphaned stories
  --detect-uncovered          Detect uncovered features
  --help                      Show this help

Examples:
  $0 --generate-coverage
  $0 --validate-linkage STORY-083
  $0 --stories-for-epic EPIC-015
  $0 --epic-for-story STORY-084

Output: JSON objects with coverage statistics
EOF
}

case "${1:-}" in
    --generate-coverage)
        generate_coverage_report
        ;;
    --validate-linkage)
        [ -n "${2:-}" ] || { echo '{"error":"Missing story ID argument"}'; exit 1; }
        validate_epic_linkage "$2"
        ;;
    --stories-for-epic)
        [ -n "${2:-}" ] || { echo '{"error":"Missing epic ID argument"}'; exit 1; }
        stories_for_epic "$2"
        ;;
    --epic-for-story)
        [ -n "${2:-}" ] || { echo '{"error":"Missing story ID argument"}'; exit 1; }
        epic_for_story "$2"
        ;;
    --detect-orphans)
        detect_orphaned_stories
        ;;
    --detect-uncovered)
        detect_uncovered_features
        ;;
    --help|-h)
        show_help
        ;;
    *)
        echo '{"error":"Unknown command. Use --help for usage."}'
        exit 1
        ;;
esac
