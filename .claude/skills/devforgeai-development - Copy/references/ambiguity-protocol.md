# Ambiguity Resolution Protocol

**Purpose:** When and how to use AskUserQuestion during development workflow.

**Core Principle:** Ask, don't assume. Zero technical debt from wrong assumptions.

**Applies:** Throughout all TDD phases when ambiguity detected

---

## Overview

DevForgeAI enforces explicit decision-making. When requirements are ambiguous or conflict with context files, HALT and ask the user.

**Never assume. Always ask.**

---

## CRITICAL: Use AskUserQuestion for ALL Ambiguities

### Common Ambiguity Scenarios

#### Scenario 1: Technology Choice Ambiguous

Spec requires functionality not explicitly covered in tech-stack.md.

**Response:**

```
Question: "Spec requires [technology/feature], but tech-stack.md doesn't specify. Which should be used?"
Header: "[Category]"
Description: "This will be added to tech-stack.md as a LOCKED choice"
Options:
  - "[Option 1] (benefits: ...)"
  - "[Option 2] (benefits: ...)"
  - "[Option 3] (benefits: ...)"
multiSelect: false
```

After answer:
1. Update tech-stack.md
2. Create ADR documenting decision
3. Update dependencies.md if needed
4. Proceed with implementation

#### Scenario 2: Pattern Not Specified

Implementation needs pattern not in coding-standards.md or architecture-constraints.md.

**Use AskUserQuestion to clarify which pattern to use**

Example:
```
Question: "Story requires data access pattern, but architecture-constraints.md doesn't specify. Which pattern?"
Header: "Data Access"
Options:
  - "Repository Pattern"
  - "Active Record"
  - "Data Mapper"
multiSelect: false
```

#### Scenario 3: File Location Unclear

New file type not covered in source-tree.md.

**Use AskUserQuestion to determine correct location**

Example:
```
Question: "Where should [FileType] files be placed? source-tree.md doesn't specify."
Header: "File Location"
Options:
  - "src/[location1]/"
  - "src/[location2]/"
  - "src/[location3]/"
multiSelect: false
```

After answer:
- Update source-tree.md with new file type location
- Proceed with file creation

#### Scenario 4: Conflicting Requirements

Spec requirement conflicts with existing context files.

**Use AskUserQuestion to resolve conflict**

Example:
```
Question: "Spec requires [X], but tech-stack.md specifies [Y]. Which is correct?"
Header: "Spec Conflict"
Options:
  - "Follow tech-stack.md (use [Y])"
  - "Update tech-stack.md (use [X] + create ADR)"
multiSelect: false
```

#### Scenario 5: Version Ambiguity

Package version not specified in dependencies.md.

**Use AskUserQuestion to determine version**

Example:
```
Question: "Which version of [Package] should be used? dependencies.md doesn't specify."
Header: "Package Version"
Options:
  - "Latest stable ([version])"
  - "LTS version ([version])"
  - "Match existing ([version])"
multiSelect: false
```

After answer:
- Update dependencies.md with chosen version
- Document rationale if non-obvious choice

---

## When to HALT and Ask

**HALT development and use AskUserQuestion when:**

1. **Technology not in tech-stack.md**
   - New library, framework, or tool needed
   - Technology substitution required

2. **Multiple valid implementation approaches**
   - Different patterns could work
   - Trade-offs not clear from requirements
   - Performance vs maintainability decisions

3. **Spec conflicts with context files**
   - Story requires X, context specifies Y
   - Must resolve before implementation

4. **Security-sensitive decisions**
   - Authentication method selection
   - Encryption algorithm choice
   - Authorization pattern

5. **Performance targets unclear**
   - "Fast" without metrics
   - "Scalable" without numbers
   - SLA requirements ambiguous

6. **File placement unclear**
   - New file type not in source-tree.md
   - Ambiguous layer (domain vs application)

7. **Dependency version unclear**
   - Package not in dependencies.md
   - Version range not specified

---

## Question Structure

**Use clear, specific questions:**

```
AskUserQuestion:
    question: "[Specific question with context]?"
    header: "[Category - max 12 chars]"
    options:
        - label: "[Option 1 - concise]"
          description: "[What this means, implications]"
        - label: "[Option 2 - concise]"
          description: "[What this means, implications]"
        - label: "[Option 3 - concise]"
          description: "[What this means, implications]"
    multiSelect: false
```

**Provide 2-4 options with clear descriptions.**

---

## After User Answers

**Always document the decision:**

1. **Update context files** (tech-stack.md, dependencies.md, etc.)
2. **Create ADR** (if significant architectural decision)
3. **Update story** (if requirements clarified)
4. **Proceed with implementation** (use user's choice)

**Never second-guess user decisions. Follow their choice exactly.**

---

## Anti-Patterns to Avoid

**❌ DON'T:**
- Assume technology choice ("I'll use library X since it's popular")
- Guess user preference ("I think they meant pattern Y")
- Skip context file updates ("I'll just implement, not document")
- Make security decisions autonomously
- Estimate performance targets ("I assume 'fast' means <100ms")

**✅ DO:**
- Ask explicitly for all ambiguities
- Provide clear options with trade-offs
- Document decisions in context files
- Create ADRs for significant choices
- Follow user decisions exactly

---

## Success Criteria

Ambiguity resolution succeeds when:
- [ ] All ambiguities identified before implementation
- [ ] User explicitly chose option for each ambiguity
- [ ] Decisions documented in context files
- [ ] ADRs created for significant decisions
- [ ] No assumptions made during implementation
- [ ] Zero technical debt from unclear requirements

---

## See Also

**Framework documentation:**
- `CLAUDE.md` - Core principle: "Ask, don't assume"
- `devforgeai/specs/context/` - All 6 context files that guide development
- `devforgeai/specs/adrs/` - Architecture Decision Records

**Related workflows:**
- `preflight-validation.md` - Phase 01.6 validates spec vs context conflicts
- `dod-validation-checkpoint.md` - Layer 2 user approval for deferrals
