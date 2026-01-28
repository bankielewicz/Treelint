---
title: Parallel Orchestration Implementation Guide
audience: DevForgeAI developers (Opus skill authors)
version: 1.0
created: 2025-12-04
related_research: parallel-orchestration-research.md
---

# Parallel Orchestration Implementation Guide for DevForgeAI

## Quick Reference

### When to Use Parallel Patterns

✓ **Use Parallelism For:**
- Independent subagent tasks (test-automator + code-reviewer simultaneously)
- Long-running operations (npm build, test suite)
- Multiple file reads from different locations
- Analysis of different code sections concurrently

✗ **Avoid Parallelism For:**
- Tasks with dependencies (output of task A → input of task B)
- Sequential validation (code must pass linting before review)
- Operations requiring shared state modifications
- Critical error-handling paths (must be sequential)

### Three Parallel Patterns Available

| Pattern | Use Case | Complexity | Token Impact | Time Savings |
|---------|----------|-----------|--------------|--------------|
| **Native Subagent** | Delegate work to specialized agents | Low | None | 30-40% |
| **Background Tasks** | Long-running operations (tests, builds) | Low | None | 60-80% |
| **Parallel Tools** | Multiple independent file reads/searches | Very Low | None | 15-25% |

---

## Pattern 1: Parallel Subagent Invocation (RECOMMENDED)

### Safe Implementation

```python
# Step 1: Prepare tasks (non-parallel, synchronous)
tasks = [
    {
        "type": "test-automator",
        "description": "Generate unit tests for validation layer",
        "prompt": "Generate comprehensive unit tests for src/validation/..."
    },
    {
        "type": "code-analyzer",
        "description": "Analyze code complexity and maintainability",
        "prompt": "Analyze complexity metrics for src/models/..."
    },
    {
        "type": "requirements-analyst",
        "description": "Extract technical requirements from story",
        "prompt": "Extract tech requirements from STORY-XXX..."
    }
]

# Step 2: Validate parallel feasibility
if len(tasks) > 10:
    logger.error(f"Too many parallel tasks: {len(tasks)} (max 10)")
    # Fall back to sequential
    return invoke_sequential(tasks)

if contains_dependencies(tasks):
    logger.warning("Tasks have dependencies, executing sequentially")
    return invoke_sequential(tasks)

# Step 3: Invoke all tasks in SINGLE MESSAGE
logger.info(f"Invoking {len(tasks)} parallel subagents")

# CRITICAL: All Task() calls must be in same message block
Task(
    subagent_type=tasks[0]["type"],
    description=tasks[0]["description"],
    prompt=tasks[0]["prompt"]
)

Task(
    subagent_type=tasks[1]["type"],
    description=tasks[1]["description"],
    prompt=tasks[1]["prompt"]
)

Task(
    subagent_type=tasks[2]["type"],
    description=tasks[2]["description"],
    prompt=tasks[2]["prompt"]
)

# Step 4: Implicit synchronization
# ^ Main thread WAITS HERE until all tasks complete
# This is automatic - no additional code needed

# Step 5: Process results
logger.info("All parallel subagents completed, processing results...")
results = [task1_result, task2_result, task3_result]

return results
```

### Safety Guidelines

**Task Count:**
```
1-4 tasks:   Always safe (low overhead)
5-6 tasks:   Recommended maximum for stability
7-10 tasks:  Maximum framework limit, proceed with caution
11+ tasks:   Implement batching (6 per batch, sync between batches)
```

**Dependency Detection:**
```python
def contains_dependencies(tasks: List[Task]) -> bool:
    """Check if tasks have ordering dependencies."""
    # Example: task B uses output of task A
    # If yes: return True (must execute sequentially)
    # If no: return False (can parallelize)

    for i, task_a in enumerate(tasks):
        for task_b in tasks[i+1:]:
            if task_b.prompt.mentions(task_a.expected_output):
                return True
    return False
```

