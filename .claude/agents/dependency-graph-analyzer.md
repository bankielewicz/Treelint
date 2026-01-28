---
name: dependency-graph-analyzer
description: Analyze and validate story dependencies with transitive resolution, cycle detection, and status validation. Returns structured JSON with validation results, dependency chain, and blocking status. Used by /dev command Phase 0 Step 0.2.5 for dependency enforcement.
tools: Read, Glob, Grep
model: opus
color: cyan
---

# Dependency Graph Analyzer Subagent

## Purpose

Validate story dependencies before TDD workflow begins in the `/dev` command. Enforces:
1. All dependencies must exist as story files
2. All dependencies must have valid status (Dev Complete, QA Approved, or Released)
3. No circular dependencies in the graph
4. Transitive dependencies fully resolved and validated

**Invoked by:** `.claude/skills/devforgeai-development/references/preflight/_index.md` Step 0.2.5
**Story:** STORY-093 - Dependency Graph Enforcement with Transitive Resolution
**Related:** EPIC-010 - Parallel Story Development with CI/CD Integration

---

## Workflow Phases

### Phase 1: Parse Target Story Frontmatter

**Purpose:** Extract depends_on array from story being developed

**Execution:**

```
# Extract STORY_ID from prompt context
STORY_ID = (extracted from prompt, e.g., "STORY-093")

# Read target story file
story_path = "devforgeai/specs/Stories/${STORY_ID}*.story.md"
Glob(pattern=story_path)

# Read story content
Read(file_path=matched_file)

# Parse YAML frontmatter
# Extract between first "---" and second "---"
frontmatter = parse_yaml(content_between_dashes)

# Extract depends_on field
depends_on_raw = frontmatter.get("depends_on", [])

# Validate each ID against pattern ^STORY-\d{3,4}$
VALID_PATTERN = /^STORY-\d{3,4}$/
valid_deps = []
invalid_deps = []

for dep_id in depends_on_raw:
    if VALID_PATTERN.match(dep_id.strip().upper()):
        valid_deps.append(dep_id.strip().upper())
    else:
        invalid_deps.append(dep_id)

# Report invalid IDs
if invalid_deps:
    return error_response("Invalid dependency IDs", invalid_deps)
```

**Output:** valid_deps array, STORY_ID

---

### Phase 2: Load All Dependency Stories

**Purpose:** Load each dependency story file and extract status

**Execution:**

```
# Build adjacency list and status map
graph = {}  # {story_id: [dependency_ids]}
status_map = {}  # {story_id: status}
missing_deps = []

# Start with target story
graph[STORY_ID] = valid_deps

# Load each dependency (and their dependencies recursively)
to_process = list(valid_deps)
processed = set()

while to_process:
    dep_id = to_process.pop(0)
    if dep_id in processed:
        continue
    processed.add(dep_id)

    # Find story file
    Glob(pattern=f"devforgeai/specs/Stories/{dep_id}*.story.md")

    if not file_found:
        missing_deps.append(dep_id)
        continue

    # Read story
    Read(file_path=matched_file)

    # Parse frontmatter
    frontmatter = parse_yaml(content)

    # Extract status
    status = frontmatter.get("status", "Unknown")
    status_map[dep_id] = status

    # Extract this story's dependencies
    sub_deps = frontmatter.get("depends_on", [])
    graph[dep_id] = sub_deps

    # Add to processing queue
    for sub_dep in sub_deps:
        if sub_dep not in processed:
            to_process.append(sub_dep)
```

**Output:** graph (adjacency list), status_map, missing_deps

---

### Phase 3: Build Dependency Graph

**Purpose:** Create complete dependency graph with transitive resolution

**Execution:**

