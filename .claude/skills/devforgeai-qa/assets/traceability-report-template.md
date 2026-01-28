# AC-to-DoD Traceability Report Templates

**Purpose:** Standardized display formatting for Phase 0.9 traceability validation results

**Used By:** devforgeai-qa skill Phase 0.9

**Version:** 1.0 (RCA-012 Phase 2)
**Created:** 2025-01-21

---

## Template Selection Logic

```
IF traceability_score == 100 AND (dod_unchecked == 0 OR deferral_status == "VALID"):
  USE: PASS_TEMPLATE
  ACTION: Display success, continue to Phase 1

ELSE IF traceability_score < 100:
  USE: FAIL_TEMPLATE_TRACEABILITY
  ACTION: Display missing requirements, HALT workflow

ELSE IF dod_unchecked > 0 AND deferral_status contains "INVALID":
  USE: FAIL_TEMPLATE_DEFERRALS
  ACTION: Display deferral template, HALT workflow
```

---

## PASS Template (100% Traceability)

**Use When:** All AC requirements have DoD coverage AND (DoD 100% complete OR deferrals documented)

**Template:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 0.9: AC-DoD Traceability Validation (RCA-012)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Acceptance Criteria Analysis:
  • Template version: {template_version}
  • Total ACs: {ac_count}
  • Total requirements (granular): {total_ac_requirements}
  • DoD items: {dod_total}

Traceability Mapping:
  • AC#1 ({ac1_req_count} requirements) → {ac1_dod_count} DoD items ✓
  • AC#2 ({ac2_req_count} requirements) → {ac2_dod_count} DoD items ✓
  • AC#3 ({ac3_req_count} requirements) → {ac3_dod_count} DoD items ✓
  {... repeat for each AC ...}

Traceability Score: {traceability_score}% ✅

DoD Completion Status:
  • Total items: {dod_total}
  • Complete [x]: {dod_checked}
  • Incomplete [ ]: {dod_unchecked}
  • Completion: {dod_completion_pct}%

{IF dod_unchecked > 0}:
Deferral Documentation: {deferral_status}
  • Approved Deferrals section: EXISTS ✓
  • User approval timestamp: {user_approval_timestamp}
  • Documented deferrals: {documented_count}/{dod_unchecked} items (100%)

✓ PASS - Traceability validated, story ready for QA validation

Proceeding to Phase 1 (Validation Mode Selection)...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Variable Substitutions:**
- `{template_version}`: "v2.1+" or "v2.0" or "v1.0"
- `{ac_count}`: Number of ACs (e.g., 6)
- `{total_ac_requirements}`: Total granular requirements (e.g., 30)
- `{dod_total}`: Total DoD items (e.g., 26)
- `{acN_req_count}`: Requirements for AC#N (e.g., 6 for AC#1)
- `{acN_dod_count}`: DoD items covering AC#N (e.g., 6 items)
- `{traceability_score}`: Percentage (e.g., 100)
- `{dod_checked}`: Checked items (e.g., 26)
- `{dod_unchecked}`: Unchecked items (e.g., 0 or 7)
- `{dod_completion_pct}`: Completion percentage (e.g., 100 or 68)
- `{deferral_status}`: "VALID (...)" or "N/A (DoD 100% complete)"
- `{user_approval_timestamp}`: "2025-11-14 14:30 UTC"
- `{documented_count}`: Count of documented deferrals

---

## FAIL Template: Missing Traceability

**Use When:** One or more AC requirements have no DoD coverage (traceability_score < 100%)

**Template:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 0.9: AC-DoD Traceability Validation (RCA-012)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Acceptance Criteria Analysis:
  • Template version: {template_version}
  • Total ACs: {ac_count}
  • Total requirements: {total_ac_requirements}
  • DoD items: {dod_total}

Traceability Mapping:
  • AC#1 ({ac1_req_count} requirements) → {ac1_dod_count} DoD items ✓
  • AC#2 ({ac2_req_count} requirements) → NOT FOUND ✗
  • AC#3 ({ac3_req_count} requirements) → {ac3_dod_count} DoD items ✓
  {... highlight ✗ FAIL for ACs with missing coverage ...}

