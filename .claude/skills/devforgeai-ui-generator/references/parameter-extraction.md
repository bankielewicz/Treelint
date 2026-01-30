# Parameter Extraction from Conversation Context

How UI generator skill extracts parameters (story ID or component description) from conversation.

**CRITICAL:** Skills CANNOT accept runtime parameters. All information must be extracted from conversation context.

---

## How Slash Commands Pass "Parameters" to Skills

When `/create-ui` command invokes this skill, it provides context in two modes:

**Story Mode:**
1. Loads story file via @file reference: `@devforgeai/specs/Stories/STORY-XXX.story.md`
2. States: "Story ID: STORY-XXX" and "Mode: story"
3. Invokes: `Skill(command="devforgeai-ui-generator")`

**Standalone Mode:**
1. States: "Component description: Login form with validation" and "Mode: standalone"
2. Invokes: `Skill(command="devforgeai-ui-generator")`

**You must detect which mode and extract appropriate parameters from conversation.**

---

## Two Operating Modes

### Story Mode
- **Triggered by:** Loaded story file or `**Story ID:** STORY-XXX` marker
- **Parameter:** Story ID extracted from YAML frontmatter
- **Use case:** Generate UI from existing story requirements

### Standalone Mode
- **Triggered by:** Component description in conversation
- **Parameter:** Description extracted from user message
- **Use case:** Generate UI without story context

---

## Mode Detection Algorithm

**Check conversation for indicators:**
```
IF conversation contains YAML frontmatter with "id: STORY-XXX":
  MODE = "story"
  Extract story ID from frontmatter

ELSE IF conversation contains "devforgeai/specs/Stories/STORY-XXX":
  MODE = "story"
  Extract story ID from file path

ELSE IF conversation contains "Component description:":
  MODE = "standalone"
  Extract description from statement

ELSE IF conversation contains "Mode: story":
  MODE = "story"
  Search for story ID in conversation

ELSE IF conversation contains "Mode: standalone":
  MODE = "standalone"
  Search for component description

ELSE:
  # Cannot determine - ask user
  AskUserQuestion:
  Question: "Should I generate UI for a story or standalone component?"
  Header: "UI Generation Mode"
  Options:
    - "Story-based (use acceptance criteria from story)"
    - "Standalone (I'll describe the component)"
  multiSelect: false
```

---

## Story ID Extraction (Story Mode)

**Methods to extract story ID:**

1. **From YAML frontmatter** (most reliable):
   ```
   Search conversation for:
   ---
   id: STORY-XXX
   ---

   Extract: STORY-XXX
   ```

2. **From @file reference:**
   ```
   Search conversation for:
   @devforgeai/specs/Stories/STORY-XXX-title.story.md

   Extract: STORY-XXX
   ```

3. **From explicit statement:**
   ```
   Search conversation for:
   **Story ID:** STORY-XXX

   Extract: STORY-XXX
   ```

---

## Component Description Extraction (Standalone Mode)

**Look for description in conversation:**

```
Search for patterns:
  - "Component description: [text]" → DESCRIPTION = [text]
  - "Create UI for: [text]" → DESCRIPTION = [text]
  - User's original argument captured in conversation
```

**Example extractions:**
- "Component description: Login form with email and password" → "Login form with email and password"
- "Create UI for: Dashboard with charts and KPIs" → "Dashboard with charts and KPIs"

---

## Validation Before Proceeding

Before starting UI generation, verify:
- [ ] Mode determined (story or standalone)
- [ ] If story mode: Story ID extracted, story content available
- [ ] If standalone mode: Component description extracted
- [ ] Ready to proceed with UI generation phases

**If extraction fails:**
```
HALT with error:
"Cannot determine UI generation mode from conversation context.

Expected to find:
  - Story mode: Story file loaded OR 'Story ID: STORY-XXX' stated
  - Standalone mode: 'Component description: [text]' stated

Please ensure context is provided via slash command or state explicitly."
```

---

## Example Conversation Contexts

### Example 1: Story Mode (via @file)
```
@devforgeai/specs/Stories/STORY-042-user-login.story.md
Skill(command="devforgeai-ui-generator")

Detection:
- Found: @file reference with STORY-042
- Mode: story
- Story ID: STORY-042
```

### Example 2: Story Mode (via explicit statement)
```
**Story ID:** STORY-042
**Mode:** story
Skill(command="devforgeai-ui-generator")

Detection:
- Found: Explicit "Story ID: STORY-042"
- Mode: story
- Story ID: STORY-042
```

### Example 3: Standalone Mode
```
**Component description:** Login form with email and password validation
**Mode:** standalone
Skill(command="devforgeai-ui-generator")

Detection:
- Found: "Component description: ..."
- Mode: standalone
- Description: "Login form with email and password validation"
```

### Example 4: Ambiguous (triggers AskUserQuestion)
```
Skill(command="devforgeai-ui-generator")

Detection:
- No story file loaded
- No "Story ID:" or "Component description:" found
- Mode: UNKNOWN
- Action: Ask user to clarify (story vs standalone)
```
