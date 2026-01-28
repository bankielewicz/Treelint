---
description: Generate UI component specifications and code
argument-hint: [STORY-ID or component-description]
model: opus
allowed-tools: Read, Write, Edit, Glob, Grep, Skill, AskUserQuestion
---

# Create UI Command

Generates UI component specifications and implementation code with framework-specific patterns, styling, and documentation.

## Arguments

- `STORY-ID or component-description` (required): Either a story ID (e.g., "STORY-001") or standalone component description (e.g., "Login form with validation")

## Workflow

### Phase 0: Argument Validation

**Determine argument type:**
```
ARG = $1

IF ARG matches pattern "STORY-[0-9]+":
  MODE = "story"
  STORY_ID = ARG
ELSE IF ARG is empty:
  AskUserQuestion:
  Question: "No argument provided. What UI should I generate?"
  Header: "UI Generation"
  Options:
    - "List available stories with UI requirements"
    - "Standalone component (I'll describe it)"
    - "Show correct /create-ui syntax"
  multiSelect: false

  Handle based on user response
ELSE:
  # Treat as standalone component description
  MODE = "standalone"
  COMPONENT_DESCRIPTION = ARG
```

**If story mode, validate story:**
```
Glob(pattern="devforgeai/specs/Stories/${STORY_ID}*.story.md")

IF no matches found:
  AskUserQuestion:
  Question: "Story ${STORY_ID} not found. What should I do?"
  Header: "Story not found"
  Options:
    - "List all available stories"
    - "Use standalone mode (describe component)"
    - "Cancel command"
  multiSelect: false
```

**Validation summary:**
```
IF MODE == "story":
  ✓ Mode: Story-based UI generation
  ✓ Story ID: ${STORY_ID}
  ✓ Story file: ${STORY_FILE}
ELSE:
  ✓ Mode: Standalone UI generation
  ✓ Component: ${COMPONENT_DESCRIPTION}

✓ Proceeding with UI generation...
```

---

### Phase 1: Validate Context Files

**Check for required context files:**

```bash
Glob(pattern="devforgeai/specs/context/*.md")
```

**Verify 6 context files exist:**
- tech-stack.md (frontend framework, styling approach)
- source-tree.md (component location rules)
- dependencies.md (approved UI libraries)
- coding-standards.md (component patterns)
- architecture-constraints.md (component boundaries)
- anti-patterns.md (UI anti-patterns)

**If context files missing:**
```
❌ Context files required before UI generation

Missing: [list missing files]

Action Required:
  Run /create-context to generate architectural context files
  Context defines:
    - Frontend framework (React, Vue, Angular, Svelte)
    - Styling approach (CSS Modules, Tailwind, styled-components)
    - Component structure (file organization, naming)
    - State management (Redux, Zustand, Pinia, Context API)

Cannot proceed without context constraints.
```

**Exit if context missing.**

**Read tech-stack.md to determine frontend stack:**

```bash
Read(file_path="devforgeai/specs/context/tech-stack.md")
```

**Extract frontend technologies:**
- UI Framework (React, Vue, Angular, Svelte, etc.)
- Styling solution (Tailwind, CSS Modules, styled-components, Sass)
- State management (Redux, Zustand, MobX, Pinia, Context API)
- UI component library (Material-UI, Chakra UI, Ant Design, shadcn/ui)
- Form library (React Hook Form, Formik, VeeValidate)
- Validation library (Zod, Yup, Joi)

### Phase 2: Invoke UI Generator Skill

**Execute UI generator skill:**

**Prepare context for skill:**

**If story mode:**
```
Story content already loaded via @file reference (if argument was STORY-ID)
Story ID: ${STORY_ID}
```

**If standalone mode:**
```
Component description: ${COMPONENT_DESCRIPTION}
No story file associated (standalone UI generation)
```

**Invoke skill (without parameters):**
```bash
Skill(command="devforgeai-ui-generator")
```

**After skill invocation:**
- Skill's SKILL.md content expands inline in conversation
- **YOU execute the skill's workflow phases** (not waiting for external result)
- Follow the skill's instructions phase by phase
- Produce output as skill instructs

**The skill instructs you to perform 7-phase workflow:**