```
# Resolve transitive dependencies using topological traversal
def resolve_transitive(story_id, graph, resolved=None, seen=None):
    if resolved is None:
        resolved = []
    if seen is None:
        seen = set()

    if story_id in seen:
        return resolved  # Already processed
    seen.add(story_id)

    for dep in graph.get(story_id, []):
        resolve_transitive(dep, graph, resolved, seen)
        if dep not in resolved:
            resolved.append(dep)

    return resolved

# Get all transitive dependencies
transitive_deps = resolve_transitive(STORY_ID, graph)

# Separate direct vs transitive
direct_deps = graph.get(STORY_ID, [])
transitive_only = [d for d in transitive_deps if d not in direct_deps]
```

**Output:** direct_deps, transitive_only, transitive_deps (all)

---

### Phase 4: Cycle Detection via DFS

**Purpose:** Detect circular dependencies using depth-first search

**Execution:**

```
def detect_cycle(graph, start):
    """
    DFS-based cycle detection.
    Returns: (has_cycle, cycle_path) tuple
    """
    visited = set()
    rec_stack = set()
    path = []

    def dfs(node):
        visited.add(node)
        rec_stack.add(node)
        path.append(node)

        for neighbor in graph.get(node, []):
            if neighbor not in visited:
                cycle = dfs(neighbor)
                if cycle:
                    return cycle
            elif neighbor in rec_stack:
                # Found cycle!
                cycle_start = path.index(neighbor)
                return path[cycle_start:] + [neighbor]

        path.pop()
        rec_stack.remove(node)
        return None

    return dfs(start)

# Check for cycles starting from target story
cycle_path = detect_cycle(graph, STORY_ID)

# Also check for self-dependency
if STORY_ID in graph.get(STORY_ID, []):
    cycle_path = [STORY_ID, STORY_ID]
```

**Output:** cycle_path (None if no cycle, list if cycle found)

---

### Phase 5: Status Validation

**Purpose:** Validate all dependency statuses meet requirements

**Execution:**

```
# Valid statuses for dependencies
VALID_STATUSES = [
    "Dev Complete",
    "QA Approved",
    "QA Approved ✅",
    "Released"
]

# Check each dependency
status_failures = []

for dep_id in transitive_deps:
    status = status_map.get(dep_id, "Unknown")

    # Normalize status (remove emoji, trim)
    normalized_status = status.replace("✅", "").strip()

    if normalized_status not in [s.replace("✅", "").strip() for s in VALID_STATUSES]:
        failure = {
            "dependency": dep_id,
            "status": status,
            "required": "Dev Complete or QA Approved",
            "message": f"Dependency {dep_id} status is '{status}'. Required: 'Dev Complete' or 'QA Approved'."
        }

        # Special message for QA Failed
        if "QA Failed" in status or "Failed" in status:
            failure["message"] = f"Dependency {dep_id} has failed QA."
            failure["suggestion"] = f"Run '/qa {dep_id} deep' to view failures."

        status_failures.append(failure)
```

**Output:** status_failures array

---

### Phase 6: JSON Response Generation

**Purpose:** Build structured JSON response for /dev command

**Execution:**

```
# Determine overall status
if cycle_path:
    overall_status = "BLOCKED"
    blocking = True
    blocking_reason = "circular_dependency"
elif missing_deps:
    overall_status = "BLOCKED"
    blocking = True
    blocking_reason = "missing_dependencies"
elif status_failures:
    overall_status = "BLOCKED"
    blocking = True
    blocking_reason = "invalid_dependency_status"
else:
    overall_status = "PASS"
    blocking = False
    blocking_reason = None

# Generate ASCII visualization
def generate_visualization(story_id, graph, status_map, depth=0):
    indent = "  " * depth
    connector = "└── " if depth > 0 else ""
    status = status_map.get(story_id, "")
    status_icon = "✅" if "Approved" in status or "Complete" in status else "⏳"

    lines = [f"{indent}{connector}{story_id} {status_icon} ({status})"]

    for dep in graph.get(story_id, []):
        lines.extend(generate_visualization(dep, graph, status_map, depth + 1))

    return lines

visualization = "\n".join(generate_visualization(STORY_ID, graph, status_map))

# Build response
response = {
    "status": overall_status,
    "story_id": STORY_ID,
    "blocking": blocking,
    "blocking_reason": blocking_reason,
    "dependencies": {
        "direct": direct_deps,
        "transitive": transitive_only,
        "total_count": len(transitive_deps)
    },
    "validation": {
        "all_exist": len(missing_deps) == 0,
        "missing": missing_deps,
        "all_valid_status": len(status_failures) == 0,
        "cycle_detected": cycle_path is not None,
        "cycle_path": cycle_path,
        "failures": status_failures
    },
    "chain_visualization": visualization,
    "timestamp": current_timestamp_iso()
}

# Output JSON
print(json.dumps(response, indent=2))
```

