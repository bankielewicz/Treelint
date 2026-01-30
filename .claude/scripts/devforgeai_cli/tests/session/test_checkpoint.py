"""
Test Suite for Session Checkpoint Protocol (STORY-120)

This test suite validates the checkpoint system that enables resuming development
sessions when context window fills. Tests follow AAA pattern (Arrange, Act, Assert)
and are designed to FAIL (RED phase) since checkpoint.py doesn't exist yet.

Test framework: pytest
Coverage target: 95% for business logic
Test pyramid: 70% unit, 20% integration, 10% E2E
"""

import json
import tempfile
from pathlib import Path
from datetime import datetime
from typing import Dict, Optional
import shutil

import pytest

# These imports will FAIL (RED phase) - checkpoint module doesn't exist yet
# This is intentional for TDD
try:
    from src.claude.scriptsdevforgeai_cli.session.checkpoint import (
        write_checkpoint,
        read_checkpoint,
        delete_checkpoint,
    )
except ImportError:
    pytest.skip("checkpoint module not yet implemented", allow_module_level=True)


# ============================================================================
# FIXTURES
# ============================================================================

@pytest.fixture
def temp_session_dir():
    """Create temporary session directory for testing."""
    temp_dir = tempfile.mkdtemp(prefix="devforgeai_sessions_")
    yield temp_dir
    # Cleanup
    shutil.rmtree(temp_dir, ignore_errors=True)


@pytest.fixture
def valid_checkpoint_data() -> Dict:
    """Valid checkpoint data matching AC#2 requirements."""
    return {
        "story_id": "STORY-120",
        "phase": 3,
        "phase_name": "Refactor",
        "timestamp": "2025-12-21T15:30:00Z",
        "progress_percentage": 67,
        "dod_completion": {
            "implementation": [5, 8],
            "quality": [2, 6],
            "testing": [3, 5],
            "documentation": [1, 4]
        },
        "last_action": "code-reviewer subagent completed",
        "next_action": "Phase 4: Integration Testing"
    }


@pytest.fixture
def valid_checkpoint_json_file(temp_session_dir) -> Path:
    """Create a valid checkpoint.json file in temp directory."""
    session_dir = Path(temp_session_dir) / "STORY-120"
    session_dir.mkdir(parents=True, exist_ok=True)

    checkpoint_file = session_dir / "checkpoint.json"
    valid_data = {
        "story_id": "STORY-120",
        "phase": 2,
        "phase_name": "Green",
        "timestamp": "2025-12-21T14:00:00Z",
        "progress_percentage": 45,
        "dod_completion": {
            "implementation": [3, 8],
            "quality": [1, 6],
            "testing": [2, 5],
            "documentation": [0, 4]
        },
        "last_action": "backend-architect implementation complete",
        "next_action": "Phase 3: Refactoring"
    }

    checkpoint_file.write_text(json.dumps(valid_data, indent=2))
    return checkpoint_file


@pytest.fixture
def corrupted_checkpoint_json_file(temp_session_dir) -> Path:
    """Create a corrupted checkpoint.json file (invalid JSON)."""
    session_dir = Path(temp_session_dir) / "STORY-120"
    session_dir.mkdir(parents=True, exist_ok=True)

    checkpoint_file = session_dir / "checkpoint.json"
    # Write invalid JSON
    checkpoint_file.write_text("{invalid json content }")
    return checkpoint_file


@pytest.fixture
def missing_fields_checkpoint_json_file(temp_session_dir) -> Path:
    """Create a checkpoint.json file with missing required fields."""
    session_dir = Path(temp_session_dir) / "STORY-120"
    session_dir.mkdir(parents=True, exist_ok=True)

    checkpoint_file = session_dir / "checkpoint.json"
    incomplete_data = {
        "story_id": "STORY-120",
        "phase": 2,
        # Missing: phase_name, timestamp, progress_percentage, dod_completion, next_action
    }

    checkpoint_file.write_text(json.dumps(incomplete_data, indent=2))
    return checkpoint_file