1. **Context Validation** - Verify all 6 context files exist, extract frontend stack
2. **Story Analysis** (if story mode) - Extract UI requirements, acceptance criteria, user flows
3. **Interactive Discovery** - Ask 5-15 detailed UI questions:
   - Component type (form, data display, navigation, modal, dashboard, etc.)
   - Technology details (framework, styling, state management)
   - Styling preferences (theme, colors, spacing, typography)
   - Interactivity (animations, transitions, loading states)
   - Accessibility requirements (WCAG 2.1 level, ARIA patterns)
   - Responsive behavior (breakpoints, mobile-first, adaptive layouts)
   - Data handling (API integration, local state, form validation)
   - User feedback (error messages, success states, loading indicators)
4. **Template Loading** - Load framework-specific templates (React, Vue, Angular, Svelte)
5. **Code Generation** - Generate component files with:
   - Component implementation (framework-specific syntax)
   - Styling files (CSS/SCSS/styled-components/Tailwind)
   - Type definitions (TypeScript interfaces/types)
   - Test files (Jest, Vitest, Testing Library, Cypress)
   - Storybook stories (if configured)
   - Documentation (props, usage examples, accessibility notes)
6. **Documentation** - Create UI specification document, invoke ui-spec-formatter subagent
7. **Specification Validation** - Validate completeness, detect placeholders, ensure framework compliance

**Expected outputs:**
- UI specification: `devforgeai/specs/ui/${STORY_ID}-ui-spec.md` (or standalone ID)
- Component files: Generated in project structure per source-tree.md
- Summary document: `devforgeai/specs/ui/UI-SPEC-SUMMARY.md`

**Skill will ask 5-15 questions via AskUserQuestion:**
- "What type of UI component is this?" (Form, Data Display, Navigation, Modal, Card, etc.)
- "What styling approach should be used?" (From tech-stack.md or custom)
- "What theme should be applied?" (Light, Dark, System preference, Custom brand colors)
- "What accessibility level is required?" (WCAG 2.1 A, AA, AAA)
- "What responsive breakpoints are needed?" (Mobile, Tablet, Desktop, Custom)
- "What loading/error states are needed?" (Spinners, Skeletons, Error boundaries, Toast notifications)
- "What animations/transitions are needed?" (Fade, Slide, Scale, Custom)
- "What form validation is required?" (Real-time, On blur, On submit, Custom rules)
- Framework-specific questions based on tech-stack.md

### Phase 3: Verify Output Files

**Check UI specification created:**

```bash
Glob(pattern="devforgeai/specs/ui/*-ui-spec.md")
```

**If story mode, verify story updated:**

```bash
Read(file_path="devforgeai/specs/Stories/${STORY_ID}.story.md")
```

**Check for UI specification reference:**
- Should contain: "UI Specification: `devforgeai/specs/ui/${STORY_ID}-ui-spec.md`"

**If reference missing:**
- Use Edit tool to add reference to story file
- Add to "Related Documentation" section

**Verify component files generated:**

```bash
Grep(pattern="Component Files Generated:", path="devforgeai/specs/ui/", output_mode="content")
```

**Extract list of generated files from UI spec.**

**Verify summary document exists:**

```bash
Read(file_path="devforgeai/specs/ui/UI-SPEC-SUMMARY.md")
```

**If summary missing:**
- Report warning (non-critical)
- Skill should have created it

### Phase 4: Component Structure Validation

**Read generated UI specification:**

```bash
Read(file_path="devforgeai/specs/ui/${STORY_ID}-ui-spec.md")
```

**Validate specification completeness:**
- ✅ Component hierarchy defined (parent/child relationships)
- ✅ Props/API documented (types, defaults, required)
- ✅ State management approach specified
- ✅ Styling approach documented
- ✅ Accessibility considerations included
- ✅ Responsive behavior defined
- ✅ Test strategy outlined
- ✅ Usage examples provided

**Check for placeholder content:**

```bash
Grep(pattern="TODO|TBD|\\[FILL IN\\]|\\[TO BE DETERMINED\\]", path="devforgeai/specs/ui/${STORY_ID}-ui-spec.md", output_mode="content")
```

**If placeholders found:**
- Use AskUserQuestion to resolve each placeholder
- Update specification with final values

**Validate against context constraints:**

1. **Read architecture-constraints.md:**
   ```bash
   Read(file_path="devforgeai/specs/context/architecture-constraints.md")
   ```

