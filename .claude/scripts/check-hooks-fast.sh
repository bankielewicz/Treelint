#!/bin/bash
# Fast check-hooks implementation for audit-deferrals
# Bypasses Python startup overhead for <100ms requirement
#
# Usage: check-hooks-fast.sh audit-deferrals success
# Exit codes: 0 = trigger, 1 = don't trigger, 2 = error

set -euo pipefail

OPERATION="${1:-}"
STATUS="${2:-}"
CONFIG_FILE="${3:-devforgeai/config/hooks.yaml}"

# Validate arguments
if [ -z "$OPERATION" ] || [ -z "$STATUS" ]; then
  echo "ERROR: Missing required arguments" >&2
  echo "Usage: check-hooks-fast.sh OPERATION STATUS [CONFIG_FILE]" >&2
  exit 2
fi

# Check for circular invocation
if [ -n "${DEVFORGEAI_HOOK_ACTIVE:-}" ]; then
  echo "Circular hook invocation detected, skipping" >&2
  exit 1
fi

# Check if config exists
if [ ! -f "$CONFIG_FILE" ]; then
  echo "Hooks config not found, assuming disabled" >&2
  exit 1
fi

# Fast YAML parsing using grep (no Python overhead)
# Extract enabled status
enabled=$(grep "^enabled:" "$CONFIG_FILE" | awk '{print $2}' | tr -d ' ')
if [ "$enabled" != "true" ]; then
  echo "Hooks are disabled in configuration" >&2
  exit 1
fi

# Extract trigger_on rule for operation (with fallback to global)
op_trigger=$(grep -A 2 "^  $OPERATION:" "$CONFIG_FILE" | grep "trigger_on:" | awk '{print $2}' | tr -d ' ')
if [ -z "$op_trigger" ]; then
  # Fallback to global rule
  op_trigger=$(grep "^  trigger_on:" "$CONFIG_FILE" | head -1 | awk '{print $2}' | tr -d ' ')
fi

# Apply trigger rules
case "$op_trigger" in
  "all")
    # Trigger on any status
    exit 0
    ;;
  "failures-only")
    # Trigger only on failure or partial
    if [ "$STATUS" = "failure" ] || [ "$STATUS" = "partial" ]; then
      exit 0
    else
      exit 1
    fi
    ;;
  "none"|"")
    # Don't trigger
    exit 1
    ;;
  *)
    echo "Invalid trigger_on rule: $op_trigger" >&2
    exit 1
    ;;
esac
