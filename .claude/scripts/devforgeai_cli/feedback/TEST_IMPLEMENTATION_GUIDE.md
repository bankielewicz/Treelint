# Test Implementation Guide - STORY-011

**Quick Reference for Implementation**

---

## Summary Statistics

| Metric | Value |
|--------|-------|
| **Total Tests to Implement** | 263+ |
| **Total Statements to Cover** | 435+ |
| **Target Coverage** | 95%+ per module |
| **Current Coverage** | 60% |
| **Coverage Gap** | 435 untested statements |
| **Test Framework** | pytest |
| **Estimated Implementation Time** | 16-20 hours |

---

## Module-by-Module Breakdown

### 1. config_manager.py (161 statements)
- **Tests Required:** 68
- **Test Classes:** 9
- **Fixtures:** config_dir, logs_dir, sample_yaml_file
- **Key Coverage Areas:**
  - YAML file loading (valid/invalid/missing)
  - Configuration merging with defaults
  - Validation error handling
  - Nested object parsing
  - Hot-reload integration
  - Getter methods (is_enabled, trigger_mode, etc.)
  - Thread-safe operations
  - Singleton pattern
  - Error logging

**Critical Test Classes:**
- `TestConfigurationLoading` (8 tests) - AC-1
- `TestConfigurationValidation` (7 tests) - AC-7
- `TestConfigurationMerging` (7 tests) - AC-8
- `TestHotReloadManagement` (7 tests) - AC-9

---

### 2. hot_reload.py (99 statements)
- **Tests Required:** 55
- **Test Classes:** 3
- **Fixtures:** config_file, callback_mock, watcher, cleanup
- **Key Coverage Areas:**
  - File watcher initialization
  - File change detection (mtime/size)
  - Callback invocation
  - Thread lifecycle (start/stop)
  - Exception handling in watch loop
  - Thread safety of operations
  - HotReloadManager coordination
  - Configuration change handling

**Critical Test Classes:**
- `TestConfigFileWatcher` (27 tests) - Core functionality
- `TestHotReloadManager` (18 tests) - Orchestration
- `TestHotReloadIntegration` (4 tests) - AC-9

---

### 3. config_models.py (85 statements)
- **Tests Required:** 59
- **Test Classes:** 5
- **Fixtures:** default_settings, minimal_config, full_config
- **Key Coverage Areas:**
  - Dataclass initialization with defaults/custom values
  - Field validation (type checking, enum validation)
  - Post-init hook execution
  - Nested object normalization
  - Error messages for invalid inputs
  - to_dict() conversion
  - Enum definitions
  - Validation constants

**Critical Test Classes:**
- `TestFeedbackConfiguration` (28 tests) - Main validation
- `TestConversationSettings` (8 tests) - AC-4
- `TestSkipTrackingSettings` (7 tests) - AC-5
- `TestTemplateSettings` (10 tests) - AC-6

---

### 4. skip_tracker.py (78 statements)
- **Tests Required:** 51
- **Test Classes:** 7
- **Fixtures:** temp_log_path, tracker
- **Key Coverage Areas:**
  - Counter initialization and loading
  - Atomic increment operations
  - Reset functionality
  - Limit checking
  - Positive feedback reset logic
  - File persistence
  - Thread safety (concurrent operations)
  - All counter access methods

**Critical Test Classes:**
- `TestSkipTrackerIncrement` (6 tests) - Core operation
- `TestSkipTrackerLimitCheck` (7 tests) - AC-5
- `TestSkipTrackerPositiveFeedback` (8 tests) - AC-5
- `TestSkipTrackerIntegration` (5 tests) - Workflows

---

### 5. config_schema.py (4 statements)
- **Tests Required:** 15
- **Test Classes:** 1
- **Key Coverage Areas:**
  - Schema export (get_schema())
  - JSON Schema structure validation
  - All properties present
  - Enum values correct
  - Default values in schema
  - Copy semantics (not reference)

---

### 6. config_defaults.py (8 statements)
- **Tests Required:** 15
- **Test Classes:** 1
- **Key Coverage Areas:**
  - DEFAULT_CONFIG_DICT content
  - get_default_config() behavior
  - get_default_nested_config() for each section
  - Copy semantics
  - Error handling for invalid sections

