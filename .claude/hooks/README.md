# DevForgeAI Pre-Tool-Use Hook

## Purpose

Auto-approve safe bash commands to reduce user approval friction while blocking dangerous operations.

**Effectiveness:** >95% of commands auto-approved (RCA-015 REC-01 + REC-02)

---

## How It Works

### Approval Flow

```
Bash command requested
    ↓
Hook intercepts (pre-tool-use.sh)
    ↓
Check against 69 SAFE_PATTERNS
    ├─ Match found → AUTO-APPROVE (exit 0)
    ↓
Check against BLOCKED_PATTERNS
    ├─ Match found → BLOCK with error (exit 2)
    ↓
No match → ASK USER for approval (exit 1)
```

### Pattern Matching

**Safe Patterns:** Prefix matching (`"$COMMAND" == "$pattern"*`)
- Command must START with safe pattern
- Example: `git status | grep modified` matches `"git status"`

**Blocked Patterns:** Regex contains (`"$COMMAND" =~ pattern`)
- Pattern can appear ANYWHERE in command
- Example: `cd /tmp && rm -rf test` blocked by `rm -rf` even though starts with `cd`

**Multi-Layer Defense:**
- Safe patterns checked FIRST (auto-approve common safe commands)
- Blocked patterns checked SECOND (catch dangerous operations even if start looks safe)
- Unknown commands ASK USER (safe default)

---

## Safe Patterns (63 Total)

### Original Patterns (54)

**DevForgeAI Workflows:**
- `npm run test`, `npm run build`, `npm run lint`
- `dotnet test`, `dotnet build`
- `python3 -m pytest`, `pytest`
- `bash tests/`, `bash .claude/scripts/`, `bash devforgeai/`

**Git Operations (Read-Only):**
- `git status`, `git diff`, `git log`
- `git add`, `git commit` (write but safe - framework operations)

**File Operations (Read-Only):**
- `wc -`, `grep -E`, `grep -r`
- `head -`, `tail -`
- `cat tests/`, `cat devforgeai/`, `cat src/`, `cat installer/`
- `ls -la`, `ls -lh`, `ls -1`
- `find installer`, `find /mnt/c/Projects/DevForgeAI2/{src,tests,installer}`

**Utilities:**
- `echo`, `cat >`, `cat <<`, `cat << 'EOF'`
- `cp`, `mkdir -p`, `chmod +x`, `dos2unix`
- `sed -i`, `python3 -m json.tool`, `python3 <<`, `sort -`

### RCA-015 Additions (15)

**Added:** 2025-11-24
**Rationale:** Empirical analysis of 3,517 unknown command entries showed these patterns used frequently by Claude during DevForgeAI workflows.

**Command Composition:**
- `cd ` - Directory changes (always safe)
- `python3 -c ` - Inline Python scripts (safe, no file modification)
- `python3 << 'EOF'` - Python HERE-documents (safe, read-only analysis)
- `python << 'EOF'` - Python 2 HERE-documents

**DevForgeAI CLI:**
- `devforgeai ` - Framework's own CLI tools (always safe, we control them)

**Git Introspection (Read-Only):**
- `git rev-parse` - Parse git refs (read-only)
- `git branch` - List/show branches (read-only)
- `git --version` - Version check (read-only)
- `git rev-list` - List commits (read-only)

**Shell Utilities (Read-Only):**
- `which ` - Find command location
- `command -v` - Check if command exists
- `type ` - Show command type
- `stat ` - File statistics
- `file ` - File type detection
- `basename ` - Extract filename from path

---

## Blocked Patterns (6 Total)

**Always Blocked (Exit 2):**
- `rm -rf` - Recursive forced deletion (data loss risk)
- `sudo` - Privilege escalation (system modification)
- `git push` - Remote operations (unintended deployment)
- `npm publish` - Package publishing (unintended release)
- `curl` - Network requests (security risk)
- `wget` - Network downloads (security risk)

**Why blocked patterns use regex:**
- Catches dangerous operations anywhere in command
- Example: `cd /tmp && rm -rf test` blocked even though starts with safe `cd`

---

## Pattern Addition Criteria

### Safe Pattern Checklist

