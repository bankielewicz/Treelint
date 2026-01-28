---
name: devforgeai-rca
description: Performs Root Cause Analysis (RCA) with 5 Whys methodology for DevForgeAI framework breakdowns. Use when users report process failures, workflow violations, or unexpected behavior in skills, commands, or subagents. Automatically collects evidence, generates recommendations, and creates RCA documents.
model: claude-opus-4-5-20251101
---

# devforgeai-rca Skill

**Purpose:** Systematic Root Cause Analysis for DevForgeAI framework breakdowns using 5 Whys methodology

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

**Proceed to Phase 0 below and begin execution.**

---

## When to Use This Skill

Invoke this skill when:
- **User reports framework breakdown** - Process didn't work as expected
- **Workflow violations** - Skill/command didn't follow intended workflow
- **Quality gate bypass** - Story progressed without meeting criteria
- **Context file violations** - Constraints ignored or not enforced
- **Workflow state errors** - Invalid state transitions
- **Unexpected behavior** - Framework components behaving incorrectly
- **User says:** "Perform RCA: [issue description]"

**Trigger pattern:** User provides issue description (explicit or via conversation context)

---

## Skill Workflow

This skill executes 8 sequential phases to perform comprehensive root cause analysis:

### Phase 0: Issue Clarification

**Objective:** Extract complete issue information or gather via AskUserQuestion

**Step 1: Extract Issue Description**

Search conversation for:
- **Issue Description:** {text} marker
- Recent user message describing breakdown
- Explicit "Perform RCA:" statement

**Step 2: Extract Severity**

Search conversation for:
- **Severity:** {CRITICAL|HIGH|MEDIUM|LOW} marker
- Infer from issue description keywords:
  - "broken", "blocking", "data loss" → CRITICAL
  - "failed", "violation", "bypass" → HIGH
  - "improvement", "gap", "unclear" → MEDIUM
  - "minor", "cosmetic", "typo" → LOW

**Step 3: Extract Affected Component**

Search conversation or issue description for:
- Skill mentioned: "devforgeai-{name}" → Skill breakdown
- Command mentioned: "/{command}" → Command breakdown
- Subagent mentioned: "{subagent} subagent" → Subagent breakdown
- Context file mentioned: "{file}.md constraint" → Context violation
- Workflow mentioned: "story state", "quality gate" → Workflow error

**Step 4: Gather Missing Information**

IF issue description incomplete:
```
AskUserQuestion:
  Question: "To perform root cause analysis, I need complete issue information. Please provide:"
  Header: "RCA Details"
  Options:
    - "What was the expected behavior?"
    - "What actually happened?"
    - "Which component was affected (skill/command/subagent)?"
    - "When did this occur (story ID, phase, command invocation)?"
  multiSelect: true

Extract: Comprehensive issue details from responses
```

**Step 5: Generate RCA Number**

```
Glob(pattern="devforgeai/RCA/RCA-*.md")

Extract numbers from filenames: RCA-001 → 1, RCA-009 → 9

Find: highest_number = max(extracted_numbers)

Generate: rca_number = highest_number + 1
Format: RCA-{rca_number:03d} (e.g., RCA-010)
```

**Step 6: Generate RCA Title**

```
Extract keywords from issue description
Format: {Component} {Problem Type} {Brief Description}
Length: 3-6 words
Example: "Context File Validation Missing"
```

**Phase 0 Output:**
```
✓ Issue Description: {description}
✓ Severity: {CRITICAL/HIGH/MEDIUM/LOW}
✓ Affected Component: {component}
✓ RCA Number: RCA-{NNN}
✓ RCA Title: {title}

Proceeding to Phase 1: Auto-Read Relevant Files...
```

---

### Phase 1: Auto-Read Relevant Files

**Objective:** Automatically read framework files relevant to the breakdown

**Step 1: Load Framework Integration Guide**

```
Read(file_path=".claude/skills/devforgeai-rca/references/framework-integration-points.md")

Use: "Evidence Location by Breakdown Type" section to determine what to read
```

**Step 2: Determine Component Type**

