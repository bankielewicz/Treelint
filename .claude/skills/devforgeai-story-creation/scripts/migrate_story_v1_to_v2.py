#!/usr/bin/env python3
"""
Story Migration Script: v1.0 (Freeform) → v2.0 (Structured YAML)

Converts freeform technical specifications to machine-readable YAML format.

Usage:
    python migrate_story_v1_to_v2.py <story-file.md> [OPTIONS]

Options:
    --dry-run         Show changes without modifying files
    --validate        Run validator after migration
    --ai-assisted     Use AI for intelligent parsing (95%+ accuracy)
    --ai              Alias for --ai-assisted
    --backup          Create backup before migration (default: true)
    --no-backup       Skip backup creation

Examples:
    # AI-assisted migration with validation (RECOMMENDED)
    python migrate_story_v1_to_v2.py STORY-001.md --ai-assisted --validate

    # Pattern matching only (faster but less accurate)
    python migrate_story_v1_to_v2.py STORY-001.md --validate

    # Dry run to preview changes
    python migrate_story_v1_to_v2.py STORY-001.md --ai --dry-run

Exit codes:
    0: Migration successful
    1: Migration failed (validation errors or conversion issues)
    2: Invalid arguments or prerequisites not met
"""

import re
import sys
import yaml
import shutil
import os
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Any, Optional

# Check if anthropic library available
try:
    import anthropic
    ANTHROPIC_AVAILABLE = True
except ImportError:
    ANTHROPIC_AVAILABLE = False


class AIConverter:
    """AI-assisted conversion using Claude API."""

    def __init__(self):
        """Initialize AI converter with Claude API."""
        self.api_key = os.environ.get("ANTHROPIC_API_KEY")
        self.client = None
        self.prompt_template = None
        self.schema_reference = None

        if self.api_key and ANTHROPIC_AVAILABLE:
            try:
                self.client = anthropic.Anthropic(api_key=self.api_key)
            except Exception as e:
                print(f"⚠️ Failed to initialize Claude API: {e}")
        elif not ANTHROPIC_AVAILABLE and self.api_key:
            print("⚠️ anthropic package not installed. Run: pip install anthropic")

    def is_available(self) -> bool:
        """Check if AI conversion is available."""
        return self.client is not None

    def convert(self, freeform_text: str) -> Optional[str]:
        """
        Convert freeform text to YAML using Claude API.

        Args:
            freeform_text: Freeform technical specification text

        Returns:
            YAML string or None if conversion failed
        """
        if not self.is_available():
            return None

        # Build conversion prompt
        prompt = self._build_prompt(freeform_text)

        try:
            # Call Claude API (Haiku model - fast and cost-effective)
            response = self.client.messages.create(
                model="claude-3-haiku-20240307",
                max_tokens=4000,
                temperature=0.3,  # Low temperature for consistent output
                messages=[{
                    "role": "user",
                    "content": prompt
                }]
            )

            yaml_text = response.content[0].text

            # Extract YAML if wrapped in markdown code blocks
            yaml_text = self._extract_yaml(yaml_text)

            return yaml_text

        except Exception as e:
            print(f"⚠️ Claude API error: {e}")
            return None

    def _build_prompt(self, freeform_text: str) -> str:
        """Build conversion prompt with template and schema reference."""

        # Load prompt template (cached)
        if self.prompt_template is None:
            template_file = Path(__file__).parent / "conversion_prompt_template.txt"
            if template_file.exists():
                self.prompt_template = template_file.read_text(encoding='utf-8')
            else:
                # Fallback to basic template if file missing
                self.prompt_template = "Convert freeform tech spec to v2.0 YAML:\n\n{freeform_text}"

        # Format prompt with freeform text
        prompt = self.prompt_template.format(freeform_text=freeform_text)

        return prompt

    def _extract_yaml(self, response_text: str) -> str:
        """Extract YAML from AI response (handle markdown wrapping)."""

        # Check for YAML code block
        if "```yaml" in response_text:
            match = re.search(r"```yaml\n(.*?)\n```", response_text, re.DOTALL)
            if match:
                return match.group(1)

        # Check for generic code block
        if "```" in response_text:
            match = re.search(r"```\n(.*?)\n```", response_text, re.DOTALL)
            if match:
                return match.group(1)

        # No wrapping, return as-is
        return response_text.strip()


