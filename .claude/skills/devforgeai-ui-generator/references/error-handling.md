# Error Handling & Recovery Procedures

Error handling for UI generation workflow.

---

## Error 1: Context Files Missing

**Detection:** Phase 1 validation fails

**Error message:**
```
"❌ Context files are missing. Cannot proceed with UI generation.

Missing files: ${missing_files}

Recovery:
Please invoke the devforgeai-architecture skill first to create the required context files:

Skill(command=\"devforgeai-architecture\")

Or use the slash command:
/create-context ${project_name}

After context files are created, run /create-ui again."
```

**Recovery:** User creates context files, then re-runs /create-ui

**EXIT:** Skill halts immediately

---

## Error 2: Story File Not Found (Story Mode)

**Detection:** Cannot load story file in Phase 2

**Error message:**
```
"❌ Story file not found for ${STORY_ID}

Attempted: devforgeai/specs/Stories/${STORY_ID}.story.md"
```

**Recovery:**
```
Use Glob to search:
Glob(pattern="devforgeai/specs/Stories/${STORY_ID}*.story.md")

IF files found:
  Use first match
  Display: "Found: ${file_path}"
  Continue with that file

ELSE:
  AskUserQuestion:
    Question: "Story ${STORY_ID} not found. How should I proceed?"
    Options: [
      "Continue in standalone mode (describe component manually)",
      "Cancel UI generation"
    ]

  IF "Continue in standalone mode":
    Switch to standalone mode
    Proceed to Phase 3

  ELSE:
    EXIT skill
```

---

## Error 3: Technology Not in tech-stack.md

**Detection:** User selects unapproved technology in Phase 3

**Error message:**
```
"⚠️ Technology conflict detected:
Selected: ${CHOSEN_TECH}
Approved: ${APPROVED_TECH} (per tech-stack.md)"
```

**Recovery:**
```
AskUserQuestion:
  Question: "You selected ${CHOSEN_TECH}, but tech-stack.md specifies ${APPROVED_TECH}. Which should be used?"
  Header: "Tech Conflict"
  Options: [
    "Use ${CHOSEN_TECH} (update tech-stack.md and create ADR)",
    "Use ${APPROVED_TECH} (follow existing standard)"
  ]

Handle user decision:
- Update tech-stack.md if requested
- Create ADR stub if technology changed
- Proceed with selected technology
```

**EXIT:** Never (user resolves conflict, workflow continues)

---

## Error 4: Template Loading Fails

**Detection:** Template file missing from assets/ in Phase 4

**Error message:**
```
"❌ Template file not found: ${template_path}

Expected location: .claude/skills/devforgeai-ui-generator/assets/${template_file}"
```

**Recovery:**
```
Validate assets directory:
Glob(pattern=".claude/skills/devforgeai-ui-generator/assets/*")

Display available templates
Ask user to select from available templates

OR

Display error:
"Cannot proceed without template. Available options:
1. Add missing template to assets/
2. Select different technology with available template
3. Cancel UI generation"

AskUserQuestion for resolution
```

**EXIT:** If user cancels, otherwise continue with corrected template

---

## Error 5: Code Generation Incomplete

**Detection:** Generated code has placeholders in Phase 7 Step 7.2

**Error message:**
```
"⚠️ Found ${placeholder_count} placeholder(s) in generated code:
${List first 3 placeholders}"
```

**Recovery:**
```
AskUserQuestion:
  Question: "How should I proceed with placeholders?"
  Options: [
    "Resolve now (I'll provide values)",
    "Accept as-is (I'll resolve manually)",
    "Show all placeholders"
  ]

User resolves placeholders via AskUserQuestion loop
Spec marked as PARTIAL if user accepts placeholders
```

**EXIT:** Never (user decides, workflow completes with appropriate status)

---

## Error 6: Validation Failures (Phase 7)

**Detection:** Phase 7 Steps 7.1-7.3 detect issues

**Error types:**
- Missing sections (completeness check)
- Placeholders (TODO/TBD)
- Framework violations (HIGH severity)
- Warnings (MEDIUM/LOW severity)

**Recovery:**
```
User resolution via AskUserQuestion (see specification-validation.md Step 7.4)

Options presented:
- Fix issues now
- Use framework defaults
- Accept as-is (mark as PARTIAL)
- Regenerate specification

User makes decision, skill follows user's choice
```

**EXIT:** If user declines to fix CRITICAL issues and declines PARTIAL status

---

## Error 7: Write Permission Errors

**Detection:** Cannot write file in Phase 5 or Phase 6

**Error message:**
```
"❌ Permission denied writing to: ${file_path}

This may be due to:
- Directory doesn't exist
- Insufficient write permissions
- File locked by another process"
```

**Recovery:**
```
AskUserQuestion:
  Question: "Cannot write to ${file_path}. How should I proceed?"
  Options: [
    "Try alternative location",
    "Check permissions and retry",
    "Cancel UI generation"
  ]

IF "Try alternative location":
  Ask user for alternative path
  Validate path exists and is writable
  Retry write operation

ELSE IF "Check permissions and retry":
  Wait for user to fix permissions
  Retry write operation

ELSE:
  EXIT skill
```

---

## General Recovery Strategy

**For all errors:**

1. **Detect issue** in appropriate phase
2. **Provide clear error message** with context
3. **Offer recovery options** via AskUserQuestion
4. **Do NOT auto-correct** (user authority principle)
5. **HALT if critical issue** cannot be resolved
6. **Document decision** if user accepts issue
7. **Re-validate** after user resolution

**Error priority:**
- **CRITICAL:** Context files missing, template missing → HALT immediately
- **HIGH:** Framework violations, missing required sections → User must resolve or accept PARTIAL
- **MEDIUM:** Warnings, non-critical issues → User can accept and proceed
- **LOW:** Informational, suggestions → Display but don't block

**User always has final authority.**
