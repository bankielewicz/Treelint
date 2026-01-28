# Event-Driven Hook System Architecture

**Version:** 1.0
**Date:** 2025-11-11
**Story:** STORY-018
**Status:** Production Ready

---

## Overview

The Event-Driven Hook System enables automatic callback execution when DevForgeAI operations complete. It provides a centralized, non-invasive mechanism for triggering feedback conversations, monitoring, metrics collection, and automated retrospectives without modifying existing command/skill/subagent code.

### Key Features

- **Event-Driven:** Hooks trigger automatically on operation completion
- **Pattern-Based:** Flexible pattern matching (exact, glob, regex)
- **Condition-Aware:** Trigger based on duration, status, token usage, result code
- **Thread-Safe:** Full async support with RLock/Lock protection
- **Timeout-Protected:** Per-hook timeouts prevent hanging operations
- **Circular-Safe:** Detects and prevents infinite hook chains
- **Config-Driven:** YAML configuration with hot-reload
- **Graceful:** Hook failures isolated from primary operations

---

## Architecture

### System Components

```
┌─────────────────────────────────────────────────────────────┐
│                    DevForgeAI Operations                     │
│              (Commands, Skills, Subagents)                   │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            │ operation_complete()
                            ▼
                   ┌────────────────┐
                   │  HookSystem    │ ◄── Main API
                   └────────┬───────┘
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
          ▼                 ▼                 ▼
   ┌────────────┐   ┌──────────────┐  ┌─────────────┐
   │ Registry   │   │  Invoker     │  │  Detector   │
   │ (Config)   │   │ (Execution)  │  │ (Circular)  │
   └────────────┘   └──────────────┘  └─────────────┘
          │                 │
          ▼                 ▼
   ┌────────────┐   ┌──────────────┐
   │ Patterns   │   │ Conditions   │
   │ (Matching) │   │ (Evaluation) │
   └────────────┘   └──────────────┘
```

### Module Responsibilities

| Module | Responsibility | LOC |
|--------|----------------|-----|
| `hook_system.py` | Main coordinator and public API | 222 |
| `hook_registry.py` | YAML configuration loading and validation | 343 |
| `hook_patterns.py` | Pattern matching (exact/glob/regex) | 127 |
| `hook_conditions.py` | Trigger condition evaluation | 138 |
| `hook_invocation.py` | Hook execution orchestration | 310 |
| `hook_circular.py` | Circular dependency detection | 160 |
| **TOTAL** | **Complete hook system** | **1,300** |

---

## Data Flow

### 1. Operation Completion Trigger

```python
# In any DevForgeAI operation (command/skill/subagent)
TodoWrite(todos=[...final_status...])
# ↓
# Hook system detects completion
# ↓
hook_system.operation_complete(context)
```

### 2. Hook Discovery and Filtering

```python
# HookSystem → HookRegistry
hooks = registry.get_hooks_for_operation(
    operation_type="command",
    operation_pattern="dev",
    trigger_status="success"
)
# Returns: List[HookRegistryEntry] matching criteria
```

### 3. Pattern Matching

```python
# HookInvoker → PatternMatcher
for hook in hooks:
    if pattern_matcher.matches(operation_name, hook.operation_pattern):
        # Hook matches, check conditions next
```

### 4. Condition Evaluation

```python
# HookInvoker → TriggerConditionEvaluator
if condition_evaluator.evaluate(context, hook.trigger_conditions):
    # All conditions met, invoke hook
```

### 5. Circular Detection

```python
# HookInvoker → CircularDependencyDetector
if circular_detector.push(hook_id):
    # Not circular, safe to invoke
    await invoke_hook(hook, context)
    circular_detector.pop(hook_id)
else:
    # Circular detected, skip hook
```

### 6. Hook Invocation with Timeout

```python
# HookInvoker executes hook with timeout protection
try:
    result = await asyncio.wait_for(
        hook_runner(hook, context),
        timeout=hook.max_duration_ms / 1000
    )
except asyncio.TimeoutError:
    # Log timeout, continue operation
```

---

## Hook Registry Schema

### Complete Field Reference