2. **Check component location rules:**
   - Verify generated files follow source-tree.md structure
   - Check no cross-layer dependencies
   - Validate component naming conventions

3. **Read anti-patterns.md:**
   ```bash
   Read(file_path="devforgeai/specs/context/anti-patterns.md")
   ```

4. **Verify no UI anti-patterns:**
   - No prop drilling (use context/state management)
   - No inline styles (unless Tailwind/CSS-in-JS)
   - No direct DOM manipulation (use framework patterns)
   - No hardcoded text (use i18n/localization)
   - No accessibility violations (missing ARIA, poor contrast)

**If violations found:**
- Report violations to user
- Ask if should regenerate or accept

### Phase 5: Success Report

**Display detailed summary:**

```
✅ UI Component Specification Generated

Component Details:
  📋 ${MODE === "story" ? "Story: ${STORY_ID}" : "Standalone Component"}
  🎨 Component Type: [type from discovery]
  ⚛️ Framework: [framework from tech-stack.md]
  💅 Styling: [styling approach]
  ♿ Accessibility: [WCAG level]
  📱 Responsive: [breakpoints]

Generated Files:
  📁 devforgeai/specs/ui/${STORY_ID}-ui-spec.md
  [List component files generated by skill]
  📊 devforgeai/specs/ui/UI-SPEC-SUMMARY.md

Component Structure:
  🔹 [Component 1 name] - [Component 1 type]
  🔹 [Component 2 name] - [Component 2 type]
  [... list all components ...]

Styling Approach:
  🎨 [Styling solution]
  🌈 Theme: [theme name]
  📐 Spacing: [spacing scale]
  🔤 Typography: [font families]

Accessibility:
  ♿ WCAG Level: [level]
  🗣️ ARIA: [patterns used]
  ⌨️ Keyboard Navigation: [supported]
  🎯 Focus Management: [approach]

State Management:
  📦 Approach: [state management solution]
  🔄 Data Flow: [data flow pattern]
  📡 API Integration: [API approach]

Test Coverage:
  ✅ Unit Tests: [test file locations]
  ✅ Integration Tests: [test file locations]
  ✅ E2E Tests: [test file locations]
  ✅ Accessibility Tests: [approach]

${MODE === "story" ?
  `Story Updated:
  📁 devforgeai/specs/Stories/${STORY_ID}.story.md
  ✅ UI specification reference added`
  : ""}

Next Steps:
  1. Review UI specification: devforgeai/specs/ui/${STORY_ID}-ui-spec.md
  2. Customize if needed (adjust colors, spacing, components)
  3. Implement with TDD: /dev ${MODE === "story" ? STORY_ID : ""}
  4. Run component tests: [test command from tech-stack]
  5. Preview in Storybook: [storybook command if configured]
```

### Phase N: Feedback Hook Integration

**Purpose:** Collect feedback on UI design decisions and component complexity (follows STORY-023 /dev pattern)
**Pattern Source:** devforgeai-qa/references/feedback-hooks-workflow.md (5-step non-blocking pattern)
**Requirement:** devforgeai CLI must be installed (from tech-stack.md approved tools)

**After UI specification generation completes (Phase 5), invoke feedback hooks:**

```bash
# Step N.1: Check hook eligibility
devforgeai-validate check-hooks --operation=create-ui --status=completed
HOOK_EXIT=$?

# Step N.2: Only invoke if eligible (exit code 0)
IF HOOK_EXIT == 0:
  devforgeai-validate invoke-hooks --operation=create-ui --story=${STORY_ID} \
    --context="ui_type=${UI_TYPE}&technology=${FRAMEWORK}&components=${COMPONENT_COUNT}&styling=${STYLING_APPROACH}" || {
    Log: "⚠️ Feedback system unavailable, continuing..."
  }
ELSE:
  Log: "Feedback hooks not configured for create-ui operation"
ENDIF

# Step N.3: Handle failures gracefully (non-blocking)
# Note: Errors logged but not thrown - /create-ui always succeeds
```

**Context passed to feedback:**
- `operation`: "create-ui"
- `story`: STORY ID (if story mode)
- `ui_type`: "web" | "GUI" | "terminal"
- `technology`: Framework name (React, Vue, WPF, Tkinter, etc.)
- `components`: Number of components generated
- `styling`: Styling approach (Tailwind, CSS Modules, etc.)

