# DevForgeAI Development Skill - Integration Guide

## Overview

This guide explains how the `devforgeai-development` skill integrates with your spec-driven development framework.

## Architecture Position

```
Spec-Driven Development Flow:

┌─────────────────────────────────┐
│   devforgeai-ideation (Future)  │
│   Brainstorming & Requirements  │
└─────────────────┬───────────────┘
                  ↓
┌─────────────────────────────────┐
│   devforgeai-architecture ✓     │
│   Context Files & ADRs          │
│   • tech-stack.md               │
│   • source-tree.md              │
│   • dependencies.md             │
│   • coding-standards.md         │
│   • architecture-constraints.md │
│   • anti-patterns.md            │
└─────────────────┬───────────────┘
                  ↓
┌─────────────────────────────────┐
│   devforgeai-development ✓      │ ← YOU ARE HERE
│   TDD Implementation            │
│   • Validates context           │
│   • Writes tests first          │
│   • Implements features         │
│   • Enforces constraints        │
└─────────────────┬───────────────┘
                  ↓
┌─────────────────────────────────┐
│   devforgeai-qa (Future)        │
│   Quality Validation            │
└─────────────────┬───────────────┘
                  ↓
┌─────────────────────────────────┐
│   devforgeai-release (Future)   │
│   Git & Deployment              │
└─────────────────────────────────┘
```

## Integration with devforgeai-architecture

### Automatic Context Validation

The development skill **automatically validates** that architectural context exists before allowing development:

```
Phase 0: Context Validation
├─ Check: devforgeai/specs/context/tech-stack.md
├─ Check: devforgeai/specs/context/source-tree.md
├─ Check: devforgeai/specs/context/dependencies.md
├─ Check: devforgeai/specs/context/coding-standards.md
├─ Check: devforgeai/specs/context/architecture-constraints.md
└─ Check: devforgeai/specs/context/anti-patterns.md

IF ANY MISSING:
    Skill(command="devforgeai-architecture")
    WAIT for completion
    RELOAD context files
    CONTINUE with development
```

### Context Enforcement

The development skill **enforces** architectural decisions throughout implementation:

**tech-stack.md enforcement:**
```
✅ CORRECT: Use Dapper (as specified in tech-stack.md)
❌ FORBIDDEN: Switch to Entity Framework (violates tech-stack.md)

IF ambiguous:
    AskUserQuestion("Which ORM should be used?")
    UPDATE tech-stack.md
    CREATE ADR documenting decision
```

**source-tree.md enforcement:**
```
✅ CORRECT: Place file at src/Application/Services/OrderService.cs
❌ FORBIDDEN: Place file at src/Services/OrderService.cs

IF ambiguous:
    AskUserQuestion("Where should [Component] be placed?")
    FOLLOW source-tree.md rules
```

**dependencies.md enforcement:**
```
✅ CORRECT: Use xUnit 2.5.0 (listed in dependencies.md)
❌ FORBIDDEN: Add NUnit (not in dependencies.md)

IF new package needed:
    AskUserQuestion("Should I add [Package]?")
    UPDATE dependencies.md
    CREATE ADR
```

## Integration with Claude Code (Terminal)

### Native Tool Usage

The development skill follows your `native-tools-vs-bash-efficiency-analysis.md` guidelines:

**File Operations (Native Tools - 40-73% token savings):**
```
Read  → Read files       (NOT cat)
Edit  → Edit files       (NOT sed/awk)
Write → Create files     (NOT echo >/cat <<EOF)
Glob  → Find files       (NOT find/ls)
Grep  → Search content   (NOT grep/rg)
```

**Terminal Operations (Bash):**
```
git status, git commit, git push
npm install, pip install, dotnet add package
pytest, npm test, dotnet test
dotnet build, npm run build
```

### AskUserQuestion Integration

The development skill uses `AskUserQuestion` for ALL ambiguities (as you requested):

**Scenario 1: Technology Choice**
```
Question: "Spec requires caching, but tech-stack.md doesn't specify. Which technology?"
Header: "Caching"
Options:
  - "Redis (distributed)"
  - "Memcached (simple)"
  - "IMemoryCache (built-in)"
multiSelect: false
```

