# Phase 2: Anti-Pattern Detection Workflow (v2.0 - Subagent Delegation)

**Purpose:** Detect architecture violations, security issues, and code quality problems using anti-pattern-scanner subagent.

**Token Efficiency:** ~3K tokens (vs ~8K tokens inline) = 73% reduction

---

## Workflow Overview

1. Load all 6 context files
2. Invoke anti-pattern-scanner subagent
3. Parse JSON response
4. Update blocks_qa state
5. Display violations summary

---

## Step 1: Load ALL 6 Context Files

**CRITICAL:** anti-pattern-scanner requires ALL 6 context files. Load them into conversation context:

```
context_files = {}

FOR each file in ["tech-stack", "source-tree", "dependencies", "coding-standards", "architecture-constraints", "anti-patterns"]:
  file_path = f"devforgeai/specs/context/{file}.md"

  content = Read(file_path=file_path)

  context_files[file] = {
    "path": file_path,
    "content": content
  }
```

**If ANY file missing:**
```
Display: "❌ Context file missing: {file_path}"
Display: "Run /create-context to generate context files"
HALT workflow
```

---

## Step 2: Invoke anti-pattern-scanner Subagent

**Subagent invocation pattern:**

```
anti_pattern_result = Task(
  subagent_type="anti-pattern-scanner",
  description="Scan codebase for architecture violations",
  model="claude-model: opus-4-5-20251001",
  prompt=f"""
Scan codebase for architecture violations and security issues.

**Story ID:** {story_id}
**Scan Mode:** full
**Language:** {detected_language}

**Context Files Loaded (ALL 6 REQUIRED):**

1. **tech-stack.md:**
```
{context_files["tech-stack"]["content"]}
```

2. **source-tree.md:**
```
{context_files["source-tree"]["content"]}
```

3. **dependencies.md:**
```
{context_files["dependencies"]["content"]}
```

4. **coding-standards.md:**
```
{context_files["coding-standards"]["content"]}
```

5. **architecture-constraints.md:**
```
{context_files["architecture-constraints"]["content"]}
```

6. **anti-patterns.md:**
```
{context_files["anti-patterns"]["content"]}
```

**Detection Categories (check all 6):**
- Category 1: Library Substitution (CRITICAL) - 5 technology types
- Category 2: Structure Violations (HIGH) - 3 validation checks
- Category 3: Layer Violations (HIGH) - 2 dependency checks
- Category 4: Code Smells (MEDIUM) - 3 metric checks
- Category 5: Security Vulnerabilities (CRITICAL) - 4 OWASP checks
- Category 6: Style Inconsistencies (LOW) - 2 linting checks

**Expected Output:** JSON with violations grouped by severity, blocking status, and remediation guidance.

Execute anti-pattern-scanner specification: Load context → Scan 6 categories → Return structured violations.
"""
)
```

---

## Step 3: Parse JSON Response

**Validate response structure:**

```
IF anti_pattern_result["status"] == "failure":
  Display: f"❌ Anti-pattern scanner failed: {anti_pattern_result['error']}"
  Display: f"Remediation: {anti_pattern_result['remediation']}"
  blocks_qa = true
  HALT

# Extract violations by severity
violations_critical = anti_pattern_result["violations"]["critical"]
violations_high = anti_pattern_result["violations"]["high"]
violations_medium = anti_pattern_result["violations"]["medium"]
violations_low = anti_pattern_result["violations"]["low"]

# Extract summary
total_violations = anti_pattern_result["summary"]["total_violations"]
blocks_qa_anti_pattern = anti_pattern_result["blocks_qa"]
```

---

## Step 4: Update blocks_qa State (OR Logic)

**CRITICAL:** Use OR operation to preserve existing blocks from Phase 1:

```
# Preserve previous blocking state from Phase 1 (coverage)
blocks_qa = blocks_qa OR blocks_qa_anti_pattern

IF blocks_qa_anti_pattern:
  blocking_reasons.extend(anti_pattern_result["blocking_reasons"])
```

**Example:**
- Phase 1 coverage blocks (coverage <95%) → blocks_qa = true
- Phase 2 anti-patterns clean → blocks_qa_anti_pattern = false
- Result: blocks_qa remains true (OR operation preserves Phase 1 block)

---

## Step 5: Display Violations Summary

**Display format:**

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Phase 2: Anti-Pattern Detection"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

IF total_violations == 0:
  Display: "✅ No violations detected - Code complies with all architectural constraints"
ELSE:
  Display: f"Total Violations: {total_violations}"
  Display: ""

  IF len(violations_critical) > 0:
    Display: f"❌ CRITICAL ({len(violations_critical)}):"
    FOR v in violations_critical:
      Display: f"  • {v['pattern']}: {v['file']}:{v['line']}"
      Display: f"    {v['remediation']}"

  IF len(violations_high) > 0:
    Display: f"⚠️  HIGH ({len(violations_high)}):"
    FOR v in violations_high:
      Display: f"  • {v['pattern']}: {v['file']}:{v['line']}"

  IF len(violations_medium) > 0:
    Display: f"⚠️  MEDIUM ({len(violations_medium)}) - Warnings only"

  IF len(violations_low) > 0:
    Display: f"ℹ️  LOW ({len(violations_low)}) - Advisory only"

Display: ""

IF blocks_qa_anti_pattern:
  Display: "🚫 QA BLOCKED by anti-pattern violations"
  Display: "Fix CRITICAL and/or HIGH violations before proceeding"
ELSE:
  Display: "✅ Phase 2 Complete - No blocking violations"

Display: ""
```

---

## Step 6: Store Violations for QA Report

**Add violations to QA report data:**

```
qa_report_data["anti_pattern_violations"] = {
  "total": total_violations,
  "by_severity": {
    "critical": violations_critical,
    "high": violations_high,
    "medium": violations_medium,
    "low": violations_low
  },
  "blocks_qa": blocks_qa_anti_pattern,
  "blocking_reasons": anti_pattern_result["blocking_reasons"] if blocks_qa_anti_pattern else []
}
```

---

## Error Handling

**Scenario 1: Context file missing**
```
{
  "status": "failure",
  "error": "Required context file not found: devforgeai/specs/context/tech-stack.md",
  "blocks_qa": true,
  "remediation": "Run /create-context to generate context files"
}

→ Display error, set blocks_qa=true, HALT
```

**Scenario 2: Contradictory rules**
```
{
  "status": "failure",
  "error": "Context files contradictory: tech-stack.md locks Dapper, dependencies.md lists EF",
  "blocks_qa": true,
  "remediation": "Resolve contradiction - update tech-stack.md or dependencies.md"
}

→ Display error, set blocks_qa=true, HALT
```

---

## Token Efficiency

**Before (inline):** ~8K tokens
- Manual context file loading: 1K
- Inline pattern matching (6 categories): 5K
- Violation formatting: 2K

**After (subagent):** ~3K tokens
- Context file loading: 1K
- Subagent invocation prompt: 1.5K
- Response parsing: 0.5K

**Savings:** 5K tokens (73% reduction) per QA validation

**Per 100 stories:** 500K tokens saved
