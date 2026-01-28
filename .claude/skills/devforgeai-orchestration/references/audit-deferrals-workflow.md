# Phase 8: Audit Deferrals Workflow

**Purpose:** Execute complete deferral audit workflow with feedback hook integration (STORY-050)

**Invoked by:** `/audit-deferrals` command via devforgeai-orchestration skill

**Token Cost:** ~95,000 tokens (isolated skill context)

**Execution Time:** 5-15 minutes (depends on story count)

---

## Overview

Phase 8 executes a comprehensive audit of all QA Approved and Released stories to identify deferral violations, circular chains, and resolvable deferrals. After audit completion, feedback hooks are conditionally invoked (if eligible) to capture user insights about technical debt patterns.

**Workflow:**
1. Discover QA Approved/Released stories (Phase 1)
2. Scan for deferred DoD items (Phase 2)
3. Validate blockers (Phase 2.5 - check if resolvable)
4. Validate each deferral (Phase 3 - deferral-validator subagent)
5. Aggregate results by severity (Phase 4)
6. Generate audit report (Phase 5)
7. Invoke feedback hooks (Phase 6 - conditional, non-blocking)

**7 Hook Integration Substeps (Phase 6):**
- 6.1 Check eligibility
- 6.2 Prepare context
- 6.3 Sanitize data
- 6.4 Invoke hooks
- 6.5 Log invocation
- 6.6 Handle errors
- 6.7 Prevent circular invocations

---

## Mode Detection

**Trigger:** When conversation contains marker `**Command:** audit-deferrals`

**Extraction:**
```
Grep conversation for "**Command:** audit-deferrals"
IF found:
  MODE = "audit-deferrals"
  Execute Phase 8 workflow
```

---

## Phase 1: Discover QA Approved Stories

**Find all story files eligible for audit:**

```
Glob(pattern="devforgeai/specs/Stories/*.story.md")

audit_list = []

FOR each story file:
    Read YAML frontmatter only (first 20 lines)
    Extract: id, status

    IF status == "QA Approved" OR status == "Released":
        Add to audit_list: {id, path, status}

Display: "Found {count} QA Approved/Released stories to audit"
```

**Success:** Returns list of stories to audit (0 or more)

---

## Phase 2: Scan for Deferrals

**Check each story for incomplete DoD items:**

```
deferred_stories = []

FOR each story in audit_list:
    Read full story content
    Search for "## Implementation Notes"

    IF "Implementation Notes" found:
        Search for "### Definition of Done Status"

        IF DoD Status found:
            Parse items (lines starting with "- [ ]" or "- [x]")

            FOR each item:
                IF item starts with "- [ ]" (incomplete):
                    Extract: item_description, deferral_reason
                    Record deferral: {
                        story_id: {id},
                        item: {item_description},
                        reason: {deferral_reason}
                    }

                    Add story to deferred_stories (if not already added)

Display: "Found {count} stories with deferrals"
```

**Success:** Returns stories with deferrals identified

---

## Phase 2.5: Blocker Validation (RCA-006 Phase 2)

**Purpose:** Identify deferrals that can be resolved NOW vs deferrals with valid blockers

**For each story with deferrals:**

