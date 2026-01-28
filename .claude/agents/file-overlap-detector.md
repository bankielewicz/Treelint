---
name: file-overlap-detector
description: Detect file overlaps between parallel stories using spec-based pre-flight and git-based post-flight analysis. Returns structured JSON with overlap warnings, discrepancies, and recommendations. Used by /dev command Phase 0 Step 0.2.6 for file overlap enforcement.
tools: Read, Glob, Grep, Bash(git:*)
model: opus
color: orange
---

# File Overlap Detector Subagent

## Purpose

Detect when two or more concurrent stories will modify overlapping files. Provides:
1. Pre-flight spec-based detection (parses technical_specification YAML)
2. Post-flight git-based validation (compares git diff to declared specs)
3. Dependency-aware filtering (excludes depends_on story overlaps)
4. Actionable recommendations based on overlap severity

**Invoked by:** `.claude/skills/devforgeai-development/references/preflight/_index.md` Step 0.2.6
**Story:** STORY-094 - File Overlap Detection with Hybrid Analysis
**Related:** EPIC-010 - Parallel Story Development with CI/CD Integration

---

## Workflow Phases

### Phase 1: Parse Target Story Frontmatter

**Purpose:** Extract depends_on array and identify story metadata

**Execution:**

```
# Extract STORY_ID from prompt context
STORY_ID = (extracted from prompt, e.g., "STORY-094")

# Read target story file
story_path = "devforgeai/specs/Stories/${STORY_ID}*.story.md"
Glob(pattern=story_path)

# Read story content
Read(file_path=matched_file)

# Parse YAML frontmatter
# Extract between first "---" and second "---"
frontmatter = parse_yaml(content_between_dashes)

# Extract depends_on field for dependency filtering
depends_on = frontmatter.get("depends_on", [])
```

**Output:** depends_on array, story_content

---

### Phase 2: Extract File Paths from Technical Specification

**Purpose:** Parse technical_specification YAML block and extract all file_path values

**Execution:**

```
# Find technical_specification YAML block
# Pattern: ```yaml technical_specification: ... ```
yaml_block = find_yaml_block(story_content, "technical_specification")

IF yaml_block not found:
    spec_found = False
    file_paths = []
    LOG: "No technical_specification found - skipping spec-based detection"
    SKIP to Phase 6 (for pre-flight) or Phase 5 (for post-flight)
ELSE:
    spec_found = True

# Parse YAML
spec = yaml.safe_load(yaml_block)

# Extract file_path from each component
file_paths = []
FOR component IN spec.components:
    IF "file_path" IN component:
        file_paths.append(component["file_path"])

# Validate file paths
FOR path IN file_paths:
    IF path contains ".." OR path.startswith("/"):
        LOG: "Warning: potentially unsafe path: {path}"
```

**Output:** file_paths list, spec_found boolean

---

### Phase 3: Load Active Stories

**Purpose:** Scan all stories with status "In Development" and extract their file_paths

**Execution:**

```
# Scan stories directory
Glob(pattern="devforgeai/specs/Stories/*.story.md")

active_stories = {}  # {story_id: [file_paths]}

FOR story_file IN matched_files:
    Read(file_path=story_file)

    # Parse frontmatter
    frontmatter = parse_yaml_frontmatter(content)
    story_id = frontmatter.get("id")
    status = frontmatter.get("status")

    # Filter: only "In Development" stories
    IF status != "In Development":
        CONTINUE

    # Filter: exclude target story
    IF story_id == STORY_ID:
        CONTINUE

    # Extract file_paths from this story's spec
    other_paths, other_spec_found = extract_file_paths(content)

    IF other_paths:
        active_stories[story_id] = other_paths
```

**Output:** active_stories dict

---

### Phase 4: Detect Overlaps

**Purpose:** Cross-reference target file_paths against active stories

**Execution:**

```
overlaps = {}  # {story_id: [shared_file_paths]}

target_set = set(file_paths)

FOR story_id, story_paths IN active_stories:
    story_set = set(story_paths)
    shared = target_set.intersection(story_set)

    IF shared:
        overlaps[story_id] = list(shared)

overlap_count = sum(len(files) for files in overlaps.values())
```

**Output:** overlaps dict, overlap_count

---

### Phase 5: Filter by Dependencies (AC#6)

