# Phase 4.5: Deferred Work Tracking

Tracks deferred Definition of Done (DoD) items and ensures follow-up stories/ADRs exist.

## Purpose

**Prevent technical debt from being lost:**
1. Scan stories for deferred DoD items
2. Validate that deferrals have proper tracking (story references, ADR references)
3. Create missing tracking stories or ADRs
4. Update technical debt register
5. Analyze debt trends during sprint planning
6. Prevent circular deferrals

**This phase implements RCA-006 deferral tracking enhancements.**

---

## When Triggered

**Automatic invocation:**
- Story reaches "Dev Complete" status (before QA validation)
- Sprint planning begins (analyze accumulated debt)
- Sprint retrospective (review debt trends)

**Manual invocation:**
- During technical debt review
- When updating technical-debt-register.md

---

## Step 1: Scan for Deferrals

### Extract Deferred Items

```
Read story file:
  Read(file_path="devforgeai/specs/Stories/{STORY_ID}.story.md")

Find Definition of Done section

Extract all items marked [ ] (incomplete):
  Grep(
    pattern="- \\[ \\].*",
    path=story_file,
    output_mode="content"
  )

Parse deferred items:
  FOR each item marked [ ]:
    item_text = extract item description
    deferral_reason = extract reason from parentheses or brackets

    deferred_items.append({
      "description": item_text,
      "reason": deferral_reason,
      "type": infer_deferral_type(deferral_reason)
    })

Count deferrals by type:
  story_splits = count items with "Tracked in STORY-XXX"
  scope_changes = count items with "See ADR-XXX"
  external_blockers = count items with "External blocker:" or "Blocked by:"
  other = count items without clear type

Display summary:
  "Deferred DoD Items Found: {total_count}
   - Story splits: {story_splits}
   - Scope changes: {scope_changes}
   - External blockers: {external_blockers}
   - Other: {other}"
```

---

## Step 2: Validate Deferral Tracking

### Story Split Deferrals

**Pattern:** `- [ ] {item}: [Deferred - Tracked in STORY-XXX]`

```
FOR each deferral with reason_type == "story_split":
  referenced_story = extract_story_id(reason)
  # Example: "Tracked in STORY-043" → referenced_story = "STORY-043"

  # Verify tracking story exists
  Glob(pattern="devforgeai/specs/Stories/{referenced_story}*.md")

  IF not found:
    WARN: "Referenced {referenced_story} not found"

    AskUserQuestion:
      question: "Deferral references {referenced_story} but story doesn't exist. Create it?"
      header: "Missing Story"
      options:
        - label: "Yes - create tracking story now"
          description: "Generate tracking story for this deferred work"
        - label: "No - I'll create it manually"
          description: "Skip for now, I'll create tracking story later"
        - label: "Fix reference (I meant different story)"
          description: "Update deferral reason to correct story ID"
      multiSelect: false

    IF user selects "Yes - create tracking story now":
      # Set context for story creation
      **Feature Description:** Create tracking story for deferred work: {item_text}
      **Story ID:** {referenced_story} (specific ID requested)
      **Epic:** {current_story.epic} (same epic as original story)
      **Sprint:** Backlog (will be planned later)
      **Story Type:** deferral_tracking
      **Parent Story:** {STORY_ID} (deferred work from)

      # Invoke story-creation skill
      Skill(command="devforgeai-story-creation")

      # Verify story created
      Glob(pattern="devforgeai/specs/Stories/{referenced_story}*.md")
      IF found:
        Display: "✓ Created {referenced_story}: {title}"
      ELSE:
        ERROR: "Story creation failed for {referenced_story}"

  ELSE IF found:
    # Story exists, validate it's properly linked
    Read(file_path=tracking_story_file)

    # Check for reverse reference (tracking story → original story)
    Grep(pattern="Parent Story.*{STORY_ID}|Deferred from.*{STORY_ID}")

    IF not found:
      WARN: "Tracking story {referenced_story} exists but doesn't reference parent {STORY_ID}"
      # Add reference to tracking story
      Edit tracking story to add parent reference
```

---

### Scope Change Deferrals

**Pattern:** `- [ ] {item}: [Deferred - See ADR-XXX]`

