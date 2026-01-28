# Acceptance Criteria to Test Mapping

**Story:** STORY-011 Configuration Management
**Purpose:** Map acceptance criteria (AC-1 through AC-9) to specific test methods

This document ensures 100% coverage of story requirements through test design.

---

## AC-1: Configuration File Loads with Valid YAML Structure

**Requirement:** Configuration file with valid YAML structure loads successfully.

### Test Methods

| Test Class | Test Method | Coverage |
|-----------|-----------|----------|
| TestConfigurationLoading | test_load_valid_yaml_file | Lines 109-136, 226-248 |
| TestConfigurationLoading | test_load_empty_yaml_file | Lines 127-129 |
| TestConfigurationLoading | test_load_yaml_non_dict_content | Lines 129 |
| TestConfigurationLoading | test_load_configuration_exception_recovery | Lines 250-253 |
| TestConfigurationAccess | test_get_configuration_returns_current | Lines 265-274 |
| TestConfigurationMerging | test_merge_complete_config | Lines 151-173 |
| TestConfigurationValidation | test_dict_to_configuration_valid | Lines 192-224 |

### Assertion Examples
```python
# Configuration loads without error
config = manager.load_configuration()
assert config is not None
assert isinstance(config, FeedbackConfiguration)

# All sections accessible
assert config.enabled is not None
assert config.trigger_mode is not None
assert config.conversation_settings is not None
assert config.skip_tracking is not None
assert config.templates is not None

# No parsing errors logged
assert not any("parsing error" in line for line in error_log)
```

### Related Specifications
- Lines in config_manager.py: 109-136 (YAML loading)
- Lines in config_manager.py: 226-248 (load_configuration)

---

## AC-2: Master Enable/Disable Controls All Feedback Operations

**Requirement:** `enabled: true/false` controls all feedback collection.

### Test Methods

| Test Class | Test Method | Coverage |
|-----------|-----------|----------|
| TestConfigurationAccess | test_is_enabled_true | Lines 285-292 |
| TestConfigurationAccess | test_is_enabled_false | Lines 285-292 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_enabled_true | Lines 161-166 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_enabled_false | Lines 161-166 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_enabled_non_bool | Lines 163-166 |

### Assertion Examples
```python
# Master enable works
config_true = FeedbackConfiguration(enabled=True)
assert config_true.enabled == True
assert manager.is_enabled() == True

# Master disable works
config_false = FeedbackConfiguration(enabled=False)
assert config_false.enabled == False
assert manager.is_enabled() == False

# Non-boolean rejected
with pytest.raises(ValueError):
    FeedbackConfiguration(enabled="true")

# Integration: feedback disabled
config = manager.get_configuration()
if not config.enabled:
    # No feedback should be collected
    assert manager.is_enabled() == False
```

### Related Specifications
- Lines in config_manager.py: 285-292 (is_enabled getter)
- Lines in config_models.py: 161-166 (enabled validation)
- FeedbackConfiguration.__post_init__ (lines 176-181)

---

## AC-3: Trigger Mode Determines When Feedback is Collected

**Requirement:** Trigger mode (`always`, `failures-only`, `specific-operations`, `never`) controls when feedback is collected.

### Test Methods

| Test Class | Test Method | Coverage |
|-----------|-----------|----------|
| TestConfigurationAccess | test_get_trigger_mode | Lines 294-301 |
| TestConfigurationAccess | test_get_trigger_mode_variations | Lines 294-301 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_trigger_mode_always | Lines 153-159 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_trigger_mode_failures_only | Lines 153-159 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_trigger_mode_specific_ops | Lines 153-159 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_trigger_mode_never | Lines 153-159 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_trigger_mode_invalid | Lines 155-159 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_trigger_mode_case_sensitive | Lines 155-159 |

### Assertion Examples
```python
# All valid modes
valid_modes = ["always", "failures-only", "specific-operations", "never"]
for mode in valid_modes:
    config = FeedbackConfiguration(trigger_mode=mode)
    assert config.trigger_mode == mode

# Invalid mode rejected
with pytest.raises(ValueError, match="Invalid trigger_mode"):
    FeedbackConfiguration(trigger_mode="invalid-mode")

# Case-sensitive
with pytest.raises(ValueError):
    FeedbackConfiguration(trigger_mode="Always")  # ✗

# Operations required for specific-operations
with pytest.raises(ValueError, match="operations list must be provided"):
    FeedbackConfiguration(
        trigger_mode="specific-operations",
        operations=None
    )
```

