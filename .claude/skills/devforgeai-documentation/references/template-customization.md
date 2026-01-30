# Template Customization Guide

Guide for customizing documentation templates and creating project-specific documentation formats.

---

## Purpose

Enable projects to customize documentation templates while maintaining framework consistency and quality standards.

---

## Custom Template Location

**Standard location:** `devforgeai/templates/documentation/`

**Structure:**
```
devforgeai/templates/documentation/
├── readme-template.md (overrides default)
├── developer-guide-template.md (overrides default)
├── custom-release-notes-template.md (new template)
└── variables.yaml (custom variable definitions)
```

---

## Template Override Precedence

**Template loading order:**

1. **Custom template** (highest priority)
   - Location: `devforgeai/templates/documentation/{template}.md`
   - User-defined, project-specific
   - Overrides default completely

2. **Default template**
   - Location: `.claude/skills/devforgeai-documentation/assets/templates/{template}.md`
   - Framework-provided standard
   - Used if no custom template

3. **Minimal fallback**
   - Generated on-the-fly if both missing
   - Basic structure only
   - Warning displayed to user

**Example:**
```
Skill checks:
1. devforgeai/templates/documentation/readme-template.md
   → Found! Use this

2. If not found:
   .claude/skills/devforgeai-documentation/assets/templates/readme-template.md
   → Use default

3. If not found:
   Generate minimal:
   # {{project_name}}
   {{project_description}}
```

---

## Variable System

### Standard Variables

**Project Metadata:**
- `{{project_name}}` - From package.json, git repo, or user input
- `{{project_description}}` - From story, package.json, or README
- `{{version}}` - From git tags, package.json, or version file
- `{{author}}` - From package.json or git config
- `{{license}}` - From LICENSE file or package.json
- `{{repository_url}}` - From git remote or package.json

**Timestamps:**
- `{{last_updated}}` - Current date (YYYY-MM-DD format)
- `{{creation_date}}` - Project creation date (from git)
- `{{release_date}}` - Latest release date (from git tags)

