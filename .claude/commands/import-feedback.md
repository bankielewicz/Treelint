---
description: Import feedback sessions from exported ZIP package
argument-hint: "<archive-path>"
model: opus
allowed-tools: Read, Write, Bash(python3:*), Bash(ls:*), Bash(unzip:*)
---

# /import-feedback - Import Feedback Sessions

Import feedback sessions from a shared ZIP package exported by another DevForgeAI project or maintainer.

## Quick Reference

```bash
# Import from current directory
/import-feedback feedback-export.zip

# Import from absolute path
/import-feedback ~/Downloads/feedback-export-2025-11-11.zip

# Import from relative path
/import-feedback ../shared-feedback/export.zip
```

## Syntax

**Parameters:**
- `<archive-path>` - Path to ZIP file (required)
  - Relative paths: `./export.zip`, `../shared/export.zip`
  - Absolute paths: `/home/user/exports/export.zip`
  - Windows paths: `C:\Users\Name\feedback-export.zip`

## Features

✅ **Automatic Validation**
- ZIP format verification
- Required files check (index.json, manifest.json, feedback-sessions/)
- Framework version compatibility checking
- SHA-256 integrity verification

✅ **Conflict Resolution**
- Duplicate session IDs automatically suffixed (-imported-1, -imported-2)
- Original session IDs preserved in metadata
- Collisions documented in conflict-resolution.log
- All conflicts logged for audit trail

✅ **Atomic Operations**
- No partial imports on failure
- Index updated atomically (all-or-nothing)
- Original sessions never corrupted
- Easy rollback if needed

✅ **Framework Integration**
- Seamlessly integrates with existing feedback
- Updates main feedback index
- Marks imported sessions with metadata
- Preserves chronological ordering

## Examples

**Import from current directory:**
```bash
/import-feedback feedback-export-2025-11-11.zip
```
Output: Sessions extracted to `devforgeai/feedback/imported/2025-11-11T14-30-00/`

**Import from external source:**
```bash
/import-feedback ~/Downloads/external-feedback.zip
```

**Import after downloading from GitHub:**
```bash
/import-feedback ../issues/feedback-export-issue-123.zip
```

## Output

Upon success:
```
✅ Feedback Import Complete

Archive: feedback-export-2025-11-11.zip
Extracted to: devforgeai/feedback/imported/2025-11-11T14-30-00/
Sessions: 47 imported
Duplicate IDs: 3 resolved (-imported-1, -imported-2, -imported-3)

Compatibility:
  - Framework version: 1.0.0 (min required: 1.0.0) ✅
  - Tested on versions: [1.0.0, 1.0.1] ✅

Sanitization Status:
  - Applied: Yes
  - Story IDs: Placeholders (STORY-001, STORY-002, etc.)
  - Custom fields: Values removed (field names preserved)
  - Original unsanitized: Not available in this import

Next Steps:
- Browse imported sessions in devforgeai/feedback/imported/
- Search main index to find imported sessions (marked with is_imported: true)
- Use /feedback-reindex to update searchable index
```

## Validation

The import command validates:
- ✅ ZIP file exists and is readable
- ✅ ZIP format is valid (not corrupted)
- ✅ Required files present (index.json, manifest.json, feedback-sessions/)
- ✅ Files are valid JSON (proper structure)
- ✅ Framework version compatible
- ✅ SHA-256 checksums match (data integrity)
- ✅ No malicious paths (prevents directory traversal attacks)

## Conflict Resolution

**Duplicate Session IDs:**
If imported sessions have IDs that already exist:
1. Original session ID preserved in metadata (`original_id`)
2. New ID generated with suffix: `-imported-1`, `-imported-2`, etc.
3. Collision logged: `conflict-resolution.log`
4. Metadata marks session as imported: `is_imported: true`

**Example:**
```
Original ID: 550e8400-e29b-41d4-a716-446655440000
Already exists in: devforgeai/feedback/feedback-index.json

→ New ID assigned: 550e8400-e29b-41d4-a716-446655440000-imported-1
→ Metadata: original_id: 550e8400-e29b-41d4-a716-446655440000
→ Logged in: devforgeai/feedback/imported/2025-11-11T14-30-00/conflict-resolution.log
```

## What Gets Imported?

**Session Files:**
- All feedback session files from `feedback-sessions/` directory
- Timestamps and metadata preserved
- File modification dates maintained

**Metadata:**
- Session IDs (with collision resolution if needed)
- Operation types (command, skill, etc.)
- Execution status (success, partial, failed)
- Original timestamps

**Sanitization Info:**
- Manifest details on what was sanitized
- Story ID mapping (if available)
- Framework version info
- Compatibility warnings

## Troubleshooting

**"File not found"**
- Check file path is correct (relative or absolute)
- Verify file exists: `ls -la <path>`
- Use absolute paths if relative path fails

**"Not a valid ZIP archive"**
- Export file may be corrupted
- Try importing from original source again
- Request re-export from the user

**"Missing required files"**
- Archive is incomplete or modified
- Export files needed: `index.json`, `manifest.json`, `feedback-sessions/`
- Request complete export from user

**"Framework version incompatible"**
- Export was created with older framework
- Current framework may not support old format
- Check manifest.json for min_framework_version
- May need to upgrade framework

**"Duplicate IDs - import proceeded with -imported-N suffix"**
- Sessions already imported from this source
- Check conflict-resolution.log in import directory
- Original IDs preserved in metadata
- Safe to proceed (no data lost)

**"Permission denied"**
- Check write permissions for `devforgeai/feedback/imported/`
- Ensure directory ownership is correct
- Use alternative import location if available

## FAQ

**Q: Can I import multiple exports?**
A: Yes. Each import gets its own timestamped directory and unique IDs. No conflicts.

**Q: What if archive contains duplicate sessions?**
A: Import detects duplicates, auto-suffixes IDs (-imported-1, etc.), logs collisions.

**Q: Are imported sessions marked somehow?**
A: Yes. Each imported session has:
  - `is_imported: true` - marks it as imported
  - `imported_from: {source_hash}` - tracks origin
  - `imported_at: {timestamp}` - when it was imported

**Q: Can I delete imported sessions?**
A: Yes, manually in `devforgeai/feedback/imported/{timestamp}/` directory.

**Q: What if I import sanitized feedback?**
A: Sanitization is permanent. Story IDs are placeholders (STORY-001, STORY-002). Original IDs lost.

**Q: Does import affect my original feedback?**
A: No. Original feedback in `devforgeai/feedback/sessions/` is never modified.

**Q: Can I rollback an import?**
A: Yes. Delete the `devforgeai/feedback/imported/{timestamp}/` directory, then run `/feedback-reindex` to update.

## Integration

**Imported With:** src/feedback_export_import.py
**Related:** /export-feedback (create packages to share)
**Dependencies:** STORY-013 (Feedback File Persistence), STORY-016 (Feedback Index)
**Updates:** `devforgeai/feedback-index.json` (main index)

---

**Token Budget:** ~2,000 tokens
**Status:** Production Ready
**Last Updated:** 2025-11-11