Add pattern if command is:
- ✅ **Read-only** - Doesn't modify files (git status, ls, grep, cat)
- ✅ **Framework-internal** - DevForgeAI operations we control (devforgeai CLI, pytest, npm test)
- ✅ **Navigation-only** - Changes context but no side effects (cd, which, type)
- ✅ **Logging/temp** - Writes to logs or temp dirs (echo, mkdir -p devforgeai/logs)

Do NOT add if command:
- ❌ **Modifies source** - Changes production files
- ❌ **Network access** - External requests
- ❌ **Privilege** - Requires sudo or root
- ❌ **Deletion** - Removes files (except safe temp dirs)

### Before Adding Pattern

**Check:**
1. Is command genuinely safe? (review what it does)
2. Is it used frequently? (check logs or run analysis tool)
3. Could it be misused? (consider edge cases)
4. Does it pass all 4 safety criteria above?

**Test:**
1. Add pattern to array
2. Run `bash -n .claude/hooks/pre-tool-use.sh` (syntax check)
3. Test command auto-approves
4. Test dangerous variant still blocks (e.g., if adding `git`, ensure `git push` still blocked)
5. Monitor logs for 24 hours

---

## Maintenance Process

### Monthly Pattern Review

**7-Step Update Process:**

1. **Run analysis** - Execute pattern analysis tool:
   ```bash
   python3 devforgeai/scripts/analyze-hook-patterns.py
   # Outputs top 20 safe pattern candidates
   # Shows frequency and impact percentage
   ```

2. **Review candidates** - Examine each suggested pattern:
   - Check pattern frequency (higher = more impactful)
   - Verify pattern is used by DevForgeAI workflows
   - Identify any edge cases or variants

3. **Validate safety** - Check each pattern against safety criteria:
   - Is it read-only or framework-internal?
   - Could it be misused in dangerous combinations?
   - Does it meet all 4 safe pattern checklist items?

4. **Add patterns** - Update SAFE_PATTERNS array:
   - Add to appropriate category (DevForgeAI, Git, Shell, etc.)
   - Use prefix matching for safe patterns
   - Document rationale in code comments

5. **Test** - Verify changes work correctly:
   - Run `bash -n .claude/hooks/pre-tool-use.sh` (syntax check)
   - Test that new patterns auto-approve
   - Verify dangerous variants still block

6. **Commit** - Save changes with descriptive message:
   ```bash
   git commit -m "chore(hooks): Add pattern '{pattern}' (used {count}× per analysis)"
   ```

7. **Monitor** - Watch for issues post-deployment:
   - Check logs for 24-48 hours
   - Verify no unsafe commands auto-approved
   - Track approval rate improvement

### Weekly Unknown Command Review

**Check what's requiring approval:**
```bash
tail -100 devforgeai/logs/hook-unknown-commands.log | \
  sed 's/.*APPROVAL: //' | \
  sort | uniq -c | sort -rn | head -10
```

