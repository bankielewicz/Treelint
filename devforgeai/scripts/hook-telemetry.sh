#!/bin/bash
################################################################################
# hook-telemetry.sh - Generate hook effectiveness metrics
# STORY-200: Hook Telemetry Metrics
#
# Usage:
#   hook-telemetry.sh --log <log-file> [--unknown <unknown-file>]
#
# Parameters:
#   --log      Path to pre-tool-use.log file (required)
#   --unknown  Path to hook-unknown-commands.log file (optional)
#
# Output:
#   Telemetry report with counts and approval rate
#
# Exit Codes:
#   0 - Success
#   1 - Missing required parameter or file not found
################################################################################

# Configuration constants
readonly APPROVAL_RATE_TARGET=90

# ============================================================================
# UTILITY FUNCTIONS
# ============================================================================

# Count lines matching a pattern in a file, defaulting to 0
# Usage: count_matches <pattern> <file>
count_matches() {
    local pattern="$1"
    local file="$2"
    local count
    count=$(grep -c "$pattern" "$file" 2>/dev/null || echo "0")
    if [ -z "$count" ]; then
        count=0
    fi
    echo "$count"
}

# Count non-comment, non-empty lines in a file
# Usage: count_data_lines <file>
count_data_lines() {
    local file="$1"
    local count
    count=$(grep -cvE "^#|^$" "$file" 2>/dev/null || echo "0")
    if [ -z "$count" ]; then
        count=0
    fi
    echo "$count"
}

# Calculate percentage with one decimal place
# Usage: calculate_percentage <numerator> <denominator>
calculate_percentage() {
    local numerator="$1"
    local denominator="$2"
    if [ "$denominator" -gt 0 ]; then
        awk "BEGIN {printf \"%.1f\", ($numerator / $denominator) * 100}"
    else
        echo "0.0"
    fi
}

# Get integer portion of a decimal number
# Usage: get_integer_part <decimal>
get_integer_part() {
    local value="$1"
    local int_part="${value%.*}"
    if [ -z "$int_part" ]; then
        int_part=0
    fi
    echo "$int_part"
}

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

# Parse arguments
LOG_FILE=""
UNKNOWN_FILE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --log)
            LOG_FILE="$2"
            shift 2
            ;;
        --unknown)
            UNKNOWN_FILE="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Validate log file parameter
if [ -z "$LOG_FILE" ]; then
    echo "ERROR: --log parameter is required"
    exit 1
fi

# Validate log file exists
if [ ! -f "$LOG_FILE" ]; then
    echo "ERROR: Log file not found: $LOG_FILE"
    exit 1
fi

# ============================================================================
# METRICS CALCULATION
# ============================================================================

# Calculate metrics using extracted functions (DRY principle)
total=$(count_data_lines "$LOG_FILE")
auto_approved=$(count_matches "Decision: AUTO-APPROVE" "$LOG_FILE")
blocked=$(count_matches "Decision: BLOCK" "$LOG_FILE")
manual=$(count_matches "Decision: ASK USER" "$LOG_FILE")

# Calculate approval rate
rate=$(calculate_percentage "$auto_approved" "$total")

# ============================================================================
# REPORT OUTPUT
# ============================================================================

# Output report header
echo "=== Hook Telemetry Report ==="
echo "Date: $(date +%Y-%m-%d)"
echo ""

# Output metrics
echo "Total invocations: $total"
echo "Auto-approved: $auto_approved"
echo "Blocked: $blocked"
echo "Manual approval: $manual"
echo ""

# Output approval rate
echo "Approval rate: ${rate}%"

# Warning if rate below target
rate_int=$(get_integer_part "$rate")
if [ "$rate_int" -lt "$APPROVAL_RATE_TARGET" ]; then
    echo "WARNING: Approval rate below ${APPROVAL_RATE_TARGET}% target"
fi

# ============================================================================
# UNKNOWN PATTERNS REPORT
# ============================================================================

echo ""
process_unknown_patterns() {
    local file="$1"

    if [ -z "$file" ]; then
        echo "Top 10 unknown patterns:"
        echo "  (No unknown patterns file specified)"
        return
    fi

    if [ ! -f "$file" ]; then
        echo "Unknown patterns file not found: $file (skipping)"
        return
    fi

    # Count patterns (excluding comments)
    local pattern_count
    pattern_count=$(count_data_lines "$file")

    if [ "$pattern_count" -eq 0 ]; then
        echo "No unknown patterns found"
        return
    fi

    echo "Top 10 unknown patterns:"
    # Extract pattern prefixes, group by first 2 words, count occurrences
    # Pattern format: "Pattern: cd /mnt/c/Projects/MyApp" -> extract "cd /mnt"
    grep -v "^#" "$file" | \
        sed 's/.*Pattern: //' | \
        awk '{
            # Get command prefix (first word and second word if path-like)
            if ($1 == "cd" || $1 == "python3" || $1 == "bash" || $1 == "node") {
                # For commands with arguments, group by first 2 tokens
                split($2, parts, "/")
                if (parts[1] == "" && parts[2] != "") {
                    # Absolute path like /mnt/c/...
                    print $1 " /" parts[2]
                } else {
                    print $1 " " $2
                }
            } else {
                # For other commands, just use first word
                print $1
            }
        }' | \
        sort | uniq -c | sort -rn | head -10 | \
        awk '{printf "  %d. %s", NR, $2; for(i=3;i<=NF;i++) printf " %s", $i; printf " (%d occurrences)\n", $1}'
}

process_unknown_patterns "$UNKNOWN_FILE"
