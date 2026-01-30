---
description: Update skill definitions, references, and documentation with interactive guidance
argument-hint: [SKILL-NAME] [--validate|--features|--docs|--all]
model: opus
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - WebFetch
  - WebSearch
  - AskUserQuestion
---

# /skill:update - Skill Update Wizard

Update skill specifications, reference files, and documentation with interactive guidance and validation.

---

## Quick Reference

```bash
# Interactive update wizard (recommended)
/skill:update claude-code-terminal-expert

# Update specific aspects
/skill:update my-skill --features    # Add new features
/skill:update my-skill --docs        # Update documentation only
/skill:update my-skill --validate    # Validate without changes

# Batch operations
/skill:update --all --validate       # Validate all skills
```

---

## Phase 0: Argument Parsing & User Questions

### Step 0.1: Parse Arguments

```
SKILL_NAME = null
UPDATE_MODE = "interactive"
VALIDATE_ONLY = false

FOR arg in $ARGUMENTS:
  IF arg == "--validate":
    VALIDATE_ONLY = true
  ELIF arg == "--features":
    UPDATE_MODE = "features"
  ELIF arg == "--docs":
    UPDATE_MODE = "docs"
  ELIF arg == "--all":
    UPDATE_MODE = "batch"
  ELIF NOT arg.startsWith("--"):
    SKILL_NAME = arg
```

### Step 0.2: Validate Skill Exists

```
IF UPDATE_MODE == "batch":
  Glob(pattern=".claude/skills/*/SKILL.md")
  SKILLS = all_matches
  Display: "Found {count} skills"
ELSE:
  IF SKILL_NAME empty:
    # List available skills for selection
    Glob(pattern=".claude/skills/*/SKILL.md")

    AskUserQuestion:
      question: "Which skill would you like to update?"
      header: "Skill"
      multiSelect: false
      options:
        - label: "[skill-1]"
          description: "First available skill"
        - label: "[skill-2]"
          description: "Second available skill"
        - label: "[skill-3]"
          description: "Third available skill"
        - label: "[skill-4]"
          description: "Fourth available skill"

    SKILL_NAME = user_selection

  # Validate skill path
  skill_path = ".claude/skills/${SKILL_NAME}/SKILL.md"
  Read(file_path=skill_path, limit=30)

  IF file not found:
    Display: "❌ Skill not found: ${SKILL_NAME}"
    Display: "Available skills:"
    Glob(pattern=".claude/skills/*/SKILL.md")
    HALT
```

### Step 0.3: Determine Update Intentions (Interactive Mode)

IF UPDATE_MODE == "interactive":

```
AskUserQuestion:
  question: "What type of update do you want to make?"
  header: "Update Type"
  multiSelect: true
  options:
    - label: "Add new features"
      description: "Document new capabilities, commands, or behaviors"
    - label: "Update existing docs"
      description: "Fix errors, improve clarity, update examples"
    - label: "Fix structural issues"
      description: "YAML frontmatter, missing sections, formatting"
    - label: "Sync with official docs"
      description: "Fetch latest from official documentation URLs"

UPDATE_INTENTIONS = user_selections
```

### Step 0.4: Gather Feature Details (If Adding Features)

IF "Add new features" in UPDATE_INTENTIONS:

```
AskUserQuestion:
  question: "What is the source of the new features?"
  header: "Source"
  multiSelect: false
  options:
    - label: "New software version"
      description: "Features from a new release (e.g., v2.1.0 → v2.1.23)"
    - label: "Official documentation"
      description: "Features found in official docs but not yet documented"
    - label: "User-discovered"
      description: "Features discovered through usage, not yet documented"
    - label: "Framework integration"
      description: "New integration points with DevForgeAI or other skills"

FEATURE_SOURCE = user_selection

IF FEATURE_SOURCE == "New software version":
  AskUserQuestion:
    question: "What version are you updating to?"
    header: "Version"
    multiSelect: false
    options:
      - label: "2.1.x (latest)"
        description: "Current stable release series"
      - label: "Specific version"
        description: "Enter a specific version number"

  TARGET_VERSION = user_selection
```

### Step 0.5: Confirm Update Scope

```
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "📋 Update Summary"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "Skill: ${SKILL_NAME}"
Display: "Mode: ${UPDATE_MODE}"
Display: "Validate Only: ${VALIDATE_ONLY}"
Display: "Update Types: ${UPDATE_INTENTIONS}"
IF TARGET_VERSION:
  Display: "Target Version: ${TARGET_VERSION}"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

AskUserQuestion:
  question: "Proceed with this update plan?"
  header: "Confirm"
  multiSelect: false
  options:
    - label: "Yes, proceed"
      description: "Start the update workflow"
    - label: "Modify scope"
      description: "Change update intentions"
    - label: "Cancel"
      description: "Exit without changes"

IF user_selection == "Cancel":
  Display: "Update cancelled."
  EXIT
ELIF user_selection == "Modify scope":
  GOTO Step 0.3
```

