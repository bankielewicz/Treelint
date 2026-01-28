# UI Spec Formatter Integration (Step 6.3.5)

How to invoke the ui-spec-formatter subagent and process its output.

**Purpose:** Generate user-facing display template with framework validation after UI generation completes.

**When to Execute:** Phase 6 Step 3.5 - After documentation complete, after Phase 7 validation

**Subagent:** ui-spec-formatter (framework-aware display template generator)

---

## Invocation Pattern

**Invoke ui-spec-formatter subagent:**

The formatter subagent validates the generated specification against framework constraints and creates a structured display template.

```
Task(
  subagent_type="ui-spec-formatter",
  description="Format and validate UI specification results",
  prompt="Generate user-facing display template for UI generation workflow.

**Context:**
- Mode: ${MODE} (story|standalone)
- Story ID: ${STORY_ID or 'standalone'}
- UI Spec File: devforgeai/specs/ui/${SPEC_FILE_NAME}
- Framework: ${FRAMEWORK} (from tech-stack.md)
- Styling: ${STYLING_APPROACH} (user selected)
- Component Count: ${COMPONENT_COUNT}
- Generated Files: ${GENERATED_FILES_LIST}

**Generated Files List:**
${List all files created in Phase 5}

**Validation Context from Phase 7:**
${validation_context from Step 7.5}

**Requirements:**
1. Read generated UI specification file
2. Extract component details (type, framework, styling, accessibility, responsive)
3. Load framework constraints from reference file (ui-result-formatting-guide.md)
4. Validate against context files:
   - tech-stack.md: Framework consistency
   - source-tree.md: File location compliance
   - dependencies.md: Package approval
   - coding-standards.md: Component standards
   - architecture-constraints.md: Layer boundaries
   - anti-patterns.md: Forbidden patterns
5. Determine appropriate display template (SUCCESS/PARTIAL/FAILED)
6. Generate structured result with:
   - Display template (formatted markdown)
   - Component details (structured metadata)
   - Validation issues (if any)
   - Next steps (implementation guidance)

**Output Format:**
Return structured JSON following this schema:
{
  \"status\": \"SUCCESS|PARTIAL|FAILED\",
  \"display\": {
    \"template\": \"[Full formatted display text]\",
    \"mode\": \"${MODE}\",
    \"story_id\": \"${STORY_ID or null}\"
  },
  \"component_details\": {
    \"component_type\": \"Form|Data Display|Navigation|etc\",
    \"framework\": \"React|Vue|Angular|etc\",
    \"styling\": \"Tailwind|CSS Modules|etc\",
    \"accessibility\": \"WCAG 2.1 AA|AAA\",
    \"responsive\": \"Mobile-first|Desktop-first|etc\",
    \"state_management\": \"Redux|Zustand|Context API|etc\"
  },
  \"generated_files\": [\"file1.jsx\", \"file2.css\", ...],
  \"validation\": {
    \"status\": \"PASSED|WARNING|FAILED\",
    \"issues\": [\"issue1\", \"issue2\", ...],
    \"warnings\": [\"warning1\", \"warning2\", ...]
  },
  \"next_steps\": [\"step1\", \"step2\", \"step3\"]
}

**Framework Guardrails:**
Follow constraints in .claude/skills/devforgeai-ui-generator/references/ui-result-formatting-guide.md reference file.

**Validation Rules:**
- Tech-stack consistency: STRICT (no substitution)
- File structure compliance: STRICT (source-tree.md mandatory)
- Accessibility requirements: STRICT (WCAG 2.1 AA minimum)
- Component categorization: DETERMINISTIC (forms, displays, navigation, etc.)
- Display tone: POSITIVE (use ✅ for success, ⚠️ for warnings, ❌ for failures)"
)
```

---

## Capture Subagent Response

**Store result as:** formatter_result

**Expected structure:**
```json
{
  "status": "SUCCESS|PARTIAL|FAILED",
  "display": { "template": "...", ... },
  "component_details": { ... },
  "generated_files": [...],
  "validation": { "status": "...", "issues": [...], "warnings": [...] },
  "next_steps": [...]
}
```

---

## Handle Formatter Result

**Process based on status:**

```
IF formatter_result.status == "FAILED":
  HALT workflow
  Display formatter_result.display.template
  Report validation failures
  Provide recovery steps
  EXIT skill with error status

ELSE IF formatter_result.status == "PARTIAL":
  Display formatter_result.display.template (includes warnings)
  Continue to Step 4

ELSE: # SUCCESS
  Continue to Step 4 with formatter_result
```

---

## Return Formatter Results to Command

**Display template from ui-spec-formatter:**

The formatter subagent has prepared a comprehensive display template that validates framework constraints and provides next steps.

**Return to command:**
- Display: formatter_result.display.template
- Status: formatter_result.status
- Validation: formatter_result.validation
- Next Steps: formatter_result.next_steps

**Command will output:**
The /create-ui command receives this structured result and outputs the display template directly. No additional parsing, formatting, or validation needed in the command.

**If status is PARTIAL:**
- Display includes warnings about validation issues
- User can review warnings and proceed or regenerate

**If status is FAILED:**
- Workflow halted at Step 3.5
- Error details already communicated
- Recovery steps provided to user

---

## Complete Workflow

UI generation workflow complete after formatter returns.

**Results prepared:**
- ✅ UI specification generated (Phase 5)
- ✅ Documentation created (Phase 6 Steps 1-3)
- ✅ Display template formatted (ui-spec-formatter subagent, Step 3.5)
- ✅ Framework compliance validated (Phase 7)
- ✅ Story updated (if story mode)
- ✅ Summary document created

**Returning to command:**
The /create-ui command will display the formatter_result.display.template with:
- Component details
- Generated files list
- Validation status
- Next steps for implementation

**If user needs additional components:**
Run /create-ui again with new story ID or component description.

---

## Token Impact

**Subagent invocation:**
- Invocation cost: ~500 tokens (in skill context)
- Subagent execution: ~8K tokens (isolated context, not counted in main conversation)
- Result returned: ~1.5K tokens (structured JSON with display template)
- **Total skill context:** ~2K tokens for formatter integration

**Efficiency:**
- Main conversation sees only: Invocation + result (~2K total)
- Heavy validation work (8K): Happens in isolated subagent context
- Display template: Pre-generated, command just outputs it
