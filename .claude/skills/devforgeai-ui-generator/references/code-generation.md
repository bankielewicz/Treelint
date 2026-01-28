# Phase 5: Code Generation

Generate production-ready UI component code from templates and specifications.

**Objective:** Generate UI component code respecting all constraints and best practices.

---

## Steps

### Step 5.1: Ensure Output Directory

Run script to create output directory:
```
Bash(command="python .claude/skills/devforgeai-ui-generator/scripts/ensure_spec_dir.py")
```

---

### Step 5.2: Determine Output Location

**Parse source-tree.md for correct UI component location:**
```
Read(file_path="devforgeai/specs/context/source-tree.md")

Search for UI component directory specification
```

**Determine path:**
- Default: `devforgeai/specs/ui/` (if not specified in source-tree.md)
- Follow any project-specific structure defined in source-tree.md
- Respect directory hierarchy (e.g., `src/components/`, `Views/`, etc.)

---

### Step 5.3: Generate Component Code

**Start with loaded template:**
- Template loaded in Phase 4
- Contains boilerplate structure for selected technology

**Apply user's styling choices:**
- Add styling framework imports (Tailwind, Bootstrap, etc.)
- Apply theme colors and typography
- Include responsive classes/breakpoints

**Implement component structure from Phase 3:**
- Build component hierarchy (parent/child)
- Add props/parameters interface
- Implement state management
- Add event handlers

**Follow best practices from Phase 4:**
- Best practices loaded based on UI type (web/gui/tui)
- Apply accessibility guidelines
- Follow platform conventions

**Respect coding-standards.md conventions:**
- Indentation (tabs vs spaces)
- Naming patterns (camelCase, PascalCase, kebab-case)
- File organization
- Comment style

**Add accessibility attributes:**
- ARIA roles (role="button", role="navigation")
- ARIA labels (aria-label, aria-labelledby)
- Alt text for images
- Keyboard navigation support
- Focus management

**Include comments:**
- Explain key sections
- Document complex logic
- Note integration points
- Reference story AC (if applicable)

---

### Step 5.4: Apply Anti-Pattern Prevention

**Check anti-patterns.md for forbidden patterns:**
```
Read(file_path="devforgeai/specs/context/anti-patterns.md")

Scan for UI-specific anti-patterns
```

**Common UI anti-patterns to avoid:**
- ❌ Hardcoded values (use configuration/props)
- ❌ Inline styles (use CSS classes)
- ❌ Prop drilling (use context/state management)
- ❌ God components (>500 lines)
- ❌ Direct DOM manipulation (use framework APIs)
- ❌ Missing key props in lists
- ❌ Uncontrolled inputs without validation

**Ensure proper patterns:**
- ✅ Separation of concerns (presentation vs logic)
- ✅ Dependency injection (props, context)
- ✅ Reusable components
- ✅ Proper state management
- ✅ Error boundaries (React) or error handling (other)

---

### Step 5.5: Confirm Filename

```
AskUserQuestion(
  questions: [{
    question: "I have generated the code for ${COMPONENT_NAME}. What filename should I use?",
    header: "Filename",
    multiSelect: false,
    options: [
      { label: "Use default", description: "Save as: devforgeai/specs/ui/${COMPONENT_NAME}.${ext}" },
      { label: "Custom path", description: "Specify a different filename/location" }
    ]
  }]
)
```

**If custom path requested:**
```
AskUserQuestion:
  Question: "Enter the full file path for ${COMPONENT_NAME}:"
  # User provides via "Other" option text input

Parse user input and validate:
- Check path is within project
- Check directory exists or can be created
- Check filename has appropriate extension
```

---

### Step 5.6: Write File

**Use Write tool:**
```
Write(file_path="${confirmed_path}", content="${generated_code}")
```

**Handle write errors:**
- Permission denied → Report error, suggest alternative location
- Path invalid → Validate path format, ask for correction
- File exists → Ask: "Overwrite existing file?"

---

### Step 5.7: Confirm Completion

**Inform user:**
```
"✅ Generated ${COMPONENT_NAME} and saved to ${FILE_PATH}"
```

**Record generated file:**
```
generated_files = [
  {
    component: "${COMPONENT_NAME}",
    path: "${FILE_PATH}",
    framework: "${FRAMEWORK}",
    type: "${COMPONENT_TYPE}" (form, display, navigation, etc.)
  }
]
```

---

## Output

**Phase 5 produces:**
- UI component code written to file
- Generated files list (for Phase 6 documentation)
- Component metadata (for formatter in Phase 6)

**Ready for:**
- Phase 6: Documentation & Story Update
- Phase 7: Specification Validation
