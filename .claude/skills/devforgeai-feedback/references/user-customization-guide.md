# User Customization Guide

**Version:** 1.0
**Date:** 2025-11-10
**Status:** Active

This guide explains how to customize feedback templates to match your project's needs and organizational standards.

---

## Overview

The feedback template engine supports three levels of customization:
1. **Configuration-based** - Override template selection via user configuration
2. **Template modification** - Edit existing default templates
3. **Custom templates** - Create entirely new templates for your use cases

---

## Quick Start

### Option 1: Use Default Templates (No Customization)

The template engine includes 7 default templates that work out of the box:

- `command-passed.yaml` - Successful command execution
- `command-failed.yaml` - Failed command execution
- `skill-passed.yaml` - Successful skill execution
- `skill-failed.yaml` - Failed skill execution
- `subagent-passed.yaml` - Successful subagent execution
- `subagent-failed.yaml` - Failed subagent execution
- `generic.yaml` - Fallback for any operation

**No setup required** - Templates work immediately.

### Option 2: Customize Existing Templates

Edit default templates to match your needs:

1. Navigate to: `.claude/skills/devforgeai-feedback/templates/`
2. Open template file (e.g., `command-passed.yaml`)
3. Modify field mappings or section headers
4. Save file
5. Templates immediately available (no restart needed)

### Option 3: Create Custom Templates

Add your own templates for specific workflows:

1. Create new YAML file in `.claude/skills/devforgeai-feedback/templates/`
2. Follow template format specification
3. Save file
4. Template automatically discovered

---

## Configuration-Based Customization

### User Configuration File

