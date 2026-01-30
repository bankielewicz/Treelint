#!/usr/bin/env python3
"""
Ensure UI Spec Directory Script

This script creates the devforgeai/specs/ui/ directory if it doesn't exist.
Part of the DevForgeAI UI Generator skill.

Usage:
    python ensure_spec_dir.py
"""

import os
import sys
from pathlib import Path


def ensure_spec_dir():
    """Create the UI spec directory if it doesn't exist."""
    # Determine project root (3 levels up from script location)
    script_dir = Path(__file__).resolve().parent
    project_root = script_dir.parent.parent.parent.parent

    # Define UI spec directory path
    ui_spec_dir = project_root / "devforgeai" / "specs" / "ui"

    try:
        # Create directory with parents, don't error if exists
        ui_spec_dir.mkdir(parents=True, exist_ok=True)
        print(f"✅ UI spec directory ready: {ui_spec_dir}")
        return 0
    except PermissionError:
        print(f"❌ Permission denied: Cannot create {ui_spec_dir}", file=sys.stderr)
        print("   Please check directory permissions and try again.", file=sys.stderr)
        return 1
    except Exception as e:
        print(f"❌ Unexpected error creating directory: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(ensure_spec_dir())
