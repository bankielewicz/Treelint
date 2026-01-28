# /document Command - Detailed Guide

Complete reference for the `/document` slash command and devforgeai-documentation skill.

---

## Purpose

Generate and maintain project documentation automatically from stories (greenfield) or codebase analysis (brownfield).

---

## Command Syntax

```bash
# Generate docs for specific story
/document STORY-040

# Generate specific documentation type
/document --type=readme
/document --type=api
/document --type=architecture
/document --type=roadmap
/document --type=all

# Brownfield analysis
/document --mode=brownfield --analyze

# Export formats
/document --export=html
/document --export=pdf

# List available templates
/document --list-templates
```

---

## Documentation Modes

### Greenfield Mode (Story-Based)

**When to use:**
- Generating docs from completed stories
- Project has story files with technical specifications
- Documentation should reflect implemented features

**Process:**
1. Reads story files (Dev Complete, QA Approved, or Released status)
2. Extracts: User story, acceptance criteria, technical specs, APIs
3. Generates documentation from specifications
4. Uses templates to format content
5. Writes to appropriate locations (per source-tree.md)

**Token cost:** ~15K (in isolated skill context)
**Time:** <2 minutes for single story

### Brownfield Mode (Codebase Analysis)

**When to use:**
- Existing project with code but limited/missing documentation
- Need to discover architecture and document existing features
- Gap analysis for documentation coverage

**Process:**
1. Invokes code-analyzer subagent
2. Scans codebase (all source files)
3. Discovers: Architecture pattern, layers, APIs, dependencies
4. Finds existing documentation files
5. Identifies gaps (missing, outdated, incomplete)
6. Generates coverage report with recommendations

**Token cost:** ~50K (in isolated skill context)
**Time:** <10 minutes for 500-file project

---

## Available Templates

**Location:** `.claude/skills/devforgeai-documentation/assets/templates/`

1. **readme-template.md** - Project overview, installation, quick start
2. **developer-guide-template.md** - Architecture, development workflow, conventions
3. **api-docs-template.md** - API reference, endpoints, schemas
4. **architecture-template.md** - System design, diagrams, ADRs
5. **troubleshooting-template.md** - Common issues, solutions
6. **contributing-template.md** - Contribution guidelines, PR process
7. **changelog-template.md** - Version history, release notes
8. **roadmap-template.md** - Project timeline, milestones, planned features

**Customization:**
- Place custom templates in `devforgeai/templates/documentation/`
- Override default templates by matching filename
- Add custom variables via `variables.yaml`

---

## Template Variables

Common variables populated by skill:
- `{{project_name}}` - From package.json, git repo, or context
- `{{project_description}}` - From stories or README
- `{{version}}` - From git tags or package.json
- `{{tech_stack}}` - From tech-stack.md
- `{{architecture_pattern}}` - From code analysis
- `{{api_endpoints}}` - From technical specifications
- `{{feature_list}}` - From acceptance criteria
- `{{last_updated}}` - Current timestamp

---

## Mermaid Diagram Generation

**Supported diagram types:**

1. **Flowcharts** - Application flow, decision trees
2. **Sequence Diagrams** - API calls, component interactions
3. **Architecture Diagrams** - Component relationships, layer structure
4. **ER Diagrams** - Database schema
5. **State Diagrams** - Workflow states
6. **Git Graphs** - Branch strategy

**Auto-generated from:**
- Code structure (classes, functions, dependencies)
- Story workflows (acceptance criteria flows)
- Architecture patterns (from code-analyzer)

**Validation:**
- Syntax checking (auto-fix common errors)
- Architecture constraint validation
- Rendering verification

---

## Quality Gate Integration

**Documentation Coverage Threshold: ≥80%**

**Calculated as:**
```
coverage = (documented_public_apis / total_public_apis) * 100
```

**Enforced by /release command:**
- Checks documentation coverage before deployment
- Blocks release if coverage <80%
- Lists undocumented APIs for remediation

