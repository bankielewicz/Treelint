# Test Specifications for Configuration Management System

**Story:** STORY-011 Configuration Management
**Coverage Goal:** 95%+ per module (targeting 435+ untested statements)
**Framework:** pytest
**Date:** 2025-11-10

---

## Overview

This document specifies comprehensive test outlines for 6 untested modules in the feedback configuration system. Tests are organized by module with detailed specifications for test methods, fixtures, and assertions.

**Total Lines of Code (LOC) to Cover:**
- config_manager.py: 161 statements
- hot_reload.py: 99 statements
- config_models.py: 85 statements
- skip_tracker.py: 78 statements
- config_schema.py: 4 statements
- config_defaults.py: 8 statements
- **TOTAL: 435 statements**

**Target Test Coverage Distribution:**
- Unit tests: 70% (305 statements)
- Integration tests: 20% (87 statements)
- Edge cases/error paths: 10% (43 statements)

---

# 1. TEST_CONFIG_MANAGER.PY

**Target:** 150+ lines, 95% coverage of 161 statements
**Purpose:** Validate configuration loading, validation, merging, and hot-reload integration

## Test Class: TestConfigurationManagerInitialization

Tests for initialization and setup phase of ConfigurationManager.

### Fixtures Required
```
- config_dir: temp directory for test config files
- logs_dir: temp directory for logs
- sample_yaml_file: valid YAML config file
- cleanup: auto-cleanup after tests
```

### Test Methods

#### test_init_with_default_paths
- **Purpose:** Verify default paths when no args provided
- **Setup:** Create ConfigurationManager() with no arguments
- **Assertions:**
  - config_file_path == Path("devforgeai/config/feedback.yaml")
  - logs_dir == Path("devforgeai/logs")
  - _initialized == True
  - _current_config is FeedbackConfiguration instance
- **Coverage:** Lines 34-68, 49-52, 82

#### test_init_with_custom_paths
- **Purpose:** Verify custom paths are respected
- **Setup:** ConfigurationManager(config_file_path=custom_path, logs_dir=custom_logs)
- **Assertions:**
  - config_file_path == custom_path
  - logs_dir == custom_logs
  - _current_config loaded from custom location
- **Coverage:** Lines 34-52

#### test_init_creates_logging_directory
- **Purpose:** Verify logs directory created during init
- **Setup:** ConfigurationManager with non-existent logs_dir
- **Assertions:**
  - logs_dir exists after init
  - _config_errors_log exists in logs_dir
- **Coverage:** Lines 86-87

#### test_init_with_debug_flag
- **Purpose:** Verify DEBUG_FEEDBACK_CONFIG env var enables debug mode
- **Setup:** Set DEBUG_FEEDBACK_CONFIG=true, init manager
- **Assertions:**
  - _debug == True
  - Error logging includes print() output
- **Coverage:** Lines 62-103

#### test_init_loads_initial_configuration
- **Purpose:** Verify initial config loaded and validated
- **Setup:** Valid YAML file exists, init ConfigurationManager
- **Assertions:**
  - _current_config is not None
  - _current_config is FeedbackConfiguration
  - get_configuration() returns same object
- **Coverage:** Lines 68

#### test_init_creates_skip_tracker
- **Purpose:** Verify SkipTracker initialized
- **Setup:** Init ConfigurationManager
- **Assertions:**
  - _skip_tracker is not None
  - _skip_tracker is SkipTracker instance
  - skip_log_path == logs_dir / "feedback-skips.log"
- **Coverage:** Lines 71

#### test_init_starts_hot_reload_by_default
- **Purpose:** Verify hot-reload manager created and started
- **Setup:** ConfigurationManager(enable_hot_reload=True)
- **Assertions:**
  - _hot_reload_manager is not None
  - _hot_reload_manager.is_running() == True
  - ConfigFileWatcher started
- **Coverage:** Lines 74-80

#### test_init_skips_hot_reload_when_disabled
- **Purpose:** Verify hot-reload not started if disabled
- **Setup:** ConfigurationManager(enable_hot_reload=False)
- **Assertions:**
  - _hot_reload_manager is None
  - No watcher thread running
- **Coverage:** Lines 74

#### test_init_thread_safety_lock
- **Purpose:** Verify initialization lock prevents race conditions
- **Setup:** Create two threads calling ConfigurationManager()
- **Assertions:**
  - Both threads complete without race condition
  - _initialized is True in both threads
  - Single config instance (no duplicates)
- **Coverage:** Lines 60, 82

---

## Test Class: TestConfigurationLoading

Tests for YAML file loading and validation.

### Fixtures Required
```
- config_file: temp YAML file
- empty_config: empty YAML file
- invalid_yaml: malformed YAML
- manager: ConfigurationManager instance
```

### Test Methods

#### test_load_valid_yaml_file
- **Purpose:** Load valid YAML configuration file
- **Setup:** Create valid YAML with all sections
- **Assertions:**
  - load_configuration() returns FeedbackConfiguration
  - All sections populated correctly
  - No errors logged
- **Coverage:** Lines 109-136, 226-248

#### test_load_missing_yaml_file
- **Purpose:** Handle missing config file gracefully
- **Setup:** No YAML file exists
- **Assertions:**
  - load_configuration() returns default config
  - No error exception raised
  - get_default_config() values used
- **Coverage:** Lines 119-120, 240-253

#### test_load_invalid_yaml_syntax
- **Purpose:** Handle malformed YAML
- **Setup:** YAML file with syntax error
- **Assertions:**
  - yaml.YAMLError raised
  - Error logged to config-errors.log
  - Error message contains "YAML parsing error"
- **Coverage:** Lines 131-133

#### test_load_yaml_with_ioerror
- **Purpose:** Handle file read errors
- **Setup:** Config file exists but unreadable (permission denied)
- **Assertions:**
  - IOError/OSError raised
  - Error logged to config-errors.log
  - Error message contains "File read error"
- **Coverage:** Lines 134-136

#### test_load_yaml_without_pyyaml
- **Purpose:** Handle missing PyYAML gracefully
- **Setup:** Temporarily set yaml = None in module
- **Assertions:**
  - _load_yaml_file() returns None
  - Error logged: "PyYAML not available"
  - load_configuration() falls back to defaults
- **Coverage:** Lines 123-125

#### test_load_empty_yaml_file
- **Purpose:** Handle empty YAML file
- **Setup:** YAML file with no content or only whitespace
- **Assertions:**
  - Content parsed as empty dict {}
  - Merged with defaults
  - Default configuration returned
- **Coverage:** Lines 127-129

#### test_load_yaml_non_dict_content
- **Purpose:** Handle YAML that doesn't parse to dict
- **Setup:** YAML file contains array: [1, 2, 3]
- **Assertions:**
  - Content recognized as non-dict
  - Converted to empty dict
  - Defaults applied
- **Coverage:** Lines 129

#### test_load_configuration_exception_recovery
- **Purpose:** Return defaults if any exception during load
- **Setup:** Trigger exception in any load phase
- **Assertions:**
  - load_configuration() catches exception
  - Error logged to config-errors.log
  - Returns FeedbackConfiguration() (defaults)
- **Coverage:** Lines 250-253

---

## Test Class: TestConfigurationMerging

Tests for merging loaded config with defaults.

### Fixtures Required
```
- manager: ConfigurationManager instance
```

### Test Methods

#### test_merge_none_with_defaults
- **Purpose:** None config merged to defaults
- **Setup:** loaded_config = None
- **Assertions:**
  - _merge_with_defaults(None) returns default config
  - All defaults present
- **Coverage:** Lines 160-161

#### test_merge_partial_config
- **Purpose:** Partial config merged with defaults
- **Setup:** loaded_config = {"enabled": False}
- **Assertions:**
  - enabled == False
  - All other fields have defaults
- **Coverage:** Lines 151-173

#### test_merge_complete_config
- **Purpose:** Complete config overrides all defaults
- **Setup:** loaded_config with all sections defined
- **Assertions:**
  - All loaded values present
  - No defaults override custom values
- **Coverage:** Lines 151-173

#### test_merge_nested_conversation_settings
- **Purpose:** Nested section merged correctly
- **Setup:** loaded_config["conversation_settings"] = {"max_questions": 10}
- **Assertions:**
  - max_questions == 10
  - allow_skip == default (True)
  - Other defaults preserved
- **Coverage:** Lines 138-150, 170-171

#### test_merge_nested_skip_tracking
- **Purpose:** Skip tracking section merged
- **Setup:** loaded_config["skip_tracking"] = {"enabled": False}
- **Assertions:**
  - enabled == False
  - max_consecutive_skips == default
  - reset_on_positive == default
- **Coverage:** Lines 138-150, 170-171

#### test_merge_nested_templates
- **Purpose:** Template settings merged
- **Setup:** loaded_config["templates"] = {"tone": "detailed"}
- **Assertions:**
  - tone == "detailed"
  - format == default ("structured")
- **Coverage:** Lines 138-150, 170-171

#### test_merge_nested_config_non_dict
- **Purpose:** Non-dict nested section handled
- **Setup:** loaded_config["conversation_settings"] = "invalid"
- **Assertions:**
  - Section not merged (skipped)
  - Defaults used for conversation_settings
- **Coverage:** Lines 146

#### test_merge_with_extra_top_level_keys
- **Purpose:** Extra keys in loaded config merged
- **Setup:** loaded_config = {"enabled": True, "custom_key": "value"}
- **Assertions:**
  - custom_key present in merged config
  - Standard keys merged correctly
- **Coverage:** Lines 167

---

## Test Class: TestConfigurationValidation

Tests for configuration validation and data model conversion.

### Fixtures Required
```
- manager: ConfigurationManager instance
- valid_dict: valid configuration dictionary
```

### Test Methods

#### test_dict_to_configuration_valid
- **Purpose:** Convert valid dict to FeedbackConfiguration
- **Setup:** valid config dict with all sections
- **Assertions:**
  - FeedbackConfiguration created
  - All nested objects instantiated
  - Validation passes
- **Coverage:** Lines 192-224

#### test_dict_to_configuration_parses_nested
- **Purpose:** Nested sections parsed to dataclasses
- **Setup:** config_dict["conversation_settings"] = dict
- **Assertions:**
  - conversation_settings is ConversationSettings
  - skip_tracking is SkipTrackingSettings
  - templates is TemplateSettings
- **Coverage:** Lines 206-208

