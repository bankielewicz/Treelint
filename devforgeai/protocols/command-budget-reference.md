# Command Budget Reference - Lean Orchestration Pattern

**Part of:** Lean Orchestration Pattern Protocol
**Main Document:** `lean-orchestration-pattern.md`
**Related:** `refactoring-case-studies.md`

**Version:** 1.1
**Date:** 2025-11-18 (Updated with current metrics from STORY-039)

---

## Overview

This document provides current command budget status, monitoring procedures, and detailed appendices for the lean orchestration pattern.

**Budget limits:**
- **Hard limit:** 15,000 characters (must refactor if exceeded)
- **Warning:** 12,000 characters (approaching limit)
- **Target:** 6,000-10,000 characters (optimal)

---

## Current Command Status (2025-11-18)

### Summary Statistics

| Category | Count | Commands |
|----------|-------|----------|
| **✅ Compliant** (<12K) | 4 | qa, create-epic, 3 test commands |
| **⚠️ High Usage** (12-15K) | 7 | create-sprint, ideate, create-context, audit-deferrals, orchestrate, create-story, create-agent |
| **❌ Over Budget** (>15K) | 3 | dev, release, create-ui |
| **✅ Refactored** | 7 | dev, qa, ideate, create-sprint, create-epic, orchestrate, create-story |

**⚠️ NOTE:** `/dev` command has regressed to 116% budget (was 84% after refactoring). Requires immediate attention.

### Detailed Command Budget Table

| Command | Lines | Characters | Budget % | Status | Priority |
|---------|-------|------------|----------|--------|----------|
| test-slashcommand-isolation | 31 | 987 | 7% | ✅ Compliant | — |
| test-skill-context | 57 | 1,570 | 10% | ✅ Compliant | — |
| test-arg-validation | 180 | 4,151 | 28% | ✅ Compliant | — |
| **qa** | **309** | **8,443** | **56%** | ✅ Compliant | ✅ **Reference** |
| **create-sprint** | **526** | **13,457** | **89%** | ⚠️ High | ✅ **Refactored** |
| **create-epic** | **396** | **11,532** | **76%** | ⚠️ High | ✅ **Refactored** |
| ideate | 410 | 11,717 | 78% | ⚠️ High | ✅ Refactored |
| create-context | 420 | 12,631 | 84% | ⚠️ High | 🟡 Watch |
| **dev** | **527** | **17,460** | **116%** | ❌ OVER | 🔴 **CRITICAL** |
| audit-deferrals | 452 | 13,088 | 87% | ⚠️ High | 🟡 Watch |
| **orchestrate** | **535** | **14,854** | **99%** | ⚠️ High | ✅ **Refactored** |
| **create-story** | **482** | **14,895** | **99%** | ⚠️ High | ✅ **Refactored** |
| **release** | **655** | **18,166** | **121%** | ❌ OVER | 🔴 **HIGH** |
| **create-ui** | **614** | **18,908** | **126%** | ❌ OVER | 🔴 **CRITICAL** |

### Refactoring Priority Queue

**Priority 1: CRITICAL (Over Budget)**
- **create-ui** (19K chars, 126% over) - ELEVATED to CRITICAL
  - Requires: 47% reduction to reach 10K target
  - Estimated effort: 3-4 hours
  - Already uses devforgeai-ui-generator skill
  - Extract: Integration logic to subagent (or further delegation)

**Priority 2: HIGH (Over Budget)**
- **create-ui** (19K chars, 126% over)
  - Requires: 47% reduction to reach 10K target
  - Estimated effort: 3-4 hours
  - Already uses devforgeai-ui-generator skill
  - Extract: Integration logic to subagent

- **release** (18K chars, 121% over)
  - Requires: 45% reduction to reach 10K target
  - Estimated effort: 2-3 hours
  - Subagent candidate: release-orchestrator (deployment sequence)