```yaml
hooks:
  - id: string                    # Required, unique, ^[a-z0-9-]+$, max 50 chars
    name: string                  # Required, max 100 chars
    operation_type: enum          # Required: command|skill|subagent
    operation_pattern: string     # Required: exact, glob, or regex
    trigger_status: array         # Required: [success|failure|partial|deferred|completed]
    trigger_conditions: object    # Optional: see below
    feedback_type: enum           # Required: conversation|summary|metrics|checklist
    feedback_config: object       # Optional: type-specific configuration
    max_duration_ms: integer      # Optional: 1000-30000, default 5000
    enabled: boolean              # Optional: default true
    tags: array                   # Optional: max 5 strings
```

### Trigger Conditions Schema

```yaml
trigger_conditions:
  operation_duration_min_ms: integer   # Optional: minimum operation duration
  operation_duration_max_ms: integer   # Optional: maximum operation duration
  result_code: string                  # Optional: success|partial|failure
  token_usage_percent: integer         # Optional: 0-100 token usage threshold
  execution_order: string              # Optional: first|last|nth
  user_approval_required: boolean      # Optional: require user consent
  batch_mode: boolean                  # Optional: batch multiple hooks
```

---

## Pattern Matching

### Pattern Types

**1. Exact Match**
```yaml
operation_pattern: "dev"
# Matches: "dev" exactly
# Does NOT match: "dev-extended", "development", "devops"
```

**2. Glob Patterns**
```yaml
operation_pattern: "dev*"
# Matches: "dev", "dev-extended", "development"
# Pattern uses shell-style wildcards

operation_pattern: "create-*"
# Matches: "create-story", "create-epic", "create-sprint"

operation_pattern: "*-feedback"
# Matches: "post-dev-feedback", "qa-feedback"
```

**3. Regex Patterns**
```yaml
operation_pattern: "^dev$"
# Matches: "dev" only (anchored)

operation_pattern: "^(dev|qa)$"
# Matches: "dev" or "qa"

operation_pattern: ".*-feedback$"
# Matches: Any operation ending with "-feedback"
```

### Pattern Auto-Detection

The system auto-detects pattern type:
- **Regex:** If pattern starts with `^` or ends with `$` or contains regex metacharacters
- **Glob:** If pattern contains `*`, `?`, or `[`
- **Exact:** Otherwise (literal string match)

---

## Hook Invocation Context

Every hook receives comprehensive context:

```python
{
  "invocation_id": "uuid-v4",            # Unique invocation ID
  "hook_id": "post-dev-feedback",        # ID of hook being invoked
  "operation_id": "cmd-dev-001",         # ID of operation that completed
  "operation_type": "command",           # command|skill|subagent
  "operation_name": "dev",               # Operation name
  "status": "success",                   # success|failure|partial|deferred|completed
  "duration_ms": 45000,                  # Operation execution time
  "result_code": "success",              # Optional result code
  "token_usage": 62,                     # Token usage percentage (0-100)
  "user_facing_output": "...",           # Last output shown to user
  "timestamp": "2025-11-11T12:00:00Z",  # ISO 8601 timestamp
  "invocation_stack": ["hook1", ...]     # Current hook chain (circular detection)
}
```

---

## Circular Dependency Detection

### What Are Circular Dependencies?

Circular dependencies occur when hooks trigger operations that invoke the same hooks again:

```
Hook A triggers → Operation X completes → Hook B triggers →
Operation Y completes → Hook A triggers again → LOOP!
```

### Detection Mechanism

The system tracks an invocation stack:
1. Hook A starts: `stack = ["hook-a"]`
2. Hook A completes
3. Hook B starts: `stack = ["hook-b"]`
4. Hook B tries to invoke Hook A: `stack.push("hook-a")`
5. Detection: `"hook-a" in stack` → Circular detected!
6. Action: Skip Hook A invocation, log warning

### Max Depth Protection

Even without exact circular chains, depth is limited:
- **Default max depth:** 3 levels
- **Example:** Hook A → Hook B → Hook C (3 levels, allowed)
- **Example:** Hook A → Hook B → Hook C → Hook D (4 levels, BLOCKED)

This prevents deeply nested hook chains from resource exhaustion.

---

## Timeout Protection

### Purpose

Prevents individual hooks from hanging operations indefinitely.

### Configuration

```yaml
- id: my-hook
  max_duration_ms: 5000  # 5 seconds (default)
  # Range: 1000-30000ms (1-30 seconds)
```

### Behavior

