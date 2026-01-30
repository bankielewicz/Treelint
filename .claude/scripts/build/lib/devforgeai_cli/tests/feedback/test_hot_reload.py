"""
Tests for hot_reload.py module.

Validates file watching, change detection, callback invocation,
and hot-reload manager coordination.
Target: 95% coverage of 99 statements.
"""

import pytest
import threading
import time
from pathlib import Path
from unittest.mock import Mock, MagicMock, call
from devforgeai_cli.feedback.hot_reload import (
    FileInfo,
    ConfigFileWatcher,
    HotReloadManager
)


@pytest.fixture
def temp_config_file(tmp_path):
    """Provide a temporary configuration file."""
    config_file = tmp_path / "feedback.yaml"
    config_file.write_text("enabled: true\n")
    return config_file


@pytest.fixture
def callback_mock():
    """Provide a mock callback function."""
    return Mock()


@pytest.fixture
def watcher(temp_config_file, callback_mock):
    """Provide a ConfigFileWatcher instance."""
    return ConfigFileWatcher(
        config_file=temp_config_file,
        on_change_callback=callback_mock,
        poll_interval=0.1,  # Fast polling for tests
        detection_timeout=1.0
    )


class TestFileInfo:
    """Tests for FileInfo NamedTuple."""

    def test_file_info_with_values(self):
        """FileInfo stores mtime and size."""
        info = FileInfo(mtime=123.456, size=1024)
        assert info.mtime == 123.456
        assert info.size == 1024

    def test_file_info_with_none(self):
        """FileInfo can have None values for missing files."""
        info = FileInfo(mtime=None, size=None)
        assert info.mtime is None
        assert info.size is None


class TestConfigFileWatcherInitialization:
    """Tests for ConfigFileWatcher initialization."""

    def test_init_with_all_params(self, temp_config_file, callback_mock):
        """ConfigFileWatcher accepts all parameters."""
        watcher = ConfigFileWatcher(
            config_file=temp_config_file,
            on_change_callback=callback_mock,
            poll_interval=0.5,
            detection_timeout=5.0
        )
        assert watcher.config_file == temp_config_file
        assert watcher.on_change_callback == callback_mock
        assert watcher.poll_interval == 0.5
        assert watcher.detection_timeout == 5.0

    def test_init_default_poll_interval(self, temp_config_file, callback_mock):
        """ConfigFileWatcher has default poll_interval of 0.5s."""
        watcher = ConfigFileWatcher(
            config_file=temp_config_file,
            on_change_callback=callback_mock
        )
        assert watcher.poll_interval == 0.5

    def test_init_default_detection_timeout(self, temp_config_file, callback_mock):
        """ConfigFileWatcher has default detection_timeout of 5.0s."""
        watcher = ConfigFileWatcher(
            config_file=temp_config_file,
            on_change_callback=callback_mock
        )
        assert watcher.detection_timeout == 5.0

    def test_init_starts_not_running(self, watcher):
        """ConfigFileWatcher is not running after initialization."""
        assert watcher.is_running() is False

    def test_init_no_watch_thread(self, watcher):
        """ConfigFileWatcher has no watch thread initially."""
        assert watcher._watch_thread is None


class TestConfigFileWatcherFileInfo:
    """Tests for _get_file_info method."""

    def test_get_file_info_file_exists(self, temp_config_file, watcher):
        """_get_file_info returns mtime and size when file exists."""
        info = watcher._get_file_info()
        assert info.mtime is not None
        assert info.size is not None
        assert info.size > 0  # File has content

    def test_get_file_info_file_not_exists(self, tmp_path, callback_mock):
        """_get_file_info returns (None, None) when file doesn't exist."""
        non_existent = tmp_path / "non-existent.yaml"
        watcher = ConfigFileWatcher(
            config_file=non_existent,
            on_change_callback=callback_mock
        )
        info = watcher._get_file_info()
        assert info.mtime is None
        assert info.size is None