**Priority 3: WATCH (Approaching Limit, 90-96%)**
- **orchestrate** (14.4K chars, 96% - NOW COMPLIANT after refactoring!)
  - Was 100% (15,012 chars)
  - Refactored to 96% (14,422 chars)
  - Under limit by 578 chars
  - No further action needed

**Priority 4: MONITOR (High Usage, 80-87%)**
- create-context (12.6K chars, 84%)
- dev (12.6K chars, 84%) ✅ Refactored
- audit-deferrals (13K chars, 87%)

**Priority 5: COMPLIANT (Reference Implementations)**
- qa (7.2K chars, 48%) ✅ Refactored - Best reference
- create-sprint (8K chars, 53%) ✅ Refactored - Good reference
- create-epic (11.3K chars, 75%) ✅ Refactored - Recent reference

---

## Monitoring Procedures

### Automated Budget Check

```bash
# Quick budget check for all commands
for cmd in .claude/commands/*.md; do
  chars=$(wc -c < "$cmd")
  name=$(basename "$cmd" .md)
  percent=$((chars * 100 / 15000))

  status="✅ COMPLIANT"
  [ $chars -gt 15000 ] && status="❌ OVER"
  [ $chars -gt 12000 ] && [ $chars -le 15000 ] && status="⚠️ HIGH"

  printf "%-30s %7d chars (%3d%%) %s\n" "$name" "$chars" "$percent" "$status"
done | sort -t'(' -k2 -n
```

### Monthly Budget Review

**Execute on 1st of each month:**

```bash
# 1. Generate budget report
bash devforgeai/scripts/check-command-budgets.sh > reports/budget-$(date +%Y-%m).txt

# 2. Identify growth
diff reports/budget-$(date -d 'last month' +%Y-%m).txt reports/budget-$(date +%Y-%m).txt

# 3. Flag commands that grew >10%
# (Manual review needed)

# 4. Update priority queue if needed
```

**Review checklist:**
- [ ] Any new commands over 15K?
- [ ] Any commands grew >10% since last month?
- [ ] Commands approaching 12K threshold?
- [ ] Refactored commands stayed under budget?
- [ ] New patterns to add to protocol?

### Quarterly Deep Review

**Execute every 3 months:**

1. **Audit all commands**
   - Run automated budget check
   - Review growth trends
   - Identify refactoring candidates

2. **Review refactoring effectiveness**
   - Compare before/after metrics
   - Validate token savings
   - Check for regressions

3. **Update protocol**
   - Add new patterns discovered
   - Document new anti-patterns
   - Update case studies

4. **Plan next quarter**
   - Prioritize refactorings
   - Allocate effort estimates
   - Set compliance targets

---

## Continuous Improvement

### When to Update Protocol

**Add to refactoring checklist when:**
- New anti-pattern discovered
- New extraction technique proven
- New subagent creation trigger identified
- New testing scenario needed
- Reference file pattern improved

**Update case studies when:**
- New command refactored
- Significant lessons learned
- Pattern variation discovered
- Novel solution implemented

**Enhance templates when:**
- Better command structure emerges
- Improved subagent coordination pattern
- More effective reference file structure
- Clearer testing procedures

---

## Appendix A: Command Budget Analysis (2025-11-06)

### Budget Compliance Over Time

| Date | Compliant | High | Over | Total |
|------|-----------|------|------|-------|
| 2025-10-01 | 3 | 2 | 6 | 11 |
| 2025-11-01 | 3 | 3 | 5 | 11 |
| 2025-11-05 | 5 | 5 | 4 | 14 |
| 2025-11-06 | 5 | 5 | 3 | 13 |

**Trend:** Improving (6 → 5 → 4 → 3 over-budget commands)

**Goal:** 0 over-budget commands by 2025-12-01

### Budget Distribution

**Current budget usage:**
```
0-5K:     3 commands (test commands)
5-10K:    2 commands (qa, create-sprint) ✅ Optimal
10-12K:   3 commands (create-epic, ideate, create-context)
12-15K:   3 commands (dev, audit-deferrals, orchestrate)
15-20K:   2 commands (release, create-ui)
20-25K:   1 command (create-story)

Target: Shift all commands to 5-12K range
```

