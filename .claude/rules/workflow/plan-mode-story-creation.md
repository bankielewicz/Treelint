# Plan Mode Story Creation Verification

**Purpose:** Ensure ground truth verification when creating stories from plan specifications

**Trigger:** Before creating ANY story file from plan specifications

**Reference:** RCA-028 (Manual Story Creation Ground Truth Validation Failure)

---

## Pre-Flight Verification (MANDATORY)

Before creating ANY story file from plan specifications, execute these verification steps:

### 1. Verify Target Files Exist

```
FOR each file in story.files_to_modify:
  Read(file_path=file)

  IF file doesn't exist (Read fails):
    HALT: "Target file not found - cannot create story with invalid reference"

    Display error:
      File: {file}
      Story: {story_id}
      Resolution: Fix path, remove reference, or create file
```

### 2. Verify Test Paths Against source-tree.md

```
Read(file_path="devforgeai/specs/context/source-tree.md")

FOR each test_path in story.test_files:
  IF test_path pattern not found in source_tree.md:
    HALT: "Test path not in source-tree.md - invalid test location"

    Display error:
      Path: {test_path}
      Story: {story_id}
      Resolution: Use documented pattern or update source-tree.md (requires ADR)
```

### 3. Check Git Status for Deleted Files

```
Review git status header in conversation context

Extract deleted files: lines matching "^D " or "^ D " pattern
# Note: "^D " = staged delete, "^ D " = unstaged delete

FOR each referenced_file in story:
  IF referenced_file in deleted_files:
    HALT: "Referenced file is deleted - cannot reference deleted files"

    Display error:
      File: {referenced_file}
      Git Status: D (deleted)
      Story: {story_id}
      Resolution: Use existing file, recreate file, or remove reference
```

---

## HALT Trigger Summary

**If ANY verification fails:**
- Do NOT create story file
- Display specific error message
- Suggest using `/create-story` skill instead (has built-in validation)

**Success Criteria:**
- All target files exist and readable
- All test paths match source-tree.md patterns
- No references to deleted files

---

## Reference

- **RCA-028:** `devforgeai/RCA/RCA-028-manual-story-creation-ground-truth-validation-failure.md`
- **citation-requirements.md:** `.claude/rules/core/citation-requirements.md` (Read-Quote-Cite-Verify protocol)
- **source-tree.md:** `devforgeai/specs/context/source-tree.md` (Canonical test path patterns)
- **create-story skill:** `.claude/skills/devforgeai-story-creation/SKILL.md` (Built-in validation)

---

**Version:** 1.0 | **Created:** 2026-01-26 | **Source:** RCA-028 REC-2