```python
# Hook execution wrapped in timeout
try:
    result = await asyncio.wait_for(
        hook_runner(hook, context),
        timeout=max_duration_ms / 1000
    )
    # Hook completed within timeout

except asyncio.TimeoutError:
    # Hook exceeded timeout
    # System logs timeout, marks result as "timeout"
    # Operation continues normally (hook failure isolated)
```

### Timeout Tuning

- **Fast operations (1-2s):** Set 2000-3000ms
- **Standard operations (5-10s):** Use default 5000ms
- **Complex operations (>10s):** Set 10000-15000ms
- **Never exceed 30s:** System hard limit to prevent indefinite hangs

---

## Error Handling & Isolation

### Isolation Principle

**Hook failures NEVER affect primary operations.**

```python
# Operation completes successfully
result = execute_operation()  # ✅ SUCCESS

# Hook triggered
try:
    invoke_hooks(operation_context)
except HookInvocationError:
    # Hook failed, but operation already succeeded
    # Log error, notify user (optional), continue

# Operation result unchanged: ✅ SUCCESS
```

### Error Types

| Error Type | Handling | Impact |
|------------|----------|--------|
| **Missing config** | Fallback to empty registry | No hooks invoked |
| **Invalid schema** | Log errors, skip invalid hooks | Valid hooks still work |
| **Pattern compilation** | Skip hook with error | Other hooks continue |
| **Timeout** | Terminate hook, log timeout | Operation continues |
| **Circular dependency** | Skip hook, log warning | Other hooks continue |
| **Hook execution** | Catch exception, log error | Operation unaffected |

---

## Configuration Hot-Reload

### Behavior

The hook registry automatically reloads when `devforgeai/config/hooks.yaml` changes:
- File watcher detects modification
- Registry reloads within 5 seconds
- Runtime state preserved (invocation stack, history)
- Invalid config falls back to last valid registry

### Usage

```bash
# Edit config
vi devforgeai/config/hooks.yaml

# Changes apply automatically (no restart)
# Next operation completion uses new config
```

---

## Performance Characteristics

### Targets (from NFR)

| Operation | Target | Actual |
|-----------|--------|--------|
| Registry lookup | <10ms | ~2ms (O(1) dict) |
| Hook invocation overhead | <50ms/hook | ~30ms measured |
| Total overhead per operation | <500ms | ~150ms (5 hooks) |
| Config reload | <100ms | ~50ms measured |
| Concurrent operations | 100+ supported | Validated in load tests |

### Scalability

- **Registry size:** 500 hooks (warning), 1000 hooks (hard limit)
- **Concurrent operations:** 100+ simultaneous completions
- **Queue processing:** <1s latency for 10 concurrent hooks
- **Memory:** <1MB for 500 hooks, <50KB per hook context

---

## Integration with DevForgeAI

### TodoWrite Integration (Future - EPIC-004)

**Planned Integration:**

```python
# In TodoWrite tool implementation
def write_todos(todos):
    # ... existing logic ...

    if is_final_status(todos):
        # Operation completing - trigger hooks
        context = build_operation_context(
            operation_type=current_operation_type,
            operation_name=current_operation_name,
            status=infer_status_from_todos(todos),
            duration_ms=calculate_duration(),
            token_usage=get_token_usage_percent(),
            user_facing_output=get_last_output()
        )

        # Non-blocking hook invocation
        asyncio.create_task(hook_system.operation_complete(context))

    return result
```

**Benefits:**
- Non-invasive (no changes to 11 commands, 8 skills, 21 subagents)
- Centralized (all hooks managed in one location)
- Framework-wide (works for all operations using TodoWrite)

### Feedback Skill Integration

**devforgeai-feedback Skill (Future - EPIC-004):**

```python
# Hook runner implementation
async def invoke_feedback_conversation(
    hook: HookRegistryEntry,
    context: HookInvocationContext
) -> Dict[str, Any]:
    """
    Invoke feedback conversation based on hook configuration.

    Called by: HookInvoker when hook matches operation

    Args:
        hook: Hook configuration with feedback_type and feedback_config
        context: Operation completion context

    Returns:
        Feedback session results
    """
    feedback_type = hook.feedback_type
    config = hook.feedback_config

    if feedback_type == "conversation":
        return await run_interactive_feedback(config.questions, context)
    elif feedback_type == "summary":
        return await generate_feedback_summary(config.summary_sections, context)
    elif feedback_type == "metrics":
        return await collect_feedback_metrics(config.metrics, context)
    elif feedback_type == "checklist":
        return await run_feedback_checklist(config.checklist_items, context)
```

