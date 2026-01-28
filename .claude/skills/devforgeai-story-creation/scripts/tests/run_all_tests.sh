#!/bin/bash
# Master Test Runner for Phase 2 Week 3
# Executes all validator and migration tests

set -e  # Exit on first error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║                    PHASE 2 WEEK 3 - COMPREHENSIVE TEST SUITE                ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo ""

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

#===============================================================================
# VALIDATOR TESTS (TC-V1 to TC-V12)
#===============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "VALIDATOR TESTS (TC-V1 to TC-V12)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# TC-V1: Valid v2.0 story (should PASS with 0 errors)
echo "Running TC-V1: Valid v2.0 story..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V1-valid-v2.story.md > /dev/null 2>&1; then
    echo -e "${GREEN}✅ TC-V1 PASSED${NC}"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V1 FAILED${NC}"
    ((FAILED_TESTS++))
fi

# TC-V2: Missing format_version (should FAIL with error)
echo "Running TC-V2: Missing format_version..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V2-missing-version.story.md 2>&1 | grep -q "Missing 'format_version'"; then
    echo -e "${GREEN}✅ TC-V2 PASSED${NC} (error caught correctly)"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V2 FAILED${NC}"
    ((FAILED_TESTS++))
fi

# TC-V3: Invalid component type (should FAIL with error)
echo "Running TC-V3: Invalid component type..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V3-invalid-type.story.md 2>&1 | grep -q "Unknown type"; then
    echo -e "${GREEN}✅ TC-V3 PASSED${NC} (invalid type caught)"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V3 FAILED${NC}"
    ((FAILED_TESTS++))
fi

# TC-V4: Missing required field (should FAIL with error)
echo "Running TC-V4: Missing required field..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V4-missing-field.story.md 2>&1 | grep -q "Missing required field"; then
    echo -e "${GREEN}✅ TC-V4 PASSED${NC} (missing field caught)"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V4 FAILED${NC}"
    ((FAILED_TESTS++))
fi

# TC-V5: Missing test_requirement (should WARN)
echo "Running TC-V5: Missing test_requirement..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V5-no-test-req.story.md 2>&1 | grep -q "Warning"; then
    echo -e "${GREEN}✅ TC-V5 PASSED${NC} (warning generated)"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V5 FAILED${NC}"
    ((FAILED_TESTS++))
fi

# TC-V6: Bad test format (should WARN)
echo "Running TC-V6: Bad test requirement format..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V6-bad-test-format.story.md 2>&1 | grep -q "should start with 'Test:'"; then
    echo -e "${GREEN}✅ TC-V6 PASSED${NC} (format warning generated)"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V6 FAILED${NC}"
    ((FAILED_TESTS++))
fi

# TC-V7: Duplicate IDs (should FAIL with error)
echo "Running TC-V7: Duplicate IDs..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V7-duplicate-ids.story.md 2>&1 | grep -q "Duplicate ID"; then
    echo -e "${GREEN}✅ TC-V7 PASSED${NC} (duplicate caught)"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V7 FAILED${NC}"
    ((FAILED_TESTS++))
fi

# TC-V8: All 7 component types (should PASS)
echo "Running TC-V8: All 7 component types..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V8-all-types.story.md > /dev/null 2>&1; then
    echo -e "${GREEN}✅ TC-V8 PASSED${NC}"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V8 FAILED${NC}"
    ((FAILED_TESTS++))
fi

# TC-V9: Empty components (should FAIL with error)
echo "Running TC-V9: Empty components array..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V9-empty-components.story.md 2>&1 | grep -q "No components defined"; then
    echo -e "${GREEN}✅ TC-V9 PASSED${NC} (empty array caught)"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V9 FAILED${NC}"
    ((FAILED_TESTS++))
fi

# TC-V10: Invalid YAML (should FAIL with error)
echo "Running TC-V10: Invalid YAML syntax..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V10-bad-yaml.story.md 2>&1 | grep -q "Invalid YAML"; then
    echo -e "${GREEN}✅ TC-V10 PASSED${NC} (YAML error caught)"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V10 FAILED${NC}"
    ((FAILED_TESTS++))
fi

# TC-V11: Vague NFR metric (should WARN)
echo "Running TC-V11: Vague NFR metric..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V11-vague-metric.story.md 2>&1 | grep -q "metric should be measurable"; then
    echo -e "${GREEN}✅ TC-V11 PASSED${NC} (vague metric warned)"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V11 FAILED${NC}"
    ((FAILED_TESTS++))
fi

# TC-V12: v1.0 story (should WARN, not fail)
echo "Running TC-V12: v1.0 freeform format..."
((TOTAL_TESTS++))
if python3 ../validate_tech_spec.py fixtures/TC-V12-v1-story.story.md 2>&1 | grep -q "v1.0 freeform format"; then
    echo -e "${GREEN}✅ TC-V12 PASSED${NC} (v1.0 detected)"
    ((PASSED_TESTS++))
else
    echo -e "${RED}❌ TC-V12 FAILED${NC}"
    ((FAILED_TESTS++))
fi

echo ""
echo "Validator Tests: $PASSED_TESTS/$TOTAL_TESTS passed"
echo ""

#===============================================================================
# SUMMARY
#===============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Total Tests:   $TOTAL_TESTS"
echo -e "Passed:        ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed:        ${RED}$FAILED_TESTS${NC}"
echo "Pass Rate:     $(echo "scale=1; $PASSED_TESTS * 100 / $TOTAL_TESTS" | bc)%"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}✅ ALL TESTS PASSED${NC}"
    echo ""
    echo "Week 3 Testing: COMPLETE"
    echo "Recommendation: ✅ PROCEED TO WEEK 4 (Pilot Migration)"
    exit 0
else
    echo -e "${RED}❌ SOME TESTS FAILED${NC}"
    echo ""
    echo "Action Required: Fix failing tests before proceeding to Week 4"
    exit 1
fi