```
FOR each deferral with reason_type == "scope_change":
  adr_reference = extract_adr_id(reason)
  # Example: "See ADR-012" → adr_reference = "ADR-012"

  # Verify ADR exists
  Glob(pattern="devforgeai/specs/adrs/{adr_reference}*.md")

  IF not found:
    WARN: "Referenced {adr_reference} not found"

    AskUserQuestion:
      question: "Deferral references {adr_reference} but ADR doesn't exist. Create it?"
      header: "Missing ADR"
      options:
        - label: "Yes - document scope change in ADR"
          description: "Generate ADR documenting why this work was deferred"
        - label: "No - I'll create it manually"
          description: "Skip for now, I'll create ADR later"
        - label: "Change justification (not scope change)"
          description: "Update deferral reason to different type"
      multiSelect: false

    IF user selects "Yes - document scope change in ADR":
      Task(
        subagent_type="architect-reviewer",
        description="Create scope change ADR",
        prompt="Create {adr_reference} documenting scope change decision.

                Context:
                - Story: {STORY_ID}
                - Deferred item: {item_text}
                - Reason: {deferral_reason}

                Document:
                - Why this work was deferred (business justification)
                - Scope impact (what's included vs excluded)
                - Future work plan (when to address deferred item)
                - Decision rationale (why deferral acceptable)

                Output: devforgeai/specs/adrs/{adr_reference}-scope-change-{STORY_ID}.md"
      )

      # Verify ADR created
      Glob(pattern="devforgeai/specs/adrs/{adr_reference}*.md")
      IF found:
        Display: "✓ Created {adr_reference}: Scope change documentation"
      ELSE:
        ERROR: "ADR creation failed for {adr_reference}"
```

---

### External Blocker Deferrals

**Pattern:** `- [ ] {item}: [External blocker: {description}]` OR `- [ ] {item}: [Blocked by: {dependency}]`

```
FOR each deferral with reason_type == "external_blocker":
  # Verify logged in technical debt register

  # Check if technical debt register exists
  Glob(pattern="devforgeai/technical-debt-register.md")

  IF not found:
    # Auto-create from template
    Read(file_path=".claude/skills/devforgeai-orchestration/assets/templates/technical-debt-register-template.md")
    template_content = file_content

    Write(
      file_path="devforgeai/technical-debt-register.md",
      content=template_content
    )

    Display: "Created technical debt register from template"

  # Read debt register
  Read(file_path="devforgeai/technical-debt-register.md")

  # Search for existing entry for this deferral
  Grep(
    pattern="{STORY_ID}.*{item_text}",
    path="devforgeai/technical-debt-register.md"
  )

  IF not found:
    # Append new debt entry to register

    debt_entry = format_debt_entry(
      story_id=STORY_ID,
      item=item_text,
      reason=deferral_reason,
      blocker=external_blocker_description,
      date_deferred=current_date,
      priority="High"  # External blockers are high priority
    )

    Edit(
      file_path="devforgeai/technical-debt-register.md",
      old_string="## Open Items\n\n",
      new_string=f"## Open Items\n\n{debt_entry}\n\n"
    )

    Display: "✓ Added to technical debt register: {item_text}"

  ELSE:
    # Entry exists, verify it's current
    Display: "✓ Already tracked in debt register: {item_text}"
```

**Debt entry format:**
```markdown
### {STORY_ID} - {Item Description}

**Deferred Date:** {YYYY-MM-DD}
**Reason:** {external_blocker_description}
**Priority:** High
**Epic:** {epic_id}
**Sprint:** {sprint_number}
**Status:** Open

**Resolution Plan:**
- [ ] {blocker} resolved
- [ ] Schedule follow-up story
- [ ] Complete deferred work

**Updates:**
- {timestamp}: Entry created
```

---

## Step 3: Analyze Debt Trends (Periodic)

### When to Analyze

**Triggered during:**
- Sprint planning (before selecting stories)
- Sprint retrospective (after sprint complete)
- Quarterly technical debt reviews
- Manual invocation: `/audit-deferrals`

### Invoke Technical Debt Analyzer

```
IF invoked during sprint planning OR retrospective:
  Task(
    subagent_type="technical-debt-analyzer",
    description="Analyze accumulated technical debt",
    prompt="Analyze current technical debt from deferred DoD items.

            Data sources:
            - All story files: devforgeai/specs/Stories/*.story.md
            - Debt register: devforgeai/technical-debt-register.md

            Generate trends, identify oldest items, recommend actions.

            Focus on:
            - Items >90 days old (stale debt)
            - Circular deferrals (CRITICAL - must resolve)
            - Pattern detection (common deferral reasons)
            - Deferral rate trends by sprint

            Provide recommendations for upcoming sprint planning.

            Output: Debt analysis report with metrics and recommendations"
  )

  debt_analysis = result from technical-debt-analyzer subagent

  Display debt analysis summary to user
```

---

### Debt Analysis Actions

