# DevForgeAI UI Generator Guide

Complete guide to the `devforgeai-ui-generator` skill for interactive UI specification and code generation.

---

## Overview

The `devforgeai-ui-generator` skill generates front-end UI specifications and code through an interactive, constraint-aware workflow.

---

## When to Use

Use this skill when:
- A story requires UI components (forms, dashboards, dialogs, tables)
- Generating visual specifications from acceptance criteria
- Creating mockups-as-code for web (React, Blazor, ASP.NET), GUI (WPF, Tkinter), or terminal interfaces
- Translating requirements into tangible UI components

**Invocation:**
```
Skill(command="devforgeai-ui-generator")
Skill(command="devforgeai-ui-generator --story=STORY-042")
```

**Or via slash command:**
```
> /create-ui STORY-042
```

---

## Core Features

### Context Validation

**Requirements:**
- All 6 DevForgeAI context files must exist (tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns)
- Halts if context missing → directs user to run `devforgeai-architecture` first
- Validates technology choices against tech-stack.md

### Interactive Discovery

Uses AskUserQuestion to guide through:
- UI type selection (Web/GUI/Terminal)
- Technology stack selection (React, Blazor, WPF, Tkinter, etc.)
- Styling preferences (Tailwind, Bootstrap, plain CSS)
- Component structure definition

### Story Integration

- Reads story files to extract UI requirements from acceptance criteria
- Updates story with generated UI component references
- Creates UI-SPEC-SUMMARY.md with component inventory

### Code Generation

- Loads appropriate templates from `assets/` directory
- Applies best practices from `references/` directory
- Generates production-ready code following coding-standards.md
- Saves to `devforgeai/specs/ui/` (or location specified in source-tree.md)

---

## Supported Technologies

### Web

- **React:** JSX functional components with hooks
- **Blazor Server/WASM:** Razor components with @code blocks
- **ASP.NET Core MVC:** Razor views with @model
- **Plain HTML5:** Semantic markup

### Native GUI

- **C# WPF:** XAML with MVVM pattern
- **Python Tkinter:** Class-based components
- **.NET MAUI:** Cross-platform

### Terminal UI

- Formatted tables with box-drawing characters
- ANSI color-coded output
- Progress bars and status indicators

---

## 6-Phase Workflow

1. **Context Validation** - Verify 6 context files exist and are valid
2. **Story Analysis** - Extract UI requirements from story file (optional)
3. **Interactive Discovery** - Guide user through technology and styling decisions
4. **Template Loading** - Load appropriate templates and best practices
5. **Code Generation** - Generate UI components respecting all constraints
6. **Documentation** - Update story file and create spec summary

---

## Token Efficiency

**Estimated token usage per component:** ~35,000 tokens

**Efficiency achieved through:**
- Native tool usage (Read/Write/Edit/Glob/Grep) instead of Bash (40-73% savings)
- Progressive loading (only load relevant templates and references)
- Context validation happens once at start
- Focused generation per component

---

## Example Usage

```
User: "Generate a login form for STORY-042"

Workflow:
1. ✅ Validates context files
2. ✅ Reads STORY-042.story.md → finds "email and password fields"
3. ❓ Asks: "What type of UI?" → User selects "Web UI"
4. ❓ Asks: "Web technology?" → User selects "React"
5. ❓ Asks: "Styling?" → User selects "Tailwind CSS"
6. ❓ Asks: "Theme?" → User selects "Dark Mode"
7. ✅ Generates LoginForm.jsx with email/password inputs
8. ✅ Saves to devforgeai/specs/ui/LoginForm.jsx
9. ✅ Updates STORY-042.story.md with UI reference
10. ✅ Creates UI-SPEC-SUMMARY.md
```

---

## Conflict Resolution

When user's technology choice conflicts with tech-stack.md:

```
User selects: Vue.js
Context specifies: React

Action: Use AskUserQuestion
"You selected Vue.js, but tech-stack.md specifies React. Which should be used?"
Options:
  - Use React (Follow existing standard)
  - Use Vue.js (Update tech-stack.md and create ADR)
```

