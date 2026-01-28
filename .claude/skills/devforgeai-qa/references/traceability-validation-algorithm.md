# AC-to-DoD Traceability Validation Algorithm

**Purpose:** Map every Acceptance Criterion requirement to Definition of Done coverage, ensuring 100% traceability and preventing quality gate bypass.

**Used By:** devforgeai-qa skill Phase 0.9

**Version:** 1.0 (RCA-012 Phase 2)
**Created:** 2025-01-21

---

## Algorithm Overview

**5-Step Validation Process:**

1. **Extract AC Requirements** - Parse AC headers and granular requirements from story file
2. **Extract DoD Items** - Count and parse Definition of Done checkboxes
3. **Map AC → DoD** - Match requirements to DoD items via keyword analysis
4. **Calculate Traceability Score** - Determine coverage percentage (target: 100%)
5. **Validate Deferrals** - If DoD incomplete, verify "Approved Deferrals" section exists

**Inputs:** Story file path (already loaded in QA skill context)

**Outputs:**
- `traceability_score`: 0-100%
- `dod_completion_pct`: 0-100%
- `deferral_status`: "VALID" / "INVALID" / "N/A"
- `missing_traceability[]`: List of unmapped AC requirements
- `undocumented_deferrals[]`: List of incomplete DoD items without approval

**Quality Gate:** QA HALTS if `traceability_score < 100` OR `deferral_status == "INVALID"`

---

## Step 1: Extract AC Requirements

### Step 1.1: Count AC Headers

**Support Multiple Template Versions:**

```
# For Template v2.1+ (current)
ac_headers_v21 = grep count "^### AC#[0-9]+" story_file

# For Template v2.0/v1.0 (legacy)
ac_headers_v20 = grep count "^### [0-9]+\. \[" story_file

# Combined (use whichever is > 0)
IF ac_headers_v21 > 0:
  ac_count = ac_headers_v21
  template_version = "v2.1+"
ELSE IF ac_headers_v20 > 0:
  ac_count = ac_headers_v20
  template_version = "v2.0 or earlier"
ELSE:
  ac_count = 0
  Display: "⚠️ Warning: No AC headers found in story"
```

---

### Step 1.2: Extract Granular Requirements

**For each AC section, parse:**

**A. Given/When/Then Scenarios:**
```
# Find AC section boundaries
FOR i = 1 to ac_count:
  IF template_version == "v2.1+":
    ac_section = extract_between("^### AC#{i}:", "^### AC#{i+1}:|^##")
  ELSE:
    ac_section = extract_between("^### {i}\. \[", "^### {i+1}\. \[|^##")

  # Extract Then clauses
  then_clauses = grep_all "^\*\*Then\*\*(.+)$" in ac_section
  and_clauses = grep_all "^\*\*And\*\*(.+)$" in ac_section

  FOR each then_clause:
    ac_requirements.append({
      ac_number: i,
      text: then_clause,
      type: "functional",
      source: "Then clause"
    })

  FOR each and_clause:
    ac_requirements.append({
      ac_number: i,
      text: and_clause,
      type: "functional",
      source: "And clause"
    })
```

**Example:**
```markdown
### AC#1: User Authentication

**Given** user provides valid credentials
**When** user submits login
**Then** system authenticates user        ← Requirement 1
**And** system generates JWT token        ← Requirement 2
**And** token valid for 24 hours          ← Requirement 3
```

**Extracted:**
```
ac_requirements = [
  {ac_number: 1, text: "system authenticates user", type: "functional"},
  {ac_number: 1, text: "system generates JWT token", type: "functional"},
  {ac_number: 1, text: "token valid for 24 hours", type: "functional"}
]
```

---

**B. Bullet List Requirements:**
```
# Find bullet points after "Then" clause
bullet_requirements = grep_all "^- (.+)$" in ac_section (within Then block)

FOR each bullet:
  ac_requirements.append({
    ac_number: i,
    text: bullet,
    type: "content",
    source: "Bullet list"
  })
```