```
FOR each story in deferred_stories:
    FOR each deferral in story.deferrals:

        # Extract deferral details
        IF "Deferred to STORY-" in deferral.reason:
            target_story = extract STORY-ID from reason
            blocker_type = "dependency"

        ELIF "Blocked by:" in deferral.reason:
            blocker_description = extract text after "Blocked by:"
            blocker_type = "external"

        ELIF "Out of scope: ADR-" in deferral.reason:
            adr_reference = extract ADR-XXX
            blocker_type = "scope_change"

        # Validate blocker is still valid
        SWITCH blocker_type:

        CASE "dependency":
            # Check if target story exists and is complete
            Glob(pattern="devforgeai/specs/Stories/{target_story}*.story.md")
            IF found:
                Read YAML frontmatter
                target_status = extract status

                IF target_status IN ["Released", "QA Approved"]:
                    # Blocker resolved!
                    categorize as: RESOLVABLE
                    reason: "Dependency story {target_story} now complete"
                ELSE:
                    categorize as: VALID
                    reason: "Dependency story {target_story} still in progress (status: {target_status})"
            ELSE:
                categorize as: INVALID
                reason: "Dependency story {target_story} not found (broken reference)"

        CASE "external":
            # Check if blocker description references toolchain, environment, artifact
            IF blocker_description mentions toolchain (rustup, npm, dotnet):
                Bash(command="which {tool}")
                IF tool found:
                    categorize as: RESOLVABLE
                    reason: "Toolchain {tool} now available"
                ELSE:
                    categorize as: VALID
                    reason: "Toolchain {tool} still not installed"

            ELIF blocker_description mentions artifact/file:
                extract: file_path from description
                Glob(pattern="{file_path}")
                IF found:
                    categorize as: RESOLVABLE
                    reason: "Artifact {file_path} now exists"
                ELSE:
                    categorize as: VALID
                    reason: "Artifact {file_path} still missing"

            ELSE:
                # Generic blocker - assume still valid
                categorize as: VALID
                reason: "External blocker: {blocker_description}"

        CASE "scope_change":
            # Check if ADR exists
            Glob(pattern="devforgeai/specs/adrs/{adr_reference}*.md")
            IF found:
                categorize as: VALID
                reason: "ADR {adr_reference} documents scope change"
            ELSE:
                categorize as: INVALID
                reason: "Missing ADR {adr_reference} (scope change not documented)"
```

**Output:** Deferrals categorized as RESOLVABLE, VALID, or INVALID

---

## Phase 3: Validate Each Deferral

**Invoke deferral-validator subagent for comprehensive validation:**

```
validation_results = []

FOR each story in deferred_stories:
    Task(
        subagent_type="deferral-validator",
        description="Validate deferrals in {story.id}",
        prompt="Validate all deferred DoD items in story {story.id}.

        Story content already in conversation.

        Check for:
        1. Multi-level deferral chains (STORY-A defers to STORY-B which defers to STORY-C)
        2. Circular deferrals (STORY-A → STORY-B → STORY-A)
        3. Referenced story existence and status
        4. ADR requirement for scope changes
        5. Deferral justification quality

        Return:
        {
          story_id: '{story.id}',
          deferrals_count: N,
          violations: [
            {severity: 'CRITICAL', type: 'circular_deferral', description: '...'},
            {severity: 'HIGH', type: 'unjustified', description: '...'}
          ],
          valid_deferrals: N,
          invalid_deferrals: N
        }"
    )

    Parse subagent JSON response
    Add to validation_results
```

**Success:** All stories validated, violations identified

---

## Phase 4: Aggregate Results

**Categorize findings by severity:**

```
CRITICAL_violations = []
HIGH_violations = []
MEDIUM_violations = []
LOW_violations = []

FOR each result in validation_results:
    FOR each violation in result.violations:
        SWITCH violation.severity:
            CASE "CRITICAL": Add to CRITICAL_violations
            CASE "HIGH": Add to HIGH_violations
            CASE "MEDIUM": Add to MEDIUM_violations
            CASE "LOW": Add to LOW_violations

# Count by category (from Phase 2.5)
resolvable_count = count deferrals categorized as RESOLVABLE
valid_count = count deferrals categorized as VALID
invalid_count = count deferrals categorized as INVALID
```

**Calculate technical debt metrics:**

```
total_deferrals = resolvable_count + valid_count + invalid_count
total_age_days = sum of all deferral ages
average_age = total_age_days / total_deferrals
oldest_age = max deferral age
```

---

## Phase 5: Generate Audit Report

**Create comprehensive audit report:**

