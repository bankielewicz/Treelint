# QA Report: Story {{STORY_ID}} - {{STORY_TITLE}}

**Date:** {{TIMESTAMP}}
**Status:** {{STATUS}} ✅ PASS / ❌ FAIL
**Validator:** devforgeai-qa skill
**Validation Mode:** Deep Analysis

---

## Executive Summary

**Overall Status:** {{OVERALL_STATUS}}

**Key Metrics:**
- Test Coverage: {{COVERAGE_PERCENTAGE}}%
- Critical Issues: {{CRITICAL_COUNT}}
- High Issues: {{HIGH_COUNT}}
- Medium Issues: {{MEDIUM_COUNT}}
- Low Issues: {{LOW_COUNT}}
- Quality Score: {{QUALITY_SCORE}}/100

**Decision:** {{DECISION}}
- ✅ **PASS** - Ready for release
- ❌ **FAIL** - Requires fixes before release

---

## Test Coverage Analysis

### Coverage Summary

| Layer | Coverage | Threshold | Status |
|-------|----------|-----------|--------|
| Business Logic | {{BL_COVERAGE}}% | 95% | {{BL_STATUS}} |
| Application | {{APP_COVERAGE}}% | 85% | {{APP_STATUS}} |
| Infrastructure | {{INFRA_COVERAGE}}% | 80% | {{INFRA_STATUS}} |
| **Overall** | {{OVERALL_COVERAGE}}% | 80% | {{OVERALL_STATUS}} |

### Test Distribution (Test Pyramid)

| Type | Count | Percentage | Target | Status |
|------|-------|------------|--------|--------|
| Unit | {{UNIT_COUNT}} | {{UNIT_PCT}}% | 70% | {{UNIT_STATUS}} |
| Integration | {{INT_COUNT}} | {{INT_PCT}}% | 20% | {{INT_STATUS}} |
| E2E | {{E2E_COUNT}} | {{E2E_PCT}}% | 10% | {{E2E_STATUS}} |

### Coverage Gaps