#### test_dict_to_configuration_invalid_trigger_mode
- **Purpose:** Invalid trigger_mode raises ValueError
- **Setup:** config_dict["trigger_mode"] = "invalid-mode"
- **Assertions:**
  - ValueError raised
  - Error logged: "Invalid trigger_mode value"
  - Error contains expected modes list
- **Coverage:** Lines 220-224

#### test_dict_to_configuration_missing_operations_specific_ops
- **Purpose:** Specific-operations mode requires operations list
- **Setup:** trigger_mode="specific-operations", operations=None
- **Assertions:**
  - ValueError raised
  - Error message references operations requirement
  - Error logged
- **Coverage:** Lines 220-224

#### test_dict_to_configuration_invalid_enabled_type
- **Purpose:** Non-boolean enabled rejected
- **Setup:** config_dict["enabled"] = "true" (string)
- **Assertions:**
  - ValueError raised
  - Error includes type information
- **Coverage:** Lines 220-224

#### test_dict_to_configuration_defaults_missing_sections
- **Purpose:** Missing sections get defaults
- **Setup:** config_dict = {"enabled": True, "trigger_mode": "always"}
- **Assertions:**
  - FeedbackConfiguration created with defaults
  - conversation_settings uses defaults
  - skip_tracking uses defaults
  - templates uses defaults
- **Coverage:** Lines 206-208

#### test_parse_nested_settings_valid
- **Purpose:** Parse nested settings section
- **Setup:** Call _parse_nested_settings("conversation_settings", dict, ConversationSettings)
- **Assertions:**
  - Returns ConversationSettings instance
  - Fields populated from dict
- **Coverage:** Lines 175-190

#### test_parse_nested_settings_missing
- **Purpose:** Missing section returns None
- **Setup:** section_name not in config_dict
- **Assertions:**
  - Returns None
- **Coverage:** Lines 175-190

#### test_parse_nested_settings_non_dict
- **Purpose:** Non-dict section returns None
- **Setup:** loaded_config["conversation_settings"] = "not a dict"
- **Assertions:**
  - Returns None
- **Coverage:** Lines 188

---

## Test Class: TestConfigurationAccess

Tests for getter methods and configuration retrieval.

### Fixtures Required
```
- manager: ConfigurationManager instance
- sample_config: FeedbackConfiguration instance
```

### Test Methods

#### test_get_configuration_returns_current
- **Purpose:** Get current configuration
- **Setup:** manager.get_configuration()
- **Assertions:**
  - Returns current FeedbackConfiguration
  - Same object as _current_config
- **Coverage:** Lines 265-274

#### test_get_configuration_thread_safe
- **Purpose:** Concurrent reads are thread-safe
- **Setup:** 5 threads calling get_configuration()
- **Assertions:**
  - All threads get valid config
  - No race conditions
- **Coverage:** Lines 271

#### test_get_configuration_initializes_if_none
- **Purpose:** Load config if _current_config is None
- **Setup:** _current_config = None, call get_configuration()
- **Assertions:**
  - Loads configuration
  - Returns non-None FeedbackConfiguration
- **Coverage:** Lines 272-274

#### test_update_configuration
- **Purpose:** Update current configuration
- **Setup:** Create new config, call update_configuration(new_config)
- **Assertions:**
  - _current_config updated
  - get_configuration() returns new config
- **Coverage:** Lines 276-283

#### test_update_configuration_thread_safe
- **Purpose:** Update is thread-safe
- **Setup:** Update config in one thread while reading in another
- **Assertions:**
  - All operations atomic
  - No partial updates visible
- **Coverage:** Lines 282

#### test_is_enabled_true
- **Purpose:** Check if feedback enabled
- **Setup:** config.enabled = True
- **Assertions:**
  - is_enabled() returns True
- **Coverage:** Lines 285-292

#### test_is_enabled_false
- **Purpose:** Check if feedback disabled
- **Setup:** config.enabled = False
- **Assertions:**
  - is_enabled() returns False
- **Coverage:** Lines 285-292

#### test_get_trigger_mode
- **Purpose:** Retrieve trigger mode
- **Setup:** config.trigger_mode = "failures-only"
- **Assertions:**
  - get_trigger_mode() returns "failures-only"
- **Coverage:** Lines 294-301

#### test_get_trigger_mode_variations
- **Purpose:** All trigger modes retrievable
- **Setup:** Test each trigger mode: always, failures-only, specific-operations, never
- **Assertions:**
  - Each mode returned correctly
- **Coverage:** Lines 294-301

#### test_get_operations_returns_list
- **Purpose:** Get specific operations list
- **Setup:** config.operations = ["qa", "deployment"]
- **Assertions:**
  - get_operations() returns ["qa", "deployment"]
- **Coverage:** Lines 303-310

#### test_get_operations_returns_none
- **Purpose:** Operations None when not specified
- **Setup:** config.operations = None
- **Assertions:**
  - get_operations() returns None
- **Coverage:** Lines 303-310

#### test_get_conversation_settings
- **Purpose:** Retrieve conversation settings
- **Setup:** config has conversation_settings
- **Assertions:**
  - get_conversation_settings() returns ConversationSettings
  - All fields present
- **Coverage:** Lines 325-331

#### test_get_skip_tracking_settings
- **Purpose:** Retrieve skip tracking settings
- **Setup:** config has skip_tracking
- **Assertions:**
  - get_skip_tracking_settings() returns SkipTrackingSettings
- **Coverage:** Lines 333-339

#### test_get_template_settings
- **Purpose:** Retrieve template settings
- **Setup:** config has templates
- **Assertions:**
  - get_template_settings() returns TemplateSettings
- **Coverage:** Lines 341-347

#### test_get_nested_config_attribute
- **Purpose:** Get nested config by attribute name
- **Setup:** Call _get_nested_config("conversation_settings")
- **Assertions:**
  - Returns ConversationSettings
- **Coverage:** Lines 312-323

#### test_get_nested_config_missing
- **Purpose:** Missing nested config returns None
- **Setup:** Call _get_nested_config("nonexistent")
- **Assertions:**
  - Returns None
- **Coverage:** Lines 323

#### test_get_skip_tracker
- **Purpose:** Get skip tracker instance
- **Setup:** manager.get_skip_tracker()
- **Assertions:**
  - Returns SkipTracker instance
  - Same instance as _skip_tracker
- **Coverage:** Lines 349-355

---

## Test Class: TestHotReloadManagement

Tests for hot-reload lifecycle management.

### Fixtures Required
```
- manager: ConfigurationManager instance
- config_file: temp config file
```

### Test Methods

#### test_is_hot_reload_enabled_true
- **Purpose:** Check hot-reload status when enabled
- **Setup:** ConfigurationManager(enable_hot_reload=True)
- **Assertions:**
  - is_hot_reload_enabled() returns True
- **Coverage:** Lines 357-365

#### test_is_hot_reload_enabled_false
- **Purpose:** Check hot-reload status when disabled
- **Setup:** ConfigurationManager(enable_hot_reload=False)
- **Assertions:**
  - is_hot_reload_enabled() returns False
- **Coverage:** Lines 357-365

#### test_start_hot_reload
- **Purpose:** Start hot-reload if not running
- **Setup:** manager with hot-reload disabled, then start
- **Assertions:**
  - start_hot_reload() starts manager
  - is_hot_reload_enabled() returns True
  - Watcher thread active
- **Coverage:** Lines 367-371

#### test_start_hot_reload_already_running
- **Purpose:** Starting already-running hot-reload is safe
- **Setup:** Hot-reload already running, call start_hot_reload()
- **Assertions:**
  - No error
  - Still running
  - No duplicate threads
- **Coverage:** Lines 369-370

#### test_stop_hot_reload
- **Purpose:** Stop hot-reload
- **Setup:** manager with hot-reload enabled, call stop
- **Assertions:**
  - stop_hot_reload() stops manager
  - is_hot_reload_enabled() returns False
  - Watcher thread terminated
- **Coverage:** Lines 373-376

#### test_stop_hot_reload_not_running
- **Purpose:** Stopping non-running hot-reload is safe
- **Setup:** hot-reload disabled, call stop_hot_reload()
- **Assertions:**
  - No error
- **Coverage:** Lines 375

#### test_shutdown
- **Purpose:** Shutdown stops hot-reload
- **Setup:** manager.shutdown()
- **Assertions:**
  - Hot-reload stopped
  - is_hot_reload_enabled() returns False
- **Coverage:** Lines 378-380

---

## Test Class: TestConfigurationHotReload

Integration tests for hot-reload functionality.

### Fixtures Required
```
- temp_config_file: temp YAML file
- manager: ConfigurationManager with hot-reload
- time_control: sleep/time control for reliable testing
```

### Test Methods

#### test_hot_reload_detects_file_change
- **Purpose:** Configuration reloaded when file changes (AC-9)
- **Setup:** Start manager, modify config file, wait for detection
- **Assertions:**
  - Change detected within 5 seconds (default timeout + poll interval)
  - _reload_config_callback() called
- **Coverage:** Lines 255-263, 74-80 (integration)

#### test_hot_reload_updates_current_config
- **Purpose:** Configuration updated in memory after file change
- **Setup:** Start with enabled:true, change file to enabled:false
- **Assertions:**
  - get_configuration().enabled changes from True to False
  - is_enabled() reflects change
- **Coverage:** Lines 255-263, 74-80 (integration)

#### test_hot_reload_recovers_from_invalid_yaml
- **Purpose:** Previous config retained on invalid file change (AC-9)
- **Setup:** Valid config → modify to invalid YAML → wait
- **Assertions:**
  - get_configuration() still returns valid config
  - Error logged to config-errors.log
  - No exception thrown
- **Coverage:** Lines 255-263, 131-133 (integration)

#### test_hot_reload_callback_exception_handling
- **Purpose:** Exception in reload callback handled
- **Setup:** Force exception in load_configuration()
- **Assertions:**
  - Exception caught
  - Previous config remains valid
  - Callback can be called again
- **Coverage:** Lines 255-263 (integration)

---

## Test Class: TestConfigurationManager_GlobalSingleton

Tests for global configuration manager singleton.

### Fixtures Required
```
- cleanup: reset_config_manager() before/after
```

### Test Methods

#### test_get_config_manager_creates_instance
- **Purpose:** First call creates global instance
- **Setup:** reset_config_manager(), call get_config_manager()
- **Assertions:**
  - Returns ConfigurationManager instance
  - Same instance on second call
- **Coverage:** Lines 388-413

#### test_get_config_manager_singleton
- **Purpose:** Multiple calls return same instance
- **Setup:** Call get_config_manager() twice
- **Assertions:**
  - Both calls return same object
  - id(first) == id(second)
