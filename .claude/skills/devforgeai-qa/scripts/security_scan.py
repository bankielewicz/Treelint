#!/usr/bin/env python3
"""
Security Vulnerability Scanner

Scans codebase for common security vulnerabilities:
- SQL injection (CWE-89)
- XSS (CWE-79)
- Hardcoded secrets
- Insecure cryptography
- Path traversal

Generates JSON security report with CWE categorization.
"""

import json
import re
import sys
from pathlib import Path
from typing import Dict, List
from dataclasses import dataclass, asdict
import argparse


@dataclass
class SecurityViolation:
    """Single security violation"""
    severity: str  # CRITICAL, HIGH, MEDIUM, LOW
    category: str
    cwe: str
    file: str
    line: int
    code_snippet: str
    description: str
    remediation: str


@dataclass
class SecurityReport:
    """Complete security scan report"""
    total_files_scanned: int
    violations_critical: int
    violations_high: int
    violations_medium: int
    violations_low: int
    violations: List[SecurityViolation]


class SecurityScanner:
    """Scan codebase for security vulnerabilities"""

    def __init__(self, source_path: str):
        self.source_path = Path(source_path)

        # Define vulnerability patterns
        self.patterns = {
            'sql_injection': {
                'patterns': [
                    r'ExecuteRawSql\s*\([^)]*\+',  # C# string concatenation in SQL
                    r'string\.Format\s*\([^)]*SELECT',  # String.Format with SQL
                    r'f["\']SELECT.*\{',  # Python f-string SQL
                    r'`SELECT.*\$\{',  # JavaScript template literal SQL
                    r'\.query\s*\([^)]*\+',  # Generic query with concatenation
                ],
                'severity': 'CRITICAL',
                'category': 'SQL Injection',
                'cwe': 'CWE-89',
                'description': 'SQL query uses string concatenation - vulnerable to SQL injection',
                'remediation': 'Use parameterized queries with bound parameters'
            },
            'xss': {
                'patterns': [
                    r'\.innerHTML\s*=',  # JavaScript innerHTML assignment
                    r'dangerouslySetInnerHTML',  # React dangerous prop
                    r'document\.write\s*\(',  # document.write
                    r'\.html\s*\([^)]*\+',  # jQuery .html() with concatenation
                ],
                'severity': 'CRITICAL',
                'category': 'Cross-Site Scripting (XSS)',
                'cwe': 'CWE-79',
                'description': 'Unsafe HTML insertion - XSS vulnerability',
                'remediation': 'Use safe DOM methods, sanitize input, or use framework escape functions'
            },
            'hardcoded_secrets': {
                'patterns': [
                    r'(?i)(password|apikey|api_key|secret|token|connectionstring)\s*=\s*["\'][^"\']{8,}["\']',
                    r'(?i)Bearer\s+[A-Za-z0-9_-]{20,}',  # JWT tokens
                    r'(?i)sk_live_[A-Za-z0-9]{24,}',  # Stripe keys
                    r'(?i)AKIA[0-9A-Z]{16}',  # AWS keys
                ],
                'severity': 'CRITICAL',
                'category': 'Hardcoded Secret',
                'cwe': 'CWE-798',
                'description': 'Hardcoded secret or credential detected',
                'remediation': 'Move secrets to environment variables or secret manager'
            },
            'weak_crypto': {
                'patterns': [
                    r'\bMD5\b',
                    r'\bSHA1\b',
                    r'\bDES\.',
                    r'\bTripleDES\b',
                    r'crypto\.createHash\s*\(\s*["\']md5["\']',
                ],
                'severity': 'HIGH',
                'category': 'Weak Cryptography',
                'cwe': 'CWE-327',
                'description': 'Using weak or broken cryptographic algorithm',
                'remediation': 'Use SHA-256, SHA-384, or SHA-512 for hashing; AES for encryption'
            },
            'path_traversal': {
                'patterns': [
                    r'File\s*\(\s*[^)]*\+',  # File operations with concatenation
                    r'Path\.Combine\s*\([^)]*userInput',
                    r'open\s*\([^)]*\+',  # Python open with concatenation
                    r'fs\.readFile\s*\([^)]*\+',  # Node.js file read with concatenation
                ],
                'severity': 'HIGH',
                'category': 'Path Traversal',
                'cwe': 'CWE-22',
                'description': 'File path constructed from user input - path traversal risk',
                'remediation': 'Validate and sanitize file paths, use whitelist of allowed paths'
            },
            'insecure_random': {
                'patterns': [
                    r'\bRandom\s*\(',  # C# System.Random
                    r'Math\.random\s*\(',  # JavaScript Math.random
                    r'random\.random\s*\(',  # Python random (not secrets)
                ],
                'severity': 'MEDIUM',
                'category': 'Insecure Randomness',
                'cwe': 'CWE-330',
                'description': 'Using non-cryptographic random number generator',
                'remediation': 'Use cryptographically secure RNG (RandomNumberGenerator, crypto.getRandomValues, secrets module)'
            },
            'command_injection': {
                'patterns': [
                    r'exec\s*\([^)]*\+',  # Command execution with concatenation
                    r'eval\s*\(',  # eval
                    r'Process\.Start\s*\([^)]*\+',  # C# process start
                    r'subprocess\.call\s*\([^)]*\+',  # Python subprocess
                ],
                'severity': 'CRITICAL',
                'category': 'Command Injection',
                'cwe': 'CWE-78',
                'description': 'Command execution with unsanitized input',
                'remediation': 'Avoid dynamic command construction, use parameterized APIs'
            }
        }

    def scan(self) -> SecurityReport:
        """Run security scan"""
        violations = []

        # Get all source files
        source_files = self._get_source_files()

        for file_path in source_files:
            file_violations = self._scan_file(file_path)
            violations.extend(file_violations)

        # Calculate statistics
        critical = sum(1 for v in violations if v.severity == 'CRITICAL')
        high = sum(1 for v in violations if v.severity == 'HIGH')
        medium = sum(1 for v in violations if v.severity == 'MEDIUM')
        low = sum(1 for v in violations if v.severity == 'LOW')

        return SecurityReport(
            total_files_scanned=len(source_files),
            violations_critical=critical,
            violations_high=high,
            violations_medium=medium,
            violations_low=low,
            violations=violations
        )

    def _get_source_files(self) -> List[Path]:
        """Get all source files to scan"""
        patterns = ['*.py', '*.cs', '*.js', '*.ts', '*.tsx', '*.java', '*.php', '*.rb']
        files = []

        for pattern in patterns:
            files.extend(self.source_path.rglob(pattern))

        # Filter out minified, generated, and test files
        filtered = []
        for f in files:
            path_str = str(f).lower()
            if '.min.' not in path_str and 'node_modules' not in path_str and 'generated' not in path_str:
                filtered.append(f)

        return filtered

    def _scan_file(self, file_path: Path) -> List[SecurityViolation]:
        """Scan single file for vulnerabilities"""
        violations = []

        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                lines = f.readlines()
        except Exception as e:
            print(f"⚠️  Failed to read {file_path}: {e}", file=sys.stderr)
            return []

        # Check each vulnerability pattern
        for vuln_type, vuln_config in self.patterns.items():
            for pattern_str in vuln_config['patterns']:
                pattern = re.compile(pattern_str, re.IGNORECASE)

                for line_num, line in enumerate(lines, 1):
                    match = pattern.search(line)
                    if match:
                        # Filter false positives
                        if self._is_false_positive(line, vuln_type):
                            continue

                        violation = SecurityViolation(
                            severity=vuln_config['severity'],
                            category=vuln_config['category'],
                            cwe=vuln_config['cwe'],
                            file=str(file_path),
                            line=line_num,
                            code_snippet=line.strip()[:100],
                            description=vuln_config['description'],
                            remediation=vuln_config['remediation']
                        )
                        violations.append(violation)

        return violations

    def _is_false_positive(self, line: str, vuln_type: str) -> bool:
        """Check if match is likely a false positive"""
        line_lower = line.lower()

        # Check for comments
        if line.strip().startswith(('//','#', '/*', '*')):
            return True

        # Specific false positive checks
        if vuln_type == 'hardcoded_secrets':
            # Ignore if it's a variable name, not a value
            if '= password' in line_lower or '= secret' in line_lower:
                return True
            # Ignore placeholder values
            if any(placeholder in line_lower for placeholder in ['your_', 'example', 'test', 'dummy', '***']):
                return True

        if vuln_type == 'insecure_random':
            # Allow in test files
            if 'test' in line_lower:
                return True

        return False

    def generate_json_report(self, output_file: str):
        """Generate JSON security report"""
        report = self.scan()

        # Convert to dict
        report_dict = {
            'summary': {
                'total_files_scanned': report.total_files_scanned,
                'total_violations': len(report.violations),
                'critical': report.violations_critical,
                'high': report.violations_high,
                'medium': report.violations_medium,
                'low': report.violations_low
            },
            'violations': [asdict(v) for v in report.violations]
        }

        # Write JSON
        with open(output_file, 'w') as f:
            json.dump(report_dict, f, indent=2)

        print(f"✅ Security report generated: {output_file}")
        print(f"   Files scanned: {report.total_files_scanned}")
        print(f"   CRITICAL: {report.violations_critical}")
        print(f"   HIGH: {report.violations_high}")
        print(f"   MEDIUM: {report.violations_medium}")
        print(f"   LOW: {report.violations_low}")

        return report

    def print_violations(self):
        """Print security violations to console"""
        report = self.scan()

        print(f"\n🔒 Security Scan Report")
        print(f"   Files scanned: {report.total_files_scanned}")
        print(f"   Total violations: {len(report.violations)}")

        if len(report.violations) == 0:
            print("\n✅ No security violations found!")
            return

        # Group by severity
        by_severity = {
            'CRITICAL': [v for v in report.violations if v.severity == 'CRITICAL'],
            'HIGH': [v for v in report.violations if v.severity == 'HIGH'],
            'MEDIUM': [v for v in report.violations if v.severity == 'MEDIUM'],
            'LOW': [v for v in report.violations if v.severity == 'LOW']
        }

        for severity, violations in by_severity.items():
            if not violations:
                continue

            print(f"\n{'🔴' if severity == 'CRITICAL' else '🟠' if severity == 'HIGH' else '🟡' if severity == 'MEDIUM' else '🔵'} {severity} ({len(violations)}):\n")

            for v in violations[:5]:  # Show top 5 per severity
                print(f"   {v.category} ({v.cwe})")
                print(f"   {v.file}:{v.line}")
                print(f"   {v.description}")
                print(f"   Code: {v.code_snippet}")
                print(f"   Fix: {v.remediation}\n")

            if len(violations) > 5:
                print(f"   ... and {len(violations) - 5} more {severity} violations\n")


def main():
    parser = argparse.ArgumentParser(
        description='Scan codebase for security vulnerabilities',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Scan with console output
  python security_scan.py src/

  # Generate JSON report
  python security_scan.py src/ --output security-report.json

  # Scan specific directory
  python security_scan.py src/Application/ --output app-security.json
        """
    )
    parser.add_argument('source_path', help='Source code directory path')
    parser.add_argument('--output', help='Output JSON file')

    args = parser.parse_args()

    try:
        scanner = SecurityScanner(args.source_path)

        if args.output:
            report = scanner.generate_json_report(args.output)
            # Exit with error code if critical/high violations found
            if report.violations_critical > 0 or report.violations_high > 0:
                return 1
        else:
            scanner.print_violations()

        return 0
    except Exception as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        return 1


if __name__ == '__main__':
    sys.exit(main())