class StoryMigrator:
    """Migrates DevForgeAI stories from v1.0 to v2.0 format."""

    def __init__(self, story_file_path: str, dry_run: bool = False, create_backup: bool = True, use_ai: bool = False):
        """
        Initialize migrator.

        Args:
            story_file_path: Path to story .md file
            dry_run: If True, don't modify files
            create_backup: If True, create backup before migration
            use_ai: If True, use AI-assisted conversion (95%+ accuracy)
        """
        self.story_file = Path(story_file_path)
        self.dry_run = dry_run
        self.create_backup = create_backup
        self.use_ai = use_ai
        self.original_content = None
        self.migrated_content = None
        self.errors: List[str] = []
        self.warnings: List[str] = []

        # Initialize AI converter if requested
        self.ai_converter = None
        if use_ai:
            self.ai_converter = AIConverter()
            if not self.ai_converter.is_available():
                print("⚠️ AI conversion not available (no API key or anthropic not installed)")
                print("   Falling back to pattern matching (60-70% accuracy)")
                print("   To enable AI: pip install anthropic && export ANTHROPIC_API_KEY='your-key'")

        if not self.story_file.exists():
            raise FileNotFoundError(f"Story file not found: {story_file_path}")

    def migrate(self) -> bool:
        """
        Execute migration from v1.0 to v2.0.

        Returns:
            True if successful, False otherwise
        """
        try:
            # Step 1: Read story file
            self.original_content = self.story_file.read_text(encoding='utf-8')

            # Step 2: Check if already v2.0
            if self._is_v2_format(self.original_content):
                print(f"✓ {self.story_file.name}: Already v2.0 format (no migration needed)")
                return True

            # Step 3: Create backup
            if self.create_backup and not self.dry_run:
                self._create_backup()

            # Step 4: Extract freeform tech spec
            tech_spec_text = self._extract_freeform_tech_spec()
            if not tech_spec_text:
                self.errors.append("No Technical Specification section found")
                return False

            # Step 5: Convert to structured format
            print(f"🔄 Converting {self.story_file.name} to v2.0 format...")
            structured_spec = self._convert_to_structured_format(tech_spec_text)

            if not structured_spec:
                self.errors.append("Failed to convert tech spec to structured format")
                return False

            # Step 6: Generate YAML
            yaml_spec = self._generate_yaml(structured_spec)

            # Step 7: Replace tech spec section
            self.migrated_content = self._replace_tech_spec_section(yaml_spec)

            # Step 8: Update format version in frontmatter
            self.migrated_content = self._update_format_version()

            # Step 9: Write migrated content (unless dry-run)
            if not self.dry_run:
                self.story_file.write_text(self.migrated_content, encoding='utf-8')
                print(f"✅ {self.story_file.name}: Migrated to v2.0")
            else:
                print(f"🔍 DRY RUN: Would migrate {self.story_file.name}")
                print("\n--- YAML Tech Spec Preview ---")
                print(yaml_spec[:500] + "..." if len(yaml_spec) > 500 else yaml_spec)

            return True

        except Exception as e:
            self.errors.append(f"Migration exception: {e}")
            import traceback
            traceback.print_exc()
            return False

    def _is_v2_format(self, content: str) -> bool:
        """Check if story already uses v2.0 format."""
        return "```yaml\ntechnical_specification:" in content or \
               '```yaml\ntechnical_specification:' in content or \
               'format_version: "2.0"' in content

    def _create_backup(self):
        """Create backup of original file."""
        timestamp = datetime.now().strftime("%Y%m%d-%H%M%S")
        backup_dir = Path("devforgeai/backups/phase2-migration")
        backup_dir.mkdir(parents=True, exist_ok=True)

        backup_file = backup_dir / f"{self.story_file.stem}-{timestamp}.md"
        shutil.copy2(self.story_file, backup_file)
        print(f"📁 Backup created: {backup_file}")

    def _extract_freeform_tech_spec(self) -> Optional[str]:
        """Extract freeform technical specification section."""
        # Find tech spec section (starts with ## Technical Specification, ends with next ## or end of file)
        match = re.search(
            r"## Technical Specification\s+(.*?)(?=\n## |\Z)",
            self.original_content,
            re.DOTALL
        )

        if not match:
            return None

        return match.group(1).strip()

    def _convert_to_structured_format(self, freeform_text: str) -> Dict[str, Any]:
        """
        Convert freeform text to structured format.

        Strategy:
        1. Try AI-assisted conversion (95%+ accuracy) if enabled and available
        2. Fall back to pattern matching (60-70% accuracy)

        Args:
            freeform_text: Freeform technical specification text

        Returns:
            Structured specification dictionary
        """

        # Strategy 1: AI-Assisted Conversion (PREFERRED)
        if self.use_ai and self.ai_converter and self.ai_converter.is_available():
            print("🤖 Using AI-assisted conversion (95%+ accuracy)...")

            yaml_text = self.ai_converter.convert(freeform_text)

            if yaml_text:
                try:
                    # Parse YAML response
                    full_spec = yaml.safe_load(yaml_text)

                    # Extract technical_specification portion
                    if "technical_specification" in full_spec:
                        tech_spec = full_spec["technical_specification"]
                        # Remove format_version (will be added later)
                        tech_spec.pop("format_version", None)
                        return tech_spec
                    else:
                        # AI might have returned just the inner portion
                        return full_spec

                except yaml.YAMLError as e:
                    self.warnings.append(f"AI generated invalid YAML: {e}")
                    print(f"⚠️ AI conversion produced invalid YAML, falling back to pattern matching")
            else:
                print(f"⚠️ AI conversion failed, falling back to pattern matching")

        # Strategy 2: Pattern Matching (FALLBACK)
        print("🔍 Using pattern matching (60-70% accuracy)")
        return self._convert_with_pattern_matching(freeform_text)

    def _convert_with_pattern_matching(self, freeform_text: str) -> Dict[str, Any]:
        """
        Convert using pattern matching (fallback method).

        Args:
            freeform_text: Freeform technical specification text

        Returns:
            Structured specification dictionary
        """
        structured = {
            "components": [],
            "business_rules": [],
            "non_functional_requirements": []
        }

        # Parse components based on keywords

        # Detect Workers (polling, background, scheduled)
        if re.search(r'\b(worker|polling|background|scheduled)\b', freeform_text, re.IGNORECASE):
            worker_name = self._extract_class_name(freeform_text, r'(\w+Worker)')
            if worker_name:
                structured["components"].append({
                    "type": "Worker",
                    "name": worker_name,
                    "file_path": f"src/Workers/{worker_name}.cs",  # Inferred
                    "interface": "BackgroundService",
                    "requirements": [
                        {
                            "id": "WKR-001",
                            "description": "Extracted from freeform text",
                            "testable": True,
                            "test_requirement": "Test: Worker executes as specified",
                            "priority": "High"
                        }
                    ]
                })

        # Detect Configuration
        if re.search(r'\b(appsettings|configuration|config)\b', freeform_text, re.IGNORECASE):
            config_keys = self._extract_config_keys(freeform_text)
            if config_keys:
                structured["components"].append({
                    "type": "Configuration",
                    "name": "appsettings.json",
                    "file_path": "src/appsettings.json",
                    "required_keys": config_keys
                })

        # Detect Logging
        if re.search(r'\b(serilog|logging|log)\b', freeform_text, re.IGNORECASE):
            sinks = self._extract_log_sinks(freeform_text)
            if sinks:
                structured["components"].append({
                    "type": "Logging",
                    "name": "Serilog",
                    "file_path": "src/Program.cs",
                    "sinks": sinks
                })

        # Detect Repositories
        if re.search(r'\b(repository|data access|dapper|ef core)\b', freeform_text, re.IGNORECASE):
            repo_name = self._extract_class_name(freeform_text, r'(\w+Repository)')
            if repo_name:
                structured["components"].append({
                    "type": "Repository",
                    "name": repo_name,
                    "file_path": f"src/Infrastructure/Repositories/{repo_name}.cs",
                    "interface": f"I{repo_name}",
                    "data_access": "Dapper",  # Common default
                    "requirements": [
                        {
                            "id": "REPO-001",
                            "description": "Must use parameterized queries",
                            "testable": True,
                            "test_requirement": "Test: Query uses @parameters, not string concatenation",
                            "priority": "Critical"
                        }
                    ]
                })

        # Extract business rules
        business_rules = self._extract_business_rules(freeform_text)
        if business_rules:
            structured["business_rules"] = business_rules

        # Extract NFRs
        nfrs = self._extract_nfrs(freeform_text)
        if nfrs:
            structured["non_functional_requirements"] = nfrs

        # Add note if no components detected
        if not structured["components"]:
            self.warnings.append(
                "No components auto-detected. Manual review required. "
                "Consider using AI-assisted conversion for complex specs."
            )

        return structured

    def _extract_class_name(self, text: str, pattern: str) -> Optional[str]:
        """Extract class name using regex pattern."""
        match = re.search(pattern, text)
        return match.group(1) if match else None

    def _extract_config_keys(self, text: str) -> List[Dict[str, Any]]:
        """Extract configuration keys from freeform text."""
        keys = []

        # Look for ConnectionStrings patterns
        if "ConnectionStrings" in text or "connection string" in text.lower():
            keys.append({
                "key": "ConnectionStrings.DefaultConnection",
                "type": "string",
                "required": True,
                "test_requirement": "Test: Configuration loads connection string"
            })

        # Look for other config mentions
        config_matches = re.findall(r'(\w+\.\w+)\s*(?:=|:)\s*(\w+|"[^"]*")', text)
        for key, value in config_matches[:3]:  # Limit to 3 to avoid noise
            keys.append({
                "key": key,
                "type": "string",
                "required": False,
                "test_requirement": f"Test: Configuration loads {key}"
            })

        return keys

    def _extract_log_sinks(self, text: str) -> List[Dict[str, Any]]:
        """Extract logging sinks from freeform text."""
        sinks = []

        if re.search(r'\b(file|log file)\b', text, re.IGNORECASE):
            sinks.append({
                "name": "File",
                "path": "logs/app-.txt",
                "test_requirement": "Test: Log file created in logs/ directory"
            })

        if re.search(r'\b(event log|windows event)\b', text, re.IGNORECASE):
            sinks.append({
                "name": "EventLog",
                "source": "Application",
                "test_requirement": "Test: Entry written to Windows Event Log"
            })

        if re.search(r'\b(database log|log table)\b', text, re.IGNORECASE):
            sinks.append({
                "name": "Database",
                "table": "Logs",
                "test_requirement": "Test: Log entry written to Logs table"
            })

        return sinks

    def _extract_business_rules(self, text: str) -> List[Dict[str, Any]]:
        """Extract business rules from freeform text."""
        rules = []

        # Look for "Rule:" or "Business Rule:" patterns
        rule_matches = re.findall(
            r'(?:Business )?Rule:?\s*([^\n]+)',
            text,
            re.IGNORECASE
        )

        for idx, rule_text in enumerate(rule_matches[:5], 1):  # Limit to 5 rules
            rules.append({
                "id": f"BR-{idx:03d}",
                "rule": rule_text.strip(),
                "validation": "See implementation",
                "test_requirement": f"Test: {rule_text.strip()}"
            })

        return rules

    def _extract_nfrs(self, text: str) -> List[Dict[str, Any]]:
        """Extract non-functional requirements from freeform text."""
        nfrs = []

        # Look for performance metrics
        perf_matches = re.findall(r'<\s*(\d+)\s*(ms|seconds?|sec)\b', text, re.IGNORECASE)
        for idx, (value, unit) in enumerate(perf_matches[:3], 1):
            nfrs.append({
                "id": f"NFR-{idx:03d}",
                "category": "Performance",
                "requirement": f"Performance requirement extracted from spec",
                "metric": f"< {value} {unit}",
                "test_requirement": f"Test: Measure execution time, assert < {value} {unit}"
            })

        return nfrs

    def _generate_yaml(self, structured_spec: Dict[str, Any]) -> str:
        """
        Generate YAML string from structured specification.

        Args:
            structured_spec: Structured spec dictionary

        Returns:
            YAML formatted string
        """
        # Wrap in technical_specification root with format_version
        full_spec = {
            "technical_specification": {
                "format_version": "2.0",
                **structured_spec
            }
        }

        # Generate YAML with custom formatting
        yaml_str = yaml.dump(full_spec, default_flow_style=False, sort_keys=False, allow_unicode=True)

        return yaml_str

    def _replace_tech_spec_section(self, yaml_spec: str) -> str:
        """Replace freeform tech spec section with YAML."""
        # Find and replace tech spec section
        pattern = r"(## Technical Specification\s+)(.*?)(?=\n## |\Z)"

        replacement = f"\\1```yaml\n{yaml_spec}```\n\n"

        migrated = re.sub(pattern, replacement, self.original_content, flags=re.DOTALL)

        return migrated

    def _update_format_version(self) -> str:
        """Add or update format_version in YAML frontmatter."""
        # Find frontmatter
        frontmatter_match = re.search(r"^---\n(.*?)\n---", self.migrated_content, re.DOTALL)

        if not frontmatter_match:
            self.warnings.append("No YAML frontmatter found, cannot update format_version")
            return self.migrated_content

        frontmatter_text = frontmatter_match.group(1)

        # Parse existing frontmatter
        try:
            frontmatter = yaml.safe_load(frontmatter_text)
        except yaml.YAMLError as e:
            self.warnings.append(f"Invalid frontmatter YAML: {e}")
            return self.migrated_content

        # Add format_version
        frontmatter["format_version"] = "2.0"

        # Regenerate frontmatter YAML
        new_frontmatter_text = yaml.dump(frontmatter, default_flow_style=False, sort_keys=False)

        # Replace in content
        updated = re.sub(
            r"^---\n.*?\n---",
            f"---\n{new_frontmatter_text}---",
            self.migrated_content,
            flags=re.DOTALL,
            count=1
        )

        return updated

    def get_report(self) -> str:
        """Generate migration report."""
        report = []

        if self.dry_run:
            report.append(f"🔍 DRY RUN: {self.story_file.name}")
        else:
            if not self.errors:
                report.append(f"✅ MIGRATION SUCCESS: {self.story_file.name}")
            else:
                report.append(f"❌ MIGRATION FAILED: {self.story_file.name}")

        if self.errors:
            report.append("\nErrors:")
            for error in self.errors:
                report.append(f"  - {error}")

        if self.warnings:
            report.append("\nWarnings:")
            for warning in self.warnings:
                report.append(f"  - {warning}")

        return "\n".join(report)


def main():
    """CLI entry point."""
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(2)

    story_file = sys.argv[1]
    dry_run = "--dry-run" in sys.argv
    validate = "--validate" in sys.argv
    create_backup = "--no-backup" not in sys.argv
    use_ai = "--ai-assisted" in sys.argv or "--ai" in sys.argv

    try:
        # Execute migration
        migrator = StoryMigrator(
            story_file,
            dry_run=dry_run,
            create_backup=create_backup,
            use_ai=use_ai
        )
        success = migrator.migrate()

        # Print report
        print(migrator.get_report())

        # Run validation if requested
        if validate and success and not dry_run:
            print("\n🔍 Running validation...")
            import subprocess
            result = subprocess.run(
                [sys.executable, "validate_tech_spec.py", story_file],
                capture_output=True,
                text=True
            )
            print(result.stdout)
            if result.returncode != 0:
                print("⚠️ Validation failed after migration")
                sys.exit(1)

        sys.exit(0 if success else 1)

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
