---
id: user-input-integration-guide
title: User Input Guidance Integration Guide - devforgeai-story-creation Skill
version: "1.0"
created: 2025-01-21
updated: 2025-01-21
status: Published
audience: DevForgeAI Framework (Internal)
---

# User Input Guidance Integration Guide

**Purpose:** Documentation for integrating user-input-guidance.md patterns into devforgeai-story-creation skill's Phase 1 to improve question quality and reduce subagent re-invocations.

---

## 1. Pattern Mapping Table

Maps Phase 1 Story Discovery questions to user-input-guidance.md patterns.

### Mapping (YAML Format)

```yaml
integration_version: "1.0"
skill_name: "devforgeai-story-creation"
patterns_file: "src/claude/skills/devforgeai-ideation/references/user-input-guidance.md"

phase_1_pattern_mapping:

  step_3_epic_selection:
    question_type: "Epic Association"
    pattern_name: "Explicit Classification + Bounded Choice"
    pattern_normalized: "explicit classification bounded choice"
    rationale: "Epic selection requires explicit list + bounded choice"

  step_4_sprint_assignment:
    question_type: "Sprint Assignment"
    pattern_name: "Bounded Choice"
    pattern_normalized: "bounded choice"
    rationale: "Sprint assignment requires bounded list with capacity"

  step_5_priority_selection:
    question_type: "Story Priority"
    pattern_name: "Explicit Classification"
    pattern_normalized: "explicit classification"
    rationale: "Priority has 4 defined values (Critical/High/Medium/Low)"

  step_5_story_points:
    question_type: "Story Complexity Estimation"
    pattern_name: "Fibonacci Bounded Choice"
    pattern_normalized: "fibonacci bounded choice"
    rationale: "Story points use Fibonacci sequence for estimation"
```

---

## 2. Batch Mode Caching Strategy

When `/create-story` invokes batch mode (e.g., epic decomposition → 9 stories):

- Story 1: Load guidance, store in GUIDANCE_CACHE
- Stories 2-9: Reuse GUIDANCE_CACHE from conversation context
- Token efficiency: 89 tokens/story amortized (1,000 / 9 ÷ 1.25)

---

## 3. Token Budget Optimization

### Targets
- Step 0 overhead: ≤ 1,000 tokens (strict)
- Phase 1 increase: ≤ 5% vs baseline
- Read tool: Called once per session, not per question

### Selective Loading (If Budget Exceeded)
- Extract only 4 critical patterns for story-creation
- If still >1,000 tokens: Fallback to baseline

---

## 4. Backward Compatibility Mechanisms

### Graceful Degradation Hierarchy
```
Level 1: Full Patterns (all available)
Level 2: Partial Patterns (some available)
Level 3: Selective Loading (token budget reduced)
Level 4: Baseline (guidance unavailable)

Guarantee: Workflow ALWAYS completes
```

### Per-Question Fallback
Each question has independent fallback:
- Pattern available → Apply pattern
- Pattern missing → Use baseline
- Pattern throws exception → Use baseline

---

## 5. Pattern Update Process

When user-input-guidance.md changes:
- Pattern renamed: Mapping auto-adapts (normalized names)
- New pattern: Update mapping table if applicable
- Deprecated: Mark in guidance, add migration logic
- NO SKILL.md changes needed (patterns loaded dynamically)

---

## 6. Example Transformations

### Epic Selection: Before/After

**Baseline:**
```
Question: "Which epic does this story belong to?"
Options: [EPIC-001, EPIC-002, EPIC-003, None]
```

**Enhanced:**
```
Header: "Epic Association - Enables feature tracking and traceability"
Options:
  - EPIC-001: User Management (Status: Active, 8 features, Medium complexity)
  - EPIC-002: Reporting (Status: Active, 12 features, High complexity)
  - None - Standalone story (Independent story)
```

**Benefits:** Context, clarity, no ambiguity

### Priority Selection: Before/After

**Baseline:**
```
Options: [Critical, High, Medium, Low]
```

**Enhanced:**
```
- Critical: Blocks other work, production issue. Example: Auth broken
- High: Important for release. Example: Dashboard metrics
- Medium: Improves product, can wait. Example: Notification optimization
- Low: Nice to have, tech debt. Example: Documentation improvements
```

