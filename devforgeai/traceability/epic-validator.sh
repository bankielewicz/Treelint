#!/bin/bash

##############################################################################
# Epic Validator - STORY-089 AC#1
# Purpose: Validate epic structure for /create-epic workflow
#
# Validates:
# - Epic has at least one feature defined
# - Each feature has unique identifier (Feature N: format)
# - Feature descriptions are non-empty (min 10 chars)
# - Epic frontmatter contains required fields (epic_id, title, status, priority)
#
# Exit codes:
# - 0: Validation passed
# - 1: Validation failed (specific errors)
# - 2: File not found or unreadable
# - 3: Internal error
##############################################################################

set -o pipefail

# Script info
SCRIPT_NAME="epic-validator.sh"
VERSION="1.0.0"

# Paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR}/../.."

# Minimum description length
MIN_DESCRIPTION_LENGTH=10

# Required frontmatter fields (epic_id OR id is accepted)
REQUIRED_FIELDS=("title" "status" "priority")
EPIC_ID_FIELDS=("epic_id" "id")  # Either one is acceptable

##############################################################################
# Helper Functions
##############################################################################

log_error() {
    echo "ERROR: $1" >&2
}

log_warn() {
    echo "WARNING: $1" >&2
}

log_info() {
    echo "INFO: $1"
}

usage() {
    cat << EOF
Usage: $SCRIPT_NAME [OPTIONS] <epic-file>

Validate epic file structure for /create-epic workflow.

Options:
    --validate-epic <file>    Validate a single epic file
    --validate-dir <dir>      Validate all epic files in directory
    --verbose                 Show detailed output
    --json                    Output results as JSON
    --help                    Show this help message
    --version                 Show version

Exit Codes:
    0    Validation passed
    1    Validation failed (see errors)
    2    File not found or unreadable
    3    Internal error

Examples:
    $SCRIPT_NAME --validate-epic devforgeai/specs/Epics/EPIC-015.epic.md
    $SCRIPT_NAME --validate-dir devforgeai/specs/Epics/
EOF
}

##############################################################################
# Frontmatter Parsing
##############################################################################

extract_frontmatter() {
    local file="$1"

    # Check if file starts with --- (handle Windows line endings)
    local first_line
    first_line=$(head -1 "$file" | tr -d '\r')

    if [[ "$first_line" != "---" ]]; then
        echo ""
        return 1
    fi

    # Extract content between --- delimiters (strip Windows line endings)
    awk '/^---/{if(++n==2)exit}n==1' "$file" | tail -n +1 | tr -d '\r'
}

get_frontmatter_field() {
    local frontmatter="$1"
    local field="$2"

    echo "$frontmatter" | grep -E "^${field}:" | sed "s/^${field}:[[:space:]]*//" | sed 's/^"\(.*\)"$/\1/' | sed "s/^'\(.*\)'$/\1/"
}

