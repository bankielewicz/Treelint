#!/usr/bin/env python3
"""
Validate subagent output against YAML contract.

Usage:
    python validate_contract.py <output_file> <contract_yaml>

Example:
    python validate_contract.py /tmp/subagent-output.txt contracts/requirements-analyst-contract.yaml

Returns:
    Exit code 0: Validation passed
    Exit code 1: Validation failed (violations detected)
"""

import re
import sys
import yaml
from datetime import datetime
from pathlib import Path

def load_contract(contract_path):
    """Load and parse YAML contract."""
    try:
        with open(contract_path, 'r') as f:
            return yaml.safe_load(f)
    except FileNotFoundError:
        print(f"✗ Contract file not found: {contract_path}")
        sys.exit(2)
    except yaml.YAMLError as e:
        print(f"✗ Invalid YAML in contract: {e}")
        sys.exit(2)

def extract_section(text, section_name):
    """Extract content between section header and next header."""
    # Split text by ## headers, find the section we want
    sections = re.split(r'^##\s+', text, flags=re.MULTILINE)

    for section in sections:
        # Check if this section starts with our section name
        if section.strip().startswith(section_name):
            # Extract content after the section name line
            lines = section.split('\n', 1)
            if len(lines) > 1:
                return lines[1].strip()
            else:
                return ""

    return ""

def validate_output(output_text, contract):
    """Validate output against contract specifications."""
    violations = []

    # Validation 1: No file creation
    if contract.get('constraints', {}).get('no_file_creation', {}).get('enabled', False):
        patterns = contract.get('validation', {}).get('check_no_file_paths', {}).get('prohibited_patterns', [])

        for pattern in patterns:
            try:
                if re.search(pattern, output_text, re.IGNORECASE):
                    violations.append({
                        "type": "FILE_CREATION",
                        "pattern": pattern,
                        "severity": "CRITICAL",
                        "constraint": "no_file_creation"
                    })
            except re.error as e:
                print(f"Warning: Invalid regex pattern '{pattern}': {e}")
                continue

    # Validation 2: Required sections
    if contract.get('validation', {}).get('check_sections_present', {}).get('enabled', False):
        required = contract['validation']['check_sections_present'].get('required_sections', [])

        for section in required:
            if f"## {section}" not in output_text:
                violations.append({
                    "type": "MISSING_SECTION",
                    "section": section,
                    "severity": "HIGH",
                    "constraint": "check_sections_present"
                })

    # Validation 3: AC format (if applicable)
    if contract.get('validation', {}).get('check_ac_format', {}).get('enabled', False):
        config = contract['validation']['check_ac_format']
        min_count = config.get('min_count', 3)
        keywords = config.get('required_keywords', [])

        ac_section = extract_section(output_text, "Acceptance Criteria")
        ac_count = ac_section.count("### AC") if ac_section else 0

        if ac_count < min_count:
            violations.append({
                "type": "INSUFFICIENT_AC",
                "actual": ac_count,
                "required": min_count,
                "severity": "HIGH",
                "constraint": "check_ac_format"
            })

        for keyword in keywords:
            if keyword not in ac_section:
                violations.append({
                    "type": "MISSING_AC_KEYWORD",
                    "keyword": keyword,
                    "severity": "MEDIUM",
                    "constraint": "check_ac_format"
                })

    # Validation 4: NFR measurability (if applicable)
    if contract.get('validation', {}).get('check_nfr_measurability', {}).get('enabled', False):
        config = contract['validation']['check_nfr_measurability']
        prohibited = config.get('prohibited_vague_terms', [])
        nfr_section = extract_section(output_text, "Non-Functional Requirements")

        for term in prohibited:
            # Use word boundaries to avoid false positives
            if re.search(rf'\b{term}\b', nfr_section, re.IGNORECASE):
                violations.append({
                    "type": "VAGUE_NFR",
                    "term": term,
                    "severity": "MEDIUM",
                    "constraint": "check_nfr_measurability"
                })

    # Validation 5: Size limit
    max_length = contract.get('constraints', {}).get('max_output_length', {}).get('value', 50000)
    actual_length = len(output_text)

    if actual_length > max_length:
        violations.append({
            "type": "SIZE_EXCEEDED",
            "actual": actual_length,
            "max": max_length,
            "severity": "MEDIUM",
            "constraint": "max_output_length"
        })

    return violations

