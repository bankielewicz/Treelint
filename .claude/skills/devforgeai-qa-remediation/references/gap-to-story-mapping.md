# Gap-to-Story Mapping

**Phase:** 05 (Batch Story Creation)
**Purpose:** Convert selected gaps to story context markers for batch creation.

---

## Context Marker Format

Each gap generates a context marker block for `devforgeai-story-creation` batch mode:

```markdown
**Story ID:** STORY-{next_available_id}
**Epic ID:** {$EPIC_ID or null}
**Feature Name:** {generated_title}
**Feature Description:** {generated_description}
**Priority:** {mapped_priority}
**Points:** {estimated_points}
**Type:** {story_type}
**Sprint:** {$SPRINT or "Backlog"}
**Batch Mode:** true
**Source Report:** {source_gap_file}
**Source Gap Type:** {gap_type}
**Source Gap Index:** {index_in_original_array}
```

---

## Feature Name Generation

### Templates by Gap Type

| Gap Type | Template |
|----------|----------|
| coverage_gap | "Improve test coverage for {layer} layer in {filename}" |
| anti_pattern | "Fix {violation_type} violation in {filename}" |
| code_quality | "Reduce {metric} in {filename}" |
| deferral | "Resolve deferred DoD item: {item_summary}" |

### Examples

**coverage_gap:**
```
Input:
  file: "src/treelint/index/indexer.py"
  layer: "Business Logic"

Output:
  "Improve test coverage for Business Logic layer in indexer.py"
```

**anti_pattern:**
```
Input:
  file: "src/services/user_service.py"
  type: "god_object"

Output:
  "Fix god_object violation in user_service.py"
```

**code_quality:**
```
Input:
  file: "src/processors/order_processor.py"
  metric: "cyclomatic_complexity"

Output:
  "Reduce cyclomatic_complexity in order_processor.py"
```

**deferral:**
```
Input:
  item: "Integration tests for external API"

Output:
  "Resolve deferred DoD item: Integration tests for external API"
```

---

## Feature Description Generation

### Template Structure

```
{Context paragraph explaining the gap}

{Specific details section}

{Metrics section with current vs target}

{Remediation guidance if available}

**Source:** {gap_file} (QA {date})
```

### By Gap Type

#### coverage_gap Description

```markdown
Add tests to achieve {target}% {layer} coverage target in {file}.

**Current State:**
- Coverage: {current}%
- Target: {target}%
- Gap: {gap}%

**Critical Gaps Identified:**
{for each critical_gap}
- Lines {lines}: {description}
{end for}

**Suggested Tests:**
{for each suggested_test}
- {test_name}
{end for}

**Source:** {source_file} (QA {qa_date})
```

#### anti_pattern Description

```markdown
Fix {type} anti-pattern detected in {file}.

**Violation Details:**
- Type: {type}
- Severity: {severity}
- Location: Line {line}
- Description: {description}

**Remediation:**
{remediation_text}

**Reference:** devforgeai/specs/context/anti-patterns.md

**Source:** {source_file} (QA {qa_date})
```

#### code_quality Description

```markdown
Refactor to reduce {metric} in {file}.

**Quality Metrics:**
- Metric: {metric}
- Current: {current_value}
- Threshold: {threshold}
- Severity: {severity}

**Remediation:**
{remediation_text}

**Source:** {source_file} (QA {qa_date})
```

#### deferral Description

```markdown
Complete previously deferred Definition of Done item.

**Deferred Item:**
{item_text}

**Violation Type:** {violation_type}
- autonomous: Item deferred without user approval
- circular: Deferral references another deferred item
- invalid_reference: Referenced story/ADR doesn't exist

**Required Action:**
Either implement the deferred item or provide proper justification with user approval.

**Source:** {source_file} (QA {qa_date})
```

---

## Priority Mapping

### From Severity

| Gap Severity | Story Priority |
|--------------|----------------|
| CRITICAL | High |
| HIGH | High |
| MEDIUM | Medium |
| LOW | Low |

### Coverage Layer Override

| Layer | Story Priority |
|-------|----------------|
| Business Logic | High |
| Application | Medium |
| Infrastructure | Low |

### Deferral Override

All deferral issues are **High** priority (RCA compliance).

---

## Points Estimation

### Coverage Gaps

| Gap Percentage | Points |
|----------------|--------|
| < 5% | 2 |
| 5% - 15% | 3 |
| > 15% | 5 |

### Anti-Pattern Violations

| Severity | Points |
|----------|--------|
| CRITICAL | 5 |
| HIGH | 3 |
| MEDIUM | 2 |
| LOW | 1 |

### Code Quality Violations

| Severity | Points |
|----------|--------|
| HIGH | 3 |
| MEDIUM | 2 |
| LOW | 1 |

### Deferral Issues

All deferral issues: **3 points**

---

## Story Type Mapping

| Gap Type | Story Type | Rationale |
|----------|------------|-----------|
| coverage_gap | refactor | Adding tests without changing functionality |
| anti_pattern | refactor | Improving code structure |
| code_quality | refactor | Improving code metrics |
| deferral | bugfix | Compliance/requirement fix |

---

## Next Story ID Resolution

### Algorithm

```
1. Glob existing stories:
   Glob(pattern="devforgeai/specs/Stories/STORY-*.story.md")

2. Extract IDs:
   ids = [extract_id(file) for file in files]
   // STORY-001.story.md → 1
   // STORY-042.story.md → 42

3. Find maximum:
   max_id = max(ids) or 0

4. Next ID:
   next_id = max_id + 1

5. Format:
   story_id = "STORY-{next_id:03d}"
   // 1 → "STORY-001"
   // 42 → "STORY-043"
```

### Gap Handling

If there are gaps in sequence (e.g., STORY-001, STORY-003), still use max+1.
Do NOT fill gaps (could conflict with archived stories).

---

## Batch Mode Invocation

### Story Creation Call

```
Skill(command="devforgeai-story-creation")
```

With context markers in conversation before invocation.

### Batch Mode Detection

Story creation skill detects batch mode via:
```
**Batch Mode:** true
```

In batch mode, skill:
- Skips interactive Phase 1 questions
- Uses provided context markers
- Generates minimal acceptance criteria
- Creates story file directly

---

## Tracking Created Stories

### Success Tracking

```
$CREATED_STORIES.append({
    "story_id": "STORY-XXX",
    "gap_id": "GAP-YYY",
    "gap_type": "coverage_gap",
    "source_file": "src/module/file.py",
    "priority": "High",
    "points": 3
})
```

### Failure Tracking

```
$FAILED_STORIES.append({
    "gap_id": "GAP-ZZZ",
    "error": "Story creation failed: {error_message}",
    "gap_details": { /* original gap */ }
})
```

### Failure Isolation

If one story creation fails, continue to next gap.
Do NOT abort entire batch.
