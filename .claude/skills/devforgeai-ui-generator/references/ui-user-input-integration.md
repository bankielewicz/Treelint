---
id: ui-user-input-integration
title: UI-Generator Skill - User Input Guidance Integration
version: "1.0"
created: 2025-01-21
updated: 2025-01-21
status: Published
audience: DevForgeAI Development Team
parent_document: user-input-guidance.md
skill: devforgeai-ui-generator
---

# UI-Generator Skill - User Input Guidance Integration Reference

**Purpose:** Document how devforgeai-ui-generator skill integrates user-input-guidance.md with conditional loading logic for standalone vs story-driven UI generation.

**Context:** This skill generates front-end UI specifications and code for Web, Desktop GUI, Mobile, or Terminal interfaces. It can operate in two modes: standalone (component description) or story-driven (extract requirements from story).

---

## Section 1: Conditional Loading Logic

### 1.1 Standalone vs Story-Driven Mode Detection

**Standalone Mode:** User provides component description without story file
- **Detection:** No story file loaded via `@STORY-NNN.story.md` marker
- **Action:** Load user-input-guidance.md and apply guidance patterns
- **Rationale:** User needs interactive guidance for UI type, framework, styling decisions
- **Token Cost:** ~1,000 tokens (guidance + pattern application)

**Story Mode:** User runs `/create-ui STORY-042` with story file context
- **Detection:** Story file loaded OR context marker "**Story ID:**" present
- **Action:** Skip guidance, extract UI requirements from story acceptance criteria
- **Rationale:** Story defines requirements; no user interaction needed for scope
- **Token Cost:** 0 tokens (no guidance loaded)

### 1.2 Conditional Loading Pseudocode

```python
def phase_2_story_analysis():
    """Phase 2: Story Analysis with conditional guidance loading"""

    # Step 0: Detect mode and load guidance conditionally
    # Check conversation context for story markers
    is_story_mode = (
        conversation_contains("@devforgeai/specs/Stories/STORY-") or
        context_marker_exists("**Story ID:**") or
        context_marker_exists("**Story file:**")
    )

    if is_story_mode:
        # Story mode - story file already loaded
        mode = "story"
        skip_guidance = True
        log_info("Story mode detected. Skipping user-input-guidance.md (using story UI requirements).")
        # Extract UI requirements from story AC
        ui_requirements = extract_from_story_ac()

    else:
        # Standalone mode - no story file, user provides component description
        mode = "standalone"
        skip_guidance = False
        log_info("Standalone mode detected. Loading user-input-guidance.md...")
        guidance_content = Read(file_path=".claude/skills/devforgeai-ui-generator/references/user-input-guidance.md")
        # Use guidance patterns for interactive discovery

    # Step 1-N: Execute Phase 2 workflow
    # In standalone mode, use guidance patterns below for UI decisions
    # In story mode, skip guidance and use story requirements
```

---

## Section 2: Pattern Mapping for UI Questions

### 2.1 Phase 2 Question-to-Pattern Mapping

**This table shows which user-input-guidance patterns apply to Phase 2 questions:**

| Phase | Step | Question | Pattern Name | Pattern # | Guidance Template | Options/Bounds | Rationale |
|-------|------|----------|--------------|-----------|------------------|----------------|-----------|
| **2** | **1** | "What UI type?" | Explicit Classification | 3 | FUN-001 | 4 options: Web UI / Desktop GUI / Mobile App / Terminal UI | Distinct UI paradigms |
| **2** | **2** | "Framework choice?" | Bounded Choice | 2 | CONST-001 | 3-5 options (filtered by UI type & tech-stack.md) | Known frameworks per UI type |
| **2** | **3** | "Styling approach?" | Bounded Choice | 2 | NFR-002 | 5 options: Tailwind CSS / Bootstrap / Material UI / Custom CSS / None | Common styling solutions |

### 2.2 Pattern Application Logic