**Never assume** - Always use AskUserQuestion for technology conflicts.

---

## Integration with Other Skills

### devforgeai-architecture
- UI Generator requires context files from architecture skill
- If missing → HALT and direct user to run architecture first

### devforgeai-development
- Generated UI specs serve as input for TDD implementation
- Development skill reads specs to write tests and implementation

### devforgeai-qa
- QA validates generated UI matches acceptance criteria
- Checks for accessibility, best practices, anti-patterns

### devforgeai-orchestration
- Orchestration can automatically invoke UI Generator when story has UI requirements
- Workflow: detect UI story → invoke ui-generator → proceed to development

---

## Skill Structure

```
.claude/skills/devforgeai-ui-generator/
├── SKILL.md                          # Main skill definition
├── scripts/
│   ├── ensure_spec_dir.py            # Create output directory
│   └── validate_context.py           # Validate 6 context files
├── references/
│   ├── devforgeai-integration-guide.md  # Framework integration details
│   ├── web_best_practices.md           # Semantic HTML, accessibility, validation
│   ├── gui_best_practices.md           # Layout, naming, keyboard navigation
│   └── tui_best_practices.md           # Column alignment, colors, box drawing
└── assets/
    ├── web-template.html             # Plain HTML5 boilerplate
    ├── web-template.jsx              # React functional component
    ├── web-template.blazor.razor     # Blazor component with @code
    ├── web-template.aspnet.cshtml    # ASP.NET MVC view
    ├── gui-template.py               # Python Tkinter app
    ├── gui-template.wpf.xaml         # WPF Window with Grid
    └── tui-template.py               # Terminal formatting functions
```

---

## Quality Standards

Generated UI code must:
- ✅ Follow coding-standards.md conventions
- ✅ Use technologies from tech-stack.md only
- ✅ Place files according to source-tree.md
- ✅ Use dependencies from dependencies.md
- ✅ Avoid anti-patterns from anti-patterns.md
- ✅ Include accessibility attributes (ARIA, semantic HTML)
- ✅ Apply best practices from references/
- ✅ Match story acceptance criteria

---

## Reference Files

Load these as needed during UI generation:
- `.claude/skills/devforgeai-ui-generator/references/devforgeai-integration-guide.md` - Framework integration and context validation
- `.claude/skills/devforgeai-ui-generator/references/web_best_practices.md` - Semantic HTML, accessibility, responsive design
- `.claude/skills/devforgeai-ui-generator/references/gui_best_practices.md` - Layout organization, naming, keyboard navigation
- `.claude/skills/devforgeai-ui-generator/references/tui_best_practices.md` - Terminal formatting, box-drawing, color usage

---

## Workflow Position

**In Development Lifecycle:**
1. devforgeai-architecture (create context files)
   ↓
2. **devforgeai-ui-generator** (generate UI specs) [OPTIONAL]
   ↓
3. devforgeai-development (implement UI with tests)
   ↓
4. devforgeai-qa (validate UI implementation)

**When to invoke:**
- After architecture phase (requires context files)
- Before or during development phase
- Only if story has UI components

---

## Output Files

**UI Specifications:**
- Location: `devforgeai/specs/ui/` (or per source-tree.md)
- Format: Technology-specific (JSX, Razor, XAML, Python, etc.)
- Documentation: UI-SPEC-SUMMARY.md (component inventory)

**Story Updates:**
- Adds UI component references to story file
- Links to generated specifications
- Updates workflow history

---

## Best Practices

1. **Always validate context first** - UI Generator requires all 6 context files
2. **Let users choose technologies** - Use AskUserQuestion for all UI decisions
3. **Respect tech-stack.md** - Never generate code for unapproved technologies
4. **Follow accessibility standards** - All web UIs include ARIA attributes
5. **Generate production-ready code** - Not sketches or prototypes
6. **Update story files** - Always link generated UI to story for traceability