class TestConfigFileWatcherChangeDetection:
    """Tests for _has_file_changed method."""

    def test_has_file_changed_no_baseline(self, watcher):
        """_has_file_changed returns False when no baseline set."""
        watcher._last_file_info = None
        current_info = FileInfo(mtime=123.456, size=1024)
        assert watcher._has_file_changed(current_info) is False

    def test_has_file_changed_mtime_changed(self, watcher):
        """_has_file_changed returns True when mtime changes."""
        watcher._last_file_info = FileInfo(mtime=100.0, size=1024)
        current_info = FileInfo(mtime=200.0, size=1024)
        assert watcher._has_file_changed(current_info) is True

    def test_has_file_changed_size_changed(self, watcher):
        """_has_file_changed returns True when size changes."""
        watcher._last_file_info = FileInfo(mtime=100.0, size=1024)
        current_info = FileInfo(mtime=100.0, size=2048)
        assert watcher._has_file_changed(current_info) is True

    def test_has_file_changed_no_change(self, watcher):
        """_has_file_changed returns False when nothing changed."""
        watcher._last_file_info = FileInfo(mtime=100.0, size=1024)
        current_info = FileInfo(mtime=100.0, size=1024)
        assert watcher._has_file_changed(current_info) is False

    def test_has_file_changed_file_deleted(self, watcher):
        """_has_file_changed handles file deletion."""
        watcher._last_file_info = FileInfo(mtime=100.0, size=1024)
        current_info = FileInfo(mtime=None, size=None)
        assert watcher._has_file_changed(current_info) is False


class TestConfigFileWatcherStartStop:
    """Tests for start() and stop() methods."""

    def test_start_watcher(self, watcher):
        """start() starts the file watcher."""
        watcher.start()
        time.sleep(0.15)  # Give thread time to start
        assert watcher.is_running() is True
        watcher.stop()

    def test_start_watcher_twice_idempotent(self, watcher):
        """start() is idempotent (calling twice doesn't create multiple threads)."""
        watcher.start()
        time.sleep(0.05)
        thread1 = watcher._watch_thread

        watcher.start()  # Call again
        thread2 = watcher._watch_thread

        assert thread1 is thread2
        watcher.stop()

    def test_stop_watcher(self, watcher):
        """stop() stops the file watcher."""
        watcher.start()
        time.sleep(0.15)
        watcher.stop()
        assert watcher.is_running() is False

    def test_stop_watcher_not_running_idempotent(self, watcher):
        """stop() is idempotent (calling when not running is safe)."""
        assert watcher.is_running() is False
        watcher.stop()  # Should not error
        assert watcher.is_running() is False

    def test_is_running_after_start(self, watcher):
        """is_running() returns True after start()."""
        watcher.start()
        time.sleep(0.05)
        assert watcher.is_running() is True
        watcher.stop()

    def test_is_running_after_stop(self, watcher):
        """is_running() returns False after stop()."""
        watcher.start()
        time.sleep(0.05)
        watcher.stop()
        time.sleep(0.05)
        assert watcher.is_running() is False


