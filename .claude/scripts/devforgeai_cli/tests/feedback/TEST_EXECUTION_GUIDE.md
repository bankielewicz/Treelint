# STORY-011: Configuration Management - Test Execution Guide

## Quick Start

### Run All Tests
```bash
cd /mnt/c/Projects/DevForgeAI2
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py -v
```

### Run Specific Test Categories

#### YAML Parsing Tests
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py::TestYamlParsing -v
```

#### Validation Tests
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py::TestConfigurationValidation -v
```

#### Integration Tests
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py::TestConfigurationLoading -v
```

#### Edge Cases
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py::TestEdgeCases -v
```

#### Performance Tests
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py::TestPerformance -v
```

## Advanced Usage

### Run with Coverage Report
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py \
  --cov=.claude/scripts/devforgeai_cli/feedback \
  --cov-report=html \
  --cov-report=term
```

### Run Single Test
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py::TestYamlParsing::test_valid_yaml_structure_parses_successfully -v
```

### Run with Verbose Output
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py -vv -s
```

### Run and Stop on First Failure
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py -x
```

### Run with Warnings
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py -W all
```

### Generate Test Report
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py \
  --html=test_report.html \
  --self-contained-html
```

## Installation Requirements

### Ensure pytest is installed
```bash
pip install pytest pytest-cov pytest-html pytest-watch
```

### Install Dependencies (if needed)
```bash
pip install pyyaml
```

## Understanding Test Results

### All Tests Pass (Green) ✅
```
========== 67 passed in 2.34s ===========
```
**Meaning:** Implementation correctly satisfies all requirements.

### Tests Fail Initially (Red) ❌
```
========== 67 failed in 0.45s ===========
```
**Expected:** This is TDD Red phase - no implementation yet.

### After Implementation (Green) ✅
```
========== 67 passed in 1.89s ===========
Coverage: 96% ✅
```
**Expected:** All tests pass after implementation complete.

## Test Execution Flow

### Phase 1: Red (Currently Here)
1. ✅ Tests written (this file)
2. ✅ All tests fail (no implementation)
3. Next: Implement code

### Phase 2: Green (Next Step)
1. ❌ Implement FeedbackConfiguration class
2. ❌ Implement YAML parser
3. ❌ Implement validation logic
4. ❌ Implement hot-reload
5. ✅ All tests pass

### Phase 3: Refactor (After Green)
1. ✅ Improve code quality
2. ✅ Extract common patterns
3. ✅ Optimize performance
4. ✅ Ensure tests still pass

## Test Categories and Expected Status

| Category | Count | Expected Status | Time |
|----------|-------|-----------------|------|
| YAML Parsing | 5 | ❌ FAIL | <1s |
| Validation | 10 | ❌ FAIL | <1s |
| Defaults | 5 | ❌ FAIL | <1s |
| Master Control | 3 | ❌ FAIL | <1s |
| Trigger Modes | 6 | ❌ FAIL | <1s |
| Conversation | 5 | ❌ FAIL | <1s |
| Skip Tracking | 4 | ❌ FAIL | <1s |
| Templates | 6 | ❌ FAIL | <1s |
| Hot-Reload | 4 | ❌ FAIL | <1s |
| Integration | 3 | ❌ FAIL | <1s |
| Edge Cases | 7 | ❌ FAIL | <2s |
| Performance | 4 | ❌ FAIL | <5s |
| **TOTAL** | **~67** | **❌ ALL FAIL** | **<20s** |

## Debugging Failed Tests

### Identify Which Tests Failed
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py \
  --tb=short  # Short traceback
```

### See Detailed Error
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py::TestYamlParsing::test_valid_yaml_structure_parses_successfully \
  -vv --tb=long
```

### See Test Setup and Fixtures
```bash
pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py \
  --setup-show
