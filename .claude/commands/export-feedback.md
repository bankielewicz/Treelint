---
description: Export feedback sessions to portable ZIP package
argument-hint: "[--date-range RANGE] [--sanitize true/false] [--output PATH]"
model: opus
allowed-tools: Read, Write, Bash(python3:*), Bash(ls:*)
---

# /export-feedback - Export Feedback Sessions

Export your project's feedback sessions into a portable, shareable ZIP package with optional sanitization for privacy.

## Quick Reference

```bash
# Export last 30 days (default, with sanitization)
/export-feedback

# Export specific date range
/export-feedback --date-range last-7-days
/export-feedback --date-range last-90-days
/export-feedback --date-range all

# Export to custom location
/export-feedback --output ~/feedback-exports/
```

## Syntax

**Parameters (all optional):**
- `--date-range` - Filter by date (default: `last-30-days`)
  - Valid: `last-7-days`, `last-30-days`, `last-90-days`, `all`
- `--sanitize` - Apply sanitization (default: `true`)
  - Valid: `true`, `false` (secure by default)
- `--output` - Custom output directory (default: current directory)

## Features

✅ **Automatic Sanitization (Secure by Default)**
- Story IDs replaced with placeholders (STORY-042 → STORY-001)
- Custom field values removed (field names preserved)
- File paths and repo names masked
- Original feedback preserved in `devforgeai/feedback/`

✅ **Portable ZIP Archives**
- Cross-platform format (Windows, macOS, Linux)
- Self-documenting structure:
  - `feedback-sessions/` - All feedback files
  - `index.json` - Session metadata and filtering info
  - `manifest.json` - Export details, sanitization rules, compatibility
- Deterministic output (same input → same archive)

✅ **Shareable Format**
- ZIP archives < 10MB typical (compressed)
- Includes sanitization mappings for transparency
- Framework version compatibility info
- SHA-256 checksums for integrity verification

## Examples

**Export last 7 days with sanitization:**
```bash
/export-feedback --date-range last-7-days
```
Output: `devforgeai-feedback-export-2025-11-11T14-30-00.zip`

**Export everything to shared directory:**
```bash
/export-feedback --date-range all --output ~/Desktop/exports/
```

**Export without sanitization (framework maintainers only):**
```bash
/export-feedback --sanitize false
```

## Output

Upon success:
```
✅ Feedback Export Complete

Archive: devforgeai-feedback-export-2025-11-11T14-30-00.zip
Size: 4.2 MB
Sessions: 47
Date Range: last-30-days (2025-10-12 to 2025-11-11)
Sanitization: Applied
  - Story IDs: 12 unique IDs replaced with placeholders
  - Custom fields: 8 field values removed
  - File paths: 15 paths masked

Next Steps:
- Share the ZIP archive with DevForgeAI maintainers
- Include any context about the issues you encountered
- Or keep for your own records (unsanitized version in devforgeai/feedback/)
```

## What Gets Sanitized?

**Replaced:**
- Story IDs (STORY-042 → STORY-001)
- File paths (/home/user/project → {REMOVED})
- Repository URLs (git@github.com:user/repo → {REMOVED})
- Custom field values (user-specific data)

**Preserved:**
- Operation types (command, skill)
- Status (success, partial, failed)
- Timestamps (when feedback was created)
- Framework version
- Feedback text and observations

## Troubleshooting

**"No sessions match date range"**
- Change `--date-range` to broader range (e.g., `all`)
- Check if feedback sessions exist in `devforgeai/feedback/sessions/`

**"Archive size exceeds 100MB"**
- Use narrower date range (e.g., `last-7-days`)
- Archive is too large to share easily

**"Permission denied"**
- Check write permissions for output directory
- Use `--output` to specify writable location

**"Archive not valid ZIP"**
- Re-run export command
- Report issue to DevForgeAI maintainers

## FAQ

**Q: Is sanitization reversible?**
A: No. Sanitization removes sensitive data permanently. Original feedback stored unsanitized in `devforgeai/feedback/` if you need to recover it.

**Q: Can I export without sanitization?**
A: Yes, with `--sanitize false`, but this sends sensitive data. Only use for authorized recipients.

**Q: How do I share the export?**
A: Email the `.zip` file to DevForgeAI maintainers, or attach to GitHub issues. The sanitization makes it safe to share.

**Q: What's the format?**
A: Standard ZIP archive with JSON metadata files. You can open with any zip tool or import into another DevForgeAI project.

## Integration

**Exported With:** src/feedback_export_import.py
**Related:** /import-feedback (import exported packages)
**Dependencies:** STORY-013 (Feedback File Persistence), STORY-016 (Feedback Index)

---

**Token Budget:** ~2,000 tokens
**Status:** Production Ready
**Last Updated:** 2025-11-11