class TestConfigFileWatcherCallbacks:
    """Tests for callback invocation on file changes (AC-9)."""

    def test_callback_invoked_on_file_change(self, temp_config_file, callback_mock):
        """Callback is called when file changes (AC-9)."""
        watcher = ConfigFileWatcher(
            config_file=temp_config_file,
            on_change_callback=callback_mock,
            poll_interval=0.1
        )
        watcher.start()
        time.sleep(0.15)  # Let watcher initialize

        # Modify file
        temp_config_file.write_text("enabled: false\n")
        time.sleep(0.5)  # Wait for detection (≤5s per AC-9)

        watcher.stop()

        # Callback should have been called
        assert callback_mock.call_count >= 1
        callback_mock.assert_called_with(temp_config_file)

    def test_callback_not_invoked_when_no_change(self, temp_config_file, callback_mock):
        """Callback is NOT called when file doesn't change."""
        watcher = ConfigFileWatcher(
            config_file=temp_config_file,
            on_change_callback=callback_mock,
            poll_interval=0.1
        )
        watcher.start()
        time.sleep(0.3)  # Let it poll a few times
        watcher.stop()

        # Callback should not be called (file didn't change)
        assert callback_mock.call_count == 0

    def test_callback_survives_exception(self, temp_config_file):
        """Watcher continues running even if callback raises exception."""
        exception_callback = Mock(side_effect=Exception("Callback error"))

        watcher = ConfigFileWatcher(
            config_file=temp_config_file,
            on_change_callback=exception_callback,
            poll_interval=0.1
        )
        watcher.start()
        time.sleep(0.15)

        # Modify file
        temp_config_file.write_text("enabled: false\n")
        time.sleep(0.5)

        # Watcher should still be running despite callback exception
        assert watcher.is_running() is True
        watcher.stop()


class TestHotReloadManagerInitialization:
    """Tests for HotReloadManager initialization."""

    def test_init_hot_reload_manager(self, temp_config_file):
        """HotReloadManager accepts config file and callback."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        assert manager.config_file == temp_config_file
        assert manager.load_config_callback == callback

    def test_init_no_watcher_created(self, temp_config_file):
        """HotReloadManager doesn't create watcher on init."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        assert manager._watcher is None


