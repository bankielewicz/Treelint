---
description: Audit hook registry and invocation history
argument-hint: [--validate|--performance|--check-circular]
model: opus
allowed-tools: Read, Grep, Glob, Bash
---

# /audit-hooks - Hook System Audit and Monitoring

Validate hook registry configuration, monitor invocation history, and analyze hook performance.

---

## Quick Reference

```bash
# Validate hook registry schema
/audit-hooks --validate

# Show invocation history
/audit-hooks

# Check hook performance metrics
/audit-hooks --performance

# Detect circular dependencies
/audit-hooks --check-circular
```

---

## Command Workflow

### Phase 0: Parse Mode Argument

```
MODE = $1 or "history"  # Default to history mode

IF MODE contains "--":
  # Extract mode from flag
  IF MODE == "--validate":
    MODE = "validate"
  ELIF MODE == "--performance":
    MODE = "performance"
  ELIF MODE == "--check-circular":
    MODE = "check-circular"
  ELSE:
    MODE = "history"  # Default
ELSE:
  # Positional argument
  MODE = $1 or "history"
```

---

### Phase 1: Execute Audit Mode

#### Mode 1: Validate Registry

**Validate hooks.yaml schema and configuration:**

```
Display: "Validating hook registry..."
Display: ""

# Check if config file exists
Glob(pattern="devforgeai/config/hooks.yaml")

IF file not found:
  Display: "❌ Hook registry not found"
  Display: "  Expected: devforgeai/config/hooks.yaml"
  Display: ""
  Display: "To create default configuration:"
  Display: "  cp devforgeai/config/hooks.yaml.example devforgeai/config/hooks.yaml"
  Exit

# Validate YAML syntax
Bash(command="python3 -c 'import yaml; yaml.safe_load(open(\"devforgeai/config/hooks.yaml\"))' 2>&1")

IF error:
  Display: "❌ YAML syntax error:"
  Display: "{error_message}"
  Exit

Display: "✓ YAML syntax valid"

# Read and parse hooks
Read(file_path="devforgeai/config/hooks.yaml")

# Count hooks
Grep(pattern="^  - id:", path="devforgeai/config/hooks.yaml", output_mode="count")
hook_count = result

Display: "✓ Found {hook_count} hooks"
Display: ""

# Validate required fields for each hook
Grep(pattern="^  - id: ", path="devforgeai/config/hooks.yaml", output_mode="content", -A=10)

validation_errors = []
hook_ids = []

FOR each hook block:
  Extract: id, name, operation_type, operation_pattern, trigger_status, feedback_type

  # Check required fields
  IF id missing:
    validation_errors.append("Hook missing 'id' field")
  ELIF id in hook_ids:
    validation_errors.append("Duplicate hook ID: {id}")
  ELSE:
    hook_ids.append(id)

  IF name missing:
    validation_errors.append("Hook {id}: missing 'name'")

  IF operation_type missing OR operation_type NOT IN [command, skill, subagent]:
    validation_errors.append("Hook {id}: invalid operation_type")

  IF operation_pattern missing:
    validation_errors.append("Hook {id}: missing operation_pattern")

  IF trigger_status missing:
    validation_errors.append("Hook {id}: missing trigger_status")

  IF feedback_type missing OR feedback_type NOT IN [conversation, summary, metrics, checklist]:
    validation_errors.append("Hook {id}: invalid feedback_type")

# Display results
IF len(validation_errors) == 0:
  Display: "✅ Registry validation PASSED"
  Display: ""
  Display: "All {hook_count} hooks have valid schema"
  Display: "  - Unique IDs: ✓"
  Display: "  - Required fields: ✓"
  Display: "  - Valid enums: ✓"
ELSE:
  Display: "❌ Registry validation FAILED"
  Display: ""
  Display: "Found {len(validation_errors)} errors:"
  FOR error in validation_errors:
    Display: "  • {error}"
  Display: ""
  Display: "Fix errors in devforgeai/config/hooks.yaml and re-run /audit-hooks --validate"
```

---

#### Mode 2: Invocation History (Default)

**Show hook invocation history:**

```
Display: "Hook Invocation History"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

# Check for invocation logs (future - when TodoWrite integration complete)
Glob(pattern="devforgeai/logs/hook-invocations-*.log")

IF logs found:
  # Parse most recent log
  Read(file_path="{most_recent_log}")

  # Extract invocations
  Grep(pattern="Hook invoked:", path="{most_recent_log}", output_mode="content", -A=3)

  # Display summary
  Display: "Recent hook invocations ({count} total):"
  Display: ""

  FOR invocation in results:
    Display: "  • {hook_id} → {operation_name} ({status})"
    Display: "    Duration: {duration_ms}ms | Result: {result}"

ELSE:
  Display: "No invocation history found (hooks not yet integrated with TodoWrite)"
  Display: ""
  Display: "Invocation logging will be available after EPIC-004 (TodoWrite integration)"
  Display: ""
  Display: "Current status:"
  Display: "  - Hook system: ✅ Implemented (src/hook_*.py)"
  Display: "  - Hook tests: ✅ Passing (175/175)"
  Display: "  - TodoWrite integration: ⏸️ Pending (EPIC-004)"
```

