# /create-epic Refactoring Plan: Lean Orchestration Pattern

**Date:** 2025-11-05
**Status:** Planning
**Priority:** 🟡 MEDIUM (95% of budget - approaching limit)
**Pattern:** Lean Orchestration (proven in /dev, /qa, /ideate, /create-story)

---

## Executive Summary

Refactor `/create-epic` slash command from 526 lines (14,309 chars - 95% of budget) to ~250 lines (~8K chars - 53% of budget) by extracting business logic to `devforgeai-orchestration` skill which already has epic creation infrastructure.

**Key Finding:** `devforgeai-orchestration` skill ALREADY declares epic creation capability (line 50: `--create-epic`) and references `epic-management.md` (496 lines) and `epic-template.md` (265 lines) which exist but are not fully utilized.

**Strategy:** Enhance orchestration skill's existing epic creation mode, refactor command to lean orchestration pattern.

---

## Current State Analysis

### /create-epic Command (526 lines, 14,309 chars - 95% budget)

**Character Budget Status:**
```
Current: 14,309 characters
Hard Limit: 15,000 characters
Warning: 12,000 characters
Target: 6,000-10,000 characters
Compliance: ⚠️ HIGH USAGE (95% - approaching limit)
```

**Architecture Violations:**

| Should Do | Currently Does | Violation |
|-----------|----------------|-----------|
| Parse arguments | ✅ Lines 13-15 | ✅ Compliant |
| Load context via @file | ❌ None | ❌ MISSING |
| Set context markers | ❌ None | ❌ MISSING |
| Invoke skill | ❌ None | ❌ MISSING |
| Display results | ❌ None | ❌ MISSING |

| Should NOT Do | Currently Does | Violation |
|---------------|----------------|-----------|
| Business logic | ✅ Phases 1-7 (350+ lines) | ❌ VIOLATES |
| Subagent invocation | ✅ Lines 92-158 (requirements-analyst, technical-assessor) | ❌ VIOLATES |
| Template generation | ✅ Lines 186-284 (epic document) | ❌ VIOLATES |
| Complex decision-making | ✅ Lines 171-179 (tech conflict resolution) | ❌ VIOLATES |
| Error recovery | ✅ Lines 387-440 (fallback strategies) | ❌ VIOLATES |

**Business Logic in Command (Should be in Skill):**

1. **Phase 1: Epic Discovery** (Lines 17-40)
   - Find existing epics via Glob
   - Generate next epic ID
   - Check for duplicate names via Grep
   - Duplicate resolution via AskUserQuestion

2. **Phase 2: Context Gathering** (Lines 42-86)
   - Epic goal (AskUserQuestion)
   - Timeline/priority/business value (AskUserQuestion)
   - Stakeholders (AskUserQuestion)
   - Success criteria (AskUserQuestion)

3. **Phase 3: Feature Decomposition** (Lines 88-130)
   - Invoke requirements-analyst subagent
   - Review and confirm features
   - Iterate if modifications requested

4. **Phase 4: Technical Assessment** (Lines 132-179)
   - Invoke technical-assessor subagent
   - Check context files
   - Validate against tech-stack.md
   - Resolve technology conflicts

5. **Phase 5: Epic File Creation** (Lines 181-293)
   - Generate epic document (YAML + markdown)
   - Write to devforgeai/specs/Epics/{EPIC-ID}.epic.md

6. **Phase 6: Requirements Spec (Optional)** (Lines 295-348)
   - AskUserQuestion for requirements spec
   - Invoke requirements-writer subagent
   - Write to devforgeai/specs/requirements/
   - Update epic with requirements link

7. **Phase 7: Success Report** (Lines 350-384)
   - Display epic summary
   - List features and technical assessment
   - Show files created
   - Provide next steps

**Token Impact:**
- Current: ~10,000 tokens in main conversation
- After refactoring: ~2,000 tokens in main conversation
- **Savings: 80% reduction**

---

