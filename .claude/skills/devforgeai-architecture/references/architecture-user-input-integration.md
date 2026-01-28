---
id: architecture-user-input-integration
title: Architecture Skill - User Input Guidance Integration
version: "1.0"
created: 2025-01-21
updated: 2025-01-21
status: Published
audience: DevForgeAI Development Team
parent_document: user-input-guidance.md
skill: devforgeai-architecture
---

# Architecture Skill - User Input Guidance Integration Reference

**Purpose:** Document how devforgeai-architecture skill integrates user-input-guidance.md with conditional loading logic for greenfield vs brownfield project contexts.

**Context:** This skill creates immutable context files (tech-stack.md, source-tree.md, dependencies.md, coding-standards.md, architecture-constraints.md, anti-patterns.md) that define architectural boundaries and enforce constraints.

---

## Section 1: Conditional Loading Logic

### 1.1 Greenfield vs Brownfield Detection

**Greenfield Mode:** New project with NO existing context files
- **Detection:** `Glob(pattern="devforgeai/specs/context/*.md")` returns 0 files
- **Action:** Load user-input-guidance.md and apply guidance patterns
- **Rationale:** User making initial technology decisions; patterns guide quality questions
- **Token Cost:** ~1,000 tokens (guidance + pattern application)

**Brownfield Mode:** Existing project with ALL 6 context files
- **Detection:** `Glob(pattern="devforgeai/specs/context/*.md")` returns 6 files
- **Action:** Skip guidance, use existing context files as constraints
- **Rationale:** Technologies already locked; existing context defines boundaries
- **Token Cost:** 0 tokens (no guidance loaded)

**Partial Mode:** Unusual state (3-5 context files exist)
- **Detection:** `Glob(pattern="devforgeai/specs/context/*.md")` returns 3-5 files
- **Action:** Load guidance (treat as greenfield) to fill gaps
- **Rationale:** Some context exists but incomplete; guidance helps complete it
- **Token Cost:** ~800-1,000 tokens

### 1.2 Conditional Loading Pseudocode

```python
def phase_1_context_discovery():
    """Phase 1: Project Context Discovery with conditional guidance loading"""

    # Step 0: Detect mode and load guidance conditionally
    context_files = Glob(pattern="devforgeai/specs/context/*.md")
    context_count = len(context_files)

    if context_count == 6:
        # Brownfield mode - all context files exist
        mode = "brownfield"
        skip_guidance = True
        log_info("Brownfield mode detected (6 context files exist). Skipping user-input-guidance.md.")
        # Use existing context files as constraints for decisions

    elif context_count == 0:
        # Greenfield mode - no context files exist
        mode = "greenfield"
        skip_guidance = False
        log_info("Greenfield mode detected (no context files). Loading user-input-guidance.md...")
        guidance_content = Read(file_path=".claude/skills/devforgeai-architecture/references/user-input-guidance.md")

    else:
        # Partial greenfield - some context files exist but not all
        mode = f"partial_greenfield ({context_count}/6 files)"
        skip_guidance = False
        log_warning(f"Partial context detected ({context_count}/6 files exist). Loading user-input-guidance.md to fill gaps...")
        guidance_content = Read(file_path=".claude/skills/devforgeai-architecture/references/user-input-guidance.md")

    # Step 1-N: Execute Phase 1 workflow
    # In greenfield/partial modes, use guidance patterns below
    # In brownfield mode, skip guidance and proceed with standard workflow
```

---

## Section 2: Pattern Mapping for Architecture Questions

### 2.1 Phase 1 Question-to-Pattern Mapping

**This table shows which user-input-guidance patterns apply to each Phase 1 question:**

