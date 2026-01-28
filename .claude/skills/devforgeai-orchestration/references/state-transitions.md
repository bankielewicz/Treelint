# State Transition Rules Reference

Complete validation rules and requirements for all story workflow state transitions.

## Transition Matrix

| FROM State | TO State | Trigger | Validation Required | Can Fail? |
|------------|----------|---------|---------------------|-----------|
| Backlog | Architecture | Story assigned to sprint | Has acceptance criteria | Yes |
| Architecture | Ready for Dev | Context files complete | All 6 files exist & valid | No |
| Ready for Dev | In Development | Developer starts | Context loaded | No |
| In Development | Dev Complete | Dev workflow done | All tests pass, build succeeds | Yes |
| Dev Complete | QA In Progress | Auto or manual trigger | No compilation errors | No |
| QA In Progress | QA Approved | Deep validation PASS | All quality gates passed | N/A |
| QA In Progress | QA Failed | Deep validation FAIL | Violations detected | N/A |
| QA Failed | In Development | Developer fixes | Action items documented | No |
| QA Approved | Releasing | Release initiated | QA report PASS, no blockers | Yes |
| Releasing | Released | Deployment success | Health checks pass | Yes |
| Any State | Blocked | External dependency | Blocker documented | No |
| Blocked | Previous State | Blocker resolved | Blocker removal verified | No |

---

## Detailed Transition Rules

### Transition 1: Backlog → Architecture

**Trigger:**
- Story assigned to active sprint
- Sprint has begun or about to begin
- Story prioritized for immediate work

**Pre-conditions:**
```
✓ Story document exists: ai_docs/Stories/{story-id}.story.md
✓ Story has frontmatter with required fields:
  - id
  - title
  - epic
  - sprint
  - status: Backlog
  - points
  - priority
✓ Story has at least one acceptance criterion
✓ Acceptance criteria are testable (Given/When/Then format preferred)
✓ Story has technical specification (can be minimal)
```

**Validation Logic:**
```
Read(file_path="ai_docs/Stories/{story_id}.story.md")

# Parse frontmatter
metadata = parse_yaml_frontmatter(content)

# Check required fields
required_fields = ["id", "title", "epic", "sprint", "status", "points", "priority"]
FOR field in required_fields:
    IF field not in metadata:
        FAIL: "Missing required field: {field}"

# Validate status
IF metadata.status != "Backlog":
    FAIL: "Story not in Backlog state (current: {metadata.status})"

# Check acceptance criteria
criteria_section = extract_section("Acceptance Criteria")
IF criteria_section is None OR criteria_section.empty:
    FAIL: "Story has no acceptance criteria"

criteria_count = count_criteria(criteria_section)
IF criteria_count < 1:
    FAIL: "Story must have at least 1 acceptance criterion"

# Check technical specification exists
tech_spec = extract_section("Technical Specification")
IF tech_spec is None:
    WARN: "Story missing technical specification"
    AskUserQuestion: "Proceed without detailed technical spec?"

# Check dependencies resolved
dependencies = extract_section("Dependencies")
IF dependencies.has_blocking_dependencies:
    AskUserQuestion: "Story has blocking dependencies. Proceed or block?"

# All validations passed
PASS: Ready for Architecture transition
```

**Actions on Success:**
```
1. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="status: Backlog",
        new_string="status: Architecture")

2. Append workflow history:
   """
   ### {timestamp} - Architecture
   - Previous Status: Backlog
   - Action: Story assigned to sprint, moving to architecture validation
   - Next: Create/validate context files
   """

3. Invoke architecture skill:
   Skill(command="devforgeai-architecture")
```

**Can Transition Fail:** Yes
- Missing acceptance criteria → BLOCK
- Story not assigned to sprint → BLOCK
- Unresolved blocking dependencies → BLOCK or request user decision

---

### Transition 2: Architecture → Ready for Dev

**Trigger:**
- devforgeai-architecture skill completes
- All context files created/validated