```
IF debt_analysis.open_items > 10:
  RECOMMEND: "High technical debt ({count} items) - schedule debt reduction sprint"

  AskUserQuestion:
    question: "Technical debt is high ({count} open items). Schedule debt reduction sprint?"
    header: "Debt Reduction"
    options:
      - label: "Yes - create debt reduction sprint"
        description: "Dedicate next sprint to resolving technical debt"
      - label: "No - continue with planned work"
        description: "Acknowledge debt, will address later"
      - label: "Review debt items first"
        description: "Show me the debt register before deciding"
    multiSelect: false

IF debt_analysis.circular_deferrals detected:
  CRITICAL: "Circular deferrals must be resolved immediately"

  List circular chains:
    FOR chain in debt_analysis.circular_chains:
      Display: "- {chain.story_1} → {chain.story_2} → {chain.story_3} → {chain.story_1}"

  Display:
    "⚠️  CIRCULAR DEFERRAL DETECTED

    Story A defers work to Story B
    Story B defers work to Story C
    Story C defers work to Story A

    This creates infinite loop - no story can be completed.

    Resolution required:
    1. Break the circle (complete at least one story fully)
    2. Redesign feature decomposition
    3. Merge circular stories into one larger story"

  HALT orchestration until circular deferrals resolved

IF debt_analysis.stale_items (>90 days) exist:
  WARN: "{count} stale debt items - review and close or escalate"

  Display stale items:
    FOR item in debt_analysis.stale_items:
      Display: "- {item.story_id}: {item.description} (deferred {item.days_old} days ago)"

  RECOMMEND: "Review stale items and either:
              1. Complete deferred work
              2. Close as no longer needed
              3. Document why still deferred"
```

---

## Deferral Type Classification

### Story Split (Most Common)

**Pattern:** Work deferred to separate story

**Valid reasons:**
- Feature too large for single story
- Work identified during implementation
- Separate sprint planning needed

**Required tracking:** Story reference (STORY-XXX)

**Example:**
```
- [ ] Performance optimization: [Deferred - Tracked in STORY-043]
```

---

### Scope Change

**Pattern:** Work excluded from original scope

**Valid reasons:**
- Requirements changed
- Feature no longer needed
- Architectural decision to exclude

**Required tracking:** ADR reference (ADR-XXX)

**Example:**
```
- [ ] OAuth integration: [Deferred - See ADR-015 (scope reduced to basic auth)]
```

---

### External Blocker

**Pattern:** Work cannot proceed due to external dependency

**Valid reasons:**
- Third-party API not available
- Infrastructure not ready
- Upstream team dependency

**Required tracking:** Debt register entry

**Example:**
```
- [ ] Stripe payment integration: [External blocker: Stripe API keys not provisioned]
```

---

### Invalid Deferral (No Tracking)

**Pattern:** Item deferred without story/ADR reference

**Action:** QA validation FAILS (detected by deferral-validator subagent)

**Example (INVALID):**
```
- [ ] Integration testing: [Deferred due to time constraints]
```

**Fix required:**
1. Create tracking story for integration testing
2. Update reason: `[Deferred - Tracked in STORY-044]`
3. OR justify with ADR: `[Deferred - See ADR-016 (integration tests moved to phase 2)]`

---

## Technical Debt Register Structure

### File Location

`devforgeai/technical-debt-register.md`

### Template Format

```markdown
# Technical Debt Register

Track deferred work, external blockers, and technical debt items.

## Summary

- **Total Open Items:** {count}
- **Oldest Item:** {days} days old
- **By Type:**
  - Story splits: {count}
  - Scope changes: {count}
  - External blockers: {count}

## Open Items

### {STORY-ID} - {Item Description}

**Deferred Date:** {YYYY-MM-DD}
**Reason:** {external_blocker_description}
**Priority:** High
**Epic:** {epic_id}
**Sprint:** {sprint_number}
**Status:** Open

**Resolution Plan:**
- [ ] {blocker} resolved
- [ ] Schedule follow-up story
- [ ] Complete deferred work

**Updates:**
- {timestamp}: Entry created
- {timestamp}: Blocker status checked
- {timestamp}: Escalated to {team}

---

## Resolved Items

### {STORY-ID} - {Item Description}

**Deferred Date:** {YYYY-MM-DD}
**Resolved Date:** {YYYY-MM-DD}
**Resolution:** {how_resolved}
**Days Open:** {days}

---

## Circular Deferrals (CRITICAL)

### Chain 1: {STORY-A} → {STORY-B} → {STORY-A}

**Detected:** {timestamp}
**Status:** Unresolved
**Action Required:** Break circle (complete at least one story)

---

## Debt Trends

### By Sprint

| Sprint | Items Deferred | Items Resolved | Net Change |
|--------|----------------|----------------|------------|
| Sprint-1 | 3 | 0 | +3 |
| Sprint-2 | 5 | 2 | +3 |
| Sprint-3 | 2 | 4 | -2 |

### By Type

| Type | Count | Avg Age (days) |
|------|-------|----------------|
| Story splits | 8 | 12 |
| Scope changes | 3 | 45 |
| External blockers | 4 | 67 |
```

**Auto-created:** If doesn't exist, skill creates from template in `assets/templates/technical-debt-register-template.md`

---

## Step 3: Analyze Debt Trends (Periodic)