```
timestamp = current UTC timestamp (YYYY-MM-DDTHH-MM-SS)
report_path = "devforgeai/qa/deferral-audit-{timestamp}.md"

Write(
    file_path=report_path,
    content="""
# Deferral Audit Report

**Date:** {timestamp}
**Stories Audited:** {audit_list.count}
**Stories with Deferrals:** {deferred_stories.count}

---

## Executive Summary

### Severity Breakdown
- 🔴 CRITICAL: {CRITICAL_violations.count} (requires immediate action)
- 🟠 HIGH: {HIGH_violations.count} (address in current sprint)
- 🟡 MEDIUM: {MEDIUM_violations.count} (address in next sprint)
- 🟢 LOW: {LOW_violations.count} (monitor)

### Categorization
- 🟡 Resolvable: {resolvable_count} (blockers resolved, can retry now)
- 🟢 Valid: {valid_count} (blockers still present, properly deferred)
- 🔴 Invalid: {invalid_count} (missing references, circular chains)

### Technical Debt Metrics
- Total Deferrals: {total_deferrals}
- Total Age: {total_age_days} days
- Average Age: {average_age} days
- Oldest Deferral: {oldest_age} days ({oldest_story_id})

---

## Critical Issues ({CRITICAL_violations.count})

{FOR each violation in CRITICAL_violations:}
### {violation.story_id}: {violation.type}

**Description:** {violation.description}

**Severity:** CRITICAL

**Action Required:**
{violation.remediation}

**Priority:** Immediate

---

## High Priority Issues ({HIGH_violations.count})

{FOR each violation in HIGH_violations:}
### {violation.story_id}: {violation.type}

**Description:** {violation.description}

**Action:** {violation.remediation}

---

## Resolvable Deferrals ({resolvable_count})

{List each resolvable deferral with:}
- Story ID
- Deferred item description
- Original blocker
- Why resolvable now
- Command to resolve: /dev {story_id}

---

## Valid Deferrals ({valid_count})

{List each valid deferral with:}
- Story ID
- Deferred item
- Current blocker status
- Estimated resolution timeline

---

## Invalid Deferrals ({invalid_count})

{List each invalid deferral with:}
- Story ID
- Issue (broken reference, missing ADR, etc.)
- Required action

---

## Recommendations

{IF resolvable_count >= 3:}
1. Create debt reduction sprint
   - Include {resolvable_count} resolvable stories
   - Estimated effort: {estimate} points
   - Target completion: Next sprint

{IF CRITICAL_violations.count > 0:}
2. Resolve circular dependencies immediately
   - Review deferral chains
   - Break cycles by redesigning dependencies

{IF stale_deferrals > 5:}
3. Review aging deferrals (>30 days)
   - {stale_deferrals} deferrals older than 30 days
   - Oldest: {oldest_age} days
   - Consider: Re-prioritize or remove from backlog

---

## Audit Scope

**Stories Audited:**
{FOR each story in audit_list:}
- {story.id} (status: {story.status})

**Stories with Deferrals:**
{FOR each story in deferred_stories:}
- {story.id} ({story.deferrals.count} deferrals)

---

**Audit completed:** {timestamp}
**Report location:** devforgeai/qa/deferral-audit-{timestamp}.md
"""
)

Display: "✓ Audit report generated: {report_path}"
```

**Success:** Report file created and saved

---

## Phase 6: Invoke Feedback Hooks (STORY-033)

**After audit report generation, invoke feedback hooks to capture insights about audit findings.**

**Purpose:** Capture user insights about deferral patterns, debt reduction strategies, and process improvements while analysis is fresh.

**Pattern:** Follows STORY-023 (/dev pilot) pattern - eligibility check → conditional invocation → graceful degradation (non-blocking)

### Step 6.1: Check Hook Eligibility

```bash
# Determine if feedback hooks should be triggered
# Use optimized bash version for <100ms latency requirement (13ms P95 vs 164ms Python CLI)
bash .claude/scripts/check-hooks-fast.sh audit-deferrals success

IF exit_code == 0:
  # Hooks are eligible, proceed to Step 6.2
  ELIGIBLE = true
ELSE:
  # Hooks not eligible (disabled, config issue, etc.)
  ELIGIBLE = false
  Display: "ℹ️ Feedback hooks not triggered (disabled or not configured)"
  Skip to Phase 7 (workflow complete)
```

### Step 6.2: Prepare Audit Context (Conditional - Executable Bash)

**IF ELIGIBLE is true, extract audit_summary from generated audit report:**

