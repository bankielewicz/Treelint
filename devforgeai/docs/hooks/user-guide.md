# User Guide: Phase 6 Hook Integration Design for /dev Command

**Status:** DESIGN DOCUMENTATION ONLY - Not yet implemented in executable code
**Story:** STORY-023 (Design phase complete, implementation pending)

---

## Current Implementation Status

**What exists:**
- ✅ Design documentation in `.claude/commands/dev.md` (Phase 2.3)
- ✅ Integration test suite (23 tests) - tests the design pattern
- ✅ CLI tools available: `devforgeai check-hooks`, `devforgeai invoke-hooks`

**What does NOT exist yet:**
- ❌ Actual executable Phase 6 code in devforgeai-development skill
- ❌ Live hook triggering when running `/dev` command
- ❌ Configuration file `devforgeai/config/hooks.yaml` (will be created during implementation)

**Next step:** Actual implementation in devforgeai-development skill

---

## Planned Configuration (Design)

**When implemented, configuration will be** (`devforgeai/config/hooks.yaml`):
```yaml
hooks:
  enabled: true           # Master switch for all hooks
  mode: "all"            # Options: "all", "failures_only", "none"
  operations:
    dev:
      enabled: true       # Enable hooks for /dev command
      on_success: true    # Trigger on successful completion
      on_failure: false   # Don't trigger on failure
```

**Planned behavior:**
1. When `devforgeai/config/hooks.yaml` has `hooks.enabled: true`
2. And `hooks.operations.dev.enabled: true`
3. And trigger conditions match (on_success or on_failure)
4. Then `/dev STORY-ID` will trigger feedback conversation automatically

## Planned Disable Options (Design)

**These are planned configurations for when Phase 6 is implemented:**

**Option 1:** Disable all hooks globally (not yet testable)
**Option 2:** Disable only /dev hooks (not yet testable)
**Option 3:** Use failures-only mode (not yet testable)
**Option 4:** Skip tracking auto-disable (not yet testable)

## Actual Current State

**What you can do NOW:**
- Test the CLI tools directly:
  ```bash
  devforgeai check-hooks --operation=dev --status=completed
  devforgeai invoke-hooks --operation=dev --story=STORY-023
  ```

**What you CANNOT do yet:**
- Hooks don't automatically trigger from `/dev` command
- Configuration file doesn't affect `/dev` behavior yet
- Skip tracking not active in live `/dev` runs

## Design Reference

For the complete planned design, see:
- `.claude/commands/dev.md` (Phase 2.3) - Shows intended Phase 6 code
- Test suite validates the design pattern will work when implemented

---

**Created:** 2025-11-13 (STORY-023)
**Related:** STORY-021 (check-hooks), STORY-022 (invoke-hooks), EPIC-006