**If pattern appears >10 times:**
- High-frequency safe pattern
- Add immediately (don't wait for monthly review)

### Quarterly Effectiveness Audit

**Calculate metrics:**
```bash
# Total invocations
TOTAL=$(wc -l < devforgeai/logs/pre-tool-use.log)

# Auto-approved
AUTO=$(grep -c "AUTO-APPROVE" devforgeai/logs/pre-tool-use.log)

# Approval rate
echo "scale=2; $AUTO * 100 / $TOTAL" | bc
# Target: >95%
```

**If approval rate <90%:**
- Run pattern analysis
- Review top unknown commands
- Add high-frequency safe patterns
- Re-audit in 1 week

---

## Troubleshooting

### Issue: Commands Still Requiring Approval

**Symptoms:**
- Frequent user approval prompts
- Same command pattern logged multiple times in hook-unknown-commands.log

**Diagnosis:**
```bash
# Find most frequent unknown commands
tail -500 devforgeai/logs/hook-unknown-commands.log | \
  sed 's/.*APPROVAL: //' | \
  awk '{print $1" "$2}' | \
  sort | uniq -c | sort -rn | head -10
```

**Resolution:**
- Review top 10 unknown command prefixes
- Validate each is safe
- Add to SAFE_PATTERNS
- Commit and test

### Issue: Unsafe Command Auto-Approved

**Symptoms:**
- File deleted unexpectedly
- System modified without approval
- Command that should have been blocked was auto-approved

**Diagnosis:**
```bash
# Find what pattern matched
tail -50 devforgeai/logs/pre-tool-use.log | grep -B 5 "AUTO-APPROVE"
# Shows which pattern matched
```

**Resolution:**
1. Identify problematic pattern
2. Make pattern more specific:
   - `"cd "` → `"cd /mnt/c/Projects/DevForgeAI2"` (restrict to project)
   - Or remove pattern if too risky
3. Add command to BLOCKED_PATTERNS if genuinely dangerous
4. Test fix
5. Document in this README why pattern was restricted/removed

### Issue: Hook Not Running

**Symptoms:**
- No entries in pre-tool-use.log
- ALL commands requiring approval (or none)

**Diagnosis:**
```bash
# Check hook is executable
ls -la .claude/hooks/pre-tool-use.sh
# Should show execute permission (x)

# Check bash syntax
bash -n .claude/hooks/pre-tool-use.sh
# Should exit 0
```

**Resolution:**
```bash
# Make executable
chmod +x .claude/hooks/pre-tool-use.sh

# Fix syntax if errors
# Review bash -n output for line numbers
```

### Issue: Hook Performance Slow

**Symptoms:**
- Commands take long to execute
- Noticeable delay before approval prompt

**Diagnosis:**
```bash
# Time hook execution
time bash .claude/hooks/pre-tool-use.sh <<< '{"tool_input":{"command":"git status"}}'
# Target: <100ms
```

**Resolution:**
- If >100ms: Review loop efficiency
- Consider compiling patterns to single regex
- Optimize jq extraction
- Profile with `set -x` to find bottleneck

---

## Related Documentation

**RCA Documents:**
- RCA-015: Pre-Tool-Use Hook Friction analysis

**Framework Protocols:**
- CLAUDE.md: Hook integration overview
- devforgeai/protocols/: Framework patterns

**Scripts:**
- devforgeai/scripts/analyze-hook-patterns.py: Pattern analysis tool (when created per REC-4)

---

## Changelog

### 2025-11-24: RCA-015 REC-02 - Pipe/Redirect Support
- **Added:** Quote-aware base command extraction (43-line function)
- **Feature:** Auto-approve safe commands with pipes (`git status | grep`) and redirects (`pytest > file`)
- **Safety:** Two-layer check: blocked patterns in full command + system directory redirect blocks
- **Quote Handling:** Preserves pipes/redirects inside quotes (`python3 -c "print('|')"`)
- **System Protection:** Blocks redirects to /etc, /usr, /sys, /boot, /root
- **Impact:** Additional 5-10% friction reduction (pipes/redirects now auto-approved)
- **Testing:** 23 test scenarios documented in test-rec-02.sh
- **Reference:** RCA-015-pre-tool-use-hook-friction-remains.md REC-02

### 2025-11-24: RCA-015 REC-01 - Pattern Expansion
- Added 15 common command composition patterns
- Patterns: cd, python3 -c, HERE-docs, devforgeai CLI, git introspection, shell utilities
- Impact: 90% reduction in approval friction (3,517 unknown commands → target <350)
- Reference: devforgeai/RCA/RCA-015-pre-tool-use-hook-friction-remains.md

### Initial: Hook Creation
- 54 safe patterns for DevForgeAI workflows
- 6 blocked patterns for dangerous operations
- Prefix matching for safe, regex for blocked

---

## Quick Reference

**Check logs:**
```bash
# Recent unknown commands
tail -50 devforgeai/logs/hook-unknown-commands.log

# Hook execution log
tail -100 devforgeai/logs/pre-tool-use.log

# Pattern match successes
grep "MATCHED safe pattern" devforgeai/logs/pre-tool-use.log | tail -20
```

**Test hook:**
```bash
# Test safe command
cd /tmp && echo "test"
# Should auto-approve

# Test blocked command
echo "testing" # This line just shows syntax, DON'T RUN: rm -rf /tmp/test
# Should block with error
```

**Metrics:**
```bash
# Approval rate
AUTO=$(grep -c "AUTO-APPROVE" devforgeai/logs/pre-tool-use.log)
TOTAL=$(wc -l < devforgeai/logs/pre-tool-use.log)
echo "Approval rate: $(echo "scale=1; $AUTO * 100 / $TOTAL" | bc)%"
# Target: >95%
```

---

## QA Lifecycle Hooks

**Purpose:** Automatically execute custom actions when QA validation completes, enabling post-test automation, result processing, and notifications.

**Invocation Point:** Phase 4.2 (QA Validation) in devforgeai-qa skill, triggered after QA passes/fails/warns.

**Current Working Directory:** Project root (`/mnt/c/Projects/DevForgeAI2`)

---

### Hook Names and Triggers

#### post-qa-success.sh
**Trigger:** QA validation PASSED (all acceptance criteria verified, no critical/high violations)

**Invocation:**
```bash
if [ -f .claude/hooks/post-qa-success.sh ]; then
    bash .claude/hooks/post-qa-success.sh "$STORY_ID"
fi
```

**Arguments:**
- `$1` = STORY_ID (e.g., "STORY-189")

**Exit Code Convention:**
- `0` = Success, proceed with next phase
- `1` = Non-fatal warning, log and continue
- `2` = Fatal error, halt QA workflow

#### post-qa-failure.sh
**Trigger:** QA validation FAILED (critical/high violations detected, acceptance criteria not met)

**Invocation:**
```bash
if [ -f .claude/hooks/post-qa-failure.sh ]; then
    bash .claude/hooks/post-qa-failure.sh "$STORY_ID"
fi
```

**Arguments:**
- `$1` = STORY_ID (e.g., "STORY-189")

**Exit Code Convention:**
- `0` = Hook completed (failure was expected/handled)
- `1` = Hook encountered warning
- `2` = Hook encountered fatal error

#### post-qa-warning.sh
**Trigger:** QA validation PASSED WITH WARNINGS (tests pass but warnings generated: coverage gaps, style issues, documentation gaps)

**Invocation:**
```bash
if [ -f .claude/hooks/post-qa-warning.sh ]; then
    bash .claude/hooks/post-qa-warning.sh "$STORY_ID"
fi
```

**Arguments:**
- `$1` = STORY_ID (e.g., "STORY-189")

**Exit Code Convention:**
- `0` = Warnings acknowledged, proceed to release
- `1` = Request manual review before release
- `2` = Escalate warnings (block release)

---

### Example Implementations

#### Example 1: Auto-Generate QA Report

**Purpose:** Automatically create QA report when tests pass, saving to `tests/results/`

**File:** `.claude/hooks/post-qa-success.sh`

```bash
#!/bin/bash
# QA Success Hook: Generate QA report and update story status

STORY_ID="${1:-UNKNOWN}"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
REPORT_DIR="tests/results"

# Create report directory if missing
mkdir -p "$REPORT_DIR"

# Generate report
REPORT_FILE="$REPORT_DIR/QA-$STORY_ID-$TIMESTAMP.md"

cat > "$REPORT_FILE" << 'EOF'
# QA Report

**Story:** STORY_PLACEHOLDER
**Date:** TIMESTAMP_PLACEHOLDER
**Status:** PASSED

## Summary
All acceptance criteria verified. No critical/high violations detected.

## Test Results
- Unit Tests: PASS
- Integration Tests: PASS
- Coverage: Meets threshold

## Acceptance Criteria Verification
- AC#1: VERIFIED
- AC#2: VERIFIED
- AC#3: VERIFIED

## Artifacts
- Test logs: tests/logs/
- Coverage report: coverage/
- Story file: devforgeai/specs/Stories/STORY_PLACEHOLDER.story.md

---
EOF

# Replace placeholders
sed -i "s/STORY_PLACEHOLDER/$STORY_ID/g" "$REPORT_FILE"
sed -i "s/TIMESTAMP_PLACEHOLDER/$TIMESTAMP/g" "$REPORT_FILE"

echo "✓ QA report generated: $REPORT_FILE"
exit 0
```

#### Example 2: Notification on Failure

**Purpose:** Log QA failures for developer review, post notification to CI/CD system

**File:** `.claude/hooks/post-qa-failure.sh`

```bash
#!/bin/bash
# QA Failure Hook: Log failure details and notify CI system

STORY_ID="${1:-UNKNOWN}"
TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
FAILURE_LOG="devforgeai/logs/qa-failures.log"

# Create log directory if missing
mkdir -p "$(dirname "$FAILURE_LOG")"

# Log failure with timestamp and story ID
echo "[${TIMESTAMP}] FAILED: $STORY_ID" >> "$FAILURE_LOG"

# Optional: Post to CI/CD webhook (if configured)
if [ -n "$CI_WEBHOOK_URL" ]; then
    curl -X POST "$CI_WEBHOOK_URL" \
        -H "Content-Type: application/json" \
        -d "{\"status\":\"failed\",\"story\":\"$STORY_ID\",\"timestamp\":\"$TIMESTAMP\"}" \
        2>/dev/null || true
fi

# Print summary to console
echo "❌ QA Failed for $STORY_ID"
echo "   Details logged to: $FAILURE_LOG"
echo "   Timestamp: $TIMESTAMP"

exit 0
```

#### Example 3: Documentation Coverage Check on Warnings

**Purpose:** Flag documentation gaps when QA passes with warnings, generate reminder for docs update

**File:** `.claude/hooks/post-qa-warning.sh`

```bash
#!/bin/bash
# QA Warning Hook: Document coverage warnings and create action item

STORY_ID="${1:-UNKNOWN}"
WARNING_DIR="devforgeai/warnings"
STORY_FILE="devforgeai/specs/Stories/$STORY_ID.story.md"

# Create warning directory
mkdir -p "$WARNING_DIR"

# Check if documentation section exists and is incomplete
if grep -q "## Generated Documentation" "$STORY_FILE"; then
    DOC_COVERAGE=$(grep -A 2 "Coverage:" "$STORY_FILE" | grep "Coverage:" | sed 's/.*: //')

    if [ -n "$DOC_COVERAGE" ] && [ "${DOC_COVERAGE%\%}" -lt 80 ]; then
        # Generate warning action item
        cat > "$WARNING_DIR/$STORY_ID-doc-warning.md" << EOF
# Documentation Coverage Warning

**Story:** $STORY_ID
**Date:** $(date +%Y-%m-%d)
**Issue:** Documentation coverage below 80% threshold

**Current Coverage:** $DOC_COVERAGE
**Threshold:** 80%

**Action Required:** Run documentation generation before release

**Next Steps:**
1. Review undocumented APIs in test results
2. Run: \`/document $STORY_ID\`
3. Re-run QA to verify coverage meets threshold
4. Delete this warning file when resolved

EOF

        echo "⚠️  Documentation warning created: $WARNING_DIR/$STORY_ID-doc-warning.md"
        exit 1  # Request manual review
    fi
fi

# No blocking warnings found
exit 0
```

---

### Hook Development Guidelines

**When Creating QA Lifecycle Hooks:**

1. **Always accept STORY_ID as argument:**
   ```bash
   STORY_ID="${1:-UNKNOWN}"
   ```

2. **Use project root for paths:**
   ```bash
   mkdir -p "devforgeai/logs"        # Create if missing
   FILE="devforgeai/logs/qa.log"     # Use absolute project paths
   ```

3. **Follow exit code convention:**
   - `0` = Success/proceed
   - `1` = Warning/review requested
   - `2` = Error/halt workflow

4. **Log operations for visibility:**
   ```bash
   echo "✓ Operation completed: description"
   echo "⚠️  Warning: description"
   echo "❌ Error: description"
   ```

5. **Test IF EXISTS pattern:**
   ```bash
   if [ -f .claude/hooks/post-qa-success.sh ]; then
       bash .claude/hooks/post-qa-success.sh "$STORY_ID"
   fi
   ```

6. **Avoid interactive prompts:**
   - Hooks run in automation context
   - Use exit codes and logging instead
   - If user input needed, create action items for manual completion

---

### Hook Testing

**Test hook execution manually:**

```bash
# Make hook executable
chmod +x .claude/hooks/post-qa-success.sh

# Test with sample story ID
bash .claude/hooks/post-qa-success.sh "STORY-189"

# Check exit code
echo $?  # Should be 0 (success)

# Verify artifacts created
ls -la devforgeai/logs/
ls -la tests/results/
```

**Validate hook syntax:**

```bash
# Check for bash syntax errors
bash -n .claude/hooks/post-qa-success.sh
# Should exit 0 with no output
```

---

### Related Documentation

**QA Workflow:**
- `.claude/skills/devforgeai-qa.md` - QA validation skill with hook integration

**Story Lifecycle:**
- `devforgeai/specs/Stories/STORY-189.story.md` - Original QA Lifecycle Hooks story

**Existing Hooks:**
- `.claude/hooks/pre-tool-use.sh` - Pre-tool-use hook for bash command approval

---

**QA Hooks Version:** 1.0
**Added:** 2025-01-08 (STORY-189)
**Status:** Ready for use
