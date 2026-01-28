"""
GrepFallbackAnalyzer - Grep-based pattern matching fallback.

Used when ast-grep is unavailable. Provides ~60-75% accuracy
compared to ast-grep's 90-95% accuracy.

Output format matches ast-grep schema for interoperability.
"""

import re
import json
import logging
from pathlib import Path
from typing import List, Dict, Tuple, Optional
from dataclasses import dataclass, field, asdict

logger = logging.getLogger(__name__)


# =============================================================================
# Data Classes
# =============================================================================

@dataclass
class GrepPattern:
    """Pattern definition for grep-based detection."""
    id: str
    pattern: str  # Regex pattern
    severity: str  # CRITICAL, HIGH, MEDIUM, LOW
    message: str
    category: str  # security, anti-pattern, etc.
    languages: List[str] = field(default_factory=list)


@dataclass
class Violation:
    """Standardized violation result."""
    file: str
    line: int
    column: int
    rule_id: str
    severity: str
    message: str
    evidence: str
    analysis_method: str = "grep-fallback"
    category: str = "security"

    def to_dict(self) -> Dict:
        """Convert to dictionary for JSON serialization."""
        return asdict(self)


# =============================================================================
# Utility Functions
# =============================================================================

def log_fallback_warning() -> None:
    """Log warning about reduced accuracy in fallback mode."""
    logger.warning(
        "Using grep-based fallback for code analysis. "
        "Accuracy: 60-75% (vs 90-95% with ast-grep). "
        "Install ast-grep-cli for better results: pip install ast-grep-cli"
    )


# =============================================================================
# Main Analyzer Class
# =============================================================================

