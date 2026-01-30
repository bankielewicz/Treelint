#!/usr/bin/env python3
"""
Tests for DoD Validator

Tests the core autonomous deferral detection logic.
"""

import pytest
from pathlib import Path

from devforgeai_cli.validators.dod_validator import DoDValidator


# Test fixtures directory
FIXTURES_DIR = Path(__file__).parent / 'fixtures'


class TestDoDValidator:
    """Test DoD validation logic."""

    def setup_method(self):
        """Setup test validator."""
        # Use parent directory as project root for testing
        self.validator = DoDValidator(project_root=FIXTURES_DIR.parent.parent.parent.parent)

    def test_valid_story_passes(self):
        """Test that story with all DoD complete passes validation."""
        story_file = FIXTURES_DIR / 'valid-story-complete.md'

        is_valid, violations = self.validator.validate(str(story_file))

        assert is_valid == True, f"Expected valid, got violations: {violations}"
        assert len(violations) == 0, "Expected no violations"

    def test_autonomous_deferral_detected(self):
        """Test detection of DoD [x] but Impl [ ] without user approval."""
        story_file = FIXTURES_DIR / 'autonomous-deferral-story.md'

        is_valid, violations = self.validator.validate(str(story_file))

        assert is_valid == False, "Expected validation to fail for autonomous deferral"

        # Should have CRITICAL violation
        critical_violations = [v for v in violations if v['severity'] == 'CRITICAL']
        assert len(critical_violations) > 0, "Expected CRITICAL violation"

        # Check violation details
        deferral_violation = next(
            (v for v in critical_violations if 'AUTONOMOUS DEFERRAL' in v['error']),
            None
        )
        assert deferral_violation is not None, "Expected autonomous deferral violation"
        assert deferral_violation['violation_type'] == 'autonomous_deferral'
        assert 'Deadlock retry' in deferral_violation['item']

    def test_valid_deferral_with_user_approval_passes(self):
        """Test that deferral with 'User approved:' marker passes."""
        story_file = FIXTURES_DIR / 'valid-deferral-story.md'

        is_valid, violations = self.validator.validate(str(story_file))

        # Should pass (no CRITICAL/HIGH violations)
        critical_high = [v for v in violations if v['severity'] in ['CRITICAL', 'HIGH']]
        assert len(critical_high) == 0, f"Expected no critical/high violations, got: {critical_high}"

    def test_missing_implementation_notes_fails(self):
        """Test that story without Implementation Notes fails."""
        story_file = FIXTURES_DIR / 'missing-impl-notes.md'

        is_valid, violations = self.validator.validate(str(story_file))

        assert is_valid == False, "Expected validation to fail"

        # Should have HIGH violation for missing section
        high_violations = [v for v in violations if v['severity'] == 'HIGH']
        assert len(high_violations) > 0, "Expected HIGH violation"

        impl_notes_violation = next(
            (v for v in high_violations if 'Implementation Notes' in v['error']),
            None
        )
        assert impl_notes_violation is not None, "Expected Implementation Notes violation"

    def test_story_file_not_found(self):
        """Test handling of non-existent story file."""
        story_file = FIXTURES_DIR / 'nonexistent.md'

        is_valid, violations = self.validator.validate(str(story_file))

        assert is_valid == False, "Expected validation to fail for missing file"
        assert len(violations) > 0, "Expected violations"
        assert violations[0]['severity'] == 'CRITICAL'


# Run tests if executed directly
if __name__ == '__main__':
    pytest.main([__file__, '-v'])