- **Coverage:** Lines 405-411

#### test_get_config_manager_thread_safe
- **Purpose:** Singleton creation is thread-safe
- **Setup:** 3 threads call get_config_manager() simultaneously
- **Assertions:**
  - All threads get same instance
  - No race conditions
- **Coverage:** Lines 405-411

#### test_get_config_manager_uses_provided_paths
- **Purpose:** Paths passed to first call only
- **Setup:** First call with custom paths, second call with different paths
- **Assertions:**
  - First call paths used
  - Second call paths ignored
- **Coverage:** Lines 397-411

#### test_reset_config_manager
- **Purpose:** Reset global instance for testing
- **Setup:** Create instance, reset, create again
- **Assertions:**
  - reset_config_manager() stops hot-reload
  - New call creates fresh instance
- **Coverage:** Lines 416-423

---

## Test Class: TestConfigurationManager_ErrorHandling

Tests for error handling and logging.

### Fixtures Required
```
- logs_dir: temp directory for logs
- manager: ConfigurationManager instance
```

### Test Methods

#### test_log_error_writes_to_file
- **Purpose:** Errors logged to config-errors.log
- **Setup:** Call _log_error("Test error")
- **Assertions:**
  - File exists: logs_dir / "config-errors.log"
  - File contains error message with timestamp
- **Coverage:** Lines 88-104

#### test_log_error_handles_write_failure
- **Purpose:** Log write failure doesn't crash
- **Setup:** Make logs_dir unwritable, call _log_error()
- **Assertions:**
  - No exception raised
  - Process continues normally
- **Coverage:** Lines 98-100

#### test_log_error_debug_mode
- **Purpose:** Debug mode prints to stdout
- **Setup:** DEBUG_FEEDBACK_CONFIG=true, _log_error()
- **Assertions:**
  - stderr/stdout contains error message
  - Message prefixed with "[CONFIG]"
- **Coverage:** Lines 102-103

#### test_ensure_config_directory_creates_path
- **Purpose:** Config directory created if missing
- **Setup:** Call _ensure_config_directory() with non-existent path
- **Assertions:**
  - config_file_path.parent created
- **Coverage:** Lines 105-107

#### test_ensure_config_directory_exists
- **Purpose:** Existing directory not recreated
- **Setup:** Call _ensure_config_directory() twice
- **Assertions:**
  - No error
  - Directory exists after both calls
- **Coverage:** Lines 105-107

---

## Coverage Summary for config_manager.py

| Section | Lines | Coverage | Status |
|---------|-------|----------|--------|
| Initialization | 34-82 | ~100% | 10 tests |
| YAML Loading | 109-136 | ~100% | 8 tests |
| Merging | 138-173 | ~100% | 7 tests |
| Validation | 192-224 | ~100% | 7 tests |
| Getters | 265-347 | ~100% | 15 tests |
| Hot-Reload | 357-380 | ~100% | 7 tests |
| Callbacks | 255-263 | ~100% | 4 tests (integration) |
| Singleton | 388-423 | ~100% | 5 tests |
| Error Handling | 88-104 | ~100% | 5 tests |
| **TOTAL** | **161** | **95%+** | **68 tests** |

---

# 2. TEST_HOT_RELOAD.PY

**Target:** 90+ lines, 95% coverage of 99 statements
**Purpose:** Validate file watching, change detection, and configuration reloading

## Test Class: TestFileInfo

Tests for FileInfo named tuple.

### Test Methods

#### test_fileinfo_creation
- **Purpose:** Create FileInfo instance
- **Setup:** FileInfo(mtime=1234567890.0, size=1024)
- **Assertions:**
  - mtime == 1234567890.0
  - size == 1024
- **Coverage:** Lines 16-20

#### test_fileinfo_named_tuple_properties
- **Purpose:** Access properties by name and index
- **Setup:** Create FileInfo(100.0, 200)
- **Assertions:**
  - fi.mtime == 100.0 and fi[0] == 100.0
  - fi.size == 200 and fi[1] == 200
- **Coverage:** Lines 16-20

---

## Test Class: TestConfigFileWatcher

Tests for file watching and change detection.

### Fixtures Required
```
- config_file: temp config file
- callback_mock: mock callback function
- watcher: ConfigFileWatcher instance
- cleanup: stop watcher after tests
```

### Test Methods

#### test_watcher_init
- **Purpose:** Initialize ConfigFileWatcher
- **Setup:** ConfigFileWatcher(config_file, callback)
- **Assertions:**
  - config_file set
  - on_change_callback set
  - poll_interval == 0.5 (default)
  - detection_timeout == 5.0 (default)
  - _is_running == False
- **Coverage:** Lines 29-52

#### test_watcher_custom_poll_interval
- **Purpose:** Custom poll interval respected
- **Setup:** ConfigFileWatcher(..., poll_interval=0.1)
- **Assertions:**
  - poll_interval == 0.1
- **Coverage:** Lines 29-48

#### test_watcher_custom_detection_timeout
- **Purpose:** Custom detection timeout respected
- **Setup:** ConfigFileWatcher(..., detection_timeout=10.0)
- **Assertions:**
  - detection_timeout == 10.0
- **Coverage:** Lines 29-48

#### test_get_file_info_existing_file
- **Purpose:** Get file info for existing file
- **Setup:** config_file exists with known mtime/size
- **Assertions:**
  - FileInfo.mtime == config_file.stat().st_mtime
  - FileInfo.size == config_file.stat().st_size
- **Coverage:** Lines 55-67

#### test_get_file_info_missing_file
- **Purpose:** Get file info for non-existent file
- **Setup:** config_file doesn't exist
- **Assertions:**
  - FileInfo(None, None) returned
- **Coverage:** Lines 55-67

#### test_get_file_info_ioerror
- **Purpose:** OSError/IOError handled gracefully
- **Setup:** config_file.stat() raises OSError
- **Assertions:**
  - Returns FileInfo(None, None)
  - No exception raised
- **Coverage:** Lines 65-67

#### test_has_file_changed_first_call
- **Purpose:** First call (no baseline) returns False
- **Setup:** _last_file_info = None, _has_file_changed(new_info)
- **Assertions:**
  - Returns False
- **Coverage:** Lines 94-109

#### test_has_file_changed_mtime
- **Purpose:** Modification time change detected
- **Setup:** _last_file_info with mtime=100, current with mtime=200
- **Assertions:**
  - Returns True
- **Coverage:** Lines 106-109

#### test_has_file_changed_size
- **Purpose:** File size change detected
- **Setup:** _last_file_info with size=1000, current with size=2000
- **Assertions:**
  - Returns True
- **Coverage:** Lines 108-109

#### test_has_file_changed_no_change
- **Purpose:** No change detected correctly
- **Setup:** _last_file_info and current_info identical
- **Assertions:**
  - Returns False
- **Coverage:** Lines 94-109

#### test_has_file_changed_last_info_none
- **Purpose:** No change when baseline is None
- **Setup:** _last_file_info = None, current_info with values
- **Assertions:**
  - Returns False
- **Coverage:** Lines 103-104

#### test_has_file_changed_current_mtime_none
- **Purpose:** No change when current mtime is None
- **Setup:** _last_file_info has mtime, current_info.mtime = None
- **Assertions:**
  - Returns False
- **Coverage:** Lines 106-107

#### test_has_file_changed_both_none
- **Purpose:** No change when both mtime values None
- **Setup:** Both _last_file_info.mtime and current_info.mtime = None
- **Assertions:**
  - Returns False
- **Coverage:** Lines 106

#### test_start_watching
- **Purpose:** Start file watcher
- **Setup:** watcher.start()
- **Assertions:**
  - _is_running == True
  - _watch_thread is not None
  - _watch_thread.is_alive() == True
  - _stop_event cleared
- **Coverage:** Lines 111-120

#### test_start_watching_already_running
- **Purpose:** Starting already-running watcher is safe
- **Setup:** watcher.start(), then start again
- **Assertions:**
  - No error
  - Still running
  - No duplicate threads (thread count == 1)
- **Coverage:** Lines 113-115

#### test_stop_watching
- **Purpose:** Stop file watcher
- **Setup:** watcher.start(), then watcher.stop()
- **Assertions:**
  - _is_running == False
  - _stop_event is set
  - _watch_thread no longer alive (or None)
- **Coverage:** Lines 122-131

#### test_stop_watching_not_running
- **Purpose:** Stopping non-running watcher is safe
- **Setup:** watcher.stop() without start
- **Assertions:**
  - No error
  - _is_running == False
- **Coverage:** Lines 124-126

#### test_is_running_true
- **Purpose:** Check running status when active
- **Setup:** watcher.start(), check is_running()
- **Assertions:**
  - is_running() returns True
- **Coverage:** Lines 133-140

#### test_is_running_false
- **Purpose:** Check running status when stopped
- **Setup:** watcher.stop(), check is_running()
- **Assertions:**
  - is_running() returns False
- **Coverage:** Lines 133-140

#### test_watch_loop_initializes_baseline
- **Purpose:** Watch loop sets initial baseline
- **Setup:** Start watcher, observe _last_file_info
- **Assertions:**
  - _last_file_info initialized in _watch_loop
- **Coverage:** Lines 69-72

#### test_watch_loop_detects_modification
- **Purpose:** File modification triggers callback
- **Setup:** Start watcher, modify config file
- **Assertions:**
  - Callback called within poll_interval + tolerance
  - Callback called with config_file path
- **Coverage:** Lines 74-88

#### test_watch_loop_callback_exception_ignored
- **Purpose:** Exception in callback doesn't crash watcher
- **Setup:** Callback raises exception
- **Assertions:**
  - Watcher continues running
  - Still detects further changes
- **Coverage:** Lines 83-87

#### test_watch_loop_exception_ignored
- **Purpose:** Exception in watch loop handled
- **Setup:** Force exception in _get_file_info
- **Assertions:**
  - Watcher continues running
  - No thread crash
- **Coverage:** Lines 90-92

#### test_watch_loop_thread_timing
- **Purpose:** Watch loop respects poll_interval
- **Setup:** Set poll_interval=0.1, modify file, measure detection time
- **Assertions:**
  - Detection time ~poll_interval (within tolerance)
  - Not polling more frequently than interval
- **Coverage:** Lines 76 (integration timing test)

#### test_watch_loop_respects_stop_event
- **Purpose:** Watch loop exits on stop signal
- **Setup:** Start watcher, stop(), wait for thread
- **Assertions:**
  - Thread joins within timeout
  - Loop exits cleanly