def format_violations(violations):
    """Format violations for display."""
    output = []
    for v in violations:
        detail = v.get('pattern') or v.get('section') or v.get('term') or v.get('keyword') or 'N/A'
        if v['type'] == 'SIZE_EXCEEDED':
            detail = f"{v.get('actual', 0)}/{v.get('max', 0)} chars"
        elif v['type'] == 'INSUFFICIENT_AC':
            detail = f"{v.get('actual', 0)}/{v.get('required', 3)} criteria"

        output.append(f"  [{v['severity']}] {v['type']}: {detail}")
    return "\n".join(output)

def apply_error_handling(violations, contract, subagent_output, retry_count=0):
    """Apply contract error handling rules."""
    if not violations:
        return {"action": "continue", "output": subagent_output}

    # Get highest severity violation
    critical = [v for v in violations if v['severity'] == "CRITICAL"]
    high = [v for v in violations if v['severity'] == "HIGH"]

    if critical:
        primary_violation = critical[0]
    elif high:
        primary_violation = high[0]
    else:
        # Medium violations - log and continue
        return {"action": "log_and_continue", "output": subagent_output}

    # Get error handling config for this violation type
    error_type = primary_violation['type'].lower()
    error_config = contract.get('error_handling', {}).get(f'on_{error_type}', {})

    action = error_config.get('action', 're_invoke')
    max_retries = error_config.get('max_retries', 1)

    if retry_count >= max_retries:
        # Retries exhausted - apply fallback
        fallback = error_config.get('fallback', 'HALT')
        return {"action": fallback, "output": subagent_output, "violations": violations}

    # Return recovery action
    return {
        "action": action,
        "retry_count": retry_count + 1,
        "max_retries": max_retries,
        "violations": violations
    }

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python validate_contract.py <output_file> <contract_yaml>")
        print("\nExample:")
        print("  python validate_contract.py /tmp/output.txt contracts/requirements-analyst-contract.yaml")
        sys.exit(1)

    output_file = sys.argv[1]
    contract_file = sys.argv[2]

    # Check files exist
    if not Path(output_file).exists():
        print(f"✗ Output file not found: {output_file}")
        sys.exit(2)

    if not Path(contract_file).exists():
        print(f"✗ Contract file not found: {contract_file}")
        sys.exit(2)

    # Load files
    try:
        with open(output_file, 'r') as f:
            output_text = f.read()
    except Exception as e:
        print(f"✗ Error reading output file: {e}")
        sys.exit(2)

    contract = load_contract(contract_file)

    # Validate
    violations = validate_output(output_text, contract)

    # Display results
    print(f"\nContract Validation: {contract['skill']} <-> {contract['subagent']}")
    print(f"Contract Version: {contract.get('contract_version', 'N/A')}")
    print(f"Phase: {contract.get('phase', 'N/A')}")
    print(f"Output Size: {len(output_text)} characters\n")

    if violations:
        # Categorize by severity
        critical = [v for v in violations if v['severity'] == "CRITICAL"]
        high = [v for v in violations if v['severity'] == "HIGH"]
        medium = [v for v in violations if v['severity'] == "MEDIUM"]

        print(f"✗ Validation FAILED ({len(violations)} violations)\n")

        if critical:
            print(f"CRITICAL ({len(critical)}):")
            print(format_violations(critical))
            print()

        if high:
            print(f"HIGH ({len(high)}):")
            print(format_violations(high))
            print()

        if medium:
            print(f"MEDIUM ({len(medium)}):")
            print(format_violations(medium))
            print()

        sys.exit(1)
    else:
        print("✓ Validation PASSED")
        print(f"  Output compliant with {contract['skill']} <-> {contract['subagent']} contract")
        print(f"  All constraints satisfied ✅")
        sys.exit(0)
