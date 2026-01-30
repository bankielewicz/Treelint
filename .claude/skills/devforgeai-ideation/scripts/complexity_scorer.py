#!/usr/bin/env python3
"""
Complexity Scorer - DevForgeAI Ideation Skill

Automated complexity calculation from user responses to determine
appropriate architecture tier (Simple → Enterprise).

Usage:
    python complexity_scorer.py --answers answers.json
    python complexity_scorer.py --interactive

Output:
    - complexity_score: int (0-60)
    - recommended_tier: str ("Tier 1: Simple Application")
    - rationale: str (explanation of key drivers)
    - detailed_scoring: dict (breakdown by dimension)
"""

import json
import argparse
import sys
from typing import Dict, List, Tuple, Any


class ComplexityScorer:
    """Calculate complexity score and recommend architecture tier."""

    # Tier thresholds
    TIERS = [
        (0, 15, "Tier 1: Simple Application"),
        (16, 30, "Tier 2: Moderate Application"),
        (31, 45, "Tier 3: Complex Platform"),
        (46, 60, "Tier 4: Enterprise Platform")
    ]

    def __init__(self):
        self.scores = {
            "functional": 0,
            "technical": 0,
            "team_org": 0,
            "non_functional": 0
        }
        self.drivers = []  # Key complexity drivers for rationale

    def score_user_roles(self, count: int) -> int:
        """
        Score based on number of user roles.

        Args:
            count: Number of distinct user roles

        Returns:
            Score (0-10 points)
        """
        if count <= 2:
            score = 2
            level = "Low"
        elif count <= 6:
            score = 5
            level = "Medium"
        else:
            score = 10
            level = "High"

        if score >= 7:
            self.drivers.append(f"{level} role complexity ({count} roles)")

        return score

    def score_entities(self, count: int) -> int:
        """
        Score based on number of core data entities.

        Args:
            count: Number of core entities

        Returns:
            Score (0-10 points)
        """
        if count <= 3:
            score = 2
            level = "Low"
        elif count <= 10:
            score = 5
            level = "Medium"
        elif count <= 20:
            score = 8
            level = "High"
        else:
            score = 10
            level = "Very High"

        if score >= 8:
            self.drivers.append(f"{level} entity count ({count} entities)")

        return score

    def score_integrations(self, count: int, has_complex: bool = False) -> int:
        """
        Score based on number and complexity of integrations.

        Args:
            count: Number of external integrations
            has_complex: Whether any integrations are complex (legacy, SOAP, etc.)

        Returns:
            Score (0-10 points)
        """
        if count == 0:
            return 0
        elif count <= 2:
            score = 3
        elif count <= 5:
            score = 6
        else:
            score = 10

        # Add penalty for complex integrations
        if has_complex:
            score = min(10, score + 3)
            self.drivers.append(f"Complex integrations ({count} integrations)")

        if score >= 8:
            self.drivers.append(f"Many integrations ({count} systems)")

        return score

    def score_workflow_complexity(self, complexity_type: str) -> int:
        """
        Score based on workflow complexity.

        Args:
            complexity_type: "linear", "branching", "state_machine", or "orchestration"

        Returns:
            Score (0-10 points)
        """
        scores = {
            "linear": 2,
            "branching": 5,
            "state_machine": 8,
            "orchestration": 10
        }

        score = scores.get(complexity_type.lower(), 2)

        if score >= 8:
            self.drivers.append(f"Complex workflows ({complexity_type})")

        return score

    def score_data_volume(self, volume_estimate: str) -> int:
        """
        Score based on expected data volume.

        Args:
            volume_estimate: "small" (<10k), "medium" (10k-1M), "large" (1M-100M), "massive" (>100M)

        Returns:
            Score (0-10 points)
        """
        scores = {
            "small": 2,
            "medium": 5,
            "large": 8,
            "massive": 10
        }

        score = scores.get(volume_estimate.lower(), 2)

        if score >= 8:
            self.drivers.append(f"Large data volume ({volume_estimate})")

        return score

    def score_concurrency(self, concurrent_users: int) -> int:
        """
        Score based on concurrent user expectations.

        Args:
            concurrent_users: Expected number of concurrent users

        Returns:
            Score (0-10 points)
        """
        if concurrent_users < 100:
            score = 2
        elif concurrent_users < 1000:
            score = 5
        elif concurrent_users < 10000:
            score = 8
        else:
            score = 10

        if score >= 8:
            self.drivers.append(f"High concurrency ({concurrent_users} concurrent users)")

        return score

    def score_realtime(self, realtime_type: str) -> int:
        """
        Score based on real-time requirements.

        Args:
            realtime_type: "none", "polling", "websockets", or "hard_realtime"

        Returns:
            Score (0-10 points)
        """
        scores = {
            "none": 0,
            "polling": 3,
            "websockets": 7,
            "hard_realtime": 10
        }

        score = scores.get(realtime_type.lower(), 0)

        if score >= 7:
            self.drivers.append(f"Real-time requirements ({realtime_type})")

        return score

    def score_team_size(self, size: int) -> int:
        """
        Score based on team size.

        Args:
            size: Number of developers

        Returns:
            Score (0-5 points)
        """
        if size <= 3:
            return 1
        elif size <= 10:
            return 3
        else:
            return 5

    def score_team_distribution(self, distribution: str) -> int:
        """
        Score based on team distribution.

        Args:
            distribution: "colocated", "remote_same_tz", or "distributed_multi_tz"

        Returns:
            Score (0-5 points)
        """
        scores = {
            "colocated": 1,
            "remote_same_tz": 3,
            "distributed_multi_tz": 5
        }

        score = scores.get(distribution.lower(), 1)

        if score >= 5:
            self.drivers.append("Distributed team (multi-timezone)")

        return score

    def score_performance_requirements(self, requirement: str) -> int:
        """
        Score based on performance requirements.

        Args:
            requirement: "relaxed", "standard", or "high"

        Returns:
            Score (0-5 points)
        """
        scores = {
            "relaxed": 1,
            "standard": 3,
            "high": 5
        }

        score = scores.get(requirement.lower(), 1)

        if score >= 5:
            self.drivers.append("High performance requirements")

        return score

    def score_compliance(self, compliance_level: str) -> int:
        """
        Score based on compliance requirements.

        Args:
            compliance_level: "none", "basic" (GDPR), "industry" (SOC2, PCI), or "strict" (HIPAA, banking)

        Returns:
            Score (0-5 points)
        """
        scores = {
            "none": 0,
            "basic": 2,
            "industry": 4,
            "strict": 5
        }

        score = scores.get(compliance_level.lower(), 0)

        if score >= 4:
            self.drivers.append(f"Compliance requirements ({compliance_level})")

        return score

    def calculate_score(self, answers: Dict[str, Any]) -> Tuple[int, str, str, Dict]:
        """
        Calculate total complexity score from answers.

        Args:
            answers: Dictionary of answers to complexity questions

        Returns:
            Tuple of (total_score, tier, rationale, detailed_scoring)
        """
        # Functional Complexity (0-20 points)
        self.scores["functional"] += self.score_user_roles(answers.get("user_roles", 1))
        self.scores["functional"] += self.score_entities(answers.get("entities", 1))
        self.scores["functional"] += self.score_integrations(
            answers.get("integrations", 0),
            answers.get("has_complex_integrations", False)
        )
        self.scores["functional"] += self.score_workflow_complexity(
            answers.get("workflow_complexity", "linear")
        )

        # Technical Complexity (0-20 points)
        self.scores["technical"] += self.score_data_volume(
            answers.get("data_volume", "small")
        )
        self.scores["technical"] += self.score_concurrency(
            answers.get("concurrent_users", 100)
        )
        self.scores["technical"] += self.score_realtime(
            answers.get("realtime_requirements", "none")
        )

        # Team/Organizational Complexity (0-10 points)
        self.scores["team_org"] += self.score_team_size(answers.get("team_size", 1))
        self.scores["team_org"] += self.score_team_distribution(
            answers.get("team_distribution", "colocated")
        )

        # Non-Functional Complexity (0-10 points)
        self.scores["non_functional"] += self.score_performance_requirements(
            answers.get("performance_requirements", "standard")
        )
        self.scores["non_functional"] += self.score_compliance(
            answers.get("compliance", "none")
        )

        # Calculate total
        total_score = sum(self.scores.values())

        # Determine tier
        tier = self._determine_tier(total_score)

        # Generate rationale
        rationale = self._generate_rationale(total_score, tier)

        return total_score, tier, rationale, self.scores

    def _determine_tier(self, score: int) -> str:
        """Determine architecture tier based on score."""
        for min_score, max_score, tier_name in self.TIERS:
            if min_score <= score <= max_score:
                return tier_name
        return "Tier 4: Enterprise Platform"  # Fallback for scores > 60

    def _generate_rationale(self, score: int, tier: str) -> str:
        """Generate human-readable rationale for the recommendation."""
        rationale = f"Complexity score of {score}/60 suggests {tier}.\n\n"

        if self.drivers:
            rationale += "Key complexity drivers:\n"
            for i, driver in enumerate(self.drivers, 1):
                rationale += f"{i}. {driver}\n"
        else:
            rationale += "Relatively low complexity across all dimensions.\n"

        rationale += "\n" + self._get_tier_description(tier)

        return rationale

    def _get_tier_description(self, tier: str) -> str:
        """Get description of recommended tier."""
        descriptions = {
            "Tier 1: Simple Application": (
                "Recommended: Monolithic architecture with 2-3 layers. "
                "Single database, simple deployment (Vercel, Heroku). "
                "Suitable for solo developer or small team."
            ),
            "Tier 2: Moderate Application": (
                "Recommended: Modular Monolith or simple microservices. "
                "3-4 layers (API, Application, Domain, Infrastructure). "
                "Primary database + caching (Redis). Load-balanced deployment."
            ),
            "Tier 3: Complex Platform": (
                "Recommended: Microservices (5-10 services) or Clean Architecture with DDD. "
                "Polyglot persistence (SQL + NoSQL + caching). "
                "Kubernetes deployment with service mesh."
            ),
            "Tier 4: Enterprise Platform": (
                "Recommended: Distributed microservices + event-driven architecture. "
                "CQRS + Event Sourcing patterns. Polyglot persistence with event store. "
                "Multi-region Kubernetes with auto-scaling."
            )
        }
        return descriptions.get(tier, "")


