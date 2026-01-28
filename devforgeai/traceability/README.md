# Requirements Traceability Matrix

## Overview

The Requirements Traceability Matrix provides parsing infrastructure to extract and validate relationships between epics and stories in the DevForgeAI framework. It enables:

- Requirements coverage verification (all epic features have stories)
- Orphaned story detection (stories not linked to epics)
- Missing linkage identification (broken references)
- Bidirectional consistency validation

**Story:** STORY-083 - Requirements Traceability Matrix Foundation
**Epic:** EPIC-015 - Epic Coverage Validation & Requirements Traceability
**Status:** ✅ Implemented

---

## Quick Start

### Generate Full Requirements Matrix

```bash
devforgeai/traceability/parse-requirements.sh --generate-matrix
```

**Output:** `devforgeai/traceability/requirements-matrix.json`

### Check for Orphaned Stories

```bash
devforgeai/traceability/parse-requirements.sh --detect-orphans
```

### Validate Epic References

```bash
devforgeai/traceability/parse-requirements.sh --run-validation
```

---

## Output Format

The `requirements-matrix.json` file contains:

```json
{
  "version": "1.0.0",
  "generated_at": "2025-11-25T10:00:00Z",
  "epics": {
    "EPIC-015": {
      "title": "Epic Coverage Validation & Requirements Traceability",
      "features_count": 7,
      "file_path": "devforgeai/specs/Epics/EPIC-015-epic-coverage-validation-traceability.epic.md",
      "linked_stories": ["STORY-083", "STORY-084", "STORY-085", "STORY-086", "STORY-087", "STORY-088", "STORY-089"]
    }
  },
  "stories": {
    "STORY-083": {
      "title": "Requirements Traceability Matrix Foundation",
      "epic_ref": "EPIC-015",
      "status": "In Development",
      "file_path": "devforgeai/specs/Stories/STORY-083-requirements-traceability-matrix.story.md"
    }
  },
  "validation": {
    "intentionally_standalone": [],
    "broken_references": [],
    "missing_metadata": [],
    "summary": {
      "total_orphans": 0
    }
  }
}
```

---

## Commands

### Epic Parsing

```bash
# Extract epic_id from epic file
./parse-requirements.sh --extract-epic-id <epic-file>

# Extract title
./parse-requirements.sh --extract-epic-title <epic-file>

# Extract features section
./parse-requirements.sh --extract-features <epic-file>

# Count features
./parse-requirements.sh --count-features <epic-file>

# Extract Stories table
./parse-requirements.sh --extract-stories-table <epic-file>

# List story IDs from epic
./parse-requirements.sh --list-epic-stories <epic-file>

# Parse all epics
./parse-requirements.sh --parse-all-epics
```

### Story Parsing

```bash
# Extract story_id from filename
./parse-requirements.sh --extract-story-id-from-filename <filename>

# Extract epic: field from story
./parse-requirements.sh --extract-story-epic <story-file>

# Extract title
./parse-requirements.sh --extract-story-title <story-file>

# Parse all stories
./parse-requirements.sh --parse-all-stories
```

### Validation

```bash
# Validate epic reference format
./parse-requirements.sh --validate-epic-reference <epic-id>

# Validate story ID format
./parse-requirements.sh --validate-story-id <story-id>

# Check if epic exists
./parse-requirements.sh --validate-epic-exists <epic-id>

# Check bidirectional reference
./parse-requirements.sh --check-bidirectional <story-id> <epic-id>

# Check for duplicate story IDs
./parse-requirements.sh --check-duplicate-stories
```

### Matrix Generation