**Pre-conditions:**
```
✓ Story in Architecture state
✓ devforgeai-architecture skill has run
```

**Validation Logic:**
```
# Check all 6 context files exist
context_files = [
    "devforgeai/specs/context/tech-stack.md",
    "devforgeai/specs/context/source-tree.md",
    "devforgeai/specs/context/dependencies.md",
    "devforgeai/specs/context/coding-standards.md",
    "devforgeai/specs/context/architecture-constraints.md",
    "devforgeai/specs/context/anti-patterns.md"
]

missing_files = []
FOR file in context_files:
    IF NOT file_exists(file):
        missing_files.append(file)

IF missing_files:
    FAIL: "Missing context files: {missing_files}"

# Validate files are not empty
FOR file in context_files:
    Read(file_path=file)
    IF file_size < 100:  # Arbitrary minimum
        FAIL: "{file} appears to be empty or placeholder"

    # Check for placeholder content
    IF "TODO" in content OR "TBD" in content OR "To be determined" in content:
        WARN: "{file} contains placeholder content"
        AskUserQuestion: "Context file has TODOs. Proceed anyway?"

# Validate tech stack is locked
Read(file_path="devforgeai/specs/context/tech-stack.md")
IF "LOCKED" not in content:
    WARN: "Tech stack not locked - may change during development"

# All validations passed
PASS: Ready for Dev transition
```

**Actions on Success:**
```
1. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="status: Architecture",
        new_string="status: Ready for Dev")

2. Check workflow box:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="- [ ] Architecture phase complete",
        new_string="- [x] Architecture phase complete")

3. Append workflow history:
   """
   ### {timestamp} - Ready for Dev
   - Previous Status: Architecture
   - Action: All context files created and validated
   - Context Files: 6/6 complete
   - Next: Developer can start TDD workflow
   """

4. Report:
   "✅ Architecture phase complete. Story ready for development."
```

**Can Transition Fail:** No (but can be delayed)
- If context files missing → Re-invoke devforgeai-architecture
- Will retry until context files valid

---

### Transition 3: Ready for Dev → In Development

**Trigger:**
- Developer manually starts work
- devforgeai-orchestration invoked with --story={id}

**Pre-conditions:**
```
✓ Story in Ready for Dev state
✓ All context files exist and valid
```

**Validation Logic:**
```
# Verify context files still exist
context_files_valid = verify_context_files_exist()

IF NOT context_files_valid:
    FAIL: "Context files missing or corrupted since architecture phase"
    ACTION: Re-run architecture phase

# Check for staleness (optional warning)
FOR file in context_files:
    age_days = (now - file_modified_time(file)).days
    IF age_days > 30:
        WARN: "Context file {file} is {age_days} days old"
        AskUserQuestion: "Context files may be stale. Review architecture?"

# All validations passed
PASS: Ready to start development
```

**Actions on Success:**
```
1. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="status: Ready for Dev",
        new_string="status: In Development")

2. Append workflow history:
   """
   ### {timestamp} - In Development
   - Previous Status: Ready for Dev
   - Action: Developer started TDD workflow
   - Next: Execute 6 phases (Context → Test-First → Implement → Refactor → Integrate → Git)
   """

3. Invoke development skill:
   Skill(command="devforgeai-development --story={story_id}")

4. Report:
   "🚀 Starting TDD development workflow for {story_id}"
```

**Can Transition Fail:** No
- Always succeeds once Ready for Dev validated

---

### Transition 4: In Development → Dev Complete

**Trigger:**
- devforgeai-development skill completes Phase 6 (Git workflow)
- All tests passing
- Build succeeds

**Pre-conditions:**
```
✓ Story in In Development state
✓ Development workflow Phase 6 complete
```