### Technical Debt Analyzer Subagent

**Invoked during:**
- Sprint planning (before story selection)
- Sprint retrospective (after sprint complete)
- Quarterly reviews
- Manual audit: `/audit-deferrals`

```
Task(
  subagent_type="technical-debt-analyzer",
  description="Analyze accumulated technical debt",
  prompt="Analyze current technical debt from deferred DoD items.

          Data sources:
          - All story files: devforgeai/specs/Stories/*.story.md
          - Debt register: devforgeai/technical-debt-register.md

          Generate trends, identify oldest items, recommend actions.

          Focus on:
          - Items >90 days old (stale debt)
          - Circular deferrals (CRITICAL - must resolve)
          - Pattern detection (common deferral reasons)
          - Deferral rate trends by sprint

          Provide recommendations for upcoming sprint planning.

          Output: Debt analysis report with metrics and recommendations"
)
```

**Subagent analyzes:**
- Total open items
- Age distribution (new, moderate, stale >90 days)
- Circular deferral detection
- Deferral patterns (common reasons)
- Sprint trends (deferral rate increasing/decreasing)

**Subagent returns:**
```json
{
  "open_items": 12,
  "stale_items": 3,
  "circular_deferrals": [
    {"chain": ["STORY-042", "STORY-043", "STORY-042"], "severity": "CRITICAL"}
  ],
  "patterns": {
    "most_common_reason": "Integration testing deferred",
    "frequency": 5
  },
  "trends": {
    "sprint_1": 3,
    "sprint_2": 8,
    "sprint_3": 5,
    "direction": "decreasing"
  },
  "recommendations": [
    "Schedule debt reduction sprint",
    "Resolve circular deferrals immediately",
    "Review stale items >90 days"
  ]
}
```

---

### High Debt Action (>10 Open Items)

```
IF debt_analysis.open_items > 10:
  Display:
  "⚠️  HIGH TECHNICAL DEBT DETECTED

  Open Items: {count}
  Oldest Item: {oldest_age} days old
  Recommendation: Schedule debt reduction sprint

  Items by type:
  - Story splits: {story_splits_count} (need to plan tracking stories)
  - Scope changes: {scope_changes_count} (documented in ADRs)
  - External blockers: {external_count} (waiting on dependencies)

  Action: Dedicate next sprint to closing deferred work"

  RECOMMEND: "High technical debt ({count} items) - schedule debt reduction sprint"
```

---

### Circular Deferral Detection (CRITICAL)

```
IF debt_analysis.circular_deferrals detected:
  Display:
  "🔴 CIRCULAR DEFERRAL DETECTED - CRITICAL

  Circular chains found:
  {FOR chain in debt_analysis.circular_chains:}
    - {chain.story_1} → {chain.story_2} → {chain.story_3} → {chain.story_1}
  {END FOR}

  This creates infinite loop - no story can be completed.

  Resolution required:
  1. Break the circle (complete at least one story fully)
  2. Redesign feature decomposition
  3. Merge circular stories into one larger story

  Orchestration HALTED until circular deferrals resolved."

  CRITICAL: "Circular deferrals must be resolved immediately"

  HALT orchestration until circles broken
```

---

### Stale Debt Warning (>90 Days)

```
IF debt_analysis.stale_items (>90 days) exist:
  Display:
  "⚠️  STALE TECHNICAL DEBT

  Items deferred >90 days ago:
  {FOR item in debt_analysis.stale_items:}
    - {item.story_id}: {item.description}
      Deferred: {item.deferred_date} ({item.days_old} days ago)
      Reason: {item.reason}
  {END FOR}

  Recommended Actions:
  1. Review each item - still needed?
  2. Complete deferred work if still relevant
  3. Close as 'no longer needed' if obsolete
  4. Document why still deferred if blocking"

  WARN: "{count} stale debt items - review and close or escalate"
```

---

## Output

**Deferral Tracking Result:**
```json
{
  "deferrals_found": 5,
  "story_splits": 3,
  "scope_changes": 1,
  "external_blockers": 1,
  "tracking_complete": true,
  "missing_references_created": 2,
  "debt_register_updated": true,
  "debt_analysis": {
    "open_items": 12,
    "stale_items": 3,
    "circular_deferrals": []
  },
  "recommendations": [
    "Monitor external blocker: Stripe API provisioning",
    "Schedule STORY-043 for next sprint"
  ]
}
```

---

## Related Files

- **qa-retry-workflow.md** - Phase 3.5, handles deferral-caused QA failures
- **story-status-update.md** - Phase 3A, updates DoD checkboxes
- **orchestration-finalization.md** - Phase 6, includes debt summary in completion
- **quality-gates.md** - Gate 3 (QA Approval) validates no invalid deferrals
- **story-management.md** - Complete story lifecycle with deferral tracking
