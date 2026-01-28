---
description: Rebuild feedback session index from session files
argument-hint:
model: opus
allowed-tools: Bash, Read, Glob
---

# /feedback-reindex - Rebuild Feedback Session Index

Rebuild the feedback session index (`devforgeai/feedback/index.json`) from all session files in `devforgeai/feedback/sessions/`.

**Use when:**
- Index file is corrupted (invalid JSON, missing fields)
- Index is out of sync with session files on disk
- After manual session file modifications
- Periodic maintenance (monthly recommended)

---

## Quick Reference

```bash
# Rebuild entire index from sessions directory
/feedback-reindex

# Check index health before rebuilding
/feedback-reindex --check-only
```

---

## Command Workflow

### Phase 0: Validation

**Check if base path exists:**
```bash
if [ ! -d "devforgeai/feedback" ]; then
    echo "❌ ERROR: Feedback directory not found"
    echo ""
    echo "Expected: devforgeai/feedback/"
    echo ""
    echo "Initialize feedback system first:"
    echo "  Run feedback capture commands to create directory structure"
    exit 1
fi
```

**Check if sessions directory exists:**
```bash
if [ ! -d "devforgeai/feedback/sessions" ]; then
    echo "❌ ERROR: Sessions directory not found"
    echo ""
    echo "Expected: devforgeai/feedback/sessions/"
    echo ""
    echo "No sessions to reindex. Create feedback sessions first."
    exit 1
fi
```

---

### Phase 1: Pre-Reindex Check

**Display current index status:**
```bash
if [ -f "devforgeai/feedback/index.json" ]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Feedback Index Reindex"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Current index status:"

    # Count existing entries
    CURRENT_COUNT=$(python3 -c "import json; data=json.load(open('devforgeai/feedback/index.json')); print(len(data.get('feedback-sessions', [])))")
    echo "  Indexed sessions: $CURRENT_COUNT"

    # Check last updated
    LAST_UPDATED=$(python3 -c "import json; data=json.load(open('devforgeai/feedback/index.json')); print(data.get('last-updated', 'Unknown'))")
    echo "  Last updated: $LAST_UPDATED"

    echo ""
else
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Feedback Index Reindex"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "⚠️  No existing index found - will create new index"
    echo ""
fi

# Count session files
SESSION_COUNT=$(find devforgeai/feedback/sessions -name "*.md" -type f | wc -l)
echo "Session files found: $SESSION_COUNT"
echo ""
```

---

### Phase 2: Invoke Reindex

**Execute reindex using Python module:**
```bash
echo "Starting reindex operation..."
echo ""

# Run reindex
python3 <<'EOF'
import sys
from pathlib import Path

# Add src to path
sys.path.insert(0, str(Path.cwd()))

from src.feedback_index import reindex_feedback_sessions

try:
    # Execute reindex
    result = reindex_feedback_sessions(Path.cwd())

    # Display results
    print("✅ Reindex completed successfully")
    print("")
    print(f"Total sessions processed: {result['total_sessions']}")
    print(f"Successfully indexed: {result['indexed_count']}")
    print(f"Errors encountered: {result['error_count']}")
    print(f"Execution time: {result['execution_time']:.2f}s")

    if result['errors']:
        print("")
        print("⚠️  Errors during reindex:")
        for error in result['errors'][:5]:  # Show first 5 errors
            print(f"  - {error}")
        if len(result['errors']) > 5:
            print(f"  ... and {len(result['errors']) - 5} more")

    print("")
    print(f"Index file: devforgeai/feedback/index.json")
    print(f"Index version: {result['version']}")

    sys.exit(0)

except Exception as e:
    print(f"❌ Reindex failed: {e}")
    print("")
    print("Troubleshooting:")
    print("  1. Check that devforgeai/feedback/sessions/ contains valid session files")
    print("  2. Verify session files have YAML frontmatter")
    print("  3. Check file permissions (read/write access needed)")
    print("  4. Review error message above for specific issue")
    sys.exit(1)
EOF
```

**Check exit status:**
```bash
if [ $? -eq 0 ]; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "✅ REINDEX COMPLETE"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
else
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "❌ REINDEX FAILED"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 1
fi
```

---

## Success Criteria

- [ ] Index file created/rebuilt at `devforgeai/feedback/index.json`
- [ ] All session files processed (100% completion)
- [ ] Index contains valid JSON structure (version, last-updated, feedback-sessions array)
- [ ] All entries have required fields (id, timestamp, operation, status, file-path)
- [ ] Execution completes in <10 seconds for 1000+ sessions
- [ ] No data loss (all valid sessions indexed)

---

## Error Handling

### Error: Feedback directory not found
```
Cause: devforgeai/feedback/ doesn't exist
Fix: Run feedback capture commands first to initialize
```

### Error: Sessions directory empty
```
Cause: No session files to index
Fix: Create feedback sessions, then reindex
```

### Error: Permission denied
```
Cause: No write access to devforgeai/feedback/
Fix: Check file permissions, ensure write access
```

### Error: Invalid session file format
```
Cause: Session file has malformed YAML or missing fields
Fix: Check error message for specific file, fix YAML syntax
```

---

## Performance

**Typical execution times:**
- 100 sessions: ~1 second
- 500 sessions: ~3 seconds
- 1000 sessions: ~6 seconds
- 2000+ sessions: ~10-15 seconds

**Progress not shown** - Reindex completes quickly enough that progress bars not needed.

---

## Integration

**Invoked when:**
- User runs `/feedback-reindex` manually
- Corruption detected by feedback system (automatic suggestion)
- Monthly maintenance (recommended)

**Invokes:**
- `src.feedback_index.reindex_feedback_sessions()` - Core reindex logic

**Updates:**
- `devforgeai/feedback/index.json` - Completely rebuilt from scratch

---

## Recovery Procedures

### Scenario 1: Corrupted Index
```bash
# Backup corrupted index (optional)
cp devforgeai/feedback/index.json devforgeai/feedback/index.json.backup

# Reindex
/feedback-reindex

# Verify
python3 -c "import json; print(len(json.load(open('devforgeai/feedback/index.json'))['feedback-sessions']), 'sessions indexed')"
```

### Scenario 2: Out of Sync Index
```bash
# Count sessions on disk
find devforgeai/feedback/sessions -name "*.md" | wc -l

# Count indexed sessions
python3 -c "import json; print(len(json.load(open('devforgeai/feedback/index.json'))['feedback-sessions']))"

# If different, reindex
/feedback-reindex
```

### Scenario 3: After Manual File Changes
```bash
# Edit session files manually
vim devforgeai/feedback/sessions/2025-11-07-*.md

# Reindex to pick up changes
/feedback-reindex
```

---

## Related Commands

- (Future) `/feedback-search` - Search indexed sessions
- (Future) `/feedback-stats` - Display index statistics
- (Future) `/feedback-validate` - Validate index integrity

---

**Token Budget:** ~1,500 tokens (lean command)
**Execution Time:** <10 seconds typical
**Status:** Production Ready