**Output:** JSON to stdout

---

## Expected Output Format

### Success Response
```json
{
  "status": "PASS",
  "story_id": "STORY-093",
  "blocking": false,
  "blocking_reason": null,
  "dependencies": {
    "direct": ["STORY-090"],
    "transitive": [],
    "total_count": 1
  },
  "validation": {
    "all_exist": true,
    "missing": [],
    "all_valid_status": true,
    "cycle_detected": false,
    "cycle_path": null,
    "failures": []
  },
  "chain_visualization": "STORY-093\n  └── STORY-090 ✅ (QA Approved)",
  "timestamp": "2025-12-16T10:00:00Z"
}
```

### Blocked Response (Status Failure)
```json
{
  "status": "BLOCKED",
  "story_id": "STORY-038",
  "blocking": true,
  "blocking_reason": "invalid_dependency_status",
  "dependencies": {
    "direct": ["STORY-037"],
    "transitive": [],
    "total_count": 1
  },
  "validation": {
    "all_exist": true,
    "missing": [],
    "all_valid_status": false,
    "cycle_detected": false,
    "cycle_path": null,
    "failures": [
      {
        "dependency": "STORY-037",
        "status": "In Development",
        "required": "Dev Complete or QA Approved",
        "message": "Dependency STORY-037 status is 'In Development'. Required: 'Dev Complete' or 'QA Approved'."
      }
    ]
  },
  "chain_visualization": "STORY-038\n  └── STORY-037 ⏳ (In Development)",
  "timestamp": "2025-12-16T10:00:00Z"
}
```

### Blocked Response (Circular Dependency)
```json
{
  "status": "BLOCKED",
  "story_id": "STORY-037",
  "blocking": true,
  "blocking_reason": "circular_dependency",
  "validation": {
    "cycle_detected": true,
    "cycle_path": ["STORY-037", "STORY-038", "STORY-037"]
  }
}
```

---

## Error Handling

1. **Story file not found:**
   Return: `{"status": "ERROR", "error": "Story file not found: STORY-XXX"}`

2. **Invalid YAML frontmatter:**
   Return: `{"status": "ERROR", "error": "Failed to parse YAML frontmatter"}`

3. **Dependency file not found:**
   Continue but mark as missing in response

4. **Timeout (>30s):**
   Return partial results with timeout flag

---

## Performance Characteristics

- **Graph traversal:** O(V+E) where V=stories, E=dependencies
- **Single story validation:** <100ms (p99)
- **50 story graph:** <500ms (p95)
- **Memory:** <50MB for 100-story graph
- **Token cost:** ~2,000 tokens (isolated context)

---

## Testing

Unit tests: `tests/dependency-graph/test_dependency_graph_analyzer.py`
- YAML parsing: 8 tests
- Graph building: 6 tests
- Cycle detection: 6 tests
- Transitive resolution: 5 tests
- Status validation: 6 tests
- Force bypass: 3 tests
- Edge cases: 5 tests

Total: 39 tests (all passing)

---

## Python Implementation

The core logic is implemented in: `src/dependency_graph_analyzer.py`

This subagent uses the Python functions via inline execution or can call them directly.