**Optimal distribution:**
```
5-8K:  Simple commands (qa, create-sprint, audit-budget)
8-10K: Standard commands (create-epic, ideate)
10-12K: Complex commands (dev, orchestrate)
```

---

## Appendix B: Token Efficiency Comparison

### Per-Command Token Savings

| Command | Before (tokens) | After (tokens) | Savings | % Reduction |
|---------|-----------------|----------------|---------|-------------|
| /dev | 15,000 | 5,000 | 10,000 | 67% |
| /qa | 8,000 | 2,700 | 5,300 | 66% |
| /create-sprint | 12,000 | 5,000 | 7,000 | 58% |
| /create-epic | 10,000 | 2,000 | 8,000 | 80% |
| /orchestrate | 4,000 | 2,500 | 1,500 | 37% |
| **AVERAGE** | **9,800** | **3,440** | **6,360** | **62%** |

### Framework-Wide Impact

**Total token budget:** 1,000,000 tokens available

**Before refactorings (all 11 commands):**
- Estimated: ~110,000 tokens consumed by commands
- Available for work: ~890,000 tokens

**After refactorings (6 refactored, 5 pending):**
- Estimated: ~75,000 tokens consumed by commands (6 refactored + 5 old)
- Available for work: ~925,000 tokens
- **Gain:** +35,000 tokens

**Projected (all 11 commands refactored):**
- Estimated: ~42,000 tokens consumed by commands
- Available for work: ~958,000 tokens
- **Gain:** +68,000 tokens (62% reduction)

---

## Appendix C: Reference File Patterns

### Effective Reference Files (Examples)

**qa-result-formatting-guide.md** (580 lines):
- DevForgeAI context (workflow states, quality gates)
- Framework constraints (coverage thresholds, violation rules, deferral handling)
- Display guidelines (templates, tone, emoji usage)
- Integration points (context files, related components)

**Key Success Factor:** Makes implicit constraints explicit for subagents

**sprint-planning-guide.md** (631 lines):
- Capacity guidelines (20-40 points recommended)
- Sprint date calculation algorithms
- Story status transition rules
- Workflow history templates
- Epic integration patterns

**Key Success Factor:** Prevents autonomous behavior through explicit rules

**feature-decomposition-patterns.md** (903 lines):
- Epic → feature breakdown patterns by domain
- CRUD feature templates
- Authentication workflow patterns
- API endpoint decomposition
- Reporting feature structures

**Key Success Factor:** Guides requirements-analyst subagent with proven patterns

### Reference File Quality Checklist

**Excellent reference file has:**
- [ ] Purpose clearly stated (prevent what behavior?)
- [ ] Framework context provided (workflow states, quality gates)
- [ ] Constraints documented (immutable rules with rationale)
- [ ] Guidelines specific (how to do task within constraints)
- [ ] Examples included (concrete cases from framework)
- [ ] Integration points (what to reference, when to invoke others)
- [ ] Error scenarios (how to handle gracefully)
- [ ] Testing checklist (validate framework compliance)

**Target size:** 200-900 lines (focused, comprehensive)

### Reference File Organization

**By purpose:**
- **Workflow files:** Step-by-step execution procedures (coverage-analysis-workflow.md)
- **Guide files:** Reference material, patterns, checklists (coverage-analysis.md)
- **Template files:** Structured templates for generation (epic-template.md)

**By usage:**
- **Always loaded:** Core workflow files (loaded at phase start)
- **Conditionally loaded:** Mode-specific files (deep-only, light-only)
- **Rarely loaded:** Troubleshooting, edge cases (loaded on error)

---

## Appendix D: Rollback Procedures

### Immediate Rollback (<15 minutes)

**When to rollback:**
- Refactored command fails tests
- Functionality changed unexpectedly
- Performance degraded significantly
- Users report issues