| Phase | Step | Question | Pattern Name | Pattern # | Guidance Template | Options/Bounds | Rationale |
|-------|------|----------|--------------|-----------|------------------|----------------|-----------|
| **1** | **1** | "What languages/frameworks?" | Open-Ended Discovery | 1 | N/A (free text) | No preset list, any language valid | Unbounded possibilities; user decision |
| **1** | **0** | "Is this greenfield?" | Closed Confirmation | 4 | N/A (yes/no) | Binary decision | Clear state check |
| **1** | **2** | "Architecture style?" | Explicit Classification | 3 | CONST-001 | 4 options: Monolithic / Microservices / Serverless / Hybrid | Well-defined patterns |
| **1** | **3** | "Backend framework?" | Bounded Choice | 2 | CONST-001 | 5-10 frameworks (filtered by language) | Known frameworks per language |
| **1** | **4** | "Database system?" | Bounded Choice | 2 | CONST-001 | 6-8 databases (PostgreSQL, MySQL, MongoDB, etc.) | Known databases per stack |
| **1** | **5** | "Testing framework?" | Explicit Classification | 3 | CONST-001 | 3-5 frameworks (pytest, Jest, JUnit, xUnit, etc.) | Language-specific options |

### 2.2 Pattern Application Logic

**Pattern 1: Open-Ended Discovery**
- **Use for:** Technology inventory (languages, frameworks, tools)
- **Application:** Ask "What languages and frameworks will you use?" with NO preset options
- **Reasoning:** Unbounded possibilities; any answer valid
- **User Input:** Free-text response describing technology choices
- **Result:** Convert response to structured tech-stack entry

**Pattern 2: Bounded Choice**
- **Use for:** Selecting from known options (frameworks, databases, testing)
- **Application:** Present list of options with clear descriptions
- **Reasoning:** Known solutions with trade-offs; guide selection
- **User Input:** Select one option (single-select) or multiple (multi-select)
- **Result:** Lock selected technology in tech-stack.md

**Pattern 3: Explicit Classification**
- **Use for:** Architecture styles, testing frameworks (clear categories)
- **Application:** Present mutually exclusive categories with descriptions
- **Reasoning:** Well-defined patterns with distinct characteristics
- **User Input:** Select one category that best describes their architecture
- **Result:** Document architecture pattern in architecture-constraints.md

**Pattern 4: Closed Confirmation**
- **Use for:** Yes/no decisions (greenfield check, context file detection)
- **Application:** Confirm mode (greenfield vs brownfield)
- **Reasoning:** Binary state check for mode detection
- **User Input:** Yes/No confirmation
- **Result:** Branch workflow accordingly

---

## Section 3: Guidance Pattern Examples

### 3.1 Example: Technology Inventory Question

**Without Guidance (Current Baseline):**
```
Question: "What technologies will you use?"
Answer: [User types anything, unclear what to include]
Result: Ambiguous, may miss important technologies
```

**With Open-Ended Discovery Pattern:**
```
Question: "What languages and frameworks will you use for your backend, frontend, and database?"
Description: "Please list all primary technologies you've chosen or are considering. Include language (e.g., Python, JavaScript, C#), frameworks (e.g., Django, Express, .NET), and database systems (e.g., PostgreSQL, MongoDB)."
Answer: [User provides comprehensive list with context]
Result: Clear, complete technology inventory
```

### 3.2 Example: Architecture Style Selection

**Without Guidance:**
```
Question: "What's your architecture?"
Answer: "Modern" or "Distributed" [ambiguous]
Result: Unclear if monolithic, microservices, or hybrid
```

**With Explicit Classification Pattern:**
```
AskUserQuestion(
    question: "Which architecture style best describes your project?",
    header: "Architecture Pattern",
    options: [
        {label: "Monolithic", description: "Single codebase, tightly coupled modules. Best for: small-to-medium teams, rapid prototyping. Example: e-commerce platform v1.0"},
        {label: "Microservices", description: "Multiple independent services, loosely coupled. Best for: large teams, scaling specific components. Example: Netflix, Uber"},
        {label: "Serverless", description: "Functions as a service, event-driven. Best for: variable workloads, minimal ops. Example: API backends, scheduled tasks"},
        {label: "Hybrid", description: "Mix of approaches (e.g., monolith + microservices). Best for: legacy systems transitioning, complex domains. Example: enterprise apps"}
    ],
    multiSelect: false
)
```