Based on affected component:
- **Skill:** Component name contains "devforgeai-" or is in skills list
- **Command:** Component name starts with "/" or is in commands list
- **Subagent:** Component name ends with "subagent" or is in agents list
- **Context File:** Issue mentions constraint violation or specific .md file
- **Workflow:** Issue mentions state, gate, or transition

**Step 3: Read Primary Files**

```
IF component_type == "Skill":
  Read(file_path=".claude/skills/{skill}/SKILL.md")

IF component_type == "Command":
  Read(file_path=".claude/commands/{command}.md")

IF component_type == "Subagent":
  Read(file_path=".claude/agents/{subagent}.md")

IF component_type == "Context File":
  Read(file_path="devforgeai/specs/context/{context-file}.md")

IF component_type == "Workflow":
  Read(file_path="devforgeai/specs/Stories/{STORY-ID}.story.md")
  # Extract STORY-ID from issue description if present
```

**Step 4: Read Secondary Files**

Based on component type, read related files:

**For Skill:**
```
# Check if skill invokes subagents
Grep(pattern="Task\(.*subagent_type", path=".claude/skills/{skill}/SKILL.md", output_mode="content")

FOR each subagent mentioned:
  Read(file_path=".claude/agents/{subagent}.md")

# Check command that invokes this skill
Grep(pattern="Skill\(command=\"{skill}", path=".claude/commands/", output_mode="files_with_matches")

Read(file_path="{found command}")
```

**For Command:**
```
# Read skill invoked by command
Extract skill name from Skill(command="...") line
Read(file_path=".claude/skills/{skill}/SKILL.md")

# Read pattern documentation
Read(file_path="devforgeai/protocols/lean-orchestration-pattern.md")
```

**For Subagent:**
```
# Find skills that invoke this subagent
Grep(pattern="subagent_type=\"{subagent}", path=".claude/skills/", output_mode="files_with_matches")

Read(file_path="{found skill}/SKILL.md")

# Check for reference file
Glob(pattern=".claude/skills/*/references/*{subagent}*.md")
IF found:
  Read(file_path="{found reference}")
```

**Step 5: Read Context Files (If Relevant)**

IF issue involves constraint violation:
```
Glob(pattern="devforgeai/specs/context/*.md")

Read all 6 context files:
- tech-stack.md
- source-tree.md
- dependencies.md
- coding-standards.md
- architecture-constraints.md
- anti-patterns.md
```

**Step 6: Search for Related RCAs**

```
Grep(pattern="{keywords from issue}", path="devforgeai/RCA/", output_mode="files_with_matches")

Read(file_path="{related RCA files}")

Store: related_rcas[] for "Related RCAs" section
```

**Step 7: Store File Metadata**

For each file read:
```
files_examined.append({
  path: {absolute path},
  lines_read: {range or "all"},
  relevant_sections: [{section_name, line_range}],
  excerpts: [{lines, text, significance}]
})
```

**Phase 1 Output:**
```
✓ Files Read: {count}
  - Primary: {skill/command/subagent}
  - Secondary: {related components}
  - Context: {context files if applicable}
  - Related RCAs: {count}

✓ Evidence collected: {count} relevant excerpts

Proceeding to Phase 2: 5 Whys Analysis...
```

---

### Phase 2: 5 Whys Analysis

**Objective:** Perform systematic questioning to identify root cause

**Step 1: Load 5 Whys Methodology**

```
Read(file_path=".claude/skills/devforgeai-rca/references/5-whys-methodology.md")

Use: "How to Ask Effective Why Questions" section
```

**Step 2: Load 5 Whys Template**

```
Read(file_path=".claude/skills/devforgeai-rca/assets/5-whys-template.md")

Use: Template structure for analysis
```

**Step 3: Perform Why #1 (Surface Level)**

```
Question: "Why did {issue from issue_description} happen?"

Search files read in Phase 1 for:
- Immediate observable cause
- Direct trigger
- First-level explanation

Answer: {Immediate cause based on evidence}

Evidence: {File path}:{line range} - {quote showing immediate cause}

Record: why_1 = {question, answer, evidence}
```

**Step 4: Perform Why #2 (First Layer Deeper)**