**Validation Logic:**
```
# Check build status
build_result = check_build_status()
IF build_result != "success":
    FAIL: "Build failed - cannot transition to Dev Complete"
    Detail: {build_errors}
    ACTION: Fix build errors, stay in In Development

# Check test status
test_result = check_test_status()
IF test_result.has_failures:
    FAIL: "Tests failing - cannot transition to Dev Complete"
    Detail: {failed_tests}
    ACTION: Fix failing tests, stay in In Development

# Verify light validation passed
# (devforgeai-qa ran during dev phases 3, 4, 5)
IF light_validation_blocked:
    FAIL: "Light validation found blocking issues"
    Detail: {validation_issues}
    ACTION: Fix validation issues, stay in In Development

# Check git workflow complete
IF NOT git_commits_pushed:
    WARN: "Changes not pushed to remote"
    AskUserQuestion: "Proceed to QA without pushing to remote?"

# All validations passed
PASS: Ready for Dev Complete transition
```

**Actions on Success:**
```
1. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="status: In Development",
        new_string="status: Dev Complete")

2. Check workflow box:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="- [ ] Development phase complete",
        new_string="- [x] Development phase complete")

3. Append workflow history:
   """
   ### {timestamp} - Dev Complete
   - Previous Status: In Development
   - Action: TDD workflow complete, all tests passing
   - Build Status: Success
   - Test Status: All passing
   - Next: Deep QA validation
   """

4. Report:
   "✅ Development complete. All tests passing. Ready for QA validation."
```

**Can Transition Fail:** Yes
- Build failures → BLOCK
- Test failures → BLOCK
- Light validation issues → BLOCK
- Must fix issues and retry

---

### Transition 5: Dev Complete → QA In Progress

**Trigger:**
- Automatic after Dev Complete
- Manual trigger via orchestration

**Pre-conditions:**
```
✓ Story in Dev Complete state
✓ Build succeeds
✓ Tests passing
```

**Validation Logic:**
```
# Double-check build and tests (safety check)
build_ok = verify_build_success()
tests_ok = verify_tests_passing()

IF NOT build_ok OR NOT tests_ok:
    FAIL: "Build or tests failed since Dev Complete"
    ACTION: Return to In Development

# Verify no compilation errors
compilation_errors = check_compilation()
IF compilation_errors:
    FAIL: "Compilation errors detected"
    ACTION: Return to In Development

# All validations passed
PASS: Ready to start QA
```

**Actions on Success:**
```
1. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="status: Dev Complete",
        new_string="status: QA In Progress")

2. Append workflow history:
   """
   ### {timestamp} - QA In Progress
   - Previous Status: Dev Complete
   - Action: Starting deep QA validation
   - Validation Phases: 5 (Coverage, Anti-Patterns, Spec, Quality, Report)
   - Next: QA Approved or QA Failed
   """

3. Invoke QA skill:
   Skill(command="devforgeai-qa --mode=deep --story={story_id}")

4. Report:
   "🔍 Starting deep QA validation..."
```

**Can Transition Fail:** No
- Automatic transition if validations pass
- If build/tests fail, returns to In Development

---

### Transition 6A: QA In Progress → QA Approved

**Trigger:**
- devforgeai-qa skill completes with status: PASS

**Pre-conditions:**
```
✓ Story in QA In Progress state
✓ QA validation complete
✓ QA result: PASS
```

**Validation Logic:**
```
# Read QA report
Read(file_path="devforgeai/qa/reports/{story_id}-qa-report.md")

# Parse QA status
qa_status = parse_qa_status(report)
IF qa_status != "PASS":
    FAIL: "QA report exists but status is not PASS"

# Verify coverage thresholds met
coverage = parse_coverage(report)
IF coverage.business_logic < 95%:
    FAIL: "Business logic coverage {coverage.business_logic}% < 95%"
IF coverage.application < 85%:
    FAIL: "Application coverage {coverage.application}% < 85%"
IF coverage.infrastructure < 80%:
    FAIL: "Infrastructure coverage {coverage.infrastructure}% < 80%"
IF coverage.overall < 80%:
    FAIL: "Overall coverage {coverage.overall}% < 80%"

# Verify zero critical violations
critical_count = count_violations(report, severity="CRITICAL")
IF critical_count > 0:
    FAIL: "{critical_count} CRITICAL violations detected"

# Verify zero high violations (or approved exceptions)
high_count = count_violations(report, severity="HIGH")
IF high_count > 0:
    IF NOT has_approved_exceptions(report):
        FAIL: "{high_count} HIGH violations without approval"

# Verify all acceptance criteria validated
criteria_validated = check_criteria_validation(report)
IF NOT criteria_validated:
    FAIL: "Not all acceptance criteria validated"

# All validations passed
PASS: QA Approved
```

