# Step 6.6: Technical Debt Register Update Workflow

**Skill Reference:** devforgeai-development
**Phase:** Phase 06 (Deferral Challenge)
**Step:** Step 6.6 (executed after user approves a deferral in Step 6)
**Token Cost:** ~3,000-4,000 tokens (progressive disclosure from phase-06-deferral-challenge.md)

**Purpose:** Automatically add user-approved deferrals to the technical debt register when user approves a deferral (UNCONDITIONAL - no opt-out).

**Trigger:** User selects "Keep deferred (blocker is valid)" OR "Update justification (blocker changed)" in Step 6

**Integration Pattern:** The phase-06-deferral-challenge.md loads this reference via:
```
Read(file_path=".claude/skills/devforgeai-development/references/technical-debt-register-workflow.md")
```

---

## Business Rules

| Rule | Description |
|------|-------------|
| BR-001 | Register update is UNCONDITIONAL (no opt-out, no conditions) |
| BR-002 | Source field MUST be "dev_phase_06" (constant) |
| BR-003 | User approval timestamp MUST exist before register write (prevents autonomous writes) |
| BR-004 | Type derived from justification text patterns (default: External Blocker) |

---

## Non-Functional Requirements

| NFR | Description |
|-----|-------------|
| NFR-003 | Atomic writes - no partial register updates |
| NFR-004 | Handle missing register by creating from template |

---

## 6.6.1 Pre-Flight Check (BR-003 Enforcement)

```
# MANDATORY: Verify user approval timestamp exists before ANY register write
# This prevents autonomous writes (BR-003)

IF NOT deferral.user_approval_timestamp:
    HALT: "AUTONOMOUS WRITE ATTEMPTED - User approval timestamp missing"
    Display: "❌ Cannot update register without user approval"
    Display: "   Re-run Step 6 to get user approval for this deferral"
    EXIT Step 6.6
```

---

## 6.6.2 Register Existence Check (NFR-004)

```
# Check if register exists
register_path = "devforgeai/technical-debt-register.md"
template_path = ".claude/skills/devforgeai-orchestration/assets/templates/technical-debt-register-template.md"

register_exists = Glob(pattern=register_path)

IF NOT register_exists:
    # Create from template (NFR-004)
    Display: "ℹ️ Register not found - creating from template"

    template_content = Read(file_path=template_path)

    IF template_content fails:
        HALT: "TEMPLATE NOT FOUND - Cannot create register"
        Display: "❌ Template missing at: {template_path}"
        EXIT Step 6.6

    Write(file_path=register_path, content=template_content)
    Display: "✓ Register created from template"
```

---

## 6.6.3 DEBT-NNN ID Generation (AC#2)

For full details, see: [technical-debt-id-generation.md](technical-debt-id-generation.md) (ID Generation Algorithm)

**Summary:** Generate sequential DEBT-NNN identifiers with collision detection and edge case handling for empty registers and ID exhaustion.

---

## 6.6.4 Derive Entry Fields (AC#3)

For full details, see: [technical-debt-type-derivation.md](technical-debt-type-derivation.md) (Type Derivation Algorithm)

**Summary:** Derive debt type from justification patterns (Story Split, Scope Change, External Blocker), extract priority/effort from story frontmatter.

---

## 6.6.5 Build Entry Row (AC#3)

```
# Build markdown table row with all 8 fields
# Format: | ID | Date | Source | Type | Priority | Status | Effort | Follow-up |

entry_row = f"| {DEBT_ID} | {current_date} | {source} | {type} | {priority} | {status} | {effort} | {follow_up} |"

# Add description as comment below row
entry_description = f"<!-- {DEBT_ID}: {deferral.text} -->"

# Combined entry
new_entry = f"{entry_row}\n{entry_description}"

# ENTRY VALIDATION BEFORE WRITE:
# validate entry before writing to register
# check before write that all required fields are populated
# verify fields are present before attempting write
IF NOT all([DEBT_ID, current_date, source, type, priority, status]):
    HALT: "ENTRY VALIDATION FAILED - Required fields missing"
    Display: "❌ Cannot write entry - missing required fields"
    EXIT Step 6.6 with error

# Return DEBT_ID for display in confirmation
# The generated ID must be available for display in Step 6.6.8
return_debt_id = DEBT_ID  # ID returned for display

Display: "  Entry built: {DEBT_ID}"
```

---

## 6.6.6 Update Analytics Counters (AC#4)

