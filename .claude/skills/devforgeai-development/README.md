# DevForgeAI Development Skill

**Purpose:** Implement features using Test-Driven Development (TDD) while enforcing architectural constraints to prevent technical debt.

## Overview

The `devforgeai-development` skill transforms user stories and feature specifications into tested, production-ready code that complies with architectural decisions documented in context files.

### Key Capabilities

1. **Context-Driven Development**
   - Validates tech-stack.md, source-tree.md, dependencies.md before coding
   - Auto-invokes devforgeai-architecture skill if context files missing
   - Prevents technical debt from ambiguous assumptions

2. **Test-Driven Development Workflow**
   - Red → Green → Refactor cycle
   - Tests written BEFORE implementation
   - Multiple test levels: Unit, Integration, Contract, E2E

3. **Ambiguity Resolution**
   - Uses AskUserQuestion for ALL unclear decisions
   - Technology choices (libraries, frameworks, databases)
   - Architecture patterns (Repository, DTO, Service patterns)
   - File locations and structure decisions

4. **Native Tool Efficiency**
   - Uses Read/Edit/Write/Glob/Grep for file operations (40-73% token savings)
   - Reserves Bash for terminal operations only (git, npm, pytest, etc.)
   - Achieves <80,000 token target per feature implementation

5. **Anti-Pattern Prevention**
   - Validates against anti-patterns.md during implementation
   - Prevents library substitution (e.g., Dapper → EF Core)
   - Prevents structure violations (wrong file locations)
   - Prevents cross-layer dependencies (Domain → Infrastructure)

## When to Use

Activate this skill when:
- Implementing user stories or features
- Writing new code for existing projects
- Refactoring code while maintaining specs
- Converting requirements into tested code
- Ensuring code complies with architectural decisions

## Workflow Phases

### Phase 0: Context Validation
- Check for all 6 context files (tech-stack.md, source-tree.md, etc.)
- Auto-invoke devforgeai-architecture if missing
- Load story/spec documentation
- Validate spec against context for conflicts

### Phase 1: Test-First Design (Red)
- Analyze acceptance criteria
- Design test cases at appropriate levels
- Determine test file locations (consult source-tree.md)
- Write failing tests following TDD patterns
- Run tests → verify RED (fail for right reason)

### Phase 2: Implementation (Green)
- Determine implementation file location (consult source-tree.md)
- Validate dependencies (consult dependencies.md)
- Implement following coding-standards.md
- Validate architecture-constraints.md (layer boundaries)
- Use native tools for file operations
- Write minimal code to pass tests
- Run tests → verify GREEN

### Phase 3: Refactor
- Check anti-patterns.md for violations
- Apply SOLID principles
- Improve code quality (extract methods, rename, DRY)
- Run tests → verify still GREEN

### Phase 4: Integration & Validation
- Run full test suite with coverage
- Static analysis (linters)
- Build validation
- Update documentation (API, schema changes)

### Phase 5: Git Workflow
- Review changes (git status, git diff)
- Stage and commit with proper messages
- Push to remote

## Integration with Other Skills

### devforgeai-architecture
**When invoked:** Context files missing or need updates

**Interaction:**
```
if context_files_missing:
    Skill(command="devforgeai-architecture")
    # Wait for completion, then reload context
```

### devforgeai-qa (Future)
**When invoked:** Implementation complete, ready for QA

**Handoff:** Tests passing, coverage met, no anti-patterns, build succeeds

### devforgeai-release (Future)
**When invoked:** QA approved, ready for release

**Handoff:** All validations passed, documentation updated, commits created

## Reference Materials

### TDD Patterns (`references/tdd-patterns.md`)
Comprehensive guide to Test-Driven Development including:
- Red → Green → Refactor cycle patterns
- Test structure patterns (AAA, Given-When-Then)
- Test types and when to use each
- Mocking patterns
- Test data builders
- Edge case testing
- Code coverage guidance
- TDD anti-patterns to avoid
- Complete worked examples

**Load when:** Designing tests, implementing TDD workflow

