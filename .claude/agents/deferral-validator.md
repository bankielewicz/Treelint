---
name: deferral-validator
description: Validates that deferred Definition of Done items have justified technical reasons and proper documentation. Detects circular deferrals, validates story/ADR references, checks implementation feasibility. Use during QA validation when stories have deferred DoD items.
model: opus
color: green
allowed-tools:
  - Read
  - Glob
  - Grep
  - AskUserQuestion
---

# Deferral Validator Subagent

## Purpose

Validate all deferred Definition of Done items in a story to ensure:
1. Technical blockers are documented and verified
2. No unnecessary deferrals (work could be done now)
3. Scope changes have ADR documentation
4. No circular deferral chains
5. Referenced stories exist and include deferred work

## When to Invoke

**Invoked by:**
- devforgeai-development skill (Phase 6, Step 1.5 - before git commit)
- devforgeai-qa skill (Phase 0, Step 3 - during deferral validation)

**Trigger Condition:**
- Story has ANY DoD items marked [ ] (incomplete)

## Input (from conversation context)

Extract from loaded story file:
- Story ID (from YAML frontmatter)
- Definition of Done section
- Technical Specification section
- Implementation Notes (for context)

## Validation Workflow

### Substep 1: Extract Deferral Information

Parse DoD Status section:
```
FOR each item marked [ ]:
    Extract:
        - ITEM = Item description
        - REASON = Deferral reason/justification
        - STORY_REF = Referenced story ID (if "Deferred to STORY-XXX")
        - ADR_REF = Referenced ADR (if "Out of scope: ADR-XXX")
```

### Substep 2: Validate Deferral Type

Check reason matches valid patterns:

Valid formats:
- "Blocked by {external_system}: {specific_reason}"
- "Deferred to STORY-XXX: {justification}"
- "Out of scope: ADR-XXX"
- "User approved via AskUserQuestion: {context}"

Invalid formats:
- "Will add later" ❌
- "Not enough time" ❌
- "Too complex" ❌
- "Deferred" (no details) ❌
- Empty reason ❌

IF invalid format:
```
VIOLATION:
    type: "Invalid deferral reason format"
    severity: "MEDIUM"
    item: {ITEM}
    reason: {REASON}
    message: "Reason must specify: blocker, target story, or ADR"
    remediation: "Use format: 'Deferred to STORY-XXX: {reason}'"
```

### Substep 3: Validate Technical Blocker (If Claimed)

IF reason contains "Blocked by":
```
Extract blocker: {BLOCKER_NAME}

Validate blocker is external:
    Internal indicators: "our code", "our API", "our module", "internal"
    External indicators: "API v2", "third-party", "platform", "service"

IF blocker appears internal:
    VIOLATION:
        type: "Internal blocker (not valid)"
        severity: "HIGH"
        message: "Blocker '{BLOCKER_NAME}' appears to be internal code/decision"
        remediation: "Internal blockers should be resolved in story. Only external dependencies are valid blockers."

Validate blocker has resolution condition:
    Look for: "available {date}", "when {condition}", "ETA: {date}"

IF no resolution condition:
    VIOLATION:
        type: "Blocker missing resolution condition"
        severity: "MEDIUM"
        message: "External blocker must include ETA or condition for resolution"
```

### Substep 4: Check Implementation Feasibility

Read Technical Specification section:
```
Search spec for code patterns related to {ITEM}

Feasibility indicators:
1. Code pattern provided? (search for code blocks, examples)
2. Estimated size mentioned? (look for "15 lines", "simple", etc.)
3. Dependencies available? (check tech-stack.md, dependencies.md)
   Read(file_path="devforgeai/specs/context/tech-stack.md")   
   Read(file_path="devforgeai/specs/context/dependencies.md")
   
IF ALL true (feasible now):
    AND no technical blocker documented:
    VIOLATION:
        type: "Unnecessary deferral - implementation feasible"
        severity: "HIGH"
        item: {ITEM}
        evidence:
            - "Code pattern found in spec at lines {X-Y}"
            - "Estimated: ~{N} lines"
            - "Dependencies: All available"
        message: "This item could be implemented NOW"
        remediation: "Complete in current story OR create proper justification (ADR for scope change)"
```