**Pattern 3: Explicit Classification**
- **Use for:** UI type selection (Web, Desktop, Mobile, Terminal)
- **Application:** Present 4 mutually exclusive UI paradigm options with descriptions
- **Reasoning:** Each UI type has fundamentally different constraints and technologies
- **User Input:** Select one UI type (single-select)
- **Result:** Filter framework options based on selected UI type

**Pattern 2: Bounded Choice**
- **Use for:** Framework selection (filtered by UI type and tech-stack.md)
- **Application:** Present list of frameworks compatible with selected UI type
- **Reasoning:** Known frameworks with trade-offs per UI type
- **User Input:** Select one framework (single-select)
- **Result:** Filter styling options and generate UI specification

**Pattern 2: Bounded Choice (Styling)**
- **Use for:** Styling approach (Tailwind, Bootstrap, Material UI, Custom, None)
- **Application:** Present 5 styling options with descriptions
- **Reasoning:** Known solutions for styling; depends on framework choice
- **User Input:** Select one styling approach (single-select)
- **Result:** Load appropriate style templates and generate UI code

---

## Section 3: Guidance Pattern Examples

### 3.1 Example: UI Type Selection with Explicit Classification

**Without Guidance:**
```
Question: "What type of UI?"
Answer: "Modern interface" [ambiguous - could be any platform]
Result: Unclear which platform (web, mobile, desktop, terminal)
```

**With Explicit Classification Pattern:**
```
AskUserQuestion(
    question: "What type of user interface are you building?",
    header: "UI Platform Type",
    description: "Each UI type has different constraints, technologies, and design patterns. Choose the platform that best matches your requirements.",
    options: [
        {
            label: "Web UI",
            description: "Browser-based (React, Vue, Angular, etc.). Best for: cross-platform, easy deployment, responsive design. Examples: Dashboards, SaaS apps, e-commerce"
        },
        {
            label: "Desktop GUI",
            description: "Native desktop application (WPF, Electron, PyQt, etc.). Best for: offline-first, system access, rich interactions. Examples: Video editors, IDEs, finance apps"
        },
        {
            label: "Mobile App",
            description: "Mobile application (React Native, Flutter, native iOS/Android). Best for: on-the-go, touch interactions, device integration. Examples: Messaging, fitness tracking, mobile banking"
        },
        {
            label: "Terminal UI",
            description: "Command-line interface (Blessed, Curses, Ink, etc.). Best for: developers, automation, server environments. Examples: Git clients, deployment tools, system monitoring"
        }
    ],
    multiSelect: false
)
```

**Result:** User selects "Web UI" → Framework selection filters to web frameworks

### 3.2 Example: Framework Selection with Bounded Choice (Filtered)

**Without Guidance:**
```
Question: "What framework?"
Answer: [User lists all frameworks regardless of UI type]
Result: Conflicting framework choices (Can't use React Native AND Flutter)
```

**With Bounded Choice Pattern (Filtered by UI Type):**
```
# After user selected "Web UI":

AskUserQuestion(
    question: "Which web framework will you use?",
    header: "Frontend Framework",
    description: "Select the primary framework for your web UI. Each has different strengths for component patterns, state management, and build tooling.",
    options: [
        {
            label: "React (with TypeScript)",
            description: "Component-based, large ecosystem, excellent tooling. Best for: complex UIs, teams familiar with React. Community: Largest"
        },
        {
            label: "Vue.js (with TypeScript)",
            description: "Progressive framework, gentle learning curve, good performance. Best for: medium complexity, teams learning frontend. Community: Large, growing"
        },
        {
            label: "Angular",
            description: "Full framework, enterprise patterns, strong typing. Best for: large teams, enterprise apps, strict structure. Community: Established"
        },
        {
            label: "Svelte",
            description: "Compiler-based, minimal runtime, excellent performance. Best for: performance-critical apps, smaller teams. Community: Growing"
        },
        {
            label: "Next.js (React meta-framework)",
            description: "Server-side rendering, static generation, API routes. Best for: full-stack apps, SEO-critical, edge deployment. Community: Growing fast"
        }
    ],
    multiSelect: false
)
```

**Note:** If user selected "Desktop GUI", framework options would be different:
- Electron
- WPF
- PyQt
- Tauri
- NW.js

