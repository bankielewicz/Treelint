# Technical Debt Atomic Write Workflow

**Skill Reference:** devforgeai-development
**Parent Workflow:** technical-debt-register-workflow.md
**Section:** 6.6.7-6.6.9 (Atomic Write, Error Handling)

---

## Purpose

Perform atomic writes to the technical debt register with rollback capability. Ensures no partial updates occur (NFR-003 compliance).

**Token Cost:** ~600-800 tokens (extracted from main workflow for progressive disclosure)

---

## Non-Functional Requirements

| NFR | Description |
|-----|-------------|
| NFR-003 | Atomic writes - no partial register updates |
| NFR-004 | Handle missing register by creating from template |

---

## Entry Insertion (Step 6.6.7)

```
# Strategy: Build complete updated content, then single Write()

# Step 1: Find "Open Debt Items" section marker
open_items_marker = "## Open Debt Items"

# Step 2: Find table header row
table_header = "| ID | Date | Source | Type | Priority | Status | Effort | Follow-up |"
table_separator = "|---|---|---|---|---|---|---|---|"

# Step 3: Insert new entry after table separator
Edit(
    file_path=register_path,
    old_string=table_separator,
    new_string=f"{table_separator}\n{new_entry}"
)

Display: "  ✓ Entry added to register"
```

---

## Analytics Update (Step 6.6.6)

```
# Step 4: Update all analytics counters (atomic batch)
FOR each update in analytics_updates:
    Edit(
        file_path=register_path,
        old_string=update.old,
        new_string=update.new
    )

Display: "  ✓ Analytics updated"
```

---

## Error Handling with Rollback (Step 6.6.9)

```
# Handle entry addition failure
IF Edit() fails for entry addition:
    HALT: "REGISTER UPDATE FAILED - Entry addition failed"
    Display: "❌ Could not add entry to register"
    Display: "   Check file permissions and format"
    EXIT with error

# Handle analytics update failure with rollback
IF Edit() fails for analytics update:
    Display: "⚠️ Analytics update failed - attempting rollback"

    # Remove the entry we just added (rollback)
    Edit(
        file_path=register_path,
        old_string=new_entry + "\n",
        new_string=""
    )

    HALT: "REGISTER UPDATE FAILED - Analytics update failed, entry rolled back"
    Display: "❌ Partial update prevented (NFR-003 atomic writes enforced)"
    EXIT with error
```

---

## Success Confirmation (Step 6.6.8)

```
IF register_update_successful:
    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "📋 TECHNICAL DEBT REGISTERED"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "  ✓ ID:       {DEBT_ID}"
    Display: "  ✓ Item:     {deferral.text}"
    Display: "  ✓ Type:     {type}"
    Display: "  ✓ Priority: {priority}"
    Display: "  ✓ Source:   dev_phase_06"
    Display: ""
    Display: "  📁 Register: devforgeai/technical-debt-register.md"
    Display: "  📊 Technical debt: {new_total_open} open items"
    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

---

## Error Display

```
ELSE:
    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "❌ TECHNICAL DEBT REGISTRATION FAILED"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "  Error: {error_message}"
    Display: "  Item:  {deferral.text}"
    Display: ""
    Display: "  Please check:"
    Display: "  - Register file exists and is writable"
    Display: "  - YAML frontmatter is valid"
    Display: "  - Template available if register needs creation"
    Display: ""
    HALT: "Register update failed - see error above"
```

---

## Tools Used

| Tool | Purpose |
|------|---------|
| Edit() | Insert entry row, update analytics |
| Write() | Create register from template (NFR-004) |
| Read() | Load register content for parsing |

---

## Integration

**Called by:** technical-debt-register-workflow.md (Steps 6.6.7-6.6.9)
**Inputs:** new_entry (string), analytics_updates (list), register_path (string)
**Outputs:** register_update_successful (boolean)

---

## Change Log

| Date | Change | Reference |
|------|--------|-----------|
| 2026-01-24 | Extracted from technical-debt-register-workflow.md | STORY-305 |