**Benefits:** Clear decision criteria, consistent prioritization

---

## 7. Edge Case Handling

| Edge Case | Recovery |
|-----------|----------|
| Guidance file missing during batch | Log single warning, use baseline for all stories |
| User selects "None" for epic | Valid choice, continue with epic_id = null |
| Non-Fibonacci points request | Follow-up question, convert to closest Fibonacci |
| Metadata collection interrupted | Detect incomplete, HALT with clear message |
| Batch mode + pattern conflict | No conflict (batch skips patterns, uses markers) |
| Guidance exceeds token budget | Selective loading (4 critical patterns) |
| Pattern lookup fails mid-execution | Per-question fallback (one miss doesn't cascade) |

---

## 8. Testing Procedures

### Unit Tests (15 minimum)
- Step 0 loads guidance successfully
- Step 0 handles missing file gracefully
- Pattern extraction works
- Pattern name normalization correct
- Pattern lookup succeeds/fails
- Token budget validated
- Baseline fallback when guidance unavailable
- Epic, sprint, priority, points pattern application verified

### Integration Tests (12 minimum)
- Full Phase 1 with/without guidance
- Subagent re-invocation reduction ≥30%
- Token overhead ≤5%
- Backward compatibility (30+ existing tests)
- Batch mode caching (Read once)
- End-to-end workflow

### Regression Tests (10 minimum)
- All existing Phase 1 questions work
- Phases 2-8 unaffected
- Skill output format unchanged
- Story template structure unchanged

### Performance Tests (8 minimum)
- Step 0 p95 < 2 seconds
- Token overhead ≤ 1,000
- Pattern lookup < 50ms
- Memory < 5MB

---

## 9. Troubleshooting Guide

### Pattern Not Applying to Epic Question
**Root Causes:** Missing file | Not readable | Parse failed | Name mismatch
**Check:** `grep "GUIDANCE_AVAILABLE\|patterns extracted" logs`
**Fix:** Restore file, fix permissions, verify markdown syntax

### Guidance Exceeds Token Budget
**Root Causes:** File too large (>15K) | Too many patterns (>20)
**Check:** File size and pattern count
**Fix:** Split patterns, trim examples, selective loading

### Batch Mode Not Using Cache
**Root Causes:** BATCH_MODE not detected | Cache not persisted
**Check:** Batch marker in context, Read call count
**Fix:** Verify marker spelling, ensure context preserved

### Pattern Fails Silently
**Root Causes:** apply_pattern threw exception | Pattern incomplete
**Check:** Exception messages, pattern template structure
**Fix:** Debug exception, verify pattern format

---

## 10. Glossary

### Patterns
- **Explicit Classification:** Fixed set of predefined values (no "Other")
- **Bounded Choice:** Finite list (not open-ended)
- **Fibonacci Bounded Choice:** Bounded choice using Fibonacci [1,2,3,5,8,13]

### Framework Terms
- **Epic:** Business initiative spanning stories (`devforgeai/specs/Epics/`)
- **Sprint:** 2-week iteration (`devforgeai/specs/Sprints/`)
- **Priority:** Urgency: Critical/High/Medium/Low
- **Story Points:** Complexity: 1,2,3,5,8,13
- **GUIDANCE_AVAILABLE:** Boolean flag - patterns ready
- **BATCH_MODE:** Boolean flag - batch creation active
- **GUIDANCE_CACHE:** In-memory guidance, reused in batch

---

## 11. Related Documentation

### Source Patterns
- **[user-input-guidance.md](../../../devforgeai-ideation/references/user-input-guidance.md)** - Pattern definitions and integration framework used by all target skills

### Related Stories
- **STORY-056:** devforgeai-story-creation Integration (this implementation) - Implements patterns in skill Phase 1
- **STORY-055:** devforgeai-ideation Integration (parallel implementation) - Implements patterns in ideation skill phases
- **STORY-064:** devforgeai-story-creation Integration Validation (test execution) - Validates patterns work in production

### Bidirectional Navigation
- user-input-guidance.md Section 5.2 → References this guide for devforgeai-story-creation implementation details
- This guide Section 1 → References user-input-guidance.md for pattern definitions

---

**Last Updated:** 2025-01-21
**Version:** 1.0
**Status:** Ready for Production
**Framework Version:** 1.0.1