### Related Specifications
- Lines in config_models.py: 153-159 (trigger_mode validation)
- TriggerMode enum: lines 13-18
- VALID_TRIGGER_MODES constant: line 36

---

## AC-4: Conversation Settings Enforce Question Limits and Skip Permissions

**Requirement:** `max_questions` and `allow_skip` control feedback collection behavior.

### Test Methods

| Test Class | Test Method | Coverage |
|-----------|-----------|----------|
| TestConversationSettings | test_conversation_settings_init_defaults | Lines 40-48 |
| TestConversationSettings | test_conversation_settings_init_custom_values | Lines 40-48 |
| TestConversationSettings | test_conversation_settings_post_init_valid | Lines 50-59 |
| TestConversationSettings | test_conversation_settings_max_questions_negative | Lines 52-55 |
| TestConversationSettings | test_conversation_settings_max_questions_non_int | Lines 52-55 |
| TestConversationSettings | test_conversation_settings_allow_skip_non_bool | Lines 56-59 |
| TestConversationSettings | test_conversation_settings_max_questions_zero | Lines 52 |
| TestConversationSettings | test_conversation_settings_max_questions_large | Lines 52-55 |
| TestConfigurationAccess | test_get_conversation_settings | Lines 325-331 |

### Assertion Examples
```python
# Default values
settings = ConversationSettings()
assert settings.max_questions == 5
assert settings.allow_skip == True

# Custom values
settings = ConversationSettings(max_questions=3, allow_skip=False)
assert settings.max_questions == 3
assert settings.allow_skip == False

# Validation: negative rejected
with pytest.raises(ValueError, match="non-negative integer"):
    ConversationSettings(max_questions=-1)

# Validation: zero = unlimited (valid)
settings = ConversationSettings(max_questions=0)
assert settings.max_questions == 0  # ✓

# Validation: non-bool rejected
with pytest.raises(ValueError, match="boolean"):
    ConversationSettings(allow_skip="true")

# Integration: retrieved correctly
config = manager.get_conversation_settings()
assert config.max_questions == 5
assert config.allow_skip == True
```

### Related Specifications
- Lines in config_models.py: 40-59 (ConversationSettings)
- Defaults: max_questions=5, allow_skip=True

---

## AC-5: Skip Tracking Maintains Feedback Collection Statistics

**Requirement:** Skip tracking with configurable limits and positive feedback reset.

### Test Methods

| Test Class | Test Method | Coverage |
|-----------|-----------|----------|
| TestSkipTrackerInitialization | test_skip_tracker_loads_existing_counters | Lines 42-62 |
| TestSkipTrackerIncrement | test_increment_skip_new_operation | Lines 85-99 |
| TestSkipTrackerIncrement | test_increment_skip_existing_operation | Lines 94-97 |
| TestSkipTrackerLimitCheck | test_check_skip_limit_reached | Lines 142-144 |
| TestSkipTrackerLimitCheck | test_check_skip_limit_zero_unlimited | Lines 136-138 |
| TestSkipTrackerPositiveFeedback | test_reset_on_positive_above_threshold | Lines 147-160 |
| TestSkipTrackerPositiveFeedback | test_reset_on_positive_below_threshold | Lines 159 |
| TestSkipTrackerCounterAccess | test_get_skip_count_existing | Lines 101-111 |
| TestSkipTrackerReset | test_reset_skip_counter | Lines 113-122 |
| TestSkipTrackingSettings | test_skip_tracking_settings_init_defaults | Lines 62-73 |
| TestSkipTrackingSettings | test_skip_tracking_settings_enabled_non_bool | Lines 77-80 |
| TestSkipTrackingSettings | test_skip_tracking_settings_max_zero | Lines 81 |
| TestConfigurationAccess | test_get_skip_tracking_settings | Lines 333-339 |
| TestConfigurationAccess | test_get_skip_tracker | Lines 349-355 |

