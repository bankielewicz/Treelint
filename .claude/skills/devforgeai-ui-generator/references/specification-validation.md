# Phase 7: Specification Validation

Comprehensive validation of generated UI specifications with user-driven issue resolution.

**Purpose:** Ensure specification is production-ready with no TODO/TBD content, all sections complete, and framework constraints validated. All issues resolved through explicit user decisions.

**Core Principle:** **"Ask, Don't Assume"** - Never auto-fix anything, even if it seems minor.

**When to Execute:** After Phase 6 Step 3 (documentation complete), before Step 3.5 (formatter invocation)

---

## Step 7.1: Specification Completeness Check

**Read generated UI specification:**

```
Read(file_path="devforgeai/specs/ui/${SPEC_FILE_NAME}")
```

**Validate required sections exist:**

Check specification includes:
- [ ] Component hierarchy defined (parent/child relationships)
- [ ] Props/API documented (types, defaults, required)
- [ ] State management approach specified
- [ ] Styling approach documented (classes, themes, responsive)
- [ ] Accessibility considerations included (WCAG level, ARIA, keyboard nav)
- [ ] Responsive behavior defined (breakpoints, mobile-first)
- [ ] Test strategy outlined (unit, integration, accessibility)
- [ ] Usage examples provided (code snippets)
- [ ] Integration instructions included
- [ ] Dependencies listed (npm packages, imports)

**Record missing sections:**
```
missing_sections = [list of missing sections]

IF missing_sections not empty:
  VALIDATION_STATUS = "INCOMPLETE"
  Proceed to Step 7.4 (user resolution)
ELSE:
  VALIDATION_STATUS = "COMPLETE"
  Proceed to Step 7.2
```

---

## Step 7.2: Placeholder Detection

**Scan for placeholder content:**

```
Grep(
  pattern="TODO|TBD|\\[FILL IN\\]|\\[TO BE DETERMINED\\]|\\[TBD\\]|\\[PLACEHOLDER\\]",
  path="devforgeai/specs/ui/${SPEC_FILE_NAME}",
  output_mode="content",
  -n=true
)
```

**Record placeholders found:**
```
placeholder_count = [count from Grep]
placeholder_list = [list with line numbers and content]

IF placeholder_count > 0:
  VALIDATION_STATUS = "HAS_PLACEHOLDERS"
  Proceed to Step 7.4 (user resolution)
ELSE:
  Proceed to Step 7.3
```

---

## Step 7.3: Framework Constraint Validation

**Validate against context files:**

**1. Tech-Stack Consistency:**
```
Read(file_path="devforgeai/specs/context/tech-stack.md")

Check specification uses approved framework:
- Extract framework from spec
- Compare to tech-stack.md approved frameworks
- If mismatch: Record as validation issue (SEVERITY: HIGH)
```

**2. File Structure Compliance:**
```
Read(file_path="devforgeai/specs/context/source-tree.md")

Check generated files follow structure:
- Extract file paths from generated_files list
- Compare to source-tree.md component location rules
- If violations: Record as validation issue (SEVERITY: MEDIUM)
```

**3. Dependency Approval:**
```
Read(file_path="devforgeai/specs/context/dependencies.md")

Check all packages are approved:
- Extract dependencies from spec
- Compare to dependencies.md approved packages
- If unapproved: Record as validation issue (SEVERITY: LOW)
```

**4. Anti-Pattern Detection:**
```
Read(file_path="devforgeai/specs/context/anti-patterns.md")

Scan specification for forbidden patterns:
- Grep for common anti-patterns in generated code
- Check for hardcoded values, prop drilling, inline styles, etc.
- If detected: Record as validation issue (SEVERITY: HIGH)
```

**Compile validation results:**
```
validation_summary = {
  "tech_stack": "PASSED|FAILED",
  "file_structure": "PASSED|FAILED",
  "dependencies": "PASSED|FAILED",
  "anti_patterns": "PASSED|FAILED",
  "issues": [
    {severity: "HIGH", category: "tech_stack", description: "Spec uses Vue but tech-stack.md specifies React"},
    {severity: "MEDIUM", category: "file_structure", description: "Component in wrong directory"},
    {severity: "LOW", category: "dependencies", description: "react-icons not in dependencies.md"}
  ]
}
```

