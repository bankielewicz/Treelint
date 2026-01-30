# Story Creation Skill Integration Guide

How this skill integrates with other DevForgeAI skills and commands.

## Overview

The devforgeai-story-creation skill is a core framework component invoked by multiple skills and commands for user story generation.

---

## Integration Points

### Invoked By

**devforgeai-orchestration skill**
- Epic decomposition → story creation (Phase 4B)
- Sprint planning → story generation (Phase 3)
- Manages story lifecycle through workflow states

**devforgeai-development skill**
- Deferred DoD items → tracking story creation (Phase 5)
- Scope changes → follow-up story creation
- Creates stories to document technical debt

**/create-story command**
- User-initiated story creation
- Standalone feature description input
- Delegates to this skill for all story generation

---

### Invokes

**requirements-analyst subagent** (Phase 2)
- User story generation (As a/I want/So that format)
- Acceptance criteria creation (Given/When/Then)
- Edge cases identification
- NFR specification

**api-designer subagent** (Phase 3, conditional)
- API contract design (OpenAPI 3.0 style)
- Endpoint specification (method, path, schemas)
- Request/response schema definition
- Invoked only if API endpoints detected in feature description

---

### Provides Output To

**devforgeai-ui-generator skill**
- Input: Story acceptance criteria → UI requirements extraction
- Input: Feature description → Component specification
- Flow: Story creation → UI generation (optional)

**devforgeai-development skill**
- Input: Acceptance criteria → Test generation (TDD Red phase)
- Input: Technical specification → Implementation guidance
- Input: API contracts → API implementation
- Input: Data models → Entity/repository creation
- Flow: Story creation → Development (TDD)

**devforgeai-qa skill**
- Input: Acceptance criteria → Validation targets
- Input: Technical specification → Compliance checks
- Input: NFRs → Performance/security validation
- Flow: Development → QA validation

---

## Data Flow

### Primary Flow

```
Feature Description (input)
  ↓
Story Creation Workflow (8 phases)
  ↓
Complete Story Document (output)
  ↓
Development → QA → Release
```

### Epic Decomposition Flow

```
Epic Features (input from orchestration)
  ↓
For each feature:
  Story Creation Workflow
  ↓
Multiple Stories (output)
  ↓
Sprint Planning
```

### Deferred Work Flow

```
Dev Complete (with deferrals)
  ↓
Development skill identifies deferred DoD items
  ↓
Story Creation Workflow (tracking story)
  ↓
Follow-up Story (output)
  ↓
Added to Backlog
```

---

## Context Requirements

### Required

**Mandatory inputs:**
- Feature description (from user, command, or orchestration)

**No other prerequisites:**
- Can create first story (generates STORY-001)
- Works in greenfield (before context files)
- No epic/sprint needed (can be standalone)

### Optional

**Enhances quality when present:**
- Epic context (epic ID, name) - enables linking
- Sprint context (sprint ID, name) - enables assignment
- Priority (defaults to "Medium")
- Points (defaults to 5)
- Context files (tech-stack.md, etc.) - enables framework validation

### Not Required

**Story creation works without:**
- Context files (can create stories before architecture phase)
- Existing stories (first story generates STORY-001)
- Epic/sprint association (stories can be standalone)
- Pre-defined metadata (defaults provided)

---

## Reusability

This skill is invoked by **4+ framework components:**

1. **/create-story command** (user-initiated)
   - Direct user request for story generation
   - Standalone feature description

2. **devforgeai-orchestration skill** (epic/sprint decomposition)
   - Epic features → multiple stories
   - Sprint planning → story selection

3. **devforgeai-development skill** (deferred work tracking)
   - Deferred DoD items → follow-up stories
   - Scope changes → new story creation

4. **Manual invocation** (explicit skill call)
   - `Skill(command="devforgeai-story-creation")`
   - For advanced workflows or custom automation

---

## Framework Awareness

### Context File Integration

Story creation respects all 6 context files **when present**:

**tech-stack.md**
- Technology references in technical specification
- Validates proposed technologies match approved stack
- Used by api-designer subagent
- **Validated in:** Step 3.6, Step 7.7

**coding-standards.md**
- Code pattern references in technical spec
- Naming conventions for entities, endpoints
- Coverage thresholds by architectural layer (95%/85%/80%)
- **Validated in:** Step 7.7

**architecture-constraints.md**
- Layer boundary awareness in technical spec
- Ensures proposed design respects constraints
- Validates API placement, data access patterns
- **Validated in:** Step 3.6, Step 7.7

**anti-patterns.md**
- Forbidden pattern avoidance in specifications
- Alerts if proposed design matches anti-patterns
- Prevents technical debt at specification stage
- **Validated in:** Step 3.6, Step 7.7

**dependencies.md**
- Package references in Dependencies section
- Validates proposed dependencies are approved
- Flags unapproved packages for user decision
- **Validated in:** Step 3.6, Step 7.7

**source-tree.md**
- File location awareness for implementation notes
- Suggests correct directories for code placement
- **CRITICAL:** Validates story output directory
- **Validated in:** Step 5.0, Step 7.7

### Context Validation Integration Points

**Reference:** `.claude/skills/devforgeai-story-creation/references/context-validation.md`