Traceability Score: {traceability_score}% ❌ (100% required)

Missing DoD Coverage for AC Requirements:
  {FOR each req in missing_traceability}:
  • AC#{req.ac_number}: {req.text}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
❌ QA VALIDATION FAILED - AC-DoD Traceability Insufficient
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Traceability Score: {traceability_score}% (100% required)

Missing Coverage: {missing_count} AC requirements have no DoD validation

ACTION REQUIRED (Choose One):

Option 1: Add DoD Items (Recommended)
────────────────────────────────────────
Add to appropriate DoD subsection (Implementation/Quality/Testing/Documentation):

### Implementation
{FOR each missing req where type == "functional"}:
- [ ] {req.text with measurable criteria}

### Testing
{FOR each missing req where type == "testable"}:
- [ ] {req.text with test description}

Example DoD Items:
- [ ] Performance validated: API response <200ms (p95)
- [ ] Error handling tested for 5 failure scenarios
- [ ] Logging implemented for all transactions (debug level)

After adding items, re-run: /qa {STORY_ID}

Option 2: Clarify Existing Coverage
────────────────────────────────────────
If DoD items exist but different wording prevented detection:

1. Review DoD items for implicit coverage of missing requirements
2. Update DoD item text to explicitly mention AC requirement
3. Add AC reference for clarity: "(validates AC#N: {requirement})"

Example:
  Before: "- [x] All tests passing"
  After:  "- [x] All tests passing (validates AC#2: error handling, AC#3: performance)"

After clarifying, re-run: /qa {STORY_ID}

Option 3: Update AC (If Requirement Invalid)
────────────────────────────────────────
If AC requirement is obsolete, incorrectly stated, or no longer relevant:

1. Edit story file AC section (remove or revise requirement)
2. Document decision in Implementation Notes:
   "AC requirement '{text}' removed on {date} - Reason: {explanation}"
3. Re-run: /qa {STORY_ID}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
QA WORKFLOW HALTED - Fix traceability issues before proceeding
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Variable Substitutions:**
- `{missing_count}`: Count of unmapped requirements
- `{req.text}`: Text of each missing requirement
- `{req.ac_number}`: Which AC the requirement belongs to
- `{req.type}`: Type (functional, testable, measurable, content)

---

## FAIL Template: Undocumented Deferrals

**Use When:** DoD has unchecked items but no "Approved Deferrals" section OR section incomplete

**Template:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 0.9: AC-DoD Traceability Validation (RCA-012)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Acceptance Criteria Analysis:
  • Total ACs: {ac_count}
  • Total requirements: {total_ac_requirements}

Traceability Mapping:
  • AC#1 → {dod_count} DoD items ✓
  • AC#2 → {dod_count} DoD items ✓
  {... all ACs show ✓ if traceability is 100% ...}

Traceability Score: {traceability_score}% ✅

DoD Completion Status:
  • Total items: {dod_total}
  • Complete [x]: {dod_checked}
  • Incomplete [ ]: {dod_unchecked}
  • Completion: {dod_completion_pct}%

Deferral Documentation: {deferral_status}

{IF no deferral section}:
  • Approved Deferrals section: MISSING ✗
  • Incomplete items: {dod_unchecked} (none documented)

{IF partial documentation}:
  • Approved Deferrals section: EXISTS ✓
  • User approval timestamp: {EXISTS / MISSING}
  • Documented deferrals: {documented_count}/{dod_unchecked} items
  • Undocumented: {undocumented_count} items ✗

Undocumented Incomplete DoD Items:
  {FOR each item in undocumented_deferrals}:
  • {item.section}: {item.text}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
❌ QA VALIDATION FAILED - Incomplete DoD Without Approval
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DoD Completion: {dod_completion_pct}% ({dod_checked}/{dod_total} items complete)