### 3.3 Example: Styling Approach with Bounded Choice

**Without Guidance:**
```
Question: "Styling approach?"
Answer: "Good styling" [vague, no clear direction]
Result: Inconsistent styling solutions across components
```

**With Bounded Choice Pattern:**
```
# After framework selection (e.g., React):

AskUserQuestion(
    question: "What styling approach will you use?",
    header: "CSS/Styling Strategy",
    description: "Choose how you'll style your components. Consider team familiarity, project size, and design system needs.",
    options: [
        {
            label: "Tailwind CSS",
            description: "Utility-first, rapid prototyping. Best for: rapid development, design systems, responsive design. Learning curve: Moderate"
        },
        {
            label: "Bootstrap",
            description: "Component-based, ready-made components. Best for: quick projects, teams learning CSS, consistency. Learning curve: Low"
        },
        {
            label: "Material UI (MUI)",
            description: "Design system implementation, comprehensive components. Best for: professional apps, Material Design requirements. Learning curve: Moderate"
        },
        {
            label: "Custom CSS / CSS-in-JS",
            description: "Full control, styled-components or Emotion. Best for: unique designs, complex animations, custom branding. Learning curve: High"
        },
        {
            label: "None (Semantic HTML only)",
            description: "Minimal styling, focus on structure. Best for: content-heavy sites, accessibility-first, minimal overhead. Learning curve: Low"
        }
    ],
    multiSelect: false
)
```

**Result:** Styling approach selected → Load corresponding style templates

---

## Section 4: Conditional Guidance Triggers

### 4.1 When to Load Guidance (Standalone Mode)

**Load guidance in these scenarios:**
1. ✅ `/create-ui "Component description"` (no story file)
2. ✅ User runs `/create-ui` with text description (not story ID)
3. ✅ No story file loaded in conversation

**Skip guidance in these scenarios:**
1. ❌ `/create-ui STORY-042` (story file provided)
2. ❌ Story file loaded via `@STORY-042.story.md`
3. ❌ Context contains "**Story ID:**" marker

### 4.2 Guidance Loading at Phase 2 Step 0

**Add this Step 0 to Phase 2 in SKILL.md:**

```markdown
### Phase 2: Story Analysis (Story Mode Only)

#### Step 0: Conditional Guidance Load (NEW - STORY-057)

**Purpose:** Load user-input-guidance.md only in standalone mode for interactive UI discovery

**Implementation:**
- Detect mode: Check conversation context
  - Story file loaded? (Check for "@STORY-" marker, "**Story ID:**" context)
    - YES → Story mode (SKIP guidance)
    - NO → Standalone mode (LOAD guidance)
- If standalone mode:
  - Execute: `Read(file_path=".claude/skills/devforgeai-ui-generator/references/user-input-guidance.md")`
  - Log: "Standalone mode detected. Loading user-input-guidance.md..."
  - Available for Steps 1-3 pattern references
- If story mode:
  - Log: "Story mode detected. Skipping user-input-guidance.md (using story UI requirements)."
  - Extract UI requirements from story AC in Phase 2 Step 1
  - Proceed to Phase 3 (only ask about missing details)

**Rationale:** Token efficiency (skip guidance when story defines UI), better UX (guided discovery in standalone)

**Token Cost:** ~1,000 tokens in standalone mode (guidance + patterns), 0 in story mode
```

---

## Section 5: Reference Deployment

### 5.1 File Locations

**Master File:** `src/claude/skills/devforgeai-ideation/references/user-input-guidance.md`

**UI-Generator Deployment:**
- Location: `src/claude/skills/devforgeai-ui-generator/references/user-input-guidance.md`
- Deployment: Copy from master using:
  ```bash
  cp src/claude/skills/devforgeai-ideation/references/user-input-guidance.md \
     src/claude/skills/devforgeai-ui-generator/references/user-input-guidance.md
  ```

**Operational Folder:**
- Also copied to: `.claude/skills/devforgeai-ui-generator/references/user-input-guidance.md`

### 5.2 Checksum Validation

