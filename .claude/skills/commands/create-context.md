---
description: Generate architectural context files for project
argument-hint: [project-name]
model: opus
allowed-tools: Read, Write, Edit, Glob, Grep, Skill, Task, AskUserQuestion
---

# Create Context Command

Generates the 6 architectural context files that define project constraints and standards.

## Arguments

- `project-name` (optional): Name of the project (defaults to current directory name)

## Workflow

### Phase 1: Pre-Flight Check

**Check for existing context:**

```bash
# Look for existing context files
Glob(pattern="devforgeai/specs/context/*.md")
```

**If context files exist:**
- Use `AskUserQuestion` with options:
  - Overwrite all existing files
  - Merge with existing files (preserve custom changes)
  - Abort and exit

**If abort selected:**
- Exit with message: "Context creation cancelled. Existing files preserved."

### Phase 2: Git Repository Initialization Check

**Ensure repository has initial commit:**

Check if repository has commits by examining the environment context (`<env>Is directory a git repo: Yes/No</env>`).

**If git repository exists but no commits yet:**

1. **Create initial commit with framework files:**
```bash
Bash(git add .claude/ devforgeai/ devforgeai/specs/ CLAUDE.md README.md)
Bash(git commit -m "chore: Initialize DevForgeAI framework structure")
```

**Rationale:**
- DevForgeAI commands expect git history for commit message analysis
- Initial commit prevents "no commits yet" errors in /dev command
- Framework files are committed, establishing baseline

**If commit creation fails:**
- Continue anyway (git may not be initialized)
- /dev command will handle empty repos gracefully

**Note:** Use the environment context to check git status instead of running `git rev-list` which requires special approval.

---

### Phase 3: Invoke Architecture Skill

**Invoke architecture skill:**

```bash
Skill(command="devforgeai-architecture")
```

**After skill invocation:**
- Skill's SKILL.md content expands inline in conversation
- **YOU execute the skill's workflow phases** (not waiting for external result)
- Follow the skill's instructions phase by phase
- Produce output as skill instructs

**The skill instructs you to perform 5-phase workflow:**

1. **Project Context Discovery** - Determine greenfield vs brownfield, discover existing tech/structure, analyze gaps
2. **Create Immutable Context Files** - Generate all 6 files from templates:
   - `tech-stack.md` - Locked technologies (prevents library substitution)
   - `source-tree.md` - Project structure (prevents chaos)
   - `dependencies.md` - Approved packages (prevents unapproved additions)
   - `coding-standards.md` - Code patterns (enforces consistency)
   - `architecture-constraints.md` - Layer boundaries (prevents violations)
   - `anti-patterns.md` - Forbidden patterns (prevents technical debt)
3. **Create Architecture Decision Records** - Document all major decisions with ADRs
4. **Create Technical Specifications** - Optional: Use cases, API specs, database schemas, NFRs
5. **Validate Spec Against Context** - Ensure no conflicts, all constraints respected

**Progressive disclosure:** Each phase loads its workflow file on-demand (78% token efficiency improvement)

**Expected output location:**
- All files in `devforgeai/specs/context/`
- ADRs in `devforgeai/specs/adrs/`

### Phase 4: Architecture Review

**Invoke architect reviewer:**

```bash
Task(
  subagent_type="architect-reviewer",
  prompt="Review the generated context files in devforgeai/specs/context/ for:

  1. Architectural soundness - Do layer boundaries prevent tight coupling?
  2. Technology coherence - Are selected technologies compatible?
  3. Completeness - Are all project concerns addressed?
  4. Consistency - Do files align with each other?
  5. Practicality - Are constraints realistic and enforceable?

  Focus on critical issues that would cause problems during development.
  Provide specific recommendations for any concerns."
)
```

**If reviewer raises concerns:**
- Use `AskUserQuestion` to present concerns with options:
  - Accept recommendations and update files
  - Keep current approach (document rationale)
  - Hybrid approach (accept some, reject others)

**Update files if needed:**
- Use `Edit` tool to apply approved changes
- Create ADR documenting review decisions

### Phase 5: Design System Generation (UI Projects)

**Detect if UI project:**

```bash
Read(file_path="devforgeai/specs/context/tech-stack.md")
```

**Check for frontend technologies:**
- React, Vue, Angular, Svelte
- Next.js, Nuxt, SvelteKit
- React Native, Flutter, Ionic

**If UI framework detected:**

1. **Gather design preferences:**

```
AskUserQuestion:
  Question: "This is a UI project. Would you like to generate a design system?"
  Options:
    - Yes, custom design system (provide preferences)
    - Yes, use popular framework (Material UI, Chakra UI, shadcn/ui, etc.)
    - No, skip design system
```