**To fix low coverage:**
```bash
/document --type=api  # Generate API documentation
/qa STORY-XXX        # Re-validate coverage
/release STORY-XXX   # Retry release
```

---

## Incremental Updates

**For existing documentation:**

**Preservation strategy:**
- Detects user-authored sections (via markers or git blame)
- Creates backup before updating
- Merges new content with existing
- Adds changelog entry
- Maintains documentation consistency

**User content markers:**
```markdown
<!-- USER CONTENT START -->
Custom content here will be preserved
<!-- USER CONTENT END -->
```

---

## Export Formats

### HTML Export

**Requirements:** pandoc (optional)

```bash
/document STORY-040 --export=html
```

**Output:**
- Converts Markdown to styled HTML
- Includes table of contents
- Preserves Mermaid diagrams
- Applies CSS styling (if available)

### PDF Export

**Requirements:** pandoc + xelatex OR wkhtmltopdf (optional)

```bash
/document STORY-040 --export=pdf
```

**Fallback:** If dependencies missing, returns Markdown with install instructions

---

## Integration with DevForgeAI Workflow

**Updated SDLC:**

```
Dev Complete → QA Validation → Documentation → Release
```

**When to run:**
- After QA approval (before release)
- After major feature completion
- During sprint retrospective (update roadmap)
- Before external deployment (ensure docs current)

---

## Example Workflows

### Example 1: Document Single Story

```bash
# After story completes QA
/document STORY-042

# Output:
# ✅ README.md updated (added task filtering feature)
# ✅ API.md updated (added GET /api/tasks?filter=... endpoint)
# ✅ CHANGELOG.md updated (added v1.2.0 entry)
# Documentation coverage: 85% ✅
```

### Example 2: Brownfield Project Analysis

```bash
# Analyze existing codebase
/document --mode=brownfield --analyze

# Output:
# Analyzed: 342 source files
# Architecture: Clean Architecture (detected)
# Existing docs: 3 files found
# Gaps: README.md missing, API docs outdated (120 days)
# Undocumented APIs: 15 (coverage: 68%)
# Recommendations:
#   - Create README.md (CRITICAL)
#   - Update API.md (HIGH) - 15 undocumented endpoints
#   - Add architecture diagram (MEDIUM)
```

### Example 3: Generate All Documentation

```bash
# Generate complete documentation set
/document --type=all

# Output:
# ✅ README.md (1,240 words)
# ✅ docs/DEVELOPER.md (2,180 words)
# ✅ docs/API.md (1,560 words)
# ✅ docs/ARCHITECTURE.md (890 words) + 3 Mermaid diagrams
# ✅ docs/TROUBLESHOOTING.md (640 words)
# ✅ CONTRIBUTING.md (520 words)
# ✅ CHANGELOG.md (updated)
# Documentation coverage: 92% ✅
```

---

## Troubleshooting

### Issue: "Context files required"
**Solution:** Run `/create-context [project-name]` first

### Issue: "No completed stories found"
**Solution:** Complete and QA approve stories first, OR use brownfield mode

### Issue: "PDF export failed"
**Solution:** Install dependencies: `sudo apt install pandoc texlive-xelatex`

### Issue: "Coverage below 80%"
**Solution:** Run `/document --type=api` to document missing APIs

---

## Reference Documentation

**For comprehensive guides, see:**
- `.claude/skills/devforgeai-documentation/references/documentation-standards.md` - Style guide
- `.claude/skills/devforgeai-documentation/references/greenfield-workflow.md` - Story-based docs
- `.claude/skills/devforgeai-documentation/references/brownfield-analysis.md` - Code analysis
- `.claude/skills/devforgeai-documentation/references/diagram-generation-guide.md` - Mermaid diagrams
- `.claude/skills/devforgeai-documentation/references/template-customization.md` - Custom templates

---

**Created:** 2025-11-18 (STORY-040)
**Version:** 1.0.0
