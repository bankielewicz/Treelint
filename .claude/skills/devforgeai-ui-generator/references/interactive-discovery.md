# Phase 3: Interactive Discovery

Guide user through technology and styling decisions using AskUserQuestion.

**Objective:** Collect all necessary decisions for UI generation through interactive questions:
1. UI type (Web/GUI/Terminal)
2. Technology stack
3. Styling approach
4. Component structure

**Important:** Respect context files at all times. If user requests technology not in `tech-stack.md`, use AskUserQuestion to resolve conflict.

---

## Step 3.1: Determine UI Type

Use AskUserQuestion to ask:

```
AskUserQuestion(
  questions: [{
    question: "What type of UI are we building?",
    header: "UI Type",
    multiSelect: false,
    options: [
      {
        label: "Web UI",
        description: "Browser-based interface (React, Blazor, ASP.NET, HTML)"
      },
      {
        label: "Native GUI",
        description: "Desktop application (WPF, Tkinter, .NET MAUI)"
      },
      {
        label: "Terminal UI",
        description: "Command-line interface with formatted text/ASCII art"
      }
    ]
  }]
)
```

**Capture response:**
```
ui_type = response["UI Type"]
```

---

## Step 3.2: Technology Selection (Web UI)

**If Web UI selected:**

```
AskUserQuestion(
  questions: [
    {
      question: "What web technology should be used for this UI?",
      header: "Web Tech",
      multiSelect: false,
      options: [
        { label: "React", description: "Modern component-based framework" },
        { label: "Blazor Server", description: "C# server-side rendering" },
        { label: "Blazor WASM", description: "C# client-side WebAssembly" },
        { label: "ASP.NET MVC", description: "Server-side MVC views" },
        { label: "Plain HTML", description: "Static HTML/CSS/JS" }
      ]
    },
    {
      question: "What styling approach should be used?",
      header: "Styling",
      multiSelect: false,
      options: [
        { label: "Tailwind CSS", description: "Utility-first CSS framework" },
        { label: "Bootstrap", description: "Component library with pre-built styles" },
        { label: "Plain CSS", description: "Custom CSS from scratch" },
        { label: "CSS Modules", description: "Scoped CSS per component" }
      ]
    },
    {
      question: "What theme/color scheme should be applied?",
      header: "Theme",
      multiSelect: false,
      options: [
        { label: "Dark Mode", description: "Dark background, light text" },
        { label: "Light Mode", description: "Light background, dark text" },
        { label: "Minimalist", description: "Clean, simple design with minimal colors" },
        { label: "Vibrant", description: "Bold colors and gradients" }
      ]
    }
  ]
)
```

**Capture responses:**
```
web_tech = response["Web Tech"]
styling = response["Styling"]
theme = response["Theme"]
```

---

## Step 3.2: Technology Selection (Native GUI)

**If Native GUI selected:**

```
AskUserQuestion(
  questions: [{
    question: "What native GUI technology stack should be used?",
    header: "GUI Tech",
    multiSelect: false,
    options: [
      { label: "C# WPF", description: "Windows Presentation Foundation (XAML)" },
      { label: "Python Tkinter", description: "Python's standard GUI library" },
      { label: "C# .NET MAUI", description: "Cross-platform .NET UI framework" },
      { label: "Python PyQt", description: "Python bindings for Qt framework" }
    ]
  }]
)
```

**Capture response:**
```
gui_tech = response["GUI Tech"]
```

---

## Step 3.2: Technology Selection (Terminal UI)

**If Terminal UI selected:**

```
AskUserQuestion(
  questions: [{
    question: "What formatting style should be used for the terminal UI?",
    header: "TUI Format",
    multiSelect: false,
    options: [
      { label: "Box Drawing", description: "Unicode box-drawing characters for tables" },
      { label: "Simple ASCII", description: "Plain dashes and pipes" },
      { label: "Color Coded", description: "ANSI color codes for emphasis" },
      { label: "Rich Tables", description: "Python rich library for advanced formatting" }
    ]
  }]
)
```

