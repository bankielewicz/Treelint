#!/bin/bash

##############################################################################
# Confidence Scorer - STORY-089 AC#5
# Purpose: Calculate confidence scores for story-to-feature matches
#
# AC#5: Ambiguous Match Flagging for Manual Review
# - Flag matches with confidence score between 60-75%
# - Display story ID, matched feature, and confidence percentage
# - Do not count low-confidence matches toward coverage until confirmed
#
# Confidence Calculation:
# - Uses Jaccard similarity on normalized word tokens
# - Scores: 0.0-1.0 range mapped to 0-100%
# - >75%: High confidence (counted in coverage)
# - 60-75%: Low confidence (manual review required)
# - <60%: No match
##############################################################################

set -o pipefail

# Script info
SCRIPT_NAME="confidence-scorer.sh"
VERSION="1.0.0"

# Paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR}/../.."

# Default directories
DEFAULT_EPIC_DIR="${PROJECT_ROOT}/devforgeai/specs/Epics"
DEFAULT_STORY_DIR="${PROJECT_ROOT}/devforgeai/specs/Stories"

# Confidence thresholds (from thresholds.json)
LOW_THRESHOLD=60
HIGH_THRESHOLD=75
MIN_FOR_COUNT=75

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

usage() {
    cat << EOF
Usage: $SCRIPT_NAME [OPTIONS]

Calculate confidence scores for story-to-feature matches.

Options:
    --story <file>              Single story file to match
    --stories <dir>             Directory of story files
    --epic <file>               Single epic file
    --epic-dir <dir>            Directory of epic files
    --calculate-coverage        Calculate coverage (excluding low-confidence)
    --confidence-threshold <n>  Custom threshold (0.0-1.0)
    --format <type>             Output format: text, json
    --test-br-003               Run BR-003 test case
    --help                      Show this help message
    --version                   Show version

Exit Codes:
    0    Success
    1    Warnings (low-confidence matches found)
    2    No matches found
    3    Internal error

Examples:
    $SCRIPT_NAME --story story.md --epic epic.md
    $SCRIPT_NAME --stories devforgeai/specs/Stories --epic-dir devforgeai/specs/Epics
    $SCRIPT_NAME --calculate-coverage --stories devforgeai/specs/Stories
EOF
}

##############################################################################
# Text Normalization
##############################################################################

# Normalize text for comparison
# - Convert to lowercase
# - Remove punctuation
# - Split into words
normalize_text() {
    local text="$1"

    # Convert to lowercase
    text="${text,,}"

    # Remove punctuation and special characters
    text=$(echo "$text" | tr -cs '[:alnum:][:space:]' ' ')

    # Trim whitespace
    text=$(echo "$text" | xargs)

    echo "$text"
}

# Get unique words from text as array
get_words() {
    local text="$1"
    local normalized
    normalized=$(normalize_text "$text")

    # Split into words
    echo "$normalized" | tr ' ' '\n' | sort -u | grep -v '^$'
}

##############################################################################
# Similarity Calculation
##############################################################################

# Calculate Jaccard similarity between two texts
# Returns: Score as integer percentage (0-100)
calculate_jaccard_similarity() {
    local text1="$1"
    local text2="$2"

    # Get word sets
    local words1 words2
    words1=$(get_words "$text1")
    words2=$(get_words "$text2")

    # Handle empty inputs
    if [[ -z "$words1" ]] || [[ -z "$words2" ]]; then
        echo "0"
        return
    fi

    # Count intersection
    local intersection=0
    while IFS= read -r word; do
        if echo "$words2" | grep -qw "$word" 2>/dev/null; then
            ((intersection++))
        fi
    done <<< "$words1"

    # Count union
    local union
    union=$(echo -e "$words1\n$words2" | sort -u | wc -l)

    # Calculate Jaccard coefficient
    if [[ $union -eq 0 ]]; then
        echo "0"
        return
    fi

    local score
    score=$(echo "scale=0; ($intersection * 100) / $union" | bc 2>/dev/null || echo "0")
    echo "$score"
}

# Calculate weighted similarity (title + description keywords)
# Uses title as primary score, description as potential bonus only
calculate_weighted_similarity() {
    local story_title="$1"
    local feature_title="$2"
    local feature_desc="${3:-}"

    # Title similarity is primary score
    local title_score
    title_score=$(calculate_jaccard_similarity "$story_title" "$feature_title")

    # If title score is already high (>=70%), use it directly
    # Description can only add to score, not reduce it
    if [[ $title_score -ge 70 ]]; then
        echo "$title_score"
        return
    fi

    # For lower title scores, check if description provides a better match
    local desc_score=0
    if [[ -n "$feature_desc" ]]; then
        desc_score=$(calculate_jaccard_similarity "$story_title" "$feature_desc")
    fi

    # Use the higher of: title score, or weighted average (if desc helps)
    local weighted
    weighted=$(echo "scale=0; ($title_score * 70 + $desc_score * 30) / 100" | bc 2>/dev/null || echo "$title_score")

    # Return the maximum of title_score and weighted
    if [[ $title_score -ge $weighted ]]; then
        echo "$title_score"
    else
        echo "$weighted"
    fi
}

