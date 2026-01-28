#!/usr/bin/env python3
"""
Code Duplication Detector

Finds duplicate code blocks across codebase (minimum 6 lines).
Calculates duplication percentage and provides refactoring suggestions.

Uses simple token-based matching for language-agnostic detection.
"""

import json
import sys
import hashlib
from pathlib import Path
from typing import Dict, List, Set, Tuple
from dataclasses import dataclass, asdict
from collections import defaultdict
import argparse


@dataclass
class DuplicateBlock:
    """Represents a duplicate code block"""
    file1: str
    line1_start: int
    line1_end: int
    file2: str
    line2_start: int
    line2_end: int
    lines: int
    fragment: str


@dataclass
class DuplicationReport:
    """Complete duplication analysis report"""
    total_lines: int
    duplicate_lines: int
    duplication_percentage: float
    duplicate_blocks: List[DuplicateBlock]
    files_with_duplicates: int


class DuplicationDetector:
    """Detect code duplication in codebase"""

    def __init__(self, source_path: str, min_lines: int = 6, file_patterns: List[str] = None):
        self.source_path = Path(source_path)
        self.min_lines = min_lines
        self.file_patterns = file_patterns or ['*.py', '*.cs', '*.js', '*.ts', '*.tsx', '*.java']

    def analyze(self) -> DuplicationReport:
        """Run duplication analysis"""
        # Get all source files
        files = self._get_source_files()

        # Extract code blocks from all files
        file_blocks = {}
        total_lines = 0

        for file_path in files:
            blocks = self._extract_blocks(file_path)
            if blocks:
                file_blocks[file_path] = blocks
                total_lines += sum(len(block[1]) for block in blocks)

        # Find duplicates
        duplicates = self._find_duplicates(file_blocks)

        # Calculate statistics
        duplicate_lines = sum(d.lines * 2 for d in duplicates)  # Count both occurrences
        duplication_percentage = (duplicate_lines / total_lines * 100) if total_lines > 0 else 0.0

        files_with_dups = len(set(d.file1 for d in duplicates) | set(d.file2 for d in duplicates))

        return DuplicationReport(
            total_lines=total_lines,
            duplicate_lines=duplicate_lines,
            duplication_percentage=round(duplication_percentage, 2),
            duplicate_blocks=duplicates,
            files_with_duplicates=files_with_dups
        )

    def _get_source_files(self) -> List[Path]:
        """Get all source files matching patterns"""
        files = []
        for pattern in self.file_patterns:
            files.extend(self.source_path.rglob(pattern))

        # Filter out test files and generated files
        filtered = []
        for f in files:
            path_str = str(f).lower()
            if 'test' not in path_str and 'generated' not in path_str and '.min.' not in path_str:
                filtered.append(f)

        return filtered

    def _extract_blocks(self, file_path: Path) -> List[Tuple[int, List[str], str]]:
        """Extract code blocks from file (line_start, lines, hash)"""
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                lines = f.readlines()
        except Exception as e:
            print(f"⚠️  Failed to read {file_path}: {e}", file=sys.stderr)
            return []

        blocks = []

        # Create sliding window of min_lines
        for i in range(len(lines) - self.min_lines + 1):
            block_lines = lines[i:i + self.min_lines]

            # Normalize lines (remove whitespace, comments for better matching)
            normalized = [self._normalize_line(line) for line in block_lines]

            # Filter out blocks that are mostly empty/comments
            non_empty = [line for line in normalized if line]
            if len(non_empty) < self.min_lines // 2:
                continue

            # Create hash of normalized block
            block_hash = hashlib.md5(''.join(normalized).encode()).hexdigest()

            blocks.append((i + 1, block_lines, block_hash))

        return blocks

    def _normalize_line(self, line: str) -> str:
        """Normalize line for comparison (remove whitespace, comments)"""
        # Remove leading/trailing whitespace
        line = line.strip()

        # Skip comments
        if line.startswith('//') or line.startswith('#') or line.startswith('/*'):
            return ''

        # Remove all whitespace for comparison
        line = ''.join(line.split())

        return line

    def _find_duplicates(self, file_blocks: Dict[Path, List[Tuple]]) -> List[DuplicateBlock]:
        """Find duplicate blocks across files"""
        duplicates = []
        seen_pairs = set()

        # Group blocks by hash
        hash_to_blocks = defaultdict(list)
        for file_path, blocks in file_blocks.items():
            for line_start, block_lines, block_hash in blocks:
                hash_to_blocks[block_hash].append((file_path, line_start, block_lines))

        # Find duplicates (same hash in different locations)
        for block_hash, occurrences in hash_to_blocks.items():
            if len(occurrences) < 2:
                continue

            # Compare all pairs of occurrences
            for i in range(len(occurrences)):
                for j in range(i + 1, len(occurrences)):
                    file1, line1, lines1 = occurrences[i]
                    file2, line2, lines2 = occurrences[j]

                    # Create unique pair key
                    pair_key = tuple(sorted([
                        (str(file1), line1),
                        (str(file2), line2)
                    ]))

                    if pair_key in seen_pairs:
                        continue

                    seen_pairs.add(pair_key)

                    # Create duplicate block
                    duplicate = DuplicateBlock(
                        file1=str(file1),
                        line1_start=line1,
                        line1_end=line1 + len(lines1) - 1,
                        file2=str(file2),
                        line2_start=line2,
                        line2_end=line2 + len(lines2) - 1,
                        lines=len(lines1),
                        fragment=''.join(lines1[:3]).strip()[:100] + '...'  # First 3 lines preview
                    )
                    duplicates.append(duplicate)

        # Sort by number of lines (largest first)
        duplicates.sort(key=lambda d: d.lines, reverse=True)

        return duplicates

    def generate_json_report(self, output_file: str):
        """Generate JSON duplication report"""
        report = self.analyze()

        # Convert to dict
        report_dict = {
            'summary': {
                'total_lines': report.total_lines,
                'duplicate_lines': report.duplicate_lines,
                'duplication_percentage': report.duplication_percentage,
                'duplicate_blocks': len(report.duplicate_blocks),
                'files_with_duplicates': report.files_with_duplicates
            },
            'duplicates': [asdict(d) for d in report.duplicate_blocks]
        }

        # Write JSON
        with open(output_file, 'w') as f:
            json.dump(report_dict, f, indent=2)

        print(f"✅ Duplication report generated: {output_file}")
        print(f"   Total lines analyzed: {report.total_lines}")
        print(f"   Duplicate lines: {report.duplicate_lines}")
        print(f"   Duplication percentage: {report.duplication_percentage}%")
        print(f"   Duplicate blocks: {len(report.duplicate_blocks)}")

        return report

    def print_duplicates(self):
        """Print duplication results to console"""
        report = self.analyze()

        print(f"\n📊 Duplication Analysis")
        print(f"   Total lines: {report.total_lines}")
        print(f"   Duplicate lines: {report.duplicate_lines}")
        print(f"   Duplication: {report.duplication_percentage}%")

        if report.duplication_percentage > 5:
            print("\n❌ Duplication exceeds 5% threshold!")
        else:
            print("\n✅ Duplication within acceptable limits (<5%)")

        if report.duplicate_blocks:
            print(f"\n🔍 Found {len(report.duplicate_blocks)} duplicate blocks:\n")

            for i, dup in enumerate(report.duplicate_blocks[:10], 1):  # Show top 10
                print(f"{i}. {dup.lines} lines duplicated:")
                print(f"   {dup.file1}:{dup.line1_start}-{dup.line1_end}")
                print(f"   {dup.file2}:{dup.line2_start}-{dup.line2_end}")
                print(f"   Preview: {dup.fragment}\n")

            if len(report.duplicate_blocks) > 10:
                print(f"   ... and {len(report.duplicate_blocks) - 10} more")


def main():
    parser = argparse.ArgumentParser(
        description='Detect code duplication in codebase',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Detect duplicates with default settings (minimum 6 lines)
  python detect_duplicates.py src/

  # Detect larger duplicates only
  python detect_duplicates.py src/ --min-lines 10

  # Generate JSON report
  python detect_duplicates.py src/ --output duplication-report.json

  # Analyze specific file types
  python detect_duplicates.py src/ --patterns "*.py" "*.js"
        """
    )
    parser.add_argument('source_path', help='Source code directory path')
    parser.add_argument('--min-lines', type=int, default=6,
                        help='Minimum lines for duplicate block (default: 6)')
    parser.add_argument('--patterns', nargs='+',
                        help='File patterns to analyze (e.g., "*.py" "*.cs")')
    parser.add_argument('--output', help='Output JSON file')

    args = parser.parse_args()

    try:
        detector = DuplicationDetector(
            args.source_path,
            args.min_lines,
            args.patterns
        )

        if args.output:
            detector.generate_json_report(args.output)
        else:
            detector.print_duplicates()

        return 0
    except Exception as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        return 1


if __name__ == '__main__':
    sys.exit(main())