**Scenario 2: Library Choice**
```
Question: "Implementation needs ORM functionality. Use Dapper or EF Core?"
Header: "ORM"
Options:
  - "Dapper 2.1.x (micro-ORM, explicit SQL)"
  - "Entity Framework Core (full ORM)"
multiSelect: false
```

**Scenario 3: RDBMS Choice**
```
Question: "Which database should this project use?"
Header: "Database"
Options:
  - "Microsoft SQL Server"
  - "PostgreSQL"
  - "MySQL"
multiSelect: false
```

**Scenario 4: Language/Framework**
```
Question: "Which backend language should be used?"
Header: "Language"
Options:
  - "C# with .NET 8.0"
  - "Python with FastAPI"
  - "Node.js with TypeScript"
multiSelect: false
```

## Project Management Integration

### Story-Driven Development

As you specified: "Ultimately should be translated to stories for developers"

**Epic → Sprint → Story → Development:**

```
1. EPIC: E-Commerce Checkout Flow
   ├─ Sprint 1: Core Shopping Cart
   │   ├─ Story 1.1: Add items to cart
   │   │   └─ devforgeai-development (implements with TDD)
   │   ├─ Story 1.2: Update quantities
   │   │   └─ devforgeai-development (implements with TDD)
   │   └─ Story 1.3: Remove items
   │       └─ devforgeai-development (implements with TDD)
   │
   └─ Sprint 2: Discount Engine
       ├─ Story 2.1: Apply coupon codes
       │   └─ devforgeai-development (implements with TDD)
       └─ Story 2.2: Calculate discounts
           └─ devforgeai-development (implements with TDD)
```

### Story Structure

**Expected input format:**

```markdown
# Story 1.1: Add Items to Cart

## Description
As a customer, I want to add items to my shopping cart so that I can purchase multiple products.

## Acceptance Criteria
- [ ] User can add product to cart with quantity
- [ ] Cart displays total items count
- [ ] Cart persists across sessions
- [ ] Duplicate products increment quantity

## Technical Spec
- API endpoint: POST /api/cart/items
- Request: { productId: int, quantity: int }
- Response: { cartId: string, itemCount: int, total: decimal }

## Non-Functional Requirements
- Response time: < 200ms
- Concurrent users: 1000+
- Error handling: Result Pattern (per coding-standards.md)
```

**Development skill consumes:**
1. **Description** → Understand feature intent
2. **Acceptance Criteria** → Design test cases
3. **Technical Spec** → Implementation details
4. **NFRs** → Performance/security requirements

## Test-Driven Development Workflow

### Phase Flow

```
Story Input
    ↓
┌─────────────────────────────────────────┐
│ Phase 0: Context Validation             │
│ • Check all 6 context files exist       │
│ • Auto-invoke architecture if missing   │
│ • Load story specification              │
│ • Validate spec vs context              │
└─────────────────┬───────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│ Phase 1: Test-First Design (RED)        │
│ • Analyze acceptance criteria           │
│ • Design test cases (unit/integration)  │
│ • Determine test file location          │
│ • Write failing tests                   │
│ • Run tests → verify RED                │
└─────────────────┬───────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│ Phase 2: Implementation (GREEN)         │
│ • Determine file location               │
│ • Validate dependencies                 │
│ • Implement following standards         │
│ • Use native tools (Read/Edit/Write)    │
│ • Run tests → verify GREEN              │
└─────────────────┬───────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│ Phase 3: Refactor                       │
│ • Check anti-patterns.md                │
│ • Apply SOLID principles                │
│ • Improve code quality                  │
│ • Run tests → still GREEN               │
└─────────────────┬───────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│ Phase 4: Integration & Validation       │
│ • Run full test suite + coverage        │
│ • Static analysis (linters)             │
│ • Build validation                      │
│ • Update documentation                  │
└─────────────────┬───────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│ Phase 5: Git Workflow                   │
│ • Review changes (git status/diff)      │
│ • Stage and commit                      │
│ • Push to remote                        │
└─────────────────────────────────────────┘
                  ↓
    Ready for devforgeai-qa (future)
```