## Target State Architecture

### Lean /create-epic Command (~250 lines, ~8K chars - 53% budget)

```markdown
---
description: Create epic with feature breakdown
argument-hint: [epic-name]
model: opus
allowed-tools: AskUserQuestion, Skill
---

# Create Epic Command

Creates a new epic by delegating to devforgeai-orchestration skill.

## Arguments

- `epic-name` (required): Name of the epic (e.g., "User Authentication System")

## Workflow

### Phase 0: Argument Validation (15 lines)

epic_name="$1"

if [ -z "$epic_name" ]; then
  echo "❌ Error: Epic name required"
  echo ""
  echo "Usage: /create-epic [epic-name]"
  echo "Example: /create-epic User Authentication System"
  exit 1
fi

Validate epic name:
- Minimum 10 characters
- Maximum 100 characters
- Alphanumeric, spaces, hyphens, underscores only

If invalid:
  Display error with examples
  HALT

### Phase 1: Set Context Markers (10 lines)

Provide explicit context for orchestration skill:

**Epic name:** $epic_name
**Command:** create-epic
**Mode:** interactive

### Phase 2: Invoke Skill (10 lines)

Skill(command="devforgeai-orchestration")

The orchestration skill will:
1. Detect "create-epic" mode from context markers
2. Execute 7-phase epic creation workflow
3. Invoke subagents (requirements-analyst, technical-assessor)
4. Generate epic document and optional requirements spec
5. Return completion summary

### Phase 3: Display Results (15 lines)

[Skill returns structured summary - display directly]

✅ Epic Created Successfully

Epic Details:
  📋 ID: {EPIC-ID}
  🎯 Title: {epic-name}
  🏆 Priority: {priority}
  📊 Business Value: {business value}
  📅 Timeline: {timeline estimate}

Features: {count} features identified
  ✨ {Feature 1 name} - {complexity}
  ✨ {Feature 2 name} - {complexity}
  [... from skill output ...]

Technical Assessment:
  🔧 Complexity Score: {score}/10
  ⚠️ Key Risks: {count} identified
  📦 Prerequisites: {count}

Files Created:
  📁 devforgeai/specs/Epics/{EPIC-ID}.epic.md
  [If requirements created]
  📁 devforgeai/specs/requirements/{EPIC-ID}-requirements.md

### Phase 4: Next Steps Guidance (10 lines)

Next Steps:
  1. Review epic document: devforgeai/specs/Epics/{EPIC-ID}.epic.md
  2. Create sprint: /create-sprint {sprint-number}
  3. Break features into stories during sprint planning
  4. Implement stories: /dev {STORY-ID}

---

## Error Handling (20 lines)

### Error: Invalid Epic Name
- Validation in Phase 0
- Clear error message with requirements
- Examples provided

### Error: Skill Invocation Failed
- Display skill error message
- Suggest recovery steps
- Do NOT attempt fallback logic (let skill handle)

### Error: Directory Structure Missing
- Skill will create devforgeai/specs/Epics/ directory
- Command does not handle directory creation

---

## Success Criteria

- [x] Epic name validated
- [x] Skill invoked with proper context
- [x] Results displayed from skill output
- [x] Next steps guidance provided
- [x] Character budget < 8,000 (target: 53% of limit)
- [x] Token usage < 2,000 in main conversation

---

## Integration

**Invokes:**
- devforgeai-orchestration skill (--create-epic mode)

**Prerequisites:**
- None (can create epics before context files)

**Enables:**
- /create-sprint command
- Epic → Sprint → Story workflow
```

**Estimated Metrics:**
- Lines: ~250 (down from 526)
- Characters: ~8,000 (down from 14,309)
- Token usage: ~2,000 (down from ~10,000)
- **Reduction: 52% lines, 44% characters, 80% tokens**

---

## Enhanced devforgeai-orchestration Skill

### Current State

