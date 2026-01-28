---
description: Export feedback data with selection criteria (STORY-020 implementation)
argument-hint: [--format] [--date-range] [--story-ids] [--severity] [--status]
model: opus
allowed-tools: Skill
---

# /feedback-export-data - Export Feedback Data (STORY-020)

**Note:** This is the STORY-020 implementation of feedback data export with selection criteria. For ZIP package exports with sanitization, see `/export-feedback`.

Export feedback entries to JSON, CSV, or Markdown format with filtering.

---

## Quick Reference

```bash
# Export all feedback (JSON)
devforgeai export-feedback

# Export with specific format
devforgeai export-feedback --format=csv
devforgeai export-feedback --format=markdown

# Export by date range
devforgeai export-feedback --date-range=2025-11-01..2025-11-07

# Export specific stories
devforgeai export-feedback --story-ids=STORY-001,STORY-002

# Export with filters
devforgeai export-feedback --severity=high --status=open
```

---

## Implementation Notes

This command was implemented as part of STORY-020 to provide **data export** functionality distinct from the existing `/export-feedback` ZIP package export (STORY-013).

**Key Differences:**

| Feature | /export-feedback (STORY-013) | devforgeai export-feedback (STORY-020) |
|---------|------------------------------|----------------------------------------|
| **Output** | ZIP package with sessions | Single file (JSON/CSV/Markdown) |
| **Sanitization** | Always applied | Raw data export |
| **Use Case** | Share with maintainers | Data analysis, reporting |
| **Format** | ZIP (sessions + manifest) | JSON, CSV, or Markdown |
| **Selection** | Date range only | Date range + story IDs + filters |

---

## Success Criteria

- [x] Export to JSON, CSV, Markdown formats
- [x] Selection criteria (date range, story IDs, severity, status)
- [x] File path generation with timestamps
- [x] Metadata included (export ID, timestamp, config snapshot)
- [x] Empty export handling (0 entries is valid)
- [x] Validation prevents unsupported formats
- [x] Clear error messages
- [x] Exit code 0 (success) or 1 (error)

---

## Documentation

For complete documentation, see the inline implementation at:
`.claude/scripts/devforgeai_cli/feedback/commands.py:handle_export_feedback()`

---

## Related

- `/export-feedback` - ZIP package export with sanitization
- `/feedback-search` - Query feedback before export
- `/feedback` - Capture feedback entries