---

## Phase 1: Load Skill Context

### Step 1.1: Read Current Skill State

```
Read(file_path=".claude/skills/${SKILL_NAME}/SKILL.md")
Read(file_path=".claude/skills/${SKILL_NAME}/README.md")  # If exists

# List reference files
Glob(pattern=".claude/skills/${SKILL_NAME}/references/*.md")
REFERENCE_FILES = results

# List asset files
Glob(pattern=".claude/skills/${SKILL_NAME}/assets/*.md")
ASSET_FILES = results
```

### Step 1.2: Extract Current Metadata

```
FROM SKILL.md frontmatter, extract:
- current_version
- last_updated
- compatibility
- allowed_tools
- topics
```

### Step 1.3: Validate Current Structure

```
VALIDATION_RESULTS = []

# Check YAML frontmatter
IF frontmatter has syntax errors:
  VALIDATION_RESULTS.append({
    "severity": "HIGH",
    "issue": "YAML frontmatter syntax error",
    "location": "SKILL.md lines 1-20",
    "fix": "Correct YAML syntax"
  })

# Check required sections
required_sections = [
  "Execution Model",
  "When to Use",
  "Progressive Disclosure"
]

FOR section in required_sections:
  IF section NOT in SKILL.md:
    VALIDATION_RESULTS.append({
      "severity": "MEDIUM",
      "issue": f"Missing section: {section}",
      "location": "SKILL.md",
      "fix": f"Add {section} section"
    })

# Check Success Criteria (DevForgeAI pattern)
IF "Success Criteria" NOT in SKILL.md:
  VALIDATION_RESULTS.append({
    "severity": "MEDIUM",
    "issue": "Missing Success Criteria section",
    "location": "SKILL.md",
    "fix": "Add Success Criteria section per DevForgeAI pattern"
  })
```

---

## Phase 2: Research Updates (If Syncing with Official Docs)

### Step 2.1: Fetch Official Documentation

IF "Sync with official docs" in UPDATE_INTENTIONS:

```
# Extract official URLs from SKILL.md
Grep(pattern="https://", path=".claude/skills/${SKILL_NAME}/SKILL.md")
OFFICIAL_URLS = results

FOR url in OFFICIAL_URLS (limit 5):
  WebFetch(url=url, prompt="Extract all features, commands, and configuration options")
  FETCHED_DOCS.append(result)
```

### Step 2.2: Search for New Features

IF "Add new features" in UPDATE_INTENTIONS AND FEATURE_SOURCE == "New software version":

```
WebSearch(query="${SKILL_NAME} ${TARGET_VERSION} changelog new features")
WebSearch(query="${SKILL_NAME} ${TARGET_VERSION} release notes")

NEW_FEATURES = extract_features_from_search_results()
```

### Step 2.3: Compare and Identify Gaps

```
FOR feature in NEW_FEATURES:
  Grep(pattern=feature.name, path=".claude/skills/${SKILL_NAME}/")

  IF no matches:
    GAPS.append({
      "feature": feature.name,
      "description": feature.description,
      "source": feature.source_url,
      "target_file": determine_target_file(feature)
    })

Display: "Found ${GAPS.length} features not yet documented"
```

---

## Phase 3: Generate Update Plan

### Step 3.1: Compile Changes

```
CHANGES = {
  "structural_fixes": VALIDATION_RESULTS.filter(severity="HIGH"),
  "new_features": GAPS,
  "doc_updates": [...],
  "metadata_updates": {
    "version": increment_version(current_version),
    "last_updated": today(),
    "compatibility": TARGET_VERSION
  }
}
```

### Step 3.2: Present Update Plan

```
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "📝 Update Plan"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

IF CHANGES.structural_fixes:
  Display: ""
  Display: "🔧 Structural Fixes (HIGH priority):"
  FOR fix in CHANGES.structural_fixes:
    Display: "  • ${fix.issue}"
    Display: "    Location: ${fix.location}"

IF CHANGES.new_features:
  Display: ""
  Display: "✨ New Features to Document:"
  FOR feature in CHANGES.new_features:
    Display: "  • ${feature.feature}"
    Display: "    Target: ${feature.target_file}"

Display: ""
Display: "📊 Metadata Updates:"
Display: "  • Version: ${current_version} → ${CHANGES.metadata_updates.version}"
Display: "  • Last Updated: ${CHANGES.metadata_updates.last_updated}"

Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

### Step 3.3: Confirm or Modify

```
IF VALIDATE_ONLY:
  Display: "Validation complete. No changes made (--validate mode)."
  EXIT