**Result:** User selects specific architecture → Clearly documented in architecture-constraints.md

### 3.3 Example: Framework Selection with Bounded Choice

**Without Guidance:**
```
Question: "What backend framework?"
Answer: [User lists 5 frameworks without clear preference]
Result: Unclear which is primary, confusing constraints
```

**With Bounded Choice Pattern (Filtered by Language):**
```
# First, user selected "Python" for language
# Now present Python-specific frameworks:

AskUserQuestion(
    question: "Which Python backend framework will you use?",
    header: "Backend Framework",
    description: "Select the primary framework for your Python backend. Each has different strengths for API development, database integration, and deployment.",
    options: [
        {label: "Django", description: "Full-featured, batteries-included. Great for: monolithic apps, admin panels, quick development. Community: Very large"},
        {label: "FastAPI", description: "Modern, async-first, excellent for APIs. Great for: microservices, async tasks, high performance. Community: Growing fast"},
        {label: "Flask", description: "Lightweight, flexible. Great for: simple APIs, custom solutions, learning. Community: Large, established"},
        {label: "Pyramid", description: "Flexible, scalable. Great for: complex business logic, enterprise apps. Community: Smaller, dedicated"}
    ],
    multiSelect: false
)
```

**Result:** Bounded options, language-aware → User selects one → Documented in tech-stack.md

---

## Section 4: Conditional Guidance Triggers

### 4.1 When to Load Guidance (Greenfield Mode)

**Load guidance in these scenarios:**
1. ✅ No `devforgeai/specs/context/` files exist (fresh project)
2. ✅ 1-5 context files exist (partial project, gaps to fill)
3. ✅ User explicitly requesting architecture help (`/create-context` command)

**Skip guidance in these scenarios:**
1. ❌ All 6 context files exist and are complete (brownfield project)
2. ❌ User updating/modifying existing context (use existing files as constraints)

### 4.2 Guidance Loading at Phase 1 Step 0

**Add this Step 0 to Phase 1 in SKILL.md:**

```markdown
### Phase 1: Project Context Discovery

#### Step 0: Conditional Guidance Load (NEW - STORY-057)

**Purpose:** Load user-input-guidance.md only in greenfield mode to enhance question quality

**Implementation:**
- Detect mode: Run `Glob(pattern="devforgeai/specs/context/*.md")`
  - 0 files → Greenfield mode (LOAD guidance)
  - 6 files → Brownfield mode (SKIP guidance)
  - 1-5 files → Partial mode (LOAD guidance to fill gaps)
- If loading guidance:
  - Execute: `Read(file_path=".claude/skills/devforgeai-architecture/references/user-input-guidance.md")`
  - Log: "Greenfield mode detected. Loading user-input-guidance.md..."
  - Available for Steps 1-5 pattern references
- If skipping guidance:
  - Log: "Brownfield mode detected (6 context files exist). Skipping user-input-guidance.md."
  - Use existing context files as constraints
  - Proceed to Step 1 with standard workflow

**Rationale:** Token efficiency (avoid unnecessary loading in brownfield), better UX in greenfield (guided questions)

**Token Cost:** ~1,000 tokens in greenfield mode (loaded once), 0 in brownfield
```

---

## Section 5: Reference Deployment

### 5.1 File Locations

**Master File:** `src/claude/skills/devforgeai-ideation/references/user-input-guidance.md`

**Architecture Deployment:**
- Location: `src/claude/skills/devforgeai-architecture/references/user-input-guidance.md`
- Deployment: Copy from master using:
  ```bash
  cp src/claude/skills/devforgeai-ideation/references/user-input-guidance.md \
     src/claude/skills/devforgeai-architecture/references/user-input-guidance.md
  ```

**Operational Folder:**
- Also copied to: `.claude/skills/devforgeai-architecture/references/user-input-guidance.md`