**Example:**
```markdown
**Then** the document contains:
- Introduction explaining why clear input matters (≥200 words)
- Command-specific guidance for 11 commands
- 20-30 before/after examples
- Quick reference checklist
```

**Extracted:**
```
ac_requirements += [
  {ac_number: 1, text: "Introduction (≥200 words)", type: "content"},
  {ac_number: 1, text: "11 commands", type: "content"},
  {ac_number: 1, text: "20-30 examples", type: "content"},
  {ac_number: 1, text: "Quick reference checklist", type: "content"}
]
```

---

**C. Measurable Criteria:**
```
# Extract quantitative requirements
metric_patterns = [
  r"≥(\d+)",      # Greater than or equal
  r"≤(\d+)",      # Less than or equal
  r"<(\d+)",      # Less than
  r">(\d+)",      # Greater than
  r"(\d+)-(\d+)", # Range
  r"(\d+)%",      # Percentage
  r"(\d+)\s+(words|items|commands|examples|ms|seconds)"  # Quantities
]

FOR each pattern in metric_patterns:
  metrics = grep_all pattern in ac_section

  FOR each metric:
    ac_requirements.append({
      ac_number: i,
      text: metric_context,  # Full sentence containing metric
      type: "measurable",
      metric: metric_value,
      source: "Metric"
    })
```

**Example:**
```markdown
**Then** API responds in <200ms (p95)
```

**Extracted:**
```
{ac_number: 2, text: "API responds in <200ms", type: "measurable", metric: "200ms"}
```

---

### Step 1.3: Store Results

```
total_ac_requirements = ac_requirements.length

Display (for debugging/transparency):
  "AC Analysis: {ac_count} ACs, {total_ac_requirements} granular requirements"
```

**Example Output:**
```
STORY-052:
  AC Count: 6
  Granular Requirements: 30
    AC#1: 6 requirements
    AC#2: 5 requirements
    AC#3: 5 requirements
    AC#4: 4 requirements
    AC#5: 5 requirements
    AC#6: 5 requirements
```

---

## Step 2: Extract DoD Items

### Step 2.1: Locate DoD Section

```
# Find DoD section boundaries
dod_section = extract_between("^## Definition of Done", "^## Workflow Status|^## Notes|^---$")

IF dod_section.length == 0:
  Display: "⚠️ Error: No Definition of Done section found"
  HALT QA (story structure invalid)
```

---

### Step 2.2: Parse DoD Subsections

```
# DoD has 4 standard subsections
subsections = ["Implementation", "Quality", "Testing", "Documentation"]

dod_items = []

FOR each subsection in subsections:
  subsection_content = extract_between("^### {subsection}", "^###|^##")

  # Find all checkbox lines
  checkbox_lines = grep_all "^- \[(x| )\] (.+)$" in subsection_content

  FOR each checkbox_line:
    Extract:
      - status: "x" or " " (space)
      - text: full item text after checkbox

    dod_items.append({
      section: subsection,
      status: status,
      text: text,
      checked: (status == "x")
    })
```

---

### Step 2.3: Calculate DoD Metrics

```
dod_total = dod_items.length
dod_checked = count(items where checked == true)
dod_unchecked = count(items where checked == false)

dod_completion_pct = (dod_checked / dod_total) × 100
```

**Example (STORY-052):**
```
DoD Analysis:
  Total items: 26
  Checked [x]: 26
  Unchecked [ ]: 0
  Completion: 100%
```

**Example (STORY-023 with deferrals):**
```
DoD Analysis:
  Total items: 22
  Checked [x]: 15
  Unchecked [ ]: 7
  Completion: 68%
```

---

## Step 3: Map AC Requirements to DoD Coverage

### Step 3.1: Keyword Extraction Function

```
FUNCTION extract_keywords(text):
  # Convert to lowercase
  text = lowercase(text)

  # Remove stop words
  stop_words = ["the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "is", "are", "has", "have", "be", "been"]

  words = split(text, by: whitespace and punctuation)

  keywords = []
  FOR each word in words:
    IF word NOT in stop_words AND word.length > 2:
      keywords.append(word)

  # Include numbers/metrics as-is
  numbers = extract_numbers(text)
  keywords.extend(numbers)

  RETURN keywords
```