```
Question: "Why did {answer from Why #1} occur?"

Search files for:
- Why that immediate cause happened
- What allowed the trigger
- Missing mechanism or broken logic

Answer: {Deeper cause based on evidence}

Evidence: {File path}:{line range} - {quote showing deeper cause}

Record: why_2 = {question, answer, evidence}
```

**Step 5: Perform Why #3 (Second Layer Deeper)**

```
Question: "Why did {answer from Why #2} occur?"

Search files for:
- Why the mechanism was missing or broken
- Assumption that was made
- Design decision that led to this

Answer: {Even deeper cause based on evidence}

Evidence: {File path}:{line range} - {quote showing assumption/decision}

Record: why_3 = {question, answer, evidence}
```

**Step 6: Perform Why #4 (Third Layer Deeper)**

```
Question: "Why did {answer from Why #3} occur?"

Search files for:
- Why that assumption was made
- Process gap that allowed this
- Missing validation or review

Answer: {Root cause emerging}

Evidence: {File path}:{line range} - {quote showing process gap}

Record: why_4 = {question, answer, evidence}
```

**Step 7: Perform Why #5 (ROOT CAUSE)**

```
Question: "Why did {answer from Why #4} occur?"

Search files for:
- Fundamental framework/process/architecture issue
- Systemic gap that allowed all previous causes
- Fixable root cause within framework control

Answer: **ROOT CAUSE:** {Fundamental underlying issue}

Evidence: {File path}:{line range} - {quote showing root cause}

Record: why_5 = {question, answer, evidence, root_cause: true}
```

**Step 8: Validate Root Cause**

Apply validation from 5-whys-methodology.md:

**Q1: Would fixing this prevent recurrence?**
- Search evidence for proof that fix would work
- Check similar patterns in other RCAs

**Q2: Does this explain all symptoms?**
- Review issue description symptoms
- Verify root cause explains each symptom

**Q3: Is this within framework control?**
- Not external dependency
- Not user error
- Framework can fix this

**Q4: Is this evidence-based?**
- Not assumption
- Backed by file examination
- Provable from evidence

IF any validation fails:
```
HALT Phase 2

Display:
"⚠️ Root cause validation failed:
- {Which validation question failed}
- {Why it failed}

Need to revise analysis. Reconsidering Why #5..."

Revise why_5 with stronger evidence or different root cause
Re-validate
```

**Phase 2 Output:**
```
✓ 5 Whys Analysis Complete:
  Why #1: {answer 1}
  Why #2: {answer 2}
  Why #3: {answer 3}
  Why #4: {answer 4}
  Why #5 (ROOT CAUSE): {root cause}

✓ Root Cause Validated:
  - Would fixing prevent recurrence? ✓
  - Explains all symptoms? ✓
  - Within framework control? ✓
  - Evidence-based? ✓

Proceeding to Phase 3: Evidence Collection...
```

---

### Phase 3: Evidence Collection

**Objective:** Organize evidence into comprehensive, well-structured section

**Step 1: Load Evidence Collection Guide**

```
Read(file_path=".claude/skills/devforgeai-rca/references/evidence-collection-guide.md")

Use: "Evidence Organization" section
```

**Step 2: Load Evidence Template**

```
Read(file_path=".claude/skills/devforgeai-rca/assets/evidence-section-template.md")

Use: Template structure
```

**Step 3: Organize Files Examined**

From Phase 1 files_examined[]:

```
FOR each file in files_examined:
  Determine significance: CRITICAL | HIGH | MEDIUM | LOW

  IF significance >= MEDIUM:
    Extract relevant excerpts
    Format per template:
      - Lines examined
      - Finding
      - Excerpt (10-30 lines with context)
      - Significance explanation

Sort files by significance (CRITICAL first)
```

**Step 4: Validate Context Files**

```
Glob(pattern="devforgeai/specs/context/*.md")

EXPECTED = [
  "tech-stack.md",
  "source-tree.md",
  "dependencies.md",
  "coding-standards.md",
  "architecture-constraints.md",
  "anti-patterns.md"
]

FOR each expected_file:
  IF exists:
    status = "EXISTS"
    IF issue involves this constraint:
      Read file
      Check if constraint violated
      status = "PASS" or "FAIL: {violation}"
  ELSE:
    status = "MISSING"

Create context_files_status section
```

