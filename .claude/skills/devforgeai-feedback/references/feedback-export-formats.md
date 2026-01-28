# Feedback Export Formats

**Version:** 1.0
**Date:** 2025-11-20
**Status:** Active

This guide documents the export formats supported by the DevForgeAI feedback system for cross-project sharing, analysis, and long-term archival.

---

## Overview

The feedback system supports multiple export formats:
- **ZIP Archive** - Portable package with sessions, index, and config (default)
- **JSON** - Structured data for programmatic analysis
- **CSV** - Tabular format for spreadsheet analysis
- **Markdown** - Human-readable summary reports

---

## ZIP Archive Format (Default)

### Purpose

- **Cross-project portability:** Share feedback between teams/projects
- **Complete package:** All sessions, index, and configuration
- **Safe sharing:** Optional sanitization removes sensitive data
- **Version control:** Suitable for Git LFS or external storage

### Package Structure

```
feedback-export-20251120-143045.zip
├── manifest.json                # Export metadata
├── sessions/                    # Session markdown files
│   ├── 20251120-083045-dev-STORY-001.md
│   ├── 20251120-091523-qa-STORY-002.md
│   └── 20251120-143012-release-STORY-003.md
├── index.json                   # Searchable metadata index
├── config.yaml                  # User configuration (sanitized)
└── README.md                    # Import instructions
```

### Manifest Schema

```json
{
  "export-version": "1.0",
  "export-date": "2025-11-20T14:30:45Z",
  "source-project": "MyProject",
  "source-org": "Acme Corp",
  "session-count": 15,
  "date-range": {
    "start": "2025-11-01T00:00:00Z",
    "end": "2025-11-20T23:59:59Z"
  },
  "filters": {
    "date": "2025-11",
    "operation": null,
    "status": null,
    "story-id": null,
    "keywords": []
  },
  "sanitized": true,
  "sanitization-applied": [
    "file-paths-replaced",
    "credentials-removed",
    "emails-redacted"
  ],
  "checksum": "sha256:a1b2c3d4e5f6...",
  "devforgeai-version": "1.0.1",
  "export-tool": "devforgeai-feedback",
  "export-format": "zip-v1"
}
```

### Export Command

```bash
# Export all sessions from November 2025
/export-feedback --date=2025-11

# Export with sanitization (recommended for external sharing)
/export-feedback --date=2025-11 --sanitize

# Export specific operation
/export-feedback --operation=/qa --date=last-90-days

# Export specific story
/export-feedback --story=STORY-001

# Export with custom output path
/export-feedback --date=2025-11 --output=/path/to/archives/
```

### Import Command

```bash
# Import from ZIP package
/import-feedback feedback-export-20251120-143045.zip

# Import with merge strategy (default: skip duplicates)
/import-feedback feedback-export-20251120-143045.zip --merge=skip

# Import and overwrite duplicates
/import-feedback feedback-export-20251120-143045.zip --merge=overwrite

# Import and create duplicates with new session IDs
/import-feedback feedback-export-20251120-143045.zip --merge=duplicate
```

### Sanitization Details

**What is sanitized:**

1. **File Paths:**
   - Absolute paths → `{PROJECT_ROOT}/path/to/file`
   - User home directories → `{HOME}/path/to/file`
   - System paths → `{SYSTEM}/path/to/file`

2. **Credentials:**
   - API keys → `***API_KEY***`
   - Tokens → `***TOKEN***`
   - Passwords → `***PASSWORD***`
   - SSH keys → `***SSH_KEY***`

3. **Personal Information:**
   - Email addresses → `***EMAIL***`
   - User names → `***USER***` (if --redact-names flag)
   - IP addresses → `***IP***`

4. **Project-Specific:**
   - Database connection strings → `***DB_CONNECTION***`
   - Cloud resource IDs → `***RESOURCE_ID***`
   - Internal URLs → `***INTERNAL_URL***`