**Example:**
```
Text: "Document includes introduction (≥200 words)"
Keywords: ["document", "includes", "introduction", "200", "words"]

Text: "API response time <200ms (p95)"
Keywords: ["api", "response", "time", "200", "ms", "p95"]
```

---

### Step 3.2: Keyword Matching

```
traceability_map = {}
missing_traceability = []

FOR each ac_req in ac_requirements:
  ac_keywords = extract_keywords(ac_req.text)

  best_match = null
  best_match_score = 0

  # Search all DoD items for matches
  FOR each dod_item in dod_items:
    dod_keywords = extract_keywords(dod_item.text)

    # Calculate overlap
    common_keywords = intersection(ac_keywords, dod_keywords)
    match_score = common_keywords.length / ac_keywords.length

    IF match_score > best_match_score:
      best_match = dod_item
      best_match_score = match_score

  # Apply threshold
  IF best_match_score >= 0.5:  # 50%+ keyword overlap required
    # Coverage found
    coverage_type = determine_coverage_type(ac_req, best_match)

    traceability_map[ac_req] = {
      dod_item: best_match,
      coverage_type: coverage_type,
      match_score: best_match_score,
      status: "COVERED"
    }

  ELSE:
    # No sufficient match found
    missing_traceability.append(ac_req)
```

---

### Step 3.3: Determine Coverage Type

```
FUNCTION determine_coverage_type(ac_requirement, dod_item):
  # Type 1: Explicit Checkbox
  # DoD item directly mentions requirement with measurable criteria

  IF dod_item.text contains ac_requirement key terms:
    IF dod_item.text contains numbers/metrics:
      RETURN "Explicit Checkbox with Metric"
    ELSE:
      RETURN "Explicit Checkbox"

  # Type 2: Test Validation
  # DoD item references test that validates requirement

  IF dod_item.text contains "test" AND ac_requirement.ac_number:
    # Example: "Integration test (AC#2 - PASS)"
    IF "AC#{ac_requirement.ac_number}" in dod_item.text:
      RETURN "Test Validation (AC-specific)"
    ELSE IF "test" in dod_item.text.lowercase():
      RETURN "Test Validation (General)"

  # Type 3: Metric Validation
  # DoD item provides quantitative proof

  IF dod_item.text contains metric AND ac_requirement.metric:
    RETURN "Metric Validation"

  # Default
  RETURN "Implicit Coverage"
```

**Example Mappings:**

```
AC Req: "Introduction ≥200 words"
DoD Item: "Document includes introduction (648 words)"
Coverage Type: "Explicit Checkbox with Metric"
Match Score: 1.0 (100% - all keywords match)

AC Req: "Examples show realistic user input"
DoD Item: "Example validation test (AC#2 - PASS)"
Coverage Type: "Test Validation (AC-specific)"
Match Score: 0.6 (60% - "examples" matches, test validates)

AC Req: "System processes in <100ms"
DoD Item: "Performance validated (<100ms p95)"
Coverage Type: "Metric Validation"
Match Score: 0.8 (80% - performance, 100ms match)
```

---

### Step 3.4: Calculate Traceability Score

```
covered_requirements = total_ac_requirements - missing_traceability.length
traceability_score = (covered_requirements / total_ac_requirements) × 100
```

**Example (STORY-052):**
```
Total Requirements: 30
Covered: 30
Missing: 0
Score: 100% ✅
```

**Example (Incomplete Story):**
```
Total Requirements: 10
Covered: 7
Missing: 3
  - AC#2: "response time <200ms" (no DoD item)
  - AC#3: "error handling returns message" (no DoD item)
  - AC#4: "logs all transactions" (no DoD item)
Score: 70% ❌ (100% required)
```

---