**Step 5: Analyze Workflow State**

IF issue involves workflow/story:
```
From story file YAML frontmatter:
  actual_state = status field

From workflow documentation:
  expected_state = what state should be based on criteria

From story Workflow History:
  recent_transitions = last 3-5 transitions

Analyze:
  discrepancy = expected vs actual
  missing_transition = expected transition not recorded
  invalid_transition = transition violated state-transitions.md
```

**Step 6: Check Sufficiency**

Use sufficiency criteria from evidence-collection-guide.md:

```
Verify:
- [ ] All 5 Whys have supporting evidence
- [ ] Root cause clearly demonstrated
- [ ] All files mentioned in 5 Whys examined
- [ ] At least 3 pieces of CRITICAL or HIGH evidence
- [ ] Can answer: "Where exactly do we fix this?"

IF insufficient:
  Collect additional evidence (re-read files with focus on gaps)
```

**Phase 3 Output:**
```
✓ Evidence Organized:
  - Files examined: {count}
  - Excerpts captured: {count}
  - Context files validated: {PASS/FAIL counts}
  - Workflow state analyzed: {if applicable}

✓ Evidence Sufficiency: PASS

Proceeding to Phase 4: Recommendation Generation...
```

---

### Phase 4: Recommendation Generation

**Objective:** Generate actionable, prioritized recommendations with exact implementation

**Step 1: Load Recommendation Framework**

```
Read(file_path=".claude/skills/devforgeai-rca/references/recommendation-framework.md")

Use: Priority criteria, structure requirements, implementation details
```

**Step 2: Load Recommendation Template**

```
Read(file_path=".claude/skills/devforgeai-rca/assets/recommendation-template.md")

Use: Template structure for each recommendation
```

**Step 3: Identify Solutions**

From 5 Whys analysis:
```
root_cause = why_5.answer
contributing_factors = [why_4.answer, why_3.answer] (if significant)

FOR each cause (root + contributing):
  Identify solution that fixes cause:
    - Add missing validation
    - Fix broken logic
    - Update documentation
    - Create new component
    - Refactor existing component
```

**Step 4: Categorize Priority**

Use priority criteria from recommendation-framework.md:

```
FOR each solution:
  Apply priority decision tree:

    Does fix prevent CRITICAL framework failure?
    ├─ YES → priority = CRITICAL
    └─ NO → Does fix prevent quality degradation?
              ├─ YES → priority = HIGH
              └─ NO → Does fix improve UX or quality?
                        ├─ YES → priority = MEDIUM
                        └─ NO → priority = LOW
```

**Step 5: Specify Exact Implementation**

```
FOR each solution:
  FROM evidence in Phase 3:
    Extract: exact_file_path
    Extract: exact_section (Phase X, Step Y, Lines Z-W)
    Identify: change_type (Add | Modify | Delete)

  IF change_type == "Add":
    Write: exact_text_to_add (copy-paste ready)
    Specify: insertion_point (after Step X)

  IF change_type == "Modify":
    Write: old_text (current incorrect text)
    Write: new_text (corrected text)

  IF change_type == "Delete":
    Specify: lines_to_remove
    Explain: why deletion fixes issue
```

**Step 6: Write Rationale**

Use rationale guidelines from recommendation-framework.md:

```
FOR each recommendation:
  Write rationale addressing:
    1. Why this solution? (mechanism)
    2. How does it prevent recurrence?
    3. What evidence supports this? (reference Phase 3)
    4. What are trade-offs? (if any)

  Include:
    - Evidence references (file:line)
    - Prevention explanation
    - Pattern references (if similar fix exists)
    - Trade-off acknowledgment
```

**Step 7: Define Testing Procedures**

```
FOR each recommendation:
  Write testing procedure:
    1. Setup/precondition (create test scenario)
    2. Execute action (run command, invoke skill)
    3. Verify result (check expected outcome)
    4. Additional verification (edge cases)

  Specify:
    - Expected outcome (clear success statement)
    - Success criteria (3+ checkboxes)
    - Failure indicators (what shows fix didn't work)
```