**Failure Recovery:**
```python
def invoke_parallel_with_fallback(tasks: List[Task]) -> List[Result]:
    """Try parallel, fall back to sequential on failure."""

    try:
        return invoke_parallel(tasks)
    except ParallelExecutionError as e:
        logger.warning(f"Parallel execution failed: {e}")
        logger.info("Falling back to sequential execution")
        return invoke_sequential(tasks)
```

### Real-World Example

From **devforgeai-orchestration skill Phase 3 Step 3.2:**

```markdown
## Step 3.2: Parallel Feature Analysis (NEW)

Given a story with 3-5 features, analyze them in parallel:

**Before (Sequential - 15 minutes):**
1. Analyze feature 1 (5 min)
2. Analyze feature 2 (5 min)
3. Analyze feature 3 (5 min)

**After (Parallel - 6 minutes):**
1. Analyze features 1, 2, 3 SIMULTANEOUSLY (6 min total)

**Implementation:**
```
Task(subagent_type="requirements-analyst", description="Analyze Feature 1", ...)
Task(subagent_type="requirements-analyst", description="Analyze Feature 2", ...)
Task(subagent_type="requirements-analyst", description="Analyze Feature 3", ...)
# Main thread waits here (implicit synchronization)
```

**All 3 tasks execute in parallel, combined results returned.**
```

---

## Pattern 2: Background Task Execution

### Use Case: Long-Running Operations

```python
# Step 1: Start long-running task in background
logger.info("Starting test suite in background...")

test_task = Bash(
    command="npm test -- --coverage --reporters=json",
    run_in_background=True,
    description="Unit tests with coverage analysis",
    timeout=600000  # 10 minutes max
)

# CRITICAL: Task returns IMMEDIATELY with ID
task_id = test_task.task_id  # e.g., "bash_1"

logger.info(f"Test suite started (task ID: {task_id})")

# Step 2: Continue with other work while tests run
logger.info("Proceeding to Phase 3 (implementation) while tests execute...")

# ... implement code, write docs, etc. ...
# Tests execute concurrently in background

# Step 3: Retrieve results when needed (Phase 4)
logger.info(f"Retrieving test results from background task {task_id}...")

output = BashOutput(bash_id=task_id)

# Parse output
coverage = extract_coverage(output.content)
failures = extract_test_failures(output.content)

if coverage < THRESHOLD:
    logger.error(f"Coverage {coverage}% below threshold {THRESHOLD}%")
    # Handle coverage failure

return {
    "task_id": task_id,
    "coverage": coverage,
    "pass_count": failures.passed,
    "fail_count": failures.failed
}
```

### Managing Multiple Background Tasks

```python
# Tracking multiple concurrent operations
background_tasks = {
    "tests": None,
    "linting": None,
    "coverage": None
}

# Step 1: Spawn all tasks in parallel
background_tasks["tests"] = Bash(
    command="npm test",
    run_in_background=True
).task_id

background_tasks["linting"] = Bash(
    command="npm run lint",
    run_in_background=True
).task_id

background_tasks["coverage"] = Bash(
    command="npm run coverage",
    run_in_background=True
).task_id

# Step 2: Wait for all to complete
# Use polling pattern if eager validation needed
def wait_for_all_tasks(task_ids: Dict[str, str], timeout: int = 600) -> Dict:
    """Wait for all background tasks to complete."""
    results = {}
    deadline = time.time() + timeout

    while any(task_id for task_id in task_ids.values() if task_id):
        for name, task_id in task_ids.items():
            if not task_id:
                continue

            try:
                output = BashOutput(bash_id=task_id)
                if output.is_complete:
                    results[name] = output.content
                    task_ids[name] = None  # Mark as done
            except:
                pass  # Task still running

        if time.time() > deadline:
            logger.error("Timeout waiting for background tasks")
            break

    return results

results = wait_for_all_tasks(background_tasks)

# Step 3: Process results
test_output = parse_tests(results.get("tests"))
lint_output = parse_linting(results.get("linting"))
coverage_output = parse_coverage(results.get("coverage"))
```

### Task Cleanup

```python
def cleanup_background_tasks(task_ids: List[str]):
    """Clean up abandoned background tasks."""

    for task_id in task_ids:
        try:
            KillBash(shell_id=task_id)
            logger.info(f"Killed background task {task_id}")
        except:
            logger.warning(f"Failed to kill task {task_id} (may already be complete)")
```

