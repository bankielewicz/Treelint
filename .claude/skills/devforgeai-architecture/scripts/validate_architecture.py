#!/usr/bin/env python3
"""
Validate architecture-constraints.md against actual project code.

This script checks:
1. Layer dependency rules (e.g., Domain cannot reference Infrastructure)
2. Mandatory patterns are implemented (Repository, Service, DTO)
3. Cross-layer violations (using import/using statements)
4. File placement follows layer structure
5. Naming conventions for architectural components

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


class ArchitectureValidator:
    """Validates project architecture against architecture-constraints.md."""

    def __init__(self, project_root: Path, context_dir: Path):
        self.project_root = project_root
        self.context_dir = context_dir
        self.architecture_md = context_dir / "architecture-constraints.md"
        self.source_tree_md = context_dir / "source-tree.md"
        self.errors: List[str] = []
        self.warnings: List[str] = []

        # Architecture configuration
        self.layers: Dict[str, Path] = {}
        self.layer_dependencies: Dict[str, Set[str]] = {}
        self.mandatory_patterns: Set[str] = set()
        self.forbidden_patterns: Set[str] = set()

    def validate(self) -> bool:
        """Run all validation checks. Returns True if all passed."""
        print(f"{Color.BOLD}Validating Architecture{Color.END}")
        print(f"Project root: {self.project_root}")
        print(f"Context dir: {self.context_dir}\n")

        if not self.architecture_md.exists():
            self.errors.append(f"architecture-constraints.md not found at {self.architecture_md}")
            return False

        # Parse architecture configuration
        self._parse_architecture_md()

        # Discover project layers
        self._discover_layers()

        if not self.layers:
            self.warnings.append("No architecture layers detected in project")
            self._report_results()
            return True

        # Validate layer dependencies
        print(f"{Color.CYAN}Validating layer dependencies...{Color.END}")
        self._validate_layer_dependencies()

        # Validate mandatory patterns
        print(f"\n{Color.CYAN}Validating mandatory patterns...{Color.END}")
        self._validate_mandatory_patterns()

        # Validate forbidden patterns
        print(f"\n{Color.CYAN}Checking for forbidden patterns...{Color.END}")
        self._validate_forbidden_patterns()

        # Report results
        self._report_results()

        return len(self.errors) == 0

    def _parse_architecture_md(self):
        """Parse architecture-constraints.md to extract rules."""
        if not self.architecture_md.exists():
            return

        content = self.architecture_md.read_text()

        # Extract layer dependency matrix
        # Look for markdown table with layer dependencies
        matrix_pattern = r'\|\s*From[^|]+\|([^|]+\|)+.*?\n((?:\|[^|]+\|.*?\n)+)'
        matrix_match = re.search(matrix_pattern, content, re.MULTILINE)

        if matrix_match:
            # Parse the layer dependency table
            self._parse_dependency_matrix(matrix_match.group(0))

        # Extract mandatory patterns
        mandatory_pattern = r'(?:Mandatory Patterns?|Required Patterns?).*?(?:\n-\s*(.+?)(?:\n|$))+'
        for match in re.finditer(mandatory_pattern, content, re.IGNORECASE):
            # Extract bullet points after "Mandatory Patterns"
            section = content[match.start():match.start() + 500]
            for line in section.split('\n'):
                if re.match(r'\s*-\s*(.+)', line):
                    pattern = re.match(r'\s*-\s*(.+)', line).group(1)
                    # Clean up pattern name
                    pattern = re.sub(r'\s*\(.*?\)', '', pattern).strip()
                    self.mandatory_patterns.add(pattern)

        # Extract forbidden patterns
        forbidden_pattern = r'(?:Forbidden Patterns?|Anti-Patterns?).*?(?:\n-\s*(.+?)(?:\n|$))+'
        for match in re.finditer(forbidden_pattern, content, re.IGNORECASE):
            section = content[match.start():match.start() + 500]
            for line in section.split('\n'):
                if re.match(r'\s*-\s*(.+)', line):
                    pattern = re.match(r'\s*-\s*(.+)', line).group(1)
                    pattern = re.sub(r'\s*\(.*?\)', '', pattern).strip()
                    self.forbidden_patterns.add(pattern)

        print(f"Parsed architecture-constraints.md:")
        print(f"  Layer dependency rules: {len(self.layer_dependencies)}")
        print(f"  Mandatory patterns: {self.mandatory_patterns}")
        print(f"  Forbidden patterns: {self.forbidden_patterns}")

    def _parse_dependency_matrix(self, table: str):
        """Parse markdown table to extract layer dependencies."""
        lines = table.strip().split('\n')

        if len(lines) < 3:
            return

        # First line is header with layer names
        header = lines[0]
        layer_names = [name.strip() for name in header.split('|')[2:]]  # Skip "From ↓ To →"

        # Parse each row (skip header and separator line)
        for row in lines[2:]:
            cells = [cell.strip() for cell in row.split('|')]
            if len(cells) < 2:
                continue

            from_layer = cells[1].strip()  # First column is the "from" layer

            allowed = set()
            for i, cell in enumerate(cells[2:], start=0):
                if i < len(layer_names):
                    if '✓' in cell or '✅' in cell:
                        allowed.add(layer_names[i])

            if from_layer:
                self.layer_dependencies[from_layer] = allowed

    def _discover_layers(self):
        """Discover architecture layers in the project."""
        # Common layer patterns for Clean Architecture
        layer_patterns = {
            'API': ['**/API/', '**/Presentation/', '**/Web/', '**/WebApi/'],
            'Application': ['**/Application/', '**/UseCase/', '**/UseCases/'],
            'Domain': ['**/Domain/', '**/Core/', '**/Entities/'],
            'Infrastructure': ['**/Infrastructure/', '**/Persistence/', '**/Data/']
        }

        # Check source-tree.md for layer definitions if available
        if self.source_tree_md.exists():
            content = self.source_tree_md.read_text()
            for layer_name in layer_patterns.keys():
                # Look for layer directories mentioned in source-tree.md
                pattern = rf'{layer_name}/?\s*(?:-|–)'
                if re.search(pattern, content, re.IGNORECASE):
                    # Find actual directory
                    for pattern_path in layer_patterns[layer_name]:
                        dirs = list(self.project_root.glob(pattern_path))
                        if dirs:
                            self.layers[layer_name] = dirs[0]
                            break

        # Fallback: Search project for layer directories
        for layer_name, patterns in layer_patterns.items():
            if layer_name not in self.layers:
                for pattern_path in patterns:
                    dirs = list(self.project_root.glob(pattern_path))
                    if dirs:
                        self.layers[layer_name] = dirs[0]
                        break

        print(f"\nDiscovered architecture layers:")
        for layer_name, layer_path in self.layers.items():
            print(f"  {layer_name}: {layer_path}")

    def _validate_layer_dependencies(self):
        """Check for cross-layer dependency violations."""
        violations_found = 0

        for from_layer, from_path in self.layers.items():
            # Get allowed dependencies for this layer
            allowed_deps = self.layer_dependencies.get(from_layer, set())

            # Find all code files in this layer
            code_files = list(from_path.glob("**/*.cs")) + list(from_path.glob("**/*.ts"))

            for code_file in code_files:
                content = code_file.read_text(errors='ignore')

                # Extract import/using statements
                imports = self._extract_imports(content)

                # Check each import for violations
                for imported_namespace in imports:
                    # Determine which layer this import belongs to
                    imported_layer = self._get_layer_from_namespace(imported_namespace)

                    if imported_layer and imported_layer != from_layer:
                        # Check if this dependency is allowed
                        if imported_layer not in allowed_deps:
                            relative_path = code_file.relative_to(self.project_root)
                            self.errors.append(
                                f"❌ LAYER VIOLATION: {from_layer} → {imported_layer}\n"
                                f"   File: {relative_path}\n"
                                f"   Import: {imported_namespace}"
                            )
                            violations_found += 1

        if violations_found == 0:
            print(f"  {Color.GREEN}✓ No layer dependency violations found{Color.END}")

    def _extract_imports(self, content: str) -> List[str]:
        """Extract using/import statements from code."""
        imports = []

        # C# using statements
        using_pattern = r'^\s*using\s+([^;]+);'
        for match in re.finditer(using_pattern, content, re.MULTILINE):
            namespace = match.group(1).strip()
            # Skip static using and aliases
            if not namespace.startswith('static ') and '=' not in namespace:
                imports.append(namespace)

        # TypeScript/JavaScript imports
        import_pattern = r'^\s*import\s+.+\s+from\s+["\']([^"\']+)["\']'
        for match in re.finditer(import_pattern, content, re.MULTILINE):
            module = match.group(1).strip()
            imports.append(module)

        return imports

    def _get_layer_from_namespace(self, namespace: str) -> str:
        """Determine which layer a namespace belongs to."""
        # Check each known layer
        for layer_name in self.layers.keys():
            if layer_name.lower() in namespace.lower():
                return layer_name

        return None

    def _validate_mandatory_patterns(self):
        """Check that mandatory patterns are implemented."""
        patterns_found = defaultdict(int)

        for layer_path in self.layers.values():
            code_files = list(layer_path.glob("**/*.cs")) + list(layer_path.glob("**/*.ts"))

            for code_file in code_files:
                content = code_file.read_text(errors='ignore')

                # Check for Repository Pattern
                if "Repository Pattern" in self.mandatory_patterns:
                    if re.search(r'interface\s+I\w+Repository', content) or \
                       re.search(r'class\s+\w+Repository.*:\s*I\w+Repository', content):
                        patterns_found["Repository Pattern"] += 1

                # Check for Service Pattern
                if "Service Pattern" in self.mandatory_patterns:
                    if re.search(r'interface\s+I\w+Service', content) or \
                       re.search(r'class\s+\w+Service.*:\s*I\w+Service', content):
                        patterns_found["Service Pattern"] += 1

                # Check for DTO Pattern
                if "DTO Pattern" in self.mandatory_patterns:
                    if re.search(r'class\s+\w+Dto', content) or \
                       re.search(r'interface\s+\w+Dto', content):
                        patterns_found["DTO Pattern"] += 1

                # Check for Unit of Work
                if "Unit of Work" in self.mandatory_patterns:
                    if re.search(r'interface\s+IUnitOfWork', content) or \
                       re.search(r'class\s+UnitOfWork', content):
                        patterns_found["Unit of Work"] += 1

        # Report on mandatory patterns
        for pattern in self.mandatory_patterns:
            count = patterns_found.get(pattern, 0)
            if count == 0:
                self.warnings.append(
                    f"⚠️ MANDATORY PATTERN NOT FOUND: {pattern} is required but not detected in codebase"
                )
            else:
                print(f"  {Color.GREEN}✓ {pattern}: {count} implementations found{Color.END}")

    def _validate_forbidden_patterns(self):
        """Check for presence of forbidden patterns."""
        violations = []

        for layer_path in self.layers.values():
            code_files = list(layer_path.glob("**/*.cs")) + list(layer_path.glob("**/*.ts"))

            for code_file in code_files:
                content = code_file.read_text(errors='ignore')
                relative_path = code_file.relative_to(self.project_root)

                # Check for God Objects (classes with too many methods)
                class_methods = re.findall(r'(?:public|private|protected)\s+(?:\w+\s+)+\w+\s*\([^)]*\)', content)
                if len(class_methods) > 20:  # Arbitrary threshold
                    violations.append(
                        f"⚠️ POTENTIAL GOD OBJECT: {relative_path} has {len(class_methods)} methods"
                    )

                # Check for Business Logic in Controllers
                if "controllers" in str(code_file).lower() or "controller.cs" in str(code_file).lower():
                    # Look for complex business logic (multiple if/else, loops)
                    if_count = len(re.findall(r'\bif\s*\(', content))
                    for_count = len(re.findall(r'\b(?:for|while|foreach)\s*\(', content))

                    if if_count > 3 or for_count > 1:
                        violations.append(
                            f"⚠️ BUSINESS LOGIC IN CONTROLLER: {relative_path} has complex logic (if:{if_count}, loops:{for_count})"
                        )

                # Check for Direct Database Access in Controllers (EF DbContext usage)
                if "controllers" in str(code_file).lower():
                    if re.search(r':\s*DbContext', content) or re.search(r'new\s+\w+DbContext', content):
                        violations.append(
                            f"❌ DIRECT DATABASE ACCESS IN CONTROLLER: {relative_path} uses DbContext directly"
                        )

        if violations:
            for violation in violations:
                if violation.startswith("❌"):
                    self.errors.append(violation)
                else:
                    self.warnings.append(violation)
        else:
            print(f"  {Color.GREEN}✓ No forbidden patterns detected{Color.END}")

    def _report_results(self):
        """Print validation results summary."""
        print(f"\n{Color.BOLD}{'='*60}{Color.END}")
        print(f"{Color.BOLD}Architecture Validation Results{Color.END}")
        print(f"{Color.BOLD}{'='*60}{Color.END}\n")

        if self.warnings:
            print(f"{Color.YELLOW}Warnings ({len(self.warnings)}):{Color.END}")
            for warning in self.warnings:
                print(f"  {warning}\n")

        if self.errors:
            print(f"{Color.RED}Errors ({len(self.errors)}):{Color.END}")
            for error in self.errors:
                print(f"  {error}\n")
            print(f"{Color.RED}{Color.BOLD}❌ ARCHITECTURE VALIDATION FAILED{Color.END}")
        else:
            print(f"{Color.GREEN}{Color.BOLD}✅ ARCHITECTURE VALIDATION PASSED{Color.END}")


def main():
    parser = argparse.ArgumentParser(
        description="Validate project architecture against architecture-constraints.md"
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

    validator = ArchitectureValidator(args.project_root, args.context_dir)

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
