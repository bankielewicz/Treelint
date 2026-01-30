# Phase 6: Documentation & Story Update

Update story file with UI references and create UI spec summary.

**Objective:** Document the generated UI and update story file if applicable.

---

## Steps

### Step 6.1: Generate UI Spec Summary

**Create a summary document listing:**
- Components generated
- Technology stack used
- File locations
- Dependencies required
- Integration instructions

**Summary structure:**
```markdown
# UI Specification Summary

**Generated:** ${date}
**Mode:** ${story|standalone}
**Story ID:** ${STORY_ID or 'N/A'}

## Components Generated

1. **${Component1}**
   - File: ${path1}
   - Framework: ${framework1}
   - Type: ${type1} (form, display, navigation, etc.)

2. **${Component2}**
   - File: ${path2}
   - Framework: ${framework2}
   - Type: ${type2}

## Technology Stack

- **Framework:** ${framework}
- **Styling:** ${styling_approach}
- **Theme:** ${theme}
- **State Management:** ${state_management}

## Dependencies

${List all npm packages, imports, or libraries required}

## Integration Instructions

${How to integrate these components into the application}

## Next Steps

1. Review generated components in devforgeai/specs/ui/
2. Implement components in source code (run /dev ${STORY_ID})
3. Write tests for UI components
4. Validate against acceptance criteria
```

---

### Step 6.2: Write Summary File

**Use Write tool:**
```
Write(
  file_path="devforgeai/specs/ui/UI-SPEC-SUMMARY.md",
  content="${ui_spec_summary}"
)
```

**Confirm creation:**
```
"✅ Created UI-SPEC-SUMMARY.md"
```

---

### Step 6.3: Update Story File (if story provided)

**Only execute if in story mode:**

**Use Edit tool to add reference to generated UI components:**
```
Edit(
  file_path="devforgeai/specs/Stories/${STORY_ID}.story.md",
  old_string="## Technical Specification",
  new_string="## Technical Specification

### UI Components
- Generated: ${component_list}
- Location: devforgeai/specs/ui/
- See: devforgeai/specs/ui/UI-SPEC-SUMMARY.md
"
)
```

**Handle edit errors:**
- If "## Technical Specification" not found → Search for alternative section
- If file locked → Report error, suggest manual update
- If story file missing → Skip story update (standalone mode)

**Confirm story updated:**
```
"✅ Updated ${STORY_ID}.story.md with UI component references"
```

---

## Output

**Phase 6 produces:**
- UI-SPEC-SUMMARY.md created
- Story file updated with UI references (if story mode)
- Documentation complete

**Ready for:**
- Phase 6 Step 3.5: Invoke ui-spec-formatter subagent
- Phase 7: Specification Validation
