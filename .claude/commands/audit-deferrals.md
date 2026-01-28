---
description: Audit all QA Approved stories for invalid deferrals and deferral chains
argument-hint: (no arguments required)
model: opus
allowed-tools: Skill, Bash
---

# /audit-deferrals - Audit Deferred Work in Stories

Audit all QA Approved and Released stories for invalid deferrals, circular deferral chains, and missing ADR documentation.

---

## Quick Reference

```bash
# Audit all completed stories
/audit-deferrals

# Typical duration: 5-15 minutes depending on story count
# Output: devforgeai/qa/deferral-audit-{timestamp}.md
```

---

## Command Workflow

### Phase 0: Argument Validation

**Validate command invocation:**
```
IF $1 provided (unexpected):
  Note: "/audit-deferrals takes no arguments"
  Continue with standard audit workflow

Proceed to Phase 1
```

---

### Phase 1: Set Context and Invoke Skill

**Set context markers for orchestration skill:**

```
**Command:** audit-deferrals
**Mode:** full-audit
```

**Invoke devforgeai-orchestration skill:**

```
Skill(command="devforgeai-orchestration")
```

**What the skill does:**

The orchestration skill executes the complete audit workflow:

1. **Phase 1 (Discover)** - Find all QA Approved and Released stories
2. **Phase 2 (Scan)** - Scan each story for deferred DoD items
3. **Phase 2.5 (Validate Blockers)** - Check if blockers are still valid or resolvable
4. **Phase 3 (Validate Deferrals)** - Invoke deferral-validator subagent for each story
   - Multi-level deferral chain detection
   - Circular deferral detection
   - Referenced story validation
   - ADR requirement verification
5. **Phase 4 (Aggregate)** - Categorize findings by severity
6. **Phase 5 (Generate Report)** - Create comprehensive audit report
7. **Phase 7 (Hook Integration)** - Execute feedback workflow if eligible (STORY-033)

---

### Phase 2: Display Results

**Display audit summary from skill:**

```
Output skill-generated summary:
- Total stories audited
- Stories with deferrals found
- Violations by severity (CRITICAL, HIGH, MEDIUM, LOW)
- Resolvable vs valid vs invalid deferrals
- Link to full audit report
```

---

### Phase 3: Provide Next Steps

**Display recommendations:**

```
IF violations found:
  Display priority-ordered actions:
  1. Critical violations requiring immediate action
  2. Resolvable deferrals (can be retried now)
  3. Invalid deferrals (must create stories/ADRs)
  4. Stale deferrals (>30 days old)

ELSE:
  "✅ All deferrals validated successfully"
  "No violations detected."
```

---

## Error Handling

### Story not found
```
IF no QA Approved or Released stories found:
  Message: "No completed stories to audit. Run /dev and /qa first."
  Action: Exit gracefully
```

### Skill execution failed
```
IF devforgeai-orchestration skill fails:
  Message: Display skill error message
  Recommendation: Check context files exist
  Action: Manual review of story files recommended
```

### Deferral validator issues
```
IF deferral-validator subagent fails:
  Message: "Validation incomplete for story {X}"
  Action: Re-run /audit-deferrals to retry
```

---

## Success Criteria

- [ ] All QA Approved stories scanned
- [ ] All Released stories included
- [ ] Deferrals categorized (resolvable, valid, invalid)
- [ ] Audit report generated and saved
- [ ] User presented with actionable recommendations
- [ ] No violations = audit passes

---

## Integration

**Invokes:**
- devforgeai-orchestration skill (Phases 1-7)
  - deferral-validator subagent (per story with deferrals)
  - feedback hooks (if eligible, STORY-033)

**Generates:**
- `devforgeai/qa/deferral-audit-{timestamp}.md` (comprehensive report)
- `devforgeai/feedback/logs/hook-invocations.log` (if hooks enabled)

**Updates:**
- None (read-only audit, doesn't modify stories)

**Uses:**
- Story files in `devforgeai/specs/Stories/` (read-only)
- Context files for framework validation
- ADR files for scope change verification

---

## Recommended Usage

**After major milestones:**
- After implementing RCA-007 fixes (audit baseline)
- After major quality gate changes
- When investigating technical debt

**Regular cadence:**
- End of each sprint (retrospective)
- Quarterly (comprehensive audit)
- Ad-hoc when needed

**Expected duration:**
- Small projects (<10 stories): 2-3 minutes
- Medium projects (10-50 stories): 5-10 minutes
- Large projects (50+ stories): 15-20 minutes

**Typical output size:**
- Summary display: ~50 lines in main conversation
- Full report: 200-400 lines in `devforgeai/qa/deferral-audit-{timestamp}.md`
- Feedback log entry: 1 line (if hooks enabled)

---

## Notes

**Framework Integration:**
- Follows lean orchestration pattern (command delegates to skill)
- Skill coordinates deferral-validator subagent for each story
- Phase 6 hook integration (STORY-033) is non-blocking
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
- `.claude/skills/devforgeai-orchestration/SKILL.md` (skill Phase 7 audit workflow)
