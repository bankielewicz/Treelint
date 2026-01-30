---
description: Audit slash command character budgets against lean orchestration protocol
model: opus
allowed-tools: Read, Glob, Bash(wc:*)
---

# /audit-budget - Command Character Budget Audit

Automated compliance check for all slash commands against the 15K character budget limit defined in lean-orchestration-pattern.md protocol.

---

## Quick Reference

```bash
# Audit all commands
/audit-budget

# Show detailed breakdown
/audit-budget --verbose

# Check specific command
/audit-budget qa
```

---

## Command Workflow

### Phase 0: Load Protocol Standards

**Load lean orchestration protocol:**
```
Read(file_path="devforgeai/protocols/lean-orchestration-pattern.md")
```

**Extract budget thresholds from protocol:**
- Hard Limit: 15,000 characters (MUST refactor if exceeded)
- Warning Threshold: 12,000 characters (SHOULD refactor if approached)
- Target Range: 6,000-10,000 characters (optimal)
- Minimum: 1,000 characters (avoid over-optimization)

**Extract refactoring priority queue (if exists):**
- Protocol lines 126-148 contain current command status
- Protocol Appendix A (lines 1364-1398) contains detailed audit

---

### Phase 1: Scan All Commands

**Find all command files:**
```
Glob(pattern=".claude/commands/*.md")
```

**For each command file:**
```
# Calculate metrics
Bash(command="wc -l [file]")  # Line count
Bash(command="wc -c [file]")  # Character count

# Determine status
IF chars > 15000:
    status = "❌ OVER BUDGET"
    priority = "CRITICAL"
ELIF chars > 12000:
    status = "⚠️ HIGH USAGE"
    priority = "WARNING"
ELIF chars > 10000:
    status = "⚠️ APPROACHING"
    priority = "MONITOR"
ELSE:
    status = "✅ COMPLIANT"
    priority = "OK"

# Calculate budget percentage
budget_pct = (chars / 15000) * 100
```

**Collect results:**
```
results = []
for each command:
    results.append({
        "name": command_name,
        "lines": line_count,
        "chars": char_count,
        "budget_pct": budget_percentage,
        "status": status,
        "priority": priority
    })
```

---

### Phase 2: Analyze and Categorize

**Group commands by status:**
```
over_budget = filter(results, status == "❌ OVER BUDGET")
high_usage = filter(results, status == "⚠️ HIGH USAGE")
approaching = filter(results, status == "⚠️ APPROACHING")
compliant = filter(results, status == "✅ COMPLIANT")
```

**Sort each group:**
```
# Over-budget: Worst first (highest char count)
over_budget.sort(by=chars, descending=true)

# High usage: Closest to limit first
high_usage.sort(by=chars, descending=true)

# Approaching: Monitor list
approaching.sort(by=chars, descending=true)

# Compliant: Reference implementations
compliant.sort(by=chars, ascending=true)
```

**Calculate summary statistics:**
```
total_commands = results.count
over_count = over_budget.count
high_count = high_usage.count
compliant_count = compliant.count

compliance_rate = (compliant_count / total_commands) * 100
violation_rate = (over_count / total_commands) * 100
```

---

### Phase 3: Generate Compliance Report

