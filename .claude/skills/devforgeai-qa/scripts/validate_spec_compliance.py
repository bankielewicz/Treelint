#!/usr/bin/env python3
"""
Spec Compliance Validator

Validates implementation against story specification by:
- Extracting acceptance criteria from story markdown
- Finding tests that validate each criterion
- Checking if tests pass
- Generating compliance report

Integrates with story-driven development workflow.
"""

import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, asdict
import argparse


@dataclass
class AcceptanceCriterion:
    """Single acceptance criterion"""
    index: int
    text: str
    keywords: List[str]


@dataclass
class TestMatch:
    """Test that matches a criterion"""
    test_file: str
    test_name: str
    test_line: int


@dataclass
class CriterionCompliance:
    """Compliance status for acceptance criterion"""
    criterion: AcceptanceCriterion
    status: str  # PASS, FAIL, NO_TESTS
    matching_tests: List[TestMatch]
    failing_tests: List[str]
    message: str


@dataclass
class ComplianceReport:
    """Complete spec compliance report"""
    story_id: str
    total_criteria: int
    criteria_pass: int
    criteria_fail: int
    criteria_no_tests: int
    compliance_percentage: float
    criteria: List[CriterionCompliance]


class SpecComplianceValidator:
    """Validate implementation against story specification"""

    def __init__(self, story_file: str, test_dir: str):
        self.story_file = Path(story_file)
        self.test_dir = Path(test_dir)

    def validate(self) -> ComplianceReport:
        """Run spec compliance validation"""
        # Parse story file
        story_id, criteria = self._parse_story()

        # Find tests for each criterion
        criterion_compliance = []
        for criterion in criteria:
            compliance = self._validate_criterion(criterion)
            criterion_compliance.append(compliance)

        # Calculate statistics
        total = len(criteria)
        passed = sum(1 for c in criterion_compliance if c.status == 'PASS')
        failed = sum(1 for c in criterion_compliance if c.status == 'FAIL')
        no_tests = sum(1 for c in criterion_compliance if c.status == 'NO_TESTS')

        compliance_pct = (passed / total * 100) if total > 0 else 0.0

        return ComplianceReport(
            story_id=story_id,
            total_criteria=total,
            criteria_pass=passed,
            criteria_fail=failed,
            criteria_no_tests=no_tests,
            compliance_percentage=round(compliance_pct, 2),
            criteria=criterion_compliance
        )

    def _parse_story(self) -> Tuple[str, List[AcceptanceCriterion]]:
        """Parse story file and extract acceptance criteria"""
        with open(self.story_file, 'r', encoding='utf-8') as f:
            content = f.read()

        # Extract story ID from frontmatter or filename
        story_id_match = re.search(r'id:\s*(\S+)', content)
        if story_id_match:
            story_id = story_id_match.group(1)
        else:
            story_id = self.story_file.stem

        # Extract acceptance criteria section
        criteria_section = re.search(
            r'##\s*Acceptance\s*Criteria\s*\n(.*?)(?=\n##|\Z)',
            content,
            re.DOTALL | re.IGNORECASE
        )

        if not criteria_section:
            raise ValueError("No acceptance criteria section found in story file")

        criteria_text = criteria_section.group(1)

        # Parse individual criteria (numbered list or checkboxes)
        criteria_matches = re.findall(
            r'(?:\d+\.|\-\s*\[[ x]\])\s*(.+?)(?=\n(?:\d+\.|\-\s*\[|##|\Z))',
            criteria_text,
            re.DOTALL
        )

        criteria = []
        for i, text in enumerate(criteria_matches, 1):
            text = text.strip()
            keywords = self._extract_keywords(text)

            criteria.append(AcceptanceCriterion(
                index=i,
                text=text,
                keywords=keywords
            ))

        if not criteria:
            raise ValueError("No acceptance criteria found")

        return story_id, criteria

    def _extract_keywords(self, text: str) -> List[str]:
        """Extract searchable keywords from criterion text"""
        # Remove common words
        stop_words = {'the', 'a', 'an', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for',
                      'of', 'with', 'by', 'from', 'as', 'is', 'are', 'can', 'should', 'must'}

        # Extract words (alphanumeric)
        words = re.findall(r'\b[a-zA-Z]{3,}\b', text.lower())

        # Filter stop words and get unique keywords
        keywords = [w for w in words if w not in stop_words]

        # Return top keywords (most relevant)
        return list(dict.fromkeys(keywords))[:5]

    def _validate_criterion(self, criterion: AcceptanceCriterion) -> CriterionCompliance:
        """Validate single acceptance criterion"""
        # Find matching tests
        matching_tests = self._find_matching_tests(criterion)

        if not matching_tests:
            return CriterionCompliance(
                criterion=criterion,
                status='NO_TESTS',
                matching_tests=[],
                failing_tests=[],
                message=f"No tests found for criterion: {criterion.text[:50]}..."
            )

        # Run tests
        failing_tests = self._run_tests(matching_tests)

        if failing_tests:
            return CriterionCompliance(
                criterion=criterion,
                status='FAIL',
                matching_tests=matching_tests,
                failing_tests=failing_tests,
                message=f"{len(failing_tests)} test(s) failing"
            )
        else:
            return CriterionCompliance(
                criterion=criterion,
                status='PASS',
                matching_tests=matching_tests,
                failing_tests=[],
                message=f"All {len(matching_tests)} test(s) passing"
            )

    def _find_matching_tests(self, criterion: AcceptanceCriterion) -> List[TestMatch]:
        """Find tests that validate criterion"""
        matching_tests = []

        # Get all test files
        test_files = list(self.test_dir.rglob('*[Tt]est*.py')) + \
                     list(self.test_dir.rglob('*[Tt]est*.cs')) + \
                     list(self.test_dir.rglob('*.test.js')) + \
                     list(self.test_dir.rglob('*.test.ts'))

        for test_file in test_files:
            try:
                with open(test_file, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()

                # Find test methods mentioning keywords
                for keyword in criterion.keywords:
                    # Pattern for test methods in various languages
                    patterns = [
                        rf'(def\s+test_\w*{keyword}\w*\()',  # Python
                        rf'(\[(?:Fact|Test)\]\s+public\s+.*{keyword}\w*\()',  # C#
                        rf'(it\([\'"].*{keyword}.*[\'"],)',  # JavaScript/TypeScript
                        rf'(test\([\'"].*{keyword}.*[\'"],)'  # Jest
                    ]

                    for pattern in patterns:
                        matches = re.finditer(pattern, content, re.IGNORECASE)
                        for match in matches:
                            # Extract test name
                            test_name = self._extract_test_name(match.group(1))
                            test_line = content[:match.start()].count('\n') + 1

                            matching_tests.append(TestMatch(
                                test_file=str(test_file),
                                test_name=test_name,
                                test_line=test_line
                            ))

            except Exception as e:
                print(f"⚠️  Failed to read {test_file}: {e}", file=sys.stderr)
                continue

        # Remove duplicates
        unique_tests = []
        seen = set()
        for test in matching_tests:
            key = (test.test_file, test.test_name)
            if key not in seen:
                seen.add(key)
                unique_tests.append(test)

        return unique_tests

    def _extract_test_name(self, match_text: str) -> str:
        """Extract clean test name from match"""
        # Extract function/method name
        name_match = re.search(r'(?:def|public|it|test)\s+(\w+)', match_text)
        if name_match:
            return name_match.group(1)

        # Extract from string literal
        string_match = re.search(r'[\'"](.+?)[\'"]', match_text)
        if string_match:
            return string_match.group(1)

        return match_text.strip()

    def _run_tests(self, tests: List[TestMatch]) -> List[str]:
        """Run tests and return list of failing test names"""
        # Group tests by file extension to determine test runner
        test_files = set(t.test_file for t in tests)
        failing = []

        for test_file in test_files:
            ext = Path(test_file).suffix

            try:
                if ext == '.py':
                    # Run pytest
                    result = subprocess.run(
                        ['pytest', test_file, '-v'],
                        capture_output=True,
                        text=True,
                        timeout=60
                    )
                elif ext == '.cs':
                    # Run dotnet test
                    result = subprocess.run(
                        ['dotnet', 'test', test_file],
                        capture_output=True,
                        text=True,
                        timeout=60
                    )
                elif ext in ['.js', '.ts']:
                    # Run npm test
                    result = subprocess.run(
                        ['npm', 'test', '--', test_file],
                        capture_output=True,
                        text=True,
                        timeout=60
                    )
                else:
                    continue

                # Check if tests passed
                if result.returncode != 0:
                    # Extract failing test names from output
                    for test in tests:
                        if test.test_file == test_file:
                            failing.append(test.test_name)

            except subprocess.TimeoutExpired:
                print(f"⚠️  Test timeout: {test_file}", file=sys.stderr)
                for test in tests:
                    if test.test_file == test_file:
                        failing.append(test.test_name)
            except FileNotFoundError:
                print(f"⚠️  Test runner not found for {test_file}", file=sys.stderr)
                continue

        return failing

    def generate_json_report(self, output_file: str):
        """Generate JSON compliance report"""
        report = self.validate()

        # Convert to dict
        report_dict = {
            'story_id': report.story_id,
            'summary': {
                'total_criteria': report.total_criteria,
                'criteria_pass': report.criteria_pass,
                'criteria_fail': report.criteria_fail,
                'criteria_no_tests': report.criteria_no_tests,
                'compliance_percentage': report.compliance_percentage
            },
            'criteria': [
                {
                    'index': c.criterion.index,
                    'text': c.criterion.text,
                    'status': c.status,
                    'matching_tests': [asdict(t) for t in c.matching_tests],
                    'failing_tests': c.failing_tests,
                    'message': c.message
                }
                for c in report.criteria
            ]
        }

        # Write JSON
        with open(output_file, 'w') as f:
            json.dump(report_dict, f, indent=2)

        print(f"✅ Compliance report generated: {output_file}")
        print(f"   Story: {report.story_id}")
        print(f"   Compliance: {report.compliance_percentage}%")
        print(f"   Passing: {report.criteria_pass}/{report.total_criteria}")
        print(f"   No tests: {report.criteria_no_tests}")

        return report

    def print_compliance(self):
        """Print compliance results to console"""
        report = self.validate()

        print(f"\n📋 Spec Compliance Report: {report.story_id}")
        print(f"   Compliance: {report.compliance_percentage}%")
        print(f"   Criteria passing: {report.criteria_pass}/{report.total_criteria}")
        print(f"   Criteria failing: {report.criteria_fail}")
        print(f"   No tests: {report.criteria_no_tests}")

        if report.criteria_fail > 0 or report.criteria_no_tests > 0:
            print("\n❌ Compliance issues found:\n")

            for c in report.criteria:
                if c.status != 'PASS':
                    print(f"   {c.criterion.index}. {c.criterion.text[:60]}...")
                    print(f"      Status: {c.status}")
                    print(f"      {c.message}\n")
        else:
            print("\n✅ All acceptance criteria validated!")


def main():
    parser = argparse.ArgumentParser(
        description='Validate implementation against story specification',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Validate story against tests
  python validate_spec_compliance.py ai_docs/Stories/1.1.story.md tests/

  # Generate JSON report
  python validate_spec_compliance.py story.md tests/ --output compliance.json
        """
    )
    parser.add_argument('story_file', help='Story markdown file path')
    parser.add_argument('test_dir', help='Test directory path')
    parser.add_argument('--output', help='Output JSON file')

    args = parser.parse_args()

    try:
        validator = SpecComplianceValidator(args.story_file, args.test_dir)

        if args.output:
            validator.generate_json_report(args.output)
        else:
            validator.print_compliance()

        return 0
    except Exception as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        return 1


if __name__ == '__main__':
    sys.exit(main())