**Step 8: Estimate Effort**

Use effort estimation methodology from recommendation-framework.md:

```
FOR each recommendation:
  Calculate: base_effort = change_complexity + testing_time + documentation_time

  Determine complexity:
    - Simple add: 15-30 min
    - Complex add: 30-60 min
    - Modify logic: 1-2 hours
    - Refactor: 2-4 hours
    - New component: 2-3 hours

  Add testing time: ~1 hour
  Add documentation time: ~30 min

  Categorize: Low | Medium | High
  Dependencies: List other recommendations needed first
```

**Step 9: Analyze Impact**

```
FOR each recommendation:
  Identify benefit:
    - What improves?
    - How much?
    - Who benefits?

  Identify risk:
    - What could go wrong?
    - How to mitigate?

  Identify scope:
    - Files affected: {count}
    - Workflows affected: {list}
    - Users affected: {all/some/specific}
```

**Step 10: Sort Recommendations**

```
Group by priority: CRITICAL, HIGH, MEDIUM, LOW

Within each priority:
  Sort by dependencies (foundation first, dependent second)
  Then by effort (quick wins first)

Assign recommendation IDs: REC-1, REC-2, REC-3...
```

**Phase 4 Output:**
```
✓ Recommendations Generated: {count}
  - CRITICAL: {count}
  - HIGH: {count}
  - MEDIUM: {count}
  - LOW: {count}

✓ All recommendations have:
  - Exact file paths
  - Specific sections
  - Copy-paste ready code/text
  - Evidence-based rationale
  - Testing procedures
  - Effort estimates

Proceeding to Phase 5: RCA Document Creation...
```

---

### Phase 5: RCA Document Creation

**Objective:** Generate complete RCA document from template

**Step 1: Load RCA Writing Guide**

```
Read(file_path=".claude/skills/devforgeai-rca/references/rca-writing-guide.md")

Use: Document structure, formatting standards
```

**Step 2: Load RCA Document Template**

```
Read(file_path=".claude/skills/devforgeai-rca/assets/rca-document-template.md")

Template has placeholders: {NUMBER}, {TITLE}, {DATE}, etc.
```

**Step 3: Populate Header**

```
Replace placeholders:
- {NUMBER} → rca_number (from Phase 0)
- {TITLE} → rca_title (from Phase 0)
- {DATE} → current_date (YYYY-MM-DD format)
- {REPORTER} → "User" or extracted from conversation
- {COMPONENT} → affected_component (from Phase 0)
- {SEVERITY} → severity (from Phase 0)
```

**Step 4: Populate Issue Description**

```
Replace: {ISSUE_DESCRIPTION} → from Phase 0 issue_description

Ensure includes:
- What happened
- When happened
- Where happened
- Expected vs actual
- Impact
```

**Step 5: Populate 5 Whys Section**

```
Replace: {ISSUE_STATEMENT} → issue_description brief version

FOR i = 1 to 5:
  Replace: {ANSWER_i} → why_answers[i].answer
  If i == 5:
    Emphasize: **ROOT CAUSE:** prefix

Format per 5-whys-formatting section in rca-writing-guide.md
```

**Step 6: Populate Evidence Section**

```
Replace: {FILE_LIST} →
  FOR each file in files_examined (HIGH/CRITICAL significance):
    Format per evidence template:
      **{path}**
      - Lines: {range}
      - Finding: {discovery}
      - Excerpt: ```{text}```
      - Significance: {why matters}

Replace: {CONTEXT_STATUS} →
  Context files validation section from Phase 3

Replace: {WORKFLOW_STATE} →
  Workflow state analysis from Phase 3 (if applicable)
```

**Step 7: Populate Recommendations Section**

```
FOR each priority (CRITICAL, HIGH, MEDIUM, LOW):
  IF recommendations_at_priority.length > 0:
    FOR each recommendation:
      Format per recommendation template:
        - Title
        - Problem Addressed
        - Proposed Solution
        - Implementation Details (File, Section, Code)
        - Rationale
        - Testing
        - Effort Estimate
        - Impact
  ELSE:
    Insert: "None"
```