**Actions on Success:**
```
1. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="status: QA In Progress",
        new_string="status: QA Approved")

2. Check workflow box:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="- [ ] QA phase complete",
        new_string="- [x] QA phase complete")

3. Append QA results section to story:
   """
   ## QA Results

   **Date:** {timestamp}
   **Status:** ✅ PASS
   **Validator:** devforgeai-qa skill

   ### Summary
   - Overall Coverage: {coverage_percentage}%
   - Critical Issues: 0
   - High Issues: 0
   - Medium Issues: {medium_count}
   - Low Issues: {low_count}

   ### Report
   - Full Report: `devforgeai/qa/reports/{story_id}-qa-report.md`

   ### Next Steps
   - Story approved for release
   - Can proceed to production deployment
   """

4. Append workflow history:
   """
   ### {timestamp} - QA Approved
   - Previous Status: QA In Progress
   - Action: Deep QA validation PASSED
   - Coverage: Business {bl}%, Application {app}%, Infrastructure {inf}%, Overall {overall}%
   - Violations: 0 Critical, 0 High, {medium} Medium, {low} Low
   - Next: Ready for release
   """

5. Report:
   """
   ✅ QA Validation PASSED

   Coverage:
   - Business Logic: {coverage.business_logic}%
   - Application: {coverage.application}%
   - Infrastructure: {coverage.infrastructure}%
   - Overall: {coverage.overall}%

   Quality:
   - Zero CRITICAL violations
   - Zero HIGH violations
   - All acceptance criteria validated

   Status: Story approved for release!
   """
```

---

### Transition 6B: QA In Progress → QA Failed

**Trigger:**
- devforgeai-qa skill completes with status: FAIL

**Pre-conditions:**
```
✓ Story in QA In Progress state
✓ QA validation complete
✓ QA result: FAIL
```

**Validation Logic:**
```
# Read QA report
Read(file_path="devforgeai/qa/reports/{story_id}-qa-report.md")

# Parse QA status
qa_status = parse_qa_status(report)
IF qa_status != "FAIL":
    # Should not happen, but handle gracefully
    WARN: "QA status unclear, treating as FAIL for safety"

# Parse violation counts
violations = parse_violations(report)

# Always transitions to QA Failed (no validation needed)
PASS: Transition to QA Failed
```

**Actions on Success (of transition):**
```
1. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="status: QA In Progress",
        new_string="status: QA Failed")

2. Append QA results section:
   """
   ## QA Results

   **Date:** {timestamp}
   **Status:** ❌ FAIL
   **Validator:** devforgeai-qa skill

   ### Summary
   - Overall Coverage: {coverage_percentage}%
   - Critical Issues: {critical_count}
   - High Issues: {high_count}
   - Medium Issues: {medium_count}
   - Low Issues: {low_count}

   ### Blocking Issues
   {list_critical_and_high_violations}

   ### Report
   - Full Report: `devforgeai/qa/reports/{story_id}-qa-report.md`

   ### Action Items
   {action_items_from_report}

   ### Next Steps
   - Fix blocking violations
   - Re-run development workflow
   - QA will re-validate after fixes
   """

3. Append workflow history:
   """
   ### {timestamp} - QA Failed
   - Previous Status: QA In Progress
   - Action: Deep QA validation FAILED
   - Critical: {critical_count}, High: {high_count}
   - Coverage: {coverage}% (thresholds: 95%/85%/80%)
   - Next: Return to development to fix issues
   """

4. Report:
   """
   ❌ QA Validation FAILED

   Violations:
   - CRITICAL: {critical_count}
   - HIGH: {high_count}
   - MEDIUM: {medium_count}
   - LOW: {low_count}

   Coverage: {coverage}%

   Action Required:
   1. Review QA report: devforgeai/qa/reports/{story_id}-qa-report.md
   2. Fix CRITICAL and HIGH violations
   3. Add tests to meet coverage thresholds
   4. Re-run development workflow

   See QA report for detailed fix guidance.
   """
```

