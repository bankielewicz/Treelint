"""
Feedback Template Engine (STORY-010)

Implements template selection, field mapping, template rendering, and persistence.
- Template selection with priority chain (custom > operation+status > operation > fallback)
- Field mapping from conversation responses to template sections
- Template rendering with YAML frontmatter and markdown content
- File persistence with unique filenames (timestamp + UUID)
"""

import re
import yaml
from pathlib import Path
from datetime import datetime
from datetime import timezone as dt_timezone
from uuid import uuid4
from typing import Dict, Any, Optional


# =============================================================================
# CONSTANTS
# =============================================================================

VALID_OPERATION_TYPES = {"command", "skill", "subagent", "workflow"}
VALID_STATUS_VALUES = {"passed", "failed", "partial"}
DEFAULT_RESPONSE_MESSAGE = "No response provided"
DEFAULT_TEMPLATE_SECTION_HEADER = "## Additional Feedback"
TEMPLATE_FILENAME_PATTERN = r"^(\w+-)*\w+\.md$"


# =============================================================================
# TEMPLATE SELECTION
# =============================================================================

def select_template(
    operation_type: str,
    status: str,
    user_config: Optional[Dict[str, Any]] = None,
    template_dir: Optional[str] = None
) -> str:
    """
    Select template with priority chain.

    Priority:
    1. Custom template (from user_config)
    2. Operation+Status specific (e.g., command-passed)
    3. Operation generic (e.g., command-generic)
    4. Fallback generic

    Args:
        operation_type: Operation type (command, skill, subagent, workflow)
        status: Status (passed, failed, partial)
        user_config: User configuration dict with custom template paths
        template_dir: Directory containing template files

    Returns:
        Template content as string

    Raises:
        ValueError: If operation_type or status invalid
        FileNotFoundError: If no templates found and no fallback available
    """
    # Validate inputs
    if not operation_type or not isinstance(operation_type, str):
        raise ValueError("operation_type must be non-empty string")
    if not status or not isinstance(status, str):
        raise ValueError("status must be non-empty string")

    # Validate status value
    if status.lower() not in VALID_STATUS_VALUES:
        raise ValueError(
            f"status must be one of {VALID_STATUS_VALUES}, got {status}"
        )

    if template_dir is None:
        template_dir = "devforgeai/templates"

    template_path = Path(template_dir)

    # Priority 1: Check custom templates in user_config
    if user_config and "templates" in user_config and "custom" in user_config["templates"]:
        custom_templates = user_config["templates"]["custom"]
        if operation_type.lower() in custom_templates:
            custom_path = custom_templates[operation_type.lower()]
            expanded_path = Path(custom_path).expanduser()
            if expanded_path.exists():
                return expanded_path.read_text(encoding="utf-8")

    # Priority 2: Operation + Status specific (e.g., command-passed.md)
    operation_status_file = template_path / f"{operation_type.lower()}-{status.lower()}.md"
    if operation_status_file.exists():
        template_content = operation_status_file.read_text(encoding="utf-8")
        return _enhance_template_with_field_mappings(template_content)

    # Priority 3: Operation generic (e.g., command-generic.md)
    operation_generic_file = template_path / f"{operation_type.lower()}-generic.md"
    if operation_generic_file.exists():
        template_content = operation_generic_file.read_text(encoding="utf-8")
        return _enhance_template_with_field_mappings(template_content)

    # Priority 4: Fallback generic.md
    generic_file = template_path / "generic.md"
    if generic_file.exists():
        template_content = generic_file.read_text(encoding="utf-8")
        return _enhance_template_with_field_mappings(template_content)

    # No templates found - check if this is a testing scenario
    if not template_path.exists():
        raise FileNotFoundError(
            f"Template directory not found: {template_dir}"
        )

    # Directory exists but no matching templates found
    all_templates = list(template_path.glob("*.md"))
    if not all_templates:
        # Empty template directory - raise error
        raise FileNotFoundError(
            f"No templates found in {template_dir}"
        )

    # Template directory has files but none match our criteria
    # This shouldn't happen in normal flow, but might in tests
    raise FileNotFoundError(
        f"No template found for operation_type={operation_type}, status={status}"
    )


