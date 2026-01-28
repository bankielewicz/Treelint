# Technical Debt ID Generation Workflow

**Skill Reference:** devforgeai-development
**Parent Workflow:** technical-debt-register-workflow.md
**Section:** 6.6.3 (ID Generation) + 6.6.11 (Edge Cases)

---

## Purpose

Generate sequential DEBT-NNN identifiers for new technical debt entries with collision detection and edge case handling.

**Token Cost:** ~500-700 tokens (extracted from main workflow for progressive disclosure)

---

## ID Format Specification

| Format | Description | Example |
|--------|-------------|---------|
| DEBT-NNN | 3-digit zero-padded | DEBT-001, DEBT-042, DEBT-999 |

---

## ID Generation Algorithm

```
# Step 1: Read register to parse existing IDs
register_content = Read(file_path=register_path)

# Step 2: Extract all existing DEBT-NNN IDs using regex
# Pattern: DEBT-[0-9]{3} (3-digit zero-padded)
existing_ids = Grep(
    pattern="DEBT-[0-9]{3}",
    path=register_path,
    output_mode="content"
)

# Step 3: Parse IDs and find maximum
max_id = 0
FOR each match in existing_ids:
    id_num = int(match.replace("DEBT-", ""))
    IF id_num > max_id:
        max_id = id_num

# Step 4: Generate next sequential ID
next_id_num = max_id + 1
DEBT_ID = f"DEBT-{next_id_num:03d}"

Display: "  Generated ID: {DEBT_ID}"
```

---

## Edge Case: Empty Register (DEBT-001)

```
# When register is empty (no existing DEBT-NNN IDs):
IF max_id == 0:
    # Register is empty - first ID must be DEBT-001
    next_id_num = 1
    DEBT_ID = "DEBT-001"

Display: "ℹ️ Empty register detected - generating DEBT-001"
```

---

## Collision Detection Loop

```
# Verify ID doesn't already exist (handles manual additions)
WHILE DEBT_ID in existing_ids:
    next_id_num += 1
    DEBT_ID = f"DEBT-{next_id_num:03d}"

# Example: Register has DEBT-001, DEBT-002, DEBT-005 (manual)
# max_id = 5 (from DEBT-005)
# next_id_num = 6
# DEBT_ID = "DEBT-006" (skips gap, uses next sequential)
```

---

## Edge Case: Maximum ID Exceeded (DEBT-999)

```
# When DEBT-999 already exists and new ID needed:
IF next_id_num > 999:
    HALT: "DEBT ID SPACE EXHAUSTED - exceeded 999"
    Display: "❌ Cannot generate new DEBT ID after DEBT-999"
    Display: "   Maximum 3-digit ID space exhausted"
    Display: "   Consider archiving resolved debt items"
    EXIT with error
```

---

## Multiple Deferrals in Same Session

```
# Track IDs generated in this session to prevent collisions
session_generated_ids = []

FOR each approved_deferral:
    WHILE DEBT_ID in existing_ids OR DEBT_ID in session_generated_ids:
        next_id_num += 1
        DEBT_ID = f"DEBT-{next_id_num:03d}"

    session_generated_ids.append(DEBT_ID)

Display: "  ✓ {len(session_generated_ids)} deferrals registered in this session"
```

---

## Integration

**Called by:** technical-debt-register-workflow.md (Step 6.6.3)
**Returns:** DEBT_ID (string, format DEBT-NNN)

---

## Change Log

| Date | Change | Reference |
|------|--------|-----------|
| 2026-01-24 | Extracted from technical-debt-register-workflow.md | STORY-305 |