## Token Efficiency Strategy

### Budget Allocation

**Target: <80,000 tokens per story implementation**

```
Phase 0: Context Validation       ~5,000 tokens   (6%)
    - Read 6 context files
    - Read story specification
    - Validate conflicts

Phase 1: Test Design              ~15,000 tokens  (19%)
    - Analyze acceptance criteria
    - Design test cases
    - Write failing tests
    - Run test suite

Phase 2: Implementation            ~30,000 tokens  (38%)
    - Determine locations
    - Read existing code
    - Implement features
    - Run tests

Phase 3: Refactor                  ~20,000 tokens  (25%)
    - Read anti-patterns
    - Refactor code
    - Extract methods
    - Run tests

Phase 4: Validation                ~10,000 tokens  (12%)
    - Full test suite
    - Build validation
    - Update docs

TOTAL:                             ~80,000 tokens  (100%)
```

### Efficiency Techniques

**1. Native Tool Usage (40-73% savings):**
```
❌ Bash: cat src/Services/OrderService.cs      ~2,500 tokens
✅ Native: Read(file_path="src/...")           ~1,500 tokens
SAVINGS:                                       1,000 tokens (40%)
```

**2. Progressive Disclosure:**
```
# Don't load all at once
✅ CORRECT:
    1. Load tech-stack.md (only when checking tech)
    2. Load coding-standards.md (only when implementing)
    3. Load anti-patterns.md (only when refactoring)

❌ WRONG:
    1. Load ALL context files upfront (wastes tokens)
```

**3. Parallel Operations:**
```
✅ CORRECT (parallel):
    Read(file_path="src/OrderService.cs")
    Read(file_path="tests/OrderServiceTests.cs")
    Glob(pattern="src/**/*.cs")
    # All execute simultaneously

❌ WRONG (sequential):
    Read file 1 → wait → Read file 2 → wait → Glob
```

**4. Targeted Searching:**
```
✅ CORRECT:
    Grep(pattern="class OrderService", type="cs", output_mode="files_with_matches")
    # Returns only file paths (~500 tokens)

❌ WRONG:
    Grep(pattern="class OrderService", output_mode="content")
    # Returns full file contents (~5,000 tokens)
```

## Example: Complete Story Implementation

### Story: Implement Order Discount Calculation

**Input:**
```markdown
# Story 2.2: Calculate Order Discounts

## Acceptance Criteria
- Apply coupon discount percentage to order total
- Expired coupons should not apply
- Invalid coupon codes return error

## Technical Spec
- Method: OrderService.CalculateDiscount(Order, Coupon)
- Returns: DiscountResult { DiscountedTotal, DiscountAmount }
- Error handling: Result Pattern
```

**Development Workflow:**

**Phase 0: Context Validation (~5k tokens)**
```
Read(file_path="devforgeai/specs/context/tech-stack.md")
    → ORM: Dapper ✓
Read(file_path="devforgeai/specs/context/coding-standards.md")
    → Error Handling: Result Pattern ✓
Read(file_path="devforgeai/specs/context/source-tree.md")
    → Services: src/Application/Services/ ✓
```

**Phase 1: Test Design (~15k tokens)**
```
# Create test file
Write(file_path="tests/Application.Tests/Services/OrderServiceTests.cs",
      content="[test class with 3 failing tests]")

# Tests:
1. CalculateDiscount_ValidCoupon_ReturnsDiscountedPrice
2. CalculateDiscount_ExpiredCoupon_ReturnsOriginalPrice
3. CalculateDiscount_InvalidCouponCode_ReturnsError

# Run
Bash(command="dotnet test")
    → ❌ RED (tests fail - methods don't exist)
```

**Phase 2: Implementation (~30k tokens)**
```
# Create implementation
Edit(file_path="src/Application/Services/OrderService.cs",
     old_string="[class body]",
     new_string="[class body + CalculateDiscount method]")

# Implementation follows coding-standards.md:
- Result Pattern for error handling
- Dependency injection
- Async/await with ConfigureAwait(false)

# Run
Bash(command="dotnet test")
    → ✅ GREEN (all tests pass)
```