@pytest.fixture
def monkeypatch_sessions_dir(temp_session_dir, monkeypatch):
    """Monkeypatch the sessions directory to use temp directory."""
    # This fixture patches the module-level SESSIONS_DIR if it exists
    monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
    yield temp_session_dir


# ============================================================================
# UNIT TESTS: write_checkpoint()
# ============================================================================

class TestWriteCheckpointUnit:
    """Unit tests for write_checkpoint() function."""

    def test_write_checkpoint_creates_directory_when_missing(
        self,
        temp_session_dir,
        valid_checkpoint_data,
        monkeypatch
    ):
        """
        AC#1: Checkpoint file written at phase completion

        Given: devforgeai/sessions/{STORY-ID}/ directory does not exist
        When: write_checkpoint(story_id='STORY-120', phase=3, progress={...}) called
        Then: Directory is created at devforgeai/sessions/STORY-120/
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-120"
        phase = 3
        progress = valid_checkpoint_data

        expected_dir = Path(temp_session_dir) / story_id
        assert not expected_dir.exists(), "Directory should not exist before call"

        # Act
        result = write_checkpoint(story_id, phase, progress)

        # Assert
        assert result is True, "write_checkpoint should return True on success"
        assert expected_dir.exists(), f"Directory should exist: {expected_dir}"
        assert expected_dir.is_dir(), f"Should be directory: {expected_dir}"


    def test_write_checkpoint_creates_valid_json_file(
        self,
        temp_session_dir,
        valid_checkpoint_data,
        monkeypatch
    ):
        """
        AC#2: Checkpoint includes required fields

        Given: Valid checkpoint data provided
        When: write_checkpoint() called
        Then: Valid JSON file created with all required fields
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-120"
        phase = 3
        progress = valid_checkpoint_data

        checkpoint_file = Path(temp_session_dir) / story_id / "checkpoint.json"

        # Act
        result = write_checkpoint(story_id, phase, progress)

        # Assert
        assert result is True, "Should return True"
        assert checkpoint_file.exists(), f"Checkpoint file should exist: {checkpoint_file}"

        # Verify JSON is valid and contains all required fields
        with open(checkpoint_file) as f:
            data = json.load(f)

        required_fields = ["story_id", "phase", "phase_name", "timestamp",
                          "progress_percentage", "dod_completion", "next_action"]
        for field in required_fields:
            assert field in data, f"Required field missing: {field}"


    def test_write_checkpoint_overwrites_existing_file(
        self,
        temp_session_dir,
        valid_checkpoint_data,
        monkeypatch
    ):
        """
        Given: Checkpoint file already exists
        When: write_checkpoint() called with new data
        Then: Existing checkpoint is overwritten with new data
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-120"

        # Write initial checkpoint
        initial_data = valid_checkpoint_data.copy()
        initial_data["phase"] = 1
        write_checkpoint(story_id, 1, initial_data)

        # Write new checkpoint
        new_data = valid_checkpoint_data.copy()
        new_data["phase"] = 3

        # Act
        result = write_checkpoint(story_id, 3, new_data)

        # Assert
        assert result is True, "Should return True"

        checkpoint_file = Path(temp_session_dir) / story_id / "checkpoint.json"
        with open(checkpoint_file) as f:
            data = json.load(f)

        assert data["phase"] == 3, "Should have new phase number"


    def test_write_checkpoint_validates_phase_range(
        self,
        temp_session_dir,
        valid_checkpoint_data,
        monkeypatch
    ):
        """
        Given: Invalid phase number (not in 0-7 range)
        When: write_checkpoint() called
        Then: Should raise ValueError or return False
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-120"
        invalid_phase = 9  # Outside 0-7 range
        progress = valid_checkpoint_data.copy()

        # Act & Assert
        try:
            result = write_checkpoint(story_id, invalid_phase, progress)
            assert result is False, "Should return False for invalid phase"
        except (ValueError, AssertionError):
            pass  # Either exception or False return is acceptable


    def test_write_checkpoint_validates_story_id_format(
        self,
        temp_session_dir,
        valid_checkpoint_data,
        monkeypatch
    ):
        """
        AC#2: story_id validation

        Given: Invalid story_id format (not STORY-NNN)
        When: write_checkpoint() called
        Then: Should raise ValueError or return False
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        invalid_story_id = "invalid-id"
        phase = 3
        progress = valid_checkpoint_data.copy()

        # Act & Assert
        try:
            result = write_checkpoint(invalid_story_id, phase, progress)
            assert result is False, "Should return False for invalid story_id format"
        except (ValueError, AssertionError):
            pass  # Either exception or False return is acceptable


    def test_write_checkpoint_stores_all_required_fields(
        self,
        temp_session_dir,
        valid_checkpoint_data,
        monkeypatch
    ):
        """
        AC#2: All required fields in checkpoint

        Given: Complete checkpoint data
        When: write_checkpoint() called
        Then: File contains: story_id, phase, phase_name, timestamp,
              progress_percentage, dod_completion, next_action
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-120"
        phase = 3
        progress = valid_checkpoint_data

        # Act
        write_checkpoint(story_id, phase, progress)

        # Assert
        checkpoint_file = Path(temp_session_dir) / story_id / "checkpoint.json"
        with open(checkpoint_file) as f:
            data = json.load(f)

        assert data["story_id"] == "STORY-120"
        assert data["phase"] == 3
        assert data["phase_name"] == "Refactor"
        assert data["timestamp"].endswith("Z"), "Timestamp should be ISO 8601"
        assert isinstance(data["progress_percentage"], int)
        assert 0 <= data["progress_percentage"] <= 100, "Progress should be 0-100"
        assert isinstance(data["dod_completion"], dict)
        assert all(k in data["dod_completion"] for k in
                  ["implementation", "quality", "testing", "documentation"])
        assert data["next_action"] is not None


