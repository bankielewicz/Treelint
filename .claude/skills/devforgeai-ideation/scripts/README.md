# DevForgeAI Ideation Scripts

This directory contains utility scripts for the ideation skill to automate complexity assessment and requirements validation.

## Scripts

### 1. complexity_scorer.py

**Purpose:** Calculate complexity score and recommend appropriate architecture tier.

**Usage:**

```bash
# From JSON file
python complexity_scorer.py --answers answers.json

# Interactive mode
python complexity_scorer.py --interactive

# Save output to file
python complexity_scorer.py --answers answers.json --output results.json
```

**Input Format (answers.json):**
```json
{
  "user_roles": 5,
  "entities": 12,
  "integrations": 4,
  "has_complex_integrations": false,
  "workflow_complexity": "branching",
  "data_volume": "medium",
  "concurrent_users": 1000,
  "realtime_requirements": "websockets",
  "team_size": 6,
  "team_distribution": "remote_same_tz",
  "performance_requirements": "standard",
  "compliance": "basic"
}
```

**Output:**
```json
{
  "complexity_score": 28,
  "recommended_tier": "Tier 2: Moderate Application",
  "rationale": "Complexity score of 28/60 suggests Tier 2: Moderate Application...",
  "detailed_scoring": {
    "functional_complexity": 12,
    "technical_complexity": 10,
    "team_organizational_complexity": 4,
    "non_functional_complexity": 2
  },
  "inputs": { ... }
}
```

**Interactive Mode Example:**

```bash
$ python complexity_scorer.py --interactive

DevForgeAI Complexity Scorer - Interactive Mode

Answer the following questions to calculate complexity score:

=== Functional Complexity ===

Number of user roles (e.g., 1-10): 3
Number of core data entities (e.g., 1-25): 8
Number of external integrations (e.g., 0-15): 2
Any complex integrations (legacy/SOAP)? (y/n): n

Workflow complexity:
  1. Linear
  2. Branching
  3. State machine
  4. Orchestration
Select (1-4): 2

...
```

---

### 2. requirements_validator.py

**Purpose:** Validate requirements documents for completeness, testability, and quality.

**Usage:**

```bash
# Validate requirements specification
python requirements_validator.py --spec requirements.md

# Validate epic document
python requirements_validator.py --epic EPIC-001.epic.md

# Strict mode (treat warnings as errors)
python requirements_validator.py --spec requirements.md --strict
```

**What It Checks:**

1. **Required Sections:**
   - Problem Statement
   - Solution Overview
   - Functional Requirements
   - Non-Functional Requirements
   - Success Criteria

2. **Vague Language Detection:**
   - "fast", "scalable", "secure" without metrics
   - "intuitive", "user-friendly", "easy to use"
   - "robust", "reliable", "efficient"

3. **User Story Quality:**
   - Presence of "As a... I want... So that..." format
   - Acceptance criteria for each story
   - Placeholder detection (TODO, TBD, [role])

4. **NFR Quality:**
   - Coverage of key categories (Performance, Security, Scalability, Availability)
   - Presence of specific metrics (< 500ms, 99.9%, X users)

5. **Success Metrics:**
   - Quantifiable metrics with numbers/percentages
   - Measurable outcomes

**Example Output:**

```
============================================================
Requirements Validation Report
============================================================

Result: WARNINGS

⚠️  HIGH PRIORITY ISSUES:
  - Line 45: Vague term 'fast' without specific metric
    Context: The system should be fast and responsive.

  - Line 0: User story found without acceptance criteria
    Context: As a user, I want to search products...

⚠️  MEDIUM PRIORITY WARNINGS:
  - Line 120: NFR section missing categories: Availability
  - Line 0: NFRs have few specific metrics (2 found, recommend 5+)

💡 RECOMMENDATIONS:
  - Line 45: Replace 'fast' with specific metric (e.g., '<500ms response time' instead of 'fast')
  - Add acceptance criteria for each user story (checkbox list or Given/When/Then format)
  - Add NFR categories: Availability
  - Add specific metrics to NFRs (e.g., '<500ms response time', '99.9% uptime')

============================================================
Total Issues: 4
Total Recommendations: 4
============================================================
```

---

## Python Dependencies

Both scripts use only Python standard library (no external dependencies).

**Tested with:**
- Python 3.8+

---

## Integration with DevForgeAI Workflow

### When to Use These Scripts

**complexity_scorer.py:**
- **Phase 3** of ideation skill (Complexity Assessment)
- After gathering requirements via AskUserQuestion
- Before recommending architecture tier

**requirements_validator.py:**
- **Phase 6** of ideation skill (Requirements Documentation)
- After generating epic and requirements spec documents
- Before transitioning to architecture skill

### Example Workflow

```bash
# 1. Ideation skill gathers requirements and creates answers.json
# 2. Calculate complexity score
python complexity_scorer.py --answers answers.json --output complexity_results.json

# 3. Use results in epic document
# 4. After generating requirements spec, validate it
python requirements_validator.py --spec devforgeai/specs/requirements/project-requirements.md

# 5. Fix any issues found
# 6. Validate epic documents
python requirements_validator.py --epic devforgeai/specs/Epics/EPIC-001-user-management.epic.md

# 7. Proceed to architecture skill if validation passes
```

---

## Exit Codes

Both scripts use standard exit codes:

- **0:** Success (validation passed or score calculated)
- **1:** Failure (validation failed or file not found)

In strict mode (`--strict`), warnings also trigger exit code 1.

---

## Troubleshooting

### complexity_scorer.py

**Issue:** "File not found"
- **Solution:** Ensure answers.json path is correct and file exists

**Issue:** "Invalid JSON"
- **Solution:** Validate JSON syntax (commas, quotes, brackets)

### requirements_validator.py

**Issue:** "Too many vague term warnings"
- **Solution:** Add specific metrics after vague terms (e.g., "fast (<500ms)")

**Issue:** "Missing required sections"
- **Solution:** Ensure markdown headers match expected names exactly

**Issue:** "No user stories found"
- **Solution:** Use format: "As a [role], I want [action], so that [benefit]"

---

## Future Enhancements

Potential improvements for these scripts:

1. **complexity_scorer.py:**
   - Add support for loading questions from YAML
   - Generate architecture recommendations (technology stack)
   - Export to Markdown report format

2. **requirements_validator.py:**
   - Check for duplicate requirements
   - Validate data model (entity relationships)
   - Check for orphaned acceptance criteria
   - Generate suggestions for missing NFRs based on domain

---

## License

These scripts are part of the DevForgeAI framework.
