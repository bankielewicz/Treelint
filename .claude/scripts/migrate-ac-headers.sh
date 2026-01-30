#!/bin/bash
# Migrate story template v2.0 → v2.1 (remove AC header checkboxes)
# Usage: migrate-ac-headers.sh <story-file-or-directory>

TARGET="$1"

if [[ -z "$TARGET" ]]; then
  echo "Usage: migrate-ac-headers.sh <story-file-or-directory>"
  echo ""
  echo "Examples:"
  echo "  migrate-ac-headers.sh devforgeai/specs/Stories/STORY-052.story.md"
  echo "  migrate-ac-headers.sh devforgeai/specs/Stories/"
  exit 1
fi

# Track if any file migration failed
MIGRATION_FAILED=0

# ============================================================================
# Helper: update_format_version(file)
# Updates format_version from 2.0 to 2.1 in YAML frontmatter
# Handles three quote formats: double quotes, single quotes, and unquoted
# ============================================================================
update_format_version() {
  local file="$1"

  # Handle double quotes: format_version: "2.0"
  sed -i 's/format_version: "2.0"/format_version: "2.1"/' "$file"

  # Handle single quotes: format_version: '2.0'
  sed -i "s/format_version: '2.0'/format_version: \"2.1\"/" "$file"

  # Handle unquoted: format_version: 2.0
  sed -i 's/format_version: 2.0$/format_version: "2.1"/' "$file"
}

# ============================================================================
# Helper: migrate_ac_headers(file)
# Converts AC header format from v2.0 to v2.1
# Changes: ### N. [ ] Title → ### AC#N: Title
# ============================================================================
migrate_ac_headers() {
  local file="$1"

  sed -i 's/^### \([0-9]\+\)\. \[ \] /### AC#\1: /' "$file"
}

# ============================================================================
# Helper: backup_file(file)
# Creates backup copy of file with .backup extension
# ============================================================================
backup_file() {
  local file="$1"

  cp "$file" "$file.backup"
}

# ============================================================================
# Function: migrate_file(file)
# Main entry point for migrating a single story file
# Performs: backup → migrate AC headers → update format version
# ============================================================================
migrate_file() {
  local file="$1"

  if [[ ! -f "$file" ]]; then
    echo "Error: File not found: $file"
    return 1
  fi

  backup_file "$file"
  migrate_ac_headers "$file"
  update_format_version "$file"

  echo "Migrated: $file"
  echo "  Backup: $file.backup"
}

# ============================================================================
# Main Logic: Process TARGET (file or directory)
# ============================================================================

if [[ -d "$TARGET" ]]; then
  # Directory mode - migrate all story files
  # Tracks failures and reports summary
  FILES_PROCESSED=0
  FILES_FAILED=0

  for file in "$TARGET"/*.story.md; do
    if [[ -f "$file" ]]; then
      FILES_PROCESSED=$((FILES_PROCESSED + 1))
      if ! migrate_file "$file"; then
        FILES_FAILED=$((FILES_FAILED + 1))
        MIGRATION_FAILED=1
      fi
    fi
  done

  # Report directory migration summary
  if [[ $FILES_PROCESSED -gt 0 ]]; then
    echo ""
    echo "Directory migration complete:"
    echo "  Files processed: $FILES_PROCESSED"
    if [[ $FILES_FAILED -gt 0 ]]; then
      echo "  Files failed:    $FILES_FAILED"
    fi
  fi
else
  # Single file mode
  if ! migrate_file "$TARGET"; then
    MIGRATION_FAILED=1
  fi
fi

echo ""
echo "Migration complete!"
echo "To undo, restore from .backup files"

# Exit with error if migration failed
if [[ $MIGRATION_FAILED -eq 1 ]]; then
  exit 1
fi