### 5.2 Checksum Validation

**Verify deployment integrity:**
```bash
# All 3 architecture deployments should have identical SHA256
sha256sum src/claude/skills/devforgeai-architecture/references/user-input-guidance.md \
          .claude/skills/devforgeai-architecture/references/user-input-guidance.md

# If hashes match: ✅ Deployment successful
# If hashes differ: ❌ Files out of sync, redeploy
```

### 5.3 Sync Process

**When updating master guidance file:**
1. Update: `src/claude/skills/devforgeai-ideation/references/user-input-guidance.md`
2. Deploy: Copy to all 3 skills (architecture, ui-generator, orchestration)
3. Verify: Check checksums match
4. Commit: Document deployment in commit message

---

## Section 6: Testing Strategy

### 6.1 Unit Tests for Conditional Loading

**Test 1: Greenfield mode loads guidance**
```python
def test_greenfield_loads_guidance():
    """Verify guidance loads when no context files exist"""
    # Setup: Mock Glob to return empty list
    # Execute: Phase 1 Step 0
    # Assert: Read called with guidance file path
    # Assert: Log contains "Greenfield mode"
```

**Test 2: Brownfield mode skips guidance**
```python
def test_brownfield_skips_guidance():
    """Verify guidance skipped when 6 context files exist"""
    # Setup: Mock Glob to return 6 files
    # Execute: Phase 1 Step 0
    # Assert: Read NOT called for guidance
    # Assert: Log contains "Brownfield mode"
```

**Test 3: Partial mode loads guidance**
```python
def test_partial_greenfield_loads_guidance():
    """Verify guidance loads when 3 of 6 context files exist"""
    # Setup: Mock Glob to return 3 files
    # Execute: Phase 1 Step 0
    # Assert: Read called with guidance file path
    # Assert: Log contains "Partial context detected"
```

### 6.2 Pattern Application Tests

**Test 4: Open-Ended pattern applied to tech inventory**
```python
def test_open_ended_discovery_applied_tech_inventory():
    """Verify Open-Ended Discovery pattern used for tech question"""
    # Setup: Greenfield mode, guidance loaded
    # Execute: Phase 1 Step 1 (technology question)
    # Assert: Question matches pattern (no preset options)
    # Assert: User input parsed and stored in tech inventory
```

**Test 5: Bounded Choice pattern applied to framework selection**
```python
def test_bounded_choice_applied_framework():
    """Verify Bounded Choice pattern filters by language"""
    # Setup: User selected "Python" language
    # Execute: Framework selection question
    # Assert: Options include only Python frameworks (Django, FastAPI, Flask)
    # Assert: Options exclude non-Python frameworks (Express, ASP.NET, Rails)
```

### 6.3 Integration Tests

**Test 6: Architecture greenfield + guidance + pattern flow**
```python
def test_architecture_greenfield_full_flow():
    """Full greenfield workflow: detect mode → load guidance → apply patterns"""
    # Setup: No context files, user runs /create-context
    # Execute: Phase 1 complete
    # Assert: Step 0 detected greenfield
    # Assert: Guidance loaded
    # Assert: Step 1 applied Open-Ended pattern
    # Assert: Step 2 applied Classification pattern
    # Assert: All context files created with user responses
```

**Test 7: Backward compatibility - brownfield unchanged**
```python
def test_backward_compat_brownfield_no_guidance():
    """Verify brownfield behavior unchanged (no guidance)"""
    # Setup: All 6 context files exist
    # Execute: Phase 1 (update context)
    # Assert: No guidance loaded
    # Assert: Same output as before STORY-057
    # Assert: All 15 existing architecture tests still pass
```

---

## Section 7: Token Budget Analysis

### 7.1 Greenfield Mode Token Cost