**Behavior:**
- ✅ Check hook eligibility (non-blocking)
- ✅ Invoke if eligible (non-blocking)
- ✅ Display message: "Feedback system ready" or "Feedback system unavailable"
- ✅ Continue regardless of hook outcome
- ✅ /create-ui always succeeds (spec generated, hooks optional)

**When hooks unavailable:**
```
⚠️ Feedback system unavailable

Possible causes:
  - devforgeai CLI not installed
  - hooks.yaml not configured
  - Hook trigger disabled for create-ui

UI specifications have been generated successfully.
Continue with: /dev [STORY-ID] or edit components as needed.
```

## Error Handling

### Error: Story Not Found

**Condition:** Story ID provided but file does not exist

**Action:**
```
❌ Story Not Found: ${STORY_ID}

Searched: devforgeai/specs/Stories/${STORY_ID}.story.md

Action Required:
  Create story first with: /create-story [story-description]
  Or use standalone mode: /create-ui "component description"

Available stories:
  [List existing stories from devforgeai/specs/Stories/]
```

### Error: Context Files Missing

**Condition:** One or more of 6 context files do not exist

**Action:**
```
❌ Context Files Required

Missing:
  [List missing context files]

Context files define architectural constraints:
  - tech-stack.md: Frontend framework, styling, state management
  - source-tree.md: Component location rules
  - dependencies.md: Approved UI libraries
  - coding-standards.md: Component patterns
  - architecture-constraints.md: Layer boundaries
  - anti-patterns.md: UI anti-patterns to avoid

Action Required:
  Run /create-context to generate context files
  Cannot proceed without architectural constraints
```

### Error: Frontend Stack Not Defined

**Condition:** tech-stack.md exists but no frontend framework specified

**Action:**
```
❌ Frontend Stack Not Defined

tech-stack.md does not specify frontend framework.

Action Required:
  Update tech-stack.md to include:
    - UI Framework (React, Vue, Angular, Svelte)
    - Styling solution (Tailwind, CSS Modules, styled-components)
    - State management (Redux, Zustand, Context API, Pinia)

Or re-run: /create-context (with frontend option)
```

### Error: UI Generator Skill Failed

**Condition:** Skill invocation returns error

**Recovery:**
1. Display error message from skill
2. Check common causes:
   - Invalid story format
   - Missing context files
   - Unsupported framework
   - Template files missing
3. Suggest fixes:
   - Verify `.claude/skills/devforgeai-ui-generator/SKILL.md` exists
   - Check skill syntax
   - Verify story file format
   - Check tech-stack.md frontend section

**Report to user:**
```
❌ UI Generator Skill Failed: [error message]

Possible causes:
  - [Cause 1]
  - [Cause 2]

Recovery steps:
  1. [Step 1]
  2. [Step 2]

Debug information:
  - Mode: ${MODE}
  - Story ID: ${STORY_ID}
  - Frontend Framework: [framework from tech-stack.md]

Try again with: /create-ui ${ARGUMENTS}
```

### Error: Specification Validation Failed

**Condition:** Generated UI spec incomplete or has placeholders

**Action:**
1. List specific validation failures:
   - Missing sections: [list]
   - Placeholder content: [list with line numbers]
   - Constraint violations: [list]
2. For each issue:
   - If missing sections: Ask user to provide details
   - If placeholders: Use AskUserQuestion to resolve
   - If violations: Regenerate or request override

**Report format:**
```
⚠️ Specification Validation Issues

Missing Sections:
  - Accessibility considerations
  - Responsive behavior

Placeholder Content:
  - Line 42: TODO: Define color palette
  - Line 87: TBD: Choose animation library

Constraint Violations:
  - Component location violates source-tree.md
  - Dependency not in dependencies.md: [package-name]

Action Required:
  [Specific questions to resolve issues]
```

## Success Criteria

- [x] UI specification generated in `devforgeai/specs/ui/`
- [x] Component files generated per source-tree.md structure
- [x] No TODO/TBD/placeholder content
- [x] All context constraints validated
- [x] Story updated with UI reference (if story mode)
- [x] Summary document updated
- [x] Accessibility requirements documented
- [x] Responsive behavior defined
- [x] Test files generated
- [x] Token usage < 40K