def _validate_operation_type(operation_type: str) -> None:
    """Validate operation type format."""
    if not operation_type or not isinstance(operation_type, str):
        raise ValueError("operation_type must be non-empty string")
    if operation_type.lower() not in VALID_OPERATION_TYPES:
        raise ValueError(
            f"operation_type must be one of {VALID_OPERATION_TYPES}, got {operation_type}"
        )


def _validate_status(status: str) -> None:
    """Validate status format."""
    if not status or not isinstance(status, str):
        raise ValueError("status must be non-empty string")
    if status.lower() not in VALID_STATUS_VALUES:
        raise ValueError(
            f"status must be one of {VALID_STATUS_VALUES}, got {status}"
        )


def _extract_field_mappings_from_markdown(markdown_text: str) -> Dict[str, Any]:
    """Extract field mappings from markdown section."""
    field_mappings = {}

    lines = markdown_text.split("\n")
    current_field = None

    for line in lines:
        line = line.rstrip()

        # Skip empty lines and markdown headers
        if not line or line.startswith("#"):
            continue

        # Lines with colons at root level are field mappings
        if ":" in line and not line.startswith("  "):
            # Parse field: value format
            parts = line.split(":", 1)
            field_name = parts[0].strip()
            current_field = field_name
            field_mappings[field_name] = {}
        elif line.startswith("  ") and current_field:
            # Nested properties (question-id, section)
            nested_line = line.strip()
            if ":" in nested_line:
                key, value = nested_line.split(":", 1)
                key = key.strip()
                value = value.strip().strip('"\'')

                # Normalize key names
                if key == "question-id":
                    key = "question_id"
                elif key == "section":
                    key = "section"

                field_mappings[current_field][key] = value

    return field_mappings


def _enhance_template_with_field_mappings(template_content: str) -> str:
    """
    Enhance template by moving field_mappings from markdown to YAML frontmatter.

    This enables templates to be read with just YAML extraction while
    maintaining readable markdown documentation.
    """
    parts = template_content.split("---")
    if len(parts) < 3:
        # Not a valid template format, return as-is
        return template_content

    yaml_section = parts[1].strip()
    markdown_section = "---".join(parts[2:])

    # Extract field_mappings from markdown
    field_mappings = _extract_field_mappings_from_markdown(markdown_section)

    if not field_mappings:
        # No field_mappings found, return template as-is
        return template_content

    # Add field_mappings to YAML section
    # Parse existing YAML
    try:
        yaml_dict = yaml.safe_load(yaml_section) or {}
    except yaml.YAMLError:
        # If YAML parsing fails, return as-is
        return template_content

    # Add field_mappings to YAML dict
    yaml_dict["field_mappings"] = field_mappings

    # Regenerate YAML section with field_mappings
    new_yaml = yaml.dump(yaml_dict, default_flow_style=False, allow_unicode=True).strip()

    # Reconstruct template
    return f"---\n{new_yaml}\n---\n{markdown_section}"


# =============================================================================
# FIELD MAPPING
# =============================================================================

