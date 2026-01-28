# DevForgeAI Integration Guide for UI Generator

This guide explains how the `devforgeai-ui-generator` skill integrates with the DevForgeAI spec-driven development framework.

## Overview

The UI Generator skill is designed to work seamlessly within the DevForgeAI workflow, respecting architectural constraints and integrating with other framework skills.

---

## Context File Requirements

The skill **requires** all 6 DevForgeAI context files to exist before proceeding:

### 1. tech-stack.md

**Purpose:** Defines approved technologies, frameworks, and libraries.

**UI Generator Usage:**
- Check which UI frameworks/libraries are approved (React, Blazor, WPF, etc.)
- Verify styling frameworks (Tailwind, Bootstrap, plain CSS)
- Ensure proposed technology matches approved stack

**Example Check:**
```markdown
## Frontend Technologies

### Web Framework
- **React 18.2** - Component-based UI library
- **Blazor Server** - C# server-side rendering

### Styling
- **Tailwind CSS 3.3** - Utility-first CSS framework

### State Management
- **Zustand** - Lightweight state management (NOT Redux)
```

**Action if Mismatch:**
- User selects React → ✅ Approved, proceed
- User selects Vue → ❌ Not approved, use AskUserQuestion to resolve conflict

---

### 2. source-tree.md

**Purpose:** Defines project directory structure and file organization rules.

**UI Generator Usage:**
- Determine correct output location for generated UI components
- Follow naming conventions for UI files
- Respect module organization patterns

**Example Check:**
```markdown
## Frontend Structure

src/
├── components/         # Reusable UI components
│   ├── forms/         # Form components (LoginForm, etc.)
│   ├── layout/        # Layout components (Header, Footer)
│   └── shared/        # Shared utilities
├── pages/             # Page-level components
└── styles/            # Global styles and themes
```

**Action:**
- Generated UI components go to `src/components/forms/` (not `devforgeai/specs/ui/`)
- If source-tree.md doesn't specify UI location → default to `devforgeai/specs/ui/`

---

### 3. dependencies.md

**Purpose:** Lists approved packages and their versions.

**UI Generator Usage:**
- Verify UI framework version matches approved version
- Check styling library versions
- Ensure no unapproved dependencies are introduced

**Example Check:**
```markdown
## Frontend Dependencies

### Production
- react: ^18.2.0
- react-dom: ^18.2.0
- tailwindcss: ^3.3.0

### Development
- @types/react: ^18.2.0
- vite: ^4.4.0
```

**Action:**
- Generate package.json references with exact versions from dependencies.md
- Include installation instructions using approved versions

---

### 4. coding-standards.md

**Purpose:** Defines code style, naming conventions, and formatting rules.

**UI Generator Usage:**
- Apply correct indentation (spaces vs tabs, 2 vs 4 spaces)
- Follow naming conventions (PascalCase for components, camelCase for functions)
- Structure components according to standards (functional vs class components)
- Add appropriate comments and documentation

**Example Check:**
```markdown
## Frontend Standards

### Component Structure
- Use functional components with hooks
- PropTypes or TypeScript interfaces required
- Destructure props in function signature

### Naming
- Components: PascalCase (e.g., LoginForm.jsx)
- Functions: camelCase (e.g., handleSubmit)
- Constants: UPPER_SNAKE_CASE (e.g., API_BASE_URL)

### File Organization
- One component per file
- Co-locate tests (LoginForm.jsx, LoginForm.test.jsx)
```

**Action:**
- Generate code following exact standards
- Apply correct indentation and naming
- Include PropTypes/TypeScript interfaces as required

---

### 5. architecture-constraints.md

**Purpose:** Defines architectural boundaries and layer separation rules.

**UI Generator Usage:**
- Ensure UI components don't violate layer boundaries
- Follow separation of concerns (presentation vs logic)
- Respect data flow patterns (props down, events up)

