---
name: devforgeai-ui-generator
description: This skill generates front-end UI specifications and code for Web, GUI, or Terminal interfaces within the DevForgeAI framework. It validates context files, respects architectural constraints, and interactively guides users through technology and styling decisions. Use when stories require UI components or when generating visual specifications from requirements.
license: Complete terms in LICENSE.txt
model: claude-model: opus-4-5-20251001
---

# DevForgeAI UI Generator Skill

Generate front-end user interface specifications and code through interactive, constraint-aware workflow.

---

## ⚠️ EXECUTION MODEL: This Skill Expands Inline

**After invocation, YOU (Claude) execute these instructions phase by phase.**

**When you invoke this skill:**
1. This SKILL.md content is now in your conversation
2. You execute each phase sequentially
3. You display results as you work through phases
4. You complete with success/failure report

**Do NOT:**
- ❌ Wait passively for skill to "return results"
- ❌ Assume skill is executing elsewhere
- ❌ Stop workflow after invocation

**Proceed to "Parameter Extraction" section below and begin execution.**

---

## Parameter Extraction

This skill operates in two modes:
- **Story Mode:** Extract story ID from loaded file or context markers
- **Standalone Mode:** Extract component description from conversation

**See `references/parameter-extraction.md` for complete mode detection and parameter extraction logic.**

---

## When to Use

**Use when:**
- Story requires UI components (forms, dashboards, dialogs, tables)
- Generating visual specifications from acceptance criteria
- Creating mockups-as-code for web, native GUI, or terminal interfaces
- Translating requirements into tangible UI components

**Prerequisites:**
- Context files must exist (6 files in devforgeai/specs/context/)
- For story mode: Story file must exist
- For standalone: Component description in conversation

---

## Core Workflow (7 Phases)

**⚠️ EXECUTION STARTS HERE - You are now executing the skill's workflow.**

Each phase loads its reference file on-demand for detailed implementation.

### Phase 1: Context Validation
**Purpose:** Verify all 6 context files exist
**Reference:** `context-validation.md`
**Halts if:** Context files missing → directs to devforgeai-architecture

### Phase 2: Story Analysis & Interactive Discovery

**Purpose:** Extract UI requirements from story AC (Story Mode) OR guide user through UI decisions (Standalone Mode)

**Reference:** `story-analysis.md`, `interactive-discovery.md`

**Step 0: Conditional User Input Guidance Loading [MANDATORY]**

Detect mode and load guidance patterns conditionally:

```python
# Detect mode via conversation context markers
is_story_mode = (
    conversation_contains("@devforgeai/specs/Stories/STORY-") or
    context_marker_exists("**Story ID:**") or
    context_marker_exists("**Story file:**")
)

if is_story_mode:
    # Story mode - story file already loaded
    mode = "story"
    Display: "Story mode detected. Skipping user-input-guidance.md (using story UI requirements)."
    # Extract UI requirements from story AC (proceed to story-analysis.md)

else:
    # Standalone mode - no story file
    mode = "standalone"
    Display: "Standalone mode detected. Loading user-input-guidance.md..."
    guidance = Read(file_path=".claude/skills/devforgeai-ui-generator/references/user-input-guidance.md")
    Display: "✓ Guidance loaded - applying patterns to UI questions"
```

**Pattern Application (if guidance loaded in standalone mode):**
- **Step 1:** Use **Explicit Classification** pattern for UI type (4 options: Web UI/Desktop GUI/Mobile App/Terminal UI)
- **Step 2:** Use **Bounded Choice** pattern for framework selection (filtered by UI type and tech-stack.md)
- **Step 3:** Use **Bounded Choice** pattern for styling approach (5 options: Tailwind/Bootstrap/Material/Custom/None)

**See:** `references/ui-user-input-integration.md` for complete pattern mappings and examples.

**Output:** UI component requirements, user flows

### Phase 3: Interactive Discovery
**Purpose:** Guide user through technology and styling choices
**Reference:** `interactive-discovery.md`
**Interactions:** 3-5 AskUserQuestion flows (UI type, tech stack, styling, theme)
**Conflict resolution:** If selections conflict with tech-stack.md

### Phase 4: Template & Best Practices Loading
**Purpose:** Load appropriate templates from assets/
**Reference:** `template-loading.md`
**Templates:** 7 templates available (React, Blazor, ASP.NET, HTML, WPF, Tkinter, Terminal)
**Best practices:** Load web/gui/tui best practices reference

### Phase 5: Code Generation
**Purpose:** Generate production-ready UI component code
**Reference:** `code-generation.md`
**Output:** Component files in devforgeai/specs/ui/

### Phase 6: Documentation & Story Update
**Purpose:** Create UI spec summary and update story
**Reference:** `documentation-update.md`
**Output:** UI-SPEC-SUMMARY.md, story updated with UI references

**Step 6.3.5: Invoke ui-spec-formatter Subagent**
**Reference:** `ui-spec-formatter-integration.md`
**Subagent:** ui-spec-formatter (validates and formats results)
**Output:** Structured JSON with display template

