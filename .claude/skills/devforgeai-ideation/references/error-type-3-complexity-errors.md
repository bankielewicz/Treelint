# Error Type 3: Complexity Assessment Errors

Handling invalid complexity scores and tier calculation issues.

---

## Error Detection

**Symptom:** Complexity score invalid, out of range, or breakdown missing

**Detected during:** Phase 3 (Complexity Assessment) or Phase 6.4 (Validation)

**Examples:**
- Score <0 or >60
- Score breakdown missing dimensions
- Tier doesn't match score range
- Functional complexity >20, Technical >20, Team/Org >10, NFR >10

**Detection logic:**

```
if complexity_score < 0 or complexity_score > 60:
    trigger complexity_assessment_error_recovery

if functional > 20 or technical > 20 or team_org > 10 or nfr > 10:
    trigger dimension_overflow_error_recovery

if tier_mismatch(score, tier):
    trigger tier_mismatch_error_recovery
```

---

## Recovery Procedures

### Step 1: Recalculate Using Assessment Matrix

```
ERROR: Invalid complexity score {complexity_score}

# Load assessment matrix
Read(file_path=".claude/skills/devforgeai-ideation/references/complexity-assessment-matrix.md")

# Recalculate all dimensions using Phase 2 data
functional_score = calculate_functional_complexity(
    user_roles_count,
    entities_count,
    integrations_count,
    workflow_complexity
)  # Result: 0-20

technical_score = calculate_technical_complexity(
    data_volume,
    concurrency_target,
    realtime_requirements
)  # Result: 0-20

team_org_score = calculate_team_complexity(
    team_size,
    distribution
)  # Result: 0-10

nfr_score = calculate_nfr_complexity(
    performance_target,
    compliance_requirements
)  # Result: 0-10

total_score = functional + technical + team_org + nfr

# Validate totals
assert 0 <= total_score <= 60
assert 0 <= functional <= 20
assert 0 <= technical <= 20
assert 0 <= team_org <= 10
assert 0 <= nfr <= 10

✓ Recalculated complexity: {total_score}/60
```

### Step 2: Verify All 4 Dimensions Scored

```
Required dimensions:
- [ ] Functional Complexity (0-20)
- [ ] Technical Complexity (0-20)
- [ ] Team/Organizational Complexity (0-10)
- [ ] Non-Functional Complexity (0-10)

If any dimension missing:
    # Recalculate missing dimension using assessment matrix
    # Use Phase 2-5 discovery data for inputs
    missing_dimension_score = calculate_dimension(phase_data)

    # Update total score
    total_score += missing_dimension_score
```

### Step 3: Update Requirements Spec with Corrected Assessment

```
# Read existing requirements spec
Read(file_path="devforgeai/specs/requirements/{project}-requirements.md")

# Find complexity assessment section
complexity_section = Grep(
    pattern="## Complexity Assessment",
    path="devforgeai/specs/requirements/{project}-requirements.md",
    -A=20,
    output_mode="content"
)

# Replace with corrected assessment
Edit(
    file_path="devforgeai/specs/requirements/{project}-requirements.md",
    old_string="## Complexity Assessment\n{old_content}",
    new_string="## Complexity Assessment\n\n**Total Score:** {corrected_score}/60\n**Architecture Tier:** {correct_tier}\n\n**Score Breakdown:**\n- Functional: {functional}/20\n- Technical: {technical}/20\n- Team/Org: {team_org}/10\n- NFR: {nfr}/10\n..."
)

✓ Complexity assessment corrected in requirements spec
```

### Step 4: Re-Validate Architecture Tier

```
# Map score to tier
if score >= 0 and score <= 15:
    tier = "Tier 1: Simple Application"
elif score >= 16 and score <= 30:
    tier = "Tier 2: Moderate Application"
elif score >= 31 and score <= 45:
    tier = "Tier 3: Complex Platform"
elif score >= 46 and score <= 60:
    tier = "Tier 4: Enterprise Platform"

# Verify tier is documented in requirements spec
# Update technology recommendations for tier (from output-templates.md)

✓ Architecture tier validated: {tier}
```

---

## Example Scenarios

### Scenario 1: Score Out of Range

**Error:** Complexity score is 75 (max is 60)

**Recovery:**
1. Load complexity-assessment-matrix.md
2. Recalculate each dimension with proper caps
3. Update requirements spec with corrected score

### Scenario 2: Missing Dimension

**Error:** Team/Org complexity not calculated

**Recovery:**
1. Identify missing dimension
2. Calculate using team size and distribution data
3. Add to total score
4. Update requirements spec

### Scenario 3: Tier Mismatch

**Error:** Score is 42 but tier shows "Tier 2: Moderate"

**Recovery:**
1. Recalculate correct tier (42 = Tier 3: Complex Platform)
2. Update tier in requirements spec
3. Verify technology recommendations match tier

---

## Max Recovery Attempts

**Attempt 1:** Recalculate using assessment matrix, update requirements spec

**If still failing:** HALT - Critical calculation error, require manual review

---

## Related Patterns

- See [complexity-assessment-matrix.md](complexity-assessment-matrix.md) for scoring rubric
- See [output-templates.md](output-templates.md) for tier-based technology recommendations
- See [error-handling-index.md](error-handling-index.md) for error type decision tree

---

## Phase Context

This error occurs during **Phase 3: Complexity Assessment** when calculating project complexity across 4 dimensions.

It may also be detected during **Phase 6.4: Self-Validation** when validating the requirements specification artifact.

Recovery must complete before advancing to Phase 4 (Output Generation), which depends on accurate tier calculation.

---

**Token Budget:** ~1,000-2,000 tokens per recalculation