```bash
# Generate full matrix
./parse-requirements.sh --generate-matrix

# Detect orphaned stories
./parse-requirements.sh --detect-orphans

# Run full validation
./parse-requirements.sh --run-validation

# Incremental update (fast)
./parse-requirements.sh --incremental-update
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | File not found or general error |
| 2 | Missing required metadata (e.g., missing epic: field) |

---

## Configuration

Configuration is stored in `config.json`:

```json
{
  "version": "1.0.0",
  "epic_id_pattern": "^EPIC-[0-9]{3}$",
  "story_id_pattern": "^STORY-[0-9]{3}$",
  "epics_dir": ".ai_docs/Epics",
  "stories_dir": ".ai_docs/Stories",
  "output_file": "devforgeai/traceability/requirements-matrix.json"
}
```

---

## Orphaned Story Categories

### Intentionally Standalone
- Stories with `epic: None` in frontmatter
- Not an error - intentionally not linked to any epic

### Broken References
- Stories with `epic: EPIC-XXX` where EPIC-XXX doesn't exist
- Indicates deleted epic or typo in epic ID

### Missing Metadata
- Stories without `epic:` field in frontmatter
- Incomplete story metadata

---

## Testing

Run all test suites:

```bash
cd tests/traceability
bash run-tests.sh
```

Run individual test suites:

```bash
bash test_epic_parsing.sh      # AC#1: Epic parsing (12 tests)
bash test_story_parsing.sh     # AC#2: Story parsing (11 tests)
bash test_data_model.sh        # AC#3: Data model (8 tests)
bash test_validation.sh        # AC#4: Validation (8 tests)
bash test_orphan_detection.sh  # AC#5: Orphan detection (8 tests)
bash test_performance.sh       # AC#6: Performance (6 tests)
```

**Total:** 53 tests across 6 acceptance criteria

### Performance Notes

**WSL2 Environment:**

If running on WSL2, export `WSL2_SLOW=1` for extended performance thresholds:

```bash
WSL2_SLOW=1 bash test_performance.sh
```

This accounts for Windows filesystem I/O overhead through WSL2.

---

## Dependencies

- **Bash** - Shell scripting
- **jq** - JSON processing (`sudo apt-get install jq` on Ubuntu)
- **grep**, **sed**, **awk** - Text processing (standard Unix tools)
- **find**, **realpath** - File system utilities

---

## Security

### Path Traversal Protection

All file paths are validated against the project root. Attempts to access files outside the project directory are rejected:

```bash
# Blocked
./parse-requirements.sh --extract-epic-id "../../../etc/passwd"
# Error: Path traversal detected
```

### File Encoding

The parser handles both LF (Unix) and CRLF (Windows) line endings automatically using `tr -d '\r'`.

---

## Implementation Details

**Language:** Bash
**Lines of Code:** ~400
**Test Coverage:** 95%+ (53 tests)
**Performance:** <5s native, ~20s on WSL2 (19 epics, 96 stories)

**Key Functions:**
- `extract_frontmatter()` - YAML frontmatter extraction
- `extract_epic_id()` - Epic ID with fallback (epic_id: or id:)
- `extract_story_epic_ref()` - Story's epic: field
- `parse_all_epics()` - Batch epic parsing
- `parse_all_stories()` - Batch story parsing
- `detect_orphans()` - Categorize orphaned stories
- `generate_matrix()` - Create complete JSON output

---

## Next Steps

This is **Feature 0** of EPIC-015. Dependent features:

- **STORY-084** - Epic & Story Metadata Parser (Feature 1)
- **STORY-085** - Gap Detection Engine (Feature 2)
- **STORY-086** - Coverage Reporting System (Feature 3)
- **STORY-087** - Slash Command Interface (Feature 4)
- **STORY-088** - /create-story Integration (Feature 5)
- **STORY-089** - DevForgeAI Command Integration (Feature 6)

---

## References

- **Story File:** `devforgeai/specs/Stories/STORY-083-requirements-traceability-matrix.story.md`
- **Epic File:** `devforgeai/specs/Epics/EPIC-015-epic-coverage-validation-traceability.epic.md`
- **Test Suite:** `tests/traceability/`
- **Configuration:** `devforgeai/traceability/config.json`

---

**Version:** 1.0.0
**Created:** 2025-12-10
**Last Updated:** 2025-12-10