- **Coverage:** Lines 74, 128-130

#### test_thread_safety_concurrent_start_stop
- **Purpose:** Concurrent start/stop operations are safe
- **Setup:** 3 threads: 1 start, 1 stop, 1 check status
- **Assertions:**
  - All operations complete without race
  - Final state correct
- **Coverage:** Lines 113, 124 (locking)

#### test_thread_safety_concurrent_is_running
- **Purpose:** is_running() thread-safe during changes
- **Setup:** One thread start/stop, others query status
- **Assertions:**
  - All queries consistent
  - No race conditions
- **Coverage:** Lines 139 (locking)

---

## Test Class: TestHotReloadManager

Tests for hot-reload coordination.

### Fixtures Required
```
- config_file: temp config file
- load_config_callback: mock function returning config
- manager: HotReloadManager instance
```

### Test Methods

#### test_hotreload_manager_init
- **Purpose:** Initialize HotReloadManager
- **Setup:** HotReloadManager(config_file, callback)
- **Assertions:**
  - config_file set
  - load_config_callback set
  - _watcher is None (not started)
  - _current_config is None
- **Coverage:** Lines 149-165

#### test_start_creates_watcher
- **Purpose:** Start creates ConfigFileWatcher
- **Setup:** manager.start()
- **Assertions:**
  - _watcher is not None
  - _watcher is ConfigFileWatcher
  - _watcher.is_running() == True
- **Coverage:** Lines 182-192

#### test_start_already_running
- **Purpose:** Starting already-running manager is safe
- **Setup:** manager.start(), then start again
- **Assertions:**
  - No error
  - Same _watcher instance
  - Still running
- **Coverage:** Lines 184-186

#### test_on_config_change_callback
- **Purpose:** File change triggers load_config_callback
- **Setup:** Modify file while manager running
- **Assertions:**
  - load_config_callback() called
  - _current_config updated
- **Coverage:** Lines 167-180

#### test_on_config_change_updates_config
- **Purpose:** Loaded config stored in _current_config
- **Setup:** Mock load_config_callback to return new config
- **Assertions:**
  - _current_config updated to new config
  - get_current_config() returns updated config
- **Coverage:** Lines 174-177

#### test_on_config_change_exception_handling
- **Purpose:** Exception in callback keeps previous config
- **Setup:** load_config_callback raises exception
- **Assertions:**
  - Exception caught
  - _current_config unchanged
  - Manager still running
- **Coverage:** Lines 178-180

#### test_on_config_change_thread_safe
- **Purpose:** Config change is atomic
- **Setup:** Concurrent read while config_change modifies
- **Assertions:**
  - No torn reads (partial config)
  - All readers see consistent state
- **Coverage:** Lines 174 (locking)

#### test_stop_stops_watcher
- **Purpose:** Stop stops the watcher
- **Setup:** manager.start(), then manager.stop()
- **Assertions:**
  - _watcher.is_running() == False
  - _watcher is None
- **Coverage:** Lines 194-199

#### test_stop_not_running
- **Purpose:** Stopping non-running manager is safe
- **Setup:** manager.stop() without start
- **Assertions:**
  - No error
  - _watcher is None
- **Coverage:** Lines 196

#### test_is_running_true
- **Purpose:** Check running status when active
- **Setup:** manager.start(), check is_running()
- **Assertions:**
  - is_running() returns True
- **Coverage:** Lines 201-208

#### test_is_running_false
- **Purpose:** Check running status when stopped
- **Setup:** manager.stop(), check is_running()
- **Assertions:**
  - is_running() returns False
- **Coverage:** Lines 201-208

#### test_is_running_not_started
- **Purpose:** is_running() returns False before start
- **Setup:** manager.is_running() without start
- **Assertions:**
  - Returns False
- **Coverage:** Lines 206-207

#### test_get_current_config_returns_loaded
- **Purpose:** Get current configuration
- **Setup:** Set _current_config, call get_current_config()
- **Assertions:**
  - Returns _current_config
- **Coverage:** Lines 210-217

#### test_get_current_config_none
- **Purpose:** Get current config when none loaded
- **Setup:** _current_config = None
- **Assertions:**
  - Returns None
- **Coverage:** Lines 210-217

#### test_get_current_config_thread_safe
- **Purpose:** Reading config is thread-safe
- **Setup:** Concurrent reads/writes
- **Assertions:**
  - All reads return consistent config
- **Coverage:** Lines 216 (locking)

#### test_set_current_config
- **Purpose:** Set current configuration
- **Setup:** call set_current_config(new_config)
- **Assertions:**
  - _current_config updated
  - get_current_config() returns new config
- **Coverage:** Lines 219-226

#### test_set_current_config_thread_safe
- **Purpose:** Setting config is thread-safe
- **Setup:** Concurrent set/get operations
- **Assertions:**
  - All operations atomic
  - No torn writes
- **Coverage:** Lines 225 (locking)

---

## Test Class: TestHotReloadIntegration

Integration tests for hot-reload system.

### Fixtures Required
```
- temp_config_file: temp YAML file
- cleanup: stop manager after tests
```

### Test Methods

#### test_hotreload_full_lifecycle
- **Purpose:** Complete hot-reload workflow
- **Setup:** Start manager, modify file, wait, stop
- **Assertions:**
  - Watcher detects change
  - Callback called
  - Config updated
  - Manager stops cleanly
- **Coverage:** Lines full integration

#### test_hotreload_multiple_changes
- **Purpose:** Multiple file changes detected
- **Setup:** Start manager, modify file 3 times
- **Assertions:**
  - Each change detected
  - Config updated each time
- **Coverage:** Lines 74-88 (repeated)

#### test_hotreload_rapid_changes
- **Purpose:** Rapid changes handled correctly
- **Setup:** Modify file 10 times in 1 second
- **Assertions:**
  - All changes eventually detected
  - No missed changes
  - No crashes
- **Coverage:** Lines full integration

#### test_hotreload_callback_timing
- **Purpose:** Callback timing within tolerance
- **Setup:** Time callback execution
- **Assertions:**
  - Callback completes <100ms typically
  - No excessive delays
- **Coverage:** Lines 167-180 (performance)

---

## Coverage Summary for hot_reload.py

| Section | Lines | Coverage | Status |
|---------|-------|----------|--------|
| FileInfo | 16-20 | 100% | 2 tests |
| Watcher Init | 29-52 | 100% | 3 tests |
| File Info Methods | 55-67 | 100% | 3 tests |
| Change Detection | 94-109 | 100% | 7 tests |
| Start/Stop | 111-131 | 100% | 8 tests |
| Is Running | 133-140 | 100% | 3 tests |
| Watch Loop | 69-92 | 100% | 8 tests |
| Manager Init | 149-165 | 100% | 1 test |
| Manager Start/Stop | 182-199 | 100% | 6 tests |
| Manager Config | 210-226 | 100% | 6 tests |
| Config Change | 167-180 | 100% | 4 tests |
| Integration | Full | 100% | 4 tests |
| **TOTAL** | **99** | **95%+** | **55 tests** |

---

# 3. TEST_CONFIG_MODELS.PY

**Target:** 80+ lines, 95% coverage of 85 statements
**Purpose:** Validate data model initialization, validation, and error handling

## Test Class: TestConversationSettings

Tests for ConversationSettings dataclass.

### Fixtures Required
```
- default_settings: ConversationSettings()
```

### Test Methods

#### test_conversation_settings_init_defaults
- **Purpose:** Initialize with defaults
- **Setup:** ConversationSettings()
- **Assertions:**
  - max_questions == 5
  - allow_skip == True
- **Coverage:** Lines 40-48

#### test_conversation_settings_init_custom_values
- **Purpose:** Initialize with custom values
- **Setup:** ConversationSettings(max_questions=10, allow_skip=False)
- **Assertions:**
  - max_questions == 10
  - allow_skip == False
- **Coverage:** Lines 40-48

#### test_conversation_settings_post_init_valid
- **Purpose:** Validation passes with valid values
- **Setup:** ConversationSettings(max_questions=0, allow_skip=True)
- **Assertions:**
  - No exception raised
  - Object created successfully
- **Coverage:** Lines 50-59

#### test_conversation_settings_max_questions_negative
- **Purpose:** Negative max_questions rejected
- **Setup:** ConversationSettings(max_questions=-1)
- **Assertions:**
  - ValueError raised
  - Message contains "non-negative integer"
- **Coverage:** Lines 52-55

#### test_conversation_settings_max_questions_non_int
- **Purpose:** Non-integer max_questions rejected
- **Setup:** ConversationSettings(max_questions="5")
- **Assertions:**
  - ValueError raised
  - Message contains type information
- **Coverage:** Lines 52-55

#### test_conversation_settings_allow_skip_non_bool
- **Purpose:** Non-boolean allow_skip rejected
- **Setup:** ConversationSettings(allow_skip="true")
- **Assertions:**
  - ValueError raised
  - Message contains boolean requirement
- **Coverage:** Lines 56-59

#### test_conversation_settings_max_questions_zero
- **Purpose:** max_questions=0 is valid (unlimited)
- **Setup:** ConversationSettings(max_questions=0)
- **Assertions:**
  - No error
  - max_questions == 0
- **Coverage:** Lines 52

#### test_conversation_settings_max_questions_large
- **Purpose:** Large max_questions values accepted
- **Setup:** ConversationSettings(max_questions=9999)
- **Assertions:**
  - No error
  - max_questions == 9999
- **Coverage:** Lines 52-55

---

## Test Class: TestSkipTrackingSettings

Tests for SkipTrackingSettings dataclass.

### Fixtures Required
```
- default_settings: SkipTrackingSettings()
```

### Test Methods

#### test_skip_tracking_settings_init_defaults
- **Purpose:** Initialize with defaults
- **Setup:** SkipTrackingSettings()
- **Assertions:**
  - enabled == True
  - max_consecutive_skips == 3
  - reset_on_positive == True
- **Coverage:** Lines 62-73

#### test_skip_tracking_settings_init_custom_values
- **Purpose:** Initialize with custom values
- **Setup:** SkipTrackingSettings(enabled=False, max_consecutive_skips=10, reset_on_positive=False)
- **Assertions:**
  - Values set correctly
- **Coverage:** Lines 62-73

#### test_skip_tracking_settings_enabled_non_bool
- **Purpose:** Non-boolean enabled rejected
- **Setup:** SkipTrackingSettings(enabled="true")
- **Assertions:**
  - ValueError raised
- **Coverage:** Lines 77-80