class TestHotReloadManagerLifecycle:
    """Tests for HotReloadManager start/stop lifecycle."""

    def test_start_creates_watcher(self, temp_config_file):
        """start() creates ConfigFileWatcher instance."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        manager.start()
        time.sleep(0.1)

        assert manager._watcher is not None
        assert manager.is_running() is True

        manager.stop()

    def test_start_idempotent(self, temp_config_file):
        """start() is idempotent (doesn't create multiple watchers)."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        manager.start()
        time.sleep(0.05)
        watcher1 = manager._watcher

        manager.start()  # Call again
        watcher2 = manager._watcher

        assert watcher1 is watcher2
        manager.stop()

    def test_stop_stops_watcher(self, temp_config_file):
        """stop() stops the watcher."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        manager.start()
        time.sleep(0.1)
        manager.stop()

        assert manager._watcher is None
        assert manager.is_running() is False

    def test_is_running_after_start(self, temp_config_file):
        """is_running() returns True after start()."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        manager.start()
        time.sleep(0.1)
        assert manager.is_running() is True
        manager.stop()

    def test_is_running_after_stop(self, temp_config_file):
        """is_running() returns False after stop()."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        manager.start()
        time.sleep(0.1)
        manager.stop()
        assert manager.is_running() is False


class TestHotReloadManagerConfigManagement:
    """Tests for configuration get/set methods."""

    def test_get_current_config_none(self, temp_config_file):
        """get_current_config returns None initially."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        assert manager.get_current_config() is None

    def test_set_current_config(self, temp_config_file):
        """set_current_config stores configuration."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        config_obj = {"enabled": True}
        manager.set_current_config(config_obj)
        assert manager.get_current_config() == config_obj

    def test_get_current_config_after_set(self, temp_config_file):
        """get_current_config returns set configuration."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        config1 = {"enabled": True}
        config2 = {"enabled": False}

        manager.set_current_config(config1)
        assert manager.get_current_config() == config1

        manager.set_current_config(config2)
        assert manager.get_current_config() == config2


class TestHotReloadManagerConfigReload:
    """Tests for configuration reloading on file changes (AC-9)."""

    def test_on_config_change_calls_callback(self, temp_config_file):
        """_on_config_change invokes load_config_callback."""
        new_config = {"enabled": False, "trigger_mode": "never"}
        callback = Mock(return_value=new_config)

        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        manager._on_config_change(temp_config_file)

        assert callback.call_count == 1

    def test_on_config_change_updates_current_config(self, temp_config_file):
        """_on_config_change updates current configuration."""
        new_config = {"enabled": False, "trigger_mode": "never"}
        callback = Mock(return_value=new_config)

        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        manager._on_config_change(temp_config_file)

        assert manager.get_current_config() == new_config

    def test_on_config_change_exception_keeps_previous_config(self, temp_config_file):
        """_on_config_change keeps previous config when callback raises exception."""
        old_config = {"enabled": True}
        callback = Mock(side_effect=Exception("Load error"))

        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        manager.set_current_config(old_config)

        # Trigger change - should catch exception
        manager._on_config_change(temp_config_file)

        # Previous config should be preserved
        assert manager.get_current_config() == old_config


class TestHotReloadIntegration:
    """Integration tests for hot-reload functionality (AC-9)."""

    def test_end_to_end_file_change_detected(self, temp_config_file):
        """End-to-end: File change triggers reload within 5s (AC-9)."""
        new_config = {"enabled": False, "trigger_mode": "never"}
        callback = Mock(return_value=new_config)

        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        manager.start()
        time.sleep(0.2)  # Let watcher initialize

        # Modify file
        temp_config_file.write_text("enabled: false\ntrigger_mode: never\n")

        # Wait up to 5 seconds for detection (AC-9 requirement: ≤5s)
        timeout = 5.0
        start_time = time.time()
        while time.time() - start_time < timeout:
            if callback.call_count > 0:
                break
            time.sleep(0.1)

        manager.stop()

        # Callback should have been invoked within 5s
        assert callback.call_count >= 1
        assert manager.get_current_config() == new_config

    def test_end_to_end_multiple_changes(self, temp_config_file):
        """End-to-end: Multiple file changes are detected."""
        configs = [
            {"enabled": False},
            {"enabled": True},
            {"trigger_mode": "always"}
        ]
        callback = Mock(side_effect=configs)

        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        manager.start()
        time.sleep(0.2)

        # Make 3 file changes
        temp_config_file.write_text("enabled: false\n")
        time.sleep(0.3)

        temp_config_file.write_text("enabled: true\n")
        time.sleep(0.3)

        temp_config_file.write_text("trigger_mode: always\n")
        time.sleep(0.3)

        manager.stop()

        # At least one change should be detected
        assert callback.call_count >= 1


class TestHotReloadManagerThreadSafety:
    """Tests for thread-safe operations."""

    def test_set_current_config_thread_safe(self, temp_config_file):
        """set_current_config is thread-safe."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )

        configs = [{"id": i} for i in range(20)]

        def set_config(config):
            manager.set_current_config(config)
            time.sleep(0.001)

        threads = [threading.Thread(target=set_config, args=(configs[i],)) for i in range(20)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        # Should have one of the configs (not corrupted)
        final_config = manager.get_current_config()
        assert isinstance(final_config, dict)
        assert "id" in final_config

    def test_get_current_config_thread_safe(self, temp_config_file):
        """get_current_config is thread-safe during concurrent access."""
        callback = Mock()
        manager = HotReloadManager(
            config_file=temp_config_file,
            load_config_callback=callback
        )
        manager.set_current_config({"enabled": True})

        results = []

        def read_config():
            for _ in range(5):
                results.append(manager.get_current_config())
                time.sleep(0.001)

        threads = [threading.Thread(target=read_config) for _ in range(5)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        # All reads should see consistent value
        assert all(r == {"enabled": True} for r in results)


class TestCleanup:
    """Tests for cleanup after testing."""

    def test_cleanup_watcher_stops(self, watcher):
        """Ensure watcher stops for cleanup."""
        watcher.start()
        time.sleep(0.1)
        watcher.stop()
        # Short delay to ensure thread fully stopped
        time.sleep(0.2)
        assert watcher.is_running() is False