**Skill already declares epic creation:**
- Line 50: `Skill(command="devforgeai-orchestration --create-epic --title='User Management'")`
- Lines 234-244: Epic Creation section (placeholder)
- References: `references/epic-management.md` (496 lines - EXISTS)
- Template: `assets/templates/epic-template.md` (265 lines - EXISTS)

**Implementation gap:** Epic creation workflow exists in command but NOT in skill.

### Enhancement Strategy

**Add epic creation implementation to orchestration skill:**

```markdown
## Phase 4: Epic and Sprint Management (ENHANCED)

### Epic Creation Mode

**Entry point:** Detect "create-epic" in conversation context

**Workflow (7 phases):**

1. **Phase 1: Epic Discovery**
   - Load epic-management.md reference (progressive disclosure)
   - Find existing epics via Glob
   - Generate next epic ID (EPIC-001, EPIC-002, etc.)
   - Check for duplicate names via Grep
   - Resolve duplicates via AskUserQuestion

2. **Phase 2: Epic Context Gathering**
   - Epic goal (AskUserQuestion - 6 options)
   - Timeline/priority/business value (AskUserQuestion)
   - Stakeholders (AskUserQuestion - free-form)
   - Success criteria (AskUserQuestion - measurable outcomes)

3. **Phase 3: Feature Decomposition**
   - Invoke requirements-analyst subagent with epic context
   - Generate 3-8 features (functional units)
   - Review with user via AskUserQuestion
   - Iterate if modifications requested

4. **Phase 4: Technical Assessment**
   - Check if context files exist (devforgeai/context/*.md)
   - Invoke technical-assessor subagent OR architect-reviewer subagent
   - If context files exist: Validate against tech-stack.md
   - Resolve conflicts via AskUserQuestion
   - Generate complexity score (0-10), risks, prerequisites

5. **Phase 5: Epic File Creation**
   - Load epic-template.md (265 lines)
   - Populate template with gathered data
   - Write to devforgeai/specs/Epics/{EPIC-ID}.epic.md
   - Validate file created successfully

6. **Phase 6: Requirements Specification (Optional)**
   - AskUserQuestion: Create detailed requirements spec?
   - If yes: Invoke requirements-analyst subagent
   - Generate comprehensive requirements document
   - Write to devforgeai/specs/requirements/{EPIC-ID}-requirements.md
   - Update epic file with requirements link

7. **Phase 7: Completion Summary**
   - Generate structured summary (for command to display)
   - Include: Epic ID, features, technical assessment, files created
   - Provide next steps guidance
   - Return to command for display

**For detailed implementation:** See references/epic-management.md (496 lines)
**Template:** assets/templates/epic-template.md (265 lines)

**Subagents used:**
- requirements-analyst (Phase 3, Phase 6 optional)
- technical-assessor OR architect-reviewer (Phase 4)
```

**Estimated skill enhancement:**
- Add ~300 lines to orchestration skill (currently ~600 lines total)
- New total: ~900 lines
- All in isolated skill context (minimal main conversation impact)

---

## Subagent Analysis

### Existing Subagents Used

**requirements-analyst** (473 lines)
- **Purpose:** User story creation, acceptance criteria, epic feature decomposition
- **Used in:** Phase 3 (feature decomposition), Phase 6 (requirements spec)
- **Status:** ✅ EXISTS - Framework-aware, references context files
- **Enhancement needed:** ❌ NONE - Already handles epic feature decomposition

**technical-assessor** (NOT A SUBAGENT!)
- **Current:** Called as subagent in /create-epic command (lines 137-158)
- **Status:** ❌ DOES NOT EXIST as subagent
- **Alternative:** Use architect-reviewer subagent (528 lines - EXISTS)
- **Action:** Replace technical-assessor with architect-reviewer

**architect-reviewer** (528 lines)
- **Purpose:** Architecture validation, design patterns, complexity assessment
- **Capabilities:** Technology stack assessment, architecture impact, risk identification
- **Status:** ✅ EXISTS - Framework-aware
- **Enhancement needed:** ❌ NONE - Already handles technical assessment

