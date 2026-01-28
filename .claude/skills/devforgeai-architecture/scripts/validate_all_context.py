#!/usr/bin/env python3
"""
Master validation script - runs all context file validations.

This script orchestrates:
1. validate_dependencies.py - Validates dependencies.md compliance
2. validate_architecture.py - Validates architecture-constraints.md compliance
3. detect_anti_patterns.py - Detects anti-patterns from anti-patterns.md

Also validates:
4. tech-stack.md exists and is properly formatted
5. source-tree.md exists and is properly formatted
6. coding-standards.md exists and is properly formatted

Exit codes:
0 - All validations passed
1 - One or more validations failed
2 - Configuration or file errors
"""

import argparse
import subprocess
import sys
from pathlib import Path
from typing import List, Tuple


class Color:
    """ANSI color codes for terminal output."""
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    BOLD = '\033[1m'
    END = '\033[0m'


class MasterValidator:
    """Orchestrates all context file validations."""

    def __init__(self, project_root: Path, context_dir: Path, scripts_dir: Path):
        self.project_root = project_root
        self.context_dir = context_dir
        self.scripts_dir = scripts_dir

        self.results: List[Tuple[str, bool, str]] = []  # (name, success, output)

    def validate_all(self) -> bool:
        """Run all validations. Returns True if all passed."""
        print(f"{Color.BOLD}{'='*70}{Color.END}")
        print(f"{Color.BOLD}DevForgeAI Context Validation Suite{Color.END}")
        print(f"{Color.BOLD}{'='*70}{Color.END}\n")

        print(f"Project root: {self.project_root}")
        print(f"Context dir: {self.context_dir}")
        print(f"Scripts dir: {self.scripts_dir}\n")

        # Step 1: Validate context files exist
        print(f"{Color.CYAN}{Color.BOLD}Step 1: Validating Context Files Exist{Color.END}")
        print(f"{Color.CYAN}{'─'*70}{Color.END}\n")

        context_files_valid = self._validate_context_files_exist()

        if not context_files_valid:
            print(f"\n{Color.RED}{Color.BOLD}❌ CONTEXT FILES MISSING - Cannot proceed with validation{Color.END}")
            return False

        # Step 2: Run validate_dependencies.py
        print(f"\n{Color.CYAN}{Color.BOLD}Step 2: Validating Dependencies{Color.END}")
        print(f"{Color.CYAN}{'─'*70}{Color.END}\n")

        deps_success, deps_output = self._run_script("validate_dependencies.py")
        self.results.append(("Dependencies Validation", deps_success, deps_output))

        # Step 3: Run validate_architecture.py
        print(f"\n{Color.CYAN}{Color.BOLD}Step 3: Validating Architecture{Color.END}")
        print(f"{Color.CYAN}{'─'*70}{Color.END}\n")

        arch_success, arch_output = self._run_script("validate_architecture.py")
        self.results.append(("Architecture Validation", arch_success, arch_output))

        # Step 4: Run detect_anti_patterns.py
        print(f"\n{Color.CYAN}{Color.BOLD}Step 4: Detecting Anti-Patterns{Color.END}")
        print(f"{Color.CYAN}{'─'*70}{Color.END}\n")

        anti_success, anti_output = self._run_script("detect_anti_patterns.py")
        self.results.append(("Anti-Pattern Detection", anti_success, anti_output))

        # Step 5: Summary
        self._print_summary()

        # Overall success
        all_success = all(success for _, success, _ in self.results)
        return all_success

    def _validate_context_files_exist(self) -> bool:
        """Validate that all 6 context files exist."""
        required_files = [
            ("tech-stack.md", "Technology stack and locked choices"),
            ("source-tree.md", "Project structure and file organization"),
            ("dependencies.md", "Approved packages and versions"),
            ("coding-standards.md", "Project-specific code patterns"),
            ("architecture-constraints.md", "Layer boundaries and design rules"),
            ("anti-patterns.md", "Forbidden patterns and practices"),
        ]

        all_exist = True

        for filename, description in required_files:
            filepath = self.context_dir / filename

            if filepath.exists():
                # Check file is not empty
                size = filepath.stat().st_size
                if size == 0:
                    print(f"  {Color.RED}✗ {filename} - EXISTS but EMPTY{Color.END}")
                    all_exist = False
                elif size < 500:
                    print(f"  {Color.YELLOW}⚠ {filename} - EXISTS but very small ({size} bytes){Color.END}")
                else:
                    print(f"  {Color.GREEN}✓ {filename} - {description}{Color.END}")
            else:
                print(f"  {Color.RED}✗ {filename} - MISSING{Color.END}")
                all_exist = False

        if all_exist:
            print(f"\n{Color.GREEN}{Color.BOLD}✅ All context files exist{Color.END}")
        else:
            print(f"\n{Color.RED}{Color.BOLD}❌ Some context files are missing or empty{Color.END}")
            print(f"\n{Color.YELLOW}To create context files, run:{Color.END}")
            print(f"  {self.scripts_dir / 'init_context.sh'}")

        return all_exist

    def _run_script(self, script_name: str) -> Tuple[bool, str]:
        """Run a validation script and capture output."""
        script_path = self.scripts_dir / script_name

        if not script_path.exists():
            error_msg = f"Script not found: {script_path}"
            print(f"{Color.RED}{error_msg}{Color.END}")
            return False, error_msg

        try:
            # Run script with same arguments
            result = subprocess.run(
                [
                    sys.executable,
                    str(script_path),
                    "--project-root", str(self.project_root),
                    "--context-dir", str(self.context_dir),
                ],
                capture_output=True,
                text=True,
                timeout=60  # 60 second timeout
            )

            # Print output in real-time
            print(result.stdout)
            if result.stderr:
                print(result.stderr, file=sys.stderr)

            success = result.returncode == 0
            return success, result.stdout

        except subprocess.TimeoutExpired:
            error_msg = f"Script timed out: {script_name}"
            print(f"{Color.RED}{error_msg}{Color.END}")
            return False, error_msg

        except Exception as e:
            error_msg = f"Error running {script_name}: {e}"
            print(f"{Color.RED}{error_msg}{Color.END}")
            return False, error_msg

    def _print_summary(self):
        """Print summary of all validation results."""
        print(f"\n{Color.BOLD}{'='*70}{Color.END}")
        print(f"{Color.BOLD}Validation Summary{Color.END}")
        print(f"{Color.BOLD}{'='*70}{Color.END}\n")

        passed = sum(1 for _, success, _ in self.results if success)
        failed = len(self.results) - passed

        for name, success, _ in self.results:
            status = f"{Color.GREEN}✓ PASS{Color.END}" if success else f"{Color.RED}✗ FAIL{Color.END}"
            print(f"  {status}  {name}")

        print(f"\n{Color.BOLD}Results:{Color.END}")
        print(f"  Passed: {Color.GREEN}{passed}{Color.END}")
        print(f"  Failed: {Color.RED if failed > 0 else Color.GREEN}{failed}{Color.END}")

        if failed == 0:
            print(f"\n{Color.GREEN}{Color.BOLD}{'='*70}")
            print(f"✅ ALL VALIDATIONS PASSED")
            print(f"{'='*70}{Color.END}")
        else:
            print(f"\n{Color.RED}{Color.BOLD}{'='*70}")
            print(f"❌ {failed} VALIDATION(S) FAILED")
            print(f"{'='*70}{Color.END}")

            print(f"\n{Color.YELLOW}Review the output above for details on failures.{Color.END}")


def main():
    parser = argparse.ArgumentParser(
        description="Master validation script - runs all context file validations"
    )
    parser.add_argument(
        "--project-root",
        type=Path,
        default=Path.cwd(),
        help="Root directory of the project (default: current directory)"
    )
    parser.add_argument(
        "--context-dir",
        type=Path,
        default=Path.cwd() / "devforgeai" / "context",
        help="Directory containing context files (default: devforgeai/context)"
    )
    parser.add_argument(
        "--scripts-dir",
        type=Path,
        help="Directory containing validation scripts (default: same dir as this script)"
    )

    args = parser.parse_args()

    # Default scripts-dir to the directory containing this script
    if args.scripts_dir is None:
        args.scripts_dir = Path(__file__).parent

    validator = MasterValidator(args.project_root, args.context_dir, args.scripts_dir)

    try:
        success = validator.validate_all()
        sys.exit(0 if success else 1)
    except KeyboardInterrupt:
        print(f"\n{Color.YELLOW}Validation interrupted by user{Color.END}")
        sys.exit(130)
    except Exception as e:
        print(f"{Color.RED}FATAL ERROR: {e}{Color.END}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(2)


if __name__ == "__main__":
    main()