---

## Pattern 3: Parallel Tool Calling (Automatic)

### No Code Required - Model Decides

Claude Opus 4.5 automatically parallelizes independent tool calls.

**Trigger Parallel Tool Execution:**
```markdown
Read the following files in PARALLEL to build comprehensive context:
- devforgeai/specs/context/tech-stack.md (technology constraints)
- devforgeai/specs/context/architecture-constraints.md (layer rules)
- devforgeai/specs/context/anti-patterns.md (forbidden patterns)

Then search the codebase in PARALLEL for:
- All "TODO" comments (pattern: TODO\(.*\))
- All hardcoded secrets (pattern: (API_KEY|password).*=)
- All direct instantiation anti-patterns (pattern: new\s+\w+Service)
```

**What Happens:**
1. Model sees multiple independent Read/Grep invocations
2. Automatically calls all tools in single message
3. All tools execute in parallel
4. Results returned together
5. Model continues processing

**Example Output:**
```
Assistant Response (Single Message):
├─ tool_use[1]: Read tech-stack.md
├─ tool_use[2]: Read architecture-constraints.md
├─ tool_use[3]: Read anti-patterns.md
├─ tool_use[4]: Grep "TODO"
├─ tool_use[5]: Grep "API_KEY|password"
└─ tool_use[6]: Grep "new\s+\w+Service"

(All 6 tools execute in parallel)

User Response (Single Message):
├─ tool_result[1]: Content of tech-stack.md
├─ tool_result[2]: Content of architecture-constraints.md
├─ tool_result[3]: Content of anti-patterns.md
├─ tool_result[4]: 23 TODO matches
├─ tool_result[5]: 2 hardcoded secrets found
└─ tool_result[6]: 5 direct instantiation patterns found
```

**Best Practices:**
- Group independent reads/searches together
- Use explicit parallelism hints in prompts
- Expect 15-25% faster execution
- Works automatically - no code changes needed

---

## Anti-Patterns (Don't Do This)

### Anti-Pattern 1: Task Dependencies Without Sequencing

```python
# ❌ WRONG: Tasks have dependencies but invoked in parallel

Task(subagent_type="code-generator", prompt="Write src/models/User.ts")
Task(subagent_type="test-automator", prompt="Write tests for User model")

# Problem: test-automator might run before code-generator finishes
# Result: Tests written for non-existent code, massive failure
```

**Correct Approach:**
```python
# ✅ RIGHT: Wait for dependency to complete

result1 = invoke_and_wait(
    Task(subagent_type="code-generator", prompt="Write src/models/User.ts")
)

result2 = invoke_and_wait(
    Task(
        subagent_type="test-automator",
        prompt="Write tests for User model (file: src/models/User.ts)"
    )
)
```

---

### Anti-Pattern 2: Spawning Too Many Parallel Tasks

```python
# ❌ WRONG: 50 parallel subagents (will cause JSON serialization freeze)

for i in range(50):
    Task(subagent_type="code-analyzer", prompt=f"Analyze module {i}")

# Problem: Framework limit is 10, causes 100% CPU freeze on large projects
```

**Correct Approach:**
```python
# ✅ RIGHT: Batch in groups of 6 with synchronization

BATCH_SIZE = 6

for batch_start in range(0, 50, BATCH_SIZE):
    batch_end = min(batch_start + BATCH_SIZE, 50)
    batch = task_list[batch_start:batch_end]

    # Invoke batch
    for task in batch:
        Task(subagent_type=task.type, prompt=task.prompt)

    # Implicit synchronization - all tasks complete before next batch
    logger.info(f"Batch {batch_start}-{batch_end} complete")
```

---

### Anti-Pattern 3: Ignoring Background Task Results

```python
# ❌ WRONG: Start background task but never retrieve results

test_task_id = Bash(
    command="npm test",
    run_in_background=True
).task_id

# ... proceed to Phase 5 without checking test results ...

# Problem: Unknown test status, potential hidden failures released to production
```

