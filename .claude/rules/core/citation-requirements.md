---
description: Citation format standards for framework recommendations
version: "1.1"
created: 2025-12-18
source: STORY-101, STORY-102
---

# Citation Requirements for Framework Recommendations

Technology and architecture recommendations MUST cite authoritative sources to enable verification, build trust, and maintain accuracy.

## Citation Format Templates

### 1. Framework File Citations
**Format:** `(Source: {relative-path}, lines {start}-{end})`
**Example:** TypeScript is required for all new code (Source: devforgeai/specs/context/tech-stack.md, lines 12-14)
**Rules:** Path relative to project root (no leading `./`), line range ≤20 lines, file must exist and be readable, cited lines must support the recommendation.

### 2. Memory File Citations
**Format:** `(Source: {relative-path}, section {section-identifier})`
**Example:** The /dev command uses TDD workflow (Source: .claude/memory/skills-reference.md, section devforgeai-development)
**Rules:** Section identifier matches actual heading in file, use heading text or numbered format (e.g., "3.2"), path follows `.claude/memory/*.md` pattern.

### 3. Code Example Citations
**Format:** `(Source: {file-path}, lines {start}-{end})`
**Example:** Follow this test pattern (Source: devforgeai/tests/STORY-041/test-ac1-directory-structure.sh, lines 44-60)
**Rules:** Line range ≤50 lines for code citations, cited content must match example shown, larger examples require multiple citations.

## MUST Cite Categories (Blocking)

| Recommendation Type | Required Source | MUST cite |
|---------------------|-----------------|-----------|
| Technology decisions | `devforgeai/specs/context/tech-stack.md` | MUST cite with line numbers |
| Architecture decisions | `devforgeai/specs/context/architecture-constraints.md` | MUST cite with line numbers |
| Anti-pattern warnings | `devforgeai/specs/context/anti-patterns.md` | MUST cite with line numbers |
| Source tree guidance | `devforgeai/specs/context/source-tree.md` | MUST cite with line numbers |

**HALT Trigger:** If technology/architecture recommendation not documented in context files → HALT + AskUserQuestion

## SHOULD Cite Categories (Recommended)

| Recommendation Type | Suggested Source | SHOULD cite |
|---------------------|------------------|-------------|
| Coding patterns | `devforgeai/specs/context/coding-standards.md` | SHOULD cite when referencing standards |
| Workflow guidance | `.claude/memory/skills-reference.md` or `SKILL.md` | SHOULD cite skill documentation |
| Command usage | `.claude/memory/commands-reference.md` | SHOULD cite command reference |

## Validation Rules
- **File paths:** Relative to project root, forward slashes only
- **Line numbers:** Positive integers, start ≤ end
- **Placement:** Citation within 2 lines of recommendation
- **Sensitive files:** Never cite `.env`, `*secret*`, `*credential*`, `*password*` files

## Grounding Protocol (Read-Quote-Cite-Verify)

Before making technology/architecture recommendations, follow this 4-step workflow:
**Step 1: Read** - Use `Read(file_path="...")` tool to access source file before making recommendation.
**Step 2: Quote** - Extract exact, word-for-word passage (minimum 2 lines) supporting your recommendation.
**Step 3: Cite** - Reference source using citation format above.
**Step 4: Verify** - Confirm recommendation matches quoted content. If Read fails or content doesn't support recommendation, HALT.

## Grounding Examples

### Example 1: Technology Decision
1. Read: `Read(file_path="devforgeai/specs/context/tech-stack.md")`
2. Quote (lines 151-153):
   > **PROHIBITED**:
   > ❌ `Bash(command="cat file.txt")` - Use Read() instead
   > ❌ `Bash(command="echo 'content' > file.txt")` - Use Write() instead
3. Cite: (Source: devforgeai/specs/context/tech-stack.md, lines 151-153)
4. Recommendation: "Use native Read/Write tools instead of Bash commands for file operations."

### Example 2: Architecture Decision
1. Read: `Read(file_path="devforgeai/specs/context/architecture-constraints.md")`
2. Quote (lines 28-34):
   > **Single Responsibility Principle**:
   > - Each skill handles ONE phase of development lifecycle
   > - ✅ devforgeai-development: TDD implementation only
   > - ✅ devforgeai-qa: Quality validation only
3. Cite: (Source: devforgeai/specs/context/architecture-constraints.md, lines 28-34)
4. Recommendation: "Split the combined dev-qa skill into separate skills per Single Responsibility Principle."

## Verification Checklist

Before delivering a framework recommendation, verify:
- [ ] Read tool was used to access source file
- [ ] Quoted text is word-for-word from source (minimum 2 lines)
- [ ] Citation format matches standards above
- [ ] Recommendation directly relates to quoted content

**HALT Trigger:** If verification fails on any item, do not proceed with recommendation.