Incomplete Items: {dod_unchecked}

Deferral Documentation: {deferral_status}

{IF no section}:
Story has {dod_unchecked} incomplete DoD items without documented user approval.
No "Approved Deferrals" section found in Implementation Notes.

{IF partial}:
"Approved Deferrals" section exists but only documents {documented_count}/{dod_unchecked} items.
{undocumented_count} items lack approval.

ACTION REQUIRED (Choose One):

Option 1: Complete All Items (Preferred)
────────────────────────────────────────
Implement all {dod_unchecked} incomplete DoD items:

{FOR each incomplete item}:
• {item.section}: {item.text}
  Suggested action: {implementation_hint based on item type}

After implementing, mark items [x] and re-run: /qa {STORY_ID}

Option 2: Document Deferrals (If Valid Blockers)
────────────────────────────────────────
Add "Approved Deferrals" section to Implementation Notes:

## Approved Deferrals

**User Approval:** {current_timestamp} UTC
**Approval Type:** {Design-Phase / Low-Priority / Blocker-Dependent}

**Deferred Items:**
{FOR each incomplete item}:
{N}. **{item.text}**
   - Reason: {Why this is deferred - be specific about blocker}
   - Blocker Type: {Dependency / Toolchain / Artifact / ADR / Low-Priority}
   - Follow-up: {Story reference (STORY-XXX) OR completion condition}
   - Impact: {Optional - significance of deferring this item}

{Example for first item}:
1. **{undocumented_deferrals[0].text}**
   - Reason: No load testing infrastructure available for benchmarking
   - Blocker Type: Toolchain (requires JMeter setup, STORY-XXX prerequisite)
   - Follow-up: Implement after STORY-XXX (load testing infrastructure) complete
   - Impact: Core functionality tested, performance validation deferred to production

**Total Deferred:** {dod_unchecked} items ({deferral_pct}% of DoD)
**Completion Status:** {dod_checked}/{dod_total} items complete