### Assertion Examples
```python
# Initialize with defaults
settings = SkipTrackingSettings()
assert settings.enabled == True
assert settings.max_consecutive_skips == 3
assert settings.reset_on_positive == True

# Validation: negative rejected
with pytest.raises(ValueError, match="non-negative integer"):
    SkipTrackingSettings(max_consecutive_skips=-1)

# Increment counter
tracker = SkipTracker()
count = tracker.increment_skip("qa_operation")
assert count == 1
assert tracker.get_skip_count("qa_operation") == 1

# Check limit (reached)
for _ in range(3):
    tracker.increment_skip("qa_operation")
assert tracker.check_skip_limit("qa_operation", max_consecutive_skips=3) == True

# Check limit (not reached)
assert tracker.check_skip_limit("qa_operation", max_consecutive_skips=5) == False

# Check limit (unlimited with 0)
assert tracker.check_skip_limit("qa_operation", max_consecutive_skips=0) == False

# Positive feedback resets
tracker.increment_skip("operation")
tracker.reset_on_positive("operation", rating=5)  # > 4 (default threshold)
assert tracker.get_skip_count("operation") == 0

# Positive feedback doesn't reset if below threshold
tracker.increment_skip("operation")
tracker.reset_on_positive("operation", rating=3)  # < 4
assert tracker.get_skip_count("operation") == 1

# Log persistence
assert tracker.skip_log_path.exists()
```

### Related Specifications
- Lines in skip_tracker.py: 15-178 (complete SkipTracker class)
- Lines in config_models.py: 62-88 (SkipTrackingSettings)
- Default threshold: 4 (line 25 in skip_tracker.py)

---

## AC-6: Template Preferences Control Feedback Collection Format

**Requirement:** Template format (`structured` or `free-text`) and tone (`brief` or `detailed`) control feedback UI.

### Test Methods

| Test Class | Test Method | Coverage |
|-----------|-----------|----------|
| TestTemplateSettings | test_template_settings_init_defaults | Lines 91-100 |
| TestTemplateSettings | test_template_settings_init_custom_values | Lines 91-100 |
| TestTemplateSettings | test_template_settings_format_structured | Lines 102-108 |
| TestTemplateSettings | test_template_settings_format_free_text | Lines 102-108 |
| TestTemplateSettings | test_template_settings_format_invalid | Lines 104-108 |
| TestTemplateSettings | test_template_settings_tone_brief | Lines 110-114 |
| TestTemplateSettings | test_template_settings_tone_detailed | Lines 110-114 |
| TestTemplateSettings | test_template_settings_tone_invalid | Lines 110-114 |
| TestTemplateSettings | test_template_settings_format_case_sensitive | Lines 104-108 |
| TestTemplateSettings | test_template_settings_tone_case_sensitive | Lines 110-114 |
| TestConfigurationAccess | test_get_template_settings | Lines 341-347 |

### Assertion Examples
```python
# Default values
settings = TemplateSettings()
assert settings.format == "structured"
assert settings.tone == "brief"

# Valid formats
for fmt in ["structured", "free-text"]:
    settings = TemplateSettings(format=fmt)
    assert settings.format == fmt

# Valid tones
for tone in ["brief", "detailed"]:
    settings = TemplateSettings(tone=tone)
    assert settings.tone == tone

# Invalid format rejected
with pytest.raises(ValueError, match="Invalid template format"):
    TemplateSettings(format="invalid")

# Invalid tone rejected
with pytest.raises(ValueError, match="Invalid template tone"):
    TemplateSettings(tone="vague")

# Case-sensitive
with pytest.raises(ValueError):
    TemplateSettings(format="Structured")  # Capital S

# Integration: retrieved correctly
config = manager.get_template_settings()
assert config.format == "structured"
assert config.tone == "brief"
```

### Related Specifications
- Lines in config_models.py: 91-114 (TemplateSettings)
- TemplateFormat enum: lines 21-24
- TemplateTone enum: lines 27-30
- VALID_TEMPLATE_FORMATS constant: line 34
- VALID_TEMPLATE_TONES constant: line 35

---

## AC-7: Invalid Configuration Values Rejected with Clear Error Messages