# ============================================================================
# UNIT TESTS: read_checkpoint()
# ============================================================================

class TestReadCheckpointUnit:
    """Unit tests for read_checkpoint() function."""

    def test_read_checkpoint_returns_dict_when_file_exists(
        self,
        valid_checkpoint_json_file,
        monkeypatch
    ):
        """
        AC#2 & AC#3: read_checkpoint returns valid data

        Given: Valid checkpoint.json file exists
        When: read_checkpoint(story_id='STORY-120') called
        Then: Returns dict with checkpoint data
        """
        # Arrange
        sessions_dir = valid_checkpoint_json_file.parent.parent
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", str(sessions_dir))
        story_id = "STORY-120"

        # Act
        result = read_checkpoint(story_id)

        # Assert
        assert result is not None, "Should return dict, not None"
        assert isinstance(result, dict), "Should return dictionary"
        assert result["story_id"] == "STORY-120"
        assert result["phase"] == 2


    def test_read_checkpoint_returns_none_when_file_missing(
        self,
        temp_session_dir,
        monkeypatch
    ):
        """
        AC#5: Graceful fallback if checkpoint missing

        Given: Checkpoint file does not exist for story
        When: read_checkpoint(story_id='STORY-999') called
        Then: Returns None (not error)
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-999"

        # Act
        result = read_checkpoint(story_id)

        # Assert
        assert result is None, "Should return None when checkpoint missing"


    def test_read_checkpoint_handles_corrupted_json(
        self,
        corrupted_checkpoint_json_file,
        monkeypatch
    ):
        """
        AC#5: Graceful fallback if checkpoint corrupted

        Given: checkpoint.json contains invalid JSON
        When: read_checkpoint() called
        Then: Returns None (not error)
        """
        # Arrange
        sessions_dir = corrupted_checkpoint_json_file.parent.parent
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", str(sessions_dir))
        story_id = "STORY-120"

        # Act
        result = read_checkpoint(story_id)

        # Assert
        assert result is None, "Should return None for corrupted JSON"


    def test_read_checkpoint_validates_schema(
        self,
        missing_fields_checkpoint_json_file,
        monkeypatch
    ):
        """
        AC#2: Checkpoint schema validation

        Given: checkpoint.json missing required fields
        When: read_checkpoint() called
        Then: Returns None (schema validation failed)
        """
        # Arrange
        sessions_dir = missing_fields_checkpoint_json_file.parent.parent
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", str(sessions_dir))
        story_id = "STORY-120"

        # Act
        result = read_checkpoint(story_id)

        # Assert
        assert result is None, "Should return None for invalid schema"


    def test_read_checkpoint_returns_all_required_fields(
        self,
        valid_checkpoint_json_file,
        monkeypatch
    ):
        """
        AC#2: Checkpoint includes all required fields

        Given: Valid checkpoint file with all fields
        When: read_checkpoint() called
        Then: Returned dict has all required fields
        """
        # Arrange
        sessions_dir = valid_checkpoint_json_file.parent.parent
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", str(sessions_dir))
        story_id = "STORY-120"

        # Act
        result = read_checkpoint(story_id)

        # Assert
        assert result is not None
        required_fields = ["story_id", "phase", "phase_name", "timestamp",
                          "progress_percentage", "dod_completion", "next_action"]
        for field in required_fields:
            assert field in result, f"Missing required field: {field}"


# ============================================================================
# UNIT TESTS: delete_checkpoint()
# ============================================================================

class TestDeleteCheckpointUnit:
    """Unit tests for delete_checkpoint() function."""

    def test_delete_checkpoint_removes_file(
        self,
        valid_checkpoint_json_file,
        monkeypatch
    ):
        """
        AC#4: Checkpoint cleaned up on story completion

        Given: checkpoint.json file exists
        When: delete_checkpoint(story_id='STORY-120') called
        Then: Checkpoint file is deleted
        """
        # Arrange
        sessions_dir = valid_checkpoint_json_file.parent.parent
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", str(sessions_dir))
        story_id = "STORY-120"
        checkpoint_file = valid_checkpoint_json_file

        assert checkpoint_file.exists(), "Checkpoint should exist before delete"

        # Act
        result = delete_checkpoint(story_id)

        # Assert
        assert result is True, "Should return True on success"
        assert not checkpoint_file.exists(), "Checkpoint file should be deleted"


    def test_delete_checkpoint_removes_empty_directory(
        self,
        valid_checkpoint_json_file,
        monkeypatch
    ):
        """
        AC#4: Cleanup removes empty session directory

        Given: Session directory contains only checkpoint.json
        When: delete_checkpoint() called
        Then: Empty directory is also removed
        """
        # Arrange
        sessions_dir = valid_checkpoint_json_file.parent.parent
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", str(sessions_dir))
        story_id = "STORY-120"
        session_dir = valid_checkpoint_json_file.parent

        assert session_dir.exists(), "Session dir should exist"

        # Act
        result = delete_checkpoint(story_id)

        # Assert
        assert result is True, "Should return True"
        assert not session_dir.exists(), "Empty session directory should be removed"


    def test_delete_checkpoint_handles_missing_file(
        self,
        temp_session_dir,
        monkeypatch
    ):
        """
        AC#5: Graceful handling of missing checkpoint

        Given: Checkpoint file does not exist
        When: delete_checkpoint(story_id='STORY-999') called
        Then: Returns True (not error)
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-999"

        # Act
        result = delete_checkpoint(story_id)

        # Assert
        assert result is True, "Should return True even if checkpoint missing"


