# Error Type 1: Incomplete User Answers

Handling vague or incomplete responses during requirements elicitation.

---

## Error Detection

**Symptom:** User responses too vague or incomplete for requirements elicitation

**Detected during:** Phase 2 (Requirements Elicitation)

**Examples:**
- User says "I don't know" to critical questions
- User provides vague terms: "fast", "scalable", "secure", "user-friendly"
- User cannot quantify NFRs
- User selects "Other" and provides unclear free-text

**Detection logic:**

```
if user_answer contains ["I don't know", "not sure", "maybe", "TBD"]:
    trigger incomplete_answer_recovery

if user_answer contains vague_terms AND no quantification:
    vague_terms = ["fast", "slow", "scalable", "secure", "reliable", "good UX"]
    trigger vague_answer_recovery
```

---

## Recovery Procedures

### Step 1: Ask Follow-Up Questions

```
If user answer is vague:
    # Load domain-specific probing questions
    Read(file_path=".claude/skills/devforgeai-ideation/references/requirements-elicitation-guide.md")

    Use domain-specific probing:
    - E-commerce: "What payment methods should be supported?"
    - SaaS: "What user roles need different permissions?"
    - Fintech: "What compliance standards apply (PCI-DSS, SOC2, regulations)?"
    - Healthcare: "What patient data will be stored (PHI vs non-PHI)?"
```

### Step 2: Use AskUserQuestion with More Specific Options

```
# Instead of: "What performance do you need?"
# Ask with quantified options:

AskUserQuestion(
    question: "What response time is acceptable?",
    header: "Performance target",
    options: [
        {
            label: "High performance",
            description: "<100ms API response, >10k concurrent users"
        },
        {
            label: "Standard performance",
            description: "<500ms API response, 1k-10k users"
        },
        {
            label: "Moderate performance",
            description: "<2s response time, <1k users"
        }
    ]
)
```

### Step 3: Provide Examples of Good Answers

```
Explain to user:

"To help define requirements precisely, here are examples:

Instead of 'fast':
  → 'API responses under 500ms for 95th percentile'
  → 'Page loads under 2 seconds on 4G connection'

Instead of 'secure':
  → 'OAuth2 authentication with JWT tokens'
  → 'RBAC authorization with 5 roles'
  → 'AES-256 encryption for data at rest'
  → 'TLS 1.3 for data in transit'

Instead of 'scalable':
  → 'Support 10,000 concurrent users'
  → 'Horizontal scaling to 50 instances'
  → 'Database sharding for >1M records'

Would you like to provide more specific requirements?"
```

### Step 4: Document Assumptions with Validation Flags

```
If user cannot provide specifics after follow-ups:
    # Document as assumption
    Add to requirements spec:

    **ASSUMPTION:** Average order size is <100 line items
    **VALIDATION NEEDED:** Confirm with stakeholders during architecture phase
    **RISK:** If assumption wrong, may need to redesign data model

    **ASSUMPTION:** Users have modern browsers (Chrome/Firefox/Safari/Edge latest 2 versions)
    **VALIDATION NEEDED:** Check analytics data for actual browser usage
    **RISK:** If older browsers needed, may require polyfills and testing
```

---

## Example Scenarios

### Scenario 1: Vague Performance Requirement

**User says:** "The app needs to be fast"

**Recovery:**
1. Follow-up: "What response time is acceptable? <100ms, <500ms, or <2s?"
2. Provide examples of what "fast" means in similar applications
3. If still vague, document assumption with validation flag

### Scenario 2: Unclear Security Needs

**User says:** "It needs to be secure"

**Recovery:**
1. Ask about authentication method (OAuth, SAML, Basic)
2. Ask about authorization model (RBAC, ABAC, simple)
3. Ask about compliance requirements (GDPR, HIPAA, PCI-DSS)
4. Document specific security requirements or assumptions

### Scenario 3: "I don't know" Response

**User says:** "I don't know how many users we'll have"

**Recovery:**
1. Ask for order of magnitude: "Tens, hundreds, thousands, or millions?"
2. Ask about growth expectations
3. Document assumption with validation flag and risk assessment

---

## Max Recovery Attempts

**Attempt 1:** Follow-up questions with specific options
**Attempt 2:** Provide examples, ask again with guidance
**Attempt 3:** Document as assumption, flag for validation

**If still incomplete:** Continue with assumptions flagged, will be validated in architecture phase

---

## Related Patterns

- See [error-type-4-validation-failures.md](error-type-4-validation-failures.md) for validation of documented assumptions
- See [requirements-elicitation-guide.md](requirements-elicitation-guide.md) for domain-specific probing questions
- See [error-handling-index.md](error-handling-index.md) for error type decision tree

---

## Phase Context

This error occurs during **Phase 2: Requirements Elicitation** of the ideation workflow.

Recovery happens before advancing to Phase 3 (Complexity Assessment), which depends on clear, quantifiable requirements.

---

**Token Budget:** ~500-1,000 tokens per recovery attempt