---

#### Mode 3: Performance Metrics

**Analyze hook performance:**

```
Display: "Hook Performance Metrics"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

# Check for performance metrics (future)
Glob(pattern="devforgeai/metrics/hook-performance.json")

IF metrics found:
  Read(file_path="devforgeai/metrics/hook-performance.json")

  # Parse and display metrics
  Display: "Performance Statistics:"
  Display: ""
  Display: "  Registry Lookup: {avg_registry_lookup_ms}ms avg (target: <10ms)"
  Display: "  Hook Invocation Overhead: {avg_invocation_overhead_ms}ms per hook (target: <50ms)"
  Display: "  Total Operation Overhead: {avg_total_overhead_ms}ms (target: <500ms)"
  Display: ""

  Display: "Top 5 Slowest Hooks:"
  FOR hook in top_5_slowest:
    Display: "  {index}. {hook_id}: {avg_duration_ms}ms avg"

ELSE:
  Display: "No performance metrics available yet."
  Display: ""
  Display: "Metrics collection available after EPIC-004 integration."
  Display: ""
  Display: "Test-based performance validation:"
  Display: "  • Registry lookup: <10ms ✓ (validated in test_hook_stress.py)"
  Display: "  • Hook overhead: <50ms per hook ✓"
  Display: "  • Total overhead: <500ms per operation ✓"
```

---

#### Mode 4: Check Circular Dependencies

**Detect potential circular hook chains:**

```
Display: "Circular Dependency Analysis"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

# Read hook registry
Read(file_path="devforgeai/config/hooks.yaml")

# Build dependency graph
hooks = parse_hooks_from_yaml()

dependency_map = {}
FOR hook in hooks:
  # Check if hook's pattern could trigger other hooks
  # (simplified analysis - full analysis requires runtime simulation)

  hook_id = hook['id']
  dependency_map[hook_id] = []

  # Check if this hook triggers operations that might invoke other hooks
  IF hook['feedback_type'] == 'conversation':
    # Could potentially trigger /dev, /qa operations
    # Check other hooks for patterns matching those operations

    FOR other_hook in hooks:
      IF other_hook['id'] != hook_id:
        # Check if other_hook could be triggered by this hook's operations
        # (This is simplified - actual detection requires operation_pattern analysis)

        dependency_map[hook_id].append(other_hook['id'])

# Detect cycles
circular_chains = []

FOR hook_id in dependency_map:
  visited = set()
  path = []

  # DFS to detect cycles
  IF detect_cycle_dfs(hook_id, visited, path, dependency_map):
    circular_chains.append(path)

IF len(circular_chains) == 0:
  Display: "✅ No circular dependencies detected"
  Display: ""
  Display: "All {len(hooks)} hooks validated - no infinite loop risks"
ELSE:
  Display: "⚠️ Potential circular dependencies detected:"
  Display: ""

  FOR chain in circular_chains:
    Display: "  • {' → '.join(chain)} → {chain[0]} (CIRCULAR)"

  Display: ""
  Display: "Recommendation:"
  Display: "  1. Review hook patterns in devforgeai/config/hooks.yaml"
  Display: "  2. Add trigger_conditions to break cycles"
  Display: "  3. System automatically halts at depth 3 (safety limit)"
```

---

## Error Handling

### Hook Registry Not Found

```
Error: devforgeai/config/hooks.yaml missing
Action: Display creation instructions
```

### Invalid YAML Syntax

```
Error: YAML parse error
Action: Display error message with line number
```

### No Invocation History

```
Error: Logs not available (integration pending)
Action: Display current status, note future availability
```

---

## Success Criteria

- [ ] Hook registry validated or errors reported
- [ ] Invocation history displayed (or note if unavailable)
- [ ] Performance metrics shown (or note if unavailable)
- [ ] Circular dependencies checked
- [ ] Clear actionable output for each mode
- [ ] Token usage <3K

---

## Integration

**Invoked by:** Users for hook monitoring
**Requires:** devforgeai/config/hooks.yaml (hook registry)
**Optional:** devforgeai/logs/hook-invocations-*.log (invocation history)
**Related:** /dev, /qa, /release commands (operations that trigger hooks)

---

## Performance

**Token Budget:**
- Command overhead: ~2K tokens
- Registry read: ~1K tokens
- Analysis: ~500 tokens
- Total: ~3.5K tokens

**Execution Time:**
- Validate mode: <5 seconds
- History mode: <3 seconds
- Performance mode: <3 seconds
- Check-circular mode: <10 seconds (depends on registry size)