**Determine validation status:**
```
high_severity_count = count issues with severity "HIGH"
medium_severity_count = count issues with severity "MEDIUM"
low_severity_count = count issues with severity "LOW"

IF high_severity_count > 0:
  VALIDATION_STATUS = "FAILED"
ELSE IF medium_severity_count > 0 OR low_severity_count > 0:
  VALIDATION_STATUS = "WARNING"
ELSE:
  VALIDATION_STATUS = "PASSED"
```

---

## Step 7.4: User Resolution of All Issues

**CRITICAL: Never auto-fix. Always ask user.**

### For Missing Sections

```
IF missing_sections not empty:

  AskUserQuestion(
    questions: [{
      question: "The UI specification is missing ${missing_sections.length} required section(s): ${missing_sections.join(', ')}. How should I proceed?",
      header: "Incomplete Spec",
      multiSelect: false,
      options: [
        {
          label: "Provide missing information",
          description: "I'll answer questions to complete each section"
        },
        {
          label: "Use framework defaults",
          description: "Apply DevForgeAI standard defaults with my approval"
        },
        {
          label: "Accept as-is",
          description: "Mark as PARTIAL, I'll complete manually"
        },
        {
          label: "Regenerate specification",
          description: "Start Phase 5 over with different approach"
        }
      ]
    }]
  )

  Handle response:

  IF "Provide missing information":
    FOR EACH missing section:
      Ask specific question for that section

      Example for Accessibility:
        AskUserQuestion:
          Question: "What accessibility level is required for this UI?"
          Header: "Accessibility"
          Options: [
            "WCAG 2.1 AA (standard)",
            "WCAG 2.1 AAA (strict)",
            "Custom requirements (I'll specify)"
          ]

      Example for Responsive:
        AskUserQuestion:
          Question: "What responsive design approach should be used?"
          Header: "Responsive"
          Options: [
            "Mobile-first (framework standard)",
            "Desktop-first",
            "Adaptive (different layouts per breakpoint)"
          ]

      Apply user's answers to spec via Edit tool

    Re-validate completeness

  ELSE IF "Use framework defaults":
    Display defaults being applied:
    "DevForgeAI Standard Defaults:
    - Accessibility: WCAG 2.1 AA (minimum framework standard)
    - Responsive: Mobile-first (progressive enhancement)
    - Test Strategy: Test pyramid (70% unit, 20% integration, 10% E2E)
    - State Management: ${extract from tech-stack.md}"

    AskUserQuestion:
      Question: "Apply these DevForgeAI standard defaults to specification?"
      Header: "Confirm Defaults"
      Options: [
        "Yes, apply defaults",
        "No, let me specify custom values",
        "Show what each default means"
      ]

    IF user approves:
      Edit spec to add sections with approved defaults
      Document: "Added via user-approved framework defaults"

    ELSE IF "Show what each default means":
      Display detailed explanation of each default
      Then re-ask approval question

    ELSE:
      Use "Provide missing information" flow

  ELSE IF "Accept as-is":
    Set SPEC_QUALITY = "PARTIAL"
    Record: "User accepted incomplete spec"
    Continue to formatter (will show warnings)

  ELSE: # Regenerate
    Display: "Regenerating specification with corrections..."
    Return to Phase 5 Code Generation
```

### For Placeholders (max 10 to prevent endless loops)