#### test_skip_tracking_settings_max_negative
- **Purpose:** Negative max_consecutive_skips rejected
- **Setup:** SkipTrackingSettings(max_consecutive_skips=-1)
- **Assertions:**
  - ValueError raised
- **Coverage:** Lines 81-84

#### test_skip_tracking_settings_max_non_int
- **Purpose:** Non-integer max_consecutive_skips rejected
- **Setup:** SkipTrackingSettings(max_consecutive_skips="3")
- **Assertions:**
  - ValueError raised
- **Coverage:** Lines 81-84

#### test_skip_tracking_settings_reset_non_bool
- **Purpose:** Non-boolean reset_on_positive rejected
- **Setup:** SkipTrackingSettings(reset_on_positive="true")
- **Assertions:**
  - ValueError raised
- **Coverage:** Lines 85-88

#### test_skip_tracking_settings_max_zero
- **Purpose:** max_consecutive_skips=0 is valid (unlimited)
- **Setup:** SkipTrackingSettings(max_consecutive_skips=0)
- **Assertions:**
  - No error
- **Coverage:** Lines 81

---

## Test Class: TestTemplateSettings

Tests for TemplateSettings dataclass.

### Fixtures Required
```
- default_settings: TemplateSettings()
```

### Test Methods

#### test_template_settings_init_defaults
- **Purpose:** Initialize with defaults
- **Setup:** TemplateSettings()
- **Assertions:**
  - format == "structured"
  - tone == "brief"
- **Coverage:** Lines 91-100

#### test_template_settings_init_custom_values
- **Purpose:** Initialize with custom values
- **Setup:** TemplateSettings(format="free-text", tone="detailed")
- **Assertions:**
  - format == "free-text"
  - tone == "detailed"
- **Coverage:** Lines 91-100

#### test_template_settings_format_structured
- **Purpose:** Structured format valid
- **Setup:** TemplateSettings(format="structured")
- **Assertions:**
  - No error
- **Coverage:** Lines 102-108

#### test_template_settings_format_free_text
- **Purpose:** Free-text format valid
- **Setup:** TemplateSettings(format="free-text")
- **Assertions:**
  - No error
- **Coverage:** Lines 102-108

#### test_template_settings_format_invalid
- **Purpose:** Invalid format rejected
- **Setup:** TemplateSettings(format="invalid-format")
- **Assertions:**
  - ValueError raised
  - Message lists valid options
- **Coverage:** Lines 104-108

#### test_template_settings_tone_brief
- **Purpose:** Brief tone valid
- **Setup:** TemplateSettings(tone="brief")
- **Assertions:**
  - No error
- **Coverage:** Lines 110-114

#### test_template_settings_tone_detailed
- **Purpose:** Detailed tone valid
- **Setup:** TemplateSettings(tone="detailed")
- **Assertions:**
  - No error
- **Coverage:** Lines 110-114

#### test_template_settings_tone_invalid
- **Purpose:** Invalid tone rejected
- **Setup:** TemplateSettings(tone="invalid-tone")
- **Assertions:**
  - ValueError raised
  - Message lists valid options
- **Coverage:** Lines 110-114

#### test_template_settings_format_case_sensitive
- **Purpose:** Format values are case-sensitive
- **Setup:** TemplateSettings(format="Structured")
- **Assertions:**
  - ValueError raised
- **Coverage:** Lines 104-108

#### test_template_settings_tone_case_sensitive
- **Purpose:** Tone values are case-sensitive
- **Setup:** TemplateSettings(tone="Brief")
- **Assertions:**
  - ValueError raised
- **Coverage:** Lines 110-114

---

## Test Class: TestFeedbackConfiguration

Tests for main FeedbackConfiguration dataclass.

### Fixtures Required
```
- minimal_config: FeedbackConfiguration()
- full_config: Complete FeedbackConfiguration with all sections
```

### Test Methods

#### test_feedback_configuration_init_defaults
- **Purpose:** Initialize with defaults
- **Setup:** FeedbackConfiguration()
- **Assertions:**
  - enabled == True
  - trigger_mode == "failures-only"
  - operations == None
  - All nested objects initialized with defaults
- **Coverage:** Lines 117-151

#### test_feedback_configuration_init_custom_values
- **Purpose:** Initialize with custom values
- **Setup:** FeedbackConfiguration(enabled=False, trigger_mode="always")
- **Assertions:**
  - Values set correctly
- **Coverage:** Lines 117-151

#### test_feedback_configuration_normalize_nested_dict_conversation
- **Purpose:** Dict conversation_settings converted to object
- **Setup:** FeedbackConfiguration(conversation_settings={"max_questions": 10})
- **Assertions:**
  - conversation_settings is ConversationSettings
  - max_questions == 10
- **Coverage:** Lines 136-141

#### test_feedback_configuration_normalize_nested_dict_skip_tracking
- **Purpose:** Dict skip_tracking converted to object
- **Setup:** FeedbackConfiguration(skip_tracking={"enabled": False})
- **Assertions:**
  - skip_tracking is SkipTrackingSettings
  - enabled == False
- **Coverage:** Lines 143-146

#### test_feedback_configuration_normalize_nested_dict_templates
- **Purpose:** Dict templates converted to object
- **Setup:** FeedbackConfiguration(templates={"tone": "detailed"})
- **Assertions:**
  - templates is TemplateSettings
  - tone == "detailed"
- **Coverage:** Lines 148-151

#### test_feedback_configuration_normalize_nested_none_conversation
- **Purpose:** None conversation_settings gets default
- **Setup:** FeedbackConfiguration(conversation_settings=None)
- **Assertions:**
  - conversation_settings is ConversationSettings
  - Has default values
- **Coverage:** Lines 140-141

#### test_feedback_configuration_normalize_nested_none_skip_tracking
- **Purpose:** None skip_tracking gets default
- **Setup:** FeedbackConfiguration(skip_tracking=None)
- **Assertions:**
  - skip_tracking is SkipTrackingSettings
  - Has default values
- **Coverage:** Lines 145-146

#### test_feedback_configuration_normalize_nested_none_templates
- **Purpose:** None templates gets default
- **Setup:** FeedbackConfiguration(templates=None)
- **Assertions:**
  - templates is TemplateSettings
  - Has default values
- **Coverage:** Lines 151

#### test_feedback_configuration_validate_trigger_mode_always
- **Purpose:** Trigger mode "always" valid
- **Setup:** FeedbackConfiguration(trigger_mode="always")
- **Assertions:**
  - No error
- **Coverage:** Lines 153-159

#### test_feedback_configuration_validate_trigger_mode_failures_only
- **Purpose:** Trigger mode "failures-only" valid
- **Setup:** FeedbackConfiguration(trigger_mode="failures-only")
- **Assertions:**
  - No error
- **Coverage:** Lines 153-159

#### test_feedback_configuration_validate_trigger_mode_specific_ops
- **Purpose:** Trigger mode "specific-operations" valid
- **Setup:** FeedbackConfiguration(trigger_mode="specific-operations", operations=["qa"])
- **Assertions:**
  - No error
- **Coverage:** Lines 153-159

#### test_feedback_configuration_validate_trigger_mode_never
- **Purpose:** Trigger mode "never" valid
- **Setup:** FeedbackConfiguration(trigger_mode="never")
- **Assertions:**
  - No error
- **Coverage:** Lines 153-159

#### test_feedback_configuration_validate_trigger_mode_invalid
- **Purpose:** Invalid trigger_mode rejected
- **Setup:** FeedbackConfiguration(trigger_mode="invalid-mode")
- **Assertions:**
  - ValueError raised
  - Message lists valid modes
- **Coverage:** Lines 155-159

#### test_feedback_configuration_validate_trigger_mode_case_sensitive
- **Purpose:** Trigger modes are case-sensitive
- **Setup:** FeedbackConfiguration(trigger_mode="Always")
- **Assertions:**
  - ValueError raised
- **Coverage:** Lines 155-159

#### test_feedback_configuration_validate_enabled_true
- **Purpose:** enabled=True valid
- **Setup:** FeedbackConfiguration(enabled=True)
- **Assertions:**
  - No error
- **Coverage:** Lines 161-166

#### test_feedback_configuration_validate_enabled_false
- **Purpose:** enabled=False valid
- **Setup:** FeedbackConfiguration(enabled=False)
- **Assertions:**
  - No error
- **Coverage:** Lines 161-166

#### test_feedback_configuration_validate_enabled_non_bool
- **Purpose:** Non-boolean enabled rejected
- **Setup:** FeedbackConfiguration(enabled="true")
- **Assertions:**
  - ValueError raised
- **Coverage:** Lines 163-166

#### test_feedback_configuration_validate_operations_specific_ops_required
- **Purpose:** specific-operations mode requires operations
- **Setup:** FeedbackConfiguration(trigger_mode="specific-operations", operations=None)
- **Assertions:**
  - ValueError raised
  - Message mentions operations requirement
- **Coverage:** Lines 168-174

#### test_feedback_configuration_validate_operations_specific_ops_empty
- **Purpose:** specific-operations mode requires non-empty operations
- **Setup:** FeedbackConfiguration(trigger_mode="specific-operations", operations=[])
- **Assertions:**
  - ValueError raised
- **Coverage:** Lines 171-174

#### test_feedback_configuration_validate_operations_specific_ops_non_list
- **Purpose:** specific-operations mode requires list operations
- **Setup:** FeedbackConfiguration(trigger_mode="specific-operations", operations="qa")
- **Assertions:**
  - ValueError raised
- **Coverage:** Lines 171-174

#### test_feedback_configuration_validate_operations_other_modes
- **Purpose:** Other modes don't require operations
- **Setup:** FeedbackConfiguration(trigger_mode="always", operations=None)
- **Assertions:**
  - No error
- **Coverage:** Lines 170

#### test_feedback_configuration_validate_operations_list_with_values
- **Purpose:** Valid operations list
- **Setup:** FeedbackConfiguration(trigger_mode="specific-operations", operations=["qa", "dev"])
- **Assertions:**
  - No error
  - operations == ["qa", "dev"]
- **Coverage:** Lines 171-174

#### test_feedback_configuration_to_dict
- **Purpose:** Convert configuration to dictionary
- **Setup:** config.to_dict()
- **Assertions:**
  - Returns dict
  - All fields present
  - Nested objects converted to dicts
- **Coverage:** Lines 183-192

#### test_feedback_configuration_to_dict_contains_enabled
- **Purpose:** to_dict includes enabled
- **Setup:** config.to_dict()
- **Assertions:**
  - dict["enabled"] == config.enabled