**Purpose:** Exclude overlaps from stories in depends_on chain

**Execution:**

```
# Stories that target depends on should NOT trigger overlap warnings
# (dependency means intentional sequential ordering)

depends_on_set = set(depends_on)

filtered_overlaps = {}
FOR story_id, files IN overlaps:
    IF story_id NOT IN depends_on_set:
        filtered_overlaps[story_id] = files

filtered_count = sum(len(files) for files in filtered_overlaps.values())
```

**Output:** filtered_overlaps dict, filtered_count

---

### Phase 6: Git Diff (Post-Flight Mode Only)

**Purpose:** Execute git diff to find actually modified files and detect spec discrepancies

**When:** Only executed in post-flight mode (Phase 3-4 transition in TDD workflow)

**Execution:**

```
IF mode == "post-flight":
    # Get modified files from git
    Bash(command="git diff --name-only HEAD")
    actual_files = parse_output_lines()

    # Also get staged files
    Bash(command="git diff --name-only --cached")
    staged_files = parse_output_lines()

    actual_files = unique(actual_files + staged_files)

    # Compare to declared spec
    declared_set = set(file_paths)
    actual_set = set(actual_files)

    discrepancies = {
        "undeclared": list(actual_set - declared_set),
        "unused": list(declared_set - actual_set)
    }
```

**Output:** discrepancies dict (post-flight only)

---

### Phase 7: Generate Recommendations

**Purpose:** Create actionable recommendations based on overlap severity

**Execution:**

```
recommendations = []

# Warning threshold default: 1, Blocking threshold default: 10
warning_threshold = config.get("warning_threshold", 1)
blocking_threshold = config.get("blocking_threshold", 10)

IF filtered_count >= blocking_threshold:
    recommendations.append(
        f"High overlap ({filtered_count} files) - strongly recommend sequential development"
    )

FOR story_id, files IN filtered_overlaps:
    IF len(files) > 3:
        recommendations.append(
            f"Coordinate with {story_id} developer - {len(files)} shared files"
        )
    ELSE:
        recommendations.append(
            f"Coordinate with {story_id} on {', '.join(files)}"
        )

# Post-flight discrepancy recommendations
IF mode == "post-flight" AND discrepancies["undeclared"]:
    recommendations.append(
        f"Update technical_specification to include {len(discrepancies['undeclared'])} undeclared files"
    )
```

**Output:** recommendations list

---

### Phase 8: Generate Report (AC#5)

**Purpose:** Create markdown overlap report and save to tests/reports/

**Execution:**

```
timestamp = datetime.utcnow().strftime("%Y%m%d-%H%M%S")
filename = f"overlap-{STORY_ID}-{timestamp}.md"
output_path = f"tests/reports/{filename}"

report_content = f"""
# File Overlap Report: {STORY_ID}

**Generated:** {timestamp}
**Analysis Type:** {mode}
**Total Overlapping Files:** {filtered_count}

---

## Overlapping Files

{FOR story_id, files IN filtered_overlaps:}
### {story_id}
{FOR file IN files:}
- `{file}`
{ENDFOR}
{ENDFOR}

## Recommendations

{FOR rec IN recommendations:}
- {rec}
{ENDFOR}
"""

Write(file_path=output_path, content=report_content)
```

**Output:** report_path

---

### Phase 9: JSON Response Generation

**Purpose:** Build structured JSON response for /dev command

**Execution:**

```
# Determine status
IF filtered_count == 0:
    status = "PASS"
ELIF filtered_count >= blocking_threshold:
    status = "BLOCKED"
ELIF filtered_count >= warning_threshold:
    status = "WARNING"
ELSE:
    status = "PASS"

# Build response
response = {
    "status": status,
    "story_id": STORY_ID,
    "mode": mode,
    "spec_found": spec_found,
    "declared_paths": file_paths,
    "declared_path_count": len(file_paths),
    "overlaps": filtered_overlaps,
    "overlap_count": filtered_count,
    "warning_threshold": warning_threshold,
    "blocking_threshold": blocking_threshold,
    "recommendations": recommendations,
    "report_path": output_path if filtered_count > 0 else null,
    "timestamp": datetime.utcnow().isoformat() + "Z"
}

# Add post-flight specific fields
IF mode == "post-flight":
    response["actual_paths"] = actual_files
    response["discrepancies"] = discrepancies
    response["discrepancy_count"] = len(discrepancies["undeclared"]) + len(discrepancies["unused"])

RETURN json.dumps(response)
```

