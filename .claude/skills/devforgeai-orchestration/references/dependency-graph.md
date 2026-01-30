# Dependency Graph Analysis for Parallel Execution

**Story:** STORY-111
**Version:** 1.0
**Purpose:** Detect dependencies between tasks to prevent unsafe parallelization.

---

## Quick Reference

| Attribute | Value |
|-----------|-------|
| **Purpose** | Safe parallelization |
| **Safety Principle** | Never parallelize dependent tasks |
| **Detection Method** | Keyword matching + frontmatter parsing |
| **Circular Handling** | Detect and HALT |

---

## Dependency Detection Algorithm

### Step 1: Extract Dependencies from Frontmatter

Parse the `depends_on` field from story/feature files:

```yaml
# Example frontmatter
depends_on: ["STORY-108", "STORY-110"]
```

### Step 2: Keyword Pattern Matching

Search for dependency keywords in descriptions:

| Pattern | Example | Relationship |
|---------|---------|--------------|
| `depends on` | "Feature 2 depends on Feature 1" | Hard dependency |
| `requires` | "Requires API infrastructure" | Hard dependency |
| `blocks` | "Blocked by authentication" | Hard dependency |
| `after` | "Run after setup complete" | Ordering dependency |
| `prerequisite` | "Prerequisite: Feature A" | Hard dependency |

### Step 3: Build Dependency Graph

```
dependencies = {}

FOR each task:
    deps = extract_frontmatter_depends(task)
    deps += extract_keyword_depends(task.description)
    dependencies[task.id] = deps
```

---

## Sequential Execution for Dependent Tasks

### Pattern: Wait for Prerequisites

When Task B depends on Task A:

```markdown
# Message 1: Execute Task A
Task(subagent_type="requirements-analyst", prompt="Analyze Feature A...")

# Wait for Task A to complete before Message 2

# Message 2: Execute Task B (depends on A)
Task(subagent_type="requirements-analyst", prompt="Analyze Feature B...")
```

### Decision Logic

```
IF task_has_unresolved_dependencies(task):
    WAIT for dependencies to complete
    THEN execute task
ELSE:
    ADD to parallel batch
```

---

## Parallel Execution for Independent Tasks

### Pattern: Batch Independent Tasks

When tasks have no dependencies on each other:

```markdown
# Single message with all independent tasks
Task(subagent_type="requirements-analyst", prompt="Analyze Feature 1...")
Task(subagent_type="requirements-analyst", prompt="Analyze Feature 2...")
Task(subagent_type="requirements-analyst", prompt="Analyze Feature 3...")
# All execute concurrently
```

### Independence Check

```
is_independent(task_a, task_b):
    RETURN task_a.id NOT IN dependencies[task_b.id]
       AND task_b.id NOT IN dependencies[task_a.id]
```

---

## Transitive Dependency Handling

### Chain Resolution (A → B → C)

When dependencies form a chain:

```
Task A: No dependencies
Task B: depends_on [A]
Task C: depends_on [B]
```

**Transitive closure:** C depends on both B AND A (transitively).

### Resolution Algorithm

```
resolve_transitive(task_id, dependencies):
    direct = dependencies[task_id]
    all_deps = set(direct)

    FOR dep IN direct:
        all_deps.union(resolve_transitive(dep, dependencies))

    RETURN all_deps
```

### Execution Order

```
# Layer 1: No dependencies
Task A (parallel eligible)

# Layer 2: Depends only on Layer 1
Task B (executes after A completes)

# Layer 3: Depends on Layer 2
Task C (executes after B completes)
```

---

## Circular Dependency Detection

### Cycle Detection Algorithm

Detect circular dependencies before execution:

```
detect_cycle(task_id, dependencies, visited=[]):
    IF task_id IN visited:
        RETURN True  # Cycle found!

    visited.append(task_id)

    FOR dep IN dependencies[task_id]:
        IF detect_cycle(dep, dependencies, visited):
            RETURN True

    visited.remove(task_id)
    RETURN False
```

### Example Circular Dependency

```
Task A: depends_on [C]
Task B: depends_on [A]
Task C: depends_on [B]

# Cycle: A → C → B → A
```

### HALT Behavior

```
IF detect_cycle(any_task):
    HALT: "Circular dependency detected: {cycle_path}"
    RECOMMEND: "Review task dependencies and remove cycle"
```

---

## Integration with Feature Analyzer

### Pre-Analysis Validation

Before parallel feature analysis, validate dependency safety:

```markdown
# Step 1: Build dependency graph
dependencies = build_dependency_graph(features)

# Step 2: Check for cycles
IF has_circular_dependencies(dependencies):
    HALT("Circular dependency in features")

# Step 3: Extract parallel layers
layers = topological_sort(features, dependencies)

# Step 4: Execute by layer
FOR layer IN layers:
    # All tasks in layer are independent
    execute_parallel(layer.tasks)
    # Wait for layer completion
```

---

## Related Documentation

- `feature-analyzer.md` - Parallel feature analysis using dependency graph
- `parallel-config.md` - Concurrency limits
- `error-handling-patterns.md` - Error handling for failed dependencies