## Step 4: Validate Deferrals (If DoD Incomplete)

### Step 4.1: Check for Incomplete Items

```
IF dod_unchecked > 0:
  # Story has incomplete DoD items - need to validate deferrals

  # Search for "Approved Deferrals" section in Implementation Notes
  impl_notes = extract_section("^## Implementation Notes", story_file)

  has_deferral_section = search_in(impl_notes, "^## Approved Deferrals")

  IF has_deferral_section:
    # Section exists - validate it
    GOTO Step 4.2
  ELSE:
    # Section missing
    deferral_status = "INVALID (no Approved Deferrals section found)"
    undocumented_deferrals = all items where checked == false
    GOTO Step 5 (quality gate decision)
```

---

### Step 4.2: Extract Deferral Documentation

```
deferral_section = extract_section("^## Approved Deferrals", impl_notes)

# Extract user approval timestamp
timestamp_pattern = r"User Approval:\s*(.+?UTC)"
user_approval = extract_match(timestamp_pattern, deferral_section)

IF NOT user_approval:
  deferral_status = "INVALID (missing user approval timestamp)"
  GOTO Step 5

# Extract deferred items list
deferred_items_text = extract_between("^**Deferred Items:**", "^##|^**Total|^**Rationale")

# Parse numbered list
deferred_items = []
FOR each line matching "^\d+\.\s+\*\*(.+?)\*\*":
  item_text = extract_match(line)
  deferred_items.append(item_text)
```

---

### Step 4.3: Match Unchecked DoD Items to Deferral List

```
documented_deferrals = []
undocumented_deferrals = []

FOR each dod_item in dod_items WHERE dod_item.checked == false:
  # Search deferred items list for match
  found = false

  FOR each deferred_item in deferred_items:
    # Match by keyword overlap
    keywords_dod = extract_keywords(dod_item.text)
    keywords_deferred = extract_keywords(deferred_item)

    common = intersection(keywords_dod, keywords_deferred)
    match_score = common.length / keywords_dod.length

    IF match_score >= 0.6:  # 60%+ match
      documented_deferrals.append(dod_item)
      found = true
      BREAK

  IF NOT found:
    undocumented_deferrals.append(dod_item)

# Determine status
IF undocumented_deferrals.length == 0:
  deferral_status = "VALID (all {dod_unchecked} items user-approved on {user_approval})"
ELSE:
  deferral_status = "INVALID ({undocumented_deferrals.length} items lack approval)"
```

**Example (STORY-023 - Valid Deferrals):**
```
Unchecked DoD Items: 7
Deferred Items in Section: 7
Match: 7/7 (100%)
Deferral Status: VALID (all 7 items user-approved on 2025-11-14 14:30 UTC)
```

**Example (STORY-038 - Invalid):**
```
Unchecked DoD Items: 4
Deferred Items in Section: 0 (no section)
Match: 0/4 (0%)
Deferral Status: INVALID (no Approved Deferrals section found)
```

---

## Step 5: Quality Gate Decision

### Decision Logic

```
# Rule 1: Traceability must be 100%
IF traceability_score < 100:
  result = "FAIL: Insufficient Traceability"
  action = "HALT"
  remediation = "Add DoD items for missing AC requirements"
  EXIT Phase 0.9 (do not proceed to Phase 1)

# Rule 2: Incomplete DoD must have valid deferrals
IF dod_unchecked > 0 AND deferral_status contains "INVALID":
  result = "FAIL: Undocumented Deferrals"
  action = "HALT"
  remediation = "Add Approved Deferrals section with user approval"
  EXIT Phase 0.9 (do not proceed to Phase 1)

# Both rules pass
IF traceability_score == 100 AND (dod_unchecked == 0 OR deferral_status == "VALID"):
  result = "PASS"
  action = "CONTINUE"
  remediation = "None needed"
  Proceed to Phase 1 (Validation Mode Selection)
```

---

## Edge Cases and Handling

### Edge Case 1: Multiple DoD Items Cover Single AC Requirement