**Location:** `devforgeai/feedback/config.yaml` (create if doesn't exist)

**Basic Structure:**
```yaml
feedback:
  templates:
    custom:
      command:
        passed: "custom-command-success"
        failed: "custom-command-failure"
      skill:
        passed: "custom-skill-success"
```

**Effect:** When template engine selects template for `command/passed`, it will use `custom-command-success.yaml` instead of `command-passed.yaml`.

### Configuration Options

#### Override Default Template Selection

```yaml
feedback:
  templates:
    # Directory containing custom templates
    directory: "devforgeai/feedback/custom-templates"

    # Map operation types to custom template IDs
    custom:
      command:
        passed: "my-command-success-template"
        failed: "my-command-failure-template"
        partial: "my-command-partial-template"

      skill:
        passed: "my-skill-success-template"
        failed: "my-skill-failure-template"

      subagent:
        passed: "my-subagent-success-template"
        failed: "my-subagent-failure-template"
```

#### Disable Auto-Populated Sections

```yaml
feedback:
  auto-populate:
    context: true        # Keep context section (default: true)
    sentiment: false     # Disable sentiment section
    insights: true       # Keep actionable insights (default: true)
```

#### Configure Output Location

```yaml
feedback:
  output:
    base-directory: "devforgeai/feedback"    # Default location
    use-subdirectories: true                  # Organize by operation type
    filename-format: "{timestamp}-{uuid}-retrospective.md"  # Default format
```

---

## Template Modification

### Editing Field Mappings

**Scenario:** Add new question to command success template

**Steps:**

1. **Open template file:**
```bash
.claude/skills/devforgeai-feedback/templates/command-passed.yaml
```

2. **Add new field mapping:**
```yaml
field-mappings:
  # Existing mappings...

  team-collaboration:
    question-id: "cmd_passed_05"
    section: "## Team Collaboration"
    description: "How well the team coordinated during command execution"
```

3. **Update template sections list:**
```yaml
## Template Sections

- # Retrospective: {operation}
- ## What Went Well
- ## What Could Improve
- ## Efficiency Notes
- ## Suggestions for Next Time
- ## Team Collaboration           # NEW
- ## Context (auto-populated)
- ## User Sentiment (auto-populated)
- ## Actionable Insights (auto-extracted)
```

4. **Save file** - Template immediately available

### Changing Section Headers

**Scenario:** Rename section from "What Could Improve" to "Areas for Optimization"

**Steps:**

1. **Update field mapping:**
```yaml
field-mappings:
  what-could-improve:
    question-id: "cmd_passed_02"
    section: "## Areas for Optimization"  # Changed
    description: "Areas where the workflow could be more efficient"
```

2. **Update template sections list:**
```yaml
## Template Sections

- # Retrospective: {operation}
- ## What Went Well
- ## Areas for Optimization        # Changed
- ## Efficiency Notes
- ## Suggestions for Next Time
...
```

3. **Save file**

**Note:** Changing section headers affects all future rendered templates. Historical templates with old headers remain unchanged.

### Removing Field Mappings

**Scenario:** Remove "Efficiency Notes" section (not needed for your workflow)

**Steps:**

1. **Comment out or delete field mapping:**
```yaml
field-mappings:
  what-went-well:
    question-id: "cmd_passed_01"
    section: "## What Went Well"

  what-could-improve:
    question-id: "cmd_passed_02"
    section: "## What Could Improve"

  # REMOVED: efficiency-notes
  # efficiency-notes:
  #   question-id: "cmd_passed_03"
  #   section: "## Efficiency Notes"

  suggestions:
    question-id: "cmd_passed_04"
    section: "## Suggestions for Next Time"
```

2. **Update template sections list:**
```yaml
## Template Sections

- # Retrospective: {operation}
- ## What Went Well
- ## What Could Improve
# REMOVED: - ## Efficiency Notes
- ## Suggestions for Next Time
...
```

3. **Save file**

**Caution:** If conversation contains response for `cmd_passed_03`, it will appear in "Additional Feedback" section (unmapped response handling).

---

## Creating Custom Templates

### Scenario 1: Project-Specific Template

**Use Case:** Your project has unique CI/CD workflow requiring custom feedback

**Steps:**

1. **Create template file:**
```bash
.claude/skills/devforgeai-feedback/templates/cicd-deployment.yaml
```

2. **Define template structure:**
```yaml
---
template-id: cicd-deployment
operation-type: workflow
success-status: passed
version: "1.0"
description: "Template for CI/CD deployment feedback"
---

# Template: CI/CD Deployment Retrospective

## Field Mappings

field-mappings:
  build-phase:
    question-id: "cicd_01"
    section: "## Build Phase"
    description: "Build process feedback"

  test-phase:
    question-id: "cicd_02"
    section: "## Test Phase"
    description: "Test execution feedback"

  deployment-phase:
    question-id: "cicd_03"
    section: "## Deployment Phase"
    description: "Deployment execution feedback"

  smoke-tests:
    question-id: "cicd_04"
    section: "## Smoke Tests"
    description: "Post-deployment validation"

## Template Sections

- # CI/CD Deployment: {operation}
- ## Build Phase
- ## Test Phase
- ## Deployment Phase
- ## Smoke Tests
- ## Context (auto-populated)
- ## User Sentiment (auto-populated)
- ## Actionable Insights (auto-extracted)
```

3. **Use template:**
```python
from devforgeai_cli.feedback.template_engine import select_template

template = select_template(
    operation_type="workflow",
    status="passed",
    user_config=None,
    template_dir=".claude/skills/devforgeai-feedback/templates"
)
```

Template engine will find `cicd-deployment.yaml` if filename matches `{operation-type}-{status}.yaml` pattern (or fallback to workflow-passed.yaml, workflow-generic.yaml, generic.yaml).

### Scenario 2: Organizational Standard Template

**Use Case:** Your organization requires specific sections in all retrospectives

**Steps:**

1. **Create organization template directory:**
```bash
mkdir -p devforgeai/feedback/org-templates
```

2. **Create template:**
```bash
devforgeai/feedback/org-templates/org-command-passed.yaml
```

3. **Define with organizational sections:**
```yaml
---
template-id: org-command-passed
operation-type: command
success-status: passed
version: "1.0"
description: "Organizational standard template for command success"
---

# Template: Command Success (Org Standard)

## Field Mappings

field-mappings:
  what-went-well:
    question-id: "org_cmd_01"
    section: "## What Went Well"

  what-went-poorly:
    question-id: "org_cmd_02"
    section: "## What Went Poorly"

  compliance-notes:
    question-id: "org_cmd_03"
    section: "## Compliance Notes"
    description: "How well command execution followed organizational policies"

  security-observations:
    question-id: "org_cmd_04"
    section: "## Security Observations"
    description: "Security-related observations during execution"

  cost-efficiency:
    question-id: "org_cmd_05"
    section: "## Cost Efficiency"
    description: "Cloud resource usage and cost observations"

## Template Sections

- # Retrospective: {operation} (Org Standard)
- ## What Went Well
- ## What Went Poorly
- ## Compliance Notes
- ## Security Observations
- ## Cost Efficiency
- ## Context (auto-populated)
- ## User Sentiment (auto-populated)
- ## Actionable Insights (auto-extracted)
```

4. **Configure to use organizational templates:**
```yaml
# devforgeai/feedback/config.yaml
feedback:
  templates:
    directory: "devforgeai/feedback/org-templates"
    custom:
      command:
        passed: "org-command-passed"
```

5. **All command success feedback now uses organizational template**

---

## Advanced Customization

### Conditional Field Mappings

**Use Case:** Different questions based on story type

**Approach:** Create multiple templates for different scenarios

**Example:**
- `command-passed-crud.yaml` - For CRUD operations
- `command-passed-api.yaml` - For API development
- `command-passed-ui.yaml` - For UI work

**Configuration:**
```yaml
feedback:
  templates:
    custom:
      command:
        passed:
          crud: "command-passed-crud"
          api: "command-passed-api"
          ui: "command-passed-ui"
```

**Selection logic:**
```python
# Determine story type from story file
story_type = determine_story_type(story_file)  # Returns: "crud", "api", or "ui"

# Select template based on story type
template = select_template(
    operation_type="command",
    status="passed",
    user_config={"templates": {"custom": {"command": {"passed": f"command-passed-{story_type}"}}}},
    template_dir=".claude/skills/devforgeai-feedback/templates"
)
```

### Multi-Language Templates

**Use Case:** International teams want templates in multiple languages

**Approach:** Create language-specific template directories

**Structure:**
```
devforgeai/feedback/templates/
├── en/                           # English (default)
│   ├── command-passed.yaml
│   ├── command-failed.yaml
│   └── ...
├── es/                           # Spanish
│   ├── command-passed.yaml
│   ├── command-failed.yaml
│   └── ...
└── fr/                           # French
    ├── command-passed.yaml
    ├── command-failed.yaml
    └── ...
```

**Configuration:**
```yaml
# devforgeai/feedback/config.yaml
feedback:
  templates:
    directory: "devforgeai/feedback/templates/es"  # Use Spanish templates
```

**Template Example (Spanish):**
```yaml
---
template-id: command-passed-es
operation-type: command
success-status: passed
version: "1.0"
description: "Plantilla para ejecución exitosa de comandos"
---

# Plantilla: Retrospectiva de Comando Exitoso

## Field Mappings

field-mappings:
  que-salio-bien:
    question-id: "cmd_passed_01"
    section: "## ¿Qué Salió Bien?"
    description: "Aspectos positivos de la ejecución del comando"

  que-podria-mejorar:
    question-id: "cmd_passed_02"
    section: "## ¿Qué Podría Mejorar?"
    description: "Áreas donde el flujo de trabajo podría ser más eficiente"

  sugerencias:
    question-id: "cmd_passed_03"
    section: "## Sugerencias para la Próxima Vez"
    description: "Recomendaciones específicas para la próxima ejecución"
```

### Template Inheritance

**Use Case:** Share common field mappings across multiple templates

**Approach:** Create base template, reference in specific templates

**Base template:**
```yaml
# base-success.yaml
field-mappings:
  what-went-well:
    question-id: "base_01"
    section: "## What Went Well"

  what-could-improve:
    question-id: "base_02"
    section: "## What Could Improve"

  suggestions:
    question-id: "base_03"
    section: "## Suggestions for Next Time"
```

**Specific template:**
```yaml
# command-passed.yaml
---
template-id: command-passed
operation-type: command
success-status: passed
extends: "base-success"  # Reference base template
version: "1.0"
---

# Additional field mappings specific to commands
field-mappings:
  efficiency-notes:
    question-id: "cmd_passed_04"
    section: "## Efficiency Notes"
```

**Note:** Template inheritance is not currently implemented in template_engine.py. This example shows conceptual approach for future enhancement.

---

## Best Practices

### 1. Version Your Custom Templates

**Why:** Track changes, enable rollback, maintain compatibility

**How:**
- Increment `version` field in YAML frontmatter with each change
- Use semantic versioning (MAJOR.MINOR.PATCH)
- Document changes in template comments

**Example:**
```yaml
---
template-id: command-passed
operation-type: command
success-status: passed
version: "1.1.0"  # Incremented from 1.0.0
description: "Template for successful command execution"
# CHANGELOG:
# 1.1.0 (2025-11-15): Added team-collaboration field
# 1.0.0 (2025-11-10): Initial version
---
```

### 2. Backup Before Modifying

**Why:** Preserve original templates for rollback

**How:**
```bash
# Before editing command-passed.yaml
cp .claude/skills/devforgeai-feedback/templates/command-passed.yaml \
   .claude/skills/devforgeai-feedback/templates/command-passed.yaml.backup
```

### 3. Test Custom Templates

**Why:** Verify rendering before deploying to team

**How:**
```python
# test_custom_template.py
from devforgeai_cli.feedback.template_engine import (
    select_template,
    map_fields,
    render_template
)

# Test template selection
template = select_template("command", "passed", user_config, template_dir)
assert template["template-id"] == "my-custom-template"

# Test field mapping
test_responses = {
    "cmd_passed_01": "Test response 1",
    "cmd_passed_02": "Test response 2",
}
sections = map_fields(template, test_responses)
assert "## What Went Well" in sections

# Test rendering
metadata = {
    "operation": "/dev STORY-042",
    "type": "command",
    "status": "passed",
    "timestamp": "2025-11-10T10:00:00+00:00",
}
rendered = render_template(template, test_responses, metadata)
assert "---" in rendered  # YAML frontmatter present
assert "# Retrospective" in rendered  # Title present
```

### 4. Document Custom Templates

**Why:** Help team understand custom field mappings

**How:** Create README in custom template directory

**Example:**
```markdown
# Custom Templates

## command-passed-crud.yaml

**Purpose:** Feedback template for CRUD operations

**Field Mappings:**
- `cmd_crud_01`: Database performance observations
- `cmd_crud_02`: Query optimization opportunities
- `cmd_crud_03`: Data validation insights
- `cmd_crud_04`: Transaction handling feedback

**Usage:**
Configure in `devforgeai/feedback/config.yaml`:
```yaml
feedback:
  templates:
    custom:
      command:
        passed: "command-passed-crud"
```

**Questions:**
During retrospective conversation, respond to:
1. "How was database performance during CRUD operations?"
2. "Any query optimization opportunities identified?"
3. "Were data validations effective?"
4. "How well did transaction handling work?"
```

### 5. Share Templates Across Projects

**Why:** Standardize feedback across organization

**How:** Create shared template repository

**Structure:**
```
org-feedback-templates/
├── README.md
├── command/
│   ├── command-passed.yaml
│   ├── command-failed.yaml
│   └── ...
├── skill/
│   ├── skill-passed.yaml
│   └── ...
└── examples/
    └── sample-rendered-outputs/
```

**Configuration (per project):**
```yaml
# devforgeai/feedback/config.yaml
feedback:
  templates:
    directory: "~/org-feedback-templates/command"
```

**Benefits:**
- Consistent feedback format across projects
- Centralized template maintenance
- Easy onboarding for new team members
- Aggregate feedback analysis across projects

---

## Troubleshooting

### Custom Template Not Found

**Symptom:** Template engine uses generic fallback instead of custom template

**Causes:**
1. Filename doesn't match pattern `{operation-type}-{status}.yaml`
2. Template directory path incorrect in configuration
3. YAML frontmatter has wrong `operation-type` or `success-status`

**Fix:**
- Verify filename: `ls devforgeai/feedback/custom-templates/`
- Check template YAML frontmatter matches filename
- Verify directory path in config.yaml

### Field Mapping Not Working

**Symptom:** Response not appearing in rendered output

**Causes:**
1. Question ID mismatch (template vs conversation)
2. Response value is `None`, `""`, or missing
3. Section header format incorrect (missing `##`)

**Fix:**
- Print conversation responses: `print(conversation_responses.keys())`
- Verify question ID matches exactly (case-sensitive)
- Check section header starts with `##`

### YAML Parse Error

**Symptom:** Template fails to load with YAML error

**Causes:**
1. Invalid YAML syntax (indentation, colons, quotes)
2. YAML frontmatter delimiter missing (`---`)
3. Special characters not escaped

**Fix:**
- Validate YAML: `python3 -c "import yaml; yaml.safe_load(open('template.yaml'))"`
- Check indentation (2 spaces per level, no tabs)
- Escape special YAML characters: `:`, `#`, `[`, `]`

### Rendered Output Missing Sections

**Symptom:** Expected section not in rendered template

**Causes:**
1. Field mapping references section not in template sections list
2. Auto-populated section disabled in configuration
3. Response empty or missing for that question ID

**Fix:**
- Verify section in template sections list
- Check configuration: `feedback.auto-populate`
- Confirm response exists: `print(conversation_responses)`

---

## Migration Guide

### Migrating from v1.0 to v2.0 (Future)

**When template engine is updated with breaking changes:**

1. **Backup current templates:**
```bash
cp -r .claude/skills/devforgeai-feedback/templates \
      .claude/skills/devforgeai-feedback/templates-v1.0-backup
```

2. **Review changelog:**
- Read CHANGELOG.md in template engine
- Identify breaking changes
- Understand migration requirements

3. **Update templates:**
- Apply required changes to YAML frontmatter
- Update field mapping syntax if changed
- Test rendering with sample data

4. **Deploy incrementally:**
- Update one template type at a time (command, then skill, then subagent)
- Test each template before proceeding
- Monitor rendered outputs for issues

5. **Update configuration:**
- Modify `devforgeai/feedback/config.yaml` if new options available
- Enable new features gradually

---

## FAQ

### Q: Can I disable auto-populated sections?

**A:** Yes, via configuration:
```yaml
feedback:
  auto-populate:
    context: false       # Disable context section
    sentiment: false     # Disable sentiment section
    insights: false      # Disable actionable insights
```

### Q: Can I change the output directory for rendered templates?

**A:** Yes, via configuration:
```yaml
feedback:
  output:
    base-directory: "/custom/feedback/location"
    use-subdirectories: false  # All in base directory, no operation-type folders
```

### Q: Can I use JSON instead of YAML for templates?

**A:** No, templates must be YAML format. YAML is more human-readable and easier to edit.

### Q: Can I have multiple templates for the same operation type and status?

**A:** Yes, use custom templates with different template IDs and select via configuration.

### Q: How do I share templates across projects?

**A:** Create shared template repository, configure `feedback.templates.directory` to point to shared location.

### Q: Can I programmatically generate templates?

**A:** Yes, templates are YAML files. Generate them programmatically and save to template directory.

---

## References

- **Template Format Specification:** `template-format-specification.md`
- **Field Mapping Guide:** `field-mapping-guide.md`
- **Template Examples:** `template-examples.md`
- **Template Engine Implementation:** `.claude/scripts/devforgeai_cli/feedback/template_engine.py`

---

**Version History:**
- **1.0 (2025-11-10):** Initial customization guide for STORY-010
