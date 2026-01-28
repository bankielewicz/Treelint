#!/usr/bin/env python3
"""
Requirements Validator - DevForgeAI Ideation Skill

Validates requirements documents for completeness, testability, and quality.
Detects ambiguous language and missing acceptance criteria.

Usage:
    python requirements_validator.py --spec requirements.md
    python requirements_validator.py --epic EPIC-001.epic.md
    python requirements_validator.py --spec requirements.md --strict

Output:
    - validation_result: pass / fail / warnings
    - issues: List of detected issues with severity
    - recommendations: Suggested improvements
"""

import re
import argparse
import sys
from pathlib import Path
from typing import List, Dict, Tuple


class RequirementsValidator:
    """Validate requirements documents for quality and completeness."""

    # Ambiguous terms that should have metrics
    VAGUE_TERMS = [
        r'\bfast\b',
        r'\bscalable\b',
        r'\bsecure\b',
        r'\bintuitive\b',
        r'\buser-friendly\b',
        r'\buser friendly\b',
        r'\beasy to use\b',
        r'\bsimple\b',
        r'\brobust\b',
        r'\breliable\b',
        r'\befficient\b',
        r'\bperformant\b',
        r'\bhigh quality\b',
        r'\breal-time\b' + r'(?!\s*\()',  # Only flag if no metric follows
    ]

    # Required sections in requirements spec
    REQUIRED_SECTIONS = [
        "Problem Statement",
        "Solution Overview",
        "Functional Requirements",
        "Non-Functional Requirements",
        "Success Criteria"
    ]

    # Required sections in epic
    EPIC_REQUIRED_SECTIONS = [
        "Business Goal",
        "Success Metrics",
        "Requirements"
    ]

    def __init__(self, strict_mode: bool = False):
        self.strict_mode = strict_mode
        self.issues = []
        self.warnings = []
        self.recommendations = []

    def validate_file(self, filepath: str) -> Tuple[str, List[Dict], List[str]]:
        """
        Validate requirements document.

        Args:
            filepath: Path to requirements document

        Returns:
            Tuple of (result, issues, recommendations)
            result: "pass", "warnings", or "fail"
        """
        path = Path(filepath)

        if not path.exists():
            self.issues.append({
                "severity": "critical",
                "message": f"File not found: {filepath}",
                "line": 0
            })
            return "fail", self.issues, self.recommendations

        content = path.read_text()

        # Determine document type
        if "epic.md" in filepath.lower():
            doc_type = "epic"
        else:
            doc_type = "requirements"

        # Run validation checks
        self._check_required_sections(content, doc_type)
        self._check_vague_language(content)
        self._check_acceptance_criteria(content, doc_type)
        self._check_user_stories(content)
        self._check_nfr_quality(content, doc_type)
        self._check_success_metrics(content, doc_type)

        # Determine result
        if self.issues:
            result = "fail"
        elif self.warnings:
            result = "warnings"
        else:
            result = "pass"

        return result, self.issues + self.warnings, self.recommendations

    def _check_required_sections(self, content: str, doc_type: str):
        """Check if all required sections are present."""
        required = (
            self.EPIC_REQUIRED_SECTIONS if doc_type == "epic"
            else self.REQUIRED_SECTIONS
        )

        for section in required:
            # Check for section header (supports ## or ### or ####)
            pattern = rf'^#+\s+{re.escape(section)}'
            if not re.search(pattern, content, re.MULTILINE | re.IGNORECASE):
                self.issues.append({
                    "severity": "high",
                    "message": f"Missing required section: '{section}'",
                    "line": 0
                })

    def _check_vague_language(self, content: str):
        """Check for ambiguous terms that need specific metrics."""
        lines = content.split('\n')

        for line_num, line in enumerate(lines, 1):
            # Skip markdown headers, code blocks, and links
            if line.strip().startswith('#') or line.strip().startswith('```'):
                continue
            if '[' in line and '](' in line:  # Skip markdown links
                continue

            for vague_term_pattern in self.VAGUE_TERMS:
                matches = re.finditer(vague_term_pattern, line, re.IGNORECASE)
                for match in matches:
                    term = match.group(0)

                    # Check if metric follows (within 50 chars)
                    context = line[match.end():match.end() + 50]

                    # Look for metrics: numbers, < >, parentheses with numbers
                    has_metric = bool(re.search(r'[\d<>]|\(\d', context))

                    if not has_metric:
                        severity = "high" if self.strict_mode else "medium"
                        self.warnings.append({
                            "severity": severity,
                            "message": f"Vague term '{term}' without specific metric",
                            "line": line_num,
                            "context": line.strip()[:80]
                        })

                        self.recommendations.append(
                            f"Line {line_num}: Replace '{term}' with specific metric "
                            f"(e.g., '<500ms response time' instead of 'fast')"
                        )

    def _check_acceptance_criteria(self, content: str, doc_type: str):
        """Check for presence and quality of acceptance criteria."""
        # Look for user stories
        user_story_pattern = r'As a .+, I want .+, so that'
        user_stories = re.findall(user_story_pattern, content, re.IGNORECASE)

        if not user_stories:
            if doc_type != "epic":  # Epics might not have detailed user stories
                self.warnings.append({
                    "severity": "medium",
                    "message": "No user stories found (As a... I want... So that...)",
                    "line": 0
                })

        # Check for acceptance criteria near user stories
        sections = content.split('\n\n')

        for i, section in enumerate(sections):
            if re.search(user_story_pattern, section, re.IGNORECASE):
                # Look in this section and next few sections for acceptance criteria
                context = '\n\n'.join(sections[i:i+3])

                # Look for acceptance criteria markers
                has_ac = bool(re.search(
                    r'(acceptance criteria|given.+when.+then|\\[\\s*\\])',
                    context,
                    re.IGNORECASE
                ))

                if not has_ac:
                    self.warnings.append({
                        "severity": "high",
                        "message": "User story found without acceptance criteria",
                        "line": 0,
                        "context": section[:100]
                    })

                    self.recommendations.append(
                        "Add acceptance criteria for each user story "
                        "(checkbox list or Given/When/Then format)"
                    )

    def _check_user_stories(self, content: str):
        """Check quality of user stories."""
        # Find all user stories
        pattern = r'As a (.+?), I want (.+?), so that (.+?)(?:\.|$)'
        matches = re.finditer(pattern, content, re.IGNORECASE | re.DOTALL)

        for match in matches:
            role = match.group(1).strip()
            action = match.group(2).strip()
            benefit = match.group(3).strip()

            # Check for placeholders
            if '[' in role or 'TODO' in role.upper() or 'TBD' in role.upper():
                self.warnings.append({
                    "severity": "high",
                    "message": f"User story has placeholder role: '{role}'",
                    "line": 0
                })

            # Check if benefit is too vague
            if len(benefit) < 10:
                self.warnings.append({
                    "severity": "medium",
                    "message": f"User story benefit is too vague: '{benefit}'",
                    "line": 0
                })

                self.recommendations.append(
                    f"Expand benefit statement '{benefit}' to explain specific value"
                )

    def _check_nfr_quality(self, content: str, doc_type: str):
        """Check quality of non-functional requirements."""
        if doc_type == "epic":
            nfr_section = re.search(
                r'#+\s*Non-Functional Requirements.*?(?=#+|$)',
                content,
                re.IGNORECASE | re.DOTALL
            )
        else:
            nfr_section = re.search(
                r'#+\s*Non-Functional Requirements.*?(?=#+|$)',
                content,
                re.IGNORECASE | re.DOTALL
            )

        if not nfr_section:
            self.warnings.append({
                "severity": "high",
                "message": "Non-functional requirements section not found or empty",
                "line": 0
            })
            return

        nfr_text = nfr_section.group(0)

        # Check for specific NFR categories
        nfr_categories = {
            "Performance": r'\bperformance\b',
            "Security": r'\bsecurity\b',
            "Scalability": r'\bscalability\b',
            "Availability": r'\bavailability\b'
        }

        missing_categories = []
        for category, pattern in nfr_categories.items():
            if not re.search(pattern, nfr_text, re.IGNORECASE):
                missing_categories.append(category)

        if missing_categories:
            self.warnings.append({
                "severity": "medium",
                "message": f"NFR section missing categories: {', '.join(missing_categories)}",
                "line": 0
            })

            self.recommendations.append(
                f"Add NFR categories: {', '.join(missing_categories)}"
            )

        # Check for metrics in NFRs
        # Should have patterns like: < 500ms, > 99%, X concurrent users
        metric_pattern = r'[<>≤≥]\s*\d+|99\.\d+%|\d+\s*(ms|sec|second|min|minute|user|request)'
        metrics_found = len(re.findall(metric_pattern, nfr_text, re.IGNORECASE))

        if metrics_found < 3:
            self.warnings.append({
                "severity": "medium",
                "message": f"NFRs have few specific metrics ({metrics_found} found, recommend 5+)",
                "line": 0
            })

            self.recommendations.append(
                "Add specific metrics to NFRs (e.g., '<500ms response time', '99.9% uptime')"
            )

    def _check_success_metrics(self, content: str, doc_type: str):
        """Check quality of success metrics."""
        # Find success metrics section
        metrics_section = re.search(
            r'#+\s*Success (Metrics|Criteria).*?(?=#+|$)',
            content,
            re.IGNORECASE | re.DOTALL
        )

        if not metrics_section:
            self.issues.append({
                "severity": "high",
                "message": "Success metrics/criteria section not found",
                "line": 0
            })
            return

        metrics_text = metrics_section.group(0)

        # Check for quantifiable metrics
        # Should have numbers, percentages, or measurable outcomes
        quantifiable_pattern = r'\d+[%]?|\bmeasure\b|\btrack\b|\bmetric\b'
        quantifiable_count = len(re.findall(quantifiable_pattern, metrics_text, re.IGNORECASE))

        if quantifiable_count < 2:
            self.warnings.append({
                "severity": "high",
                "message": "Success metrics should be quantifiable (found few numbers/percentages)",
                "line": 0
            })

            self.recommendations.append(
                "Make success metrics quantifiable (e.g., '500 signups in 30 days', '90% task completion')"
            )

    def generate_report(self, result: str, issues: List[Dict], recommendations: List[str]) -> str:
        """Generate human-readable validation report."""
        report = []
        report.append("=" * 60)
        report.append("Requirements Validation Report")
        report.append("=" * 60)
        report.append(f"\nResult: {result.upper()}\n")

        if not issues and not recommendations:
            report.append("✅ All validation checks passed!")
            report.append("\nNo issues found. Requirements document is complete and well-formed.")
            return '\n'.join(report)

        # Group issues by severity
        critical_issues = [i for i in issues if i['severity'] == 'critical']
        high_issues = [i for i in issues if i['severity'] == 'high']
        medium_issues = [i for i in issues if i['severity'] == 'medium']

        if critical_issues:
            report.append("\n❌ CRITICAL ISSUES:")
            for issue in critical_issues:
                report.append(f"  - {issue['message']}")
                if 'context' in issue:
                    report.append(f"    Context: {issue['context']}")

        if high_issues:
            report.append("\n⚠️  HIGH PRIORITY ISSUES:")
            for issue in high_issues:
                report.append(f"  - Line {issue['line']}: {issue['message']}")
                if 'context' in issue:
                    report.append(f"    Context: {issue['context']}")

        if medium_issues:
            report.append("\n⚠️  MEDIUM PRIORITY WARNINGS:")
            for issue in medium_issues:
                report.append(f"  - Line {issue['line']}: {issue['message']}")
                if 'context' in issue:
                    report.append(f"    Context: {issue['context'][:60]}...")

        if recommendations:
            report.append("\n💡 RECOMMENDATIONS:")
            for rec in recommendations[:10]:  # Limit to top 10
                report.append(f"  - {rec}")

        report.append("\n" + "=" * 60)
        report.append(f"Total Issues: {len(issues)}")
        report.append(f"Total Recommendations: {len(recommendations)}")
        report.append("=" * 60)

        return '\n'.join(report)


def main():
    parser = argparse.ArgumentParser(
        description="Validate requirements documents for completeness and quality"
    )
    parser.add_argument(
        "--spec",
        help="Path to requirements specification file"
    )
    parser.add_argument(
        "--epic",
        help="Path to epic document file"
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Strict mode: treat warnings as errors"
    )

    args = parser.parse_args()

    if not args.spec and not args.epic:
        print("Error: Must specify either --spec or --epic", file=sys.stderr)
        parser.print_help()
        sys.exit(1)

    filepath = args.spec or args.epic

    # Validate file
    validator = RequirementsValidator(strict_mode=args.strict)
    result, issues, recommendations = validator.validate_file(filepath)

    # Generate and print report
    report = validator.generate_report(result, issues, recommendations)
    print(report)

    # Exit with appropriate code
    if result == "fail":
        sys.exit(1)
    elif result == "warnings" and args.strict:
        sys.exit(1)
    else:
        sys.exit(0)


if __name__ == "__main__":
    main()
