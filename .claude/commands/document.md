---
description: Generate and maintain project documentation automatically
argument-hint: [STORY-ID | --type=TYPE | --mode=MODE]
model: opus
allowed-tools: Read, Glob, Skill, AskUserQuestion
---

# /document - Automated Documentation Generation

Generate comprehensive project documentation from stories (greenfield) or codebase analysis (brownfield).

---

## Quick Reference

```bash
# Generate docs for specific story
/document STORY-040

# Generate specific documentation type
/document --type=readme
/document --type=api
/document --type=architecture
/document --type=roadmap

# Brownfield analysis
/document --mode=brownfield --analyze

# Export formats
/document --export=html
/document --export=pdf

# Generate all documentation
/document --type=all

# List available templates
/document --list-templates
```

---

## Command Workflow

### Phase 0: Argument Validation and Parsing

**Parse arguments:**
```
STORY_ID=""
TYPE="readme"
MODE="greenfield"
EXPORT="markdown"
LIST_TEMPLATES=false

# Parse $1
IF $1 matches "STORY-[0-9]+":
    STORY_ID=$1
    MODE="greenfield"
ELIF $1 == "--type="*:
    TYPE=${1#--type=}
ELIF $1 == "--mode="*:
    MODE=${1#--mode=}
ELIF $1 == "--export="*:
    EXPORT=${1#--export=}
ELIF $1 == "--list-templates":
    LIST_TEMPLATES=true
ELSE:
    IF $1 not empty:
        Display: "Invalid argument: $1"
        Display: "Usage: /document [STORY-ID | --type=TYPE | --mode=MODE]"
        HALT
```

**Load story file** (if story ID provided):
```
IF STORY_ID not empty:
    @devforgeai/specs/Stories/$STORY_ID*.story.md

    IF file not found:
        Display: "Story not found: $STORY_ID"
        Display: "Run: Glob(pattern='devforgeai/specs/Stories/*.story.md') to list stories"
        HALT

    Display: "✓ Story loaded: $STORY_ID"
```

**Handle template listing**:
```
IF LIST_TEMPLATES:
    Display: ""
    Display: "Available Documentation Templates:"
    Display: "  1. readme - Project overview, installation, quick start"
    Display: "  2. developer-guide - Architecture, development workflow"
    Display: "  3. api - API reference, endpoints, schemas"
    Display: "  4. troubleshooting - Common issues, solutions"
    Display: "  5. contributing - Contribution guidelines, PR process"
    Display: "  6. changelog - Version history, release notes"
    Display: "  7. architecture - System design, diagrams, ADRs"
    Display: ""
    Display: "Usage: /document --type=<template-name>"
    EXIT
```

**Validation summary:**
```
Display: ""
Display: "Documentation Generation"
Display: "  Story: {STORY_ID or 'All completed stories'}"
Display: "  Type: {TYPE}"
Display: "  Mode: {MODE}"
Display: "  Export: {EXPORT}"
Display: ""
```

---

### Phase 1: Invoke Skill

**Set context markers for skill:**
```
Display: "**Story ID:** {STORY_ID}"
Display: "**Documentation Type:** {TYPE}"
Display: "**Mode:** {MODE}"
Display: "**Export Format:** {EXPORT}"
Display: ""
```

**Invoke devforgeai-documentation skill:**
```
Skill(command="devforgeai-documentation")
```

**Skill executes:**
- Phase 0: Mode detection and validation
- Phase 1: Discovery and analysis (greenfield or brownfield)
- Phase 2: Content generation (invoke subagents)
- Phase 3: Template application
- Phase 4: Integration and updates
- Phase 5: Validation and quality check
- Phase 6: Export and finalization
- Phase 7: Completion summary

**Skill returns structured result to command.**

---

### Phase 2: Display Results

**Skill has returned result object:**

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Documentation Generated Successfully"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

Display: "Files Created/Updated:"
FOR file in result.files:
    Display: "  ✓ {file.path}"
    Display: "    Type: {file.type}"
    Display: "    Words: {file.word_count}"
    Display: "    Coverage: {file.coverage}%"
    Display: ""

IF result.diagrams:
    Display: "Diagrams Generated:"
    FOR diagram in result.diagrams:
        Display: "  ✓ {diagram.path}"
        Display: "    Type: {diagram.type}"
        Display: "    Components: {diagram.component_count}"
        Display: ""

Display: "Overall Documentation Coverage: {result.overall_coverage}%"

IF result.overall_coverage >= 80:
    Display: "  ✅ Meets release quality gate (≥80%)"
ELSE:
    Display: "  ⚠️ Below release threshold (≥80%)"
    Display: "  Undocumented items: {result.undocumented_count}"

Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

---

### Phase 3: Next Steps

**Display recommendations:**
```
Display: ""
Display: "Next Steps:"

IF STORY_ID:
    Display: "  • Review generated documentation"
    Display: "  • Run: /qa {STORY_ID}"
    Display: "  • Customize documentation with project-specific details"
ELSE:
    Display: "  • Review generated documentation"
    Display: "  • Update documentation with project-specific content"
    Display: "  • Consider generating story-specific docs with /document STORY-ID"

IF result.overall_coverage < 80:
    Display: "  • Add documentation for {result.undocumented_count} undocumented items"
    Display: "  • Run: /document again after updates"

Display: ""
```

---

## Error Handling

**Invalid Story ID:**
```
Error: "Story not found: {story_id}"
Action: List available stories, display usage
```

**Context Files Missing:**
```
Error: "Context files required. Run /create-context first."
Action: HALT, direct to architecture setup
```

**No Completed Stories:**
```
Error: "No completed stories found (greenfield mode)"
Action: Suggest brownfield mode OR wait for story completion
```

**Export Dependencies Missing:**
```
Error: "PDF export requires pandoc or wkhtmltopdf"
Action: Display install command, fallback to Markdown
```

**Coverage Below Threshold:**
```
Warning: "Documentation coverage {X}% < 80%"
Action: List undocumented APIs, suggest fixes
```

---

## Success Criteria

- [ ] Documentation files created/updated
- [ ] Coverage ≥80% (if quality gate mode)
- [ ] All diagrams render correctly
- [ ] Story file updated (if story-based)
- [ ] Export formats created (if requested)
- [ ] Token usage <5K in main conversation
- [ ] Character count <12K

---

## Integration

**Invoked by:** Users, `/release` command (quality gate), `/orchestrate` (after QA)
**Invokes:** `devforgeai-documentation` skill
**Updates:** Documentation files, story files, CHANGELOG.md

---

## Performance

**Token Budget:**
- Command overhead: ~3K
- Skill execution: ~50K (isolated)
- Total main conversation: ~3K

**Execution Time:**
- Greenfield (single story): <2 minutes
- Brownfield (500 files): <10 minutes
- Architecture diagrams: <30 seconds per diagram
- Incremental updates: <1 minute

---

**Created:** 2025-11-18
**Pattern:** Lean Orchestration
**Character Count:** ~5,500 (37% of budget)