def load_answers(filepath: str) -> Dict[str, Any]:
    """Load answers from JSON file."""
    try:
        with open(filepath, 'r') as f:
            return json.load(f)
    except FileNotFoundError:
        print(f"Error: File '{filepath}' not found.", file=sys.stderr)
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON in '{filepath}': {e}", file=sys.stderr)
        sys.exit(1)


def interactive_mode() -> Dict[str, Any]:
    """Interactive mode to gather answers from user."""
    print("DevForgeAI Complexity Scorer - Interactive Mode\n")
    print("Answer the following questions to calculate complexity score:\n")

    answers = {}

    # Functional Complexity
    print("=== Functional Complexity ===\n")

    answers["user_roles"] = int(input("Number of user roles (e.g., 1-10): "))
    answers["entities"] = int(input("Number of core data entities (e.g., 1-25): "))
    answers["integrations"] = int(input("Number of external integrations (e.g., 0-15): "))

    if answers["integrations"] > 0:
        complex_integ = input("Any complex integrations (legacy/SOAP)? (y/n): ").lower()
        answers["has_complex_integrations"] = complex_integ == 'y'

    print("\nWorkflow complexity:")
    print("  1. Linear")
    print("  2. Branching")
    print("  3. State machine")
    print("  4. Orchestration")
    workflow_choice = int(input("Select (1-4): "))
    workflow_map = {1: "linear", 2: "branching", 3: "state_machine", 4: "orchestration"}
    answers["workflow_complexity"] = workflow_map.get(workflow_choice, "linear")

    # Technical Complexity
    print("\n=== Technical Complexity ===\n")

    print("Data volume:")
    print("  1. Small (<10k records)")
    print("  2. Medium (10k-1M)")
    print("  3. Large (1M-100M)")
    print("  4. Massive (>100M)")
    volume_choice = int(input("Select (1-4): "))
    volume_map = {1: "small", 2: "medium", 3: "large", 4: "massive"}
    answers["data_volume"] = volume_map.get(volume_choice, "small")

    answers["concurrent_users"] = int(input("Expected concurrent users (e.g., 100-10000): "))

    print("\nReal-time requirements:")
    print("  1. None")
    print("  2. Polling (5-30 sec)")
    print("  3. WebSockets (<1 sec)")
    print("  4. Hard real-time (<100ms)")
    realtime_choice = int(input("Select (1-4): "))
    realtime_map = {1: "none", 2: "polling", 3: "websockets", 4: "hard_realtime"}
    answers["realtime_requirements"] = realtime_map.get(realtime_choice, "none")

    # Team/Organizational Complexity
    print("\n=== Team/Organizational Complexity ===\n")

    answers["team_size"] = int(input("Team size (number of developers): "))

    print("\nTeam distribution:")
    print("  1. Co-located (same office)")
    print("  2. Remote (same timezone)")
    print("  3. Distributed (multi-timezone)")
    dist_choice = int(input("Select (1-3): "))
    dist_map = {1: "colocated", 2: "remote_same_tz", 3: "distributed_multi_tz"}
    answers["team_distribution"] = dist_map.get(dist_choice, "colocated")

    # Non-Functional Complexity
    print("\n=== Non-Functional Complexity ===\n")

    print("Performance requirements:")
    print("  1. Relaxed (<5 sec)")
    print("  2. Standard (<1 sec)")
    print("  3. High (<200ms)")
    perf_choice = int(input("Select (1-3): "))
    perf_map = {1: "relaxed", 2: "standard", 3: "high"}
    answers["performance_requirements"] = perf_map.get(perf_choice, "standard")

    print("\nCompliance requirements:")
    print("  1. None")
    print("  2. Basic (GDPR)")
    print("  3. Industry (SOC2, PCI-DSS)")
    print("  4. Strict (HIPAA, banking)")
    compliance_choice = int(input("Select (1-4): "))
    compliance_map = {1: "none", 2: "basic", 3: "industry", 4: "strict"}
    answers["compliance"] = compliance_map.get(compliance_choice, "none")

    return answers