**Capture response:**
```
tui_format = response["TUI Format"]
```

---

## Step 3.3: Validate Against Context

After receiving user's technology choices:

**1. Check tech-stack.md:** Verify chosen technology is approved.

```
Read(file_path="devforgeai/specs/context/tech-stack.md")

Search for user's chosen technology in tech-stack.md

IF technology not found in tech-stack.md:
  CONFLICT_DETECTED = true
  CHOSEN_TECH = ${user selection}
  APPROVED_TECH = ${extract from tech-stack.md}
```

**2. If mismatch detected:**
```
AskUserQuestion(
  questions: [{
    question: "You selected ${CHOSEN_TECH}, but tech-stack.md specifies ${APPROVED_TECH}. Which should be used?",
    header: "Tech Conflict",
    multiSelect: false,
    options: [
      { label: "Use ${CHOSEN_TECH}", description: "Update tech-stack.md and create ADR" },
      { label: "Use ${APPROVED_TECH}", description: "Follow existing standard" }
    ]
  }]
)
```

**Handle resolution:**
```
IF user selects "Use ${CHOSEN_TECH}":
  Display: "This requires updating tech-stack.md and creating an ADR."

  AskUserQuestion:
    Question: "Update tech-stack.md now or handle manually?"
    Options: [
      "Update now (create ADR stub)",
      "I'll update manually after UI generation"
    ]

  IF "Update now":
    Edit tech-stack.md to add ${CHOSEN_TECH}
    Create ADR stub in devforgeai/specs/adrs/
    Proceed with ${CHOSEN_TECH}

  ELSE:
    Record: "tech-stack.md update required (user will handle manually)"
    Proceed with ${CHOSEN_TECH}

ELSE:
  # Use approved tech
  SELECTED_TECH = ${APPROVED_TECH}
  Proceed with ${APPROVED_TECH}
```

---

## Step 3.4: Define Components

Based on story requirements (if available) or user prompt, propose components:

**Analyze requirements and propose component list:**
```
IF story_mode:
  Extract component needs from acceptance criteria
  Build suggested_components list

ELSE: # standalone mode
  Analyze component description from user
  Build suggested_components list
```

**Ask user to confirm/modify:**
```
AskUserQuestion(
  questions: [{
    question: "Based on the requirements, I suggest building these components: ${suggested_components.join(', ')}. What elements should each component include?",
    header: "Components",
    multiSelect: false,
    options: [
      { label: "Use suggested components", description: "Build: ${suggested_components.join(', ')}" },
      { label: "Modify components", description: "Specify custom components" },
      { label: "Add more components", description: "Extend the suggested list" }
    ]
  }]
)
```

**If user selects "Modify" or "Add more":**

```
AskUserQuestion(
  questions: [{
    question: "Please describe the components you want to build (e.g., 'LoginForm: email input, password input, submit button')",
    header: "Custom Components",
    multiSelect: false,
    options: [
      { label: "Provide description", description: "I'll specify component details in text" }
    ]
  }]
)

# Parse user's "Other" text input
component_list = parse_component_descriptions(user_input)
```

**Finalize component specifications:**
```
final_components = [
  {
    name: "LoginForm",
    elements: ["email input", "password input", "submit button", "forgot password link"],
    type: "form"
  },
  {
    name: "Header",
    elements: ["logo", "navigation menu", "user profile dropdown"],
    type: "navigation"
  }
  # ... etc
]
```

**Output:** Complete technology and component specification.

**Summary of Phase 3 Decisions:**
- UI Type: ${ui_type}
- Technology: ${selected_tech}
- Styling: ${styling} (if web)
- Theme: ${theme} (if web)
- Format: ${tui_format} (if terminal)
- Components: ${final_components}
- Context validation: ${conflicts_resolved ? 'Conflicts resolved' : 'No conflicts'}