**What is preserved:**
- Feedback content (insights, suggestions, sentiments)
- Metadata (timestamps, operation types, statuses)
- Structure (sections, field mappings)
- Metrics (duration, success rates, counts)

**Example:**

```markdown
# Before sanitization
Database connection failed: postgresql://user:pass@db.internal.acme.com:5432/prod
File not found: /home/alice/projects/myproject/src/api/handler.py
API key invalid: sk_live_a1b2c3d4e5f6g7h8i9j0

# After sanitization
Database connection failed: ***DB_CONNECTION***
File not found: {PROJECT_ROOT}/src/api/handler.py
API key invalid: ***API_KEY***
```

---

## JSON Format

### Purpose

- **Programmatic analysis:** Import into BI tools, databases
- **API integration:** Feed into analytics platforms
- **Machine learning:** Train models on feedback patterns
- **Custom dashboards:** Build visualization tools

### Structure

```json
{
  "export-metadata": {
    "version": "1.0",
    "export-date": "2025-11-20T14:30:45Z",
    "session-count": 15,
    "date-range": {
      "start": "2025-11-01T00:00:00Z",
      "end": "2025-11-20T23:59:59Z"
    }
  },
  "sessions": [
    {
      "session-id": "20251120-083045-uuid4",
      "operation": "dev",
      "operation-type": "command",
      "status": "passed",
      "story-id": "STORY-001",
      "timestamp": "2025-11-20T08:30:45Z",
      "duration-ms": 1234567,
      "template-used": "command-passed",
      "questions-answered": 4,
      "questions-skipped": 0,
      "sentiment": "positive",
      "keywords": ["TDD", "edge cases", "code review"],
      "responses": {
        "cmd_passed_01": "TDD workflow was clear and well-structured...",
        "cmd_passed_02": "Test generation took 15 minutes...",
        "cmd_passed_03": "No unexpected challenges",
        "cmd_passed_04": "4"
      },
      "metadata": {
        "test-count": 45,
        "coverage": 92.3,
        "phase-durations": {
          "phase-0": 135000,
          "phase-1": 930000,
          "phase-2": 525000,
          "phase-3": 312000,
          "phase-4": 263000,
          "phase-5": 89000
        }
      }
    }
  ]
}
```

### Export Command

```bash
# Export as JSON
/export-feedback --date=2025-11 --format=json

# Export pretty-printed JSON
/export-feedback --date=2025-11 --format=json --pretty

# Export minified JSON (smaller file size)
/export-feedback --date=2025-11 --format=json --minify
```

### Use Cases

**Load into PostgreSQL:**
```sql
CREATE TABLE feedback_sessions (
  session_id VARCHAR(50) PRIMARY KEY,
  operation VARCHAR(50),
  operation_type VARCHAR(20),
  status VARCHAR(20),
  story_id VARCHAR(20),
  timestamp TIMESTAMPTZ,
  duration_ms INTEGER,
  sentiment VARCHAR(20),
  responses JSONB
);

-- Import JSON
COPY feedback_sessions FROM '/path/to/export.json' FORMAT JSON;
```

**Load into pandas (Python):**
```python
import pandas as pd
import json

with open('feedback-export.json') as f:
    data = json.load(f)

df = pd.DataFrame(data['sessions'])
print(df.describe())
```

**Load into Excel:**
1. Open Excel
2. Data → Get Data → From File → From JSON
3. Select `feedback-export.json`
4. Expand "sessions" column
5. Load

---

## CSV Format

### Purpose

- **Spreadsheet analysis:** Excel, Google Sheets
- **Simple reporting:** Pivot tables, charts
- **Non-technical users:** Familiar format
- **Quick filtering:** Sort, filter, search

### Structure

