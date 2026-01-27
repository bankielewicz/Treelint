"""Test fixture: Python imports and variables for symbol extraction testing."""

import os
import sys
from pathlib import Path
from typing import Dict, List, Optional
from collections import defaultdict as dd

# Module-level constants
MAX_RETRIES: int = 3
DEFAULT_TIMEOUT: float = 30.0
API_BASE_URL = "https://api.example.com"
DEBUG = False

# Module-level variables
_cache: Dict[str, str] = {}
counter = 0

# Type alias
PathList = List[Path]


def use_imports() -> None:
    """Function that uses the imports."""
    print(os.getcwd())
    print(sys.version)