### Phase 7: Specification Validation
**Purpose:** Comprehensive validation with user resolution
**Reference:** `specification-validation.md`
**Validations:** Completeness (10 sections), placeholders, framework constraints
**Resolution:** User resolves ALL issues (no self-healing)
**Status:** SUCCESS (proceed) | PARTIAL (warnings) | FAILED (halt)

**See individual phase reference files for complete implementation details.**

### Phase N: Feedback Hook Integration
**Purpose:** Collect feedback on UI design, technology fit, and component complexity (NEW - RCA-009 STORY-032)
**Pattern:** Follows STORY-023 (/dev) and STORY-031 (/ideate) implementation
**Steps:**
1. **Step N.1:** Call `devforgeai-validate check-hooks --operation=create-ui --status=completed`
2. **Step N.2:** If eligible (exit code 0), call `devforgeai-validate invoke-hooks --operation=create-ui`
3. **Step N.3:** Pass context: ui_type, technology, components, styling, story ID (if applicable)
4. **Step N.4:** Handle failures gracefully (log warning, continue - non-blocking)
5. **Step N.5:** Display status message (feedback enabled/unavailable)

**Output:** Feedback conversation may trigger if hooks configured, or silent success if not enabled

---

## Subagent Coordination

**ui-spec-formatter** (Step 6.3.5)
- Validates generated specifications
- Formats results for display
- Returns structured JSON
- Respects framework guardrails (see ui-result-formatting-guide.md)

---

## Integration Points

**Invoked by:**
- `/create-ui` command (user-initiated)
- devforgeai-orchestration (when story has UI requirements)
- devforgeai-development (during implementation)

**Invokes:**
- ui-spec-formatter subagent (Phase 6 Step 3.5)

**Provides output to:**
- devforgeai-development (UI specs → implementation)
- devforgeai-qa (UI specs → validation)

---

## Resource Map

### Workflow References (11 files)
- **parameter-extraction.md** - Mode and parameter detection
- **context-validation.md** - Phase 1: 6 context files check
- **story-analysis.md** - Phase 2: Extract UI requirements from story
- **interactive-discovery.md** - Phase 3: Tech/styling selection
- **template-loading.md** - Phase 4: Load from assets/
- **code-generation.md** - Phase 5: Generate component code
- **documentation-update.md** - Phase 6: Update story and summary
- **ui-spec-formatter-integration.md** - Step 6.3.5: Formatter invocation
- **specification-validation.md** - Phase 7: Validation workflow
- **ui-generation-examples.md** - Complete usage examples
- **error-handling.md** - Recovery procedures

### Best Practices Guides (5 files)
- **web_best_practices.md** - Semantic HTML, accessibility, responsive design
- **gui_best_practices.md** - Layout organization, keyboard navigation
- **tui_best_practices.md** - Terminal formatting, box-drawing, ANSI colors
- **ui-result-formatting-guide.md** - ui-spec-formatter guardrails
- **devforgeai-integration-guide.md** - Framework integration patterns

### Assets (7 templates)
- **web-template.jsx** - React functional component
- **web-template.blazor.razor** - Blazor component
- **web-template.aspnet.cshtml** - ASP.NET MVC view
- **web-template.html** - Plain HTML5
- **gui-template.wpf.xaml** - WPF Window
- **gui-template.py** - Python Tkinter app
- **tui-template.py** - Terminal formatting functions

---

## Quick Start Example

```
User: "Generate login form for STORY-042"

Skill workflow:
1. Phase 1: Validate context files ✓
2. Phase 2: Read STORY-042.story.md → find "email and password fields"
3. Phase 3: Ask user (Web UI, React, Tailwind, Dark mode)
4. Phase 4: Load web-template.jsx + web_best_practices.md
5. Phase 5: Generate LoginForm.jsx
6. Phase 6: Invoke ui-spec-formatter → format results
7. Phase 7: Validate (completeness, placeholders, framework)
8. Output: LoginForm.jsx in devforgeai/specs/ui/

Token usage: ~3,500 tokens (entry 1.5K + phases 2K)
```

**For detailed examples → See `references/ui-generation-examples.md`**

---

## Quality Standards

Generated UI code must:
- ✅ Follow coding-standards.md conventions
- ✅ Use technologies from tech-stack.md only
- ✅ Place files per source-tree.md
- ✅ Include accessibility (ARIA, semantic HTML)
- ✅ Match story acceptance criteria
- ✅ Apply best practices from references/

---

## Error Handling

**Common errors:**
1. Context files missing → Direct to /create-context
2. Technology conflict → AskUserQuestion for resolution
3. Template not found → Validate assets directory
4. Validation failures → User resolves (Phase 7)

**See `references/error-handling.md` for complete recovery procedures.**

---

## Token Efficiency

**Target:** ~35,000 tokens per component (isolated context)

**Efficiency achieved through:**
- Native tool usage (Read/Write/Edit not Bash)
- Progressive loading (entry 1.5K, phases as needed)
- Context validation once at start
- Focused generation per component

**Load references progressively:**
- Load only UI type-specific best practices (web OR gui OR tui)
- Load phase references only when executing that phase
- Don't load all references upfront