```csv
session_id,timestamp,operation,operation_type,status,story_id,duration_ms,sentiment,questions_answered,questions_skipped,keywords,response_1,response_2,response_3,response_4
20251120-083045-uuid4,2025-11-20T08:30:45Z,dev,command,passed,STORY-001,1234567,positive,4,0,"TDD;edge cases;code review","TDD workflow was clear...","Test generation took 15 minutes...","No unexpected challenges","4"
20251120-091523-uuid4,2025-11-20T09:15:23Z,qa,command,failed,STORY-002,45000,negative,5,1,"coverage;threshold;violations","Coverage below 80%","Unclear error message","Need better logging","2"
```

### Export Command

```bash
# Export as CSV
/export-feedback --date=2025-11 --format=csv

# Export with custom delimiter
/export-feedback --date=2025-11 --format=csv --delimiter="|"

# Export with header row
/export-feedback --date=2025-11 --format=csv --header
```

### Use Cases

**Excel Pivot Table:**
1. Import CSV into Excel
2. Insert → PivotTable
3. Rows: operation
4. Columns: sentiment
5. Values: Count of session_id

**Google Sheets Dashboard:**
1. File → Import → Upload CSV
2. Create charts:
   - Sentiment distribution (pie chart)
   - Success rate by operation (bar chart)
   - Duration trend over time (line chart)

**Command-line analysis:**
```bash
# Count sessions by operation
cut -d, -f3 export.csv | sort | uniq -c

# Average duration by operation
awk -F, '{sum[$3]+=$7; count[$3]++} END {for(op in sum) print op, sum[op]/count[op]}' export.csv

# Sentiment distribution
cut -d, -f8 export.csv | sort | uniq -c
```

---

## Markdown Format

### Purpose

- **Human-readable reports:** Share with stakeholders
- **Documentation:** Include in project wiki
- **Git-friendly:** Version control summary reports
- **Presentation:** Copy-paste into slides

### Structure

```markdown
# Feedback Summary Report

**Export Date:** 2025-11-20 14:30:45 UTC
**Date Range:** 2025-11-01 to 2025-11-20
**Total Sessions:** 15
**Success Rate:** 93% (14/15)

---

## Overview

- **Operations Covered:** /dev (9), /qa (4), /release (2)
- **Sentiment Distribution:** 11 positive (73%), 3 neutral (20%), 1 negative (7%)
- **Average Duration:** 12m 34s
- **Engagement:** 97% questions answered (3% skip rate)

---

## Top Themes

### 1. Test Performance (8 mentions)

**Status:** Improving
**Sentiment:** Mixed

**Key Feedback:**
- "Test generation took 15 minutes, expected 5" (STORY-001)
- "Parallel test execution helped significantly" (STORY-004)
- "Still slow for integration tests" (STORY-007)

**Action:** Continue optimization, target <5m for unit tests

### 2. Error Message Clarity (5 mentions)

**Status:** Needs Improvement
**Sentiment:** Negative

**Key Feedback:**
- "Unclear what failed in validation" (STORY-002)
- "Error didn't mention which field" (STORY-005)
- "No suggestion for how to fix" (STORY-008)

**Action:** Implement structured error template (STORY-XXX)

---

## Session Details

### 2025-11-20 08:30:45 - /dev STORY-001 ✅

**Duration:** 20m 34s | **Sentiment:** Positive

**What Went Well:**
TDD workflow was clear and well-structured. Tests caught 2 edge cases early.

**Improvements:**
Test generation took longer than expected (15m vs 5m expected).

**Suggestions:**
Add database mocking examples to test-automator.

---

### 2025-11-20 09:15:23 - /qa STORY-002 ❌

**Duration:** 45s | **Sentiment:** Negative

**What Went Wrong:**
Coverage below 80% threshold, but error message didn't indicate which module.

**Improvements:**
Need better error logging, show specific files below threshold.

**Suggestions:**
Add coverage report summary to error output.

---

## Metrics

| Metric | Value |
|--------|-------|
| Total Sessions | 15 |
| Success Rate | 93% |
| Average Duration | 12m 34s |
| Positive Sentiment | 73% |
| Skip Rate | 3% |

---

## Recommendations

1. **HIGH:** Address error message clarity (5 mentions, negative sentiment)
2. **MEDIUM:** Continue test performance optimization (8 mentions, mixed sentiment)
3. **LOW:** Add more documentation examples (2 mentions)

---

**Generated by:** devforgeai-feedback v1.0
**Report Type:** Monthly Summary
**Next Review:** 2025-12-20
```