**Rationale for Approval:**
{Explain why these deferrals are acceptable - what's complete vs. deferred}

After adding section, re-run: /qa {STORY_ID}

Option 3: Remove Items (If No Longer Relevant)
────────────────────────────────────────
If DoD items are obsolete or incorrectly included:

1. Edit story file Definition of Done section
2. Remove obsolete items (delete checkbox lines)
3. Document removal decision in Implementation Notes:
   "Removed DoD items on {date}: {item list} - Reason: {explanation}"
4. Re-run: /qa {STORY_ID}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
QA WORKFLOW HALTED - Fix deferral documentation before proceeding
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

This validation prevents quality gate bypass (RCA-012 remediation).
Stories cannot reach "QA Approved" with undocumented incomplete DoD items.

See: CLAUDE.md "Acceptance Criteria vs. Tracking Mechanisms" for guidance
See: devforgeai/RCA/RCA-012/ for complete remediation analysis
```

**Variable Substitutions:**
- `{current_timestamp}`: Current date/time in UTC
- `{deferral_pct}`: Percentage deferred (unchecked/total × 100)
- `{undocumented_count}`: Count of items lacking approval
- `{documented_count}`: Count of items in deferral section
- `{item.section}`: Implementation / Quality / Testing / Documentation
- `{item.text}`: Full DoD item text
- `{implementation_hint}`: Contextual suggestion based on item type

---

## Display Examples

### Example 1: STORY-007 (Perfect - PASS)

**Data:**
- AC count: 6
- Total requirements: 16
- DoD items: 22
- DoD completion: 100%
- Traceability: 100%

**Output:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 0.9: AC-DoD Traceability Validation (RCA-012)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Acceptance Criteria Analysis:
  • Template version: v2.0
  • Total ACs: 6
  • Total requirements (granular): 16
  • DoD items: 22

Traceability Mapping:
  • AC#1 (3 requirements) → 6 DoD items ✓
  • AC#2 (3 requirements) → 4 DoD items ✓
  • AC#3 (2 requirements) → 2 DoD items ✓
  • AC#4 (3 requirements) → 4 DoD items ✓
  • AC#5 (2 requirements) → 3 DoD items ✓
  • AC#6 (3 requirements) → 3 DoD items ✓

Traceability Score: 100% ✅

DoD Completion Status:
  • Total items: 22
  • Complete [x]: 22
  • Incomplete [ ]: 0
  • Completion: 100%

✓ PASS - Traceability validated, story ready for QA validation

Proceeding to Phase 1 (Validation Mode Selection)...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

### Example 2: Missing Coverage (FAIL - Traceability)

**Data:**
- AC#1: 3 requirements (all covered)
- AC#2: 2 requirements (1 covered, 1 missing: "response <100ms")
- AC#3: 1 requirement (missing: "logs transactions")
- Traceability: 70% (5/7 covered)

**Output:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 0.9: AC-DoD Traceability Validation (RCA-012)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Acceptance Criteria Analysis:
  • Total ACs: 3
  • Total requirements: 7

Traceability Mapping:
  • AC#1 (3 requirements) → 3 DoD items ✓
  • AC#2 (2 requirements) → 1 DoD item ⚠️ (1 missing)
  • AC#3 (1 requirement) → NOT FOUND ✗

Traceability Score: 71% ❌ (100% required)

Missing DoD Coverage for AC Requirements:
  • AC#2: response time <100ms (performance requirement)
  • AC#3: logs all transactions (logging requirement)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
❌ QA VALIDATION FAILED - AC-DoD Traceability Insufficient
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Missing Coverage: 2 AC requirements have no DoD validation

ACTION REQUIRED:

Add to Definition of Done:

### Implementation
- [ ] Logging implemented for all transactions (debug level)

### Quality
- [ ] Performance validated: Response time <100ms (p95)

After fixing, re-run: /qa {STORY_ID}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
QA WORKFLOW HALTED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

### Example 3: STORY-038 Pattern (FAIL - Deferrals)

**Data:**
- Traceability: 100% (all ACs have DoD coverage)
- DoD completion: 87% (27/31 items)
- Deferrals: INVALID (no "Approved Deferrals" section)
- Unchecked: 4 items (performance test, 2 edge cases, documentation)

**Output:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 0.9: AC-DoD Traceability Validation (RCA-012)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Acceptance Criteria Analysis:
  • Total ACs: 7
  • Total requirements: 21

Traceability Mapping:
  • AC#1 (3 requirements) → 3 DoD items ✓
  • AC#2 (3 requirements) → 3 DoD items ✓
  • AC#3 (3 requirements) → 3 DoD items ✓
  • AC#4 (3 requirements) → 3 DoD items ✓
  • AC#5 (3 requirements) → 3 DoD items ✓
  • AC#6 (3 requirements) → 3 DoD items ✓
  • AC#7 (3 requirements) → 3 DoD items ✓

Traceability Score: 100% ✅

DoD Completion Status:
  • Total items: 31
  • Complete [x]: 27
  • Incomplete [ ]: 4
  • Completion: 87%

Deferral Documentation: INVALID (no Approved Deferrals section found)
  • Approved Deferrals section: MISSING ✗
  • Incomplete items: 4 (none documented)

Undocumented Incomplete DoD Items:
  • Testing: Performance test - 10K LOC analysis <30s
  • Testing: Edge case test - Zero-line files
  • Testing: Edge case test - Binary files (non-code)
  • Documentation: Threshold violation test - Extreme values

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
❌ QA VALIDATION FAILED - Incomplete DoD Without Approval
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DoD Completion: 87% (27/31 items)

Incomplete Items: 4 (lack user approval documentation)

ACTION REQUIRED:

Add "Approved Deferrals" section to Implementation Notes:

## Approved Deferrals

**User Approval:** 2025-01-21 {current_time} UTC
**Approval Type:** Low-Priority Enhancement Deferral

**Deferred Items:**
1. **Performance test: 10K LOC analysis <30s**
   - Reason: No large codebase available for realistic benchmarking
   - Blocker Type: Artifact (requires 10K+ LOC codebase)
   - Follow-up: Test in real project usage (Phase 4 validation)

2. **Edge case test: Zero-line files**
   - Reason: Edge case unlikely in practice
   - Blocker Type: Low-Priority (not critical path)
   - Follow-up: Implement if user reports issue

3. **Edge case test: Binary files**
   - Reason: Quality tools skip binary files automatically
   - Blocker Type: Toolchain (handled externally)
   - Follow-up: None required

4. **Threshold violation test: Extreme values**
   - Reason: Current validation covers normal ranges
   - Blocker Type: Low-Priority (edge case)
   - Follow-up: Implement if needed in production

**Total Deferred:** 4 items (13% of DoD)
**Rationale:** Core functionality complete, enhancements deferred

After adding section, re-run: /qa {STORY_ID}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
QA WORKFLOW HALTED - Add deferral documentation before proceeding
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

This is the STORY-038 pattern that RCA-012 prevents.
Quality gate bypass is no longer possible after Phase 0.9 enforcement.
```

---

### Example 4: STORY-023 (Documented Deferrals - PASS)

**Data:**
- Traceability: 100%
- DoD completion: 68% (15/22)
- Deferrals: VALID (7 items documented, user approval 2025-11-14 14:30 UTC)

**Output:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 0.9: AC-DoD Traceability Validation (RCA-012)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Acceptance Criteria Analysis:
  • Total ACs: 7
  • Total requirements: 18

Traceability Mapping:
  • AC#1 (3 requirements) → 3 DoD items ✓
  • AC#2 (2 requirements) → 2 DoD items ✓
  • AC#3 (3 requirements) → 3 DoD items ✓
  • AC#4 (2 requirements) → 2 DoD items ✓
  • AC#5 (3 requirements) → 3 DoD items ✓
  • AC#6 (2 requirements) → 2 DoD items ✓
  • AC#7 (3 requirements) → 3 DoD items ✓

Traceability Score: 100% ✅

DoD Completion Status:
  • Total items: 22
  • Complete [x]: 15
  • Incomplete [ ]: 7
  • Completion: 68%

Deferral Documentation: VALID (all 7 items user-approved)
  • Approved Deferrals section: EXISTS ✓
  • User approval timestamp: 2025-11-14 14:30 UTC ✓
  • Documented deferrals: 7/7 items (100%)
  • Follow-up story: STORY-024 (status: QA Approved, completed)

✓ PASS - Traceability validated, deferrals properly documented

Proceeding to Phase 1 (Validation Mode Selection)...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Note: This is design-phase story (implementation deferred to STORY-024).
All design requirements complete, implementation properly deferred.
```

---

## Template Usage Notes

### Populating Templates

**During Phase 0.9 execution:**

1. Execute algorithm (Steps 1-5)
2. Collect data (scores, counts, lists)
3. Select appropriate template (PASS / FAIL-traceability / FAIL-deferrals)
4. Substitute variables with actual values
5. Display populated template
6. Take action (CONTINUE or HALT)

### Customization for Story Context

**Adjust remediation hints based on:**
- Missing requirement type (functional → Implementation, performance → Quality)
- DoD item type (test → Testing section, docs → Documentation section)
- Blocker inference (if item mentions "load test" suggest Toolchain blocker)

### Terminal Formatting

**Box Drawing:** Use `━` for horizontal lines (consistent width)
**Width:** 80 columns (standard terminal width)
**Sections:** Clear visual separation with dividers
**Status Icons:** ✅ (pass), ❌ (fail), ✓ (checkmark), ✗ (cross), ⚠️ (warning)

---

**Templates Complete**

**Total Templates:** 3 (PASS, FAIL-traceability, FAIL-deferrals)
**Total Lines:** ~200 (with examples and documentation)
**Variability:** ~20 variables per template (high customization)
**Clarity:** Actionable remediation guidance (3 options each)