**Step 8: Generate Implementation Checklist**

```
From recommendations:
  Extract action items:
    - Implement REC-X for each CRITICAL
    - Update specific files
    - Add tests
    - Documentation updates

Add standard items:
  - Review all recommendations
  - Prioritize by impact/effort
  - Mark RCA as RESOLVED
  - Commit changes
```

**Step 9: Generate Prevention Strategy**

```
Short-term (from CRITICAL recommendations):
  - {Primary fix from REC-1}
  - {Secondary fix from REC-2}

Long-term (from HIGH/MEDIUM recommendations):
  - Pattern improvements
  - Process enhancements
  - Architecture changes

Monitoring:
  - What to watch for (error patterns)
  - When to audit (frequency)
  - Escalation criteria
```

**Step 10: Populate Related RCAs**

```
IF related_rcas.length > 0:
  FOR each related_rca:
    Extract: number, title from filename
    Determine: relationship (similar cause, same component, related pattern)
    Format: "- **{number}:** {title} ({relationship})"
ELSE:
  Insert: "None"
```

**Step 11: Write RCA Document**

```
Generate filename:
  slug = rca_title.lowercase().replace(" ", "-")
  filename = "RCA-{rca_number}-{slug}.md"
  path = "devforgeai/RCA/{filename}"

Write document:
  Write(file_path=path, content=populated_template)

Verify:
  Read(file_path=path)
  Check: All placeholders replaced (no {PLACEHOLDER} remaining)
```

**Phase 5 Output:**
```
✓ RCA Document Created:
  - File: devforgeai/RCA/RCA-{NNN}-{slug}.md
  - Sections: 8/8 complete
  - Recommendations: {count}
  - Evidence: {count} files examined

Proceeding to Phase 6: Validation & Self-Check...
```

---

### Phase 6: Validation & Self-Check

**Objective:** Verify RCA document completeness and quality

**Step 1: Validate Structure**

```
Read(file_path="devforgeai/RCA/RCA-{rca_number}-{slug}.md")

Check required sections exist:
- [ ] Header (with all metadata)
- [ ] Issue Description
- [ ] 5 Whys Analysis
- [ ] Evidence Collected
- [ ] Recommendations (by priority)
- [ ] Implementation Checklist
- [ ] Prevention Strategy
- [ ] Related RCAs

IF any section missing:
  CRITICAL ERROR: "Section {name} missing from RCA document"
  Self-heal: Add missing section with placeholder
```

**Step 2: Validate 5 Whys**

```
Check:
- [ ] All 5 Whys answered
- [ ] Each "why" has evidence reference
- [ ] Why #5 marked as ROOT CAUSE
- [ ] Root cause is specific (not vague)

IF any check fails:
  WARNING: "5 Whys validation issue: {issue}"
  Self-heal: Add missing evidence references or clarify root cause
```

**Step 3: Validate Evidence**

```
Check:
- [ ] At least 3 files examined
- [ ] Each file has excerpts
- [ ] Excerpts include line numbers
- [ ] Significance explained for each
- [ ] Context files validated (if relevant)

IF <3 files:
  WARNING: "Evidence insufficient: Only {count} files examined"
  Recommendation: "Collect additional evidence in next RCA iteration"
```

**Step 4: Validate Recommendations**

```
Check:
- [ ] At least 3 recommendations generated
- [ ] All recommendations have priority
- [ ] All have exact file paths
- [ ] All have exact code/text
- [ ] All have testing procedures
- [ ] All have effort estimates

IF any check fails:
  CRITICAL ERROR: "Recommendation {ID} incomplete: {missing}"
  Self-heal: Complete missing details from Phase 4 data
```

**Step 5: Validate Placeholders**

```
Search document for: \{[A-Z_]+\} (regex for {PLACEHOLDER})

IF placeholders found:
  CRITICAL ERROR: "Placeholders not replaced: {list}"
  Self-heal: Replace with actual values from Phase data
```

**Step 6: Validate File Paths**

```
FOR each file path in recommendations:
  Extract path from "File: `{path}`"

  IF path exists:
    ✓ Valid
  ELSE:
    WARNING: "File path may be incorrect: {path}"
    Note: "Verify path before implementing recommendation"
```

