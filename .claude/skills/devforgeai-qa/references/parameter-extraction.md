# Parameter Extraction from Conversation Context

How the QA skill extracts story ID and validation mode from conversation when invoked.

---

## Overview

**IMPORTANT:** Skills CANNOT accept runtime parameters like `--mode=deep`. All information must be extracted from conversation context.

### How Slash Commands Pass "Parameters" to Skills

When `/qa` command invokes this skill, it:
1. Loads story file via @file reference: `@devforgeai/specs/Stories/STORY-XXX.story.md`
2. States context explicitly: "Story ID: STORY-XXX" and "Validation mode: deep"
3. Invokes skill WITHOUT arguments: `Skill(command="devforgeai-qa")`

**You must extract story ID and mode from the conversation.**

---

## Story ID Extraction

### Method 1: Read YAML Frontmatter

```
Look for YAML frontmatter in conversation:
  ---
  id: STORY-XXX
  title: ...
  status: ...
  ---

Extract: id field = Story ID
```

### Method 2: Search for File Reference

```
Search conversation for pattern:
  "devforgeai/specs/Stories/STORY-XXX"

Extract STORY-XXX from file path
```

### Method 3: Search for Explicit Statement

```
Search conversation for:
  "Story ID: STORY-XXX"
  "Story: STORY-XXX"

Extract STORY-XXX
```

---

## Validation Mode Extraction

### Look for Mode in Conversation

```
Search for patterns:
  - "Validation mode: deep" → MODE = deep
  - "Validation mode: light" → MODE = light
  - "Mode: deep" → MODE = deep
  - "Mode: light" → MODE = light
```

### If Not Found - Infer from Story Status

```
Check story status from YAML frontmatter:
  - status: "Dev Complete" → MODE = deep (comprehensive validation before approval)
  - status: "In Development" → MODE = light (quick checks during development)
  - Other status → MODE = deep (default to thorough validation)
```

**Default:** deep (if unable to determine from context)

---

## Validation Before Proceeding

Before starting QA validation, verify:
- [ ] Story ID extracted successfully
- [ ] Story content available in conversation
- [ ] Validation mode determined (light or deep)
- [ ] Ready to proceed with QA phases

### If Extraction Fails

```
HALT with error:
"Cannot extract required parameters from conversation context.

Expected to find:
  - Story ID: YAML frontmatter with 'id: STORY-XXX' OR file reference
  - Validation mode: 'Validation mode: deep' OR inferred from story status

Please ensure story is loaded via slash command or provide parameters explicitly."
```

---

## Extraction Success Indicators

When extraction succeeds, display:

```
✓ Story ID: STORY-XXX (extracted from {source})
✓ Validation mode: {deep/light} (extracted from {source})
✓ Proceeding with QA validation...
```

**Sources:**
- "YAML frontmatter"
- "file reference"
- "explicit statement"
- "inferred from story status"

---

**This extraction happens in every skill invocation before workflow execution begins.**