**Technical:**
- `{{tech_stack}}` - From tech-stack.md (comma-separated list)
- `{{architecture_pattern}}` - From code analysis or architecture-constraints.md
- `{{primary_language}}` - Detected from codebase (Python, JavaScript, C#, etc.)

**From Stories:**
- `{{feature_list}}` - Bullet points from acceptance criteria
- `{{api_endpoints}}` - API endpoints from technical specifications
- `{{usage_examples}}` - Code examples from AC Given/When/Then
- `{{troubleshooting_entries}}` - From edge cases section

**From Code Analysis:**
- `{{entry_points}}` - Main files (src/index.ts, src/main.py)
- `{{public_apis}}` - List of public functions/classes
- `{{dependencies}}` - External packages
- `{{layer_structure}}` - Directory organization

---

### Custom Variables

**Define in variables.yaml:**

```yaml
custom_variables:
  company_name: "Acme Corp"
  product_tagline: "The best task manager"
  support_email: "support@example.com"
  slack_channel: "#dev-support"
  deployment_url: "https://tasks.example.com"
```

**Use in templates:**
```markdown
# {{project_name}}

**By {{company_name}}**

{{product_tagline}}

## Support

Contact: {{support_email}}
Slack: {{slack_channel}}
```

**Skill reads variables.yaml** and adds to variable map before substitution.

---

## Conditional Sections

### If/Unless Blocks

**Show section only if variable has value:**
```markdown
{{#if api_endpoints}}
## API Reference

{{api_endpoints}}

For complete API documentation, see [API.md](docs/API.md).
{{/if}}
```

**Show warning if variable missing:**
```markdown
{{#unless installation_steps}}
> ⚠️ Installation instructions not yet documented.
> Please add installation steps manually.
{{/unless}}
```

### Switch Blocks

**Show different content based on value:**
```markdown
{{#switch architecture_pattern}}
{{#case "Clean Architecture"}}
The system follows Clean Architecture with 4 layers:
Domain, Application, Infrastructure, Presentation.
{{/case}}

{{#case "MVC"}}
The system follows MVC pattern:
Models, Views, Controllers.
{{/case}}

{{#default}}
The system uses a custom architecture pattern.
{{/default}}
{{/switch}}
```

---

## Template Structure Best Practices

### Sections to Include (README)

**Mandatory:**
- Title and description
- Installation instructions
- Quick start example
- Basic usage

**Recommended:**
- Features list
- Configuration guide
- API reference (summary)
- Contributing guidelines
- License

**Optional:**
- Architecture overview
- Advanced usage
- Troubleshooting
- FAQ

### Sections to Include (Developer Guide)

**Mandatory:**
- Architecture overview
- Development setup
- Coding standards
- Testing strategy

**Recommended:**
- Directory structure
- Build and deployment
- Debugging guide
- Best practices

---

## Customization Examples

### Example 1: Company Branding

**Custom template with branding:**
```markdown
<!-- devforgeai/templates/documentation/readme-template.md -->

<div align="center">
  <img src="{{company_logo_url}}" alt="{{company_name}}" width="200"/>

  # {{project_name}}

  **{{product_tagline}}**

  [![Build]({{ci_badge_url}})]({{ci_url}})
  [![License]({{license_badge_url}})](LICENSE)
</div>

---

{{project_description}}

...
```

### Example 2: API-First Documentation

**Custom API template with OpenAPI:**
```markdown
<!-- devforgeai/templates/documentation/api-docs-template.md -->

# {{project_name}} API

**Version:** {{api_version}}
**Base URL:** {{api_base_url}}

## OpenAPI Specification

Download: [openapi.yaml]({{openapi_spec_url}})

`` `yaml
{{openapi_spec_content}}
`` `

## Authentication

{{authentication_section}}

...
```

### Example 3: Minimal Docs

**Streamlined template for internal tools:**
```markdown
<!-- devforgeai/templates/documentation/readme-template.md -->

# {{project_name}}

{{project_description}}

## Installation

`` `bash
{{installation_command}}
`` `

## Usage

`` `bash
{{usage_command}}
`` `

**That's it!** {{project_name}} is designed to be simple.
```

---

## Project-Specific Customizations

### From coding-standards.md

**Extract documentation preferences:**
```
Read("devforgeai/specs/context/coding-standards.md")

Look for section: "Documentation Style" or "Documentation Standards"

Extract:
- Heading style preference (ATX # vs Setext underlines)
- Code block formatting
- Comment style (/** vs ///)
- Example format preferences

Apply to generated docs
```

**Example extraction:**
```markdown
## Documentation Standards (from coding-standards.md)

- Use ATX-style headers (# ## ###)
- Code blocks always have language tag
- API examples show both request and response
- Include table of contents for docs >500 lines
```

**Apply during generation:**
```
IF coding_standards["table_of_contents_threshold"]:
    IF generated_doc_length > threshold:
        Add table of contents
```

---

## Template Testing

### Validation Checklist

Before using custom template:

- [ ] All {{variables}} are recognized (no typos)
- [ ] Conditional blocks are properly closed (`{{/if}}`, `{{/switch}}`)
- [ ] Markdown syntax is valid (headings, links, code blocks)
- [ ] Mermaid diagrams (if embedded) render correctly
- [ ] No hardcoded project-specific values (use variables)
- [ ] Template works with minimal variable set
- [ ] Template works with complete variable set

### Test with Sample Data

**Create test variable map:**
```yaml
# test-variables.yaml
project_name: "TestProject"
project_description: "A test project for template validation"
version: "1.0.0"
tech_stack: "Python, Flask, PostgreSQL"
author: "Test User"
```

**Generate documentation:**
```bash
/document --template=custom-readme --test-mode
```

**Verify output:**
- All variables substituted
- No {{placeholders}} remaining
- Structure makes sense
- Content is coherent

---

## Framework Integration

### Respecting Context Files

**tech-stack.md:**
- Use exact technology names from tech-stack
- Include version numbers if specified
- Don't document unapproved technologies

**source-tree.md:**
- Place documentation files per source-tree rules
- Use documented directory structure in examples
- Explain module organization from source-tree

**coding-standards.md:**
- Follow documentation style from standards
- Use code formatting preferences
- Include standard examples

---

## Error Handling

### Missing Variable

**When variable has no value:**

**Option 1: Use default**
```
{{project_description OR "A software project"}}
```

**Option 2: Prompt user**
```
Display: "Variable {{project_description}} has no value"
AskUserQuestion: "Enter project description:"
Use response as value
```

**Option 3: Leave placeholder**
```
{{project_description OR "[Add project description]"}}
```

**Strategy:** Use Option 2 for critical variables (project_name), Option 1 for optional (tagline)

### Template Not Found

**When template file missing:**

```
IF custom_template not found AND default_template not found:
    Display: "Template {name} not found"
    Display: "Available templates:"
    FOR template in available:
        Display: "  - {template}"

    AskUserQuestion: "Select template or create new?"

    IF "Create new":
        Generate minimal template
        Save to devforgeai/templates/documentation/{name}.md
        Prompt user to customize
```

---

## Advanced Customization

### Multi-Language Projects

**Template with language-specific sections:**
```markdown
## Installation

=== "Python"
    `` `bash
    {{python_install_command}}
    `` `

=== "JavaScript"
    `` `bash
    {{javascript_install_command}}
    `` `

=== "C#"
    `` `bash
    {{csharp_install_command}}
    `` `
```

**Variable population:**
```
IF "Python" in tech_stack:
    python_install_command = "pip install -r requirements.txt"

IF "JavaScript" in tech_stack:
    javascript_install_command = "npm install"

IF "C#" in tech_stack:
    csharp_install_command = "dotnet restore"
```

### Versioned Documentation

**Template with version-specific content:**
```markdown
## What's New in {{version}}

{{#if version_major_changes}}
### Breaking Changes
{{version_major_changes}}
{{/if}}

### New Features
{{version_new_features}}

### Improvements
{{version_improvements}}

---

## Upgrading from {{previous_version}}

{{upgrade_guide}}
```

---

**Last Updated:** 2025-11-18
**Version:** 1.0.0
**Lines:** 290 (target met)