**Scenario:**
```
AC#1: "User authentication with email and password"

DoD Coverage:
  - [x] Email validation implemented
  - [x] Password hashing implemented (bcrypt)
  - [x] JWT token generation implemented
```

**Handling:**
- All 3 DoD items match keywords from AC requirement
- Count as COVERED (collective validation acceptable)
- Coverage type: "Multiple Explicit Checkboxes"

**Algorithm:**
```
# Multiple matches allowed
IF match_score >= 0.5 for ANY dod_item:
  Mark ac_req as COVERED
  Store all matching dod_items (not just best)
```

---

### Edge Case 2: Single DoD Item Covers Multiple AC Requirements

**Scenario:**
```
AC#1: Feature works correctly
AC#2: Error handling functions
AC#3: Performance targets met

DoD Item:
  - [x] All tests passing (validates AC#1, AC#2, AC#3)
```

**Handling:**
- DoD item explicitly references multiple ACs
- Count as COVERED for all referenced ACs
- Coverage type: "Test Validation (Rollup)"

**Algorithm:**
```
# Check for AC references in DoD text
IF dod_item.text contains "AC#{number}":
  Extract: referenced_acs = all AC numbers mentioned

  FOR each referenced_ac in referenced_acs:
    Mark all requirements from that AC as COVERED
    Coverage type: "Test Validation (Rollup)"
```

---

### Edge Case 3: Test-Based Validation for Multiple Requirements

**Scenario:**
```
AC#2: Example Quality (5 granular requirements)
  - Realistic user input
  - Specific improvements
  - ≥50 words explanations
  - References actual commands
  - Measurable improvements

DoD Item:
  - [x] Example validation test (AC#2 - PASS)
```

**Handling:**
- Single test validates all 5 requirements algorithmically
- Count as COVERED for all AC#2 requirements
- Coverage type: "Test Validation (Comprehensive)"

**Algorithm:**
```
IF dod_item.text matches "test.*AC#{ac_number}":
  # This test covers ALL requirements from that AC
  FOR each req in ac_requirements WHERE req.ac_number == ac_number:
    Mark req as COVERED
    Coverage type: "Test Validation (Comprehensive)"
```

---

### Edge Case 4: Design-Phase Story (Implementation Deferred)

**Scenario:** STORY-023 (design complete, implementation deferred to STORY-024)

**Expected:**
- AC requirements: Design-related (7 ACs)
- DoD items: 22 total (15 design items complete, 7 implementation items deferred)
- Deferrals: Documented with user approval, reference STORY-024

**Handling:**
- Traceability: 100% (design requirements have design DoD items)
- DoD completion: 68% (implementation deferred)
- Deferral status: VALID (all 7 items documented with follow-up story)

**Algorithm treats this as PASS:**
```
Traceability: 100% (design ACs → design DoD items) ✓
Deferrals: VALID (7 items in Approved Deferrals section) ✓
Result: PASS (design complete, implementation properly deferred)
```

---

## Keyword Matching Examples

### Example 1: Direct Match

```
AC Requirement: "Introduction explaining why clear input matters (≥200 words)"
Keywords: ["introduction", "explaining", "clear", "input", "matters", "200", "words"]

DoD Item: "Document includes introduction (648 words explaining purpose and value)"
Keywords: ["document", "includes", "introduction", "648", "words", "explaining", "purpose", "value"]

Common: ["introduction", "words", "explaining"]
Match Score: 3/7 = 43%

Threshold: 50%
Result: FAIL (below threshold)
```

**Issue:** Match score too low despite being valid coverage.

**Fix:** Adjust keywords to include "document"/"intro" as synonyms:
```
AC Keywords (enhanced): ["intro", "introduction", "200", "words", "explaining"]
DoD Keywords (enhanced): ["intro", "introduction", "648", "words", "explaining"]

Common: ["intro", "introduction", "words", "explaining"]
Match Score: 4/5 = 80%

Result: PASS (above threshold) ✅
```

---

### Example 2: Test Validation Match

