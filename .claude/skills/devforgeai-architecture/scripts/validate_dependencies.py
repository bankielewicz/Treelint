#!/usr/bin/env python3
"""
Validate dependencies.md against actual project dependencies.

This script checks:
1. All packages in project files are documented in dependencies.md
2. All documented packages exist in project files
3. Package versions match between dependencies.md and project files
4. No FORBIDDEN packages are present in project
5. LOCKED packages are not substituted

Exit codes:
0 - All validations passed
1 - Validation errors found
2 - Configuration or file errors
"""

import argparse
import re
import sys
from pathlib import Path
from typing import Dict, List, Set, Tuple
import xml.etree.ElementTree as ET


class Color:
    """ANSI color codes for terminal output."""
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    BOLD = '\033[1m'
    END = '\033[0m'


class DependencyValidator:
    """Validates project dependencies against dependencies.md."""

    def __init__(self, project_root: Path, context_dir: Path):
        self.project_root = project_root
        self.context_dir = context_dir
        self.dependencies_md = context_dir / "dependencies.md"
        self.errors: List[str] = []
        self.warnings: List[str] = []

    def validate(self) -> bool:
        """Run all validation checks. Returns True if all passed."""
        print(f"{Color.BOLD}Validating Dependencies{Color.END}")
        print(f"Project root: {self.project_root}")
        print(f"Context dir: {self.context_dir}\n")

        if not self.dependencies_md.exists():
            self.errors.append(f"dependencies.md not found at {self.dependencies_md}")
            return False

        # Parse dependencies.md
        approved_packages, locked_packages, forbidden_packages = self._parse_dependencies_md()

        # Check .NET dependencies (NuGet)
        dotnet_packages = self._extract_dotnet_packages()
        if dotnet_packages:
            print(f"{Color.CYAN}Validating .NET NuGet packages...{Color.END}")
            self._validate_dotnet_packages(dotnet_packages, approved_packages, locked_packages, forbidden_packages)

        # Check npm dependencies
        npm_packages = self._extract_npm_packages()
        if npm_packages:
            print(f"\n{Color.CYAN}Validating npm packages...{Color.END}")
            self._validate_npm_packages(npm_packages, approved_packages, locked_packages, forbidden_packages)

        # Report results
        self._report_results()

        return len(self.errors) == 0

    def _parse_dependencies_md(self) -> Tuple[Dict[str, str], Set[str], Set[str]]:
        """
        Parse dependencies.md to extract approved, locked, and forbidden packages.

        Returns:
            - approved_packages: {package_name: version_constraint}
            - locked_packages: {package_name} (packages marked as LOCKED)
            - forbidden_packages: {package_name} (packages in FORBIDDEN comments)
        """
        approved: Dict[str, str] = {}
        locked: Set[str] = set()
        forbidden: Set[str] = set()

        content = self.dependencies_md.read_text()

        # Extract NuGet packages
        nuget_pattern = r'<PackageReference Include="([^"]+)" Version="([^"]+)"'
        for match in re.finditer(nuget_pattern, content):
            package_name = match.group(1)
            version = match.group(2)
            approved[package_name] = version

            # Check if package is in a LOCKED section
            # Look for "LOCKED" keyword within 200 characters before the package
            start = max(0, match.start() - 200)
            context = content[start:match.start()]
            if "LOCKED" in context or "CRITICAL" in context:
                locked.add(package_name)

        # Extract npm packages
        npm_pattern = r'"([^"]+)":\s*"([^"]+)"'
        in_npm_section = False
        for line in content.split('\n'):
            if '"dependencies"' in line or '"devDependencies"' in line:
                in_npm_section = True
            elif in_npm_section and '}' in line:
                in_npm_section = False
            elif in_npm_section:
                match = re.search(npm_pattern, line)
                if match:
                    package_name = match.group(1)
                    version = match.group(2)
                    approved[package_name] = version

                    # Check for LOCKED packages in npm section
                    if "LOCKED" in line or "CRITICAL" in line:
                        locked.add(package_name)

        # Extract FORBIDDEN packages (in comments)
        forbidden_nuget_pattern = r'<!--\s*<PackageReference Include="([^"]+)"'
        for match in re.finditer(forbidden_nuget_pattern, content):
            forbidden.add(match.group(1))

        forbidden_npm_pattern = r'//\s*"([^"]+)":\s*"\*"'
        for match in re.finditer(forbidden_npm_pattern, content):
            forbidden.add(match.group(1))

        print(f"Parsed dependencies.md:")
        print(f"  Approved packages: {len(approved)}")
        print(f"  Locked packages: {len(locked)}")
        print(f"  Forbidden packages: {len(forbidden)}")

        return approved, locked, forbidden

    def _extract_dotnet_packages(self) -> Dict[str, str]:
        """Extract all NuGet packages from .csproj files."""
        packages: Dict[str, str] = {}

        csproj_files = list(self.project_root.glob("**/*.csproj"))

        for csproj_file in csproj_files:
            try:
                tree = ET.parse(csproj_file)
                root = tree.getroot()

                for pkg_ref in root.findall(".//PackageReference"):
                    package_name = pkg_ref.get("Include")
                    version = pkg_ref.get("Version")

                    if package_name and version:
                        packages[package_name] = version
            except ET.ParseError as e:
                self.warnings.append(f"Failed to parse {csproj_file}: {e}")

        return packages

    def _extract_npm_packages(self) -> Dict[str, str]:
        """Extract all npm packages from package.json."""
        packages: Dict[str, str] = {}

        package_json = self.project_root / "package.json"

        if not package_json.exists():
            return packages

        import json
        try:
            data = json.loads(package_json.read_text())

            # Extract dependencies
            if "dependencies" in data:
                packages.update(data["dependencies"])

            # Extract devDependencies
            if "devDependencies" in data:
                packages.update(data["devDependencies"])

        except json.JSONDecodeError as e:
            self.warnings.append(f"Failed to parse package.json: {e}")

        return packages

    def _validate_dotnet_packages(
        self,
        actual_packages: Dict[str, str],
        approved_packages: Dict[str, str],
        locked_packages: Set[str],
        forbidden_packages: Set[str]
    ):
        """Validate .NET NuGet packages against dependencies.md."""

        # Check for unapproved packages
        for package_name, version in actual_packages.items():
            if package_name not in approved_packages:
                self.errors.append(
                    f"❌ UNAPPROVED PACKAGE: {package_name} {version} is in .csproj but not in dependencies.md"
                )

        # Check for forbidden packages
        for package_name in actual_packages:
            if package_name in forbidden_packages:
                self.errors.append(
                    f"❌ FORBIDDEN PACKAGE: {package_name} is explicitly forbidden in dependencies.md"
                )

        # Check for missing approved packages
        for package_name in approved_packages:
            # Skip npm packages (they won't be in .csproj)
            if package_name.startswith("@") or "/" in package_name or "-" in package_name and not "." in package_name:
                continue

            if package_name not in actual_packages and package_name in locked_packages:
                self.warnings.append(
                    f"⚠️ LOCKED PACKAGE MISSING: {package_name} is in dependencies.md but not in project files"
                )

        # Check version compliance
        for package_name, actual_version in actual_packages.items():
            if package_name in approved_packages:
                approved_version = approved_packages[package_name]
                if not self._version_matches(actual_version, approved_version):
                    severity = "ERROR" if package_name in locked_packages else "WARNING"
                    message = f"{'❌' if severity == 'ERROR' else '⚠️'} VERSION MISMATCH: {package_name} is {actual_version} but dependencies.md specifies {approved_version}"

                    if severity == "ERROR":
                        self.errors.append(message)
                    else:
                        self.warnings.append(message)

    def _validate_npm_packages(
        self,
        actual_packages: Dict[str, str],
        approved_packages: Dict[str, str],
        locked_packages: Set[str],
        forbidden_packages: Set[str]
    ):
        """Validate npm packages against dependencies.md."""

        # Check for unapproved packages
        for package_name, version in actual_packages.items():
            if package_name not in approved_packages:
                self.errors.append(
                    f"❌ UNAPPROVED PACKAGE: {package_name} {version} is in package.json but not in dependencies.md"
                )

        # Check for forbidden packages
        for package_name in actual_packages:
            if package_name in forbidden_packages:
                self.errors.append(
                    f"❌ FORBIDDEN PACKAGE: {package_name} is explicitly forbidden in dependencies.md"
                )

        # Check for missing locked packages
        for package_name in locked_packages:
            # Only check npm-style packages
            if package_name.startswith("@") or ("/" not in package_name and "-" in package_name):
                if package_name not in actual_packages:
                    self.warnings.append(
                        f"⚠️ LOCKED PACKAGE MISSING: {package_name} is in dependencies.md but not in package.json"
                    )

        # Check version compliance
        for package_name, actual_version in actual_packages.items():
            if package_name in approved_packages:
                approved_version = approved_packages[package_name]
                if not self._version_matches(actual_version, approved_version):
                    severity = "ERROR" if package_name in locked_packages else "WARNING"
                    message = f"{'❌' if severity == 'ERROR' else '⚠️'} VERSION MISMATCH: {package_name} is {actual_version} but dependencies.md specifies {approved_version}"

                    if severity == "ERROR":
                        self.errors.append(message)
                    else:
                        self.warnings.append(message)

    def _version_matches(self, actual: str, approved: str) -> bool:
        """
        Check if actual version matches approved version constraint.

        Supports:
        - Exact: "1.2.3" matches "1.2.3"
        - Caret: "1.2.3" matches "^1.2.0" (compatible with 1.x.x)
        - Tilde: "1.2.3" matches "~1.2.0" (compatible with 1.2.x)
        - Wildcard: "1.2.3" matches "*" or "[VERSION]"
        """
        # Remove leading ^, ~, or >= from versions
        actual_clean = actual.lstrip("^~>=")
        approved_clean = approved.lstrip("^~>=")

        # Wildcard match
        if approved in ["*", "[VERSION]"]:
            return True

        # Exact match
        if actual_clean == approved_clean:
            return True

        # Caret (^) - major version must match
        if approved.startswith("^"):
            actual_parts = actual_clean.split(".")
            approved_parts = approved_clean.split(".")
            return actual_parts[0] == approved_parts[0]

        # Tilde (~) - major and minor must match
        if approved.startswith("~"):
            actual_parts = actual_clean.split(".")
            approved_parts = approved_clean.split(".")
            return actual_parts[0:2] == approved_parts[0:2]

        return False

    def _report_results(self):
        """Print validation results summary."""
        print(f"\n{Color.BOLD}{'='*60}{Color.END}")
        print(f"{Color.BOLD}Validation Results{Color.END}")
        print(f"{Color.BOLD}{'='*60}{Color.END}\n")

        if self.warnings:
            print(f"{Color.YELLOW}Warnings ({len(self.warnings)}):{Color.END}")
            for warning in self.warnings:
                print(f"  {warning}")
            print()

        if self.errors:
            print(f"{Color.RED}Errors ({len(self.errors)}):{Color.END}")
            for error in self.errors:
                print(f"  {error}")
            print()
            print(f"{Color.RED}{Color.BOLD}❌ VALIDATION FAILED{Color.END}")
        else:
            print(f"{Color.GREEN}{Color.BOLD}✅ ALL VALIDATIONS PASSED{Color.END}")


def main():
    parser = argparse.ArgumentParser(
        description="Validate project dependencies against dependencies.md"
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

    args = parser.parse_args()

    validator = DependencyValidator(args.project_root, args.context_dir)

    try:
        success = validator.validate()
        sys.exit(0 if success else 1)
    except Exception as e:
        print(f"{Color.RED}FATAL ERROR: {e}{Color.END}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(2)


if __name__ == "__main__":
    main()