```

## Implementation Checklist

Use this checklist when implementing to ensure all requirements met:

- [ ] FeedbackConfiguration dataclass created
- [ ] ConversationSettings dataclass created
- [ ] SkipTrackingSettings dataclass created
- [ ] TemplateSettings dataclass created
- [ ] TriggerMode enum defined
- [ ] TemplateFormat enum defined
- [ ] TemplateTone enum defined
- [ ] YAML parser implemented
- [ ] Configuration loader implemented with defaults
- [ ] Validation logic for all 10 fields
- [ ] Master enable/disable control
- [ ] Trigger mode filtering
- [ ] Question limit tracking
- [ ] Skip tracking with atomic counter
- [ ] Hot-reload with file watcher
- [ ] Error logging
- [ ] Performance optimization (<100ms load)

## CI/CD Integration

### GitHub Actions Example
```yaml
name: Test STORY-011

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-python@v2
        with:
          python-version: '3.9'
      - run: pip install -r requirements.txt
      - run: pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py --cov --cov-report=xml
      - uses: codecov/codecov-action@v2
```

## Expected Test Behavior

### During Development

```
# Initial run (Red phase)
$ pytest test_configuration_management.py
FAILED ❌ test_valid_yaml_structure_parses_successfully - No such file
FAILED ❌ test_enabled_true_allows_feedback_collection - AttributeError
FAILED ❌ test_trigger_mode_always_triggers_unconditionally - NameError
...
========== 67 failed in 0.34s ===========

# After implementation (Green phase)
$ pytest test_configuration_management.py
PASSED ✅ test_valid_yaml_structure_parses_successfully
PASSED ✅ test_enabled_true_allows_feedback_collection
PASSED ✅ test_trigger_mode_always_triggers_unconditionally
...
========== 67 passed in 1.89s ===========

# After refactoring (Refactor phase - should still pass)
$ pytest test_configuration_management.py --cov
PASSED ✅ test_valid_yaml_structure_parses_successfully
PASSED ✅ test_enabled_true_allows_feedback_collection
PASSED ✅ test_trigger_mode_always_triggers_unconditionally
...
========== 67 passed in 1.92s ===========
Coverage: 96% ✅
```

## Troubleshooting

### Issue: "ModuleNotFoundError: No module named 'pytest'"
**Solution:** `pip install pytest`

### Issue: "ModuleNotFoundError: No module named 'yaml'"
**Solution:** `pip install pyyaml`

### Issue: Tests timeout
**Solution:** Increase timeout: `pytest --timeout=10`

### Issue: Permission errors on temp files
**Solution:** Ensure temp directory is writable: `chmod 777 /tmp`

### Issue: Cannot create temp directory
**Solution:** Check disk space: `df -h`

## Test Metrics

### Test File Statistics
- **Total Lines:** 1,338
- **Test Classes:** 12
- **Test Methods:** 67+
- **Parametrized Tests:** 5 functions with multiple parameters
- **Documentation Lines:** 200+
- **Code Lines:** 900+

### Coverage Targets
- Acceptance Criteria: 100% (9/9 ACs tested)
- Data Fields: 100% (10/10 fields tested)
- Edge Cases: 100% (7/7 cases tested)
- Business Logic: >95%

### Execution Metrics
- **Total Test Time:** <20 seconds
- **Average Test:** <150ms
- **Slowest Test:** ~5s (performance tests intentional)
- **Fastest Test:** <1ms (mocked operations)

## References

- [pytest Documentation](https://docs.pytest.org/)
- [Test Summary Report](./STORY-011-test-generation-summary.md)
- [Configuration Management Story](../../../devforgeai/specs/Stories/STORY-011-configuration-management.story.md)
- [DevForgeAI Framework](../../../CLAUDE.md)

## Contact & Support

For test-related questions:
1. Review test docstrings (explain purpose)
2. Check test comments (explain setup)
3. Review the test summary document
4. Check the story acceptance criteria

---

**Ready for Green Phase Implementation!** 🚀

All 67 tests are ready to guide implementation of STORY-011.