**Example Check:**
```markdown
## Frontend Architecture

### Layer Rules
1. **Presentation Layer** (components/) - UI only, no business logic
2. **Application Layer** (hooks/, services/) - Business logic, API calls
3. **Domain Layer** (models/, types/) - Data structures, validation

### Constraints
- Components MUST NOT make direct API calls (use hooks/services)
- State management MUST use approved library (Zustand)
- No direct DOM manipulation (use React refs sparingly)
```

**Action:**
- Generate components that delegate logic to hooks/services
- Don't include API calls directly in components
- Follow approved state management patterns

---

### 6. anti-patterns.md

**Purpose:** Lists forbidden coding patterns and practices.

**UI Generator Usage:**
- Avoid generating code with known anti-patterns
- Check for hardcoded values, inline styles, direct DOM manipulation
- Ensure accessibility best practices are followed

**Example Check:**
```markdown
## Frontend Anti-Patterns

### ❌ FORBIDDEN

1. **Hardcoded Configuration**
   - Don't: `const API_URL = "https://api.example.com"`
   - Do: `const API_URL = import.meta.env.VITE_API_URL`

2. **Inline Styles**
   - Don't: `<div style={{color: 'red'}}>Error</div>`
   - Do: `<div className="text-red-500">Error</div>`

3. **Missing Accessibility**
   - Don't: `<div onClick={handleClick}>Click me</div>`
   - Do: `<button onClick={handleClick} aria-label="Submit form">Click me</button>`

4. **Direct API Calls in Components**
   - Don't: `fetch('/api/users').then(...)`
   - Do: `const { users } = useUsers()` (custom hook)
```

**Action:**
- Generate code without these anti-patterns
- Use configuration/environment variables
- Include accessibility attributes (ARIA labels, semantic HTML)
- Delegate API calls to hooks/services

---

## Workflow Integration

### Integration with devforgeai-architecture

**Dependency:** UI Generator requires context files created by architecture skill.

**Flow:**
```
devforgeai-architecture (creates context files)
    ↓
devforgeai-ui-generator (validates context, generates UI)
```

**Example:**
```
User: "Generate a login form"

Skill:
1. Check for context files
2. If missing → HALT
3. Message: "Context files missing. Please run devforgeai-architecture first."

User: [runs devforgeai-architecture]

Skill:
1. Context files exist ✓
2. Proceed with UI generation
```

---

### Integration with devforgeai-orchestration

**Invocation:** Orchestration can automatically invoke UI Generator when story contains UI requirements.

**Flow:**
```
devforgeai-orchestration (detects UI requirements in story)
    ↓
devforgeai-ui-generator (generates UI components)
    ↓
devforgeai-development (implements UI with TDD)
```

**Example:**
```
Story STORY-042:
  Title: Implement user login
  Acceptance Criteria:
    - Given a login form with email and password fields
    - When user enters valid credentials
    - Then user is authenticated

Orchestration:
1. Parses acceptance criteria
2. Detects "login form" → UI component needed
3. Invokes devforgeai-ui-generator
4. Passes story ID to UI Generator

UI Generator:
1. Reads STORY-042.story.md
2. Extracts "email and password fields"
3. Generates LoginForm component
4. Updates story with UI spec reference

Development:
1. Reads UI spec
2. Writes tests for LoginForm
3. Implements component to pass tests
```

---

### Integration with devforgeai-development

**Purpose:** UI specs serve as input for TDD implementation.

**Flow:**
```
devforgeai-ui-generator (generates UI spec)
    ↓
devforgeai-development (implements UI with tests)
    ↓
devforgeai-qa (validates implementation)
```

**Example:**
```
UI Generator Output:
  File: devforgeai/specs/ui/LoginForm.jsx
  Spec: Email input, password input, submit button

Development Skill (TDD):
1. Red Phase: Write failing tests
   - Test: LoginForm renders email input
   - Test: LoginForm renders password input
   - Test: LoginForm calls onSubmit with credentials

2. Green Phase: Implement component
   - Create LoginForm.jsx
   - Add input fields
   - Wire up submit handler

3. Refactor Phase: Improve code
   - Extract validation logic
   - Add error handling
   - Improve accessibility
```

---

### Integration with devforgeai-qa

**Purpose:** QA validates generated UI meets acceptance criteria.