## Tool Usage Protocol

**MANDATORY: Native tools for file operations**

### File Operations (ALWAYS Native Tools)
- **Reading:** `Read` tool, NOT `cat`/`head`/`tail`
- **Searching:** `Grep` tool, NOT `grep`/`rg`
- **Finding:** `Glob` tool, NOT `find`/`ls`
- **Editing:** `Edit` tool, NOT `sed`/`awk`
- **Creating:** `Write` tool, NOT `echo >`/`cat <<EOF`

**Why:** 40-73% token savings per operation (documented in native-tools-vs-bash-efficiency-analysis.md)

### Terminal Operations (Use Bash)
- **Version control:** `git status`, `git commit`, `git push`
- **Package management:** `npm install`, `pip install`, `dotnet add package`
- **Test execution:** `pytest`, `npm test`, `dotnet test`
- **Build:** `dotnet build`, `npm run build`, `mvn package`

## Ambiguity Resolution Patterns

### Technology Choice Ambiguous
**Trigger:** Implementation needs functionality not in tech-stack.md

**Response:** Use AskUserQuestion
```
Question: "Spec requires [feature], but tech-stack.md doesn't specify technology. Which should be used?"
Header: "Tech choice"
Options: [2-4 technology options]
multiSelect: false
```

### Pattern Not Specified
**Trigger:** Implementation needs pattern not in coding-standards.md

**Response:** Use AskUserQuestion
```
Question: "Implementation needs [pattern], but coding-standards.md doesn't specify. Which approach?"
Header: "Pattern"
Options: [2-4 pattern choices]
multiSelect: false
```

### File Location Unclear
**Trigger:** New file type not in source-tree.md

**Response:** Use AskUserQuestion
```
Question: "Where should [ComponentType] be placed? source-tree.md doesn't specify."
Header: "Location"
Options: [2-4 location choices]
multiSelect: false
```

### Conflicting Requirements
**Trigger:** Spec requirement conflicts with context files

**Response:** Use AskUserQuestion
```
Question: "Spec says [X], but tech-stack.md specifies [Y]. Which is correct?"
Header: "Conflict"
Options:
  - "Use spec [X] (update tech-stack.md)"
  - "Use context [Y] (update spec)"
  - "Other approach"
multiSelect: false
```

## Success Criteria

Implementation succeeds when:

- [x] Context files validated before development
- [x] All ambiguities resolved via AskUserQuestion
- [x] Tests written BEFORE implementation (TDD)
- [x] Implementation follows ALL context constraints
- [x] No anti-patterns introduced
- [x] All tests pass (new and existing)
- [x] Code coverage meets requirements (typically 80%+)
- [x] Build succeeds
- [x] Native tools used for file operations
- [x] Documentation updated
- [x] Git commits created

## Efficiency Metrics

**Token Budget:** <80,000 tokens per feature

**Breakdown:**
- Context validation: ~5,000 tokens
- Test design & writing: ~15,000 tokens
- Implementation: ~30,000 tokens
- Refactoring & validation: ~20,000 tokens
- Documentation: ~10,000 tokens

**Achieved through:**
- Native tool usage (40-73% savings vs Bash)
- Efficient file reading (Read vs cat)
- Structured searching (Grep vs grep)
- Progressive disclosure (load context only when needed)

## Example Usage

### Scenario: Implement order discount calculation

**Input:** Story specifies "Apply coupon discount percentage to order total"

**Workflow:**

1. **Context Validation**
   - Load tech-stack.md → Confirm ORM is Dapper
   - Load source-tree.md → Confirm service location
   - Load dependencies.md → Confirm test framework is xUnit