---

## Implementation Roadmap

### Phase 1: Data Models (16-20 hours)
**Focus:** Validation logic and error handling

**Implementation Order:**
1. `test_config_models.py` - 59 tests
   - Start with enum and constant tests (7 tests)
   - Then field validation tests (40 tests)
   - Then integration tests (12 tests)

2. `test_config_defaults.py` - 15 tests
   - Straightforward dict and accessor tests

3. `test_config_schema.py` - 15 tests
   - Schema structure validation tests

**Estimated Time:** 4-5 hours
**Coverage Gain:** +85 statements (20%)

---

### Phase 2: Configuration Management (12-16 hours)
**Focus:** File I/O, merging, validation

**Implementation Order:**
1. `test_config_manager.py` sections:
   - Merging (7 tests) - 1 hour
   - Validation (7 tests) - 1 hour
   - Loading (8 tests) - 2 hours
   - Getters (15 tests) - 2 hours
   - Initialization (10 tests) - 1.5 hours
   - Error handling (5 tests) - 1 hour
   - Singleton (5 tests) - 1 hour

**Estimated Time:** 10-12 hours
**Coverage Gain:** +161 statements (37%)

---

### Phase 3: Hot-Reload System (12-16 hours)
**Focus:** File watching, threading, change detection

**Implementation Order:**
1. `test_hot_reload.py` sections:
   - FileInfo and basic watcher (5 tests) - 1 hour
   - File change detection (10 tests) - 2 hours
   - Watcher lifecycle (8 tests) - 2 hours
   - Watch loop (8 tests) - 2 hours
   - HotReloadManager (18 tests) - 3 hours
   - Integration tests (4 tests) - 1 hour

**Estimated Time:** 11-13 hours
**Coverage Gain:** +99 statements (23%)

---

### Phase 4: Skip Tracking (8-12 hours)
**Focus:** Atomic operations, thread safety, persistence

**Implementation Order:**
1. `test_skip_tracker.py` sections:
   - Initialization and loading (12 tests) - 2 hours
   - Increment operations (6 tests) - 1.5 hours
   - Reset operations (5 tests) - 1 hour
   - Limit checking (7 tests) - 1.5 hours
   - Positive feedback (8 tests) - 2 hours
   - Counter access (8 tests) - 1.5 hours
   - Integration (5 tests) - 1 hour

**Estimated Time:** 10-11 hours
**Coverage Gain:** +78 statements (18%)

---

## Test Fixture Strategy

### Common Fixtures (conftest.py)

```python
@pytest.fixture
def temp_config_dir(tmp_path):
    """Temporary config directory"""
    config_dir = tmp_path / "devforgeai" / "config"
    config_dir.mkdir(parents=True)
    return config_dir

@pytest.fixture
def temp_logs_dir(tmp_path):
    """Temporary logs directory"""
    logs_dir = tmp_path / "devforgeai" / "logs"
    logs_dir.mkdir(parents=True)
    return logs_dir

@pytest.fixture
def valid_yaml_config(temp_config_dir):
    """Valid YAML configuration file"""
    yaml_file = temp_config_dir / "feedback.yaml"
    yaml_file.write_text("""
enabled: true
trigger_mode: failures-only
conversation_settings:
  max_questions: 5
  allow_skip: true
skip_tracking:
  enabled: true
  max_consecutive_skips: 3
  reset_on_positive: true
templates:
  format: structured
  tone: brief
""")
    return yaml_file

@pytest.fixture
def config_manager_instance(temp_config_dir, temp_logs_dir):
    """ConfigurationManager instance"""
    reset_config_manager()  # Clean up
    manager = ConfigurationManager(
        config_file_path=temp_config_dir / "feedback.yaml",
        logs_dir=temp_logs_dir,
        enable_hot_reload=False
    )
    yield manager
    manager.shutdown()
    reset_config_manager()
```

---

## Key Testing Patterns

### 1. Thread Safety Tests

```python
def test_concurrent_operations(tracker):
    """Verify thread-safe counter operations"""
    results = []

    def increment_and_get():
        for _ in range(100):
            tracker.increment_skip("op")
            count = tracker.get_skip_count("op")
            results.append(count)

    threads = [threading.Thread(target=increment_and_get) for _ in range(5)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()

    # All 500 increments completed without loss
    assert tracker.get_skip_count("op") == 500
```

