#!/bin/bash
# Reusable helper function for feedback hook integration
# Used by: /ideate, /create-story, /create-epic, /create-sprint, /create-context, /dev
# Pattern: STORY-023 pilot, implemented across STORY-027-031

set -euo pipefail

# Usage: invoke_feedback_hooks.sh <operation> <status> [additional-args...]
# Examples:
#   invoke_feedback_hooks.sh ideate completed --artifacts='[epic1.md]' --complexity-score=42
#   invoke_feedback_hooks.sh dev completed --story=STORY-001
#   invoke_feedback_hooks.sh create-context completed

OPERATION="${1:-}"
STATUS="${2:-completed}"
shift 2 || true  # Remove first 2 args, keep remaining as passthrough

# Validate required arguments
if [ -z "$OPERATION" ]; then
    echo "❌ ERROR: Operation required" >&2
    echo "Usage: invoke_feedback_hooks.sh <operation> <status> [additional-args...]" >&2
    exit 1
fi

# Phase 1: Check Hook Eligibility
echo "Checking hook eligibility for operation: $OPERATION..."

devforgeai check-hooks --operation="$OPERATION" --status="$STATUS" 2>/dev/null
CHECK_EXIT=$?

# Interpret exit code
case $CHECK_EXIT in
    0)
        # Eligible - proceed to invocation
        echo "✓ Hooks eligible for $OPERATION"
        ;;
    1)
        # Not eligible (disabled/rate-limited) - silent exit
        exit 0
        ;;
    *)
        # Unexpected error - warn and continue
        echo "⚠️ Hook eligibility check failed (exit code: $CHECK_EXIT), continuing..." >&2
        exit 0
        ;;
esac

# Phase 2: Invoke Feedback Hooks
echo "Invoking feedback hooks for $OPERATION..."

devforgeai invoke-hooks --operation="$OPERATION" "$@" 2>/dev/null || {
    # Hook invocation failed - non-blocking
    echo "⚠️ Post-$OPERATION feedback skipped (hook system unavailable)" >&2
    exit 0
}

# Phase 3: Display Success
echo "✓ Post-$OPERATION feedback initiated"
exit 0