**Rollback steps:**

```bash
# 1. Restore original command
git checkout HEAD~ .claude/commands/[command].md

# 2. Remove new subagent (if created)
rm .claude/agents/[new-subagent].md

# 3. Remove reference file (if created)
rm .claude/skills/[skill]/references/[new-guide].md

# 4. Restore skill (if modified)
git checkout HEAD~ .claude/skills/[skill]/SKILL.md

# 5. Restore memory references
git checkout HEAD~ .claude/memory/*.md

# 6. Restart terminal
# Terminal will reload original commands

# 7. Verify original behavior
/[command] [test-args]

# 8. Document issue
# Create rollback report: devforgeai/specs/enhancements/[COMMAND]-rollback-report.md
```

**Validation:**
- [ ] Original command restored
- [ ] New artifacts removed
- [ ] Terminal restarted
- [ ] Original behavior verified
- [ ] Rollback documented

### Root Cause Analysis

After rollback, investigate:
- [ ] **What failed?** (skill invocation, subagent, parsing)
- [ ] **Why failed?** (missing context, wrong tool access, logic error)
- [ ] **Test gaps?** (which test case missed the issue)
- [ ] **Fix approach?** (update subagent, skill, or command)
- [ ] **Prevention?** (add to testing checklist)

**Document findings in:**
- `devforgeai/specs/enhancements/[COMMAND]-rollback-report.md`

---

## Appendix E: Framework Evolution

### Command Character Budget Trend

**Historical timeline:**
- **Phase 1 (Oct 2025):** Commands created, no budget awareness
  - Commands grew organically
  - No character limits enforced
  - Some exceeded 30K chars

- **Phase 2 (Oct 2025):** Budget violations identified
  - /audit-budget command created
  - lean-orchestration-pattern.md protocol written
  - Refactoring methodology established

- **Phase 3 (Nov 2025):** First refactorings
  - /dev refactored (860 → 513 lines, 40% reduction)
  - /qa refactored (692 → 295 lines, 57% reduction)
  - Pattern proven effective

- **Phase 4 (Nov 2025):** Pattern maturity
  - /create-sprint refactored (497 → 250 lines, 50% reduction)
  - /create-epic refactored (526 → 392 lines, 25% reduction)
  - /orchestrate refactored (599 → 527 lines, 12% reduction)
  - Pattern refined through experience

- **Phase 5 (Nov 2025):** Skill refactorings planned
  - 8 skills identified for progressive disclosure refactoring
  - Reddit article validates 200-line rule
  - Comprehensive refactoring plans created

- **Phase 6 (Future):** Remaining command refactorings
  - create-story, create-ui, release
  - Target: 100% budget compliance
  - All commands 6-12K optimal range

### Pattern Maturity Evolution

**Lessons from 5 refactorings:**

1. **Lean orchestration is achievable** (25-57% reduction proven across all 5)
2. **Subagents essential for context isolation** (35-40K tokens moved to isolated contexts)
3. **Reference files critical for framework constraints** (prevents autonomous behavior, guides subagents)
4. **Testing prevents regressions** (30+ test cases standard, 100% pass rate)
5. **Token efficiency improves significantly** (37-80% savings in main conversation)
6. **User interaction stays in command** (AskUserQuestion preserved where needed)
7. **Implementation time improving** (73 min for /create-sprint, 180 min for /create-epic with skill enhancement)
8. **Educational content belongs in reference docs** (not commands) - enables further size reduction
9. **Two-pass trimming sometimes necessary** (first pass may still be over budget)
10. **Mode detection enables multi-purpose skills** (orchestration handles epic, sprint, story modes)

**Next evolution:**
- Apply pattern to remaining 3 over-budget commands
- Refactor 8 skills using 200-line rule (Reddit article pattern)
- Establish automated budget monitoring CI/CD
- Target: 100% lean pattern compliance (all 11 commands)
- Create command scaffolding tool
- Build refactoring runbook automation