```
AC Requirement: "Examples show realistic user input patterns"
Keywords: ["examples", "realistic", "user", "input", "patterns"]

DoD Item: "Example validation test (AC#2 - PASS)"
Keywords: ["example", "validation", "test", "ac", "2", "pass"]

Common: ["example"] (note: "examples" → "example" after stemming)
Match Score: 1/5 = 20%

Threshold: 50%
Result: FAIL (below threshold)
```

**But:** DoD item explicitly references "AC#2"

**Special Rule:**
```
IF dod_item contains "AC#{ac_req.ac_number}":
  # Explicit AC reference overrides keyword matching
  Mark as COVERED
  Coverage type: "Test Validation (AC-specific)"
  Result: PASS ✅
```

---

### Example 3: Metric Match

```
AC Requirement: "API response time <200ms (p95)"
Keywords: ["api", "response", "time", "200", "ms", "p95"]

DoD Item: "Performance validated: <200ms p95 under load"
Keywords: ["performance", "validated", "200", "ms", "p95", "load"]

Common: ["200", "ms", "p95"]
Match Score: 3/6 = 50%

Result: PASS (exactly at threshold) ✅
Coverage type: "Metric Validation"
```

---

## Display Output Format

### When Traceability Validation Runs

**Display Header:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 0.9: AC-DoD Traceability Validation (RCA-012)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display Analysis:**
```
Acceptance Criteria Analysis:
  • Template version: {v2.1+ / v2.0 / v1.0}
  • Total ACs: {ac_count}
  • Total requirements (granular): {total_ac_requirements}
  • DoD items: {dod_total}

Traceability Mapping:
  • AC#1 ({req_count} requirements) → {dod_count} DoD items {✓ / ✗}
  • AC#2 ({req_count} requirements) → {dod_count} DoD items {✓ / ✗}
  {... for each AC ...}

Traceability Score: {score}% {✅ PASS / ❌ FAIL}
```

**Display DoD Status:**
```
DoD Completion Status:
  • Total items: {dod_total}
  • Complete [x]: {dod_checked}
  • Incomplete [ ]: {dod_unchecked}
  • Completion: {completion_pct}%
```

**Display Deferrals (if applicable):**
```
{IF dod_unchecked > 0}:
Deferral Documentation: {deferral_status}
  • Approved Deferrals section: {EXISTS / MISSING}
  • User approval timestamp: {timestamp / MISSING}
  • Documented deferrals: {count}/{dod_unchecked} items
```

**Display Result:**
```
{IF PASS}:
✓ PASS - Traceability validated, story ready for QA validation
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

{IF FAIL}:
❌ FAIL - {failure reason}
{remediation guidance}
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## Implementation Notes

**Keyword Synonym Expansion:**
To improve matching accuracy, expand keywords with common synonyms:

```
synonyms = {
  "introduction": ["intro", "introduction", "preface"],
  "document": ["doc", "document", "guide", "file"],
  "test": ["test", "testing", "validation", "verify"],
  "implement": ["implement", "implemented", "implementation", "create", "created"],
  "complete": ["complete", "completed", "done", "finished"]
}

FUNCTION expand_keywords(keywords):
  expanded = []
  FOR each keyword in keywords:
    expanded.append(keyword)  # Original
    IF keyword in synonyms:
      expanded.extend(synonyms[keyword])  # Add synonyms
  RETURN unique(expanded)
```

**Match Threshold Tuning:**
- Current: 50% (conservative, may have false negatives)
- Adjust if needed: 40% (more lenient, may have false positives)
- Test with real stories to find optimal threshold

**AC Reference Override:**
- If DoD item explicitly mentions "AC#N", automatic coverage for that AC
- Overrides keyword matching (explicit > implicit)
- Prevents false negatives for test validation items

---

**Algorithm Complete**

**Complexity:** O(A × D) where A = AC requirements, D = DoD items
**Performance:** ~2K tokens, <5 seconds for typical story
**Accuracy:** Expected >95% with synonym expansion and threshold tuning
