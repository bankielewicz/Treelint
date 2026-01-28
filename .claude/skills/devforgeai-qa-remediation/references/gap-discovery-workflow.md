# Gap Discovery Workflow

**Phase:** 02 (Discovery & Parsing)
**Purpose:** Find and parse all QA gap files from configured sources.

---

## Source Path Resolution

### Step 1: Determine Source Paths

Based on `$SOURCE` argument:

| Source Value | Glob Pattern(s) |
|--------------|-----------------|
| `local` | `devforgeai/qa/reports/*-gaps.json` |
| `imports` | `devforgeai/qa/imports/**/*-gaps.json` |
| `all` | Both patterns combined |

### Step 2: Execute Glob

```
# For local source
Glob(pattern="devforgeai/qa/reports/*-gaps.json")

# For imports (recursive to support project subdirectories)
Glob(pattern="devforgeai/qa/imports/**/*-gaps.json")
```

Store results in `$GAP_FILES[]` array.

---

## JSON Parsing

### Step 3: Parse Each File

For each file in `$GAP_FILES`:

```
content = Read(file_path="{gap_file}")
gap_data = JSON.parse(content)
```

### Step 4: Validate Required Fields

**Root Level (Required):**
```json
{
  "story_id": "string",      // Required: Source story identifier
  "qa_result": "string"      // Required: "PASSED" or "FAILED"
}
```

**Root Level (Optional):**
```json
{
  "qa_date": "string",       // Optional: ISO date
  "qa_mode": "string",       // Optional: "light" or "deep"
  "test_summary": {},        // Optional: Test statistics
  "remediation_sequence": [] // Optional: Ordered fix sequence
}
```

---

## Gap Type Extraction

### coverage_gaps Array

**Schema:**
```json
{
  "file": "src/module/file.py",           // Required
  "layer": "Business Logic",               // Required: Business Logic|Application|Infrastructure
  "current_coverage": 85.0,                // Required: 0-100
  "target_coverage": 95.0,                 // Required: Threshold
  "gap_percentage": 10.0,                  // Optional: Calculated
  "critical_gaps": [                       // Optional: Specific gaps
    {"lines": "45-50", "description": "..."}
  ],
  "suggested_tests": ["test_name"]         // Optional: Recommended tests
}
```

**Severity Derivation:**
```
Business Logic → CRITICAL (95% threshold)
Application → HIGH (85% threshold)
Infrastructure → MEDIUM (80% threshold)
```

### anti_pattern_violations Array

**Schema:**
```json
{
  "file": "src/module/file.py",    // Required
  "line": 120,                      // Optional: Line number
  "type": "god_object",             // Required: Violation type
  "severity": "HIGH",               // Required: CRITICAL|HIGH|MEDIUM|LOW
  "description": "...",             // Required: Human-readable
  "remediation": "..."              // Optional: Fix guidance
}
```

**Known Types:**
- `god_object` - Class exceeds size limits
- `sql_injection` - Unsafe query construction
- `hardcoded_secret` - Credentials in code
- `circular_dependency` - Import cycles
- `direct_instantiation` - Missing DI

### code_quality_violations Array

**Schema:**
```json
{
  "file": "src/module/file.py",    // Required
  "metric": "cyclomatic_complexity", // Required: Metric name
  "current_value": 25,              // Required: Measured value
  "threshold": 10,                  // Required: Allowed maximum
  "severity": "HIGH",               // Required: Based on deviation
  "remediation": "..."              // Optional: Fix guidance
}
```

**Known Metrics:**
- `cyclomatic_complexity` - Control flow complexity
- `maintainability_index` - Code maintainability score
- `duplication` - Code repetition percentage
- `nesting_depth` - Maximum nesting level

### deferral_issues Array

**Schema:**
```json
{
  "item": "Deferred DoD item text",     // Required
  "violation_type": "autonomous",        // Required: autonomous|circular|invalid_reference
  "severity": "CRITICAL",                // Required: Usually CRITICAL or HIGH
  "remediation": "..."                   // Optional: Fix guidance
}
```

---

## Unified Gap List Construction

### Step 5: Normalize to Common Format

For each gap entry extracted, create normalized entry:

```json
{
  "id": "GAP-{sequence_number}",
  "source_file": "{gap_file_path}",
  "source_story": "{story_id from file}",
  "source_array": "coverage_gaps|anti_pattern_violations|...",
  "source_index": 0,
  "type": "coverage_gap|anti_pattern|code_quality|deferral",
  "file": "{affected_file_path}",
  "severity": "CRITICAL|HIGH|MEDIUM|LOW",
  "description": "{human_readable_description}",
  "details": { /* original gap entry */ }
}
```

### Step 6: Generate Description

**For coverage_gap:**
```
"{layer} coverage gap in {file}: {current}% vs {target}% target ({gap}% gap)"
```

**For anti_pattern:**
```
"{type} violation in {file}: {description}"
```

**For code_quality:**
```
"{metric} issue in {file}: {current} vs {threshold} threshold"
```

**For deferral:**
```
"Deferred DoD: {item} ({violation_type})"
```

---

## Error Handling

### Invalid JSON

```
If JSON.parse fails:
  - Log warning: "Skipping {file}: Invalid JSON"
  - Continue to next file
  - Add to $PARSE_ERRORS[]
```

### Missing Required Fields

```
If required field missing:
  - Log warning: "Skipping gap in {file}: Missing {field}"
  - Skip individual gap entry
  - Continue processing other gaps in file
```

### Empty Gap File

```
If all gap arrays empty:
  - Log info: "{file} has no gaps (QA passed)"
  - Skip file (not an error)
```

---

## Output Variables

| Variable | Description |
|----------|-------------|
| `$GAP_FILES` | Array of processed file paths |
| `$ALL_GAPS` | Array of normalized gap entries |
| `$FILES_PROCESSED` | Count of files successfully parsed |
| `$TOTAL_GAPS` | Total count of gaps extracted |
| `$PARSE_ERRORS` | Array of files that failed to parse |