### Missing Subagent: epic-result-interpreter

**Question:** Do we need specialized display interpreter like qa-result-interpreter?

**Analysis:**
- /qa uses qa-result-interpreter (300 lines) to parse complex QA reports into user-friendly display
- Epic creation output is simpler (YAML frontmatter + features list)
- Command can display skill output directly without parsing

**Decision:** ❌ NOT NEEDED
- Epic output is structured and simple
- No complex parsing required
- Command displays skill summary directly

### Verdict: No New Subagents Needed

**Existing subagents sufficient:**
- ✅ requirements-analyst (feature decomposition, requirements spec)
- ✅ architect-reviewer (technical assessment - replaces non-existent technical-assessor)

**Framework-aware integration:**
- Both subagents reference context files
- Both understand DevForgeAI constraints
- Both operate in isolated contexts (token efficient)

---

## Reference Files Analysis

### Existing Reference Files (devforgeai-orchestration skill)

**✅ epic-management.md (496 lines)**
- Epic planning procedures
- Feature decomposition patterns
- Estimation methodologies
- Status: EXISTS - ready for use

**✅ epic-template.md (265 lines)**
- Epic document structure (YAML + markdown)
- All required sections
- Status: EXISTS - ready for use

**✅ sprint-planning.md (620 lines)**
- Sprint capacity calculation
- Story selection
- Status: EXISTS

**✅ story-management.md (691 lines)**
- Story structure, status updates
- Status: EXISTS

### Missing Reference Files

**❌ feature-decomposition-patterns.md**
- **Need:** Specific patterns for epic → feature breakdown
- **Content:** User management features, e-commerce features, reporting features, API features
- **Size:** ~400 lines
- **Action:** CREATE via agent-generator

**❌ technical-assessment-guide.md**
- **Need:** Technical complexity scoring methodology
- **Content:** Complexity scoring rubric (0-10), risk identification patterns, prerequisite templates
- **Size:** ~300 lines
- **Action:** CREATE via agent-generator

**❌ epic-validation-checklist.md**
- **Need:** Self-validation before completion (like story-creation skill has validation-checklists.md)
- **Content:** Epic quality checks, feature validation, technical assessment validation
- **Size:** ~200 lines
- **Action:** CREATE via agent-generator

### Action Items for Reference Files

1. ✅ **Use existing:** epic-management.md, epic-template.md
2. 🔴 **Create new:** feature-decomposition-patterns.md (400 lines)
3. 🔴 **Create new:** technical-assessment-guide.md (300 lines)
4. 🔴 **Create new:** epic-validation-checklist.md (200 lines)

**Total new reference content:** ~900 lines
**Progressive disclosure:** Load only when needed (Phase 3, 4, 7)

---

## Refactoring Implementation Plan

### Step 1: Create Missing Reference Files (via agent-generator)

**Invoke agent-generator to create:**

1. **feature-decomposition-patterns.md**
   - Purpose: Guide requirements-analyst subagent for epic feature breakdown
   - Content: Patterns for common epic types (CRUD, auth, API, reporting, workflow, dashboard)
   - Framework constraints: Reference tech-stack.md, architecture-constraints.md
   - Size: ~400 lines

2. **technical-assessment-guide.md**
   - Purpose: Guide architect-reviewer subagent for technical complexity scoring
   - Content: Complexity rubric (0-10 scale), risk categories, prerequisite templates
   - Framework constraints: Must validate against context files if they exist
   - Size: ~300 lines

3. **epic-validation-checklist.md**
   - Purpose: Self-validation before epic creation completes
   - Content: Epic quality checks, feature count (3-8), stakeholder validation, success criteria measurability
   - Framework constraints: Ensure all required sections populated
   - Size: ~200 lines

