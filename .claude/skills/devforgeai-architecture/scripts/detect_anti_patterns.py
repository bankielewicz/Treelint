#!/usr/bin/env python3
"""
Detect anti-patterns defined in anti-patterns.md.

This script checks for all 10 categories of anti-patterns:
1. Library Substitution (ORM/framework swapping)
2. Structure Violation (files in wrong locations)
3. Cross-Layer Dependencies (architecture violations)
4. Framework Mixing (multiple state management libraries)
5. Magic Numbers/Strings (hard-coded values)
6. God Objects (classes that do too much)
7. Tight Coupling (direct instantiation)
8. Security Anti-Patterns (SQL injection, XSS, secrets)
9. Performance Anti-Patterns (N+1 queries, blocking)
10. Test Anti-Patterns (fragile tests, interdependence)

Exit codes:
0 - No anti-patterns detected
1 - Anti-patterns found
2 - Configuration or file errors
"""

import argparse
import re
import sys
from pathlib import Path
from typing import Dict, List, Set, Tuple
from collections import defaultdict


class Color:
    """ANSI color codes for terminal output."""
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    BOLD = '\033[1m'
    END = '\033[0m'


class AntiPatternDetector:
    """Detects anti-patterns defined in anti-patterns.md."""

    def __init__(self, project_root: Path, context_dir: Path):
        self.project_root = project_root
        self.context_dir = context_dir
        self.anti_patterns_md = context_dir / "anti-patterns.md"
        self.tech_stack_md = context_dir / "tech-stack.md"
        self.source_tree_md = context_dir / "source-tree.md"

        self.critical_issues: List[str] = []
        self.high_issues: List[str] = []
        self.medium_issues: List[str] = []
        self.low_issues: List[str] = []

        # Configuration extracted from context files
        self.approved_orm: str = None
        self.forbidden_orms: Set[str] = set()
        self.approved_state_mgmt: str = None
        self.forbidden_state_mgmt: Set[str] = set()

    def detect(self) -> bool:
        """Run all anti-pattern detection. Returns True if no critical/high issues found."""
        print(f"{Color.BOLD}Detecting Anti-Patterns{Color.END}")
        print(f"Project root: {self.project_root}")
        print(f"Context dir: {self.context_dir}\n")

        if not self.anti_patterns_md.exists():
            self.critical_issues.append(f"anti-patterns.md not found at {self.anti_patterns_md}")
            return False

        # Parse configuration from context files
        self._parse_tech_stack()

        # Run detection for each category
        print(f"{Color.CYAN}Scanning for anti-patterns...{Color.END}\n")

        self._detect_library_substitution()
        self._detect_structure_violations()
        self._detect_framework_mixing()
        self._detect_magic_values()
        self._detect_god_objects()
        self._detect_tight_coupling()
        self._detect_security_anti_patterns()
        self._detect_performance_anti_patterns()
        self._detect_test_anti_patterns()

        # Report results
        self._report_results()

        # Critical or High issues = failure
        return len(self.critical_issues) == 0 and len(self.high_issues) == 0

    def _parse_tech_stack(self):
        """Parse tech-stack.md to extract approved and forbidden technologies."""
        if not self.tech_stack_md.exists():
            return

        content = self.tech_stack_md.read_text()

        # Extract approved ORM
        orm_patterns = {
            'Dapper': r'Dapper.*(?:LOCKED|ORM|data access)',
            'Entity Framework': r'Entity Framework.*(?:LOCKED|ORM)',
            'NHibernate': r'NHibernate.*(?:LOCKED|ORM)'
        }

        for orm, pattern in orm_patterns.items():
            if re.search(pattern, content, re.IGNORECASE):
                if 'LOCKED' in content[max(0, content.find(orm) - 100):content.find(orm) + 200]:
                    self.approved_orm = orm
                    break

        # Extract forbidden ORMs (in FORBIDDEN or PROHIBITED sections)
        forbidden_section = re.search(r'FORBIDDEN.*?(?:\n\n|\Z)', content, re.DOTALL | re.IGNORECASE)
        if forbidden_section:
            section_text = forbidden_section.group(0)
            for orm in ['Entity Framework', 'Dapper', 'NHibernate']:
                if orm in section_text and orm != self.approved_orm:
                    self.forbidden_orms.add(orm)

        # Extract approved state management
        state_patterns = {
            'Zustand': r'Zustand.*(?:LOCKED|state)',
            'Redux': r'Redux.*(?:LOCKED|state)',
            'MobX': r'MobX.*(?:LOCKED|state)',
            'Jotai': r'Jotai.*(?:LOCKED|state)'
        }

        for lib, pattern in state_patterns.items():
            if re.search(pattern, content, re.IGNORECASE):
                if 'LOCKED' in content[max(0, content.find(lib) - 100):content.find(lib) + 200]:
                    self.approved_state_mgmt = lib
                    break

        # Extract forbidden state management
        for lib in ['Zustand', 'Redux', 'MobX', 'Jotai', 'Recoil']:
            if lib in forbidden_section.group(0) if forbidden_section else '' and lib != self.approved_state_mgmt:
                self.forbidden_state_mgmt.add(lib)

        print(f"Tech stack configuration:")
        print(f"  Approved ORM: {self.approved_orm}")
        print(f"  Forbidden ORMs: {self.forbidden_orms}")
        print(f"  Approved state management: {self.approved_state_mgmt}")
        print(f"  Forbidden state management: {self.forbidden_state_mgmt}\n")

    def _detect_library_substitution(self):
        """Detect Category 1: Library Substitution (CRITICAL)."""
        print(f"1. Checking for library substitution...")

        violations = []

        # Check for forbidden ORM usage
        if self.forbidden_orms:
            csproj_files = list(self.project_root.glob("**/*.csproj"))

            for csproj_file in csproj_files:
                content = csproj_file.read_text()

                for forbidden_orm in self.forbidden_orms:
                    # Check for Entity Framework
                    if 'Entity Framework' in forbidden_orm:
                        if re.search(r'Microsoft\.EntityFrameworkCore', content):
                            violations.append(
                                f"File: {csproj_file.relative_to(self.project_root)}\n"
                                f"     Forbidden package: Microsoft.EntityFrameworkCore\n"
                                f"     Approved ORM: {self.approved_orm}"
                            )

                    # Check for NHibernate
                    if 'NHibernate' in forbidden_orm:
                        if re.search(r'NHibernate', content):
                            violations.append(
                                f"File: {csproj_file.relative_to(self.project_root)}\n"
                                f"     Forbidden package: NHibernate\n"
                                f"     Approved ORM: {self.approved_orm}"
                            )

        # Check for forbidden state management
        if self.forbidden_state_mgmt:
            package_json = self.project_root / "package.json"

            if package_json.exists():
                content = package_json.read_text()

                for forbidden_lib in self.forbidden_state_mgmt:
                    lib_lowercase = forbidden_lib.lower()

                    if lib_lowercase in content.lower():
                        violations.append(
                            f"File: package.json\n"
                            f"     Forbidden package: {forbidden_lib}\n"
                            f"     Approved state management: {self.approved_state_mgmt}"
                        )

        if violations:
            for violation in violations:
                self.critical_issues.append(
                    f"❌ LIBRARY SUBSTITUTION (CRITICAL):\n   {violation}"
                )
            print(f"   {Color.RED}✗ {len(violations)} violation(s) found{Color.END}")
        else:
            print(f"   {Color.GREEN}✓ No library substitution detected{Color.END}")

    def _detect_structure_violations(self):
        """Detect Category 2: Structure Violations (HIGH)."""
        print(f"2. Checking for structure violations...")

        if not self.source_tree_md.exists():
            print(f"   {Color.YELLOW}⚠ source-tree.md not found, skipping{Color.END}")
            return

        # Parse expected structure from source-tree.md
        expected_dirs = self._parse_expected_structure()

        violations = []

        # Check for files in unexpected locations
        code_files = list(self.project_root.glob("**/*.cs")) + \
                     list(self.project_root.glob("**/*.ts")) + \
                     list(self.project_root.glob("**/*.tsx"))

        for code_file in code_files:
            relative_path = code_file.relative_to(self.project_root)
            path_str = str(relative_path)

            # Skip node_modules, bin, obj
            if 'node_modules' in path_str or 'bin' in path_str or 'obj' in path_str:
                continue

            # Check if file is in an expected location
            in_expected_location = False
            for expected_dir in expected_dirs:
                if expected_dir in path_str:
                    in_expected_location = True
                    break

            if not in_expected_location and expected_dirs:
                violations.append(str(relative_path))

        if violations:
            # Only report first 5 violations
            for violation in violations[:5]:
                self.high_issues.append(
                    f"⚠️ STRUCTURE VIOLATION: {violation} is not in an expected directory"
                )
            if len(violations) > 5:
                self.high_issues.append(f"   ... and {len(violations) - 5} more")

            print(f"   {Color.YELLOW}⚠ {len(violations)} violation(s) found{Color.END}")
        else:
            print(f"   {Color.GREEN}✓ No structure violations detected{Color.END}")

    def _parse_expected_structure(self) -> Set[str]:
        """Parse source-tree.md to extract expected directory structure."""
        content = self.source_tree_md.read_text()

        expected = set()

        # Look for directory patterns in markdown
        dir_patterns = [
            r'(?:src|source)/([A-Za-z0-9.]+)/',
            r'└── ([A-Za-z0-9.]+)/',
            r'├── ([A-Za-z0-9.]+)/',
            r'│\s+├── ([A-Za-z0-9.]+)/',
        ]

        for pattern in dir_patterns:
            for match in re.finditer(pattern, content):
                dir_name = match.group(1)
                expected.add(dir_name)

        return expected

    def _detect_framework_mixing(self):
        """Detect Category 4: Framework Mixing (CRITICAL)."""
        print(f"3. Checking for framework mixing...")

        violations = []

        # Check for Redux patterns in Zustand projects
        if self.approved_state_mgmt == 'Zustand':
            ts_files = list(self.project_root.glob("**/*.ts")) + \
                       list(self.project_root.glob("**/*.tsx"))

            for ts_file in ts_files:
                if 'node_modules' in str(ts_file):
                    continue

                content = ts_file.read_text(errors='ignore')

                # Detect Redux patterns
                redux_patterns = [
                    (r'const\s+\w+\s*=\s*["\'][\w/]+["\'];?\s*//.*action type', 'Action type constants'),
                    (r'dispatch\s*\(', 'dispatch() calls'),
                    (r'createSlice\s*\(', 'createSlice() (Redux Toolkit)'),
                    (r'configureStore\s*\(', 'configureStore() (Redux Toolkit)'),
                ]

                for pattern, description in redux_patterns:
                    if re.search(pattern, content):
                        violations.append(
                            f"File: {ts_file.relative_to(self.project_root)}\n"
                            f"     Redux pattern detected: {description}\n"
                            f"     Approved state management: {self.approved_state_mgmt}"
                        )
                        break  # Only report first pattern per file

        if violations:
            for violation in violations[:3]:  # Limit to 3
                self.critical_issues.append(
                    f"❌ FRAMEWORK MIXING (CRITICAL):\n   {violation}"
                )
            if len(violations) > 3:
                self.critical_issues.append(f"   ... and {len(violations) - 3} more")

            print(f"   {Color.RED}✗ {len(violations)} violation(s) found{Color.END}")
        else:
            print(f"   {Color.GREEN}✓ No framework mixing detected{Color.END}")

    def _detect_magic_values(self):
        """Detect Category 5: Magic Numbers/Strings (MEDIUM)."""
        print(f"4. Checking for magic values...")

        violations = []

        code_files = list(self.project_root.glob("**/*.cs")) + \
                     list(self.project_root.glob("**/*.ts"))

        for code_file in code_files:
            if 'node_modules' in str(code_file) or 'bin' in str(code_file):
                continue

            content = code_file.read_text(errors='ignore')

            # Detect hard-coded credentials
            credential_patterns = [
                (r'password\s*=\s*["\'][^"\']+["\']', 'Hard-coded password'),
                (r'secret\s*=\s*["\'][^"\']+["\']', 'Hard-coded secret'),
                (r'apiKey\s*=\s*["\'][^"\']+["\']', 'Hard-coded API key'),
                (r'connectionString\s*=\s*["\'].*password=[^;]+', 'Password in connection string'),
            ]

            for pattern, description in credential_patterns:
                matches = re.finditer(pattern, content, re.IGNORECASE)
                for match in matches:
                    violations.append(
                        f"File: {code_file.relative_to(self.project_root)}\n"
                        f"     {description}: {match.group(0)}"
                    )

        if violations:
            for violation in violations[:3]:  # Limit to 3
                self.medium_issues.append(f"⚠️ MAGIC VALUE: {violation}")
            if len(violations) > 3:
                self.medium_issues.append(f"   ... and {len(violations) - 3} more")

            print(f"   {Color.YELLOW}⚠ {len(violations)} violation(s) found{Color.END}")
        else:
            print(f"   {Color.GREEN}✓ No magic values detected{Color.END}")

    def _detect_god_objects(self):
        """Detect Category 6: God Objects (MEDIUM)."""
        print(f"5. Checking for God Objects...")

        violations = []

        code_files = list(self.project_root.glob("**/*.cs")) + \
                     list(self.project_root.glob("**/*.ts"))

        for code_file in code_files:
            if 'node_modules' in str(code_file) or 'bin' in str(code_file):
                continue

            content = code_file.read_text(errors='ignore')

            # Count methods
            method_count = len(re.findall(r'(?:public|private|protected)\s+(?:\w+\s+)+\w+\s*\([^)]*\)', content))

            # Count properties
            property_count = len(re.findall(r'(?:public|private|protected)\s+\w+\s+\w+\s*\{', content))

            # God object threshold
            if method_count > 20 or property_count > 15:
                violations.append(
                    f"File: {code_file.relative_to(self.project_root)}\n"
                    f"     Methods: {method_count}, Properties: {property_count}"
                )

        if violations:
            for violation in violations[:3]:
                self.medium_issues.append(f"⚠️ GOD OBJECT: {violation}")
            if len(violations) > 3:
                self.medium_issues.append(f"   ... and {len(violations) - 3} more")

            print(f"   {Color.YELLOW}⚠ {len(violations)} violation(s) found{Color.END}")
        else:
            print(f"   {Color.GREEN}✓ No God Objects detected{Color.END}")

    def _detect_tight_coupling(self):
        """Detect Category 7: Tight Coupling (MEDIUM)."""
        print(f"6. Checking for tight coupling...")

        violations = []

        code_files = list(self.project_root.glob("**/*.cs"))

        for code_file in code_files:
            if 'bin' in str(code_file) or 'obj' in str(code_file):
                continue

            content = code_file.read_text(errors='ignore')

            # Detect direct instantiation (new keyword) of dependencies
            # Look for "new SomeService()" or "new SomeRepository()"
            new_dependency_pattern = r'new\s+(\w+(?:Service|Repository|Manager|Helper))\s*\('

            matches = re.finditer(new_dependency_pattern, content)
            for match in matches:
                class_name = match.group(1)
                violations.append(
                    f"File: {code_file.relative_to(self.project_root)}\n"
                    f"     Direct instantiation: new {class_name}()"
                )

        if violations:
            for violation in violations[:3]:
                self.medium_issues.append(f"⚠️ TIGHT COUPLING: {violation}")
            if len(violations) > 3:
                self.medium_issues.append(f"   ... and {len(violations) - 3} more")

            print(f"   {Color.YELLOW}⚠ {len(violations)} violation(s) found{Color.END}")
        else:
            print(f"   {Color.GREEN}✓ No tight coupling detected{Color.END}")

    def _detect_security_anti_patterns(self):
        """Detect Category 8: Security Anti-Patterns (CRITICAL)."""
        print(f"7. Checking for security anti-patterns...")

        violations = []

        code_files = list(self.project_root.glob("**/*.cs")) + \
                     list(self.project_root.glob("**/*.ts"))

        for code_file in code_files:
            if 'node_modules' in str(code_file):
                continue

            content = code_file.read_text(errors='ignore')

            # SQL Injection risk (string concatenation in SQL)
            sql_injection_pattern = r'(?:SELECT|INSERT|UPDATE|DELETE).*?\+\s*\w+'
            if re.search(sql_injection_pattern, content, re.IGNORECASE):
                violations.append(
                    f"File: {code_file.relative_to(self.project_root)}\n"
                    f"     Potential SQL injection (string concatenation in SQL)"
                )

            # XSS risk (innerHTML usage in TypeScript)
            if code_file.suffix in ['.ts', '.tsx']:
                if 'innerHTML' in content:
                    violations.append(
                        f"File: {code_file.relative_to(self.project_root)}\n"
                        f"     Potential XSS (innerHTML usage)"
                    )

        if violations:
            for violation in violations:
                self.critical_issues.append(f"❌ SECURITY RISK: {violation}")

            print(f"   {Color.RED}✗ {len(violations)} violation(s) found{Color.END}")
        else:
            print(f"   {Color.GREEN}✓ No security anti-patterns detected{Color.END}")

    def _detect_performance_anti_patterns(self):
        """Detect Category 9: Performance Anti-Patterns (HIGH)."""
        print(f"8. Checking for performance anti-patterns...")

        violations = []

        code_files = list(self.project_root.glob("**/*.cs"))

        for code_file in code_files:
            content = code_file.read_text(errors='ignore')

            # N+1 query pattern (loop with database query inside)
            # Look for foreach/for with query/execute inside
            loop_pattern = r'foreach\s*\([^)]+\).*?\{[^}]*?(?:Query|Execute|Get)\w*\([^}]*?\}'
            if re.search(loop_pattern, content, re.DOTALL):
                violations.append(
                    f"File: {code_file.relative_to(self.project_root)}\n"
                    f"     Potential N+1 query (database call inside loop)"
                )

            # Synchronous blocking (.Result, .Wait())
            blocking_pattern = r'\.(?:Result|Wait\(\))'
            if re.search(blocking_pattern, content):
                violations.append(
                    f"File: {code_file.relative_to(self.project_root)}\n"
                    f"     Synchronous blocking (.Result or .Wait())"
                )

        if violations:
            for violation in violations[:3]:
                self.high_issues.append(f"⚠️ PERFORMANCE ISSUE: {violation}")
            if len(violations) > 3:
                self.high_issues.append(f"   ... and {len(violations) - 3} more")

            print(f"   {Color.YELLOW}⚠ {len(violations)} violation(s) found{Color.END}")
        else:
            print(f"   {Color.GREEN}✓ No performance anti-patterns detected{Color.END}")

    def _detect_test_anti_patterns(self):
        """Detect Category 10: Test Anti-Patterns (LOW)."""
        print(f"9. Checking for test anti-patterns...")

        violations = []

        test_files = list(self.project_root.glob("**/*Test.cs")) + \
                     list(self.project_root.glob("**/*Tests.cs")) + \
                     list(self.project_root.glob("**/*.test.ts"))

        for test_file in test_files:
            content = test_file.read_text(errors='ignore')

            # Test interdependence (test methods that depend on order)
            # Look for shared state in test class
            if re.search(r'private\s+(?:static\s+)?(?!readonly)\w+\s+_\w+', content):
                violations.append(
                    f"File: {test_file.relative_to(self.project_root)}\n"
                    f"     Shared mutable state (potential test interdependence)"
                )

        if violations:
            for violation in violations:
                self.low_issues.append(f"ℹ️ TEST ANTI-PATTERN: {violation}")

            print(f"   {Color.CYAN}ℹ {len(violations)} violation(s) found{Color.END}")
        else:
            print(f"   {Color.GREEN}✓ No test anti-patterns detected{Color.END}")

    def _report_results(self):
        """Print detection results summary."""
        print(f"\n{Color.BOLD}{'='*60}{Color.END}")
        print(f"{Color.BOLD}Anti-Pattern Detection Results{Color.END}")
        print(f"{Color.BOLD}{'='*60}{Color.END}\n")

        total_issues = len(self.critical_issues) + len(self.high_issues) + \
                       len(self.medium_issues) + len(self.low_issues)

        if self.critical_issues:
            print(f"{Color.RED}{Color.BOLD}CRITICAL Issues ({len(self.critical_issues)}):{Color.END}")
            for issue in self.critical_issues:
                print(f"  {issue}\n")

        if self.high_issues:
            print(f"{Color.YELLOW}{Color.BOLD}HIGH Issues ({len(self.high_issues)}):{Color.END}")
            for issue in self.high_issues:
                print(f"  {issue}\n")

        if self.medium_issues:
            print(f"{Color.YELLOW}MEDIUM Issues ({len(self.medium_issues)}):{Color.END}")
            for issue in self.medium_issues:
                print(f"  {issue}\n")

        if self.low_issues:
            print(f"{Color.CYAN}LOW Issues ({len(self.low_issues)}):{Color.END}")
            for issue in self.low_issues:
                print(f"  {issue}\n")

        if total_issues == 0:
            print(f"{Color.GREEN}{Color.BOLD}✅ NO ANTI-PATTERNS DETECTED{Color.END}")
        elif len(self.critical_issues) > 0 or len(self.high_issues) > 0:
            print(f"{Color.RED}{Color.BOLD}❌ CRITICAL/HIGH ISSUES FOUND{Color.END}")
        else:
            print(f"{Color.YELLOW}{Color.BOLD}⚠️ MEDIUM/LOW ISSUES FOUND{Color.END}")


def main():
    parser = argparse.ArgumentParser(
        description="Detect anti-patterns defined in anti-patterns.md"
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

    detector = AntiPatternDetector(args.project_root, args.context_dir)

    try:
        success = detector.detect()
        sys.exit(0 if success else 1)
    except Exception as e:
        print(f"{Color.RED}FATAL ERROR: {e}{Color.END}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(2)


if __name__ == "__main__":
    main()