```
IF placeholder_count > 0:

  Display first 3 placeholders as preview:
  "Found ${placeholder_count} placeholder(s):
  1. Line ${line1}: ${text1}
  2. Line ${line2}: ${text2}
  3. Line ${line3}: ${text3}
  ${placeholder_count > 3 ? '... and ' + (placeholder_count - 3) + ' more' : ''}"

  AskUserQuestion(
    questions: [{
      question: "The UI specification has ${placeholder_count} placeholder(s). How should I proceed?",
      header: "Placeholders",
      multiSelect: false,
      options: [
        { label: "Resolve now", description: "I'll provide values for each placeholder" },
        { label: "Accept as-is", description: "Leave placeholders, I'll resolve manually" },
        { label: "Show all placeholders", description: "Display complete list before deciding" }
      ]
    }]
  )

  Handle response:

  IF "Resolve now":
    FOR EACH placeholder (max 10):
      AskUserQuestion(
        questions: [{
          question: "Line ${line_num}: '${placeholder_text}'. What should this be?",
          header: "Placeholder ${index}/${total}",
          multiSelect: false,
          options: [
            { label: "Provide value", description: "I'll specify the correct value" },
            { label: "Skip this one", description: "Leave as TODO for manual resolution" }
          ]
        }]
      )

      IF user provides value:
        Edit(file_path="devforgeai/specs/ui/${SPEC_FILE_NAME}",
             old_string="${LINE_WITH_PLACEHOLDER}",
             new_string="${LINE_WITH_USER_VALUE}")

      ELSE:
        Skip (leave placeholder, will mark as PARTIAL)

    Re-scan for remaining placeholders after edits

  ELSE IF "Accept as-is":
    Set SPEC_QUALITY = "PARTIAL"
    Record: "User accepted placeholders"
    Continue to formatter (will show warnings)

  ELSE: # Show all placeholders
    Display full Grep output with line numbers
    Then re-ask "Resolve now" vs "Accept as-is" question
```

### For Framework Violations

```
IF validation_summary has HIGH severity issues:

  Display issues summary:
  "❌ Framework Validation detected ${high_severity_count} critical issue(s):
  ${For each HIGH severity issue: display category, description}"

  AskUserQuestion(
    questions: [{
      question: "Found ${high_severity_count} critical validation issue(s). How should I proceed?",
      header: "Critical Issues",
      multiSelect: false,
      options: [
        { label: "Fix issues now", description: "I'll resolve each violation" },
        { label: "Show detailed report", description: "Display all issues with context" },
        { label: "Accept with warnings", description: "Proceed with PARTIAL status (not recommended)" },
        { label: "Regenerate specification", description: "Start over with corrected constraints" }
      ]
    }]
  )

  Handle response:

  IF "Fix issues now":
    FOR EACH HIGH severity issue:
      Ask specific resolution question

      Example for tech-stack mismatch:
        AskUserQuestion:
          Question: "Spec uses Vue but tech-stack.md specifies React. Which is correct?"
          Header: "Tech Conflict"
          Options: [
            "Use React (follow tech-stack.md)",
            "Use Vue (update tech-stack.md and create ADR)",
            "Skip (accept this violation)"
          ]

      Apply user's decision:
        IF "Use React":
          Edit spec to change framework to React
          Update all framework references

        ELSE IF "Use Vue":
          Display: "This requires updating tech-stack.md and creating ADR"
          Ask: "Update tech-stack.md now or handle manually?"
          If update: Edit tech-stack.md, create ADR stub

        ELSE: # Skip
          Record as accepted violation

    Re-validate against context files after fixes

  ELSE IF "Show detailed report":
    Display full validation_summary with line numbers and context
    Then re-ask the original question

  ELSE IF "Accept with warnings":
    Display warning:
    "⚠️ Proceeding with critical violations is not recommended.
    This may cause issues during implementation (/dev) or QA (/qa)."

    Confirm:
    AskUserQuestion:
      Question: "Confirm: Accept critical violations and mark as PARTIAL?"
      Options: ["Yes, I understand the risks", "No, let me fix them"]

    IF confirmed:
      Set SPEC_QUALITY = "PARTIAL"
      Record: "User accepted HIGH severity violations"
    ELSE:
      Return to "Fix issues now" flow

  ELSE: # Regenerate
    Display: "Returning to Phase 5 Code Generation with corrections..."
    Return to Phase 5 with corrected constraints

ELSE IF validation_summary has MEDIUM or LOW severity issues:

  Display warnings summary:
  "⚠️ Validation detected ${medium_severity_count + low_severity_count} warning(s):
  ${For each issue: display severity, category, description}"

  AskUserQuestion(
    questions: [{
      question: "These warnings won't block UI generation but should be addressed. Proceed?",
      header: "Warnings",
      multiSelect: false,
      options: [
        { label: "Proceed with warnings", description: "Mark as PARTIAL and continue" },
        { label: "Fix warnings now", description: "Resolve each warning" },
        { label: "Show details", description: "Display full validation report" }
      ]
    }]
  )

  IF "Proceed with warnings":
    Set SPEC_QUALITY = "PARTIAL"
    Continue to formatter

  ELSE IF "Fix warnings now":
    [Use same resolution flow as HIGH severity issues]

  ELSE: # Show details
    Display detailed validation_summary
    Re-ask the question
```

