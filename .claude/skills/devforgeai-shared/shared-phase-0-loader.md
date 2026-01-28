# Shared Phase 0 Reference Loader

Reusable utility for loading phase workflow reference files in Phase 0 setup.

---

## Purpose

This utility provides a **standardized pattern** for Phase 0 Step 0.N reference file loading across all DevForgeAI skills. It eliminates code duplication and ensures consistent behavior.

---

## Skill Reference Mapping

| Skill | Mode | Reference File |
|-------|------|----------------|
| devforgeai-qa | light | (inline in SKILL.md - no external reference) |
| devforgeai-qa | deep | `.claude/skills/devforgeai-qa/references/deep-validation-workflow.md` |
| devforgeai-development | light | `.claude/skills/devforgeai-development/references/tdd-light-workflow.md` (if exists) |
| devforgeai-development | deep | `.claude/skills/devforgeai-development/references/tdd-deep-workflow.md` |
| devforgeai-orchestration | any | (skill-specific references as needed) |

---

## Usage Pattern

Each skill's Phase 0 should implement this loader pattern:

```
Skill: Reference file loading for Phase 0
Params:
  skill_name: "devforgeai-qa"  // or any skill name
  mode: "deep"  // "light" or "deep"

Execute: Load appropriate reference files based on mapping table
```

---

## Implementation

### Loader Pattern (All Skills)

```text
IF mode == "deep":
    reference_path = ".claude/skills/{skill_name}/references/{workflow}-deep-workflow.md"
    Read(file_path=reference_path)
    Display: "✓ {skill_name} deep mode workflow reference loaded"
ELSE:
    # Light mode: use inline SKILL.md workflow (no external file)
    Display: "✓ Using light mode (no external references)"
```

### Mode Behavior

| Mode | Reference Loading | Use Case |
|------|-------------------|----------|
| deep | Load external `*-deep-workflow.md` | Comprehensive validation |
| light | Skip external files, use inline SKILL.md | Fast iteration feedback |

**Default:** Light mode (deep requires explicit request)

---

## Benefits

- **Eliminates code duplication** across skills
- **Standardizes Phase 0 Step 0.N** implementation
- **Single source of truth** for reference loading patterns
- **Easy skill addition** - new skills follow documented pattern

---

## Future Skills

When creating new skills that need Phase 0 reference loading:

1. Create `references/{skill-name}-deep-workflow.md` in skill directory
2. Add entry to Skill Reference Mapping table above
3. In Phase 0 Step 0.N, implement the loader pattern from "Implementation" section above

---

## References

- **Source RCA:** RCA-021 REC-5 (MEDIUM - Reference Document Auto-Load Utility)
- **Implementation Story:** STORY-219
- **Related Skills:** devforgeai-qa, devforgeai-development, devforgeai-orchestration