- **Coverage:** Lines 186

#### test_feedback_configuration_to_dict_contains_trigger_mode
- **Purpose:** to_dict includes trigger_mode
- **Setup:** config.to_dict()
- **Assertions:**
  - dict["trigger_mode"] == config.trigger_mode
- **Coverage:** Lines 187

#### test_feedback_configuration_to_dict_contains_operations
- **Purpose:** to_dict includes operations
- **Setup:** config.to_dict()
- **Assertions:**
  - dict["operations"] == config.operations
- **Coverage:** Lines 188

#### test_feedback_configuration_to_dict_contains_nested
- **Purpose:** to_dict includes nested objects as dicts
- **Setup:** config.to_dict()
- **Assertions:**
  - dict["conversation_settings"] is dict (not object)
  - dict["skip_tracking"] is dict
  - dict["templates"] is dict
- **Coverage:** Lines 189-191

#### test_feedback_configuration_post_init_order
- **Purpose:** Post-init runs in correct order
- **Setup:** Create config with invalid nested object first
- **Assertions:**
  - Normalization happens before validation
  - Validation catches normalized values
- **Coverage:** Lines 176-181

#### test_feedback_configuration_full_integration
- **Purpose:** Complete valid configuration
- **Setup:** All fields set with valid values
- **Assertions:**
  - No error
  - All nested objects present
  - Validation passes
- **Coverage:** Lines 176-181 (integration)

---

## Test Class: TestEnumClasses

Tests for enum definitions.

### Test Methods

#### test_trigger_mode_enum_values
- **Purpose:** TriggerMode enum has correct values
- **Setup:** Access TriggerMode enum members
- **Assertions:**
  - TriggerMode.ALWAYS.value == "always"
  - TriggerMode.FAILURES_ONLY.value == "failures-only"
  - TriggerMode.SPECIFIC_OPS.value == "specific-operations"
  - TriggerMode.NEVER.value == "never"
- **Coverage:** Lines 13-18

#### test_template_format_enum_values
- **Purpose:** TemplateFormat enum has correct values
- **Setup:** Access TemplateFormat enum members
- **Assertions:**
  - TemplateFormat.STRUCTURED.value == "structured"
  - TemplateFormat.FREE_TEXT.value == "free-text"
- **Coverage:** Lines 21-24

#### test_template_tone_enum_values
- **Purpose:** TemplateTone enum has correct values
- **Setup:** Access TemplateTone enum members
- **Assertions:**
  - TemplateTone.BRIEF.value == "brief"
  - TemplateTone.DETAILED.value == "detailed"
- **Coverage:** Lines 27-30

---

## Test Class: TestValidationConstants

Tests for validation constant definitions.

### Test Methods

#### test_valid_template_formats_contains_all
- **Purpose:** VALID_TEMPLATE_FORMATS has all formats
- **Setup:** Check VALID_TEMPLATE_FORMATS set
- **Assertions:**
  - "structured" in set
  - "free-text" in set
  - Length == 2
- **Coverage:** Lines 34

#### test_valid_template_tones_contains_all
- **Purpose:** VALID_TEMPLATE_TONES has all tones
- **Setup:** Check VALID_TEMPLATE_TONES set
- **Assertions:**
  - "brief" in set
  - "detailed" in set
  - Length == 2
- **Coverage:** Lines 35

#### test_valid_trigger_modes_contains_all
- **Purpose:** VALID_TRIGGER_MODES has all modes
- **Setup:** Check VALID_TRIGGER_MODES set
- **Assertions:**
  - "always" in set
  - "failures-only" in set
  - "specific-operations" in set
  - "never" in set
  - Length == 4
- **Coverage:** Lines 36

---

## Coverage Summary for config_models.py

| Section | Lines | Coverage | Status |
|---------|-------|----------|--------|
| Enums | 13-30 | 100% | 3 tests |
| Constants | 34-36 | 100% | 3 tests |
| ConversationSettings | 40-59 | 100% | 8 tests |
| SkipTrackingSettings | 62-88 | 100% | 7 tests |
| TemplateSettings | 91-114 | 100% | 10 tests |
| FeedbackConfiguration | 117-192 | 100% | 28 tests |
| **TOTAL** | **85** | **95%+** | **59 tests** |

---

# 4. TEST_SKIP_TRACKER.PY

**Target:** 75+ lines, 95% coverage of 78 statements
**Purpose:** Validate skip tracking, atomicity, and persistence

## Test Class: TestSkipTrackerInitialization

Tests for SkipTracker initialization and loading.

### Fixtures Required
```
- temp_log_path: temp file for skip log
- tracker: SkipTracker instance
```

### Test Methods

#### test_skip_tracker_init_default_path
- **Purpose:** Initialize with default path
- **Setup:** SkipTracker()
- **Assertions:**
  - skip_log_path == SkipTracker.DEFAULT_SKIP_LOG_PATH
  - _skip_counters == {}
- **Coverage:** Lines 27-40

#### test_skip_tracker_init_custom_path
- **Purpose:** Initialize with custom path
- **Setup:** SkipTracker(skip_log_path=custom_path)
- **Assertions:**
  - skip_log_path == custom_path
  - _skip_counters == {}
- **Coverage:** Lines 34-40

#### test_skip_tracker_loads_existing_counters
- **Purpose:** Load counters from existing log
- **Setup:** Create log file with skip entries, init SkipTracker
- **Assertions:**
  - Counters loaded from file
  - _skip_counters populated correctly
- **Coverage:** Lines 42-62

#### test_skip_tracker_loads_empty_log
- **Purpose:** Handle empty log file
- **Setup:** Empty log file exists, init SkipTracker
- **Assertions:**
  - No error
  - _skip_counters == {}
- **Coverage:** Lines 42-62

#### test_skip_tracker_loads_missing_log
- **Purpose:** Handle missing log file gracefully
- **Setup:** Log file doesn't exist, init SkipTracker
- **Assertions:**
  - No error
  - _skip_counters == {}
- **Coverage:** Lines 44

#### test_skip_tracker_loads_parses_format
- **Purpose:** Parse log file format correctly
- **Setup:** Log with format: "timestamp: operation: count, action=action"
- **Assertions:**
  - Operation name extracted
  - Count parsed as integer
- **Coverage:** Lines 47-59

#### test_skip_tracker_loads_skips_comments
- **Purpose:** Skip comment lines in log
- **Setup:** Log with comment lines starting with "#"
- **Assertions:**
  - Comments ignored
  - Valid entries loaded
- **Coverage:** Lines 49-50

#### test_skip_tracker_loads_skips_blank_lines
- **Purpose:** Skip blank lines in log
- **Setup:** Log with empty lines
- **Assertions:**
  - Blank lines ignored
  - Valid entries loaded
- **Coverage:** Lines 49

#### test_skip_tracker_loads_malformed_entries
- **Purpose:** Malformed log entries ignored
- **Setup:** Log with invalid format entries
- **Assertions:**
  - Malformed entries skipped
  - Valid entries loaded
  - No error
- **Coverage:** Lines 58-59

#### test_skip_tracker_loads_ioerror
- **Purpose:** Handle file read error
- **Setup:** Log file exists but unreadable
- **Assertions:**
  - IOError caught
  - _skip_counters == {}
  - No exception raised
- **Coverage:** Lines 60-62

#### test_skip_tracker_loads_uses_latest_count
- **Purpose:** Use latest count for duplicate operations
- **Setup:** Log has multiple entries for same operation
- **Assertions:**
  - Latest count used
- **Coverage:** Lines 57

#### test_skip_tracker_default_rating_threshold
- **Purpose:** DEFAULT_RATING_THRESHOLD defined
- **Setup:** Check SkipTracker.DEFAULT_RATING_THRESHOLD
- **Assertions:**
  - DEFAULT_RATING_THRESHOLD == 4
- **Coverage:** Lines 25

---

## Test Class: TestSkipTrackerIncrement

Tests for skip counter increment operations.

### Fixtures Required
```
- tracker: SkipTracker instance
```

### Test Methods

#### test_increment_skip_new_operation
- **Purpose:** Increment counter for new operation
- **Setup:** increment_skip("test_op")
- **Assertions:**
  - Returns 1
  - get_skip_count("test_op") == 1
- **Coverage:** Lines 85-99

#### test_increment_skip_existing_operation
- **Purpose:** Increment existing counter
- **Setup:** increment twice for same operation
- **Assertions:**
  - First call returns 1
  - Second call returns 2
- **Coverage:** Lines 94-97

#### test_increment_skip_multiple_operations
- **Purpose:** Track multiple operations independently
- **Setup:** Increment "op1" and "op2"
- **Assertions:**
  - get_skip_count("op1") == 1
  - get_skip_count("op2") == 1
- **Coverage:** Lines 94-97

#### test_increment_skip_multiple_times
- **Purpose:** Multiple increments accumulate
- **Setup:** increment_skip("op") 5 times
- **Assertions:**
  - Final count == 5
- **Coverage:** Lines 94-97

#### test_increment_skip_logs_operation
- **Purpose:** Skip logged to file
- **Setup:** increment_skip("op"), check log file
- **Assertions:**
  - Log file contains entry
  - Entry has: timestamp, operation name, count, action=skip
- **Coverage:** Lines 98

#### test_increment_skip_thread_safe
- **Purpose:** Concurrent increments are thread-safe
- **Setup:** 10 threads each increment same operation 10 times
- **Assertions:**
  - Final count == 100
  - No lost updates
- **Coverage:** Lines 94 (locking)

---

## Test Class: TestSkipTrackerReset

Tests for skip counter reset operations.

### Fixtures Required
```
- tracker: SkipTracker instance with initial counts
```

### Test Methods

#### test_reset_skip_counter
- **Purpose:** Reset skip counter
- **Setup:** increment "op", reset, check count
- **Assertions:**
  - get_skip_count("op") == 0
- **Coverage:** Lines 113-122

#### test_reset_skip_counter_nonexistent
- **Purpose:** Reset non-existent operation safe
- **Setup:** reset_skip_counter("nonexistent")
- **Assertions:**
  - No error
  - Still 0 (or in _skip_counters as 0)
- **Coverage:** Lines 120-121

#### test_reset_skip_counter_logs_operation
- **Purpose:** Reset logged to file
- **Setup:** reset_skip_counter("op"), check log
- **Assertions:**
  - Log contains reset entry
  - action=reset
  - count == 0
- **Coverage:** Lines 122