---

## Step 7.5: Prepare Validation Context for Formatter

**Compile validation context (all decisions user-made):**

```
validation_context = {
  "spec_file": "devforgeai/specs/ui/${SPEC_FILE_NAME}",
  "spec_quality": "${SPEC_QUALITY}",
  "validation_summary": validation_summary,
  "user_decisions": [
    # Document all user decisions made in Step 7.4
    "Accepted placeholder at line 42 (color scheme - user will specify later)",
    "Resolved tech-stack conflict: Chose React over Vue",
    "Applied framework defaults with approval (accessibility: WCAG 2.1 AA, responsive: mobile-first)"
  ],
  "remaining_issues": {
    "missing_sections": missing_sections,
    "unresolved_placeholders": [placeholders user chose to skip],
    "accepted_violations": [violations user accepted]
  },
  "framework_compliance": {
    "tech_stack": validation_summary.tech_stack,
    "file_structure": validation_summary.file_structure,
    "dependencies": validation_summary.dependencies,
    "anti_patterns": validation_summary.anti_patterns
  }
}
```

**Quality gate before formatter:**

```
IF SPEC_QUALITY == "FAILED":
  # User declined to fix critical violations
  HALT workflow immediately

  Display error:
  "❌ UI Specification Validation FAILED

  Critical Issues (user declined to fix):
  ${List HIGH severity issues}

  Cannot proceed with specification containing unresolved critical violations.

  Recovery Options:
  - Regenerate specification: Return to Phase 5
  - Fix issues manually: Edit devforgeai/specs/ui/${SPEC_FILE_NAME}
  - Run /create-ui again after corrections"

  EXIT skill with error status

ELSE IF SPEC_QUALITY == "PARTIAL":
  # Has warnings, placeholders, or user-accepted issues
  Display notice:
  "⚠️ Specification marked as PARTIAL

  User Decisions:
  ${List user_decisions}

  Remaining Issues:
  ${List remaining_issues}

  Proceeding to formatter (will include warnings in display)"

  Continue to Phase 6 Step 3.5 (formatter invocation)

ELSE: # PASSED
  # All validations passed, no issues
  Display:
  "✅ Specification validation PASSED

  All framework constraints satisfied.
  No placeholders or missing sections.

  Proceeding to formatter..."

  Continue to Phase 6 Step 3.5 (formatter invocation)
```

**Pass validation context to formatter:**

Include validation_context in formatter prompt (Phase 6 Step 3.5) so formatter knows:
- What user decided (accepted warnings, resolved placeholders, etc.)
- What issues remain (for PARTIAL display)
- What quality level to report (SUCCESS/PARTIAL/FAILED)

**Output:** All issues resolved through user decisions, validation context prepared, quality status determined, ready for formatter invocation.

**Token Impact:**
- Step 7.1: ~1K tokens (read spec, check sections)
- Step 7.2: ~500 tokens (grep placeholders)
- Step 7.3: ~3K tokens (read 4 context files, validate)
- Step 7.4: Variable (2-10K tokens depending on issue count and user interaction)
- Step 7.5: ~500 tokens (compile context)
- **Total Phase 7:** ~7-15K tokens (user interaction-heavy, acceptable for comprehensive validation)

**Note:** Phase 7 executes BEFORE Phase 6 Step 3.5 in the actual workflow sequence. Validation happens first, then formatter is invoked with validation results.