def map_fields(
    template_or_dict: Any,
    conversation_responses: Dict[str, Any]
) -> Dict[str, Any]:
    """
    Map conversation responses to template sections.

    Maps field_mappings from template to conversation responses:
    - For each field mapping, looks up question_id in responses
    - Uses response value or "No response provided" default
    - Collects unmapped responses in "## Additional Feedback" section

    Args:
        template_or_dict: Parsed template dict with field_mappings, or full template string
        conversation_responses: Dict of question_id -> response value

    Returns:
        Dict of {section_header: response_text}

    Raises:
        ValueError: If template format invalid
    """
    # Support both string (full template) and dict (parsed template)
    template_dict = template_or_dict

    if isinstance(template_dict, str):
        # Parse the full template string
        template_dict = _parse_template_content(template_dict)

    if not isinstance(template_dict, dict):
        return {}

    # Support both raw YAML dicts and parsed templates with field_mappings
    field_mappings = template_dict.get("field_mappings")

    if not field_mappings:
        # If no field_mappings found, return empty dict
        # The template may not have field_mappings defined (tests may pass raw YAML)
        return {}

    # Validate template structure
    for field_name, mapping in field_mappings.items():
        if not isinstance(mapping, dict):
            raise ValueError(f"Field mapping for {field_name} must be dict")
        if "question_id" not in mapping:
            raise ValueError(f"Field mapping for {field_name} missing 'question_id'")
        if "section" not in mapping:
            raise ValueError(f"Field mapping for {field_name} missing 'section'")

        question_id = mapping["question_id"]
        section_header = mapping["section"]

        if not question_id or not isinstance(question_id, str):
            raise ValueError(f"question_id for {field_name} must be non-empty string")
        if not section_header.startswith("##"):
            raise ValueError(
                f"section header for {field_name} must start with ##, got {section_header}"
            )

    # Map fields from responses
    mapped_sections = {}
    mapped_question_ids = set()

    for field_name, mapping in field_mappings.items():
        question_id = mapping["question_id"]
        section_header = mapping["section"]

        # Get response or use default
        if question_id in conversation_responses:
            response = conversation_responses[question_id]
            mapped_question_ids.add(question_id)

            # Handle None or empty string appropriately
            if response is None:
                response = DEFAULT_RESPONSE_MESSAGE
            # Empty string is preserved as-is, not replaced with default
        else:
            response = DEFAULT_RESPONSE_MESSAGE

        mapped_sections[section_header] = response

    # Collect unmapped responses
    unmapped = {}
    for question_id, response in conversation_responses.items():
        if question_id not in mapped_question_ids:
            # Ignore system fields like sentiment_rating
            if not question_id.startswith("sentiment_") and question_id != "additional_feedback":
                unmapped[question_id] = response

    # Add unmapped responses to Additional Feedback section
    if unmapped:
        additional_feedback_lines = []
        for question_id, response in unmapped.items():
            additional_feedback_lines.append(f"- **{question_id}**: {response}")

        mapped_sections[DEFAULT_TEMPLATE_SECTION_HEADER] = "\n".join(additional_feedback_lines)

    return mapped_sections


# =============================================================================
# TEMPLATE RENDERING
# =============================================================================

def render_template(
    template_content: str,
    responses: Dict[str, Any],
    metadata: Dict[str, Any]
) -> str:
    """
    Render template with responses and metadata.

    Generates YAML frontmatter from metadata and assembles markdown sections.
    Auto-populates Context, User Sentiment, and Actionable Insights sections.

    Args:
        template_content: Template string (YAML frontmatter + markdown)
        responses: Conversation responses dict
        metadata: Operation metadata (operation, type, status, timestamp, etc.)

    Returns:
        Rendered template string (YAML frontmatter + markdown)
    """
    # Parse template
    template_dict = _parse_template_content(template_content)

    # Map fields
    mapped_sections = map_fields(template_dict, responses)

    # Generate frontmatter (include both operation metadata and template metadata)
    frontmatter_dict = dict(metadata)  # Copy operation metadata
    if "version" in template_dict:
        frontmatter_dict["template_version"] = template_dict["version"]
    if "template-id" in template_dict:
        frontmatter_dict["template_id"] = template_dict["template-id"]

    frontmatter = _generate_frontmatter(frontmatter_dict)

    # Auto-generate Context section
    context_section = _generate_context_section(metadata)
    mapped_sections["## Context"] = context_section

    # Auto-calculate User Sentiment
    sentiment = _calculate_sentiment(responses)
    mapped_sections["## User Sentiment"] = sentiment

    # Extract actionable insights
    suggestions_text = ""
    for response in responses.values():
        if isinstance(response, str) and any(word in response.lower() for word in ["should", "could", "needs"]):
            suggestions_text = response
            break

    if suggestions_text:
        insights = _extract_insights(suggestions_text)
        if insights:
            mapped_sections["## Actionable Insights"] = "\n".join(f"- {insight}" for insight in insights)
    else:
        mapped_sections["## Actionable Insights"] = "No specific actionable insights extracted."

    # Assemble sections
    sections_content = _assemble_sections(mapped_sections)

    # Combine frontmatter and content
    rendered = f"---\n{frontmatter}---\n\n{sections_content}"

    return rendered


def _parse_template_content(template_content: str) -> Dict[str, Any]:
    """Parse template content (YAML frontmatter + field mappings in markdown)."""
    # Split on YAML delimiters
    parts = template_content.split("---")
    if len(parts) < 3:
        raise ValueError("Template must contain YAML frontmatter (delimited by ---)")

    # Parse YAML frontmatter
    yaml_content = parts[1].strip()
    template_dict = yaml.safe_load(yaml_content) or {}

    # Parse field mappings from markdown section
    markdown_section = "---".join(parts[2:]) if len(parts) > 2 else ""
    field_mappings = _extract_field_mappings_from_markdown(markdown_section)

    if field_mappings:
        template_dict["field_mappings"] = field_mappings

    return template_dict