def main():
    parser = argparse.ArgumentParser(
        description="Calculate complexity score and recommend architecture tier"
    )
    parser.add_argument(
        "--answers",
        help="Path to JSON file containing answers"
    )
    parser.add_argument(
        "--interactive",
        action="store_true",
        help="Interactive mode to gather answers"
    )
    parser.add_argument(
        "--output",
        help="Path to output JSON file (default: stdout)"
    )

    args = parser.parse_args()

    if not args.answers and not args.interactive:
        print("Error: Must specify either --answers or --interactive", file=sys.stderr)
        parser.print_help()
        sys.exit(1)

    # Load or gather answers
    if args.interactive:
        answers = interactive_mode()
    else:
        answers = load_answers(args.answers)

    # Calculate score
    scorer = ComplexityScorer()
    total_score, tier, rationale, detailed_scores = scorer.calculate_score(answers)

    # Prepare output
    result = {
        "complexity_score": total_score,
        "recommended_tier": tier,
        "rationale": rationale,
        "detailed_scoring": {
            "functional_complexity": detailed_scores["functional"],
            "technical_complexity": detailed_scores["technical"],
            "team_organizational_complexity": detailed_scores["team_org"],
            "non_functional_complexity": detailed_scores["non_functional"]
        },
        "inputs": answers
    }

    # Output result
    if args.output:
        with open(args.output, 'w') as f:
            json.dump(result, f, indent=2)
        print(f"Results written to {args.output}")
    else:
        print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
