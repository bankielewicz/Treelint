#!/usr/bin/env python3
"""
Coverage Report Generator

Parses coverage data from various formats and generates comprehensive HTML reports
with layer-level breakdowns and visualizations.

Supports:
- .NET: Cobertura XML format
- Python: pytest-cov JSON/XML
- JavaScript: Istanbul JSON format
"""

import json
import xml.etree.ElementTree as ET
import sqlite3
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass
from datetime import datetime
import argparse
import sys


@dataclass
class FileCoverage:
    """Coverage data for a single file"""
    file_path: str
    total_lines: int
    covered_lines: int
    uncovered_lines: List[int]
    coverage_percentage: float
    layer: str


@dataclass
class LayerCoverage:
    """Coverage data for a layer (domain, application, infrastructure)"""
    layer_name: str
    total_lines: int
    covered_lines: int
    coverage_percentage: float
    files: List[FileCoverage]


class CoverageReportGenerator:
    """Generate coverage reports from various coverage file formats"""

    def __init__(self, coverage_file: str, source_tree_file: Optional[str] = None):
        self.coverage_file = Path(coverage_file)
        self.source_tree_file = Path(source_tree_file) if source_tree_file else None
        self.layer_mapping = self._load_layer_mapping()

    def _load_layer_mapping(self) -> Dict[str, str]:
        """Load layer mapping from source-tree.md"""
        if not self.source_tree_file or not self.source_tree_file.exists():
            # Default layer patterns
            return {
                "Domain": "domain",
                "Application": "application",
                "Infrastructure": "infrastructure",
                "Services": "application",
                "Repositories": "infrastructure",
                "Controllers": "application",
                "API": "application"
            }

        # Parse source-tree.md to extract layer paths
        # TODO: Implement source-tree.md parsing
        return {
            "Domain": "domain",
            "Application": "application",
            "Infrastructure": "infrastructure"
        }

    def parse_coverage(self) -> Dict[str, FileCoverage]:
        """Parse coverage file based on format"""
        suffix = self.coverage_file.suffix

        if suffix == '.json':
            return self._parse_coverage_json()
        elif suffix == '.xml':
            return self._parse_coverage_xml()
        elif self.coverage_file.name == '.coverage':
            return self._parse_coverage_sqlite()
        else:
            raise ValueError(f"Unsupported coverage format: {suffix}")

    def _parse_coverage_json(self) -> Dict[str, FileCoverage]:
        """Parse JSON coverage (Python pytest-cov, JavaScript Istanbul)"""
        with open(self.coverage_file) as f:
            data = json.load(f)

        file_coverage = {}

        # Check format: pytest-cov vs Istanbul
        if 'files' in data:
            # pytest-cov format
            for file_path, file_data in data['files'].items():
                summary = file_data.get('summary', {})
                total = summary.get('num_statements', 0)
                covered = summary.get('covered_lines', 0)
                missing = file_data.get('missing_lines', [])

                if total > 0:
                    percentage = (covered / total) * 100
                else:
                    percentage = 0.0

                file_coverage[file_path] = FileCoverage(
                    file_path=file_path,
                    total_lines=total,
                    covered_lines=covered,
                    uncovered_lines=missing,
                    coverage_percentage=percentage,
                    layer=self._identify_layer(file_path)
                )
        elif 'total' in data:
            # Istanbul format
            for file_path, file_data in data.items():
                if file_path == 'total':
                    continue

                statements = file_data.get('s', {})
                total = len(statements)
                covered = sum(1 for count in statements.values() if count > 0)
                uncovered = [line for line, count in statements.items() if count == 0]

                if total > 0:
                    percentage = (covered / total) * 100
                else:
                    percentage = 0.0

                file_coverage[file_path] = FileCoverage(
                    file_path=file_path,
                    total_lines=total,
                    covered_lines=covered,
                    uncovered_lines=uncovered,
                    coverage_percentage=percentage,
                    layer=self._identify_layer(file_path)
                )

        return file_coverage

    def _parse_coverage_xml(self) -> Dict[str, FileCoverage]:
        """Parse XML coverage (Cobertura format - .NET, Java)"""
        tree = ET.parse(self.coverage_file)
        root = tree.getroot()

        file_coverage = {}

        # Navigate Cobertura structure: packages/package/classes/class
        for package in root.findall('.//package'):
            for cls in package.findall('.//class'):
                filename = cls.get('filename')
                if not filename:
                    continue

                lines = cls.findall('.//line')
                total = len(lines)
                covered = sum(1 for line in lines if int(line.get('hits', 0)) > 0)
                uncovered = [int(line.get('number')) for line in lines if int(line.get('hits', 0)) == 0]

                if total > 0:
                    percentage = (covered / total) * 100
                else:
                    percentage = 0.0

                file_coverage[filename] = FileCoverage(
                    file_path=filename,
                    total_lines=total,
                    covered_lines=covered,
                    uncovered_lines=uncovered,
                    coverage_percentage=percentage,
                    layer=self._identify_layer(filename)
                )

        return file_coverage

    def _parse_coverage_sqlite(self) -> Dict[str, FileCoverage]:
        """Parse SQLite coverage (.NET coverage binary)"""
        # Requires coverage.py library
        try:
            from coverage import Coverage
            cov = Coverage(data_file=str(self.coverage_file))
            cov.load()

            file_coverage = {}
            for filename in cov.get_data().measured_files():
                analysis = cov.analysis(filename)
                total = len(analysis[1])  # executable lines
                covered = len(analysis[2])  # executed lines
                missing = list(analysis[3])  # missing lines

                if total > 0:
                    percentage = (covered / total) * 100
                else:
                    percentage = 0.0

                file_coverage[filename] = FileCoverage(
                    file_path=filename,
                    total_lines=total,
                    covered_lines=covered,
                    uncovered_lines=missing,
                    coverage_percentage=percentage,
                    layer=self._identify_layer(filename)
                )

            return file_coverage
        except ImportError:
            raise RuntimeError("coverage.py library required for .coverage file parsing")

    def calculate_layer_coverage(self, file_coverage: Dict[str, FileCoverage]) -> List[LayerCoverage]:
        """Group files by layer and calculate layer-level coverage"""
        layers = {}

        for file_path, coverage in file_coverage.items():
            layer = coverage.layer

            if layer not in layers:
                layers[layer] = {
                    "total_lines": 0,
                    "covered_lines": 0,
                    "files": []
                }

            layers[layer]["total_lines"] += coverage.total_lines
            layers[layer]["covered_lines"] += coverage.covered_lines
            layers[layer]["files"].append(coverage)

        # Calculate percentages and create LayerCoverage objects
        layer_coverage = []
        for layer_name, data in layers.items():
            if data["total_lines"] > 0:
                percentage = (data["covered_lines"] / data["total_lines"]) * 100
            else:
                percentage = 0.0

            layer_coverage.append(LayerCoverage(
                layer_name=layer_name,
                total_lines=data["total_lines"],
                covered_lines=data["covered_lines"],
                coverage_percentage=percentage,
                files=data["files"]
            ))

        # Sort by layer name
        layer_coverage.sort(key=lambda x: x.layer_name)
        return layer_coverage

    def _identify_layer(self, file_path: str) -> str:
        """Identify layer from file path"""
        file_path_lower = file_path.lower()

        for pattern, layer in self.layer_mapping.items():
            if pattern.lower() in file_path_lower:
                return layer

        return "other"

    def generate_html_report(self, output_file: str):
        """Generate HTML report with coverage visualization"""
        file_coverage = self.parse_coverage()
        layer_coverage = self.calculate_layer_coverage(file_coverage)

        # Calculate overall coverage
        total_lines = sum(fc.total_lines for fc in file_coverage.values())
        covered_lines = sum(fc.covered_lines for fc in file_coverage.values())
        overall_percentage = (covered_lines / total_lines * 100) if total_lines > 0 else 0

        # Generate HTML
        html = self._generate_html(overall_percentage, layer_coverage, file_coverage)

        # Write to file
        Path(output_file).write_text(html, encoding='utf-8')
        print(f"✅ Coverage report generated: {output_file}")
        print(f"   Overall coverage: {overall_percentage:.2f}%")

    def _generate_html(self, overall: float, layers: List[LayerCoverage], files: Dict[str, FileCoverage]) -> str:
        """Generate HTML report content"""
        html = f"""<!DOCTYPE html>
<html>
<head>
    <title>Test Coverage Report</title>
    <meta charset="UTF-8">
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: #f5f5f5;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        h1 {{
            color: #333;
            border-bottom: 3px solid #4CAF50;
            padding-bottom: 10px;
        }}
        .summary {{
            background: #f9f9f9;
            padding: 20px;
            border-radius: 5px;
            margin: 20px 0;
        }}
        .summary h2 {{
            margin-top: 0;
            color: #555;
        }}
        .coverage-badge {{
            display: inline-block;
            padding: 10px 20px;
            border-radius: 5px;
            font-size: 24px;
            font-weight: bold;
            margin: 10px 0;
        }}
        .coverage-high {{ background: #4CAF50; color: white; }}
        .coverage-medium {{ background: #FF9800; color: white; }}
        .coverage-low {{ background: #F44336; color: white; }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
        }}
        th, td {{
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }}
        th {{
            background: #4CAF50;
            color: white;
            font-weight: bold;
        }}
        tr:hover {{
            background: #f5f5f5;
        }}
        .percentage {{
            font-weight: bold;
        }}
        .file-section {{
            margin-top: 30px;
        }}
        .timestamp {{
            color: #999;
            font-size: 14px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Test Coverage Report</h1>

        <div class="summary">
            <h2>Overall Coverage</h2>
            <div class="coverage-badge {self._get_coverage_class(overall)}">
                {overall:.2f}%
            </div>
            <p class="timestamp">Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}</p>
        </div>

        <h2>Coverage by Layer</h2>
        <table>
            <tr>
                <th>Layer</th>
                <th>Coverage</th>
                <th>Lines Covered</th>
                <th>Total Lines</th>
            </tr>
"""

        for layer in layers:
            html += f"""            <tr>
                <td>{layer.layer_name.title()}</td>
                <td class="percentage {self._get_coverage_class(layer.coverage_percentage)}">
                    {layer.coverage_percentage:.2f}%
                </td>
                <td>{layer.covered_lines}</td>
                <td>{layer.total_lines}</td>
            </tr>
"""

        html += """        </table>

        <div class="file-section">
            <h2>File Details</h2>
            <table>
                <tr>
                    <th>File</th>
                    <th>Layer</th>
                    <th>Coverage</th>
                    <th>Lines Covered</th>
                    <th>Total Lines</th>
                </tr>
"""

        # Sort files by coverage percentage (lowest first)
        sorted_files = sorted(files.values(), key=lambda f: f.coverage_percentage)

        for file in sorted_files:
            html += f"""                <tr>
                    <td>{file.file_path}</td>
                    <td>{file.layer.title()}</td>
                    <td class="percentage {self._get_coverage_class(file.coverage_percentage)}">
                        {file.coverage_percentage:.2f}%
                    </td>
                    <td>{file.covered_lines}</td>
                    <td>{file.total_lines}</td>
                </tr>
"""

        html += """            </table>
        </div>
    </div>
</body>
</html>"""

        return html

    def _get_coverage_class(self, percentage: float) -> str:
        """Get CSS class based on coverage percentage"""
        if percentage >= 80:
            return "coverage-high"
        elif percentage >= 60:
            return "coverage-medium"
        else:
            return "coverage-low"


def main():
    parser = argparse.ArgumentParser(
        description='Generate coverage report from coverage data',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Generate report from Python coverage
  python generate_coverage_report.py coverage.json --output coverage-report.html

  # Generate report with layer mapping
  python generate_coverage_report.py coverage.xml --source-tree devforgeai/specs/context/source-tree.md

  # Generate report from .NET coverage
  python generate_coverage_report.py coverage.cobertura.xml --output dotnet-coverage.html
        """
    )
    parser.add_argument('coverage_file', help='Coverage data file (.json, .xml, .coverage)')
    parser.add_argument('--source-tree', help='Path to source-tree.md for layer mapping')
    parser.add_argument('--output', default='coverage-report.html', help='Output HTML file')

    args = parser.parse_args()

    try:
        generator = CoverageReportGenerator(args.coverage_file, args.source_tree)
        generator.generate_html_report(args.output)
        return 0
    except Exception as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
