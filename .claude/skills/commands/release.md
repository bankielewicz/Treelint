---
description: Release story to target environments with validation
argument-hint: [STORY-ID] [environment]
# Environment: 'staging' or 'production' (no -- prefix)
model: opus
allowed-tools: Read, Skill, AskUserQuestion, Glob
execution-mode: immediate
---

# /release - Release to Environment

Release QA-approved stories to target environments with automated validation and rollback capabilities.

---

## Quick Reference

```bash
# Safe release (defaults to test environment)
/release STORY-001

# Release to production environment
/release STORY-001 production

# Release to test environment (explicit)
/release STORY-001 test
```

---

## Command Workflow

### Phase 0: Plan Mode Detection and Argument Validation

**Step 0.0: Plan Mode Auto-Exit [execution-mode: immediate]**

This command has `execution-mode: immediate` in frontmatter. If plan mode is currently active, auto-exit plan mode before proceeding:

```
IF plan mode is active:
    Display: "Note: /release is an execution command. Exiting plan mode automatically."
    ExitPlanMode()
```

---

**Step 0.1: Validate Story ID**
- Check $1 matches format `STORY-[0-9]+`
- If invalid: Use AskUserQuestion to get valid story ID
- If empty: Prompt user for story ID

**Step 0.2: Verify story file exists**
- Glob: `devforgeai/specs/Stories/$1*.story.md`
- If not found: Guide user to list available stories

**Step 0.3: Load story context**
- Reference: `@devforgeai/specs/Stories/{STORY_ID}.story.md`
- Story content now available in conversation for skill extraction

**Step 0.4: Parse and Normalize Environment**

- Default: "test" environment (safe default if $2 not provided)
- If $2 provided: Normalize short forms to full names
- Valid values: test environment or production environment
- If invalid: AskUserQuestion for valid environment

**Step 0.5: Confirm and Proceed**

- Display: ✓ Story ID: {STORY_ID}, ✓ Environment: {ENVIRONMENT}
- Proceed to Step 0.6

---

**Step 0.6: W3 Compliance Check**

Ensure no CRITICAL W3 violations before release:

```
# Run W3 audit in quiet mode
w3_result = Invoke: /audit-w3 --quiet

IF w3_result.exit_code == 1:
    Display:
    "╔════════════════════════════════════════════════════════════════════╗"
    "║  ❌ RELEASE BLOCKED: CRITICAL W3 Violations Detected              ║"
    "╠════════════════════════════════════════════════════════════════════╣"
    "║  W3 violations prevent release to avoid token overflow and        ║"
    "║  lean orchestration principle violations.                         ║"
    "║                                                                   ║"
    "║  Run `/audit-w3` for detailed violation report.                   ║"
    "║  Run `/audit-w3 --fix-hints` for remediation patterns.            ║"
    "╚════════════════════════════════════════════════════════════════════╝"
    HALT

Display: "✓ W3 compliance check passed"
```

---

### Phase 1: Invoke Release Skill

**Set context markers for skill:**

Clear markers enable skill parameter extraction from conversation context.

**Story ID:** [validated from Phase 0]
**Environment:** [parsed and normalized from Phase 0]

Skill extracts these values from YAML frontmatter and context markers in conversation.

**Single orchestration point:**
```
Skill(command="devforgeai-release")
```

**What skill executes:**
- Validation (QA approval, tests, configuration)
- Target environment operations (from context)
- Health checks and smoke tests
- Hooks integration (retrospective feedback collection)
- Release documentation (notes, audit trail)
- Monitoring configuration

See devforgeai-release skill for complete 8-phase workflow.

---

### Phase 2: Display Results

Output skill response containing:
- Deployment status
- Release notes
- Rollback information (if applicable)

---

## Error Handling

**Story ID Invalid:** Display format requirement and use AskUserQuestion for correction

**Story File Not Found:** Guide user to list available stories and recheck ID

**Invalid Environment:** Show valid options and ask user to clarify

**Skill Execution Failed:** Display skill error output and suggest recovery actions

---

## Success Criteria

- [ ] Story ID validated and file found
- [ ] Environment determined from arguments
- [ ] Release skill invoked successfully
- [ ] Release process completed successfully
- [ ] Health checks passed
- [ ] Release notes generated
- [ ] Story updated in workflow
- [ ] Recovery information available
- [ ] Post-release monitoring active

