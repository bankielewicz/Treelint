#!/usr/bin/env python3
"""
Migration Accuracy Measurement Script

Compares AI-migrated story against ground truth to calculate accuracy metrics.

Usage:
    python measure_accuracy.py <ground-truth.yaml> <ai-migrated.md>

Example:
    python measure_accuracy.py \
      expected/test-story-1-ground-truth.yaml \
      results/test-story-1-ai-output.md

Output:
    Accuracy report with component-level and overall metrics
"""

import re
import sys
import yaml
from pathlib import Path
from typing import Dict, List, Any, Tuple


def extract_tech_spec_from_story(story_file: Path) -> Dict[str, Any]:
    """Extract technical_specification YAML from story .md file."""
    content = story_file.read_text(encoding='utf-8')

    # Find YAML code block in Technical Specification section
    match = re.search(
        r"## Technical Specification\s+```yaml\s+(.*?)\s+```",
        content,
        re.DOTALL
    )

    if not match:
        raise ValueError(f"No YAML tech spec found in {story_file}")

    yaml_content = match.group(1)

    try:
        spec = yaml.safe_load(yaml_content)
        return spec.get("technical_specification", spec)
    except yaml.YAMLError as e:
        raise ValueError(f"Invalid YAML in {story_file}: {e}")


def extract_tech_spec_from_yaml(yaml_file: Path) -> Dict[str, Any]:
    """Extract technical_specification from standalone YAML file."""
    content = yaml_file.read_text(encoding='utf-8')

    try:
        spec = yaml.safe_load(content)
        return spec.get("technical_specification", spec)
    except yaml.YAMLError as e:
        raise ValueError(f"Invalid YAML in {yaml_file}: {e}")


def calculate_component_accuracy(ground_truth: Dict, ai_output: Dict) -> Dict[str, float]:
    """
    Calculate accuracy metrics by comparing AI output to ground truth.

    Metrics:
    1. Component Detection Rate: % of ground truth components found by AI
    2. Type Classification Accuracy: % of detected components with correct type
    3. Name Extraction Accuracy: % of detected components with correct name
    4. Requirement Extraction Rate: % of ground truth requirements extracted
    5. Test Requirement Quality: % of test requirements that are specific (not generic)

    Returns:
        Dictionary with metric names and accuracy percentages (0-1)
    """
    gt_components = ground_truth.get("components", [])
    ai_components = ai_output.get("components", [])

    if not gt_components:
        return {"error": "Ground truth has no components"}

    metrics = {
        "component_detection_rate": 0.0,
        "type_classification_accuracy": 0.0,
        "name_extraction_accuracy": 0.0,
        "requirement_extraction_rate": 0.0,
        "test_requirement_quality": 0.0
    }

    # Metric 1: Component Detection Rate
    # How many ground truth components did AI find?
    metrics["component_detection_rate"] = min(len(ai_components) / len(gt_components), 1.0)

    # Metrics 2-5: Compare matched components
    types_correct = 0
    names_correct = 0
    total_gt_reqs = 0
    total_ai_reqs = 0
    specific_test_reqs = 0
    total_test_reqs = 0

    # Build component map by name for matching
    ai_by_name = {comp.get("name"): comp for comp in ai_components}

    for gt_comp in gt_components:
        gt_name = gt_comp.get("name")
        gt_type = gt_comp.get("type")

        # Find matching AI component by name
        ai_comp = ai_by_name.get(gt_name)

        if ai_comp:
            # Metric 2: Type correct?
            if ai_comp.get("type") == gt_type:
                types_correct += 1

            # Metric 3: Name correct (if found, name matches)
            names_correct += 1

            # Metric 4: Requirements extracted?
            gt_reqs = gt_comp.get("requirements", [])
            ai_reqs = ai_comp.get("requirements", [])

            total_gt_reqs += len(gt_reqs)
            total_ai_reqs += len(ai_reqs)

            # Metric 5: Test requirement quality (specific vs generic)
            for ai_req in ai_reqs:
                test_req = ai_req.get("test_requirement", "")
                total_test_reqs += 1

                # Check if specific (not generic like "Test: Verify it works")
                if _is_specific_test_requirement(test_req):
                    specific_test_reqs += 1

    # Calculate percentages
    if len(gt_components) > 0:
        metrics["type_classification_accuracy"] = types_correct / len(gt_components)
        metrics["name_extraction_accuracy"] = names_correct / len(gt_components)

    if total_gt_reqs > 0:
        metrics["requirement_extraction_rate"] = min(total_ai_reqs / total_gt_reqs, 1.0)

    if total_test_reqs > 0:
        metrics["test_requirement_quality"] = specific_test_reqs / total_test_reqs

    return metrics