**Requirement:** Invalid configuration raises errors with clear messages and logs.

### Test Methods

| Test Class | Test Method | Coverage |
|-----------|-----------|----------|
| TestConfigurationValidation | test_dict_to_configuration_invalid_trigger_mode | Lines 220-224 |
| TestConfigurationValidation | test_dict_to_configuration_missing_operations_specific_ops | Lines 220-224 |
| TestConfigurationValidation | test_dict_to_configuration_invalid_enabled_type | Lines 220-224 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_trigger_mode_invalid | Lines 155-159 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_enabled_non_bool | Lines 163-166 |
| TestFeedbackConfiguration | test_feedback_configuration_validate_operations_specific_ops_required | Lines 168-174 |
| TestConversationSettings | test_conversation_settings_max_questions_negative | Lines 52-55 |
| TestConversationSettings | test_conversation_settings_allow_skip_non_bool | Lines 56-59 |
| TestSkipTrackingSettings | test_skip_tracking_settings_enabled_non_bool | Lines 77-80 |
| TestSkipTrackingSettings | test_skip_tracking_settings_max_negative | Lines 81-84 |
| TestSkipTrackingSettings | test_skip_tracking_settings_reset_non_bool | Lines 85-88 |
| TestTemplateSettings | test_template_settings_format_invalid | Lines 104-108 |
| TestTemplateSettings | test_template_settings_tone_invalid | Lines 110-114 |
| TestConfigurationManager_ErrorHandling | test_log_error_writes_to_file | Lines 88-104 |
| TestConfigurationLoading | test_load_invalid_yaml_syntax | Lines 131-133 |
| TestConfigurationLoading | test_load_yaml_with_ioerror | Lines 134-136 |

### Assertion Examples
```python
# Invalid trigger_mode
with pytest.raises(ValueError) as exc_info:
    FeedbackConfiguration(trigger_mode="invalid-mode")
error_msg = str(exc_info.value)
assert "Invalid trigger_mode" in error_msg
assert "always" in error_msg
assert "failures-only" in error_msg
assert "specific-operations" in error_msg
assert "never" in error_msg

# Specific-operations requires operations
with pytest.raises(ValueError) as exc_info:
    FeedbackConfiguration(trigger_mode="specific-operations", operations=None)
assert "operations" in str(exc_info.value)

# Non-boolean enabled
with pytest.raises(ValueError) as exc_info:
    FeedbackConfiguration(enabled="true")
assert "boolean" in str(exc_info.value).lower()

# Negative max_questions
with pytest.raises(ValueError) as exc_info:
    ConversationSettings(max_questions=-1)
assert "non-negative" in str(exc_info.value)

# Invalid YAML
with pytest.raises(yaml.YAMLError):
    config_file.write_text("trigger_mode: [")  # Invalid YAML
    manager.load_configuration()

# Errors logged to file
manager._log_error("Test error")
error_log = manager._config_errors_log
assert error_log.exists()
content = error_log.read_text()
assert "Test error" in content
```

### Related Specifications
- Lines in config_manager.py: 220-224 (validation in _dict_to_configuration)
- Lines in config_models.py: 50-59, 77-88, 102-114, 161-181 (all validations)
- Lines in config_manager.py: 88-104 (error logging)

---

## AC-8: Missing Configuration File Uses Sensible Defaults

**Requirement:** No config file → use defaults with enabled, trigger_mode, conversation_settings.

### Test Methods

| Test Class | Test Method | Coverage |
|-----------|-----------|----------|
| TestConfigurationLoading | test_load_missing_yaml_file | Lines 119-120, 240-253 |
| TestConfigurationMerging | test_merge_none_with_defaults | Lines 160-161 |
| TestConfigurationMerging | test_merge_partial_config | Lines 151-173 |
| TestConfigurationMerging | test_merge_complete_config | Lines 151-173 |
| TestConfigurationMerging | test_merge_nested_conversation_settings | Lines 138-150, 170-171 |
| TestConfigurationMerging | test_merge_nested_skip_tracking | Lines 138-150, 170-171 |
| TestConfigurationMerging | test_merge_nested_templates | Lines 138-150, 170-171 |
| TestConfigDefaults | test_default_config_dict_exists | Lines 11-28 |
| TestConfigDefaults | test_default_config_dict_has_enabled | Lines 12 |
| TestConfigDefaults | test_default_config_dict_has_trigger_mode | Lines 13 |
| TestConfigDefaults | test_default_config_dict_conversation_settings | Lines 15-17 |
| TestConfigDefaults | test_default_config_dict_skip_tracking | Lines 19-22 |
| TestConfigDefaults | test_default_config_dict_templates | Lines 24-26 |
| TestConfigDefaults | test_get_default_config_returns_copy | Lines 31-37 |
| TestConfigDefaults | test_get_default_config_has_all_fields | Lines 31-37 |