### Substep 5: Check for ADR Requirement

IF deferral doesn't reference ADR:
```
AND item appears in original DoD (in scope):
AND no technical blocker documented:

THEN:
    VIOLATION:
        type: "Scope change without ADR"
        severity: "MEDIUM"
        item: {ITEM}
        message: "Deferring in-scope DoD item requires ADR documentation"
        justification: "Item was in Definition of Done (in scope), removing it is a scope change"
        remediation: "Create ADR-XXX documenting why work moved to future story"
```

IF deferral references ADR:
```
Glob(pattern="devforgeai/specs/adrs/{ADR_REF}*.md")

IF not found:
    VIOLATION:
        type: "ADR reference not found"
        severity: "HIGH"
        message: "Referenced {ADR_REF} does not exist"
        remediation: "Create ADR OR update deferral reference"

ELSE:
    Read ADR file
    Search for {ITEM} keywords

    IF ADR doesn't mention this item:
        VIOLATION:
            type: "ADR doesn't document this deferral"
            severity: "MEDIUM"
            message: "{ADR_REF} doesn't describe deferral of '{ITEM}'"
```

### Substep 6: Detect Circular and Multi-Level Deferral Chains

⭐ **ENHANCED (RCA-007):** Now detects both circular (A→B→A) AND multi-level (A→B→C) chains

IF reason contains "Deferred to STORY-{XXX}":
```
1. Check story exists:
   Glob(pattern="devforgeai/specs/Stories/STORY-{XXX}*.md")

   IF not found:
       VIOLATION:
           type: "Invalid story reference"
           severity: "HIGH"
           message: "Referenced STORY-{XXX} does not exist"
           remediation: "Create STORY-{XXX} OR update deferral reference OR complete work in current story"

2. Read referenced story:
   Read(file_path="devforgeai/specs/Stories/STORY-{XXX}*.md")
   Extract YAML frontmatter and status

   IF status is "Dev Complete" or "QA Approved":
       # Story already implemented, check what happened to deferred work
       Read Implementation Notes > DoD Status section

       Search for incomplete items marked [ ]

       FOR each incomplete item in referenced story (STORY-{XXX}):
           Extract item description and deferral reason

           # CHECK 1: Circular deferral (STORY-XXX → current_story_id)
           IF reason contains "Deferred to {current_story_id}":
               VIOLATION:
                   type: "Circular deferral detected"
                   severity: "CRITICAL"
                   chain: "{current_story_id} → STORY-{XXX} → {current_story_id}"
                   message: "Circular deferral chain detected - work loops back to original story"
                   evidence: "Current story defers to STORY-{XXX}, but STORY-{XXX} defers back"
                   remediation: "One story must own this work - break the cycle by implementing in one story"

           # CHECK 2: Multi-level deferral chain (STORY-XXX → STORY-YYY) ← RCA-007 NEW
           IF reason contains "Deferred to STORY-":
               Extract target_story_id from reason (STORY-YYY)

               # Check if item description matches current deferred item
               IF item_description matches {ITEM} (keyword similarity >70%):
                   VIOLATION:
                       type: "Multi-level deferral chain detected"
                       severity: "CRITICAL"
                       chain: "{current_story_id} → STORY-{XXX} → {target_story_id}"
                       message: "Work deferred multiple times creates broken chain. Deferral chains >1 level are PROHIBITED."
                       rationale: "Each additional deferral level increases risk of lost work (RCA-007: STORY-004 → STORY-005 → STORY-006)"
                       evidence:
                           - "Current story ({current_story_id}) defers '{ITEM}' to STORY-{XXX}"
                           - "STORY-{XXX} also defers similar work to {target_story_id}"
                           - "This creates 2-hop chain (risk of work being lost)"
                       remediation: "Either:
                                    1. STORY-{XXX} must implement the work (no further deferral) OR
                                    2. Create ADR-{N} justifying why work spans 3+ stories OR
                                    3. Complete work in current story ({current_story_id})"

3. Check if referenced story (STORY-{XXX}) includes deferred work:
   Extract item keywords from current story's deferred item
   Search STORY-{XXX} acceptance criteria for keywords
   Search STORY-{XXX} technical specification for keywords

   IF keywords NOT found in STORY-{XXX}:
       VIOLATION:
           type: "Referenced story missing deferred work"
           severity: "HIGH"
           message: "STORY-{XXX} does not include work deferred from {current_story_id}"
           item: "{ITEM}"
           keywords_searched: {list keywords extracted from item}
           evidence: "Searched STORY-{XXX} acceptance criteria and technical spec - no matches found"
           remediation: "Add work to STORY-{XXX} acceptance criteria OR update deferral reference OR complete in current story"
```

