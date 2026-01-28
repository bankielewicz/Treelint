devforgeai-validate  Examples below

# Check hooks enabled
Execute: devforgeai-validate check-hooks --operation=create-sprint --status=success

# Conditional invocation (non-blocking)
IF check-hooks exit == 0:
    Execute: devforgeai-validate invoke-hooks --operation=create-sprint --sprint-name="${SPRINT_NAME}" --story-count=${STORY_COUNT} --capacity=${CAPACITY_POINTS}

    IF invoke-hooks fails:
        Log to: devforgeai/feedback/logs/hook-errors.log
        Display: "⚠️ Feedback collection failed (sprint creation succeeded)"
		
# Validate Context Files Exist
Bash(command="devforgeai-validate validate-context", description="Validate 6 context files exist")

IF validation fails:
  Display: "❌ Context files missing or invalid"
  Display: "   Run: /create-context to generate context files"
  HALT

Display: "✓ Context files validated (6/6 present)"

# Initialize phase tracking before any TDD work:

```bash
# Initialize state file for this story
devforgeai-validate phase-init ${STORY_ID} --project-root=.
```
# Phase Check
devforgeai-validate phase-check ${STORY_ID} --from={N-1} --to={N}
