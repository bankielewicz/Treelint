#!/usr/bin/env python3
"""
Technical Specification Validator for DevForgeAI v2.0

Validates structured technical specifications in story files.

Usage:
    python validate_tech_spec.py <story-file.md>

Exit codes:
    0: Validation passed
    1: Validation failed (errors found)
    2: Invalid arguments or file not found
"""

import yaml
import re
import sys
from typing import Dict, List, Any, Tuple
from pathlib import Path


class TechSpecValidator:
    """Validates structured technical specifications in DevForgeAI stories."""

    # Component types and their required fields
    REQUIRED_COMPONENT_FIELDS = {
        "Service": ["type", "name", "file_path", "requirements"],
        "Worker": ["type", "name", "file_path", "requirements"],
        "Configuration": ["type", "name", "file_path", "required_keys"],
        "Logging": ["type", "name", "file_path", "sinks"],
        "Repository": ["type", "name", "file_path", "interface", "data_access", "requirements"],
        "API": ["type", "name", "endpoint", "method", "request", "response"],
        "DataModel": ["type", "name", "table", "fields"]
    }

    VALID_COMPONENT_TYPES = list(REQUIRED_COMPONENT_FIELDS.keys())

    VALID_NFR_CATEGORIES = ["Performance", "Security", "Scalability", "Reliability", "Usability", "Maintainability"]

    def __init__(self, story_file_path: str):
        """
        Initialize validator with story file path.

        Args:
            story_file_path: Path to story .md file
        """
        self.story_file = Path(story_file_path)
        self.tech_spec = None
        self.errors: List[str] = []
        self.warnings: List[str] = []

        if not self.story_file.exists():
            raise FileNotFoundError(f"Story file not found: {story_file_path}")

    def validate(self) -> bool:
        """
        Validate technical specification structure.

        Returns:
            True if valid (no errors), False otherwise
        """
        # Step 1: Extract tech spec from story file
        self.tech_spec = self._extract_tech_spec()
        if not self.tech_spec:
            self.errors.append("Technical Specification section not found or empty")
            return False

        # Step 2: Validate format version
        if not self._validate_format_version():
            return False

        # Step 3: Validate components
        if not self._validate_components():
            return False

        # Step 4: Validate test requirements
        self._validate_test_requirements()

        # Step 5: Validate business rules
        self._validate_business_rules()

        # Step 6: Validate NFRs
        self._validate_nfrs()

        # Step 7: Validate ID uniqueness
        self._validate_id_uniqueness()

        return len(self.errors) == 0

    def _extract_tech_spec(self) -> Dict[str, Any]:
        """Extract technical specification YAML from story file."""
        try:
            content = self.story_file.read_text(encoding='utf-8')
        except Exception as e:
            self.errors.append(f"Error reading file: {e}")
            return None

        # Find tech spec section with YAML code block
        match = re.search(
            r"## Technical Specification\s+```yaml\s+(.*?)\s+```",
            content,
            re.DOTALL
        )

        if not match:
            # Check if freeform format (v1.0)
            if "## Technical Specification" in content:
                self.warnings.append("Story uses v1.0 freeform format (not structured YAML)")
            return None

        # Parse YAML
        yaml_content = match.group(1)
        try:
            return yaml.safe_load(yaml_content)
        except yaml.YAMLError as e:
            self.errors.append(f"Invalid YAML in tech spec: {e}")
            return None

    def _validate_format_version(self) -> bool:
        """Validate format version is 2.0."""
        if "technical_specification" not in self.tech_spec:
            self.errors.append("Missing 'technical_specification' root key")
            return False

        version = self.tech_spec["technical_specification"].get("format_version")

        if not version:
            self.errors.append("Missing 'format_version' field")
            return False

        if version != "2.0":
            self.warnings.append(f"Format version {version} (expected 2.0)")

        return True

    def _validate_components(self) -> bool:
        """Validate all components have required fields."""
        tech_spec_root = self.tech_spec.get("technical_specification", {})
        components = tech_spec_root.get("components", [])

        if not components:
            self.errors.append("No components defined (components array is empty)")
            return False

        if not isinstance(components, list):
            self.errors.append("'components' must be an array")
            return False

        for idx, component in enumerate(components):
            if not isinstance(component, dict):
                self.errors.append(f"Component {idx}: Must be an object (found {type(component).__name__})")
                continue

            comp_type = component.get("type")
            comp_name = component.get("name", f"Component {idx}")

            # Validate type field exists
            if not comp_type:
                self.errors.append(f"{comp_name}: Missing 'type' field")
                continue

            # Validate type is recognized
            if comp_type not in self.VALID_COMPONENT_TYPES:
                self.errors.append(
                    f"{comp_name}: Unknown type '{comp_type}' "
                    f"(valid types: {', '.join(self.VALID_COMPONENT_TYPES)})"
                )
                continue

            # Validate required fields for this component type
            required_fields = self.REQUIRED_COMPONENT_FIELDS[comp_type]
            for field in required_fields:
                if field not in component:
                    self.errors.append(
                        f"{comp_name} ({comp_type}): Missing required field '{field}'"
                    )

            # Validate requirements array (if applicable)
            if "requirements" in required_fields:
                self._validate_component_requirements(component, comp_name)

        return len(self.errors) == 0

    def _validate_component_requirements(self, component: dict, comp_name: str):
        """Validate requirements array for a component."""
        requirements = component.get("requirements", [])

        if not requirements:
            self.warnings.append(f"{comp_name}: No requirements defined")
            return

        if not isinstance(requirements, list):
            self.errors.append(f"{comp_name}: 'requirements' must be an array")
            return

        for idx, req in enumerate(requirements):
            if not isinstance(req, dict):
                self.errors.append(f"{comp_name} requirement {idx}: Must be an object")
                continue

            # Validate required requirement fields
            req_id = req.get("id", f"Req {idx}")
            required_req_fields = ["id", "description", "testable", "test_requirement"]

            for field in required_req_fields:
                if field not in req:
                    self.errors.append(f"{comp_name} {req_id}: Missing '{field}' field")

            # Validate testable is boolean
            if "testable" in req and not isinstance(req["testable"], bool):
                self.errors.append(f"{comp_name} {req_id}: 'testable' must be boolean (true/false)")

            # Validate test_requirement format
            if "test_requirement" in req:
                test_req = req["test_requirement"]
                if not test_req.startswith("Test: "):
                    self.warnings.append(
                        f"{comp_name} {req_id}: test_requirement should start with 'Test: ' prefix"
                    )

    def _validate_test_requirements(self):
        """Validate all components have test requirements."""
        tech_spec_root = self.tech_spec.get("technical_specification", {})
        components = tech_spec_root.get("components", [])

        for component in components:
            comp_name = component.get("name", "Unknown")
            comp_type = component.get("type", "Unknown")

            # Check requirements field (Service, Worker, Repository)
            if comp_type in ["Service", "Worker", "Repository"]:
                if "requirements" in component:
                    for req in component["requirements"]:
                        if "test_requirement" not in req:
                            self.warnings.append(
                                f"{comp_name}: Requirement '{req.get('id', 'Unknown')}' missing 'test_requirement' field"
                            )

            # Check required_keys field (Configuration)
            elif comp_type == "Configuration":
                if "required_keys" in component:
                    for key in component["required_keys"]:
                        if "test_requirement" not in key:
                            self.warnings.append(
                                f"{comp_name}: Key '{key.get('key', 'Unknown')}' missing 'test_requirement' field"
                            )

            # Check sinks field (Logging)
            elif comp_type == "Logging":
                if "sinks" in component:
                    for sink in component["sinks"]:
                        if "test_requirement" not in sink:
                            self.warnings.append(
                                f"{comp_name}: Sink '{sink.get('name', 'Unknown')}' missing 'test_requirement' field"
                            )

            # Check fields (DataModel)
            elif comp_type == "DataModel":
                if "fields" in component:
                    fields_with_tests = [f for f in component["fields"] if "test_requirement" in f]
                    if len(fields_with_tests) == 0:
                        self.warnings.append(f"{comp_name}: No fields have test requirements")

    def _validate_business_rules(self):
        """Validate business rules structure."""
        tech_spec_root = self.tech_spec.get("technical_specification", {})
        rules = tech_spec_root.get("business_rules", [])

        if not isinstance(rules, list):
            if rules:  # Only error if business_rules exists but isn't a list
                self.errors.append("'business_rules' must be an array")
            return

        for idx, rule in enumerate(rules):
            if not isinstance(rule, dict):
                self.errors.append(f"Business rule {idx}: Must be an object")
                continue

            rule_id = rule.get("id", f"Rule {idx}")

            # Validate required fields
            required_fields = ["id", "rule", "test_requirement"]
            for field in required_fields:
                if field not in rule:
                    self.warnings.append(f"Business rule {rule_id}: Missing '{field}' field")

            # Validate test requirement format
            if "test_requirement" in rule:
                test_req = rule["test_requirement"]
                if not test_req.startswith("Test: "):
                    self.warnings.append(
                        f"Business rule {rule_id}: test_requirement should start with 'Test: ' prefix"
                    )

    def _validate_nfrs(self):
        """Validate non-functional requirements."""
        tech_spec_root = self.tech_spec.get("technical_specification", {})
        nfrs = tech_spec_root.get("non_functional_requirements", [])

        if not isinstance(nfrs, list):
            if nfrs:  # Only error if nfrs exists but isn't a list
                self.errors.append("'non_functional_requirements' must be an array")
            return

        for idx, nfr in enumerate(nfrs):
            if not isinstance(nfr, dict):
                self.errors.append(f"NFR {idx}: Must be an object")
                continue

            nfr_id = nfr.get("id", f"NFR {idx}")

            # Validate required fields
            required_fields = ["id", "requirement", "metric", "test_requirement"]
            for field in required_fields:
                if field not in nfr:
                    self.warnings.append(f"NFR {nfr_id}: Missing '{field}' field")

            # Validate category if present
            if "category" in nfr:
                category = nfr["category"]
                if category not in self.VALID_NFR_CATEGORIES:
                    self.warnings.append(
                        f"NFR {nfr_id}: Unknown category '{category}' "
                        f"(valid: {', '.join(self.VALID_NFR_CATEGORIES)})"
                    )

            # Validate metric is measurable (contains number or threshold)
            if "metric" in nfr:
                metric = str(nfr["metric"])
                has_number = bool(re.search(r'\d+', metric))
                has_threshold = any(word in metric.lower() for word in ["<", ">", "within", "max", "min", "range"])

                if not (has_number or has_threshold):
                    self.warnings.append(
                        f"NFR {nfr_id}: metric should be measurable (contain number or threshold)"
                    )

    def _validate_id_uniqueness(self):
        """Validate all IDs are unique within their scope."""
        tech_spec_root = self.tech_spec.get("technical_specification", {})

        # Collect all IDs
        all_ids: Dict[str, List[str]] = {
            "component_requirements": [],
            "business_rules": [],
            "nfrs": []
        }

        # Collect component requirement IDs
        components = tech_spec_root.get("components", [])
        for component in components:
            if "requirements" in component:
                for req in component["requirements"]:
                    if "id" in req:
                        all_ids["component_requirements"].append(req["id"])

        # Collect business rule IDs
        rules = tech_spec_root.get("business_rules", [])
        for rule in rules:
            if "id" in rule:
                all_ids["business_rules"].append(rule["id"])

        # Collect NFR IDs
        nfrs = tech_spec_root.get("non_functional_requirements", [])
        for nfr in nfrs:
            if "id" in nfr:
                all_ids["nfrs"].append(nfr["id"])

        # Check for duplicates within each scope
        for scope, ids in all_ids.items():
            seen = set()
            for id_val in ids:
                if id_val in seen:
                    self.errors.append(f"Duplicate ID '{id_val}' in {scope}")
                seen.add(id_val)

    def get_report(self) -> str:
        """
        Generate validation report.

        Returns:
            Formatted report string with errors and warnings
        """
        report = []

        if self.errors:
            report.append("❌ VALIDATION FAILED\n")
            report.append("Errors:")
            for error in self.errors:
                report.append(f"  - {error}")
        else:
            report.append("✅ VALIDATION PASSED\n")

        if self.warnings:
            report.append("\nWarnings:")
            for warning in self.warnings:
                report.append(f"  - {warning}")

        if not self.errors and not self.warnings:
            report.append("No issues found. Technical specification is valid.")

        return "\n".join(report)

    def get_summary(self) -> Dict[str, Any]:
        """
        Get validation summary as dictionary.

        Returns:
            Dictionary with validation results
        """
        tech_spec_root = self.tech_spec.get("technical_specification", {}) if self.tech_spec else {}

        components = tech_spec_root.get("components", [])
        rules = tech_spec_root.get("business_rules", [])
        nfrs = tech_spec_root.get("non_functional_requirements", [])

        return {
            "valid": len(self.errors) == 0,
            "format_version": tech_spec_root.get("format_version", "unknown"),
            "component_count": len(components),
            "business_rule_count": len(rules),
            "nfr_count": len(nfrs),
            "error_count": len(self.errors),
            "warning_count": len(self.warnings),
            "errors": self.errors,
            "warnings": self.warnings
        }


def main():
    """CLI entry point."""
    if len(sys.argv) < 2:
        print("Usage: validate_tech_spec.py <story-file.md>")
        print("\nValidates structured technical specifications (v2.0 format)")
        print("\nExit codes:")
        print("  0: Validation passed")
        print("  1: Validation failed (errors found)")
        print("  2: Invalid arguments or file not found")
        sys.exit(2)

    story_file = sys.argv[1]

    try:
        validator = TechSpecValidator(story_file)
        is_valid = validator.validate()

        print(validator.get_report())

        # Print summary
        summary = validator.get_summary()
        print(f"\nSummary:")
        print(f"  Components: {summary['component_count']}")
        print(f"  Business Rules: {summary['business_rule_count']}")
        print(f"  NFRs: {summary['nfr_count']}")
        print(f"  Errors: {summary['error_count']}")
        print(f"  Warnings: {summary['warning_count']}")

        sys.exit(0 if is_valid else 1)

    except FileNotFoundError as e:
        print(f"❌ Error: {e}")
        sys.exit(2)
    except Exception as e:
        print(f"❌ Unexpected error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(2)


if __name__ == "__main__":
    main()