**Flow:**
```
devforgeai-development (implements UI)
    ↓
devforgeai-qa (validates implementation)
    ↓
[PASS] → devforgeai-release
[FAIL] → Back to development
```

**QA Checks:**
1. **Spec Compliance:** Does implementation match UI spec?
2. **Acceptance Criteria:** Do tests cover all Given/When/Then scenarios?
3. **Anti-Patterns:** Any forbidden patterns detected?
4. **Accessibility:** ARIA labels, semantic HTML present?
5. **Coverage:** Test coverage meets thresholds?

---

## Technology Conflict Resolution

When user's technology choice conflicts with `tech-stack.md`:

### Scenario 1: User Selects Unapproved Technology

```
User: "Generate a Vue.js login form"

Context: tech-stack.md specifies React

Action:
1. Detect conflict (Vue not in tech-stack.md)
2. Use AskUserQuestion:

Question: "You selected Vue.js, but tech-stack.md specifies React. Which should be used?"
Options:
  - "Use React" (Follow existing standard)
  - "Use Vue.js" (Update tech-stack.md and create ADR)
```

### Scenario 2: User Confirms New Technology

```
User selects: "Use Vue.js"

Action:
1. Inform user to create ADR:
   "To use Vue.js, an Architecture Decision Record (ADR) must be created."

2. Guide ADR creation:
   - Create devforgeai/specs/adrs/ADR-NNN-vue-adoption.md
   - Document rationale for Vue.js over React
   - Update tech-stack.md to include Vue.js

3. After ADR created:
   - Proceed with Vue.js UI generation
   - Use Vue templates instead of React templates
```

### Scenario 3: User Follows Existing Standard

```
User selects: "Use React"

Action:
1. Acknowledge:
   "Using React as specified in tech-stack.md."

2. Proceed:
   - Load React templates
   - Generate React component
   - Follow React best practices
```

---

## Output Location Determination

The UI Generator determines output location in this priority order:

### Priority 1: source-tree.md Specification

If `source-tree.md` defines UI component location:
```markdown
src/
├── components/
│   ├── forms/         # Form components go here
```

→ Output to: `src/components/forms/LoginForm.jsx`

### Priority 2: Project Convention

If project has existing UI components:
```
Use Glob to find existing components:
  Glob(pattern="**/components/**/*.jsx")

If found in src/components/:
  → Follow same pattern
  → Output to: src/components/LoginForm.jsx
```

### Priority 3: Default Location

If no specification or convention found:
```
→ Output to: devforgeai/specs/ui/LoginForm.jsx
```

This ensures generated specs can be reviewed before integration into source tree.

---

## Story File Updates

When a story ID is provided, UI Generator updates the story file with references to generated UI.

### Before Update

```markdown
## Technical Specification

### API Endpoints
- POST /api/auth/login

### Data Models
- User: { email, password }
```

### After Update

```markdown
## Technical Specification

### UI Components
- Generated: LoginForm
- Location: devforgeai/specs/ui/LoginForm.jsx
- See: devforgeai/specs/ui/UI-SPEC-SUMMARY.md

### API Endpoints
- POST /api/auth/login

### Data Models
- User: { email, password }
```

### Update Implementation

```python
# Use Edit tool (NOT Bash sed)
Edit(
  file_path="devforgeai/specs/Stories/STORY-042.story.md",
  old_string="## Technical Specification",
  new_string="## Technical Specification\n\n### UI Components\n- Generated: LoginForm\n- Location: devforgeai/specs/ui/LoginForm.jsx\n- See: devforgeai/specs/ui/UI-SPEC-SUMMARY.md\n"
)
```

---

## Token Efficiency Best Practices

### Use Native Tools Only

**✅ CORRECT:**
```python
# Read context files
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/source-tree.md")

# Find existing components
Glob(pattern="src/**/*.jsx")

# Search for patterns
Grep(pattern="export default", type="js")

# Write generated code
Write(file_path="devforgeai/specs/ui/LoginForm.jsx", content="...")

# Update story
Edit(file_path="devforgeai/specs/Stories/STORY-042.story.md", ...)
```