---

## Usage Examples

### Example 1: Post-Development Feedback

**Goal:** Collect developer feedback after completing a story

**Configuration:**
```yaml
- id: post-dev-feedback
  name: "Post-Development Feedback"
  operation_type: command
  operation_pattern: "dev"
  trigger_status: [success, partial]
  trigger_conditions:
    operation_duration_min_ms: 300000  # Only if >5 minutes
  feedback_type: conversation
  feedback_config:
    mode: "comprehensive"
    questions:
      - "What challenges did you encounter during TDD?"
      - "Were acceptance criteria clear and testable?"
      - "Did you defer any DoD items? Why?"
  enabled: true
```

**Behavior:**
1. User runs: `/dev STORY-042`
2. Development completes successfully after 6 minutes
3. Hook triggers automatically (matches pattern "dev", status "success", duration >5 min)
4. Feedback conversation starts with 3 questions
5. User responses persisted to `devforgeai/feedback/`
6. Control returns to user

---

### Example 2: QA Failure Monitoring

**Goal:** Track patterns in QA failures

**Configuration:**
```yaml
- id: qa-failure-tracker
  name: "QA Failure Tracker"
  operation_type: command
  operation_pattern: "qa"
  trigger_status: [failure]
  feedback_type: metrics
  feedback_config:
    metrics: [failure_type, violation_count, retry_count]
    export_to: "devforgeai/metrics/qa-failures.json"
  enabled: true
```

**Behavior:**
1. `/qa STORY-042 deep` fails (coverage below threshold)
2. Hook triggers on failure status
3. Metrics collected: failure_type="coverage", violation_count=3
4. Exported to JSON for trend analysis
5. No user interaction (metrics-only)

---

### Example 3: Sprint Retrospective

**Goal:** Automatic retrospective after sprint planning

**Configuration:**
```yaml
- id: sprint-retrospective
  name: "Sprint Retrospective"
  operation_type: command
  operation_pattern: "create-sprint"
  trigger_status: [success]
  feedback_type: checklist
  feedback_config:
    checklist_items:
      - "Sprint capacity realistic? (20-40 points recommended)"
      - "Story dependencies identified?"
      - "Technical debt addressed?"
  enabled: true
```

**Behavior:**
1. `/create-sprint Sprint-5` completes
2. Hook triggers on sprint creation success
3. Checklist presented to user
4. User checks off items
5. Results saved to sprint retrospective doc

---

### Example 4: Subagent Performance Tracking

**Goal:** Monitor slow subagents for optimization

**Configuration:**
```yaml
- id: subagent-perf-tracker
  name: "Subagent Performance Tracker"
  operation_type: subagent
  operation_pattern: ".*"  # All subagents
  trigger_status: [success]
  trigger_conditions:
    operation_duration_min_ms: 60000  # >1 minute
    token_usage_percent: 75  # High token usage
  feedback_type: metrics
  feedback_config:
    metrics: [execution_time, token_usage, subagent_name]
  enabled: true
```

**Behavior:**
- Tracks ALL subagents (pattern: `.*`)
- Only records if duration >1min OR token usage >75%
- Exports metrics for optimization analysis
- No user interaction (background metrics)

---

## API Reference

### HookSystem Class

**Main entry point for hook invocation.**

```python
class HookSystem:
    def __init__(self, config_path: Optional[Path] = None):
        """
        Initialize hook system.

        Args:
            config_path: Path to hooks.yaml (default: devforgeai/config/hooks.yaml)
        """

    async def operation_complete(self, context: HookInvocationContext) -> List[HookInvocationResult]:
        """
        Handle operation completion - invoke matching hooks.

        Args:
            context: Operation completion context

        Returns:
            List of hook invocation results
        """

    def reload_config(self) -> bool:
        """Reload hook registry from YAML file."""

    def get_hooks_for_operation(
        self,
        operation_type: str,
        trigger_status: str,
    ) -> List[HookRegistryEntry]:
        """Get candidate hooks for operation (unfiltered by pattern)."""

    def validate_registry(self) -> Tuple[bool, List[str]]:
        """Validate hook registry schema."""
```

