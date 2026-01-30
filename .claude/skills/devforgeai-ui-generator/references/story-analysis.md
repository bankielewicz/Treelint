# Phase 2: Story Analysis (Story Mode Only)

Extract UI requirements from story acceptance criteria.

**Objective:** Extract UI requirements from story file if provided.

**When to Execute:** Only in story mode (skip if standalone mode)

---

## Steps

### Step 2.1: Check for Story Parameter

**Determine if user provided a story ID or path:**
- Story ID detected in parameter extraction (see parameter-extraction.md)
- Mode = "story"

**If standalone mode:**
- Skip Phase 2 entirely
- Proceed directly to Phase 3 (Interactive Discovery)

---

### Step 2.2: Locate Story File

**If story ID provided (e.g., `STORY-001`):**
```
Read(file_path="devforgeai/specs/Stories/${STORY_ID}.story.md")
```

**Handle file not found:**
```
IF file not found:
  Use Glob to search for story:
  Glob(pattern="devforgeai/specs/Stories/${STORY_ID}*.story.md")

  IF files found:
    Use first match
  ELSE:
    Display error: "Story file not found for ${STORY_ID}"
    AskUserQuestion: "Continue in standalone mode or cancel?"
    If standalone: Proceed to Phase 3
    If cancel: EXIT skill
```

---

### Step 2.3: Extract UI Requirements

**Parse story sections:**

**1. Acceptance Criteria section (Given/When/Then format):**
- Search for UI-related keywords: "form", "display", "table", "chart", "button", "input", "dropdown", "modal", "dashboard"
- Extract user interactions: "click", "enter", "select", "submit"
- Extract data elements: "name", "email", "date", "status"

**2. Technical Specification section for UI details:**
- Look for API endpoints (data sources for UI)
- Look for data models (what data UI displays)
- Look for business rules (validation, formatting)

**3. Non-Functional Requirements for UI performance/accessibility:**
- Performance targets (load time, render time)
- Accessibility requirements (WCAG level, screen reader support)
- Responsive design requirements (mobile, tablet, desktop)

**4. Identify components mentioned:**
- "login form" → LoginForm component
- "dashboard" → Dashboard component
- "data table" → DataTable component
- "user profile" → UserProfile component

---

### Step 2.4: Summarize Requirements

**Create a brief summary of:**

**Components needed:**
```
components = [
  {name: "LoginForm", elements: ["email input", "password input", "submit button"]},
  {name: "Dashboard", elements: ["KPI cards", "charts", "recent activity table"]}
]
```

**User interactions required:**
```
interactions = [
  "User enters email and password",
  "User clicks submit",
  "System validates credentials",
  "System displays success/error message"
]
```

**Data displayed/collected:**
```
data = {
  collected: ["email", "password"],
  displayed: ["error messages", "success confirmation"]
}
```

**Accessibility requirements:**
```
accessibility = {
  level: "WCAG 2.1 AA",
  features: ["keyboard navigation", "screen reader support", "focus indicators"]
}
```

---

## Output

**Phase 2 produces:**
- UI requirements summary
- Component list
- User interaction flows
- Data requirements
- Accessibility requirements

**Ready for:**
- Phase 3: Interactive Discovery (with story context)