## Token Efficiency

**Target:** < 40K tokens total

**Optimization strategies:**
1. Use Skill tool (UI generator self-contained)
2. Read context files once, cache in memory
3. Parallel validation reads when possible
4. Use Glob for file discovery (not Bash)
5. Use Grep for placeholder detection (not cat | grep)
6. Only read story file once

**Estimated token breakdown:**
- Phase 1 (Parse & Load): ~3K tokens
- Phase 2 (Context Validation): ~5K tokens
- Phase 3 (UI Generator Skill): ~20K tokens (skill's budget)
- Phase 4 (Verify Output): ~3K tokens
- Phase 5 (Validation): ~6K tokens
- Phase 6 (Report): ~2K tokens
- **Total: ~39K tokens** (within 40K budget)

## Integration Points

**Invokes:**
- `devforgeai-ui-generator` skill (Phase 3)

**Prerequisites:**
- All 6 context files must exist (tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns)
- Frontend stack defined in tech-stack.md
- Optional: Story file (for story mode)

**Enables:**
- `/dev` command (implement UI components)
- `/validate` command (QA validation of UI)

**Related Commands:**
- `/create-story` - Create story before UI generation
- `/create-context` - Generate context files (required)
- `/dev` - Implement UI components with TDD
- `/validate` - Run QA validation

## Notes

**Framework philosophy:**
- UI generation respects architectural constraints
- No library substitution (locked in tech-stack.md)
- Component structure follows source-tree.md
- Styling approach enforced from context
- Accessibility is mandatory (WCAG 2.1 minimum)

**When to use:**
- Creating new UI components
- Implementing story with UI requirements
- Standardizing component patterns
- Generating UI documentation
- Bootstrapping component structure

**When NOT to use:**
- Minor style adjustments (edit component directly)
- Backend-only stories (no UI)
- Refactoring existing components (use /dev)
- Design system updates (edit design-system.md)

**UI component types supported:**
- Forms (login, registration, checkout, data entry)
- Data Display (tables, lists, cards, charts)
- Navigation (menus, breadcrumbs, tabs, sidebars)
- Feedback (modals, toasts, alerts, notifications)
- Layout (grids, flex containers, responsive wrappers)
- Interactive (buttons, dropdowns, accordions, carousels)

**Framework-specific features:**
- **React**: Hooks, Context API, React Router, Error Boundaries
- **Vue**: Composition API, Vue Router, Pinia, Teleport
- **Angular**: Services, RxJS, Router Guards, Change Detection
- **Svelte**: Stores, SvelteKit routing, Transitions

**Styling solutions supported:**
- Tailwind CSS (utility-first)
- CSS Modules (scoped styles)
- styled-components (CSS-in-JS)
- Emotion (CSS-in-JS)
- Sass/SCSS (preprocessor)
- Plain CSS (vanilla)
- UI libraries (Material-UI, Chakra UI, Ant Design, shadcn/ui)

**State management patterns:**
- Redux (global predictable state)
- Zustand (lightweight global state)
- MobX (reactive state)
- Pinia (Vue state management)
- Context API (React built-in)
- Services (Angular)
- Stores (Svelte)

**Accessibility features:**
- WCAG 2.1 compliance (A, AA, AAA)
- ARIA attributes (roles, labels, descriptions)
- Keyboard navigation (focus management, shortcuts)
- Screen reader support (semantic HTML, ARIA)
- Color contrast validation (4.5:1 minimum for AA)
- Focus indicators (visible focus states)

**Responsive design:**
- Mobile-first approach (default)
- Breakpoints (mobile, tablet, desktop, custom)
- Adaptive layouts (different layouts per breakpoint)
- Fluid typography (clamp, viewport units)
- Flexible grids (CSS Grid, Flexbox)
- Touch-friendly targets (44x44px minimum)

**Test generation:**
- Unit tests (component logic, utilities)
- Integration tests (component interaction)
- Accessibility tests (axe-core, jest-axe)
- Visual regression tests (Chromatic, Percy)
- E2E tests (Cypress, Playwright)

**Documentation generated:**
- Component API (props, events, slots)
- Usage examples (code snippets)
- Accessibility notes (ARIA, keyboard)
- Styling guide (classes, theming)
- Storybook stories (if configured)