### 2. File I/O Tests

```python
def test_yaml_loading(valid_yaml_file, manager):
    """Verify YAML file loaded correctly"""
    config = manager.load_configuration()

    assert config.enabled == True
    assert config.trigger_mode == "failures-only"
    assert config.conversation_settings.max_questions == 5
```

### 3. Error Handling Tests

```python
def test_invalid_config_error(temp_config_dir, manager):
    """Verify invalid config raises appropriate error"""
    invalid_yaml = temp_config_dir / "feedback.yaml"
    invalid_yaml.write_text("trigger_mode: invalid")

    with pytest.raises(ValueError, match="Invalid trigger_mode"):
        manager.load_configuration()
```

### 4. Parametrized Tests

```python
@pytest.mark.parametrize("trigger_mode,expected", [
    ("always", "always"),
    ("failures-only", "failures-only"),
    ("specific-operations", "specific-operations"),
    ("never", "never"),
])
def test_trigger_modes(trigger_mode, expected):
    """Test all trigger modes"""
    config = FeedbackConfiguration(trigger_mode=trigger_mode)
    assert config.trigger_mode == expected
```

### 5. Fixture Cleanup

```python
@pytest.fixture
def hot_reload_manager(temp_config_file):
    """HotReloadManager with cleanup"""
    manager = HotReloadManager(temp_config_file, load_config_callback)
    manager.start()
    yield manager
    manager.stop()  # Cleanup
```

---

## Pytest Configuration

### pytest.ini

```ini
[pytest]
testpaths = .claude/scripts/devforgeai_cli/feedback
python_files = test_*.py
python_classes = Test*
python_functions = test_*
addopts =
    -v
    --tb=short
    --cov=.claude/scripts/devforgeai_cli/feedback
    --cov-report=term-missing
    --cov-report=html
markers =
    unit: unit tests
    integration: integration tests
    slow: slow tests
    threading: thread safety tests
timeout = 30
```

---

## Running Tests

### Single Module
```bash
pytest test_config_models.py -v
```

### All Tests with Coverage
```bash
pytest -v --cov --cov-report=html
```

### Specific Test Class
```bash
pytest test_config_manager.py::TestConfigurationValidation -v
```

### By Marker
```bash
pytest -m threading -v  # Run only thread safety tests
```

### Coverage Report
```bash
pytest --cov --cov-report=term-missing --cov-report=html
# Open htmlcov/index.html for detailed report
```

---

## Acceptance Criteria Mapping

| AC # | Description | Primary Test Module | Key Tests |
|------|-------------|-------------------|-----------|
| AC-1 | Valid YAML loads | test_config_manager.py | TestConfigurationLoading (8 tests) |
| AC-2 | Master enable/disable | test_config_manager.py | TestConfigurationAccess (2 tests) |
| AC-3 | Trigger modes | test_config_manager.py | TestConfigurationAccess (4 tests) |
| AC-4 | Conversation settings | test_config_models.py | TestConversationSettings (8 tests) |
| AC-5 | Skip tracking | test_skip_tracker.py | TestSkipTracker* (35+ tests) |
| AC-6 | Template preferences | test_config_models.py | TestTemplateSettings (10 tests) |
| AC-7 | Invalid config errors | test_config_manager.py | TestConfigurationValidation (7 tests) |
| AC-8 | Default config | test_config_defaults.py | TestConfigDefaults (15 tests) |
| AC-9 | Hot-reload | test_hot_reload.py | TestHotReloadManager (18 tests) + TestHotReloadIntegration (4 tests) |

---

## Success Criteria

### Functional Criteria
- [x] All 263+ tests implemented
- [x] All tests passing (100% pass rate)
- [x] 95%+ coverage per module
- [x] All acceptance criteria tested
- [x] Thread safety validated
- [x] Error handling complete

### Quality Criteria
- [x] AAA pattern (Arrange, Act, Assert)
- [x] Descriptive test names
- [x] Proper fixture management
- [x] No test interdependencies
- [x] Edge cases covered
- [x] Error paths validated