**Step 7: Check Quality Standards**

```
From rca-writing-guide.md "Document Quality Checklist":

Verify:
- [ ] Title 3-6 words
- [ ] No aspirational content
- [ ] No vague recommendations
- [ ] All evidence-based
- [ ] Copy-paste ready code

IF violations found:
  Self-heal: Fix violations (shorten title, make specific, etc.)
```

**Phase 6 Output:**
```
✓ Validation Complete:
  - Structure: PASS ({8}/8 sections)
  - 5 Whys: PASS (all answered with evidence)
  - Evidence: PASS ({count} files, {count} excerpts)
  - Recommendations: PASS ({count} complete)
  - Placeholders: PASS (all replaced)
  - File paths: {valid_count}/{total_count} valid
  - Quality: PASS

{If warnings}: ⚠️ Warnings: {count}
  - {Warning 1}
  - {Warning 2}

Proceeding to Phase 7: Completion Report...
```

---

### Phase 7: Completion Report

**Objective:** Return structured summary to /rca command for display

**Step 1: Generate Summary**

```
Create summary:
  rca_number: "RCA-{NNN}"
  rca_title: "{title}"
  rca_file: "devforgeai/RCA/RCA-{NNN}-{slug}.md"
  severity: "{CRITICAL/HIGH/MEDIUM/LOW}"
  root_cause_brief: "{1-2 sentence summary of why_5}"
  recommendation_counts: {
    CRITICAL: {count},
    HIGH: {count},
    MEDIUM: {count},
    LOW: {count}
  }
  total_recommendations: {count}
```

**Step 2: Determine Next Steps**

```
IF recommendation_counts.CRITICAL > 0:
  next_steps = "Review CRITICAL recommendations immediately. Create story for implementation if substantial work (>2 hours)."

ELSE IF recommendation_counts.HIGH > 0:
  next_steps = "Review HIGH recommendations. Plan implementation in current sprint."

ELSE IF recommendation_counts.MEDIUM > 0:
  next_steps = "Review MEDIUM recommendations. Add to next sprint backlog."

ELSE:
  next_steps = "Review LOW priority improvements. Implement opportunistically."

Add:
  next_steps += "\n\nRead complete RCA: {rca_file}"
```

**Step 3: Format Completion Report**

```
Format report for command display:

═══════════════════════════════════════════════
RCA COMPLETE: {rca_number}
═══════════════════════════════════════════════

Title: {rca_title}
Severity: {severity}
File: {rca_file}

ROOT CAUSE:
{root_cause_brief}

RECOMMENDATIONS:
- CRITICAL: {count} (implement immediately)
- HIGH: {count} (implement this sprint)
- MEDIUM: {count} (next sprint)
- LOW: {count} (backlog)

NEXT STEPS:
{next_steps}

═══════════════════════════════════════════════
```

**Step 4: Return to Command**

```
Return: completion_report (formatted above)

Command will display this report to user
```

**Phase 7 Output:**
```
✓ RCA Complete
✓ Document: {rca_file}
✓ Recommendations: {total}
✓ Report returned to command

{Display completion report}
```

---

## Error Handling

### Phase 0 Errors

**Error: Issue description empty/unclear**
```
Recovery: Use AskUserQuestion to gather complete information
Proceed: After information collected
```

**Error: Cannot determine affected component**
```
Recovery: AskUserQuestion with options (skill/command/subagent/other)
Proceed: After component identified
```

**Error: Cannot generate RCA number (no existing RCAs)**
```
Recovery: Start with RCA-001
Proceed: Continue with RCA-001
```

### Phase 1 Errors

**Error: Primary file not found**
```
Recovery:
  Display: "⚠️ Expected file not found: {path}"
  AskUserQuestion: "Should I continue without this file or HALT?"

  IF continue: Proceed with available evidence, note gap in RCA
  IF HALT: Exit workflow, report to user
```

**Error: Too many files to read (>20)**
```
Recovery:
  Filter to most relevant (based on significance)
  Limit to top 15 files
  Note in RCA: "Evidence limited to {count} most relevant files"
```