---

## Appendix F: Success Metrics

### Command Quality Targets

| Metric | Target | Best (/qa) | Good (/sprint) | Acceptable (/dev) |
|--------|--------|------------|----------------|-------------------|
| **Lines** | 150-300 | 295 ✅ | 250 ✅ | 513 ⚠️ |
| **Characters** | 6K-12K | 7.2K ✅ | 8K ✅ | 12.6K ⚠️ |
| **Budget %** | <80% | 48% ✅ | 53% ✅ | 84% ⚠️ |
| **Phases** | 3-5 | 4 ✅ | 4 ✅ | 4 ✅ |
| **Token Overhead** | <3K | 2.7K ✅ | 5K ⚠️ | 5K ⚠️ |

### Framework Compliance Checklist

- [x] All commands <15K characters (budget compliance) - 3 remaining
- [x] Commands delegate business logic (lean orchestration) - 6 of 11
- [x] Skills contain comprehensive implementation - Yes
- [x] Subagents handle specialized tasks (framework-aware) - Yes
- [x] Reference files provide guardrails (explicit constraints) - Yes
- [x] No duplication between command/skill/subagent - Validated
- [x] Token efficiency >50% improvement (vs pre-refactoring) - 37-80% achieved

**Progress:** 6 of 11 commands (55%) fully compliant with lean pattern

**Target:** 11 of 11 commands (100%) by December 2025

---

## Appendix G: Testing Strategy

### Standard Test Suite (Per Command)

**Unit Tests (10-15 cases):**
- Argument validation scenarios
  - Valid input
  - Invalid format
  - Missing arguments
  - Extra arguments
  - Flag syntax (if applicable)
- Context loading edge cases
  - File not found
  - Invalid file format
  - Corrupted YAML
- Skill invocation with various inputs
  - Different story IDs
  - Different modes/environments
  - Edge case parameters
- Subagent output parsing (if applicable)
  - Valid output
  - Incomplete output
  - Error responses
- Error handling paths
  - Each error type
  - Recovery procedures
  - User guidance

**Integration Tests (8-12 cases):**
- Full workflow with real stories
  - Story in each workflow state
  - Different story types (CRUD, auth, etc.)
- Mode/option variations
  - All modes tested (light/deep, staging/production, etc.)
  - Default vs explicit parameters
- Failure scenarios with recovery
  - Skill fails
  - Subagent fails
  - Validation fails
- Status transitions
  - Each valid transition
  - Invalid transitions blocked
- Retry cycles (if applicable)
  - Max retries enforced
  - Loop prevention works

**Regression Tests (8-10 cases):**
- Behavior unchanged from original
  - Same output format
  - Same error messages
  - Same success criteria
- Quality gates still enforced
  - No gates bypassed
  - Same thresholds
- Thresholds still respected
  - Coverage requirements
  - Validation rules
- Status transitions same
  - Same progression rules
- Error messages preserved
  - User-friendly messages
  - Actionable guidance

**Performance Tests:**
- Token budget: Command <3K tokens
- Character budget: <12K chars (target) or <15K (max)
- Execution time: Within expected range
  - Simple commands: <2 minutes
  - Complex commands: <10 minutes

### Test Execution

```bash
# Run all tests for a command
bash devforgeai/tests/commands/test-[command].sh

# Expected output:
# Unit Tests: 15/15 passed ✅
# Integration Tests: 12/12 passed ✅
# Regression Tests: 10/10 passed ✅
# Performance Tests: 4/4 passed ✅
# Overall: 41/41 passed ✅
```

---

## Appendix H: Refactoring Roadmap

### Immediate Actions (Priority 1 - Next 2 Weeks)

**Commands requiring refactoring NOW:**

1. **create-story** (23K chars, 153% over budget) - CRITICAL
   - Current: 857 lines, massive
   - Target: ~300 lines, ~10K chars
   - Extract to: story-formatter subagent (YAML + markdown generation)
   - Reference: story-template-guide.md
   - Estimated: 3-4 hours
   - Blocker: None (independent)