```
Step 0: Conditional detection + file operations
  - Glob(pattern="devforgeai/specs/context/*.md"): ~20 tokens
  - Log messages: ~30 tokens
  - Subtotal: ~50 tokens

Step 0: Guidance loading (if greenfield)
  - Read file (600 lines, ~50KB): ~500-700 tokens
  - Pattern reference lookups: ~100-150 tokens
  - Subtotal: ~600-850 tokens

Steps 1-5: Pattern application (using loaded guidance)
  - Pattern lookups: ~50-100 tokens
  - Question generation: ~100-200 tokens
  - Subtotal: ~150-300 tokens

TOTAL per skill (greenfield): 800-1,200 tokens (~1,000 avg)
```

### 7.2 Brownfield Mode Token Cost

```
Step 0: Conditional detection
  - Glob(pattern="devforgeai/specs/context/*.md"): ~20 tokens
  - Log message (skip): ~20 tokens
  - Subtotal: ~40 tokens

Steps 1-5: Standard workflow (no guidance)
  - Existing behavior unchanged: ~200-300 tokens
  - Subtotal: ~200-300 tokens

TOTAL per skill (brownfield): 240-340 tokens (near zero guidance cost)
```

### 7.3 Cumulative Impact

**Single architecture execution:**
- Greenfield: ~1,000 tokens (guidance + patterns)
- Brownfield: ~300 tokens (no guidance overhead)
- Savings vs monolithic guidance: 200-400 tokens in brownfield

**No cumulative cost across skills:**
- Each skill has isolated context (Skills tool isolates contexts)
- Architecture guidance loads in architecture context only
- UI-generator guidance loads in UI context only
- Orchestration guidance loads in orchestration context only
- Main conversation not charged for skill-specific guidance

---

## Section 8: Skill Integration Checklist

### 8.1 SKILL.md Modifications

- [ ] **Phase 1 Head:** Add note "Step 0: Conditional guidance loading per STORY-057"
- [ ] **Phase 1 Step 0 (NEW):** Add ~15 lines of conditional loading logic
- [ ] **Phase 1 Step 1-5:** Reference user-input-guidance patterns where applicable
- [ ] **Total additions:** ~30 lines to SKILL.md (conditional + pattern references)

### 8.2 Reference File Deployment

- [ ] **Master:** user-input-guidance.md exists in ideation/references/
- [ ] **Architecture Copy:** user-input-guidance.md copied to architecture/references/
- [ ] **Operational:** user-input-guidance.md synced to .claude/skills/devforgeai-architecture/
- [ ] **Checksums:** All 3 copies have identical SHA256 hashes

### 8.3 Testing

- [ ] **Unit Tests:** 5 conditional loading tests (greenfield, brownfield, partial, pattern application x2)
- [ ] **Integration Tests:** 2 full-flow tests (greenfield + brownfield unchanged)
- [ ] **Regression Tests:** 15 existing architecture tests all passing
- [ ] **Total:** 22 architecture-specific tests (5 new + 2 new integration + 15 existing)

### 8.4 Documentation

- [ ] **This Reference File:** Created and reviewed
- [ ] **Pattern Mapping Table:** Completed (Section 2.1)
- [ ] **Examples:** 3 before/after examples documented
- [ ] **Deployment Process:** Step-by-step instructions provided

---

## Section 9: Success Validation

**Greenfield mode validation:**
✅ No context files exist
✅ Glob detects 0 files
✅ guidance_content = Read(...) executes
✅ Log: "Greenfield mode detected"
✅ Phase 1 Steps 1-5 use patterns from guidance
✅ Context files created with guided questions

**Brownfield mode validation:**
✅ All 6 context files exist
✅ Glob detects 6 files
✅ Read NOT executed
✅ Log: "Brownfield mode detected"
✅ Phase 1 uses existing context as constraints
✅ No additional user questions (uses existing decisions)

**Backward compatibility validation:**
✅ All 15 existing architecture tests pass
✅ Brownfield output identical to pre-STORY-057
✅ No breaking changes to SKILL interface
✅ Guidance is non-blocking (if file missing, continues with fallback)

---

**Version 1.0** | **Status: Published** | **Created: 2025-01-21**
