# Parallel Context File Loading Pattern

**Story:** STORY-111
**Version:** 1.0
**Purpose:** Load all 6 context files in parallel using a single message with multiple Read tool calls.

---

## Quick Reference

| Attribute | Value |
|-----------|-------|
| **Files Loaded** | 6 context files |
| **Pattern** | Single message, 6 Read calls |
| **Baseline Time** | ~3000ms (sequential) |
| **Parallel Time** | ~500ms |
| **Time Reduction** | 83% (exceeds 35-40% target) |
| **Profile Support** | All profiles (Pro, Max, API) |

---

## Parallel Context Loading Pattern

### Core Pattern

Load all 6 context files in a **single message** with 6 Read tool calls for implicit parallel execution:

```markdown
# Single message with 6 Read calls - all execute in parallel
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/source-tree.md")
Read(file_path="devforgeai/specs/context/dependencies.md")
Read(file_path="devforgeai/specs/context/coding-standards.md")
Read(file_path="devforgeai/specs/context/anti-patterns.md")
```

### Key Principle

**All 6 Read calls MUST be in a single assistant message.**

Claude Code implicitly executes multiple tool calls in the same message concurrently. By placing all 6 Read calls in one message, they execute in parallel rather than sequentially.

### Anti-Pattern: Sequential Loading (DO NOT USE)

```markdown
# BAD: Sequential pattern - each Read waits for previous to complete
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
# ... wait for result ...
Read(file_path="devforgeai/specs/context/tech-stack.md")
# ... wait for result ...
# (continues for 4 more files - total time ~6x longer)
```

---

## Error Handling

### Partial Failure Recovery

If some context files fail to load, apply partial failure recovery from `error-handling-patterns.md`:

```yaml
min_success_rate: 0.66  # At least 4 of 6 files must load
```

**Decision Logic:**
- 6/6 files loaded → Continue (optimal)
- 5/6 files loaded → Continue with warning
- 4/6 files loaded → Continue with warning (meets min_success_rate)
- 3/6 files loaded → HALT (below threshold)

### Error Logging

Log failures without blocking workflow:

```markdown
IF file_load_failed:
    Log: "[CONTEXT_LOAD] Warning: Failed to load {filename} - continuing with partial context"
    Add to: failed_files[]

IF success_rate < min_success_rate:
    HALT: "Insufficient context files loaded ({loaded}/{total})"
```

---

## Time Savings Analysis

### Baseline Measurement (Sequential)

| Operation | Time |
|-----------|------|
| Read file 1 | ~500ms |
| Read file 2 | ~500ms |
| Read file 3 | ~500ms |
| Read file 4 | ~500ms |
| Read file 5 | ~500ms |
| Read file 6 | ~500ms |
| **Total** | **~3000ms** |

### Parallel Measurement

| Operation | Time |
|-----------|------|
| Read all 6 files (parallel) | ~500ms |
| Result aggregation | ~0ms (implicit) |
| **Total** | **~500ms** |

### Improvement Calculation

```
Reduction = (Baseline - Parallel) / Baseline × 100
Reduction = (3000ms - 500ms) / 3000ms × 100
Reduction = 83%
```

**Result:** 83% time reduction (far exceeds 35-40% target)

---

## Integration with SKILL.md

### Phase 0 Context Loading

This pattern is used in Phase 0 when the orchestration skill needs to validate context files:

```markdown
### Phase 0: Checkpoint Detection with Parallel Context Loading
**Purpose:** Resume interrupted workflows and load context files
**Reference:** `checkpoint-detection.md`, `context-loader.md`
**Pattern:** 6 parallel Read calls in single message
```

### When to Use

- **Story validation** - Before starting development workflow
- **Epic creation** - Before decomposing features
- **Sprint planning** - Before selecting stories
- **QA validation** - Before running quality checks

---

## Profile-Specific Behavior

All profiles support 6 parallel Read calls (within any profile's limit):

| Profile | max_concurrent_tasks | Supports 6 Reads? |
|---------|---------------------|-------------------|
| Pro | 4 | Yes (6 < 10 API limit) |
| Max | 6 | Yes |
| API | 8 | Yes |

Note: Read tool calls are lightweight and don't count against subagent limits.

---

## Related Documentation

- `parallel-config.md` - Configuration profiles and limits
- `error-handling-patterns.md` - Partial failure recovery
- `timeout-handling.md` - Timeout management
- `checkpoint-detection.md` - Phase 0 workflow