validate_frontmatter() {
    local file="$1"
    local errors=()

    # Extract frontmatter
    local frontmatter
    frontmatter=$(extract_frontmatter "$file")

    if [[ -z "$frontmatter" ]]; then
        echo "Missing frontmatter delimiters (---)"
        return 1
    fi

    # Check for truncated YAML (missing closing ---) - handle Windows line endings
    local delimiter_count
    delimiter_count=$(tr -d '\r' < "$file" | grep -c "^---$" 2>/dev/null || echo "0")
    if [[ "$delimiter_count" -lt 2 ]]; then
        echo "Missing closing frontmatter delimiter (---)"
        return 1
    fi

    # Check required fields
    for field in "${REQUIRED_FIELDS[@]}"; do
        local value
        value=$(get_frontmatter_field "$frontmatter" "$field")

        if [[ -z "$value" ]]; then
            errors+=("Missing required field: $field")
        fi
    done

    # Check for epic_id OR id (either is acceptable)
    local has_epic_id=false
    for id_field in "${EPIC_ID_FIELDS[@]}"; do
        local value
        value=$(get_frontmatter_field "$frontmatter" "$id_field")
        if [[ -n "$value" ]]; then
            has_epic_id=true
            break
        fi
    done
    if [[ "$has_epic_id" == "false" ]]; then
        errors+=("Missing required field: epic_id (or id)")
    fi

    if [[ ${#errors[@]} -gt 0 ]]; then
        printf '%s\n' "${errors[@]}"
        return 1
    fi

    return 0
}

##############################################################################
# Feature Parsing
##############################################################################

extract_features() {
    local file="$1"

    # Extract Feature N: patterns from headers (handle Windows line endings)
    tr -d '\r' < "$file" | grep -E "^###[[:space:]]+Feature[[:space:]]+[0-9]+:" 2>/dev/null || true
}

get_feature_ids() {
    local file="$1"

    # Extract just the feature numbers from Feature N headers only
    tr -d '\r' < "$file" | grep -E "^###[[:space:]]+Feature[[:space:]]+[0-9]+:" | grep -oE "Feature[[:space:]]+[0-9]+" | grep -oE "[0-9]+" || true
}

get_feature_description() {
    local file="$1"
    local feature_num="$2"

    # Get content after Feature N: header until next ## or ### header
    awk -v num="$feature_num" '
        /^###[[:space:]]+Feature[[:space:]]+'"$feature_num"':/ {
            found=1
            next
        }
        found && /^##/ {
            exit
        }
        found {
            gsub(/^[[:space:]]+|[[:space:]]+$/, "")
            if (length > 0) print
        }
    ' "$file" | head -5 | tr '\n' ' ' | sed 's/[[:space:]]*$//'
}

validate_features() {
    local file="$1"
    local errors=()

    # Get all features
    local features
    features=$(extract_features "$file")

    # Check for at least one feature
    if [[ -z "$features" ]]; then
        echo "Epic must have at least one feature defined (use ### Feature N: format)"
        return 1
    fi

    # Get feature IDs
    local feature_ids
    feature_ids=$(get_feature_ids "$file")

    # Check for duplicates
    local unique_ids
    unique_ids=$(echo "$feature_ids" | sort -u)
    local total_count
    total_count=$(echo "$feature_ids" | wc -l)
    local unique_count
    unique_count=$(echo "$unique_ids" | wc -l)

    if [[ "$total_count" -ne "$unique_count" ]]; then
        # Find duplicate IDs
        local duplicates
        duplicates=$(echo "$feature_ids" | sort | uniq -d)
        errors+=("Duplicate feature IDs found: Feature $duplicates")
    fi

    # Check feature ID format and descriptions
    while IFS= read -r id; do
        if [[ -z "$id" ]]; then
            continue
        fi

        # Get description for this feature
        local desc
        desc=$(get_feature_description "$file" "$id")

        # Check description length
        local desc_length
        desc_length=${#desc}

        if [[ $desc_length -eq 0 ]]; then
            errors+=("Feature $id has empty description")
        elif [[ $desc_length -lt $MIN_DESCRIPTION_LENGTH ]]; then
            errors+=("Feature $id description too short ($desc_length chars, minimum 10 characters required)")
        fi
    done <<< "$feature_ids"

    # Check for invalid format (e.g., "Feat 1" instead of "Feature 1")
    local invalid_formats
    invalid_formats=$(grep -E "^###[[:space:]]+(Feat|FEATURE|feature)[[:space:]]+[0-9]+:" "$file" 2>/dev/null || true)
    if [[ -n "$invalid_formats" ]]; then
        errors+=("Invalid feature format detected. Use 'Feature N:' (exact case)")
    fi

    if [[ ${#errors[@]} -gt 0 ]]; then
        printf '%s\n' "${errors[@]}"
        return 1
    fi

    return 0
}

##############################################################################
# YAML Validation
##############################################################################

validate_yaml_structure() {
    local file="$1"

    # Check for common YAML errors in frontmatter
    local frontmatter
    frontmatter=$(extract_frontmatter "$file")

    if [[ -z "$frontmatter" ]]; then
        return 0  # No frontmatter to validate
    fi

    # Check for invalid indentation (spaces after colon in key)
    local line_num=1
    while IFS= read -r line; do
        # Check for invalid indent after colon-space
        if [[ "$line" =~ ^[[:space:]]+ ]] && [[ ! "$line" =~ ^[[:space:]]+-[[:space:]] ]]; then
            # Check if this looks like a badly indented value
            if [[ "$line" =~ ^[[:space:]]+[a-z_]+: ]]; then
                echo "YAML error at line $((line_num + 1)): invalid indentation. Suggestion: Ensure consistent indentation for nested values."
                return 1
            fi
        fi
        line_num=$((line_num + 1))
    done <<< "$frontmatter"

    return 0
}

##############################################################################
# Main Validation Function
##############################################################################

validate_epic() {
    local file="$1"
    local verbose="${2:-false}"
    local all_errors=()
    local validation_passed=true

    # Check file exists and is readable
    if [[ ! -f "$file" ]]; then
        log_error "File not found: $file"
        return 2
    fi

    if [[ ! -r "$file" ]]; then
        log_error "File not readable: $file"
        return 2
    fi

    # Validate YAML structure
    local yaml_errors
    yaml_errors=$(validate_yaml_structure "$file")
    if [[ -n "$yaml_errors" ]]; then
        all_errors+=("$yaml_errors")
        validation_passed=false
    fi

    # Validate frontmatter
    local fm_errors
    fm_errors=$(validate_frontmatter "$file")
    if [[ $? -ne 0 ]]; then
        while IFS= read -r err; do
            all_errors+=("$err")
        done <<< "$fm_errors"
        validation_passed=false
    fi

    # Validate features
    local feat_errors
    feat_errors=$(validate_features "$file")
    if [[ $? -ne 0 ]]; then
        while IFS= read -r err; do
            all_errors+=("$err")
        done <<< "$feat_errors"
        validation_passed=false
    fi

    # Output results
    if [[ "$validation_passed" == "true" ]]; then
        if [[ "$verbose" == "true" ]]; then
            log_info "Validation PASSED: $file"
        fi
        return 0
    else
        echo "Validation FAILED for: $file"
        for err in "${all_errors[@]}"; do
            echo "  - $err"
        done

        # Provide fix suggestions
        echo ""
        echo "Fix suggestions:"
        echo "  - Ensure frontmatter has: epic_id, title, status, priority"
        echo "  - Use format: ### Feature N: Title"
        echo "  - Feature descriptions should be at least 10 characters"
        echo ""
        echo "Example correct format:"
        echo "---"
        echo "epic_id: EPIC-001"
        echo "title: My Epic Title"
        echo "status: Planning"
        echo "priority: High"
        echo "---"
        echo ""
        echo "## Features"
        echo ""
        echo "### Feature 1: Feature Title"
        echo ""
        echo "Description of the feature (at least 10 characters)."

        return 1
    fi
}

validate_directory() {
    local dir="$1"
    local verbose="${2:-false}"
    local valid_count=0
    local invalid_count=0
    local error_files=()

    if [[ ! -d "$dir" ]]; then
        log_error "Directory not found: $dir"
        return 2
    fi

    # Find all epic files
    local epic_files
    epic_files=$(find "$dir" -name "*.epic.md" -o -name "epic-*.md" 2>/dev/null)

    if [[ -z "$epic_files" ]]; then
        log_warn "No epic files found in: $dir"
        return 0
    fi

    while IFS= read -r file; do
        if [[ -z "$file" ]]; then
            continue
        fi

        if validate_epic "$file" "$verbose" > /dev/null 2>&1; then
            valid_count=$((valid_count + 1))
        else
            invalid_count=$((invalid_count + 1))
            error_files+=("$file")
        fi
    done <<< "$epic_files"

    echo ""
    echo "Validation Summary:"
    echo "  Files validated: $((valid_count + invalid_count))"
    echo "  Valid: $valid_count"
    echo "  Invalid: $invalid_count"

    if [[ $invalid_count -gt 0 ]]; then
        echo ""
        echo "Files with errors:"
        for f in "${error_files[@]}"; do
            echo "  - $f"
        done
        return 1
    fi

    return 0
}

##############################################################################
# JSON Output
##############################################################################

output_json() {
    local file="$1"
    local result="$2"
    local errors="$3"

    cat << EOF
{
  "file": "$file",
  "valid": $result,
  "errors": [
$(echo "$errors" | sed 's/^/    "/; s/$/"/' | paste -sd ',' -)
  ]
}
EOF
}

##############################################################################
# Main Entry Point
##############################################################################

main() {
    local mode=""
    local target=""
    local verbose=false
    local json_output=false

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --validate-epic)
                mode="epic"
                target="$2"
                shift 2
                ;;
            --validate-dir)
                mode="dir"
                target="$2"
                shift 2
                ;;
            --verbose)
                verbose=true
                shift
                ;;
            --json)
                json_output=true
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
                # Assume it's a file path
                if [[ -z "$target" ]]; then
                    mode="epic"
                    target="$1"
                fi
                shift
                ;;
        esac
    done

    if [[ -z "$target" ]]; then
        usage
        exit 3
    fi

    case "$mode" in
        epic)
            validate_epic "$target" "$verbose"
            exit $?
            ;;
        dir)
            validate_directory "$target" "$verbose"
            exit $?
            ;;
        *)
            usage
            exit 3
            ;;
    esac
}

# Run if not sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