**Correct Approach:**
```python
# ✅ RIGHT: Retrieve and validate results before proceeding

test_task_id = Bash(
    command="npm test",
    run_in_background=True
).task_id

# ... proceed to Phase 4 ...

# Phase 4.5 (before release)
test_output = BashOutput(bash_id=test_task_id)

if test_output.exit_code != 0:
    logger.error("Tests failed, cannot proceed to release")
    return BLOCKED

logger.info("Tests passed, proceeding to Phase 5")
```

---

## Troubleshooting

### Problem: Parallel Tasks Get Stuck

**Symptom:**
```
Invoking 4 parallel subagents...
Waiting for all subagents to complete...
[30 seconds pass, no progress]
```

**Causes:**
1. One subagent crashed silently
2. Circular dependency between tasks
3. Subagent waiting for user input

**Solution:**
```python
# Add timeout
timeout = 120  # seconds
start_time = time.time()

while time.time() - start_time < timeout:
    # Check results
    try:
        results = get_task_results()
        if all_complete(results):
            return results
    except:
        pass

# Timeout reached - fallback to sequential
logger.error(f"Parallel execution timeout after {timeout}s")
logger.info("Falling back to sequential execution")
return invoke_sequential(tasks)
```

---

### Problem: Background Task Never Completes

**Symptom:**
```
Tests running in background (task ID: bash_1)
Proceeding to Phase 4...
Retrieving results from bash_1...
[No output, command seems to hang]
```

**Causes:**
1. Process deadlock (waiting for input)
2. Resource exhaustion (disk full, memory)
3. Infinite loop in test suite

**Solution:**
```python
# Add kill-switch for hung processes
def get_background_result_with_timeout(task_id: str, timeout: int = 300) -> str:
    """Retrieve background task with timeout."""

    start = time.time()
    while time.time() - start < timeout:
        try:
            output = BashOutput(bash_id=task_id)
            if output.is_complete:
                return output.content
        except:
            pass
        time.sleep(2)  # Check every 2 seconds

    # Timeout - kill the task
    logger.warning(f"Background task {task_id} timeout after {timeout}s, killing...")
    KillBash(shell_id=task_id)
    raise TimeoutError(f"Background task {task_id} did not complete within {timeout}s")
```

---

## Migration Path: Sequential → Parallel

### Step 1: Identify Independent Tasks

In existing skill, find tasks that:
- Don't depend on each other's output
- Can run simultaneously
- Are CPU/IO bound (not waiting on locks)

**Example from orchestration skill Phase 3:**
```markdown
Current (Sequential):
1. Analyze story acceptance criteria (5 min)
2. Extract technical requirements (5 min)
3. Identify dependencies (2 min)
Total: 12 minutes

↓ Identify independent tasks...
- Step 1 & 2 are independent (can parallelize)
- Step 3 depends on step 2 output (must be sequential)

Optimized:
1. [Parallel] Analyze AC + Extract requirements (6 min)
2. [Sync point - wait for step 1 completion]
3. Identify dependencies (2 min)
Total: 8 minutes (33% faster)
```

### Step 2: Refactor to Parallel Pattern

```python
# BEFORE (Sequential)
def analyze_story(story_id: str) -> StoryAnalysis:
    criteria_analysis = invoke_and_wait(
        Task(subagent_type="requirements-analyst", prompt="Analyze AC...")
    )

    tech_requirements = invoke_and_wait(
        Task(subagent_type="api-designer", prompt="Extract requirements...")
    )

    dependencies = invoke_and_wait(
        Task(subagent_type="dependency-analyzer", prompt="Identify deps...")
    )

    return StoryAnalysis(criteria_analysis, tech_requirements, dependencies)

# AFTER (Parallel where possible)
def analyze_story(story_id: str) -> StoryAnalysis:
    # Step 1: Parallel tasks (no dependencies between them)
    Task(subagent_type="requirements-analyst", prompt="Analyze AC...")
    Task(subagent_type="api-designer", prompt="Extract requirements...")

    # Step 2: Implicit sync point (wait for both to complete)

    # Step 3: Sequential (depends on step 1 output)
    dependencies = invoke_and_wait(
        Task(subagent_type="dependency-analyzer", prompt="Identify deps...")
    )

    return StoryAnalysis(criteria_analysis, tech_requirements, dependencies)
```