---

## Integration with Framework

**Invoked by:**
- Developer: `/release STORY-ID [environment]`
- devforgeai-orchestration skill (automated progression)

**Invokes:**
- devforgeai-release skill (8-phase release workflow)

**Skill executes 6 core phases:**
1. Pre-release validation (QA approval, tests, config)
2. Environment-specific release process
3. Health verification and smoke tests
4. Hook integration for retrospective feedback
5. Release documentation and audit trail
6. Post-release monitoring setup

**Quality gates:**
- Release Readiness gate enforced
- Requires: Story status "QA Approved"

**Story updates:**
- Status transition recorded
- Workflow history annotated
- Release information linked

---

## Related Commands

- `/qa STORY-ID` - Run QA validation before release
- `/dev STORY-ID` - Return to development if needed
- `/orchestrate STORY-ID` - Full lifecycle (dev → qa → release)

---

## Workflow Overview

**Typical release sequence:**
1. `/dev STORY-001` - Implement and test
2. `/qa STORY-001` - Validate quality (deep mode)
3. `/release STORY-001` - Release (defaults to test environment)
4. Verify test environment release works
5. `/release STORY-001 production` - Release to production
6. Monitor post-release for 24 hours

---

## Hook Integration (STORY-025)

**Retrospective feedback collection after deployments:**

The devforgeai-release skill automatically triggers feedback hooks:

**Phase 2.5: Post-Staging Hooks**
- Triggered after staging deployment (success or failure)
- Collects deployment feedback
- Non-blocking (never affects deployment)

**Phase 3.5: Post-Production Hooks**
- Triggered after production deployment
- Default: Failures-only (feedback on failures only)
- Non-blocking (never affects deployment)

See `devforgeai/docs/hook-integration-pattern.md` for details.

---

## Implementation Notes

**Architecture (Refactored 2025-11-18 - Lean Orchestration Pattern):**

This command follows **lean orchestration** pattern:
- **Argument validation:** Minimal (50 lines)
- **Story loading:** Via @file reference
- **Context markers:** Explicit parameters for skill extraction
- **Skill invocation:** Single call (1 line)
- **Result display:** Pass-through (no parsing)
- **Error handling:** Minimal (skill communicates errors)

**Total: 320 lines (vs 655 before) = 51% reduction**

**Before refactoring:**
- Lines: 655
- Characters: 18,166 (121% over 15K budget)
- Business logic: Deployment steps, error matrices, templates in command
- Token usage: ~8K in main conversation

**After refactoring:**
- Lines: 320
- Characters: 9,547 (64% of budget, compliant)
- Business logic: All moved to devforgeai-release skill
- Token usage: ~2.5K in main conversation (69% savings)

**Business logic location:**
- Argument validation: Command (minimal, 50 lines)
- Deployment orchestration: devforgeai-release skill (8 phases)
- Pre-release checks: Skill Phase 1
- Staging deployment: Skill Phase 2
- Production deployment: Skill Phase 3
- Smoke tests: Skill Phase 4
- Documentation: Skill Phase 5
- Monitoring: Skill Phase 6
- Hooks: Skill Phases 2.5, 3.5
- Command: Pure orchestration (validate → invoke → display)

**Lean pattern compliance:**
- ✅ Phase 0: Argument validation (~50 lines)
- ✅ Phase 1: Context markers (explicit parameters)
- ✅ Phase 2: Skill invocation (single call)
- ✅ Phase 3: Result display (pass-through)
- ✅ No business logic in command (all in skill)
- ✅ No branching on deployment types (delegated to skill)
- ✅ No template generation (delegated to skill)
- ✅ No file parsing (delegated to skill)
- ✅ No complex error handling (skill handles)

**Token efficiency:**
- Command overhead: ~2.5K tokens (down from ~8K)
- Skill deployment: ~40K tokens (isolated context)
- Main conversation: ~2.5K (command only)
- Savings vs pre-refactoring: 69%

---

**Last Updated:** 2025-11-18
**Status:** Lean Orchestration Compliant
**Model:** Sonnet
**Character Budget:** 9,547 / 15,000 (64% - WITHIN BUDGET)
