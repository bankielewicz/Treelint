#!/usr/bin/env python3
"""
Validate technical specifications against context files.

This script validates that technical specifications (in devforgeai/specs/)
respect all constraints defined in the 6 context files:
1. tech-stack.md - Technology compliance
2. dependencies.md - Package compliance
3. source-tree.md - File structure compliance
4. coding-standards.md - Naming and style compliance
5. architecture-constraints.md - Layer boundary compliance
6. anti-patterns.md - Anti-pattern detection

This implements Phase 5 of the DevForgeAI architecture workflow.

Exit codes:
0 - Specification is valid
1 - Specification violates context constraints
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


class SpecValidator:
    """Validates technical specifications against context files."""

    def __init__(self, spec_file: Path, context_dir: Path):
        self.spec_file = spec_file
        self.context_dir = context_dir
        self.errors: List[str] = []
        self.warnings: List[str] = []

        # Context file paths
        self.tech_stack_md = context_dir / "tech-stack.md"
        self.dependencies_md = context_dir / "dependencies.md"
        self.source_tree_md = context_dir / "source-tree.md"
        self.coding_standards_md = context_dir / "coding-standards.md"
        self.architecture_constraints_md = context_dir / "architecture-constraints.md"
        self.anti_patterns_md = context_dir / "anti-patterns.md"

        # Parsed context data
        self.allowed_technologies: Set[str] = set()
        self.allowed_packages: Set[str] = set()
        self.forbidden_packages: Set[str] = set()
        self.required_directories: Dict[str, str] = {}
        self.forbidden_patterns: List[str] = []
        self.layer_rules: Dict[str, Set[str]] = {}

    def validate(self) -> bool:
        """Run all validations. Returns True if spec is valid."""
        print(f"\n{Color.BOLD}{Color.CYAN}=== DevForgeAI Spec Validator ==={Color.END}\n")

        # Check if spec file exists
        if not self.spec_file.exists():
            self._error(f"Specification file not found: {self.spec_file}")
            return False

        print(f"Validating: {Color.CYAN}{self.spec_file}{Color.END}")
        print(f"Against context files in: {Color.CYAN}{self.context_dir}{Color.END}\n")

        # Load context files
        if not self._load_context_files():
            return False

        # Load spec content
        spec_content = self.spec_file.read_text(encoding='utf-8')

        # Run validations
        self._validate_tech_stack_compliance(spec_content)
        self._validate_dependency_compliance(spec_content)
        self._validate_structure_compliance(spec_content)
        self._validate_architecture_compliance(spec_content)
        self._validate_anti_patterns(spec_content)
        self._validate_coding_standards(spec_content)

        # Report results
        return self._report_results()

    def _load_context_files(self) -> bool:
        """Load and parse all context files."""
        print(f"{Color.BOLD}Loading context files...{Color.END}")

        required_files = [
            self.tech_stack_md,
            self.dependencies_md,
            self.source_tree_md,
            self.coding_standards_md,
            self.architecture_constraints_md,
            self.anti_patterns_md
        ]

        missing_files = [f for f in required_files if not f.exists()]
        if missing_files:
            for f in missing_files:
                self._error(f"Missing context file: {f.name}")
            return False

        # Parse tech-stack.md
        self._parse_tech_stack()

        # Parse dependencies.md
        self._parse_dependencies()

        # Parse source-tree.md
        self._parse_source_tree()

        # Parse architecture-constraints.md
        self._parse_architecture_constraints()

        # Parse anti-patterns.md
        self._parse_anti_patterns()

        print(f"{Color.GREEN}✓ All context files loaded{Color.END}\n")
        return True

    def _parse_tech_stack(self):
        """Parse tech-stack.md to extract allowed technologies."""
        content = self.tech_stack_md.read_text(encoding='utf-8')

        # Extract technologies from markdown lists and tables
        # Look for patterns like: - Technology: Value or | Technology | Value |
        tech_patterns = [
            r'-\s+([A-Za-z\s\.]+):\s*([^\n]+)',
            r'\|\s*([A-Za-z\s\.]+)\s*\|\s*([^\|\n]+)\s*\|'
        ]

        for pattern in tech_patterns:
            matches = re.finditer(pattern, content)
            for match in matches:
                tech = match.group(2).strip()
                if tech and tech not in ['---', 'N/A', 'TBD']:
                    self.allowed_technologies.add(tech.lower())

    def _parse_dependencies(self):
        """Parse dependencies.md to extract allowed/forbidden packages."""
        content = self.dependencies_md.read_text(encoding='utf-8')

        # Extract ALLOWED packages
        allowed_section = re.search(r'##\s+ALLOWED.*?(?=##|$)', content, re.DOTALL | re.IGNORECASE)
        if allowed_section:
            packages = re.findall(r'-\s+`([^`]+)`|^\s*([A-Za-z0-9\._-]+)', allowed_section.group(0), re.MULTILINE)
            for pkg_tuple in packages:
                pkg = pkg_tuple[0] or pkg_tuple[1]
                if pkg and not pkg.startswith('#'):
                    self.allowed_packages.add(pkg.strip())

        # Extract FORBIDDEN packages
        forbidden_section = re.search(r'##\s+FORBIDDEN.*?(?=##|$)', content, re.DOTALL | re.IGNORECASE)
        if forbidden_section:
            packages = re.findall(r'-\s+`([^`]+)`|^\s*([A-Za-z0-9\._-]+)', forbidden_section.group(0), re.MULTILINE)
            for pkg_tuple in packages:
                pkg = pkg_tuple[0] or pkg_tuple[1]
                if pkg and not pkg.startswith('#'):
                    self.forbidden_packages.add(pkg.strip())

    def _parse_source_tree(self):
        """Parse source-tree.md to extract required directory structure."""
        content = self.source_tree_md.read_text(encoding='utf-8')

        # Extract directory mappings from tables
        # Look for: | Component Type | Directory | Example |
        table_rows = re.finditer(r'\|\s*([^|]+)\s*\|\s*`([^`]+)`\s*\|', content)
        for match in table_rows:
            component_type = match.group(1).strip()
            directory = match.group(2).strip()
            if component_type and directory and not component_type.lower() in ['component', 'type', '---']:
                self.required_directories[component_type.lower()] = directory

    def _parse_architecture_constraints(self):
        """Parse architecture-constraints.md to extract layer rules."""
        content = self.architecture_constraints_md.read_text(encoding='utf-8')

        # Extract layer dependency rules
        # Look for patterns like: "Domain layer CANNOT depend on Infrastructure"
        cannot_patterns = re.finditer(
            r'([A-Za-z]+)\s+layer\s+(?:CANNOT|must not|should not)\s+(?:depend on|reference)\s+([A-Za-z]+)',
            content,
            re.IGNORECASE
        )

        for match in cannot_patterns:
            layer = match.group(1).strip().lower()
            forbidden_dep = match.group(2).strip().lower()
            if layer not in self.layer_rules:
                self.layer_rules[layer] = set()
            self.layer_rules[layer].add(forbidden_dep)

    def _parse_anti_patterns(self):
        """Parse anti-patterns.md to extract forbidden patterns."""
        content = self.anti_patterns_md.read_text(encoding='utf-8')

        # Extract anti-pattern names from headers and sections
        anti_pattern_headers = re.finditer(r'###\s+(.+?)(?:\n|$)', content)
        for match in anti_pattern_headers:
            pattern = match.group(1).strip()
            if pattern and not pattern.startswith('Example'):
                self.forbidden_patterns.append(pattern.lower())

    def _validate_tech_stack_compliance(self, spec_content: str):
        """Validate spec doesn't require technologies not in tech-stack.md."""
        print(f"{Color.BOLD}1. Validating Tech Stack Compliance...{Color.END}")

        # Common technology keywords to check
        tech_keywords = [
            'redis', 'mongodb', 'postgresql', 'mysql', 'kafka', 'rabbitmq',
            'react', 'vue', 'angular', 'express', 'fastapi', 'django',
            'docker', 'kubernetes', 'aws', 'azure', 'gcp'
        ]

        violations = []
        for keyword in tech_keywords:
            if re.search(rf'\b{keyword}\b', spec_content, re.IGNORECASE):
                if keyword.lower() not in self.allowed_technologies:
                    # Check if it's a common mention vs actual requirement
                    requirement_context = re.search(
                        rf'(?:use|using|requires?|implement|with)\s+{keyword}',
                        spec_content,
                        re.IGNORECASE
                    )
                    if requirement_context:
                        violations.append(keyword)

        if violations:
            self._warning(
                f"Spec mentions technologies not in tech-stack.md: {', '.join(violations)}\n"
                f"  → Verify these are required or add to tech-stack.md with ADR"
            )
        else:
            print(f"  {Color.GREEN}✓ No tech stack violations{Color.END}")

    def _validate_dependency_compliance(self, spec_content: str):
        """Validate spec doesn't require packages not in dependencies.md."""
        print(f"\n{Color.BOLD}2. Validating Dependency Compliance...{Color.END}")

        # Extract package references from spec
        # Look for common package name patterns
        package_patterns = [
            r'(?:npm|pip|nuget)\s+(?:install|add)\s+([A-Za-z0-9\._-]+)',
            r'`([A-Za-z][A-Za-z0-9\._-]+)`\s+package',
            r'using\s+([A-Za-z][A-Za-z0-9\.]+);',
            r'import\s+([A-Za-z][A-Za-z0-9\.]+)',
        ]

        found_packages = set()
        for pattern in package_patterns:
            matches = re.finditer(pattern, spec_content)
            for match in matches:
                found_packages.add(match.group(1))

        # Check for forbidden packages
        forbidden_found = found_packages.intersection(self.forbidden_packages)
        if forbidden_found:
            for pkg in forbidden_found:
                self._error(
                    f"Spec requires FORBIDDEN package: {pkg}\n"
                    f"  → This package is explicitly forbidden in dependencies.md"
                )

        # Check for undocumented packages
        if self.allowed_packages:
            undocumented = found_packages - self.allowed_packages - self.forbidden_packages
            if undocumented:
                self._warning(
                    f"Spec mentions packages not in dependencies.md: {', '.join(undocumented)}\n"
                    f"  → Add to dependencies.md if required"
                )

        if not forbidden_found:
            print(f"  {Color.GREEN}✓ No dependency violations{Color.END}")

    def _validate_structure_compliance(self, spec_content: str):
        """Validate spec file locations match source-tree.md."""
        print(f"\n{Color.BOLD}3. Validating Structure Compliance...{Color.END}")

        violations = []

        # Extract file path specifications
        # Look for patterns like: "in src/Controllers/" or "create src/Services/"
        path_patterns = [
            r'(?:in|create|add|place)\s+([a-zA-Z0-9_/\.\-]+)',
            r'`([a-zA-Z0-9_/\.\-]+\.(?:cs|js|py|ts|jsx|tsx))`'
        ]

        specified_paths = set()
        for pattern in path_patterns:
            matches = re.finditer(pattern, spec_content)
            for match in matches:
                path = match.group(1)
                if '/' in path and not path.startswith('http'):
                    specified_paths.add(path)

        # Check if specified paths align with required directories
        for path in specified_paths:
            # Check against required directory structure
            path_valid = False
            for component_type, required_dir in self.required_directories.items():
                if required_dir in path:
                    path_valid = True
                    break

            if not path_valid and self.required_directories:
                violations.append(path)

        if violations:
            self._warning(
                f"Spec contains paths that may not match source-tree.md:\n"
                + "\n".join(f"  → {v}" for v in violations[:5])
            )
        else:
            print(f"  {Color.GREEN}✓ No structure violations{Color.END}")

    def _validate_architecture_compliance(self, spec_content: str):
        """Validate spec respects layer boundaries."""
        print(f"\n{Color.BOLD}4. Validating Architecture Compliance...{Color.END}")

        violations = []

        # Check for forbidden layer dependencies
        for layer, forbidden_deps in self.layer_rules.items():
            for forbidden_dep in forbidden_deps:
                # Look for mentions of layer depending on forbidden layer
                pattern = rf'{layer}.*(?:depends? on|references?|uses?|imports?).*{forbidden_dep}'
                if re.search(pattern, spec_content, re.IGNORECASE):
                    violations.append(f"{layer} depends on {forbidden_dep}")

        if violations:
            for violation in violations:
                self._error(
                    f"Architecture violation: {violation}\n"
                    f"  → This violates layer boundaries in architecture-constraints.md"
                )
        else:
            print(f"  {Color.GREEN}✓ No architecture violations{Color.END}")

    def _validate_anti_patterns(self, spec_content: str):
        """Validate spec doesn't describe anti-patterns."""
        print(f"\n{Color.BOLD}5. Validating Anti-Pattern Compliance...{Color.END}")

        found_patterns = []

        for anti_pattern in self.forbidden_patterns:
            # Check if anti-pattern is mentioned in a positive context
            if re.search(rf'\b{re.escape(anti_pattern)}\b', spec_content, re.IGNORECASE):
                # Check if it's in a "don't do this" context or actual implementation
                positive_context = re.search(
                    rf'(?:implement|use|create|add).*{re.escape(anti_pattern)}',
                    spec_content,
                    re.IGNORECASE
                )
                if positive_context:
                    found_patterns.append(anti_pattern)

        if found_patterns:
            for pattern in found_patterns:
                self._warning(
                    f"Spec may describe anti-pattern: {pattern}\n"
                    f"  → Review against anti-patterns.md"
                )
        else:
            print(f"  {Color.GREEN}✓ No anti-patterns detected{Color.END}")

    def _validate_coding_standards(self, spec_content: str):
        """Validate spec follows coding standards."""
        print(f"\n{Color.BOLD}6. Validating Coding Standards...{Color.END}")

        # This is a basic check - can be expanded based on coding-standards.md content
        standards_content = self.coding_standards_md.read_text(encoding='utf-8')

        # Check for naming convention mentions
        violations = []

        # Extract naming examples from spec
        class_names = re.findall(r'class\s+([A-Za-z0-9_]+)', spec_content)
        method_names = re.findall(r'(?:method|function)\s+([A-Za-z0-9_]+)', spec_content)

        # Basic checks (can be enhanced)
        for name in class_names:
            if name and not name[0].isupper():
                violations.append(f"Class name '{name}' should start with uppercase")

        if violations:
            for violation in violations[:5]:  # Limit to first 5
                self._warning(f"Coding standard issue: {violation}")
        else:
            print(f"  {Color.GREEN}✓ No coding standard violations{Color.END}")

    def _error(self, message: str):
        """Record an error."""
        self.errors.append(message)

    def _warning(self, message: str):
        """Record a warning."""
        self.warnings.append(message)

    def _report_results(self) -> bool:
        """Print validation results and return success status."""
        print(f"\n{Color.BOLD}{'='*60}{Color.END}")
        print(f"{Color.BOLD}Validation Results{Color.END}")
        print(f"{Color.BOLD}{'='*60}{Color.END}\n")

        if self.errors:
            print(f"{Color.RED}{Color.BOLD}✗ ERRORS ({len(self.errors)}):{Color.END}")
            for i, error in enumerate(self.errors, 1):
                print(f"{Color.RED}{i}. {error}{Color.END}\n")

        if self.warnings:
            print(f"{Color.YELLOW}{Color.BOLD}⚠ WARNINGS ({len(self.warnings)}):{Color.END}")
            for i, warning in enumerate(self.warnings, 1):
                print(f"{Color.YELLOW}{i}. {warning}{Color.END}\n")

        if not self.errors and not self.warnings:
            print(f"{Color.GREEN}{Color.BOLD}✓ SPECIFICATION VALID{Color.END}")
            print(f"{Color.GREEN}All context constraints satisfied.{Color.END}\n")
            return True
        elif not self.errors:
            print(f"{Color.YELLOW}{Color.BOLD}⚠ SPECIFICATION VALID WITH WARNINGS{Color.END}")
            print(f"{Color.YELLOW}Review warnings before proceeding.{Color.END}\n")
            return True
        else:
            print(f"{Color.RED}{Color.BOLD}✗ SPECIFICATION INVALID{Color.END}")
            print(f"{Color.RED}Fix errors before proceeding to development.{Color.END}\n")
            return False


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description='Validate technical specifications against DevForgeAI context files.',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Validate a specific spec file
  python validate_spec.py devforgeai/specs/user-authentication-spec.md

  # Validate with custom context directory
  python validate_spec.py spec.md --context-dir /path/to/context

Exit codes:
  0 - Specification is valid
  1 - Specification violates context constraints
  2 - Configuration or file errors
        """
    )

    parser.add_argument(
        'spec_file',
        type=Path,
        help='Path to technical specification file to validate'
    )

    parser.add_argument(
        '--context-dir',
        type=Path,
        default=Path('devforgeai/context'),
        help='Path to context files directory (default: devforgeai/context)'
    )

    parser.add_argument(
        '--strict',
        action='store_true',
        help='Treat warnings as errors'
    )

    args = parser.parse_args()

    # Validate arguments
    if not args.context_dir.exists():
        print(f"{Color.RED}Error: Context directory not found: {args.context_dir}{Color.END}")
        print(f"{Color.YELLOW}Run /create-context to initialize context files.{Color.END}")
        return 2

    # Run validation
    validator = SpecValidator(args.spec_file, args.context_dir)
    is_valid = validator.validate()

    # Handle strict mode
    if args.strict and validator.warnings:
        print(f"{Color.RED}Strict mode: Warnings treated as errors{Color.END}")
        return 1

    return 0 if is_valid else 1


if __name__ == '__main__':
    sys.exit(main())