| Phase | Step | Validation Performed |
|-------|------|---------------------|
| 3 | 3.6 | Tech stack, dependencies, architecture, anti-patterns |
| 5 | 5.0 | Output directory validation against source-tree.md |
| 7 | 7.7 | Comprehensive validation of all 6 context files |

**Blocking Behavior:**
- CRITICAL/HIGH violations block story completion
- User prompted via AskUserQuestion for resolution
- Options: Fix in story, Update context file (ADR required), Defer

**Greenfield Mode:**
- When `devforgeai/specs/context/` is empty
- Validation steps are skipped
- Story creation proceeds without constraints
- Recommend: Run `/create-context` to enable validation

### Operating Modes

**Greenfield mode:**
- Context files optional (before /create-context)
- Story creation proceeds without framework validation
- Generates stories that can be validated later

**Brownfield mode:**
- Context files exist and respected
- Framework validation enabled
- Specifications reference approved technologies
- Architecture constraints applied

---

## Skill Invocation Pattern

### From Commands

```
# /create-story command sets context markers
**Feature Description:** User login with email and password

# Then invokes skill
Skill(command="devforgeai-story-creation")

# Skill extracts feature description from conversation
# Executes 8-phase workflow
# Returns completion summary
```

### From Skills

```
# devforgeai-orchestration sets context
**Feature Description:** {epic_feature_description}
**Epic ID:** EPIC-001
**Story Index:** 1 of 5

# Invokes story creation
Skill(command="devforgeai-story-creation")

# Story creation extracts context
# Associates story with epic automatically
```

---

## Output Files

### Primary Output

**Story file:**
- Location: `devforgeai/specs/Stories/{STORY-ID}-{slug}.story.md`
- Format: YAML frontmatter + Markdown sections
- Size: ~200-500 lines (varies by complexity)

**Sections generated:**
1. User Story
2. Acceptance Criteria (3+ criteria)
3. Technical Specification (API, data models, rules, dependencies)
4. UI Specification (if applicable - components, mockups, accessibility)
5. Non-Functional Requirements (performance, security, usability, scalability)
6. Edge Cases & Error Handling
7. Definition of Done (implementation, quality, testing, documentation checklists)
8. Workflow History (creation timestamp)

### Secondary Outputs

**Epic file updates (if associated):**
- Adds story to "## Stories" section
- Updates story count

**Sprint file updates (if associated):**
- Adds story to "## Sprint Backlog" section
- Updates capacity (total points)

---

## Token Efficiency

**Isolated context usage:**
- Story creation executes in skill context (~90K tokens typical)
- Main conversation sees only summary (~3K tokens)
- Progressive loading of references (only needed phases)

**Reference file loading:**
- Phase 1: story-discovery.md (~2.4K tokens)
- Phase 2: requirements-analysis.md + acceptance-criteria-patterns.md (~11K tokens)
- Phase 3: technical-specification-creation.md + technical-specification-guide.md (~12K tokens)
- Phase 4: ui-specification-creation.md + ui-specification-guide.md (~13K tokens)
- Phase 5: story-file-creation.md + story-structure-guide.md (~8K tokens)
- Phase 6: epic-sprint-linking.md (~1K tokens)
- Phase 7: story-validation-workflow.md + validation-checklists.md (~10K tokens)
- Phase 8: completion-report.md (~1.2K tokens)

**Total typical:** ~60K tokens in references (loaded progressively, not all at once)

**Efficiency:** Main conversation 97% efficient (3K consumed, 87K in isolated skill context)

---

## Best Practices for Integration

### From Commands

1. **Set clear context markers** - Use `**Feature Description:**` format
2. **Load story files via @file** - If updating existing story
3. **Provide epic/sprint context** - If known (enables auto-linking)
4. **Display skill output** - Don't parse, just show completion summary

### From Skills

1. **Provide feature description** - Minimum 10 words, clear WHO/WHAT
2. **Set epic context if decomposing** - Enables automatic linking
3. **Invoke without parameters** - Skills don't accept CLI-style params
4. **Use skill output** - Returned summary has story_id, file_path

### Manual Invocation

1. **Ensure conversation context has feature description**
2. **Optionally specify epic/sprint via explicit statements**
3. **Invoke: `Skill(command="devforgeai-story-creation")`**
4. **Skill will ask for missing context via AskUserQuestion**

---

## Testing Integration

**Integration test scenarios:**

1. **Command → Skill:** `/create-story "User login"` → Story created
2. **Orchestration → Skill:** Epic decomposition → 5 stories created
3. **Development → Skill:** Deferred DoD → Follow-up story created
4. **Manual → Skill:** Direct invocation → Story created

**Validation:**
- [ ] All 4 invocation paths work
- [ ] Context extraction successful
- [ ] Epic/sprint linking works
- [ ] Output format consistent

---

## Related Documentation

**Skill documentation:**
- `.claude/skills/devforgeai-story-creation/SKILL.md` - Entry point

**Reference files:**
- 8 phase workflow files (story-discovery.md through completion-report.md)
- 6 supporting guides (acceptance-criteria-patterns.md, story-examples.md, etc.)
- error-handling.md (this file's sibling)

**Framework documentation:**
- `.claude/memory/skills-reference.md` - All skills overview
- `.claude/memory/commands-reference.md` - Command integration
- `CLAUDE.md` - Framework overview