def _is_specific_test_requirement(test_req: str) -> bool:
    """
    Determine if test requirement is specific (not generic).

    Specific test requirements:
    - Contain numbers: "polls at 30s intervals"
    - Contain specific actions: "throws ArgumentException"
    - Contain specific conditions: "when CancellationToken signals"

    Generic test requirements:
    - "Verify it works"
    - "Test functionality"
    - "Check behavior"
    """
    if not test_req or len(test_req) < 20:
        return False  # Too short, probably generic

    # Check for specificity indicators
    has_numbers = bool(re.search(r'\d+', test_req))
    has_specific_action = any(word in test_req.lower() for word in
                              ['throws', 'returns', 'creates', 'updates', 'deletes',
                               'loads', 'validates', 'signals', 'measures'])
    has_specific_condition = any(word in test_req.lower() for word in
                                 ['when', 'until', 'within', 'after', 'before',
                                  'if', 'unless', 'given'])

    # Generic indicators
    has_generic_words = any(word in test_req.lower() for word in
                           ['verify', 'check', 'test', 'ensure', 'confirm'])

    # Specific if has numbers OR (specific action AND condition) AND not purely generic
    is_specific = (has_numbers or (has_specific_action and has_specific_condition)) and \
                  len(test_req) > 30

    return is_specific


def calculate_overall_accuracy(metrics: Dict[str, float]) -> float:
    """Calculate overall accuracy as average of all metrics."""
    valid_metrics = [v for v in metrics.values() if isinstance(v, float)]
    return sum(valid_metrics) / len(valid_metrics) if valid_metrics else 0.0


def generate_report(ground_truth_file: str, ai_output_file: str,
                   metrics: Dict[str, float], overall: float) -> str:
    """Generate human-readable accuracy report."""
    report = []

    report.append("=" * 80)
    report.append("MIGRATION ACCURACY REPORT")
    report.append("=" * 80)
    report.append(f"\nGround Truth: {ground_truth_file}")
    report.append(f"AI Output:    {ai_output_file}")
    report.append(f"\nOverall Accuracy: {overall * 100:.1f}%")

    if overall >= 0.95:
        status = "✅ EXCELLENT (≥95%)"
    elif overall >= 0.90:
        status = "✅ GOOD (90-95%)"
    elif overall >= 0.80:
        status = "⚠️ ACCEPTABLE (80-90%)"
    else:
        status = "❌ POOR (<80%)"

    report.append(f"Status: {status}")

    report.append("\n" + "-" * 80)
    report.append("DETAILED METRICS:")
    report.append("-" * 80)

    for metric_name, value in metrics.items():
        if isinstance(value, float):
            report.append(f"  {metric_name:.<50} {value * 100:>5.1f}%")

    report.append("=" * 80)

    return "\n".join(report)


def main():
    """CLI entry point."""
    if len(sys.argv) < 3:
        print(__doc__)
        sys.exit(2)

    ground_truth_file = Path(sys.argv[1])
    ai_output_file = Path(sys.argv[2])

    # Validate files exist
    if not ground_truth_file.exists():
        print(f"❌ Ground truth file not found: {ground_truth_file}")
        sys.exit(2)

    if not ai_output_file.exists():
        print(f"❌ AI output file not found: {ai_output_file}")
        sys.exit(2)

    try:
        # Extract tech specs
        if ground_truth_file.suffix == '.yaml':
            gt_spec = extract_tech_spec_from_yaml(ground_truth_file)
        else:
            gt_spec = extract_tech_spec_from_story(ground_truth_file)

        ai_spec = extract_tech_spec_from_story(ai_output_file)

        # Calculate accuracy
        metrics = calculate_component_accuracy(gt_spec, ai_spec)
        overall = calculate_overall_accuracy(metrics)

        # Generate report
        report = generate_report(
            str(ground_truth_file),
            str(ai_output_file),
            metrics,
            overall
        )

        print(report)

        # Exit code based on accuracy
        if overall >= 0.95:
            sys.exit(0)  # Excellent
        elif overall >= 0.90:
            sys.exit(0)  # Good
        else:
            sys.exit(1)  # Below target

    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(2)


if __name__ == "__main__":
    main()