AskUserQuestion:
  question: "How would you like to proceed?"
  header: "Apply"
  multiSelect: false
  options:
    - label: "Apply all changes"
      description: "Execute the complete update plan"
    - label: "Structural fixes only"
      description: "Fix issues but skip new features"
    - label: "Review individually"
      description: "Approve each change one by one"
    - label: "Cancel"
      description: "Exit without changes"

APPLY_MODE = user_selection
```

---

## Phase 4: Apply Updates

### Step 4.1: Apply Structural Fixes

IF "structural_fixes" in scope:

```
FOR fix in CHANGES.structural_fixes:
  Display: "Applying: ${fix.issue}"

  # Apply the fix using Edit tool
  Edit(file_path=fix.location, old_string=fix.old, new_string=fix.new)

  Display: "  ✓ Fixed"
```

### Step 4.2: Add New Feature Documentation

IF "new_features" in scope:

```
FOR feature in CHANGES.new_features:
  Display: "Adding: ${feature.feature}"

  # Generate documentation section
  section_content = generate_feature_section(feature)

  # Append to target file
  Read(file_path=feature.target_file)
  Edit(file_path=feature.target_file,
       old_string="[APPROPRIATE_ANCHOR]",
       new_string=section_content + "\n[APPROPRIATE_ANCHOR]")

  Display: "  ✓ Added to ${feature.target_file}"
```

### Step 4.3: Update Metadata

```
Read(file_path=".claude/skills/${SKILL_NAME}/SKILL.md", limit=30)

Edit(file_path=".claude/skills/${SKILL_NAME}/SKILL.md",
     old_string="version: \"${current_version}\"",
     new_string="version: \"${CHANGES.metadata_updates.version}\"")

Edit(file_path=".claude/skills/${SKILL_NAME}/SKILL.md",
     old_string="last-updated: \"${current_date}\"",
     new_string="last-updated: \"${CHANGES.metadata_updates.last_updated}\"")

Display: "✓ Metadata updated"
```

---

## Phase 5: Verification

### Step 5.1: Validate Changes

```
# Re-read and validate
Read(file_path=".claude/skills/${SKILL_NAME}/SKILL.md", limit=30)

# Check YAML is valid
IF frontmatter_valid:
  Display: "✓ YAML frontmatter valid"
ELSE:
  Display: "❌ YAML validation failed - manual fix required"
  HALT

# Check character budget
char_count = count_characters(SKILL.md)
IF char_count > 15500:
  Display: "⚠️ Character budget exceeded: ${char_count}/15000"
  Display: "   Consider moving content to references"
ELSE:
  Display: "✓ Character budget OK: ${char_count}/15000"
```

### Step 5.2: Generate Summary

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "✅ Update Complete"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "Skill: ${SKILL_NAME}"
Display: "Version: ${current_version} → ${new_version}"
Display: ""
Display: "Changes Applied:"
Display: "  • Structural fixes: ${structural_fixes_count}"
Display: "  • New features: ${new_features_count}"
Display: "  • Documentation updates: ${doc_updates_count}"
Display: ""
Display: "Files Modified:"
FOR file in modified_files:
  Display: "  • ${file}"
Display: ""
Display: "Next Steps:"
Display: "  1. Review changes in modified files"
Display: "  2. Test skill triggers work correctly"
Display: "  3. Commit changes: git add .claude/skills/${SKILL_NAME}/"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

---

## Error Handling

### Skill Not Found
```
Display: "❌ Skill not found: ${SKILL_NAME}"
Display: ""
Display: "Available skills:"
Glob and list skills
AskUserQuestion for retry or cancel
```

### WebFetch Failed
```
Display: "⚠️ Could not fetch: ${url}"
Display: "Continuing with local information only"
# Don't halt - continue with available data
```

### Edit Conflict
```
Display: "❌ Edit conflict in ${file}"
Display: "The expected content was not found."
Display: ""
Display: "Expected: ${old_string}"
Display: ""
AskUserQuestion:
  question: "How should I proceed?"
  header: "Conflict"
  options:
    - label: "Show file content"
    - label: "Skip this change"
    - label: "Cancel update"
```

---

## Success Criteria

- [ ] Skill name validated or selected interactively
- [ ] Update intentions gathered via questions
- [ ] Current skill state analyzed
- [ ] Validation issues identified
- [ ] Update plan presented for approval
- [ ] Changes applied (or validation-only completed)
- [ ] Post-update verification passed
- [ ] Summary with next steps displayed

---

## Integration

**Invoked by:**
- Developers: `/skill:update [skill-name]`
- Maintenance: `/skill:update --all --validate`

**Works with:**
- All skills in `.claude/skills/` directory
- DevForgeAI skill patterns and standards

**Token Budget:** ~4,000 tokens (command only)
**Character Budget:** ~12,000 characters (within 15K limit)
