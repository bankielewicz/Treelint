#!/usr/bin/env python3
"""
Cyclomatic Complexity Analyzer

Calculates cyclomatic complexity for codebase using radon or lizard.
Flags methods with complexity > 10 and classes with complexity > 50.

Supports multiple languages through lizard.
"""

import json
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Optional
from dataclasses import dataclass, asdict
import argparse


@dataclass
class MethodComplexity:
    """Complexity data for a method/function"""
    name: str
    file: str
    line: int
    complexity: int
    threshold: int = 10
    violates_threshold: bool = False


@dataclass
class ClassComplexity:
    """Complexity data for a class"""
    name: str
    file: str
    complexity: int
    methods: List[MethodComplexity]
    threshold: int = 50
    violates_threshold: bool = False


@dataclass
class ComplexityReport:
    """Complete complexity analysis report"""
    total_methods: int
    total_classes: int
    violations_method: int
    violations_class: int
    average_method_complexity: float
    methods: List[MethodComplexity]
    classes: List[ClassComplexity]


class ComplexityAnalyzer:
    """Analyze cyclomatic complexity for codebase"""

    def __init__(self, source_path: str, method_threshold: int = 10, class_threshold: int = 50):
        self.source_path = Path(source_path)
        self.method_threshold = method_threshold
        self.class_threshold = class_threshold

    def analyze(self) -> ComplexityReport:
        """Run complexity analysis"""
        # Try radon first (Python-specific, more accurate)
        if self._is_python_project():
            try:
                return self._analyze_with_radon()
            except (FileNotFoundError, subprocess.CalledProcessError):
                print("⚠️  Radon not available, falling back to lizard")

        # Fall back to lizard (multi-language)
        try:
            return self._analyze_with_lizard()
        except (FileNotFoundError, subprocess.CalledProcessError) as e:
            raise RuntimeError(
                "Neither radon nor lizard is available. Install one:\n"
                "  pip install radon\n"
                "  pip install lizard"
            ) from e

    def _is_python_project(self) -> bool:
        """Check if this is primarily a Python project"""
        python_files = list(self.source_path.rglob("*.py"))
        return len(python_files) > 0

    def _analyze_with_radon(self) -> ComplexityReport:
        """Analyze using radon (Python only)"""
        # Run radon cc (cyclomatic complexity)
        result = subprocess.run(
            ["radon", "cc", str(self.source_path), "-a", "-j"],
            capture_output=True,
            text=True,
            check=True
        )

        data = json.loads(result.stdout)

        methods = []
        classes = {}

        for file_path, functions in data.items():
            for func in functions:
                complexity = func['complexity']
                method = MethodComplexity(
                    name=func['name'],
                    file=file_path,
                    line=func['lineno'],
                    complexity=complexity,
                    threshold=self.method_threshold,
                    violates_threshold=(complexity > self.method_threshold)
                )
                methods.append(method)

                # Extract class name if method belongs to class
                if '.' in func['name']:
                    class_name = func['name'].split('.')[0]
                    if class_name not in classes:
                        classes[class_name] = {
                            'file': file_path,
                            'complexity': 0,
                            'methods': []
                        }
                    classes[class_name]['complexity'] += complexity
                    classes[class_name]['methods'].append(method)

        # Create class complexity objects
        class_list = []
        for class_name, class_data in classes.items():
            class_obj = ClassComplexity(
                name=class_name,
                file=class_data['file'],
                complexity=class_data['complexity'],
                methods=class_data['methods'],
                threshold=self.class_threshold,
                violates_threshold=(class_data['complexity'] > self.class_threshold)
            )
            class_list.append(class_obj)

        return self._create_report(methods, class_list)

    def _analyze_with_lizard(self) -> ComplexityReport:
        """Analyze using lizard (multi-language)"""
        # Run lizard
        result = subprocess.run(
            ["lizard", str(self.source_path), "-l", "python", "-l", "csharp", "-l", "javascript", "--csv"],
            capture_output=True,
            text=True,
            check=True
        )

        methods = []
        classes = {}

        # Parse CSV output (skip header)
        lines = result.stdout.strip().split('\n')[1:]

        for line in lines:
            if not line.strip():
                continue

            parts = line.split(',')
            if len(parts) < 4:
                continue

            try:
                complexity = int(parts[0])
                nloc = int(parts[1])
                func_name = parts[3].strip('"')
                file_path = parts[4].strip('"') if len(parts) > 4 else "unknown"
                line_no = int(parts[2]) if parts[2].isdigit() else 0

                method = MethodComplexity(
                    name=func_name,
                    file=file_path,
                    line=line_no,
                    complexity=complexity,
                    threshold=self.method_threshold,
                    violates_threshold=(complexity > self.method_threshold)
                )
                methods.append(method)

                # Try to extract class name
                if '::' in func_name or '.' in func_name:
                    separator = '::' if '::' in func_name else '.'
                    class_name = func_name.split(separator)[0]

                    if class_name not in classes:
                        classes[class_name] = {
                            'file': file_path,
                            'complexity': 0,
                            'methods': []
                        }
                    classes[class_name]['complexity'] += complexity
                    classes[class_name]['methods'].append(method)

            except (ValueError, IndexError) as e:
                print(f"⚠️  Failed to parse line: {line}", file=sys.stderr)
                continue

        # Create class complexity objects
        class_list = []
        for class_name, class_data in classes.items():
            class_obj = ClassComplexity(
                name=class_name,
                file=class_data['file'],
                complexity=class_data['complexity'],
                methods=class_data['methods'],
                threshold=self.class_threshold,
                violates_threshold=(class_data['complexity'] > self.class_threshold)
            )
            class_list.append(class_obj)

        return self._create_report(methods, class_list)

    def _create_report(self, methods: List[MethodComplexity], classes: List[ClassComplexity]) -> ComplexityReport:
        """Create complexity report from analyzed data"""
        violations_method = sum(1 for m in methods if m.violates_threshold)
        violations_class = sum(1 for c in classes if c.violates_threshold)

        avg_complexity = sum(m.complexity for m in methods) / len(methods) if methods else 0.0

        return ComplexityReport(
            total_methods=len(methods),
            total_classes=len(classes),
            violations_method=violations_method,
            violations_class=violations_class,
            average_method_complexity=round(avg_complexity, 2),
            methods=methods,
            classes=classes
        )

    def generate_json_report(self, output_file: str):
        """Generate JSON complexity report"""
        report = self.analyze()

        # Convert to dict
        report_dict = {
            'summary': {
                'total_methods': report.total_methods,
                'total_classes': report.total_classes,
                'violations_method': report.violations_method,
                'violations_class': report.violations_class,
                'average_method_complexity': report.average_method_complexity
            },
            'methods': [asdict(m) for m in report.methods],
            'classes': [
                {
                    'name': c.name,
                    'file': c.file,
                    'complexity': c.complexity,
                    'threshold': c.threshold,
                    'violates_threshold': c.violates_threshold,
                    'method_count': len(c.methods)
                }
                for c in report.classes
            ]
        }

        # Write JSON
        with open(output_file, 'w') as f:
            json.dump(report_dict, f, indent=2)

        print(f"✅ Complexity report generated: {output_file}")
        print(f"   Total methods: {report.total_methods}")
        print(f"   Method violations (>{self.method_threshold}): {report.violations_method}")
        print(f"   Class violations (>{self.class_threshold}): {report.violations_class}")
        print(f"   Average complexity: {report.average_method_complexity}")

        return report

    def print_violations(self):
        """Print complexity violations to console"""
        report = self.analyze()

        if report.violations_method > 0:
            print("\n❌ Method Complexity Violations:")
            for method in report.methods:
                if method.violates_threshold:
                    print(f"   {method.file}:{method.line} - {method.name} (complexity: {method.complexity})")

        if report.violations_class > 0:
            print("\n❌ Class Complexity Violations:")
            for cls in report.classes:
                if cls.violates_threshold:
                    print(f"   {cls.file} - {cls.name} (complexity: {cls.complexity})")

        if report.violations_method == 0 and report.violations_class == 0:
            print("\n✅ No complexity violations found!")