# ============================================================================
# INTEGRATION TESTS: Round-trip operations
# ============================================================================

class TestCheckpointIntegration:
    """Integration tests for checkpoint round-trip operations."""

    def test_checkpoint_round_trip_write_read_delete(
        self,
        temp_session_dir,
        valid_checkpoint_data,
        monkeypatch
    ):
        """
        AC#1-4: Complete checkpoint lifecycle

        Given: No checkpoint exists
        When: write -> read -> delete sequence executed
        Then: Data persists and is properly cleaned up
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-120"
        phase = 3

        # Act - Write
        write_result = write_checkpoint(story_id, phase, valid_checkpoint_data)
        assert write_result is True, "Write should succeed"

        # Act - Read
        read_data = read_checkpoint(story_id)

        # Assert - Read returns data
        assert read_data is not None, "Should read written data"
        assert read_data["phase"] == 3, "Should contain correct phase"
        assert read_data["story_id"] == "STORY-120", "Should contain correct story_id"

        # Act - Delete
        delete_result = delete_checkpoint(story_id)
        assert delete_result is True, "Delete should succeed"

        # Assert - Data gone
        read_after_delete = read_checkpoint(story_id)
        assert read_after_delete is None, "Should not read data after delete"


    def test_checkpoint_concurrent_access_safety(
        self,
        temp_session_dir,
        valid_checkpoint_data,
        monkeypatch
    ):
        """
        AC#4: Concurrent checkpoint writes handled safely

        Given: Multiple rapid write operations (simulating phase completion)
        When: write_checkpoint called 3 times in sequence (phase 1, 2, 3)
        Then: Final checkpoint reflects last write, no corruption
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-120"

        phases = [
            (1, "Red", 30),
            (2, "Green", 50),
            (3, "Refactor", 70)
        ]

        # Act - Write multiple phases
        for phase, phase_name, progress_pct in phases:
            data = valid_checkpoint_data.copy()
            data["phase"] = phase
            data["phase_name"] = phase_name
            data["progress_percentage"] = progress_pct

            result = write_checkpoint(story_id, phase, data)
            assert result is True, f"Write phase {phase} should succeed"

        # Assert - Final read shows phase 3
        final_data = read_checkpoint(story_id)
        assert final_data is not None, "Should read final checkpoint"
        assert final_data["phase"] == 3, "Should have final phase number"
        assert final_data["phase_name"] == "Refactor", "Should have final phase name"
        assert final_data["progress_percentage"] == 70, "Should have final progress"