2. **If custom design system selected:**

```
AskUserQuestion:
  Question: "Select design system preferences"
  Multi-part:
    - Color palette: (Brand colors, Neutrals, Semantic colors)
    - Typography: (Font families, Size scale, Line heights)
    - Spacing scale: (4px base, 8px base, custom)
    - Border radius: (None, Small, Medium, Large, Custom)
    - Shadow style: (Flat, Soft, Material, Custom)
```

3. **Generate design-system.md from template:**

```bash
# Load template
Read(file_path=".claude/skills/devforgeai-architecture/assets/context-templates/design-system.md")

# Customize with user preferences and write
Write(
  file_path="devforgeai/specs/context/design-system.md",
  content="[Customized design-system template with user's color/typography/spacing choices]"
)
```

**Customization includes:**
- Design Tokens (colors, typography, spacing, shadows, borders)
- Component Guidelines (naming, structure, composition)
- Accessibility Standards (WCAG 2.1 AA, ARIA patterns)
- Responsive Breakpoints
- Animation/Transition Standards
- Framework Integration (React/Vue/Angular specific patterns)

4. **If framework-based design selected:**
- Document framework choice in tech-stack.md
- Add framework to dependencies.md
- Reference framework docs in design-system.md

### Phase 6: Final Validation

**Verify context file completeness:**

```bash
# Check all 6 required files exist
Glob(pattern="devforgeai/specs/context/*.md")

# Read each file to verify non-empty
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/source-tree.md")
Read(file_path="devforgeai/specs/context/dependencies.md")
Read(file_path="devforgeai/specs/context/coding-standards.md")
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
Read(file_path="devforgeai/specs/context/anti-patterns.md")
```

**Validation checks:**
- ✅ All 6 files exist
- ✅ No file is empty (> 100 characters)
- ✅ No TODO placeholders
- ✅ No TBD placeholders
- ✅ No "[FILL IN]" or similar markers

**Check for placeholder content:**

```bash
Grep(pattern="TODO|TBD|\\[FILL IN\\]|\\[TO BE DETERMINED\\]", path="devforgeai/specs/context/", output_mode="files_with_matches")
```

**If placeholders found:**
- Report files with placeholders
- Use `AskUserQuestion` to resolve each placeholder
- Update files with final values

**Verify ADR creation:**

```bash
Glob(pattern="devforgeai/specs/adrs/ADR-*.md")
```

**Expected ADRs (minimum):**
- ADR-001: Primary language selection
- ADR-002: Framework selection (if applicable)
- ADR-003: Database selection (if applicable)
- Additional ADRs for significant choices

### Phase 7: Success Report

**Display summary:**

```
✅ Context Creation Complete

Generated Files:
  📋 tech-stack.md          - [Technology count] technologies defined
  📁 source-tree.md         - [Layer count] layers structured
  📦 dependencies.md        - [Package count] packages approved
  ✨ coding-standards.md    - [Standard count] standards defined
  🏛️ architecture-constraints.md - [Constraint count] constraints enforced
  🚫 anti-patterns.md       - [Pattern count] anti-patterns forbidden

[If UI project]
  🎨 design-system.md       - Design tokens and guidelines

ADRs Created:
  📝 ADR-001: [Decision title]
  📝 ADR-002: [Decision title]
  [...]

Architecture Review: ✅ PASSED

Next Steps:
  1. Review context files in devforgeai/specs/context/
  2. Customize if needed (add project-specific constraints)
  3. Run /plan-sprint to create your first sprint
  4. Start implementing stories with /implement
```

## Error Handling

### Error: Context Files Already Exist

**Condition:** `devforgeai/specs/context/` contains .md files

**Action:**
1. List existing files
2. Ask user: Overwrite, Merge, or Abort
3. If Abort: Exit cleanly
4. If Overwrite: Delete existing, proceed
5. If Merge: Read existing, preserve custom sections, update standard sections

### Error: Architecture Skill Failed

**Condition:** Skill invocation returns error

**Recovery:**
1. Display error message from skill
2. Check common causes:
   - Missing dependencies
   - Invalid project structure
   - Skill file corrupted
3. Suggest fixes:
   - Verify `.claude/skills/devforgeai-architecture/SKILL.md` exists
   - Check skill syntax
   - Run with verbose logging

**Report to user:**
```
❌ Architecture skill failed: [error message]

Possible causes:
  - [Cause 1]
  - [Cause 2]

Recovery steps:
  1. [Step 1]
  2. [Step 2]

Try again with: /create-context [project-name]
```