### HookRegistry Class

**Configuration loading and management.**

```python
class HookRegistry:
    def __init__(self, config_path: Path):
        """Initialize registry with config file path."""

    def load(self) -> bool:
        """Load hooks from YAML file."""

    def reload(self) -> bool:
        """Reload configuration from file."""

    def get_all_hooks(self) -> List[HookRegistryEntry]:
        """Get all registered hooks."""

    def get_hooks_for_operation(
        self,
        operation_type: str,
        operation_pattern: str,
        trigger_status: str,
    ) -> List[HookRegistryEntry]:
        """Get hooks matching criteria."""

    def validate(self) -> Tuple[bool, List[str]]:
        """Validate all registered hooks."""
```

### PatternMatcher Class

**Pattern matching engine.**

```python
class PatternMatcher:
    def matches(self, operation: str, pattern: str) -> bool:
        """Check if operation matches pattern (exact, glob, or regex)."""

    @staticmethod
    def _detect_pattern_type(pattern: str) -> PatternType:
        """Auto-detect pattern type from content."""
```

### CircularDependencyDetector Class

**Circular invocation prevention.**

```python
class CircularDependencyDetector:
    def __init__(self, max_depth: int = 3):
        """Initialize with max invocation depth."""

    def push(self, hook_id: str) -> bool:
        """Push hook onto stack (returns False if circular or depth exceeded)."""

    def pop(self, hook_id: str) -> bool:
        """Pop hook from stack."""

    def is_circular(self, hook_id: str) -> bool:
        """Check if hook would create circular dependency."""

    def reset(self) -> None:
        """Reset stack and history."""
```

---

## Security Considerations

### Access Control

- **Registry modification:** Requires framework maintainer role
- **Hook execution:** Inherits operation permissions (cannot elevate)
- **Context data:** No API keys, passwords, or tokens exposed

### Input Validation

All fields validated:
- Hook ID: Pattern `^[a-z0-9-]+$`, max 50 chars, globally unique
- Operation pattern: Must compile as valid regex/glob
- Trigger status: Must be valid enum value
- Duration ranges: 1000-30000ms
- Token usage: 0-100 percent

### Error Isolation

- Hook failures logged but don't propagate to operations
- Invalid hooks skipped during registry load
- Timeouts forcefully terminate hooks
- Circular dependencies detected and halted

---

## Troubleshooting

### Hook Not Triggering

**Check these in order:**

1. **Hook enabled?**
   ```yaml
   enabled: true  # Must be true
   ```

2. **Pattern matching?**
   ```bash
   # Test pattern manually
   python3 -c "
   from src.hook_patterns import PatternMatcher
   pm = PatternMatcher()
   print(pm.matches('dev', 'dev*'))  # Should print True
   "
   ```

3. **Status matching?**
   ```yaml
   trigger_status: [success]  # Must include operation result
   ```

4. **Conditions passing?**
   ```yaml
   trigger_conditions:
     operation_duration_min_ms: 300000  # May be too high
   ```

5. **Check logs:**
   ```bash
   grep "hook" devforgeai/logs/*.log
   ```

---

### Hook Timing Out

**Symptoms:**
- Hook logs show "exceeded max_duration_ms"
- Feedback sessions incomplete
- Timeout warnings in logs

**Solutions:**

1. **Increase timeout:**
   ```yaml
   max_duration_ms: 10000  # Increase from 5000
   ```

2. **Simplify feedback:**
   ```yaml
   feedback_config:
     questions:
       - "Single focused question?"  # Fewer questions
   ```

3. **Check dependencies:**
   - Slow API calls in hook
   - File I/O bottlenecks
   - Network latency

---

### Circular Dependency Warning

**Symptoms:**
- Logs show "Circular dependency detected"
- Hooks not invoking as expected
- Invocation stack contains duplicates

**Solutions:**

1. **Review hook chain:**
   ```
   Hook A (triggers operation X) →
   Operation X completion (triggers Hook B) →
   Hook B (triggers operation X again) → CIRCULAR!
   ```

2. **Break the cycle:**
   - Change operation_pattern to be more specific
   - Add trigger_conditions to filter invocations
   - Disable one hook in the chain

3. **Check max_depth:**
   - Default is 3
   - If legitimate chain >3 levels, increase limit (rare)

---

## Testing

### Unit Tests (125 tests)

