# Greenfield Documentation Workflow

Workflow for generating documentation from completed stories (greenfield mode).

---

## Purpose

Guide the devforgeai-documentation skill through story-based documentation generation, ensuring all implemented features are properly documented.

---

## When to Use

**Greenfield mode applies when:**
- Generating docs for specific story (e.g., `/document STORY-040`)
- Project has completed stories with technical specifications
- Documentation should reflect actual implemented features
- Source of truth is story files, not codebase scanning

---

## Workflow Phases

### Phase 1: Story Discovery and Selection

**Step 1.1: Determine story scope**

**If story ID provided** (e.g., `/document STORY-040`):
```
stories_to_document = [STORY-040]
```

**If no story ID** (e.g., `/document --type=readme`):
```
Glob(pattern="devforgeai/specs/Stories/*.story.md")

Filter stories by status:
- "Dev Complete" ✅
- "QA Approved" ✅
- "Released" ✅

Exclude:
- "Backlog"
- "In Development"
- "Ready for Dev"
- "QA Failed"

stories_to_document = filtered_list
```

**Step 1.2: Load story files**

```
FOR each story_id in stories_to_document:
    Read(file_path="devforgeai/specs/Stories/{story_id}*.story.md")

    Extract from YAML frontmatter:
    - id
    - title
    - epic
    - status
    - points
    - tags

    Extract from markdown sections:
    - User story (As a/I want/So that)
    - Acceptance criteria (Given/When/Then)
    - Technical specification (API, data models, rules)
    - Non-functional requirements
    - Edge cases
```

---

### Phase 2: Content Extraction

**Step 2.1: Extract features from acceptance criteria**

```
FOR each story:
    FOR each AC in story.acceptance_criteria:
        Parse Given/When/Then:
        - Given: Preconditions
        - When: User action
        - Then: Expected outcome

        Transform to feature description:
        Feature: {AC.when} → {AC.then}

Example:
AC: "When I run /document STORY-040, Then system generates README.md"
Feature: "Automated README generation from story specifications"
```

**Step 2.2: Extract API endpoints from technical spec**

```
Read technical specification section

Grep(pattern="### API Endpoints|## API Contract")

FOR each endpoint found:
    Extract:
    - HTTP method (GET, POST, PUT, DELETE)
    - Path (/api/tasks, /api/users/:id)
    - Parameters (query, path, body)
    - Response schema
    - Status codes

Store in api_endpoints list
```

**Step 2.3: Extract configuration from NFRs**

```
Read non-functional requirements section

Extract:
- Performance requirements → Add to README performance section
- Security requirements → Add to security documentation
- Scalability requirements → Add to architecture docs
```

**Step 2.4: Extract troubleshooting from edge cases**

```
Read edge cases section

FOR each edge case:
    Transform to troubleshooting entry:

    Edge Case: "No implemented code found"
    Troubleshooting: "Error: No implementation found
                      Solution: Verify story status is Dev Complete"

    Edge Case: "Conflicting existing documentation"
    Troubleshooting: "Warning: Existing README.md found
                      Solution: Choose merge or replace option"
```

---

### Phase 3: Template Selection and Loading

**Step 3.1: Determine required templates**

**Based on documentation type:**
```
IF type == "readme":
    templates = ["readme-template.md"]

ELIF type == "api":
    templates = ["api-docs-template.md"]

ELIF type == "architecture":
    templates = ["architecture-template.md"]

ELIF type == "all":
    templates = [
        "readme-template.md",
        "developer-guide-template.md",
        "api-docs-template.md",
        "troubleshooting-template.md"
    ]
```

**Step 3.2: Load templates**

```
FOR each template_name in templates:
    # Check for custom template
    custom_path = "devforgeai/templates/documentation/{template_name}"

    IF custom_path exists:
        content = Read(custom_path)
        Display: "Using custom template: {template_name}"
    ELSE:
        default_path = "assets/templates/{template_name}"
        content = Read(default_path)

    Store: template_content[template_name] = content
```

---

### Phase 4: Variable Population

**Step 4.1: Gather project metadata**

```
# From package.json (if Node.js)
IF package.json exists:
    Read("package.json")
    Extract: name, version, description, author, license

# From git
Bash(command="git config --get remote.origin.url")
repository_url = parse_output

Bash(command="git describe --tags --abbrev=0")
version = parse_output OR "1.0.0"

# From context files
Read("devforgeai/specs/context/tech-stack.md")
Extract technology list → tech_stack variable
```

**Step 4.2: Build variable map**

```
variables = {
    # Project basics
    "project_name": from package.json OR repo name,
    "project_description": from story OR package.json,
    "version": from git tags OR package.json,
    "last_updated": current_date,

    # From stories
    "feature_list": bullets from all AC,
    "usage_examples": code from AC Given/When/Then,
    "api_endpoints": formatted from tech specs,

    # From context files
    "tech_stack": from tech-stack.md,
    "coding_standards": from coding-standards.md,
    "architecture_overview": from architecture-constraints.md,

    # From templates or defaults
    "license": from LICENSE file OR "MIT",
    "support_information": default template
}
```