```bash
# Get the most recent audit report (Phase 5 just created it)
audit_report=$(ls -t devforgeai/qa/deferral-audit-*.md | head -1)

if [ ! -f "$audit_report" ]; then
  echo "⚠️ Audit report not found, skipping feedback" >&2
  exit 0
fi

# Extract counts from audit report using grep
resolvable_count=$(grep -c "🟡 Resolvable:" "$audit_report" || echo "0")
valid_count=$(grep -c "🟢 Valid:" "$audit_report" || echo "0")
invalid_count=$(grep -c "🔴 Invalid:" "$audit_report" || echo "0")

# Extract oldest deferral age (search for "**Oldest Deferral:** N days")
oldest_age=$(grep -oP '(?<=\*\*Oldest Deferral:\*\* )\d+' "$audit_report" | head -1 || echo "null")

# Extract circular chains (search for STORY-IDs in "Circular Deferrals" section)
circular_chains=$(awk '/## Critical Issues/,/^---/ {print}' "$audit_report" | \
  grep -oP 'STORY-\d+' | sort -u | paste -sd',' - || echo "")

# Build JSON using jq (proper escaping)
audit_summary=$(jq -cn \
  --argjson resolvable "$resolvable_count" \
  --argjson valid "$valid_count" \
  --argjson invalid "$invalid_count" \
  --arg oldest "${oldest_age:-null}" \
  --arg chains "$circular_chains" \
  '{
    resolvable_count: $resolvable,
    valid_count: $valid,
    invalid_count: $invalid,
    oldest_age: (if $oldest == "null" then null else ($oldest | tonumber) end),
    circular_chains: (if $chains == "" then [] else ($chains | split(",")) end)
  }')

# Enforce 50KB context size limit
context_size=$(echo "$audit_summary" | wc -c)
if [ "$context_size" -gt 51200 ]; then
  # Truncate to top 20 deferrals by priority:
  #   1. Circular dependencies (CRITICAL)
  #   2. Oldest resolvable items (HIGH)
  #   3. Remaining by age (MEDIUM)
  # Extract just the critical chains and top resolvable items
  audit_summary=$(jq -c \
    --argjson resolvable "$resolvable_count" \
    --argjson valid "$valid_count" \
    --argjson invalid "$invalid_count" \
    --arg oldest "${oldest_age:-null}" \
    --arg chains_limited "$(echo "$circular_chains" | cut -d',' -f1-5)" \
    '{
      resolvable_count: $resolvable,
      valid_count: $valid,
      invalid_count: $invalid,
      oldest_age: (if $oldest == "null" then null else ($oldest | tonumber) end),
      circular_chains: (if $chains_limited == "" then [] else ($chains_limited | split(",")) end),
      note: "Truncated to top 20 by priority (full report available on disk)"
    }')
fi
# Full report remains at $audit_report for reference
```

### Step 6.3: Sanitize Sensitive Data (Executable Bash)

**Remove credentials from audit_summary JSON before passing to hooks:**

```bash
# Sanitize sensitive patterns in the JSON payload
# Supports: api_key, API_KEY, secret, password, token, bearer, auth_code, etc.
audit_summary=$(echo "$audit_summary" | \
  sed -E 's/(api_?key|secret|password|token|bearer|auth_?code|refresh_?token|access_?token)["\s:=]+[^ ",}]+/\1=[REDACTED]/gi')

# Verify sanitization worked (should contain [REDACTED] if credentials were present)
# This is defensive - ensures no secrets leak to feedback system
```

### Step 6.4: Invoke Feedback Hooks (Executable Bash)

**IF ELIGIBLE is true, invoke hooks with sanitized audit context:**

```bash
# Extract affected story IDs from audit report
affected_stories=$(grep -oP 'STORY-\d+' "$audit_report" | sort -u | paste -sd',' -)

# Build context summary string
context_msg="Audit complete. Found: ${resolvable_count} resolvable, ${valid_count} valid, ${invalid_count} invalid deferrals."
if [ "$oldest_age" != "null" ]; then
  context_msg="$context_msg Oldest: ${oldest_age} days."
fi

# Invoke feedback hooks with structured metadata
devforgeai-validate invoke-hooks \
  --operation=audit-deferrals \
  --operation_metadata="$audit_summary" \
  --story_ids="$affected_stories" \
  --context="$context_msg" || {
    # Hook invocation failed - log but don't block audit
    echo "⚠️ Hook invocation failed (see logs), audit continues..." >&2
    exit_code=1
  }

# Expected: Feedback conversation launches with audit-specific context
# Questions adapt to reference audit findings (e.g., "You found X resolvable deferrals...")
# Hook failures are non-blocking (handled in Step 6.6)
```

### Step 6.5: Log Hook Invocation

**Log all hook operations to `devforgeai/feedback/logs/hook-invocations.log`:**