class GrepFallbackAnalyzer:
    """Grep-based pattern matching for code analysis."""

    # Built-in security patterns
    SECURITY_PATTERNS: List[GrepPattern] = [
        GrepPattern(
            id="SEC-001",
            pattern=r'(SELECT|INSERT|UPDATE|DELETE).*WHERE.*[\+\{]|[\+\{].*WHERE|execute\(["\']SELECT.*[\+\{]|f["\']SELECT.*\{|\.format\(.*SELECT',
            severity="CRITICAL",
            message="Potential SQL injection via string concatenation",
            category="security",
            languages=["python", "py", "csharp", "cs"]
        ),
        GrepPattern(
            id="SEC-002",
            pattern=r'(API_KEY|SECRET|PASSWORD|TOKEN|ACCESS_KEY)\s*=\s*["\']\w{8,}["\']',
            severity="HIGH",
            message="Hardcoded secret detected",
            category="security",
            languages=["python", "py", "javascript", "js", "typescript", "ts", "csharp", "cs"]
        ),
        GrepPattern(
            id="SEC-003",
            pattern=r'(AWS_ACCESS_KEY|AWS_SECRET|AKIA[0-9A-Z]{16})',
            severity="CRITICAL",
            message="AWS credentials detected",
            category="security",
            languages=["python", "py", "javascript", "js", "typescript", "ts", "csharp", "cs"]
        ),
    ]

    def __init__(self, custom_patterns: Optional[List[GrepPattern]] = None):
        """
        Initialize analyzer with patterns.

        Args:
            custom_patterns: Additional patterns to use
        """
        self.patterns = self.SECURITY_PATTERNS.copy()
        if custom_patterns:
            self.patterns.extend(custom_patterns)

    def analyze_file(self, file_path: str) -> List[Dict]:
        """
        Analyze single file with grep patterns.

        Args:
            file_path: Path to file to analyze

        Returns:
            List of violations found (as dicts for compatibility)
        """
        violations = []
        file_path_obj = Path(file_path)

        if not file_path_obj.exists():
            return violations

        if not file_path_obj.is_file():
            return violations

        # Skip binary files
        try:
            with open(file_path_obj, 'r', encoding='utf-8') as f:
                content = f.read()
        except (UnicodeDecodeError, PermissionError):
            # Binary file or permission denied - skip gracefully
            return violations

        # Apply each pattern
        for pattern in self.patterns:
            # Check if pattern applies to this file type
            if pattern.languages:
                file_ext = file_path_obj.suffix.lstrip('.')
                if file_ext not in pattern.languages and not any(lang in file_ext for lang in pattern.languages):
                    continue

            # Search for pattern in content
            matches = re.finditer(pattern.pattern, content, re.MULTILINE | re.IGNORECASE)

            for match in matches:
                # Find line number and column
                line_num = content[:match.start()].count('\n') + 1
                line_start = content.rfind('\n', 0, match.start()) + 1
                column = match.start() - line_start + 1

                # Extract evidence (the matching line)
                line_end = content.find('\n', match.start())
                if line_end == -1:
                    line_end = len(content)
                evidence = content[line_start:line_end].strip()

                violation = Violation(
                    file=str(file_path_obj),
                    line=line_num,
                    column=column,
                    rule_id=pattern.id,
                    severity=pattern.severity,
                    message=pattern.message,
                    evidence=evidence,
                    analysis_method="grep-fallback",
                    category=pattern.category
                )

                violations.append(violation.to_dict())

        return violations

    def analyze_directory(self, directory: str,
                          category: Optional[str] = None,
                          language: Optional[str] = None) -> List[Dict]:
        """
        Analyze directory recursively.

        Args:
            directory: Directory to scan
            category: Filter by category (security, anti-patterns)
            language: Filter by language (python, typescript, csharp)

        Returns:
            List of all violations found
        """
        all_violations = []
        directory_path = Path(directory)

        if not directory_path.exists() or not directory_path.is_dir():
            return all_violations

        # Find all relevant files
        if language:
            # Map language to file extensions
            ext_map = {
                'python': ['.py'],
                'typescript': ['.ts', '.tsx'],
                'javascript': ['.js', '.jsx'],
                'csharp': ['.cs']
            }
            extensions = ext_map.get(language, [])
        else:
            extensions = ['.py', '.ts', '.tsx', '.js', '.jsx', '.cs']

        # Scan all files
        for ext in extensions:
            for file_path in directory_path.rglob(f'*{ext}'):
                if file_path.is_file():
                    violations = self.analyze_file(str(file_path))

                    # Filter by category if specified
                    if category:
                        violations = [v for v in violations if v.get('category') == category]

                    all_violations.extend(violations)

        return all_violations

    def format_results(self, violations: List[Dict], format: str = "json") -> str:
        """
        Format results for output.

        Args:
            violations: List of violations
            format: Output format (json, text, markdown)

        Returns:
            Formatted output string
        """
        if format == "json":
            return self._format_json(violations)
        elif format == "text":
            return self._format_text(violations)
        elif format == "markdown":
            return self._format_markdown(violations)
        else:
            return self._format_json(violations)

    def _format_json(self, violations: List[Dict]) -> str:
        """Format as JSON matching ast-grep schema."""
        # Calculate summary
        by_severity = {}
        for v in violations:
            severity = v.get('severity', 'UNKNOWN')
            by_severity[severity] = by_severity.get(severity, 0) + 1

        result = {
            "violations": violations,
            "analysis_method": "grep-fallback",
            "summary": {
                "total_violations": len(violations),
                "by_severity": by_severity,
                "accuracy_note": "60-75% vs 90-95% with ast-grep"
            }
        }

        return json.dumps(result, indent=2)

    def _format_text(self, violations: List[Dict]) -> str:
        """Format as human-readable text."""
        if not violations:
            return "No violations found.\n"

        lines = []
        lines.append(f"Found {len(violations)} violation(s):\n")

        for v in violations:
            lines.append(f"{v.get('severity', 'UNKNOWN')}: {v.get('file', 'unknown')}")
            lines.append(f"  Line {v.get('line', '?')}, Column {v.get('column', '?')}")
            lines.append(f"  {v.get('message', 'No message')}")
            lines.append(f"  Evidence: {v.get('evidence', 'N/A')}")
            lines.append("")

        return "\n".join(lines)

    def _format_markdown(self, violations: List[Dict]) -> str:
        """Format as Markdown."""
        if not violations:
            return "## Code Analysis Results\n\nNo violations found.\n"

        lines = []
        lines.append("## Code Analysis Results\n")
        lines.append(f"**Total Violations:** {len(violations)}\n")
        lines.append("**Analysis Method:** grep-fallback (60-75% accuracy)\n")
        lines.append("### Violations\n")

        for v in violations:
            lines.append(f"#### {v.get('severity', 'UNKNOWN')}: {v.get('rule_id', 'UNKNOWN')}\n")
            lines.append(f"**File:** `{v.get('file', 'unknown')}`  ")
            lines.append(f"**Line:** {v.get('line', '?')}, **Column:** {v.get('column', '?')}\n")
            lines.append(f"**Message:** {v.get('message', 'No message')}\n")
            lines.append(f"**Evidence:**\n```\n{v.get('evidence', 'N/A')}\n```\n")

        return "\n".join(lines)
