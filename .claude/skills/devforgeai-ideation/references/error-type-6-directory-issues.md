# Error Type 6: Directory Structure Issues

Handling missing directories and file system structure problems.

---

## Error Detection

**Symptom:** Expected directories don't exist or have wrong structure

**Detected during:** Phase 6.1 (Artifact Generation)

**Examples:**
- `devforgeai/specs/` directory missing
- `devforgeai/` directory missing
- Subdirectories (Epics/, specs/requirements/) don't exist
- Permission denied when creating directories

**Detection logic:**

```
required_dirs = [
    "devforgeai/specs/Epics/",
    "devforgeai/specs/requirements/",
    "devforgeai/specs/adrs/",
    "devforgeai/specs/context/"
]

for dir in required_dirs:
    exists = Glob(pattern=dir)
    if not exists:
        trigger directory_missing_error(dir)
```

---

## Recovery Procedures

### Step 1: Create Required Directories

```
required_dirs = [
    "devforgeai/specs/Epics/",
    "devforgeai/specs/requirements/",
    "devforgeai/specs/adrs/",
    "devforgeai/specs/context/"
]

for dir in required_dirs:
    # Create directory using Write/.gitkeep pattern (Constitutional C1 compliant)
    Write(file_path=f"{dir}.gitkeep", content="")

    # Verify creation
    check = Glob(pattern=dir)

    if not check:
        ERROR: Could not create directory {dir}
        # Will report to user in Step 3
```

### Step 2: Check Write Permissions

```
# Test write to each directory
for dir in required_dirs:
    test_file = f"{dir}/.test-{timestamp}"

    try:
        Write(file_path=test_file, content="test")
        # Clean up test file
        Bash(command=f"rm {test_file}")
        ✓ {dir} writable
    except:
        ERROR: No write permission to {dir}
        permission_errors.append(dir)
```

### Step 3: Report Permission Errors to User

```
If len(permission_errors) > 0:
    Report: """
    ❌ File System Permission Errors

    Cannot write to directories:
    {list permission_errors}

    Required actions:
    1. Grant write permissions:
       bash
       chmod 755 {dir}

    2. Or create directories manually:
       bash
       mkdir -p devforgeai/specs/Epics
       mkdir -p devforgeai/specs/requirements

    3. Then re-run ideation skill

    If running in container or CI/CD, ensure volume mounts have write permissions.
    """
```

---

## Example Scenarios

### Scenario 1: Fresh Project (No devforgeai/)

**Error:** `devforgeai/` directory doesn't exist

**Recovery:**
1. Create devforgeai/ using Write/.gitkeep
2. Create all subdirectories
3. Verify creation successful

### Scenario 2: Partial Structure

**Error:** `devforgeai/specs/` exists but `devforgeai/specs/Epics/` missing

**Recovery:**
1. Detect missing subdirectory
2. Create using Write/.gitkeep
3. Continue with artifact generation

### Scenario 3: Permission Denied

**Error:** Cannot create `devforgeai/specs/Epics/`

**Recovery:**
1. Detect permission error
2. Report to user with chmod instructions
3. Provide manual mkdir commands

---

## Max Recovery Attempts

**Attempt 1:** Create directories with Write/.gitkeep pattern
**Attempt 2:** Test write permissions, verify creation

**If still failing:** Report to user with manual creation steps

---

## Related Patterns

- See [error-type-2-artifact-failures.md](error-type-2-artifact-failures.md) for file write errors
- See [error-handling-index.md](error-handling-index.md) for error type decision tree
- Constitutional C1: Use Write/.gitkeep instead of Bash mkdir

---

## Phase Context

This error occurs during **Phase 6.1: Artifact Generation** when the ideation skill attempts to create files but required directories don't exist.

It is closely related to error-type-2 (Artifact Failures), but focuses specifically on directory structure validation and creation.

Recovery must complete before file creation can proceed.

---

## Directory Structure Reference

**Required structure for ideation artifacts:**

```
devforgeai/
├── specs/
│   ├── Epics/           # Epic files (EPIC-NNN.epic.md)
│   ├── requirements/    # Requirements specs
│   ├── adrs/            # Architecture Decision Records
│   ├── context/         # Context files (6 files)
│   └── brainstorms/     # Brainstorm sessions
```

**Validation:** All directories must exist before artifact generation

---

**Token Budget:** ~500-1,000 tokens per directory operation