---

### Transition 7: QA Failed → In Development

**Trigger:**
- Developer ready to fix violations
- Manual invocation of development workflow

**Pre-conditions:**
```
✓ Story in QA Failed state
✓ QA report exists with violations
✓ Action items documented
```

**Validation Logic:**
```
# Verify QA report exists
qa_report_exists = file_exists(f"devforgeai/qa/reports/{story_id}-qa-report.md")
IF NOT qa_report_exists:
    FAIL: "QA report missing - cannot determine what to fix"

# Parse action items
Read(file_path=f"devforgeai/qa/reports/{story_id}-qa-report.md")
action_items = parse_action_items(report)

IF action_items.empty:
    WARN: "No action items in QA report - unusual"

# Load action items into development context
development_context = {
    "previous_status": "QA Failed",
    "qa_report": report,
    "action_items": action_items,
    "focus": "Fix QA violations"
}

# All validations passed
PASS: Ready to return to development
```

**Actions on Success:**
```
1. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="status: QA Failed",
        new_string="status: In Development")

2. Append workflow history:
   """
   ### {timestamp} - In Development (Re-work)
   - Previous Status: QA Failed
   - Action: Developer fixing QA violations
   - Focus: {top_priority_action_items}
   - Next: Fix violations, re-run QA validation
   """

3. Invoke development skill with context:
   Skill(command="devforgeai-development --story={story_id} --qa-rework")

4. Report:
   """
   🔧 Returning to development to fix QA violations

   Priority Actions:
   {format_action_items(action_items, priority_only=True)}

   After fixes:
   - Development workflow will run
   - QA validation will automatically re-run
   - If PASS, story will move to QA Approved
   """
```

**Can Transition Fail:** No
- Always allowed to return to development from QA Failed

---

### Transition 8: QA Approved → Releasing

**Trigger:**
- Release manually initiated
- Coordinated sprint release

**Pre-conditions:**
```
✓ Story in QA Approved state
✓ QA report exists with PASS status
✓ All workflow checkboxes complete
```

**Validation Logic:**
```
# Verify QA report still shows PASS
Read(file_path=f"devforgeai/qa/reports/{story_id}-qa-report.md")
qa_status = parse_qa_status(report)

IF qa_status != "PASS":
    FAIL: "QA status changed since approval"
    ACTION: Re-run QA validation

# Verify all workflow checkboxes complete
Read(file_path="ai_docs/Stories/{story_id}.story.md")
workflow_complete = check_workflow_checkboxes(content)

IF NOT workflow_complete.architecture:
    FAIL: "Architecture phase not marked complete"
IF NOT workflow_complete.development:
    FAIL: "Development phase not marked complete"
IF NOT workflow_complete.qa:
    FAIL: "QA phase not marked complete"

# Check for blocking dependencies
dependencies = extract_section("Dependencies")
IF dependencies.has_blocking_dependencies:
    blocking_deps = check_dependency_status(dependencies)
    IF blocking_deps:
        FAIL: "Blocking dependencies not resolved: {blocking_deps}"

# Check for unresolved blockers
IF any_active_blockers():
    FAIL: "Story has active blockers"

# All validations passed
PASS: Ready for release
```