2. **create-ui** (19K chars, 126% over budget) - HIGH
   - Current: 614 lines
   - Target: ~300 lines, ~10K chars
   - Already uses devforgeai-ui-generator skill ✓
   - Issue: Too much integration logic
   - Extract to: Enhance skill with ui-spec-formatter integration (already done)
   - Estimated: 2-3 hours
   - Blocker: None (independent)

3. **release** (18K chars, 121% over budget) - HIGH
   - Current: 655 lines
   - Target: ~300 lines, ~10K chars
   - Extract to: release-orchestrator subagent (deployment sequence)
   - Reference: release-workflow-guide.md
   - Estimated: 2-3 hours
   - Blocker: None (independent)

**Total effort:** 7-10 hours (can be done in parallel across 3 sessions)

### Near-Term Actions (Priority 2-3 - Next Month)

**Refactor 8 skills for progressive disclosure (Reddit article pattern):**

All 8 DevForgeAI skills need 200-line entry point refactoring.

**Priority order:**
1. devforgeai-orchestration (3,249 lines → ~200) - 4-6 hours
2. devforgeai-story-creation (1,840 lines → ~180) - 3-4 hours
3. devforgeai-development (1,782 lines → ~180) - 3-4 hours
4. devforgeai-ui-generator (1,451 lines → ~190) - 3-4 hours
5. devforgeai-ideation (1,416 lines → ~185) - 3 hours
6. devforgeai-qa (1,330 lines → ~190) - 3-4 hours
7. devforgeai-architecture (978 lines → ~195) - 2-3 hours
8. devforgeai-release (791 lines → ~195) - 2-3 hours

**Total effort:** 24-32 hours (can be done in parallel across 8 sessions)

**Detailed plans created:** `devforgeai/specs/analysis/devforgeai-[skillname].md` (8 documents)

### Long-Term Strategy (Next Quarter)

**Continuous improvement:**
- Automated budget monitoring (CI/CD integration)
- Command scaffolding generator
- Refactoring runbook automation
- Budget trend analysis dashboard

**Maintenance:**
- Monthly budget audits
- Quarterly deep reviews
- Pattern updates as learned
- Case study additions

---

## Appendix I: Related Documentation

### Core Principles

- `.ai_docs/claude-skills.md` - Skills architecture and progressive disclosure
- `.ai_docs/Terminal/slash-commands-best-practices.md` - Command design patterns
- `.ai_docs/Terminal/sub-agents.md` - Subagent architecture and context isolation
- `.ai_docs/native-tools-vs-bash-efficiency-analysis.md` - Tool efficiency (40-73% savings)

### Implementation Examples

**Commands (Refactored):**
- `.claude/commands/dev.md` - Reference implementation (513 lines, 84% budget)
- `.claude/commands/qa.md` - Reference implementation (295 lines, 48% budget) ⭐ Best
- `.claude/commands/create-sprint.md` - Reference implementation (250 lines, 53% budget)
- `.claude/commands/create-epic.md` - Reference implementation (392 lines, 75% budget)
- `.claude/commands/orchestrate.md` - Reference implementation (527 lines, 96% budget)

**Subagents (Created During Refactorings):**
- `.claude/agents/qa-result-interpreter.md` - Display template generation (300 lines)
- `.claude/agents/sprint-planner.md` - Document generation (467 lines)
- `.claude/agents/git-validator.md` - Pre-flight validation (250 lines)
- `.claude/agents/tech-stack-detector.md` - Technology validation (300 lines)

**Reference Files (Created During Refactorings):**
- `.claude/skills/devforgeai-qa/references/qa-result-formatting-guide.md` (580 lines)
- `.claude/skills/devforgeai-orchestration/references/sprint-planning-guide.md` (631 lines)
- `.claude/skills/devforgeai-orchestration/references/feature-decomposition-patterns.md` (903 lines)
- `.claude/skills/devforgeai-orchestration/references/technical-assessment-guide.md` (914 lines)
- `.claude/skills/devforgeai-orchestration/references/epic-validation-checklist.md` (760 lines)