{{#COVERAGE_GAPS}}
#### {{FILE}}

**Coverage:** {{COVERAGE_PCT}}% (Below {{THRESHOLD}}%)

**Uncovered Lines:** {{UNCOVERED_LINES}}

**Uncovered Methods:**
{{#UNCOVERED_METHODS}}
- `{{METHOD_NAME}}` (lines {{LINE_RANGE}})
{{/UNCOVERED_METHODS}}

**Suggested Tests:**
{{#SUGGESTED_TESTS}}
- [ ] {{TEST_NAME}}
{{/SUGGESTED_TESTS}}

---
{{/COVERAGE_GAPS}}

### Test Quality Assessment

- **Assertions per test:** {{AVG_ASSERTIONS}} (target: 1.5-5)
- **Mocking ratio:** {{MOCK_RATIO}} mocks/test (target: < 2)
- **Tests without assertions:** {{EMPTY_TESTS_COUNT}}

{{#TEST_QUALITY_WARNINGS}}
⚠️ **Warning:** {{WARNING_MESSAGE}}
{{/TEST_QUALITY_WARNINGS}}

---

## Anti-Pattern Detection

### Violations by Severity

- **CRITICAL:** {{CRITICAL_COUNT}} 🔴
- **HIGH:** {{HIGH_COUNT}} 🟠
- **MEDIUM:** {{MEDIUM_COUNT}} 🟡
- **LOW:** {{LOW_COUNT}} ⚪

### CRITICAL Violations

{{#CRITICAL_VIOLATIONS}}
#### {{CATEGORY}} - {{FILE}}:{{LINE}}

**Issue:** {{ISSUE}}

**Vulnerable Code:**
```{{LANGUAGE}}
{{CODE_SNIPPET}}
```

**Security Impact:** {{SECURITY_IMPACT}}

**Fix Required:**
```{{LANGUAGE}}
{{FIX_CODE}}
```

**Reference:** {{REFERENCE}}

---
{{/CRITICAL_VIOLATIONS}}

### HIGH Violations

{{#HIGH_VIOLATIONS}}
#### {{CATEGORY}} - {{FILE}}:{{LINE}}

**Issue:** {{ISSUE}}

**Expected:** {{EXPECTED}}

**Actual:** {{ACTUAL}}

**Fix:** {{FIX_DESCRIPTION}}

---
{{/HIGH_VIOLATIONS}}

### MEDIUM Violations

{{#MEDIUM_VIOLATIONS}}
- {{CATEGORY}} in `{{FILE}}`: {{ISSUE}}
{{/MEDIUM_VIOLATIONS}}

### LOW Violations

{{#LOW_VIOLATIONS}}
- {{CATEGORY}} in `{{FILE}}`: {{ISSUE}}
{{/LOW_VIOLATIONS}}

---

## Spec Compliance Validation

### Acceptance Criteria

{{#ACCEPTANCE_CRITERIA}}
- [{{STATUS_ICON}}] {{CRITERION}}
  {{#IF_FAIL}}
  - **Issue:** {{ISSUE}}
  - **Missing Tests:** {{MISSING_TESTS}}
  - **Action:** {{ACTION_REQUIRED}}
  {{/IF_FAIL}}
{{/ACCEPTANCE_CRITERIA}}

**Summary:** {{AC_PASS_COUNT}}/{{AC_TOTAL_COUNT}} criteria met

### API Contract Validation

{{#API_ENDPOINTS}}
#### {{METHOD}} {{PATH}}

**Status:** {{STATUS}}

{{#IF_VIOLATIONS}}
**Request Model:**
- Expected: `{{EXPECTED_REQUEST}}`
- Actual: `{{ACTUAL_REQUEST}}`
- Issue: {{REQUEST_ISSUE}}

**Response Model:**
- Expected: `{{EXPECTED_RESPONSE}}`
- Actual: `{{ACTUAL_RESPONSE}}`
- Issue: {{RESPONSE_ISSUE}}
{{/IF_VIOLATIONS}}

**Contract Tests:** {{CONTRACT_TEST_STATUS}}

---
{{/API_ENDPOINTS}}

### Error Handling Compliance

**Standard:** {{ERROR_HANDLING_PATTERN}} (from coding-standards.md)

{{#ERROR_HANDLING_VIOLATIONS}}
- {{FILE}}: {{ISSUE}}
{{/ERROR_HANDLING_VIOLATIONS}}

### Non-Functional Requirements

{{#NFRS}}
#### {{NFR_TYPE}}: {{NFR_DESCRIPTION}}

**Status:** {{STATUS}}

{{#IF_PERFORMANCE}}
**Results:**
- Average response time: {{AVG_RESPONSE}}ms (target: < {{TARGET_RESPONSE}}ms)
- P95 response time: {{P95_RESPONSE}}ms
- Throughput: {{THROUGHPUT}} req/s
{{/IF_PERFORMANCE}}

{{#IF_SECURITY}}
**Security Scan Results:**
- Vulnerabilities found: {{VULN_COUNT}}
- Critical: {{CRITICAL_VULN}}
- High: {{HIGH_VULN}}
{{/IF_SECURITY}}

---
{{/NFRS}}

---

## Code Quality Metrics

### Cyclomatic Complexity

**Methods exceeding threshold (> 10):**
{{#COMPLEX_METHODS}}
- `{{METHOD_NAME}}` in {{FILE}}: Complexity {{COMPLEXITY}}
{{/COMPLEX_METHODS}}

**Classes exceeding threshold (> 50):**
{{#COMPLEX_CLASSES}}
- `{{CLASS_NAME}}` in {{FILE}}: Complexity {{COMPLEXITY}}
{{/COMPLEX_CLASSES}}

### Maintainability Index

**Files below threshold (< 70):**
{{#LOW_MAINTAINABILITY}}
- {{FILE}}: {{INDEX}} (factors: complexity {{COMPLEXITY}}, LOC {{LOC}})
{{/LOW_MAINTAINABILITY}}

### Code Duplication

**Duplication:** {{DUPLICATION_PCT}}% (threshold: 5%)

{{#IF_DUPLICATION_HIGH}}
**Duplicate Blocks:**
{{#DUPLICATES}}
- {{FILE1}}:{{LINE1}} ↔ {{FILE2}}:{{LINE2}} ({{LINES}} lines)
{{/DUPLICATES}}
{{/IF_DUPLICATION_HIGH}}

### Documentation Coverage

**Public API Documentation:** {{DOC_COVERAGE}}% (threshold: 80%)

{{#IF_DOC_LOW}}
**Undocumented Methods:**
{{#UNDOCUMENTED}}
- {{FILE}}: {{METHOD_NAME}}
{{/UNDOCUMENTED}}
{{/IF_DOC_LOW}}

### Dependency Analysis

**Circular Dependencies:** {{CIRCULAR_COUNT}}
{{#CIRCULAR_DEPS}}
- {{CYCLE_PATH}}
{{/CIRCULAR_DEPS}}

**High Coupling (> 10 dependencies):**
{{#HIGH_COUPLING}}
- {{FILE}}: {{DEPENDENCY_COUNT}} dependencies
{{/HIGH_COUPLING}}

---

## Recommendations

### Priority 0 - BLOCKING (Must Fix)

{{#P0_RECOMMENDATIONS}}
{{INDEX}}. **{{TITLE}}**
   - File: `{{FILE}}`
   - Issue: {{ISSUE}}
   - Fix: {{FIX}}
   - Estimate: {{ESTIMATE}}
{{/P0_RECOMMENDATIONS}}

### Priority 1 - HIGH (Should Fix Before Release)

{{#P1_RECOMMENDATIONS}}
{{INDEX}}. **{{TITLE}}**
   - Issue: {{ISSUE}}
   - Fix: {{FIX}}
{{/P1_RECOMMENDATIONS}}

### Priority 2 - MEDIUM (Technical Debt)

{{#P2_RECOMMENDATIONS}}
{{INDEX}}. **{{TITLE}}** - {{ISSUE}}
{{/P2_RECOMMENDATIONS}}

### Priority 3 - LOW (Nice to Have)

{{#P3_RECOMMENDATIONS}}
{{INDEX}}. **{{TITLE}}** - {{ISSUE}}
{{/P3_RECOMMENDATIONS}}

---

## Action Items

{{#ACTION_ITEMS}}
- [ ] **{{PRIORITY}}** {{ACTION_DESCRIPTION}}
  - File: {{FILE}}
  - Estimate: {{ESTIMATE}}
{{/ACTION_ITEMS}}

---

## Next Steps

{{#IF_PASS}}
### ✅ QA Approved - Ready for Release

**Summary:**
- All critical and high priority issues resolved
- Test coverage meets thresholds
- Spec compliance validated
- Code quality within acceptable limits

**Next Phase:**
- Story status updated to "QA Approved ✅"
- Ready for `devforgeai-release` skill
- Can proceed with deployment

**Release Checklist:**
- [ ] All tests passing
- [ ] Coverage ≥ {{COVERAGE_THRESHOLD}}%
- [ ] No critical/high violations
- [ ] Spec compliance 100%
- [ ] QA report reviewed
{{/IF_PASS}}

{{#IF_FAIL}}
### ❌ QA Failed - Fixes Required

**Blocking Issues:**
- {{CRITICAL_COUNT}} CRITICAL violations
- {{HIGH_COUNT}} HIGH violations
- Coverage {{ACTUAL_COVERAGE}}% < {{REQUIRED_COVERAGE}}%
- {{FAILED_AC_COUNT}} acceptance criteria not met

**Required Actions:**
1. Fix all CRITICAL violations (security, architecture)
2. Fix all HIGH violations (spec compliance, structure)
3. Add tests to meet coverage thresholds
4. Validate all acceptance criteria

**Next Phase:**
- Return to `devforgeai-development` skill
- Apply fixes from action items
- Re-run QA validation after fixes
- Story status: "QA Failed ❌"
{{/IF_FAIL}}

---

## Traceability Matrix

| Requirement | Tests | Implementation | Status |
|-------------|-------|----------------|--------|
{{#TRACEABILITY}}
| {{REQUIREMENT}} | {{TESTS}} | {{IMPLEMENTATION}} | {{STATUS}} |
{{/TRACEABILITY}}

---

**Generated by:** devforgeai-qa skill (Deep Validation Mode)
**Report Location:** `devforgeai/qa/reports/{{STORY_ID}}-qa-report.md`
**Story Updated:** `ai_docs/Stories/{{STORY_ID}}.story.md`