**Actions on Success:**
```
1. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="status: QA Approved",
        new_string="status: Releasing")

2. Append workflow history:
   """
   ### {timestamp} - Releasing
   - Previous Status: QA Approved
   - Action: Release initiated
   - Target Environment: Production
   - Next: Deployment in progress
   """

3. Invoke release skill:
   Skill(command="devforgeai-release --story={story_id}")

4. Report:
   "🚀 Starting release to production..."
```

**Can Transition Fail:** Yes
- Missing dependencies → BLOCK
- Workflow incomplete → BLOCK
- Active blockers → BLOCK

---

### Transition 9: Releasing → Released

**Trigger:**
- devforgeai-release skill completes successfully
- Deployment verified

**Pre-conditions:**
```
✓ Story in Releasing state
✓ Deployment successful
✓ Health checks passed
```

**Validation Logic:**
```
# Verify deployment successful
deployment_status = check_deployment_status()

IF deployment_status != "success":
    FAIL: "Deployment failed or health checks failed"
    Detail: {deployment_errors}
    ACTION: Rollback or fix deployment

# Verify health checks
health_status = check_production_health()

IF NOT health_status.healthy:
    FAIL: "Production health checks failing"
    Detail: {health_issues}
    ACTION: Rollback deployment

# All validations passed
PASS: Release successful
```

**Actions on Success:**
```
1. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="status: Releasing",
        new_string="status: Released")

2. Check workflow box:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="- [ ] Released",
        new_string="- [x] Released")

3. Add completion timestamp to frontmatter:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="created: {created_date}",
        new_string=f"created: {created_date}\ncompleted: {timestamp}")

4. Append release info:
   """
   ## Release Information

   **Released:** {timestamp}
   **Version:** {version_number}
   **Environment:** Production
   **Deployment Method:** {deployment_method}
   **Health Status:** ✅ Healthy

   ### Release Notes
   {release_notes}
   """

5. Append workflow history:
   """
   ### {timestamp} - Released
   - Previous Status: Releasing
   - Action: Successfully deployed to production
   - Version: {version_number}
   - Health: All checks passing
   - Story COMPLETE
   """

6. Update sprint progress:
   update_sprint_progress(story.sprint, story.points)

7. Update epic progress:
   update_epic_progress(story.epic, story.points)

8. Report:
   """
   🎉 Release Successful!

   Story: {story_id} - {story_title}
   Version: {version_number}
   Environment: Production
   Timestamp: {timestamp}

   Health: ✅ All checks passing

   Story lifecycle complete.
   """
```

**Can Transition Fail:** Yes
- Deployment failure → BLOCK, rollback
- Health check failure → BLOCK, rollback
- Must investigate and retry

---

### Transition 10: Any State → Blocked

**Trigger:**
- External dependency identified
- Blocker prevents further progress

**Pre-conditions:**
```
✓ Story in any state
✓ Blocker identified and documented
```

**Validation Logic:**
```
# Verify blocker is documented
blocker_reason = get_blocker_reason()

IF blocker_reason.empty:
    FAIL: "Must document blocker reason before blocking story"
    AskUserQuestion: "What is blocking this story?"

# Capture previous state for return
previous_state = current_status

# All validations passed
PASS: Can transition to Blocked
```

**Actions on Success:**
```
1. Capture previous state:
   previous_state = current_story_status

2. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string=f"status: {previous_state}",
        new_string="status: Blocked")

3. Add blocker section:
   """
   ## Blocker Information

   **Blocked Date:** {timestamp}
   **Previous State:** {previous_state}
   **Blocker Reason:** {blocker_reason}
   **Blocker Owner:** {owner}
   **Expected Resolution:** {eta}

   ### Resolution Plan
   {resolution_plan}
   """

4. Append workflow history:
   """
   ### {timestamp} - Blocked
   - Previous Status: {previous_state}
   - Action: Story blocked
   - Reason: {blocker_reason}
   - Owner: {blocker_owner}
   - Next: Wait for blocker resolution
   """

5. Notify stakeholders:
   notify_blocker(story_id, blocker_reason, previous_state)

6. Report:
   """
   🚧 Story Blocked

   Reason: {blocker_reason}
   Previous State: {previous_state}
   Owner: {blocker_owner}
   ETA: {expected_resolution}

   Resolution plan:
   {resolution_steps}
   """
```