**❌ WRONG:**
```python
# Never use Bash for file operations
Bash(command="cat devforgeai/specs/context/tech-stack.md")      # NO
Bash(command="find src/ -name '*.jsx'")                    # NO
Bash(command="grep 'export default' src/**/*.jsx")         # NO
Bash(command="echo 'content' > devforgeai/specs/ui/LoginForm.jsx")  # NO
```

**Savings:** 40-73% fewer tokens per operation.

### Progressive Loading

**Don't load all references upfront:**
```python
# ❌ WRONG: Load all references immediately
Read(file_path=".claude/skills/devforgeai-ui-generator/references/web_best_practices.md")
Read(file_path=".claude/skills/devforgeai-ui-generator/references/gui_best_practices.md")
Read(file_path=".claude/skills/devforgeai-ui-generator/references/tui_best_practices.md")
```

**✅ CORRECT: Load only what's needed:**
```python
# User selected Web UI
Read(file_path=".claude/skills/devforgeai-ui-generator/references/web_best_practices.md")
# (Don't load GUI or TUI references)
```

---

## Error Handling

### Context Files Missing

**Detection:**
```python
Glob(pattern="devforgeai/specs/context/*.md")
# Returns: 0 files found
```

**Response:**
```
❌ Context files are missing.

Resolution:
  Please invoke the 'devforgeai-architecture' skill to create the required context files.

Command:
  Skill(command="devforgeai-architecture")

After context files are created, re-run this skill.
```

**HALT:** Do not proceed with UI generation.

### Technology Conflict

**Detection:**
```python
user_choice = "Vue.js"
tech_stack_content = Read(file_path="devforgeai/specs/context/tech-stack.md")
# Vue.js not found in tech-stack.md
```

**Response:**
```
⚠️ Technology conflict detected.

Requested: Vue.js
Approved: React (in tech-stack.md)

Use AskUserQuestion to resolve this conflict.
```

### Story File Not Found

**Detection:**
```python
Read(file_path="devforgeai/specs/Stories/STORY-999.story.md")
# Error: FileNotFoundError
```

**Response:**
```
⚠️ Story file not found: STORY-999.story.md

Proceeding without story context.
UI generation will use interactive discovery only.
```

**Continue:** Skip Phase 2 (Story Analysis), proceed to Phase 3.

### Write Permission Denied

**Detection:**
```python
Write(file_path="devforgeai/specs/ui/LoginForm.jsx", content="...")
# Error: PermissionError
```

**Response:**
```
❌ Permission denied: Cannot write to devforgeai/specs/ui/

Resolution:
1. Check directory permissions
2. Ensure devforgeai/specs/ui/ is writable

Alternative: Specify a different output location using AskUserQuestion.
```

---

## Quality Assurance Checklist

Before completing UI generation, verify:

- [ ] All 6 context files validated
- [ ] Technology choice matches tech-stack.md (or conflict resolved)
- [ ] Output location follows source-tree.md structure
- [ ] Generated code follows coding-standards.md conventions
- [ ] No anti-patterns from anti-patterns.md present
- [ ] Accessibility attributes included (ARIA labels, semantic HTML)
- [ ] Best practices applied (web/gui/tui specific)
- [ ] Story file updated (if story provided)
- [ ] UI-SPEC-SUMMARY.md created
- [ ] User informed of completion with file paths

---

## Summary

The `devforgeai-ui-generator` skill is deeply integrated with the DevForgeAI framework:

1. **Context-First:** Always validates 6 context files before proceeding
2. **Constraint-Aware:** Respects tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns
3. **Conflict Resolution:** Uses AskUserQuestion to resolve technology mismatches
4. **Story-Driven:** Reads story files to extract UI requirements
5. **Token-Efficient:** Uses native tools exclusively (40-73% savings)
6. **Quality-Focused:** Applies best practices and prevents anti-patterns
7. **Well-Documented:** Updates story files and creates spec summaries

This integration ensures generated UI components are production-ready, compliant with project standards, and seamlessly fit into the spec-driven development workflow.