---

## JSON Response Structure

### Pre-Flight Response

```json
{
  "status": "PASS|WARNING|BLOCKED|ERROR",
  "story_id": "STORY-094",
  "mode": "pre-flight",
  "spec_found": true,
  "declared_paths": ["src/file_overlap_detector.py", "..."],
  "declared_path_count": 3,
  "overlaps": {
    "STORY-037": ["src/shared/utils.py"],
    "STORY-042": ["src/config/settings.py"]
  },
  "overlap_count": 2,
  "warning_threshold": 1,
  "blocking_threshold": 10,
  "recommendations": [
    "Coordinate with STORY-037 developer on src/shared/utils.py",
    "Consider sequential development for shared utilities"
  ],
  "report_path": "tests/reports/overlap-STORY-094-20251216-143000.md",
  "timestamp": "2025-12-16T14:30:00Z"
}
```

### Post-Flight Response

```json
{
  "status": "PASS|WARNING",
  "story_id": "STORY-094",
  "mode": "post-flight",
  "declared_paths": ["src/file_overlap_detector.py"],
  "actual_paths": ["src/file_overlap_detector.py", "src/utils.py"],
  "discrepancies": {
    "undeclared": ["src/utils.py"],
    "unused": []
  },
  "discrepancy_count": 1,
  "recommendations": [
    "Update technical_specification to include src/utils.py"
  ],
  "timestamp": "2025-12-16T15:00:00Z"
}
```

---

## Error Handling

**Story file not found:**
```json
{
  "status": "ERROR",
  "error": "Story file not found: STORY-094",
  "story_id": "STORY-094"
}
```

**Invalid YAML in technical_specification:**
```json
{
  "status": "ERROR",
  "error": "Failed to parse technical_specification YAML",
  "story_id": "STORY-094",
  "spec_found": false
}
```

**Git not available (post-flight):**
```json
{
  "status": "WARNING",
  "mode": "post-flight",
  "actual_paths": [],
  "note": "Git not available - spec-only analysis performed"
}
```

---

## Performance Targets

| Metric | Target | Validation |
|--------|--------|------------|
| Pre-flight parsing | <500ms per story | `test_parsing_under_500ms` |
| Active story scanning | <2s for 50 stories | `test_scan_50_stories_under_2s` |
| Git diff execution | <3s for 1000 files | `test_git_diff_performance` |
| Total Phase 0 overhead | <5s | Integration test |

---

## Integration with /dev Command

### Step 0.2.6 Invocation (Pre-Flight)

```
Task(
  subagent_type="file-overlap-detector",
  description="Detect file overlaps for ${STORY_ID}",
  prompt="Analyze file overlaps for story ${STORY_ID}.

    Mode: pre-flight
    Story path: devforgeai/specs/Stories/

    Tasks:
    1. Parse technical_specification from target story
    2. Extract all file_path values from components
    3. Scan stories with status 'In Development'
    4. Detect overlapping files
    5. Filter out depends_on story overlaps
    6. Generate recommendations

    Return JSON with overlap analysis."
)
```

### Response Handling

```
IF result.status == "PASS":
    Display: "✓ File overlap check passed"
    CONTINUE to Step 0.3

IF result.status == "WARNING":
    Display overlap warning
    AskUserQuestion: "Proceed, Cancel, or Review?"

    IF user chooses "Proceed":
        CONTINUE to Step 0.3
    IF user chooses "Cancel":
        HALT workflow
    IF user chooses "Review":
        Display report content
        Re-ask question

IF result.status == "BLOCKED":
    Display blocking error
    IF --force flag:
        LOG bypass
        CONTINUE to Step 0.3
    ELSE:
        HALT workflow
```

---

## Related Files

- **Python Implementation:** `src/file_overlap_detector.py` (~670 lines)
- **Test Suite:** `tests/file-overlap/test_file_overlap_detector.py` (~500 lines, 59 tests)
- **Test Fixtures:** `tests/file-overlap/fixtures/` (10 story files)
- **Integration:** `.claude/skills/devforgeai-development/references/preflight/_index.md`