```
# Read current analytics from YAML frontmatter
# Analytics are in YAML frontmatter between --- markers
# IMPORTANT: Preserve all other frontmatter fields (version, description, etc.)

# Parse current counters
current_total_open = extract_yaml_value("total_open", register_content)
current_by_type = extract_yaml_value(f"by_type.{type.lower().replace(' ', '_')}", register_content)
current_by_priority = extract_yaml_value(f"by_priority.{priority.lower()}", register_content)
current_by_source = extract_yaml_value("by_source.dev_phase_06", register_content)

# Calculate new values - INCREMENT each counter
new_total_open = current_total_open + 1
new_by_type = current_by_type + 1       # INCREMENT by_type[type] counter
new_by_priority = current_by_priority + 1  # INCREMENT by_priority[priority] counter
new_by_source = current_by_source + 1

# TYPE COUNTER MAPPINGS (by_type section):
# - "Story Split" → increment by_type.story_split counter
# - "Scope Change" → increment by_type.scope_change counter
# - "External Blocker" → increment by_type.external_blocker counter

# PRIORITY COUNTER MAPPINGS (by_priority section):
# - "Critical" → increment by_priority.critical counter
# - "High" → increment by_priority.high counter
# - "Medium" → increment by_priority.medium counter
# - "Low" → increment by_priority.low counter

# Build analytics update edits
analytics_updates = [
    {"old": f"total_open: {current_total_open}", "new": f"total_open: {new_total_open}"},
    {"old": f"{type.lower().replace(' ', '_')}: {current_by_type}", "new": f"{type.lower().replace(' ', '_')}: {new_by_type}"},
    {"old": f"{priority.lower()}: {current_by_priority}", "new": f"{priority.lower()}: {new_by_priority}"},
    {"old": f"dev_phase_06: {current_by_source}", "new": f"dev_phase_06: {new_by_source}"},
    {"old": f"last_updated: {old_date}", "new": f"last_updated: {current_date}"}
]

# Preservation: Only update analytics counters, preserve all other frontmatter fields
# DO NOT modify: version, description, created_date, etc.

Display: "  Analytics prepared: total_open → {new_total_open}"
```

---

## 6.6.7-6.6.9 Atomic Write and Error Handling

For full details, see: [technical-debt-atomic-write.md](technical-debt-atomic-write.md) (Atomic Write Workflow)

**Summary:** Perform atomic writes with NFR-003 compliance, insert entry rows, update analytics counters, handle errors with rollback capability.

---

## 6.6.10 Multiple Deferrals Handling

```
# If multiple deferrals approved in same session, each gets sequential ID

# Track IDs generated in this session to prevent collisions
session_generated_ids = []

FOR each approved_deferral:
    # Generate ID (accounting for session IDs)
    WHILE DEBT_ID in existing_ids OR DEBT_ID in session_generated_ids:
        next_id_num += 1
        DEBT_ID = f"DEBT-{next_id_num:03d}"

    # Execute Steps 6.6.4 - 6.6.8 for this deferral
    # ...

    # Track ID
    session_generated_ids.append(DEBT_ID)

Display: "  ✓ {len(session_generated_ids)} deferrals registered in this session"

# Sequential ID Example:
# First deferral → DEBT-001
# Second deferral (same session) → DEBT-002
# Third deferral (same session) → DEBT-003
```

---

## 6.6.11 Edge Cases (MANDATORY HANDLING)

Edge case handling is implemented in the helper files:

- **Empty Register:** See [technical-debt-id-generation.md](technical-debt-id-generation.md) (Edge Case: Empty Register)
- **ID Collision Detection:** See [technical-debt-id-generation.md](technical-debt-id-generation.md) (Collision Detection Loop)
- **Maximum ID Exceeded:** See [technical-debt-id-generation.md](technical-debt-id-generation.md) (Edge Case: Maximum ID Exceeded)
- **Malformed YAML:** See [technical-debt-atomic-write.md](technical-debt-atomic-write.md) (Error Handling)

---

## Integration Notes

**Invoked by:** phase-06-deferral-challenge.md (Step 6.6)

**Trigger Conditions:**
- User selected "Keep deferred (blocker is valid)" in Step 6
- User selected "Update justification (blocker changed)" in Step 6

**Updates:**
- `devforgeai/technical-debt-register.md` (entry + analytics)

**References:**
- `.claude/skills/devforgeai-orchestration/assets/templates/technical-debt-register-template.md` (template)
- `devforgeai/technical-debt-register.md` (register file)

**Token Efficiency:**
- Progressive disclosure: This file is loaded ONLY when user approves a deferral
- Estimated: ~3,000-4,000 tokens per execution
- Extracted from phase-06-deferral-challenge.md to reduce base token cost

---

## Change Log

| Date | Change | Reference |
|------|--------|-----------|
| 2026-01-24 | Extracted from phase-06-deferral-challenge.md | STORY-304 |
| 2026-01-24 | Refactored nested conditionals into 3 helper files | STORY-305 |