#### test_reset_skip_counter_thread_safe
- **Purpose:** Reset is thread-safe
- **Setup:** Concurrent increment and reset
- **Assertions:**
  - No race conditions
  - Final state consistent
- **Coverage:** Lines 119 (locking)

#### test_reset_skip_counter_multiple_operations
- **Purpose:** Reset one operation, others unchanged
- **Setup:** Increment "op1" and "op2", reset "op1"
- **Assertions:**
  - get_skip_count("op1") == 0
  - get_skip_count("op2") > 0
- **Coverage:** Lines 120-121

---

## Test Class: TestSkipTrackerLimitCheck

Tests for skip limit enforcement.

### Fixtures Required
```
- tracker: SkipTracker instance
```

### Test Methods

#### test_check_skip_limit_not_reached
- **Purpose:** Limit not reached returns False
- **Setup:** increment 2 times, check_skip_limit with max=5
- **Assertions:**
  - Returns False
- **Coverage:** Lines 124-145

#### test_check_skip_limit_reached
- **Purpose:** Limit reached returns True
- **Setup:** increment 5 times, check_skip_limit with max=5
- **Assertions:**
  - Returns True
- **Coverage:** Lines 142-144

#### test_check_skip_limit_exceeded
- **Purpose:** Exceeded limit returns True
- **Setup:** increment 10 times, check_skip_limit with max=5
- **Assertions:**
  - Returns True
- **Coverage:** Lines 142-144

#### test_check_skip_limit_zero_unlimited
- **Purpose:** max=0 means unlimited (always False)
- **Setup:** increment 1000 times, check_skip_limit with max=0
- **Assertions:**
  - Returns False
- **Coverage:** Lines 136-138

#### test_check_skip_limit_logs_block
- **Purpose:** Limit block logged to file
- **Setup:** Reach limit, check_skip_limit, verify log
- **Assertions:**
  - Log contains block entry
  - action=block
- **Coverage:** Lines 143

#### test_check_skip_limit_thread_safe
- **Purpose:** Limit check is thread-safe
- **Setup:** Concurrent increment and limit check
- **Assertions:**
  - No race conditions
  - Correct result
- **Coverage:** Lines 140 (locking)

#### test_check_skip_limit_nonexistent_operation
- **Purpose:** Non-existent operation has 0 count
- **Setup:** check_skip_limit("new_op", max=5)
- **Assertions:**
  - Returns False (0 < 5)
- **Coverage:** Lines 141

---

## Test Class: TestSkipTrackerPositiveFeedback

Tests for positive feedback reset functionality.

### Fixtures Required
```
- tracker: SkipTracker instance
```

### Test Methods

#### test_reset_on_positive_above_threshold
- **Purpose:** Positive feedback resets counter
- **Setup:** increment, reset_on_positive with rating=5 (default threshold=4)
- **Assertions:**
  - get_skip_count("op") == 0
- **Coverage:** Lines 147-160

#### test_reset_on_positive_at_threshold
- **Purpose:** Rating at threshold resets counter
- **Setup:** reset_on_positive with rating=4 (threshold=4)
- **Assertions:**
  - Counter reset
- **Coverage:** Lines 159

#### test_reset_on_positive_below_threshold
- **Purpose:** Rating below threshold doesn't reset
- **Setup:** increment 3 times, reset_on_positive with rating=3 (threshold=4)
- **Assertions:**
  - get_skip_count("op") == 3 (unchanged)
- **Coverage:** Lines 159

#### test_reset_on_positive_custom_threshold
- **Purpose:** Custom rating threshold respected
- **Setup:** reset_on_positive with rating=6, threshold=10
- **Assertions:**
  - Counter NOT reset (6 < 10)
- **Coverage:** Lines 156-157

#### test_reset_on_positive_zero_rating
- **Purpose:** Rating of 0 is below threshold
- **Setup:** reset_on_positive with rating=0
- **Assertions:**
  - Counter NOT reset
- **Coverage:** Lines 159

#### test_reset_on_positive_negative_rating
- **Purpose:** Negative rating is below threshold
- **Setup:** reset_on_positive with rating=-5
- **Assertions:**
  - Counter NOT reset
- **Coverage:** Lines 159

#### test_reset_on_positive_uses_default_threshold
- **Purpose:** Default threshold used when not provided
- **Setup:** reset_on_positive with rating=4, no threshold arg
- **Assertions:**
  - Counter reset (4 >= 4 default)
- **Coverage:** Lines 156-157

#### test_reset_on_positive_thread_safe
- **Purpose:** Reset is thread-safe
- **Setup:** Concurrent increment and reset_on_positive
- **Assertions:**
  - No race conditions
- **Coverage:** Lines 160 (locking via reset_skip_counter)

---

## Test Class: TestSkipTrackerCounterAccess

Tests for counter read and management.

### Fixtures Required
```
- tracker: SkipTracker instance
```

### Test Methods

#### test_get_skip_count_existing
- **Purpose:** Get count for existing operation
- **Setup:** increment "op", get_skip_count("op")
- **Assertions:**
  - Returns correct count
- **Coverage:** Lines 101-111

#### test_get_skip_count_nonexistent
- **Purpose:** Get count for non-existent operation
- **Setup:** get_skip_count("nonexistent")
- **Assertions:**
  - Returns 0
- **Coverage:** Lines 111

#### test_get_skip_count_thread_safe
- **Purpose:** Counter reads are thread-safe
- **Setup:** Concurrent increment and get_skip_count
- **Assertions:**
  - Consistent reads
- **Coverage:** Lines 110 (locking)

#### test_get_all_counters
- **Purpose:** Get copy of all counters
- **Setup:** increment multiple operations, get_all_counters()
- **Assertions:**
  - Returns dict with all operations
  - Changes to returned dict don't affect tracker
- **Coverage:** Lines 162-169

#### test_get_all_counters_returns_copy
- **Purpose:** Returned dict is a copy, not reference
- **Setup:** Get all counters, modify returned dict, get again
- **Assertions:**
  - Original counters unchanged
- **Coverage:** Lines 169

#### test_get_all_counters_empty
- **Purpose:** Empty counters returns empty dict
- **Setup:** get_all_counters() with no operations
- **Assertions:**
  - Returns {}
- **Coverage:** Lines 162-169

#### test_get_all_counters_thread_safe
- **Purpose:** Reading all counters is thread-safe
- **Setup:** Concurrent modifications and reads
- **Assertions:**
  - Consistent snapshots
- **Coverage:** Lines 168 (locking)

#### test_clear_all_counters
- **Purpose:** Clear all skip counters
- **Setup:** increment multiple, clear_all_counters(), get_all_counters()
- **Assertions:**
  - Returns {}
- **Coverage:** Lines 171-177

#### test_clear_all_counters_thread_safe
- **Purpose:** Clear is thread-safe
- **Setup:** Concurrent operations and clear
- **Assertions:**
  - No race conditions
- **Coverage:** Lines 176 (locking)

---

## Test Class: TestSkipTrackerIntegration

Integration tests for complete workflows.

### Fixtures Required
```
- tracker: SkipTracker instance
```

### Test Methods

#### test_skip_tracking_workflow_basic
- **Purpose:** Complete skip tracking workflow
- **Setup:** Increment → check limit → reset
- **Assertions:**
  - Each step produces correct results
- **Coverage:** Full integration

#### test_skip_tracking_workflow_with_positive_feedback
- **Purpose:** Skip tracking with positive feedback reset
- **Setup:** Increment → positive feedback → check count
- **Assertions:**
  - Counter resets on positive feedback
- **Coverage:** Lines 147-160 (integration)

#### test_skip_tracking_workflow_limit_then_positive
- **Purpose:** Reach limit, then positive feedback
- **Setup:** Increment to limit → positive feedback → check limit
- **Assertions:**
  - Limit reached, then feedback resets
  - Limit no longer reached after reset
- **Coverage:** Full integration

#### test_skip_tracking_concurrent_increments
- **Purpose:** Multiple threads incrementing safely
- **Setup:** 10 threads, each increment 10 times
- **Assertions:**
  - Final count == 100
  - No lost updates
- **Coverage:** Full integration (threading)

#### test_skip_tracking_log_persistence
- **Purpose:** Counters persist across instances
- **Setup:** Create tracker, increment, shutdown, new tracker
- **Assertions:**
  - New tracker loads previous counts
- **Coverage:** Full integration (persistence)

---

## Coverage Summary for skip_tracker.py

| Section | Lines | Coverage | Status |
|---------|-------|----------|--------|
| Initialization | 27-62 | 100% | 12 tests |
| Increment | 85-99 | 100% | 6 tests |
| Reset | 113-122 | 100% | 5 tests |
| Limit Check | 124-145 | 100% | 7 tests |
| Positive Feedback | 147-160 | 100% | 8 tests |
| Counter Access | 101-177 | 100% | 8 tests |
| Integration | Full | 100% | 5 tests |
| **TOTAL** | **78** | **95%+** | **51 tests** |

---

# 5. TEST_CONFIG_SCHEMA.PY

**Target:** 10+ lines, 95% coverage of 4 statements
**Purpose:** Validate JSON Schema export and structure

## Test Class: TestConfigSchema

Tests for schema retrieval and structure.

### Test Methods

#### test_get_schema_returns_dict
- **Purpose:** get_schema() returns dictionary
- **Setup:** schema = get_schema()
- **Assertions:**
  - isinstance(schema, dict)
- **Coverage:** Lines 134-140

#### test_get_schema_returns_copy
- **Purpose:** Returned schema is a copy, not reference
- **Setup:** schema1 = get_schema(), modify, schema2 = get_schema()
- **Assertions:**
  - schema1 != schema2 (different objects)
  - Original FEEDBACK_CONFIG_SCHEMA unchanged
- **Coverage:** Lines 140

#### test_get_schema_has_required_top_level_keys
- **Purpose:** Schema contains required top-level keys
- **Setup:** schema = get_schema()
- **Assertions:**
  - "$schema" in schema
  - "title" in schema
  - "description" in schema
  - "type" in schema
  - "properties" in schema
- **Coverage:** Lines 12-131

#### test_get_schema_type_is_object
- **Purpose:** Schema type is "object"
- **Setup:** schema = get_schema()
- **Assertions:**
  - schema["type"] == "object"
- **Coverage:** Lines 16

#### test_get_schema_has_enabled_property
- **Purpose:** Schema defines "enabled" property
- **Setup:** schema = get_schema()
- **Assertions:**
  - "enabled" in schema["properties"]
  - schema["properties"]["enabled"]["type"] == "boolean"