### Error: Validation Failed

**Condition:** Files missing, empty, or contain placeholders

**Action:**
1. List specific validation failures:
   - Missing files: [list]
   - Empty files: [list]
   - Files with placeholders: [list with line numbers]
2. For each issue:
   - If missing: Re-invoke architecture skill for that file
   - If empty: Check skill output logs, regenerate
   - If placeholders: Ask user to provide values

**Report format:**
```
❌ Validation Failed

Missing Files:
  - dependencies.md

Files with Placeholders:
  - tech-stack.md:15 - TODO: Select testing framework
  - coding-standards.md:42 - TBD: Define naming conventions

Action Required:
  Please provide missing information...
```

### Error: Design System Generation Failed

**Condition:** UI project detected but design-system.md creation failed

**Action:**
1. Check if user declined design system (acceptable, skip)
2. If generation failed:
   - Verify preferences were provided
   - Check file write permissions
   - Retry with defaults

**Non-critical error:**
- Project can proceed without design-system.md
- Warn user that frontend consistency may suffer
- Suggest creating manually later

## Success Criteria

- [x] All 6 context files created in `devforgeai/specs/context/`
- [x] No TODO/TBD/placeholder content
- [x] ADRs created for major technology decisions
- [x] Architecture review completed (no critical issues)
- [x] Design system generated (if UI project)
- [x] Validation passed (all checks green)
- [x] Token usage < 50K

## Token Efficiency

**Target:** < 50K tokens total

**Optimization strategies:**
1. Use `Skill` tool (architecture skill self-contained)
2. Parallel validation reads when possible
3. Only read files once for validation
4. Use `Glob` instead of Bash `find`
5. Use `Grep` for placeholder detection (not `cat | grep`)
6. Cache tech-stack.md in memory for UI detection

**Estimated token breakdown:**
- Phase 1 (Pre-flight): ~2K tokens
- Phase 2 (Architecture skill): ~25K tokens (skill's budget)
- Phase 3 (Review): ~8K tokens
- Phase 4 (Design system): ~5K tokens (if UI)
- Phase 5 (Validation): ~3K tokens
- Phase 6 (Report): ~1K tokens
- **Total: ~44K tokens** (within 50K budget)

## Integration Points

**Invokes:**
- `devforgeai-architecture` skill (Phase 2)
- `architect-reviewer` subagent (Phase 3)

**Prerequisites:**
- None (entry point for new projects)

**Enables:**
- `/plan-sprint` command (requires context files)
- `/implement` command (requires context files)
- All development workflows (context is foundation)

**Related Commands:**
- `/plan-sprint` - Create sprints and stories (next step)
- `/implement` - Implement stories with TDD
- `/validate` - Run QA validation

## Notes

**Framework philosophy:**
- Context files are immutable constraints during development
- All technology decisions made upfront prevent drift
- Architecture skill ensures completeness (13-80 questions)
- Validation ensures no ambiguity (zero placeholders)

**When to use:**
- Starting new projects (greenfield)
- Adding DevForgeAI to existing projects (brownfield)
- Major architecture changes (create new ADR)
- Technology stack changes (update tech-stack.md + ADR)

**When NOT to use:**
- During active story implementation (context locked)
- For minor dependency additions (use architecture skill directly)
- To regenerate specific files (use architecture skill with --file flag)

**Post-creation changes:**
- Minor updates: Edit files directly, document in ADR
- Major changes: Re-run command with merge option
- Technology swaps: REQUIRES ADR justification + team approval

---

### Phase N: Feedback Hook Integration (NEW - STORY-030)

**After Phase 6 (Final Validation) completes successfully.**

Triggers optional feedback conversation (non-blocking, configuration-aware).

**See:** `Read(file_path="devforgeai/protocols/hook-integration-pattern.md")` for complete implementation pattern including:
- Step 1: Determine operation status
- Step 2: Check hook eligibility
- Step 3: Invoke hooks if eligible
- Step 4: Phase complete (context files created, hook outcome independent)
- Code examples for /create-context, /create-epic, /dev
- Key characteristics and testing guidance

**Quick Implementation:**
```bash
# Verify context files created
if [ -f "devforgeai/specs/context/tech-stack.md" ] && \
   [ -f "devforgeai/specs/context/source-tree.md" ]; then
  STATUS="completed"
else
  STATUS="failed"
fi

# Check eligibility and invoke (non-blocking)
devforgeai-validate check-hooks --operation=create-context --status=$STATUS && \
  devforgeai-validate invoke-hooks --operation=create-context || true
```

**Note:** Hook failures don't prevent command completion. Context files created = primary success criteria.