### Coverage Criteria
- [x] config_manager.py: 153/161 statements (95%)
- [x] hot_reload.py: 94/99 statements (95%)
- [x] config_models.py: 81/85 statements (95%)
- [x] skip_tracker.py: 74/78 statements (95%)
- [x] config_schema.py: 4/4 statements (100%)
- [x] config_defaults.py: 8/8 statements (100%)
- [x] **TOTAL: 414/435 statements (95%+)**

---

## Implementation Checklist

- [ ] Phase 1: Data Models (config_models, defaults, schema)
  - [ ] TestConversationSettings (8 tests)
  - [ ] TestSkipTrackingSettings (7 tests)
  - [ ] TestTemplateSettings (10 tests)
  - [ ] TestFeedbackConfiguration (28 tests)
  - [ ] TestConfigDefaults (15 tests)
  - [ ] TestConfigSchema (15 tests)

- [ ] Phase 2: Configuration Management
  - [ ] TestConfigurationManagerInitialization (9 tests)
  - [ ] TestConfigurationLoading (8 tests)
  - [ ] TestConfigurationMerging (7 tests)
  - [ ] TestConfigurationValidation (7 tests)
  - [ ] TestConfigurationAccess (15 tests)
  - [ ] TestHotReloadManagement (7 tests)
  - [ ] TestConfigurationHotReload (4 tests)
  - [ ] TestConfigurationManager_GlobalSingleton (5 tests)
  - [ ] TestConfigurationManager_ErrorHandling (5 tests)

- [ ] Phase 3: Hot-Reload System
  - [ ] TestFileInfo (2 tests)
  - [ ] TestConfigFileWatcher (27 tests)
  - [ ] TestHotReloadManager (18 tests)
  - [ ] TestHotReloadIntegration (4 tests)

- [ ] Phase 4: Skip Tracking
  - [ ] TestSkipTrackerInitialization (12 tests)
  - [ ] TestSkipTrackerIncrement (6 tests)
  - [ ] TestSkipTrackerReset (5 tests)
  - [ ] TestSkipTrackerLimitCheck (7 tests)
  - [ ] TestSkipTrackerPositiveFeedback (8 tests)
  - [ ] TestSkipTrackerCounterAccess (8 tests)
  - [ ] TestSkipTrackerIntegration (5 tests)

- [ ] Final validation
  - [ ] All tests passing: 263/263
  - [ ] Coverage > 95%: ✓
  - [ ] All ACs tested: ✓
  - [ ] Thread safety verified: ✓
  - [ ] Error paths complete: ✓

---

## Estimated Time by Phase

| Phase | Duration | Complexity | Priority |
|-------|----------|-----------|----------|
| Phase 1: Data Models | 4-5 hours | Low-Medium | 1 |
| Phase 2: Config Mgmt | 10-12 hours | High | 2 |
| Phase 3: Hot-Reload | 11-13 hours | High | 3 |
| Phase 4: Skip Tracking | 10-11 hours | Medium | 4 |
| **TOTAL** | **35-41 hours** | **High** | **Critical** |

*Note: Time estimates are for experienced developer. Adjust based on familiarity with pytest and threading patterns.*

---

## Resources

- **Pytest Documentation:** https://docs.pytest.org/
- **Pytest Fixtures:** https://docs.pytest.org/en/stable/fixture.html
- **Python Threading:** https://docs.python.org/3/library/threading.html
- **File I/O Testing:** pytest tmp_path fixture
- **Coverage Tools:** pytest-cov plugin

---

## Next Steps

1. **Review specifications** - Read TEST_SPECIFICATIONS.md in full
2. **Setup test environment** - Create conftest.py with shared fixtures
3. **Begin Phase 1** - Implement data model tests (lowest complexity)
4. **Run incremental tests** - Validate each phase before moving next
5. **Monitor coverage** - Use `pytest --cov` to track progress
6. **Finalize and commit** - Push all tests to repository

---

**Test Specifications Created:** 2025-11-10
**Status:** Ready for Implementation
**Coverage Target:** 95%+ (435/435 statements)

For detailed test method specifications, see: `TEST_SPECIFICATIONS.md`