- **Coverage:** Lines 18-22

#### test_get_schema_has_trigger_mode_property
- **Purpose:** Schema defines "trigger_mode" property
- **Setup:** schema = get_schema()
- **Assertions:**
  - "trigger_mode" in schema["properties"]
  - schema["properties"]["trigger_mode"]["type"] == "string"
  - "enum" in schema["properties"]["trigger_mode"]
- **Coverage:** Lines 23-28

#### test_get_schema_trigger_mode_enum_values
- **Purpose:** Trigger mode enum has all valid values
- **Setup:** schema = get_schema()
- **Assertions:**
  - "always" in enum
  - "failures-only" in enum
  - "specific-operations" in enum
  - "never" in enum
- **Coverage:** Lines 25

#### test_get_schema_has_operations_property
- **Purpose:** Schema defines "operations" property
- **Setup:** schema = get_schema()
- **Assertions:**
  - "operations" in schema["properties"]
  - "array" in schema["properties"]["operations"]["type"]
- **Coverage:** Lines 29-36

#### test_get_schema_has_conversation_settings
- **Purpose:** Schema defines "conversation_settings"
- **Setup:** schema = get_schema()
- **Assertions:**
  - "conversation_settings" in schema["properties"]
  - "properties" in schema["properties"]["conversation_settings"]
  - "max_questions" and "allow_skip" defined
- **Coverage:** Lines 37-57

#### test_get_schema_has_skip_tracking
- **Purpose:** Schema defines "skip_tracking"
- **Setup:** schema = get_schema()
- **Assertions:**
  - "skip_tracking" in schema["properties"]
  - Contains "enabled", "max_consecutive_skips", "reset_on_positive"
- **Coverage:** Lines 59-85

#### test_get_schema_has_templates
- **Purpose:** Schema defines "templates"
- **Setup:** schema = get_schema()
- **Assertions:**
  - "templates" in schema["properties"]
  - Contains "format" and "tone"
- **Coverage:** Lines 87-109

#### test_get_schema_has_default_values
- **Purpose:** Schema includes default values
- **Setup:** schema = get_schema()
- **Assertions:**
  - "default" in schema
  - schema["default"]["enabled"] == True
  - schema["default"]["trigger_mode"] == "failures-only"
- **Coverage:** Lines 113-130

#### test_get_schema_no_additional_properties
- **Purpose:** Schema disallows additional properties
- **Setup:** schema = get_schema()
- **Assertions:**
  - schema["additionalProperties"] == False
- **Coverage:** Lines 111

#### test_get_schema_json_schema_draft_07
- **Purpose:** Schema specifies JSON Schema draft 07
- **Setup:** schema = get_schema()
- **Assertions:**
  - schema["$schema"] == "http://json-schema.org/draft-07/schema#"
- **Coverage:** Lines 13

#### test_feedback_config_schema_global_defined
- **Purpose:** Global FEEDBACK_CONFIG_SCHEMA defined
- **Setup:** Check FEEDBACK_CONFIG_SCHEMA existence
- **Assertions:**
  - FEEDBACK_CONFIG_SCHEMA is dict
  - Not empty
- **Coverage:** Lines 12

---

## Coverage Summary for config_schema.py

| Section | Lines | Coverage | Status |
|---------|-------|----------|--------|
| Schema Retrieval | 134-140 | 100% | 2 tests |
| Schema Structure | 12-131 | 100% | 13 tests |
| **TOTAL** | **4** | **95%+** | **15 tests** |

---

# 6. TEST_CONFIG_DEFAULTS.PY

**Target:** 15+ lines, 95% coverage of 8 statements
**Purpose:** Validate default configuration values and retrieval

## Test Class: TestConfigDefaults

Tests for default configuration access.

### Test Methods

#### test_default_config_dict_exists
- **Purpose:** DEFAULT_CONFIG_DICT defined
- **Setup:** Check DEFAULT_CONFIG_DICT
- **Assertions:**
  - DEFAULT_CONFIG_DICT is dict
  - Not empty
- **Coverage:** Lines 11-28

#### test_default_config_dict_has_enabled
- **Purpose:** Defaults include enabled field
- **Setup:** DEFAULT_CONFIG_DICT
- **Assertions:**
  - DEFAULT_CONFIG_DICT["enabled"] == True
- **Coverage:** Lines 12

#### test_default_config_dict_has_trigger_mode
- **Purpose:** Defaults include trigger_mode
- **Setup:** DEFAULT_CONFIG_DICT
- **Assertions:**
  - DEFAULT_CONFIG_DICT["trigger_mode"] == "failures-only"
- **Coverage:** Lines 13

#### test_default_config_dict_has_operations
- **Purpose:** Defaults include operations
- **Setup:** DEFAULT_CONFIG_DICT
- **Assertions:**
  - DEFAULT_CONFIG_DICT["operations"] == None
- **Coverage:** Lines 14

#### test_default_config_dict_has_nested_sections
- **Purpose:** Defaults include all nested sections
- **Setup:** DEFAULT_CONFIG_DICT
- **Assertions:**
  - "conversation_settings" in dict
  - "skip_tracking" in dict
  - "templates" in dict
- **Coverage:** Lines 15-27

#### test_default_config_dict_conversation_settings
- **Purpose:** Conversation settings have correct defaults
- **Setup:** DEFAULT_CONFIG_DICT["conversation_settings"]
- **Assertions:**
  - max_questions == 5
  - allow_skip == True
- **Coverage:** Lines 15-17

#### test_default_config_dict_skip_tracking
- **Purpose:** Skip tracking has correct defaults
- **Setup:** DEFAULT_CONFIG_DICT["skip_tracking"]
- **Assertions:**
  - enabled == True
  - max_consecutive_skips == 3
  - reset_on_positive == True
- **Coverage:** Lines 19-22

#### test_default_config_dict_templates
- **Purpose:** Templates have correct defaults
- **Setup:** DEFAULT_CONFIG_DICT["templates"]
- **Assertions:**
  - format == "structured"
  - tone == "brief"
- **Coverage:** Lines 24-26

#### test_get_default_config_returns_dict
- **Purpose:** get_default_config() returns dictionary
- **Setup:** config = get_default_config()
- **Assertions:**
  - isinstance(config, dict)
- **Coverage:** Lines 31-37

#### test_get_default_config_returns_copy
- **Purpose:** get_default_config() returns copy, not reference
- **Setup:** config1 = get_default_config(), modify, config2 = get_default_config()
- **Assertions:**
  - config1 != config2 (different objects)
  - DEFAULT_CONFIG_DICT unchanged
- **Coverage:** Lines 37

#### test_get_default_config_has_all_fields
- **Purpose:** Returned config has all required fields
- **Setup:** config = get_default_config()
- **Assertions:**
  - "enabled" in config
  - "trigger_mode" in config
  - All nested sections present
- **Coverage:** Lines 31-37

#### test_get_default_nested_config_valid_section
- **Purpose:** Get default for specific section
- **Setup:** get_default_nested_config("conversation_settings")
- **Assertions:**
  - Returns dict with max_questions and allow_skip
- **Coverage:** Lines 40-58

#### test_get_default_nested_config_all_sections
- **Purpose:** All sections accessible
- **Setup:** Test each section: conversation_settings, skip_tracking, templates
- **Assertions:**
  - Each returns dict with correct fields
- **Coverage:** Lines 40-58

#### test_get_default_nested_config_invalid_section
- **Purpose:** Invalid section raises error
- **Setup:** get_default_nested_config("invalid_section")
- **Assertions:**
  - ValueError raised
  - Message contains "Unknown configuration section"
  - Lists valid sections
- **Coverage:** Lines 52-56

#### test_get_default_nested_config_returns_copy
- **Purpose:** Returned section is a copy
- **Setup:** config1 = get_default_nested_config(...), modify, config2 = get_default_nested_config(...)
- **Assertions:**
  - config1 != config2
  - DEFAULT_CONFIG_DICT unchanged
- **Coverage:** Lines 58

---

## Coverage Summary for config_defaults.py

| Section | Lines | Coverage | Status |
|---------|-------|----------|--------|
| DEFAULT_CONFIG_DICT | 11-28 | 100% | 8 tests |
| get_default_config | 31-37 | 100% | 3 tests |
| get_default_nested_config | 40-58 | 100% | 4 tests |
| **TOTAL** | **8** | **95%+** | **15 tests** |

---

# Summary & Implementation Guide

## Overall Coverage

**Total Test Methods to Implement: 255+ tests**

- test_config_manager.py: 68 tests (161 statements)
- test_hot_reload.py: 55 tests (99 statements)
- test_config_models.py: 59 tests (85 statements)
- test_skip_tracker.py: 51 tests (78 statements)
- test_config_schema.py: 15 tests (4 statements)
- test_config_defaults.py: 15 tests (8 statements)
- **TOTAL: 263 tests** targeting **435 statements** (~50% coverage improvement)

## Implementation Priority

**Phase 1 (Critical - AC 4,5,6,7,8):**
1. test_config_models.py (data validation, 59 tests)
2. test_config_manager.py - Validation section (7 tests)
3. test_config_defaults.py (15 tests)

**Phase 2 (High - AC 1,9):**
1. test_config_manager.py - Loading/Merging (15 tests)
2. test_hot_reload.py - File watching (55 tests)

**Phase 3 (Medium - AC 5):**
1. test_skip_tracker.py (51 tests)

**Phase 4 (Low):**
1. test_config_schema.py (15 tests)

## Key Testing Patterns

1. **Thread Safety Tests:** Use 3-5 concurrent threads per module
2. **File I/O Tests:** Use `pytest.tmp_path` fixture for temp directories
3. **Mock Tests:** Fixtures for mocking callbacks, file I/O
4. **Parametrized Tests:** Use `@pytest.mark.parametrize` for multiple trigger modes, formats, etc.
5. **Integration Tests:** End-to-end workflows combining multiple components

## Pytest Fixture Setup

**Recommended fixture file (.conftest.py):**

```python
# conftest.py organization
- temp_config_file (cleanup)
- temp_logs_dir (cleanup)
- config_manager instance
- hot_reload_manager instance
- skip_tracker instance
- mock callbacks
- sample YAML files (valid/invalid)
```

## Performance Targets

- File I/O: <100ms per operation
- Hot-reload detection: <5 seconds
- Thread-safe operations: atomic updates
- Skip tracking: <10ms per counter operation

---

**Next Step:** Implement tests following this specification. Each test should be independent, use proper fixtures, and target specific lines of code for maximum coverage.