**Agent-generator prompt:**
```
Create 3 reference files for devforgeai-orchestration skill to support epic creation workflow.

Context: The /create-epic command is being refactored to use lean orchestration pattern. Business logic is moving to devforgeai-orchestration skill. The skill already has epic-management.md (496 lines) and epic-template.md (265 lines) but needs additional reference files for:

1. Feature decomposition patterns (guide requirements-analyst subagent)
2. Technical assessment methodology (guide architect-reviewer subagent)
3. Epic validation checklist (self-validation before completion)

Requirements:
- Framework-aware (reference context files: tech-stack.md, architecture-constraints.md, anti-patterns.md)
- Prevent silos (understand DevForgeAI Spec-Driven Framework integration)
- AI-optimized (structured markdown, clear sections, examples)
- Progressive disclosure (loaded only when needed by skill phases)

Output location: .claude/skills/devforgeai-orchestration/references/

Target sizes: 400 lines, 300 lines, 200 lines respectively
```

### Step 2: Enhance devforgeai-orchestration Skill

**Add epic creation implementation:**

1. Update SKILL.md (Phase 4: Epic and Sprint Management)
2. Add epic creation workflow (7 phases)
3. Reference new reference files progressively
4. Use architect-reviewer (NOT technical-assessor)
5. Return structured summary for command display

**Estimated addition:** ~300 lines to skill
**Total skill size:** ~900 lines (all in isolated context)

### Step 3: Refactor /create-epic Command

**Create lean command:**

1. Argument validation (epic name)
2. Set context markers (epic name, mode=create-epic)
3. Invoke skill: `Skill(command="devforgeai-orchestration")`
4. Display results (from skill summary)
5. Next steps guidance

**Target metrics:**
- Lines: ~250 (down from 526)
- Characters: ~8,000 (down from 14,309)
- Compliance: ✅ 53% of budget (down from 95%)

### Step 4: Testing and Validation

**Test scenarios:**

1. **Greenfield epic** (no context files)
   - Should create epic successfully
   - Should work without tech-stack.md validation

2. **Brownfield epic** (context files exist)
   - Should validate against tech-stack.md
   - Should detect technology conflicts
   - Should offer conflict resolution via AskUserQuestion

3. **Duplicate epic name**
   - Should detect via Grep
   - Should offer resolution options
   - Should allow creation with different name

4. **Optional requirements spec**
   - Should ask user via AskUserQuestion
   - Should create spec if requested
   - Should update epic with link

5. **Character budget compliance**
   - Command < 8,000 characters ✅
   - Within 15K hard limit ✅
   - Token usage < 2,000 in main conversation ✅

### Step 5: Documentation Updates

**Update framework documentation:**

1. **CLAUDE.md**
   - Update command count (no change - still 11 commands)
   - Update character budget status (create-epic: HIGH → COMPLIANT)

2. **.claude/memory/commands-reference.md**
   - Update /create-epic command documentation
   - Show lean orchestration pattern
   - Update estimated token usage

3. **.claude/memory/skills-reference.md**
   - Document orchestration skill epic creation mode
   - List reference files (including 3 new ones)

4. **lean-orchestration-pattern.md**
   - Add create-epic as reference implementation
   - Update command budget status table

---

## Success Metrics

### Command Metrics

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Lines | 526 | ~250 | 52% reduction |
| Characters | 14,309 | ~8,000 | 44% reduction |
| Budget usage | 95% | 53% | 42 percentage points |
| Token usage | ~10,000 | ~2,000 | 80% reduction |
| Business logic in command | 350+ lines | 0 lines | 100% removed |
| Skill invocations | 0 | 1 | Pattern compliant |

### Skill Metrics

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| Epic creation workflow | Not implemented | 7 phases implemented | Feature added |
| Lines | ~600 | ~900 | +300 lines |
| Context isolation | N/A | ✅ Yes | Token efficient |
| Reference files | 2 | 5 | +3 new files |

