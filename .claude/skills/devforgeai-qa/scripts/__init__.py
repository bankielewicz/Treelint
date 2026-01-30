"""
DevForgeAI QA Automation Scripts

Python utilities for comprehensive code quality analysis:
- Coverage reporting
- Complexity analysis
- Duplication detection
- Spec compliance validation
- Security scanning
- Test stub generation

These scripts are invoked by the devforgeai-qa skill during deep validation workflows.
"""

__version__ = "1.0.0"
__author__ = "DevForgeAI"

# Script metadata
SCRIPTS = {
    'coverage': {
        'name': 'generate_coverage_report.py',
        'description': 'Parse coverage data and generate HTML reports',
        'supports': ['.NET', 'Python', 'JavaScript']
    },
    'complexity': {
        'name': 'analyze_complexity.py',
        'description': 'Calculate cyclomatic complexity',
        'supports': ['Python', 'C#', 'JavaScript', 'Java']
    },
    'duplication': {
        'name': 'detect_duplicates.py',
        'description': 'Find duplicate code blocks',
        'supports': ['All languages']
    },
    'compliance': {
        'name': 'validate_spec_compliance.py',
        'description': 'Validate against story acceptance criteria',
        'supports': ['Markdown stories + pytest/xUnit/Jest']
    },
    'security': {
        'name': 'security_scan.py',
        'description': 'Scan for security vulnerabilities',
        'supports': ['Python', 'C#', 'JavaScript', 'Java', 'PHP', 'Ruby']
    },
    'test_stubs': {
        'name': 'generate_test_stubs.py',
        'description': 'Auto-generate test templates',
        'supports': ['xUnit', 'pytest', 'Jest']
    }
}