**Can Transition Fail:** No
- Always allowed to block a story
- Must document reason

---

### Transition 11: Blocked → Previous State

**Trigger:**
- Blocker resolved
- Manual unblock

**Pre-conditions:**
```
✓ Story in Blocked state
✓ Blocker resolved or workaround created
```

**Validation Logic:**
```
# Read blocker information
Read(file_path="ai_docs/Stories/{story_id}.story.md")
blocker_info = extract_section("Blocker Information")

# Get previous state
previous_state = blocker_info.previous_state

# Verify blocker resolved
blocker_resolved = verify_blocker_resolution(blocker_info)

IF NOT blocker_resolved:
    WARN: "Blocker may not be fully resolved"
    AskUserQuestion: "Proceed with unblock even though blocker not fully resolved?"

# All validations passed
PASS: Can return to previous state
```

**Actions on Success:**
```
1. Update story status:
   Edit(file_path="ai_docs/Stories/{story_id}.story.md",
        old_string="status: Blocked",
        new_string=f"status: {previous_state}")

2. Update blocker section:
   """
   ## Blocker Information (RESOLVED)

   **Blocked Date:** {block_timestamp}
   **Unblocked Date:** {unblock_timestamp}
   **Previous State:** {previous_state}
   **Blocker Reason:** {blocker_reason}
   **Resolution:** {resolution_description}
   **Duration:** {blocked_duration}
   """

3. Append workflow history:
   """
   ### {timestamp} - {previous_state} (Unblocked)
   - Previous Status: Blocked
   - Action: Blocker resolved, returning to {previous_state}
   - Resolution: {resolution_summary}
   - Next: Resume workflow from {previous_state}
   """

4. Report:
   """
   ✅ Blocker Resolved

   Blocker: {blocker_reason}
   Resolution: {resolution_summary}
   Duration Blocked: {blocked_duration}

   Returning to: {previous_state}

   Resuming workflow...
   """

5. Resume workflow:
   # Re-run orchestration from previous state
   Skill(command="devforgeai-orchestration --story={story_id}")
```

**Can Transition Fail:** No
- Always allowed to unblock
- May warn if blocker not fully resolved

---

## Transition Validation Summary

### Mandatory Validations (Cannot Skip)

1. **All Transitions:**
   - Story document exists
   - Current status matches expected FROM state
   - Story not in invalid state

2. **Backlog → Architecture:**
   - Has acceptance criteria (at least 1)

3. **Architecture → Ready for Dev:**
   - All 6 context files exist

4. **In Development → Dev Complete:**
   - Build succeeds
   - All tests pass

5. **QA In Progress → QA Approved:**
   - Coverage meets thresholds (95%/85%/80%)
   - Zero CRITICAL violations
   - Zero HIGH violations (or approved)

6. **QA Approved → Releasing:**
   - All workflow checkboxes complete
   - No blocking dependencies

7. **Releasing → Released:**
   - Deployment successful
   - Health checks pass

### Optional Validations (Warnings)

- Context files stale (>30 days old)
- Missing technical specification
- Placeholder content in context files
- Blocker not fully resolved on unblock

### User Decision Points

Transitions that may require `AskUserQuestion`:
- Backlog → Architecture: If missing tech spec or has blocking dependencies
- Architecture → Ready for Dev: If context has TODOs/placeholders
- Ready for Dev → In Development: If context files stale
- Dev Complete → QA In Progress: If changes not pushed to remote
- QA In Progress → QA Approved: If coverage exception requested
- Any → Blocked: To document blocker reason
- Blocked → Previous: To confirm blocker resolved

---

**Use this reference when:**
- Implementing state transition logic
- Debugging workflow issues
- Understanding validation requirements
- Adding new workflow features
- Training on orchestration system