### Substep 7: Generate Validation Report

Return structured JSON:
```json
{
    "story_id": "STORY-XXX",
    "total_deferred": 2,
    "validation_results": [
        {
            "item": "Exit code 0 for success, 2 for error",
            "reason": "Deferred to STORY-005: Exit code handling will be in error framework story",
            "violations": [
                {
                    "type": "Unnecessary deferral",
                    "severity": "HIGH",
                    "message": "Implementation feasible NOW (15 lines, no blockers)",
                    "evidence": {
                        "code_pattern": "lines 270-300 in spec",
                        "estimated_lines": 15,
                        "dependencies_met": true,
                        "blocker_documented": false
                    },
                    "remediation": "Complete in current story OR create ADR for scope change"
                },
                {
                    "type": "Scope change without ADR",
                    "severity": "MEDIUM",
                    "message": "DoD item deferred but no ADR documenting scope change"
                },
                {
                    "type": "Circular deferral risk",
                    "severity": "CRITICAL",
                    "message": "STORY-005 also defers this work",
                    "chain": "STORY-004 → STORY-005 → STORY-004"
                }
            ]
        }
    ],
    "summary": {
        "critical_violations": 1,
        "high_violations": 2,
        "medium_violations": 1,
        "low_violations": 0,
        "recommendation": "FAIL - Fix violations before approval"
    }
}
```

## Integration

**In devforgeai-development skill (Phase 6, Step 1.5 - NEW):**
```markdown
After updating Implementation Notes with DoD status:

IF any DoD items marked [ ] (incomplete):
    Task(
        subagent_type="deferral-validator",
        description="Validate deferral justifications",
        prompt="Validate all deferred Definition of Done items.

                Story already loaded in conversation.

                Check for:
                - Valid deferral reasons
                - Technical blockers documented
                - ADR for scope changes
                - Circular deferrals
                - Referenced stories exist and include work

                Return JSON validation report."
    )

    IF validation returns CRITICAL or HIGH violations:
        HALT development
        Display violations to user
        User must fix before proceeding to git commit
```

**In devforgeai-qa skill (Phase 0, Step 3 - NEW):**
```markdown
After validating test results:

Read Implementation Notes > Definition of Done Status

IF any incomplete items found:
    Task(
        subagent_type="deferral-validator",
        description="Validate deferral justifications for QA",
        prompt="Validate all deferred DoD items for QA approval.

                Story loaded in conversation.

                Perform comprehensive validation:
                - Technical blocker verification
                - Implementation feasibility check
                - ADR requirement for scope changes
                - Circular deferral detection
                - Referenced story validation

                Return JSON validation report."
    )

    Parse validation results

    IF CRITICAL or HIGH violations:
        Add to QA report violations section
        QA Status: FAILED
        Story status: QA Failed
        HALT QA approval
```

## Token Efficiency

- Haiku model (cost-effective for validation)
- Focused validation (~5K tokens per story)
- Structured JSON output (efficient parsing)
- Invoked only when deferrals exist

## Success Criteria

**Pass conditions:**
- All deferrals have valid technical justification
- All referenced stories exist and include work
- No circular deferrals detected
- ADRs exist for scope changes
- No unnecessary deferrals (when feasible now)

**Fail conditions (block approval):**
- CRITICAL: Circular deferrals
- HIGH: Unjustified deferrals, invalid story references
- MEDIUM: Scope changes without ADR

**CRITICAL: This subagent MUST be invoked - it's the enforcement mechanism!**