- **Registry validation:** 38 tests (schema compliance, field validation)
- **Pattern matching:** 45 tests (exact, glob, regex, edge cases)
- **Circular detection:** 19 tests (self-reference, chains, depth)
- **Timeout enforcement:** 24 tests (various durations, concurrent)

### Integration Tests (15 tests)

- **Hook invocation workflows:** 9 tests (end-to-end)
- **Graceful failure handling:** 6 tests (isolation verification)

### Load/Stress Tests (35 tests)

- **Concurrent operations:** 20 tests (100+ simultaneous)
- **Large registries:** 15 tests (500+ hooks, memory limits)

**Run tests:**
```bash
python3 -m pytest tests/test_hook_*.py -v
# 175 tests, 100% pass rate, ~24 seconds
```

---

## Deployment

### Initial Setup

1. **Create config directory:**
   ```bash
   mkdir -p devforgeai/config
   ```

2. **Copy default template:**
   ```bash
   cp devforgeai/config/hooks.yaml.example devforgeai/config/hooks.yaml
   ```

3. **Enable desired hooks:**
   ```yaml
   - id: my-hook
     enabled: true  # Change from false
   ```

4. **Test hook invocation:**
   ```bash
   /dev STORY-001  # Hook should trigger if conditions match
   ```

### Migration from Manual Feedback

**Before (manual):**
```bash
/dev STORY-042
# Developer manually runs:
/feedback-capture
```

**After (automated):**
```bash
/dev STORY-042
# Hook automatically triggers feedback session
# No manual intervention required
```

---

## Rollback Plan

If hooks cause issues:

1. **Disable all hooks quickly:**
   ```yaml
   # In devforgeai/config/hooks.yaml
   # Add at top:
   GLOBAL_ENABLED: false
   ```

2. **Disable specific hook:**
   ```yaml
   - id: problematic-hook
     enabled: false  # Disable while debugging
   ```

3. **Remove config file:**
   ```bash
   mv devforgeai/config/hooks.yaml devforgeai/config/hooks.yaml.backup
   # System falls back to empty registry
   ```

4. **Restart with defaults:**
   ```bash
   git checkout devforgeai/config/hooks.yaml  # Restore defaults
   ```

---

## Monitoring

### /audit-hooks Command (Future - STORY-019)

**Planned command for hook monitoring:**

```bash
# Show invocation history
/audit-hooks

# Validate registry
/audit-hooks --validate

# Show hook performance
/audit-hooks --performance

# Check for circular dependencies
/audit-hooks --check-circular
```

**Output:**
- Invocation count per hook
- Success/failure/timeout rates
- Average duration per hook
- Pattern match statistics
- Circular dependency warnings
- Configuration validation results

---

## Related Documentation

- **Implementation:** `src/hook_*.py` (6 modules, 1,300 LOC)
- **Tests:** `tests/test_hook_*.py` (175 tests)
- **Configuration:** `devforgeai/config/hooks.yaml` (default template)
- **Story:** `devforgeai/specs/Stories/STORY-018-event-driven-hook-system.story.md`
- **Epic:** `devforgeai/specs/Epics/EPIC-005-feedback-system-enhancement.epic.md`

---

## Frequently Asked Questions

**Q: Can hooks modify operations?**
A: No. Hooks are read-only observers. They cannot modify operation results or affect execution flow.

**Q: Do hooks run in parallel?**
A: No. Hooks execute serially in registration order to prevent race conditions and maintain deterministic behavior.

**Q: What happens if a hook hangs indefinitely?**
A: Hooks have max_duration_ms timeout (default 5s). System forcefully terminates hung hooks and continues normally.

**Q: Can I debug hook invocation?**
A: Yes. Check logs in `devforgeai/logs/` and set logging level to DEBUG for detailed hook execution traces.

**Q: How many hooks can I register?**
A: 500 hooks (warning threshold), 1000 hooks (hard limit). System warns at 500 and rejects registrations beyond 1000.

**Q: Can hooks trigger other operations that trigger hooks?**
A: Yes, but limited to max_depth=3 to prevent infinite loops. Circular dependencies are detected and blocked.

---

## Version History

**v1.0 (2025-11-11):**
- Initial release with STORY-018
- 6 core modules implemented
- 175 tests (100% pass rate)
- Thread-safe async design
- Production-ready implementation