```bash
# Create log directory if doesn't exist
mkdir -p devforgeai/feedback/logs

# Append structured log entry with file locking
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
LOG_ENTRY="$TIMESTAMP | operation=audit-deferrals | status=completed | exit_code=${exit_code:-0} | session_id=$(uuidgen) | resolvable=$resolvable_count | valid=$valid_count | invalid=$invalid_count"

# Use file locking to prevent write conflicts in concurrent audits
(flock -x 200
echo "$LOG_ENTRY" >> devforgeai/feedback/logs/hook-invocations.log
) 200>devforgeai/feedback/logs/hook-invocations.log.lock

Display: "✓ Hook invocation logged"
```

### Step 6.6: Error Handling & Graceful Degradation (Executable Bash)

**Hook invocation is non-blocking. All errors logged, command succeeds (exit 0):**

```bash
# Capture hook exit code (from Step 6.4)
hook_exit_code=${exit_code:-0}

if [ $hook_exit_code -ne 0 ]; then
  # Hook failed, log and continue
  echo "⚠️ Feedback hook failed with exit code $hook_exit_code" >&2
  echo "   Audit report still generated successfully" >&2
  echo "   Check devforgeai/feedback/logs/hook-invocations.log for details" >&2

  # Log failure details
  echo "$TIMESTAMP | operation=audit-deferrals | hook_status=failed | exit_code=$hook_exit_code | reason=hook_invocation_error" \
    >> devforgeai/feedback/logs/hook-invocations.log
else
  echo "✓ Feedback hook triggered successfully" >&2
fi

# Command continues regardless of hook status (non-blocking)
exit 0
```

**Error scenarios gracefully handled:**
- Hook configuration missing → Skip hooks, audit succeeds
- Hook script errors → Log error, audit succeeds
- Hook timeout → Log timeout, audit succeeds
- Circular hook invocation → Detected in Step 6.7, prevents loop

### Step 6.7: Circular Invocation Prevention

**Prevent infinite loops from hooks invoking commands that trigger hooks:**

```bash
# Check invocation depth (environment variable set by invoke-hooks script)
INVOCATION_DEPTH=${DEVFORGEAI_HOOK_DEPTH:-0}

if [ $INVOCATION_DEPTH -ge 3 ]; then
  echo "⚠️ Maximum hook depth reached ($INVOCATION_DEPTH), preventing circular invocation" >&2
  echo "$TIMESTAMP | operation=audit-deferrals | status=skipped | reason=circular_prevention | depth=$INVOCATION_DEPTH" \
    >> devforgeai/feedback/logs/hook-invocations.log
  exit 0  # Skip hook, audit succeeds
fi

# Increment depth for child processes (prevents infinite recursion)
export DEVFORGEAI_HOOK_DEPTH=$((INVOCATION_DEPTH + 1))
```

**Circular prevention verified:** Depth tracking prevents command → hook → command → hook loops

---

## Phase 7: Return Audit Summary

**Generate user-facing summary for command display:**

```
audit_summary = {
  "stories_audited": audit_list.count,
  "stories_with_deferrals": deferred_stories.count,
  "violations": {
    "CRITICAL": CRITICAL_violations.count,
    "HIGH": HIGH_violations.count,
    "MEDIUM": MEDIUM_violations.count,
    "LOW": LOW_violations.count
  },
  "categorization": {
    "resolvable": resolvable_count,
    "valid": valid_count,
    "invalid": invalid_count
  },
  "technical_debt": {
    "total_age": total_age_days,
    "average_age": average_age,
    "oldest_age": oldest_age
  },
  "recommendations": [
    {recommendations from Phase 4 analysis}
  ],
  "report_path": report_path,
  "hook_triggered": ELIGIBLE
}

Return audit_summary to /audit-deferrals command
```

**Command displays:** Formatted summary, recommendations, report link

---

## Success Criteria

Phase 8 succeeds when:

- [ ] All QA Approved/Released stories scanned
- [ ] Deferrals categorized (resolvable/valid/invalid)
- [ ] Violations identified by severity
- [ ] Audit report generated and saved
- [ ] Hooks invoked if eligible (non-blocking)
- [ ] Audit summary returned to command
- [ ] Token usage <100K (isolated skill context)
- [ ] Execution time 5-15 minutes (depends on story count)