### Framework Metrics

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Commands over budget | 5 | 4 | 1 fewer |
| Commands at warning | 5 | 6 | 1 more (moved from over) |
| Commands compliant | 1 | 2 | 1 more |
| Lean pattern compliance | 4/11 (36%) | 5/11 (45%) | +9 percentage points |

---

## Risk Analysis

### Technical Risks

**Risk 1: Epic creation mode detection**
- **Issue:** Skill must detect "create-epic" from context markers
- **Mitigation:** Use explicit marker: `**Command:** create-epic`
- **Precedent:** /dev uses `**Story ID:**`, /qa uses `**Validation mode:**`

**Risk 2: Subagent context**
- **Issue:** requirements-analyst and architect-reviewer need epic context
- **Mitigation:** Build detailed prompts with all gathered data
- **Precedent:** /create-story does this successfully (Phase 2, 3)

**Risk 3: Reference file loading**
- **Issue:** 5 reference files (2 existing + 3 new) might be too many
- **Mitigation:** Progressive disclosure - load only when phase needs
- **Precedent:** devforgeai-story-creation loads 6 reference files (7,477 lines)

### Process Risks

**Risk 4: User experience disruption**
- **Issue:** Command behavior changes (now delegates to skill)
- **Mitigation:** Output format remains identical
- **Impact:** User sees same epic creation flow, just more efficient

**Risk 5: Testing coverage**
- **Issue:** New code paths need validation
- **Mitigation:** Test 5 scenarios (greenfield, brownfield, duplicate, requirements, budget)
- **Timeline:** ~2 hours testing

---

## Timeline Estimate

### Implementation Phases

**Phase 1: Reference Files Creation (1-2 hours)**
- Invoke agent-generator for 3 new reference files
- Review and validate generated content
- Ensure framework-aware and AI-optimized

**Phase 2: Skill Enhancement (2-3 hours)**
- Add epic creation workflow to orchestration skill
- Test epic creation mode detection
- Validate subagent integration

**Phase 3: Command Refactoring (1 hour)**
- Create lean command following pattern
- Remove all business logic
- Add context markers and skill invocation

**Phase 4: Testing (2 hours)**
- Test 5 scenarios
- Validate character budget compliance
- Measure token usage

**Phase 5: Documentation (1 hour)**
- Update CLAUDE.md, commands-reference.md, skills-reference.md
- Update lean-orchestration-pattern.md

**Total estimated time:** 7-9 hours

---

## Next Actions

### Immediate (Today)

1. ✅ **Invoke agent-generator** to create 3 reference files
2. ⏳ **Review generated reference files** for quality and framework integration
3. ⏳ **Enhance orchestration skill** with epic creation workflow

### Short-term (This week)

4. ⏳ **Refactor /create-epic command** to lean pattern
5. ⏳ **Test refactored implementation** (5 scenarios)
6. ⏳ **Update documentation** (4 files)

### Medium-term (Next sprint)

7. ⏳ **Apply pattern to remaining over-budget commands:**
   - /create-ui (18,908 chars - 126% - URGENT)
   - /release (18,166 chars - 121% - HIGH)
   - /orchestrate (15,012 chars - 100% - MEDIUM)

---

## Conclusion

This refactoring plan restores the DevForgeAI Spec-Driven Framework's skills-first architecture by:

1. **Moving business logic to skill** - Where it belongs per Claude Skills architecture
2. **Creating lean command** - Following proven pattern from /dev, /qa, /ideate, /create-story
3. **Leveraging existing infrastructure** - orchestration skill already has epic capabilities
4. **Adding missing reference files** - Progressive disclosure for framework-aware guidance
5. **Using existing subagents** - requirements-analyst, architect-reviewer (no new subagents needed)
6. **Achieving compliance** - 95% → 53% budget usage (42 percentage point improvement)
7. **Improving token efficiency** - 80% reduction in main conversation tokens

**Pattern proven, infrastructure exists, implementation straightforward.**

**Recommendation:** PROCEED with refactoring following this plan.