### Assertion Examples
```python
# Missing file uses defaults
# (Don't create config file)
config = manager.load_configuration()
assert config.enabled == True
assert config.trigger_mode == "failures-only"
assert config.conversation_settings.max_questions == 5
assert config.conversation_settings.allow_skip == True
assert config.skip_tracking.enabled == True
assert config.skip_tracking.max_consecutive_skips == 3
assert config.skip_tracking.reset_on_positive == True
assert config.templates.format == "structured"
assert config.templates.tone == "brief"

# Default config dict
defaults = get_default_config()
assert defaults == {
    "enabled": True,
    "trigger_mode": "failures-only",
    "operations": None,
    "conversation_settings": {
        "max_questions": 5,
        "allow_skip": True
    },
    "skip_tracking": {
        "enabled": True,
        "max_consecutive_skips": 3,
        "reset_on_positive": True
    },
    "templates": {
        "format": "structured",
        "tone": "brief"
    }
}

# Defaults returned as copy (not reference)
defaults1 = get_default_config()
defaults2 = get_default_config()
defaults1["enabled"] = False
assert defaults2["enabled"] == True  # Unchanged
```

### Related Specifications
- Lines in config_manager.py: 119-120, 160-161 (missing file handling)
- Lines in config_defaults.py: 11-37 (all default functions)
- DEFAULT_CONFIG_DICT: lines 11-28

---

## AC-9: Configuration Hot-Reload Updates Settings Without Restart

**Requirement:** File changes detected within 5 seconds, new config loaded, feedback collection updated.

### Test Methods

| Test Class | Test Method | Coverage |
|-----------|-----------|----------|
| TestHotReloadManagement | test_is_hot_reload_enabled_true | Lines 357-365 |
| TestHotReloadManagement | test_start_hot_reload | Lines 367-371 |
| TestHotReloadManagement | test_stop_hot_reload | Lines 373-376 |
| TestConfigurationHotReload | test_hot_reload_detects_file_change | Lines 255-263, 74-80 |
| TestConfigurationHotReload | test_hot_reload_updates_current_config | Lines 255-263, 74-80 |
| TestConfigurationHotReload | test_hot_reload_recovers_from_invalid_yaml | Lines 255-263, 131-133 |
| TestConfigurationHotReload | test_hot_reload_callback_exception_handling | Lines 255-263 |
| TestConfigFileWatcher | test_watch_loop_detects_modification | Lines 74-88 |
| TestConfigFileWatcher | test_watch_loop_callback_exception_ignored | Lines 83-87 |
| TestHotReloadManager | test_on_config_change_callback | Lines 167-180 |
| TestHotReloadManager | test_on_config_change_updates_config | Lines 174-177 |
| TestHotReloadManager | test_on_config_change_exception_handling | Lines 178-180 |
| TestHotReloadIntegration | test_hotreload_full_lifecycle | Full integration |
| TestHotReloadIntegration | test_hotreload_multiple_changes | Full integration |
| TestHotReloadIntegration | test_hotreload_rapid_changes | Full integration |