2. **Test Design**
   - Write test: `CalculateDiscount_ValidCoupon_ReturnsDiscountedPrice`
   - Run → RED (method doesn't exist)

3. **Implementation**
   - Create `OrderService.CalculateDiscount()` method
   - Use Dapper patterns from coding-standards.md
   - Run → GREEN

4. **Refactor**
   - Extract magic numbers to constants
   - Extract helper methods
   - Run → still GREEN

5. **Validation**
   - Full test suite passes
   - Coverage: 95%
   - Build succeeds

6. **Git**
   - Commit: "feat: Implement order discount calculation"
   - Push to remote

**Result:** Tested, compliant feature implemented in <80k tokens

## Tips for Success

1. **Always validate context first** - Prevents rework from wrong assumptions
2. **Use AskUserQuestion liberally** - Better to ask than guess wrong
3. **Write smallest test first** - Build complexity incrementally
4. **Keep tests independent** - Each test sets up its own data
5. **Refactor ruthlessly** - Clean code is maintainable code
6. **Use native tools exclusively** - Massive token savings
7. **Follow the cycle strictly** - Red → Green → Refactor, repeat

## Common Pitfalls to Avoid

❌ **Skipping context validation** → Technical debt from wrong assumptions
❌ **Writing implementation before tests** → Not true TDD
❌ **Using Bash for file operations** → 40-73% token waste
❌ **Assuming technology choices** → May conflict with tech-stack.md
❌ **Violating layer boundaries** → Breaks architecture-constraints.md
❌ **Adding unapproved packages** → Violates dependencies.md
❌ **Creating files in wrong locations** → Violates source-tree.md

## Next Steps

After using this skill, consider:

1. **Invoke devforgeai-qa skill** - Validate implementation quality
2. **Update ADRs** - Document any architecture decisions made
3. **Create PR** - Use devforgeai-release skill for PR creation
4. **Iterate** - Refine based on code review feedback

## Reference Files

The skill uses **progressive disclosure** via reference files loaded on-demand:

### Workflow References (Loaded When Needed)

1. **`references/dod-validation-checkpoint.md`** (487 lines)
   - **When loaded:** Phase 5 Step 1b - Only if story has incomplete DoD items
   - **Purpose:** Mandatory user interaction for all deferrals (Layer 2 validation)
   - **Enforcement:** Blocks git commit until user approval obtained
   - **Invokes:** requirements-analyst, architect-reviewer subagents
   - **Token cost:** ~5,000-10,000 tokens (isolated context)
   - **Progressive:** NOT loaded if all DoD items complete (token optimization)

2. **`references/tdd-patterns.md`** (1,013 lines)
   - **When loaded:** As needed for TDD guidance during Phases 1-4
   - **Purpose:** Red-Green-Refactor patterns, test structure, mocking

3. **`references/refactoring-patterns.md`** (797 lines)
   - **When loaded:** Phase 3 (Refactor) for code improvement techniques
   - **Purpose:** Extract method, remove duplication, improve naming

4. **`references/git-workflow-conventions.md`** (885 lines)
   - **When loaded:** Phase 5 (Git Workflow) for commit conventions
   - **Purpose:** Conventional commits, branch naming, staging strategies

5. **`references/story-documentation-pattern.md`** (792 lines)
   - **When loaded:** Phase 5 Step 1b for Implementation Notes templates
   - **Purpose:** Document decisions, files, tests, acceptance verification

6. **`references/slash-command-argument-validation-pattern.md`** (812 lines)
   - **When loaded:** For slash command integration patterns
   - **Purpose:** Argument handling, error messages, validation

**Total references:** 4,786 lines of detailed guidance
**Progressive loading:** Only loaded when specific phase/step needs them
**Token efficiency:** Prevents loading all guidance upfront (40-73% savings)

## Related Skills

- **devforgeai-architecture** - Creates context files this skill enforces
- **devforgeai-qa** - Validates implementation quality (light/deep modes)
- **devforgeai-release** - Handles deployment to staging/production
- **devforgeai-orchestration** - Coordinates multi-skill workflows
- **devforgeai-story-creation** - Creates stories that this skill implements

## Feedback & Iteration

This skill improves through usage. After using it:

1. Note any friction points or ambiguities
2. Identify missing patterns or guidance
3. Update SKILL.md and references
4. Test changes on real features
5. Iterate based on results

**Goal:** Make spec-driven TDD development as smooth and debt-free as possible.