**Verify deployment integrity:**
```bash
# All 3 ui-generator deployments should have identical SHA256
sha256sum src/claude/skills/devforgeai-ui-generator/references/user-input-guidance.md \
          .claude/skills/devforgeai-ui-generator/references/user-input-guidance.md

# If hashes match: ✅ Deployment successful
# If hashes differ: ❌ Files out of sync, redeploy
```

---

## Section 6: Testing Strategy

### 6.1 Unit Tests for Conditional Loading

**Test 1: Standalone mode loads guidance**
```python
def test_standalone_loads_guidance():
    """Verify guidance loads when no story file provided"""
    # Setup: No @STORY- marker, no "Story ID:" context
    # Execute: Phase 2 Step 0
    # Assert: Read called with guidance file path
    # Assert: Log contains "Standalone mode"
```

**Test 2: Story mode skips guidance**
```python
def test_story_mode_skips_guidance():
    """Verify guidance skipped when story file loaded"""
    # Setup: Mock story file loaded with "@STORY-042" marker
    # Execute: Phase 2 Step 0
    # Assert: Read NOT called for guidance
    # Assert: Log contains "Story mode"
```

**Test 3: Empty story file (standalone fallback)**
```python
def test_empty_story_loads_guidance():
    """Verify guidance loaded if story file empty/invalid"""
    # Setup: Story file exists but empty or no UI spec
    # Execute: Phase 2 Step 0
    # Assert: Read called for guidance (fallback)
    # Assert: Log indicates fallback mode
```

### 6.2 Pattern Application Tests

**Test 4: Explicit Classification applied to UI type**
```python
def test_explicit_classification_applied_ui_type():
    """Verify 4-option UI type classification question"""
    # Setup: Standalone mode, guidance loaded
    # Execute: Phase 2 Step 1 (UI type question)
    # Assert: AskUserQuestion has exactly 4 options
    # Assert: Options are: Web UI, Desktop GUI, Mobile App, Terminal UI
```

**Test 5: Bounded Choice filtered by UI type**
```python
def test_bounded_choice_filtered_by_ui_type():
    """Verify framework options filtered based on UI type"""
    # Setup: User selected "Web UI"
    # Execute: Framework selection question
    # Assert: Options include only web frameworks (React, Vue, Angular, etc.)
    # Assert: Options exclude desktop frameworks (WPF, PyQt, Electron)
```

**Test 6: Bounded Choice for styling (5 options)**
```python
def test_bounded_choice_styling_5_options():
    """Verify styling question has exactly 5 options"""
    # Setup: Framework selected
    # Execute: Styling approach question
    # Assert: Exactly 5 options: Tailwind, Bootstrap, Material, Custom, None
```

### 6.3 Integration Tests

**Test 7: Standalone UI generation full flow**
```python
def test_ui_standalone_full_flow():
    """Full standalone workflow: detect mode → load guidance → apply patterns"""
    # Setup: No story file, user runs `/create-ui "Login form"`
    # Execute: Phase 2 complete
    # Assert: Step 0 detected standalone mode
    # Assert: Guidance loaded
    # Assert: Step 1 applied Classification pattern (4 UI types)
    # Assert: Framework options filtered by UI type
    # Assert: Styling options presented (5 options)
    # Assert: UI specification generated with guided choices
```

**Test 8: Story-mode UI generation**
```python
def test_ui_story_mode_no_guidance():
    """Story-mode workflow: extract from AC, skip guidance"""
    # Setup: Story file loaded, user runs `/create-ui STORY-042`
    # Execute: Phase 2 complete
    # Assert: Step 0 detected story mode
    # Assert: Guidance NOT loaded
    # Assert: UI requirements extracted from story AC
    # Assert: Only missing details asked (if any)
```

**Test 9: Backward compatibility - existing tests pass**
```python
def test_backward_compat_ui_generator():
    """Verify all 15 existing UI-generator tests pass"""
    # Setup: Run full existing test suite
    # Assert: 15/15 tests pass
    # Assert: No behavior changes in story or standalone modes
```

---

