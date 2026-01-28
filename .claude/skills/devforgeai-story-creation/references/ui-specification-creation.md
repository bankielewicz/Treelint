# Phase 4: UI Specification Creation

Generate UI specifications including components, mockups, interactions, and accessibility requirements.

## Overview

This phase documents the user interface aspects of the story when UI components are needed. Supports web, desktop, mobile, and terminal interfaces.

---

## Step 4.1: Detect UI Requirements

**Objective:** Determine if story requires user interface components

**Keyword analysis:**

```
ui_keywords = ["screen", "page", "form", "button", "display", "show", "view",
               "modal", "dialog", "table", "chart", "dashboard", "input",
               "dropdown", "checkbox", "click", "navigate"]

requires_ui = any(keyword in feature_description.lower() or
                  keyword in ac_text.lower()
                  for keyword in ui_keywords)
```

**Confirm with user:**
```
AskUserQuestion(
  questions=[{
    question: "Does this story require UI components or user interface?",
    header: "UI needed",
    options: [
      {
        label: "Yes - web UI",
        description: "Browser-based user interface (React, Vue, HTML)"
      },
      {
        label: "Yes - desktop/mobile UI",
        description: "Native application interface (WPF, MAUI, Flutter)"
      },
      {
        label: "Yes - terminal UI",
        description: "Command-line interface with formatted output"
      },
      {
        label: "No - backend only",
        description: "API or business logic without user interface"
      }
    ],
    multiSelect: false
  }]
)
```

**If no UI needed:**
- Skip Steps 4.2-4.6
- Phase 4 outputs = null
- Proceed to Phase 5

---

## Step 4.2: Document UI Components

**Objective:** Identify and specify UI components from acceptance criteria

**Load UI specification guide:**
```
Read(file_path=".claude/skills/devforgeai-story-creation/references/ui-specification-guide.md")
```

**For each component identified in acceptance criteria:**

```
Component documentation:

Component: {ComponentName}
Type: [Form | Table | Modal | Card | Chart | Navigation | Layout]
Purpose: {1-sentence description}

Data Bindings:
- Input: {state properties component reads}
- Output: {events/callbacks component emits}
- State: {local component state}

Example:
Component: RegistrationForm
Type: Form
Purpose: Collect user registration details (email, password, name)

Data Bindings:
- Input: None (initial render)
- Output: onSubmit(RegistrationData), onCancel()
- State: formData, validationErrors, isSubmitting
```

---

## Step 4.3: Create ASCII Layout Mockup

**Objective:** Visual representation of UI layout using ASCII art

**Use box-drawing characters for visual layout:**

```
+------------------------------------------+
|           [Page Title/Header]            |
+------------------------------------------+
| Section Label                            |
| [ Input Field: Placeholder         ]    |
| [ Dropdown: Select option         v]    |
| [ ] Checkbox with label text            |
|                                          |
| [Button: Primary]  [Button: Secondary]  |
+------------------------------------------+
| [ Data Table                          ] |
| | Column 1 | Column 2 | Actions        ||
| | Value A  | Value B  | [Edit] [Delete]||
| | Value C  | Value D  | [Edit] [Delete]||
+------------------------------------------+
```

**Guidelines from ui-specification-guide.md:**
- Use +-| for borders
- Align elements consistently
- Show interactive elements with brackets []
- Indicate dropdowns with v
- Show data tables with | separators
- Label all sections clearly

---

## Step 4.4: Define Component Interfaces

**Objective:** Specify component props/parameters in typed language

**If tech stack uses TypeScript/C#/typed language:**

```
Generate component interface/props:

TypeScript Example:
```typescript
interface RegistrationFormProps {
  onSubmit: (data: RegistrationData) => Promise<void>;
  onCancel: () => void;
  isLoading: boolean;
  error: string | null;
}

interface RegistrationData {
  email: string;
  password: string;
  name: string;
  agreedToTerms: boolean;
}
```

C# Example:
```csharp
public class RegistrationFormProps
{
    public Action<RegistrationData> OnSubmit { get; set; }
    public Action OnCancel { get; set; }
    public bool IsLoading { get; set; }
    public string Error { get; set; }
}