**Phase 3: Refactor (~20k tokens)**
```
# Check anti-patterns
Read(file_path="devforgeai/specs/context/anti-patterns.md")
    → No violations ✓

# Refactor
Edit(file_path="src/Application/Services/OrderService.cs",
     old_string="[magic numbers]",
     new_string="[extracted constants]")

# Run
Bash(command="dotnet test")
    → ✅ GREEN (tests still pass)
```

**Phase 4: Validation (~10k tokens)**
```
# Full test suite
Bash(command="dotnet test --collect:'XPlat Code Coverage'")
    → ✅ All tests pass, 95% coverage

# Build
Bash(command="dotnet build")
    → ✅ Build succeeds

# Update docs (if needed)
Edit(file_path="docs/api/orders.md", ...)
```

**Phase 5: Git (~2k tokens)**
```
Bash(command="git add src/ tests/")
Bash(command="git commit -m 'feat: Implement order discount calculation...'")
Bash(command="git push origin feature/order-discounts")
```

**TOTAL: ~82k tokens** (within budget!)

**Results:**
- ✅ All acceptance criteria met
- ✅ Tests written first (TDD)
- ✅ All context constraints followed
- ✅ No anti-patterns introduced
- ✅ 95% code coverage
- ✅ Build succeeds
- ✅ Ready for QA review

## Next Steps: Building the Full Suite

### Recommended Skill Creation Order

**1. ✅ devforgeai-development (COMPLETED)**
   - Implements features with TDD
   - Enforces architectural constraints
   - Uses AskUserQuestion for ambiguities

**2. devforgeai-qa (NEXT - High Priority)**
   - Validates test coverage
   - Detects anti-patterns
   - Ensures spec compliance
   - Reviews code quality

**3. devforgeai-orchestration (NEXT - High Priority)**
   - Coordinates skill workflows
   - Manages Epic → Sprint → Story flow
   - Auto-invokes skills in sequence
   - Validates state transitions

**4. devforgeai-ideation (Medium Priority)**
   - Brainstorming & requirements
   - User story creation
   - Acceptance criteria definition
   - Feeds into architecture skill

**5. devforgeai-release (Medium Priority)**
   - Git workflow automation
   - PR creation with proper descriptions
   - Deployment validation
   - Release notes generation

### Integration Testing

Before creating more skills, test the development skill:

**Test Case 1: Simple Feature**
- Story: "Add item to cart"
- Expected: Context validated, tests written, implementation follows constraints

**Test Case 2: Missing Context**
- Story: "Calculate shipping costs"
- Context: Missing tech-stack.md
- Expected: Auto-invoke architecture skill, then proceed

**Test Case 3: Ambiguous Requirement**
- Story: "Implement caching"
- Context: tech-stack.md doesn't specify caching
- Expected: AskUserQuestion for technology choice

**Test Case 4: Conflicting Requirement**
- Story: "Use MongoDB"
- Context: tech-stack.md specifies SQL Server
- Expected: AskUserQuestion to resolve conflict

### Feedback Loop

After testing:

1. **Document friction points** - Where did workflow struggle?
2. **Identify missing patterns** - What guidance was unclear?
3. **Update skill** - Refine SKILL.md and references
4. **Re-test** - Verify improvements
5. **Iterate** - Continuous improvement

## Summary

The `devforgeai-development` skill is designed to:

✅ **Enforce architectural decisions** from context files
✅ **Use TDD methodology** (Red → Green → Refactor)
✅ **Resolve ambiguities** via AskUserQuestion
✅ **Achieve token efficiency** through native tools
✅ **Prevent technical debt** via anti-pattern validation
✅ **Integrate seamlessly** with architecture skill
✅ **Support story-driven development** (Epic → Sprint → Story)

**Ready to use:** Invoke with `/devforgeai-development` in Claude Code terminal

**Next action:** Test on real stories, gather feedback, iterate on design