**Display summary:**
```markdown
# DevForgeAI Command Budget Audit

**Audit Date:** {current_date}
**Protocol:** lean-orchestration-pattern.md (v1.0)
**Character Limit:** 15,000 (hard), 12,000 (warning), 6-10K (target)

---

## Summary Statistics

| Metric | Count | Percentage |
|--------|-------|------------|
| **Total Commands** | {total} | 100% |
| **✅ Compliant** | {compliant_count} | {compliance_rate}% |
| **⚠️ High Usage** | {high_count} | {high_rate}% |
| **❌ Over Budget** | {over_count} | {violation_rate}% |

**Compliance Status:** {PASS if over_count == 0 else FAIL}

---

## Over-Budget Commands (Immediate Action Required)

{IF over_count > 0:}

| Priority | Command | Lines | Characters | Over Budget | Refactoring Status |
|----------|---------|-------|------------|-------------|-------------------|
{FOR each in over_budget:}
| {priority_emoji} {priority} | {name} | {lines} | {chars} | +{chars - 15000} (+{((chars / 15000) - 1) * 100}%) | {status} |

**Estimated Refactoring Effort:**
- Total commands: {over_count}
- Time per command: 4-6 hours
- **Total effort: {over_count * 5} hours** (assuming 5 hours average)

**Refactoring Protocol:**
See `devforgeai/protocols/lean-orchestration-pattern.md` for methodology

{ELSE:}
✅ No over-budget commands detected. All commands within 15K character limit.
{END IF}

---

## High Usage Commands (Monitor for Growth)

{IF high_count > 0:}

| Command | Lines | Characters | Budget % | Growth Risk |
|---------|-------|------------|----------|-------------|
{FOR each in high_usage:}
| {name} | {lines} | {chars} | {budget_pct}% | {risk_level} |

**Recommendation:**
- Review quarterly for character growth
- Consider proactive refactoring if approaching 14K
- Apply lean orchestration pattern early (easier than emergency refactoring)

{ELSE:}
✅ No commands in warning zone (12K-15K).
{END IF}

---

## Compliant Commands (Reference Implementations)

{IF compliant_count > 0:}

| Command | Lines | Characters | Budget % | Notes |
|---------|-------|------------|----------|-------|
{FOR each in compliant:}
| ✅ {name} | {lines} | {chars} | {budget_pct}% | {notes} |

**Best Practices from Compliant Commands:**
- Lean orchestration: 150-300 lines
- Minimal overhead: 6-10K characters
- Pure delegation: Invoke skill, display results
- Character budget: 40-70% of limit (plenty of headroom)

{END IF}

---

## Refactoring Priority Queue

{IF over_count > 0:}

Based on lean orchestration protocol (Appendix A), refactor in this order:

1. **{worst_command}** - {chars} chars ({over_pct}% over budget)
   - Estimated effort: 4-6 hours
   - Likely extraction: {suggested_subagent} subagent
   - Reference: qa-result-interpreter pattern

2. **{second_command}** - {chars} chars ({over_pct}% over budget)
   - Estimated effort: 4-6 hours
   - Likely extraction: {suggested_subagent} subagent

[Continue for all over-budget commands]

**Total Timeline:** {total_hours} hours ({total_weeks} weeks at 10 hours/week)

{END IF}

---

## Recommendations

### Immediate Actions

{IF over_count > 0:}
- 🔴 **Refactor {over_count} over-budget commands** (blocking issue)
- 📖 **Follow protocol:** `devforgeai/protocols/lean-orchestration-pattern.md`
- 🧪 **Test comprehensively:** 30+ test cases per command (protocol section: Testing Strategy)
- ✅ **Use proven pattern:** qa-result-interpreter and dev command refactorings as reference

{ELSE:}
- ✅ **Maintain compliance:** Continue monitoring quarterly
- 📊 **Track growth:** Watch high-usage commands (>12K)

{END IF}

### Long-Term Strategy

- **Quarterly audits:** Re-run /audit-budget every 3 months
- **Automated monitoring:** Add to CI/CD (pre-commit hook)
- **Pattern enforcement:** New commands follow lean orchestration template
- **Knowledge sharing:** Document refactoring lessons learned

---

## Protocol Reference

**For complete refactoring methodology:**
```
Read: devforgeai/protocols/lean-orchestration-pattern.md

Key sections:
- Refactoring Methodology (lines 191-329)
- Command Template (lines 628-782)
- Subagent Creation Guidelines (lines 783-916)
- Case Studies (lines 1216-1264)
```

**For refactoring help:**
- Use agent-generator to create command-related subagents
- agent-generator now references protocol automatically (lines 833-1132)
- Follow validation checklist (protocol lines 1095-1107)

---

## Success Criteria

- [ ] All commands scanned successfully
- [ ] Character counts accurate (wc -c validation)
- [ ] Budget thresholds from protocol applied correctly
- [ ] Results categorized (over, high, compliant)
- [ ] Priority queue generated (if violations exist)
- [ ] Refactoring recommendations provided
- [ ] Protocol methodology referenced
- [ ] Token usage <3K (command itself is lean example)
```

---

**Display summary:**
```
Audit complete. See report above.

{IF over_count > 0:}
❌ Action required: {over_count} commands over budget
Priority 1: Refactor {worst_command} ({chars} chars, {pct}% over)

Next step: Apply lean orchestration protocol
Read: devforgeai/protocols/lean-orchestration-pattern.md

{ELSE:}
✅ All commands compliant. Framework healthy.

{END IF}
```

---

## Integration

**Invoked by:** Developers, quarterly reviews, CI/CD pipelines
**References:** lean-orchestration-pattern.md (budget limits, methodology)
**Generates:** Compliance report with priority queue
**Updates:** None (read-only audit)

**Related commands:**
- `/audit-deferrals` - Audit deferred work in stories
- This complements that with command budget audit

---

## Performance

**Token Budget:**
- Command overhead: ~2K (lean example)
- Execution: ~1K (simple scanning and calculation)
- Total: ~3K

**Execution Time:**
- Typical: 10-30 seconds (14 command files)
- Per command: ~2 seconds (wc operations)

**Character Budget:**
- Target: ~6K (exemplar lean orchestration)
- Max: <8K (demonstrates compliance)

---

## Notes

This command is itself an example of lean orchestration:
- No skill needed (task is simple enough)
- Minimal logic (scan, calculate, categorize, display)
- References protocol (active leverage, not passive documentation)
- Read-only (no modifications)
- Fast execution (<30 seconds)
- Character budget compliant (~6K)

When tasks are simple and don't require:
- Multi-phase validation
- Complex business logic
- Subagent coordination
- State modifications

...then commands can handle them directly while staying lean.

**Lean orchestration doesn't mean "always use skills"**
**It means: "Don't put business logic in commands, but simple utilities are fine"**
