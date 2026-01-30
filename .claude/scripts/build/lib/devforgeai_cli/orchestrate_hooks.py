"""
DevForgeAI Orchestrate Hooks - Workflow Context Extraction

Extracts comprehensive workflow context for /orchestrate command hooks.

Features:
- Workflow status determination (SUCCESS/FAILURE)
- Phase execution tracking (dev, qa, release)
- Quality gate aggregation
- Checkpoint resume detection
- Failure reason extraction
- Context validation and JSON serialization

Story: STORY-026 - Wire hooks into /orchestrate command
"""

import json
import logging
import re
from datetime import datetime, timedelta
from typing import Dict, List, Any, Optional
from uuid import uuid4

logger = logging.getLogger(__name__)

# ============================================================================
# Module-Level Constants
# ============================================================================

# Phase identifiers and labels
PHASE_DEVELOPMENT = "development"
PHASE_QA = "qa"
PHASE_RELEASE = "release"
PHASE_LABELS = {
    PHASE_DEVELOPMENT: "Development",
    PHASE_QA: "QA",
    PHASE_RELEASE: "Release",
}

# Workflow status values
STATUS_PASSED = "PASSED"
STATUS_FAILED = "FAILED"
STATUS_NOT_RUN = "NOT_RUN"
VALID_STATUSES = {STATUS_PASSED, STATUS_FAILED, STATUS_NOT_RUN}

# Workflow completion statuses
WORKFLOW_SUCCESS = "SUCCESS"
WORKFLOW_FAILURE = "FAILURE"

# Quality gate identifiers
GATE_CONTEXT_VALIDATION = "context_validation"
GATE_TEST_PASSING = "test_passing"
GATE_COVERAGE = "coverage"
GATE_QA_APPROVED = "qa_approved"

# Regex patterns
PATTERN_TIMESTAMP_ISO8601 = r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z)"
PATTERN_DURATION = r"Duration:\s*(\d+)"
PATTERN_QA_ATTEMPT = r"### QA Attempt (\d+)"
PATTERN_STATUS_LINE = r"Status:\s*(\w+)"
PATTERN_STATUS_BULLET = r"-\s*Status:\s*(\w+)"
PATTERN_CHECKPOINT = r"Checkpoint:\s*(\w+)"
PATTERN_PREVIOUS_DURATION = r"previous_duration:\s*(\d+)"
PATTERN_QA_ATTEMPTS_FIELD = r"qa_attempts:\s*(\d+)"
PATTERN_FAILED_CRITERION = r"Failed Criterion:\s*(.+?)$"
PATTERN_FAILURE_REASON = r"failure_reason:\s*(.+?)$"