### Step 3: Measure Impact

```python
import time

def measure_performance(skill_function, iterations: int = 3):
    """Measure average execution time."""

    times = []
    for i in range(iterations):
        start = time.time()
        result = skill_function()
        elapsed = time.time() - start
        times.append(elapsed)

    avg = sum(times) / len(times)
    improvement = ((times[0] - avg) / times[0]) * 100

    logger.info(f"Average time: {avg:.1f}s")
    logger.info(f"Time improvement: {improvement:.1f}%")

    return avg
```

---

## Integration with DevForgeAI

### Updated Architecture Constraints

Add to `devforgeai/specs/context/architecture-constraints.md`:

```markdown
## Parallel Execution Rules

### Allowed Parallelism Patterns
- Multiple Task() invocations in single message (max 10 concurrent)
- Background Bash execution with run_in_background=true
- Parallel tool calling (automatically used by Opus 4.5)

### Task Count Limits
- Recommended: 4-6 parallel subagents per batch
- Maximum: 10 concurrent tasks
- Beyond 10: Implement explicit batching with synchronization points

### Dependency Rules
- Parallel tasks MUST be independent (no cross-task dependencies)
- If task B uses output of task A: execute sequentially
- Synchronization points required between dependent batches

### Background Task Rules
- Timeout required for all background tasks (60-600 seconds)
- Results must be retrieved before next phase
- Cleanup on error or deferral

### Failure Recovery
- Primary: Attempt parallel execution
- Fallback: Silently retry as sequential if parallel fails
- Logging: Record actual vs expected parallelism
```

---

## Quick Wins (Quick Implementation Opportunities)

### Quick Win 1: Parallel Phase 0 Context Loading

**Current:** Sequential Read calls (~5 files, ~8 seconds)
**Optimized:** Parallel Read calls (~8 seconds, same time, but sets precedent)

```python
# Load 6 context files in PARALLEL
# Currently done sequentially in each skill Phase 0

Read("devforgeai/specs/context/tech-stack.md")
Read("devforgeai/specs/context/source-tree.md")
Read("devforgeai/specs/context/dependencies.md")
Read("devforgeai/specs/context/coding-standards.md")
Read("devforgeai/specs/context/architecture-constraints.md")
Read("devforgeai/specs/context/anti-patterns.md")

# Model will parallelize these automatically in Opus 4.5
```

**Effort:** 0 hours (already works automatically)
**Benefit:** Sets pattern for other parallel uses

---

### Quick Win 2: Parallel Story Feature Analysis

**Current:** Sequential Grep searches for each story (10 features × 2 searches each = 20 sequential operations)
**Optimized:** Parallel Grep searches (all 20 concurrent via tool parallelism)

```python
# In devforgeai-orchestration Phase 3 Step 3.1

# Parallelize these Grep operations
for feature in story_features:
    Grep(pattern=f"class.*{feature.name}", glob="**/*.ts")  # All parallel
    Grep(pattern=f"(AC|test|requirement).*{feature.name}", glob="**/*.md")  # All parallel
```

**Effort:** 1 hour (update orchestration skill)
**Benefit:** 50-60% faster feature extraction

---

## Next Steps

1. **Review** this guide with team
2. **Implement** Quick Win 1 (verify automatic parallelism works)
3. **Test** Quick Win 2 (measure time improvement)
4. **Document** findings in research report
5. **Plan** Phase 2 (Programmatic Tool Calling)
6. **Execute** as part of STORY-075 (Orchestration Skill Refactor)

---

## See Also

- `devforgeai/specs/research/parallel-orchestration-research.md` - Full research report
- `devforgeai/specs/context/architecture-constraints.md` - Updated constraints
- `.claude/skills/devforgeai-orchestration/SKILL.md` - Orchestration skill implementation
- `.claude/skills/devforgeai-development/SKILL.md` - Development skill with background tasks