def _generate_frontmatter(metadata: Dict[str, Any]) -> str:
    """Generate YAML frontmatter from metadata dict."""
    # Use yaml.dump to properly escape special characters
    frontmatter_yaml = yaml.dump(metadata, default_flow_style=False, allow_unicode=True)
    return frontmatter_yaml


def _generate_context_section(metadata: Dict[str, Any]) -> str:
    """Auto-generate context section from metadata."""
    context_lines = []

    if "operation" in metadata:
        context_lines.append(f"**Operation**: {metadata['operation']}")

    if "type" in metadata:
        context_lines.append(f"**Type**: {metadata['type']}")

    if "status" in metadata:
        context_lines.append(f"**Status**: {metadata['status']}")

    if "timestamp" in metadata:
        context_lines.append(f"**Timestamp**: {metadata['timestamp']}")

    if "story_id" in metadata and metadata["story_id"]:
        context_lines.append(f"**Story ID**: {metadata['story_id']}")

    if "duration_seconds" in metadata:
        context_lines.append(f"**Duration**: {metadata['duration_seconds']} seconds")

    if "token_usage" in metadata:
        context_lines.append(f"**Token Usage**: {metadata['token_usage']} tokens")

    return "\n".join(context_lines) if context_lines else "No context available"


def _calculate_sentiment(responses: Dict[str, Any]) -> str:
    """Calculate user sentiment from responses."""
    # Look for sentiment_rating field
    if "sentiment_rating" in responses:
        rating = responses["sentiment_rating"]
        if isinstance(rating, int):
            if rating >= 4:
                return f"Positive ({rating}/5)"
            elif rating >= 3:
                return f"Neutral ({rating}/5)"
            else:
                return f"Negative ({rating}/5)"

    return "Neutral"


def _extract_insights(suggestions_text: str) -> list:
    """Extract actionable insights from suggestions text."""
    insights = []

    # Look for sentences with action words
    action_words = ["should", "could", "needs", "must", "recommend"]

    for word in action_words:
        pattern = rf"[^.!?]*\b{word}\b[^.!?]*[.!?]"
        matches = re.findall(pattern, suggestions_text, re.IGNORECASE)
        for match in matches:
            insight = match.strip()
            if insight and insight not in insights:
                insights.append(insight)

    return insights


def _assemble_sections(sections: Dict[str, Any]) -> str:
    """Assemble markdown sections with headers."""
    lines = []

    for section_header, content in sections.items():
        lines.append(f"{section_header}\n")

        # Handle different content types
        if isinstance(content, str):
            lines.append(content)
        elif isinstance(content, (int, float)):
            lines.append(str(content))
        elif isinstance(content, list):
            lines.append("\n".join(content))
        else:
            lines.append(str(content))

        lines.append("")  # Blank line between sections

    return "\n".join(lines)


# =============================================================================
# TEMPLATE PERSISTENCE
# =============================================================================

def save_rendered_template(
    rendered_content: str,
    operation_type: str,
    output_dir: str = "devforgeai/feedback"
) -> Path:
    """
    Save rendered template to file.

    Generates unique filename: {timestamp}-{uuid}-retrospective.md
    Creates output directory if needed.

    Args:
        rendered_content: Rendered template string
        operation_type: Operation type (for directory organization)
        output_dir: Output directory (default: devforgeai/feedback)

    Returns:
        Path to created file
    """
    # Create output directory with operation-type subdirectory
    output_path = Path(output_dir) / operation_type
    output_path.mkdir(parents=True, exist_ok=True)

    # Generate unique filename (timestamp + UUID to prevent collisions)
    timestamp = datetime.now(dt_timezone.utc).strftime("%Y%m%d-%H%M%S")
    unique_id = str(uuid4())[:8]  # Use first 8 chars of UUID
    filename = f"{timestamp}-{unique_id}-retrospective.md"

    # Write file
    filepath = output_path / filename
    filepath.write_text(rendered_content, encoding="utf-8")

    return filepath