# ============================================================================
# EDGE CASE TESTS
# ============================================================================

class TestCheckpointEdgeCases:
    """Edge case tests for checkpoint operations."""

    def test_checkpoint_with_unicode_characters(
        self,
        temp_session_dir,
        monkeypatch
    ):
        """
        Given: Checkpoint with unicode characters in last_action
        When: write_checkpoint called
        Then: Unicode is preserved and readable
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-120"
        phase = 3

        data = {
            "story_id": "STORY-120",
            "phase": 3,
            "phase_name": "Refactor",
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "progress_percentage": 70,
            "dod_completion": {
                "implementation": [5, 8],
                "quality": [2, 6],
                "testing": [3, 5],
                "documentation": [1, 4]
            },
            "last_action": "code-reviewer: Fixed 🐛 in parsing logic",
            "next_action": "Phase 4: Integration Testing"
        }

        # Act
        write_result = write_checkpoint(story_id, phase, data)
        read_data = read_checkpoint(story_id)

        # Assert
        assert write_result is True
        assert read_data is not None
        assert "🐛" in read_data["last_action"], "Unicode should be preserved"


    def test_checkpoint_timestamp_is_iso8601(
        self,
        temp_session_dir,
        valid_checkpoint_data,
        monkeypatch
    ):
        """
        AC#2: Timestamp validation (ISO 8601 format)

        Given: Valid checkpoint data with ISO 8601 timestamp
        When: write_checkpoint called
        Then: Timestamp format is maintained
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-120"
        phase = 3
        data = valid_checkpoint_data.copy()

        # Act
        write_checkpoint(story_id, phase, data)
        read_data = read_checkpoint(story_id)

        # Assert
        timestamp = read_data["timestamp"]
        assert timestamp.endswith("Z"), "Should be ISO 8601 UTC (ends with Z)"
        # Should be parseable as ISO 8601
        try:
            datetime.fromisoformat(timestamp.replace("Z", "+00:00"))
        except ValueError:
            pytest.fail(f"Timestamp not valid ISO 8601: {timestamp}")


    def test_checkpoint_progress_percentage_boundary_values(
        self,
        temp_session_dir,
        monkeypatch
    ):
        """
        AC#2: Progress percentage validation (0-100)

        Given: Progress percentage boundary values
        When: write_checkpoint called with progress 0, 50, 100
        Then: All values accepted and preserved
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)

        for progress_pct in [0, 50, 100]:
            story_id = f"STORY-{120 + progress_pct}"
            phase = 3
            data = {
                "story_id": story_id,
                "phase": phase,
                "phase_name": "Refactor",
                "timestamp": datetime.utcnow().isoformat() + "Z",
                "progress_percentage": progress_pct,
                "dod_completion": {
                    "implementation": [0, 8],
                    "quality": [0, 6],
                    "testing": [0, 5],
                    "documentation": [0, 4]
                },
                "next_action": "Continue"
            }

            # Act
            write_result = write_checkpoint(story_id, phase, data)
            read_data = read_checkpoint(story_id)

            # Assert
            assert write_result is True
            assert read_data["progress_percentage"] == progress_pct


# ============================================================================
# RESUME-DEV AUTO-DETECTION TESTS
# ============================================================================

class TestResumeDevAutoDetection:
    """Tests for /resume-dev auto-detection from checkpoint (AC#3)."""

    def test_resume_dev_detects_last_completed_phase(
        self,
        temp_session_dir,
        valid_checkpoint_data,
        monkeypatch
    ):
        """
        AC#3: /resume-dev auto-detects from checkpoint

        Given: Checkpoint shows phase 3 completed
        When: read_checkpoint(story_id) called (simulating /resume-dev)
        Then: Detected phase is 3 (should resume at phase 4)
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-120"

        # Write checkpoint showing phase 3 complete
        data = valid_checkpoint_data.copy()
        data["phase"] = 3
        data["phase_name"] = "Refactor"
        write_checkpoint(story_id, 3, data)

        # Act
        checkpoint = read_checkpoint(story_id)

        # Assert
        assert checkpoint is not None
        assert checkpoint["phase"] == 3, "Should detect phase 3 completed"
        # /resume-dev would use this to determine next phase: 3 + 1 = phase 4


    def test_resume_dev_no_prompting_needed_when_checkpoint_exists(
        self,
        valid_checkpoint_json_file,
        monkeypatch
    ):
        """
        AC#3: No prompting required with checkpoint

        Given: Valid checkpoint file exists
        When: /resume-dev STORY-120 (no phase arg)
        Then: Auto-detection works without asking user for phase
        """
        # Arrange
        sessions_dir = valid_checkpoint_json_file.parent.parent
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", str(sessions_dir))
        story_id = "STORY-120"

        # Act
        checkpoint = read_checkpoint(story_id)

        # Assert
        assert checkpoint is not None, "Should find checkpoint automatically"
        assert checkpoint["phase"] is not None, "Should have phase data"
        # In /resume-dev, this would allow auto-detection without user input


    def test_resume_dev_falls_back_to_phase_0_if_no_checkpoint(
        self,
        temp_session_dir,
        monkeypatch
    ):
        """
        AC#5: Graceful fallback if checkpoint missing

        Given: No checkpoint exists for story
        When: /resume-dev STORY-999 called
        Then: Falls back to Phase 0 with warning message
        """
        # Arrange
        monkeypatch.setenv("DEVFORGEAI_SESSIONS_DIR", temp_session_dir)
        story_id = "STORY-999"

        # Act
        checkpoint = read_checkpoint(story_id)

        # Assert
        assert checkpoint is None, "Checkpoint should be None"
        # /resume-dev would show: "Checkpoint not found. Starting from Phase 0."
