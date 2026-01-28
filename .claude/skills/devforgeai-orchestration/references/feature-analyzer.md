# Parallel Feature Analysis Pattern

**Story:** STORY-111
**Version:** 1.0
**Purpose:** Analyze multiple epic features concurrently using parallel Task() calls.

---

## Quick Reference

| Attribute | Value |
|-----------|-------|
| **Purpose** | Parallel feature decomposition |
| **Pattern** | Multiple Task() calls in single message |
| **Baseline Time** | ~10000ms (5 features sequential) |
| **Parallel Time** | ~4000ms (with batching) |
| **Time Reduction** | 60% (exceeds 35-40% target) |
| **Batching** | Uses max_concurrent_tasks from config |
| **Subagent** | requirements-analyst |

---

## Step 1: Load Parallel Configuration

Load the `max_concurrent_tasks` value from configuration:

```markdown
# Read configuration
config = Read(file_path="devforgeai/config/parallel-orchestration.yaml")

# Extract profile settings (default: pro)
profile = config.default_profile  # "pro"
max_concurrent = config.profiles[profile].max_concurrent_tasks  # 4
timeout_ms = config.profiles[profile].timeout_ms  # 120000
```

| Profile | max_concurrent_tasks | Batch Size |
|---------|---------------------|------------|
| Pro | 4 | Up to 4 features per batch |
| Max | 6 | Up to 6 features per batch |
| API | 8 | Up to 8 features per batch |

---

## Step 2: Detect Feature Dependencies

Use `dependency-graph.md` to identify dependencies:

```markdown
# Build dependency graph
dependencies = build_dependency_graph(features)

# Check for circular dependencies (HALT if found)
IF has_circular_dependencies(dependencies):
    HALT("Circular dependency detected")

# Separate independent and dependent features
independent_features = get_independent_features(features, dependencies)
dependent_features = get_dependent_features(features, dependencies)
```

---

## Step 3: Batch Independent Features

Chunk independent features respecting `max_concurrent_tasks`:

```markdown
# Example: 7 features with max_concurrent_tasks = 4

batch_size = min(len(features), max_concurrent_tasks)  # 4

batches = [
    [Feature1, Feature2, Feature3, Feature4],  # Batch 1: 4 features
    [Feature5, Feature6, Feature7]              # Batch 2: 3 features
]
```

### Batching Algorithm

```
create_batches(features, max_concurrent):
    batches = []
    current_batch = []

    FOR feature IN features:
        current_batch.append(feature)

        IF len(current_batch) >= max_concurrent:
            batches.append(current_batch)
            current_batch = []

    IF current_batch:  # Remaining features
        batches.append(current_batch)

    RETURN batches
```

---

## Step 4: Execute Parallel Analysis

### Single Message Pattern (Correct)

Execute all features in a batch with a **single message**:

```markdown
# Batch 1: 4 features in single message
Task(
    subagent_type="requirements-analyst",
    description="Analyze Feature 1: Guest Checkout",
    prompt="Decompose Feature 1: Guest Checkout into 3-6 user stories..."
)
Task(
    subagent_type="requirements-analyst",
    description="Analyze Feature 2: Saved Payment Methods",
    prompt="Decompose Feature 2: Saved Payment Methods into 3-6 user stories..."
)
Task(
    subagent_type="requirements-analyst",
    description="Analyze Feature 3: Multi-Currency Support",
    prompt="Decompose Feature 3: Multi-Currency Support into 3-6 user stories..."
)
Task(
    subagent_type="requirements-analyst",
    description="Analyze Feature 4: Progress Indicator",
    prompt="Decompose Feature 4: Progress Indicator into 3-6 user stories..."
)

# All 4 Task() calls execute concurrently (implicit parallelization)
```

### Task Prompt Template

```markdown
Decompose Feature {N}: {feature_name} into 3-6 user stories.

Context:
- Epic goal: {epic_goal}
- Feature description: {feature_description}
- Complexity estimate: {complexity_score}/10

Return:
- Story list with points estimate
- Dependencies between stories
- Technical requirements
- Acceptance criteria summary
```

---

## Step 5: Aggregate Results

After each batch completes, merge results:

```markdown
# Collect results from all parallel tasks
all_stories = []

FOR batch IN batches:
    batch_results = execute_parallel_batch(batch)

    # Aggregate successful results
    FOR result IN batch_results.successes:
        all_stories.extend(result.stories)

    # Log failures (non-blocking)
    FOR failure IN batch_results.failures:
        log_warning(f"Feature analysis failed: {failure.task_id}")

# Validate completeness
IF len(all_stories) < expected_minimum:
    log_warning("Fewer stories than expected - some features may need manual analysis")
```

---

## Anti-Pattern: Sequential FOR Loop (DO NOT USE)

```markdown
# BAD: Sequential pattern - each Task waits for previous to complete
FOR feature IN features:
    result = Task(subagent_type="requirements-analyst", prompt="...")
    # ... wait for result ...
    stories.append(result)
# Total time: N × single_task_time
```

**Why this is wrong:**
- Each Task() executes one at a time
- No parallelization benefit
- 5 features take ~10 seconds instead of ~2 seconds

---

## Time Savings Analysis

### Baseline Measurement (Sequential)

| Operation | Time |
|-----------|------|
| Analyze Feature 1 | ~2000ms |
| Analyze Feature 2 | ~2000ms |
| Analyze Feature 3 | ~2000ms |
| Analyze Feature 4 | ~2000ms |
| Analyze Feature 5 | ~2000ms |
| **Total** | **~10000ms** |

### Parallel Measurement (Pro Profile, max=4)

| Operation | Time |
|-----------|------|
| Batch 1: 4 features (parallel) | ~2000ms |
| Batch 2: 1 feature | ~2000ms |
| **Total** | **~4000ms** |

### Improvement Calculation

```
Reduction = (Baseline - Parallel) / Baseline × 100
Reduction = (10000ms - 4000ms) / 10000ms × 100
Reduction = 60%
```

**Result:** 60% time reduction (exceeds 35-40% target)

---

## Profile-Specific Examples

### Pro Profile (4 concurrent)

```
8 features → 2 batches (4 + 4)
5 features → 2 batches (4 + 1)
3 features → 1 batch (3)
```

### Max Profile (6 concurrent)

```
8 features → 2 batches (6 + 2)
5 features → 1 batch (5)
```

### API Profile (8 concurrent)

```
8 features → 1 batch (8)
5 features → 1 batch (5)
```

---

## Integration with Epic Management

### epic-management.md Step 5: Feature Decomposition

This pattern replaces the sequential FOR loop in epic-management.md:

```markdown
### Step 5: Parallel Feature Decomposition

**Reference:** `feature-analyzer.md`

1. Load parallel configuration
2. Build dependency graph
3. Batch independent features
4. Execute parallel Task() calls (single message per batch)
5. Aggregate results

See `references/feature-analyzer.md` for complete parallel execution pattern.
```

---

## Related Documentation

- `dependency-graph.md` - Dependency detection for safe parallelization
- `parallel-config.md` - Configuration profiles and limits
- `error-handling-patterns.md` - Partial failure recovery
- `epic-management.md` - Epic creation workflow