### Export Command

```bash
# Export as markdown summary
/export-feedback --date=2025-11 --format=markdown

# Export detailed (includes full sessions)
/export-feedback --date=2025-11 --format=markdown --detailed

# Export executive summary (high-level only)
/export-feedback --date=2025-11 --format=markdown --summary
```

---

## Custom Export Formats

### Purpose

- **Organization-specific needs:** Compliance, audit trails
- **Integration:** Feed into internal tools
- **Legacy systems:** Support older formats

### YAML Format

```yaml
export_metadata:
  version: "1.0"
  export_date: "2025-11-20T14:30:45Z"
  session_count: 15

sessions:
  - session_id: "20251120-083045-uuid4"
    operation: "dev"
    status: "passed"
    responses:
      cmd_passed_01: "TDD workflow was clear..."
      cmd_passed_02: "Test generation took 15 minutes..."
```

**Export:**
```bash
/export-feedback --date=2025-11 --format=yaml
```

### XML Format

```xml
<?xml version="1.0" encoding="UTF-8"?>
<feedback-export version="1.0">
  <metadata>
    <export-date>2025-11-20T14:30:45Z</export-date>
    <session-count>15</session-count>
  </metadata>
  <sessions>
    <session>
      <session-id>20251120-083045-uuid4</session-id>
      <operation>dev</operation>
      <status>passed</status>
      <responses>
        <response question-id="cmd_passed_01">TDD workflow was clear...</response>
        <response question-id="cmd_passed_02">Test generation took 15 minutes...</response>
      </responses>
    </session>
  </sessions>
</feedback-export>
```

**Export:**
```bash
/export-feedback --date=2025-11 --format=xml
```

### SQL Dump Format

```sql
-- Export as SQL INSERT statements
INSERT INTO feedback_sessions VALUES (
  '20251120-083045-uuid4',
  'dev',
  'command',
  'passed',
  'STORY-001',
  '2025-11-20 08:30:45+00',
  1234567,
  'positive',
  '{"cmd_passed_01": "TDD workflow was clear..."}'
);
```

**Export:**
```bash
/export-feedback --date=2025-11 --format=sql
```

---

## Comparison of Formats

| Format | Human Readable | Machine Parsable | Size | Use Case |
|--------|----------------|------------------|------|----------|
| ZIP | ⭐⭐⭐ | ⭐⭐ | Large | Cross-project sharing, archival |
| JSON | ⭐⭐ | ⭐⭐⭐ | Medium | API integration, BI tools |
| CSV | ⭐⭐⭐ | ⭐⭐ | Small | Spreadsheets, simple analysis |
| Markdown | ⭐⭐⭐ | ⭐ | Medium | Reports, documentation |
| YAML | ⭐⭐ | ⭐⭐⭐ | Medium | Config-driven tools |
| XML | ⭐ | ⭐⭐⭐ | Large | Legacy systems, enterprise |
| SQL | ⭐ | ⭐⭐⭐ | Small | Direct database import |

**Recommendation:**
- **Default:** ZIP (complete, portable)
- **Analysis:** JSON (programmatic)
- **Reporting:** Markdown (human-readable)
- **Spreadsheets:** CSV (simple)

---

## Advanced Export Options

### Filtering

```bash
# Export only failures
/export-feedback --status=failed --date=2025-11

# Export specific operation
/export-feedback --operation=/dev --date=last-90-days

# Export multiple stories
/export-feedback --story=STORY-001,STORY-002,STORY-003

# Export by keyword
/export-feedback --keyword="slow test" --date=2025-11
```

### Transformation

