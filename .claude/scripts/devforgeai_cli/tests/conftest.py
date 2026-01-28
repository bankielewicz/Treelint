"""
Pytest configuration and shared fixtures for devforgeai_cli tests.

Provides common fixtures for all test modules:
- Environment setup
- Temporary directories
- Mock utilities
"""

import os
from pathlib import Path

import pytest


@pytest.fixture(scope="session")
def project_root():
    """Get the project root directory."""
    return Path(__file__).parent.parent.parent.parent.parent.parent


@pytest.fixture
def monkeypatch_devforgeai_home(monkeypatch, tmp_path):
    """Set up a temporary DevForgeAI home directory for testing."""
    devforgeai_home = tmp_path / "devforgeai"
    devforgeai_home.mkdir(parents=True, exist_ok=True)

    monkeypatch.setenv("DEVFORGEAI_HOME", str(devforgeai_home))
    yield devforgeai_home