def main():
    parser = argparse.ArgumentParser(
        description='Analyze cyclomatic complexity for codebase',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Analyze with default thresholds (method: 10, class: 50)
  python analyze_complexity.py src/

  # Analyze with custom thresholds
  python analyze_complexity.py src/ --method-threshold 15 --class-threshold 75

  # Generate JSON report
  python analyze_complexity.py src/ --output complexity-report.json

  # Print violations only
  python analyze_complexity.py src/ --violations-only
        """
    )
    parser.add_argument('source_path', help='Source code directory path')
    parser.add_argument('--method-threshold', type=int, default=10,
                        help='Maximum complexity for methods (default: 10)')
    parser.add_argument('--class-threshold', type=int, default=50,
                        help='Maximum complexity for classes (default: 50)')
    parser.add_argument('--output', help='Output JSON file')
    parser.add_argument('--violations-only', action='store_true',
                        help='Print violations only (no JSON report)')

    args = parser.parse_args()

    try:
        analyzer = ComplexityAnalyzer(
            args.source_path,
            args.method_threshold,
            args.class_threshold
        )

        if args.violations_only:
            analyzer.print_violations()
        elif args.output:
            analyzer.generate_json_report(args.output)
        else:
            # Default: print violations
            analyzer.print_violations()

        return 0
    except Exception as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
