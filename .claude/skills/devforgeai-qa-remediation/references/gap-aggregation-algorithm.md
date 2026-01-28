# Gap Aggregation Algorithm

**Phase:** 03 (Aggregation & Prioritization)
**Purpose:** Deduplicate, score, filter, and sort gaps for processing.

---

## Step 1: Deduplication

### Duplicate Detection Criteria

Two gaps are considered duplicates if ALL match:
1. Same `file` path
2. Same `type` (coverage_gap, anti_pattern, etc.)
3. Same or overlapping `description` (fuzzy match)

### Deduplication Algorithm

```
$UNIQUE_GAPS = []
$SEEN = {}  // key: "{file}:{type}:{normalized_description}"

for each gap in $ALL_GAPS:
    key = generate_dedup_key(gap)

    if key in $SEEN:
        // Merge with existing
        existing = $SEEN[key]
        existing.occurrences += 1
        existing.source_files.append(gap.source_file)

        // Keep higher severity
        if severity_rank(gap.severity) > severity_rank(existing.severity):
            existing.severity = gap.severity
    else:
        // New unique gap
        gap.occurrences = 1
        gap.source_files = [gap.source_file]
        $SEEN[key] = gap
        $UNIQUE_GAPS.append(gap)
```

### Key Generation

```
function generate_dedup_key(gap):
    normalized_desc = gap.description.lower()
                          .replace(/\s+/g, ' ')
                          .substring(0, 100)
    return "{gap.file}:{gap.type}:{normalized_desc}"
```

---

## Step 2: Severity Scoring

### Base Severity Weights

From configuration (`qa-remediation.yaml`):

| Severity | Weight |
|----------|--------|
| CRITICAL | 100 |
| HIGH | 75 |
| MEDIUM | 50 |
| LOW | 25 |

### Type Modifiers

Additional scoring based on gap type:

| Gap Type | Condition | Modifier |
|----------|-----------|----------|
| deferral_issue | any | +25 (RCA compliance priority) |
| coverage_gap | Business Logic layer | +10 |
| coverage_gap | Application layer | +5 |
| anti_pattern | security-related type | +15 |
| code_quality | complexity metric | +5 |

### Security-Related Anti-Patterns

```
SECURITY_TYPES = [
    "sql_injection",
    "hardcoded_secret",
    "xss_vulnerability",
    "authentication_bypass",
    "insecure_deserialization"
]
```

### Scoring Algorithm

```
function calculate_score(gap):
    // Base score from severity
    score = SEVERITY_WEIGHTS[gap.severity]

    // Type modifiers
    if gap.type == "deferral":
        score += 25

    if gap.type == "coverage_gap":
        if gap.details.layer == "Business Logic":
            score += 10
        elif gap.details.layer == "Application":
            score += 5

    if gap.type == "anti_pattern":
        if gap.details.type in SECURITY_TYPES:
            score += 15

    if gap.type == "code_quality":
        if gap.details.metric == "cyclomatic_complexity":
            score += 5

    // Occurrence bonus (duplicates indicate systemic issue)
    if gap.occurrences > 1:
        score += min(gap.occurrences * 2, 10)

    return score
```

---

## Step 3: Severity Filtering

### Filter Based on $MIN_SEVERITY

```
SEVERITY_RANK = {
    "CRITICAL": 4,
    "HIGH": 3,
    "MEDIUM": 2,
    "LOW": 1
}

function filter_by_severity(gaps, min_severity):
    min_rank = SEVERITY_RANK[min_severity]

    $FILTERED_GAPS = []
    $DEFERRED_GAPS = []

    for each gap in gaps:
        gap_rank = SEVERITY_RANK[gap.severity]

        if gap_rank >= min_rank:
            $FILTERED_GAPS.append(gap)
        else:
            gap.deferred_reason = "Below --min-severity {min_severity}"
            $DEFERRED_GAPS.append(gap)

    return ($FILTERED_GAPS, $DEFERRED_GAPS)
```

### Severity Filter Matrix

| $MIN_SEVERITY | Includes | Defers |
|---------------|----------|--------|
| `CRITICAL` | CRITICAL | HIGH, MEDIUM, LOW |
| `HIGH` | CRITICAL, HIGH | MEDIUM, LOW |
| `MEDIUM` | CRITICAL, HIGH, MEDIUM | LOW |
| `LOW` | All | None |

---

## Step 4: Sorting

### Primary Sort: Score (Descending)

```
$FILTERED_GAPS.sort(
    key=lambda gap: gap.score,
    reverse=True
)
```

### Secondary Sort: Type Priority (for equal scores)

```
TYPE_PRIORITY = {
    "deferral": 1,      // Highest priority
    "anti_pattern": 2,
    "coverage_gap": 3,
    "code_quality": 4   // Lowest priority
}

$FILTERED_GAPS.sort(
    key=lambda gap: (gap.score * -1, TYPE_PRIORITY[gap.type])
)
```

---

## Output Variables

| Variable | Description |
|----------|-------------|
| `$UNIQUE_GAPS` | Gaps after deduplication |
| `$FILTERED_GAPS` | Gaps above severity threshold (sorted by score) |
| `$DEFERRED_GAPS` | Gaps below threshold (for debt register) |
| `$GAPS_DEDUPLICATED` | Count of duplicates removed |
| `$GAPS_ABOVE_THRESHOLD` | Count passing severity filter |
| `$GAPS_DEFERRED` | Count filtered out |

---

## Example Scoring

| Gap | Severity | Type | Layer | Score Calculation | Final |
|-----|----------|------|-------|-------------------|-------|
| Deferral violation | CRITICAL | deferral | - | 100 + 25 = | **125** |
| SQL injection | CRITICAL | anti_pattern | - | 100 + 15 = | **115** |
| Business Logic coverage | HIGH | coverage_gap | Business | 75 + 10 = | **85** |
| Complexity warning | MEDIUM | code_quality | - | 50 + 5 = | **55** |
| Style issue | LOW | anti_pattern | - | 25 = | **25** |