class OrchestrateHooksContextExtractor:
    """
    Extracts comprehensive workflow context from orchestrate operations.

    This extractor parses story files to determine workflow status, phase
    execution, quality gates, and failure information. All extracted data
    is validated and returned in a consistent JSON-serializable format.

    Typical usage:
        extractor = OrchestrateHooksContextExtractor()
        context = extractor.extract_workflow_context(story_content, "STORY-001")
    """

    def __init__(self) -> None:
        """Initialize the context extractor (no state required)."""

    def extract_workflow_context(
        self,
        story_content: str,
        story_id: str,
        workflow_start_time: Optional[str] = None,
    ) -> Dict[str, Any]:
        """
        Extract complete workflow context from story file.

        Parses story file to determine:
        - Phase execution status (dev, qa, release)
        - Workflow completion status (SUCCESS/FAILURE)
        - Quality gate status and failure reasons
        - Checkpoint resume information
        - Workflow timing and durations

        Args:
            story_content: Complete story file content
            story_id: Story ID (e.g., STORY-001)
            workflow_start_time: Optional ISO8601 start time

        Returns:
            Dictionary with keys: workflow_id, story_id, status, total_duration,
            start_time, end_time, phases_executed, quality_gates, checkpoint_info.
            If status is FAILURE, also includes: failed_phase, failure_summary,
            phases_aborted, and optionally qa_attempts.

        Raises:
            No exceptions raised; errors logged and error context returned.
        """
        try:
            phases_executed = self._extract_phases(story_content)
            status = self._determine_status(phases_executed)
            quality_gates = self._extract_quality_gates(story_content, phases_executed)
            checkpoint_info = self._extract_checkpoint_info(story_content, phases_executed)
            start_time, end_time, duration = self._calculate_duration(
                story_content, phases_executed, workflow_start_time
            )

            context = self._build_context(
                story_id=story_id,
                status=status,
                duration=duration,
                start_time=start_time,
                end_time=end_time,
                phases_executed=phases_executed,
                quality_gates=quality_gates,
                checkpoint_info=checkpoint_info,
            )

            # Add failure-specific fields if workflow failed
            if status == WORKFLOW_FAILURE:
                self._add_failure_context(context, story_content, phases_executed)

            return context

        except Exception as e:
            logger.error(f"Context extraction error: {str(e)}")
            return self._create_error_context(story_id)

    def _build_context(
        self,
        story_id: str,
        status: str,
        duration: int,
        start_time: str,
        end_time: str,
        phases_executed: List[Dict[str, Any]],
        quality_gates: Dict[str, Any],
        checkpoint_info: Dict[str, Any],
    ) -> Dict[str, Any]:
        """
        Build base workflow context dictionary.

        Args:
            story_id: Story identifier
            status: Workflow status (SUCCESS/FAILURE)
            duration: Total workflow duration in seconds
            start_time: ISO8601 start timestamp
            end_time: ISO8601 end timestamp
            phases_executed: List of phase execution data
            quality_gates: Quality gate status dictionary
            checkpoint_info: Checkpoint resume information

        Returns:
            Context dictionary with all provided fields
        """
        return {
            "workflow_id": str(uuid4()),
            "story_id": story_id,
            "status": status,
            "total_duration": duration,
            "start_time": start_time,
            "end_time": end_time,
            "phases_executed": phases_executed,
            "quality_gates": quality_gates,
            "checkpoint_info": checkpoint_info,
        }

    def _add_failure_context(
        self,
        context: Dict[str, Any],
        story_content: str,
        phases_executed: List[Dict[str, Any]],
    ) -> None:
        """
        Add failure-specific information to context (mutates context dict).

        Determines which phase failed, extracts failure reasons, identifies
        aborted phases, and counts QA attempts.

        Args:
            context: Context dictionary to update with failure info
            story_content: Story file content for parsing
            phases_executed: List of phase execution data
        """
        failed_phase = self._get_failed_phase(phases_executed)
        context["failed_phase"] = failed_phase
        context["failure_summary"] = self._extract_failure_summary(
            story_content, failed_phase
        )
        context["phases_aborted"] = self._get_aborted_phases(phases_executed)

        qa_attempts = self._extract_qa_attempts(story_content)
        if qa_attempts is not None:
            context["qa_attempts"] = qa_attempts

    def _extract_phases(self, story_content: str) -> List[Dict[str, Any]]:
        """
        Extract phase information from workflow history.

        Parses story content to find development, QA, and release phases.
        Returns phases in execution order.

        Args:
            story_content: Story content to parse

        Returns:
            List of phase dictionaries with status and metadata
        """
        phases = []

        # Extract each phase in order
        for phase_key, phase_label in PHASE_LABELS.items():
            phase_data = self._extract_phase(story_content, phase_key, phase_label)
            if phase_data:
                phases.append(phase_data)

        return phases

    def _extract_phase(
        self, content: str, phase_key: str, phase_label: str
    ) -> Optional[Dict[str, Any]]:
        """
        Extract specific phase information from story content.

        Searches for phase section, extracts status and duration, and adds
        phase-specific metadata (qa_attempts for QA phase).

        Args:
            content: Story content to search
            phase_key: Phase identifier (development, qa, release)
            phase_label: Phase label as it appears in content

        Returns:
            Phase dictionary with keys: phase, status, duration (if available),
            and phase-specific fields. Returns None if phase not found.
        """
        phase_pattern = rf"### {phase_label}.*?\n(.*?)(?=###|$)"
        match = re.search(phase_pattern, content, re.IGNORECASE | re.DOTALL)

        if not match:
            return None

        phase_content = match.group(1)
        status = self._extract_status(phase_content)

        if not status:
            return None

        phase_data: Dict[str, Any] = {
            "phase": phase_key,
            "status": status,
        }

        # Extract optional duration
        duration = self._extract_duration_from_phase(phase_content)
        if duration:
            phase_data["duration"] = duration

        # Extract phase-specific details
        if phase_key == PHASE_QA:
            self._add_qa_phase_details(phase_data, phase_content)

        return phase_data

    def _add_qa_phase_details(self, phase_data: Dict[str, Any], content: str) -> None:
        """
        Add QA-specific phase details to phase data (mutates phase_data dict).

        Extracts QA attempt count and failure reason from content.

        Args:
            phase_data: Phase dictionary to update
            content: Phase content to parse
        """
        qa_attempts = self._extract_qa_attempt_count(content)
        if qa_attempts:
            phase_data["qa_attempts"] = qa_attempts

        failure_reason = self._extract_failure_reason(content)
        if failure_reason:
            phase_data["failure_reason"] = failure_reason

    def _extract_status(self, content: str) -> Optional[str]:
        """
        Extract status from phase content.

        Checks for status in "Status: VALUE" or "- Status: VALUE" format.

        Args:
            content: Phase content to search

        Returns:
            Status string (PASSED, FAILED, NOT_RUN) or None if not found
        """
        # Try pattern with leading status line
        match = re.search(PATTERN_STATUS_LINE, content)
        if match:
            status = match.group(1).upper()
            if status in VALID_STATUSES:
                return status

        # Try pattern with bullet point "- Status: VALUE"
        match = re.search(PATTERN_STATUS_BULLET, content)
        if match:
            status = match.group(1).upper()
            if status in VALID_STATUSES:
                return status

        return None

    def _extract_duration_from_phase(self, content: str) -> Optional[int]:
        """
        Extract duration in seconds from phase content.

        Searches for "Duration: N" pattern where N is a number of seconds.

        Args:
            content: Phase content to search

        Returns:
            Duration in seconds or None if not found
        """
        match = re.search(PATTERN_DURATION, content)
        if match:
            return int(match.group(1))
        return None

    def _extract_qa_attempt_count(self, content: str) -> Optional[int]:
        """
        Extract QA attempt count from phase content.

        Counts "### QA Attempt N" sections or looks for qa_attempts field.

        Args:
            content: Phase content to search

        Returns:
            Number of QA attempts or None if not found
        """
        # Count "### QA Attempt N" sections
        attempts = re.findall(PATTERN_QA_ATTEMPT, content)
        if attempts:
            return len(attempts)

        # Also check for qa_attempts field
        match = re.search(PATTERN_QA_ATTEMPTS_FIELD, content)
        if match:
            return int(match.group(1))

        return None

    def _extract_failure_reason(self, content: str) -> Optional[str]:
        """
        Extract failure reason from phase content.

        Searches for "Failed Criterion:" or "failure_reason:" patterns.

        Args:
            content: Phase content to search

        Returns:
            Failure reason string or None if not found
        """
        match = re.search(PATTERN_FAILED_CRITERION, content, re.MULTILINE)
        if match:
            return match.group(1).strip()

        match = re.search(PATTERN_FAILURE_REASON, content, re.MULTILINE)
        if match:
            return match.group(1).strip()

        return None

    def _determine_status(self, phases: List[Dict[str, Any]]) -> str:
        """
        Determine overall workflow status based on phase statuses.

        All phases must have PASSED status for workflow to be SUCCESS.
        If any phase is FAILED or NOT_RUN, workflow is FAILURE.

        Args:
            phases: List of phase dictionaries with status field

        Returns:
            Workflow completion status (SUCCESS or FAILURE)
        """
        if not phases:
            return WORKFLOW_FAILURE

        # All phases must be PASSED for SUCCESS
        for phase in phases:
            if phase.get("status") != STATUS_PASSED:
                return WORKFLOW_FAILURE

        return WORKFLOW_SUCCESS

    def _extract_quality_gates(
        self, content: str, phases: List[Dict[str, Any]]
    ) -> Dict[str, Any]:
        """
        Extract quality gate information from phase data.

        Initializes all gates with PASSED status, then marks failed gates
        based on phase failure information.

        Args:
            content: Story content (reserved for future use)
            phases: List of phase dictionaries with status and metadata

        Returns:
            Dictionary with quality gate status for each gate
        """
        gates = self._initialize_quality_gates()

        # Check if any phase failed and update gates accordingly
        for phase in phases:
            if phase.get("status") == STATUS_FAILED:
                self._update_failed_gates(gates, phase)

        return gates

    def _initialize_quality_gates(self) -> Dict[str, Any]:
        """
        Initialize all quality gates with PASSED status.

        Returns:
            Dictionary with all quality gates initialized to PASSED
        """
        return {
            GATE_CONTEXT_VALIDATION: {"status": STATUS_PASSED},
            GATE_TEST_PASSING: {"status": STATUS_PASSED},
            GATE_COVERAGE: {"status": STATUS_PASSED},
            GATE_QA_APPROVED: {"status": STATUS_PASSED},
        }

    def _update_failed_gates(self, gates: Dict[str, Any], phase: Dict[str, Any]) -> None:
        """
        Update gate statuses based on phase failure (mutates gates dict).

        If the failed phase is QA, marks QA gate as FAILED and includes
        failure reason if available.

        Args:
            gates: Quality gates dictionary to update
            phase: Phase data with failure information
        """
        if phase.get("phase") == PHASE_QA:
            gates[GATE_QA_APPROVED]["status"] = STATUS_FAILED
            if "failure_reason" in phase:
                gates[GATE_QA_APPROVED]["reason"] = phase["failure_reason"]

    def _extract_checkpoint_info(
        self, content: str, phases: List[Dict[str, Any]]
    ) -> Dict[str, Any]:
        """
        Extract checkpoint/resume information from story content.

        Detects checkpoint resume patterns and extracts resume point,
        previous phase execution, and cumulative durations.

        Args:
            content: Story content to search
            phases: List of phase dictionaries

        Returns:
            Checkpoint info dictionary with resume status and details
        """
        checkpoint_info: Dict[str, Any] = {
            "checkpoint_resumed": False,
            "resume_point": None,
            "phases_skipped": [],
        }

        # Check if checkpoint/resume pattern exists
        if "Checkpoint:" not in content or "Resume" not in content:
            return checkpoint_info

        checkpoint_info["checkpoint_resumed"] = True

        # Extract resume point (e.g., QA_APPROVED)
        match = re.search(PATTERN_CHECKPOINT, content)
        if match:
            checkpoint_info["resume_point"] = match.group(1)

        # Track phases in previous sessions
        phases_in_previous = self._extract_previous_phases(content, phases)
        if phases_in_previous:
            checkpoint_info["phases_in_previous_sessions"] = phases_in_previous

        # Extract previous duration if available
        match = re.search(PATTERN_PREVIOUS_DURATION, content)
        if match:
            checkpoint_info["previous_phases_duration"] = int(match.group(1))

        return checkpoint_info

    def _extract_previous_phases(
        self, content: str, phases: List[Dict[str, Any]]
    ) -> List[Dict[str, str]]:
        """
        Extract phases that ran in previous checkpoint sessions.

        Identifies phases marked as "previous" in content.

        Args:
            content: Story content to search
            phases: List of all phase dictionaries

        Returns:
            List of phase data from previous sessions
        """
        phases_in_previous = []

        for phase in phases:
            # Check if phase is marked as from previous session
            if "previous" in content.lower() and phase.get("phase") in content:
                phases_in_previous.append({
                    "phase": phase.get("phase"),
                    "status": phase.get("status"),
                })

        return phases_in_previous

    def _calculate_duration(
        self,
        content: str,
        phases: List[Dict[str, Any]],
        workflow_start_time: Optional[str] = None,
    ) -> tuple:
        """
        Calculate workflow timing information.

        Sums phase durations and extracts/generates start and end times.
        If workflow_start_time provided, uses that as start. If end time
        not found, calculates from start time + total duration.

        Args:
            content: Story content to search for timestamps
            phases: List of phase dictionaries with duration field
            workflow_start_time: Optional ISO8601 start time

        Returns:
            Tuple of (start_time, end_time, total_duration_seconds)
        """
        # Calculate total duration from phase durations
        total_duration = sum(p.get("duration", 0) for p in phases)

        # Extract timestamps from content
        start_time_str, end_time_str = self._extract_timestamps_from_content(content)

        # Override start time if provided
        if workflow_start_time:
            start_time_str = workflow_start_time

        # Generate end time if not found
        if not end_time_str:
            end_time_str = self._calculate_end_time(start_time_str, total_duration)

        # Ensure both times are set
        start_time_str = start_time_str or datetime.utcnow().isoformat() + "Z"
        end_time_str = end_time_str or datetime.utcnow().isoformat() + "Z"

        return start_time_str, end_time_str, total_duration

    def _extract_timestamps_from_content(self, content: str) -> tuple:
        """
        Extract ISO8601 timestamps from story content.

        Searches for first and last ISO8601 timestamp in content.

        Args:
            content: Story content to search

        Returns:
            Tuple of (start_timestamp, end_timestamp) or (None, None)
        """
        times = re.findall(PATTERN_TIMESTAMP_ISO8601, content)

        start_time_str = times[0] if times else None
        end_time_str = times[-1] if len(times) > 1 else None

        return start_time_str, end_time_str

    def _calculate_end_time(self, start_time_str: Optional[str], duration: int) -> str:
        """
        Calculate end time from start time and duration.

        Parses ISO8601 start time, adds duration in seconds, returns
        resulting ISO8601 timestamp. Falls back to current time on error.

        Args:
            start_time_str: ISO8601 start timestamp or None
            duration: Duration in seconds to add

        Returns:
            ISO8601 end timestamp
        """
        if not start_time_str:
            return datetime.utcnow().isoformat() + "Z"

        try:
            start_dt = datetime.fromisoformat(start_time_str.replace("Z", "+00:00"))
            end_dt = start_dt + timedelta(seconds=duration)
            return end_dt.isoformat().replace("+00:00", "Z")
        except (ValueError, OverflowError):
            return datetime.utcnow().isoformat() + "Z"

    def _get_failed_phase(self, phases: List[Dict[str, Any]]) -> Optional[str]:
        """
        Get the first failed phase.

        Args:
            phases: List of phase dictionaries

        Returns:
            Phase identifier (development, qa, release) or None
        """
        for phase in phases:
            if phase.get("status") == STATUS_FAILED:
                return phase.get("phase")

        return None

    def _get_aborted_phases(self, phases: List[Dict[str, Any]]) -> List[str]:
        """
        Get phases that were aborted (NOT_RUN after failure).

        Phases are aborted when a previous phase fails and stops workflow.

        Args:
            phases: List of phase dictionaries

        Returns:
            List of aborted phase identifiers
        """
        aborted = []
        found_failure = False

        for phase in phases:
            if phase.get("status") == STATUS_FAILED:
                found_failure = True
            elif found_failure and phase.get("status") == STATUS_NOT_RUN:
                aborted.append(phase.get("phase"))

        return aborted

    def _extract_failure_summary(self, content: str, failed_phase: Optional[str]) -> str:
        """
        Extract failure summary from story content.

        Generates a human-readable failure message for the failed phase.
        Attempts to extract specific failure reason from content, falls back
        to generic phase failure message.

        Args:
            content: Story content to search for failure details
            failed_phase: Phase identifier that failed (or None)

        Returns:
            Failure summary message string
        """
        if not failed_phase:
            return "Unknown failure"

        # Define phase-specific failure patterns and messages
        phase_configs = {
            PHASE_QA: ("QA.*?Failed.*?:\s*(.+?)(?:\n|$)", "QA validation failed"),
            PHASE_RELEASE: ("Release.*?Failed.*?:\s*(.+?)(?:\n|$)", "Release failed"),
            PHASE_DEVELOPMENT: (
                "Development.*?Failed.*?:\s*(.+?)(?:\n|$)",
                "Development failed",
            ),
        }

        if failed_phase in phase_configs:
            pattern, default_msg = phase_configs[failed_phase]
            match = re.search(pattern, content)
            if match:
                return f"{default_msg}: {match.group(1)}"
            return default_msg

        return f"{failed_phase} phase failed"

    def _extract_qa_attempts(self, content: str) -> Optional[int]:
        """
        Extract total QA attempt count from story content.

        Counts "### QA Attempt N" sections or looks for qa_attempts field.
        This is a top-level count across all QA phases in story history.

        Args:
            content: Story content to search

        Returns:
            Total number of QA attempts or None if not found
        """
        # Count "### QA Attempt N" sections at top level
        attempts = re.findall(PATTERN_QA_ATTEMPT, content)
        if attempts:
            return len(attempts)

        # Also check for qa_attempts field at story level
        match = re.search(PATTERN_QA_ATTEMPTS_FIELD, content)
        if match:
            return int(match.group(1))

        return None

    def _create_error_context(self, story_id: str) -> Dict[str, Any]:
        """
        Create minimal error context for extraction failures.

        Returns a consistent error structure with failure status and
        no phase data. Used when context extraction encounters an exception.

        Args:
            story_id: Story identifier

        Returns:
            Minimal context with FAILURE status and empty phases
        """
        return {
            "workflow_id": str(uuid4()),
            "story_id": story_id,
            "status": WORKFLOW_FAILURE,
            "total_duration": 0,
            "start_time": datetime.utcnow().isoformat() + "Z",
            "end_time": datetime.utcnow().isoformat() + "Z",
            "phases_executed": [],
            "quality_gates": self._initialize_quality_gates(),
            "checkpoint_info": {"checkpoint_resumed": False},
            "failure_summary": "Context extraction failed",
        }


def extract_orchestrate_context(
    story_content: str,
    story_id: str,
    workflow_start_time: Optional[str] = None,
) -> Dict[str, Any]:
    """
    Public API for extracting orchestrate workflow context.

    Main entry point for extracting comprehensive workflow context from
    story files. Instantiates extractor and delegates to extract_workflow_context.

    Example:
        story_content = Path("STORY-001.story.md").read_text()
        context = extract_orchestrate_context(story_content, "STORY-001")
        print(context["status"])  # "SUCCESS" or "FAILURE"

    Args:
        story_content: Complete story file content
        story_id: Story ID (e.g., STORY-001)
        workflow_start_time: Optional ISO8601 start timestamp

    Returns:
        Dictionary with keys: workflow_id, story_id, status, total_duration,
        start_time, end_time, phases_executed, quality_gates, checkpoint_info.
        If status is FAILURE, also includes: failed_phase, failure_summary,
        phases_aborted, and optionally qa_attempts.
    """
    extractor = OrchestrateHooksContextExtractor()
    return extractor.extract_workflow_context(story_content, story_id, workflow_start_time)
