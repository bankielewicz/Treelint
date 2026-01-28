# Error Type 2: Artifact Generation Failures

Handling file creation errors during epic and requirements generation.

---

## Error Detection

**Symptom:** Epic or requirements file not created due to write errors, permissions, or path issues

**Detected during:** Phase 6.1 (Artifact Generation)

**Examples:**
- Write() tool returns permission error
- Directories don't exist (`devforgeai/specs/Epics/` not found)
- Disk full or read-only filesystem
- Invalid filename characters

**Detection logic:**

```
try:
    Write(file_path="devforgeai/specs/Epics/EPIC-001.epic.md", content=epic_content)
except FileWriteError as error:
    trigger artifact_generation_failure_recovery(error)
```

---

## Recovery Procedures

### Step 1: Check Directory Permissions

```
# Ensure directories exist using Write/.gitkeep pattern (Constitutional C1 compliant)
# See error-type-6-directory-issues.md for detailed directory structure validation
required_dirs = [
    "devforgeai/specs/Epics/",
    "devforgeai/specs/requirements/",
    "devforgeai/specs/adrs/",
    "devforgeai/specs/context/"
]

for dir in required_dirs:
    # Create if missing
    Write(file_path=f"{dir}.gitkeep", content="")

# Retry write operation
Write(file_path="devforgeai/specs/Epics/EPIC-001.epic.md", content=epic_content)
```

### Step 2: Verify File System Paths

```
# Check if directories were created successfully
for dir in required_dirs:
    check = Glob(pattern=f"{dir}")

    if not check:
        ERROR: Cannot create directory {dir}

        Report to user: """
        ❌ File system permissions prevent directory creation

        Manual steps required:
        1. Create directory: {dir}
        2. Grant write permissions
        3. Re-run ideation skill

        Or run:
        bash
        mkdir -p {dir}
        chmod 755 {dir}

        Then retry.
        """
```

### Step 3: Retry with Error Handling

```
max_retries = 2

for artifact in artifacts_to_create:
    attempt = 0

    while attempt < max_retries:
        try:
            Write(file_path=artifact.path, content=artifact.content)
            ✓ Created {artifact.path}
            break
        except FileWriteError:
            attempt += 1
            if attempt >= max_retries:
                # Max retries exceeded
                Report error with manual creation instructions
```

### Step 4: Provide Manual Creation Instructions

```
If persistent failure after retries:
    Report: """
    ❌ Could not create {artifact_name} automatically

    Manual creation steps:
    1. Create file: {file_path}
    2. Copy this content:

    ```
    {full_artifact_content}
    ```

    3. Save file
    4. Continue to next epic or run /create-context
    """
```

---

## Example Scenarios

### Scenario 1: Missing Directory

**Error:** `devforgeai/specs/Epics/` directory not found

**Recovery:**
1. Create directory using Write/.gitkeep pattern
2. Retry artifact creation
3. If still fails, provide manual mkdir instructions

### Scenario 2: Permission Denied

**Error:** Cannot write to `devforgeai/specs/Epics/EPIC-001.epic.md`

**Recovery:**
1. Check if directory exists
2. Report permission error to user
3. Provide chmod command for manual fix

### Scenario 3: Disk Full

**Error:** No space left on device

**Recovery:**
1. Report disk space issue
2. Provide full artifact content for manual creation
3. Suggest disk cleanup before retry

---

## Max Recovery Attempts

**Attempt 1:** Create directories, retry write
**Attempt 2:** Verify paths, retry with error handling

**If still failing:** Provide manual creation instructions, continue with remaining artifacts

---

## Related Patterns

- See [error-type-6-directory-issues.md](error-type-6-directory-issues.md) for directory structure validation
- See [error-handling-index.md](error-handling-index.md) for error type decision tree
- Constitutional C1 compliant: Use Write/.gitkeep instead of Bash mkdir

---

## Phase Context

This error occurs during **Phase 6.1: Artifact Generation** when the ideation skill attempts to create epic or requirements specification files.

It may also be encountered during **Phase 6.4: Self-Validation** if artifacts cannot be read back after creation.

---

**Token Budget:** ~500-1,500 tokens per recovery attempt
