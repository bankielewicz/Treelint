#!/usr/bin/env python3
"""
STORY-198: Command Pattern Analysis Tool

Analyzes hook-unknown-commands.log to recommend safe patterns for auto-approval.
Used for data-driven hook optimization on a monthly basis.

Usage:
    python3 devforgeai/scripts/analyze_hook_patterns.py
    python3 devforgeai/scripts/analyze_hook_patterns.py --json
    python3 devforgeai/scripts/analyze_hook_patterns.py --log-file path/to/log

Requirements:
    - Python 3 standard library only (no pip install)
    - Uses: re, collections, pathlib, argparse
"""

import re
import argparse
from pathlib import Path
from collections import Counter
from typing import List, Tuple


# Constants required by tests
DANGEROUS_PREFIXES: set = {
    "rm",
    "sudo",
    "curl",
    "wget",
    "dd",
    "chmod",
    "chown",
    "mkfs",
    "fdisk",
    "mount",
    "umount",
    "kill",
    "killall",
    "shutdown",
    "reboot",
    "ssh",
    "scp",
    "rsync",
}

SAFE_PREFIXES: set = {
    "cd",
    "git",
    "python",
    "python3",
    "devforgeai",
    "devforgeai-validate",
    "which",
    "stat",
    "file",
    "basename",
    "ls",
    "cat",
    "grep",
    "find",
    "echo",
    "pwd",
    "head",
    "tail",
    "wc",
    "sort",
    "uniq",
    "diff",
    "mkdir",
    "touch",
    "cp",
    "mv",
    "npm",
    "npx",
    "node",
    "pip",
    "pip3",
    "pytest",
    "dotnet",
    "go",
    "cargo",
    "make",
}

# Regex pattern for UNKNOWN COMMAND entries
# Use [ \t]* (space/tab only, not \s which includes newline) + [^\n]+ for same-line matching
UNKNOWN_COMMAND_PATTERN = re.compile(
    r"UNKNOWN COMMAND REQUIRING APPROVAL:[ \t]*([^\n]+)"
)


def parse_log_entries(log_content: str) -> List[str]:
    """
    Parse hook log for UNKNOWN COMMAND REQUIRING APPROVAL entries.

    Args:
        log_content: Raw log file content

    Returns:
        List of command strings extracted from log entries

    Handles:
        - Empty log files
        - Malformed entries (empty command after marker)
        - Mixed content with non-UNKNOWN COMMAND lines
    """
    if not log_content:
        return []

    entries = []
    for match in UNKNOWN_COMMAND_PATTERN.finditer(log_content):
        command = match.group(1).strip()
        if command:  # Skip empty commands
            entries.append(command)

    return entries


def extract_prefix(command: str) -> str:
    """
    Extract first 2 words from command as pattern prefix.

    Args:
        command: Full command string

    Returns:
        First 2 words joined by space, or single word if only one present

    Examples:
        'cd /tmp && python' -> 'cd /tmp'
        'git status' -> 'git status'
        'ls' -> 'ls'
    """
    # Normalize whitespace: split on any whitespace, filter empty
    words = command.split()
    if not words:
        return ""

    # Return first 2 words (or just first if only one)
    return " ".join(words[:2])


def is_safe_prefix(prefix: str) -> bool:
    """
    Check if prefix is safe (not in dangerous list).

    Safety logic:
        - If first word is in DANGEROUS_PREFIXES -> False
        - Otherwise -> True (whitelist not required for True)

    Args:
        prefix: Command prefix (first 2 words)

    Returns:
        True if safe, False if dangerous
    """
    if not prefix:
        return False

    # Extract first word for danger check
    first_word = prefix.split()[0].lower()

    # Check if first word is dangerous
    if first_word in DANGEROUS_PREFIXES:
        return False

    return True


def analyze_frequencies(prefixes: List[str]) -> List[Tuple[str, int, float]]:
    """
    Analyze prefix frequencies and return top 20.

    Args:
        prefixes: List of safe command prefixes

    Returns:
        List of (prefix, count, percentage) tuples, sorted by count descending
        Limited to top 20 entries
    """
    if not prefixes:
        return []

    total = len(prefixes)
    counter = Counter(prefixes)

    # Get top 20 sorted by count descending
    top_20 = counter.most_common(20)

    # Add percentage to each entry
    result = []
    for prefix, count in top_20:
        percentage = (count / total) * 100
        result.append((prefix, count, percentage))

    return result