### Detailed Refactoring Documentation

**Analysis documents:**
- `devforgeai/specs/enhancements/QA-COMMAND-REFACTORING-ANALYSIS.md` - Deep dive
- `devforgeai/specs/enhancements/QA-COMMAND-REFACTORING-SUMMARY.md` - Architecture overview
- `devforgeai/specs/enhancements/QA-COMMAND-REFACTORING-CHECKLIST.md` - Testing strategy
- `devforgeai/specs/enhancements/CREATE-SPRINT-REFACTORING-PLAN.md` - Refactoring plan
- `devforgeai/specs/enhancements/CREATE-SPRINT-REFACTORING-SUMMARY.md` - Implementation summary
- `devforgeai/specs/enhancements/ORCHESTRATE-COMPLETE-2025-11-06.md` - Orchestrate refactoring complete

**Skill refactoring plans (NEW):**
- `devforgeai/specs/analysis/devforgeai-orchestration.md` (2,088 lines)
- `devforgeai/specs/analysis/devforgeai-story-creation.md` (1,956 lines)
- `devforgeai/specs/analysis/devforgeai-development.md` (1,923 lines)
- `devforgeai/specs/analysis/devforgeai-ui-generator.md` (1,875 lines)
- `devforgeai/specs/analysis/devforgeai-ideation.md` (1,804 lines)
- `devforgeai/specs/analysis/devforgeai-qa.md` (1,843 lines)
- `devforgeai/specs/analysis/devforgeai-architecture.md` (1,712 lines)
- `devforgeai/specs/analysis/devforgeai-release.md` (1,603 lines)

---

## Quick Reference Cards

### Command Checklist

Before deploying a new or updated command:
- [ ] Lines: 150-300 (target)
- [ ] Characters: <12K (target) or <15K (max)
- [ ] Phases: 3-5 (lean workflow)
- [ ] Business logic: None (delegated to skill)
- [ ] Display templates: None (generated by skill/subagent)
- [ ] Error handling: Minimal (3-5 types)
- [ ] Documentation: Concise (integration notes only)
- [ ] Tests: 30+ cases (unit, integration, regression)
- [ ] Token budget: <3K in main conversation

### Refactoring Trigger Checklist

Refactor when:
- [ ] Command >15K characters (MUST)
- [ ] Command >12K characters (SHOULD)
- [ ] Command has business logic (validation, calculations)
- [ ] Command has display templates (>50 lines)
- [ ] Command reads files it didn't create (duplication)
- [ ] Command has complex error matrix (>50 lines)
- [ ] Command bypasses skill layer (command → subagent directly)

### Pattern Application Checklist

1. **Analyze:**
   - [ ] Count lines per phase
   - [ ] Identify business logic
   - [ ] Find duplication
   - [ ] Measure token usage

2. **Extract:**
   - [ ] Move business logic to skill
   - [ ] Create subagent for specialized tasks
   - [ ] Create reference files for guardrails
   - [ ] Delete redundant content

3. **Test:**
   - [ ] 10-15 unit tests
   - [ ] 8-12 integration tests
   - [ ] 8-10 regression tests
   - [ ] 4 performance tests

4. **Deploy:**
   - [ ] Create backup
   - [ ] Refactor command
   - [ ] Update skill/subagents
   - [ ] Verify budget <15K
   - [ ] Commit changes

5. **Document:**
   - [ ] Update analysis doc
   - [ ] Update commands-reference.md
   - [ ] Update subagents-reference.md (if applicable)
   - [ ] Add case study to refactoring-case-studies.md

---

**This budget reference is a living document. Update monthly with current command status and quarterly with trend analysis.**

**Character count:** ~17,264 characters (well under 40K limit)
