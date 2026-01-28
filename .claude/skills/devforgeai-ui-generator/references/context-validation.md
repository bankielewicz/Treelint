# Phase 1: Context Validation

Validate that all 6 DevForgeAI context files exist before UI generation.

**Objective:** Ensure DevForgeAI context files exist and are valid before proceeding.

---

## Overview

UI generation requires architectural context to ensure generated code respects framework constraints:
- **tech-stack.md** - Approved technologies
- **source-tree.md** - Project structure
- **dependencies.md** - Approved packages
- **coding-standards.md** - Code conventions
- **architecture-constraints.md** - Layer boundaries
- **anti-patterns.md** - Forbidden patterns

Without these files, the skill cannot validate generated UI against project standards.

---

## Steps

### Step 1.1: Check Context Files

**Use Glob to verify all 6 required context files exist:**
```
Glob(pattern="devforgeai/specs/context/*.md")
```

**Required files:**
- `tech-stack.md`
- `source-tree.md`
- `dependencies.md`
- `coding-standards.md`
- `architecture-constraints.md`
- `anti-patterns.md`

**Validation:**
```
IF file_count < 6:
  missing_files = [determine which files are missing]
  HALT workflow
  Display error (see Step 1.3)
ELSE:
  Proceed to Step 1.2
```

---

### Step 1.2: Validate Context

**Run the validation script:**
```
Bash(command="python .claude/skills/devforgeai-ui-generator/scripts/validate_context.py")
```

**Script checks:**
- All 6 files exist
- All files are non-empty
- No placeholder content (TODO, TBD)
- Valid markdown format

**Handle validation result:**
```
IF validation fails:
  Display specific validation errors
  HALT workflow
  Proceed to Step 1.3 (error handling)
ELSE:
  Proceed to Step 1.4 (load context)
```

---

### Step 1.3: Handle Missing Context

**If context files missing:**

```
HALT and inform user:

"❌ Context files are missing. Cannot proceed with UI generation.

Missing files: ${missing_files.join(', ')}

**Recovery:**
Please invoke the `devforgeai-architecture` skill first to create the required context files:

Skill(command=\"devforgeai-architecture\")

Or use the slash command:
/create-context ${project_name}

After context files are created, run /create-ui again."

EXIT skill with error status
```

**If validation fails:**
```
Display specific issues:
"❌ Context validation failed:
${validation_errors}

**Recovery:**
Review and fix the context files, then run /create-ui again."

EXIT skill with error status
```

---

### Step 1.4: Load Context Files

**Use Read tool to load critical context:**
```
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/source-tree.md")
Read(file_path="devforgeai/specs/context/dependencies.md")
```

**Parse for UI-specific constraints:**
- **tech-stack.md:** Extract approved UI frameworks/libraries
- **source-tree.md:** Extract correct output location for UI components
- **dependencies.md:** Extract approved package versions

**Store constraints:**
```
ui_constraints = {
  "approved_frameworks": [list from tech-stack.md],
  "output_location": "path from source-tree.md",
  "approved_packages": [list from dependencies.md]
}
```

---

## Output

**Phase 1 produces:**
- Context validated
- Constraints loaded into memory
- Ready to proceed to Phase 2 (Story Analysis) or Phase 3 (Interactive Discovery)

**If validation fails:**
- Skill exits with clear error message
- User directed to create context files first