public class RegistrationData
{
    public string Email { get; set; }
    public string Password { get; set; }
    public string Name { get; set; }
    public bool AgreedToTerms { get; set; }
}
```
```

---

## Step 4.5: Document User Interaction Flows

**Objective:** Define step-by-step user interaction sequences

**Step-by-step interaction sequences:**

```
User Interaction Flow:

1. User navigates to [page/screen]
2. [Initial state description]
3. User [action 1] (e.g., clicks button, types text)
4. System [response 1] (e.g., validates input, shows loading)
5. User [action 2]
6. System [response 2]
7. Success path: [final outcome]
8. Error path: [error handling, user can recover]

Include:
- User actions (clicks, types, selects, navigates)
- System responses (validation, loading states, success/error messages)
- State changes (form fields, data display, navigation)
- Branching logic (if/else scenarios)

Example:
1. User navigates to /register
2. Empty registration form displays
3. User types email → Real-time validation (email format check)
4. User types password → Strength indicator updates (weak/medium/strong)
5. User types name → Validation (minimum 2 characters)
6. User checks "I agree to Terms" checkbox
7. User clicks "Create Account" button
8. Loading state: Button shows spinner, form disabled
9. Success: Redirect to /verify-email-sent page
10. Error: Error message displays above form, form remains editable
```

---

## Step 4.6: Specify Accessibility Requirements

**Objective:** Define WCAG 2.1 Level AA compliance requirements

**Following WCAG 2.1 Level AA standards:**

```
Accessibility Requirements:

**Keyboard Navigation:**
- Tab order: {list elements in tab sequence}
- Enter/Space: {actions triggered by Enter or Space keys}
- Escape: {cancel/close actions}
- Arrow keys: {navigation in lists/dropdowns}

**Screen Reader Support:**
- aria-label: {labels for icon-only buttons, form inputs}
- aria-describedby: {field descriptions, error messages}
- aria-live: {dynamic content announcements (polite, assertive)}
- role: {semantic roles for custom components}
- Example:
  - aria-label="Email address" on email input
  - aria-describedby="password-requirements" on password field
  - aria-live="polite" on error messages

**Focus Management:**
- Visual focus indicators (outline, border change)
- Focus trap in modals (focus stays in modal, Escape closes)
- Auto-focus on first input when form loads
- Focus on error message when validation fails

**Color Contrast:**
- Text on background: Minimum 4.5:1 contrast ratio
- UI components: Minimum 3:1 contrast ratio
- Error text: High contrast (e.g., #D32F2F on #FFFFFF = 7:1)
- Success text: High contrast (e.g., #2E7D32 on #FFFFFF = 4.9:1)

**Error Announcements:**
- Screen reader announces validation errors
- Error messages linked to form fields
- Clear error recovery guidance
```

---

## Subagent Coordination

**No subagents invoked in this phase.**

UI specification is derived from:
- Acceptance criteria analysis
- Feature description keywords
- User confirmation (AskUserQuestion)

---

## Output

**Phase 4 produces (if UI needed):**
- ✅ UI components documented (type, purpose, data bindings)
- ✅ ASCII layout mockup created
- ✅ Component interfaces defined (TypeScript/C#/etc.)
- ✅ User interaction flows documented
- ✅ Accessibility requirements specified (WCAG AA)

**If no UI needed:**
- Phase 4 outputs = null

---

## Error Handling

**Error 1: UI type ambiguous**
- **Detection:** Feature description mentions UI but unclear which type (web, desktop, terminal)
- **Recovery:** Use AskUserQuestion to clarify UI type

**Error 2: Component interfaces incomplete**
- **Detection:** Missing props, events, or state properties
- **Recovery:** Extract from acceptance criteria or ask user

**Error 3: Accessibility requirements missing**
- **Detection:** No ARIA attributes, keyboard navigation, or focus management
- **Recovery:** Apply WCAG AA defaults from ui-specification-guide.md

See `error-handling.md` for comprehensive error recovery procedures.

---

## Next Phase

**After Phase 4 completes →** Phase 5: Story File Creation

Load `story-file-creation.md` for Phase 5 workflow.
