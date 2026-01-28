# Output Contract - anti-pattern-scanner Subagent

**Purpose:** Define exact JSON structure returned by anti-pattern-scanner

---

## Success Response

```json
{
  "status": "success",
  "violations": {
    "critical": [
      {
        "file": "/absolute/path/to/file.cs",
        "line": 42,
        "pattern": "ORM substitution",
        "evidence": "using Microsoft.EntityFrameworkCore;",
        "remediation": "Replace Entity Framework with Dapper. See tech-stack.md line 45",
        "severity": "CRITICAL",
        "category": "library_substitution",
        "locked_technology": "Dapper",
        "detected_technology": "Entity Framework Core"
      }
    ],
    "high": [
      {
        "file": "/absolute/path/to/domain-service.cs",
        "line": 15,
        "pattern": "Infrastructure concern in Domain layer",
        "evidence": "private readonly DbContext _context;",
        "remediation": "Move to Infrastructure layer or use repository pattern",
        "severity": "HIGH",
        "category": "structure_violation"
      }
    ],
    "medium": [
      {
        "file": "/absolute/path/to/service.cs",
        "line": 1,
        "pattern": "God object",
        "evidence": "OrderService has 28 methods, 450 lines",
        "remediation": "Refactor into smaller classes following SRP",
        "severity": "MEDIUM",
        "category": "code_smell"
      }
    ],
    "low": [
      {
        "file": "/absolute/path/to/calculator.cs",
        "line": 10,
        "pattern": "Missing documentation",
        "evidence": "Public method Calculate lacks XML documentation",
        "remediation": "Add /// <summary> documentation comment",
        "severity": "LOW",
        "category": "style"
      }
    ]
  },
  "summary": {
    "total_violations": 4,
    "critical_count": 1,
    "high_count": 1,
    "medium_count": 1,
    "low_count": 1,
    "categories_checked": 6,
    "files_scanned": 127
  },
  "blocks_qa": true,
  "blocking_reasons": [
    "1 CRITICAL violation(s) detected (library substitution)",
    "1 HIGH violation(s) detected (structure violation)"
  ],
  "recommendations": [
    "CRITICAL: Replace Entity Framework with Dapper in 1 file(s)",
    "HIGH: Move infrastructure concerns out of Domain layer in 1 file(s)",
    "MEDIUM: Refactor god objects in 1 file(s)",
    "LOW: Add documentation to 1 item(s)"
  ],
  "metadata": {
    "story_id": "STORY-062",
    "scan_mode": "full",
    "timestamp": "2025-11-24T12:34:56Z",
    "execution_time_ms": 2847,
    "version": "1.0"
  }
}
```

---

## Failure Response (Missing Context File)

```json
{
  "status": "failure",
  "error": "Required context file not found: devforgeai/specs/context/tech-stack.md",
  "blocks_qa": true,
  "remediation": "Run /create-context to generate architectural context files before QA validation",
  "metadata": {
    "timestamp": "2025-11-24T12:34:56Z"
  }
}
```

---

## Failure Response (Contradictory Rules)

```json
{
  "status": "failure",
  "error": "Context files contradictory: tech-stack.md locks Dapper as ORM, but dependencies.md lists Entity Framework",
  "blocks_qa": true,
  "remediation": "Resolve contradiction - update tech-stack.md or dependencies.md to match. Run /create-context if context files corrupted.",
  "contradiction_details": {
    "file1": "devforgeai/specs/context/tech-stack.md",
    "file1_value": "ORM: Dapper (locked)",
    "file2": "devforgeai/specs/context/dependencies.md",
    "file2_value": "Microsoft.EntityFrameworkCore 8.0.0"
  },
  "metadata": {
    "timestamp": "2025-11-24T12:34:56Z"
  }
}
```

---

## Violation Object Schema

**Required fields (all violations must include):**
- `file` (string): Absolute path to violating file
- `line` (integer): Line number where violation occurs
- `pattern` (string): What pattern was violated
- `evidence` (string): Code snippet proving violation (1-3 lines)
- `remediation` (string): Specific fix instruction
- `severity` (string): CRITICAL | HIGH | MEDIUM | LOW

**Optional fields:**
- `category` (string): library_substitution | structure_violation | layer_violation | code_smell | security_vulnerability | style
- `owasp` (string): OWASP Top 10 reference (security violations only)
- `locked_technology` (string): Technology locked in tech-stack.md (library substitution only)
- `detected_technology` (string): Alternative technology detected (library substitution only)

---

## Blocking Logic

```
blocks_qa = (critical_count > 0) OR (high_count > 0)

IF blocks_qa:
  blocking_reasons = []

  IF critical_count > 0:
    blocking_reasons.append(f"{critical_count} CRITICAL violation(s) detected")

  IF high_count > 0:
    blocking_reasons.append(f"{high_count} HIGH violation(s) detected")
```

**MEDIUM and LOW violations NEVER block QA.**

---

## Response Validation

**QA skill should validate:**
1. `status` field exists and equals "success" or "failure"
2. If status="success", `violations` object exists
3. If status="success", `blocks_qa` boolean exists
4. If status="failure", `error` and `remediation` fields exist
5. All violation objects have 6 required fields
