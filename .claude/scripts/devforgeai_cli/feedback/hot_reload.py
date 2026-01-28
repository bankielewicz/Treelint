"""
Hot-reload system for feedback configuration.

This module provides file watching and automatic configuration reloading
when the configuration file changes.
"""

import threading
import time
import os
from pathlib import Path
from typing import Optional, Callable, Any, NamedTuple
from datetime import datetime


class FileInfo(NamedTuple):
    """File information (modification time and size)."""
    mtime: Optional[float]
    size: Optional[int]


class ConfigFileWatcher:
    """Watches configuration file for changes and triggers reloads.

    Monitors devforgeai/config/feedback.yaml for changes and calls
    a callback function when modifications are detected.
    """

    def __init__(
        self,
        config_file: Path,
        on_change_callback: Callable[[Path], None],
        poll_interval: float = 0.5,
        detection_timeout: float = 5.0
    ):
        """Initialize the file watcher.

        Args:
            config_file: Path to the configuration file to watch.
            on_change_callback: Function to call when file changes.
                               Called with Path to the changed file.
            poll_interval: How often to check for changes (seconds).
            detection_timeout: Maximum time to detect change (seconds).
        """
        self.config_file = config_file
        self.on_change_callback = on_change_callback
        self.poll_interval = poll_interval
        self.detection_timeout = detection_timeout
        self._last_file_info: Optional[FileInfo] = None
        self._watch_thread: Optional[threading.Thread] = None
        self._stop_event = threading.Event()
        self._is_running = False
        self._lock = threading.Lock()

    def _get_file_info(self) -> FileInfo:
        """Get file modification time and size.

        Returns:
            FileInfo with mtime and size, or (None, None) if file doesn't exist.
        """
        try:
            if self.config_file.exists():
                stat = self.config_file.stat()
                return FileInfo(stat.st_mtime, stat.st_size)
        except (OSError, IOError):
            pass
        return FileInfo(None, None)

    def _watch_loop(self) -> None:
        """Main file watching loop (runs in thread)."""
        # Initialize baseline
        self._last_file_info = self._get_file_info()

        while not self._stop_event.is_set():
            try:
                time.sleep(self.poll_interval)

                current_file_info = self._get_file_info()

                # Check if file changed
                if self._has_file_changed(current_file_info):
                    # File changed - trigger callback
                    try:
                        self.on_change_callback(self.config_file)
                    except Exception:
                        # Silently ignore callback errors
                        pass
                    self._last_file_info = current_file_info

            except Exception:
                # Silently ignore watch loop errors
                pass

    def _has_file_changed(self, current_info: FileInfo) -> bool:
        """Check if file information has changed.

        Args:
            current_info: Current file information.

        Returns:
            True if file has changed, False otherwise.
        """
        if self._last_file_info is None:
            return False

        return (current_info.mtime is not None and
                self._last_file_info.mtime is not None and
                (current_info.mtime != self._last_file_info.mtime or
                 current_info.size != self._last_file_info.size))

    def start(self) -> None:
        """Start watching the configuration file."""
        with self._lock:
            if self._is_running:
                return

            self._stop_event.clear()
            self._watch_thread = threading.Thread(target=self._watch_loop, daemon=True)
            self._watch_thread.start()
            self._is_running = True

    def stop(self) -> None:
        """Stop watching the configuration file."""
        with self._lock:
            if not self._is_running:
                return

            self._stop_event.set()
            if self._watch_thread and self._watch_thread.is_alive():
                self._watch_thread.join(timeout=1.0)
            self._is_running = False

    def is_running(self) -> bool:
        """Check if watcher is currently running.

        Returns:
            True if watcher is active, False otherwise.
        """
        with self._lock:
            return self._is_running


class HotReloadManager:
    """Manages configuration hot-reload lifecycle.

    Coordinates file watching and configuration updates.
    """

    def __init__(
        self,
        config_file: Path,
        load_config_callback: Callable[[], Any]
    ):
        """Initialize hot-reload manager.

        Args:
            config_file: Path to the configuration file.
            load_config_callback: Function that loads and returns new configuration.
                                 Called when file changes are detected.
        """
        self.config_file = config_file
        self.load_config_callback = load_config_callback
        self._watcher: Optional[ConfigFileWatcher] = None
        self._lock = threading.Lock()
        self._current_config: Optional[Any] = None

    def _on_config_change(self, changed_file: Path) -> None:
        """Handle configuration file change.

        Args:
            changed_file: Path to the changed file.
        """
        try:
            with self._lock:
                # Load new configuration
                new_config = self.load_config_callback()
                self._current_config = new_config
        except Exception:
            # Keep previous valid configuration on error
            pass

    def start(self) -> None:
        """Start the hot-reload manager."""
        with self._lock:
            if self._watcher is not None:
                return

            self._watcher = ConfigFileWatcher(
                self.config_file,
                self._on_config_change
            )
            self._watcher.start()

    def stop(self) -> None:
        """Stop the hot-reload manager."""
        with self._lock:
            if self._watcher is not None:
                self._watcher.stop()
                self._watcher = None

    def is_running(self) -> bool:
        """Check if hot-reload is active.

        Returns:
            True if hot-reload manager is running, False otherwise.
        """
        with self._lock:
            return self._watcher is not None and self._watcher.is_running()

    def get_current_config(self) -> Optional[Any]:
        """Get the current loaded configuration.

        Returns:
            Current configuration object or None if not loaded.
        """
        with self._lock:
            return self._current_config

    def set_current_config(self, config: Any) -> None:
        """Set the current configuration (used during initialization).

        Args:
            config: Configuration object to set as current.
        """
        with self._lock:
            self._current_config = config
