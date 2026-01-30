---
description: Manually trigger feedback collection with optional context
argument-hint: [context]
model: opus
allowed-tools: Skill
---

# /feedback - Manual Feedback Trigger

Manually capture feedback with optional context (story ID, operation details).

---

## Quick Reference

```bash
# Basic usage (no context)
/feedback

# With story context
/feedback STORY-001 after-dev-completion

# With operation context
/feedback regression-testing phase-1

# With detailed context
/feedback STORY-042 qa-validation performance-issue-detected
```

---

## Command Workflow

### Phase 0: Parse Arguments

**Extract context (optional):**
```
CONTEXT = All arguments joined as string
```

**Validate context:**
- Max length: 500 characters
- Allowed characters: alphanumeric, hyphens, underscores, spaces
- If exceeds limit or contains invalid characters → Error with guidance

---

### Phase 1: Invoke devforgeai-feedback Skill

**Set context markers:**
```
**Feedback Context:** ${CONTEXT}
**Feedback Source:** manual
```

**Invoke skill:**
```
Skill(command="devforgeai-feedback")
```

**What the skill does:**
1. Generate unique feedback ID (FB-YYYY-MM-DD-###)
2. Capture session metadata (timestamp ISO8601, story ID, operation type)
3. Record feedback in feedback-register.md
4. Return confirmation with feedback ID and next steps

---

### Phase 2: Display Results

**Success response:**
```json
{
  "status": "success",
  "feedback_id": "FB-2025-11-07-001",
  "timestamp": "2025-11-07T14:30:00Z",
  "context": "STORY-001 after-dev-completion",
  "next_steps": "Feedback captured. View recent feedback with: /feedback-search --limit=5",
  "message": "Feedback captured successfully"
}
```

**Error response:**
```json
{
  "status": "error",
  "message": "Context exceeds maximum length of 500 characters (received: 537)",
  "suggested_action": "Reduce context length and retry"
}
```

---

## Error Handling

### Invalid Characters in Context
```
Error: Context contains invalid characters
Constraint: Only alphanumeric, hyphens, underscores, and spaces allowed
Action: Remove special characters and retry
```

### Context Too Long
```
Error: Context exceeds maximum length of 500 characters
Constraint: Max 500 characters
Action: Reduce context length and retry
```

### Feedback Directory Not Writable
```
Error: Cannot write to devforgeai/feedback/ directory
Constraint: Requires write permissions
Action: Check directory permissions and retry
```

---

## Success Criteria

- [x] Feedback ID generated in format FB-YYYY-MM-DD-###
- [x] Timestamp in ISO8601 format
- [x] Context captured (if provided)
- [x] Entry written to feedback-register.md
- [x] Confirmation displayed with next steps
- [x] Validation prevents invalid inputs
- [x] Error messages are actionable
- [x] Exit code 0 (success) or 1 (error)

---

## Integration

**Invoked by:** User manually, devforgeai-orchestration (after operation completion)

**Invokes:** devforgeai-feedback skill (captures and stores feedback)

**Updates:** devforgeai/feedback/feedback-register.md (appends new entry)

**Used with:**
- `/feedback-search` - Query captured feedback
- `/export-feedback` - Export feedback history
- `/feedback-config` - Configure feedback system

---

## Performance

**Target Response Time:** <200ms

**Typical Usage:**
- Parse context: <10ms
- Generate ID: <50ms
- Write to register: <100ms
- Display response: <10ms

---

## Examples

### Example 1: Capture After Story Development
```bash
/feedback STORY-042 dev-complete all-tests-passing
```

**Result:**
```
Feedback captured: FB-2025-11-07-003
Timestamp: 2025-11-07T15:42:00Z
Context: STORY-042 dev-complete all-tests-passing

Next steps: View recent feedback with: /feedback-search --limit=5
```

### Example 2: Capture During Manual Testing
```bash
/feedback manual-testing found-edge-case-bug
```

**Result:**
```
Feedback captured: FB-2025-11-07-004
Timestamp: 2025-11-07T16:15:00Z
Context: manual-testing found-edge-case-bug
```

### Example 3: No Context
```bash
/feedback
```

**Result:**
```
Feedback captured: FB-2025-11-07-005
Timestamp: 2025-11-07T16:20:00Z
Context: N/A
```

---

## Troubleshooting

### Issue: "Context contains invalid characters"

**Symptoms:** Error when running `/feedback` with special characters

**Cause:** Context contains characters outside allowed set (alphanumeric, hyphens, underscores, spaces)

**Resolution:**
```bash
# ❌ Wrong (contains @, #, !, parentheses)
/feedback STORY-001 (high-priority) #urgent @review!

# ✅ Correct (only allowed characters)
/feedback STORY-001 high-priority urgent review
```

### Issue: "Context exceeds maximum length"

**Symptoms:** Error when providing long context string

**Cause:** Context > 500 characters

**Resolution:** Reduce context length
```bash
# ❌ Wrong (too long)
/feedback STORY-001 This is a very long context with lots of details about what happened...

# ✅ Correct (concise)
/feedback STORY-001 dev-phase performance-optimization
```

### Issue: "Cannot write to feedback directory"

**Symptoms:** Permission error when capturing feedback

**Cause:** Insufficient permissions on `devforgeai/feedback/` directory

**Resolution:**
```bash
# Check permissions
ls -la devforgeai/feedback/

# Fix permissions if needed
chmod 755 devforgeai/feedback/
```

### Issue: Feedback ID not incrementing

**Symptoms:** Multiple feedback entries have same ID on same day

**Cause:** Race condition or register file corruption

**Resolution:**
```bash
# Check feedback register for duplicates
cat devforgeai/feedback/feedback-register.md | grep "^## FB-"

# If duplicates found, manually edit to fix sequence numbers
```

---

## Related Commands

- `/feedback-search` - Query feedback history
- `/feedback-config` - Configure feedback system
- `/export-feedback` - Export feedback data
- `/qa` - QA validation (auto-triggers feedback)
- `/orchestrate` - Full lifecycle (includes feedback capture)

---

## See Also

- devforgeai-feedback skill documentation
- STORY-020: Feedback CLI Commands (implementation story)
- devforgeai/feedback/feedback-register.md (feedback storage)
- devforgeai/feedback/config.yaml (configuration file)