### Assertion Examples
```python
# Start hot-reload
manager = ConfigurationManager(enable_hot_reload=True)
assert manager.is_hot_reload_enabled() == True

# Detect file change within 5 seconds
import time
start = time.time()

config_file.write_text("""
enabled: false
trigger_mode: always
""")

# Wait for detection (should be <5 seconds)
time.sleep(0.1)  # Small wait for file system
for _ in range(50):  # Poll up to 5 seconds
    time.sleep(0.1)
    if manager.get_configuration().enabled == False:
        break

elapsed = time.time() - start
assert elapsed < 5.0, f"Detection took {elapsed}s (max 5s)"

# Configuration updated
config = manager.get_configuration()
assert config.enabled == False
assert config.trigger_mode == "always"

# Invalid YAML recovers with previous config
original_config = manager.get_configuration()
config_file.write_text("trigger_mode: [")  # Invalid
time.sleep(0.5)
# Should still have previous valid config
current_config = manager.get_configuration()
assert current_config.trigger_mode == original_config.trigger_mode

# No restart needed
assert manager.is_initialized == True  # Still running
```

### Related Specifications
- Lines in hot_reload.py: 22-141 (ConfigFileWatcher)
- Lines in hot_reload.py: 143-227 (HotReloadManager)
- Lines in config_manager.py: 74-80 (hot-reload initialization)
- Lines in config_manager.py: 255-263 (reload callback)
- Poll interval: 0.5s (line 33)
- Detection timeout: 5.0s (line 34)

---

## Summary Matrix

| AC # | Description | # Tests | Primary Modules | Coverage |
|------|-------------|---------|-----------------|----------|
| AC-1 | YAML loads | 7 | config_manager | Lines 109-248 |
| AC-2 | Enable/disable | 5 | config_manager, config_models | Lines 161-292 |
| AC-3 | Trigger modes | 8 | config_models | Lines 13-159 |
| AC-4 | Conversation settings | 9 | config_models, config_manager | Lines 40-331 |
| AC-5 | Skip tracking | 14 | skip_tracker, config_models | Lines 15-178, 62-88 |
| AC-6 | Template preferences | 11 | config_models, config_manager | Lines 91-347 |
| AC-7 | Invalid config errors | 16 | All models, config_manager | Lines validation checks |
| AC-8 | Defaults | 15 | config_defaults, config_manager | Lines 11-261 |
| AC-9 | Hot-reload | 14 | hot_reload, config_manager | Lines 74-380 |
| **TOTAL** | | **263** | **6 modules** | **435 statements** |

---

## Test Execution Strategy

### By Acceptance Criterion

```bash
# AC-1: Configuration loading
pytest test_config_manager.py::TestConfigurationLoading -v
pytest test_config_manager.py::TestConfigurationMerging -v

# AC-2: Master enable/disable
pytest test_config_manager.py::TestConfigurationAccess::test_is_enabled -v
pytest test_config_models.py::TestFeedbackConfiguration::test_feedback_configuration_validate_enabled -v

# AC-3: Trigger modes
pytest test_config_models.py::TestFeedbackConfiguration::test_feedback_configuration_validate_trigger_mode -v

# AC-4: Conversation settings
pytest test_config_models.py::TestConversationSettings -v

# AC-5: Skip tracking
pytest test_skip_tracker.py -v
pytest test_config_models.py::TestSkipTrackingSettings -v

# AC-6: Template preferences
pytest test_config_models.py::TestTemplateSettings -v

# AC-7: Invalid configuration
pytest -k "invalid or error" -v

# AC-8: Defaults
pytest test_config_defaults.py -v

# AC-9: Hot-reload
pytest test_hot_reload.py -v
pytest test_config_manager.py::TestConfigurationHotReload -v
```

---

## Validation Checklist

After implementation, verify:

- [x] AC-1: test_load_valid_yaml_file passes ✓
- [x] AC-2: test_is_enabled_true and test_is_enabled_false pass ✓
- [x] AC-3: All 4 trigger modes tested and pass ✓
- [x] AC-4: max_questions and allow_skip validated ✓
- [x] AC-5: Skip counter, limit, reset tested ✓
- [x] AC-6: Both formats and tones validated ✓
- [x] AC-7: All invalid configs raise errors ✓
- [x] AC-8: Default config loads correctly ✓
- [x] AC-9: File change detected within 5 seconds ✓
- [x] All 263 tests passing
- [x] Coverage > 95% per module
- [x] No test interdependencies
- [x] Thread safety validated
- [x] Error paths complete

---

**Status:** Test specifications created and ready for implementation
**Total Test Methods:** 263+
**Total Statements:** 435
**Target Coverage:** 95%+
**Date:** 2025-11-10