### Phase 2 Errors

**Error: Cannot find evidence for "why" answer**
```
Recovery:
  Mark as: {answer} (evidence pending further investigation)
  Continue analysis
  Flag in Phase 6 validation
```

**Error: Root cause validation fails**
```
Recovery:
  Revise Why #5 with different root cause
  Re-validate
  If still fails: Mark as "Preliminary root cause (requires deeper investigation)"
```

### Phase 4 Errors

**Error: Cannot determine exact file path for recommendation**
```
Recovery:
  Use: "File: {best guess - verify before implementing}"
  Mark recommendation: "⚠️ Path verification needed"
  Explain in rationale: "Exact path requires verification"
```

**Error: Solution not clear**
```
Recovery:
  Mark recommendation priority as MEDIUM (lower than original)
  Note: "Solution requires additional investigation"
  Provide: Investigation directions instead of exact fix
```

### Phase 5 Errors

**Error: Cannot write RCA document (permission denied)**
```
Recovery:
  Display document content to user
  Instruct: "Please save manually to: {path}"
  Provide: Complete document text
```

**Error: RCA file already exists**
```
Recovery:
  Increment RCA number
  Retry write with new number
```

---

## Token Efficiency

### Progressive Loading

**Load reference files ONLY when needed:**

**Phase 0:** No references (extraction only)
**Phase 1:** framework-integration-points.md (determine what to read)
**Phase 2:** 5-whys-methodology.md, 5-whys-template.md
**Phase 3:** evidence-collection-guide.md, evidence-section-template.md
**Phase 4:** recommendation-framework.md, recommendation-template.md
**Phase 5:** rca-writing-guide.md, rca-document-template.md
**Phase 6:** (Use already-loaded guides for validation)
**Phase 7:** No references (summary generation)

**Total References Loaded:** 5 files (~4,000 lines) loaded progressively across phases

### Context Isolation

**This skill operates in isolated context:**
- Main conversation sees: Skill invocation (~3K tokens) + completion report (~2K tokens)
- Skill context sees: All phases, all references, all evidence (~50-80K tokens)
- Efficiency: 90% of work in isolated context

---

## Integration with DevForgeAI Framework

### Invoked By

**Commands:**
- `/rca` slash command (primary invocation)

**Manual invocation:**
```
**Issue Description:** {description}
**Severity:** {CRITICAL/HIGH/MEDIUM/LOW}

Skill(command="devforgeai-rca")
```

### References

**Context Files:**
- Validates against all 6 files (if constraint issue)
- Understands constraint structure
- Knows validation patterns

**Quality Gates:**
- Knows all 4 gates
- Understands gate criteria
- Identifies gate bypass patterns

**Workflow States:**
- Knows all 11 states
- Understands valid transitions
- Detects state errors

**Lean Pattern:**
- Knows command/skill/subagent responsibilities
- Detects architectural violations
- References pattern documentation

### Outputs

**Primary Output:**
- RCA document in `devforgeai/RCA/RCA-{NNN}-{slug}.md`

**Secondary Output:**
- Completion report (returned to command for display)

**Updates:**
- None (RCA is read-only analysis, doesn't modify framework)

---

## Success Criteria

RCA skill execution successful when:
- [ ] All 8 phases executed
- [ ] RCA document created
- [ ] 5 Whys complete with evidence
- [ ] 3+ recommendations generated
- [ ] All recommendations have exact implementation
- [ ] Evidence comprehensive (3+ files examined)
- [ ] Root cause validated
- [ ] Completion report returned to command

---

## Reference

**This skill uses:**
- 5 reference files (progressive loading)
- 4 asset templates (document generation)
- DevForgeAI framework knowledge (context files, gates, states)

**Related Skills:**
- devforgeai-development (may need RCA)
- devforgeai-qa (may need RCA)
- devforgeai-orchestration (may need RCA)

**Related Documentation:**
- `CLAUDE.md` - Framework overview
- `devforgeai/protocols/lean-orchestration-pattern.md` - Architecture pattern
- `.claude/memory/*.md` - Framework reference

---

**End of devforgeai-rca Skill**

**Total: ~1,500 lines**