**Step 4.3: Substitute variables in templates**

```
FOR each template in template_content:
    output = template

    FOR each variable in variables:
        # Replace {{variable}} with value
        pattern = "{{" + variable + "}}"
        output = output.replace(pattern, variables[variable])

    # Handle conditional sections
    Remove sections where {{#if variable}} but variable empty

    generated_docs[template_name] = output
```

---

### Phase 5: Incremental Updates

**For existing documentation files:**

**Step 5.1: Detect existing files**

```
documentation_dir = from source-tree.md OR "docs/"

Glob(pattern="{documentation_dir}/*.md")
existing_files = results
```

**Step 5.2: Read existing content**

```
FOR each existing_file in existing_files:
    content = Read(existing_file)

    # Check for user-authored sections
    Check for markers:
    - <!-- USER CONTENT START -->
    - <!-- USER CONTENT END -->

    Extract user sections
    Store for preservation
```

**Step 5.3: Merge strategies**

**Strategy 1: Preserve user content**
```
IF user markers found:
    Keep: Content between <!-- USER CONTENT START/END -->
    Update: Everything else

merged_content = replace_non_user_sections(existing, new)
```

**Strategy 2: Side-by-side (if conflicts)**
```
IF substantial differences AND no markers:
    AskUserQuestion: "Merge or replace?"
    Options:
    - Merge (preserve existing + add new)
    - Replace (use new, backup old)
    - Manual (show diff, I'll merge)
```

**Step 5.4: Create backup**

```
IF updating existing file:
    timestamp = current_timestamp
    backup_path = "{file}.backup-{timestamp}"

    Bash(command="cp {file} {backup_path}")

    Display: "Backup created: {backup_path}"
```

**Step 5.5: Add changelog entry**

```
IF CHANGELOG.md exists:
    entry = "- {date}: Updated {section} for STORY-{id}"

    Edit(
        file_path="CHANGELOG.md",
        old_string="## [Unreleased]",
        new_string="## [Unreleased]\n\n### Documentation\n{entry}"
    )
```

---

### Phase 6: Story File Updates

**Add documentation references to story:**

```
Edit story file to add section (or update existing):

## Generated Documentation

- **README.md**: {path} (generated {timestamp})
- **Developer Guide**: {path} (generated {timestamp})
- **API Documentation**: {path} (generated {timestamp})
- **Coverage**: {coverage}%
- **Last Generated**: {timestamp}

Documentation validates:
✅ All acceptance criteria covered in docs
✅ API endpoints documented
✅ Configuration explained
✅ Troubleshooting guide includes edge cases
```

---

## Version Control Integration

**Git workflow for documentation updates:**

**Step 1: Stage documentation files**
```
Bash(command="git add README.md docs/*.md CHANGELOG.md")
```

**Step 2: Create documentation commit**
```
git commit -m "docs(STORY-{id}): Update documentation for {feature}

- Generated README.md with {word_count} words
- Updated API documentation ({endpoint_count} endpoints)
- Added troubleshooting for {edge_case_count} edge cases
- Coverage: {coverage}%

Generated by: /document STORY-{id}"
```

**Step 3: Update story workflow history**
```
Edit story file:

## Story Workflow History

### {timestamp} - Documentation Generated
- README.md: {coverage}% coverage
- API Docs: {endpoint_count} endpoints documented
- Files: {file_count} documentation files created/updated
- Command: /document STORY-{id}
```

---

## Success Criteria

**Documentation generation succeeds when:**

- [ ] All required templates loaded
- [ ] All variables substituted (no {{placeholders}} remaining)
- [ ] Generated files written to correct locations (per source-tree.md)
- [ ] Existing files backed up (if updating)
- [ ] Story file updated with documentation references
- [ ] Coverage ≥80% (if quality gate mode)
- [ ] All Mermaid diagrams render
- [ ] Changelog updated (if exists)
- [ ] Git commit created (if requested)

---

## Example: Full Greenfield Flow

**Input:** `/document STORY-042`

**Flow:**
1. Load STORY-042.story.md
2. Extract:
   - User story: "As a user, I want to see task list..."
   - AC1: "Given tasks exist, When I view list, Then see all tasks"
   - Tech spec: "GET /api/tasks endpoint returns Task[]"
3. Load readme-template.md
4. Populate variables:
   - {{project_name}} = "TaskManager"
   - {{feature_list}} = "• View task list\n• Filter tasks\n• Sort tasks"
   - {{api_endpoints}} = "GET /api/tasks - List all tasks"
5. Generate README.md
6. Write to project root
7. Update STORY-042 with documentation reference
8. Display: "✅ README.md generated (1,240 words, 85% coverage)"

---

**Last Updated:** 2025-11-18
**Version:** 1.0.0
**Lines:** 380 (target met)