```bash
# Export with custom field selection (JSON only)
/export-feedback --format=json --fields=session-id,operation,status,sentiment

# Export aggregated (summary statistics only)
/export-feedback --format=json --aggregate

# Export normalized (flatten nested structures)
/export-feedback --format=json --normalize
```

### Compression

```bash
# Export with gzip compression
/export-feedback --date=2025-11 --compress=gzip

# Export with bzip2 (better compression, slower)
/export-feedback --date=2025-11 --compress=bzip2

# Export uncompressed (faster, larger)
/export-feedback --date=2025-11 --compress=none
```

---

## Import Strategies

### Merge Modes

**skip (default):**
- Skip sessions with duplicate session-id
- Preserve existing data
- Safe for incremental imports

**overwrite:**
- Replace existing sessions with imported versions
- Use when imported data is more authoritative
- Caution: Overwrites local edits

**duplicate:**
- Create new sessions with new session-ids
- Preserves both local and imported data
- Useful for testing or comparing datasets

**Example:**
```bash
# Skip duplicates (default)
/import-feedback export.zip

# Overwrite duplicates
/import-feedback export.zip --merge=overwrite

# Create duplicates
/import-feedback export.zip --merge=duplicate
```

### Validation

```bash
# Dry-run (preview import without applying)
/import-feedback export.zip --dry-run

# Validate package integrity
/import-feedback export.zip --validate-only

# Import with strict validation (fail on any error)
/import-feedback export.zip --strict
```

---

## Performance Considerations

### Export Performance

| Session Count | ZIP Export | JSON Export | CSV Export |
|---------------|------------|-------------|------------|
| 100 | <1s | <1s | <1s |
| 1,000 | ~3s | ~2s | ~1s |
| 10,000 | ~15s | ~10s | ~5s |
| 100,000 | ~90s | ~60s | ~30s |

**Optimization tips:**
- Use filters to export subsets (faster)
- Use CSV for large exports (smallest size)
- Use compression for archival (smaller storage)

### Import Performance

| Session Count | ZIP Import | JSON Import | CSV Import |
|---------------|------------|-------------|------------|
| 100 | <1s | <1s | <1s |
| 1,000 | ~5s | ~3s | ~2s |
| 10,000 | ~30s | ~20s | ~15s |
| 100,000 | ~180s | ~120s | ~90s |

**Optimization tips:**
- Use `--skip` merge mode (fastest, no overwrites)
- Disable validation for trusted sources (`--no-validate`)
- Import during off-hours for large datasets

---

## Security Considerations

### Data Classification

**Public:**
- Aggregated metrics (success rates, durations)
- Sentiment distributions
- Question skip rates

**Internal:**
- Session content (feedback responses)
- Operation details (story IDs, commands)
- Performance metrics (phase durations)

**Confidential:**
- File paths (may reveal project structure)
- Error messages (may contain credentials)
- User names (if personally identifiable)

### Export Best Practices

1. **Always sanitize** when sharing outside organization
2. **Review exports** before sharing (spot-check for leaks)
3. **Use encryption** for sensitive exports (zip with password or gpg)
4. **Limit retention** of exports (delete after 90 days)
5. **Track exports** (log who exported what, when)

### Import Best Practices

1. **Validate source** (trust only known sources)
2. **Scan for malware** (if from external sources)
3. **Review manifest** before importing (check session count, date range)
4. **Use dry-run** first (preview before applying)
5. **Backup before import** (in case of corruption)

---

## Related Documentation

- **Feedback Question Templates:** `feedback-question-templates.md`
- **Feedback Persistence Guide:** `feedback-persistence-guide.md`
- **Feedback Analysis Patterns:** `feedback-analysis-patterns.md`
- **Template Format Specification:** `template-format-specification.md`

---

**Export formats enable feedback data to flow across projects, tools, and teams. Choose the right format for your use case: ZIP for portability, JSON for analysis, CSV for spreadsheets, Markdown for reports. Always sanitize when sharing externally to protect sensitive information.**