---

## Subagents Invoked

**deferral-validator:**
- Invoked once per story with deferrals
- Validates deferral justifications
- Detects circular chains
- Checks story/ADR references
- Returns violations by severity

**Note:** No other subagents needed (Phase 2.5 blocker validation is inline logic, not subagent)

---

## Error Handling

### No Stories Found

```
IF audit_list is empty:
  Display: "No QA Approved or Released stories found to audit"
  Display: "Run /dev and /qa first to create auditable stories"
  Return empty audit summary
  Exit gracefully (no error)
```

### Deferral Validator Fails

```
IF deferral-validator subagent fails for a story:
  Log error for that story
  Continue with remaining stories
  Include partial results in audit report
  Flag incomplete audit in summary
```

### Audit Report Write Failure

```
IF Write(file_path=report_path) fails:
  Display: "❌ ERROR: Could not write audit report"
  Check: devforgeai/qa/ directory permissions
  Fallback: Display audit summary in console only
  Hook invocation skipped (no report to reference)
```

### Hook Invocation Errors

```
Hook errors are NON-BLOCKING:
- Hook config missing → Skip hooks, audit succeeds
- Hook script error → Log error, audit succeeds
- Hook timeout → Log timeout, audit succeeds

All hook errors logged to:
  devforgeai/feedback/logs/hook-invocations.log
```

---

## Integration with Other Workflows

### Sprint Retrospective (Phase 7)

**Automatic audit trigger:**
- Sprint retrospective auto-invokes /audit-deferrals
- Uses audit results to create debt reduction sprint
- Coordination: Phase 7 → Phase 8 → Debt analysis

### Manual Audit

**Direct invocation:**
- User runs /audit-deferrals command
- Command sets `**Command:** audit-deferrals` marker
- Skill executes Phase 8 workflow

---

## Performance Optimization

**Token efficiency:**
- Grep for pattern matching (vs reading full files)
- Read YAML frontmatter only (first 20 lines) for status checks
- Progressive file loading (read full content only for stories with deferrals)
- Subagent context isolation (deferral-validator work not in main conversation)

**Execution speed:**
- Hook eligibility check: <100ms (bash script, not Python CLI)
- Context preparation: <500ms (grep + jq operations)
- Hook invocation: <5 seconds (subprocess call)
- Total hook overhead: <5.6 seconds (meets <100ms P95 requirement for eligibility check only)

---

## Testing

**Verify Phase 8 implementation:**

1. Run /audit-deferrals command
2. Verify all 7 phases execute
3. Check audit report generated
4. Verify hooks triggered if eligible
5. Confirm graceful degradation on hook failure
6. Validate backward compatibility (audit behavior unchanged)

**Test cases:**
- No stories found → Empty audit, graceful exit
- Some stories with deferrals → Audit categorizes correctly
- Circular deferrals detected → CRITICAL violations raised
- Hooks disabled → Skip hooks, audit succeeds
- Hook failure → Log error, audit succeeds
- Performance within 10% baseline → No regression

---

## Notes

**Framework Integration:**
- Follows lean orchestration pattern (command delegates to skill)
- Skill coordinates deferral-validator subagent for each story
- Hook integration (STORY-033) is non-blocking
- Hook failures don't prevent audit completion

**Audit Scope:**
- 100% coverage of QA Approved stories
- 100% coverage of Released stories
- All deferrals validated with consistent criteria
- Automated validation via subagent (more thorough than manual)

**RCA References:**
- RCA-006 Phase 2: Blocker validation (resolvable vs valid vs invalid)
- RCA-007: Multi-level deferral chain detection
- STORY-033: Feedback hook integration for insights capture

**See also:**
- `devforgeai/RCA/RCA-006-autonomous-deferrals.md` (deferral validation policy)
- `devforgeai/RCA/RCA-007-multi-file-story-creation.md` (multi-level chains)
- `.claude/agents/deferral-validator.md` (validation subagent)
- `.claude/commands/audit-deferrals.md` (refactored command - 5.8K chars)

---

**Character count:** ~10,200 characters (reference file - loaded on-demand only)
**Phase 8 workflow:** Complete audit + hook integration (7 substeps)
**Status:** Production Ready (STORY-050 implementation)