## Section 7: Token Budget Analysis

### 7.1 Standalone Mode Token Cost

```
Step 0: Mode detection + conditional logic
  - Context checking (story markers): ~30 tokens
  - Log messages: ~20 tokens
  - Subtotal: ~50 tokens

Step 0: Guidance loading (if standalone)
  - Read file (600 lines, ~50KB): ~500-700 tokens
  - Pattern reference lookups: ~100-150 tokens
  - Subtotal: ~600-850 tokens

Steps 1-3: Pattern application (using loaded guidance)
  - Pattern lookups: ~50-100 tokens
  - Question generation: ~100-150 tokens
  - Subtotal: ~150-250 tokens

TOTAL per skill (standalone): 800-1,150 tokens (~1,000 avg)
```

### 7.2 Story Mode Token Cost

```
Step 0: Mode detection
  - Context checking (story markers): ~30 tokens
  - Log message (skip): ~20 tokens
  - Subtotal: ~50 tokens

Steps 1-3: Extract from story AC (no guidance)
  - Existing behavior unchanged: ~200-250 tokens
  - Subtotal: ~200-250 tokens

TOTAL per skill (story): 250-300 tokens (near zero guidance cost)
```

### 7.3 Cumulative Impact

**Single UI-generator execution:**
- Standalone: ~1,000 tokens (guidance + patterns)
- Story: ~300 tokens (no guidance overhead)
- Savings vs monolithic guidance: 150-300 tokens in story mode

---

## Section 8: Skill Integration Checklist

### 8.1 SKILL.md Modifications

- [ ] **Phase 2 Head:** Add note "Step 0: Conditional guidance loading per STORY-057"
- [ ] **Phase 2 Step 0 (NEW):** Add ~15 lines of mode detection logic
- [ ] **Phase 2 Step 1-3:** Reference user-input-guidance patterns where applicable
- [ ] **Total additions:** ~30 lines to SKILL.md (conditional + pattern references)

### 8.2 Reference File Deployment

- [ ] **Master:** user-input-guidance.md exists in ideation/references/
- [ ] **UI-Generator Copy:** user-input-guidance.md copied to ui-generator/references/
- [ ] **Operational:** user-input-guidance.md synced to .claude/skills/devforgeai-ui-generator/
- [ ] **Checksums:** All 3 copies have identical SHA256 hashes

### 8.3 Testing

- [ ] **Unit Tests:** 3 conditional loading tests (standalone, story, empty story)
- [ ] **Pattern Tests:** 3 pattern application tests (classification, bounded x2)
- [ ] **Integration Tests:** 2 full-flow tests (standalone + story-mode unchanged)
- [ ] **Regression Tests:** 15 existing UI-generator tests all passing
- [ ] **Total:** 23 UI-generator-specific tests (3 + 3 + 2 new + 15 existing)

### 8.4 Documentation

- [ ] **This Reference File:** Created and reviewed
- [ ] **Pattern Mapping Table:** Completed (Section 2.1)
- [ ] **Examples:** 3 before/after examples documented
- [ ] **Deployment Process:** Step-by-step instructions provided

---

## Section 9: Success Validation

**Standalone mode validation:**
✅ No story file in conversation
✅ Context marker check returns negative
✅ guidance_content = Read(...) executes
✅ Log: "Standalone mode detected"
✅ Phase 2 Steps 1-3 use patterns from guidance
✅ UI specification generated from guided questions

**Story mode validation:**
✅ Story file loaded (@STORY-NNN.story.md)
✅ Context marker "**Story ID:**" present
✅ Read NOT executed
✅ Log: "Story mode detected"
✅ Phase 2 extracts UI requirements from story AC
✅ Only missing details asked (if any)

**Backward compatibility validation:**
✅ All 15 existing UI-generator tests pass
✅ Story-mode output identical to pre-STORY-057
✅ Standalone-mode output improved (more guided)
✅ No breaking changes to SKILL interface
✅ Guidance is non-blocking (if file missing, continues)

---

**Version 1.0** | **Status: Published** | **Created: 2025-01-21**