def calculate_impact(top_20: List[Tuple[str, int, float]], total: int) -> float:
    """
    Calculate total impact percentage of adding top 20 patterns.

    Impact = (sum of top 20 counts / total commands) * 100

    Args:
        top_20: List of (prefix, count, percentage) tuples
        total: Total number of commands analyzed

    Returns:
        Impact percentage (0-100)
    """
    if total == 0:
        return 0.0

    top_20_sum = sum(count for _, count, _ in top_20)
    return (top_20_sum / total) * 100


def analyze_log_file(log_path: Path) -> dict:
    """
    Complete analysis workflow for a log file.

    Args:
        log_path: Path to hook-unknown-commands.log

    Returns:
        Dictionary with analysis results
    """
    # Read log file
    if not log_path.exists():
        return {
            "error": f"Log file not found: {log_path}",
            "entries": [],
            "top_20": [],
            "impact": 0.0,
            "total": 0,
        }

    log_content = log_path.read_text(encoding="utf-8")

    # Step 1: Parse log entries
    entries = parse_log_entries(log_content)

    # Step 2: Extract prefixes
    prefixes = [extract_prefix(e) for e in entries]

    # Step 3: Filter safe prefixes
    safe_prefixes = [p for p in prefixes if is_safe_prefix(p)]

    # Step 4: Analyze frequencies
    top_20 = analyze_frequencies(safe_prefixes)

    # Step 5: Calculate impact
    total_entries = len(entries)
    impact = calculate_impact(top_20, total_entries)

    return {
        "error": None,
        "entries": entries,
        "top_20": top_20,
        "impact": impact,
        "total": total_entries,
        "safe_count": len(safe_prefixes),
        "filtered_count": len(entries) - len(safe_prefixes),
    }


def format_report(results: dict) -> str:
    """Format analysis results as human-readable report."""
    if results.get("error"):
        return f"Error: {results['error']}"

    lines = [
        "=" * 60,
        "Command Pattern Analysis Report",
        "=" * 60,
        "",
        f"Total unknown commands: {results['total']}",
        f"Safe prefixes (candidates): {results['safe_count']}",
        f"Filtered (dangerous): {results['filtered_count']}",
        "",
        "Top 20 Safe Patterns to Add:",
        "-" * 40,
    ]

    for i, (prefix, count, pct) in enumerate(results["top_20"], 1):
        lines.append(f"  {i:2d}. {prefix:<30} {count:5d} ({pct:5.1f}%)")

    lines.extend([
        "",
        "-" * 40,
        f"Total Impact: {results['impact']:.1f}%",
        f"  (Adding these patterns would auto-approve {results['impact']:.1f}% of unknown commands)",
        "",
        "=" * 60,
    ])

    return "\n".join(lines)


def format_json(results: dict) -> str:
    """Format analysis results as JSON."""
    import json

    # Handle error case
    if results.get("error"):
        return json.dumps({"error": results["error"]}, indent=2)

    output = {
        "total_commands": results["total"],
        "safe_candidates": results["safe_count"],
        "filtered_dangerous": results["filtered_count"],
        "impact_percentage": round(results["impact"], 2),
        "top_20_patterns": [
            {"prefix": p, "count": c, "percentage": round(pct, 2)}
            for p, c, pct in results["top_20"]
        ],
    }
    return json.dumps(output, indent=2)


def main():
    """CLI entry point."""
    parser = argparse.ArgumentParser(
        description="Analyze hook logs and recommend safe patterns to add",
        epilog="Example: python3 analyze_hook_patterns.py --log-file hooks.log",
    )
    parser.add_argument(
        "--log-file",
        type=Path,
        default=Path("hook-unknown-commands.log"),
        help="Path to hook unknown commands log (default: hook-unknown-commands.log)",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Output results as JSON",
    )

    args = parser.parse_args()

    # Run analysis
    results = analyze_log_file(args.log_file)

    # Output results
    if args.json:
        print(format_json(results))
    else:
        print(format_report(results))


if __name__ == "__main__":
    main()