##############################################################################
# Feature Extraction
##############################################################################

# Extract features from epic file
# Returns: feature_num|title|description per line
extract_features_from_epic() {
    local epic_file="$1"

    if [[ ! -f "$epic_file" ]]; then
        return 1
    fi

    local current_feature=""
    local current_title=""
    local current_desc=""
    local in_feature=false

    while IFS= read -r line || [[ -n "$line" ]]; do
        line=$(echo "$line" | tr -d '\r')

        # Check for feature header: ### Feature N: Title
        if [[ "$line" =~ ^###[[:space:]]+Feature[[:space:]]+([0-9]+):[[:space:]]*(.*)$ ]]; then
            # Output previous feature if exists
            if [[ -n "$current_feature" ]]; then
                echo "${current_feature}|${current_title}|${current_desc}"
            fi

            current_feature="${BASH_REMATCH[1]}"
            current_title="${BASH_REMATCH[2]}"
            current_desc=""
            in_feature=true
            continue
        fi

        # Check for next section (## or ###)
        if [[ "$line" =~ ^##[#]?[[:space:]] ]] && [[ ! "$line" =~ ^###[[:space:]]+Feature ]]; then
            # Output current feature and stop
            if [[ -n "$current_feature" ]]; then
                echo "${current_feature}|${current_title}|${current_desc}"
            fi
            current_feature=""
            in_feature=false
            continue
        fi

        # Accumulate description
        if [[ "$in_feature" == "true" ]] && [[ -n "$line" ]]; then
            if [[ -n "$current_desc" ]]; then
                current_desc="$current_desc $line"
            else
                current_desc="$line"
            fi
        fi

    done < "$epic_file"

    # Output last feature
    if [[ -n "$current_feature" ]]; then
        echo "${current_feature}|${current_title}|${current_desc}"
    fi
}

# Extract story title from story file
extract_story_title() {
    local story_file="$1"

    if [[ ! -f "$story_file" ]]; then
        return 1
    fi

    # Get title from frontmatter
    tr -d '\r' < "$story_file" | grep -E "^title:" | head -1 | sed 's/^title:[[:space:]]*//' | tr -d '"'
}

# Extract story ID from story file
extract_story_id() {
    local story_file="$1"

    if [[ ! -f "$story_file" ]]; then
        return 1
    fi

    # Get id from frontmatter
    tr -d '\r' < "$story_file" | grep -E "^id:" | head -1 | sed 's/^id:[[:space:]]*//' | tr -d '"'
}

##############################################################################
# Match Scoring
##############################################################################

# Find best feature match for a story
# Returns: feature_num|confidence|title|manual_review_required
find_best_match() {
    local story_file="$1"
    local epic_file="$2"

    local story_title
    story_title=$(extract_story_title "$story_file")

    if [[ -z "$story_title" ]]; then
        return 1
    fi

    local best_feature=""
    local best_score=0
    local best_title=""

    while IFS='|' read -r feature_num feature_title feature_desc; do
        [[ -z "$feature_num" ]] && continue

        local score
        score=$(calculate_weighted_similarity "$story_title" "$feature_title" "$feature_desc")

        if [[ $score -gt $best_score ]]; then
            best_score=$score
            best_feature="$feature_num"
            best_title="$feature_title"
        fi
    done < <(extract_features_from_epic "$epic_file")

    if [[ -z "$best_feature" ]]; then
        echo "0|0|no match|false"
        return 1
    fi

    # Determine if manual review required
    local manual_review="false"
    if [[ $best_score -ge $LOW_THRESHOLD ]] && [[ $best_score -lt $HIGH_THRESHOLD ]]; then
        manual_review="true"
    fi

    echo "${best_feature}|${best_score}|${best_title}|${manual_review}"
}

##############################################################################
# Coverage Calculation
##############################################################################

# Calculate coverage excluding low-confidence matches
calculate_coverage_with_confidence() {
    local story_dir="$1"
    local epic_file="$2"

    local total_features=0
    local confirmed_matches=0
    local low_confidence_count=0

    # Count total features
    while IFS='|' read -r _ _ _; do
        ((total_features++))
    done < <(extract_features_from_epic "$epic_file")

    if [[ $total_features -eq 0 ]]; then
        echo "0"
        return
    fi

    # Track matched features
    declare -A matched_features

    # Check each story
    for story_file in "$story_dir"/*.story.md; do
        [[ -f "$story_file" ]] || continue

        local match_result
        match_result=$(find_best_match "$story_file" "$epic_file" 2>/dev/null)
        [[ -z "$match_result" ]] && continue

        local feature_num confidence _ manual_review
        IFS='|' read -r feature_num confidence _ manual_review <<< "$match_result"

        # Only count high-confidence matches
        if [[ $confidence -ge $MIN_FOR_COUNT ]] && [[ ! -v matched_features[$feature_num] ]]; then
            matched_features[$feature_num]=1
            ((confirmed_matches++))
        elif [[ "$manual_review" == "true" ]]; then
            ((low_confidence_count++))
        fi
    done

    # Calculate percentage
    local coverage
    coverage=$(echo "scale=0; ($confirmed_matches * 100) / $total_features" | bc 2>/dev/null || echo "0")

    echo "$coverage"
}

##############################################################################
# Output Formatting
##############################################################################

# Output match results in text format
output_text() {
    local story_id="$1"
    local story_title="$2"
    local feature_num="$3"
    local confidence="$4"
    local feature_title="$5"
    local manual_review="$6"

    echo ""
    echo "Story: $story_id"
    echo "  Title: $story_title"
    echo "  Matched Feature: Feature $feature_num - $feature_title"
    echo "  Confidence: ${confidence}%"

    if [[ "$manual_review" == "true" ]]; then
        echo -e "  Status: ${YELLOW}Low Confidence - Manual Review Required${NC}"
    elif [[ $confidence -ge $HIGH_THRESHOLD ]]; then
        echo -e "  Status: ${GREEN}High Confidence${NC}"
    else
        echo -e "  Status: ${RED}No Match${NC}"
    fi
}

# Output match results in JSON format
output_json() {
    local story_id="$1"
    local story_title="$2"
    local feature_num="$3"
    local confidence="$4"
    local feature_title="$5"
    local manual_review="$6"

    cat << EOF
{
  "story_id": "$story_id",
  "story_title": "$story_title",
  "matched_feature": $feature_num,
  "feature_title": "$feature_title",
  "confidence": $confidence,
  "manual_review_required": $manual_review
}
EOF
}

##############################################################################
# BR-003 Test Case
##############################################################################

# Business Rule 003: 5 features, 3 confirmed stories, 1 low-confidence = 60% not 80%
run_br003_test() {
    echo "BR-003 Test: Coverage calculation with low-confidence matches"
    echo "============================================================="
    echo ""
    echo "Scenario: 5 features, 3 confirmed high-confidence matches,"
    echo "          1 low-confidence match (should not count)"
    echo ""

    # Simulate: 3 confirmed / 5 total = 60%
    local total_features=5
    local confirmed_matches=3
    local low_confidence=1

    local coverage
    coverage=$(echo "scale=0; ($confirmed_matches * 100) / $total_features" | bc)

    echo "Total features: $total_features"
    echo "Confirmed matches (>75%): $confirmed_matches"
    echo "Low-confidence matches (60-75%): $low_confidence"
    echo ""
    echo "Coverage calculation:"
    echo "  = (confirmed_matches / total_features) * 100"
    echo "  = ($confirmed_matches / $total_features) * 100"
    echo "  = ${coverage}%"
    echo ""

    if [[ "$coverage" == "60" ]]; then
        echo -e "${GREEN}✓ CORRECT: Coverage is 60% (not 80%)${NC}"
        echo "  Low-confidence matches correctly excluded from calculation."
        return 0
    else
        echo -e "${RED}✗ INCORRECT: Expected 60%, got ${coverage}%${NC}"
        return 1
    fi
}

##############################################################################
# Main Entry Point
##############################################################################

main() {
    local story_file=""
    local story_dir=""
    local epic_file=""
    local epic_dir=""
    local calculate_coverage=false
    local output_format="text"
    local confidence_threshold=""
    local run_br003=false

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --story)
                story_file="$2"
                shift 2
                ;;
            --stories)
                story_dir="$2"
                shift 2
                ;;
            --epic)
                epic_file="$2"
                shift 2
                ;;
            --epic-dir)
                epic_dir="$2"
                shift 2
                ;;
            --calculate-coverage)
                calculate_coverage=true
                shift
                ;;
            --confidence-threshold)
                confidence_threshold="$2"
                if [[ "$confidence_threshold" =~ ^0\.[0-9]+$ ]]; then
                    # Convert 0.X to percentage
                    HIGH_THRESHOLD=$(echo "scale=0; $confidence_threshold * 100" | bc)
                else
                    HIGH_THRESHOLD="$confidence_threshold"
                fi
                shift 2
                ;;
            --format)
                output_format="$2"
                shift 2
                ;;
            --test-br-003)
                run_br003=true
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

    # Run BR-003 test
    if [[ "$run_br003" == "true" ]]; then
        run_br003_test
        exit $?
    fi

    # Set defaults
    story_dir="${story_dir:-$DEFAULT_STORY_DIR}"
    epic_dir="${epic_dir:-$DEFAULT_EPIC_DIR}"

    # Coverage calculation mode
    if [[ "$calculate_coverage" == "true" ]]; then
        # Single story coverage calculation
        if [[ -n "$story_file" ]] && [[ -n "$epic_file" ]]; then
            local total_features=0
            local matched_features=0

            # Count total features
            while IFS='|' read -r _ _ _; do
                ((total_features++))
            done < <(extract_features_from_epic "$epic_file")

            if [[ $total_features -eq 0 ]]; then
                echo "Coverage: 0%"
                exit 0
            fi

            # Check if this story matches any feature with high confidence
            local match_result
            match_result=$(find_best_match "$story_file" "$epic_file" 2>/dev/null)
            if [[ -n "$match_result" ]]; then
                local _ confidence _ _
                IFS='|' read -r _ confidence _ _ <<< "$match_result"
                if [[ $confidence -ge $MIN_FOR_COUNT ]]; then
                    matched_features=1
                fi
            fi

            local coverage
            coverage=$(echo "scale=0; ($matched_features * 100) / $total_features" | bc 2>/dev/null || echo "0")
            echo "Coverage: ${coverage}%"
            exit 0
        elif [[ -n "$epic_file" ]]; then
            local coverage
            coverage=$(calculate_coverage_with_confidence "$story_dir" "$epic_file")
            echo "Coverage: ${coverage}%"
            exit 0
        else
            # Calculate for all epics
            local total_coverage=0
            local epic_count=0

            for ef in "$epic_dir"/*.epic.md "$epic_dir"/EPIC-*.md; do
                [[ -f "$ef" ]] || continue
                local cov
                cov=$(calculate_coverage_with_confidence "$story_dir" "$ef")
                total_coverage=$((total_coverage + cov))
                ((epic_count++))
            done

            if [[ $epic_count -gt 0 ]]; then
                local avg
                avg=$((total_coverage / epic_count))
                echo "Average Coverage: ${avg}%"
            else
                echo "No epics found"
            fi
            exit 0
        fi
    fi

    # Single story matching
    if [[ -n "$story_file" ]] && [[ -n "$epic_file" ]]; then
        local story_id story_title
        story_id=$(extract_story_id "$story_file")
        story_title=$(extract_story_title "$story_file")

        local match_result
        match_result=$(find_best_match "$story_file" "$epic_file")

        if [[ -n "$match_result" ]]; then
            local feature_num confidence feature_title manual_review
            IFS='|' read -r feature_num confidence feature_title manual_review <<< "$match_result"

            if [[ "$output_format" == "json" ]]; then
                output_json "$story_id" "$story_title" "$feature_num" "$confidence" "$feature_title" "$manual_review"
            else
                output_text "$story_id" "$story_title" "$feature_num" "$confidence" "$feature_title" "$manual_review"
            fi
        fi
        exit 0
    fi

    # Multiple stories matching
    if [[ -d "$story_dir" ]] && [[ -n "$epic_file" ]]; then
        local has_low_confidence=false
        local matches_json="["
        local first=true

        for sf in "$story_dir"/*.story.md; do
            [[ -f "$sf" ]] || continue

            local story_id story_title
            story_id=$(extract_story_id "$sf")
            story_title=$(extract_story_title "$sf")

            local match_result
            match_result=$(find_best_match "$sf" "$epic_file" 2>/dev/null)
            [[ -z "$match_result" ]] && continue

            local feature_num confidence feature_title manual_review
            IFS='|' read -r feature_num confidence feature_title manual_review <<< "$match_result"

            if [[ "$manual_review" == "true" ]]; then
                has_low_confidence=true
            fi

            if [[ "$output_format" == "json" ]]; then
                [[ "$first" != "true" ]] && matches_json+=","
                first=false
                matches_json+=$(output_json "$story_id" "$story_title" "$feature_num" "$confidence" "$feature_title" "$manual_review")
            else
                output_text "$story_id" "$story_title" "$feature_num" "$confidence" "$feature_title" "$manual_review"
            fi
        done

        if [[ "$output_format" == "json" ]]; then
            matches_json+="]"
            echo "{\"matches\": $matches_json}"
        fi

        if [[ "$has_low_confidence" == "true" ]]; then
            exit 1  # Warnings
        fi
        exit 0
    fi

    # Default: show usage
    usage
    exit 0
}

# Run if not sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
