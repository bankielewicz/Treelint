# Phase 1: Context Loading - Detailed Workflow

**Purpose:** Load and validate ALL 6 context files required for anti-pattern detection.

**Inputs:** Story ID, codebase path
**Outputs:** context_data object with all 6 files loaded
**Duration:** <2 seconds

---

## Step 1.1: Locate Context Files

**Action:** Find context directory in project root

```
context_dir = "devforgeai/specs/context/"

required_files = [
  "tech-stack.md",
  "source-tree.md",
  "dependencies.md",
  "coding-standards.md",
  "architecture-constraints.md",
  "anti-patterns.md"
]
```

---

## Step 1.2: Validate File Existence

**For each file in required_files:**

```
file_path = context_dir + file_name

IF NOT file_exists(file_path):
  RETURN {
    "status": "failure",
    "error": f"Required context file not found: {file_path}",
    "blocks_qa": true,
    "remediation": "Run /create-context to generate architectural context files"
  }
```

**If ANY file missing → HALT with failure response**

---

## Step 1.3: Read File Contents

**For each file:**

```
content = Read(file_path=file_path)

context_data[file_name] = {
  "path": file_path,
  "content": content,
  "size": len(content)
}
```

---

## Step 1.4: Extract Key Information

**From tech-stack.md:**
- Locked technologies (ORM, state manager, HTTP client, validation, testing)
- Language/framework constraints
- Approved dependencies

**From source-tree.md:**
- Layer definitions (Domain, Application, Infrastructure, Presentation)
- Directory structure rules
- File location constraints

**From dependencies.md:**
- Approved packages and versions
- Forbidden dependencies

**From coding-standards.md:**
- Naming conventions
- Documentation requirements
- Code style rules

**From architecture-constraints.md:**
- Layer dependency rules (Domain → independent, Application → Domain only)
- Circular dependency prohibitions
- Dependency inversion patterns

**From anti-patterns.md:**
- Forbidden code patterns
- God object thresholds (>15 methods, >300 lines)
- Long method thresholds (>50 lines)
- Magic number rules

---

## Step 1.5: Validate Consistency

**Check for contradictions:**

```
IF tech_stack locks ORM="Dapper"
   AND dependencies includes "Entity Framework":
  RETURN {
    "status": "failure",
    "error": "Context files contradictory: tech-stack.md locks Dapper, dependencies.md lists Entity Framework",
    "blocks_qa": true,
    "remediation": "Resolve contradiction - update tech-stack.md or dependencies.md to match"
  }
```

---

## Step 1.6: Return Context Data

**Success response:**

```json
{
  "status": "success",
  "context_loaded": true,
  "files_loaded": 6,
  "tech_stack": {...},
  "source_tree": {...},
  "dependencies": {...},
  "coding_standards": {...},
  "architecture_constraints": {...},
  "anti_patterns": {...}
}
```

**This context_data object is passed to all subsequent phases.**

---

## Error Scenarios

**Scenario 1: Missing context files**
- Error: "Required context file not found: devforgeai/specs/context/tech-stack.md"
- Remediation: "Run /create-context to generate architectural context files"
- Blocks QA: true

**Scenario 2: Contradictory rules**
- Error: "Context files contradictory: tech-stack.md specifies X, dependencies.md lists Y"
- Remediation: "Resolve contradiction by updating one of the context files"
- Blocks QA: true

**Scenario 3: Empty/corrupted files**
- Error: "Context file empty or corrupted: anti-patterns.md"
- Remediation: "Regenerate context file or check file permissions"
- Blocks QA: true
